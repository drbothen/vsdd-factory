//! Tests for RetryPolicy jitter behaviour (S-4.04).
//!
//! AC: "Exponential backoff with jitter: delay = min(base * 2^n + jitter, max)"
//! Jitter must be in [0, base_delay_ms * jitter_factor].
//! Traces to: BC-3.NN.NNN-retry-policy-exponential-backoff

use sink_core::resilience::RetryPolicy;
use std::time::Duration;

/// Exercises postcondition: delay_for_attempt returns a value >= the
/// deterministic (no-jitter) delay.
///
/// Jitter is always non-negative, so the jittered delay is >= no-jitter delay.
#[test]
fn test_BC_3_03_002_jitter_delay_gte_no_jitter() {
    let policy = RetryPolicy {
        max_retries: 5,
        base_delay_ms: 100,
        max_delay_ms: 30_000,
        jitter_factor: 0.5,
    };
    for n in 0..5 {
        let no_jitter = policy.delay_for_attempt_no_jitter(n);
        let with_jitter = policy.delay_for_attempt(n);
        assert!(
            with_jitter >= no_jitter,
            "attempt {n}: jittered delay {with_jitter:?} must be >= no-jitter delay {no_jitter:?}"
        );
    }
}

/// Exercises postcondition: jitter is bounded above by base_delay_ms * jitter_factor.
///
/// delay_for_attempt(n) <= delay_for_attempt_no_jitter(n) + base_delay_ms * jitter_factor
/// (both before the max_delay_ms cap — so we use a large cap here).
#[test]
fn test_BC_3_03_002_jitter_bounded_above_by_jitter_factor() {
    let policy = RetryPolicy {
        max_retries: 5,
        base_delay_ms: 200,
        max_delay_ms: 1_000_000, // large cap to isolate jitter bound test
        jitter_factor: 0.25,
    };
    let max_jitter_ms = (200_f64 * 0.25).ceil() as u64; // 50ms ceiling
    for n in 0..5 {
        let no_jitter = policy.delay_for_attempt_no_jitter(n);
        let with_jitter = policy.delay_for_attempt(n);
        let diff = with_jitter.saturating_sub(no_jitter);
        assert!(
            diff <= Duration::from_millis(max_jitter_ms + 1),
            "attempt {n}: jitter {diff:?} exceeded bound of {max_jitter_ms}ms"
        );
    }
}

/// Exercises invariant: two consecutive calls to delay_for_attempt produce
/// different values (jitter is not constant / not zero when jitter_factor > 0).
///
/// This uses enough calls that the probability of all being equal by chance is
/// negligible (binomial: (1/N)^9 where N is the jitter range in ms).
#[test]
fn test_BC_3_03_002_jitter_produces_variation_across_calls() {
    let policy = RetryPolicy {
        max_retries: 20,
        base_delay_ms: 1_000,
        max_delay_ms: 100_000,
        jitter_factor: 0.5, // 500ms jitter window — ample variation
    };
    let n = 1u32;
    let delays: Vec<Duration> = (0..10).map(|_| policy.delay_for_attempt(n)).collect();
    let all_equal = delays.windows(2).all(|w| w[0] == w[1]);
    assert!(
        !all_equal,
        "10 calls to delay_for_attempt({n}) all returned {:?} — jitter not applied",
        delays[0]
    );
}

/// Exercises postcondition: jitter is zero when jitter_factor is 0.0.
#[test]
fn test_BC_3_03_002_jitter_zero_when_factor_is_zero() {
    let policy = RetryPolicy {
        max_retries: 5,
        base_delay_ms: 100,
        max_delay_ms: 30_000,
        jitter_factor: 0.0,
    };
    for n in 0..5 {
        let no_jitter = policy.delay_for_attempt_no_jitter(n);
        let with_jitter = policy.delay_for_attempt(n);
        assert_eq!(
            with_jitter, no_jitter,
            "attempt {n}: jitter_factor=0.0 must produce zero jitter"
        );
    }
}
