//! One method per TfL endpoint, generated from the Swagger document. Do not edit.
//!
//! These only build a path and query string; every request goes through
//! [`Client::get`], which owns authentication, caching and retries.

use crate::{Client, Result};

impl Client {
    /// Gets all accident details for accidents occuring in the specified year
    ///
    /// `GET /AccidentStats/{year}`
    pub async fn accident_stats_get(
        &self,
        year: i32,
    ) -> Result<Vec<crate::generated::models::AccidentDetail>> {
        let __path = format!("/AccidentStats/{year}", year = year);
        let mut __query: Vec<(&str, String)> = Vec::new();
        self.get_list(&__path, &__query).await
    }
}

impl Client {
    /// Gets air quality data feed
    ///
    /// `GET /AirQuality`
    pub async fn air_quality_get(&self) -> Result<serde_json::Value> {
        let __path = format!("/AirQuality");
        let mut __query: Vec<(&str, String)> = Vec::new();
        self.get(&__path, &__query).await
    }
}

impl Client {
    /// Gets all bike point locations. The Place object has an addtionalProperties array which contains the nbBikes, nbDocks and nbSpaces
    /// numbers which give the status of the BikePoint. A mismatch in these numbers i.e. nbDocks - (nbBikes + nbSpaces) != 0 indicates broken docks.
    ///
    /// `GET /BikePoint`
    pub async fn bike_point_get_all(&self) -> Result<Vec<crate::generated::models::Place>> {
        let __path = format!("/BikePoint");
        let mut __query: Vec<(&str, String)> = Vec::new();
        self.get_list(&__path, &__query).await
    }
}

impl Client {
    /// Search for bike stations by their name, a bike point's name often contains information about the name of the street
    /// or nearby landmarks, for example. Note that the search result does not contain the PlaceProperties i.e. the status
    /// or occupancy of the BikePoint, to get that information you should retrieve the BikePoint by its id on /BikePoint/id.
    ///
    /// `GET /BikePoint/Search`
    pub async fn bike_point_search(
        &self,
        query: &str,
    ) -> Result<Vec<crate::generated::models::Place>> {
        let __path = format!("/BikePoint/Search");
        let mut __query: Vec<(&str, String)> = Vec::new();
        __query.push(("query", query.to_string()));
        self.get_list(&__path, &__query).await
    }
}

impl Client {
    /// Gets the bike point with the given id.
    ///
    /// `GET /BikePoint/{id}`
    pub async fn bike_point_get(&self, id: &str) -> Result<crate::generated::models::Place> {
        let __path = format!("/BikePoint/{id}", id = crate::segment(id));
        let mut __query: Vec<(&str, String)> = Vec::new();
        self.get(&__path, &__query).await
    }
}

/// Optional query parameters for [`Client::cabwise_get`].
#[derive(Debug, Clone, Default)]
pub struct CabwiseGetOptions {
    /// Operator Type e.g Minicab, Executive, Limousine
    pub optype: Option<String>,
    /// Wheelchair accessible
    pub wc: Option<String>,
    /// The radius of the bounding circle in metres
    pub radius: Option<f64>,
    /// Trading name of operating company
    pub name: Option<String>,
    /// An optional parameter to limit the number of results return. Default and maximum is 20.
    pub max_results: Option<i32>,
    /// Legacy Format
    pub legacy_format: Option<bool>,
    /// Force Xml
    pub force_xml: Option<bool>,
    /// Twenty Four Seven Only
    pub twenty_four_seven_only: Option<bool>,
}

impl Client {
    /// Gets taxis and minicabs contact information
    ///
    /// `GET /Cabwise/search`
    pub async fn cabwise_get(
        &self,
        lat: f64,
        lon: f64,
        options: &CabwiseGetOptions,
    ) -> Result<serde_json::Value> {
        let __path = format!("/Cabwise/search");
        let mut __query: Vec<(&str, String)> = Vec::new();
        __query.push(("lat", lat.to_string()));
        __query.push(("lon", lon.to_string()));
        if let Some(value) = &options.optype {
            __query.push(("optype", value.to_string()));
        }
        if let Some(value) = &options.wc {
            __query.push(("wc", value.to_string()));
        }
        if let Some(value) = &options.radius {
            __query.push(("radius", value.to_string()));
        }
        if let Some(value) = &options.name {
            __query.push(("name", value.to_string()));
        }
        if let Some(value) = &options.max_results {
            __query.push(("maxResults", value.to_string()));
        }
        if let Some(value) = &options.legacy_format {
            __query.push(("legacyFormat", value.to_string()));
        }
        if let Some(value) = &options.force_xml {
            __query.push(("forceXml", value.to_string()));
        }
        if let Some(value) = &options.twenty_four_seven_only {
            __query.push(("twentyFourSevenOnly", value.to_string()));
        }
        self.get(&__path, &__query).await
    }
}

/// Optional query parameters for [`Client::journey_journey_results`].
#[derive(Debug, Clone, Default)]
pub struct JourneyJourneyResultsOptions {
    /// Travel through point on the journey. Can be WGS84 coordinates expressed as "lat,long", a UK postcode, a Naptan (StopPoint) id, an ICS StopId, or a free-text string (will cause disambiguation unless it exactly matches a point of interest name).
    pub via: Option<String>,
    /// Does the journey cover stops outside London? eg. "nationalSearch=true"
    pub national_search: Option<bool>,
    /// The date must be in yyyyMMdd format
    pub date: Option<String>,
    /// The time must be in HHmm format
    pub time: Option<String>,
    /// Does the time given relate to arrival or leaving time? Possible options: "departing" | "arriving"
    pub time_is: Option<String>,
    /// The journey preference eg possible options: "leastinterchange" | "leasttime" | "leastwalking"
    pub journey_preference: Option<String>,
    /// The mode must be a comma separated list of modes. eg possible options: "public-bus,overground,train,tube,coach,dlr,cablecar,tram,river,walking,cycle"
    pub mode: Option<Vec<String>>,
    /// The accessibility preference must be a comma separated list eg. "noSolidStairs,noEscalators,noElevators,stepFreeToVehicle,stepFreeToPlatform"
    pub accessibility_preference: Option<Vec<String>>,
    /// An optional name to associate with the origin of the journey in the results.
    pub from_name: Option<String>,
    /// An optional name to associate with the destination of the journey in the results.
    pub to_name: Option<String>,
    /// An optional name to associate with the via point of the journey in the results.
    pub via_name: Option<String>,
    /// The max walking time in minutes for transfer eg. "120"
    pub max_transfer_minutes: Option<String>,
    /// The max walking time in minutes for journeys eg. "120"
    pub max_walking_minutes: Option<String>,
    /// The walking speed. eg possible options: "slow" | "average" | "fast".
    pub walking_speed: Option<String>,
    /// The cycle preference. eg possible options: "allTheWay" | "leaveAtStation" | "takeOnTransport" | "cycleHire"
    pub cycle_preference: Option<String>,
    /// Time adjustment command. eg possible options: "TripFirst" | "TripLast"
    pub adjustment: Option<String>,
    /// A comma separated list of cycling proficiency levels. eg possible options: "easy,moderate,fast"
    pub bike_proficiency: Option<Vec<String>>,
    /// Option to determine whether to return alternative cycling journey
    pub alternative_cycle: Option<bool>,
    /// Option to determine whether to return alternative walking journey
    pub alternative_walking: Option<bool>,
    /// Flag to determine whether certain text (e.g. walking instructions) should be output with HTML tags or not.
    pub apply_html_markup: Option<bool>,
    /// A boolean to indicate whether or not to return 3 public transport journeys, a bus journey, a cycle hire journey, a personal cycle journey and a walking journey
    pub use_multi_modal_call: Option<bool>,
    /// A boolean to indicate whether to optimize journeys using walking
    pub walking_optimization: Option<bool>,
    /// A boolean to indicate whether to return one or more taxi journeys. Note, setting this to true will override "useMultiModalCall".
    pub taxi_only_trip: Option<bool>,
    /// A boolean to indicate whether public transport routes should include directions between platforms and station entrances.
    pub route_between_entrances: Option<bool>,
    /// A boolean to indicate if we want to receive real time live arrivals data where available.
    pub use_real_time_live_arrivals: Option<bool>,
    /// A boolean to make Journey Planner calculate journeys in one temporal direction only. In other words, only calculate journeys after the 'depart' time, or before the 'arrive' time. By default, the Journey Planner engine (EFA) calculates journeys in both temporal directions.
    pub calc_one_direction: Option<bool>,
    /// A boolean to make Journey Planner return alternative routes. Alternative routes are calculated by removing one or more lines included in the fastest route and re-calculating. By default, these journeys will not be returned.
    pub include_alternative_routes: Option<bool>,
    /// An optional integer to indicate what multi modal scenario we want to use.
    pub override_multi_modal_scenario: Option<i32>,
    /// A boolean to indicate whether walking leg to station entrance and walking leg from station entrance to platform should be combined. Defaults to true
    pub combine_transfer_legs: Option<bool>,
}

impl Client {
    /// Perform a Journey Planner search from the parameters specified in simple types
    ///
    /// `GET /Journey/JourneyResults/{from}/to/{to}`
    pub async fn journey_journey_results(
        &self,
        from: &str,
        to: &str,
        options: &JourneyJourneyResultsOptions,
    ) -> Result<crate::generated::models::ItineraryResult> {
        let __path = format!(
            "/Journey/JourneyResults/{from}/to/{to}",
            from = crate::segment(from),
            to = crate::segment(to)
        );
        let mut __query: Vec<(&str, String)> = Vec::new();
        if let Some(value) = &options.via {
            __query.push(("via", value.to_string()));
        }
        if let Some(value) = &options.national_search {
            __query.push(("nationalSearch", value.to_string()));
        }
        if let Some(value) = &options.date {
            __query.push(("date", value.to_string()));
        }
        if let Some(value) = &options.time {
            __query.push(("time", value.to_string()));
        }
        if let Some(value) = &options.time_is {
            __query.push(("timeIs", value.to_string()));
        }
        if let Some(value) = &options.journey_preference {
            __query.push(("journeyPreference", value.to_string()));
        }
        if let Some(value) = &options.mode {
            for item in value {
                __query.push(("mode", item.to_string()));
            }
        }
        if let Some(value) = &options.accessibility_preference {
            for item in value {
                __query.push(("accessibilityPreference", item.to_string()));
            }
        }
        if let Some(value) = &options.from_name {
            __query.push(("fromName", value.to_string()));
        }
        if let Some(value) = &options.to_name {
            __query.push(("toName", value.to_string()));
        }
        if let Some(value) = &options.via_name {
            __query.push(("viaName", value.to_string()));
        }
        if let Some(value) = &options.max_transfer_minutes {
            __query.push(("maxTransferMinutes", value.to_string()));
        }
        if let Some(value) = &options.max_walking_minutes {
            __query.push(("maxWalkingMinutes", value.to_string()));
        }
        if let Some(value) = &options.walking_speed {
            __query.push(("walkingSpeed", value.to_string()));
        }
        if let Some(value) = &options.cycle_preference {
            __query.push(("cyclePreference", value.to_string()));
        }
        if let Some(value) = &options.adjustment {
            __query.push(("adjustment", value.to_string()));
        }
        if let Some(value) = &options.bike_proficiency {
            for item in value {
                __query.push(("bikeProficiency", item.to_string()));
            }
        }
        if let Some(value) = &options.alternative_cycle {
            __query.push(("alternativeCycle", value.to_string()));
        }
        if let Some(value) = &options.alternative_walking {
            __query.push(("alternativeWalking", value.to_string()));
        }
        if let Some(value) = &options.apply_html_markup {
            __query.push(("applyHtmlMarkup", value.to_string()));
        }
        if let Some(value) = &options.use_multi_modal_call {
            __query.push(("useMultiModalCall", value.to_string()));
        }
        if let Some(value) = &options.walking_optimization {
            __query.push(("walkingOptimization", value.to_string()));
        }
        if let Some(value) = &options.taxi_only_trip {
            __query.push(("taxiOnlyTrip", value.to_string()));
        }
        if let Some(value) = &options.route_between_entrances {
            __query.push(("routeBetweenEntrances", value.to_string()));
        }
        if let Some(value) = &options.use_real_time_live_arrivals {
            __query.push(("useRealTimeLiveArrivals", value.to_string()));
        }
        if let Some(value) = &options.calc_one_direction {
            __query.push(("calcOneDirection", value.to_string()));
        }
        if let Some(value) = &options.include_alternative_routes {
            __query.push(("includeAlternativeRoutes", value.to_string()));
        }
        if let Some(value) = &options.override_multi_modal_scenario {
            __query.push(("overrideMultiModalScenario", value.to_string()));
        }
        if let Some(value) = &options.combine_transfer_legs {
            __query.push(("combineTransferLegs", value.to_string()));
        }
        self.get(&__path, &__query).await
    }
}

impl Client {
    /// Gets a list of all of the available journey planner modes
    ///
    /// `GET /Journey/Meta/Modes`
    pub async fn journey_meta(&self) -> Result<Vec<crate::generated::models::Mode>> {
        let __path = format!("/Journey/Meta/Modes");
        let mut __query: Vec<(&str, String)> = Vec::new();
        self.get_list(&__path, &__query).await
    }
}

impl Client {
    /// Gets a list of valid disruption categories
    ///
    /// `GET /Line/Meta/DisruptionCategories`
    pub async fn line_meta_disruption_categories(&self) -> Result<Vec<String>> {
        let __path = format!("/Line/Meta/DisruptionCategories");
        let mut __query: Vec<(&str, String)> = Vec::new();
        self.get_list(&__path, &__query).await
    }
}

impl Client {
    /// Gets a list of valid modes
    ///
    /// `GET /Line/Meta/Modes`
    pub async fn line_meta_modes(&self) -> Result<Vec<crate::generated::models::Mode>> {
        let __path = format!("/Line/Meta/Modes");
        let mut __query: Vec<(&str, String)> = Vec::new();
        self.get_list(&__path, &__query).await
    }
}

impl Client {
    /// Gets a list of valid ServiceTypes to filter on
    ///
    /// `GET /Line/Meta/ServiceTypes`
    pub async fn line_meta_service_types(&self) -> Result<Vec<String>> {
        let __path = format!("/Line/Meta/ServiceTypes");
        let mut __query: Vec<(&str, String)> = Vec::new();
        self.get_list(&__path, &__query).await
    }
}

impl Client {
    /// Gets a list of valid severity codes
    ///
    /// `GET /Line/Meta/Severity`
    pub async fn line_meta_severity(
        &self,
    ) -> Result<Vec<crate::generated::models::StatusSeverity>> {
        let __path = format!("/Line/Meta/Severity");
        let mut __query: Vec<(&str, String)> = Vec::new();
        self.get_list(&__path, &__query).await
    }
}

impl Client {
    /// Gets lines that serve the given modes.
    ///
    /// `GET /Line/Mode/{modes}`
    pub async fn line_get_by_mode(
        &self,
        modes: &[&str],
    ) -> Result<Vec<crate::generated::models::Line>> {
        let __path = format!("/Line/Mode/{modes}", modes = crate::join(modes));
        let mut __query: Vec<(&str, String)> = Vec::new();
        self.get_list(&__path, &__query).await
    }
}

impl Client {
    /// Get disruptions for all lines of the given modes.
    ///
    /// `GET /Line/Mode/{modes}/Disruption`
    pub async fn line_disruption_by_mode(
        &self,
        modes: &[&str],
    ) -> Result<Vec<crate::generated::models::Disruption>> {
        let __path = format!("/Line/Mode/{modes}/Disruption", modes = crate::join(modes));
        let mut __query: Vec<(&str, String)> = Vec::new();
        self.get_list(&__path, &__query).await
    }
}

/// Optional query parameters for [`Client::line_route_by_mode`].
#[derive(Debug, Clone, Default)]
pub struct LineRouteByModeOptions {
    /// A comma seperated list of service types to filter on. Supported values: Regular, Night. Defaulted to 'Regular' if not specified
    pub service_types: Option<Vec<String>>,
}

impl Client {
    /// Gets all lines and their valid routes for given modes, including the name and id of the originating and terminating stops for each route
    ///
    /// `GET /Line/Mode/{modes}/Route`
    pub async fn line_route_by_mode(
        &self,
        modes: &[&str],
        options: &LineRouteByModeOptions,
    ) -> Result<Vec<crate::generated::models::Line>> {
        let __path = format!("/Line/Mode/{modes}/Route", modes = crate::join(modes));
        let mut __query: Vec<(&str, String)> = Vec::new();
        if let Some(value) = &options.service_types {
            for item in value {
                __query.push(("serviceTypes", item.to_string()));
            }
        }
        self.get_list(&__path, &__query).await
    }
}

/// Optional query parameters for [`Client::line_status_by_mode`].
#[derive(Debug, Clone, Default)]
pub struct LineStatusByModeOptions {
    /// Include details of the disruptions that are causing the line status including the affected stops and routes
    pub detail: Option<bool>,
    /// If specified, ensures that only those line status(es) are returned within the lines that have disruptions with the matching severity level.
    pub severity_level: Option<String>,
}

impl Client {
    /// Gets the line status of for all lines for the given modes
    ///
    /// `GET /Line/Mode/{modes}/Status`
    pub async fn line_status_by_mode(
        &self,
        modes: &[&str],
        options: &LineStatusByModeOptions,
    ) -> Result<Vec<crate::generated::models::Line>> {
        let __path = format!("/Line/Mode/{modes}/Status", modes = crate::join(modes));
        let mut __query: Vec<(&str, String)> = Vec::new();
        if let Some(value) = &options.detail {
            __query.push(("detail", value.to_string()));
        }
        if let Some(value) = &options.severity_level {
            __query.push(("severityLevel", value.to_string()));
        }
        self.get_list(&__path, &__query).await
    }
}

/// Optional query parameters for [`Client::line_route`].
#[derive(Debug, Clone, Default)]
pub struct LineRouteOptions {
    /// A comma seperated list of service types to filter on. Supported values: Regular, Night. Defaulted to 'Regular' if not specified
    pub service_types: Option<Vec<String>>,
}

impl Client {
    /// Get all valid routes for all lines, including the name and id of the originating and terminating stops for each route.
    ///
    /// `GET /Line/Route`
    pub async fn line_route(
        &self,
        options: &LineRouteOptions,
    ) -> Result<Vec<crate::generated::models::Line>> {
        let __path = format!("/Line/Route");
        let mut __query: Vec<(&str, String)> = Vec::new();
        if let Some(value) = &options.service_types {
            for item in value {
                __query.push(("serviceTypes", item.to_string()));
            }
        }
        self.get_list(&__path, &__query).await
    }
}

/// Optional query parameters for [`Client::line_search`].
#[derive(Debug, Clone, Default)]
pub struct LineSearchOptions {
    /// Optionally filter by the specified modes
    pub modes: Option<Vec<String>>,
    /// A comma seperated list of service types to filter on. Supported values: Regular, Night. Defaulted to 'Regular' if not specified
    pub service_types: Option<Vec<String>>,
}

impl Client {
    /// Search for lines or routes matching the query string
    ///
    /// `GET /Line/Search/{query}`
    pub async fn line_search(
        &self,
        query: &str,
        options: &LineSearchOptions,
    ) -> Result<crate::generated::models::RouteSearchResponse> {
        let __path = format!("/Line/Search/{query}", query = crate::segment(query));
        let mut __query: Vec<(&str, String)> = Vec::new();
        if let Some(value) = &options.modes {
            for item in value {
                __query.push(("modes", item.to_string()));
            }
        }
        if let Some(value) = &options.service_types {
            for item in value {
                __query.push(("serviceTypes", item.to_string()));
            }
        }
        self.get(&__path, &__query).await
    }
}

impl Client {
    /// Gets the line status for all lines with a given severity
    /// A list of valid severity codes can be obtained from a call to Line/Meta/Severity
    ///
    /// `GET /Line/Status/{severity}`
    pub async fn line_status_by_severity(
        &self,
        severity: i32,
    ) -> Result<Vec<crate::generated::models::Line>> {
        let __path = format!("/Line/Status/{severity}", severity = severity);
        let mut __query: Vec<(&str, String)> = Vec::new();
        self.get_list(&__path, &__query).await
    }
}

impl Client {
    /// Gets lines that match the specified line ids.
    ///
    /// `GET /Line/{ids}`
    pub async fn line_get(&self, ids: &[&str]) -> Result<Vec<crate::generated::models::Line>> {
        let __path = format!("/Line/{ids}", ids = crate::join(ids));
        let mut __query: Vec<(&str, String)> = Vec::new();
        self.get_list(&__path, &__query).await
    }
}

/// Optional query parameters for [`Client::line_arrivals`].
#[derive(Debug, Clone, Default)]
pub struct LineArrivalsOptions {
    /// Optional. The direction of travel. Can be inbound or outbound or all. If left blank, and destinationStopId is set, will default to all
    pub direction: Option<String>,
    /// Optional. Id of destination stop
    pub destination_station_id: Option<String>,
}

impl Client {
    /// Get the list of arrival predictions for given line ids based at the given stop
    ///
    /// `GET /Line/{ids}/Arrivals/{stopPointId}`
    pub async fn line_arrivals(
        &self,
        ids: &[&str],
        stop_point_id: &str,
        options: &LineArrivalsOptions,
    ) -> Result<Vec<crate::generated::models::Prediction>> {
        let __path = format!(
            "/Line/{ids}/Arrivals/{stop_point_id}",
            ids = crate::join(ids),
            stop_point_id = crate::segment(stop_point_id)
        );
        let mut __query: Vec<(&str, String)> = Vec::new();
        if let Some(value) = &options.direction {
            __query.push(("direction", value.to_string()));
        }
        if let Some(value) = &options.destination_station_id {
            __query.push(("destinationStationId", value.to_string()));
        }
        self.get_list(&__path, &__query).await
    }
}

impl Client {
    /// Get disruptions for the given line ids
    ///
    /// `GET /Line/{ids}/Disruption`
    pub async fn line_disruption(
        &self,
        ids: &[&str],
    ) -> Result<Vec<crate::generated::models::Disruption>> {
        let __path = format!("/Line/{ids}/Disruption", ids = crate::join(ids));
        let mut __query: Vec<(&str, String)> = Vec::new();
        self.get_list(&__path, &__query).await
    }
}

/// Optional query parameters for [`Client::line_line_routes_by_ids`].
#[derive(Debug, Clone, Default)]
pub struct LineLineRoutesByIdsOptions {
    /// A comma seperated list of service types to filter on. Supported values: Regular, Night. Defaulted to 'Regular' if not specified
    pub service_types: Option<Vec<String>>,
}

impl Client {
    /// Get all valid routes for given line ids, including the name and id of the originating and terminating stops for each route.
    ///
    /// `GET /Line/{ids}/Route`
    pub async fn line_line_routes_by_ids(
        &self,
        ids: &[&str],
        options: &LineLineRoutesByIdsOptions,
    ) -> Result<Vec<crate::generated::models::Line>> {
        let __path = format!("/Line/{ids}/Route", ids = crate::join(ids));
        let mut __query: Vec<(&str, String)> = Vec::new();
        if let Some(value) = &options.service_types {
            for item in value {
                __query.push(("serviceTypes", item.to_string()));
            }
        }
        self.get_list(&__path, &__query).await
    }
}

/// Optional query parameters for [`Client::line_status_by_ids`].
#[derive(Debug, Clone, Default)]
pub struct LineStatusByIdsOptions {
    /// Include details of the disruptions that are causing the line status including the affected stops and routes
    pub detail: Option<bool>,
}

impl Client {
    /// Gets the line status of for given line ids e.g Minor Delays
    ///
    /// `GET /Line/{ids}/Status`
    pub async fn line_status_by_ids(
        &self,
        ids: &[&str],
        options: &LineStatusByIdsOptions,
    ) -> Result<Vec<crate::generated::models::Line>> {
        let __path = format!("/Line/{ids}/Status", ids = crate::join(ids));
        let mut __query: Vec<(&str, String)> = Vec::new();
        if let Some(value) = &options.detail {
            __query.push(("detail", value.to_string()));
        }
        self.get_list(&__path, &__query).await
    }
}

/// Optional query parameters for [`Client::line_status`].
#[derive(Debug, Clone, Default)]
pub struct LineStatusOptions {
    /// Include details of the disruptions that are causing the line status including the affected stops and routes
    pub detail: Option<bool>,
    pub date_range_start_date: Option<String>,
    pub date_range_end_date: Option<String>,
}

impl Client {
    /// Gets the line status for given line ids during the provided dates e.g Minor Delays
    ///
    /// `GET /Line/{ids}/Status/{StartDate}/to/{EndDate}`
    pub async fn line_status(
        &self,
        ids: &[&str],
        start_date: &str,
        end_date: &str,
        options: &LineStatusOptions,
    ) -> Result<Vec<crate::generated::models::Line>> {
        let __path = format!(
            "/Line/{ids}/Status/{start_date}/to/{end_date}",
            ids = crate::join(ids),
            start_date = crate::segment(start_date),
            end_date = crate::segment(end_date)
        );
        let mut __query: Vec<(&str, String)> = Vec::new();
        if let Some(value) = &options.detail {
            __query.push(("detail", value.to_string()));
        }
        if let Some(value) = &options.date_range_start_date {
            __query.push(("dateRange.startDate", value.to_string()));
        }
        if let Some(value) = &options.date_range_end_date {
            __query.push(("dateRange.endDate", value.to_string()));
        }
        self.get_list(&__path, &__query).await
    }
}

/// Optional query parameters for [`Client::line_route_sequence`].
#[derive(Debug, Clone, Default)]
pub struct LineRouteSequenceOptions {
    /// A comma seperated list of service types to filter on. Supported values: Regular, Night. Defaulted to 'Regular' if not specified
    pub service_types: Option<Vec<String>>,
    /// That excludes crowding from line disruptions. Can be true or false.
    pub exclude_crowding: Option<bool>,
}

impl Client {
    /// Gets all valid routes for given line id, including the sequence of stops on each route.
    ///
    /// `GET /Line/{id}/Route/Sequence/{direction}`
    pub async fn line_route_sequence(
        &self,
        id: &str,
        direction: &str,
        options: &LineRouteSequenceOptions,
    ) -> Result<crate::generated::models::RouteSequence> {
        let __path = format!(
            "/Line/{id}/Route/Sequence/{direction}",
            id = crate::segment(id),
            direction = crate::segment(direction)
        );
        let mut __query: Vec<(&str, String)> = Vec::new();
        if let Some(value) = &options.service_types {
            for item in value {
                __query.push(("serviceTypes", item.to_string()));
            }
        }
        if let Some(value) = &options.exclude_crowding {
            __query.push(("excludeCrowding", value.to_string()));
        }
        self.get(&__path, &__query).await
    }
}

/// Optional query parameters for [`Client::line_stop_points`].
#[derive(Debug, Clone, Default)]
pub struct LineStopPointsOptions {
    /// If the national-rail line is requested, this flag will filter the national rail stations so that only those operated by TfL are returned
    pub tfl_operated_national_rail_stations_only: Option<bool>,
}

impl Client {
    /// Gets a list of the stations that serve the given line id
    ///
    /// `GET /Line/{id}/StopPoints`
    pub async fn line_stop_points(
        &self,
        id: &str,
        options: &LineStopPointsOptions,
    ) -> Result<Vec<crate::generated::models::StopPoint>> {
        let __path = format!("/Line/{id}/StopPoints", id = crate::segment(id));
        let mut __query: Vec<(&str, String)> = Vec::new();
        if let Some(value) = &options.tfl_operated_national_rail_stations_only {
            __query.push(("tflOperatedNationalRailStationsOnly", value.to_string()));
        }
        self.get_list(&__path, &__query).await
    }
}

impl Client {
    /// Gets the timetable for a specified station on the give line
    ///
    /// `GET /Line/{id}/Timetable/{fromStopPointId}`
    pub async fn line_timetable(
        &self,
        id: &str,
        from_stop_point_id: &str,
    ) -> Result<crate::generated::models::TimetableResponse> {
        let __path = format!(
            "/Line/{id}/Timetable/{from_stop_point_id}",
            id = crate::segment(id),
            from_stop_point_id = crate::segment(from_stop_point_id)
        );
        let mut __query: Vec<(&str, String)> = Vec::new();
        self.get(&__path, &__query).await
    }
}

impl Client {
    /// Gets the timetable for a specified station on the give line with specified destination
    ///
    /// `GET /Line/{id}/Timetable/{fromStopPointId}/to/{toStopPointId}`
    pub async fn line_timetable_to(
        &self,
        id: &str,
        from_stop_point_id: &str,
        to_stop_point_id: &str,
    ) -> Result<crate::generated::models::TimetableResponse> {
        let __path = format!(
            "/Line/{id}/Timetable/{from_stop_point_id}/to/{to_stop_point_id}",
            id = crate::segment(id),
            from_stop_point_id = crate::segment(from_stop_point_id),
            to_stop_point_id = crate::segment(to_stop_point_id)
        );
        let mut __query: Vec<(&str, String)> = Vec::new();
        self.get(&__path, &__query).await
    }
}

impl Client {
    /// Returns the service type active for a mode.
    /// Currently only supports tube
    ///
    /// `GET /Mode/ActiveServiceTypes`
    pub async fn mode_get_active_service_types(
        &self,
    ) -> Result<Vec<crate::generated::models::ActiveServiceType>> {
        let __path = format!("/Mode/ActiveServiceTypes");
        let mut __query: Vec<(&str, String)> = Vec::new();
        self.get_list(&__path, &__query).await
    }
}

/// Optional query parameters for [`Client::mode_arrivals`].
#[derive(Debug, Clone, Default)]
pub struct ModeArrivalsOptions {
    /// A number of arrivals to return for each stop, -1 to return all available.
    pub count: Option<i32>,
}

impl Client {
    /// Gets the next arrival predictions for all stops of a given mode
    ///
    /// `GET /Mode/{mode}/Arrivals`
    pub async fn mode_arrivals(
        &self,
        mode: &str,
        options: &ModeArrivalsOptions,
    ) -> Result<Vec<crate::generated::models::Prediction>> {
        let __path = format!("/Mode/{mode}/Arrivals", mode = crate::segment(mode));
        let mut __query: Vec<(&str, String)> = Vec::new();
        if let Some(value) = &options.count {
            __query.push(("count", value.to_string()));
        }
        self.get_list(&__path, &__query).await
    }
}

impl Client {
    /// Get the occupancy for bike points.
    ///
    /// `GET /Occupancy/BikePoints/{ids}`
    pub async fn occupancy_get_bike_points_occupancies(
        &self,
        ids: &[&str],
    ) -> Result<Vec<crate::generated::models::BikePointOccupancy>> {
        let __path = format!("/Occupancy/BikePoints/{ids}", ids = crate::join(ids));
        let mut __query: Vec<(&str, String)> = Vec::new();
        self.get_list(&__path, &__query).await
    }
}

impl Client {
    /// Gets the occupancy for all car parks that have occupancy data
    ///
    /// `GET /Occupancy/CarPark`
    pub async fn occupancy_get(&self) -> Result<Vec<crate::generated::models::CarParkOccupancy>> {
        let __path = format!("/Occupancy/CarPark");
        let mut __query: Vec<(&str, String)> = Vec::new();
        self.get_list(&__path, &__query).await
    }
}

impl Client {
    /// Gets the occupancy for a car park with a given id
    ///
    /// `GET /Occupancy/CarPark/{id}`
    pub async fn occupancy_get_by_id(
        &self,
        id: &str,
    ) -> Result<crate::generated::models::CarParkOccupancy> {
        let __path = format!("/Occupancy/CarPark/{id}", id = crate::segment(id));
        let mut __query: Vec<(&str, String)> = Vec::new();
        self.get(&__path, &__query).await
    }
}

impl Client {
    /// Gets the occupancy for all charge connectors
    ///
    /// `GET /Occupancy/ChargeConnector`
    pub async fn occupancy_get_all_charge_connector_status(
        &self,
    ) -> Result<Vec<crate::generated::models::ChargeConnectorOccupancy>> {
        let __path = format!("/Occupancy/ChargeConnector");
        let mut __query: Vec<(&str, String)> = Vec::new();
        self.get_list(&__path, &__query).await
    }
}

impl Client {
    /// Gets the occupancy for a charge connectors with a given id (sourceSystemPlaceId)
    ///
    /// `GET /Occupancy/ChargeConnector/{ids}`
    pub async fn occupancy_get_charge_connector_status(
        &self,
        ids: &[&str],
    ) -> Result<Vec<crate::generated::models::ChargeConnectorOccupancy>> {
        let __path = format!("/Occupancy/ChargeConnector/{ids}", ids = crate::join(ids));
        let mut __query: Vec<(&str, String)> = Vec::new();
        self.get_list(&__path, &__query).await
    }
}

/// Optional query parameters for [`Client::place_get_by_geo`].
#[derive(Debug, Clone, Default)]
pub struct PlaceGetByGeoOptions {
    /// The radius of the bounding circle in metres when only lat/lon are specified.
    pub radius: Option<f64>,
    /// An optional list of comma separated property categories to return in the Place's property bag. If null or empty, all categories of property are returned. Pass the keyword "none" to return no properties (a valid list of categories can be obtained from the /Place/Meta/categories endpoint)
    pub categories: Option<Vec<String>>,
    /// Defaults to false. If true child places e.g. individual charging stations at a charge point while be included, otherwise just the URLs of any child places will be returned
    pub include_children: Option<bool>,
    /// Place types to filter on, or null to return all types
    pub r#type: Option<Vec<String>>,
    /// An optional parameter to limit the results to active records only (Currently only the 'VariableMessageSign' place type is supported)
    pub active_only: Option<bool>,
    /// If specified, limits the number of returned places equal to the given value
    pub number_of_places_to_return: Option<i32>,
    pub place_geo_sw_lat: Option<f64>,
    pub place_geo_sw_lon: Option<f64>,
    pub place_geo_ne_lat: Option<f64>,
    pub place_geo_ne_lon: Option<f64>,
    pub place_geo_lat: Option<f64>,
    pub place_geo_lon: Option<f64>,
}

impl Client {
    /// Gets the places that lie within a geographic region. The geographic region of interest can either be specified
    /// by using a lat/lon geo-point and a radius in metres to return places within the locus defined by the lat/lon of
    /// its centre or alternatively, by the use of a bounding box defined by the lat/lon of its north-west and south-east corners.
    /// Optionally filters on type and can strip properties for a smaller payload.
    ///
    /// `GET /Place`
    pub async fn place_get_by_geo(
        &self,
        options: &PlaceGetByGeoOptions,
    ) -> Result<Vec<crate::generated::models::StopPoint>> {
        let __path = format!("/Place");
        let mut __query: Vec<(&str, String)> = Vec::new();
        if let Some(value) = &options.radius {
            __query.push(("radius", value.to_string()));
        }
        if let Some(value) = &options.categories {
            for item in value {
                __query.push(("categories", item.to_string()));
            }
        }
        if let Some(value) = &options.include_children {
            __query.push(("includeChildren", value.to_string()));
        }
        if let Some(value) = &options.r#type {
            for item in value {
                __query.push(("type", item.to_string()));
            }
        }
        if let Some(value) = &options.active_only {
            __query.push(("activeOnly", value.to_string()));
        }
        if let Some(value) = &options.number_of_places_to_return {
            __query.push(("numberOfPlacesToReturn", value.to_string()));
        }
        if let Some(value) = &options.place_geo_sw_lat {
            __query.push(("placeGeo.swLat", value.to_string()));
        }
        if let Some(value) = &options.place_geo_sw_lon {
            __query.push(("placeGeo.swLon", value.to_string()));
        }
        if let Some(value) = &options.place_geo_ne_lat {
            __query.push(("placeGeo.neLat", value.to_string()));
        }
        if let Some(value) = &options.place_geo_ne_lon {
            __query.push(("placeGeo.neLon", value.to_string()));
        }
        if let Some(value) = &options.place_geo_lat {
            __query.push(("placeGeo.lat", value.to_string()));
        }
        if let Some(value) = &options.place_geo_lon {
            __query.push(("placeGeo.lon", value.to_string()));
        }
        self.get_list(&__path, &__query).await
    }
}

/// Optional query parameters for [`Client::place_get_streets_by_post_code`].
#[derive(Debug, Clone, Default)]
pub struct PlaceGetStreetsByPostCodeOptions {
    pub postcode_input_postcode: Option<String>,
}

impl Client {
    /// Gets the set of streets associated with a post code.
    ///
    /// `GET /Place/Address/Streets/{Postcode}`
    pub async fn place_get_streets_by_post_code(
        &self,
        postcode: &str,
        options: &PlaceGetStreetsByPostCodeOptions,
    ) -> Result<serde_json::Value> {
        let __path = format!(
            "/Place/Address/Streets/{postcode}",
            postcode = crate::segment(postcode)
        );
        let mut __query: Vec<(&str, String)> = Vec::new();
        if let Some(value) = &options.postcode_input_postcode {
            __query.push(("postcodeInput.postcode", value.to_string()));
        }
        self.get(&__path, &__query).await
    }
}

impl Client {
    /// Gets a list of all of the available place property categories and keys.
    ///
    /// `GET /Place/Meta/Categories`
    pub async fn place_meta_categories(
        &self,
    ) -> Result<Vec<crate::generated::models::PlaceCategory>> {
        let __path = format!("/Place/Meta/Categories");
        let mut __query: Vec<(&str, String)> = Vec::new();
        self.get_list(&__path, &__query).await
    }
}

impl Client {
    /// Gets a list of the available types of Place.
    ///
    /// `GET /Place/Meta/PlaceTypes`
    pub async fn place_meta_place_types(
        &self,
    ) -> Result<Vec<crate::generated::models::PlaceCategory>> {
        let __path = format!("/Place/Meta/PlaceTypes");
        let mut __query: Vec<(&str, String)> = Vec::new();
        self.get_list(&__path, &__query).await
    }
}

/// Optional query parameters for [`Client::place_search`].
#[derive(Debug, Clone, Default)]
pub struct PlaceSearchOptions {
    /// A comma-separated list of the types to return. Max. approx 12 types.
    pub types: Option<Vec<String>>,
}

impl Client {
    /// Gets all places that matches the given query
    ///
    /// `GET /Place/Search`
    pub async fn place_search(
        &self,
        name: &str,
        options: &PlaceSearchOptions,
    ) -> Result<Vec<crate::generated::models::Place>> {
        let __path = format!("/Place/Search");
        let mut __query: Vec<(&str, String)> = Vec::new();
        __query.push(("name", name.to_string()));
        if let Some(value) = &options.types {
            for item in value {
                __query.push(("types", item.to_string()));
            }
        }
        self.get_list(&__path, &__query).await
    }
}

/// Optional query parameters for [`Client::place_get_by_type`].
#[derive(Debug, Clone, Default)]
pub struct PlaceGetByTypeOptions {
    /// An optional parameter to limit the results to active records only (Currently only the 'VariableMessageSign' place type is supported)
    pub active_only: Option<bool>,
}

impl Client {
    /// Gets all places of a given type
    ///
    /// `GET /Place/Type/{types}`
    pub async fn place_get_by_type(
        &self,
        types: &[&str],
        options: &PlaceGetByTypeOptions,
    ) -> Result<Vec<crate::generated::models::Place>> {
        let __path = format!("/Place/Type/{types}", types = crate::join(types));
        let mut __query: Vec<(&str, String)> = Vec::new();
        if let Some(value) = &options.active_only {
            __query.push(("activeOnly", value.to_string()));
        }
        self.get_list(&__path, &__query).await
    }
}

/// Optional query parameters for [`Client::place_get`].
#[derive(Debug, Clone, Default)]
pub struct PlaceGetOptions {
    /// Defaults to false. If true child places e.g. individual charging stations at a charge point while be included, otherwise just the URLs of any child places will be returned
    pub include_children: Option<bool>,
}

impl Client {
    /// Gets the place with the given id.
    ///
    /// `GET /Place/{id}`
    pub async fn place_get(
        &self,
        id: &str,
        options: &PlaceGetOptions,
    ) -> Result<Vec<crate::generated::models::Place>> {
        let __path = format!("/Place/{id}", id = crate::segment(id));
        let mut __query: Vec<(&str, String)> = Vec::new();
        if let Some(value) = &options.include_children {
            __query.push(("includeChildren", value.to_string()));
        }
        self.get_list(&__path, &__query).await
    }
}

impl Client {
    /// Gets any places of the given type whose geography intersects the given latitude and longitude. In practice this means the Place
    /// must be polygonal e.g. a BoroughBoundary.
    ///
    /// `GET /Place/{type}/At/{Lat}/{Lon}`
    pub async fn place_get_at(
        &self,
        r#type: &[&str],
        lat: &str,
        lon: &str,
        location_lat: f64,
        location_lon: f64,
    ) -> Result<serde_json::Value> {
        let __path = format!("/Place/{type}/At/{lat}/{lon}", type = crate::join(r#type), lat = crate::segment(lat), lon = crate::segment(lon));
        let mut __query: Vec<(&str, String)> = Vec::new();
        __query.push(("location.lat", location_lat.to_string()));
        __query.push(("location.lon", location_lon.to_string()));
        self.get(&__path, &__query).await
    }
}

impl Client {
    /// Gets the place overlay for a given set of co-ordinates and a given width/height.
    ///
    /// `GET /Place/{type}/overlay/{z}/{Lat}/{Lon}/{width}/{height}`
    pub async fn place_get_overlay(
        &self,
        r#type: &[&str],
        z: i32,
        lat: &str,
        lon: &str,
        width: i32,
        height: i32,
        location_lat: f64,
        location_lon: f64,
    ) -> Result<serde_json::Value> {
        let __path = format!("/Place/{type}/overlay/{z}/{lat}/{lon}/{width}/{height}", type = crate::join(r#type), z = z, lat = crate::segment(lat), lon = crate::segment(lon), width = width, height = height);
        let mut __query: Vec<(&str, String)> = Vec::new();
        __query.push(("location.lat", location_lat.to_string()));
        __query.push(("location.lon", location_lon.to_string()));
        self.get(&__path, &__query).await
    }
}

impl Client {
    /// Gets all roads managed by TfL
    ///
    /// `GET /Road`
    pub async fn road_get(&self) -> Result<Vec<crate::generated::models::RoadCorridor>> {
        let __path = format!("/Road");
        let mut __query: Vec<(&str, String)> = Vec::new();
        self.get_list(&__path, &__query).await
    }
}

impl Client {
    /// Gets a list of valid RoadDisruption categories
    ///
    /// `GET /Road/Meta/Categories`
    pub async fn road_meta_categories(&self) -> Result<Vec<String>> {
        let __path = format!("/Road/Meta/Categories");
        let mut __query: Vec<(&str, String)> = Vec::new();
        self.get_list(&__path, &__query).await
    }
}

impl Client {
    /// Gets a list of valid RoadDisruption severity codes
    ///
    /// `GET /Road/Meta/Severities`
    pub async fn road_meta_severities(
        &self,
    ) -> Result<Vec<crate::generated::models::StatusSeverity>> {
        let __path = format!("/Road/Meta/Severities");
        let mut __query: Vec<(&str, String)> = Vec::new();
        self.get_list(&__path, &__query).await
    }
}

/// Optional query parameters for [`Client::road_disruption_by_id`].
#[derive(Debug, Clone, Default)]
pub struct RoadDisruptionByIdOptions {
    /// Optional, defaults to false. When true, removes every property/node except for id, point, severity, severityDescription, startDate, endDate, corridor details, location and comments.
    pub strip_content: Option<bool>,
}

impl Client {
    /// Gets a list of active disruptions filtered by disruption Ids.
    ///
    /// `GET /Road/all/Disruption/{disruptionIds}`
    pub async fn road_disruption_by_id(
        &self,
        disruption_ids: &[&str],
        options: &RoadDisruptionByIdOptions,
    ) -> Result<crate::generated::models::RoadDisruption> {
        let __path = format!(
            "/Road/all/Disruption/{disruption_ids}",
            disruption_ids = crate::join(disruption_ids)
        );
        let mut __query: Vec<(&str, String)> = Vec::new();
        if let Some(value) = &options.strip_content {
            __query.push(("stripContent", value.to_string()));
        }
        self.get(&__path, &__query).await
    }
}

impl Client {
    /// Gets a list of disrupted streets. If no date filters are provided, current disruptions are returned.
    ///
    /// `GET /Road/all/Street/Disruption`
    pub async fn road_disrupted_streets(
        &self,
        start_date: &str,
        end_date: &str,
    ) -> Result<serde_json::Value> {
        let __path = format!("/Road/all/Street/Disruption");
        let mut __query: Vec<(&str, String)> = Vec::new();
        __query.push(("startDate", start_date.to_string()));
        __query.push(("endDate", end_date.to_string()));
        self.get(&__path, &__query).await
    }
}

impl Client {
    /// Gets the road with the specified id (e.g. A1)
    ///
    /// `GET /Road/{ids}`
    pub async fn road_get_by_ids(
        &self,
        ids: &[&str],
    ) -> Result<Vec<crate::generated::models::RoadCorridor>> {
        let __path = format!("/Road/{ids}", ids = crate::join(ids));
        let mut __query: Vec<(&str, String)> = Vec::new();
        self.get_list(&__path, &__query).await
    }
}

/// Optional query parameters for [`Client::road_disruption`].
#[derive(Debug, Clone, Default)]
pub struct RoadDisruptionOptions {
    /// Optional, defaults to false. When true, removes every property/node except for id, point, severity, severityDescription, startDate, endDate, corridor details, location, comments and streets
    pub strip_content: Option<bool>,
    /// an optional list of Severity names to filter on (a valid list of severities can be obtained from the /Road/Meta/severities endpoint)
    pub severities: Option<Vec<String>>,
    /// an optional list of category names to filter on (a valid list of categories can be obtained from the /Road/Meta/categories endpoint)
    pub categories: Option<Vec<String>>,
    /// Optional, defaults to true. When true, always includes disruptions that have road closures, regardless of the severity filter. When false, the severity filter works as normal.
    pub closures: Option<bool>,
}

impl Client {
    /// Get active disruptions, filtered by road ids
    ///
    /// `GET /Road/{ids}/Disruption`
    pub async fn road_disruption(
        &self,
        ids: &[&str],
        options: &RoadDisruptionOptions,
    ) -> Result<Vec<crate::generated::models::RoadDisruption>> {
        let __path = format!("/Road/{ids}/Disruption", ids = crate::join(ids));
        let mut __query: Vec<(&str, String)> = Vec::new();
        if let Some(value) = &options.strip_content {
            __query.push(("stripContent", value.to_string()));
        }
        if let Some(value) = &options.severities {
            for item in value {
                __query.push(("severities", item.to_string()));
            }
        }
        if let Some(value) = &options.categories {
            for item in value {
                __query.push(("categories", item.to_string()));
            }
        }
        if let Some(value) = &options.closures {
            __query.push(("closures", value.to_string()));
        }
        self.get_list(&__path, &__query).await
    }
}

/// Optional query parameters for [`Client::road_status`].
#[derive(Debug, Clone, Default)]
pub struct RoadStatusOptions {
    pub date_range_nullable_start_date: Option<String>,
    pub date_range_nullable_end_date: Option<String>,
}

impl Client {
    /// Gets the specified roads with the status aggregated over the date range specified, or now until the end of today if no dates are passed.
    ///
    /// `GET /Road/{ids}/Status`
    pub async fn road_status(
        &self,
        ids: &[&str],
        options: &RoadStatusOptions,
    ) -> Result<Vec<crate::generated::models::RoadCorridor>> {
        let __path = format!("/Road/{ids}/Status", ids = crate::join(ids));
        let mut __query: Vec<(&str, String)> = Vec::new();
        if let Some(value) = &options.date_range_nullable_start_date {
            __query.push(("dateRangeNullable.startDate", value.to_string()));
        }
        if let Some(value) = &options.date_range_nullable_end_date {
            __query.push(("dateRangeNullable.endDate", value.to_string()));
        }
        self.get_list(&__path, &__query).await
    }
}

impl Client {
    /// Search the site for occurrences of the query string. The maximum number of results returned is equal to the maximum page size
    /// of 100. To return subsequent pages, use the paginated overload.
    ///
    /// `GET /Search`
    pub async fn search_get(
        &self,
        query: &str,
    ) -> Result<crate::generated::models::SearchResponse> {
        let __path = format!("/Search");
        let mut __query: Vec<(&str, String)> = Vec::new();
        __query.push(("query", query.to_string()));
        self.get(&__path, &__query).await
    }
}

impl Client {
    /// Searches the bus schedules folder on S3 for a given bus number.
    ///
    /// `GET /Search/BusSchedules`
    pub async fn search_bus_schedules(
        &self,
        query: &str,
    ) -> Result<crate::generated::models::SearchResponse> {
        let __path = format!("/Search/BusSchedules");
        let mut __query: Vec<(&str, String)> = Vec::new();
        __query.push(("query", query.to_string()));
        self.get(&__path, &__query).await
    }
}

impl Client {
    /// Gets the available search categories.
    ///
    /// `GET /Search/Meta/Categories`
    pub async fn search_meta_categories(&self) -> Result<Vec<String>> {
        let __path = format!("/Search/Meta/Categories");
        let mut __query: Vec<(&str, String)> = Vec::new();
        self.get_list(&__path, &__query).await
    }
}

impl Client {
    /// Gets the available searchProvider names.
    ///
    /// `GET /Search/Meta/SearchProviders`
    pub async fn search_meta_search_providers(&self) -> Result<Vec<String>> {
        let __path = format!("/Search/Meta/SearchProviders");
        let mut __query: Vec<(&str, String)> = Vec::new();
        self.get_list(&__path, &__query).await
    }
}

impl Client {
    /// Gets the available sorting options.
    ///
    /// `GET /Search/Meta/Sorts`
    pub async fn search_meta_sorts(&self) -> Result<Vec<String>> {
        let __path = format!("/Search/Meta/Sorts");
        let mut __query: Vec<(&str, String)> = Vec::new();
        self.get_list(&__path, &__query).await
    }
}

/// Optional query parameters for [`Client::stop_point_get_by_geo_point`].
#[derive(Debug, Clone, Default)]
pub struct StopPointGetByGeoPointOptions {
    /// the radius of the bounding circle in metres (default : 200)
    pub radius: Option<i32>,
    /// Re-arrange the output into a parent/child hierarchy
    pub use_stop_point_hierarchy: Option<bool>,
    /// the list of modes to search (comma separated mode names e.g. tube,dlr)
    pub modes: Option<Vec<String>>,
    /// an optional list of comma separated property categories to return in the StopPoint's property bag. If null or empty, all categories of property are returned. Pass the keyword "none" to return no properties (a valid list of categories can be obtained from the /StopPoint/Meta/categories endpoint)
    pub categories: Option<Vec<String>>,
    /// true to return the lines that each stop point serves as a nested resource
    pub return_lines: Option<bool>,
}

impl Client {
    /// Gets a list of StopPoints within {radius} by the specified criteria
    ///
    /// `GET /StopPoint`
    pub async fn stop_point_get_by_geo_point(
        &self,
        stop_types: &[&str],
        location_lat: f64,
        location_lon: f64,
        options: &StopPointGetByGeoPointOptions,
    ) -> Result<crate::generated::models::StopPointsResponse> {
        let __path = format!("/StopPoint");
        let mut __query: Vec<(&str, String)> = Vec::new();
        for item in stop_types {
            __query.push(("stopTypes", item.to_string()));
        }
        __query.push(("location.lat", location_lat.to_string()));
        __query.push(("location.lon", location_lon.to_string()));
        if let Some(value) = &options.radius {
            __query.push(("radius", value.to_string()));
        }
        if let Some(value) = &options.use_stop_point_hierarchy {
            __query.push(("useStopPointHierarchy", value.to_string()));
        }
        if let Some(value) = &options.modes {
            for item in value {
                __query.push(("modes", item.to_string()));
            }
        }
        if let Some(value) = &options.categories {
            for item in value {
                __query.push(("categories", item.to_string()));
            }
        }
        if let Some(value) = &options.return_lines {
            __query.push(("returnLines", value.to_string()));
        }
        self.get(&__path, &__query).await
    }
}

impl Client {
    /// Gets the list of available StopPoint additional information categories
    ///
    /// `GET /StopPoint/Meta/Categories`
    pub async fn stop_point_meta_categories(
        &self,
    ) -> Result<Vec<crate::generated::models::StopPointCategory>> {
        let __path = format!("/StopPoint/Meta/Categories");
        let mut __query: Vec<(&str, String)> = Vec::new();
        self.get_list(&__path, &__query).await
    }
}

impl Client {
    /// Gets the list of available StopPoint modes
    ///
    /// `GET /StopPoint/Meta/Modes`
    pub async fn stop_point_meta_modes(&self) -> Result<Vec<crate::generated::models::Mode>> {
        let __path = format!("/StopPoint/Meta/Modes");
        let mut __query: Vec<(&str, String)> = Vec::new();
        self.get_list(&__path, &__query).await
    }
}

impl Client {
    /// Gets the list of available StopPoint types
    ///
    /// `GET /StopPoint/Meta/StopTypes`
    pub async fn stop_point_meta_stop_types(&self) -> Result<Vec<String>> {
        let __path = format!("/StopPoint/Meta/StopTypes");
        let mut __query: Vec<(&str, String)> = Vec::new();
        self.get_list(&__path, &__query).await
    }
}

/// Optional query parameters for [`Client::stop_point_get_by_mode`].
#[derive(Debug, Clone, Default)]
pub struct StopPointGetByModeOptions {
    /// The data set page to return. Page 1 equates to the first 1000 stop points, page 2 equates to 1001-2000 etc. Must be entered for bus mode as data set is too large.
    pub page: Option<i32>,
}

impl Client {
    /// Gets a list of StopPoints filtered by the modes available at that StopPoint.
    ///
    /// `GET /StopPoint/Mode/{modes}`
    pub async fn stop_point_get_by_mode(
        &self,
        modes: &[&str],
        options: &StopPointGetByModeOptions,
    ) -> Result<crate::generated::models::StopPointsResponse> {
        let __path = format!("/StopPoint/Mode/{modes}", modes = crate::join(modes));
        let mut __query: Vec<(&str, String)> = Vec::new();
        if let Some(value) = &options.page {
            __query.push(("page", value.to_string()));
        }
        self.get(&__path, &__query).await
    }
}

/// Optional query parameters for [`Client::stop_point_disruption_by_mode`].
#[derive(Debug, Clone, Default)]
pub struct StopPointDisruptionByModeOptions {
    pub include_route_blocked_stops: Option<bool>,
}

impl Client {
    /// Gets a distinct list of disrupted stop points for the given modes
    ///
    /// `GET /StopPoint/Mode/{modes}/Disruption`
    pub async fn stop_point_disruption_by_mode(
        &self,
        modes: &[&str],
        options: &StopPointDisruptionByModeOptions,
    ) -> Result<Vec<crate::generated::models::DisruptedPoint>> {
        let __path = format!(
            "/StopPoint/Mode/{modes}/Disruption",
            modes = crate::join(modes)
        );
        let mut __query: Vec<(&str, String)> = Vec::new();
        if let Some(value) = &options.include_route_blocked_stops {
            __query.push(("includeRouteBlockedStops", value.to_string()));
        }
        self.get_list(&__path, &__query).await
    }
}

/// Optional query parameters for [`Client::stop_point_search`].
#[derive(Debug, Clone, Default)]
pub struct StopPointSearchOptions {
    /// An optional, parameter separated list of the modes to filter by
    pub modes: Option<Vec<String>>,
    /// True to only return stations in that have Fares data available for single fares to another station.
    pub fares_only: Option<bool>,
    /// An optional result limit, defaulting to and with a maximum of 50. Since children of the stop point heirarchy are returned for matches,
    /// it is possible that the flattened result set will contain more than 50 items.
    pub max_results: Option<i32>,
    /// An optional, parameter separated list of the lines to filter by
    pub lines: Option<Vec<String>>,
    /// If true, returns results including HUBs.
    pub include_hubs: Option<bool>,
    /// If the national-rail mode is included, this flag will filter the national rail stations so that only those operated by TfL are returned
    pub tfl_operated_national_rail_stations_only: Option<bool>,
}

impl Client {
    /// Search StopPoints by their common name, or their 5-digit Countdown Bus Stop Code.
    ///
    /// `GET /StopPoint/Search`
    pub async fn stop_point_search(
        &self,
        query: &str,
        options: &StopPointSearchOptions,
    ) -> Result<crate::generated::models::SearchResponse> {
        let __path = format!("/StopPoint/Search");
        let mut __query: Vec<(&str, String)> = Vec::new();
        __query.push(("query", query.to_string()));
        if let Some(value) = &options.modes {
            for item in value {
                __query.push(("modes", item.to_string()));
            }
        }
        if let Some(value) = &options.fares_only {
            __query.push(("faresOnly", value.to_string()));
        }
        if let Some(value) = &options.max_results {
            __query.push(("maxResults", value.to_string()));
        }
        if let Some(value) = &options.lines {
            for item in value {
                __query.push(("lines", item.to_string()));
            }
        }
        if let Some(value) = &options.include_hubs {
            __query.push(("includeHubs", value.to_string()));
        }
        if let Some(value) = &options.tfl_operated_national_rail_stations_only {
            __query.push(("tflOperatedNationalRailStationsOnly", value.to_string()));
        }
        self.get(&__path, &__query).await
    }
}

/// Optional query parameters for [`Client::stop_point_search_by_query`].
#[derive(Debug, Clone, Default)]
pub struct StopPointSearchByQueryOptions {
    /// An optional, parameter separated list of the modes to filter by
    pub modes: Option<Vec<String>>,
    /// True to only return stations in that have Fares data available for single fares to another station.
    pub fares_only: Option<bool>,
    /// An optional result limit, defaulting to and with a maximum of 50. Since children of the stop point heirarchy are returned for matches,
    /// it is possible that the flattened result set will contain more than 50 items.
    pub max_results: Option<i32>,
    /// An optional, parameter separated list of the lines to filter by
    pub lines: Option<Vec<String>>,
    /// If true, returns results including HUBs.
    pub include_hubs: Option<bool>,
    /// If the national-rail mode is included, this flag will filter the national rail stations so that only those operated by TfL are returned
    pub tfl_operated_national_rail_stations_only: Option<bool>,
}

impl Client {
    /// Search StopPoints by their common name, or their 5-digit Countdown Bus Stop Code.
    ///
    /// `GET /StopPoint/Search/{query}`
    pub async fn stop_point_search_by_query(
        &self,
        query: &str,
        options: &StopPointSearchByQueryOptions,
    ) -> Result<crate::generated::models::SearchResponse> {
        let __path = format!("/StopPoint/Search/{query}", query = crate::segment(query));
        let mut __query: Vec<(&str, String)> = Vec::new();
        if let Some(value) = &options.modes {
            for item in value {
                __query.push(("modes", item.to_string()));
            }
        }
        if let Some(value) = &options.fares_only {
            __query.push(("faresOnly", value.to_string()));
        }
        if let Some(value) = &options.max_results {
            __query.push(("maxResults", value.to_string()));
        }
        if let Some(value) = &options.lines {
            for item in value {
                __query.push(("lines", item.to_string()));
            }
        }
        if let Some(value) = &options.include_hubs {
            __query.push(("includeHubs", value.to_string()));
        }
        if let Some(value) = &options.tfl_operated_national_rail_stations_only {
            __query.push(("tflOperatedNationalRailStationsOnly", value.to_string()));
        }
        self.get(&__path, &__query).await
    }
}

/// Optional query parameters for [`Client::stop_point_get_service_types`].
#[derive(Debug, Clone, Default)]
pub struct StopPointGetServiceTypesOptions {
    /// The lines which contain the given Naptan id (all lines relevant to the given stoppoint if empty)
    pub line_ids: Option<Vec<String>>,
    /// The modes which the lines are relevant to (all if empty)
    pub modes: Option<Vec<String>>,
}

impl Client {
    /// Gets the service types for a given stoppoint
    ///
    /// `GET /StopPoint/ServiceTypes`
    pub async fn stop_point_get_service_types(
        &self,
        id: &str,
        options: &StopPointGetServiceTypesOptions,
    ) -> Result<Vec<crate::generated::models::LineServiceType>> {
        let __path = format!("/StopPoint/ServiceTypes");
        let mut __query: Vec<(&str, String)> = Vec::new();
        __query.push(("id", id.to_string()));
        if let Some(value) = &options.line_ids {
            for item in value {
                __query.push(("lineIds", item.to_string()));
            }
        }
        if let Some(value) = &options.modes {
            for item in value {
                __query.push(("modes", item.to_string()));
            }
        }
        self.get_list(&__path, &__query).await
    }
}

/// Optional query parameters for [`Client::stop_point_get_by_sms`].
#[derive(Debug, Clone, Default)]
pub struct StopPointGetBySmsOptions {
    /// If set to "web", a 302 redirect to relevant website bus stop page is returned. Valid values are : web. All other values are ignored.
    pub output: Option<String>,
}

impl Client {
    /// Gets a StopPoint for a given sms code.
    ///
    /// `GET /StopPoint/Sms/{id}`
    pub async fn stop_point_get_by_sms(
        &self,
        id: &str,
        options: &StopPointGetBySmsOptions,
    ) -> Result<serde_json::Value> {
        let __path = format!("/StopPoint/Sms/{id}", id = crate::segment(id));
        let mut __query: Vec<(&str, String)> = Vec::new();
        if let Some(value) = &options.output {
            __query.push(("output", value.to_string()));
        }
        self.get(&__path, &__query).await
    }
}

impl Client {
    /// Gets all stop points of a given type
    ///
    /// `GET /StopPoint/Type/{types}`
    pub async fn stop_point_get_by_type(
        &self,
        types: &[&str],
    ) -> Result<Vec<crate::generated::models::StopPoint>> {
        let __path = format!("/StopPoint/Type/{types}", types = crate::join(types));
        let mut __query: Vec<(&str, String)> = Vec::new();
        self.get_list(&__path, &__query).await
    }
}

impl Client {
    /// Gets all the stop points of given type(s) with a page number
    ///
    /// `GET /StopPoint/Type/{types}/page/{page}`
    pub async fn stop_point_get_by_type_with_pagination(
        &self,
        types: &[&str],
        page: i32,
    ) -> Result<Vec<crate::generated::models::StopPoint>> {
        let __path = format!(
            "/StopPoint/Type/{types}/page/{page}",
            types = crate::join(types),
            page = page
        );
        let mut __query: Vec<(&str, String)> = Vec::new();
        self.get_list(&__path, &__query).await
    }
}

/// Optional query parameters for [`Client::stop_point_get`].
#[derive(Debug, Clone, Default)]
pub struct StopPointGetOptions {
    /// Include the crowding data (static). To Filter further use: /StopPoint/{ids}/Crowding/{line}
    pub include_crowding_data: Option<bool>,
}

impl Client {
    /// Gets a list of StopPoints corresponding to the given list of stop ids.
    ///
    /// `GET /StopPoint/{ids}`
    pub async fn stop_point_get(
        &self,
        ids: &[&str],
        options: &StopPointGetOptions,
    ) -> Result<Vec<crate::generated::models::StopPoint>> {
        let __path = format!("/StopPoint/{ids}", ids = crate::join(ids));
        let mut __query: Vec<(&str, String)> = Vec::new();
        if let Some(value) = &options.include_crowding_data {
            __query.push(("includeCrowdingData", value.to_string()));
        }
        self.get_list(&__path, &__query).await
    }
}

/// Optional query parameters for [`Client::stop_point_disruption`].
#[derive(Debug, Clone, Default)]
pub struct StopPointDisruptionOptions {
    /// Specify true to return disruptions for entire family, or false to return disruptions for just this stop point. Defaults to false.
    pub get_family: Option<bool>,
    pub include_route_blocked_stops: Option<bool>,
    /// Specify true to associate all disruptions with parent stop point. (Only applicable when getFamily is true).
    pub flatten_response: Option<bool>,
}

impl Client {
    /// Gets all disruptions for the specified StopPointId, plus disruptions for any child Naptan records it may have.
    ///
    /// `GET /StopPoint/{ids}/Disruption`
    pub async fn stop_point_disruption(
        &self,
        ids: &[&str],
        options: &StopPointDisruptionOptions,
    ) -> Result<Vec<crate::generated::models::DisruptedPoint>> {
        let __path = format!("/StopPoint/{ids}/Disruption", ids = crate::join(ids));
        let mut __query: Vec<(&str, String)> = Vec::new();
        if let Some(value) = &options.get_family {
            __query.push(("getFamily", value.to_string()));
        }
        if let Some(value) = &options.include_route_blocked_stops {
            __query.push(("includeRouteBlockedStops", value.to_string()));
        }
        if let Some(value) = &options.flatten_response {
            __query.push(("flattenResponse", value.to_string()));
        }
        self.get_list(&__path, &__query).await
    }
}

impl Client {
    /// Gets the list of arrival and departure predictions for the given stop point id (overground, Elizabeth line and thameslink only)
    ///
    /// `GET /StopPoint/{id}/ArrivalDepartures`
    pub async fn stop_point_arrival_departures(
        &self,
        id: &str,
        line_ids: &[&str],
    ) -> Result<Vec<crate::generated::models::ArrivalDeparture>> {
        let __path = format!("/StopPoint/{id}/ArrivalDepartures", id = crate::segment(id));
        let mut __query: Vec<(&str, String)> = Vec::new();
        for item in line_ids {
            __query.push(("lineIds", item.to_string()));
        }
        self.get_list(&__path, &__query).await
    }
}

impl Client {
    /// Gets the list of arrival predictions for the given stop point id
    ///
    /// `GET /StopPoint/{id}/Arrivals`
    pub async fn stop_point_arrivals(
        &self,
        id: &str,
    ) -> Result<Vec<crate::generated::models::Prediction>> {
        let __path = format!("/StopPoint/{id}/Arrivals", id = crate::segment(id));
        let mut __query: Vec<(&str, String)> = Vec::new();
        self.get_list(&__path, &__query).await
    }
}

/// Optional query parameters for [`Client::stop_point_reachable_from`].
#[derive(Debug, Clone, Default)]
pub struct StopPointReachableFromOptions {
    /// A comma-separated list of service types to filter on. If not specified. Supported values: Regular, Night. Defaulted to 'Regular' if not specified
    pub service_types: Option<Vec<String>>,
}

impl Client {
    /// Gets Stopoints that are reachable from a station/line combination.
    ///
    /// `GET /StopPoint/{id}/CanReachOnLine/{lineId}`
    pub async fn stop_point_reachable_from(
        &self,
        id: &str,
        line_id: &str,
        options: &StopPointReachableFromOptions,
    ) -> Result<Vec<crate::generated::models::StopPoint>> {
        let __path = format!(
            "/StopPoint/{id}/CanReachOnLine/{line_id}",
            id = crate::segment(id),
            line_id = crate::segment(line_id)
        );
        let mut __query: Vec<(&str, String)> = Vec::new();
        if let Some(value) = &options.service_types {
            for item in value {
                __query.push(("serviceTypes", item.to_string()));
            }
        }
        self.get_list(&__path, &__query).await
    }
}

impl Client {
    /// Gets all the Crowding data (static) for the StopPointId, plus crowding data for a given line and optionally a particular direction.
    ///
    /// `GET /StopPoint/{id}/Crowding/{line}`
    pub async fn stop_point_crowding(
        &self,
        id: &str,
        line: &str,
        direction: &str,
    ) -> Result<Vec<crate::generated::models::StopPoint>> {
        let __path = format!(
            "/StopPoint/{id}/Crowding/{line}",
            id = crate::segment(id),
            line = crate::segment(line)
        );
        let mut __query: Vec<(&str, String)> = Vec::new();
        __query.push(("direction", direction.to_string()));
        self.get_list(&__path, &__query).await
    }
}

/// Optional query parameters for [`Client::stop_point_direction`].
#[derive(Debug, Clone, Default)]
pub struct StopPointDirectionOptions {
    /// Optional line id filter e.g. victoria
    pub line_id: Option<String>,
}

impl Client {
    /// Returns the canonical direction, "inbound" or "outbound", for a given pair of stop point Ids in the direction from -&gt; to.
    ///
    /// `GET /StopPoint/{id}/DirectionTo/{toStopPointId}`
    pub async fn stop_point_direction(
        &self,
        id: &str,
        to_stop_point_id: &str,
        options: &StopPointDirectionOptions,
    ) -> Result<String> {
        let __path = format!(
            "/StopPoint/{id}/DirectionTo/{to_stop_point_id}",
            id = crate::segment(id),
            to_stop_point_id = crate::segment(to_stop_point_id)
        );
        let mut __query: Vec<(&str, String)> = Vec::new();
        if let Some(value) = &options.line_id {
            __query.push(("lineId", value.to_string()));
        }
        self.get(&__path, &__query).await
    }
}

/// Optional query parameters for [`Client::stop_point_route`].
#[derive(Debug, Clone, Default)]
pub struct StopPointRouteOptions {
    /// A comma-separated list of service types to filter on. If not specified. Supported values: Regular, Night. Defaulted to 'Regular' if not specified
    pub service_types: Option<Vec<String>>,
}

impl Client {
    /// Returns the route sections for all the lines that service the given stop point ids
    ///
    /// `GET /StopPoint/{id}/Route`
    pub async fn stop_point_route(
        &self,
        id: &str,
        options: &StopPointRouteOptions,
    ) -> Result<Vec<crate::generated::models::StopPointRouteSection>> {
        let __path = format!("/StopPoint/{id}/Route", id = crate::segment(id));
        let mut __query: Vec<(&str, String)> = Vec::new();
        if let Some(value) = &options.service_types {
            for item in value {
                __query.push(("serviceTypes", item.to_string()));
            }
        }
        self.get_list(&__path, &__query).await
    }
}

impl Client {
    /// Get a list of places corresponding to a given id and place types.
    ///
    /// `GET /StopPoint/{id}/placeTypes`
    pub async fn stop_point_get_by_id(
        &self,
        id: &str,
        place_types: &[&str],
    ) -> Result<Vec<crate::generated::models::Place>> {
        let __path = format!("/StopPoint/{id}/placeTypes", id = crate::segment(id));
        let mut __query: Vec<(&str, String)> = Vec::new();
        for item in place_types {
            __query.push(("placeTypes", item.to_string()));
        }
        self.get_list(&__path, &__query).await
    }
}

impl Client {
    /// Get car parks corresponding to the given stop point id.
    ///
    /// `GET /StopPoint/{stopPointId}/CarParks`
    pub async fn stop_point_get_car_parks_by_id(
        &self,
        stop_point_id: &str,
    ) -> Result<Vec<crate::generated::models::Place>> {
        let __path = format!(
            "/StopPoint/{stop_point_id}/CarParks",
            stop_point_id = crate::segment(stop_point_id)
        );
        let mut __query: Vec<(&str, String)> = Vec::new();
        self.get_list(&__path, &__query).await
    }
}

impl Client {
    /// Gets a list of taxi ranks corresponding to the given stop point id.
    ///
    /// `GET /StopPoint/{stopPointId}/TaxiRanks`
    pub async fn stop_point_get_taxi_ranks_by_ids(
        &self,
        stop_point_id: &str,
    ) -> Result<Vec<crate::generated::models::Place>> {
        let __path = format!(
            "/StopPoint/{stop_point_id}/TaxiRanks",
            stop_point_id = crate::segment(stop_point_id)
        );
        let mut __query: Vec<(&str, String)> = Vec::new();
        self.get_list(&__path, &__query).await
    }
}

impl Client {
    /// Gets the TravelTime overlay.
    ///
    /// `GET /TravelTimes/compareOverlay/{z}/mapcenter/{mapCenterLat}/{mapCenterLon}/pinlocation/{pinLat}/{pinLon}/dimensions/{width}/{height}`
    pub async fn travel_time_get_compare_overlay(
        &self,
        z: i32,
        map_center_lat: f64,
        map_center_lon: f64,
        pin_lat: f64,
        pin_lon: f64,
        width: i32,
        height: i32,
        scenario_title: &str,
        time_of_day_id: &str,
        mode_id: &str,
        direction: &str,
        travel_time_interval: i32,
        compare_type: &str,
        compare_value: &str,
    ) -> Result<serde_json::Value> {
        let __path = format!(
            "/TravelTimes/compareOverlay/{z}/mapcenter/{map_center_lat}/{map_center_lon}/pinlocation/{pin_lat}/{pin_lon}/dimensions/{width}/{height}",
            z = z,
            map_center_lat = map_center_lat,
            map_center_lon = map_center_lon,
            pin_lat = pin_lat,
            pin_lon = pin_lon,
            width = width,
            height = height
        );
        let mut __query: Vec<(&str, String)> = Vec::new();
        __query.push(("scenarioTitle", scenario_title.to_string()));
        __query.push(("timeOfDayId", time_of_day_id.to_string()));
        __query.push(("modeId", mode_id.to_string()));
        __query.push(("direction", direction.to_string()));
        __query.push(("travelTimeInterval", travel_time_interval.to_string()));
        __query.push(("compareType", compare_type.to_string()));
        __query.push(("compareValue", compare_value.to_string()));
        self.get(&__path, &__query).await
    }
}

impl Client {
    /// Gets the TravelTime overlay.
    ///
    /// `GET /TravelTimes/overlay/{z}/mapcenter/{mapCenterLat}/{mapCenterLon}/pinlocation/{pinLat}/{pinLon}/dimensions/{width}/{height}`
    pub async fn travel_time_get_overlay(
        &self,
        z: i32,
        map_center_lat: f64,
        map_center_lon: f64,
        pin_lat: f64,
        pin_lon: f64,
        width: i32,
        height: i32,
        scenario_title: &str,
        time_of_day_id: &str,
        mode_id: &str,
        direction: &str,
        travel_time_interval: i32,
    ) -> Result<serde_json::Value> {
        let __path = format!(
            "/TravelTimes/overlay/{z}/mapcenter/{map_center_lat}/{map_center_lon}/pinlocation/{pin_lat}/{pin_lon}/dimensions/{width}/{height}",
            z = z,
            map_center_lat = map_center_lat,
            map_center_lon = map_center_lon,
            pin_lat = pin_lat,
            pin_lon = pin_lon,
            width = width,
            height = height
        );
        let mut __query: Vec<(&str, String)> = Vec::new();
        __query.push(("scenarioTitle", scenario_title.to_string()));
        __query.push(("timeOfDayId", time_of_day_id.to_string()));
        __query.push(("modeId", mode_id.to_string()));
        __query.push(("direction", direction.to_string()));
        __query.push(("travelTimeInterval", travel_time_interval.to_string()));
        self.get(&__path, &__query).await
    }
}

impl Client {
    /// Gets the predictions for a given list of vehicle Id's.
    ///
    /// `GET /Vehicle/{ids}/Arrivals`
    pub async fn vehicle_get(
        &self,
        ids: &[&str],
    ) -> Result<Vec<crate::generated::models::Prediction>> {
        let __path = format!("/Vehicle/{ids}/Arrivals", ids = crate::join(ids));
        let mut __query: Vec<(&str, String)> = Vec::new();
        self.get_list(&__path, &__query).await
    }
}
