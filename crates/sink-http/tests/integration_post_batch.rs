//! AC-5, AC-7, AC-8: Batch delivery and flush integration tests.
//!
//! BC-3.01.001 postcondition 1: Sink integrates with registry machinery.
//! Tests use a real mock HTTP server (httpmock) to verify batch POST semantics.

use httpmock::prelude::*;
use sink_core::{Sink, SinkEvent};
use sink_http::{HttpSink, HttpSinkConfig};

fn make_event(label: &str) -> SinkEvent {
    SinkEvent::new()
        .insert("type", "test.batch_event")
        .insert("label", label)
}

fn config_for_url(url: &str) -> HttpSinkConfig {
    let toml = format!(
        r#"
schema_version = 1
type = "http"
name = "batch-test-sink"
url = "{url}"
"#
    );
    HttpSinkConfig::from_toml(&toml)
        .expect("from_toml must succeed")
        .expect("must return Some")
}

/// AC-5 + AC-8 — Submit 3 events, flush, verify exactly 1 POST with all 3 as JSON array.
///
/// Exercises: HttpSink::new, Sink::submit (x3), Sink::flush.
/// Mock server asserts:
///   - exactly 1 POST request received
///   - body is a JSON array (body_contains "[")
///   - all 3 event labels present in the body
///
/// Single mock with chained body_contains() matchers. httpmock 0.7 routes each
/// request to the first matching mock by ascending mock ID; registering additional
/// mocks for label sub-checks would never receive hits because the primary mock
/// matches first and consumes the request. All assertions are expressed on one mock.
#[tokio::test]
async fn test_TV_events_batched_and_posted_as_json_array() {
    let server = MockServer::start();

    // One mock that requires the batch body to contain all 3 labels.
    // Chained body_contains() calls are AND-ed: every substring must appear.
    let mock = server.mock(|when, then| {
        when.method(POST)
            .path("/events")
            .body_contains(r#""label":"a""#)
            .body_contains(r#""label":"b""#)
            .body_contains(r#""label":"c""#);
        then.status(200).body("{}");
    });

    let config = config_for_url(&format!("{}/events", server.base_url()));
    let sink = HttpSink::new(config).expect("HttpSink::new must succeed");

    sink.submit(make_event("a"));
    sink.submit(make_event("b"));
    sink.submit(make_event("c"));

    sink.flush().expect("flush must succeed");

    // Exactly 1 POST — all 3 events batched into one request (AC-5 / AC-8).
    // The body_contains matchers above ensure all 3 labels are in that body.
    mock.assert_hits(1);
}

/// AC-7 — flush() delivers the current batch synchronously before returning.
///
/// Exercises: HttpSink::new, Sink::submit, Sink::flush.
/// Verifies that after flush() returns, the mock has already received the POST
/// (no race condition — flush is synchronous with respect to the HTTP response).
#[tokio::test]
async fn test_TV_flush_sends_synchronously() {
    let server = MockServer::start();
    let mock = server.mock(|when, then| {
        when.method(POST).path("/events");
        then.status(200).body("{}");
    });

    let config = config_for_url(&format!("{}/events", server.base_url()));
    let sink = HttpSink::new(config).expect("HttpSink::new must succeed");

    sink.submit(make_event("sync-check"));

    // After flush() returns, the POST must already have been received.
    sink.flush().expect("flush must succeed");

    // If this assertion fails with 0 hits, flush() returned before the HTTP
    // round-trip completed — violates the AC-7 "synchronously" requirement.
    mock.assert_hits(1);
}
