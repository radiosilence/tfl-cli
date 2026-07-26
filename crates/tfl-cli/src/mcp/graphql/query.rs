//! Entry points into the graph.
//!
//! Everything here is a way in; the interesting part is what you can reach from
//! it. Start from a stop and follow `arrivals`, or start from a line and follow
//! `stopPoints` — the edges do the joining, and following the same edge across
//! a list costs one request, not one per item.

use async_graphql::{Context, Object, Result};
use tfl_api_client::{
    Error as TflError, JourneyJourneyResultsOptions, StopPointGetByGeoPointOptions,
    StopPointSearchByQueryOptions,
};

use super::{
    bike::{BikePoint, distance_metres},
    environment::{AirQuality, CabwiseBody, TaxiOperator},
    journey::JourneyPlan,
    loaders::{WholeList, client, loaders, to_gql_error, whole_list},
    places::{Accident, CarPark, ChargeConnector},
    road::{Road, RoadDisruption},
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
    #[graphql(
        complexity = "child_complexity.saturating_mul(20).saturating_add(5).min(super::COMPLEXITY_CEILING)"
    )]
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
    #[graphql(
        complexity = "child_complexity.saturating_mul(20).saturating_add(10).min(super::COMPLEXITY_CEILING)"
    )]
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
    #[graphql(
        complexity = "child_complexity.saturating_mul(20).saturating_add(10).min(super::COMPLEXITY_CEILING)"
    )]
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
    #[graphql(
        complexity = "child_complexity.saturating_mul(20).saturating_add(5).min(super::COMPLEXITY_CEILING)"
    )]
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
    #[graphql(
        complexity = "child_complexity.saturating_mul(40).saturating_add(10).min(super::COMPLEXITY_CEILING)"
    )]
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
    #[graphql(
        complexity = "child_complexity.saturating_mul(20).saturating_add(10).min(super::COMPLEXITY_CEILING)"
    )]
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
    /// number. Sent 25 per request, which is TfL's documented maximum.
    async fn vehicle_arrivals(
        &self,
        ctx: &Context<'_>,
        #[graphql(desc = "Vehicle ids, e.g. [\"LX58CFV\"].")] ids: Vec<String>,
    ) -> Result<Vec<Prediction>> {
        // TfL documents "max approx. 25 ids" for this endpoint and there is no
        // loader in front of it, so the chunking has to happen here rather than
        // being inherited.
        const VEHICLE_BATCH: usize = 25;
        let ids: Vec<&str> = ids.iter().map(String::as_str).collect();
        let chunks = ids
            .chunks(VEHICLE_BATCH)
            .map(|chunk| client(ctx).vehicle_get(chunk));
        let mut arrivals = Vec::new();
        for result in futures_util::future::join_all(chunks).await {
            arrivals.extend(result.map_err(to_gql_error)?);
        }
        Ok(arrivals.into_iter().map(Prediction).collect())
    }

    /// Every mode TfL knows about, e.g. `tube`, `bus`, `river-bus`.
    ///
    /// Worth reading first: it is the vocabulary every `modes` argument
    /// expects. One request, and TfL caches it for twelve hours.
    async fn modes(&self, ctx: &Context<'_>) -> Result<Vec<Mode>> {
        let modes: Vec<tfl_api_client::models::Mode> = whole_list(ctx, WholeList::Modes).await?;
        Ok(modes.into_iter().map(Mode::from).collect())
    }

    /// The NaPTAN stop types accepted by [`Self::stop_points_near`].
    async fn stop_types(&self, ctx: &Context<'_>) -> Result<Vec<String>> {
        whole_list(ctx, WholeList::StopTypes).await
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
            desc = "Accessibility needs. TfL accepts exactly: \"noRequirements\", \"noSolidStairs\", \"noEscalators\", \"noElevators\", \"stepFreeToVehicle\", \"stepFreeToPlatform\"."
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
    #[graphql(
        complexity = "child_complexity.saturating_mul(10).saturating_add(20).min(super::COMPLEXITY_CEILING)"
    )]
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

    /// The severity words used by **roads**, and what they mean.
    ///
    /// The vocabulary for `roadDisruptions(severities:)`. Roads grade severity
    /// in words where lines use numbers.
    async fn road_severities(&self, ctx: &Context<'_>) -> Result<Vec<Severity>> {
        let severities: Vec<tfl_api_client::models::StatusSeverity> =
            whole_list(ctx, WholeList::RoadSeverities).await?;
        Ok(severities
            .into_iter()
            .map(|s| Severity {
                level: s.severity_level,
                description: s.description,
                mode: s.mode_name,
            })
            .collect())
    }

    /// Every road corridor TfL manages, with current status. One request.
    ///
    /// Twenty-four of them — the A406, the A2 and so on — so this is the whole
    /// list rather than a page of it.
    #[graphql(
        complexity = "child_complexity.saturating_mul(24).saturating_add(5).min(super::COMPLEXITY_CEILING)"
    )]
    async fn roads(&self, ctx: &Context<'_>) -> Result<Vec<Road>> {
        let roads: Vec<tfl_api_client::models::RoadCorridor> =
            whole_list(ctx, WholeList::Roads).await?;
        Ok(roads.into_iter().map(Road).collect())
    }

    /// A road corridor by id, e.g. `a406`.
    async fn road(
        &self,
        ctx: &Context<'_>,
        #[graphql(desc = "Road id, e.g. \"a406\" or \"a2\". Case-insensitive.")] id: String,
    ) -> Result<Option<Road>> {
        Ok(loaders(ctx)
            .road
            .load_one(id)
            .await
            .map_err(to_gql_error)?
            .map(Road))
    }

    /// Everything currently disrupting London's roads.
    ///
    /// The direct answer to "why is the traffic bad". One request; around a
    /// hundred live at any time, so filter by severity or ask for `first`
    /// unless you want all of them.
    #[graphql(
        complexity = "child_complexity.saturating_mul(20).saturating_add(10).min(super::COMPLEXITY_CEILING)"
    )]
    async fn road_disruptions(
        &self,
        ctx: &Context<'_>,
        #[graphql(desc = "Restrict to these severities, e.g. [\"Serious\", \"Severe\"].")]
        severities: Option<Vec<String>>,
        #[graphql(desc = "Only disruptions closing a road, rather than merely slowing it.")]
        closures_only: Option<bool>,
        #[graphql(desc = "How many to return, most severe first.")] first: Option<usize>,
    ) -> Result<Vec<RoadDisruption>> {
        let severities = severities.unwrap_or_default();
        let options = tfl_api_client::RoadDisruptionOptions {
            // TfL embeds HTML in the descriptions unless asked not to.
            strip_content: Some(true),
            severities: (!severities.is_empty()).then(|| severities.clone()),
            closures: closures_only,
            ..Default::default()
        };
        let mut disruptions = client(ctx)
            .road_disruption(&["all"], &options)
            .await
            .map_err(to_gql_error)?;

        // Worst first: TfL returns them in no useful order, and a caller taking
        // `first` should get the ones that matter.
        disruptions.sort_by_key(|d| severity_rank(d.severity.as_deref()));
        disruptions.truncate(first.unwrap_or(usize::MAX));
        Ok(disruptions.into_iter().map(RoadDisruption).collect())
    }

    /// London's air quality forecast. One request.
    ///
    /// Bands are `Low`, `Moderate`, `High` or `Very High`. Worth pairing with a
    /// cycling or walking journey — `current { band nitrogenDioxide }` answers
    /// "is it a good day to cycle".
    async fn air_quality(&self, ctx: &Context<'_>) -> Result<AirQuality> {
        // The spec types this endpoint as an untyped object, so the shape is
        // ours to assert.
        Ok(AirQuality(whole_list(ctx, WholeList::AirQuality).await?))
    }

    /// Licensed minicab and taxi operators near a coordinate, nearest first.
    ///
    /// For when the tube has stopped running. One request.
    async fn taxi_operators(
        &self,
        ctx: &Context<'_>,
        lat: f64,
        lon: f64,
        #[graphql(desc = "How many to return. Defaults to 10, TfL's maximum is 20.")] first: Option<
            i32,
        >,
        #[graphql(desc = "Only operators that can carry a wheelchair.")]
        wheelchair_accessible: Option<bool>,
        #[graphql(desc = "Only operators taking bookings around the clock.")]
        open_twenty_four_hours: Option<bool>,
    ) -> Result<Vec<TaxiOperator>> {
        let options = tfl_api_client::CabwiseGetOptions {
            max_results: Some(first.unwrap_or(10).clamp(1, 20)),
            wc: wheelchair_accessible.map(|w| w.to_string()),
            twenty_four_seven_only: open_twenty_four_hours,
            ..Default::default()
        };
        let raw = client(ctx)
            .cabwise_get(lat, lon, &options)
            .await
            .map_err(to_gql_error)?;
        let body: CabwiseBody = serde_json::from_value(raw).unwrap_or_default();
        Ok(body
            .operators
            .unwrap_or_default()
            .operator_list
            .into_iter()
            .map(TaxiOperator::from)
            .collect())
    }

    /// Every electric-vehicle charge connector TfL knows about. One request.
    ///
    /// Around 350 of them, so pass `availableOnly` unless you want the lot.
    /// TfL gives no location on this feed — only status — so use `sourceId` to
    /// recognise a site you already know.
    #[graphql(complexity = "child_complexity.saturating_mul(50).saturating_add(10)")]
    async fn charge_connectors(
        &self,
        ctx: &Context<'_>,
        #[graphql(desc = "Only connectors free to use right now.")] available_only: Option<bool>,
        #[graphql(desc = "How many to return.")] first: Option<usize>,
    ) -> Result<Vec<ChargeConnector>> {
        let connectors: Vec<tfl_api_client::models::ChargeConnectorOccupancy> =
            whole_list(ctx, WholeList::ChargeConnectors).await?;
        let mut out: Vec<ChargeConnector> = connectors
            .into_iter()
            .filter(|c| {
                !available_only.unwrap_or(false)
                    || c.status
                        .as_deref()
                        .is_some_and(|s| s.eq_ignore_ascii_case("Available"))
            })
            .map(ChargeConnector)
            .collect();
        out.truncate(first.unwrap_or(usize::MAX));
        Ok(out)
    }

    /// How full TfL's car parks are, usually the ones at tube stations.
    ///
    /// One request. Two caveats: TfL carries **no coordinates** on this feed,
    /// only names, so "the nearest car park" cannot be answered from it; and it
    /// is unreliable enough that a 500 here usually means their problem rather
    /// than yours.
    #[graphql(complexity = "child_complexity.saturating_mul(30).saturating_add(10)")]
    async fn car_parks(&self, ctx: &Context<'_>) -> Result<Vec<CarPark>> {
        let parks: Vec<tfl_api_client::models::CarParkOccupancy> =
            whole_list(ctx, WholeList::CarParks).await?;
        Ok(parks.into_iter().map(CarPark).collect())
    }

    /// Recorded road collisions, for asking how dangerous somewhere is.
    ///
    /// **This is the expensive field.** TfL offers no filtering of its own, so
    /// a year has to be downloaded whole — fifty thousand records and around
    /// thirty-seven megabytes for 2019 — and everything is filtered here
    /// afterwards. Expect several seconds. It is downloaded once per query
    /// however many times you ask.
    ///
    /// Only **2019** has data; every later year returns an error from TfL, so
    /// this is a historical question rather than a current one.
    ///
    /// Give `lat`/`lon` to ask about a junction or street, or `borough` for an
    /// area.
    #[allow(clippy::too_many_arguments)]
    #[graphql(
        complexity = "child_complexity.saturating_mul(25).saturating_add(100).min(super::COMPLEXITY_CEILING)"
    )]
    async fn accidents(
        &self,
        ctx: &Context<'_>,
        #[graphql(desc = "Year to search. Only 2019 has data.", default = 2019)] year: i32,
        #[graphql(desc = "Centre of a geographic search.")] lat: Option<f64>,
        #[graphql(desc = "Centre of a geographic search.")] lon: Option<f64>,
        #[graphql(desc = "Radius in metres around lat/lon. Defaults to 500.")] radius: Option<f64>,
        #[graphql(desc = "Restrict to these severities: \"Fatal\", \"Serious\", \"Slight\".")]
        severities: Option<Vec<String>>,
        #[graphql(desc = "Borough name, matched loosely, e.g. \"Westminster\".")] borough: Option<
            String,
        >,
        #[graphql(desc = "How many to return, nearest first. Defaults to 25.")] first: Option<
            usize,
        >,
    ) -> Result<Vec<Accident>> {
        let near = match (lat, lon) {
            (Some(lat), Some(lon)) => Some((lat, lon)),
            (None, None) => None,
            _ => return Err(to_gql_error("give both lat and lon, or neither")),
        };
        super::places::search(
            ctx,
            year,
            near,
            radius.unwrap_or(500.0),
            &severities.unwrap_or_default(),
            borough.as_deref(),
            first.unwrap_or(25),
        )
        .await
    }

    /// The current time in London, and what it means for the network.
    ///
    /// Every timestamp TfL returns is London local with no zone marker, so
    /// "2026-07-26T00:41:00" is ambiguous without knowing what time it is
    /// there — and London is not UTC for half the year. Read this before
    /// reasoning about whether a departure is soon, or whether the tube is
    /// even running.
    ///
    /// Free: no request at all.
    async fn now(&self) -> Now {
        Now(chrono::Utc::now().with_timezone(&chrono_tz::Europe::London))
    }

    /// The severity codes used by **lines**, and what they mean.
    ///
    /// Explains the numbers on `LineStatus.severity`; 10 is "Good Service".
    ///
    /// Not the vocabulary for `roadDisruptions(severities:)`, which takes words
    /// — see `roadSeverities` — nor for `accidents(severities:)`, which takes
    /// "Fatal", "Serious" or "Slight".
    async fn severities(&self, ctx: &Context<'_>) -> Result<Vec<Severity>> {
        let severities: Vec<tfl_api_client::models::StatusSeverity> =
            whole_list(ctx, WholeList::LineSeverities).await?;
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

/// The current moment in London.
pub struct Now(chrono::DateTime<chrono_tz::Tz>);

#[async_graphql::Object]
impl Now {
    /// ISO 8601 with offset, e.g. `2026-07-26T00:41:00+01:00`.
    ///
    /// The offset is the point: TfL's own timestamps omit it.
    async fn time(&self) -> String {
        self.0.to_rfc3339()
    }

    /// The local time as TfL would write it, for comparing against its
    /// timestamps directly.
    async fn local(&self) -> String {
        self.0.format("%Y-%m-%dT%H:%M:%S").to_string()
    }

    /// e.g. `Saturday`.
    async fn weekday(&self) -> String {
        self.0.format("%A").to_string()
    }

    /// Whether London is on summer time. TfL's timestamps carry no offset, so
    /// this is the only way to know what they mean.
    async fn is_british_summer_time(&self) -> bool {
        use chrono::Offset;
        self.0.offset().fix().local_minus_utc() != 0
    }

    /// Whether the tube is likely to be running.
    ///
    /// A rule of thumb, not a timetable: roughly 05:00 to 00:30, and the Night
    /// Tube runs on some lines on Friday and Saturday nights. Use it to decide
    /// whether to warn someone, not to promise them a train.
    async fn tube_likely_running(&self) -> bool {
        use chrono::Timelike;
        let hour = self.0.hour();
        (5..24).contains(&hour) || hour == 0
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

/// Orders road disruptions worst-first.
///
/// TfL's road severities are words rather than the numbers lines use, and it
/// returns them unordered.
fn severity_rank(severity: Option<&str>) -> u8 {
    match severity.unwrap_or_default() {
        "Severe" => 0,
        "Serious" => 1,
        "Moderate" => 2,
        "Minimal" => 3,
        _ => 4,
    }
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
    let places: Vec<tfl_api_client::models::Place> = whole_list(ctx, WholeList::BikePoints).await?;
    Ok(places.into_iter().map(BikePoint::new).collect())
}
