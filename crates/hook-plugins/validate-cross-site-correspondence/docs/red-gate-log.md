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

## Finding Coverage

| Finding | Rust Unit Test | Bats Test |
|---------|---------------|-----------|
| F-S2107-P1B-003 (B3 blockquote inert) | arm_b tests | T-037 |
| F-S2107-P1B-004 (B2 spurious orphaned) | arm_b tests | AC-011 CONTROL (production fixture) |
| F-S2107-P1B-005 (BC-INDEX classified as BC) | dispatch + lib tests | T-035 |
| F-S2107-P1B-006 (escaped-pipe first token) | arm_a1 + lib tests | T-039 |
| F-S2107-P1B-007 (frontmatter false-match) | arm_a1 tests | T-039 (combined) |
| F-S2107-P1B-008 (cross-story row match) | arm_b tests | T-038 |
| F-S2107-P1B-009 (non-hex catalog accepted) | arm_b tests | — (unit sufficient) |
| F-S2107-P1B-010 (epic file as story) | dispatch tests | — (unit sufficient) |
| F-S2107-P1C-014 (15-byte last_amended) | arm_e + lib tests | T-045 |
| F-S2107-P1C-015 (absent last_amended silent) | arm_e tests | — (unit sufficient) |
| F-S2107-P1C-020 (discloses: false-match) | arm_d tests | AC-012 log-check |
| F-S2107-P1C-011 (else-if Closes/Refs) | deferred — rare ordering | — |
| F-S2107-P1C-012 (L-EDP1 line-start) | deferred — rare prose pattern | — |
