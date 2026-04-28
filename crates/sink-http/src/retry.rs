//! Pure-core backoff formula for sink-http (S-4.09).
//!
//! This module is the pure-core half of the exponential-backoff-with-jitter
//! implementation mandated by BC-3.07.001. It contains no I/O, no async code,
//! and no tokio dependency — making every function synchronously unit-testable
//! without a runtime.
//!
//! The effectful shell (tokio::time::sleep, PRNG seeding) lives in `lib.rs`.
//!
//! ## Formula (BC-3.07.001 postcondition 1)
//!
//! ```text
//! delay_ms = min(base_delay_ms * 2^attempt + jitter_ms, max_delay_ms)
//! where jitter_ms ∈ [0, base_delay_ms * jitter_factor)
//! ```

/// Compute the backoff delay in milliseconds for a given retry attempt.
///
/// # Parameters
///
/// - `base_ms`: base delay in milliseconds (`base_delay_ms` from `RetryConfig`).
///   Must be > 0 (enforced at construction time by `RetryConfig::new`).
/// - `max_ms`: maximum delay cap in milliseconds. Result is clamped to this.
/// - `jitter_factor`: fraction of `base_ms` that forms the jitter window.
///   Jitter is drawn uniformly from `[0, base_ms * jitter_factor]`.
/// - `attempt`: zero-based retry attempt index (0 = first retry after first 5xx).
/// - `jitter_ms`: pre-drawn jitter value in milliseconds, already bounded to
///   `[0, base_ms * jitter_factor]`. The caller (effectful shell) draws this
///   from a per-instance PRNG to keep the formula pure.
///
/// # Returns
///
/// `delay_ms = min(base_ms * 2^attempt + jitter_ms, max_ms)`
///
/// Always `>= base_ms` when `attempt == 0` and `base_ms > 0`.
/// Always `<= max_ms`.
///
/// # Panics
///
/// This stub panics — implementation pending (S-4.09 implementer task).
pub fn compute_backoff_ms(
    base_ms: u64,
    max_ms: u64,
    _jitter_factor: f64,
    attempt: u32,
    jitter_ms: u64,
) -> u64 {
    // STUB: panics intentionally so RED-gate tests fail at runtime, not compile time.
    // The implementer replaces this with:
    //   let exponential = base_ms.saturating_mul(1_u64.saturating_shl(attempt));
    //   (exponential + jitter_ms).min(max_ms)
    let _ = (base_ms, max_ms, attempt, jitter_ms);
    unimplemented!("compute_backoff_ms not yet implemented (S-4.09)")
}

/// Draw a jitter value uniformly from `[0, base_ms * jitter_factor]`.
///
/// This is a pure helper exposed for testing; the effectful shell passes
/// an `rng: &mut impl Rng` from a per-instance source (AC-007 / invariant 2).
///
/// # Panics
///
/// Stub — panics until S-4.09 implementer fills this in.
pub fn draw_jitter_ms(base_ms: u64, jitter_factor: f64, random_unit: f64) -> u64 {
    // STUB: random_unit is a pre-drawn f64 in [0.0, 1.0) from the caller's PRNG.
    // Implementation: (base_ms as f64 * jitter_factor * random_unit) as u64
    let _ = (base_ms, jitter_factor, random_unit);
    unimplemented!("draw_jitter_ms not yet implemented (S-4.09)")
}

#[cfg(test)]
mod tests {
    use super::*;

    // ── AC-001: backoff formula min(base * 2^N + jitter, max) ────────────────

    /// test_BC_3_07_001_formula_attempt0_no_jitter
    ///
    /// AC-001 / BC-3.07.001 postcondition 1 canonical test vector:
    /// base=100ms, max=5000ms, attempt=0, jitter=0 → delay == 100ms.
    ///
    /// Verifies the base case of the exponential formula with zero jitter.
    /// Calls compute_backoff_ms — must invoke production code, not a struct field.
    #[test]
    fn test_BC_3_07_001_formula_attempt0_no_jitter() {
        let delay = compute_backoff_ms(100, 5000, 0.5, 0, 0);
        assert_eq!(
            delay, 100,
            "attempt=0, jitter=0: expected 100ms, got {delay}ms"
        );
    }

    /// test_BC_3_07_001_formula_attempt1_no_jitter
    ///
    /// AC-001 / BC-3.07.001 postcondition 1:
    /// base=100ms, max=5000ms, attempt=1, jitter=0 → delay == 200ms.
    #[test]
    fn test_BC_3_07_001_formula_attempt1_no_jitter() {
        let delay = compute_backoff_ms(100, 5000, 0.5, 1, 0);
        assert_eq!(
            delay, 200,
            "attempt=1, jitter=0: expected 200ms, got {delay}ms"
        );
    }

    /// test_BC_3_07_001_formula_attempt2_no_jitter
    ///
    /// AC-001: base=100ms, attempt=2, jitter=0 → delay == 400ms.
    #[test]
    fn test_BC_3_07_001_formula_attempt2_no_jitter() {
        let delay = compute_backoff_ms(100, 5000, 0.5, 2, 0);
        assert_eq!(
            delay, 400,
            "attempt=2, jitter=0: expected 400ms, got {delay}ms"
        );
    }

    /// test_BC_3_07_001_formula_attempt0_max_jitter
    ///
    /// AC-001 / BC-3.07.001 canonical vector:
    /// base=100ms, max=5000ms, jitter_factor=0.5, attempt=0 → delay in [100, 150]ms.
    ///
    /// Uses maximum jitter (50ms) to verify the upper bound of the range.
    #[test]
    fn test_BC_3_07_001_formula_attempt0_max_jitter() {
        // Max jitter for base=100, factor=0.5 is 50ms.
        let delay = compute_backoff_ms(100, 5000, 0.5, 0, 50);
        assert_eq!(
            delay, 150,
            "attempt=0, jitter=50ms: expected 150ms, got {delay}ms"
        );
    }

    /// test_BC_3_07_001_formula_attempt1_max_jitter
    ///
    /// AC-001 / BC-3.07.001 canonical vector:
    /// base=100ms, max=5000ms, jitter_factor=0.5, attempt=1 → delay in [200, 250]ms.
    ///
    /// Uses maximum jitter (50ms) to verify upper bound.
    #[test]
    fn test_BC_3_07_001_formula_attempt1_max_jitter() {
        let delay = compute_backoff_ms(100, 5000, 0.5, 1, 50);
        assert_eq!(
            delay, 250,
            "attempt=1, jitter=50ms: expected 250ms, got {delay}ms"
        );
    }

    /// test_BC_3_07_001_formula_attempt0_midpoint_jitter
    ///
    /// AC-001: base=100, attempt=0, jitter=25ms → delay == 125ms.
    /// Midpoint of the jitter window.
    #[test]
    fn test_BC_3_07_001_formula_attempt0_midpoint_jitter() {
        let delay = compute_backoff_ms(100, 5000, 0.5, 0, 25);
        assert_eq!(
            delay, 125,
            "attempt=0, jitter=25ms: expected 125ms, got {delay}ms"
        );
    }

    // ── AC-002: strictly positive delay (nonzero floor) ───────────────────────

    /// test_BC_3_07_001_strictly_positive_attempt0
    ///
    /// AC-002 / BC-3.07.001 postcondition 2:
    /// delay_ms > 0 when base_delay_ms > 0, even at attempt=0 with zero jitter.
    #[test]
    fn test_BC_3_07_001_strictly_positive_attempt0() {
        let delay = compute_backoff_ms(1, 5000, 0.5, 0, 0);
        assert!(
            delay > 0,
            "delay must be strictly positive at attempt=0 with base=1ms; got {delay}"
        );
    }

    /// test_BC_3_07_001_strictly_positive_large_attempt
    ///
    /// AC-002: delay remains strictly positive at attempt=10 (clamped to max, still > 0).
    #[test]
    fn test_BC_3_07_001_strictly_positive_large_attempt() {
        let delay = compute_backoff_ms(100, 5000, 0.5, 10, 0);
        assert!(
            delay > 0,
            "delay must be strictly positive at attempt=10; got {delay}"
        );
    }

    // ── AC-003: max_delay_ms is a hard cap (jitter cannot push above) ─────────

    /// test_BC_3_07_001_clamped_at_max_attempt6
    ///
    /// AC-003 / BC-3.07.001 canonical vector:
    /// base=100ms, max=5000ms, attempt=6 → delay == 5000ms (clamped).
    ///
    /// At attempt=6: base * 2^6 = 6400ms > 5000ms cap.
    #[test]
    fn test_BC_3_07_001_clamped_at_max_attempt6() {
        let delay = compute_backoff_ms(100, 5000, 0.5, 6, 0);
        assert_eq!(
            delay, 5000,
            "attempt=6 must be clamped to max=5000ms; got {delay}ms"
        );
    }

    /// test_BC_3_07_001_clamped_at_max_attempt10
    ///
    /// AC-003: base=100, max=5000, attempt=10 → delay == 5000ms.
    /// Far past the cap; ensures clamping is robust.
    #[test]
    fn test_BC_3_07_001_clamped_at_max_attempt10() {
        let delay = compute_backoff_ms(100, 5000, 0.5, 10, 0);
        assert_eq!(
            delay, 5000,
            "attempt=10 must be clamped to max=5000ms; got {delay}ms"
        );
    }

    /// test_BC_3_07_001_jitter_cannot_exceed_max
    ///
    /// AC-003 / EC-006: jitter that would push delay above max_delay_ms is clamped.
    ///
    /// attempt=5: base * 2^5 = 3200ms + 50ms jitter = 3250ms < 5000 (not clamped).
    /// attempt=6: base * 2^6 = 6400ms + 50ms jitter; clamp to 5000ms regardless of jitter.
    #[test]
    fn test_BC_3_07_001_jitter_cannot_exceed_max() {
        // Even with maximum jitter, result must not exceed max_ms.
        let delay = compute_backoff_ms(100, 5000, 0.5, 6, 50);
        assert_eq!(
            delay, 5000,
            "jitter must not push delay above max_delay_ms=5000ms; got {delay}ms"
        );
    }

    /// test_BC_3_07_001_cap_equals_base_attempt0
    ///
    /// AC-003: When max == base, delay is always exactly max regardless of attempt.
    #[test]
    fn test_BC_3_07_001_cap_equals_base_attempt0() {
        let delay = compute_backoff_ms(200, 200, 0.0, 0, 0);
        assert_eq!(
            delay, 200,
            "when max==base, delay must equal max=200ms; got {delay}ms"
        );
    }

    // ── draw_jitter_ms tests ──────────────────────────────────────────────────

    /// test_BC_3_07_001_jitter_draw_zero_random
    ///
    /// AC-001: draw_jitter_ms with random_unit=0.0 must return 0.
    #[test]
    fn test_BC_3_07_001_jitter_draw_zero_random() {
        let jitter = draw_jitter_ms(100, 0.5, 0.0);
        assert_eq!(
            jitter, 0,
            "random_unit=0.0 must yield jitter=0; got {jitter}"
        );
    }

    /// test_BC_3_07_001_jitter_draw_max_random
    ///
    /// AC-001: draw_jitter_ms with random_unit≈1.0 must return ≤ base*factor.
    /// Uses 0.9999 to stay within the half-open interval.
    #[test]
    fn test_BC_3_07_001_jitter_draw_max_random() {
        let jitter = draw_jitter_ms(100, 0.5, 0.9999);
        // Max expected: (100 * 0.5 * 0.9999) as u64 = 49
        assert!(
            jitter <= 50,
            "jitter must not exceed base*factor=50ms; got {jitter}"
        );
    }
}
