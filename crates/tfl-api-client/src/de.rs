//! Deserialisers for the places TfL's spec and TfL's output disagree.

use serde::{Deserialize, Deserializer};

/// A boolean TfL might send as a string.
///
/// `StopPoint.status` is declared `boolean` and comes back as `"Unknown"` from
/// `/CanReachOnLine`. Since serde fails the whole response on one bad field,
/// a single stop with an opinion about its own status would otherwise lose the
/// other fourteen.
///
/// Anything unrecognised is `None` rather than a guess: "Unknown" means TfL
/// does not know, and inventing `false` would be worse than admitting it.
pub fn bool_or_string<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<Option<bool>, D::Error> {
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum Loose {
        Bool(bool),
        Text(String),
        Null,
    }

    Ok(match Option::<Loose>::deserialize(deserializer)? {
        Some(Loose::Bool(b)) => Some(b),
        Some(Loose::Text(t)) => match t.trim().to_ascii_lowercase().as_str() {
            "true" | "yes" | "1" => Some(true),
            "false" | "no" | "0" => Some(false),
            _ => None,
        },
        _ => None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Deserialize)]
    struct Holder {
        #[serde(default, deserialize_with = "bool_or_string")]
        status: Option<bool>,
    }

    fn status(json: &str) -> Option<bool> {
        serde_json::from_str::<Holder>(json)
            .expect("should not fail")
            .status
    }

    #[test]
    fn accepts_the_shapes_tfl_actually_sends() {
        assert_eq!(status(r#"{"status": true}"#), Some(true));
        assert_eq!(status(r#"{"status": "true"}"#), Some(true));
        assert_eq!(status(r#"{"status": "false"}"#), Some(false));
        assert_eq!(status(r#"{"status": null}"#), None);
        assert_eq!(status("{}"), None);
    }

    #[test]
    fn an_unrecognised_word_is_unknown_rather_than_false() {
        // TfL literally sends "Unknown". Reporting that as `false` would state
        // something it never said.
        assert_eq!(status(r#"{"status": "Unknown"}"#), None);
    }
}
