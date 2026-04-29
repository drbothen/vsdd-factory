//! Tests for CircuitBreaker state machine (S-4.04).
//!
//! AC: "Circuit breaker: opens after N consecutive failures; half-open after cool-off period"
//! Traces to: BC-3.NN.NNN-circuit-breaker-state-machine (v1.1 BC candidate)
//!           EC-002, EC-003, VP-012

use sink_core::resilience::{CircuitBreaker, CircuitState};
use std::time::Duration;

/// Exercises postcondition: new circuit breaker starts in Closed state.
#[test]
fn test_BC_3_03_002_circuit_breaker_initial_state_is_closed() {
    let breaker = CircuitBreaker::new(3, Duration::from_secs(30));
    assert_eq!(
        breaker.state(),
        CircuitState::Closed,
        "new circuit breaker must start Closed"
    );
}

/// EC-002: N consecutive failures open the circuit.
/// Exercises Closed → Open transition postcondition.
#[test]
fn test_BC_3_03_002_ec002_n_failures_opens_circuit() {
    let threshold = 3u32;
    let breaker = CircuitBreaker::new(threshold, Duration::from_secs(60));

    for i in 0..threshold {
        assert_eq!(
            breaker.state(),
            CircuitState::Closed,
            "circuit must remain Closed after {i} failures (threshold not yet reached)"
        );
        breaker.record_failure();
    }

    assert!(
        breaker.is_open(),
        "circuit must be Open after {threshold} consecutive failures"
    );
}

/// Exercises precondition violation: fewer than threshold failures must NOT open circuit.
#[test]
fn test_BC_3_03_002_fewer_than_threshold_failures_stays_closed() {
    let threshold = 5u32;
    let breaker = CircuitBreaker::new(threshold, Duration::from_secs(60));

    for _ in 0..(threshold - 1) {
        breaker.record_failure();
    }

    assert!(
        !breaker.is_open(),
        "circuit must stay Closed when failures < threshold"
    );
    assert_eq!(breaker.state(), CircuitState::Closed);
}

/// Exercises postcondition: Open circuit rejects requests without calling op.
/// is_open() must return true when Open and cool-off has not elapsed.
#[test]
fn test_BC_3_03_002_open_circuit_is_open_returns_true() {
    let breaker = CircuitBreaker::new(1, Duration::from_secs(999));
    breaker.record_failure();

    assert!(
        breaker.is_open(),
        "is_open must return true immediately after circuit opens"
    );
}

/// EC-003: Circuit half-open after cool-off; success closes it.
/// Exercises Open → HalfOpen → Closed transition.
#[test]
fn test_BC_3_03_002_ec003_halfopen_success_closes_circuit() {
    // Use a 0-duration cool-off so we can test HalfOpen without sleeping.
    let breaker = CircuitBreaker::new(1, Duration::from_millis(0));
    breaker.record_failure(); // trips circuit: Closed → Open

    // With 0ms cool-off, state() should transition to HalfOpen.
    // Give the implementation a moment to observe elapsed time.
    std::thread::sleep(Duration::from_millis(5));

    let s = breaker.state();
    assert_eq!(
        s,
        CircuitState::HalfOpen,
        "after cool-off elapsed, state must be HalfOpen, got {s:?}"
    );

    // Test request succeeds: HalfOpen → Closed.
    breaker.record_success();
    assert_eq!(
        breaker.state(),
        CircuitState::Closed,
        "success in HalfOpen must transition circuit back to Closed"
    );
    assert!(!breaker.is_open());
}

/// Exercises postcondition: HalfOpen failure re-opens the circuit.
#[test]
fn test_BC_3_03_002_halfopen_failure_reopens_circuit() {
    let breaker = CircuitBreaker::new(1, Duration::from_millis(0));
    breaker.record_failure(); // Closed → Open

    std::thread::sleep(Duration::from_millis(5)); // let cool-off elapse

    // Confirm we are in HalfOpen.
    assert_eq!(breaker.state(), CircuitState::HalfOpen);

    // Test request fails: HalfOpen → Open again.
    breaker.record_failure();
    assert!(
        breaker.is_open(),
        "failure in HalfOpen must re-open the circuit"
    );
}

/// VP-012: Sink Failure Affects Only That Sink.
/// Each CircuitBreaker instance has independent state; one opening does not
/// affect another.
#[test]
fn test_BC_3_03_002_vp012_independent_breaker_instances() {
    let breaker_a = CircuitBreaker::new(1, Duration::from_secs(60));
    let breaker_b = CircuitBreaker::new(1, Duration::from_secs(60));

    // Trip breaker_a.
    breaker_a.record_failure();

    assert!(breaker_a.is_open(), "breaker_a must be open");
    assert!(
        !breaker_b.is_open(),
        "breaker_b must remain closed — VP-012 isolation"
    );
}
