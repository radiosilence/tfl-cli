//! Entry points into the graph.
//!
//! Everything here is a way in; the interesting part is what you can reach from
//! it. Start from a stop and follow `arrivals`, or start from a line and follow
//! `stopPoints` — the edges do the joining, and following the same edge across
//! a list costs one request, not one per item.

use std::sync::Arc;

use async_graphql::{Context, Object, Result};
use tfl_api_client::{Client, StopPointGetByGeoPointOptions, StopPointSearchByQueryOptions};

use super::{
    loaders::{loaders, to_gql_error},
    types::{Line, Mode, Prediction, StopPoint},
};

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    /// A stop point by NaPTAN id, e.g. `940GZZLUKSX`.
    ///
    /// Null if there is no such stop. Use [`Self::search_stop_points`] when you
    /// have a name rather than an id.
    async fn stop_point(
        &self,
        ctx: &Context<'_>,
        #[graphql(desc = "NaPTAN id, e.g. \"940GZZLUKSX\".")] id: String,
    ) -> Result<Option<StopPoint>> {
        Ok(loaders(ctx)
            .stop_point
            .load_one(id.clone())
            .await
            .map_err(to_gql_error)?
            .map(|stop| StopPoint::requested(id, stop)))
    }

    /// Several stop points at once.
    ///
    /// One request per 20 ids. Ids TfL does not recognise are simply absent
    /// from the result rather than failing the query.
    async fn stop_points(
        &self,
        ctx: &Context<'_>,
        #[graphql(desc = "NaPTAN ids. Batched 20 per upstream request.")] ids: Vec<String>,
    ) -> Result<Vec<StopPoint>> {
        let loaded = loaders(ctx)
            .stop_point
            .load_many(ids.clone())
            .await
            .map_err(to_gql_error)?;
        Ok(ids
            .into_iter()
            .filter_map(|id| Some(StopPoint::requested(&id, loaded.get(&id)?.clone())))
            .collect())
    }

    /// Finds stop points by name, e.g. `Kings Cross` or `Baker Street`.
    ///
    /// The usual way in when a person named a station rather than an id. One
    /// request.
    async fn search_stop_points(
        &self,
        ctx: &Context<'_>,
        #[graphql(desc = "A station or stop name. Spelling is forgiven; punctuation is optional.")]
        query: String,
        #[graphql(desc = "Restrict to these modes, e.g. [\"tube\"].")] modes: Option<Vec<String>>,
    ) -> Result<Vec<StopPoint>> {
        let modes = modes.unwrap_or_default();
        let options = StopPointSearchByQueryOptions {
            modes: (!modes.is_empty()).then(|| modes.clone()),
            ..Default::default()
        };
        let response = client(ctx)
            .stop_point_search_by_query(&query, &options)
            .await
            .map_err(to_gql_error)?;

        // Search returns matches, not stop points: it carries ids and names but
        // none of the detail, so the ids go back through the loader.
        let ids: Vec<String> = response
            .matches
            .unwrap_or_default()
            .into_iter()
            .filter_map(|m| m.id)
            .collect();
        if ids.is_empty() {
            return Ok(Vec::new());
        }
        let loaded = loaders(ctx)
            .stop_point
            .load_many(ids.clone())
            .await
            .map_err(to_gql_error)?;
        Ok(ids
            .into_iter()
            .filter_map(|id| Some(StopPoint::requested(&id, loaded.get(&id)?.clone())))
            .collect())
    }

    /// Stop points near a coordinate, nearest first.
    ///
    /// `radius` is in metres. One request.
    async fn stop_points_near(
        &self,
        ctx: &Context<'_>,
        lat: f64,
        lon: f64,
        #[graphql(desc = "Search radius in metres. Defaults to 350.")] radius: Option<i32>,
        #[graphql(
            desc = "NaPTAN stop types. Defaults to metro stations and bus stops; see `stopTypes`."
        )]
        stop_types: Option<Vec<String>>,
    ) -> Result<Vec<StopPoint>> {
        let stop_types = stop_types.unwrap_or_else(|| {
            vec![
                "NaptanMetroStation".to_string(),
                "NaptanRailStation".to_string(),
                "NaptanPublicBusCoachTram".to_string(),
            ]
        });
        let types: Vec<&str> = stop_types.iter().map(String::as_str).collect();
        let options = StopPointGetByGeoPointOptions {
            radius: Some(radius.unwrap_or(350)),
            ..Default::default()
        };
        let response = client(ctx)
            .stop_point_get_by_geo_point(&types, lat, lon, &options)
            .await
            .map_err(to_gql_error)?;
        Ok(response
            .stop_points
            .unwrap_or_default()
            .into_iter()
            .map(StopPoint::new)
            .collect())
    }

    /// A line by id, e.g. `victoria`, `elizabeth`, `n29`.
    async fn line(
        &self,
        ctx: &Context<'_>,
        #[graphql(desc = "Line id, e.g. \"victoria\". Lower case.")] id: String,
    ) -> Result<Option<Line>> {
        Ok(loaders(ctx)
            .line
            .load_one(id)
            .await
            .map_err(to_gql_error)?
            .map(Line))
    }

    /// Several lines at once. One request per 20 ids.
    async fn lines(&self, ctx: &Context<'_>, ids: Vec<String>) -> Result<Vec<Line>> {
        let loaded = loaders(ctx)
            .line
            .load_many(ids.clone())
            .await
            .map_err(to_gql_error)?;
        Ok(ids
            .into_iter()
            .filter_map(|id| loaded.get(&id).cloned())
            .map(Line)
            .collect())
    }

    /// Every line on the given modes, with current status.
    ///
    /// `linesByMode(modes: ["tube"]) { name statuses { description } }` is the
    /// whole-network status board, in one request.
    async fn lines_by_mode(
        &self,
        ctx: &Context<'_>,
        #[graphql(desc = "Modes, e.g. [\"tube\", \"dlr\"]. See `modes` for the full list.")]
        modes: Vec<String>,
    ) -> Result<Vec<Line>> {
        let modes: Vec<&str> = modes.iter().map(String::as_str).collect();
        let lines = client(ctx)
            .line_status_by_mode(&modes, &Default::default())
            .await
            .map_err(to_gql_error)?;
        Ok(lines.into_iter().map(Line).collect())
    }

    /// Lines that are not running a good service.
    ///
    /// The direct answer to "is anything broken right now". One request.
    async fn disrupted_lines(
        &self,
        ctx: &Context<'_>,
        #[graphql(desc = "Modes to check. Defaults to tube, DLR, Overground and Elizabeth line.")]
        modes: Option<Vec<String>>,
    ) -> Result<Vec<Line>> {
        let modes = modes.unwrap_or_else(|| {
            ["tube", "dlr", "overground", "elizabeth-line"]
                .map(String::from)
                .to_vec()
        });
        let modes: Vec<&str> = modes.iter().map(String::as_str).collect();
        let lines = client(ctx)
            .line_status_by_mode(&modes, &Default::default())
            .await
            .map_err(to_gql_error)?;
        Ok(lines
            .into_iter()
            // Severity 10 is "Good Service"; anything else is worth reporting.
            .filter(|line| {
                line.line_statuses
                    .as_ref()
                    .is_some_and(|s| s.iter().any(|s| s.status_severity != Some(10)))
            })
            .map(Line)
            .collect())
    }

    /// Live arrivals for a vehicle, wherever it is going next.
    ///
    /// Takes TfL vehicle ids — a bus registration like `LX58CFV`, or a tube set
    /// number. Batched 20 per request.
    async fn vehicle_arrivals(
        &self,
        ctx: &Context<'_>,
        #[graphql(desc = "Vehicle ids, e.g. [\"LX58CFV\"].")] ids: Vec<String>,
    ) -> Result<Vec<Prediction>> {
        let ids: Vec<&str> = ids.iter().map(String::as_str).collect();
        let arrivals = client(ctx).vehicle_get(&ids).await.map_err(to_gql_error)?;
        Ok(arrivals.into_iter().map(Prediction).collect())
    }

    /// Every mode TfL knows about, e.g. `tube`, `bus`, `river-bus`.
    ///
    /// Worth reading first: it is the vocabulary every `modes` argument
    /// expects. One request, and TfL caches it for twelve hours.
    async fn modes(&self, ctx: &Context<'_>) -> Result<Vec<Mode>> {
        let modes = client(ctx).line_meta_modes().await.map_err(to_gql_error)?;
        Ok(modes.into_iter().map(Mode::from).collect())
    }

    /// The NaPTAN stop types accepted by [`Self::stop_points_near`].
    async fn stop_types(&self, ctx: &Context<'_>) -> Result<Vec<String>> {
        client(ctx)
            .stop_point_meta_stop_types()
            .await
            .map_err(to_gql_error)
    }

    /// TfL's severity codes and what they mean.
    ///
    /// Explains the numbers on [`super::types::LineStatus`]; 10 is "Good
    /// Service".
    async fn severities(&self, ctx: &Context<'_>) -> Result<Vec<Severity>> {
        let severities = client(ctx)
            .line_meta_severity()
            .await
            .map_err(to_gql_error)?;
        Ok(severities
            .into_iter()
            .map(|s| Severity {
                level: s.severity_level,
                description: s.description,
                mode: s.mode_name,
            })
            .collect())
    }
}

/// One of TfL's status severity codes.
#[derive(async_graphql::SimpleObject)]
pub struct Severity {
    /// The number that appears as `LineStatus.severity`.
    pub level: Option<i32>,
    /// What it means, e.g. `Good Service`, `Part Suspended`.
    pub description: Option<String>,
    /// Severity codes differ per mode; this is the mode they apply to.
    pub mode: Option<String>,
}

fn client<'a>(ctx: &Context<'a>) -> &'a Arc<Client> {
    ctx.data_unchecked::<Arc<Client>>()
}
