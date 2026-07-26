//! Proves the batching rather than assuming it.
//!
//! The whole justification for a graph over TfL is that following an edge
//! across a list costs one request instead of one per item. That is a claim
//! about *how many* requests leave the process, which no assertion on resolver
//! output can check — a schema that quietly issued sixty requests would return
//! exactly the same JSON.
//!
//! So these run the real schema against a mock TfL and count what it asks for.

use std::sync::Arc;

use tfl_api_client::{Client, Config};
use tfl_mcp::mcp::graphql;
use wiremock::{
    Mock, MockServer, ResponseTemplate,
    matchers::{method, path, path_regex},
};

/// Six arrivals over three lines, terminating at four distinct stops.
///
/// The shape matters more than the values: repeated `lineId`s and repeated
/// `destinationNaptanId`s are what the loaders are supposed to collapse.
fn arrivals() -> serde_json::Value {
    let predictions = [
        ("victoria", "DEST_A", 60),
        ("victoria", "DEST_B", 120),
        ("circle", "DEST_A", 180),
        ("circle", "DEST_C", 240),
        ("northern", "DEST_D", 300),
        ("northern", "DEST_A", 360),
    ];
    predictions
        .iter()
        .enumerate()
        .map(|(i, (line, dest, tts))| {
            serde_json::json!({
                "id": i.to_string(),
                "lineId": line,
                "naptanId": "STOP1",
                "destinationNaptanId": dest,
                "timeToStation": tts,
            })
        })
        .collect()
}

async fn mock_tfl() -> MockServer {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/StopPoint/STOP1/Arrivals"))
        .respond_with(ResponseTemplate::new(200).set_body_json(arrivals()))
        .mount(&server)
        .await;

    // A single-id lookup, which TfL answers with a bare object rather than a
    // one-element array.
    Mock::given(method("GET"))
        .and(path("/StopPoint/STOP1"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "naptanId": "STOP1",
            "commonName": "Test Station",
        })))
        .mount(&server)
        .await;

    // Batched destinations, and batched line statuses. `path_regex` because the
    // ids are comma-joined into the path, which is the thing being tested.
    Mock::given(method("GET"))
        .and(path_regex(r"^/StopPoint/DEST_[A-D](,DEST_[A-D])*$"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([
            { "naptanId": "DEST_A", "commonName": "Destination A" },
            { "naptanId": "DEST_B", "commonName": "Destination B" },
            { "naptanId": "DEST_C", "commonName": "Destination C" },
            { "naptanId": "DEST_D", "commonName": "Destination D" },
        ])))
        .mount(&server)
        .await;

    Mock::given(method("GET"))
        .and(path_regex(r"^/Line/[a-z,]+/Status$"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([
            { "id": "victoria", "name": "Victoria", "modeName": "tube" },
            { "id": "circle", "name": "Circle", "modeName": "tube" },
            { "id": "northern", "name": "Northern", "modeName": "tube" },
        ])))
        .mount(&server)
        .await;

    server
}

fn client(server: &MockServer) -> Arc<Client> {
    Arc::new(
        Client::new(Config {
            base_url: server.uri(),
            ..Default::default()
        })
        .unwrap(),
    )
}

async fn run(server: &MockServer, query: &str) -> async_graphql::Response {
    let response = graphql::schema()
        .execute(graphql::request(query, client(server)))
        .await;
    assert!(response.errors.is_empty(), "{:?}", response.errors);
    response
}

async fn paths(server: &MockServer) -> Vec<String> {
    server
        .received_requests()
        .await
        .unwrap_or_default()
        .iter()
        .map(|r| r.url.path().to_string())
        .collect()
}

#[tokio::test]
async fn following_an_edge_across_a_list_costs_one_request() {
    let server = mock_tfl().await;

    run(
        &server,
        r#"{ stopPoint(id: "STOP1") {
              commonName
              arrivals { timeToStation line { name } destination { commonName } }
            } }"#,
    )
    .await;

    let paths = paths(&server).await;

    // Six arrivals, each following two edges. Resolved one at a time that is
    // 1 + 1 + 12 = 14 requests; batched it is four, and stays four however many
    // arrivals there are.
    assert_eq!(
        paths.len(),
        4,
        "expected the two edges to batch into one request each, got:\n{paths:#?}"
    );

    // Three distinct lines in one path, not three requests.
    let lines = paths
        .iter()
        .find(|p| p.starts_with("/Line/"))
        .expect("no line request was made");
    assert!(
        lines.contains("victoria") && lines.contains("circle") && lines.contains("northern"),
        "every line should share one request, got {lines}"
    );

    // Four distinct destinations in one path — and DEST_A appeared three times
    // in the arrivals, so duplicates must have been collapsed.
    let destinations = paths
        .iter()
        .find(|p| p.starts_with("/StopPoint/DEST_"))
        .expect("no destination request was made");
    assert_eq!(
        destinations.matches("DEST_").count(),
        4,
        "duplicate destinations should be deduplicated, got {destinations}"
    );
}

#[tokio::test]
async fn a_repeated_edge_is_only_fetched_once() {
    let server = mock_tfl().await;

    // Asking for the same stop twice, and for the line edge under both, must
    // not double anything: the loaders deduplicate by key.
    run(
        &server,
        r#"{
             a: stopPoint(id: "STOP1") { arrivals { line { name } } }
             b: stopPoint(id: "STOP1") { arrivals { line { name } } }
           }"#,
    )
    .await;

    let paths = paths(&server).await;
    assert_eq!(
        paths.iter().filter(|p| p.ends_with("/Arrivals")).count(),
        1,
        "arrivals for one stop should be fetched once, got:\n{paths:#?}"
    );
    assert_eq!(
        paths.iter().filter(|p| p.starts_with("/Line/")).count(),
        1,
        "lines should be fetched once, got:\n{paths:#?}"
    );
}

#[tokio::test]
async fn loaders_do_not_survive_the_request() {
    // Transit data goes stale in seconds, so a second query must go back to
    // TfL rather than reuse the first query's answers.
    let server = mock_tfl().await;
    let query = r#"{ stopPoint(id: "STOP1") { arrivals { timeToStation } } }"#;

    run(&server, query).await;
    let after_first = paths(&server).await.len();
    run(&server, query).await;
    let after_second = paths(&server).await.len();

    assert_eq!(
        after_second,
        after_first * 2,
        "the second query should have re-fetched everything, not reused a cache"
    );
}

#[tokio::test]
async fn one_unknown_id_does_not_take_the_batch_down_with_it() {
    // TfL rejects a whole batch when any id in it is unknown, so the loader
    // retries individually. The known ids must still resolve, and the unknown
    // one must come back null rather than failing the query.
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/StopPoint/GOOD"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "naptanId": "GOOD",
            "commonName": "Real Station",
        })))
        .mount(&server)
        .await;

    Mock::given(method("GET"))
        .and(path("/StopPoint/BAD"))
        .respond_with(ResponseTemplate::new(404).set_body_string("not found"))
        .mount(&server)
        .await;

    // The batch containing both is rejected outright, as TfL does.
    Mock::given(method("GET"))
        .and(path_regex(r"^/StopPoint/[A-Z]+,[A-Z]+$"))
        .respond_with(ResponseTemplate::new(404).set_body_string("not found"))
        .mount(&server)
        .await;

    let response = graphql::schema()
        .execute(graphql::request(
            r#"{ stopPoints(ids: ["GOOD", "BAD"]) { id commonName } }"#,
            client(&server),
        ))
        .await;

    assert!(
        response.errors.is_empty(),
        "an unknown id should not fail the query: {:?}",
        response.errors
    );
    let data = response.data.into_json().unwrap();
    let stops = data["stopPoints"].as_array().unwrap();
    assert_eq!(stops.len(), 1, "only the known stop should be returned");
    assert_eq!(stops[0]["commonName"], "Real Station");
}

#[tokio::test]
async fn bike_points_are_fetched_once_and_occupancy_batched() {
    // TfL has no geographic filter for bike points, so the whole set is
    // fetched and measured locally. That is one request however many stations
    // a query touches — and the occupancy edge across all of them is one more.
    let server = MockServer::start().await;

    let stations: serde_json::Value = (1..=3)
        .map(|i| {
            serde_json::json!({
                "id": format!("BikePoints_{i}"),
                "commonName": format!("Station {i}"),
                "lat": 51.529 + (i as f64) * 0.0001,
                "lon": -0.109,
                "additionalProperties": [
                    { "key": "NbBikes", "value": "4" },
                    { "key": "NbEmptyDocks", "value": "13" },
                ],
            })
        })
        .collect();

    Mock::given(method("GET"))
        .and(path("/BikePoint"))
        .respond_with(ResponseTemplate::new(200).set_body_json(stations))
        .mount(&server)
        .await;

    Mock::given(method("GET"))
        .and(path_regex(r"^/Occupancy/BikePoints/.+$"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([
            { "id": "BikePoints_1", "bikesCount": 4, "emptyDocks": 13 },
            { "id": "BikePoints_2", "bikesCount": 4, "emptyDocks": 13 },
            { "id": "BikePoints_3", "bikesCount": 4, "emptyDocks": 13 },
        ])))
        .mount(&server)
        .await;

    run(
        &server,
        r#"{ bikePointsNear(lat: 51.529, lon: -0.109, radius: 1000) {
               commonName bikes occupancy { bikes }
             } }"#,
    )
    .await;

    let paths = paths(&server).await;
    assert_eq!(
        paths.iter().filter(|p| *p == "/BikePoint").count(),
        1,
        "the station set should be fetched once, got:\n{paths:#?}"
    );
    let occupancy: Vec<&String> = paths
        .iter()
        .filter(|p| p.starts_with("/Occupancy/"))
        .collect();
    assert_eq!(
        occupancy.len(),
        1,
        "occupancy for every station should share one request, got:\n{paths:#?}"
    );
    assert_eq!(
        occupancy[0].matches("BikePoints_").count(),
        3,
        "all three ids belong in the one path, got {}",
        occupancy[0]
    );
}

#[tokio::test]
async fn a_failed_fetch_beside_a_working_one_is_an_error_not_an_empty_answer() {
    // The worst failure this API can have. Ask about two stops at once, let one
    // arrivals request fail, and the naive behaviour is that the failed stop
    // reports `arrivals: []` — read by anyone as "no trains due" — while its
    // sibling answers normally and lends the whole response an air of success.
    // The same mechanism made a failed disruption fetch read as good service.
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path_regex(r"^/StopPoint/(GOOD|BAD)$"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([
            { "naptanId": "GOOD", "commonName": "Working" },
            { "naptanId": "BAD", "commonName": "Broken" },
        ])))
        .mount(&server)
        .await;
    Mock::given(method("GET"))
        .and(path("/StopPoint/GOOD/Arrivals"))
        .respond_with(ResponseTemplate::new(200).set_body_json(arrivals()))
        .mount(&server)
        .await;
    Mock::given(method("GET"))
        .and(path("/StopPoint/BAD/Arrivals"))
        .respond_with(ResponseTemplate::new(500).set_body_string("upstream is having a moment"))
        .mount(&server)
        .await;
    Mock::given(method("GET"))
        .and(path_regex(r"^/Line/.+/Status$"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([])))
        .mount(&server)
        .await;

    let response = graphql::schema()
        .execute(graphql::request(
            r#"{ good: stopPoint(id: "GOOD") { arrivals { timeToStation } }
                 bad:  stopPoint(id: "BAD")  { arrivals { timeToStation } } }"#,
            client(&server),
        ))
        .await;

    assert!(
        !response.errors.is_empty(),
        "the failed stop must surface an error rather than an empty list"
    );
    let reported = response
        .errors
        .iter()
        .any(|e| e.message.contains("500") || e.message.to_lowercase().contains("error"));
    assert!(
        reported,
        "the error should say what went wrong: {:?}",
        response.errors
    );
}
