//! The endpoints TfL's spec gives up on and types as `System.Object`.
//!
//! Air quality and Cabwise return perfectly good structures; the Swagger
//! document simply does not describe them, so the generated client hands back
//! raw JSON and the shapes are written out here from what TfL actually sends.
//!
//! Cabwise is the odd one: its keys are `PascalCase` where the rest of the API
//! is `camelCase`, because it comes from a different system behind the same
//! host.

use async_graphql::{Object, SimpleObject};
use serde::Deserialize;

/// London's air quality forecast.
pub struct AirQuality(pub AirQualityBody);

#[derive(Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AirQualityBody {
    #[serde(default)]
    pub current_forecast: Vec<Forecast>,
    pub disclaimer_text: Option<String>,
}

#[derive(Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct Forecast {
    pub forecast_type: Option<String>,
    pub forecast_band: Option<String>,
    pub forecast_summary: Option<String>,
    pub forecast_text: Option<String>,
    pub n_o2_band: Option<String>,
    pub o3_band: Option<String>,
    #[serde(rename = "pM10Band")]
    pub pm10_band: Option<String>,
    #[serde(rename = "pM25Band")]
    pub pm25_band: Option<String>,
    pub s_o2_band: Option<String>,
    pub from_date: Option<String>,
    pub to_date: Option<String>,
}

#[Object]
impl AirQuality {
    /// The forecast for now, when TfL is publishing one.
    async fn current(&self) -> Option<AirQualityForecast> {
        self.0
            .current_forecast
            .iter()
            .find(|f| f.forecast_type.as_deref() == Some("Current"))
            .or_else(|| self.0.current_forecast.first())
            .cloned()
            .map(AirQualityForecast)
    }

    /// Every forecast period TfL published, usually today and tomorrow.
    async fn forecasts(&self) -> Vec<AirQualityForecast> {
        self.0
            .current_forecast
            .iter()
            .cloned()
            .map(AirQualityForecast)
            .collect()
    }

    /// TfL's caveat about how the forecast should be read.
    async fn disclaimer(&self) -> Option<String> {
        self.0.disclaimer_text.as_deref().map(strip_html)
    }
}

/// A pollution forecast for one period.
pub struct AirQualityForecast(pub Forecast);

#[Object]
impl AirQualityForecast {
    /// `Current` or `Future`.
    async fn period(&self) -> Option<&str> {
        self.0.forecast_type.as_deref()
    }

    /// The headline band: `Low`, `Moderate`, `High` or `Very High`.
    ///
    /// The one field worth reading if you only read one.
    async fn band(&self) -> Option<&str> {
        self.0.forecast_band.as_deref()
    }

    /// One line summarising the forecast and when it applies.
    async fn summary(&self) -> Option<&str> {
        self.0.forecast_summary.as_deref()
    }

    /// The full explanation, with TfL's HTML removed.
    ///
    /// It arrives double-escaped (`&lt;br/&gt;`), so it is decoded here rather
    /// than handed on as markup nobody asked for.
    async fn detail(&self) -> Option<String> {
        self.0.forecast_text.as_deref().map(strip_html)
    }

    /// Nitrogen dioxide band — the pollutant traffic drives.
    async fn nitrogen_dioxide(&self) -> Option<&str> {
        self.0.n_o2_band.as_deref()
    }

    /// Ozone band.
    async fn ozone(&self) -> Option<&str> {
        self.0.o3_band.as_deref()
    }

    /// Particulates under 10 micrometres.
    async fn pm10(&self) -> Option<&str> {
        self.0.pm10_band.as_deref()
    }

    /// Particulates under 2.5 micrometres — the ones that reach the lungs.
    async fn pm25(&self) -> Option<&str> {
        self.0.pm25_band.as_deref()
    }

    /// Sulphur dioxide band.
    async fn sulphur_dioxide(&self) -> Option<&str> {
        self.0.s_o2_band.as_deref()
    }

    async fn from_date(&self) -> Option<&str> {
        self.0.from_date.as_deref()
    }

    async fn to_date(&self) -> Option<&str> {
        self.0.to_date.as_deref()
    }
}

/// Decodes TfL's escaped HTML into plain text.
///
/// The forecast text arrives as `&lt;br/&gt;` — HTML, escaped once more on the
/// way out — so it needs unescaping before the tags can be stripped. Only the
/// entities TfL actually emits are handled; anything else is left alone rather
/// than half-decoded.
fn strip_html(text: &str) -> String {
    let decoded = text
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&quot;", "\"")
        .replace("&#39;", "'")
        .replace("&nbsp;", " ")
        // Last, so a literal "&amp;lt;" does not become a tag.
        .replace("&amp;", "&");

    let mut out = String::with_capacity(decoded.len());
    let mut in_tag = false;
    for ch in decoded.chars() {
        match ch {
            '<' => in_tag = true,
            '>' => {
                in_tag = false;
                // A <br/> is a sentence break; without this the text runs on.
                if !out.ends_with(' ') && !out.is_empty() {
                    out.push(' ');
                }
            }
            _ if !in_tag => out.push(ch),
            _ => {}
        }
    }
    out.split_whitespace().collect::<Vec<_>>().join(" ")
}

// ------------------------------------------------------------------ cabwise

/// A licensed minicab or taxi operator.
#[derive(SimpleObject)]
pub struct TaxiOperator {
    /// Trading name — what is on the shopfront.
    pub name: Option<String>,
    /// The registered company behind it, where it differs.
    pub company: Option<String>,
    /// Number to ring for a booking.
    pub phone: Option<String>,
    pub email: Option<String>,
    pub address: Option<String>,
    pub postcode: Option<String>,
    /// How far TfL reckons it is from the search point.
    ///
    /// The unit is undocumented and does not match metres, kilometres or miles
    /// against known addresses, so treat it as an ordering rather than a
    /// figure to quote at anyone. Results already arrive nearest-first.
    pub distance: Option<f64>,
    /// Whether they can carry a wheelchair.
    pub wheelchair_accessible: Option<bool>,
    /// Whether they take bookings around the clock.
    pub open_twenty_four_hours: Option<bool>,
}

/// Cabwise's response.
///
/// `PascalCase` throughout, unlike the rest of the API.
#[derive(Deserialize, Default)]
pub struct CabwiseBody {
    #[serde(rename = "Operators")]
    pub operators: Option<Operators>,
}

#[derive(Deserialize, Default)]
pub struct Operators {
    #[serde(rename = "OperatorList", default)]
    pub operator_list: Vec<Operator>,
}

#[derive(Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct Operator {
    pub organisation_name: Option<String>,
    pub trading_name: Option<String>,
    pub bookings_phone_number: Option<String>,
    pub bookings_email: Option<String>,
    pub address_line1: Option<String>,
    pub address_line2: Option<String>,
    pub address_line3: Option<String>,
    pub town: Option<String>,
    pub postcode: Option<String>,
    pub distance: Option<f64>,
    pub wheel_chair_access: Option<bool>,
    pub twenty_four_seven_service: Option<bool>,
}

impl From<Operator> for TaxiOperator {
    fn from(o: Operator) -> Self {
        // TfL pads unused address lines with "," rather than leaving them out.
        let parts: Vec<String> = [o.address_line1, o.address_line2, o.address_line3, o.town]
            .into_iter()
            .flatten()
            .map(|p| p.trim().trim_matches(',').trim().to_string())
            .filter(|p| !p.is_empty())
            .collect();

        Self {
            name: o
                .trading_name
                .clone()
                .or_else(|| o.organisation_name.clone()),
            company: o
                .organisation_name
                .filter(|c| Some(c) != o.trading_name.as_ref()),
            phone: o.bookings_phone_number,
            email: o.bookings_email,
            address: (!parts.is_empty()).then(|| parts.join(", ")),
            postcode: o.postcode,
            distance: o.distance,
            wheelchair_accessible: o.wheel_chair_access,
            open_twenty_four_hours: o.twenty_four_seven_service,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decodes_and_strips_tfls_escaped_html() {
        let text = "Warm and sunny.&lt;br/&gt;&lt;br/&gt;A fresh and &#39;clean&#39; feed.";
        assert_eq!(
            strip_html(text),
            "Warm and sunny. A fresh and 'clean' feed."
        );
        assert_eq!(strip_html("plain text"), "plain text");
        assert_eq!(strip_html(""), "");
    }

    #[test]
    fn an_escaped_ampersand_does_not_become_a_tag() {
        // `&amp;lt;` is HTML for the literal text `&lt;`. One decode pass gives
        // exactly that, and it must not go round again into a `<` that then
        // gets stripped as markup — the text would silently lose everything
        // between it and the next `>`. Decoding `&amp;` last is what stops it.
        assert_eq!(strip_html("a &amp;lt;b&amp;gt; c"), "a &lt;b&gt; c");
        // The single-escaped form TfL actually sends is still stripped.
        assert_eq!(strip_html("a&lt;br/&gt;b"), "a b");
    }

    #[test]
    fn address_lines_padded_with_commas_are_dropped() {
        let operator = Operator {
            trading_name: Some("Minicab Waterloo".into()),
            organisation_name: Some("Euro Mobile Cars Limited".into()),
            address_line1: Some("135".into()),
            address_line2: Some("Mepham Street".into()),
            address_line3: Some(",".into()),
            town: Some(",".into()),
            postcode: Some("SE1 8SQ".into()),
            ..Default::default()
        };
        let taxi = TaxiOperator::from(operator);
        assert_eq!(taxi.address.as_deref(), Some("135, Mepham Street"));
        assert_eq!(taxi.name.as_deref(), Some("Minicab Waterloo"));
        assert_eq!(taxi.company.as_deref(), Some("Euro Mobile Cars Limited"));
    }

    #[test]
    fn a_company_trading_under_its_own_name_is_not_repeated() {
        let operator = Operator {
            trading_name: Some("Same Ltd".into()),
            organisation_name: Some("Same Ltd".into()),
            ..Default::default()
        };
        let taxi = TaxiOperator::from(operator);
        assert_eq!(taxi.name.as_deref(), Some("Same Ltd"));
        assert_eq!(taxi.company, None);
    }
}
