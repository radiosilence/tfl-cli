//! The GraphQL schema over TfL.
//!
//! Hand-written, unlike the REST client underneath it. TfL's Swagger document
//! describes 84 endpoints returning flat records; what it cannot describe is
//! how those records join up. That joining is the whole product, so it lives
//! here rather than being derived from a spec that does not know about it.

pub mod bike;
pub mod crowding;
pub mod environment;
pub mod journey;
pub mod loaders;
pub mod places;
pub mod query;
pub mod road;
#[cfg(test)]
mod tests;
pub mod types;

use std::sync::Arc;

use async_graphql::{EmptyMutation, EmptySubscription, Request, Schema};
use tfl_api_client::Client;

use loaders::Loaders;
use query::QueryRoot;

/// The graph has cycles — a stop's lines lead back to their stops — so depth is
/// capped. Fifteen is well past any reasonable question and far short of one
/// that would walk the network forever.
const MAX_DEPTH: usize = 15;

/// Ceiling on estimated query cost.
///
/// Depth alone does not describe how much work a query is: `linesByMode(modes:
/// ["bus"]) { stopPoints { id } }` is two levels deep and 676 lines wide, so it
/// would pass a depth check and then make 676 requests. Fields that fan out
/// declare a multiplier for how wide they typically are, and anything whose
/// product lands past this is refused before a single request goes out.
///
/// Set so every example in the README passes comfortably and the fan-out
/// queries do not.
pub const MAX_COMPLEXITY: usize = 1_000;

/// Ceiling on any single field's computed cost.
///
/// Nesting multiplies these together, which overflows `usize` a few levels in —
/// and an overflowed cost wraps to something small, so the limit silently stops
/// limiting in a release build. Clamping each field keeps every sum
/// representable. Far above [`MAX_COMPLEXITY`], so nothing a caller would
/// legitimately write notices it.
pub const COMPLEXITY_CEILING: usize = 1_000_000;

pub type TflSchema = Schema<QueryRoot, EmptyMutation, EmptySubscription>;

/// Builds the schema.
///
/// Read-only: TfL's Unified API has no write endpoints, so there is no mutation
/// root and nothing here can change anything.
///
/// The client is not baked in. It varies per request — an HTTP caller brings
/// its own `app_key` header — and the loaders must not outlive a request, so
/// both are attached by [`request`] instead.
pub fn schema() -> TflSchema {
    Schema::build(QueryRoot, EmptyMutation, EmptySubscription)
        .limit_depth(MAX_DEPTH)
        .limit_complexity(MAX_COMPLEXITY)
        .finish()
}

/// Prepares a request, attaching the client and a fresh set of loaders.
///
/// Loaders are per request on purpose: they deduplicate reads within one query,
/// which is where the batching win is, and are thrown away afterwards so no
/// arrival time is ever served twice.
pub fn request(query: &str, client: Arc<Client>) -> Request {
    Request::new(query)
        .data(Loaders::new(client.clone()))
        .data(client)
}

/// The schema as SDL — what the `tfl_schema` MCP tool returns.
pub fn sdl() -> String {
    schema().sdl()
}
