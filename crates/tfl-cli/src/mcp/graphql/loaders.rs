//! Every read of the graph goes through one of these.
//!
//! TfL's REST API hands back foreign keys and nothing else: a `Prediction`
//! carries a `lineId`, a `naptanId` and a `destinationNaptanId`, and following
//! any of them is another request. Resolving those edges naively is what makes
//! a REST-shaped API painful to ask questions of — Kings Cross returns ~60
//! arrivals, so `arrivals { line { name } destination { commonName } }` would
//! be ~120 requests and would exhaust the anonymous rate limit inside one
//! query.
//!
//! | Loader | Request | What batching buys |
//! |---|---|---|
//! | [`LineLoader`] | `/Line/{ids}` | every `line` edge in a query, in one request |
//! | [`StopPointLoader`] | `/StopPoint/{ids}` | ditto for `stopPoint`, `destination` and `affectedStops` |
//! | [`ArrivalsLoader`] | `/StopPoint/{id}/Arrivals` | deduplicates repeated stops; TfL cannot batch this one |
//! | [`LineStopPointsLoader`] | `/Line/{id}/StopPoints` | ditto per line |
//! | [`DisruptionLoader`] | `/Line/{ids}/Disruption` | all disruption edges in one request |
//!
//! Loaders are built per request, so nothing survives between queries: transit
//! data goes stale in seconds and a cache that outlived the request would serve
//! a departure that has already left.

use std::{collections::HashMap, sync::Arc};

use async_graphql::{
    Context, Error as GqlError,
    dataloader::{DataLoader, HashMapCache, Loader},
};
use tfl_api_client::{Client, Error, models};

/// TfL documents "max. approx. 20 ids" for lines and stop points and "max
/// approx. 25" for vehicles. `DataLoader` splits anything larger into
/// concurrent chunks of this size on our behalf, so the cap is configuration
/// rather than code.
const MAX_IDS: usize = 20;

/// Loaders attached to every request's context.
pub struct Loaders {
    pub line: DataLoader<LineLoader, HashMapCache>,
    pub stop_point: DataLoader<StopPointLoader, HashMapCache>,
    pub arrivals: DataLoader<ArrivalsLoader, HashMapCache>,
    pub line_stop_points: DataLoader<LineStopPointsLoader, HashMapCache>,
    pub disruption: DataLoader<DisruptionLoader, HashMapCache>,
}

impl Loaders {
    pub fn new(client: Arc<Client>) -> Self {
        let spawn = tokio::spawn;
        // Every loader caches. Batching alone only collapses keys that arrive
        // in the same window, so two branches of a query asking for the same
        // stop would each fetch it; the cache makes a repeated key free
        // regardless of when it is asked for.
        //
        // The cache is safe precisely because `Loaders` is built per request
        // and dropped with it — see the test that a second query re-fetches.
        let cache = HashMapCache::default;
        Self {
            line: DataLoader::with_cache(LineLoader(client.clone()), spawn, cache())
                .max_batch_size(MAX_IDS),
            stop_point: DataLoader::with_cache(StopPointLoader(client.clone()), spawn, cache())
                .max_batch_size(MAX_IDS),
            // No `max_batch_size` on these two. TfL has no batch form, so they
            // fan out one request per key internally and a cap cannot reduce
            // that — but a cap of 1 dispatches each key the instant it is
            // asked for, which is precisely when two identical keys would
            // otherwise have merged. Leaving it off is what makes asking for
            // the same stop twice cost one request.
            arrivals: DataLoader::with_cache(ArrivalsLoader(client.clone()), spawn, cache()),
            line_stop_points: DataLoader::with_cache(
                LineStopPointsLoader(client.clone()),
                spawn,
                cache(),
            ),
            disruption: DataLoader::with_cache(DisruptionLoader(client), spawn, cache())
                .max_batch_size(MAX_IDS),
        }
    }
}

/// Pulls the loaders out of a resolver's context.
pub fn loaders<'a>(ctx: &Context<'a>) -> &'a Loaders {
    ctx.data_unchecked::<Loaders>()
}

/// A loader error, shared between everyone waiting on the same batch.
///
/// `Loader::Error` must be `Clone` and [`tfl_api_client::Error`] is not, so the
/// error is wrapped rather than duplicated.
pub type LoadError = Arc<Error>;

/// Converts a loader error into a GraphQL one.
///
/// Both types are foreign, so this cannot be an `impl From`.
pub fn to_gql_error(error: impl std::fmt::Display) -> GqlError {
    GqlError::new(error.to_string())
}

/// Runs a batch request, falling back to individual requests if it fails.
///
/// TfL rejects a whole batch when any single id in it is unknown — asking for
/// nineteen real stop points and one typo returns a 404, not nineteen results.
/// Retrying the keys one at a time turns that into what GraphQL wants: the
/// unknown key resolves to null and its nineteen neighbours still answer.
///
/// The retry only fires on a client error. A rate limit or an outage is not
/// something to re-ask twenty times.
async fn load_batch<T, Fut, F>(keys: &[String], fetch: F) -> Result<Vec<T>, LoadError>
where
    F: Fn(Vec<String>) -> Fut,
    Fut: Future<Output = Result<Vec<T>, Error>>,
{
    match fetch(keys.to_vec()).await {
        Ok(items) => Ok(items),
        Err(error) if is_client_error(&error) && keys.len() > 1 => {
            tracing::debug!(%error, keys = keys.len(), "batch rejected, retrying individually");
            let retries = keys.iter().map(|key| fetch(vec![key.clone()]));
            let results = futures_util::future::join_all(retries).await;
            Ok(results.into_iter().flatten().flatten().collect())
        }
        Err(error) => Err(Arc::new(error)),
    }
}

fn is_client_error(error: &Error) -> bool {
    matches!(error, Error::Status { status, .. } if status.is_client_error())
}

/// Loads lines by id — `/Line/{ids}/Status`.
///
/// Deliberately not `/Line/{ids}`, which returns the same line with an empty
/// `lineStatuses`. The status form costs one request either way and carries the
/// thing anyone asking about a line actually wants, so `line(id: "victoria")
/// { statuses { description } }` answers without a second round trip.
pub struct LineLoader(Arc<Client>);

impl Loader<String> for LineLoader {
    type Value = models::Line;
    type Error = LoadError;

    async fn load(&self, keys: &[String]) -> Result<HashMap<String, Self::Value>, Self::Error> {
        let lines = load_batch(keys, |ids| async move {
            let refs: Vec<&str> = ids.iter().map(String::as_str).collect();
            self.0.line_status_by_ids(&refs, &Default::default()).await
        })
        .await?;
        Ok(key_by(lines, |line| line.id.clone()))
    }
}

/// Loads stop points by NaPTAN id — `/StopPoint/{ids}`.
pub struct StopPointLoader(Arc<Client>);

impl Loader<String> for StopPointLoader {
    type Value = models::StopPoint;
    type Error = LoadError;

    async fn load(&self, keys: &[String]) -> Result<HashMap<String, Self::Value>, Self::Error> {
        let stops = load_batch(keys, |ids| async move {
            let refs: Vec<&str> = ids.iter().map(String::as_str).collect();
            self.0.stop_point_get(&refs, &Default::default()).await
        })
        .await?;
        // TfL answers with the interchange hub rather than the stop you asked
        // for: `/StopPoint/940GZZLUKSX` returns `HUBKGX`, with the tube station
        // demoted to a child. Keying on the response's own id would therefore
        // never match the request, so each result is indexed under every id it
        // answers to — its own, its hub's, and its children's — and unasked-for
        // ids are then dropped.
        let mut by_id = HashMap::new();
        for stop in stops {
            let mut ids = vec![
                stop.naptan_id.clone(),
                stop.id.clone(),
                stop.station_naptan.clone(),
                stop.hub_naptan_code.clone(),
            ];
            ids.extend(stop.children.iter().flatten().map(|child| child.id.clone()));
            for id in ids.into_iter().flatten() {
                by_id.entry(id).or_insert_with(|| stop.clone());
            }
        }
        by_id.retain(|id, _| keys.contains(id));
        Ok(by_id)
    }
}

/// Loads live arrivals for a stop — `/StopPoint/{id}/Arrivals`.
///
/// TfL has no batch form, so this is one request per stop. It stays a loader so
/// that asking for arrivals at a stop twice in one query costs one request.
pub struct ArrivalsLoader(Arc<Client>);

impl Loader<String> for ArrivalsLoader {
    type Value = Vec<models::Prediction>;
    type Error = LoadError;

    async fn load(&self, keys: &[String]) -> Result<HashMap<String, Self::Value>, Self::Error> {
        let requests = keys.iter().map(async |id| {
            let arrivals = self.0.stop_point_arrivals(id).await;
            (id.clone(), arrivals)
        });
        collect_per_key(futures_util::future::join_all(requests).await)
    }
}

/// Loads the stops served by a line — `/Line/{id}/StopPoints`.
pub struct LineStopPointsLoader(Arc<Client>);

impl Loader<String> for LineStopPointsLoader {
    type Value = Vec<models::StopPoint>;
    type Error = LoadError;

    async fn load(&self, keys: &[String]) -> Result<HashMap<String, Self::Value>, Self::Error> {
        let requests = keys.iter().map(async |id| {
            let stops = self.0.line_stop_points(id, &Default::default()).await;
            (id.clone(), stops)
        });
        collect_per_key(futures_util::future::join_all(requests).await)
    }
}

/// Loads disruptions affecting lines — `/Line/{ids}/Disruption`.
pub struct DisruptionLoader(Arc<Client>);

impl Loader<String> for DisruptionLoader {
    type Value = Vec<models::Disruption>;
    type Error = LoadError;

    async fn load(&self, keys: &[String]) -> Result<HashMap<String, Self::Value>, Self::Error> {
        // The response does not say which line each disruption came from, so
        // one request per line keeps the association.
        let requests = keys.iter().map(async |id| {
            let disruptions = self.0.line_disruption(&[id]).await;
            (id.clone(), disruptions)
        });
        collect_per_key(futures_util::future::join_all(requests).await)
    }
}

/// Indexes a batch response by key, dropping anything TfL returned without one.
///
/// TfL does not preserve request order, so results are never matched by
/// position. A key it simply omitted stays absent, which `DataLoader` reports as
/// null — the right answer for "no such stop".
fn key_by<T>(items: Vec<T>, key: impl Fn(&T) -> Option<String>) -> HashMap<String, T> {
    items
        .into_iter()
        .filter_map(|item| Some((key(&item)?, item)))
        .collect()
}

/// Collects per-key results, failing only if every request failed.
///
/// One stop with no arrivals should not blank out the rest of the query.
fn collect_per_key<T>(
    results: Vec<(String, Result<T, Error>)>,
) -> Result<HashMap<String, T>, LoadError> {
    let mut first_error = None;
    let mut loaded = HashMap::new();
    for (key, result) in results {
        match result {
            Ok(value) => {
                loaded.insert(key, value);
            }
            Err(error) => {
                tracing::debug!(%error, key, "request failed within a batch");
                first_error.get_or_insert(error);
            }
        }
    }
    match first_error {
        Some(error) if loaded.is_empty() => Err(Arc::new(error)),
        _ => Ok(loaded),
    }
}
