---
document_type: fixture-readme
level: ops
version: "1.0"
status: stable
producer: validate-consistency
phase: skill-fixtures
---

# Tautology Detector Fixtures

These fixtures are reference inputs for **Check 8** (Test Tautology Detector).
They are NOT compiled or executed — they exist so anyone implementing or
auditing the check has canonical examples of what the predicate should
flag and what it should leave alone.

## Files

| File | Expected Verdict | Why |
|------|-----------------|-----|
| `flagged_tautological_test.rs` | FLAG | Constructs a struct, asserts on its own fields, never calls the production emitter |
| `clean_test_with_emitter_call.rs` | CLEAN | Same surface but actually calls `emit_log_entry(&entry)` — the assertion observes a real side effect |
| `clean_data_shape_pin.rs` | CLEAN (exception) | Tautological in shape but carries the `/// data-shape pin` comment that opts out of the check |
| `clean_test_with_emitter_arg.rs` | CLEAN | Constructs the struct then passes it to a production fn as an argument; assertions follow the call |

## How the predicate decides

The check fires when ALL of these hold:

1. Function name matches `test_BC_*`, `test_TV_*`, `test_*_BC_*`, or `test_*_TV_*`.
2. Body constructs a struct literal and binds it (e.g., `let entry = LogEntry { ... };`).
3. Body asserts on that struct's fields via `assert_eq!`/`assert!`/`assert_matches!`.
4. Body contains zero calls to functions whose names match the production-fn regex (`emit_*`, `process_*`, `apply_*`, `handle_*`, `execute_*`, `validate_*`, `compute_*`, `transform_*`, `render_*`, `serialize_*`, `encode_*`, `decode_*`, `parse_*`, `build_*`, `generate_*`).

If conditions 1–3 hold but condition 4 fails (i.e., a production fn IS called),
the test is exercising production code and is **not** a tautology.

## Adding new fixtures

When a real-world false-positive or false-negative is found, add a new file
prefixed `flagged_` or `clean_` with a short suffix. Update this README's table.
