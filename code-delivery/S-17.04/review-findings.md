# S-17.04 PR #184 — Review Findings & Convergence Tracking

**PR:** https://github.com/drbothen/vsdd-factory/pull/184
**Branch:** feature/S-17.04-mid-burst-heartbeat-renewal-wiring
**Base:** develop

## Convergence Table

| Cycle | Head SHA | Findings | Blocking | Fixed | Remaining | Status |
|-------|----------|----------|----------|-------|-----------|--------|
| 1 | bf525c0f | 1 | 1 | — | 1 | REQUEST_CHANGES |
| 2 | 37468974 | 0 (F-001 closed) + 1 new (CI) | 1 | 1 | 1 | REQUEST_CHANGES (CI failure) |

## Cycle 1 Findings (bf525c0f)

| ID | Severity | Category | Finding | Route | Status |
|----|----------|----------|---------|-------|--------|
| F-001 | BLOCKING | coverage/spec-fidelity | `test_state_burst_skill_contains_renew_step` bats test required by AC-001 absent from verify-state-timestamp-refresh.bats | test-writer | FIXED in 37468974 |

## Cycle 2 Findings (37468974)

| ID | Severity | Category | Finding | Route | Status |
|----|----------|----------|---------|-------|--------|
| F-001 | — | — | CLOSED — test added, PASS (ok 7) confirmed in CI log | — | CLOSED |
| CI-001 | BLOCKING | CI/ordering | `cargo-host` job runs `Run verify-state-timestamp-refresh e2e tests (P5-M1)` with `CI_REQUIRE_ARTIFACTS=1` but the native `factory-dispatcher` binary is never built in `cargo-host` (only WASM plugins are built there; dispatcher is built in `build-dispatcher` matrix job). T-1 through T-6 all fail with `FAIL: factory-dispatcher binary not present in CI`. T-7 and T-8 (grep-based, no dispatcher) pass. | Implementer — fix ci.yml step ordering or build dispatcher in cargo-host | OPEN |

## CI-001 Detail

**Failure location:** `.github/workflows/ci.yml` — `Run verify-state-timestamp-refresh e2e tests (P5-M1)` step in the `cargo-host` job.

**Root cause:** The step comment states "This step runs AFTER the dispatcher build and WASM staging steps above, so both artifacts are guaranteed present." This is incorrect. The `cargo-host` job builds WASM plugins but does NOT build the native `factory-dispatcher` binary (that binary is only built in `build-dispatcher` matrix jobs). The step was placed after WASM staging, but the `_require_artifacts` check also requires `target/release/factory-dispatcher`, which does not exist in `cargo-host`.

**Evidence from CI log (macos job):**
```
not ok 1 T-1 test_verify_state_timestamp_refresh_continues_when_timestamps_advanced
# FAIL: factory-dispatcher binary not present in CI (CI_REQUIRE_ARTIFACTS=1) — run: cargo build --release -p factory-dispatcher
...
ok 7 test_state_burst_skill_contains_renew_step       ← PASSES (no dispatcher needed)
ok 8 test_verify_state_timestamp_refresh_registry_entry_has_correct_shape  ← PASSES
```

**Fix options (for orchestrator/implementer to choose):**
1. Add `cargo build --release -p factory-dispatcher` to the `cargo-host` job BEFORE the new bats e2e step. This is the minimal fix matching the step's own comment intent.
2. Move the bats e2e step to the `build-dispatcher` matrix job (where the dispatcher IS built), after the WASM staging step. Requires duplicating/adapting the step per platform.
3. Remove `CI_REQUIRE_ARTIFACTS=1` from the e2e step and revert to skip-on-missing behavior (loses the CI enforcement guarantee, which was the point of P5-M1).

**Option 1 is the minimal correct fix.** The dispatcher builds quickly (it's already compiled from `cargo test`'s incremental cache in the same job); a `--release` build step adds minimal CI time.

## Security Review

| Reviewer | Verdict | Finding count | Date |
|----------|---------|---------------|------|
| security-reviewer (claude-sonnet-4-6) | CLEAN | 0 | 2026-06-12 |

## Review Cycle Log

- **Cycle 1** (bf525c0f): pr-reviewer REQUEST_CHANGES — F-001 (missing AC-001 bats test). test-writer fixed in 37468974.
- **Cycle 2** (37468974): F-001 CLOSED (ok 7 in CI). New CI-001 discovered: cargo-host fails T-1..T-6 due to missing native dispatcher binary. CI-001 is a BLOCKER for merge.
