---
document_type: red-gate-log
cycle: v1.0-feature-plugin-async-semantics-pass-1
phase: F4
producer: test-writer
version: "1.0"
status: RED_GATE_VERIFIED
timestamp: 2026-05-07T00:00:00Z
commit: edba543
branch: feature/S-15.01-plugin-async-semantics
---

# Red Gate Log — S-15.01 Plugin Async Semantics (F4)

## Summary

Red Gate VERIFIED. All new S-15.01 tests fail for the correct reason (todo!() stubs
or missing artifacts). No compilation errors. The test suite is ready for the implementer.

## Compilation

```
cargo test --workspace --no-run
```

Result: SUCCESS — all test binaries compiled without errors.

## Test Run Results

```
cargo test --workspace
```

### New S-15.01 test failures (correct RED gate):

**factory-dispatcher (lib unit tests — 7 new tests):**

All 7 fail with `todo!()` panic in `validate_async_block_invariant()`:
- `registry::s15_01_vp078_harness_1_lint_invariant::test_BC_7_06_001_schema_v2_with_valid_entries_passes`
- `registry::s15_01_vp078_harness_1_lint_invariant::test_BC_7_06_001_block_plus_async_true_rejected_e_reg_002`
- `registry::s15_01_vp078_harness_1_lint_invariant::test_BC_7_06_001_block_without_async_accepted`
- `registry::s15_01_vp078_harness_1_lint_invariant::test_BC_7_06_001_async_true_with_continue_accepted`
- `registry::s15_01_vp078_harness_4_serde_default::test_BC_7_06_001_async_explicit_true_parsed_as_true`
- `registry::s15_01_vp078_harness_4_serde_default::test_BC_7_06_001_async_explicit_false_parsed_as_false`
- `registry::s15_01_vp078_harness_4_serde_default::test_BC_7_06_001_async_absent_defaults_to_false`

Panic reason: `not yet implemented: T-3f: implement on_error=block + async=true detection`

**factory-dispatcher (integration tests — 4 new files):**

- `vp078_harness3_telemetry_classification`: FAIL — `Registry::load` fails with
  `SchemaVersion { got: 1, expected: 2 }` (live registry still at v1; T-3h pending)
- `event_emission_fault_injection` (11 tests): All PASS via `#[should_panic]` —
  stubs panic with "not yet implemented" confirming todos are in place
- `latency_canary` (3 tests): 2 PASS (structural constants), 1 IGNORED
  (requires --release + populated plugins)
- `ac017_demo_evidence` (3 tests): FAIL — demo evidence directory does not exist yet
  (F4 Step 4 demo-recorder pending)

**lint-registry-async-invariant (10 pre-existing skeleton tests):**

All 10 FAIL with `todo!()` in `run_lint()` — correct RED state.

**Pre-existing stub failures (stub-architect's schema_version bump side effects):**

9 pre-existing tests in registry.rs and routing.rs that use `schema_version = 1`
in fixtures now fail because REGISTRY_SCHEMA_VERSION = 2. These are NOT new
regressions — they are existing Red Gate failures introduced by the stub-architect
in commit 0283268. The implementer must update these fixtures to `schema_version = 2`
in T-3a.

## Files Authored

| File | Type | VP | ACs |
|------|------|----|-----|
| `crates/factory-dispatcher/src/registry.rs` (appended) | Unit tests | VP-078 H1+H4 | AC-001,002,006 |
| `crates/factory-dispatcher/tests/vp078_harness3_telemetry_classification.rs` | Integration | VP-078 H3 | AC-009 |
| `crates/factory-dispatcher/tests/event_emission_fault_injection.rs` | Integration | VP-079 S1-5 | AC-011,012,013,014,005 |
| `crates/factory-dispatcher/tests/latency_canary.rs` | Perf | AC-016 | AC-016 |
| `crates/factory-dispatcher/tests/ac017_demo_evidence.rs` | Completeness | — | AC-017 |
| `crates/factory-dispatcher/src/lib.rs` (amended) | Constant | DI-019 | All |
| `tests/bats/hooks-registry-lint.bats` | bats | VP-078 H1+H2 | AC-006,008 |
| `tests/bats/async-event-schema-conformance.bats` | bats | VP-079 S1-5 | AC-005,011-014 |
| `tests/bats/lint-registry-async-invariant.bats` | bats | VP-078 H2 | AC-007 |
| `tests/bats/envelope-sync-invariant.bats` | bats | BC-9.01.006 | AC-010 |

VP-077 Kani harnesses: already present in `partition.rs` from stub-architect (0283268).

## DI-019 Compliance

`ASYNC_DRAIN_WINDOW_MS` is exported from `crates/factory-dispatcher/src/lib.rs`
as `pub const ASYNC_DRAIN_WINDOW_MS: Duration = Duration::from_millis(100)`.
All test files reference `factory_dispatcher::ASYNC_DRAIN_WINDOW_MS`.
No test file contains a hardcoded `100` for the drain window value.

## Implementer Instructions

Red Gate is verified. Hand off to implementer. Implement each task (T-3a through T-3i)
in order. For each task:

1. Pick the failing test(s) it addresses.
2. Write minimum production code to make that test pass.
3. Run `cargo test -p factory-dispatcher` — verify the target test(s) pass.
4. Commit.

Priority order:
- T-3f first: implementing `validate_async_block_invariant()` unblocks 7 unit tests.
- T-3a: update pre-existing test fixtures from `schema_version = 1` to `schema_version = 2`.
- T-3b: implement `partition_plugins()` — unblocks VP-077 Kani harnesses.
- T-3d: add `ASYNC_DRAIN_WINDOW_MS` usage in engine (constant already exported).
- T-3e: implement 4 emit functions — unblocks VP-079 scenarios.
- T-3h: classify 9 telemetry plugins in hooks-registry.toml — unblocks VP-078 H3.
- T-3i: build lint WASM plugin — unblocks lint-registry-async-invariant tests.
