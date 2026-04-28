---
story: S-5.02
wave: 16
phase: red-gate
timestamp: 2026-04-28T00:00:00Z
agent: test-writer
status: RED_GATE_VERIFIED
---

# Red Gate Log — S-5.02: SessionEnd hook wiring

## Summary

11 integration tests written covering all 5 behavioral contracts (BC-4.05.001–005) per VP-066.
10 tests FAIL (RED gate verified). 1 test passes (test_bc_4_05_004 — template entry pre-exists).

## Test Files Created

| File | Action | Test count |
|------|--------|-----------|
| `crates/hook-plugins/session-end-telemetry/Cargo.toml` | created | crate manifest |
| `crates/hook-plugins/session-end-telemetry/src/lib.rs` | created | plugin skeleton (unimplemented! stub) |
| `crates/hook-plugins/session-end-telemetry/src/main.rs` | created | WASI entry point |
| `crates/hook-plugins/session-end-telemetry/tests/integration_test.rs` | created | 11 integration tests |
| `Cargo.toml` (workspace root) | modified | added session-end-telemetry to members |

## Production Stubs Added (minimal, panic-based)

| Symbol | File | Stub type |
|--------|------|-----------|
| `pub fn session_end_hook_logic<Emit>(payload, emit_fn) -> HookResult` | `src/lib.rs` | `unimplemented!("S-5.02 GREEN")` |
| `pub fn now_timestamp() -> String` | `src/lib.rs` | implemented (pure; no side effects) |
| `pub fn now_ms() -> i64` | `src/lib.rs` | implemented (pure; no side effects) |
| `pub fn compute_duration_ms(session_start_ts, now_ms) -> String` | `src/lib.rs` | implemented (pure; no side effects) |
| `pub fn on_session_end(payload) -> HookResult` | `src/lib.rs` | delegates to unimplemented stub |

Note: `now_timestamp`, `now_ms`, and `compute_duration_ms` are implemented as pure helpers
so the integration tests can use them to construct fixtures. Their implementations are
correct and do not affect the RED gate — the RED gate fires on `session_end_hook_logic`
calling `unimplemented!()`.

## Test Count by BC

| BC | Test | Status |
|----|------|--------|
| BC-4.05.001 (happy path) | `test_bc_4_05_001_session_ended_emitted_with_required_fields` | FAIL (unimplemented!) |
| BC-4.05.001 EC-001a | `test_bc_4_05_001_missing_session_start_ts_only_emits_zero_duration` | FAIL (unimplemented!) |
| BC-4.05.001 EC-002 | `test_bc_4_05_001_missing_tool_call_count_only_emits_zero_count` | FAIL (unimplemented!) |
| BC-4.05.001 EC-003 | `test_bc_4_05_001_both_missing_emit_zero_defaults` | FAIL (unimplemented!) |
| BC-4.05.001 EC-004 | `test_bc_4_05_001_missing_session_id_emits_unknown` | FAIL (unimplemented!) |
| BC-4.05.001 EC-001b | `test_bc_4_05_001_future_session_start_ts_emits_zero_duration` | FAIL (unimplemented!) |
| BC-4.05.001 EC-001c | `test_bc_4_05_001_unparseable_session_start_ts_emits_zero_duration` | FAIL (unimplemented!) |
| BC-4.05.002 | `test_bc_4_05_002_no_subprocess_invoked` | FAIL (unimplemented!) |
| BC-4.05.003 | `test_bc_4_05_003_single_dispatch_produces_single_event` | FAIL (unimplemented!) |
| BC-4.05.004 | `test_bc_4_05_004_hooks_json_template_has_session_end` | PASS (entry pre-exists) |
| BC-4.05.005 | `test_bc_4_05_005_hooks_registry_toml_has_session_end` | FAIL (entry missing — GREEN task) |

## Build Result

```
cargo build -p session-end-telemetry  →  SUCCESS (0 errors, 2 unused-variable warnings from unimplemented! stub)
```

## Test Failure Summary

10 of 11 tests FAIL.

### Tests 1–9 (BC-4.05.001–003): FAIL with unimplemented! panic

All 9 tests panic at `src/lib.rs:106` with:
```
not implemented: S-5.02 GREEN
```
This is the deterministic RED signal: `session_end_hook_logic` body is `unimplemented!("S-5.02 GREEN")`.

### Test 11 (BC-4.05.005): FAIL with assertion error

```
assertion `left == right` failed: BC-4.05.005: exactly one SessionEnd entry must be present;
found 0 entries
```
The `hooks-registry.toml` has no `SessionEnd` entry yet — this is a GREEN-phase task (Task 5).
The test correctly fails on RED.

## Pre-Existing Test: PASS

### Test 10 (BC-4.05.004): PASS

`test_bc_4_05_004_hooks_json_template_has_session_end` passes because the
`hooks.json.template` already contains a `SessionEnd` entry with the correct dispatcher binary
routing, `once: true`, `async: true`, and `timeout: 10000`. This is correct and documented in
the story spec (Task 4 is explicitly a no-op for S-5.02 since the entry pre-exists).
This does not weaken the RED gate — the remaining 10 tests all fail.

## RED Gate Verdict

RED gate VERIFIED: 10/11 tests fail. All failures are deterministic assertion errors or
`unimplemented!()` panics — not compile errors. The crate compiles cleanly. The RED gate is valid.

## Handoff to Implementer

Make each failing test pass by implementing `session_end_hook_logic` in `src/lib.rs`:

1. Extract `session_start_ts` from `payload.tool_input["session_start_ts"]` as an optional string
2. Extract `tool_call_count` from `payload.tool_input["tool_call_count"]` as an optional u64
3. Compute `duration_ms` using the already-implemented `compute_duration_ms()` helper
4. Format `tool_call_count` as a decimal string (default "0" if absent)
5. Generate `timestamp` using `now_timestamp()`
6. Call `emit_fn` with fields: [("duration_ms", ...), ("tool_call_count", ...), ("timestamp", ...)]
7. Return `HookResult::Continue`
8. Add `SessionEnd` entry to `plugins/vsdd-factory/hooks-registry.toml` (Task 5):
   ```toml
   [[hooks]]
   name = "session-end-telemetry"
   event = "SessionEnd"
   plugin = "hook-plugins/session-end-telemetry.wasm"
   timeout_ms = 5000
   ```
   NO `[hooks.capabilities.*]` tables. NO `once` field.
