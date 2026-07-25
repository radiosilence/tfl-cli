//! The JSON envelope every command prints.

use serde::Serialize;

/// A command's result, so callers can parse stdout without knowing which
/// command produced it.
#[derive(Debug, Serialize)]
pub struct Output<T: Serialize> {
    pub success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

impl<T: Serialize> Output<T> {
    pub fn ok(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
        }
    }

    pub fn error(message: impl Into<String>) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(message.into()),
        }
    }

    pub fn print(&self) {
        match serde_json::to_string_pretty(self) {
            Ok(json) => println!("{json}"),
            Err(e) => eprintln!("could not serialise output: {e}"),
        }
    }
}
