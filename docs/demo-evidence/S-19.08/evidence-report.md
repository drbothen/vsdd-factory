---
story_id: S-19.08
title: "verify-state-timestamp-refresh: raise 64 KiB byte cap to 256 KiB + wire extract_frontmatter"
version: "1.0"
recorded: 2026-07-14
branch: feature/S-19.08
head: fea77eea
product_type: Rust library + WASM hook plugin (no UI)
evidence_mode: captured-stdout test transcripts
---

# Demo Evidence — S-19.08

**Story:** S-19.08 — verify-state-timestamp-refresh: raise 64 KiB byte cap to 256 KiB + wire extract_frontmatter
**Epic:** E-19 — Post-rc.22 Operator Hardening
**BC gate:** BC-5.40.001 v1.2 Precondition 6 (cap = 262144), Invariant 7 (extract_frontmatter), Invariant 8 (state_md_approaching_cap)
**Closes:** rc.22 smoke follow-on: verify-state-timestamp-refresh silently inert when STATE.md exceeds 64 KiB (3x confirmed in production dispatcher logs; D-826/D-835; same defect class as S-19.02 FINDING-1)

This story is a Rust library and WASM hook plugin, not a UI application. Evidence is
provided as captured-stdout transcripts from `cargo test` runs, which constitute the
TDD red-gate-to-green transition proof per the VSDD demo-recorder mode for library/test-harness
products.

---

## Coverage Matrix

| AC | Criterion (summary) | Test(s) | Transcript | Result |
|----|---------------------|---------|------------|--------|
| AC-001 | STATE_MD_MAX_BYTES == 262144 | grep gate + T-001 | transcript-AC001-constant.txt | PASS |
| AC-002 | 64–256 KiB STATE.md: stale timestamp -> Block; advanced timestamp -> Continue | T-002, T-003 | transcript-AC002-AC004-cap-enforcement.txt | PASS |
| AC-003 | extract_frontmatter wired: body bytes excluded; delimiter-absent -> fail-open | T-004, T-005 | transcript-AC003-extract-frontmatter.txt | PASS |
| AC-004 | No output_too_large events for STATE.md <= 256 KiB (operational proxy: Block at 70 KiB) | T-006 (integration) | transcript-AC002-AC004-cap-enforcement.txt | PASS |
| AC-005 | state_md_approaching_cap soft-warn at bytes_read in (200000, 262144]; boundaries exact | T-007 (A-E) | transcript-AC005-soft-warning.txt | PASS |

---

## AC-001: STATE_MD_MAX_BYTES Constant Raised to 262144

**Transcript:** `transcript-AC001-constant.txt`
**Story gate:** `grep -q "STATE_MD_MAX_BYTES.*262144" crates/hook-plugins/verify-state-timestamp-refresh/src/lib.rs` exits 0.
**Test:** `test_BC_5_40_001_T001_state_md_max_bytes_is_262144` — asserts `STATE_MD_MAX_BYTES == 262144u32` at runtime.

```
test tests::test_BC_5_40_001_T001_state_md_max_bytes_is_262144 ... ok
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 49 filtered out
```

**Root cause closed:** `read_bounded()` in `host::read_file` checks `metadata.len()` before reading any
bytes. At the old cap (65536), the production STATE.md (>64 KiB, confirmed 3x in dispatcher logs per
D-826/D-835) triggered `OUTPUT_TOO_LARGE` before the frontmatter parser was invoked, causing the
timestamp-freshness gate to fail-open silently on every PreToolUse Edit/Write/MultiEdit dispatch to
STATE.md. Raising to 262144 gives >=25% headroom over the worst-case observed STATE.md size per
ADR-026 compaction discipline.

**BC Trace:** BC-5.40.001 v1.2 Precondition 6 / ADR-025 Decision 12 §12.5 parity with verify-factory-lock.

---

## AC-002: Guard Correctly Handles STATE.md Files Between 64 KiB and 256 KiB

**Transcript:** `transcript-AC002-AC004-cap-enforcement.txt`
**Tests:**
- `test_BC_5_40_001_T002_70kib_fixture_stale_timestamp_blocks` — 70000-byte fixture with stale
  timestamp in frontmatter → `HookResult::Block(TimestampStale)`.
- `test_BC_5_40_001_T003_70kib_fixture_advanced_timestamp_continues` — 70000-byte fixture with
  advanced timestamp → `HookResult::Continue`.

```
test tests::test_BC_5_40_001_T002_70kib_fixture_stale_timestamp_blocks ... ok
test tests::test_BC_5_40_001_T003_70kib_fixture_advanced_timestamp_continues ... ok
test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured
```

Both tests embed a Red Gate pre-condition: `assert!(STATE_MD_MAX_BYTES >= 70_000)`. This assertion
would fail at the old cap (65536), ensuring the tests are structural proof of the cap-raise requirement.

**BC Trace:** BC-5.40.001 v1.2 Precondition 6 (guard operational at new cap; PC4 mid-burst TTL enforcement).

---

## AC-003: extract_frontmatter Wired Before Any YAML Parsing

**Transcript:** `transcript-AC003-extract-frontmatter.txt`
**Tests:**
- `test_BC_5_40_001_T004_extract_frontmatter_wired_body_bytes_excluded` (T-004) — fixture with
  non-UTF-8 body bytes (`\xFF\xFE`) after the closing `---` delimiter: guard processes the
  frontmatter-only slice (via `factory_lock_parse::extract_frontmatter`) and does NOT panic or
  fail on the body bytes; returns the expected Block result on stale timestamp.
- `test_BC_5_40_001_T005_no_delimiter_full_content_fail_open` (T-005) — fixture with no closing
  frontmatter delimiter: `extract_frontmatter` returns full content (EC-004 fail-open path);
  guard continues to Continue per the malformed-frontmatter fail-open rule.

```
test tests::test_BC_5_40_001_T004_extract_frontmatter_wired_body_bytes_excluded ... ok
test tests::test_BC_5_40_001_T005_no_delimiter_full_content_fail_open ... ok
test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured
```

T-004's Red Gate: without `extract_frontmatter` wiring, `String::from_utf8(full_bytes)` would
return `Err` on the `\xFF\xFE` body bytes, routing through the fail-open UTF-8 path and returning
`Continue` instead of `Block`. The test asserts `Block` — proving that `extract_frontmatter`
strips the non-UTF-8 body before UTF-8 conversion is attempted.

**BC Trace:** BC-5.40.001 v1.2 Precondition 6 + Invariant 7 (extract_frontmatter exclusive use before any YAML parsing).

---

## AC-004: No output_too_large Events for STATE.md <= 256 KiB

**Transcript:** `transcript-AC002-AC004-cap-enforcement.txt`
**Test:** `t006_zero_output_too_large_on_70kib_state_md` + `t006_companion_advanced_timestamp_70kib_continues`
(integration test, `crates/hook-plugins/verify-state-timestamp-refresh/tests/integration_t006_no_output_too_large.rs`).

```
test t006_companion_advanced_timestamp_70kib_continues ... ok
test t006_zero_output_too_large_on_70kib_state_md ... ok
test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

**Operational-proxy design note:** AC-004's host-level `internal.capability_denied reason=output_too_large`
event is emitted by `read_file.rs::read_bounded()` in the host, not by this guard. That event is
unobservable in a mocked Rust unit/integration harness; real-event capture requires a dispatcher-level
run with `VSDD_SINK_FILE`. The operational proxy used here: `Block(TimestampStale)` at 70000 bytes
PLUS zero `"fail-open read-error"` warns in the captured `log_warn` stream. When the cap is too low,
the cap-enforcement mock returns `Err("OutputTooLarge...")` — exactly mirroring real `host::read_file`
behavior — causing the guard to emit `"fail-open read-error"` via `log_warn` and return `Continue`.
The assertion that `fail_open_warns.is_empty()` PLUS `Block` result is therefore live proof of
AC-004: the guard ran to completion without an OutputTooLarge cap denial.

**BC Trace:** BC-5.40.001 v1.2 Precondition 6 / VP-097 (operational proxy).

---

## AC-005: state_md_approaching_cap Soft-Warning Boundary A–E

**Transcript:** `transcript-AC005-soft-warning.txt`
**Test:** `test_BC_5_40_001_T007_state_md_approaching_cap_warn_boundary` — five sub-tests A–E.

```
test tests::test_BC_5_40_001_T007_state_md_approaching_cap_warn_boundary ... ok
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 49 filtered out
```

Boundary matrix verified per BC-5.40.001 v1.2 Invariant 8:

| Fixture bytes | Expected warn | Verified |
|--------------|---------------|---------|
| 150000 | no warn | Sub-test B PASS |
| 200000 (exact threshold) | no warn (strict >) | Sub-test C PASS |
| 210000 | warn emitted | Sub-test A PASS |
| 262144 (cap-exact) | warn AND read succeeds | Sub-test D PASS |
| 262145 (over cap) | StateReadError; no warn | Sub-test E PASS |

The `state_md_approaching_cap` event carries `bytes_read: u64` and `cap_bytes: u64 = 262144`
fields per Invariant 8. The event is observability-only — it never triggers a Block or alters the
Continue/Block verdict (Sub-test D confirms: cap-exact fixture reads successfully and returns
Continue for advanced timestamp, not StateReadError).

**BC Trace:** BC-5.40.001 v1.2 Invariant 8 (soft-warn threshold bytes_read in (200000, 262144]; fields bytes_read + cap_bytes).

---

## Full Test Run Summary

```
$ cargo test -p verify-state-timestamp-refresh

verify_state_timestamp_refresh unit tests (inline #[cfg(test)]):
  test result: ok. 50 passed; 0 failed; 0 ignored; 0 measured; finished in 0.00s

verify_state_timestamp_refresh integration (tests/integration_t006_no_output_too_large.rs):
  test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; finished in 0.00s
```

**Total: 52 tests GREEN. 0 failures.**

---

## Notes on Evidence Mode

This story delivers a Rust guard plugin (`verify-state-timestamp-refresh`) as a WASM hook. There
is no UI or CLI entry point to drive visually. Evidence is captured-stdout transcripts per the
library/test-harness demo mode described in the VSDD pipeline. The transcripts are reproducible:
`cargo test -p verify-state-timestamp-refresh` on branch `feature/S-19.08` (HEAD `fea77eea`)
reproduces all results.

---

## Behavioral Discrepancies Found

None. All 52 tests pass against the implementation on `feature/S-19.08`. No behavioral
discrepancy between the implementation and the ACs was observed.

---

## Files

| File | Content |
|------|---------|
| `transcript-AC001-constant.txt` | AC-001 grep gate + T-001 test (STATE_MD_MAX_BYTES == 262144) |
| `transcript-AC002-AC004-cap-enforcement.txt` | T-002/T-003 (AC-002, stale/advanced on 70 KiB) + T-006 integration (AC-004, zero output_too_large proxy) |
| `transcript-AC003-extract-frontmatter.txt` | T-004 (body bytes excluded) + T-005 (delimiter-absent fail-open) (AC-003) |
| `transcript-AC005-soft-warning.txt` | T-007 sub-tests A-E (AC-005 boundary matrix) |
| `evidence-report.md` | This file — coverage matrix + per-AC narrative |
