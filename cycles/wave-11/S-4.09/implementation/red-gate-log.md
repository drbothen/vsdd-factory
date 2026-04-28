---
story: S-4.09
wave: 11
phase: red-gate
timestamp: 2026-04-27T00:00:00Z
agent: test-writer
status: RED_GATE_VERIFIED
---

# Red Gate Log — S-4.09: sink-http exponential backoff with jitter

## Summary

28 new tests written. All 28 FAIL (RED gate verified). 10 pre-existing tests continue to PASS.

## Test Files Created / Modified

| File | Action | Test count |
|------|--------|-----------|
| `crates/sink-http/src/retry.rs` | created (pure-core stubs + tests) | 14 unit tests |
| `crates/sink-http/tests/bc_3_07_001_backoff.rs` | created | 14 integration tests |
| `crates/sink-http/src/lib.rs` | stub declarations added | 0 tests (production stubs only) |

## Production Stubs Added (minimal, panic-based)

The following declarations were added to production code to allow test compilation.
All implementations use `unimplemented!()` — no behavior present.

| Symbol | File | Stub type |
|--------|------|-----------|
| `pub mod retry;` | `src/lib.rs` | module declaration |
| `pub enum ConfigError { InvalidBackoff }` | `src/lib.rs` | enum |
| `pub struct RetryConfig { base_delay_ms, max_delay_ms, jitter_factor, max_retries }` | `src/lib.rs` | struct |
| `RetryConfig::new(...)` | `src/lib.rs` | `unimplemented!()` |
| `HttpSinkConfig.retry: Option<RetryConfig>` | `src/lib.rs` | field |
| `HttpSinkConfigBuilder.retry(RetryConfig)` | `src/lib.rs` | builder method |
| `pub fn compute_backoff_ms(...)` | `src/retry.rs` | `unimplemented!()` |
| `pub fn draw_jitter_ms(...)` | `src/retry.rs` | `unimplemented!()` |

## Test Count by AC

| AC | Tests | Names |
|----|-------|-------|
| AC-001 | 8 | `formula_attempt0_no_jitter`, `formula_attempt1_no_jitter`, `formula_attempt2_no_jitter`, `formula_attempt0_max_jitter`, `formula_attempt1_max_jitter`, `formula_attempt0_midpoint_jitter`, `jitter_draw_zero_random`, `jitter_draw_max_random` |
| AC-002 | 2 | `strictly_positive_attempt0`, `strictly_positive_large_attempt` |
| AC-003 | 4 | `clamped_at_max_attempt6`, `clamped_at_max_attempt10`, `jitter_cannot_exceed_max`, `cap_equals_base_attempt0` |
| AC-004 | 1 | `submit_returns_before_backoff_sleep` |
| AC-005 | 1 | `retry_uses_same_payload` |
| AC-006 | 4 | `rejects_base_zero`, `rejects_max_less_than_base`, `rejects_max_zero_base_nonzero`, `accepts_max_equals_base` |
| AC-007 | 1 | `per_instance_prng_uncorrelated` |
| AC-008 | 1 | `sleep_does_not_hold_mutex` |
| AC-009 | 3 | `exactly_n_minus_1_sleeps_full_failure`, `no_sleep_on_single_attempt`, `no_trailing_sleep_after_final_failure` |
| AC-010 | 2 | `wall_clock_delay_attempt0`, `wall_clock_delay_attempt1` |
| EC-004 | 1 | `4xx_no_backoff` |
| **Total** | **28** | |

## Build Result

```
cargo build -p sink-http --tests  →  SUCCESS (0 errors, warnings only)
```

## Test Failure Summary

All 28 new tests FAIL with `unimplemented!()` panics from the production stubs.
Failure category: `not implemented` (panics at call sites in production stubs).

### Unit tests (src/retry.rs #[cfg(test)]) — 14 tests FAIL

All fail with: `not implemented: compute_backoff_ms not yet implemented (S-4.09)`
or `not implemented: draw_jitter_ms not yet implemented (S-4.09)`

### Integration tests (tests/bc_3_07_001_backoff.rs) — 14 tests FAIL

All fail with: `not implemented: RetryConfig::new not yet implemented (S-4.09)`

## Pre-Existing Tests (no regression)

All 10 S-4.01 tests pass unchanged:

| File | Result |
|------|--------|
| `tests/contract_config_load.rs` (3 tests) | PASS |
| `tests/contract_sink_trait.rs` (2 tests) | PASS |
| `tests/error_handling.rs` (2 tests) | PASS |
| `tests/integration_post_batch.rs` (2 tests) | PASS |
| `tests/non_blocking.rs` (1 test) | PASS |

## Handoff to Implementer

Make each of the 28 tests pass by:
1. Implementing `compute_backoff_ms` in `src/retry.rs`: `(base_ms.saturating_mul(1u64 << attempt) + jitter_ms).min(max_ms)`
2. Implementing `draw_jitter_ms`: `(base_ms as f64 * jitter_factor * random_unit) as u64`
3. Implementing `RetryConfig::new` with construction-time validation (AC-006)
4. Wiring `compute_backoff_ms` + `tokio::time::sleep` into the worker loop after each 5xx (AC-001, AC-004, AC-008, AC-009)
5. Using per-instance `SmallRng::from_entropy()` stored on the sink struct (AC-007)
6. Ensuring no sleep after the final failed attempt (AC-009)
7. `rand` crate dependency must be added to `crates/sink-http/Cargo.toml` (already at `0.8` in `sink-core` but not in `sink-http`)
