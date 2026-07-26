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
//! | [`BikeOccupancyLoader`] | `/Occupancy/BikePoints/{ids}` | every docking station's live counts in one request |
//! | [`RoadLoader`] | `/Road/{ids}` | every corridor a disruption names, in one request |
//! | [`RoadDisruptionLoader`] | `/Road/{ids}/Disruption` | ditto per road |
//! | [`ChargeConnectorLoader`] | `/Occupancy/ChargeConnector/{ids}` | every connector's status in one request |
//! | [`AccidentsLoader`] | `/AccidentStats/{year}` | a 37MB year downloaded once, not per field |
//! | [`WholeListLoader`] | the argument-less feeds | each fetched once per query however often it is asked for |
//! | [`TimetableLoader`] | `/Line/{id}/Timetable/{from}` | one per line/stop/direction, deduplicated across a list |
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
    pub bike_occupancy: DataLoader<BikeOccupancyLoader, HashMapCache>,
    pub road: DataLoader<RoadLoader, HashMapCache>,
    pub road_disruption: DataLoader<RoadDisruptionLoader, HashMapCache>,
    pub charge_connector: DataLoader<ChargeConnectorLoader, HashMapCache>,
    pub accidents: DataLoader<AccidentsLoader, HashMapCache>,
    pub stop_disruption: DataLoader<StopDisruptionLoader, HashMapCache>,
    pub whole_list: DataLoader<WholeListLoader, HashMapCache>,
    pub timetable: DataLoader<TimetableLoader, HashMapCache>,
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
            disruption: DataLoader::with_cache(DisruptionLoader(client.clone()), spawn, cache())
                .max_batch_size(MAX_IDS),
            bike_occupancy: DataLoader::with_cache(
                BikeOccupancyLoader(client.clone()),
                spawn,
                cache(),
            )
            .max_batch_size(MAX_IDS),
            // Keyed by `()`: TfL has no way to ask for a subset, so the only
            // thing to batch is the whole set, once.
            road: DataLoader::with_cache(RoadLoader(client.clone()), spawn, cache())
                .max_batch_size(MAX_IDS),
            road_disruption: DataLoader::with_cache(
                RoadDisruptionLoader(client.clone()),
                spawn,
                cache(),
            )
            .max_batch_size(MAX_IDS),
            charge_connector: DataLoader::with_cache(
                ChargeConnectorLoader(client.clone()),
                spawn,
                cache(),
            )
            .max_batch_size(MAX_IDS),
            accidents: DataLoader::with_cache(AccidentsLoader(client.clone()), spawn, cache()),
            stop_disruption: DataLoader::with_cache(
                StopDisruptionLoader(client.clone()),
                spawn,
                cache(),
            )
            .max_batch_size(MAX_IDS),
            whole_list: DataLoader::with_cache(WholeListLoader(client.clone()), spawn, cache()),
            timetable: DataLoader::with_cache(TimetableLoader(client), spawn, cache()),
        }
    }
}

/// Pulls the client out of a resolver's context, for the reads that have no
/// batchable form and so do not go through a loader.
pub fn client<'a>(ctx: &Context<'a>) -> &'a Arc<Client> {
    ctx.data_unchecked::<Arc<Client>>()
}

/// Pulls the loaders out of a resolver's context.
pub fn loaders<'a>(ctx: &Context<'a>) -> &'a Loaders {
    ctx.data_unchecked::<Loaders>()
}

/// One key's result: what was fetched, or why it could not be.
///
/// Loaders that fan out per key return this so a resolver can tell an empty
/// answer from a failed one. `DataLoader` reports an absent key as `None`,
/// which is indistinguishable from a successful empty response — fine for "no
/// such stop", catastrophic for "no disruptions".
pub type Fetched<T> = std::result::Result<T, String>;

/// A loader error, shared between everyone waiting on the same batch.
///
/// `Loader::Error` must be `Clone` and [`tfl_api_client::Error`] is not, so the
/// error is wrapped rather than duplicated.
pub type LoadError = Arc<Error>;

/// Unwraps a per-key fetch, turning a failure into a GraphQL error rather than
/// an empty answer.
pub fn fetched<T: Default>(value: Option<Fetched<T>>) -> Result<T, GqlError> {
    match value {
        Some(Ok(found)) => Ok(found),
        Some(Err(error)) => Err(GqlError::new(error)),
        // Genuinely absent — no such key — which is an empty answer, not a
        // failure.
        None => Ok(T::default()),
    }
}

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

            // Discarding the individual errors would make a revoked key
            // indistinguishable from a typo'd id: twenty 403s would come back
            // as twenty nulls and an untroubled response. A retry that fails
            // for *every* key is not one bad id, so the failure is reported.
            let mut items = Vec::new();
            let mut first_error = None;
            for result in results {
                match result {
                    Ok(found) => items.extend(found),
                    Err(error) => {
                        first_error.get_or_insert(error);
                    }
                }
            }
            match first_error {
                Some(error) if items.is_empty() => Err(Arc::new(error)),
                _ => Ok(items),
            }
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
    type Value = Fetched<Vec<models::Prediction>>;
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
    type Value = Fetched<Vec<models::StopPoint>>;
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
    type Value = Fetched<Vec<models::Disruption>>;
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
fn collect_per_key<K: std::hash::Hash + Eq + std::fmt::Debug, T>(
    results: Vec<(K, Result<T, Error>)>,
) -> Result<HashMap<K, Fetched<T>>, LoadError> {
    // Failures are kept per key rather than dropped. Dropping one made it
    // absent from the map, which `DataLoader` reports as "no value" and every
    // resolver turned into an empty list — so a stop whose arrivals request
    // timed out beside siblings that succeeded read as "no trains due", and a
    // failed disruption fetch read as good service. For a live transport tool
    // that is the worst available answer, and it looked identical to the truth.
    Ok(results
        .into_iter()
        .map(|(key, result)| {
            let value = result.map_err(|error| {
                tracing::debug!(%error, ?key, "request failed within a batch");
                error.to_string()
            });
            (key, value)
        })
        .collect())
}

/// Loads live docking-station counts — `/Occupancy/BikePoints/{ids}`.
pub struct BikeOccupancyLoader(Arc<Client>);

impl Loader<String> for BikeOccupancyLoader {
    type Value = models::BikePointOccupancy;
    type Error = LoadError;

    async fn load(&self, keys: &[String]) -> Result<HashMap<String, Self::Value>, Self::Error> {
        let occupancies = load_batch(keys, |ids| async move {
            let refs: Vec<&str> = ids.iter().map(String::as_str).collect();
            self.0.occupancy_get_bike_points_occupancies(&refs).await
        })
        .await?;
        Ok(key_by(occupancies, |o| o.id.clone()))
    }
}

/// Loads road corridors by id — `/Road/{ids}`.
pub struct RoadLoader(Arc<Client>);

impl Loader<String> for RoadLoader {
    type Value = models::RoadCorridor;
    type Error = LoadError;

    async fn load(&self, keys: &[String]) -> Result<HashMap<String, Self::Value>, Self::Error> {
        let roads = load_batch(keys, |ids| async move {
            let refs: Vec<&str> = ids.iter().map(String::as_str).collect();
            self.0.road_get_by_ids(&refs).await
        })
        .await?;
        // TfL answers lower-case ids but echoes them capitalised as often as
        // not, so both spellings are indexed rather than trusting either.
        let mut by_id = HashMap::new();
        for road in roads {
            for id in [road.id.clone(), road.display_name.clone()]
                .into_iter()
                .flatten()
            {
                by_id
                    .entry(id.to_lowercase())
                    .or_insert_with(|| road.clone());
            }
        }
        Ok(keys
            .iter()
            .filter_map(|key| Some((key.clone(), by_id.get(&key.to_lowercase())?.clone())))
            .collect())
    }
}

/// Loads disruptions on a road — `/Road/{ids}/Disruption`.
///
/// One request per road rather than one batched call: the response does not say
/// which road each disruption came from beyond `corridorIds`, which lists every
/// road it touches, so batching would lose the association the caller asked
/// about.
pub struct RoadDisruptionLoader(Arc<Client>);

impl Loader<String> for RoadDisruptionLoader {
    type Value = Fetched<Vec<models::RoadDisruption>>;
    type Error = LoadError;

    async fn load(&self, keys: &[String]) -> Result<HashMap<String, Self::Value>, Self::Error> {
        let requests = keys.iter().map(async |id| {
            let disruptions = self.0.road_disruption(&[id], &Default::default()).await;
            (id.clone(), disruptions)
        });
        collect_per_key(futures_util::future::join_all(requests).await)
    }
}

/// Loads charge-connector status — `/Occupancy/ChargeConnector/{ids}`.
pub struct ChargeConnectorLoader(Arc<Client>);

impl Loader<String> for ChargeConnectorLoader {
    type Value = models::ChargeConnectorOccupancy;
    type Error = LoadError;

    async fn load(&self, keys: &[String]) -> Result<HashMap<String, Self::Value>, Self::Error> {
        let connectors = load_batch(keys, |ids| async move {
            let refs: Vec<&str> = ids.iter().map(String::as_str).collect();
            self.0.occupancy_get_charge_connector_status(&refs).await
        })
        .await?;
        Ok(key_by(connectors, |c| c.id.map(|id| id.to_string())))
    }
}

/// Loads a year of road casualty data — `/AccidentStats/{year}`.
///
/// Keyed by year because that is the only thing TfL lets you choose. The
/// response is around thirty-seven megabytes, so the loader exists to make
/// certain a query asking about accidents twice downloads it once.
pub struct AccidentsLoader(Arc<Client>);

impl Loader<i32> for AccidentsLoader {
    type Value = Fetched<Vec<models::AccidentDetail>>;
    type Error = LoadError;

    async fn load(&self, keys: &[i32]) -> Result<HashMap<i32, Self::Value>, Self::Error> {
        let requests = keys.iter().map(async |year| {
            let accidents = self.0.accident_stats_get(*year).await;
            (*year, accidents)
        });
        collect_per_key(futures_util::future::join_all(requests).await)
    }
}

/// Loads disruptions at particular stops — `/StopPoint/{ids}/Disruption`.
pub struct StopDisruptionLoader(Arc<Client>);

impl Loader<String> for StopDisruptionLoader {
    type Value = Fetched<Vec<models::DisruptedPoint>>;
    type Error = LoadError;

    async fn load(&self, keys: &[String]) -> Result<HashMap<String, Self::Value>, Self::Error> {
        // One request per stop: the response identifies the affected stop by
        // `atcoCode`, which is not always the id that was asked for, so
        // batching would lose which disruption belongs to which stop.
        let requests = keys.iter().map(async |id| {
            let disruptions = self
                .0
                .stop_point_disruption(&[id], &Default::default())
                .await;
            (id.clone(), disruptions)
        });
        collect_per_key(futures_util::future::join_all(requests).await)
    }
}

/// The feeds that take no arguments, keyed by which one.
///
/// TfL offers no way to ask these for a subset, so there is exactly one request
/// to make and the only thing worth batching is asking for it twice. Routing
/// them through a loader rather than straight at the client means every read in
/// the graph takes the same path — resolver, loader, request — and asking for
/// the vocabulary in two branches of a query costs one request rather than two.
///
/// The response is kept as raw JSON because these feeds return unrelated
/// shapes and a loader has one value type. Each caller decodes its own.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum WholeList {
    /// `/Line/Meta/Modes` — the mode vocabulary.
    Modes,
    /// `/Line/Meta/Severity` — line severity codes.
    LineSeverities,
    /// `/Road/Meta/Severities` — road severity words.
    RoadSeverities,
    /// `/StopPoint/Meta/StopTypes` — NaPTAN stop types.
    StopTypes,
    /// `/Road` — the 24 managed corridors.
    Roads,
    /// `/AirQuality` — the pollution forecast.
    AirQuality,
    /// `/Occupancy/ChargeConnector` — every EV connector's status.
    ChargeConnectors,
    /// `/Occupancy/CarPark` — car park occupancy.
    CarParks,
    /// `/BikePoint` — every docking station.
    BikePoints,
    /// `/Place/Meta/PlaceTypes` — the place-type vocabulary.
    PlaceTypes,
}

pub struct WholeListLoader(Arc<Client>);

impl Loader<WholeList> for WholeListLoader {
    type Value = Fetched<serde_json::Value>;
    type Error = LoadError;

    async fn load(
        &self,
        keys: &[WholeList],
    ) -> Result<HashMap<WholeList, Self::Value>, Self::Error> {
        let requests = keys.iter().map(async |key| {
            let path = match key {
                WholeList::Modes => "/Line/Meta/Modes",
                WholeList::LineSeverities => "/Line/Meta/Severity",
                WholeList::RoadSeverities => "/Road/Meta/Severities",
                WholeList::StopTypes => "/StopPoint/Meta/StopTypes",
                WholeList::Roads => "/Road",
                WholeList::AirQuality => "/AirQuality",
                WholeList::ChargeConnectors => "/Occupancy/ChargeConnector",
                WholeList::CarParks => "/Occupancy/CarPark",
                WholeList::BikePoints => "/BikePoint",
                WholeList::PlaceTypes => "/Place/Meta/PlaceTypes",
            };
            (*key, self.0.get::<serde_json::Value>(path, &[]).await)
        });
        collect_per_key(futures_util::future::join_all(requests).await)
    }
}

/// Fetches one of the argument-less feeds and decodes it.
pub async fn whole_list<T: serde::de::DeserializeOwned + Default>(
    ctx: &Context<'_>,
    which: WholeList,
) -> Result<T, GqlError> {
    let raw = fetched(
        loaders(ctx)
            .whole_list
            .load_one(which)
            .await
            .map_err(to_gql_error)?,
    )?;
    // These feeds are argument-less and stable in shape; a change upstream
    // degrades to empty rather than failing a query that asked for other things
    // too.
    Ok(serde_json::from_value(raw).unwrap_or_default())
}

/// Which timetable to fetch.
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct TimetableKey {
    pub line: String,
    pub from_stop: String,
    /// `inbound`/`outbound`. Absent asks TfL to decide, which it answers with
    /// a disambiguation body when it cannot.
    pub direction: Option<String>,
}

/// Loads scheduled departures — `/Line/{id}/Timetable/{from}`.
///
/// TfL offers no batch form, so this is one request per line, stop and
/// direction. It is a loader so that asking two branches of a query for the
/// same schedule — the obvious shape of "first and last train" — costs one.
pub struct TimetableLoader(Arc<Client>);

impl Loader<TimetableKey> for TimetableLoader {
    type Value = Fetched<models::TimetableResponse>;
    type Error = LoadError;

    async fn load(
        &self,
        keys: &[TimetableKey],
    ) -> Result<HashMap<TimetableKey, Self::Value>, Self::Error> {
        let requests = keys.iter().map(async |key| {
            let response = self
                .0
                .line_timetable_in_direction(&key.line, &key.from_stop, key.direction.as_deref())
                .await;
            (key.clone(), response)
        });
        collect_per_key(futures_util::future::join_all(requests).await)
    }
}
