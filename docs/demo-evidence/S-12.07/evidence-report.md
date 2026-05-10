# Demo Evidence Report — S-12.07

**Story:** S-12.07 — `vsdd-context-resolvers` Crate + WaveContextResolver (v1.4)
**Branch:** `feature/S-12.07-vsdd-context-resolvers`
**Evidence captured:** 2026-05-10
**Adversary convergence:** 8 passes, 3/3 NITPICK_ONLY streak (BC-5.39.001)

---

## AC Coverage Summary

| AC | Description | Status | Evidence File |
|----|-------------|--------|---------------|
| AC-001 | wave_context output shape (wave_id, cycle_id, stories) | GREEN | AC-001-wave-context-output-shape.txt |
| AC-002 | value: None on absent/malformed wave-state | GREEN | AC-002-absent-malformed-wave-state-yields-none.txt |
| AC-003 | empty waves / all-completed → value: None | GREEN | AC-003-empty-waves-yields-none.txt |
| AC-004 | optional fields no-panic + missing required field rejected | GREEN | AC-004-optional-fields-no-panic-missing-wave-field-rejected.txt |
| AC-005 | capability confinement — path_allow deny (VP-076-A/B/C) | DEFERRED → S-12.08 | AC-005-DEFERRED-capability-confinement-path-deny.txt |
| AC-006 | capability confinement — path_allow positive grant (VP-076-D) | DEFERRED → S-12.08 | AC-006-DEFERRED-capability-confinement-path-allow.txt |
| AC-007 | project_dir is top-level field (not nested in plugin_config) | GREEN | AC-007-project-dir-top-level-field.txt |
| AC-008 | VP-075 proptest 200 trials deterministic (release build) | GREEN | AC-008-proptest-200-trials-deterministic.txt |
| AC-009 | WASM artifact registered in resolvers-registry.toml | GREEN | AC-009-wasm-artifact-registered.txt |
| AC-010 | No unwrap/expect/panic in lib source (clippy + grep) | GREEN | AC-010-no-unwrap-expect-panic-in-lib.txt |

**Result: 8 GREEN, 2 DEFERRED (AC-005/006 → S-12.08)**

---

## GREEN ACs — Evidence Method

All GREEN ACs use cargo test output as evidence (CLI product, no VHS/Playwright needed).
Evidence files contain the exact command, captured output, and status.

### AC-001
- Test: `test_BC_4_12_002_wave_context_output_shape`
- Asserts: ResolverOutput key="wave_context", value shape {wave_id, cycle_id, stories}

### AC-002
- Tests: `test_BC_4_12_004_*` (4 tests — absent state, None cycle_id, malformed YAML, no-unwrap source check)
- Asserts: value: None on all absent/malformed inputs; never panics

### AC-003
- Tests: `test_BC_4_12_002_empty_stories_yields_none`, `test_BC_4_12_002_all_completed_waves_yields_none`
- Asserts: value: None when waves list is empty or all entries are completed

### AC-004
- Tests: `test_BC_4_12_002_missing_wave_id_no_panic`, `test_parse_wave_state_rejects_missing_required_wave_field`
- Asserts: optional field absence is non-panicking; required field absence returns parse Err

### AC-007
- Test: `test_BC_4_12_002_project_dir_is_top_level_field`
- Source: `crates/vsdd-context-resolvers/src/lib.rs` line 76 — `input.project_dir` direct access
- Asserts: field is read from ResolverInput struct, not from plugin_config map

### AC-008
- Test: `test_BC_4_12_002_prop_resolve_wave_context_is_deterministic` (`--release`)
- Asserts: 200 proptest cases pass; pure resolver is deterministic (same in = same out)
- Wall-clock: 0.02s in optimized build

### AC-009
- `ls -la plugins/vsdd-factory/hook-plugins/vsdd-context-resolvers.wasm` — 282716 bytes
- `cat plugins/vsdd-factory/resolvers-registry.toml` — canonical [[resolvers]] entry
- Test: `test_BC_4_12_001_wasm_artifact_registered_in_registry` passes

### AC-010
- `cargo clippy -p vsdd-context-resolvers --all-targets -- -D warnings -D clippy::unwrap_used -D clippy::expect_used -D clippy::panic` — clean exit (no violations)
- `grep -rn ".unwrap()\|.expect(\|panic!" crates/vsdd-context-resolvers/src/` — zero matches
- Test: `test_BC_4_12_004_no_unwrap_or_expect_in_lib` passes

---

## DEFERRED ACs — Rationale

### AC-005 and AC-006 (capability confinement, VP-076)

**Deferral authority:** VP-076 §Proof Harness Locations (pass-2 amendment, 2026-05-10)

**Architectural constraint (BC-1.13.001 INV1):** `factory-dispatcher` has ZERO
compile-time dependency on `vsdd-context-resolvers`. The WASM artifact is loaded
dynamically at runtime. Capability confinement tests require the dispatcher's
`ResolverLoader` test harness — which cannot be a compile-time dependency of this crate.

**Stubs in place:** `crates/vsdd-context-resolvers/tests/capability_confinement_test.rs`
contains three `#[ignore]` + `unimplemented!()` stubs that compile cleanly and appear
in `cargo test` output (3 ignored). They document exactly what S-12.08 must implement.

**S-12.08 action:** Author bats integration tests that:
1. Build WASM: `cargo build -p vsdd-context-resolvers --target wasm32-wasip1 --release`
2. Load via factory-dispatcher ResolverLoader with `path_allow = [".factory/"]`
3. Assert VP-076-A: CapabilityDenied for `/etc/passwd` read
4. Assert VP-076-B: no sensitive data in plugin_config
5. Assert VP-076-C: `resolver.capability_denied` audit event emitted
6. Assert VP-076-D: `.factory/` reads succeed (positive grant)

---

## Full Test Suite Result

```
cargo test -p vsdd-context-resolvers

running 0 tests  (lib unit tests)
running 3 tests  (capability_confinement_test.rs)  → 0 passed, 3 ignored
running 26 tests (wave_context_test.rs)            → 26 passed, 0 failed

test result: ok. 26 passed; 0 failed; 3 ignored
```
