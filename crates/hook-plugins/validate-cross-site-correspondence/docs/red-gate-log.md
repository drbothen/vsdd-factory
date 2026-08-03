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
