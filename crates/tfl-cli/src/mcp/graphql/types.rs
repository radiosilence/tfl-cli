//! The graph.
//!
//! TfL's REST payloads are flat records full of foreign keys — a `Prediction`
//! knows it belongs to `lineId: "victoria"` and terminates at
//! `destinationNaptanId: "940GZZLUBXN"`, but following either is another
//! request you have to know how to make. These types keep the scalar fields as
//! TfL sends them and turn every one of those keys into an edge, so a question
//! like "what is arriving at Kings Cross, on which line, going where" is one
//! query instead of a fetch-and-loop.
//!
//! Each edge resolves through a [`super::loaders`] DataLoader, so following the
//! same edge across a list costs one request rather than one per item. Field
//! documentation says what each one costs; that text is the SDL description an
//! agent reads before querying.
//!
//! Almost every field is nullable. TfL's spec marks nearly nothing as required
//! and means it — a live `/StopPoint` response carries about half the fields it
//! declares — so absence is normal rather than an error.

use async_graphql::{ComplexObject, Context, Object, Result, SimpleObject};
use tfl_api_client::models;

use super::loaders::{loaders, to_gql_error};

/// A London Underground, Overground, Elizabeth line, DLR, tram, bus or river
/// service.
pub struct Line(pub models::Line);

#[Object]
impl Line {
    /// TfL's identifier, e.g. `victoria` or `elizabeth`. Use this anywhere a
    /// line id is asked for.
    async fn id(&self) -> Option<&str> {
        self.0.id.as_deref()
    }

    /// Display name, e.g. `Victoria`.
    async fn name(&self) -> Option<&str> {
        self.0.name.as_deref()
    }

    /// The mode this line runs on, e.g. `tube`, `bus`, `elizabeth-line`.
    async fn mode(&self) -> Option<&str> {
        self.0.mode_name.as_deref()
    }

    /// Current service status — the "good service" / "severe delays" a
    /// passenger would recognise.
    ///
    /// Free: every route into a line fetches status with it.
    async fn statuses(&self) -> Vec<LineStatus> {
        self.0
            .line_statuses
            .clone()
            .unwrap_or_default()
            .into_iter()
            .map(LineStatus)
            .collect()
    }

    /// Whether the line is running normally.
    ///
    /// Derived from [`Self::statuses`]: TfL severity 10 is "Good Service".
    /// Null when no status was returned.
    async fn is_good_service(&self) -> Option<bool> {
        let statuses = self.0.line_statuses.as_ref()?;
        if statuses.is_empty() {
            return None;
        }
        Some(statuses.iter().all(|s| s.status_severity == Some(10)))
    }

    /// Disruptions currently affecting this line.
    ///
    /// Costs one request per line, batched across every line in the query.
    async fn disruptions(&self, ctx: &Context<'_>) -> Result<Vec<Disruption>> {
        let Some(id) = self.0.id.clone() else {
            return Ok(Vec::new());
        };
        let loaded = loaders(ctx)
            .disruption
            .load_one(id)
            .await
            .map_err(to_gql_error)?;
        Ok(loaded
            .unwrap_or_default()
            .into_iter()
            .map(Disruption)
            .collect())
    }

    /// Every stop this line serves, in no particular order.
    ///
    /// Costs one request per line and returns a lot — the Central line alone is
    /// 49 stops. Ask for `stopPoints { id commonName }` rather than whole stop
    /// points unless you need the detail.
    async fn stop_points(&self, ctx: &Context<'_>) -> Result<Vec<StopPoint>> {
        let Some(id) = self.0.id.clone() else {
            return Ok(Vec::new());
        };
        let loaded = loaders(ctx)
            .line_stop_points
            .load_one(id)
            .await
            .map_err(to_gql_error)?;
        Ok(loaded
            .unwrap_or_default()
            .into_iter()
            .map(StopPoint::new)
            .collect())
    }
}

/// A station, stop, platform or pier.
///
/// TfL calls everything on the network a stop point and arranges them in a
/// hierarchy: a station like Kings Cross is a parent whose children are its
/// individual platforms, each with its own NaPTAN id.
pub struct StopPoint {
    pub(crate) inner: models::StopPoint,
    /// The id this stop point was looked up by.
    ///
    /// TfL substitutes the interchange hub for a stop that belongs to one:
    /// asking for `940GZZLUKSX` returns `HUBKGX`. The hub is a fine answer for
    /// a name or a location, but it has no arrivals of its own — the platforms
    /// underneath it do — so the id that was actually asked for is kept and
    /// used for anything that would otherwise silently come back empty.
    pub(crate) requested_id: Option<String>,
}

impl StopPoint {
    pub fn new(inner: models::StopPoint) -> Self {
        Self {
            inner,
            requested_id: None,
        }
    }

    /// Wraps a stop point that was looked up by a known id.
    pub fn requested(id: impl Into<String>, inner: models::StopPoint) -> Self {
        Self {
            inner,
            requested_id: Some(id.into()),
        }
    }

    /// The id to use when asking TfL anything else about this stop.
    fn lookup_id(&self) -> Option<String> {
        self.requested_id
            .clone()
            .or_else(|| self.inner.naptan_id.clone())
            .or_else(|| self.inner.id.clone())
    }
}

#[Object]
impl StopPoint {
    /// NaPTAN identifier, e.g. `940GZZLUKSX`. Stable, and what every other
    /// stop-point field expects.
    async fn id(&self) -> Option<String> {
        self.lookup_id()
    }

    /// The name a passenger would use, e.g. `King's Cross St. Pancras
    /// Underground Station`.
    async fn common_name(&self) -> Option<&str> {
        self.inner.common_name.as_deref()
    }

    /// Modes served here, e.g. `["tube", "national-rail"]`.
    async fn modes(&self) -> Vec<String> {
        self.inner.modes.clone().unwrap_or_default()
    }

    async fn lat(&self) -> Option<f64> {
        self.inner.lat
    }

    async fn lon(&self) -> Option<f64> {
        self.inner.lon
    }

    /// The platform, where this stop point is one — e.g. `Northbound - Platform 1`.
    async fn platform_name(&self) -> Option<&str> {
        self.inner.platform_name.as_deref()
    }

    /// Zone, step-free access and similar, as TfL's untyped property bag.
    async fn properties(&self) -> Vec<Property> {
        self.inner
            .additional_properties
            .clone()
            .unwrap_or_default()
            .into_iter()
            .map(Property)
            .collect()
    }

    /// Distance in metres from the search point, when this stop point came from
    /// [`super::query::QueryRoot::stop_points_near`]. Null otherwise.
    async fn distance(&self) -> Option<f64> {
        self.inner.distance.filter(|d| *d > 0.0)
    }

    /// The lines calling here.
    ///
    /// The ids are already in the payload; asking for anything beyond `id` and
    /// `name` costs one batched request for all lines across the whole query.
    async fn lines(&self, ctx: &Context<'_>) -> Result<Vec<Line>> {
        let identifiers = self.inner.lines.clone().unwrap_or_default();
        let ids: Vec<String> = identifiers.iter().filter_map(|l| l.id.clone()).collect();
        if ids.is_empty() {
            return Ok(Vec::new());
        }
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

    /// Live arrivals, soonest first.
    ///
    /// One request per stop, deduplicated across the query. A busy interchange
    /// returns a lot — Kings Cross is around sixty predictions — so pass
    /// `first` to trim it.
    ///
    /// This is live data and goes stale in seconds; it is never cached.
    async fn arrivals(
        &self,
        ctx: &Context<'_>,
        #[graphql(desc = "Return only the soonest N arrivals.")] first: Option<usize>,
    ) -> Result<Vec<Prediction>> {
        let Some(id) = self.lookup_id() else {
            return Ok(Vec::new());
        };
        let mut arrivals = loaders(ctx)
            .arrivals
            .load_one(id)
            .await
            .map_err(to_gql_error)?
            .unwrap_or_default();

        // TfL returns predictions unordered; soonest-first is what any caller
        // actually wants.
        arrivals.sort_by_key(|p| p.time_to_station.unwrap_or(i32::MAX));
        arrivals.truncate(first.unwrap_or(usize::MAX));
        Ok(arrivals.into_iter().map(Prediction).collect())
    }

    /// Platforms and entrances belonging to this stop point. Free — already in
    /// the payload.
    async fn children(&self) -> Vec<Place> {
        self.inner
            .children
            .clone()
            .unwrap_or_default()
            .into_iter()
            .map(Place)
            .collect()
    }
}

/// A predicted arrival of a specific vehicle at a specific stop.
pub struct Prediction(pub models::Prediction);

#[Object]
impl Prediction {
    /// Seconds until the vehicle arrives. The field to sort or filter on.
    async fn time_to_station(&self) -> Option<i32> {
        self.0.time_to_station
    }

    /// Expected arrival as an ISO 8601 timestamp.
    async fn expected_arrival(&self) -> Option<&str> {
        self.0.expected_arrival.as_deref()
    }

    /// Where the vehicle is heading, as shown on the platform indicator, e.g.
    /// `Brixton`.
    async fn towards(&self) -> Option<&str> {
        self.0.towards.as_deref()
    }

    /// Destination station name.
    async fn destination_name(&self) -> Option<&str> {
        self.0.destination_name.as_deref()
    }

    /// `inbound` or `outbound`.
    async fn direction(&self) -> Option<&str> {
        self.0.direction.as_deref()
    }

    /// The platform to stand on, e.g. `Southbound - Platform 4`.
    async fn platform_name(&self) -> Option<&str> {
        self.0.platform_name.as_deref()
    }

    /// Where the vehicle is right now, in the words TfL puts on the board, e.g.
    /// `At Euston`.
    async fn current_location(&self) -> Option<&str> {
        self.0.current_location.as_deref()
    }

    /// TfL's vehicle identifier — a set number on the tube, a registration on a
    /// bus.
    async fn vehicle_id(&self) -> Option<&str> {
        self.0.vehicle_id.as_deref()
    }

    /// The line this vehicle is running on.
    ///
    /// One batched request for every `line` edge in the query, however many
    /// predictions there are.
    async fn line(&self, ctx: &Context<'_>) -> Result<Option<Line>> {
        load_line(ctx, self.0.line_id.clone()).await
    }

    /// The stop this prediction is for.
    ///
    /// Batched with every other stop point in the query.
    async fn stop_point(&self, ctx: &Context<'_>) -> Result<Option<StopPoint>> {
        load_stop_point(ctx, self.0.naptan_id.clone()).await
    }

    /// The stop this vehicle terminates at.
    ///
    /// Shares a loader with [`Self::stop_point`], so destinations and origins
    /// across a whole list of arrivals resolve together.
    async fn destination(&self, ctx: &Context<'_>) -> Result<Option<StopPoint>> {
        load_stop_point(ctx, self.0.destination_naptan_id.clone()).await
    }
}

/// The status of a line at a point in time.
pub struct LineStatus(pub models::LineStatus);

#[Object]
impl LineStatus {
    /// TfL's severity code. 10 is "Good Service"; lower is worse.
    async fn severity(&self) -> Option<i32> {
        self.0.status_severity
    }

    /// The severity in words, e.g. `Good Service`, `Minor Delays`.
    async fn description(&self) -> Option<&str> {
        self.0.status_severity_description.as_deref()
    }

    /// The passenger-facing explanation, where there is one.
    async fn reason(&self) -> Option<&str> {
        self.0.reason.as_deref()
    }

    /// The disruption behind this status. Free — already in the payload.
    async fn disruption(&self) -> Option<Disruption> {
        self.0.disruption.clone().map(Disruption)
    }

    /// The line this status belongs to.
    async fn line(&self, ctx: &Context<'_>) -> Result<Option<Line>> {
        load_line(ctx, self.0.line_id.clone()).await
    }
}

/// A disruption to the network.
pub struct Disruption(pub models::Disruption);

#[Object]
impl Disruption {
    /// e.g. `RealTime`, `PlannedWork`.
    async fn category(&self) -> Option<&str> {
        self.0.category.as_deref()
    }

    /// One-line summary.
    async fn summary(&self) -> Option<&str> {
        self.0.summary.as_deref()
    }

    /// The full description, usually the longest useful field here.
    async fn description(&self) -> Option<&str> {
        self.0.description.as_deref()
    }

    async fn additional_info(&self) -> Option<&str> {
        self.0.additional_info.as_deref()
    }

    /// When TfL last updated this, as an ISO 8601 timestamp.
    async fn last_update(&self) -> Option<&str> {
        self.0.last_update.as_deref()
    }

    /// Stops affected. Free — already in the payload, though TfL often returns
    /// none even for a disruption that clearly affects stops.
    async fn affected_stops(&self) -> Vec<StopPoint> {
        self.0
            .affected_stops
            .clone()
            .unwrap_or_default()
            .into_iter()
            .map(StopPoint::new)
            .collect()
    }
}

/// A transport mode, e.g. `tube` or `river-bus`.
#[derive(SimpleObject)]
#[graphql(complex)]
pub struct Mode {
    /// The id to pass wherever a mode is accepted, e.g. `tube`.
    pub name: Option<String>,
    /// Whether TfL runs it, as opposed to merely reporting on it.
    pub is_tfl_service: Option<bool>,
    pub is_fare_paying: Option<bool>,
    /// Whether it runs to a timetable, which is what makes arrivals meaningful.
    pub is_scheduled_service: Option<bool>,
}

#[ComplexObject]
impl Mode {
    /// Every line running on this mode, with status.
    ///
    /// One request per mode.
    async fn lines(&self, ctx: &Context<'_>) -> Result<Vec<Line>> {
        let Some(name) = self.name.clone() else {
            return Ok(Vec::new());
        };
        let client = ctx.data_unchecked::<std::sync::Arc<tfl_api_client::Client>>();
        let lines = client
            .line_status_by_mode(&[&name], &Default::default())
            .await
            .map_err(to_gql_error)?;
        Ok(lines.into_iter().map(Line).collect())
    }
}

impl From<models::Mode> for Mode {
    fn from(mode: models::Mode) -> Self {
        Self {
            name: mode.mode_name,
            is_tfl_service: mode.is_tfl_service,
            is_fare_paying: mode.is_fare_paying,
            is_scheduled_service: mode.is_scheduled_service,
        }
    }
}

/// Somewhere on the network that is not a stop — a car park, a docking station,
/// a taxi rank.
pub struct Place(pub models::Place);

#[Object]
impl Place {
    async fn id(&self) -> Option<&str> {
        self.0.id.as_deref()
    }

    async fn common_name(&self) -> Option<&str> {
        self.0.common_name.as_deref()
    }

    /// e.g. `NaptanMetroPlatform`, `CarPark`, `BikePoint`.
    async fn place_type(&self) -> Option<&str> {
        self.0.place_type.as_deref()
    }

    async fn lat(&self) -> Option<f64> {
        self.0.lat
    }

    async fn lon(&self) -> Option<f64> {
        self.0.lon
    }

    /// The untyped property bag. For a bike point this is where dock and bike
    /// counts live.
    async fn properties(&self) -> Vec<Property> {
        self.0
            .additional_properties
            .clone()
            .unwrap_or_default()
            .into_iter()
            .map(Property)
            .collect()
    }
}

/// One entry from TfL's property bag.
///
/// TfL hangs everything that does not fit its schema here — fare zone,
/// step-free access, how many bikes are free at a docking station — as
/// stringly-typed key/value pairs.
pub struct Property(pub models::AdditionalProperties);

#[Object]
impl Property {
    async fn key(&self) -> Option<&str> {
        self.0.key.as_deref()
    }

    async fn value(&self) -> Option<&str> {
        self.0.value.as_deref()
    }

    /// e.g. `Facility`, `Accessibility`, `Description`.
    async fn category(&self) -> Option<&str> {
        self.0.category.as_deref()
    }
}

async fn load_line(ctx: &Context<'_>, id: Option<String>) -> Result<Option<Line>> {
    let Some(id) = id else { return Ok(None) };
    Ok(loaders(ctx)
        .line
        .load_one(id)
        .await
        .map_err(to_gql_error)?
        .map(Line))
}

async fn load_stop_point(ctx: &Context<'_>, id: Option<String>) -> Result<Option<StopPoint>> {
    let Some(id) = id else { return Ok(None) };
    Ok(loaders(ctx)
        .stop_point
        .load_one(id.clone())
        .await
        .map_err(to_gql_error)?
        .map(|stop| StopPoint::requested(id, stop)))
}
