//! The GraphQL schema over TfL.
//!
//! Hand-written, unlike the REST client underneath it. TfL's Swagger document
//! describes 84 endpoints returning flat records; what it cannot describe is
//! how those records join up. That joining is the whole product, so it lives
//! here rather than being derived from a spec that does not know about it.

pub mod bike;
pub mod journey;
pub mod loaders;
pub mod query;
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
