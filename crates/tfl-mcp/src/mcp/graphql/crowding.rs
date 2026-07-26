//! How busy a station is, now and across a normal week.
//!
//! None of this is in TfL's Swagger document. The endpoint the spec *does*
//! describe — `/StopPoint/{id}/Crowding/{line}` — returns a plain stop point
//! with no crowding in it, which is why this looked withdrawn rather than
//! merely undocumented. The real feed lives at `/crowding/{naptan}`, alongside
//! `/Live` and per-day forms, and is written by hand for the same reason air
//! quality and Cabwise are.
//!
//! Every figure is **relative to that station's own normal**, not a headcount.
//! `0.16` means a sixth of the usual traffic for that quarter-hour, so this
//! answers "busier than usual?" and cannot answer "how many people are there".

use async_graphql::Object;
use serde::Deserialize;

/// How busy a station is right now.
#[derive(Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct LiveCrowding {
    pub data_available: Option<bool>,
    pub percentage_of_baseline: Option<f64>,
    pub time_local: Option<String>,
    pub time_utc: Option<String>,
}

#[Object]
impl LiveCrowding {
    /// Whether TfL is publishing a figure for this station at all. Many stops
    /// have no sensors, and a missing figure is not a quiet station.
    async fn data_available(&self) -> bool {
        self.data_available.unwrap_or(false)
    }

    /// How busy it is compared with normal for this time of day.
    ///
    /// `1.0` is a typical level, `0.16` is a sixth of it, `1.4` is half again
    /// as busy. Relative to this station's own baseline — never a count of
    /// people, and not comparable between stations.
    async fn relative_to_normal(&self) -> Option<f64> {
        self.percentage_of_baseline
    }

    /// A plain-language reading of [`Self::relative_to_normal`], for when a
    /// number needs describing rather than reporting.
    async fn description(&self) -> Option<&'static str> {
        Some(match self.percentage_of_baseline? {
            p if p < 0.4 => "much quieter than usual",
            p if p < 0.8 => "quieter than usual",
            p if p <= 1.2 => "about as busy as usual",
            p if p <= 1.6 => "busier than usual",
            _ => "much busier than usual",
        })
    }

    /// When TfL measured it, London local.
    async fn measured_at(&self) -> Option<&str> {
        self.time_local.as_deref()
    }
}

/// A typical day at a station, in quarter-hour bands.
#[derive(Deserialize, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DayCrowding {
    pub naptan: Option<String>,
    pub day_of_week: Option<String>,
    pub am_peak_time_band: Option<String>,
    pub pm_peak_time_band: Option<String>,
    pub is_always_quiet: Option<bool>,
    pub is_found: Option<bool>,
    #[serde(default)]
    pub time_bands: Vec<TimeBand>,
}

/// One quarter-hour.
#[derive(Deserialize, Clone, Default)]
pub struct TimeBand {
    #[serde(rename = "timeBand")]
    pub time_band: Option<String>,
    // TfL capitalises this differently here than on the live feed —
    // `percentageOfBaseLine` historically, `percentageOfBaseline` live — so it
    // has to be named explicitly rather than left to a rename rule.
    #[serde(rename = "percentageOfBaseLine")]
    pub percentage_of_base_line: Option<f64>,
}

#[Object]
impl DayCrowding {
    /// `MON` through `SUN`.
    async fn day(&self) -> Option<&str> {
        self.day_of_week.as_deref()
    }

    /// Whether TfL has data for this station. False means no sensors, not an
    /// empty station.
    async fn has_data(&self) -> bool {
        self.is_found.unwrap_or(false)
    }

    /// Whether TfL considers this station quiet all day.
    async fn is_always_quiet(&self) -> Option<bool> {
        self.is_always_quiet
    }

    /// The morning rush, e.g. `08:00-10:00`.
    async fn morning_peak(&self) -> Option<&str> {
        self.am_peak_time_band.as_deref()
    }

    /// The evening rush, e.g. `17:00-19:00`.
    async fn evening_peak(&self) -> Option<&str> {
        self.pm_peak_time_band.as_deref()
    }

    /// Every quarter-hour of the day, in order.
    ///
    /// Ninety-six bands. Ask for `quietest` or `busiest` instead unless you
    /// genuinely want the shape of the whole day.
    async fn time_bands(&self) -> Vec<TimeBand> {
        self.time_bands.clone()
    }

    /// The quietest quarter-hour, ignoring the small hours when the station is
    /// shut.
    ///
    /// "When should I travel" almost never means 03:00, so bands before 05:00
    /// are excluded — a closed station is not a good time to travel.
    async fn quietest(&self) -> Option<TimeBand> {
        quietest_band(&self.time_bands)
    }

    /// The busiest quarter-hour.
    async fn busiest(&self) -> Option<TimeBand> {
        busiest_band(&self.time_bands)
    }
}

#[Object]
impl TimeBand {
    /// The quarter-hour, e.g. `08:15-08:30`.
    async fn time(&self) -> Option<&str> {
        self.time_band.as_deref()
    }

    /// How busy, relative to this station's own normal. See
    /// `LiveCrowding.relativeToNormal`.
    async fn relative_to_normal(&self) -> Option<f64> {
        self.percentage_of_base_line
    }
}

fn quietest_band(bands: &[TimeBand]) -> Option<TimeBand> {
    travelling_hours(bands)
        .min_by(|a, b| busyness(a).total_cmp(&busyness(b)))
        .cloned()
}

fn busiest_band(bands: &[TimeBand]) -> Option<TimeBand> {
    travelling_hours(bands)
        .max_by(|a, b| busyness(a).total_cmp(&busyness(b)))
        .cloned()
}

/// Bands from 05:00 onwards — when the network is actually running.
fn travelling_hours(bands: &[TimeBand]) -> impl Iterator<Item = &TimeBand> {
    bands.iter().filter(|b| {
        b.time_band
            .as_deref()
            .and_then(|t| t.split(':').next())
            .and_then(|h| h.parse::<u32>().ok())
            .is_some_and(|hour| hour >= 5)
    })
}

fn busyness(band: &TimeBand) -> f64 {
    band.percentage_of_base_line.unwrap_or(f64::MAX)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn band(time: &str, level: f64) -> TimeBand {
        TimeBand {
            time_band: Some(time.into()),
            percentage_of_base_line: Some(level),
        }
    }

    #[test]
    fn the_quietest_time_is_not_the_middle_of_the_night() {
        // 03:00 is quietest because the station is shut. Offering it as the
        // best time to travel is technically true and useless.
        let bands = vec![
            band("03:00-03:15", 0.01),
            band("10:00-10:15", 0.4),
            band("08:00-08:15", 1.8),
        ];
        assert_eq!(
            quietest_band(&bands).and_then(|b| b.time_band).as_deref(),
            Some("10:00-10:15")
        );
        assert_eq!(
            busiest_band(&bands).and_then(|b| b.time_band).as_deref(),
            Some("08:00-08:15")
        );
    }

    #[test]
    fn a_missing_reading_is_not_a_quiet_station() {
        // The trap: absent data reads as zero crowding, i.e. "lovely and
        // empty", when TfL simply has no sensor there.
        assert_eq!(LiveCrowding::default().percentage_of_baseline, None);
        assert_eq!(LiveCrowding::default().data_available, None);
    }
}
