//! Roads: the 24 corridors TfL manages, and what is currently wrong with them.
//!
//! The tube half of this API is about *when* — the road half is about *what and
//! where*, so a disruption carries free text a person wrote (`comments`,
//! `currentUpdate`) rather than a timetable. Those two fields are the answer to
//! "why is the A406 a mess", and everything else is metadata about them.

use async_graphql::{Context, Object, Result};
use tfl_api_client::models;

use super::loaders::{fetched, loaders, to_gql_error};

/// One of TfL's managed road corridors, e.g. the A406 or the A2.
pub struct Road(pub models::RoadCorridor);

#[Object]
impl Road {
    /// TfL's identifier, lower case, e.g. `a406`.
    async fn id(&self) -> Option<&str> {
        self.0.id.as_deref()
    }

    /// The name a driver would use, e.g. `A406`.
    async fn display_name(&self) -> Option<&str> {
        self.0.display_name.as_deref()
    }

    /// Current status in words, e.g. `No Exceptional Delays`, `Serious`.
    async fn status(&self) -> Option<&str> {
        self.0.status_severity_description.as_deref()
    }

    /// The same status as TfL's severity band, e.g. `Good`, `Serious`.
    ///
    /// Note this is a word here, unlike a line's numeric severity.
    async fn severity(&self) -> Option<&str> {
        self.0.status_severity.as_deref()
    }

    /// Whether the road is running normally.
    ///
    /// Derived: TfL calls an unobstructed corridor `Good`.
    async fn is_clear(&self) -> Option<bool> {
        Some(self.0.status_severity.as_deref()? == "Good")
    }

    /// Disruptions currently on this road.
    ///
    /// One request per road, batched across every road in the query.
    async fn disruptions(&self, ctx: &Context<'_>) -> Result<Vec<RoadDisruption>> {
        let Some(id) = self.0.id.clone() else {
            return Ok(Vec::new());
        };
        let loaded = loaders(ctx)
            .road_disruption
            .load_one(id)
            .await
            .map_err(to_gql_error)?;
        Ok(fetched(loaded)?.into_iter().map(RoadDisruption).collect())
    }
}

/// Roadworks, a closure, or an incident.
pub struct RoadDisruption(pub models::RoadDisruption);

#[Object]
impl RoadDisruption {
    /// TfL's identifier, e.g. `TIMS-231236`.
    async fn id(&self) -> Option<&str> {
        self.0.id.as_deref()
    }

    /// Where it is, in TfL's own words, e.g.
    /// `[A10] GREAT CAMBRIDGE ROAD (EN1 ) (Enfield)`.
    async fn location(&self) -> Option<&str> {
        self.0.location.as_deref()
    }

    /// What is happening and why — usually the most useful field here.
    async fn comments(&self) -> Option<&str> {
        self.0.comments.as_deref()
    }

    /// How it is going right now, e.g. `Traffic is flowing well.`
    ///
    /// Written by a person and more current than [`Self::comments`], which
    /// describes the works rather than the traffic.
    async fn current_update(&self) -> Option<&str> {
        self.0.current_update.as_deref()
    }

    /// e.g. `Moderate`, `Serious`.
    async fn severity(&self) -> Option<&str> {
        self.0.severity.as_deref()
    }

    /// e.g. `Works`, `Accident`, `Event`.
    async fn category(&self) -> Option<&str> {
        self.0.category.as_deref()
    }

    /// e.g. `TfL works`, `Collision`.
    async fn sub_category(&self) -> Option<&str> {
        self.0.sub_category.as_deref()
    }

    /// Whether the road is actually closed, as opposed to merely slow.
    async fn has_closures(&self) -> Option<bool> {
        self.0.has_closures
    }

    async fn start_date_time(&self) -> Option<&str> {
        self.0.start_date_time.as_deref()
    }

    async fn end_date_time(&self) -> Option<&str> {
        self.0.end_date_time.as_deref()
    }

    /// Roughly where it is, when TfL gave a point.
    async fn lat(&self) -> Option<f64> {
        point(&self.0).map(|(lat, _)| lat)
    }

    async fn lon(&self) -> Option<f64> {
        point(&self.0).map(|(_, lon)| lon)
    }

    /// The roads this affects.
    ///
    /// Batched: every corridor mentioned across the query resolves in one
    /// request.
    async fn roads(&self, ctx: &Context<'_>) -> Result<Vec<Road>> {
        let ids = self.0.corridor_ids.clone().unwrap_or_default();
        if ids.is_empty() {
            return Ok(Vec::new());
        }
        let loaded = loaders(ctx)
            .road
            .load_many(ids.clone())
            .await
            .map_err(to_gql_error)?;
        Ok(ids
            .into_iter()
            .filter_map(|id| loaded.get(&id).cloned())
            .map(Road)
            .collect())
    }
}

/// Pulls a coordinate out of TfL's `point` field.
///
/// It is a string rather than a pair — `"51.5,-0.1"` — and occasionally
/// something else entirely, so anything unparseable is treated as absent.
fn point(disruption: &models::RoadDisruption) -> Option<(f64, f64)> {
    let point = disruption.point.as_deref()?;
    let (lat, lon) = point.split_once(',')?;
    Some((lat.trim().parse().ok()?, lon.trim().parse().ok()?))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_a_coordinate_out_of_tfls_string() {
        let with = |p: Option<&str>| models::RoadDisruption {
            point: p.map(str::to_string),
            ..Default::default()
        };
        assert_eq!(point(&with(Some("51.5,-0.12"))), Some((51.5, -0.12)));
        assert_eq!(point(&with(Some(" 51.5 , -0.12 "))), Some((51.5, -0.12)));

        // Absent or malformed is null, never a coordinate off the coast of
        // Africa — a road disruption plotted at 0,0 is worse than one with no
        // location at all.
        assert_eq!(point(&with(None)), None);
        assert_eq!(point(&with(Some("not a point"))), None);
        assert_eq!(point(&with(Some("51.5"))), None);
        assert_eq!(point(&with(Some("a,b"))), None);
    }
}
