---
story_id: S-19.02
title: "verify-factory-lock FINDING-1: frontmatter-only STATE.md read + raised byte budget"
version: "1.18"
recorded: 2026-07-11
branch: feature/S-19.02
head: 492f01c2
product_type: Rust library + WASM hook plugin (no UI)
evidence_mode: captured-stdout test transcripts
---

# Demo Evidence — S-19.02

**Story:** S-19.02 — verify-factory-lock FINDING-1: frontmatter-only STATE.md read + raised byte budget
**Epic:** E-19 — Post-rc.22 Operator Hardening
**BC gate:** BC-4.13.001 v1.15 Phase-A (Precondition 3, Invariant 9, Invariant 10)
**Closes:** rc.22 smoke FINDING-1 (verify-factory-lock silently degraded on ~90 KB STATE.md)

This story is a Rust library and WASM hook plugin, not a UI application. Evidence is
provided as captured-stdout transcripts from `cargo test` runs, which constitute the
TDD red-gate-to-green transition proof per the VSDD demo-recorder mode for library/test-harness
products.

---

## Coverage Matrix

| AC | Criterion (summary) | Test(s) | Transcript | Result |
|----|---------------------|---------|------------|--------|
| AC-001 | STATE_MD_MAX_BYTES == 262144 | grep gate + T-001 | transcript-AC001-constant.txt | PASS |
| AC-002 | 64–256 KiB STATE.md: foreign lock -> Block; no lock -> Continue | T-002, T-003 | transcript-AC002-AC004-cap-enforcement.txt | PASS |
| AC-003 | extract_frontmatter truncates at closing --- delimiter; fallback to full content | T-004, T-005 | transcript-AC003-AC005-extract-frontmatter.txt | PASS |
| AC-004 | No output_too_large events for STATE.md <= 256 KiB | T-006 (integration) | transcript-AC002-AC004-cap-enforcement.txt | PASS |
| AC-005 | extract_frontmatter byte-exact boundary (LF + CRLF) | T-007, T-008, T-010 (proptest) | transcript-AC003-AC005-extract-frontmatter.txt + transcript-AC005-proptest.txt | PASS |
| AC-006 | Soft-warning fires in (200000, 262144]; boundaries exact | T-009 (A-E) | transcript-AC006-soft-warning.txt | PASS |

---

## AC-001: STATE_MD_MAX_BYTES Constant Raised to 262144

**Transcript:** `transcript-AC001-constant.txt`
**Story gate:** `grep -q "STATE_MD_MAX_BYTES.*262144" crates/hook-plugins/verify-factory-lock/src/lib.rs` exits 0.
**Test:** `test_S1902_T001_state_md_max_bytes_is_262144` — asserts `STATE_MD_MAX_BYTES == 262144u32` at runtime.

```
test tests::test_S1902_T001_state_md_max_bytes_is_262144 ... ok
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 27 filtered out
```

**Root cause closed:** `read_bounded()` in `host::read_file` checks `metadata.len()` before reading any
bytes. At the old cap (65536), the ~90 KB production STATE.md triggered `OUTPUT_TOO_LARGE` before the
frontmatter parser was invoked, causing the lock gate to fail-open silently on every PreToolUse dispatch.
Raising to 262144 gives ≥25% headroom over the worst-case observed STATE.md size per ADR-026.

**BC Trace:** BC-4.13.001 v1.15 Phase-A Precondition 3 / ADR-025 Decision 14.

---

## AC-002: Guard Correctly Handles STATE.md Files Between 64 KiB and 256 KiB

**Transcript:** `transcript-AC002-AC004-cap-enforcement.txt`
**Tests:**
- `test_S1902_T002_70kib_fixture_foreign_lock_returns_block` — 70000-byte fixture with foreign
  unexpired lock in frontmatter → `HookResult::Block`.
- `test_S1902_T003_70kib_fixture_no_lock_returns_continue` — 70000-byte fixture with no lock
  → `HookResult::Continue`.

```
test tests::test_S1902_T002_70kib_fixture_foreign_lock_returns_block ... ok
test tests::test_S1902_T003_70kib_fixture_no_lock_returns_continue ... ok
test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured
```

Both tests embed a Red Gate pre-condition: `assert!(STATE_MD_MAX_BYTES >= 70_000)`. This assertion
would fail at the old cap (65536), ensuring the tests themselves are structural proof of the
cap-raise requirement.

**BC Trace:** BC-4.13.001 v1.15 Phase-A Precondition 3 (operational at new cap).

---

## AC-003: extract_frontmatter Truncates at Closing Delimiter

**Transcript:** `transcript-AC003-AC005-extract-frontmatter.txt`
**Tests:**
- `test_S1902_extract_frontmatter_delimiter_present_excludes_body` (T-004) — fixture with
  `\n---\n` delimiter: body bytes absent from extracted slice; slice starts with `---\n`.
- `test_S1902_extract_frontmatter_no_delimiter_returns_full_slice` (T-005) — fixture with no
  closing `---`: extracted == full input (EC-004 fallback).

```
test tests::test_S1902_extract_frontmatter_delimiter_present_excludes_body ... ok
test tests::test_S1902_extract_frontmatter_no_delimiter_returns_full_slice ... ok
test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured
```

**CRLF coverage (EC-017):**
- `test_S1902_crlf_extract_frontmatter_excludes_body` — CRLF fixture `\r\n---\r\n`: body excluded.
- `test_S1902_crlf_extract_frontmatter_byte_exact_boundary` — CRLF byte-exact prefix.

```
test tests::test_S1902_crlf_extract_frontmatter_excludes_body ... ok
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured
```

**BC Trace:** BC-4.13.001 v1.15 Phase-A Invariant 9 (frontmatter-only-parsing mandate; CRLF
delimiter forms added in v1.15 / EC-017). Guard MUST NOT parse file body content.

---

## AC-004: No output_too_large Events for STATE.md <= 256 KiB

**Transcript:** `transcript-AC002-AC004-cap-enforcement.txt`
**Test:** `t006_vp095_real_cap_enforcement_sizes` (integration test,
`crates/hook-plugins/verify-factory-lock/tests/integration_ac004_no_output_too_large.rs`).

```
test t006_vp095_real_cap_enforcement_sizes ... ok
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured
```

Cap-enforcement mock returns `Err("OutputTooLarge...")` if `fixture_size > STATE_MD_MAX_BYTES`,
exactly mirroring real `host::read_file` behavior. The assertion matrix:
- 65535, 65536, 131072, 262144 bytes → no output_too_large warn + Block (guard ran, foreign lock detected).
- 262145 bytes → fail-open Continue + StateReadError warn (EC-002 path verified).

This test was RED at the old cap: any fixture > 65536 bytes would have triggered the mock's Err
path, and the Block assertion for those sizes would have failed.

**BC Trace:** BC-4.13.001 v1.15 Phase-A Precondition 3 / VP-095.

---

## AC-005: extract_frontmatter Byte-Exact Boundary Behavior

**Transcripts:** `transcript-AC003-AC005-extract-frontmatter.txt`, `transcript-AC005-proptest.txt`

**Unit test A — boundary** (T-007): `test_S1902_extract_frontmatter_byte_exact_boundary_body_absent`
```
Fixture: b"---\nfactory_lock: null\n---\nbody content here"
delimiter_start_offset = 22 (byte index of '\n' starting '\n---\n')
Assertions: extracted.len() == 22; extracted == input[0..22]; no "body" bytes in extracted
test result: ok. 1 passed; 0 failed
```

**Unit test B — EOF delimiter** (T-008): `test_S1902_extract_frontmatter_eof_delimiter_no_trailing_body`
```
Fixture: b"---\nfactory_lock: null\n---" (no trailing newline after closing ---)
Assertions: extracted does not end with "\n---"; starts with "---\n"
test result: ok. 1 passed; 0 failed
```

**CRLF byte-exact** (T-011-B): `test_S1902_crlf_extract_frontmatter_byte_exact_boundary`
```
Fixture: b"---\r\nfactory_lock: null\r\n---\r\nbody"
delimiter_start_offset = 23 (---=3 + \r\n=2 + "factory_lock: null"=18)
Assertions: extracted.len() == 23; extracted == input[0..23]
test result: ok. 1 passed; 0 failed
```

**VP-096 proptest** (T-010): three property-based tests over arbitrary byte inputs.
```
test prop_extract_frontmatter_byte_equals_prefix ... ok   (structural oracle: 3 invariants)
test prop_extract_frontmatter_crlf_byte_equals_prefix ... ok   (CRLF known-answer)
test prop_extract_frontmatter_is_deterministic ... ok   (determinism)
test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; finished in 0.05s
```

**BC Trace:** BC-4.13.001 v1.15 Phase-A Invariant 9 (byte-exact-prefix invariant; F-P2-011:
parity-with-full-file-parse is NOT the correctness criterion). VP-096 v1.1.

---

## AC-006: Soft-Warning at state_md_approaching_cap

**Transcript:** `transcript-AC006-soft-warning.txt`
**Test:** `test_S1902_T009_state_md_approaching_cap_warn_logic` — five sub-tests A–E.

```
test tests::test_S1902_T009_state_md_approaching_cap_warn_logic ... ok
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 27 filtered out
```

Boundary matrix verified:

| Fixture bytes | Expected warn | Verified |
|--------------|---------------|---------|
| 150000 | no warn | Sub-test B PASS |
| 200000 (exact threshold) | no warn (strict >) | Sub-test C PASS |
| 210000 | warn emitted | Sub-test A PASS |
| 262144 (cap-exact) | warn AND read succeeds | Sub-test D PASS |
| 262145 (over cap) | StateReadError; no warn | Sub-test E PASS |

**BC Trace:** BC-4.13.001 v1.15 Invariant 10: soft_warn_threshold=200000 (strictly >),
upper bound inclusive at cap 262144. VP-096 (proptest covers determinism).

---

## Full Test Run Summary

```
$ cargo test -p factory-lock-parse -p verify-factory-lock

factory_lock_parse unit tests (inline #[cfg(test)]):
  test result: ok. 15 passed; 0 failed; 0 ignored; 0 measured

factory_lock_parse proptest (tests/proptest_extract_frontmatter.rs):
  test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; finished in 0.05s

verify_factory_lock unit tests (inline #[cfg(test)]):
  test result: ok. 28 passed; 0 failed; 0 ignored; 0 measured

verify_factory_lock integration (tests/integration_ac004_no_output_too_large.rs):
  test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured
```

**Total: 47 tests GREEN. 0 failures.**

---

## Notes on Evidence Mode

This story delivers a pure Rust library (`factory-lock-parse`) and a WASM hook plugin
(`verify-factory-lock`). There is no UI or CLI entry point to drive visually. Evidence is
captured-stdout transcripts per the library/test-harness demo mode described in the VSDD
pipeline. The transcripts are reproducible: `cargo test -p factory-lock-parse -p verify-factory-lock`
on branch `feature/S-19.02` (HEAD `492f01c2`) reproduces all results.

---

## Behavioral Discrepancies Found

None. All 47 tests pass against the implementation on `feature/S-19.02`. No behavioral
discrepancy between the implementation and the ACs was observed.

---

## Files

| File | Content |
|------|---------|
| `transcript-AC001-constant.txt` | AC-001 grep gate + T-001 test (STATE_MD_MAX_BYTES == 262144) |
| `transcript-AC002-AC004-cap-enforcement.txt` | T-002/T-003 (AC-002) + T-006 integration (AC-004) |
| `transcript-AC003-AC005-extract-frontmatter.txt` | T-004/T-005/T-007/T-008 + CRLF tests (AC-003, AC-005) |
| `transcript-AC005-proptest.txt` | VP-096 proptest T-010: 3 property tests (AC-005) |
| `transcript-AC006-soft-warning.txt` | T-009 sub-tests A-E (AC-006 boundary matrix) |
| `evidence-report.md` | This file — coverage matrix + per-AC narrative |
