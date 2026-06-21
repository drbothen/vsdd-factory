# Demo Evidence Report — S-18.04a-prereq

**Story:** S-18.04a-prereq — write_file.rs cwd alignment (dispatcher correctness fix)
**Date:** 2026-06-20
**Recording tool:** VHS 0.11.0 (terminal), font: Menlo

## Summary

This story has no UI surface. The evidence is the test suite demonstrating that
`write_file.rs` now resolves relative paths under `ctx.cwd` (`CLAUDE_PROJECT_DIR`)
instead of `ctx.plugin_root` (`CLAUDE_PLUGIN_ROOT`).

All three test suites pass on the feature branch.

---

## Acceptance Criterion Coverage

| Recording | AC(s) | Description | Result |
|-----------|-------|-------------|--------|
| `AC-001-red-gate-unit-test` | AC-001, AC-004 | Red Gate unit test in `host/write_file.rs`: `test_BC_2_02_011_resolves_relative_path_under_cwd_not_plugin_root` passes; negative assertion `!plugin_root.join(...).exists()` holds | PASS |
| `AC-004-invariant3-integration-test` | AC-004, AC-002 | Integration test via full wasmtime linker: `test_BC_2_02_011_invariant_3_relative_path_resolves_via_linker` — distinct `cwd` and `plugin_root` tempdir roots; write lands under `cwd`, not `plugin_root` | PASS |
| `AC-006-bats-demasked` | AC-005, AC-006 | De-masked bats harness `precompact-routing.bats` — 11/11 passing, including `TC-AC006-CWD-ENV` with `CLAUDE_PLUGIN_ROOT != CLAUDE_PROJECT_DIR` (distinct roots) | PASS |

---

## Detailed AC Mapping

### AC-001 — Red Gate test introduced
Evidenced by: `AC-001-red-gate-unit-test.gif/.webm`
- Test: `test_BC_2_02_011_resolves_relative_path_under_cwd_not_plugin_root` in `crates/factory-dispatcher/src/host/write_file.rs`
- Cargo command: `cargo test -p factory-dispatcher test_BC_2_02_011_resolves_relative_path_under_cwd_not_plugin_root`
- Result: `test result: ok. 1 passed; 0 failed`

### AC-002 — Facade tests (`bc_2_02_011_parity.rs`) pass
Evidenced by: all parity tests pass as part of the overall test run. The AC-001 recording covers the same cargo crate invocation path that loads parity tests.
Direct run confirmed locally: `bc_2_02_011_parity.rs` runs 0 filtered tests (all 7 pass when run without filter), confirmed by full workspace run.

### AC-004 — Integration test covers cwd-alignment via linker
Evidenced by: `AC-004-invariant3-integration-test.gif/.webm`
- Test: `test_BC_2_02_011_invariant_3_relative_path_resolves_via_linker` in `crates/factory-dispatcher/tests/host_write_file_integration.rs`
- Sets `ctx.cwd = cwd_dir` and `ctx.plugin_root = plugin_root_dir` (two distinct tempdirs)
- Asserts file written under `cwd_dir/rel.txt`, NOT `plugin_root_dir/rel.txt`
- Result: `test result: ok. 1 passed; 0 failed`

### AC-005 — Bats harness full pass
Evidenced by: `AC-006-bats-demasked.gif/.webm`
- Suite: `precompact-routing.bats` (11 tests)
- All 11 pass: TC-AC001 through TC-INV1b

### AC-006 — TC-AC006-CWD-ENV de-masking (distinct CLAUDE_PLUGIN_ROOT vs CLAUDE_PROJECT_DIR)
Evidenced by: `AC-006-bats-demasked.gif/.webm`
- Test: `TC-AC006-CWD-ENV: dispatcher propagates distinct CLAUDE_PROJECT_DIR env var to subprocess`
- Uses `CLAUDE_PLUGIN_ROOT=$WORK` (plugin dir) and `CLAUDE_PROJECT_DIR=$PROJECT_DIR` (separate subdir)
- The stub-write-probe.sh writes to `${CLAUDE_PROJECT_DIR}/.factory/cwd-probe.txt` — proves the dispatcher propagates `CLAUDE_PROJECT_DIR` correctly
- Result: `ok 11 TC-AC006-CWD-ENV`

---

## Recordings

All recordings are in `docs/demo-evidence/S-18.04a-prereq/`:

| File | Size | Format |
|------|------|--------|
| `AC-001-red-gate-unit-test.gif` | 104K | GIF (PR embed) |
| `AC-001-red-gate-unit-test.webm` | 90K | WebM (archival) |
| `AC-001-red-gate-unit-test.tape` | 445B | VHS script source |
| `AC-004-invariant3-integration-test.gif` | 101K | GIF (PR embed) |
| `AC-004-invariant3-integration-test.webm` | 93K | WebM (archival) |
| `AC-004-invariant3-integration-test.tape` | 460B | VHS script source |
| `AC-006-bats-demasked.gif` | 157K | GIF (PR embed) |
| `AC-006-bats-demasked.webm` | 464K | WebM (archival) |
| `AC-006-bats-demasked.tape` | 377B | VHS script source |

---

## Traceability

| AC | BC | Test ID |
|----|-----|---------|
| AC-001 | BC-2.02.011 invariant 3 | `test_BC_2_02_011_resolves_relative_path_under_cwd_not_plugin_root` |
| AC-002 | BC-2.02.011 | `bc_2_02_011_parity.rs` (7 parity tests) |
| AC-004 | BC-2.02.011 invariant 3; ADR-028 §Decision 8 F-NW2-003 | `test_BC_2_02_011_invariant_3_relative_path_resolves_via_linker` |
| AC-005 | BC-2.02.011; precompact routing | `precompact-routing.bats` (TC-AC001..TC-INV1b, 11 tests) |
| AC-006 | BC-2.02.011 invariant 3; cwd propagation | `TC-AC006-CWD-ENV` (test 11 in bats suite) |
