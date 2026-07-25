//! Journey planning.
//!
//! `journey(from: "Kings Cross", to: "Brixton")` is the question people
//! actually have, and it is the one part of TfL's API that will not take a
//! plain name without complaint: anything ambiguous comes back as `300 Multiple
//! Choices` listing the candidates, which the generated client surfaces as
//! [`tfl_api_client::Error::Ambiguous`].
//!
//! Rather than fail, [`JourneyPlan`] carries both outcomes. An ambiguous
//! request returns its candidate locations with the exact value to pass back,
//! so resolving it is one more query rather than a dead end.
//!
//! TfL's own candidates are not enough on their own. Asked to disambiguate
//! "kings cross" it offers eleven points of interest and four bus stops —
//! Kings Cross Tandoori, the Comfort Inn — and not the station, which makes the
//! obvious request unanswerable from the options given. So the ambiguous case
//! also searches stop points for the same term and puts those first: someone
//! planning a journey to a place named after a station almost always means the
//! station.

use async_graphql::{Object, SimpleObject};
use serde::Deserialize;
use tfl_api_client::models;

/// The result of planning a journey: either routes, or the question of which
/// place you meant.
pub struct JourneyPlan {
    pub(crate) journeys: Vec<models::JourneyPlannerJourney>,
    pub(crate) from_options: Vec<LocationOption>,
    pub(crate) to_options: Vec<LocationOption>,
}

#[Object]
impl JourneyPlan {
    /// Routes found, fastest first. Empty when the request was ambiguous —
    /// check `fromOptions` and `toOptions`.
    async fn journeys(&self) -> Vec<Journey> {
        self.journeys.iter().cloned().map(Journey).collect()
    }

    /// True when TfL could not tell which place was meant and returned
    /// candidates instead of routes.
    async fn is_ambiguous(&self) -> bool {
        !self.from_options.is_empty() || !self.to_options.is_empty()
    }

    /// Candidate origins when `from` was ambiguous, stations first.
    ///
    /// Re-run the query passing an option's `value` as `from` to resolve it.
    /// The ordering is only a hint: nothing is filtered out, so choose on the
    /// fields rather than taking the first.
    async fn from_options(&self) -> &[LocationOption] {
        &self.from_options
    }

    /// Candidate destinations, best match first, when `to` was ambiguous.
    async fn to_options(&self) -> &[LocationOption] {
        &self.to_options
    }
}

/// One candidate place for an ambiguous `from` or `to`.
#[derive(SimpleObject, Clone)]
#[graphql(complex)]
pub struct LocationOption {
    /// The name as TfL would describe it, e.g. `Victoria Underground Station`.
    pub name: Option<String>,
    /// Pass this straight back as `from` or `to` to plan against this place.
    ///
    /// Often a `lat,lon` pair rather than an id, which is why it is echoed
    /// verbatim rather than reconstructed. Guaranteed to be a value the planner
    /// accepts: where a search matched an interchange hub, which the planner
    /// rejects, this is the station underneath it instead.
    pub value: Option<String>,
    /// e.g. `NaptanMetroStation`, `PointOfInterest`, `Address`.
    pub place_type: Option<String>,
    /// TfL's confidence in the name match, higher is better.
    ///
    /// Only a measure of how well the *text* matched, so it rates a restaurant
    /// named after a station above the station. Weigh it against `isStation`
    /// rather than on its own.
    pub match_quality: Option<i32>,
    pub lat: Option<f64>,
    pub lon: Option<f64>,
    /// Set on options found by searching stop points rather than offered by
    /// TfL's disambiguation.
    #[graphql(skip)]
    pub is_station: bool,
}

#[async_graphql::ComplexObject]
impl LocationOption {
    /// Whether this is a station or stop, as opposed to a shop, hotel or other
    /// point of interest.
    ///
    /// Stations are listed first, but that is a hint and not a decision —
    /// every candidate TfL offered is returned, so pick whichever the person
    /// actually meant. They really do sometimes mean the restaurant.
    async fn is_station(&self) -> bool {
        self.is_station
    }
}

/// One planned route from origin to destination.
#[derive(Clone)]
pub struct Journey(pub models::JourneyPlannerJourney);

#[Object]
impl Journey {
    /// Total door-to-door time in minutes.
    async fn duration(&self) -> Option<i32> {
        self.0.duration
    }

    /// When to set off, as an ISO 8601 local time.
    async fn start_date_time(&self) -> Option<&str> {
        self.0.start_date_time.as_deref()
    }

    /// When you would arrive, as an ISO 8601 local time.
    async fn arrival_date_time(&self) -> Option<&str> {
        self.0.arrival_date_time.as_deref()
    }

    /// The legs in order — walk, then tube, then walk, and so on.
    async fn legs(&self) -> Vec<Leg> {
        self.0
            .legs
            .clone()
            .unwrap_or_default()
            .into_iter()
            .map(Leg)
            .collect()
    }

    /// How many changes this route involves, derived from the legs that are
    /// not on foot.
    async fn changes(&self) -> usize {
        self.0
            .legs
            .iter()
            .flatten()
            .filter(|leg| {
                leg.mode
                    .as_ref()
                    .and_then(|m| m.id.as_deref())
                    .is_some_and(|id| id != "walking")
            })
            .count()
            .saturating_sub(1)
    }

    /// Whether TfL offered this as an alternative rather than a main
    /// suggestion.
    async fn is_alternative(&self) -> Option<bool> {
        self.0.alternative_route
    }

    /// The fare, when TfL priced this route.
    ///
    /// Only returned for journeys it can price — a walk costs nothing and has
    /// no fare, and cash fares are not offered at all.
    async fn fare(&self) -> Option<Fare> {
        self.0.fare.clone().map(Fare)
    }
}

/// One stage of a journey.
pub struct Leg(pub models::Leg);

#[Object]
impl Leg {
    /// How this leg is travelled, e.g. `walking`, `tube`, `bus`.
    async fn mode(&self) -> Option<&str> {
        self.0.mode.as_ref()?.id.as_deref()
    }

    /// Minutes for this leg alone.
    async fn duration(&self) -> Option<i32> {
        self.0.duration
    }

    /// What to actually do, e.g. `Victoria line towards Brixton`.
    async fn summary(&self) -> Option<&str> {
        self.0.instruction.as_ref()?.summary.as_deref()
    }

    /// The longer form, where TfL gives one.
    async fn detail(&self) -> Option<&str> {
        self.0.instruction.as_ref()?.detailed.as_deref()
    }

    async fn departure_time(&self) -> Option<&str> {
        self.0.departure_time.as_deref()
    }

    async fn arrival_time(&self) -> Option<&str> {
        self.0.arrival_time.as_deref()
    }

    /// Metres covered. Present on walking and cycling legs.
    async fn distance(&self) -> Option<f64> {
        self.0.distance
    }

    /// Whether a disruption affects this leg.
    async fn is_disrupted(&self) -> Option<bool> {
        self.0.is_disrupted
    }

    /// Disruptions affecting this leg. Free — already in the payload.
    async fn disruptions(&self) -> Vec<super::types::Disruption> {
        self.0
            .disruptions
            .clone()
            .unwrap_or_default()
            .into_iter()
            .map(super::types::Disruption)
            .collect()
    }

    /// Steps and stairs on this leg, where TfL knows about them. Relevant to
    /// step-free routing.
    async fn obstacles(&self) -> Vec<Obstacle> {
        self.0
            .obstacles
            .clone()
            .unwrap_or_default()
            .into_iter()
            .map(Obstacle)
            .collect()
    }
}

/// Something in the way on a leg — stairs, an escalator, a lift.
pub struct Obstacle(pub models::Obstacle);

#[Object]
impl Obstacle {
    /// e.g. `Stairs`, `Escalator`, `Lift`.
    async fn kind(&self) -> Option<&str> {
        self.0.r#type.as_deref()
    }

    /// `Up`, `Down`, or `Level`.
    async fn incline(&self) -> Option<&str> {
        self.0.incline.as_deref()
    }

    /// Where on the leg it sits, e.g. `Start`, `Middle`, `End`.
    async fn position(&self) -> Option<&str> {
        self.0.position.as_deref()
    }
}

/// What a journey costs.
pub struct Fare(pub models::JourneyFare);

#[Object]
impl Fare {
    /// Total in pence, for the cheapest applicable fare.
    async fn total_cost(&self) -> Option<i32> {
        self.0.total_cost
    }

    /// Peak price in pence, when the journey has one.
    async fn peak(&self) -> Option<i32> {
        self.0.fares.as_ref()?.first()?.peak
    }

    /// Off-peak price in pence.
    async fn off_peak(&self) -> Option<i32> {
        self.0.fares.as_ref()?.first()?.off_peak
    }

    /// Whether the Hopper fare applies — a second bus or tram within an hour
    /// at no extra cost.
    async fn is_hopper_fare(&self) -> Option<bool> {
        self.0.fares.as_ref()?.first()?.is_hopper_fare
    }

    /// Conditions TfL attaches to the price, e.g. that railcards are not
    /// applied.
    async fn caveats(&self) -> Vec<String> {
        self.0
            .caveats
            .iter()
            .flatten()
            .filter_map(|c| c.text.clone())
            .collect()
    }
}

// -------------------------------------------------- the 300 response

/// TfL's `300 Multiple Choices` body.
///
/// Hand-written because the Swagger document does not describe this response
/// at all. Its `Disambiguation` definition exists but has different fields
/// (`description`/`uri`) from what TfL actually sends, so the generated type
/// would silently decode to nothing.
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct DisambiguationBody {
    from_location_disambiguation: Option<Disambiguation>,
    to_location_disambiguation: Option<Disambiguation>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Disambiguation {
    #[serde(default)]
    disambiguation_options: Vec<Option_>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Option_ {
    parameter_value: Option<String>,
    match_quality: Option<i32>,
    place: Option<DisambiguatedPlace>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct DisambiguatedPlace {
    common_name: Option<String>,
    place_type: Option<String>,
    lat: Option<f64>,
    lon: Option<f64>,
}

impl JourneyPlan {
    pub fn from_itinerary(result: models::ItineraryResult) -> Self {
        Self {
            journeys: result.journeys.unwrap_or_default(),
            from_options: Vec::new(),
            to_options: Vec::new(),
        }
    }

    /// Parses TfL's `300` body into candidate locations.
    ///
    /// A body that cannot be parsed yields no options rather than an error:
    /// the caller still gets an empty plan flagged ambiguous, which is more
    /// useful than a decoding failure.
    /// Adds stop points found by searching, then re-ranks.
    pub fn with_station_options(
        mut self,
        from: Vec<LocationOption>,
        to: Vec<LocationOption>,
    ) -> Self {
        if !self.from_options.is_empty() {
            self.from_options.extend(from);
            sort_and_truncate(&mut self.from_options, MAX_OPTIONS);
        }
        if !self.to_options.is_empty() {
            self.to_options.extend(to);
            sort_and_truncate(&mut self.to_options, MAX_OPTIONS);
        }
        self
    }

    /// Whether `from` was the ambiguous side.
    pub fn from_was_ambiguous(&self) -> bool {
        !self.from_options.is_empty()
    }

    /// Whether `to` was the ambiguous side.
    pub fn to_was_ambiguous(&self) -> bool {
        !self.to_options.is_empty()
    }

    pub fn from_ambiguous(body: &str) -> Self {
        let parsed: DisambiguationBody = match serde_json::from_str(body) {
            Ok(parsed) => parsed,
            Err(error) => {
                tracing::warn!(%error, "could not parse TfL's disambiguation response");
                return Self {
                    journeys: Vec::new(),
                    from_options: Vec::new(),
                    to_options: Vec::new(),
                };
            }
        };
        Self {
            journeys: Vec::new(),
            from_options: options(parsed.from_location_disambiguation),
            to_options: options(parsed.to_location_disambiguation),
        }
    }
}

/// Flattens TfL's disambiguation into candidates, best match first.
///
/// The cap is a guard against TfL returning every address on a street, not an
/// editorial judgement: choosing between candidates is the caller's job and
/// dropping one it might have wanted is the one thing this must not do. It sits
/// far above what TfL returns in practice — around fifteen for a typical
/// ambiguous name.
fn options(disambiguation: Option<Disambiguation>) -> Vec<LocationOption> {
    let mut options: Vec<LocationOption> = disambiguation
        .map(|d| d.disambiguation_options)
        .unwrap_or_default()
        .into_iter()
        .map(|option| {
            let place = option.place;
            let place_type = place.as_ref().and_then(|p| p.place_type.clone());
            LocationOption {
                name: place.as_ref().and_then(|p| p.common_name.clone()),
                value: option.parameter_value,
                is_station: is_transport_place(place_type.as_deref()),
                place_type,
                match_quality: option.match_quality,
                lat: place.as_ref().and_then(|p| p.lat),
                lon: place.as_ref().and_then(|p| p.lon),
            }
        })
        .collect();

    sort_and_truncate(&mut options, MAX_OPTIONS);
    options
}

/// An id the journey planner will actually accept for this stop.
///
/// Stop point search answers "kings cross" with the interchange hub `HUBKGX`,
/// and the journey planner rejects hubs with another `300` — so offering one
/// as a candidate produces an option that cannot resolve the very ambiguity it
/// was offered for. The hub's children include the station itself, so a hub is
/// swapped for the most useful one.
///
/// NaPTAN prefixes are the only thing distinguishing them: `940` is London
/// Underground, `910` National Rail, `490` a bus stop. A station beats a single
/// bus stop, which could be one of a dozen on the same street.
fn planner_id(stop: &models::StopPoint) -> Option<String> {
    let id = stop.naptan_id.clone().or_else(|| stop.id.clone())?;
    if !id.starts_with("HUB") {
        return Some(id);
    }

    let children = stop
        .children
        .iter()
        .flatten()
        .filter_map(|c| c.id.as_deref());
    let mut best: Option<&str> = None;
    for child in children {
        let rank = |id: &str| match () {
            _ if id.starts_with("940") => 3, // Underground / DLR
            _ if id.starts_with("910") => 2, // National Rail
            _ if id.starts_with("HUB") => 0,
            _ => 1,
        };
        if best.is_none_or(|current| rank(child) > rank(current)) {
            best = Some(child);
        }
    }
    // A hub with no usable child is dropped rather than offered: an option
    // that cannot be passed back is worse than one fewer option.
    best.map(str::to_string)
}

fn is_transport_place(place_type: Option<&str>) -> bool {
    place_type.is_some_and(|t| t.starts_with("Naptan") || t == "StopPoint")
}

/// Ranks stations above everything else, then by TfL's match quality.
///
/// The point of the journey planner is travel, so a stop that merely matched
/// less well is still a better guess than a perfectly-matching restaurant.
/// How many candidates to return before assuming TfL is listing a whole street.
pub(crate) const MAX_OPTIONS: usize = 50;

pub(crate) fn sort_and_truncate(options: &mut Vec<LocationOption>, max: usize) {
    options.sort_by_key(|o| {
        (
            std::cmp::Reverse(o.is_station),
            std::cmp::Reverse(o.match_quality.unwrap_or(0)),
        )
    });
    options.truncate(max);
}

impl LocationOption {
    /// Builds an option from a stop point found by searching.
    ///
    /// `match_quality` is set above anything TfL's disambiguation returns so
    /// these sort first among stations; TfL's own scale tops out at 1000.
    pub fn from_stop_point(stop: &models::StopPoint) -> Option<Self> {
        let id = planner_id(stop)?;
        Some(Self {
            name: stop.common_name.clone(),
            value: Some(id),
            place_type: stop.stop_type.clone(),
            match_quality: Some(1001),
            lat: stop.lat,
            lon: stop.lon,
            is_station: true,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_the_shape_tfl_actually_sends() {
        // Fields taken from a real 300 response. The generated Disambiguation
        // type describes `description`/`uri`, which appear nowhere here.
        let body = r#"{
            "toLocationDisambiguation": {
              "matchStatus": "list",
              "disambiguationOptions": [
                { "matchQuality": 900, "parameterValue": "1000248",
                  "place": { "commonName": "Victoria Station", "placeType": "NaptanMetroStation",
                             "lat": 51.49, "lon": -0.14 } },
                { "matchQuality": 1000, "parameterValue": "51.57,-0.08",
                  "place": { "commonName": "Haringey, Victoria", "placeType": "PointOfInterest" } }
              ]
            }
        }"#;

        let plan = JourneyPlan::from_ambiguous(body);
        assert!(plan.from_options.is_empty());
        assert_eq!(plan.to_options.len(), 2);

        // The station leads despite scoring lower: TfL rates "Haringey,
        // Victoria" a perfect 1000 and the actual station 900, which is how
        // asking for a journey to Victoria ends up offering a neighbourhood.
        assert!(plan.to_options[0].is_station);
        assert_eq!(plan.to_options[0].match_quality, Some(900));
        assert_eq!(
            plan.to_options[0].value.as_deref(),
            Some("1000248"),
            "the value must be echoed verbatim so it can be passed back"
        );
        assert!(!plan.to_options[1].is_station);
    }

    #[test]
    fn nothing_the_caller_might_want_is_filtered_out() {
        // Ranking is a hint; choosing is the caller's job. A restaurant that
        // outscored the station must still be reachable, because sometimes the
        // restaurant is what was meant.
        let body = r#"{
            "toLocationDisambiguation": { "disambiguationOptions": [
              { "matchQuality": 1000, "parameterValue": "51.5,-0.1",
                "place": { "commonName": "Kings Cross Tandoori", "placeType": "PointOfInterest" } },
              { "matchQuality": 900, "parameterValue": "940GZZLUKSX",
                "place": { "commonName": "King's Cross", "placeType": "NaptanMetroStation" } }
            ] }
        }"#;

        let plan = JourneyPlan::from_ambiguous(body);
        assert_eq!(plan.to_options.len(), 2, "no candidate should be dropped");
        assert!(plan.to_options[0].is_station, "the station is ranked first");
        assert!(
            plan.to_options
                .iter()
                .any(|o| o.name.as_deref() == Some("Kings Cross Tandoori")),
            "the restaurant must still be selectable"
        );
    }

    #[test]
    fn hubs_are_swapped_for_a_child_the_planner_accepts() {
        let hub = models::StopPoint {
            naptan_id: Some("HUBKGX".into()),
            children: Some(vec![
                models::Place {
                    id: Some("490000129D".into()),
                    ..Default::default()
                },
                models::Place {
                    id: Some("940GZZLUKSX".into()),
                    ..Default::default()
                },
                models::Place {
                    id: Some("910GKNGX".into()),
                    ..Default::default()
                },
            ]),
            ..Default::default()
        };
        // The Underground station, not the bus stop that happened to come first.
        assert_eq!(planner_id(&hub).as_deref(), Some("940GZZLUKSX"));

        let plain = models::StopPoint {
            naptan_id: Some("940GZZLUVIC".into()),
            ..Default::default()
        };
        assert_eq!(planner_id(&plain).as_deref(), Some("940GZZLUVIC"));

        let useless_hub = models::StopPoint {
            naptan_id: Some("HUBXXX".into()),
            ..Default::default()
        };
        assert_eq!(planner_id(&useless_hub), None);
    }

    #[test]
    fn an_unparseable_body_is_not_an_error() {
        let plan = JourneyPlan::from_ambiguous("not json");
        assert!(plan.journeys.is_empty());
        assert!(plan.to_options.is_empty());
    }
}
