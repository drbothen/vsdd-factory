//! Tests for RetryPolicy exponential backoff formula (S-4.04).
//!
//! AC: "Exponential backoff with jitter: delay = min(base * 2^n + jitter, max)"
//! Traces to: BC-3.NN.NNN-retry-policy-exponential-backoff (v1.1 BC candidate)

use sink_core::resilience::{RetryPolicy};
use std::time::Duration;

/// Test vector table (from story AC formula: delay_n = min(base * 2^n, max)):
///
/// | n | base=100ms | multiplier=2 | expected (no jitter) |
/// |---|-----------|--------------|----------------------|
/// | 0 | 100ms     | 2^0=1        | 100ms                |
/// | 1 | 100ms     | 2^1=2        | 200ms                |
/// | 2 | 100ms     | 2^2=4        | 400ms                |
/// | 3 | 100ms     | 2^3=8        | 800ms                |
/// | 4 | 100ms     | 2^4=16       | 1600ms               |

/// Exercises BC-3.NN.NNN-retry-policy-exponential-backoff postcondition:
/// delay at attempt 0 equals base_delay_ms (no jitter variant).
#[test]
fn test_BC_3_03_002_retry_backoff_attempt_0_equals_base() {
    let policy = RetryPolicy {
        max_retries: 5,
        base_delay_ms: 100,
        max_delay_ms: 30_000,
        jitter_factor: 0.0,
    };
    let delay = policy.delay_for_attempt_no_jitter(0);
    assert_eq!(
        delay,
        Duration::from_millis(100),
        "attempt 0 delay must equal base_delay_ms"
    );
}

/// Exercises postcondition: delay at attempt 1 equals base * 2^1.
#[test]
fn test_BC_3_03_002_retry_backoff_attempt_1_doubles_base() {
    let policy = RetryPolicy {
        max_retries: 5,
        base_delay_ms: 100,
        max_delay_ms: 30_000,
        jitter_factor: 0.0,
    };
    let delay = policy.delay_for_attempt_no_jitter(1);
    assert_eq!(
        delay,
        Duration::from_millis(200),
        "attempt 1 delay must equal base * 2"
    );
}

/// Exercises postcondition: delay at attempt 2 equals base * 2^2.
#[test]
fn test_BC_3_03_002_retry_backoff_attempt_2_quadruples_base() {
    let policy = RetryPolicy {
        max_retries: 5,
        base_delay_ms: 100,
        max_delay_ms: 30_000,
        jitter_factor: 0.0,
    };
    let delay = policy.delay_for_attempt_no_jitter(2);
    assert_eq!(
        delay,
        Duration::from_millis(400),
        "attempt 2 delay must equal base * 4"
    );
}

/// Exercises postcondition: delay at attempt 3 equals base * 2^3.
#[test]
fn test_BC_3_03_002_retry_backoff_attempt_3_vector() {
    let policy = RetryPolicy {
        max_retries: 5,
        base_delay_ms: 100,
        max_delay_ms: 30_000,
        jitter_factor: 0.0,
    };
    let delay = policy.delay_for_attempt_no_jitter(3);
    assert_eq!(delay, Duration::from_millis(800));
}

/// Exercises the max_delay_ms cap postcondition:
/// computed delay exceeding max_delay_ms is clamped to max_delay_ms.
#[test]
fn test_BC_3_03_002_retry_backoff_capped_at_max_delay() {
    let policy = RetryPolicy {
        max_retries: 20,
        base_delay_ms: 1_000,
        max_delay_ms: 5_000,
        jitter_factor: 0.0,
    };
    // attempt 10: 1000 * 2^10 = 1_024_000ms — massively over the cap.
    let delay = policy.delay_for_attempt_no_jitter(10);
    assert_eq!(
        delay,
        Duration::from_millis(5_000),
        "delay must be clamped to max_delay_ms"
    );
}

/// Exercises postcondition: cap is enforced even at attempt 4
/// when base * 2^4 > max_delay_ms.
#[test]
fn test_BC_3_03_002_retry_backoff_cap_applied_exactly_at_threshold() {
    let policy = RetryPolicy {
        max_retries: 10,
        base_delay_ms: 500,
        max_delay_ms: 2_000,
        jitter_factor: 0.0,
    };
    // attempt 2: 500 * 4 = 2000ms — exactly the cap.
    let delay_at_cap = policy.delay_for_attempt_no_jitter(2);
    assert_eq!(delay_at_cap, Duration::from_millis(2_000));
    // attempt 3: 500 * 8 = 4000ms — over the cap, must clamp.
    let delay_over = policy.delay_for_attempt_no_jitter(3);
    assert_eq!(delay_over, Duration::from_millis(2_000));
}
