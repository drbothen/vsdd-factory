//! Tests for circuit state event emission (S-4.04).
//!
//! AC: "Circuit state emitted as `internal.sink_circuit_opened` /
//!      `internal.sink_circuit_closed` events"
//! Traces to: BC-3.NN.NNN-circuit-event-emission (v1.1 BC candidate), VP-012

use sink_core::resilience::{CircuitBreaker, CircuitState};
use std::time::Duration;

/// Exercises postcondition: CircuitBreaker exposes a method to drain emitted
/// event names so callers can verify that `internal.sink_circuit_opened` was
/// emitted when the circuit opened.
///
/// The implementation is expected to buffer emitted event type strings in an
/// internal Vec accessible via `take_emitted_events() -> Vec<String>`.
#[test]
fn test_BC_3_03_002_circuit_opened_event_emitted_on_closed_to_open() {
    let breaker = CircuitBreaker::new(2, Duration::from_secs(60));

    breaker.record_failure();
    breaker.record_failure(); // trips: Closed → Open

    let events = breaker.take_emitted_events();
    assert!(
        events.iter().any(|e| e == "internal.sink_circuit_opened"),
        "expected `internal.sink_circuit_opened` in emitted events, got: {events:?}"
    );
}

/// Exercises postcondition: `internal.sink_circuit_closed` is emitted when
/// the circuit transitions HalfOpen → Closed on success.
#[test]
fn test_BC_3_03_002_circuit_closed_event_emitted_on_halfopen_to_closed() {
    let breaker = CircuitBreaker::new(1, Duration::from_millis(0));
    breaker.record_failure(); // Closed → Open

    std::thread::sleep(Duration::from_millis(5)); // let cool-off elapse

    // Confirm HalfOpen.
    assert_eq!(breaker.state(), CircuitState::HalfOpen);

    // Drain any events from the open transition.
    let _ = breaker.take_emitted_events();

    // Success in HalfOpen → Closed.
    breaker.record_success();

    let events = breaker.take_emitted_events();
    assert!(
        events.iter().any(|e| e == "internal.sink_circuit_closed"),
        "expected `internal.sink_circuit_closed` in emitted events, got: {events:?}"
    );
}

/// Exercises postcondition: no circuit events emitted when circuit stays Closed.
#[test]
fn test_BC_3_03_002_no_event_emitted_when_circuit_stays_closed() {
    let breaker = CircuitBreaker::new(5, Duration::from_secs(60));

    // 4 failures — below threshold.
    for _ in 0..4 {
        breaker.record_failure();
    }
    // Success.
    breaker.record_success();

    let events = breaker.take_emitted_events();
    assert!(
        events.is_empty(),
        "no circuit events should be emitted while circuit stays Closed, got: {events:?}"
    );
}
