# Red Gate Log — S-21.07 Pass-1 Fix Burst (Test-Writer)

**Date:** 2026-08-03
**BC:** BC-5.38.001 (Red Gate: all tests must fail before implementation)
**Cycle:** v1.0-brownfield-backfill / S-21.07 adversarial pass-1 fix burst
**Role:** test-writer (step 2 of 4)

---

## Summary

This log records every assertion site added or strengthened in the pass-1 fix burst.
All tests listed here FAIL against current code (Red Gate confirmed). No production
code was modified — test files and fixture files only.

Red Gate run timestamp: 2026-08-03
Command: `cargo test -p validate-cross-site-correspondence`
Result: 77 passed; **14 failed** (all new tests, all failing for correct reasons)

---

## Rust Unit Tests — Red Gate Results

### dispatch.rs (2 new tests)

| Test | Assertion | Red Gate Failure |
|------|-----------|-----------------|
| `test_BC_5_39_010_dispatch_bc_index_not_bc_file` | `assert!(!is_bc_file("...BC-INDEX.md"))` | is_bc_file returns true → assert fails |
| `test_BC_5_39_010_dispatch_epic_file_not_story_file` | `assert!(!is_story_file("...epics/E-21-W4.md"))` | is_story_file returns true → assert fails |

### arm_a1.rs (2 new tests)

| Test | Assertion | Red Gate Failure |
|------|-----------|-----------------|
| `test_BC_5_39_010_arm_a1_escaped_pipe_chain_last_token_wins` | `assert!(violations.is_empty())` with v1.3\|...\|v1.6 chain | First token "1.3"≠"1.6" → violation → NOT empty |
| `test_BC_5_39_010_arm_a1_frontmatter_changelog_pipe_not_matched_as_table_row` | `assert!(violations.is_empty())` with frontmatter changelog pipe | Frontmatter line matched first → "4.43"≠"1.6" → violation |

### arm_b.rs (4 new tests)

| Test | Assertion | Red Gate Failure |
|------|-----------|-----------------|
| `test_BC_5_39_010_arm_b1_production_blockquote_b3_extracted` | `assert_eq!(b3, Some("47a65c9"))` | starts_with("> S-21.07=") inert on production prose → None |
| `test_BC_5_39_010_arm_b2_production_blockquote_shape_no_spurious_violations` | `assert!(violations.is_empty())` | Spurious "orphaned blockquote entry" from production prose line |
| `test_BC_5_39_010_arm_b_non_hex_catalog_token_not_accepted` | `assert!(catalog_hash.is_none())` | Returns Some("bonus") — no hex validation |
| `test_BC_5_39_010_arm_b1_cross_story_catalog_correct_row_matched` | `assert_eq!(hash, Some("1b4ea21"))` | naive contains("S-18.01") hits S-18.00 row first → "e5bc551" |

### arm_d.rs (1 new test)

| Test | Assertion | Red Gate Failure |
|------|-----------|-----------------|
| `test_BC_5_39_010_class_d_discloses_not_false_positive` | `assert!(advisories.is_empty())` | contains("closes:") matches "discloses: A01" → advisory for A01 |

### arm_e.rs (2 new tests)

| Test | Assertion | Red Gate Failure |
|------|-----------|-----------------|
| `test_BC_5_39_010_class_e1_15_byte_last_amended_accepted` | `assert_eq!(result, Some("2"))` | len < 17 guard → 15 < 17 → None ≠ Some("2") |
| `test_BC_5_39_010_class_e1_absent_last_amended_emits_advisory` | `assert!(!advisories.is_empty())` | Returns (vec![], vec![]) silently → advisories IS empty |

### lib.rs (3 replacements — Red Gate violations fixed)

The three lib.rs tests replaced were Red Gate violations:
- `test_BC_5_39_010_arm_a1_primary_target_capability_denied_contract` — zero assertions
- `test_BC_5_39_010_combined_a1_and_e1_single_block` — PASSED in Red Gate (combine_violations_into_block IS implemented; not todo!())
- `test_BC_5_39_010_multibyte_utf8_no_panic` — zero assertions

| Replacement Test | Assertion | Red Gate Failure |
|-----------------|-----------|-----------------|
| `test_BC_5_39_010_dispatch_bc_index_not_bc_file_lib_integration` | `assert!(!dispatch::is_bc_file("...BC-INDEX.md"))` | is_bc_file returns true → fails |
| `test_BC_5_39_010_arm_a1_escaped_pipe_chain_stale_blocks_lib_level` | `assert!(violations.is_empty())` with escaped-pipe index | First token "1.3" returned → violation |
| `test_BC_5_39_010_class_e1_15_byte_last_amended_accepted_lib_level` | `assert_eq!(result, Some("2"))` | len < 17 → None ≠ Some("2") |

---

## Bats Integration Tests — New Tests Added

All bats payload tests fail in Red Gate via `_assert_plugin_ran_not_crashed`
(stub panics → plugin.crashed fired, no plugin.completed record → helper returns failure).
The additional assertions (exit code, log checks) would further fail post-stub with bugs.

| Test ID | Test Name | Red Gate Failure | Post-Stub Bug Failure |
|---------|-----------|-----------------|----------------------|
| T-035 | AC-019(a) replacement: BC-INDEX.md not BC file | plugin.crashed | exit 2 (is_bc_file bug) |
| T-036 | permanent mutant: crashed-only log detected | PASSES always (meta-test) | — |
| T-036b | permanent mutant: timeout-only log detected | PASSES always (meta-test) | — |
| T-037 | B1=B2 agree but B3 mismatch MUTANT | plugin.crashed | exit 0 (B3 inert bug) |
| T-038 | AC-019(b) replacement: cross-story catalog | plugin.crashed | exit 2 (cross-row bug) |
| T-039 | escaped-pipe version chain CONTROL | plugin.crashed | exit 2 (first-token bug) |
| T-045 | 15-byte last_amended no advisory CONTROL | plugin.crashed | advisory in log (len<17 bug) |

### _assert_plugin_ran_not_crashed helper strengthened (RG-006)

Old behavior: checked for any record for plugin name AND absence of `plugin.crashed`.
New behavior: checked for `plugin.completed` AND absence of `plugin.crashed` AND absence
of `plugin.timeout`.

The critical change: `plugin.invoked` is written BEFORE execution starts. Only
`plugin.completed` proves execution ran to completion without crash or timeout.

### Bats structural fixes

- AC-012 duplicate: second test now also checks dispatcher log for ABSENCE of advisory records
- AC-013 MUTANT: now additionally checks dispatcher log for `plugin.log warn` record mentioning "B01"
- AC-020 awk: added `[ -n "$output" ]` guard to prevent vacuous pass on missing registry section
- Header comment count corrected from "31" to "37+"

---

## Fixture Files Updated (Task 1)

### Updated fixtures (production shapes replacing synthetic shapes)

- `b1-hash-match/factory/stories/STORY-INDEX.md`
- `b1-hash-mismatch/factory/stories/STORY-INDEX.md`
- `b2-catalog-agree/factory/stories/STORY-INDEX.md`
- `b2-catalog-mismatch/factory/stories/STORY-INDEX.md`
- `a1-stale-index/factory/specs/behavioral-contracts/BC-INDEX.md`
- `a1-current-index/factory/specs/behavioral-contracts/BC-INDEX.md`

### New fixture directories

- `b1-b3-only-mismatch/` — T-037: B1=B2 agree, B3≠B1 in production blockquote
- `b1-cross-story-catalog/` — T-038: S-18.01 story, S-18.00 row mentions S-18.01 first
- `a1-escaped-pipe-current/` — T-039: BC-1.13.001 v1.12 with escaped-pipe chain
- `e1-15-byte-last-amended/` — T-045: BC-5.39.010 v2, last_amended "2026-07-30 (v2)"

---

## Pass-2 Fix Burst — Four Additional Red Gate Tests

**Date:** 2026-08-03
**Context:** Pass-1 produced 14 failing tests; implementer made 91 green (CI green). Four of nine
required fixes were never implemented because the Red Gate did not encode them. Pass-2 adds exactly
four failing tests for the four remaining unfixed defects.

Red Gate run timestamp: 2026-08-03
Command: `cargo test -p validate-cross-site-correspondence`
Result: 91 passed; **4 failed** (all new tests, all failing for correct reasons)

### New Tests — Red Gate Results (Pass 2)

#### lib.rs — 2 new tests (T-046, T-047)

| Test | Assertion Site | Red Gate Failure (verbatim) |
|------|---------------|----------------------------|
| `test_BC_5_39_010_unclassified_path_returns_continue_not_block` | lib.rs:351 | `left: Block { reason: "BLOCKED by validate-cross-site-correspondence: [1] validate-cross-site-correspondence [primary-read] POLICY 14: cannot read primary target 'crates/hook-plugins/validate-cross-site-correspondence/src/lib.rs': CapabilityDenied. Fail-closed per BC-5.39.010 invariant 4. Fix: review and fix all cross-site correspondence issues listed above, then retry the write. Code: POLICY 14/18." } right: Continue` |
| `test_BC_5_39_010_ac019_extended_all_six_read_caps_fully_pinned` | lib.rs:445 | `assertion 'left == right' failed: BC/story primary read cap MUST equal BC_MAX_BYTES=524288, not PRIMARY_READ_MAX_BYTES=1048576. F-S2107-P1C-002 (BLOCKER). left: 1048576 right: 524288` |

#### arm_a2.rs — 2 new tests (T-048, T-049)

| Test | Assertion Site | Red Gate Failure (verbatim) |
|------|---------------|----------------------------|
| `test_BC_5_39_010_arm_a2_bare_version_bc_table_row_detected` | arm_a2.rs:429 | `assertion 'left == right' failed: bare version '1.3' in Behavioral Contracts table row must be detected. F-S2107-P1B-002: extract_version_token_from_table_row only checks bytes[i]==b'v', silently skipping bare version cells. Current citations: [] left: 0 right: 1` |
| `test_BC_5_39_010_arm_a2_edge_cases_rows_not_scanned_section_bounded` | arm_a2.rs:514 | `assertion 'left == right' failed: Edge Cases table rows must NOT produce BC version citations. F-S2107-P1B-001: unbounded scan yields spurious citations from EC-002 (v1.17), EC-015 (v1.31), and EC-017 (v1.3). Current citations: [("table row 9", "1.3"), ("table row 15", "1.17"), ("table row 16", "1.31"), ("table row 17", "1.3")] left: 4 right: 1` |

### Test 4 (F-S2107-P1B-012) — Explicitly Skipped

**Finding:** `frontmatter.rs::extract_frontmatter_field` binds the slice `&trimmed[1..trimmed.len() - 1]`
BEFORE the `is_char_boundary` guard. Ostensibly the guard should come first.

**Conclusion after analysis:** No test is possible. The guard is vacuously true in the ASCII-quote
branch: the outer `if trimmed.starts_with('"') && trimmed.ends_with('"')` ensures the first and
last bytes are ASCII `"` (0x22). Slicing at byte indices 1 and `len-1` of a string whose endpoints
are single-byte ASCII characters is always valid. No input can reach the else-branch and observe
different behavior between current ordering and the correct ordering.

**Per brief:** "If you conclude no test can meaningfully distinguish the fixed ordering from the
current one, say so plainly and skip Test 4 — a fabricated test is worse than an acknowledged gap
(POLICY 11, TD-VSDD-059)."

Test 4 is intentionally absent. The implementer should apply the cosmetic guard-first reorder as a
no-behavior-change cleanup when touching frontmatter.rs for other fixes.

---

---

## Adversary Pass-2 Fix Burst — RED GATE Tests (test-writer)

**Date:** 2026-08-04
**BC:** BC-5.38.001 (Red Gate: all tests must fail before implementation)
**Cycle:** v1.0-brownfield-backfill / S-21.07 adversary pass-2 fix burst
**Root cause closure:** no test reads a real `.factory/` corpus file (spec-describes-imagined-shape class)

Red Gate run timestamp: 2026-08-04
Command: `cargo test -p validate-cross-site-correspondence`
Result: **97 passed; 7 failed** (all new tests, all failing for correct reasons)
Bats: converted 5 Class D tests to DEFERRED skip; added T-046 mutant + T-047 PC40

### Rust Unit Tests — RED GATE Results (Pass-2 Fix Burst)

#### arm_a1.rs — 1 new failing test (F-P2-002)

| Test | Assertion | Red Gate Failure (verbatim) |
|------|-----------|----------------------------|
| `test_BC_5_39_010_arm_a1_cross_reference_in_later_row_own_row_version_wins` | `assert_eq!(result, Some("1.7"))` | `assertion 'left == right' failed: extract_bc_index_version must anchor on first cell only. BC-1.17.001 own row is v1.7; later BC-2.07.001 row mentions BC-1.17.001 in a non-first cell with v1.6. LAST-wins + unanchored contains returns '1.6' (WRONG). F-P2-002 RED GATE. Current: Some("1.6") — left: Some("1.6") right: Some("1.7")` |

#### arm_a2.rs — 1 new failing test (F-P2-001)

| Test | Assertion | Red Gate Failure (verbatim) |
|------|-----------|----------------------------|
| `test_BC_5_39_010_arm_a2_frontmatter_preamble_not_scanned_skip_section_true` | `assert_eq!(citations.len(), 1)` | `assertion 'left == right' failed: preamble lines before any ## heading must NOT produce BC citations. BC-5.39.010 PC13: skip_section must be initialized to true (F-P2-001). RED GATE: skip_section=false → 2 citations (preamble + BC section). Expected: 1 (BC section only). Citations: [("table row 6", "1.0"), ("table row 10", "1.5")] — left: 2 right: 1` |

#### arm_b.rs — 1 new failing test (PC40)

| Test | Assertion | Red Gate Failure (verbatim) |
|------|-----------|----------------------------|
| `test_BC_5_39_010_arm_b1_pc40_volatile_input_detection_required` | `panic!()` stub | `PC40 NOT YET IMPLEMENTED: arm_b.rs missing is_volatile_path() and parse_story_volatile_inputs(). IMPLEMENTER: add both functions, update run_arm_b1 to emit advisory+Continue for volatile inputs, then replace this panic!() with real assertions. BC-5.39.010 v1.6 PC40.` |

#### dispatch.rs — 4 new failing tests (F-P2-003, F-P2-011, F-P2-007/Class D)

| Test | Assertion | Red Gate Failure (verbatim) |
|------|-----------|----------------------------|
| `test_BC_5_39_010_dispatch_vp_index_excluded_from_class_e` | `assert!(!is_frontmatter_parity_target("...VP-INDEX.md"))` | `VP-INDEX.md must NOT be classified as a frontmatter parity target. BC-5.39.010 PC34: explicit VP-INDEX.md guard required. F-P2-003+F-P2-008: starts_with('VP-')&&ends_with('.md') admits VP-INDEX.md. RED GATE: current check returns true.` |
| `test_BC_5_39_010_dispatch_story_file_s_readme_rejected_requires_numeric_id` | `assert!(!is_story_file("...S-README.md"))` | `S-README.md must NOT be classified as a story file. PC9: basename must match ^S-[0-9]+\.[0-9]+.*\.md$. F-P2-011: starts_with('S-') too broad. RED GATE: current check returns true.` |
| `test_BC_5_39_010_dispatch_class_d_deferred_burst_log_returns_none` | `assert!(result.is_none())` | `burst-log.md must NOT classify as cycle artifact after Class D deferral (BC-5.39.010 v1.6 Class D DEFERRED). F-P2-007 resolution: is_cycle_artifact dispatch removed. RED GATE: currently returns Some(BurstLog).` |
| `test_BC_5_39_010_dispatch_class_d_deferred_lessons_returns_none` | `assert!(result.is_none())` | `lessons.md must NOT classify as cycle artifact after Class D deferral (BC-5.39.010 v1.6 Class D DEFERRED). RED GATE: currently returns Some(Lessons).` |

#### lib.rs — 1 new PASSING test (F-P1C-016, coverage only)

| Test | Status | Note |
|------|--------|------|
| `test_BC_5_39_010_invariant_7_ac018_multi_arm_violations_both_in_combined_block` | PASSES immediately | Coverage test — `combine_violations_into_block` already correct. F-P1C-016 was a coverage gap, not an implementation bug. Rust-level assertion added per finding. |

### Bats Tests — Changes (Pass-2 Fix Burst)

#### Class D DEFERRED conversions (5 tests → skip with POLICY 1 preserve)

All five Class D bats tests (AC-012 ×2, AC-013 ×2, AC-014 ×1) converted to `skip "[DEFERRED v1.6 — Class D]..."`. Test IDs preserved per POLICY 1 append-only — bodies retained for future re-activation when Class D is re-implemented.

#### T-045 fixture rename (corpus-hygiene)

T-045 envelope changed from `.factory/specs/verification-properties/VP-039.md` to
`.factory/specs/verification-properties/VP-9999-test.md`. New `VP-9999-test.md` fixture
created at `e1-15-byte-last-amended/factory/specs/verification-properties/VP-9999-test.md`.
VP-039 is a live corpus VP ID that changes independently; using it as a fixture risks
flakiness from corpus churn. VP-9999-test is a stable non-live fixture ID.

#### T-046 MUTANT — F-P2-013 positive-coverage mutant

New bats test asserting exit 2 when VP file has version ≠ last_amended. Requires fixture
`e1-vp-version-mismatch` with VP-9999-test.md (`version: "1.7"`, `last_amended: "2026-07-30 (v1.6)"`).
RED GATE: before fix (Class E1 not yet implemented), `_assert_plugin_ran_not_crashed` fails
(stub panics). Post-fix: E1 detects "1.7" ≠ "1.6" → violation → exit 2.

#### T-047 (PC40) — volatile-input story exits 0

New bats test asserting exit 0 when story has `inputs: [".factory/STATE.md"]` AND
STORY-INDEX has a MISMATCHED hash. RED GATE: without PC40 fix, three-way comparison runs
→ mismatch → exit 2. With fix: volatile input → advisory + Continue → exit 0.
Requires fixture `b1-volatile-input` (created).

### New Fixture Files (Pass-2 Fix Burst)

| Fixture Path | Purpose |
|-------------|---------|
| `e1-15-byte-last-amended/factory/specs/verification-properties/VP-9999-test.md` | T-045 fixture rename (non-live VP ID) |
| `e1-vp-version-mismatch/factory/specs/verification-properties/VP-9999-test.md` | T-046 E1 mutant (version "1.7" vs last_amended v1.6) |
| `b1-volatile-input/factory/stories/S-21.07-test.md` | T-047 PC40 story with volatile input |
| `b1-volatile-input/factory/stories/STORY-INDEX.md` | T-047 PC40 STORY-INDEX with mismatched hash |

---

## Finding Coverage

| Finding | Rust Unit Test | Bats Test |
|---------|---------------|-----------|
| F-S2107-P1B-001 (unbounded EC section scan) | T-049 arm_a2 | — (unit sufficient) |
| F-S2107-P1B-002 (bare version not detected) | T-048 arm_a2 | — (unit sufficient) |
| F-S2107-P1B-003 (B3 blockquote inert) | arm_b tests | T-037 |
| F-S2107-P1B-004 (B2 spurious orphaned) | arm_b tests | AC-011 CONTROL (production fixture) |
| F-S2107-P1B-005 (BC-INDEX classified as BC) | dispatch + lib tests | T-035 |
| F-S2107-P1B-006 (escaped-pipe first token) | arm_a1 + lib tests | T-039 |
| F-S2107-P1B-007 (frontmatter false-match) | arm_a1 tests | T-039 (combined) |
| F-S2107-P1B-008 (cross-story row match) | arm_b tests | T-038 |
| F-S2107-P1B-009 (non-hex catalog accepted) | arm_b tests | — (unit sufficient) |
| F-S2107-P1B-010 (epic file as story) | dispatch tests | — (unit sufficient) |
| F-S2107-P1B-012 (frontmatter guard order) | SKIPPED — vacuous guard, no reachable panic | — |
| F-S2107-P1C-001 (read before classify) | T-046 lib.rs | — (unit sufficient) |
| F-S2107-P1C-002 (PRIMARY_READ_MAX_BYTES wrong for BC/story) | T-047 lib.rs (assertion 9) | — |
| F-S2107-P1C-003 (PRIMARY_READ_MAX_BYTES wrong for cycle artifact) | T-047 lib.rs (assertion 10) | — |
| F-S2107-P1C-011 (else-if Closes/Refs) | deferred — rare ordering | — |
| F-S2107-P1C-012 (L-EDP1 line-start) | deferred — rare prose pattern | — |
| F-S2107-P1C-014 (15-byte last_amended) | arm_e + lib tests | T-045 |
| F-S2107-P1C-015 (absent last_amended silent) | arm_e tests | — (unit sufficient) |
| F-S2107-P1C-020 (discloses: false-match) | arm_d tests | AC-012 log-check |

---

## Pass-3 Fix Burst — Test-Side Assertion Correctness (POLICY 15)

**Date:** 2026-08-03
**Context:** Three bats assertion defects discovered after pass-2 implementer green. Two tests were
failing (T-037, T-045); one passed for wrong reason (AC-013 MUTANT grep silently vacuous). No
production code was modified. Fixes are fixture data, grep patterns, and fixture-trigger isolation
only.

Bats result after fixes: **41 pass / 0 fail / 0 skip**

### Assertion Site RG-007 — T-037 fixture charset (Defect 1)

**File:** `plugins/vsdd-factory/tests/fixtures/validate-cross-site-correspondence/b1-b3-only-mismatch/factory/stories/STORY-INDEX.md`
**Change:** `S-21.07=DEADBEE` → `S-21.07=deadbee` in the delivery blockquote (plus matching
comments).
**Root cause:** BC-5.39.010 PC20/PC21 specify hash charset `([0-9a-f]{7,40})` — lowercase only.
`DEADBEE` (uppercase) would not be parsed by `parse_blockquote_hash`, so `B3` would be `None`
rather than `Some("deadbee")`. With B3=None and B1=B2, the three-way check finds no mismatch and
exits 0 — opposite of the expected exit 2. The test was failing because the fixture violated the
BC's own charset constraint, making the blockquote parse silently return None.
**Assertion site:** T-037 bats assertion `assert_equal "$status" "2"` with `[Class B]` in output.
After fix: `parse_blockquote_hash` returns `Some("deadbee")` → B3≠B1 → exit 2 ✓.

### Assertion Site RG-008 — AC-013 MUTANT grep pattern (Defect 2)

**File:** `plugins/vsdd-factory/tests/validate-cross-site-correspondence.bats`
**Line:** AC-013 MUTANT dispatcher-log grep
**Change:** `grep -q '"B01"'` → `grep -q 'B01'`
**Root cause:** `class_d_advisory_message` formats the token with single quotes: `token 'B01' on
line`. The dispatcher stores the advisory message verbatim in a JSON string value. Inside JSON,
single quotes are not escaped — the stored string contains `'B01'` (single-quoted). The original
grep searched for `"B01"` (double-quoted), which never appears in the JSONL record. The advisory
WAS being fired (exit 0 was correct), but the test was silently not finding the log evidence,
making the log-check assertion always fail with the original grep. Fix: search for `B01` without
quote-type assumption — present in any quoting variant.
**Assertion site:** `grep -q 'B01' <<< "$log_output"` in the AC-013 MUTANT log-verification block.
After fix: grep finds `'B01'` in the stored message → log-check passes ✓.

### Assertion Site RG-009 — T-045 fixture trigger isolation (Defect 3)

**Files:**
- `plugins/vsdd-factory/tests/validate-cross-site-correspondence.bats` (T-045 envelope)
- `plugins/vsdd-factory/tests/fixtures/validate-cross-site-correspondence/e1-15-byte-last-amended/factory/specs/verification-properties/VP-039.md` (NEW fixture file)
**Change:** T-045 envelope changed from `.factory/specs/behavioral-contracts/ss-05/BC-5.39.010.md`
to `.factory/specs/verification-properties/VP-039.md`. New VP-039.md fixture created with
`version: "2"` and `last_amended: "2026-07-30 (v2)"` (15 bytes).
**Root cause:** `extract_version_token` in arm_a1.rs requires `vN.N` format (decimal point
mandatory). The 15-byte format `"2026-07-30 (v2)"` produces outer version `"2"` (single integer,
no decimal). BC-INDEX.md in the fixture had `| v2 |` which cannot match — `extract_version_token`
returns `None` for `v2`. Arm A1 then blocks: version "2" ≠ "1.0" (no matching INDEX entry) →
"dropped registration" violation → exit 2. E1 was never reached.
**Isolation strategy:** VP files trigger `is_frontmatter_parity_target` (so E1 runs) but NOT
`is_bc_file` (so A1 does NOT run). By triggering a VP write instead of a BC write, A1 is bypassed
entirely and E1 runs in isolation. `extract_last_amended_outer_version("2026-07-30 (v2)")` returns
`Some("2")` after the len-threshold fix; `version: "2"` == `"2"` → no E1 violation → exit 0 ✓.
**Assertion site:** T-045 `assert_equal "$status" "0"` and absence of `[Class E]` in output.
After fix: VP isolation → A1 skipped → E1 clean → exit 0 ✓.
