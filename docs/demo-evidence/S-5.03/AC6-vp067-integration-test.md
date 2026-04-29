# AC6 — VP-067 integration test — all 11 tests GREEN

**Story:** S-5.03 — WorktreeCreate / WorktreeRemove hook wiring
**AC:** AC6 — Integration test covers all 4 BCs via VP-067
**VP:** VP-067 — Worktree Hook Plugin Surface Invariant
**GREEN commit:** `8336cd0`

---

## Full test run output

Command:
```
PATH="~/.rustup/toolchains/1.95.0-aarch64-apple-darwin/bin:$PATH" \
  cargo test -p worktree-hooks
```

```text
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.16s
     Running unittests src/lib.rs (target/debug/deps/worktree_hooks-08a61c1d873e2928)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/main.rs (target/debug/deps/worktree_hooks-a30ac6a2b378d3e7)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/integration_test.rs (target/debug/deps/integration_test-254d5f39fb0fad6a)

running 11 tests
test worktree_integration::test_bc_4_07_001_unknown_event_name_no_emit ... ok
test worktree_integration::test_bc_4_07_001_missing_worktree_name_emits_empty_default ... ok
test worktree_integration::test_bc_4_07_002_missing_worktree_path_emits_empty_default ... ok
test worktree_integration::test_bc_4_07_002_unknown_worktree_remove_no_op ... ok
test worktree_integration::test_bc_4_07_001_002_no_subprocess_invoked ... ok
test worktree_integration::test_bc_4_07_001_002_no_file_reads ... ok
test worktree_integration::test_bc_4_07_002_worktree_remove_emits_required_fields ... ok
test worktree_integration::test_bc_4_07_001_worktree_create_emits_required_fields ... ok
test worktree_integration::test_bc_4_07_001_worktree_create_idempotent_refire ... ok
test worktree_integration::test_bc_4_07_003_hooks_json_template_has_worktree_create_and_remove ... ok
test worktree_integration::test_bc_4_07_004_hooks_registry_toml_has_worktree_create_and_remove ... ok

test result: ok. 11 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

   Doc-tests worktree_hooks

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

---

## Test → BC → AC coverage map

| # | Test name | BC | AC | Notes |
|---|-----------|----|----|-------|
| 1 | `test_bc_4_07_001_worktree_create_emits_required_fields` | BC-4.07.001 (happy path) | AC2 | 10-field count; 2 plugin-set confirmed |
| 2 | `test_bc_4_07_001_worktree_create_idempotent_refire` | BC-4.07.001 EC-001 | AC2 | Re-fire on reconnect; stateless emit |
| 3 | `test_bc_4_07_001_missing_worktree_name_emits_empty_default` | BC-4.07.001 EC-003 | AC2 | Missing worktree_name → `""` default |
| 4 | `test_bc_4_07_002_worktree_remove_emits_required_fields` | BC-4.07.002 (happy path) | AC3 | 9-field count; 1 plugin-set confirmed |
| 5 | `test_bc_4_07_002_unknown_worktree_remove_no_op` | BC-4.07.002 EC-002 | AC3 | Unknown path emits normally |
| 6 | `test_bc_4_07_002_missing_worktree_path_emits_empty_default` | BC-4.07.002 EC-003 | AC3 | Missing worktree_path → `""` default |
| 7 | `test_bc_4_07_001_002_no_subprocess_invoked` | BC-4.07.001 Inv.2 + BC-4.07.002 Inv.2 + BC-4.07.004 PC.5-6 | AC2, AC3 | CountingMock == 0 for both events |
| 8 | `test_bc_4_07_001_002_no_file_reads` | BC-4.07.001 + BC-4.07.002 Option A | AC2, AC3 | Structural: no `read_file` param in API |
| 9 | `test_bc_4_07_001_unknown_event_name_no_emit` | BC-4.07.001/002 defensive | AC2, AC3 | Unknown event_name → 0 emits |
| 10 | `test_bc_4_07_003_hooks_json_template_has_worktree_create_and_remove` | BC-4.07.003 | AC1, AC4 | Layer 1 routing; `once` absent; async:true; timeout:10000 |
| 11 | `test_bc_4_07_004_hooks_registry_toml_has_worktree_create_and_remove` | BC-4.07.004 | AC1, AC5 | Layer 2 routing; two entries; zero caps; no `once` |

All 4 BCs (BC-4.07.001 through BC-4.07.004) covered. VP-067 satisfied.

---

## Harness design note

Tests use injectable-callback pattern (no WASM runtime required):

- `worktree_hook_logic` accepts `emit_fn: impl Fn(&str, &[(&str, &str)])`
- `dispatch_and_capture` simulates host enrichment (BC-1.05.012) and construction-time fields
- `CountingMock` tracks hypothetical `exec_subprocess` invocations (always 0; structural proof for Option A)
- `workspace_root()` walks up from `CARGO_MANIFEST_DIR` to find `Cargo.lock` — no hardcoded paths

This mirrors the VP-065/VP-066 harness pattern established by S-5.01 and S-5.02.

---

## Note on toolchain

The worktree's `rust-toolchain.toml` specifies `channel = "1.95.0"`.
The system PATH includes Homebrew's Rust 1.94.0 which takes precedence in this shell
environment, so tests must be run with the rustup toolchain on PATH:

```
PATH="$HOME/.rustup/toolchains/1.95.0-aarch64-apple-darwin/bin:$PATH" cargo test -p worktree-hooks
```

All 11 integration tests pass cleanly under 1.95.0 — matching the GREEN commit `8336cd0`.
