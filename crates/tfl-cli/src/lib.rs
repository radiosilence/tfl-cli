//! GraphQL and MCP over the Transport for London Unified API.
//!
//! Both a binary and a library, so a hosted MCP service can link the machinery
//! directly rather than shelling out.
//!
//! The layering is deliberate:
//!
//! - `tfl-api-client` — generated from TfL's Swagger document. Flat records,
//!   one method per endpoint, no opinions.
//! - [`mcp::graphql`] — hand-written. Turns the foreign keys in those records
//!   into a graph, which is the part no spec describes.
//! - [`mcp`] — serves that graph over MCP, and over HTTP for GraphiQL.

pub mod mcp;
pub mod output;

/// The `app_key` from the environment.
///
/// Empty is treated as absent: an unset variable arrives as `""`, and TfL
/// answers an invalid key with 429 where an anonymous caller would have got a
/// 200 — so sending a blank key is strictly worse than sending none.
pub fn app_key_from_env() -> Option<String> {
    std::env::var("TFL_APP_KEY")
        .ok()
        .map(|k| k.trim().to_string())
        .filter(|k| !k.is_empty())
}
