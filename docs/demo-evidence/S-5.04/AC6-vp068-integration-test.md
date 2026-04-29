# AC6: VP-068 Integration Test — All 9 Tests GREEN

**AC:** AC6 — All 9 VP-068 test cases pass.
**VP:** VP-068 "Tool-Failure Hook Plugin Surface Invariant — All BC-4.08.* Postconditions Hold in Integration Test"
**BCs covered:** BC-4.08.001, BC-4.08.002, BC-4.08.003

## Test Run Output

**Command:**
```
PATH="$HOME/.rustup/toolchains/1.95.0-aarch64-apple-darwin/bin:$PATH" \
RUSTUP_TOOLCHAIN=1.95.0 \
cargo test -p tool-failure-hooks
```

**Output:**
```text
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running unittests src/lib.rs (target/debug/deps/tool_failure_hooks-c0246fcb39852274)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/main.rs (target/debug/deps/tool_failure_hooks-3ce0f1054eb5afbe)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/integration_test.rs (target/debug/deps/integration_test-c6e52f7a8c422b2e)

running 9 tests
test tool_failure_integration::test_bc_4_08_001_missing_tool_name_emits_unknown_sentinel ... ok
test tool_failure_integration::test_bc_4_08_001_error_message_exactly_1000_chars_no_truncation ... ok
test tool_failure_integration::test_bc_4_08_001_error_message_truncated_at_1000_chars ... ok
test tool_failure_integration::test_bc_4_08_001_missing_error_message_emits_empty_string ... ok
test tool_failure_integration::test_bc_4_08_001_tool_error_emitted_with_required_fields ... ok
test tool_failure_integration::test_bc_4_08_001_no_subprocess_no_read_file_invoked ... ok
test tool_failure_integration::test_bc_4_08_002_hooks_json_template_post_tool_use_failure_entry ... ok
test tool_failure_integration::test_bc_4_08_002b_platform_variants_in_sync ... ok
test tool_failure_integration::test_bc_4_08_003_hooks_registry_toml_post_tool_use_failure_entry ... ok

test result: ok. 9 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

   Doc-tests tool_failure_hooks

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

GREEN commit: `81e9fc4`

## Test-to-BC-to-AC Mapping

| # | Test Function | BC Anchored | AC Covered | What It Proves |
|---|--------------|-------------|-----------|----------------|
| 1 | `test_bc_4_08_001_tool_error_emitted_with_required_fields` | BC-4.08.001 PC-1/PC-2 | AC2 | Exactly 1 `tool.error` emitted; 10 fields total; 2 plugin-set (`tool_name`, `error_message`); 4 host-enriched; 4 construction-time; `type = "tool.error"` |
| 2 | `test_bc_4_08_001_missing_tool_name_emits_unknown_sentinel` | BC-4.08.001 EC-002 | AC3 | Absent `tool_name` → `"unknown"` sentinel (not empty string); plugin does not abort |
| 3 | `test_bc_4_08_001_error_message_truncated_at_1000_chars` | BC-4.08.001 EC-001 | AC3 | 1500-char input → exactly 1000 chars emitted; first 1000 chars preserved |
| 4 | `test_bc_4_08_001_error_message_exactly_1000_chars_no_truncation` | BC-4.08.001 EC-001 boundary | AC3 | 1000-char input → 1000 chars emitted verbatim (no truncation at boundary) |
| 5 | `test_bc_4_08_001_missing_error_message_emits_empty_string` | BC-4.08.001 EC-003 | AC3 | Absent `error_message` → `""` (not `"unknown"`; asymmetric with `tool_name` fallback) |
| 6 | `test_bc_4_08_001_no_subprocess_no_read_file_invoked` | BC-4.08.001 Invariants 1–2 | AC3 | `exec_subprocess` CountingMock = 0; `read_file` CountingMock = 0; event still emitted |
| 7 | `test_bc_4_08_002_hooks_json_template_post_tool_use_failure_entry` | BC-4.08.002 PC-1–5, Invariant 1 | AC1, AC4 | `PostToolUseFailure` key present; `command` contains `factory-dispatcher` (not `.wasm`); `once` absent; `async:true`; `timeout:10000` |
| 8 | `test_bc_4_08_002b_platform_variants_in_sync` | BC-4.08.002 Invariant 5 | AC1, AC4 | All 5 `hooks.json.*` variants exist and contain `PostToolUseFailure` key; `once` absent in each |
| 9 | `test_bc_4_08_003_hooks_registry_toml_post_tool_use_failure_entry` | BC-4.08.003 PC-1–7 | AC1, AC5 | `name`, `event`, `plugin` (with prefix), `timeout_ms` correct; NO capability tables; NO `once`; exactly one entry |

## Test File Location

`crates/hook-plugins/tool-failure-hooks/tests/integration_test.rs` (822 lines)

The test file uses `workspace_root()` (walks up from `CARGO_MANIFEST_DIR` to find `Cargo.lock`) for portable path resolution — same pattern as VP-065/VP-066/VP-067. The `dispatch_and_capture` helper simulates dispatcher host-enrichment without a WASM runtime, enabling fast native test execution.
