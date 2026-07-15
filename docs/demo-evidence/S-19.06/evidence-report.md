---
story_id: S-19.06
title: "host::read_prefix bounded partial read: head-c semantics, NEVER OUTPUT_TOO_LARGE, additive FFI entry point"
version: "1.0"
recorded: 2026-07-15
branch: feature/S-19.06
head: 7156a4c3
product_type: Rust library + WASM hook-sdk FFI binding (no UI)
evidence_mode: captured-stdout test transcripts
---

# Demo Evidence — S-19.06

**Story:** S-19.06 — host::read_prefix bounded partial read: head-c semantics, NEVER OUTPUT\_TOO\_LARGE, additive FFI entry point
**Epic:** E-19 — Post-rc.22 Operator Hardening
**BC gate:** BC-1.17.001 v1.6 (PC-1..PC-6, Invariants 2/3/4/5, EC-001, §(a) layering parenthetical)
**LOCAL cascade:** CONVERGED 3/3

This story is a Rust library and WASM hook-sdk FFI binding — no UI or CLI entry point to drive
visually. Evidence is provided as captured-stdout transcripts from `cargo test` and `bats` runs,
constituting the TDD red-gate-to-green transition proof per the VSDD demo-recorder mode for
library/test-harness products.

---

## Coverage Matrix

| AC | Criterion (summary) | Test(s) | Transcript | Result |
|----|---------------------|---------|------------|--------|
| AC-001 | Bounded prefix: file > max_bytes → exactly max_bytes bytes, exit 0; byte-exact (no UTF-8 trimming) | T-001, T-002 + grep gate | transcript-AC001-bounded-prefix.txt | PASS |
| AC-002 | Short file: file < max_bytes → full content returned, no padding, exit 0 | T-003 | transcript-AC002-short-file.txt | PASS |
| AC-003 | NEVER OUTPUT\_TOO\_LARGE for any valid input (runtime + static gate) | T-004 + T-009g | transcript-AC003-never-output-too-large.txt | PASS |
| AC-004 | Capability independence: no read\_prefix block → CAPABILITY\_DENIED; read\_file-only → CAPABILITY\_DENIED | T-005, T-006, T-013a | transcript-AC004-capability-independence.txt | PASS |
| AC-005 | NOT\_FOUND (-5) + file\_not\_found event for absent allowlisted file; zero capability\_denied events | T-007 | transcript-AC005-not-found.txt | PASS |
| AC-006 | max\_bytes=0 → empty payload, exit 0; composite (absent file, max\_bytes=0) → no NOT\_FOUND, no file\_not\_found event | T-008, T-012 | transcript-AC006-max-bytes-zero.txt | PASS |
| AC-007 | Two-layer hook-sdk (safe wrapper + raw extern) + dispatcher registration + wasm32-wasip1 fixture compile/link | bats T-009a..T-009h | transcript-AC007-hook-sdk-wasm.txt | PASS |

---

## AC-001: Bounded Prefix (head-c Semantics)

**Transcript:** `transcript-AC001-bounded-prefix.txt`
**Story gate:** `grep -q "fn read_prefix" crates/factory-dispatcher/src/host/read_prefix.rs` exits 0.
**Tests:**
- `test_S19_06_T001_bounded_prefix_returns_exactly_max_bytes` — 100-byte file, max\_bytes=50
  → payload length = 50, exit code 0. Verifies head-c truncation at the byte boundary.
- `test_S19_06_T002_byte_exact_no_utf8_trimming_at_boundary` — file with a multi-byte UTF-8
  sequence straddling the max\_bytes boundary: payload is the raw first max\_bytes bytes with no
  UTF-8 boundary trimming (byte-exact per BC-1.17.001 PC-6).

```
test host::read_prefix::tests::test_S19_06_T002_byte_exact_no_utf8_trimming_at_boundary ... ok
test host::read_prefix::tests::test_S19_06_T001_bounded_prefix_returns_exactly_max_bytes ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 192 filtered out
```

The grep gate exits 0, confirming `fn read_prefix` is declared in
`crates/factory-dispatcher/src/host/read_prefix.rs`.

**BC Trace:** BC-1.17.001 v1.6 PC-1 (bounded prefix, head-c semantics) + PC-6 (byte-exact, no
UTF-8 trimming).

---

## AC-002: Short File Returns Full Content

**Transcript:** `transcript-AC002-short-file.txt`
**Test:** `test_S19_06_T003_short_file_returns_full_content_no_padding` — 30-byte file,
max\_bytes=100 → payload length = 30 (file\_size), exit code 0. Response is not padded to
max\_bytes; no truncation markers.

```
test host::read_prefix::tests::test_S19_06_T003_short_file_returns_full_content_no_padding ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 193 filtered out
```

**BC Trace:** BC-1.17.001 v1.6 PC-2 (full content when file\_size \< max\_bytes).

---

## AC-003: NEVER OUTPUT\_TOO\_LARGE

**Transcript:** `transcript-AC003-never-output-too-large.txt`
**Tests:**
- T-004 (runtime): `test_S19_06_T004_never_returns_output_too_large` — 10000-byte file,
  max\_bytes=50 → return code ≠ -3. Asserts that OUTPUT\_TOO\_LARGE is unreachable by construction
  (max\_bytes IS the cap; data beyond the cap is not read).
- T-009g (static gate, executed in bats): production-code-only awk strip + block/line comment
  strip confirms OUTPUT\_TOO\_LARGE absent from non-comment production code of `read_prefix.rs`.

```
test host::read_prefix::tests::test_S19_06_T004_never_returns_output_too_large ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 193 filtered out
```

```
ok 7 T-009g AC-003 static gate: OUTPUT_TOO_LARGE absent from non-comment production code in read_prefix.rs
```

T-004 is the runtime load-bearing check. T-009g is the static defense-in-depth gate that the
constant cannot appear in non-comment production code. Both pass.

**BC Trace:** BC-1.17.001 v1.6 PC-3 (NEVER OUTPUT\_TOO\_LARGE guarantee).

---

## AC-004: Capability Independence

**Transcript:** `transcript-AC004-capability-independence.txt`
**Tests:**
- T-005: `test_S19_06_T005_no_capability_block_returns_capability_denied` — plugin with no
  capability block → CAPABILITY\_DENIED (-1). Deny-by-default: the absence of
  `capabilities.read_prefix` is sufficient to deny all calls.
- T-006: `test_S19_06_T006_read_file_cap_only_returns_capability_denied` — plugin with
  `capabilities.read_file` only (no `read_prefix` block) → CAPABILITY\_DENIED (-1). The two
  capabilities are independent; `read_file` does not grant `read_prefix`.
- T-013a: `test_S19_06_T013a_no_capability_max_bytes_zero_returns_capability_denied` —
  no capability block + max\_bytes=0 → CAPABILITY\_DENIED (-1). The capability check (step 1)
  fires before the max\_bytes=0 short-circuit (step 3), locking step ordering.

```
test host::read_prefix::tests::test_S19_06_T006_read_file_cap_only_returns_capability_denied ... ok
test host::read_prefix::tests::test_S19_06_T005_no_capability_block_returns_capability_denied ... ok
test host::read_prefix::tests::test_S19_06_T013a_no_capability_max_bytes_zero_returns_capability_denied ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 191 filtered out
```

**BC Trace:** BC-1.17.001 v1.6 PC-4 (deny by capability absence) + Invariant 3 (capabilities
are independent; defense-in-depth).

---

## AC-005: NOT\_FOUND + file\_not\_found Event

**Transcript:** `transcript-AC005-not-found.txt`
**Test:** `test_S19_06_T007_absent_allowlisted_file_returns_not_found_and_emits_event` —
allowlisted path, file absent, `capabilities.read_prefix` present → return code -5; event stream
contains `type=internal.file_not_found`; zero `capability_denied` events captured. The zero
`capability_denied` events confirm the path successfully passed both the capability check (step 1)
and path-resolution check (step 2) before the existence check (step 4) fired NOT\_FOUND.

```
test host::read_prefix::tests::test_S19_06_T007_absent_allowlisted_file_returns_not_found_and_emits_event ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 193 filtered out
```

**BC Trace:** BC-1.17.001 v1.6 PC-5 (NOT\_FOUND on absent allowlisted file) + Invariant 5
(consistent with BC-2.07.001 part c absent-file semantics).

---

## AC-006: max\_bytes=0 Short-Circuit

**Transcript:** `transcript-AC006-max-bytes-zero.txt`
**Tests:**
- T-008: `test_S19_06_T008_max_bytes_zero_returns_empty_payload_exit_0` — file present,
  max\_bytes=0 → payload empty (0 bytes), exit code 0. The file is not opened.
- T-012: `test_S19_06_T012_absent_file_max_bytes_zero_short_circuits_before_existence_check` —
  absent allowlisted file, max\_bytes=0 → empty payload, exit 0, NO NOT\_FOUND (-5), NO
  `file_not_found` event. The max\_bytes=0 short-circuit at step 3 fires before the existence
  check at step 4; existence is never consulted.

```
test host::read_prefix::tests::test_S19_06_T008_max_bytes_zero_returns_empty_payload_exit_0 ... ok
test host::read_prefix::tests::test_S19_06_T012_absent_file_max_bytes_zero_short_circuits_before_existence_check ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 192 filtered out
```

T-012 is the architectural step-ordering lock for EC-001. The mutant regression tests
`T-012_MUTANT_VERIFY` and `T-013_MUTANT_VERIFY` (included in the full 14-test suite) verify that
reordering steps causes these tests to fail, proving the ordering is load-bearing and not incidental.

**BC Trace:** BC-1.17.001 v1.6 EC-001 (max\_bytes=0 → empty, exit 0; file not read; existence
not consulted).

---

## AC-007: Two-Layer hook-sdk + Dispatcher Registration + WASM Fixture

**Transcript:** `transcript-AC007-hook-sdk-wasm.txt`
**Tests:** bats read-prefix-wasm.bats T-009a..T-009h (8 tests, all PASS)

The suite exercises three gate classes:

**Gate 1 (T-009a):** Full safe wrapper signature in `crates/hook-sdk/src/host.rs`:
`pub fn read_prefix(path: &str, max_bytes: u32, timeout_ms: u32) -> Result<Vec<u8>, HostError>`.
Asserts the return type is `Result<Vec<u8>, HostError>` — not `-> i32` — enforcing BC-1.17.001
v1.6 §(a) layering parenthetical (safe wrapper is the hook-author interface; raw wire-ABI extern
holds the `-> i32`).

**Gate 2 (T-009b/T-009c/T-009d):** Raw wire-ABI extern in `crates/hook-sdk/src/ffi.rs`:
(i) `pub safe fn read_prefix(` with 6-param pointer/length shape (`path_len` + `out_ptr_out`
asserted); (ii) `#[link(wasm_import_module = "vsdd")]` attribute; (iii) `fn read_prefix` present
in both the `#[cfg(target_arch = "wasm32")]` extern block AND the `pub mod host_stubs` non-wasm
block.

**Gate 3 (T-009e):** `read_prefix::register` present in
`crates/factory-dispatcher/src/host/mod.rs` dispatch table (additive; no existing entries
removed or modified).

**Gate 4 (T-009f):** Fixture crate `crates/hook-plugins/read-prefix-fixture/` compiles and
links for `wasm32-wasip1` target (`cargo build -p read-prefix-fixture --target wasm32-wasip1`
exits 0). This is the load-bearing FFI boundary proof: the fixture imports `read_prefix` from
the `vsdd` namespace via the hook-sdk, proving the full two-layer path (safe wrapper →
raw extern → `#[link]` attribute → wasm32 import) is wired correctly.

**T-009h (POLICY 20):** The `read-prefix-fixture` crate is excluded from all `--workspace`
`wasm32-wasip1` builds and staging loops (via `--exclude` flags in CI and release YAMLs), ensuring
the fixture does not interfere with other plugin build steps.

```
1..8
ok 1 T-009a AC-007 Gate 1: safe wrapper pub fn read_prefix signature in hook-sdk/src/host.rs
ok 2 T-009b AC-007 Gate 2(i): raw extern pub safe fn read_prefix in hook-sdk/src/ffi.rs
ok 3 T-009c AC-007 Gate 2(ii): #[link(wasm_import_module = "vsdd")] attribute in ffi.rs
ok 4 T-009d AC-007 Gate 2(iii): read_prefix in wasm32 extern block AND host_stubs in ffi.rs
ok 5 T-009e AC-007 Gate 3: read_prefix::register in factory-dispatcher/src/host/mod.rs
ok 6 T-009f AC-007 Gate 4: fixture WASM read-prefix-fixture builds for wasm32-wasip1
ok 7 T-009g AC-003 static gate: OUTPUT_TOO_LARGE absent from non-comment production code in read_prefix.rs
ok 8 T-009h POLICY 20 exclusion presence-gate: read-prefix-fixture excluded in all wasm32-wasip1 --workspace builds and staging loops
```

**BC Trace:** BC-1.17.001 v1.6 §(a) layering parenthetical + §Architecture Anchors (hook-sdk
safe wrapper + raw extern + dispatcher registration) + Invariant 2 (read\_file unchanged).

---

## Full Unit Test Suite Summary

```
$ cargo test -p factory-dispatcher --lib -- host::read_prefix::tests

running 14 tests
test host::read_prefix::tests::test_S19_06_T005_no_capability_block_returns_capability_denied ... ok
test host::read_prefix::tests::test_S19_06_T006_read_file_cap_only_returns_capability_denied ... ok
test host::read_prefix::tests::test_S19_06_T013_MUTANT_VERIFY_hoisted_short_circuit_leaks_to_unauthorized_caller ... ok
test host::read_prefix::tests::test_S19_06_T013a_no_capability_max_bytes_zero_returns_capability_denied ... ok
test host::read_prefix::tests::test_S19_06_T003_short_file_returns_full_content_no_padding ... ok
test host::read_prefix::tests::test_S19_06_T008_max_bytes_zero_returns_empty_payload_exit_0 ... ok
test host::read_prefix::tests::test_S19_06_T012_MUTANT_VERIFY_short_circuit_reorder_causes_not_found ... ok
test host::read_prefix::tests::test_S19_06_T004_never_returns_output_too_large ... ok
test host::read_prefix::tests::test_S19_06_T013b_path_outside_allowlist_max_bytes_zero_returns_capability_denied ... ok
test host::read_prefix::tests::test_S19_06_T002_byte_exact_no_utf8_trimming_at_boundary ... ok
test host::read_prefix::tests::test_S19_06_T012_absent_file_max_bytes_zero_short_circuits_before_existence_check ... ok
test host::read_prefix::tests::test_S19_06_T001_bounded_prefix_returns_exactly_max_bytes ... ok
test host::read_prefix::tests::test_S19_06_T007_absent_allowlisted_file_returns_not_found_and_emits_event ... ok
test host::read_prefix::tests::test_S19_06_T010_path_outside_allowlist_returns_capability_denied_with_event ... ok

test result: ok. 14 passed; 0 failed; 0 ignored; 0 measured; 180 filtered out; finished in 0.01s
```

**14 unit tests GREEN. 8 bats gates GREEN. 0 failures.**

Note: The full 14-test unit suite includes 5 cascade-remediation regression locks
(T-012_MUTANT_VERIFY, T-013a, T-013b, T-013_MUTANT_VERIFY, T-013b) that were written green
to lock the step-ordering invariant and deny-by-default boundary against future mutation.
These pass without requiring a Red Gate transition (they were authored as lock tests, not
failing-first tests).

---

## Notes on Evidence Mode

S-19.06 delivers `host::read_prefix` as a new dispatcher host function (Rust) plus hook-sdk
FFI bindings (Rust). There is no UI or interactive CLI entry point. Evidence is captured-stdout
transcripts per the library/test-harness demo mode described in the VSDD pipeline. All transcripts
are reproducible by running `cargo test -p factory-dispatcher` and
`cd plugins/vsdd-factory/tests && bats read-prefix-wasm.bats` on branch `feature/S-19.06`
(HEAD `7156a4c3`).

---

## Behavioral Discrepancies Found

None. All 14 unit tests and 8 bats gates pass against the implementation on
`feature/S-19.06`. No behavioral discrepancy between the implementation and the ACs was
observed. LOCAL cascade CONVERGED 3/3 prior to demo recording.

---

## Files

| File | Content |
|------|---------|
| `transcript-AC001-bounded-prefix.txt` | T-001/T-002 (bounded prefix + byte-exact) + grep gate (AC-001) |
| `transcript-AC002-short-file.txt` | T-003 (short file full content, no padding) (AC-002) |
| `transcript-AC003-never-output-too-large.txt` | T-004 runtime + T-009g static gate (AC-003) |
| `transcript-AC004-capability-independence.txt` | T-005/T-006/T-013a (deny-by-default + read\_file independence + step-order lock) (AC-004) |
| `transcript-AC005-not-found.txt` | T-007 (NOT\_FOUND + file\_not\_found event + zero capability\_denied) (AC-005) |
| `transcript-AC006-max-bytes-zero.txt` | T-008 + T-012 (max\_bytes=0 short-circuit + EC-001 composite) (AC-006) |
| `transcript-AC007-hook-sdk-wasm.txt` | bats T-009a..T-009h (two-layer hook-sdk + dispatcher + wasm32-wasip1 fixture) (AC-007) |
| `evidence-report.md` | This file — coverage matrix + per-AC narrative |
