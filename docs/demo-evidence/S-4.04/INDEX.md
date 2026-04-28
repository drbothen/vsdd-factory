---
document_type: demo-evidence-index
story_id: S-4.04
producer: demo-recorder
phase: per-story-delivery
branch: feat/S-4.04-retry-circuit-breaker
tested_at_sha: 3a343af
timestamp: 2026-04-27T00:00:00Z
---

# S-4.04: Per-sink retry + circuit breaker — Evidence Index

**Story:** S-4.04 — Per-sink retry + circuit breaker  
**Branch:** `feat/S-4.04-retry-circuit-breaker`  
**SHA at test run:** `3a343af`  
**Test result:** 50/50 GREEN (11 existing + 39 new)

## Per-AC Evidence

| AC | Description | Evidence File | Tests | Result |
|----|-------------|---------------|-------|--------|
| AC-01 | Shared `RetryPolicy` struct: max_retries, base_delay_ms, max_delay_ms, jitter_factor — exponential backoff formula | AC-01-retry-policy-exponential-backoff.txt | 6 | GREEN |
| AC-02 | Exponential backoff with jitter: delay = min(base * 2^n + jitter, max) | AC-02-retry-policy-jitter.txt | 4 | GREEN |
| AC-03 | After max_attempts retries, returns Err carrying the last failure | AC-03-retry-policy-max-attempts.txt | 4 | GREEN |
| AC-04 | Successful operation resets retry / failure counter (EC-001) | AC-04-retry-policy-success-resets.txt | 3 | GREEN |
| AC-05 | Circuit breaker state machine: Closed → Open → HalfOpen → Closed (EC-002, EC-003) | AC-05-circuit-breaker-state-machine.txt | 7 | GREEN |
| AC-06 | Integration: retry + circuit open sequence; VP-011 async-sleep proof | AC-06-circuit-breaker-integration.txt | 4 | GREEN |
| AC-07 | `internal.sink_circuit_opened` / `internal.sink_circuit_closed` events emitted on state transitions | AC-07-circuit-events.txt | 3 | GREEN |
| AC-08 | Per-sink configurable retry policy and independent circuit breaker state (VP-012) | AC-08-per-sink-config.txt | 4 | GREEN |

## Test Count Summary

| Category | Count |
|----------|-------|
| Existing sink-core unit tests (src/lib.rs) | 11 |
| New resilience tests (8 test files) | 39 |
| **Total** | **50** |
| Failed | 0 |

## Build Hygiene

| Check | Result | Notes |
|-------|--------|-------|
| `cargo clippy -p sink-core -- -D warnings` | CLEAN (exit 0) | No warnings, no errors |
| `cargo fmt --check -p sink-core` | EXIT 1 | Pre-existing import ordering in test files only — see fmt-clean.txt |

## Verification Properties Covered

| VP | Title | Covered By |
|----|-------|-----------|
| VP-011 | Sink submit Must Not Block the Dispatcher | AC-06: `test_BC_3_03_002_vp011_with_retry_uses_async_sleep_not_thread_sleep` |
| VP-012 | Sink Failure Affects Only That Sink | AC-05: `test_BC_3_03_002_vp012_independent_breaker_instances`; AC-08: independent per-sink config tests |

## Anomalies / Deferred Items

1. **`tokio-resilience` feature does not gate tokio** — tokio is always compiled in even when the feature is absent. Tracked as follow-up in S-4.10 wiring. No test impact.

2. **Pre-existing rustfmt issues in test files** — `cargo fmt --check` exits 1 for import ordering in 5 test files. These were introduced in the RED-gate commit by the test-writer agent and are out of scope per test-writer ownership rules. Production source is fmt-clean.

3. **Homebrew cargo 1.94 vs rustup cargo 1.95** — Homebrew cargo shadows rustup on PATH on this machine. Tests were run via `PATH="$HOME/.cargo/bin:$PATH" cargo ...` to ensure rustup 1.95 was used. No mixed-compiler artifacts observed in output.

4. **Non-snake-case function names in test files** — all 35 BC-tracing test functions use `test_BC_3_03_002_*` naming for direct BC traceability. Rustc emits `non_snake_case` warnings for these. Warnings are in test files (test-writer scope) and do not constitute errors under `-D warnings` (clippy on production source). All tests pass.
