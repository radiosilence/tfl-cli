//! Santander Cycles docking stations.
//!
//! TfL returns a bike point as a generic `Place` whose interesting content is
//! buried in a stringly-typed property bag: `NbBikes=4`, `NbEBikes=1`,
//! `NbEmptyDocks=13`. Nobody can usefully query that, so the bag is parsed into
//! real fields here — the counts are the entire reason to ask about a docking
//! station.
//!
//! There are around 800 stations and TfL offers no geographic filter, so
//! [`super::query::QueryRoot::bike_points_near`] fetches the set once and
//! measures locally. One request, not eight hundred.

use async_graphql::{Context, Object, Result};
use tfl_api_client::models;

use super::loaders::{loaders, to_gql_error};

/// A Santander Cycles docking station.
#[derive(Clone)]
pub struct BikePoint {
    pub(crate) place: models::Place,
    /// Metres from the point searched for, when this came from a geographic
    /// search. Computed here because TfL cannot filter bike points by location.
    pub(crate) distance: Option<f64>,
}

impl BikePoint {
    pub fn new(place: models::Place) -> Self {
        Self {
            place,
            distance: None,
        }
    }

    /// Reads one of TfL's `additionalProperties` entries as a number.
    fn count(&self, key: &str) -> Option<i32> {
        self.place
            .additional_properties
            .iter()
            .flatten()
            .find(|p| p.key.as_deref() == Some(key))
            .and_then(|p| p.value.as_deref())
            .and_then(|v| v.parse().ok())
    }

    /// Whether the station is actually usable.
    ///
    /// TfL leaves the last-synced counts on a station it has pulled for
    /// maintenance, so a locked dock can still report four bikes. Counts alone
    /// would send someone to it.
    pub(crate) fn in_service(&self) -> bool {
        self.flag("Installed") != Some(false) && self.flag("Locked") != Some(true)
    }

    /// Whether a bike can be taken right now.
    ///
    /// A station that did not report a count is treated as having none: an
    /// absent figure is not a promise of a bike, and sending someone to an
    /// empty dock is the worse failure.
    pub(crate) fn has_bikes(&self) -> bool {
        self.in_service() && self.count("NbBikes").is_some_and(|n| n > 0)
    }

    /// Whether a bike can be returned right now.
    pub(crate) fn has_docks(&self) -> bool {
        self.in_service() && self.count("NbEmptyDocks").is_some_and(|n| n > 0)
    }

    fn flag(&self, key: &str) -> Option<bool> {
        self.place
            .additional_properties
            .iter()
            .flatten()
            .find(|p| p.key.as_deref() == Some(key))
            .and_then(|p| p.value.as_deref())
            .map(|v| v.eq_ignore_ascii_case("true"))
    }
}

#[Object]
impl BikePoint {
    /// TfL's identifier, e.g. `BikePoints_1`.
    async fn id(&self) -> Option<&str> {
        self.place.id.as_deref()
    }

    /// Where it is, in TfL's words, e.g. `River Street , Clerkenwell`.
    async fn common_name(&self) -> Option<&str> {
        self.place.common_name.as_deref()
    }

    async fn lat(&self) -> Option<f64> {
        self.place.lat
    }

    async fn lon(&self) -> Option<f64> {
        self.place.lon
    }

    /// Metres from the searched coordinate. Null unless this came from
    /// `bikePointsNear`.
    async fn distance(&self) -> Option<f64> {
        self.distance
    }

    /// Bikes available now, of any kind.
    ///
    /// Free — it arrives with the station. Note this is TfL's figure at the
    /// last sync, so it can lag reality by a few minutes.
    async fn bikes(&self) -> Option<i32> {
        self.count("NbBikes")
    }

    /// Electric bikes available now.
    async fn e_bikes(&self) -> Option<i32> {
        self.count("NbEBikes")
    }

    /// Ordinary pedal bikes available now.
    async fn standard_bikes(&self) -> Option<i32> {
        self.count("NbStandardBikes")
    }

    /// Free docks — what matters when you are returning a bike rather than
    /// taking one.
    async fn empty_docks(&self) -> Option<i32> {
        self.count("NbEmptyDocks")
    }

    /// Docks at this station in total, working or not.
    async fn total_docks(&self) -> Option<i32> {
        self.count("NbDocks")
    }

    /// Whether a bike can be taken right now.
    ///
    /// False for a station that is locked or not installed, whatever its counts
    /// say — TfL leaves stale counts on stations it has taken out of service.
    #[graphql(name = "hasBikes")]
    async fn has_bikes_field(&self) -> bool {
        self.has_bikes()
    }

    /// Whether a bike can be returned right now.
    #[graphql(name = "hasDocks")]
    async fn has_docks_field(&self) -> bool {
        self.has_docks()
    }

    /// Whether the station is in service. A station can exist but be switched
    /// off, in which case its counts are meaningless.
    async fn installed(&self) -> Option<bool> {
        self.flag("Installed")
    }

    /// Whether the station is locked, i.e. temporarily out of use.
    async fn locked(&self) -> Option<bool> {
        self.flag("Locked")
    }

    /// Whether this is a temporary station, e.g. for an event.
    async fn temporary(&self) -> Option<bool> {
        self.flag("Temporary")
    }

    /// Counts from TfL's dedicated occupancy feed rather than the station
    /// record.
    ///
    /// The numbers on this type are usually the same and cost nothing, so reach
    /// for this only when you want the occupancy feed specifically. Costs one
    /// request per 20 stations, batched across the query.
    async fn occupancy(&self, ctx: &Context<'_>) -> Result<Option<BikeOccupancy>> {
        let Some(id) = self.place.id.clone() else {
            return Ok(None);
        };
        Ok(loaders(ctx)
            .bike_occupancy
            .load_one(id)
            .await
            .map_err(to_gql_error)?
            .map(BikeOccupancy))
    }
}

/// Live counts from TfL's occupancy feed.
pub struct BikeOccupancy(pub models::BikePointOccupancy);

#[Object]
impl BikeOccupancy {
    async fn bikes(&self) -> Option<i32> {
        self.0.bikes_count
    }

    async fn e_bikes(&self) -> Option<i32> {
        self.0.e_bikes_count
    }

    async fn standard_bikes(&self) -> Option<i32> {
        self.0.standard_bikes_count
    }

    async fn empty_docks(&self) -> Option<i32> {
        self.0.empty_docks
    }

    async fn total_docks(&self) -> Option<i32> {
        self.0.total_docks
    }
}

/// Great-circle distance in metres.
///
/// Bike points are compared over a few hundred metres in central London, where
/// treating the earth as a sphere is accurate to well under a metre.
pub fn distance_metres(from: (f64, f64), to: (f64, f64)) -> f64 {
    const EARTH_RADIUS_M: f64 = 6_371_000.0;

    let (lat1, lon1) = (from.0.to_radians(), from.1.to_radians());
    let (lat2, lon2) = (to.0.to_radians(), to.1.to_radians());
    let (dlat, dlon) = (lat2 - lat1, lon2 - lon1);

    let a = (dlat / 2.0).sin().powi(2) + lat1.cos() * lat2.cos() * (dlon / 2.0).sin().powi(2);
    2.0 * a.sqrt().asin() * EARTH_RADIUS_M
}

#[cfg(test)]
mod tests {
    use super::*;

    fn bike_point(properties: &[(&str, &str)]) -> BikePoint {
        BikePoint::new(models::Place {
            id: Some("BikePoints_1".into()),
            additional_properties: Some(
                properties
                    .iter()
                    .map(|(key, value)| models::AdditionalProperties {
                        key: Some((*key).into()),
                        value: Some((*value).into()),
                        ..Default::default()
                    })
                    .collect(),
            ),
            ..Default::default()
        })
    }

    #[test]
    fn reads_counts_out_of_the_property_bag() {
        let point = bike_point(&[
            ("NbBikes", "4"),
            ("NbEBikes", "1"),
            ("NbStandardBikes", "3"),
            ("NbEmptyDocks", "13"),
            ("NbDocks", "19"),
            ("Installed", "true"),
            ("Locked", "false"),
        ]);

        assert_eq!(point.count("NbBikes"), Some(4));
        assert_eq!(point.count("NbEBikes"), Some(1));
        assert_eq!(point.count("NbEmptyDocks"), Some(13));
        assert_eq!(point.flag("Installed"), Some(true));
        assert_eq!(point.flag("Locked"), Some(false));
    }

    #[test]
    fn missing_or_unparseable_properties_are_null_not_zero() {
        // Zero bikes and "TfL did not say" are different answers, and only one
        // of them means do not walk there.
        let point = bike_point(&[("NbBikes", "not a number")]);
        assert_eq!(point.count("NbBikes"), None);
        assert_eq!(point.count("NbEmptyDocks"), None);
        assert!(!point.has_bikes());
    }

    #[test]
    fn a_locked_station_has_nothing_available_whatever_it_reports() {
        // The failure this prevents: TfL leaves the last-synced counts on a
        // station pulled for maintenance, so it reports bikes it cannot give
        // you. Trusting the count sends someone to a locked dock.
        let locked = bike_point(&[("NbBikes", "4"), ("NbEmptyDocks", "9"), ("Locked", "true")]);
        assert!(!locked.has_bikes());
        assert!(!locked.has_docks());
        assert_eq!(
            locked.count("NbBikes"),
            Some(4),
            "the raw count is still reported"
        );

        let uninstalled = bike_point(&[
            ("NbBikes", "4"),
            ("NbEmptyDocks", "9"),
            ("Installed", "false"),
        ]);
        assert!(!uninstalled.has_bikes());

        // A station that says nothing about its state is assumed fine; most
        // do not report these flags at all.
        let ordinary = bike_point(&[("NbBikes", "4"), ("NbEmptyDocks", "9")]);
        assert!(ordinary.has_bikes());
        assert!(ordinary.has_docks());
    }

    #[test]
    fn measures_distance_between_docking_stations() {
        // River Street, Clerkenwell to a point ~1km away.
        let river_street = (51.529163, -0.10997);
        let nearby = (51.538163, -0.10997);
        let metres = distance_metres(river_street, nearby);
        assert!(
            (metres - 1000.0).abs() < 20.0,
            "expected about a kilometre, got {metres}"
        );
        assert_eq!(distance_metres(river_street, river_street), 0.0);
    }
}
