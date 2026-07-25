//! Entry points into the graph.
//!
//! Everything here is a way in; the interesting part is what you can reach from
//! it. Start from a stop and follow `arrivals`, or start from a line and follow
//! `stopPoints` — the edges do the joining, and following the same edge across
//! a list costs one request, not one per item.

use std::sync::Arc;

use async_graphql::{Context, Object, Result};
use tfl_api_client::{
    Client, Error as TflError, JourneyJourneyResultsOptions, StopPointGetByGeoPointOptions,
    StopPointSearchByQueryOptions,
};

use super::{
    bike::{BikePoint, distance_metres},
    journey::JourneyPlan,
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

    /// Plans a journey between two places.
    ///
    /// `from` and `to` take whatever you have — a NaPTAN id, a station name, a
    /// postcode, a `lat,lon` pair, or a landmark. Names are the normal case and
    /// need no lookup first.
    ///
    /// When TfL cannot tell which place was meant, `journeys` comes back empty
    /// and `isAmbiguous` is true; pick from `fromOptions`/`toOptions` and
    /// re-query with that option's `value`. One request either way.
    // Each argument is a documented field in the SDL, which is what a caller
    // reads before querying. Folding them into an input object would shorten
    // the signature and lose exactly that.
    #[allow(clippy::too_many_arguments)]
    async fn journey(
        &self,
        ctx: &Context<'_>,
        #[graphql(desc = "Origin: a name, NaPTAN id, postcode, or \"lat,lon\".")] from: String,
        #[graphql(desc = "Destination, in the same forms as `from`.")] to: String,
        #[graphql(desc = "Somewhere to route via, in the same forms as `from`.")] via: Option<
            String,
        >,
        #[graphql(desc = "Date as YYYYMMDD. Defaults to today.")] date: Option<String>,
        #[graphql(desc = "Time as HHMM, 24-hour. Defaults to now.")] time: Option<String>,
        #[graphql(
            desc = "Whether `time` is when you want to depart or arrive: \"Departing\" or \"Arriving\". Defaults to departing."
        )]
        time_is: Option<String>,
        #[graphql(
            desc = "Modes to allow, e.g. [\"tube\", \"bus\", \"walking\"]. Defaults to everything."
        )]
        modes: Option<Vec<String>>,
        #[graphql(
            desc = "What to optimise for: \"LeastTime\", \"LeastWalking\", or \"LeastInterchange\"."
        )]
        preference: Option<String>,
        #[graphql(
            desc = "Accessibility needs, e.g. [\"noSolidStairs\", \"stepFreeToVehicle\", \"stepFreeToPlatform\"]."
        )]
        accessibility: Option<Vec<String>>,
        #[graphql(desc = "Walking pace: \"slow\", \"average\", or \"fast\".")]
        walking_speed: Option<String>,
        #[graphql(desc = "Longest walk to accept, in minutes.")] max_walking_minutes: Option<i32>,
    ) -> Result<JourneyPlan> {
        let modes = modes.unwrap_or_default();
        let accessibility = accessibility.unwrap_or_default();
        let options = JourneyJourneyResultsOptions {
            via,
            date,
            time,
            time_is,
            mode: (!modes.is_empty()).then_some(modes),
            journey_preference: preference,
            accessibility_preference: (!accessibility.is_empty()).then_some(accessibility),
            walking_speed,
            max_walking_minutes: max_walking_minutes.map(|m| m.to_string()),
            ..Default::default()
        };

        match client(ctx)
            .journey_journey_results(&from, &to, &options)
            .await
        {
            Ok(result) => Ok(JourneyPlan::from_itinerary(result)),
            // Not a failure: TfL is asking which place was meant. Its own
            // candidates skew heavily to points of interest, so the stop points
            // matching the same term are looked up and offered alongside —
            // see the module docs for why that is not optional.
            Err(TflError::Ambiguous { body, .. }) => {
                let plan = JourneyPlan::from_ambiguous(&body);
                let (from_stations, to_stations) = futures_util::join!(
                    station_options(ctx, &from, plan.from_was_ambiguous()),
                    station_options(ctx, &to, plan.to_was_ambiguous()),
                );
                Ok(plan.with_station_options(from_stations, to_stations))
            }
            Err(error) => Err(to_gql_error(error)),
        }
    }

    /// A Santander Cycles docking station by id, e.g. `BikePoints_1`.
    async fn bike_point(
        &self,
        ctx: &Context<'_>,
        #[graphql(desc = "Docking station id, e.g. \"BikePoints_1\".")] id: String,
    ) -> Result<Option<BikePoint>> {
        Ok(all_bike_points(ctx)
            .await?
            .into_iter()
            .find(|point| point.place.id.as_deref() == Some(id.as_str())))
    }

    /// Docking stations near a coordinate, nearest first.
    ///
    /// The usual question — "where can I get a bike" — so it defaults to
    /// stations that actually have one. Costs one request for the whole set of
    /// around 800 stations, which is then measured and filtered locally
    /// because TfL offers no geographic filter for bike points.
    #[allow(clippy::too_many_arguments)]
    async fn bike_points_near(
        &self,
        ctx: &Context<'_>,
        lat: f64,
        lon: f64,
        #[graphql(desc = "Search radius in metres. Defaults to 500.")] radius: Option<f64>,
        #[graphql(desc = "Only stations with a bike available. Defaults to true.")]
        with_bikes: Option<bool>,
        #[graphql(desc = "Only stations with a free dock, for returning a bike.")]
        with_docks: Option<bool>,
        #[graphql(desc = "How many to return. Defaults to 10.")] first: Option<usize>,
    ) -> Result<Vec<BikePoint>> {
        let radius = radius.unwrap_or(500.0);
        let want_bikes = with_bikes.unwrap_or(true);
        let want_docks = with_docks.unwrap_or(false);

        let mut near: Vec<BikePoint> = Vec::new();
        for mut point in all_bike_points(ctx).await? {
            let (Some(plat), Some(plon)) = (point.place.lat, point.place.lon) else {
                continue;
            };
            let metres = distance_metres((lat, lon), (plat, plon));
            if metres > radius {
                continue;
            }
            if want_bikes && !point.has_bikes() {
                continue;
            }
            if want_docks && !point.has_docks() {
                continue;
            }
            point.distance = Some(metres);
            near.push(point);
        }

        near.sort_by(|a, b| {
            a.distance
                .unwrap_or(f64::MAX)
                .total_cmp(&b.distance.unwrap_or(f64::MAX))
        });
        near.truncate(first.unwrap_or(10));
        Ok(near)
    }

    /// Finds docking stations by name, e.g. `Clerkenwell` or `Hyde Park`.
    ///
    /// One request. Prefer `bikePointsNear` when you have coordinates — TfL's
    /// names are street-level and inconsistent.
    async fn search_bike_points(
        &self,
        ctx: &Context<'_>,
        #[graphql(desc = "Part of a docking station name, e.g. \"River Street\".")] query: String,
    ) -> Result<Vec<BikePoint>> {
        let places = client(ctx)
            .bike_point_search(&query)
            .await
            .map_err(to_gql_error)?;
        Ok(places.into_iter().map(BikePoint::new).collect())
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

/// Stop points matching a term, as journey-planner location options.
///
/// Only runs for a side that was actually ambiguous, and never fails the
/// query: these are a supplement to TfL's own candidates, so a search that
/// errors simply contributes nothing.
async fn station_options(
    ctx: &Context<'_>,
    term: &str,
    needed: bool,
) -> Vec<super::journey::LocationOption> {
    if !needed {
        return Vec::new();
    }
    const MAX_STATIONS: usize = 5;

    let response = match client(ctx)
        .stop_point_search_by_query(term, &Default::default())
        .await
    {
        Ok(response) => response,
        Err(error) => {
            tracing::debug!(%error, term, "stop point search failed while disambiguating");
            return Vec::new();
        }
    };

    let ids: Vec<String> = response
        .matches
        .unwrap_or_default()
        .into_iter()
        .filter_map(|m| m.id)
        .take(MAX_STATIONS)
        .collect();
    if ids.is_empty() {
        return Vec::new();
    }

    let Ok(loaded) = loaders(ctx).stop_point.load_many(ids.clone()).await else {
        return Vec::new();
    };
    ids.iter()
        .filter_map(|id| super::journey::LocationOption::from_stop_point(loaded.get(id)?))
        .collect()
}

/// Every docking station, fetched once per request however many bike fields a
/// query touches.
async fn all_bike_points(ctx: &Context<'_>) -> Result<Vec<BikePoint>> {
    Ok(loaders(ctx)
        .bike_points
        .load_one(())
        .await
        .map_err(to_gql_error)?
        .unwrap_or_default()
        .into_iter()
        .map(BikePoint::new)
        .collect())
}
