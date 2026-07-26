//! Scheduled departures — first train, last train, and everything between.
//!
//! The endpoint is documented; the way to call it is not. `direction` is a
//! **query parameter** absent from the Swagger document entirely, and the docs
//! imply it is a path segment, which 404s. Without it TfL answers with a
//! disambiguation body — at **HTTP 200**, so it cannot be detected by status —
//! listing the directions it would accept.
//!
//! Times use TfL's `TwentyFourHourClockTime`, where the hour runs past 23 for
//! anything after midnight: the last Friday train from Vauxhall is `27:13`,
//! meaning 03:13 on Saturday. Printing that raw would tell someone about a
//! train at twenty-seven o'clock, so [`Departure`] exposes both the wall-clock
//! time and whether it is the following day.

use async_graphql::Object;
use tfl_api_client::models;

/// A line's schedule from one stop, in one direction.
pub struct Timetable(pub models::TimetableResponse);

#[Object]
impl Timetable {
    /// `inbound` or `outbound`, as resolved.
    async fn direction(&self) -> Option<&str> {
        self.0.direction.as_deref()
    }

    /// True when TfL could not tell which direction was meant and returned
    /// options instead of a schedule.
    ///
    /// Re-query with `direction: "inbound"` or `"outbound"`. Note this arrives
    /// as a normal 200 response, not an error.
    async fn is_ambiguous(&self) -> bool {
        self.0.disambiguation.is_some() && self.0.timetable.is_none()
    }

    /// One entry per day type — `Monday - Thursday`, `Friday`, `Saturday`,
    /// `Sunday`. Empty when ambiguous.
    ///
    /// The names carry bank-holiday meaning: `Saturday (also Good Friday)` is
    /// telling you which schedule a holiday follows.
    async fn schedules(&self) -> Vec<Schedule> {
        self.0
            .timetable
            .iter()
            .flat_map(|t| t.routes.iter().flatten())
            .flat_map(|r| r.schedules.iter().flatten())
            .cloned()
            .map(Schedule)
            .collect()
    }

    /// A link to TfL's own PDF timetable, when there is one.
    async fn pdf_url(&self) -> Option<&str> {
        self.0.pdf_url.as_deref()
    }
}

/// The schedule for one type of day.
pub struct Schedule(pub models::Schedule);

#[Object]
impl Schedule {
    /// e.g. `Monday - Thursday`, `Saturday (also Good Friday)`.
    async fn name(&self) -> Option<&str> {
        self.0.name.as_deref()
    }

    /// The first departure of the day.
    async fn first(&self) -> Option<Departure> {
        self.0.first_journey.clone().map(Departure)
    }

    /// The last departure — what "when is the last train" is asking for.
    ///
    /// Frequently after midnight, in which case `isNextDay` is true and
    /// `time` is the wall-clock time it actually leaves.
    async fn last(&self) -> Option<Departure> {
        self.0.last_journey.clone().map(Departure)
    }

    /// Every scheduled departure. Several hundred on a tube line — ask for
    /// `first` and `last` unless you need the whole day.
    #[graphql(complexity = "child_complexity.saturating_mul(200).saturating_add(10)")]
    async fn departures(&self) -> Vec<Departure> {
        self.0
            .known_journeys
            .clone()
            .unwrap_or_default()
            .into_iter()
            .map(Departure)
            .collect()
    }
}

/// One scheduled departure.
pub struct Departure(pub models::KnownJourney);

#[Object]
impl Departure {
    /// Wall-clock departure, `HH:MM`, e.g. `03:13`.
    ///
    /// Already wrapped past midnight — see [`Self::is_next_day`]. TfL's own
    /// figure would read `27:13`.
    async fn time(&self) -> Option<String> {
        let (hour, minute) = self.parts()?;
        Some(wall_clock(hour, minute))
    }

    /// Whether this departs after midnight, and so on the following calendar
    /// day from the schedule it belongs to.
    ///
    /// The Night Tube is the reason this exists: a Friday schedule's last
    /// train leaves on Saturday morning.
    async fn is_next_day(&self) -> Option<bool> {
        Some(self.parts()?.0 >= 24)
    }

    /// Minutes after midnight, unwrapped, so `27:13` is 1633.
    ///
    /// Sorts and compares correctly across the midnight boundary, which
    /// `time` does not.
    async fn minutes_after_midnight(&self) -> Option<u32> {
        let (hour, minute) = self.parts()?;
        Some(hour * 60 + minute)
    }
}

/// Wraps TfL's past-midnight hour into a real clock time.
fn wall_clock(hour: u32, minute: u32) -> String {
    format!("{:02}:{:02}", hour % 24, minute)
}

impl Departure {
    /// TfL sends the hour and minute as strings, and the hour runs past 23.
    fn parts(&self) -> Option<(u32, u32)> {
        Some((
            self.0.hour.as_deref()?.trim().parse().ok()?,
            self.0.minute.as_deref()?.trim().parse().ok()?,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parts(hour: &str, minute: &str) -> Option<(u32, u32)> {
        Departure(models::KnownJourney {
            hour: Some(hour.into()),
            minute: Some(minute.into()),
            ..Default::default()
        })
        .parts()
    }

    #[test]
    fn a_night_tube_departure_is_not_twenty_seven_oclock() {
        // TfL really does send hour 27 for the last Friday train from
        // Vauxhall. Reporting it raw tells someone about a train at a time
        // that does not exist.
        let (hour, minute) = parts("27", "13").unwrap();
        assert_eq!(wall_clock(hour, minute), "03:13");
        assert!(hour >= 24, "and it leaves on the following day");

        let (hour, minute) = parts("23", "52").unwrap();
        assert_eq!(wall_clock(hour, minute), "23:52");
        assert!(hour < 24);

        // Midnight exactly must not read as 24:34.
        let (hour, minute) = parts("24", "34").unwrap();
        assert_eq!(wall_clock(hour, minute), "00:34");
        assert!(hour >= 24);
    }

    #[test]
    fn the_unwrapped_figure_orders_across_midnight() {
        // 03:13 tomorrow is later than 23:52 tonight, which comparing
        // wall-clock times gets backwards.
        let minutes = |h: &str, m: &str| {
            let (hour, minute) = parts(h, m).unwrap();
            hour * 60 + minute
        };
        assert!(minutes("27", "13") > minutes("23", "52"));
        assert_eq!(minutes("27", "13"), 1633);
    }

    #[test]
    fn nonsense_from_tfl_is_null_rather_than_a_guess() {
        assert_eq!(parts("", ""), None);
        assert_eq!(parts("half past", "three"), None);
    }
}
