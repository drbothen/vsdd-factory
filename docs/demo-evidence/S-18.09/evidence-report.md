# Demo Evidence — S-18.09: F2 Process-Gap Lesson Gate Checks

**Story:** S-18.09 (v1.18)
**Branch:** feature/S-18.09
**Suite:** `plugins/vsdd-factory/tests/f2-process-gap-lesson-gates.bats`
**Product type:** Gate-enforcement bats suite (CLI/shell — no UI)
**Recording method:** Literal bats execution output (VHS not applicable; deliverable is a bats gate suite, not a CLI tool)

---

## Gate Run: All 8 Tests Green

Command executed from worktree root (`/Users/zious/Documents/GITHUB/vsdd-factory/.worktrees/S-18.09`):

```
bats plugins/vsdd-factory/tests/f2-process-gap-lesson-gates.bats
```

**Captured output (verbatim):**

```
1..8
ok 1 test_e18_bats_tests_use_machine_stable_assertions_not_presentation_regex
ok 2 test_e18_vp_source_bc_files_exist_and_are_reachable
ok 3 test_e18_hook_scripts_no_bypass_on_load_bearing_writes
ok 4 test_s18_08_discovery_scan_enumerates_and_counts_before_loop
ok 5 test_e18_story_behavioral_contracts_bc_ids_resolve_to_existing_bc_files
ok 6 test_e18_spec_set_no_stale_current_wave_term_in_normative_sections
ok 7 test_e18_bc_preconditions_toml_blocks_have_canonical_name_and_plugin_fields
ok 8 test_e18_ac_traces_resolve_to_real_bc_clause_numbers
```

Exit code: 0. No failures, no skipped tests.

---

## Per-Gate Summary (AC-001 through AC-008)

| Test # | Test Name | AC | What It Enforces | Result |
|--------|-----------|-----|-----------------|--------|
| 1 | `test_e18_bats_tests_use_machine_stable_assertions_not_presentation_regex` | AC-001 | E-18 bats test files (`validate-heavy-op-delegation.bats`, `pure-parse-invariant-gate.bats`) use `"code":"DelegationRecommended"` structured `plugin.log` field assertions, NOT `grep -c "^  - "` presentation-coupled regex over human-readable output (L-F2-machine-stable-count-assertion). Gate scans for 0 anti-pattern hits and >0 stable assertions. | PASS |
| 2 | `test_e18_vp_source_bc_files_exist_and_are_reachable` | AC-002 | Each E-18 VP (VP-088, VP-089, VP-090, VP-091) has a `source_bc:` frontmatter field that resolves to a real BC file on disk (L-F2-fix-at-correct-layer — VP body assertions derive only from reachable source BCs). | PASS |
| 3 | `test_e18_hook_scripts_no_bypass_on_load_bearing_writes` | AC-003 | `plugins/vsdd-factory/hooks/postcompact-reanchor.sh` exists and is non-empty (positive-coverage guard), and contains 0 `|| true` suffixes on load-bearing write operations (git commit/push/add, literal `.jsonl` appends). Intentionally fail-open advisory-log appends using variable paths (`>> "$log_file"`) are correctly out of gate scope. (L-F2-no-bypass-on-edit-failure) | PASS |
| 4 | `test_s18_08_discovery_scan_enumerates_and_counts_before_loop` | AC-004 | `plugins/vsdd-factory/tests/pure-parse-invariant-gate.bats` (S-18.08 deliverable) contains both `discovered_count` enumeration variable and a `discovered_count -eq 0` empty-discovery guard — ensuring the scan fails explicitly on zero-BC discovery rather than silently passing over an empty set. (L-F2-exhaustive-sweep-enumerate-and-count) | PASS |
| 5 | `test_e18_story_behavioral_contracts_bc_ids_resolve_to_existing_bc_files` | AC-005 | For every E-18 story (S-18.00 through S-18.09) with a non-empty `behavioral_contracts:` frontmatter array (both multi-line YAML and inline `[...]` forms), each BC ID resolves to an existing BC file whose H1 heading contains the BC ID in either the `# BC-NNN:` or `# Behavioral Contract BC-NNN:` corpus form. (D-576 cross-reference title-cite parity) | PASS |
| 6 | `test_e18_spec_set_no_stale_current_wave_term_in_normative_sections` | AC-006 | STALE_HITS=0: no BC or VP file in the E-18 spec set (ss-04, ss-05, ss-06, ss-07, VP-088/089/090/091) contains the retired `current_wave:` field in a normative behavioral claim. Historical/changelog mentions and negation/prohibition statements (including the `not stored as` cue added in v1.18 to make BC-7.07.002:88 self-sufficient) are correctly excluded via case-insensitive filter. (F2 lessons stale-term detector) | PASS |
| 7 | `test_e18_bc_preconditions_toml_blocks_have_canonical_name_and_plugin_fields` | AC-007 | Both BC-4.14.001 and BC-4.15.001 `§Preconditions` sections contain TOML `[[hooks]]` blocks with BOTH `name = "..."` AND `plugin = "hook-plugins/....wasm"` fields — enforcing the canonical native-WASM shape per D-576. Guards against regression to the v1.8 shape violation (bare logical name without `name =` field). | PASS |
| 8 | `test_e18_ac_traces_resolve_to_real_bc_clause_numbers` | AC-008 | Every `(traces to BC-X.XX.XXX PC-N / INV-N)` parenthetical in E-18 story `## Acceptance Criteria` sections resolves to a real numbered clause in the cited BC's `§Postconditions` or `§Invariants` section. Handles: compound cites (`+`/`;`/`,` separators), BC-ID carry-forward, PC-letter labels (BC-4.15.001 form), PC-numeric stripping, keyword-less `PC-N`/`INV-N` form (F-P1-001; closes S-18.13 vacuity), and fenced-code-block exclusion (prevents gate's own worked examples from self-triggering). Non-vacuity guard ensures TRACES_CHECKED > 0 for stories with non-empty `behavioral_contracts:`. (O-P4-004 AC↔PC parity gate) | PASS |

---

## What This Suite Enforces

The `f2-process-gap-lesson-gates.bats` suite machine-enforces the F2 adversarial-convergence process-gap lessons at the E-18 wave-8 terminal gate boundary. The F2 lessons were first identified during the `v1.0-brownfield-backfill` cycle convergence passes and codified in `.factory/cycles/v1.0-brownfield-backfill/lessons.md`. The suite translates four of those lessons into executable assertions:

- **L-F2-machine-stable-count-assertion** (AC-001): test harnesses must assert against structured `plugin.log code:` fields, not presentation-coupled regex over human-readable bulletin text — because rendering changes break assertion correctness silently.
- **L-F2-fix-at-correct-layer** (AC-002): VP body assertions must derive from their source BC guarantors; the gate enforces this by verifying every E-18 VP's `source_bc:` file is reachable.
- **L-F2-no-bypass-on-edit-failure** (AC-003): load-bearing hook write operations must not be suppressed with `|| true`; the gate scans the E-18 postcompact-reanchor hook deliverable.
- **L-F2-exhaustive-sweep-enumerate-and-count** (AC-004): discovery scans must enumerate and count discovered artifacts before looping — an empty discovery must fail explicitly, not vacuously succeed.

Two additional gates enforce D-576 registry-block-shape discipline: AC-005 (cross-reference title-cite parity — story frontmatter BC IDs must correspond to files with correct H1 headings) and AC-007 (BC precondition `[[hooks]]` TOML blocks must carry both `name =` and `plugin =` fields per the canonical native-WASM shape). AC-006 (stale-term detector) ensures the retired `current_wave:` field does not resurface in normative E-18 spec content. AC-008 (O-P4-004 AC↔PC parity gate) closes the recurring mis-numbering process gap by machine-verifying that every `(traces to ...)` parenthetical in E-18 story bodies resolves to a real numbered clause in the cited BC; this gate references ADR-026 §Decision 8 for the pure-parse invariant grounding.

---

## Coverage Mapping

| AC | Test | Gates Lesson / Rule | Pass |
|----|------|---------------------|------|
| AC-001 | ok 1 | L-F2-machine-stable-count-assertion | yes |
| AC-002 | ok 2 | L-F2-fix-at-correct-layer | yes |
| AC-003 | ok 3 | L-F2-no-bypass-on-edit-failure | yes |
| AC-004 | ok 4 | L-F2-exhaustive-sweep-enumerate-and-count | yes |
| AC-005 | ok 5 | D-576 cross-reference title-cite parity | yes |
| AC-006 | ok 6 | F2 stale-term detector | yes |
| AC-007 | ok 7 | D-576 BC-precondition registry-block-shape | yes |
| AC-008 | ok 8 | O-P4-004 AC↔PC parity gate | yes |

All 8 acceptance criteria covered. No error paths remain ungated (the suite uses `assert_success` + `refute_output --partial "FAIL"` on every test, enforcing both exit-code and output-content correctness per AC-008 bats @test fatal-path contract).
