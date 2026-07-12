---
story_id: S-19.03
title: "warn-pending-wave-gate FINDING-2: read_file file_not_found semantics + graceful absent-file handling"
version: "1.20"
recorded: 2026-07-11
branch: feature/S-19.03
head: 98df0edb
product_type: Rust library + WASM hook plugin (no UI)
evidence_mode: captured-stdout test transcripts + bats integration
---

# Demo Evidence — S-19.03

**Story:** S-19.03 — warn-pending-wave-gate FINDING-2: read_file file_not_found semantics + graceful absent-file handling
**Epic:** E-19 — Post-rc.22 Operator Hardening
**BC gate:** BC-2.07.001 v1.5 (host::read_file absent-file semantics); BC-2.02.011 v1.7 (host::write_file path traversal prevention)
**Closes:** rc.22 smoke FINDING-2 (warn-pending-wave-gate emits false-positive capability_denied path_not_allowed on every Stop event when .factory/wave-state.yaml is absent; dispatcher trace bc687a0f)

This story is a Rust library and WASM hook plugin, not a UI application. Evidence is
provided as captured-stdout transcripts from `cargo test` runs and a bats integration test
run through the real dispatcher binary, which constitute the TDD red-gate-to-green transition
proof per the VSDD demo-recorder mode for library/test-harness products.

---

## Coverage Matrix

| AC | Criterion (summary) | Test(s) | Transcript | Result |
|----|---------------------|---------|------------|--------|
| AC-001 | path_allowed() rejoin algorithm: absent-file-under-existing-parent resolves; escape via `..` → DeniedNotAllowed; NC-B mock-canonicalize all-fail → path_resolution_failed (not path_not_allowed) | T-001, T-002, NC-A escape tests, EC-007, path_resolution_failed tests | transcript-AC001-rejoin-algorithm.txt | PASS |
| AC-002 | Absent allowlisted file → NOT_FOUND (-5) + internal.file_not_found, ZERO capability_denied events | T-003, T-004 | transcript-AC002-AC006-not-found-semantics.txt | PASS |
| AC-003 | codes::NOT_FOUND == -5 in pub mod codes block; HostError::NotFound in hook-sdk; from_code(-5) round-trip | AC-003 grep gates + T-005 unit test + hook-sdk doctest | transcript-AC003-not-found-constant.txt | PASS |
| AC-004 | Plugin: NOT_FOUND → Continue, zero WARN; CAPABILITY_DENIED → WARN (T-006 + T-006b) | T-006, T-006b (integration tests in warn-pending-wave-gate) | transcript-AC004-AC005-plugin-handling.txt | PASS |
| AC-005 | Positive-control: wave-state.yaml with pending_gate:true → WAVE GATE REMINDER stderr + exit 0 | T-007 (bats) | transcript-AC004-AC005-plugin-handling.txt | PASS |
| AC-006 | Zero internal.capability_denied reason=path_not_allowed for plugin_name=warn-pending-wave-gate when wave-state.yaml absent | T-008 (bats) via real dispatcher | transcript-AC002-AC006-not-found-semantics.txt | PASS |

---

## AC-001: path_allowed() Rejoin Algorithm

**Transcript:** `transcript-AC001-rejoin-algorithm.txt`
**Story gate (positive):** absent allowlisted file under existing parent → `resolve_path_for_allowlist` returns `Some(canonical_path)` → `path_allowed()` returns true.
**Story gate (negative control A):** path `.factory/../secrets/key` → synthesized canonical path resolves outside allowed prefix → `resolve_path_for_allowlist` returns `None` → `DeniedNotAllowed`.
**Story gate (negative control B per EC-007):** mock canonicalize fn returns `Err(NotFound)` for every ancestor → `resolve_path_for_allowlist` exhausts all ancestors → `None` → emits `internal.capability_denied reason=path_resolution_failed` (NOT `path_not_allowed`).

```
test host::path_util::tests::test_S19_03_T001_helper_absent_target_under_existing_parent_returns_some ... ok
test host::read_file::tests::test_S19_03_T001_absent_allowlisted_file_returns_NOT_FOUND ... ok
test host::read_file::tests::test_S19_03_T001_NC_B_path_resolution_failed_token_via_path_util ... ok
test host::path_util::tests::test_S19_03_P1_001_escape_rejection_absent_path_resolves_outside_prefix ... ok
test host::read_file::tests::test_S19_03_P1_001_escape_check_path_allowed_returns_denied_not_allowed ... ok
test host::read_file::tests::test_S19_03_P1_001_escape_prepare_returns_CAPABILITY_DENIED_reason_path_not_allowed ... ok
test host::path_util::tests::test_S19_03_EC007_mock_canonicalize_all_fail_returns_none ... ok
test host::read_file::tests::test_S19_03_T002_NC_A_path_util_callable_from_read_file_context ... ok
test result: ok. (all pass)
```

The mandatory write_file.rs sibling-sweep (Architecture Mapping TD-VSDD-060) also passes:

```
test host::write_file::tests::test_S19_03_P1_002_NC_B_denied_resolution_failed_emits_path_resolution_failed_reason_write ... ok
test host::write_file::tests::test_S19_03_write_sibling_sweep_path_resolution_failed_vs_path_not_allowed ... ok
test result: ok.
```

**BC Trace:** BC-2.07.001 v1.5 part b (rejoin algorithm); BC-2.02.011 v1.7 EC-001 (traversal → CAPABILITY_DENIED via shared `resolve_path_for_allowlist` in `path_util.rs`); VP-097 (Kani traversal-defense).

---

## AC-002: Absent Allowlisted File → NOT_FOUND + internal.file_not_found Event

**Transcript:** `transcript-AC002-AC006-not-found-semantics.txt`
**Tests:**
- `test_S19_03_T003_absent_allowlisted_file_emits_file_not_found_event` (T-003) — path allowed,
  file absent → emitted event has `type=internal.file_not_found`, `reason=file_not_found`.
  Captured event stream contains zero `capability_denied` events.
- `test_S19_03_T004_absent_allowlisted_file_zero_capability_denied_events` (T-004) — direct
  assertion that no `internal.capability_denied` events are present in the captured stream.

```
test host::read_file::tests::test_S19_03_T003_absent_allowlisted_file_emits_file_not_found_event ... ok
test host::read_file::tests::test_S19_03_T004_absent_allowlisted_file_zero_capability_denied_events ... ok
test result: ok. 2 passed; 0 failed
```

These two unit tests close rc.22 FINDING-2 at the dispatcher unit-test level. T-008 (bats)
closes it end-to-end through the real dispatcher binary. See AC-006 below.

**BC Trace:** BC-2.07.001 v1.5 part c (NOT_FOUND for absent allowlisted file); VP-098 (functional: NOT_FOUND returned, zero false-positive capability_denied).

---

## AC-003: codes::NOT_FOUND == -5 Constant + HostError::NotFound SDK Variant

**Transcript:** `transcript-AC003-not-found-constant.txt`
**Gates (from story):**

```
$ grep -q "pub const NOT_FOUND: i32 = -5;" crates/factory-dispatcher/src/host/mod.rs
exit 0: AC-003 gate 1 PASS

$ grep -q "NotFound" crates/hook-sdk/src/host.rs
exit 0: AC-003 gate 2 PASS
```

Constant is at `crates/factory-dispatcher/src/host/mod.rs` line 217 in the `pub mod codes` block.
`HostError::NotFound` is at `crates/hook-sdk/src/host.rs` line 105.
`from_code(-5)` mapping at line 118.

**Architecture compliance — no #[non_exhaustive] on HostError:**
```
$ grep -nE '^\s*#\[non_exhaustive\]' crates/hook-sdk/src/host.rs
(no output — exit 1 = attribute absent = PASS per O-P2-002)
```

**T-005 unit test:**
```
test host::s19_03_codes_tests::test_S19_03_T005_NOT_FOUND_constant_equals_minus_5 ... ok
test result: ok. 1 passed; 0 failed
```

**BC Trace:** BC-2.07.001 v1.5 (NOT_FOUND = -5); ADR-025 Decision 13 (allocation policy); VP-098.

---

## AC-004: Plugin Handles NOT_FOUND Gracefully; CAPABILITY_DENIED Emits WARN

**Transcript:** `transcript-AC004-AC005-plugin-handling.txt`
**Tests:**
- `test_S19_03_T006_NOT_FOUND_returns_continue_with_zero_warn_output` (T-006) — mock
  `host::read_file` returns `NOT_FOUND (-5)` → plugin returns `Continue`, zero WARN entries.
- `test_S19_03_T006b_capability_denied_emits_warn_not_silent` (T-006b) — mock returns
  `CAPABILITY_DENIED (-1)` → plugin emits WARN (genuine allowlist violation path).

```
test warn_pending_wave_gate_integration::test_S19_03_T006_NOT_FOUND_returns_continue_with_zero_warn_output ... ok
test warn_pending_wave_gate_integration::test_S19_03_T006b_capability_denied_emits_warn_not_silent ... ok
test result: ok. 2 passed; 0 failed
```

T-006b provides the positive control required by the story: `CAPABILITY_DENIED` must NOT be
treated the same as `NOT_FOUND` — genuine allowlist violations must surface as WARN so
operators can act on real capability misconfigurations.

**BC Trace:** BC-2.07.001 v1.5 part c (plugin NOT_FOUND handling).

---

## AC-005: Positive Control — Pending Gate Emits WAVE GATE REMINDER

**Transcript:** `transcript-AC004-AC005-plugin-handling.txt`
**Test:** T-007 (bats) — real dispatcher binary, real WASM plugin, fixture `wave-state.yaml`
containing `pending_gate: true`.

```
$ bats plugins/vsdd-factory/tests/warn-pending-wave-gate.bats

1..2
ok 1 T-007: warn-pending-wave-gate: wave-state.yaml with gate_status:pending emits WAVE GATE REMINDER to stderr and exits 0
ok 2 T-008: warn-pending-wave-gate: absent wave-state.yaml emits zero internal.capability_denied reason=path_not_allowed events
```

T-007 asserts: dispatcher exits 0 (advisory; `on_error=continue`); combined output contains
"WAVE GATE REMINDER". This positive path was preserved by the S-19.03 implementation.

**BC Trace:** BC-2.07.001 v1.5 (positive-path operation).

---

## AC-006: Zero capability_denied path_not_allowed in Fresh Install

**Transcript:** `transcript-AC002-AC006-not-found-semantics.txt`
**Test:** T-008 (bats) — real dispatcher binary, real WASM plugin, no `wave-state.yaml`.

```
ok 2 T-008: warn-pending-wave-gate: absent wave-state.yaml emits zero internal.capability_denied reason=path_not_allowed events
```

The bats test checks the internal JSONL log for `capability_denied reason=path_not_allowed`
events filtered to `plugin_name=warn-pending-wave-gate`. With the S-19.03 fix in place,
`cap_denied_count=0` and the test passes.

The gate uses the F-P15-007 form (jq-e + wc-l) to avoid the `grep -c` exit-1-on-zero-matches
defect where the happy path (zero matches) would incorrectly fail the gate.

**BC Trace:** BC-2.07.001 v1.5 part c (zero false-positive capability_denied reason=path_not_allowed); VP-098.

---

## Full Test Run Summary

```
$ cargo test -p factory-dispatcher -p vsdd-hook-sdk -p warn-pending-wave-gate

factory-dispatcher unit tests (inline #[cfg(test)]):
  test result: ok. 175 passed; 0 failed; 0 ignored; 0 measured; finished in 0.91s

vsdd-hook-sdk doc-tests:
  test result: ok. 4 passed; 0 failed; 1 ignored; 0 measured; finished in 0.00s

warn-pending-wave-gate integration (tests/integration_test.rs):
  test result: ok. 25 passed; 0 failed; 0 ignored; 0 measured; finished in 0.01s

$ bats plugins/vsdd-factory/tests/warn-pending-wave-gate.bats
1..2
ok 1 T-007 (AC-005 positive control)
ok 2 T-008 (AC-006 absent file zero capability_denied)
```

**Total: 175 + 4 + 25 unit/integration tests GREEN; 2 bats tests GREEN. 0 failures.**

---

## Notes on Evidence Mode

This story delivers changes to Rust libraries (`factory-dispatcher`, `vsdd-hook-sdk`) and a
WASM hook plugin (`warn-pending-wave-gate`). There is no UI or CLI entry point to drive visually.
Evidence is captured-stdout transcripts per the library/test-harness demo mode described in the
VSDD pipeline, plus bats integration tests that exercise the real dispatcher + WASM runtime.

The transcripts are reproducible: on branch `feature/S-19.03` (HEAD `98df0edb`), running:
- `cargo test -p factory-dispatcher -p vsdd-hook-sdk -p warn-pending-wave-gate` for unit tests
- `bats plugins/vsdd-factory/tests/warn-pending-wave-gate.bats` for T-007/T-008

reproduces all results.

---

## Behavioral Discrepancies Found

None. All tests pass against the implementation on `feature/S-19.03`. No behavioral
discrepancy between the implementation and the ACs was observed.

---

## Files

| File | Content |
|------|---------|
| `transcript-AC001-rejoin-algorithm.txt` | AC-001 rejoin algorithm: T-001 (positive + NC-B), T-002 (NC-A), escape rejection tests, EC-007 mock-all-fail, sibling-sweep write_file.rs |
| `transcript-AC002-AC006-not-found-semantics.txt` | AC-002 (T-003/T-004: NOT_FOUND + file_not_found event) + AC-006 (T-008 bats: zero path_not_allowed events) |
| `transcript-AC003-not-found-constant.txt` | AC-003 grep gates + T-005 unit test + hook-sdk doctest + architecture-compliance (#[non_exhaustive] absent) |
| `transcript-AC004-AC005-plugin-handling.txt` | AC-004 (T-006 NOT_FOUND→Continue, T-006b CAPABILITY_DENIED→WARN) + AC-005 (T-007 bats positive control) |
| `evidence-report.md` | This file — coverage matrix + per-AC narrative |
