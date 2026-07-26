//! Electric-vehicle charge points, car parks, and road casualty history.
//!
//! Grouped because they share a problem: TfL will only give you all of
//! something, so the filtering that makes them answerable happens here.
//! Accidents are the extreme case — one year, thirty-seven megabytes — and are
//! deliberately awkward to ask for as a result.

use async_graphql::{Context, Object, Result, SimpleObject};
use tfl_api_client::models;

use super::{
    bike::distance_metres,
    loaders::{fetched, loaders, to_gql_error},
};

/// An electric-vehicle charging connector.
pub struct ChargeConnector(pub models::ChargeConnectorOccupancy);

#[Object]
impl ChargeConnector {
    /// TfL's numeric id for the connector.
    async fn id(&self) -> Option<String> {
        self.0.id.map(|id| id.to_string())
    }

    /// The operator's own identifier, e.g. `ChargePointESB-UT04EB-1`, which
    /// usually encodes the site and bay.
    async fn source_id(&self) -> Option<&str> {
        self.0.source_system_place_id.as_deref()
    }

    /// `Available`, `Charging`, `Out of service` and so on, straight from TfL.
    async fn status(&self) -> Option<&str> {
        self.0.status.as_deref()
    }

    /// Whether it can be plugged into right now.
    ///
    /// Anything TfL does not call `Available` counts as unavailable — an
    /// unrecognised status is not a reason to send someone across London.
    async fn is_available(&self) -> bool {
        self.0
            .status
            .as_deref()
            .is_some_and(|s| s.eq_ignore_ascii_case("Available"))
    }
}

/// How full a car park is.
pub struct CarPark(pub models::CarParkOccupancy);

#[Object]
impl CarPark {
    async fn id(&self) -> Option<&str> {
        self.0.id.as_deref()
    }

    async fn name(&self) -> Option<&str> {
        self.0.name.as_deref()
    }

    /// Bays free across the whole car park.
    async fn free_spaces(&self) -> Option<i32> {
        Some(bays(&self.0).map(|b| b.free).sum())
    }

    /// Bays in total.
    async fn total_spaces(&self) -> Option<i32> {
        Some(bays(&self.0).map(|b| b.total).sum())
    }

    /// Whether anywhere is free.
    async fn has_spaces(&self) -> bool {
        bays(&self.0).any(|b| b.free > 0)
    }

    /// Free and total bays broken down by type — disabled bays and electric
    /// charging bays are counted separately from ordinary ones.
    async fn bay_types(&self) -> Vec<BayType> {
        self.0
            .bays
            .iter()
            .flatten()
            .map(|bay| BayType {
                name: bay.bay_type.clone(),
                free: bay.free.unwrap_or(0),
                total: bay.bay_count.unwrap_or(0),
            })
            .collect()
    }
}

/// One category of parking bay.
#[derive(SimpleObject)]
pub struct BayType {
    /// e.g. `Pay and Display Parking`, `Disabled Parking`.
    pub name: Option<String>,
    pub free: i32,
    pub total: i32,
}

struct Counted {
    free: i32,
    total: i32,
}

fn bays(park: &models::CarParkOccupancy) -> impl Iterator<Item = Counted> + '_ {
    park.bays.iter().flatten().map(|bay| Counted {
        free: bay.free.unwrap_or(0),
        total: bay.bay_count.unwrap_or(0),
    })
}

// ---------------------------------------------------------------- accidents

/// A recorded road collision.
pub struct Accident {
    pub(crate) inner: models::AccidentDetail,
    /// Metres from the searched coordinate, computed at search time because
    /// TfL cannot filter this feed by location.
    pub(crate) distance: Option<f64>,
}

#[Object]
impl Accident {
    async fn id(&self) -> Option<i32> {
        self.inner.id
    }

    /// When it happened, as an ISO 8601 timestamp.
    async fn date(&self) -> Option<&str> {
        self.inner.date.as_deref()
    }

    /// Where, described rather than coded, e.g.
    /// `On Park Lane 20 metres north of The Junction With south Street`.
    async fn location(&self) -> Option<&str> {
        self.inner.location.as_deref()
    }

    /// `Slight`, `Serious` or `Fatal`.
    async fn severity(&self) -> Option<&str> {
        self.inner.severity.as_deref()
    }

    async fn borough(&self) -> Option<&str> {
        self.inner.borough.as_deref()
    }

    async fn lat(&self) -> Option<f64> {
        self.inner.lat
    }

    async fn lon(&self) -> Option<f64> {
        self.inner.lon
    }

    /// Metres from the searched coordinate. Null unless `lat`/`lon` were given.
    async fn distance(&self) -> Option<f64> {
        self.distance
    }

    /// Who was hurt, and how badly.
    async fn casualties(&self) -> Vec<Casualty> {
        self.inner
            .casualties
            .iter()
            .flatten()
            .map(|c| Casualty {
                age: c.age,
                age_band: c.age_band.clone(),
                class: c.class.clone(),
                severity: c.severity.clone(),
                mode: c.mode.clone(),
            })
            .collect()
    }

    /// The kinds of vehicle involved, e.g. `Car`, `Pedal Cycle`.
    async fn vehicles(&self) -> Vec<String> {
        self.inner
            .vehicles
            .iter()
            .flatten()
            .filter_map(|v| v.r#type.clone())
            .collect()
    }
}

/// A person hurt in a collision.
#[derive(SimpleObject)]
pub struct Casualty {
    pub age: Option<i32>,
    /// e.g. `Adult`, `Child`.
    pub age_band: Option<String>,
    /// What they were doing: `Driver`, `Pedestrian`, `Passenger`.
    pub class: Option<String>,
    /// `Slight`, `Serious` or `Fatal`.
    pub severity: Option<String>,
    /// How they were travelling, e.g. `Car`, `PedalCycle`.
    pub mode: Option<String>,
}

/// Fetches a year of casualty data and filters it locally.
///
/// TfL offers no filter of any kind on this endpoint: the only request possible
/// is "give me every collision in this year", which for 2019 is fifty thousand
/// records and thirty-seven megabytes. Everything below therefore happens after
/// the download, and the loader keeps it to one download per request no matter
/// how many accident fields a query touches.
pub async fn search(
    ctx: &Context<'_>,
    year: i32,
    near: Option<(f64, f64)>,
    radius: f64,
    severities: &[String],
    borough: Option<&str>,
    first: usize,
) -> Result<Vec<Accident>> {
    let all = fetched(
        loaders(ctx)
            .accidents
            .load_one(year)
            .await
            .map_err(to_gql_error)?,
    )?;

    let mut matched: Vec<(f64, models::AccidentDetail)> = Vec::new();
    for accident in all {
        if !severities.is_empty()
            && !severities.iter().any(|s| {
                accident
                    .severity
                    .as_deref()
                    .is_some_and(|a| a.eq_ignore_ascii_case(s))
            })
        {
            continue;
        }
        if let Some(borough) = borough
            && !accident
                .borough
                .as_deref()
                .is_some_and(|b| b.to_lowercase().contains(&borough.to_lowercase()))
        {
            continue;
        }
        let rank = match (near, accident.lat, accident.lon) {
            (Some(origin), Some(lat), Some(lon)) => {
                let metres = distance_metres(origin, (lat, lon));
                if metres > radius {
                    continue;
                }
                metres
            }
            (Some(_), _, _) => continue,
            // No location to rank by. Ranking everything equally would make
            // `first` an arbitrary slice of TfL's own ordering while the field
            // claimed to be sorted, so the date is used instead and the
            // description says so.
            (None, _, _) => f64::NAN,
        };
        matched.push((rank, accident));
    }

    if near.is_some() {
        matched.sort_by(|a, b| a.0.total_cmp(&b.0));
    } else {
        // Most recent first. String comparison is the right one here: TfL's
        // dates are ISO 8601 in a fixed zone, so lexical order is chronological.
        matched.sort_by(|a, b| b.1.date.cmp(&a.1.date));
    }
    matched.truncate(first);
    Ok(matched
        .into_iter()
        .map(|(rank, inner)| Accident {
            inner,
            distance: near.map(|_| rank),
        })
        .collect())
}
