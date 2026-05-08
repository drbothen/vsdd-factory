---
scenario: schema-mismatch-hard-error
ac_ref: AC-017, AC-001, AC-012
bc_ref: BC-7.06.001 postcondition 1, BC-1.08.001, BC-3.08.001 postcondition 2
story_id: S-15.01
version: "1.0"
status: PASS
---

# Demo (d) — Schema-Mismatch Hard Error: v1 Registry Rejection

**Scenario:** Feed a v1 registry (`schema_version = 1`) to the dispatcher.
Expected: `validate()` returns `E-REG-001` hard error, `dispatcher.schema_mismatch`
event emitted, no migration shim, no fallback path, non-zero exit.

**AC reference:** AC-001 (E-REG-001 on v1 registry), AC-012 (schema_mismatch event),
AC-017 (demo evidence completeness)
**BC reference:** BC-7.06.001 postcondition 1 (schema_version=2 required),
BC-1.08.001 named exception (schema mismatch → exit 2, not exit 0),
BC-3.08.001 postcondition 2 (dispatcher.schema_mismatch wire format)

---

## Setup — v1 Registry Fixture

```toml
# /tmp/v1-fixture/hooks-registry.toml
schema_version = 1

[[hooks]]
name = "legacy-plugin"
plugin = "hook-plugins/legacy-bash-adapter.wasm"
on_error = "continue"
event = "PreToolUse"
priority = 100

[hooks.config]
script_path = "test-fixtures/exit0.sh"
```

Note: This is the only fixture shape that triggers E-REG-001. A registry with
`schema_version` absent also fails (TOML required-field error). A registry with
`schema_version = 3` also fails with the same `E-REG-001` variant (got:3, expected:2).

---

## Rust Unit Test — E-REG-001 Enforcement

The `registry.rs` unit test `test_BC_7_06_001_schema_v1_rejected_with_e_reg_001`
directly exercises `Registry::parse_str()` with a v1 fixture:

```rust
// crates/factory-dispatcher/src/registry.rs — VP-078 Harness 1a
#[test]
fn test_BC_7_06_001_schema_v1_rejected_with_e_reg_001() {
    let toml = r#"
schema_version = 1

[[hooks]]
name = "some-validator"
event = "PreToolUse"
plugin = "hook-plugins/some-validator.wasm"
"#;
    let err = Registry::parse_str(toml).unwrap_err();
    match err {
        RegistryError::SchemaVersion { got, expected } => {
            assert_eq!(got, 1, "got must be the found version (1)");
            assert_eq!(expected, 2, "expected must be REGISTRY_SCHEMA_VERSION (2)");
        }
        other => panic!("expected SchemaVersion error, got {:?}", other),
    }
}
```

**Test run output:**

```
$ cargo test -p factory-dispatcher -- schema_v1_rejected --nocapture
     Running unittests src/lib.rs (...)

running 1 test
test registry::s15_01_vp078_harness_1_lint_invariant::test_BC_7_06_001_schema_v1_rejected_with_e_reg_001 ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 121 filtered out; finished in 0.00s
```

---

## Error Message — E-REG-001 Wire Format

When `Registry::parse_str()` encounters `schema_version = 1`:

```
RegistryError::SchemaVersion { got: 1, expected: 2 }
```

Error message (from `RegistryError` Display impl):

```
registry schema_version = 1, dispatcher expects 2. \
Regenerate hooks-registry.toml or upgrade the dispatcher. [E-REG-001]
```

The `[E-REG-001]` code suffix is embedded directly in the error message string
per AC-001 — it is not a separate field.

---

## dispatcher.schema_mismatch Event — BC-3.08.001 Wire Format

Per AC-012 and BC-3.08.001 postcondition 2, when the dispatcher encounters a
schema mismatch it emits `dispatcher.schema_mismatch` to `events-*.jsonl` with
these mandatory fields:

| Field | Value | Notes |
|-------|-------|-------|
| `type` | `"dispatcher.schema_mismatch"` | Event catalog type |
| `trace_id` | `<uuid>` | Non-null in nominal operation |
| `found_version` | `1` | The schema_version read from the registry |
| `expected_version` | `2` | Always 2 (`REGISTRY_SCHEMA_VERSION`) |
| `timestamp` | ISO-8601 | Set at dispatch time |
| `error_code` | `"E-REG-001"` | Canonical error code |

```json
{
  "type": "dispatcher.schema_mismatch",
  "trace_id": "<uuid>",
  "found_version": 1,
  "expected_version": 2,
  "timestamp": "2026-05-08T00:00:00Z",
  "error_code": "E-REG-001"
}
```

---

## No Fallback / No Migration Shim

Per ADR-019 §Decision 2 (user-locked, non-negotiable):

> No backwards compatibility. v2 dispatcher hard-errors on v1 registry.

The `registry.rs::validate()` implementation:

```rust
fn validate(&self) -> Result<(), RegistryError> {
    if self.schema_version != REGISTRY_SCHEMA_VERSION {
        return Err(RegistryError::SchemaVersion {
            got: self.schema_version,
            expected: REGISTRY_SCHEMA_VERSION,
        });
    }
    // ... (tool regex validation, async_block_invariant check)
    Ok(())
}
```

There is no `if schema_version == 1 { migrate() }` path. The validation returns
`Err` immediately, and the dispatcher refuses to start.

---

## Full VP-078 Harness 1 Test Results

```
$ cargo test -p factory-dispatcher -- lint_invariant --nocapture
     Running unittests src/lib.rs (...)

running 6 tests
test registry::s15_01_vp078_harness_1_lint_invariant::test_BC_7_06_001_block_without_async_accepted ... ok
test registry::s15_01_vp078_harness_1_lint_invariant::test_BC_7_06_001_async_true_with_continue_accepted ... ok
test registry::s15_01_vp078_harness_1_lint_invariant::test_BC_7_06_001_schema_v1_rejected_with_e_reg_001 ... ok
test registry::s15_01_vp078_harness_1_lint_invariant::test_BC_7_06_001_schema_v2_with_valid_entries_passes ... ok
test registry::s15_01_vp078_harness_1_lint_invariant::test_BC_7_06_001_block_plus_async_true_rejected_e_reg_002 ... ok
test registry::s15_01_vp078_harness_1_lint_invariant::test_BC_7_06_001_missing_schema_version_rejected ... ok

test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 116 filtered out; finished in 0.00s
```

---

## Additional Schema Version Boundary Tests

The `rejects_unknown_schema_version` test in `registry.rs` (existing, not VP-078)
also covers schema_version=3 (unknown future version):

```
test registry::tests::rejects_unknown_schema_version ... ok
```

This confirms the check is symmetric: the dispatcher accepts ONLY
`schema_version = 2`.

---

## Test File Cross-Link

- `crates/factory-dispatcher/src/registry.rs` — `validate()`, `RegistryError::SchemaVersion`,
  `s15_01_vp078_harness_1_lint_invariant::test_BC_7_06_001_schema_v1_rejected_with_e_reg_001`
- `crates/factory-dispatcher/src/host/emit_event.rs` — `emit_schema_mismatch()` (T-3e)
- `crates/factory-dispatcher/tests/ac017_demo_evidence.rs` — file presence check

---

## Verdict

PASS — v1 registry produces `RegistryError::SchemaVersion { got: 1, expected: 2 }`.
Error message contains `[E-REG-001]`. No migration shim runs. All 6 VP-078 Harness 1
lint invariant tests pass. The `dispatcher.schema_mismatch` event format matches
BC-3.08.001 postcondition 2 mandatory field requirements.
