//! Response types, generated from TfL's Swagger document. Do not edit.
//!
//! Every field is `Option` because the spec marks almost nothing as required
//! and TfL means it: a live `/StopPoint` response carries about half of the
//! fields declared here. Absence is normal, not an error.

use serde::{Deserialize, Serialize};

/// `System.Data.Spatial.DbGeography`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct DbGeography {
    pub geography: Option<crate::generated::models::DbGeographyWellKnownValue>,
}

/// `System.Data.Spatial.DbGeographyWellKnownValue`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct DbGeographyWellKnownValue {
    #[serde(rename = "coordinateSystemId")]
    pub coordinate_system_id: Option<i32>,
    #[serde(rename = "wellKnownBinary")]
    pub well_known_binary: Option<String>,
    #[serde(rename = "wellKnownText")]
    pub well_known_text: Option<String>,
}

/// `System.Object`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Object {}

/// `Tfl.Api.Common.ApiVersionInfo`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ApiVersionInfo {
    pub assemblies: Option<Vec<String>>,
    pub label: Option<String>,
    pub timestamp: Option<String>,
    pub version: Option<String>,
}

/// `Tfl.Api.Common.DateRange`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct DateRange {
    #[serde(rename = "endDate")]
    pub end_date: Option<String>,
    #[serde(rename = "startDate")]
    pub start_date: Option<String>,
}

/// `Tfl.Api.Common.DateRangeNullable`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct DateRangeNullable {
    #[serde(rename = "endDate")]
    pub end_date: Option<String>,
    #[serde(rename = "startDate")]
    pub start_date: Option<String>,
}

/// `Tfl.Api.Common.GeoPoint`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct GeoPoint {
    pub lat: Option<f64>,
    pub lon: Option<f64>,
}

/// `Tfl.Api.Common.JourneyPlanner.JpElevation`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct JpElevation {
    pub distance: Option<i32>,
    #[serde(rename = "endLat")]
    pub end_lat: Option<f64>,
    #[serde(rename = "endLon")]
    pub end_lon: Option<f64>,
    pub gradient: Option<f64>,
    #[serde(rename = "heightFromPreviousPoint")]
    pub height_from_previous_point: Option<i32>,
    #[serde(rename = "startElevation")]
    pub start_elevation: Option<i32>,
    #[serde(rename = "startLat")]
    pub start_lat: Option<f64>,
    #[serde(rename = "startLon")]
    pub start_lon: Option<f64>,
}

/// `Tfl.Api.Common.PlaceGeo`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct PlaceGeo {
    pub lat: Option<f64>,
    pub lon: Option<f64>,
    #[serde(rename = "neLat")]
    pub ne_lat: Option<f64>,
    #[serde(rename = "neLon")]
    pub ne_lon: Option<f64>,
    #[serde(rename = "swLat")]
    pub sw_lat: Option<f64>,
    #[serde(rename = "swLon")]
    pub sw_lon: Option<f64>,
}

/// `Tfl.Api.Common.PostcodeInput`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct PostcodeInput {
    pub postcode: Option<String>,
}

/// `Tfl.Api.Presentation.Entities.AccidentStats.AccidentDetail`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct AccidentDetail {
    pub borough: Option<String>,
    pub casualties: Option<Vec<crate::generated::models::Casualty>>,
    pub date: Option<String>,
    pub id: Option<i32>,
    pub lat: Option<f64>,
    pub location: Option<String>,
    pub lon: Option<f64>,
    pub severity: Option<String>,
    pub vehicles: Option<Vec<crate::generated::models::Vehicle>>,
}

/// `Tfl.Api.Presentation.Entities.AccidentStats.AccidentStatsOrderedSummary`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct AccidentStatsOrderedSummary {
    pub accidents: Option<i32>,
    pub borough: Option<String>,
    pub year: Option<i32>,
}

/// `Tfl.Api.Presentation.Entities.AccidentStats.Casualty`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Casualty {
    pub age: Option<i32>,
    #[serde(rename = "ageBand")]
    pub age_band: Option<String>,
    pub class: Option<String>,
    pub mode: Option<String>,
    pub severity: Option<String>,
}

/// `Tfl.Api.Presentation.Entities.AccidentStats.Vehicle`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Vehicle {
    pub r#type: Option<String>,
}

/// `Tfl.Api.Presentation.Entities.ActiveServiceType`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ActiveServiceType {
    pub mode: Option<String>,
    #[serde(rename = "serviceType")]
    pub service_type: Option<String>,
}

/// `Tfl.Api.Presentation.Entities.AdditionalProperties`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct AdditionalProperties {
    pub category: Option<String>,
    pub key: Option<String>,
    pub modified: Option<String>,
    #[serde(rename = "sourceSystemKey")]
    pub source_system_key: Option<String>,
    pub value: Option<String>,
}

/// `Tfl.Api.Presentation.Entities.ArrivalDeparture`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ArrivalDeparture {
    /// Reason for cancellation or delay
    pub cause: Option<String>,
    /// Status of departure
    #[serde(rename = "departureStatus")]
    pub departure_status: Option<String>,
    /// Name of the destination
    #[serde(rename = "destinationName")]
    pub destination_name: Option<String>,
    /// Naptan Identifier for the prediction's destination
    #[serde(rename = "destinationNaptanId")]
    pub destination_naptan_id: Option<String>,
    /// Estimated time of arrival
    #[serde(rename = "estimatedTimeOfArrival")]
    pub estimated_time_of_arrival: Option<String>,
    /// Estimated time of arrival
    #[serde(rename = "estimatedTimeOfDeparture")]
    pub estimated_time_of_departure: Option<String>,
    /// Estimated time of arrival
    #[serde(rename = "minutesAndSecondsToArrival")]
    pub minutes_and_seconds_to_arrival: Option<String>,
    /// Estimated time of arrival
    #[serde(rename = "minutesAndSecondsToDeparture")]
    pub minutes_and_seconds_to_departure: Option<String>,
    /// Identifier for the prediction
    #[serde(rename = "naptanId")]
    pub naptan_id: Option<String>,
    /// Platform name (for bus, this is the stop letter)
    #[serde(rename = "platformName")]
    pub platform_name: Option<String>,
    /// Estimated time of arrival
    #[serde(rename = "scheduledTimeOfArrival")]
    pub scheduled_time_of_arrival: Option<String>,
    /// Estimated time of arrival
    #[serde(rename = "scheduledTimeOfDeparture")]
    pub scheduled_time_of_departure: Option<String>,
    /// Station name
    #[serde(rename = "stationName")]
    pub station_name: Option<String>,
    /// Keep the original timestamp from MongoDb fo debugging purposes
    pub timing: Option<crate::generated::models::PredictionTiming>,
}

/// `Tfl.Api.Presentation.Entities.ArrivalDepartureWithLine`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ArrivalDepartureWithLine {
    /// Reason for cancellation or delay
    pub cause: Option<String>,
    /// Status of departure
    #[serde(rename = "departureStatus")]
    pub departure_status: Option<String>,
    /// Name of the destination
    #[serde(rename = "destinationName")]
    pub destination_name: Option<String>,
    /// Naptan Identifier for the prediction's destination
    #[serde(rename = "destinationNaptanId")]
    pub destination_naptan_id: Option<String>,
    /// Estimated time of arrival
    #[serde(rename = "estimatedTimeOfArrival")]
    pub estimated_time_of_arrival: Option<String>,
    /// Estimated time of arrival
    #[serde(rename = "estimatedTimeOfDeparture")]
    pub estimated_time_of_departure: Option<String>,
    /// Train operating company LineId
    #[serde(rename = "lineId")]
    pub line_id: Option<String>,
    /// Train operating company LineName
    #[serde(rename = "lineName")]
    pub line_name: Option<String>,
    /// Estimated time of arrival
    #[serde(rename = "minutesAndSecondsToArrival")]
    pub minutes_and_seconds_to_arrival: Option<String>,
    /// Estimated time of arrival
    #[serde(rename = "minutesAndSecondsToDeparture")]
    pub minutes_and_seconds_to_departure: Option<String>,
    /// Identifier for the prediction
    #[serde(rename = "naptanId")]
    pub naptan_id: Option<String>,
    /// Platform name (for bus, this is the stop letter)
    #[serde(rename = "platformName")]
    pub platform_name: Option<String>,
    /// Estimated time of arrival
    #[serde(rename = "scheduledTimeOfArrival")]
    pub scheduled_time_of_arrival: Option<String>,
    /// Estimated time of arrival
    #[serde(rename = "scheduledTimeOfDeparture")]
    pub scheduled_time_of_departure: Option<String>,
    /// Station name
    #[serde(rename = "stationName")]
    pub station_name: Option<String>,
    /// Keep the original timestamp from MongoDb fo debugging purposes
    pub timing: Option<crate::generated::models::PredictionTiming>,
    /// Train operating company VehicleId
    #[serde(rename = "vehicleId")]
    pub vehicle_id: Option<String>,
}

/// `Tfl.Api.Presentation.Entities.Bay`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Bay {
    #[serde(rename = "bayCount")]
    pub bay_count: Option<i32>,
    #[serde(rename = "bayType")]
    pub bay_type: Option<String>,
    pub free: Option<i32>,
    pub occupied: Option<i32>,
}

/// `Tfl.Api.Presentation.Entities.BikePointOccupancy`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct BikePointOccupancy {
    /// Total bike counts
    #[serde(rename = "bikesCount")]
    pub bikes_count: Option<i32>,
    /// Total ebikes count
    #[serde(rename = "eBikesCount")]
    pub e_bikes_count: Option<i32>,
    /// Empty docks
    #[serde(rename = "emptyDocks")]
    pub empty_docks: Option<i32>,
    /// Id of the bike point such as BikePoints_1
    pub id: Option<String>,
    /// Name / Common name of the bike point
    pub name: Option<String>,
    /// Total standard bikes count
    #[serde(rename = "standardBikesCount")]
    pub standard_bikes_count: Option<i32>,
    /// Total docks available
    #[serde(rename = "totalDocks")]
    pub total_docks: Option<i32>,
}

/// `Tfl.Api.Presentation.Entities.CarParkOccupancy`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct CarParkOccupancy {
    pub bays: Option<Vec<crate::generated::models::Bay>>,
    #[serde(rename = "carParkDetailsUrl")]
    pub car_park_details_url: Option<String>,
    pub id: Option<String>,
    pub name: Option<String>,
}

/// `Tfl.Api.Presentation.Entities.ChargeConnectorOccupancy`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ChargeConnectorOccupancy {
    pub id: Option<i32>,
    #[serde(rename = "sourceSystemPlaceId")]
    pub source_system_place_id: Option<String>,
    pub status: Option<String>,
}

/// `Tfl.Api.Presentation.Entities.ConcernedLine`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ConcernedLine {
    /// The direction of travel affected (e.g., "Inbound", "Outbound")
    pub direction: Option<String>,
    /// The ID of the concerned line (e.g., "piccadilly", "district")
    pub id: Option<String>,
}

/// `Tfl.Api.Presentation.Entities.Coordinate`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Coordinate {
    pub easting: Option<f64>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub northing: Option<f64>,
    #[serde(rename = "xCoord")]
    pub x_coord: Option<i32>,
    #[serde(rename = "yCoord")]
    pub y_coord: Option<i32>,
}

/// `Tfl.Api.Presentation.Entities.Crowding`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Crowding {
    /// Busiest times at a station (static information)
    #[serde(rename = "passengerFlows")]
    pub passenger_flows: Option<Vec<crate::generated::models::PassengerFlow>>,
    /// Train Loading on a scale 1-6, 1 being "Very quiet" and 6 being "Exceptionally busy" (static information)
    #[serde(rename = "trainLoadings")]
    pub train_loadings: Option<Vec<crate::generated::models::TrainLoading>>,
}

/// `Tfl.Api.Presentation.Entities.CycleSuperhighway`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct CycleSuperhighway {
    /// A LineString or MultiLineString that forms the route of the highway
    pub geography: Option<crate::generated::models::DbGeography>,
    /// The Id
    pub id: Option<String>,
    /// The long label to show on maps when zoomed in
    pub label: Option<String>,
    /// The short label to show on maps
    #[serde(rename = "labelShort")]
    pub label_short: Option<String>,
    /// When the data was last updated
    pub modified: Option<String>,
    /// Type of cycle route e.g CycleSuperhighways, Quietways, MiniHollands etc
    #[serde(rename = "routeType")]
    pub route_type: Option<String>,
    /// True if the route is split into segments
    #[serde(default, deserialize_with = "crate::de::bool_or_string")]
    pub segmented: Option<bool>,
    /// Cycle route status i.e Proposed, Existing etc
    pub status: Option<String>,
}

/// `Tfl.Api.Presentation.Entities.DisruptedPoint`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct DisruptedPoint {
    #[serde(rename = "additionalInformation")]
    pub additional_information: Option<String>,
    pub appearance: Option<String>,
    #[serde(rename = "atcoCode")]
    pub atco_code: Option<String>,
    #[serde(rename = "closureText")]
    pub closure_text: Option<String>,
    #[serde(rename = "commonName")]
    pub common_name: Option<String>,
    #[serde(rename = "concernedLines")]
    pub concerned_lines: Option<Vec<crate::generated::models::ConcernedLine>>,
    pub description: Option<String>,
    #[serde(rename = "fromDate")]
    pub from_date: Option<String>,
    pub mode: Option<String>,
    #[serde(rename = "stationAtcoCode")]
    pub station_atco_code: Option<String>,
    #[serde(rename = "toDate")]
    pub to_date: Option<String>,
    pub r#type: Option<String>,
}

/// `Tfl.Api.Presentation.Entities.DisruptedRoute`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct DisruptedRoute {
    /// The name of the Destination StopPoint
    #[serde(rename = "destinationName")]
    pub destination_name: Option<String>,
    /// Inbound or Outbound
    pub direction: Option<String>,
    /// The Id of the route
    pub id: Option<String>,
    /// Whether this represents the entire route section
    #[serde(rename = "isEntireRouteSection")]
    #[serde(default, deserialize_with = "crate::de::bool_or_string")]
    pub is_entire_route_section: Option<bool>,
    /// The Id of the Line
    #[serde(rename = "lineId")]
    pub line_id: Option<String>,
    /// The co-ordinates of the route's path as a geoJSON lineString
    #[serde(rename = "lineString")]
    pub line_string: Option<String>,
    /// Name such as "72"
    pub name: Option<String>,
    /// The name of the Origin StopPoint
    #[serde(rename = "originationName")]
    pub origination_name: Option<String>,
    /// The route code
    #[serde(rename = "routeCode")]
    pub route_code: Option<String>,
    #[serde(rename = "routeSectionNaptanEntrySequence")]
    pub route_section_naptan_entry_sequence:
        Option<Vec<crate::generated::models::RouteSectionNaptanEntrySequence>>,
    /// The DateTime that the Service containing this Route is valid from.
    #[serde(rename = "validFrom")]
    pub valid_from: Option<String>,
    /// The DateTime that the Service containing this Route is valid until.
    #[serde(rename = "validTo")]
    pub valid_to: Option<String>,
    /// (where applicable) via Charing Cross / Bank / King's Cross / Embankment / Newbury Park / Woodford
    pub via: Option<crate::generated::models::RouteSectionNaptanEntrySequence>,
}

/// `Tfl.Api.Presentation.Entities.Disruption`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Disruption {
    /// Gets or sets the additionaInfo of this disruption.
    #[serde(rename = "additionalInfo")]
    pub additional_info: Option<String>,
    /// Gets or sets the routes affected by this disruption
    #[serde(rename = "affectedRoutes")]
    pub affected_routes: Option<Vec<crate::generated::models::DisruptedRoute>>,
    /// Gets or sets the stops affected by this disruption
    #[serde(rename = "affectedStops")]
    pub affected_stops: Option<Vec<crate::generated::models::StopPoint>>,
    /// Gets or sets the category of this dispruption.
    pub category: Option<String>,
    /// Gets or sets the description of the category.
    #[serde(rename = "categoryDescription")]
    pub category_description: Option<String>,
    /// Text describing the closure type
    #[serde(rename = "closureText")]
    pub closure_text: Option<String>,
    /// Gets or sets the date/time when this disruption was created.
    pub created: Option<String>,
    /// Gets or sets the description of this disruption.
    pub description: Option<String>,
    /// Gets or sets the date/time when this disruption was last updated.
    #[serde(rename = "lastUpdate")]
    pub last_update: Option<String>,
    /// Gets or sets the summary of this disruption.
    pub summary: Option<String>,
    /// Gets or sets the disruption type of this dispruption.
    pub r#type: Option<String>,
}

/// `Tfl.Api.Presentation.Entities.Fares.Fare`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct FaresFare {
    pub cap: Option<f64>,
    pub cost: Option<String>,
    pub description: Option<String>,
    pub id: Option<i32>,
    pub mode: Option<String>,
    #[serde(rename = "passengerType")]
    pub passenger_type: Option<String>,
    #[serde(rename = "ticketTime")]
    pub ticket_time: Option<String>,
    #[serde(rename = "ticketType")]
    pub ticket_type: Option<String>,
    #[serde(rename = "validFrom")]
    pub valid_from: Option<String>,
    #[serde(rename = "validUntil")]
    pub valid_until: Option<String>,
    pub zone: Option<String>,
}

/// `Tfl.Api.Presentation.Entities.Fares.FareBounds`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct FareBounds {
    pub description: Option<String>,
    #[serde(rename = "displayName")]
    pub display_name: Option<String>,
    #[serde(rename = "displayOrder")]
    pub display_order: Option<i32>,
    pub from: Option<String>,
    pub id: Option<i32>,
    #[serde(rename = "isPopularFare")]
    #[serde(default, deserialize_with = "crate::de::bool_or_string")]
    pub is_popular_fare: Option<bool>,
    #[serde(rename = "isPopularTravelCard")]
    #[serde(default, deserialize_with = "crate::de::bool_or_string")]
    pub is_popular_travel_card: Option<bool>,
    #[serde(rename = "isTour")]
    #[serde(default, deserialize_with = "crate::de::bool_or_string")]
    pub is_tour: Option<bool>,
    pub messages: Option<Vec<crate::generated::models::Message>>,
    pub operator: Option<String>,
    #[serde(rename = "routeCode")]
    pub route_code: Option<String>,
    pub to: Option<String>,
    pub via: Option<String>,
}

/// `Tfl.Api.Presentation.Entities.Fares.FareDetails`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct FareDetails {
    #[serde(rename = "boundsId")]
    pub bounds_id: Option<i32>,
    #[serde(rename = "contactlessPAYGOnlyFare")]
    #[serde(default, deserialize_with = "crate::de::bool_or_string")]
    pub contactless_p_a_y_g_only_fare: Option<bool>,
    #[serde(rename = "displayName")]
    pub display_name: Option<String>,
    #[serde(rename = "displayOrder")]
    pub display_order: Option<i32>,
    #[serde(rename = "endDate")]
    pub end_date: Option<String>,
    pub from: Option<String>,
    #[serde(rename = "fromStation")]
    pub from_station: Option<String>,
    #[serde(rename = "isTour")]
    #[serde(default, deserialize_with = "crate::de::bool_or_string")]
    pub is_tour: Option<bool>,
    pub messages: Option<Vec<crate::generated::models::Message>>,
    pub mode: Option<String>,
    pub operator: Option<String>,
    #[serde(rename = "passengerType")]
    pub passenger_type: Option<String>,
    #[serde(rename = "routeCode")]
    pub route_code: Option<String>,
    #[serde(rename = "routeDescription")]
    pub route_description: Option<String>,
    #[serde(rename = "specialFare")]
    #[serde(default, deserialize_with = "crate::de::bool_or_string")]
    pub special_fare: Option<bool>,
    #[serde(rename = "startDate")]
    pub start_date: Option<String>,
    #[serde(rename = "throughFare")]
    #[serde(default, deserialize_with = "crate::de::bool_or_string")]
    pub through_fare: Option<bool>,
    #[serde(rename = "ticketsAvailable")]
    pub tickets_available: Option<Vec<crate::generated::models::Ticket>>,
    pub to: Option<String>,
    #[serde(rename = "toStation")]
    pub to_station: Option<String>,
    #[serde(rename = "validatorInformation")]
    pub validator_information: Option<String>,
    pub via: Option<String>,
}

/// `Tfl.Api.Presentation.Entities.Fares.FareStation`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct FareStation {
    #[serde(rename = "atcoCode")]
    pub atco_code: Option<String>,
    #[serde(rename = "commonName")]
    pub common_name: Option<String>,
    #[serde(rename = "fareCategory")]
    pub fare_category: Option<String>,
}

/// `Tfl.Api.Presentation.Entities.Fares.FaresMode`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct FaresMode {
    pub description: Option<String>,
    pub id: Option<i32>,
    pub name: Option<String>,
}

/// `Tfl.Api.Presentation.Entities.Fares.FaresPeriod`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct FaresPeriod {
    #[serde(rename = "endDate")]
    pub end_date: Option<String>,
    pub id: Option<i32>,
    #[serde(rename = "isFuture")]
    #[serde(default, deserialize_with = "crate::de::bool_or_string")]
    pub is_future: Option<bool>,
    #[serde(rename = "startDate")]
    pub start_date: Option<String>,
    #[serde(rename = "viewableDate")]
    pub viewable_date: Option<String>,
}

/// `Tfl.Api.Presentation.Entities.Fares.FaresSection`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct FaresSection {
    pub header: Option<String>,
    pub index: Option<i32>,
    pub journey: Option<crate::generated::models::FaresJourney>,
    pub messages: Option<Vec<crate::generated::models::Message>>,
    pub rows: Option<Vec<crate::generated::models::FareDetails>>,
}

/// `Tfl.Api.Presentation.Entities.Fares.Journey`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct FaresJourney {
    #[serde(rename = "fromStation")]
    pub from_station: Option<crate::generated::models::FareStation>,
    #[serde(rename = "toStation")]
    pub to_station: Option<crate::generated::models::FareStation>,
}

/// `Tfl.Api.Presentation.Entities.Fares.PassengerType`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct PassengerType {
    pub description: Option<String>,
    #[serde(rename = "displayName")]
    pub display_name: Option<String>,
    #[serde(rename = "displayOrder")]
    pub display_order: Option<i32>,
    pub r#type: Option<String>,
}

/// `Tfl.Api.Presentation.Entities.Fares.Recommendation`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Recommendation {
    pub cost: Option<String>,
    #[serde(rename = "discountCard")]
    pub discount_card: Option<String>,
    #[serde(rename = "fareType")]
    pub fare_type: Option<String>,
    #[serde(rename = "gettingYourTicket")]
    pub getting_your_ticket: Option<Vec<crate::generated::models::Message>>,
    pub id: Option<i32>,
    #[serde(rename = "keyFeatures")]
    pub key_features: Option<Vec<crate::generated::models::Message>>,
    pub notes: Option<Vec<crate::generated::models::Message>>,
    #[serde(rename = "priceComparison")]
    pub price_comparison: Option<String>,
    #[serde(rename = "priceDescription")]
    pub price_description: Option<String>,
    pub product: Option<String>,
    #[serde(rename = "productType")]
    pub product_type: Option<String>,
    pub rank: Option<i32>,
    #[serde(rename = "recommendedTopUp")]
    pub recommended_top_up: Option<String>,
    pub rule: Option<i32>,
    #[serde(rename = "singleFare")]
    pub single_fare: Option<f64>,
    #[serde(rename = "ticketTime")]
    pub ticket_time: Option<String>,
    #[serde(rename = "ticketType")]
    pub ticket_type: Option<String>,
    pub zones: Option<String>,
}

/// `Tfl.Api.Presentation.Entities.Fares.RecommendationResponse`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct RecommendationResponse {
    pub recommendations: Option<Vec<crate::generated::models::Recommendation>>,
}

/// `Tfl.Api.Presentation.Entities.Fares.Ticket`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Ticket {
    pub cost: Option<String>,
    pub description: Option<String>,
    #[serde(rename = "displayOrder")]
    pub display_order: Option<i32>,
    pub messages: Option<Vec<crate::generated::models::Message>>,
    pub mode: Option<String>,
    #[serde(rename = "passengerType")]
    pub passenger_type: Option<String>,
    #[serde(rename = "ticketTime")]
    pub ticket_time: Option<crate::generated::models::TicketTime>,
    #[serde(rename = "ticketType")]
    pub ticket_type: Option<crate::generated::models::TicketType>,
}

/// `Tfl.Api.Presentation.Entities.Fares.TicketTime`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct TicketTime {
    pub description: Option<String>,
    pub r#type: Option<String>,
}

/// `Tfl.Api.Presentation.Entities.Fares.TicketType`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct TicketType {
    pub description: Option<String>,
    pub r#type: Option<String>,
}

/// `Tfl.Api.Presentation.Entities.GeoCodeSearchMatch`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct GeoCodeSearchMatch {
    /// A string describing the formatted address of the place. Adds additional context to the place's Name.
    pub address: Option<String>,
    pub id: Option<String>,
    pub lat: Option<f64>,
    pub lon: Option<f64>,
    pub name: Option<String>,
    /// The type of the place e.g. "street_address"
    pub types: Option<Vec<String>>,
    pub url: Option<String>,
}

/// `Tfl.Api.Presentation.Entities.Identifier`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Identifier {
    pub crowding: Option<crate::generated::models::Crowding>,
    #[serde(rename = "fullName")]
    pub full_name: Option<String>,
    pub id: Option<String>,
    #[serde(rename = "motType")]
    pub mot_type: Option<String>,
    pub name: Option<String>,
    pub network: Option<String>,
    #[serde(rename = "routeType")]
    pub route_type: Option<String>,
    pub status: Option<String>,
    pub r#type: Option<String>,
    pub uri: Option<String>,
}

/// `Tfl.Api.Presentation.Entities.Instruction`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Instruction {
    pub detailed: Option<String>,
    pub steps: Option<Vec<crate::generated::models::InstructionStep>>,
    pub summary: Option<String>,
}

/// `Tfl.Api.Presentation.Entities.InstructionStep`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct InstructionStep {
    #[serde(rename = "atcoCode")]
    pub atco_code: Option<String>,
    #[serde(rename = "cumulativeDistance")]
    pub cumulative_distance: Option<i32>,
    #[serde(rename = "cumulativeTravelTime")]
    pub cumulative_travel_time: Option<i32>,
    pub description: Option<String>,
    #[serde(rename = "descriptionHeading")]
    pub description_heading: Option<String>,
    pub distance: Option<i32>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    #[serde(rename = "pathAttribute")]
    pub path_attribute: Option<crate::generated::models::PathAttribute>,
    #[serde(rename = "skyDirection")]
    pub sky_direction: Option<i32>,
    #[serde(rename = "skyDirectionDescription")]
    pub sky_direction_description: Option<String>,
    #[serde(rename = "streetName")]
    pub street_name: Option<String>,
    #[serde(rename = "trackType")]
    pub track_type: Option<String>,
    #[serde(rename = "travelTime")]
    pub travel_time: Option<i32>,
    #[serde(rename = "turnDirection")]
    pub turn_direction: Option<String>,
}

/// `Tfl.Api.Presentation.Entities.Interval`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Interval {
    #[serde(rename = "stopId")]
    pub stop_id: Option<String>,
    #[serde(rename = "timeToArrival")]
    pub time_to_arrival: Option<f64>,
}

/// `Tfl.Api.Presentation.Entities.JourneyPlanner.Fare`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct JourneyPlannerFare {
    #[serde(rename = "chargeLevel")]
    pub charge_level: Option<String>,
    #[serde(rename = "chargeProfileName")]
    pub charge_profile_name: Option<String>,
    pub cost: Option<i32>,
    #[serde(rename = "highZone")]
    pub high_zone: Option<i32>,
    #[serde(rename = "isHopperFare")]
    #[serde(default, deserialize_with = "crate::de::bool_or_string")]
    pub is_hopper_fare: Option<bool>,
    #[serde(rename = "lowZone")]
    pub low_zone: Option<i32>,
    #[serde(rename = "offPeak")]
    pub off_peak: Option<i32>,
    pub peak: Option<i32>,
    pub taps: Option<Vec<crate::generated::models::FareTap>>,
}

/// `Tfl.Api.Presentation.Entities.JourneyPlanner.FareCaveat`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct FareCaveat {
    pub text: Option<String>,
    pub r#type: Option<String>,
}

/// `Tfl.Api.Presentation.Entities.JourneyPlanner.FareTap`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct FareTap {
    #[serde(rename = "atcoCode")]
    pub atco_code: Option<String>,
    #[serde(rename = "tapDetails")]
    pub tap_details: Option<crate::generated::models::FareTapDetails>,
}

/// `Tfl.Api.Presentation.Entities.JourneyPlanner.FareTapDetails`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct FareTapDetails {
    #[serde(rename = "busRouteId")]
    pub bus_route_id: Option<String>,
    #[serde(rename = "hostDeviceType")]
    pub host_device_type: Option<String>,
    #[serde(rename = "modeType")]
    pub mode_type: Option<String>,
    #[serde(rename = "nationalLocationCode")]
    pub national_location_code: Option<i32>,
    #[serde(rename = "tapTimestamp")]
    pub tap_timestamp: Option<String>,
    #[serde(rename = "validationType")]
    pub validation_type: Option<String>,
}

/// `Tfl.Api.Presentation.Entities.JourneyPlanner.ItineraryResult`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ItineraryResult {
    #[serde(rename = "cycleHireDockingStationData")]
    pub cycle_hire_docking_station_data:
        Option<crate::generated::models::JourneyPlannerCycleHireDockingStationData>,
    #[serde(rename = "journeyVector")]
    pub journey_vector: Option<crate::generated::models::JourneyVector>,
    pub journeys: Option<Vec<crate::generated::models::JourneyPlannerJourney>>,
    pub lines: Option<Vec<crate::generated::models::Line>>,
    #[serde(rename = "recommendedMaxAgeMinutes")]
    pub recommended_max_age_minutes: Option<i32>,
    #[serde(rename = "searchCriteria")]
    pub search_criteria: Option<crate::generated::models::SearchCriteria>,
    #[serde(rename = "stopMessages")]
    pub stop_messages: Option<Vec<String>>,
}

/// `Tfl.Api.Presentation.Entities.JourneyPlanner.Journey`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct JourneyPlannerJourney {
    #[serde(rename = "alternativeRoute")]
    #[serde(default, deserialize_with = "crate::de::bool_or_string")]
    pub alternative_route: Option<bool>,
    #[serde(rename = "arrivalDateTime")]
    pub arrival_date_time: Option<String>,
    pub description: Option<String>,
    pub duration: Option<i32>,
    pub fare: Option<crate::generated::models::JourneyFare>,
    pub legs: Option<Vec<crate::generated::models::Leg>>,
    #[serde(rename = "startDateTime")]
    pub start_date_time: Option<String>,
}

/// `Tfl.Api.Presentation.Entities.JourneyPlanner.JourneyFare`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct JourneyFare {
    pub caveats: Option<Vec<crate::generated::models::FareCaveat>>,
    pub fares: Option<Vec<crate::generated::models::JourneyPlannerFare>>,
    #[serde(rename = "totalCost")]
    pub total_cost: Option<i32>,
}

/// `Tfl.Api.Presentation.Entities.JourneyPlanner.JourneyPlannerCycleHireDockingStationData`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct JourneyPlannerCycleHireDockingStationData {
    #[serde(rename = "destinationId")]
    pub destination_id: Option<String>,
    #[serde(rename = "destinationNumberOfBikes")]
    pub destination_number_of_bikes: Option<i32>,
    #[serde(rename = "destinationNumberOfEmptySlots")]
    pub destination_number_of_empty_slots: Option<i32>,
    #[serde(rename = "originId")]
    pub origin_id: Option<String>,
    #[serde(rename = "originNumberOfBikes")]
    pub origin_number_of_bikes: Option<i32>,
    #[serde(rename = "originNumberOfEmptySlots")]
    pub origin_number_of_empty_slots: Option<i32>,
}

/// `Tfl.Api.Presentation.Entities.JourneyPlanner.JourneyVector`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct JourneyVector {
    pub from: Option<String>,
    pub to: Option<String>,
    pub uri: Option<String>,
    pub via: Option<String>,
}

/// `Tfl.Api.Presentation.Entities.JourneyPlanner.Leg`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Leg {
    #[serde(rename = "arrivalPoint")]
    pub arrival_point: Option<crate::generated::models::Point>,
    #[serde(rename = "arrivalTime")]
    pub arrival_time: Option<String>,
    #[serde(rename = "departurePoint")]
    pub departure_point: Option<crate::generated::models::Point>,
    #[serde(rename = "departureTime")]
    pub departure_time: Option<String>,
    pub disruptions: Option<Vec<crate::generated::models::Disruption>>,
    pub distance: Option<f64>,
    pub duration: Option<i32>,
    #[serde(rename = "hasFixedLocations")]
    #[serde(default, deserialize_with = "crate::de::bool_or_string")]
    pub has_fixed_locations: Option<bool>,
    /// Describes the action the user need to take for this section, E.g. "walk to the
    /// district line"
    pub instruction: Option<crate::generated::models::Instruction>,
    #[serde(rename = "interChangeDuration")]
    pub inter_change_duration: Option<String>,
    #[serde(rename = "interChangePosition")]
    pub inter_change_position: Option<String>,
    #[serde(rename = "isDisrupted")]
    #[serde(default, deserialize_with = "crate::de::bool_or_string")]
    pub is_disrupted: Option<bool>,
    pub mode: Option<crate::generated::models::Identifier>,
    pub obstacles: Option<Vec<crate::generated::models::Obstacle>>,
    pub path: Option<crate::generated::models::Path>,
    #[serde(rename = "plannedWorks")]
    pub planned_works: Option<Vec<crate::generated::models::PlannedWork>>,
    #[serde(rename = "routeOptions")]
    pub route_options: Option<Vec<crate::generated::models::RouteOption>>,
    #[serde(rename = "scheduledArrivalTime")]
    pub scheduled_arrival_time: Option<String>,
    #[serde(rename = "scheduledDepartureTime")]
    pub scheduled_departure_time: Option<String>,
    pub speed: Option<String>,
}

/// `Tfl.Api.Presentation.Entities.JourneyPlanner.Obstacle`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Obstacle {
    pub incline: Option<String>,
    pub position: Option<String>,
    #[serde(rename = "stopId")]
    pub stop_id: Option<i32>,
    pub r#type: Option<String>,
}

/// `Tfl.Api.Presentation.Entities.JourneyPlanner.Path`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Path {
    pub elevation: Option<Vec<crate::generated::models::JpElevation>>,
    #[serde(rename = "lineString")]
    pub line_string: Option<String>,
    #[serde(rename = "stopPoints")]
    pub stop_points: Option<Vec<crate::generated::models::Identifier>>,
}

/// `Tfl.Api.Presentation.Entities.JourneyPlanner.PlannedWork`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct PlannedWork {
    #[serde(rename = "createdDateTime")]
    pub created_date_time: Option<String>,
    pub description: Option<String>,
    pub id: Option<String>,
    #[serde(rename = "lastUpdateDateTime")]
    pub last_update_date_time: Option<String>,
}

/// `Tfl.Api.Presentation.Entities.JourneyPlanner.RouteOption`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct RouteOption {
    /// The direction of the route, i.e. outbound or inbound.
    pub direction: Option<String>,
    pub directions: Option<Vec<String>>,
    /// The Id of the route
    pub id: Option<String>,
    /// The line identifier (e.g. District Line), from where you can obtain line status information e.g. the rainbow board status "good service".
    #[serde(rename = "lineIdentifier")]
    pub line_identifier: Option<crate::generated::models::Identifier>,
    /// Name such as "72"
    pub name: Option<String>,
}

/// `Tfl.Api.Presentation.Entities.JourneyPlanner.SearchCriteria`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct SearchCriteria {
    #[serde(rename = "dateTime")]
    pub date_time: Option<String>,
    #[serde(rename = "dateTimeType")]
    pub date_time_type: Option<String>,
    #[serde(rename = "timeAdjustments")]
    pub time_adjustments: Option<crate::generated::models::TimeAdjustments>,
}

/// `Tfl.Api.Presentation.Entities.JourneyPlanner.TimeAdjustment`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct TimeAdjustment {
    pub date: Option<String>,
    pub time: Option<String>,
    #[serde(rename = "timeIs")]
    pub time_is: Option<String>,
    pub uri: Option<String>,
}

/// `Tfl.Api.Presentation.Entities.JourneyPlanner.TimeAdjustments`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct TimeAdjustments {
    pub earlier: Option<crate::generated::models::TimeAdjustment>,
    pub earliest: Option<crate::generated::models::TimeAdjustment>,
    pub later: Option<crate::generated::models::TimeAdjustment>,
    pub latest: Option<crate::generated::models::TimeAdjustment>,
}

/// `Tfl.Api.Presentation.Entities.KnownJourney`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct KnownJourney {
    pub hour: Option<String>,
    #[serde(rename = "intervalId")]
    pub interval_id: Option<i32>,
    pub minute: Option<String>,
}

/// `Tfl.Api.Presentation.Entities.Line`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Line {
    pub created: Option<String>,
    pub crowding: Option<crate::generated::models::Crowding>,
    pub disruptions: Option<Vec<crate::generated::models::Disruption>>,
    pub id: Option<String>,
    #[serde(rename = "lineStatuses")]
    pub line_statuses: Option<Vec<crate::generated::models::LineStatus>>,
    #[serde(rename = "modeName")]
    pub mode_name: Option<String>,
    pub modified: Option<String>,
    pub name: Option<String>,
    #[serde(rename = "routeSections")]
    pub route_sections: Option<Vec<crate::generated::models::MatchedRoute>>,
    #[serde(rename = "serviceTypes")]
    pub service_types: Option<Vec<crate::generated::models::LineServiceTypeInfo>>,
}

/// `Tfl.Api.Presentation.Entities.LineGroup`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct LineGroup {
    #[serde(rename = "lineIdentifier")]
    pub line_identifier: Option<Vec<String>>,
    #[serde(rename = "naptanIdReference")]
    pub naptan_id_reference: Option<String>,
    #[serde(rename = "stationAtcoCode")]
    pub station_atco_code: Option<String>,
}

/// `Tfl.Api.Presentation.Entities.LineModeGroup`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct LineModeGroup {
    #[serde(rename = "lineIdentifier")]
    pub line_identifier: Option<Vec<String>>,
    #[serde(rename = "modeName")]
    pub mode_name: Option<String>,
}

/// `Tfl.Api.Presentation.Entities.LineRouteSection`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct LineRouteSection {
    pub destination: Option<String>,
    pub direction: Option<String>,
    #[serde(rename = "fromStation")]
    pub from_station: Option<String>,
    #[serde(rename = "routeId")]
    pub route_id: Option<i32>,
    #[serde(rename = "serviceType")]
    pub service_type: Option<String>,
    #[serde(rename = "toStation")]
    pub to_station: Option<String>,
    #[serde(rename = "vehicleDestinationText")]
    pub vehicle_destination_text: Option<String>,
}

/// `Tfl.Api.Presentation.Entities.LineServiceType`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct LineServiceType {
    #[serde(rename = "lineName")]
    pub line_name: Option<String>,
    #[serde(rename = "lineSpecificServiceTypes")]
    pub line_specific_service_types: Option<Vec<crate::generated::models::LineSpecificServiceType>>,
}

/// `Tfl.Api.Presentation.Entities.LineServiceTypeInfo`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct LineServiceTypeInfo {
    pub name: Option<String>,
    pub uri: Option<String>,
}

/// `Tfl.Api.Presentation.Entities.LineSpecificServiceType`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct LineSpecificServiceType {
    #[serde(rename = "serviceType")]
    pub service_type: Option<crate::generated::models::LineServiceTypeInfo>,
    #[serde(rename = "stopServesServiceType")]
    #[serde(default, deserialize_with = "crate::de::bool_or_string")]
    pub stop_serves_service_type: Option<bool>,
}

/// `Tfl.Api.Presentation.Entities.LineStatus`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct LineStatus {
    pub created: Option<String>,
    pub disruption: Option<crate::generated::models::Disruption>,
    pub id: Option<i32>,
    #[serde(rename = "lineId")]
    pub line_id: Option<String>,
    pub modified: Option<String>,
    pub reason: Option<String>,
    #[serde(rename = "statusSeverity")]
    pub status_severity: Option<i32>,
    #[serde(rename = "statusSeverityDescription")]
    pub status_severity_description: Option<String>,
    #[serde(rename = "validityPeriods")]
    pub validity_periods: Option<Vec<crate::generated::models::ValidityPeriod>>,
}

/// `Tfl.Api.Presentation.Entities.MatchedRoute`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct MatchedRoute {
    /// The Id (NaPTAN code) or the Destination StopPoint
    pub destination: Option<String>,
    /// The name of the Destination StopPoint
    #[serde(rename = "destinationName")]
    pub destination_name: Option<String>,
    /// Inbound or Outbound
    pub direction: Option<String>,
    /// Name such as "72"
    pub name: Option<String>,
    /// The name of the Origin StopPoint
    #[serde(rename = "originationName")]
    pub origination_name: Option<String>,
    /// The Id (NaPTAN code) of the Origin StopPoint
    pub originator: Option<String>,
    /// The route code
    #[serde(rename = "routeCode")]
    pub route_code: Option<String>,
    /// Regular or Night
    #[serde(rename = "serviceType")]
    pub service_type: Option<String>,
    /// The DateTime that the Service containing this Route is valid from.
    #[serde(rename = "validFrom")]
    pub valid_from: Option<String>,
    /// The DateTime that the Service containing this Route is valid until.
    #[serde(rename = "validTo")]
    pub valid_to: Option<String>,
}

/// `Tfl.Api.Presentation.Entities.MatchedRouteSections`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct MatchedRouteSections {
    pub id: Option<i32>,
}

/// `Tfl.Api.Presentation.Entities.MatchedStop`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct MatchedStop {
    #[serde(rename = "accessibilitySummary")]
    pub accessibility_summary: Option<String>,
    pub direction: Option<String>,
    #[serde(rename = "hasDisruption")]
    #[serde(default, deserialize_with = "crate::de::bool_or_string")]
    pub has_disruption: Option<bool>,
    #[serde(rename = "icsId")]
    pub ics_id: Option<String>,
    pub id: Option<String>,
    pub lat: Option<f64>,
    pub lines: Option<Vec<crate::generated::models::Identifier>>,
    pub lon: Option<f64>,
    pub modes: Option<Vec<String>>,
    pub name: Option<String>,
    #[serde(rename = "parentId")]
    pub parent_id: Option<String>,
    #[serde(rename = "routeId")]
    pub route_id: Option<i32>,
    #[serde(rename = "stationId")]
    pub station_id: Option<String>,
    #[serde(default, deserialize_with = "crate::de::bool_or_string")]
    pub status: Option<bool>,
    #[serde(rename = "stopLetter")]
    pub stop_letter: Option<String>,
    #[serde(rename = "stopType")]
    pub stop_type: Option<String>,
    #[serde(rename = "topMostParentId")]
    pub top_most_parent_id: Option<String>,
    pub towards: Option<String>,
    pub url: Option<String>,
    pub zone: Option<String>,
}

/// `Tfl.Api.Presentation.Entities.Message`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Message {
    #[serde(rename = "bulletOrder")]
    pub bullet_order: Option<i32>,
    #[serde(default, deserialize_with = "crate::de::bool_or_string")]
    pub header: Option<bool>,
    #[serde(rename = "linkText")]
    pub link_text: Option<String>,
    #[serde(rename = "messageText")]
    pub message_text: Option<String>,
    pub url: Option<String>,
}

/// `Tfl.Api.Presentation.Entities.Mode`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Mode {
    #[serde(rename = "isFarePaying")]
    #[serde(default, deserialize_with = "crate::de::bool_or_string")]
    pub is_fare_paying: Option<bool>,
    #[serde(rename = "isScheduledService")]
    #[serde(default, deserialize_with = "crate::de::bool_or_string")]
    pub is_scheduled_service: Option<bool>,
    #[serde(rename = "isTflService")]
    #[serde(default, deserialize_with = "crate::de::bool_or_string")]
    pub is_tfl_service: Option<bool>,
    #[serde(rename = "modeName")]
    pub mode_name: Option<String>,
    #[serde(rename = "motType")]
    pub mot_type: Option<String>,
    pub network: Option<String>,
}

/// `Tfl.Api.Presentation.Entities.NetworkStatus`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct NetworkStatus {
    pub message: Option<String>,
    pub operator: Option<String>,
    pub status: Option<String>,
    #[serde(rename = "statusLevel")]
    pub status_level: Option<i32>,
}

/// `Tfl.Api.Presentation.Entities.OrderedRoute`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct OrderedRoute {
    pub name: Option<String>,
    #[serde(rename = "naptanIds")]
    pub naptan_ids: Option<Vec<String>>,
    #[serde(rename = "serviceType")]
    pub service_type: Option<String>,
}

/// `Tfl.Api.Presentation.Entities.PassengerFlow`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct PassengerFlow {
    /// Time in 24hr format with 15 minute intervals e.g. 0500-0515, 0515-0530 etc.
    #[serde(rename = "timeSlice")]
    pub time_slice: Option<String>,
    /// Count of passenger flow towards a platform
    pub value: Option<i32>,
}

/// `Tfl.Api.Presentation.Entities.PathAttribute`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct PathAttribute {
    pub name: Option<String>,
    pub value: Option<String>,
}

/// `Tfl.Api.Presentation.Entities.Period`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Period {
    pub frequency: Option<crate::generated::models::ServiceFrequency>,
    #[serde(rename = "fromTime")]
    pub from_time: Option<crate::generated::models::TwentyFourHourClockTime>,
    #[serde(rename = "toTime")]
    pub to_time: Option<crate::generated::models::TwentyFourHourClockTime>,
    pub r#type: Option<String>,
}

/// `Tfl.Api.Presentation.Entities.Place`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Place {
    /// A bag of additional key/value pairs with extra information about this place.
    #[serde(rename = "additionalProperties")]
    pub additional_properties: Option<Vec<crate::generated::models::AdditionalProperties>>,
    pub children: Option<Vec<crate::generated::models::Place>>,
    #[serde(rename = "childrenUrls")]
    pub children_urls: Option<Vec<String>>,
    /// A human readable name.
    #[serde(rename = "commonName")]
    pub common_name: Option<String>,
    /// The distance of the place from its search point, if this is the result
    /// of a geographical search, otherwise zero.
    pub distance: Option<f64>,
    /// A unique identifier.
    pub id: Option<String>,
    /// WGS84 latitude of the location.
    pub lat: Option<f64>,
    /// WGS84 longitude of the location.
    pub lon: Option<f64>,
    /// The type of Place. See /Place/Meta/placeTypes for possible values.
    #[serde(rename = "placeType")]
    pub place_type: Option<String>,
    /// The unique location of this resource.
    pub url: Option<String>,
}

/// `Tfl.Api.Presentation.Entities.PlaceCategory`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct PlaceCategory {
    #[serde(rename = "availableKeys")]
    pub available_keys: Option<Vec<String>>,
    pub category: Option<String>,
}

/// `Tfl.Api.Presentation.Entities.PlacePolygon`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct PlacePolygon {
    #[serde(rename = "commonName")]
    pub common_name: Option<String>,
    #[serde(rename = "geoPoints")]
    pub geo_points: Option<Vec<crate::generated::models::GeoPoint>>,
}

/// `Tfl.Api.Presentation.Entities.Point`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Point {
    /// WGS84 latitude of the location.
    pub lat: Option<f64>,
    /// WGS84 longitude of the location.
    pub lon: Option<f64>,
}

/// `Tfl.Api.Presentation.Entities.Prediction`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Prediction {
    /// Data base version
    #[serde(rename = "baseVersion")]
    pub base_version: Option<String>,
    /// Bearing (between 0 to 359)
    pub bearing: Option<String>,
    /// The current location of the vehicle.
    #[serde(rename = "currentLocation")]
    pub current_location: Option<String>,
    /// Name of the destination
    #[serde(rename = "destinationName")]
    pub destination_name: Option<String>,
    /// Naptan Identifier for the prediction's destination
    #[serde(rename = "destinationNaptanId")]
    pub destination_naptan_id: Option<String>,
    /// Direction (unified to inbound/outbound)
    pub direction: Option<String>,
    /// The expected arrival time of the vehicle at the stop/station
    #[serde(rename = "expectedArrival")]
    pub expected_arrival: Option<String>,
    /// The identitier for the prediction
    pub id: Option<String>,
    /// Unique identifier for the Line
    #[serde(rename = "lineId")]
    pub line_id: Option<String>,
    /// Line Name
    #[serde(rename = "lineName")]
    pub line_name: Option<String>,
    /// The mode name of the station/line the prediction relates to
    #[serde(rename = "modeName")]
    pub mode_name: Option<String>,
    /// Identifier for the prediction
    #[serde(rename = "naptanId")]
    pub naptan_id: Option<String>,
    /// The type of the operation (1: is new or has been updated, 2: should be deleted from any client cache)
    #[serde(rename = "operationType")]
    pub operation_type: Option<i32>,
    /// Platform name (for bus, this is the stop letter)
    #[serde(rename = "platformName")]
    pub platform_name: Option<String>,
    /// Station name
    #[serde(rename = "stationName")]
    pub station_name: Option<String>,
    /// The expiry time for the prediction
    #[serde(rename = "timeToLive")]
    pub time_to_live: Option<String>,
    /// Prediction of the Time to station in seconds
    #[serde(rename = "timeToStation")]
    pub time_to_station: Option<i32>,
    /// Timestamp for when the prediction was inserted/modified (source column drives what objects are broadcast on each iteration)
    pub timestamp: Option<String>,
    /// Keep the original timestamp from MongoDb fo debugging purposes
    pub timing: Option<crate::generated::models::PredictionTiming>,
    /// Routing information or other descriptive text about the path of the vehicle towards the destination
    pub towards: Option<String>,
    /// TripId is used to assemble the primary key
    #[serde(rename = "tripId")]
    pub trip_id: Option<String>,
    /// The actual vehicle in transit (for train modes, the leading car of the rolling set)
    #[serde(rename = "vehicleId")]
    pub vehicle_id: Option<String>,
}

/// `Tfl.Api.Presentation.Entities.PredictionTiming`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct PredictionTiming {
    #[serde(rename = "countdownServerAdjustment")]
    pub countdown_server_adjustment: Option<String>,
    pub insert: Option<String>,
    pub read: Option<String>,
    pub received: Option<String>,
    pub sent: Option<String>,
    pub source: Option<String>,
}

/// `Tfl.Api.Presentation.Entities.Redirect`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Redirect {
    #[serde(default, deserialize_with = "crate::de::bool_or_string")]
    pub active: Option<bool>,
    #[serde(rename = "longUrl")]
    pub long_url: Option<String>,
    #[serde(rename = "shortUrl")]
    pub short_url: Option<String>,
}

/// `Tfl.Api.Presentation.Entities.RoadCorridor`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct RoadCorridor {
    /// The Bounds of the Corridor, given by the south-east followed by the north-west co-ordinate
    /// pair in geoJSON format e.g. "[[-1.241531,51.242151],[1.641223,53.765721]]"
    pub bounds: Option<String>,
    /// The display name of the Corridor e.g. "North Circular (A406)". This
    /// may be identical to the Id.
    #[serde(rename = "displayName")]
    pub display_name: Option<String>,
    /// The Envelope of the Corridor, given by the corner co-ordinates of a rectangular (four-point) polygon
    /// in geoJSON format e.g. "[[-1.241531,51.242151],[-1.241531,53.765721],[1.641223,53.765721],[1.641223,51.242151]]"
    pub envelope: Option<String>,
    /// The group name of the Corridor e.g. "Central London". Most corridors are not grouped, in which case this field can be null.
    pub group: Option<String>,
    /// The Id of the Corridor e.g. "A406"
    pub id: Option<String>,
    /// The end of the period over which status has been aggregated, or null if this is the current corridor status.
    #[serde(rename = "statusAggregationEndDate")]
    pub status_aggregation_end_date: Option<String>,
    /// The start of the period over which status has been aggregated, or null if this is the current corridor status.
    #[serde(rename = "statusAggregationStartDate")]
    pub status_aggregation_start_date: Option<String>,
    /// Standard multi-mode status severity code
    #[serde(rename = "statusSeverity")]
    pub status_severity: Option<String>,
    /// Description of the status severity as applied to RoadCorridors
    #[serde(rename = "statusSeverityDescription")]
    pub status_severity_description: Option<String>,
    /// URL to retrieve this Corridor.
    pub url: Option<String>,
}

/// `Tfl.Api.Presentation.Entities.RoadDisruption`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct RoadDisruption {
    /// Describes the nature of disruption e.g. Traffic Incidents, Works
    pub category: Option<String>,
    /// Full text of comments describing the disruption, including details of any road closures and diversions, where appropriate.
    pub comments: Option<String>,
    /// The Ids of affected corridors, if any.
    #[serde(rename = "corridorIds")]
    pub corridor_ids: Option<Vec<String>>,
    /// Text of the most recent update from the LSTCC on the state of the
    /// disruption, including the current traffic impact and any advice to
    /// road users.
    #[serde(rename = "currentUpdate")]
    pub current_update: Option<String>,
    /// The time when the last CurrentUpdate description was recorded,
    /// or null if no CurrentUpdate has been applied.
    #[serde(rename = "currentUpdateDateTime")]
    pub current_update_date_time: Option<String>,
    /// The date and time on which the disruption ended. For planned disruptions, this date will have a valid value. For unplanned
    /// disruptions in progress, this field will be omitted.
    #[serde(rename = "endDateTime")]
    pub end_date_time: Option<String>,
    /// Geography version of Point for output as GeoJSON.
    /// Can not use Geometry in a consistent way as non-TIMS disruptions do not have a polygon
    pub geography: Option<crate::generated::models::DbGeography>,
    /// GeoJSON formatted latitude/longitude (WGS84) pairs forming an enclosed polyline or polygon. The polygon will only be included where affected streets information
    /// is not available for the disruption, would be inappropriate (e.g. a very large number of streets), or is centred on an area without streets (e.g. a football stadium).
    pub geometry: Option<crate::generated::models::DbGeography>,
    /// True if any of the affected Streets have a "Full Closure" status, false otherwise. A RoadDisruption that has HasClosures is considered a
    /// Severe or Serious disruption for severity filtering purposes.
    #[serde(rename = "hasClosures")]
    #[serde(default, deserialize_with = "crate::de::bool_or_string")]
    pub has_closures: Option<bool>,
    /// Unique identifier for the road disruption
    pub id: Option<String>,
    /// True if the disruption is planned on a future date that is open to change
    #[serde(rename = "isProvisional")]
    #[serde(default, deserialize_with = "crate::de::bool_or_string")]
    pub is_provisional: Option<bool>,
    /// The date and time on which the disruption was last modified in the system. This information can reliably be used by a developer to quickly
    /// compare two instances of the same disruption to determine if it has been changed.
    #[serde(rename = "lastModifiedTime")]
    pub last_modified_time: Option<String>,
    /// This describes the level of potential impact on traffic operations of the disruption.
    /// High = e.g. a one-off disruption on a major or high profile route which will require a high level of operational attention
    /// Medium = This is the default value
    /// Low = e.g. a frequently occurring disruption which is well known
    #[serde(rename = "levelOfInterest")]
    pub level_of_interest: Option<String>,
    /// The text of any associated link
    #[serde(rename = "linkText")]
    pub link_text: Option<String>,
    /// The url of any associated link
    #[serde(rename = "linkUrl")]
    pub link_url: Option<String>,
    /// Main road name / number (borough) or preset area name where the disruption is located. This might be useful for a map popup where space is limited.
    pub location: Option<String>,
    /// An ordinal of the disruption based on severity, level of interest and corridor.
    pub ordinal: Option<i32>,
    /// Latitude and longitude (WGS84) of the centroid of the disruption, stored in a geoJSON-formatted string.
    pub point: Option<String>,
    #[serde(rename = "publishEndDate")]
    pub publish_end_date: Option<String>,
    /// TDM Additional properties
    #[serde(rename = "publishStartDate")]
    pub publish_start_date: Option<String>,
    #[serde(rename = "recurringSchedules")]
    pub recurring_schedules: Option<Vec<crate::generated::models::RoadDisruptionSchedule>>,
    #[serde(rename = "roadDisruptionImpactAreas")]
    pub road_disruption_impact_areas:
        Option<Vec<crate::generated::models::RoadDisruptionImpactArea>>,
    #[serde(rename = "roadDisruptionLines")]
    pub road_disruption_lines: Option<Vec<crate::generated::models::RoadDisruptionLine>>,
    /// Any associated road project
    #[serde(rename = "roadProject")]
    pub road_project: Option<crate::generated::models::RoadProject>,
    /// A description of the severity of the disruption.
    pub severity: Option<String>,
    /// The date and time which the disruption started. For a planned disruption (i.e. planned road works) this date will be in the future.
    /// For unplanned disruptions, this will default to the date on which the disruption was first recorded, but may be adjusted by the operator.
    #[serde(rename = "startDateTime")]
    pub start_date_time: Option<String>,
    /// This describes the status of the disruption.
    /// Active = currently in progress
    /// Active Long Term = currently in progress and long term
    /// Scheduled = scheduled to start within the next 180 days
    /// Recurring Works = planned maintenance works that follow a regular routine or pattern and whose next occurrence is to start within the next 180 days.
    /// Recently Cleared = recently cleared in the last 24 hours
    /// Note that the status of Scheduled or Recurring Works disruptions will change to Active when they start, and will change status again when they end.
    pub status: Option<String>,
    /// A collection of zero or more streets affected by the disruption.
    pub streets: Option<Vec<crate::generated::models::Street>>,
    /// Describes the sub-category of disruption e.g. Collapsed Manhole, Abnormal Load
    #[serde(rename = "subCategory")]
    pub sub_category: Option<String>,
    #[serde(rename = "timeFrame")]
    pub time_frame: Option<String>,
    /// URL to retrieve this road disruption
    pub url: Option<String>,
}

/// `Tfl.Api.Presentation.Entities.RoadDisruptionImpactArea`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct RoadDisruptionImpactArea {
    #[serde(rename = "endDate")]
    pub end_date: Option<String>,
    #[serde(rename = "endTime")]
    pub end_time: Option<String>,
    pub id: Option<i32>,
    pub polygon: Option<crate::generated::models::DbGeography>,
    #[serde(rename = "roadDisruptionId")]
    pub road_disruption_id: Option<String>,
    #[serde(rename = "startDate")]
    pub start_date: Option<String>,
    #[serde(rename = "startTime")]
    pub start_time: Option<String>,
}

/// `Tfl.Api.Presentation.Entities.RoadDisruptionLine`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct RoadDisruptionLine {
    #[serde(rename = "endDate")]
    pub end_date: Option<String>,
    #[serde(rename = "endTime")]
    pub end_time: Option<String>,
    pub id: Option<i32>,
    #[serde(rename = "isDiversion")]
    #[serde(default, deserialize_with = "crate::de::bool_or_string")]
    pub is_diversion: Option<bool>,
    #[serde(rename = "multiLineString")]
    pub multi_line_string: Option<crate::generated::models::DbGeography>,
    #[serde(rename = "roadDisruptionId")]
    pub road_disruption_id: Option<String>,
    #[serde(rename = "startDate")]
    pub start_date: Option<String>,
    #[serde(rename = "startTime")]
    pub start_time: Option<String>,
}

/// `Tfl.Api.Presentation.Entities.RoadDisruptionSchedule`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct RoadDisruptionSchedule {
    #[serde(rename = "endTime")]
    pub end_time: Option<String>,
    #[serde(rename = "startTime")]
    pub start_time: Option<String>,
}

/// `Tfl.Api.Presentation.Entities.RoadProject`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct RoadProject {
    #[serde(rename = "boroughsBenefited")]
    pub boroughs_benefited: Option<Vec<String>>,
    #[serde(rename = "constructionEndDate")]
    pub construction_end_date: Option<String>,
    #[serde(rename = "constructionStartDate")]
    pub construction_start_date: Option<String>,
    #[serde(rename = "consultationEndDate")]
    pub consultation_end_date: Option<String>,
    #[serde(rename = "consultationPageUrl")]
    pub consultation_page_url: Option<String>,
    #[serde(rename = "consultationStartDate")]
    pub consultation_start_date: Option<String>,
    #[serde(rename = "contactEmail")]
    pub contact_email: Option<String>,
    #[serde(rename = "contactName")]
    pub contact_name: Option<String>,
    #[serde(rename = "cycleSuperhighwayId")]
    pub cycle_superhighway_id: Option<String>,
    #[serde(rename = "externalPageUrl")]
    pub external_page_url: Option<String>,
    pub phase: Option<String>,
    #[serde(rename = "projectDescription")]
    pub project_description: Option<String>,
    #[serde(rename = "projectId")]
    pub project_id: Option<String>,
    #[serde(rename = "projectName")]
    pub project_name: Option<String>,
    #[serde(rename = "projectPageUrl")]
    pub project_page_url: Option<String>,
    #[serde(rename = "projectSummaryPageUrl")]
    pub project_summary_page_url: Option<String>,
    #[serde(rename = "schemeName")]
    pub scheme_name: Option<String>,
}

/// `Tfl.Api.Presentation.Entities.RouteSearchMatch`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct RouteSearchMatch {
    pub id: Option<String>,
    pub lat: Option<f64>,
    #[serde(rename = "lineId")]
    pub line_id: Option<String>,
    #[serde(rename = "lineName")]
    pub line_name: Option<String>,
    #[serde(rename = "lineRouteSection")]
    pub line_route_section: Option<Vec<crate::generated::models::LineRouteSection>>,
    pub lon: Option<f64>,
    #[serde(rename = "matchedRouteSections")]
    pub matched_route_sections: Option<Vec<crate::generated::models::MatchedRouteSections>>,
    #[serde(rename = "matchedStops")]
    pub matched_stops: Option<Vec<crate::generated::models::MatchedStop>>,
    pub mode: Option<String>,
    pub name: Option<String>,
    pub url: Option<String>,
}

/// `Tfl.Api.Presentation.Entities.RouteSearchResponse`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct RouteSearchResponse {
    pub input: Option<String>,
    #[serde(rename = "searchMatches")]
    pub search_matches: Option<Vec<crate::generated::models::RouteSearchMatch>>,
}

/// `Tfl.Api.Presentation.Entities.RouteSectionNaptanEntrySequence`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct RouteSectionNaptanEntrySequence {
    pub ordinal: Option<i32>,
    #[serde(rename = "stopPoint")]
    pub stop_point: Option<crate::generated::models::StopPoint>,
}

/// `Tfl.Api.Presentation.Entities.RouteSequence`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct RouteSequence {
    pub direction: Option<String>,
    #[serde(rename = "isOutboundOnly")]
    #[serde(default, deserialize_with = "crate::de::bool_or_string")]
    pub is_outbound_only: Option<bool>,
    #[serde(rename = "lineId")]
    pub line_id: Option<String>,
    #[serde(rename = "lineName")]
    pub line_name: Option<String>,
    #[serde(rename = "lineStrings")]
    pub line_strings: Option<Vec<String>>,
    pub mode: Option<String>,
    #[serde(rename = "orderedLineRoutes")]
    pub ordered_line_routes: Option<Vec<crate::generated::models::OrderedRoute>>,
    pub stations: Option<Vec<crate::generated::models::MatchedStop>>,
    #[serde(rename = "stopPointSequences")]
    pub stop_point_sequences: Option<Vec<crate::generated::models::StopPointSequence>>,
}

/// `Tfl.Api.Presentation.Entities.Schedule`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Schedule {
    #[serde(rename = "firstJourney")]
    pub first_journey: Option<crate::generated::models::KnownJourney>,
    #[serde(rename = "knownJourneys")]
    pub known_journeys: Option<Vec<crate::generated::models::KnownJourney>>,
    #[serde(rename = "lastJourney")]
    pub last_journey: Option<crate::generated::models::KnownJourney>,
    pub name: Option<String>,
    pub periods: Option<Vec<crate::generated::models::Period>>,
}

/// `Tfl.Api.Presentation.Entities.SearchMatch`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct SearchMatch {
    pub id: Option<String>,
    pub lat: Option<f64>,
    pub lon: Option<f64>,
    pub name: Option<String>,
    pub url: Option<String>,
}

/// `Tfl.Api.Presentation.Entities.SearchResponse`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct SearchResponse {
    pub from: Option<i32>,
    pub matches: Option<Vec<crate::generated::models::SearchMatch>>,
    #[serde(rename = "maxScore")]
    pub max_score: Option<f64>,
    pub page: Option<i32>,
    #[serde(rename = "pageSize")]
    pub page_size: Option<i32>,
    pub provider: Option<String>,
    pub query: Option<String>,
    pub total: Option<i32>,
}

/// `Tfl.Api.Presentation.Entities.ServiceFrequency`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ServiceFrequency {
    #[serde(rename = "highestFrequency")]
    pub highest_frequency: Option<f64>,
    #[serde(rename = "lowestFrequency")]
    pub lowest_frequency: Option<f64>,
}

/// `Tfl.Api.Presentation.Entities.StationInterval`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct StationInterval {
    pub id: Option<String>,
    pub intervals: Option<Vec<crate::generated::models::Interval>>,
}

/// `Tfl.Api.Presentation.Entities.StatusSeverity`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct StatusSeverity {
    pub description: Option<String>,
    #[serde(rename = "modeName")]
    pub mode_name: Option<String>,
    #[serde(rename = "severityLevel")]
    pub severity_level: Option<i32>,
}

/// `Tfl.Api.Presentation.Entities.StopPoint`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct StopPoint {
    #[serde(rename = "accessibilitySummary")]
    pub accessibility_summary: Option<String>,
    /// A bag of additional key/value pairs with extra information about this place.
    #[serde(rename = "additionalProperties")]
    pub additional_properties: Option<Vec<crate::generated::models::AdditionalProperties>>,
    pub children: Option<Vec<crate::generated::models::Place>>,
    #[serde(rename = "childrenUrls")]
    pub children_urls: Option<Vec<String>>,
    /// A human readable name.
    #[serde(rename = "commonName")]
    pub common_name: Option<String>,
    /// The distance of the place from its search point, if this is the result
    /// of a geographical search, otherwise zero.
    pub distance: Option<f64>,
    #[serde(rename = "fullName")]
    pub full_name: Option<String>,
    #[serde(rename = "hubNaptanCode")]
    pub hub_naptan_code: Option<String>,
    #[serde(rename = "icsCode")]
    pub ics_code: Option<String>,
    /// A unique identifier.
    pub id: Option<String>,
    /// The indicator of the stop point e.g. "Stop K"
    pub indicator: Option<String>,
    #[serde(rename = "individualStopId")]
    pub individual_stop_id: Option<String>,
    /// WGS84 latitude of the location.
    pub lat: Option<f64>,
    #[serde(rename = "lineGroup")]
    pub line_group: Option<Vec<crate::generated::models::LineGroup>>,
    #[serde(rename = "lineModeGroups")]
    pub line_mode_groups: Option<Vec<crate::generated::models::LineModeGroup>>,
    pub lines: Option<Vec<crate::generated::models::Identifier>>,
    /// WGS84 longitude of the location.
    pub lon: Option<f64>,
    pub modes: Option<Vec<String>>,
    #[serde(rename = "naptanId")]
    pub naptan_id: Option<String>,
    #[serde(rename = "naptanMode")]
    pub naptan_mode: Option<String>,
    /// The type of Place. See /Place/Meta/placeTypes for possible values.
    #[serde(rename = "placeType")]
    pub place_type: Option<String>,
    #[serde(rename = "platformName")]
    pub platform_name: Option<String>,
    #[serde(rename = "smsCode")]
    pub sms_code: Option<String>,
    #[serde(rename = "stationNaptan")]
    pub station_naptan: Option<String>,
    #[serde(default, deserialize_with = "crate::de::bool_or_string")]
    pub status: Option<bool>,
    /// The stop letter, if it could be cleansed from the Indicator e.g. "K"
    #[serde(rename = "stopLetter")]
    pub stop_letter: Option<String>,
    #[serde(rename = "stopType")]
    pub stop_type: Option<String>,
    /// The unique location of this resource.
    pub url: Option<String>,
}

/// `Tfl.Api.Presentation.Entities.StopPointCategory`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct StopPointCategory {
    #[serde(rename = "availableKeys")]
    pub available_keys: Option<Vec<String>>,
    pub category: Option<String>,
}

/// `Tfl.Api.Presentation.Entities.StopPointRouteSection`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct StopPointRouteSection {
    #[serde(rename = "destinationName")]
    pub destination_name: Option<String>,
    pub direction: Option<String>,
    #[serde(rename = "isActive")]
    #[serde(default, deserialize_with = "crate::de::bool_or_string")]
    pub is_active: Option<bool>,
    #[serde(rename = "lineId")]
    pub line_id: Option<String>,
    #[serde(rename = "lineString")]
    pub line_string: Option<String>,
    pub mode: Option<String>,
    #[serde(rename = "naptanId")]
    pub naptan_id: Option<String>,
    #[serde(rename = "routeSectionName")]
    pub route_section_name: Option<String>,
    #[serde(rename = "serviceType")]
    pub service_type: Option<String>,
    #[serde(rename = "validFrom")]
    pub valid_from: Option<String>,
    #[serde(rename = "validTo")]
    pub valid_to: Option<String>,
    #[serde(rename = "vehicleDestinationText")]
    pub vehicle_destination_text: Option<String>,
}

/// `Tfl.Api.Presentation.Entities.StopPointSequence`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct StopPointSequence {
    /// The id of this branch.
    #[serde(rename = "branchId")]
    pub branch_id: Option<i32>,
    pub direction: Option<String>,
    #[serde(rename = "lineId")]
    pub line_id: Option<String>,
    #[serde(rename = "lineName")]
    pub line_name: Option<String>,
    /// The ids of the next branch(es) in the sequence. Note that the next and previous branch id can be
    /// identical in the case of a looped route e.g. the Circle line.
    #[serde(rename = "nextBranchIds")]
    pub next_branch_ids: Option<Vec<i32>>,
    /// The ids of the previous branch(es) in the sequence. Note that the next and previous branch id can be
    /// identical in the case of a looped route e.g. the Circle line.
    #[serde(rename = "prevBranchIds")]
    pub prev_branch_ids: Option<Vec<i32>>,
    #[serde(rename = "serviceType")]
    pub service_type: Option<String>,
    #[serde(rename = "stopPoint")]
    pub stop_point: Option<Vec<crate::generated::models::MatchedStop>>,
}

/// `Tfl.Api.Presentation.Entities.StopPointsResponse`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct StopPointsResponse {
    /// The centre latitude/longitude of this list of StopPoints
    #[serde(rename = "centrePoint")]
    pub centre_point: Option<Vec<f64>>,
    /// The index of this page
    pub page: Option<i32>,
    /// The maximum size of the page in this response i.e. the maximum number of StopPoints
    #[serde(rename = "pageSize")]
    pub page_size: Option<i32>,
    /// Collection of stop points
    #[serde(rename = "stopPoints")]
    pub stop_points: Option<Vec<crate::generated::models::StopPoint>>,
    /// The total number of StopPoints available across all pages
    pub total: Option<i32>,
}

/// `Tfl.Api.Presentation.Entities.Street`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Street {
    /// Type of road closure. Some example values:
    /// Open = road is open, not blocked, not closed, not restricted. It maybe that the disruption has been moved out of the carriageway.
    /// Partial Closure = road is partially blocked, closed or restricted.
    /// Full Closure = road is fully blocked or closed.
    pub closure: Option<String>,
    /// The direction of the disruption on the street. Some example values:
    /// All Directions
    /// All Approaches
    /// Clockwise
    /// Anti-Clockwise
    /// Northbound
    /// Eastbound
    /// Southbound
    /// Westbound
    /// Both Directions
    pub directions: Option<String>,
    /// Street name
    pub name: Option<String>,
    /// Geographic description of the sections of this street that are affected.
    pub segments: Option<Vec<crate::generated::models::StreetSegment>>,
    /// The ID from the source system of the disruption that this street belongs to.
    #[serde(rename = "sourceSystemId")]
    pub source_system_id: Option<i64>,
    /// The key of the source system of the disruption that this street belongs to.
    #[serde(rename = "sourceSystemKey")]
    pub source_system_key: Option<String>,
}

/// `Tfl.Api.Presentation.Entities.StreetSegment`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct StreetSegment {
    /// geoJSON formatted LineString containing two latitude/longitude (WGS84) pairs that identify the start and end points of the street segment.
    #[serde(rename = "lineString")]
    pub line_string: Option<String>,
    /// The ID from the source system of the disruption that this street belongs to.
    #[serde(rename = "sourceSystemId")]
    pub source_system_id: Option<i64>,
    /// The key of the source system of the disruption that this street belongs to.
    #[serde(rename = "sourceSystemKey")]
    pub source_system_key: Option<String>,
    /// A 16 digit unique integer identifying a OS ITN (Ordnance Survey Integrated Transport Network) road link.
    pub toid: Option<String>,
}

/// `Tfl.Api.Presentation.Entities.Timetable`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Timetable {
    #[serde(rename = "departureStopId")]
    pub departure_stop_id: Option<String>,
    pub routes: Option<Vec<crate::generated::models::TimetableRoute>>,
}

/// `Tfl.Api.Presentation.Entities.TimetableResponse`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct TimetableResponse {
    pub direction: Option<String>,
    pub disambiguation: Option<crate::generated::models::Disambiguation>,
    #[serde(rename = "lineId")]
    pub line_id: Option<String>,
    #[serde(rename = "lineName")]
    pub line_name: Option<String>,
    #[serde(rename = "pdfUrl")]
    pub pdf_url: Option<String>,
    pub stations: Option<Vec<crate::generated::models::MatchedStop>>,
    #[serde(rename = "statusErrorMessage")]
    pub status_error_message: Option<String>,
    pub stops: Option<Vec<crate::generated::models::MatchedStop>>,
    pub timetable: Option<crate::generated::models::Timetable>,
}

/// `Tfl.Api.Presentation.Entities.TimetableRoute`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct TimetableRoute {
    pub schedules: Option<Vec<crate::generated::models::Schedule>>,
    #[serde(rename = "stationIntervals")]
    pub station_intervals: Option<Vec<crate::generated::models::StationInterval>>,
}

/// `Tfl.Api.Presentation.Entities.Timetables.Disambiguation`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Disambiguation {
    #[serde(rename = "disambiguationOptions")]
    pub disambiguation_options: Option<Vec<crate::generated::models::DisambiguationOption>>,
}

/// `Tfl.Api.Presentation.Entities.Timetables.DisambiguationOption`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct DisambiguationOption {
    pub description: Option<String>,
    pub uri: Option<String>,
}

/// `Tfl.Api.Presentation.Entities.TrainLoading`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct TrainLoading {
    /// Direction in regards to Journey Planner i.e. inbound or outbound
    pub direction: Option<String>,
    /// The Line Name e.g. "Victoria"
    pub line: Option<String>,
    /// Direction of the Line e.g. NB, SB, WB etc.
    #[serde(rename = "lineDirection")]
    pub line_direction: Option<String>,
    /// Naptan of the adjacent station
    #[serde(rename = "naptanTo")]
    pub naptan_to: Option<String>,
    /// Direction displayed on the platform e.g. NB, SB, WB etc.
    #[serde(rename = "platformDirection")]
    pub platform_direction: Option<String>,
    /// Time in 24hr format with 15 minute intervals e.g. 0500-0515, 0515-0530 etc.
    #[serde(rename = "timeSlice")]
    pub time_slice: Option<String>,
    /// Scale between 1-6,
    /// 1 = Very quiet, 2 = Quiet, 3 = Fairly busy, 4 = Busy, 5 = Very busy, 6 = Exceptionally busy
    pub value: Option<i32>,
}

/// `Tfl.Api.Presentation.Entities.TwentyFourHourClockTime`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct TwentyFourHourClockTime {
    pub hour: Option<String>,
    pub minute: Option<String>,
}

/// `Tfl.Api.Presentation.Entities.ValidityPeriod`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ValidityPeriod {
    /// Gets or sets the start date.
    #[serde(rename = "fromDate")]
    pub from_date: Option<String>,
    /// If true is a realtime status rather than planned or info
    #[serde(rename = "isNow")]
    #[serde(default, deserialize_with = "crate::de::bool_or_string")]
    pub is_now: Option<bool>,
    /// Gets or sets the end date.
    #[serde(rename = "toDate")]
    pub to_date: Option<String>,
}
