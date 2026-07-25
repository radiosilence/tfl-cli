//! A client for the Transport for London Unified API.
//!
//! The response types and one method per endpoint are generated from TfL's
//! Swagger document by `cargo xtask regen` and live in [`generated`]. This
//! module is the hand-written half: authentication, caching, retries and error
//! mapping — everything the spec cannot describe.

mod cache;
mod client;
pub mod de;
pub mod generated;

pub use cache::CacheStats;
pub use client::{Client, Config};
pub use generated::{endpoints::*, models};

use percent_encoding::{AsciiSet, CONTROLS, utf8_percent_encode};

/// Errors from a TfL request.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("http error: {0}")]
    Http(#[from] reqwest::Error),

    /// TfL answers an unrecognised key with `429 Invalid app_key is provided.`,
    /// which is indistinguishable from throttling unless you read the body —
    /// so it gets its own variant rather than looking like a transient failure.
    #[error("TfL rejected the app key")]
    InvalidAppKey,

    /// Anonymous callers get 50 requests/minute, registered ones 500.
    #[error("rate limited by TfL{}", .retry_after.map(|s| format!(", retry after {s}s")).unwrap_or_default())]
    RateLimited { retry_after: Option<u64> },

    /// TfL answers an ambiguous journey-planner location with `300 Multiple
    /// Choices` and a body listing the candidates. That is a useful answer
    /// rather than a failure — anyone asking for a journey between place
    /// *names* will hit it — so the body is kept whole for the caller to
    /// decode, not truncated like a genuine error.
    #[error("{path} is ambiguous; TfL returned candidate locations")]
    Ambiguous { path: String, body: String },

    #[error("{status} from {path}: {body}")]
    Status {
        status: reqwest::StatusCode,
        path: String,
        body: String,
    },

    #[error("could not decode the response from {path}: {source}")]
    Decode {
        path: String,
        #[source]
        source: serde_json::Error,
    },
}

pub type Result<T> = std::result::Result<T, Error>;

/// Formats a batch of ids the way TfL expects them: comma-separated, in a path
/// segment.
pub fn join<S: AsRef<str>>(values: &[S]) -> String {
    values
        .iter()
        .map(|v| segment(v.as_ref()))
        .collect::<Vec<_>>()
        .join(",")
}

/// Percent-encodes one path segment.
///
/// Commas are deliberately left alone — TfL separates batched ids with them, so
/// encoding commas breaks every multi-id endpoint.
pub fn segment(value: &str) -> String {
    const RESERVED: &AsciiSet = &CONTROLS
        .add(b' ')
        .add(b'"')
        .add(b'#')
        .add(b'<')
        .add(b'>')
        .add(b'?')
        .add(b'`')
        .add(b'{')
        .add(b'}')
        .add(b'/')
        .add(b'%');
    utf8_percent_encode(value, RESERVED).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn joins_ids_for_batch_endpoints() {
        assert_eq!(join(&["victoria", "circle"]), "victoria,circle");
        assert_eq!(join(&[] as &[&str]), "");
    }

    #[test]
    fn encodes_path_separators_but_leaves_batch_commas() {
        assert_eq!(segment("a/b"), "a%2Fb");
        assert_eq!(segment("Oxford Circus"), "Oxford%20Circus");
        assert_eq!(join(&["a b", "c"]), "a%20b,c");
    }
}
