//! Schema-shape tests.
//!
//! These assert on the SDL rather than on query results, so they need no
//! network. What they protect is the contract an agent reads before it queries:
//! that the entry points exist, that the foreign keys really did become edges,
//! and that the descriptions saying what things cost are still there.

use super::*;

fn sdl_text() -> String {
    sdl()
}

#[test]
fn every_entry_point_is_present() {
    let sdl = sdl_text();
    for field in [
        "stopPoint(",
        "stopPoints(",
        "searchStopPoints(",
        "stopPointsNear(",
        "line(",
        "lines(",
        "linesByMode(",
        "disruptedLines(",
        "vehicleArrivals(",
        "modes:",
        "severities:",
        "journey(",
        "bikePoint(",
        "bikePointsNear(",
        "searchBikePoints(",
        "roads:",
        "road(",
        "roadDisruptions(",
        "airQuality:",
        "taxiOperators(",
        "chargeConnectors(",
        "carParks:",
        "accidents(",
    ] {
        assert!(sdl.contains(field), "QueryRoot is missing `{field}`\n{sdl}");
    }
}

#[test]
fn foreign_keys_became_edges() {
    let sdl = sdl_text();

    // The whole point: a Prediction carries lineId/naptanId/destinationNaptanId
    // as strings, and the graph turns each into a followable type.
    assert!(
        sdl.contains("line: Line"),
        "Prediction.line should return a Line"
    );
    assert!(
        sdl.contains("destination: StopPoint"),
        "Prediction.destination should return a StopPoint"
    );
    assert!(
        sdl.contains("stopPoint: StopPoint"),
        "Prediction.stopPoint should return a StopPoint"
    );
    assert!(sdl.contains("arrivals("), "StopPoint.arrivals should exist");
    assert!(
        sdl.contains("stopPoints: [StopPoint!]!"),
        "Line.stopPoints should return stop points"
    );
}

#[test]
fn the_graph_has_a_cycle_and_a_depth_limit() {
    // StopPoint -> lines -> Line -> stopPoints -> StopPoint. The cycle is
    // intended (it is how you ask "what else does my line serve"), which is why
    // MAX_DEPTH exists.
    let sdl = sdl_text();
    assert!(sdl.contains("lines: [Line!]!"));
    assert!(sdl.contains("stopPoints: [StopPoint!]!"));
}

#[test]
fn costs_are_documented_in_the_sdl() {
    // Field descriptions are the only documentation a model gets before it
    // writes a query, so the expensive edges must say they are expensive.
    let sdl = sdl_text();
    assert!(
        sdl.contains("batched") || sdl.contains("one request"),
        "field descriptions should say what an edge costs"
    );
    assert!(
        sdl.contains("stale"),
        "arrivals should warn that they are live data"
    );
}

#[test]
fn bike_counts_are_typed_fields_not_a_property_bag() {
    // The whole point of the bike type: TfL ships `NbEBikes=1` as a string in
    // an untyped bag, which nothing can usefully query.
    let sdl = sdl_text();
    for field in [
        "bikes: Int",
        "eBikes: Int",
        "emptyDocks: Int",
        "hasBikes: Boolean!",
    ] {
        assert!(sdl.contains(field), "BikePoint is missing `{field}`\n{sdl}");
    }
}

#[test]
fn journey_planning_exposes_the_ambiguous_case() {
    // TfL answers an ambiguous location with 300 rather than routes. If that
    // is not reachable from the schema, asking for a journey between two place
    // names is a dead end.
    let sdl = sdl_text();
    assert!(sdl.contains("isAmbiguous: Boolean!"));
    assert!(sdl.contains("fromOptions: [LocationOption!]!"));
    assert!(
        sdl.contains("isStation: Boolean!"),
        "callers need to tell a station from a restaurant of the same name"
    );
}

#[tokio::test]
async fn a_wide_query_is_refused_without_touching_the_network() {
    // The query this exists for: two levels deep, inside every depth limit, and
    // 676 bus lines wide. Resolved, it would fire 676 concurrent requests and
    // burn a minute of TfL's rate limit in one tool call.
    let client = std::sync::Arc::new(
        tfl_api_client::Client::new(tfl_api_client::Config {
            // Unroutable, so a request escaping the guard fails loudly rather
            // than quietly hammering TfL from a test run.
            base_url: "http://127.0.0.1:1".to_string(),
            ..Default::default()
        })
        .unwrap(),
    );

    let response = schema()
        .execute(request(
            r#"{ linesByMode(modes: ["bus"]) { stopPoints { id } } }"#,
            client,
        ))
        .await;

    assert!(
        response
            .errors
            .iter()
            .any(|e| e.message.contains("too complex")),
        "a query this wide must be refused, got {:?}",
        response.errors
    );
}

#[tokio::test]
async fn the_documented_examples_are_within_the_limits() {
    // The complexity ceiling is only useful if it refuses the pathological
    // shapes without refusing the ones the README tells people to write.
    let client = std::sync::Arc::new(
        tfl_api_client::Client::new(tfl_api_client::Config {
            base_url: "http://127.0.0.1:1".to_string(),
            ..Default::default()
        })
        .unwrap(),
    );

    for query in [
        r#"{ stopPoint(id: "X") { arrivals(first: 40) { timeToStation line { name } destination { commonName } } } }"#,
        r#"{ line(id: "victoria") { isGoodService statuses { description } } airQuality { current { band } } bikePointsNear(lat: 51.5, lon: -0.1) { commonName eBikes } }"#,
        r#"{ journey(from: "a", to: "b") { journeys { duration legs { mode summary disruptions { description } } } } }"#,
        r#"{ roadDisruptions(first: 5) { severity location roads { displayName status } } }"#,
        r#"{ accidents(lat: 51.5, lon: -0.1) { date severity casualties { class mode } vehicles } }"#,
    ] {
        let response = schema().execute(request(query, client.clone())).await;
        assert!(
            !response
                .errors
                .iter()
                .any(|e| e.message.contains("too complex") || e.message.contains("nested too deep")),
            "a documented example was refused by the limits: {query}\n{:?}",
            response.errors
        );
    }
}

#[test]
fn expensive_fields_say_so() {
    // A caller cannot see a download size, so the one field that costs tens of
    // megabytes has to admit it in the only place they will look.
    let sdl = sdl_text();
    let accidents = sdl
        .split("accidents(")
        .next()
        .expect("accidents field should exist");
    assert!(
        accidents.contains("expensive") && accidents.contains("2019"),
        "the accidents field must warn about its cost and that only 2019 has data"
    );
}

#[test]
fn every_domain_tfl_documents_is_reachable() {
    // TfL's spec covers 15 tags. TravelTime is deliberately absent — it returns
    // map tile images, which mean nothing over this transport.
    let sdl = sdl_text();
    for domain in [
        "StopPoint",
        "Line",
        "Prediction",
        "JourneyPlan",
        "BikePoint",
        "Road",
        "RoadDisruption",
        "AirQuality",
        "TaxiOperator",
        "ChargeConnector",
        "CarPark",
        "Accident",
        "Mode",
        "Disruption",
        "LineStatus",
    ] {
        assert!(
            sdl.contains(&format!("type {domain} ")),
            "no `type {domain}` in the schema"
        );
    }
}

#[test]
fn the_schema_is_read_only() {
    // TfL's Unified API has no write endpoints, so exposing a mutation root
    // would be inventing one.
    let sdl = sdl_text();
    assert!(
        !sdl.contains("type Mutation"),
        "there should be no mutations"
    );
}

#[tokio::test]
async fn runaway_queries_are_rejected_before_any_request() {
    // A query deep enough to trip the limit must fail on the limit, not by
    // walking the network to find out.
    // Each repeat is two levels (stopPoints, then lines), so this is well past
    // MAX_DEPTH however the limit counts the root.
    let repeats = MAX_DEPTH;
    let deep = format!(
        "{{ line(id: \"victoria\") {{ {} id {} }} }}",
        "stopPoints { lines { ".repeat(repeats),
        "} }".repeat(repeats)
    );
    let client = std::sync::Arc::new(
        tfl_api_client::Client::new(tfl_api_client::Config {
            // Nothing should be sent, but point it somewhere unroutable so the
            // test fails loudly rather than silently hitting TfL.
            base_url: "http://127.0.0.1:1".to_string(),
            ..Default::default()
        })
        .unwrap(),
    );

    let response = schema().execute(request(&deep, client)).await;
    assert!(response.is_err(), "expected the depth limit to reject this");
    // Either guard is a pass. Walking the graph is both deep and wide, and
    // complexity now trips first because the fan-out fields carry multipliers —
    // which is the stronger rejection: it is the width that would have made
    // hundreds of requests, not the depth.
    assert!(
        response
            .errors
            .iter()
            .any(|e| e.message.contains("nested too deep") || e.message.contains("too complex")),
        "expected a depth or complexity rejection, got {:?}",
        response.errors
    );
}
