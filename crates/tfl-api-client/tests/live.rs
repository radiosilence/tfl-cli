//! Checks the generated client against the real API.
//!
//! These hit api.tfl.gov.uk anonymously (50 requests/minute, no key needed) and
//! are ignored by default so an offline `cargo test` still passes. Run with
//! `cargo test -p tfl-api-client -- --ignored`.
//!
//! The point is not that TfL is up — it is that the *generated* request shapes
//! and the hand-written decoding still agree with what TfL actually serves.

use tfl_api_client::{Client, Config, LineStatusByIdsOptions};

fn client() -> Client {
    Client::new(Config::default()).unwrap()
}

#[tokio::test]
#[ignore = "hits the live TfL API"]
async fn batches_lines_into_one_request() {
    let lines = client()
        .line_get(&["victoria", "circle", "northern"])
        .await
        .unwrap();

    assert_eq!(lines.len(), 3, "one request should return all three lines");
    let mut ids: Vec<_> = lines.iter().filter_map(|l| l.id.as_deref()).collect();
    ids.sort();
    assert_eq!(ids, ["circle", "northern", "victoria"]);
}

#[tokio::test]
#[ignore = "hits the live TfL API"]
async fn arrivals_carry_the_foreign_keys_the_graph_is_built_on() {
    let arrivals = client().stop_point_arrivals("940GZZLUKSX").await.unwrap();
    assert!(!arrivals.is_empty(), "Kings Cross should have arrivals");

    // Every edge in the GraphQL layer resolves one of these.
    let first = &arrivals[0];
    assert!(first.line_id.is_some(), "lineId drives Prediction.line");
    assert!(
        first.naptan_id.is_some(),
        "naptanId drives Prediction.stopPoint"
    );
    assert!(
        first.destination_naptan_id.is_some(),
        "destinationNaptanId drives Prediction.destination"
    );
}

#[tokio::test]
#[ignore = "hits the live TfL API"]
async fn optional_query_parameters_are_sent() {
    let options = LineStatusByIdsOptions {
        detail: Some(true),
        ..Default::default()
    };
    let lines = client()
        .line_status_by_ids(&["victoria"], &options)
        .await
        .unwrap();
    assert_eq!(lines.len(), 1);
    assert!(
        lines[0]
            .line_statuses
            .as_ref()
            .is_some_and(|s| !s.is_empty()),
        "detail=true should return line statuses"
    );
}

#[tokio::test]
#[ignore = "hits the live TfL API"]
async fn unknown_ids_do_not_decode_into_garbage() {
    // TfL 404s a batch containing an unknown id rather than returning partial
    // results, which is why the DataLoaders retry unknown keys individually.
    let result = client().line_get(&["definitely-not-a-line"]).await;
    assert!(result.is_err(), "expected an error, got {result:?}");
}
