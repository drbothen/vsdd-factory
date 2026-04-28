---
document_type: demo-evidence-index
story_id: S-4.09
producer: demo-recorder
phase: per-story-delivery
branch: feat/S-4.09-sink-http-retry-backoff
tested_at_sha: 9d3b4b2
timestamp: 2026-04-27T00:00:00Z
---

# S-4.09: sink-http exponential backoff with jitter — Evidence Index

**Story:** S-4.09 — sink-http retry backoff with jitter
**Branch:** `feat/S-4.09-sink-http-retry-backoff`
**SHA at test run:** `9d3b4b20f27743f027107d0e7264d57326e0e656`
**Test result:** 38/38 GREEN (14 unit in retry.rs + 14 integration in bc_3_07_001_backoff.rs + 10 pre-existing)

## Per-AC Evidence

| AC | Description | Evidence File | Tests | Result |
|----|-------------|---------------|-------|--------|
| AC-001 | Backoff formula: `delay = min(base * 2^N + jitter, max)` with uniform jitter from `[0, base * jitter_factor]` | AC-001-backoff-formula.txt | 8 | GREEN |
| AC-002 | delay_ms strictly positive when base_delay_ms > 0, even at attempt 0 | AC-002-positive-delay.txt | 2 | GREEN |
| AC-003 | delay_ms never exceeds max_delay_ms; jitter cannot push above cap | AC-003-max-cap.txt | 4 | GREEN |
| AC-004 | submit() returns immediately; backoff sleep runs on worker thread (non-blocking) | AC-004-non-blocking-submit.txt | 1 | GREEN |
| AC-005 | Retry uses same payload as failed attempt (no mutation between retries) | AC-005-same-payload.txt | 1 | GREEN |
| AC-006 | ConfigError::InvalidBackoff when base=0 or max < base; sink does not start | AC-006-config-validation.txt | 4 | GREEN |
| AC-007 | Per-instance PRNG seeding; two concurrent sinks produce uncorrelated jitter | AC-007-per-instance-prng.txt | 1 | GREEN |
| AC-008 | Backoff sleep does not hold Mutex<Vec<SinkFailure>> lock | AC-008-lock-discipline.txt | 1 | GREEN |
| AC-009 | Exactly (N-1) sleeps on full-failure sequence; no trailing sleep after final attempt | AC-009-n-minus-1-sleeps.txt | 3 | GREEN |
| AC-010 | Wall-clock delay verification: attempt=0 in [100,150]ms, attempt=1 in [200,250]ms | AC-010-wall-clock.txt | 2 | GREEN |
| EC-004 | 4xx responses: no retry, no sleep, failure recorded immediately | EC-004-4xx-no-retry.txt | 1 | GREEN |

## Test Count Summary

| Category | Count |
|----------|-------|
| Unit tests in src/retry.rs (pure formula + jitter) | 14 |
| Integration tests in tests/bc_3_07_001_backoff.rs | 14 |
| Pre-existing tests (contract_config_load, contract_sink_trait, error_handling, integration_post_batch, non_blocking) | 10 |
| **Total** | **38** |
| Failed | 0 |

## Build Hygiene

| Check | Result | Notes |
|-------|--------|-------|
| `cargo clippy -p sink-http -- -D warnings` | CLEAN (exit 0) | No warnings, no errors on production source |
| `cargo fmt --check -p sink-http` | CLEAN (exit 0) | All files pass rustfmt — including test files (formatted in 9d3b4b2) |

## Behavioral Contract Coverage

| BC ID | Title | Evidence |
|-------|-------|---------|
| BC-3.07.001 | sink-http exponential backoff with jitter between 5xx retries | All 11 AC/EC evidence files (AC-001 through EC-004) |

## Verification Properties Covered

| VP | Title | Covered By |
|----|-------|-----------|
| VP-011 | Sink submit Must Not Block the Dispatcher | AC-004: `test_BC_3_07_001_submit_returns_before_backoff_sleep` |
| VP-012 | Sink Failure Affects Only That Sink | Pre-existing: `test_VP_012_5xx_retries_then_records_failure`, `test_VP_012_4xx_drops_immediately_no_retry` |

## Evidence Files

| File | Purpose |
|------|---------|
| AC-001-backoff-formula.txt | Formula correctness (8 unit tests) |
| AC-002-positive-delay.txt | Positive delay floor (2 unit tests) |
| AC-003-max-cap.txt | Hard max_delay_ms cap (4 unit tests) |
| AC-004-non-blocking-submit.txt | Non-blocking submit path (1 integration test) |
| AC-005-same-payload.txt | Same-payload retry (1 integration test) |
| AC-006-config-validation.txt | Construction-time validation (4 integration tests) |
| AC-007-per-instance-prng.txt | Per-instance PRNG uncorrelated (1 integration test) |
| AC-008-lock-discipline.txt | Mutex not held during sleep (1 integration test) |
| AC-009-n-minus-1-sleeps.txt | Exactly (N-1) sleeps (3 integration tests) |
| AC-010-wall-clock.txt | Wall-clock delay verification (2 integration tests) |
| EC-004-4xx-no-retry.txt | 4xx non-retryable (1 integration test) |
| all-tests-summary.txt | Full suite output: 38/38 GREEN |
| clippy-clean.txt | cargo clippy -p sink-http -- -D warnings: CLEAN |
| fmt-clean.txt | cargo fmt --check -p sink-http: CLEAN (exit 0) |

## Anomalies / Notes

1. **Non-snake-case test names** — all BC-tracing test functions use `test_BC_3_07_001_*`
   naming for direct BC traceability. Rustc emits `non_snake_case` warnings for these.
   Warnings are in test files (test-writer scope) and do not constitute errors under
   `-D warnings` on production source. Consistent with S-4.04 precedent.

2. **fmt-clean is full-clean** — Unlike S-4.04, S-4.09 test files are fully formatted
   (commit 9d3b4b2 was a rustfmt cleanup pass). `cargo fmt --check` exits 0 for all files.

3. **wall-clock tests are timing-sensitive** — AC-010 tests use 100ms base delay with
   ±50ms tolerance window. These pass consistently on the recording machine but may be
   sensitive to heavily loaded CI environments. A 2x tolerance multiplier would reduce
   flakiness without changing semantics (tracked as v1.1 candidate).
