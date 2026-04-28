# AC6 — VP-066 integration test — all 11 tests GREEN

**Story:** S-5.02 — SessionEnd hook wiring  
**AC:** AC6 — Integration test covers all 5 BCs via VP-066  
**VP:** VP-066 — Session-End Plugin Surface Invariant  
**GREEN commit:** `3783847`

---

## Full test run output

Command:
```
RUSTC=~/.rustup/toolchains/1.95.0-aarch64-apple-darwin/bin/rustc \
  cargo test -p session-end-telemetry
```

```text
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.08s
     Running unittests src/lib.rs (target/debug/deps/session_end_telemetry-47f7d0cd534f674e)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/main.rs (target/debug/deps/session_end_telemetry-2e5d4c5658acc5b9)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/integration_test.rs (target/debug/deps/integration_test-177de9085b1f6038)

running 11 tests
test session_end_integration::test_bc_4_05_001_future_session_start_ts_emits_zero_duration ... ok
test session_end_integration::test_bc_4_05_001_missing_session_id_emits_unknown ... ok
test session_end_integration::test_bc_4_05_001_missing_session_start_ts_only_emits_zero_duration ... ok
test session_end_integration::test_bc_4_05_001_missing_tool_call_count_only_emits_zero_count ... ok
test session_end_integration::test_bc_4_05_002_no_subprocess_invoked ... ok
test session_end_integration::test_bc_4_05_001_both_missing_emit_zero_defaults ... ok
test session_end_integration::test_bc_4_05_001_unparseable_session_start_ts_emits_zero_duration ... ok
test session_end_integration::test_bc_4_05_003_single_dispatch_produces_single_event ... ok
test session_end_integration::test_bc_4_05_004_hooks_json_template_has_session_end ... ok
test session_end_integration::test_bc_4_05_001_session_ended_emitted_with_required_fields ... ok
test session_end_integration::test_bc_4_05_005_hooks_registry_toml_has_session_end ... ok

test result: ok. 11 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s
```

---

## Test → BC → AC coverage map

| # | Test name | BC | AC | Line |
|---|-----------|----|----|------|
| 1 | `test_bc_4_05_001_session_ended_emitted_with_required_fields` | BC-4.05.001 (happy path) | AC2 | 269 |
| 2 | `test_bc_4_05_001_missing_session_start_ts_only_emits_zero_duration` | BC-4.05.001 EC-001a | AC2 | 397 |
| 3 | `test_bc_4_05_001_missing_tool_call_count_only_emits_zero_count` | BC-4.05.001 EC-002 | AC2 | 442 |
| 4 | `test_bc_4_05_001_both_missing_emit_zero_defaults` | BC-4.05.001 EC-003 | AC2 | 494 |
| 5 | `test_bc_4_05_001_missing_session_id_emits_unknown` | BC-4.05.001 EC-004 | AC2 | 536 |
| 6 | `test_bc_4_05_001_future_session_start_ts_emits_zero_duration` | BC-4.05.001 EC-001b | AC2 | 581 |
| 7 | `test_bc_4_05_001_unparseable_session_start_ts_emits_zero_duration` | BC-4.05.001 EC-001c | AC2 | 632 |
| 8 | `test_bc_4_05_002_no_subprocess_invoked` | BC-4.05.002 | AC3 | 676 |
| 9 | `test_bc_4_05_003_single_dispatch_produces_single_event` | BC-4.05.003 | AC1, AC6 | 709 |
| 10 | `test_bc_4_05_004_hooks_json_template_has_session_end` | BC-4.05.004 | AC1, AC4 | 749 |
| 11 | `test_bc_4_05_005_hooks_registry_toml_has_session_end` | BC-4.05.005 | AC1, AC5 | 816 |

All 5 BCs (BC-4.05.001 through BC-4.05.005) covered. VP-066 satisfied.

---

## Harness design note

Tests use injectable-callback pattern (no WASM runtime required):

- `session_end_hook_logic` accepts `emit_fn: impl FnOnce(&[(&str, &str)])` 
- `dispatch_and_capture` simulates host enrichment (BC-1.05.012) and construction-time fields
- `CountingMock` tracks hypothetical `exec_subprocess` invocations (always 0)
- `workspace_root()` walks up from `CARGO_MANIFEST_DIR` to find `Cargo.lock` — no hardcoded paths

This mirrors the VP-065 harness pattern established by S-5.01.

---

## Note on toolchain

The worktree's `rust-toolchain.toml` specifies `channel = "1.95.0"`.
The system PATH includes Homebrew's Rust 1.94.0 which takes precedence in this shell
environment, so tests must be run with the rustup toolchain binary directly.
The Homebrew rustdoc triggers a doctest compile error (mixed toolchain), but all
11 integration tests pass cleanly under 1.95.0 — matching the GREEN commit `3783847`.
