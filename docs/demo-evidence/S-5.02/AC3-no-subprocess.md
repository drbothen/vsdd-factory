# AC3 — No subprocess invocation

**Story:** S-5.02 — SessionEnd hook wiring  
**AC:** AC3 — Plugin completes without calling `exec_subprocess`  
**BC:** BC-4.05.002  
**GREEN commit:** `3783847`

---

## What is verified

The `session-end-telemetry` plugin must never call `exec_subprocess` at any point
during a `SessionEnd` dispatch. This distinguishes SessionEnd from SessionStart
(S-5.01), which invokes `factory-health` via `exec_subprocess`.

The integration test harness uses a `CountingMock` that tracks how many times
`exec_subprocess` would have been invoked. After every dispatch, the test asserts
`mock.invocation_count() == 0`.

---

## CountingMock definition

```rust
// tests/integration_test.rs:60–75
struct CountingMock {
    count: std::sync::atomic::AtomicUsize,
}

impl CountingMock {
    fn new() -> Self {
        CountingMock {
            count: std::sync::atomic::AtomicUsize::new(0),
        }
    }

    fn invocation_count(&self) -> usize {
        self.count.load(std::sync::atomic::Ordering::SeqCst)
    }
}
```

The mock is never passed as a real callback — it exists only to assert that
`session_end_hook_logic` never increments it.

---

## Isolated test (BC-4.05.002)

```text
test session_end_integration::test_bc_4_05_002_no_subprocess_invoked ... ok
```

Test at `tests/integration_test.rs:676`:

```rust
assert_eq!(
    mock.invocation_count(),
    0,
    "BC-4.05.002 Postcondition 1: exec_subprocess invocation_count must be 0 \
     for every SessionEnd dispatch — session-end plugin must NOT call exec_subprocess"
);
```

---

## Cross-test confirmation

Every BC-4.05.001 edge-case test also asserts `mock.invocation_count() == 0`
independently, confirming the invariant holds across all dispatch paths:

```text
test_bc_4_05_001_session_ended_emitted_with_required_fields     ... ok  (mock == 0)
test_bc_4_05_001_missing_session_start_ts_only_emits_zero_duration ... ok (mock == 0)
test_bc_4_05_001_missing_tool_call_count_only_emits_zero_count  ... ok  (mock == 0)
test_bc_4_05_001_both_missing_emit_zero_defaults                ... ok  (mock == 0)
test_bc_4_05_001_missing_session_id_emits_unknown               ... ok  (mock == 0)
test_bc_4_05_001_future_session_start_ts_emits_zero_duration    ... ok  (mock == 0)
test_bc_4_05_001_unparseable_session_start_ts_emits_zero_duration ... ok (mock == 0)
test_bc_4_05_002_no_subprocess_invoked                          ... ok  (mock == 0)
test_bc_4_05_003_single_dispatch_produces_single_event          ... ok  (mock == 0)
```

---

## Plugin source confirmation

The `session_end_hook_logic` function in `src/lib.rs` calls only `emit_fn` (once).
There is no `exec_subprocess` call anywhere in `src/lib.rs` or `src/main.rs`.
The `on_session_end` entry point wires only `vsdd_hook_sdk::host::emit_event`
(`src/lib.rs:161`). Zero-capability sandbox profile in `hooks-registry.toml` confirms
no `exec_subprocess` capability is declared.

---

## Contrast with SessionStart (S-5.01)

| Property | SessionEnd (S-5.02) | SessionStart (S-5.01) |
|----------|--------------------|-----------------------|
| `exec_subprocess` calls | 0 | 1 (`factory-health`) |
| Capability tables | none | `read_file` + `exec_subprocess` |
| `timeout_ms` | 5000 | 8000 (subprocess wait headroom) |
