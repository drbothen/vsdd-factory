---
document_type: adversarial-review
level: ops
version: "1.0"
status: complete
producer: adversary
timestamp: 2026-04-29T00:00:00
phase: 5
inputs:
  - .factory/stories/S-5.04-post-tool-use-failure.md
  - .factory/specs/behavioral-contracts/ss-04/BC-4.08.001.md
  - .factory/specs/behavioral-contracts/ss-04/BC-4.08.002.md
  - .factory/specs/behavioral-contracts/ss-04/BC-4.08.003.md
  - .factory/specs/verification-properties/VP-068.md
  - .factory/specs/prd.md
  - .factory/specs/behavioral-contracts/BC-INDEX.md
  - .factory/specs/architecture/SS-04-plugin-ecosystem.md
  - .factory/stories/STORY-INDEX.md
input-hash: "cb977a6"
traces_to: prd.md
pass: 10
previous_review: ADV-S5.04-P09.md
pass_id: ADV-S5.04-P10
story_id: S-5.04
verdict: CLOCK_RESET
convergence_step: 0_of_3
findings_count: { CRIT: 0, HIGH: 1, MED: 2, OBS: 1, total: 4 }
---

# Adversarial Review: S-5.04 PostToolUseFailure Hook Wiring (Pass 10)

## Finding ID Convention

Finding IDs use the format: `ADV-<CYCLE>-P<PASS>-<SEV>-<SEQ>`

- `ADV`: Fixed prefix identifying adversarial findings
- `<CYCLE>`: Cycle prefix (omitted — no current-cycle file; falls back to `ADV-P<PASS>-<SEV>-<SEQ>`)
- `<PASS>`: Two-digit pass number
- `<SEV>`: Severity abbreviation (`CRIT`, `HIGH`, `MED`, `LOW`, `OBS`)
- `<SEQ>`: Three-digit sequence within the pass

Per-pass IDs used in this review: `HIGH-P10-001`, `MED-P10-001`, `MED-P10-002`, `OBS-P10-001`.

## Part A — Fix Verification (pass >= 2 only)

| ID | Previous Severity | Status | Notes |
|----|-------------------|--------|-------|
| MED-P09-001 | MED | VERIFIED | Story line 53 updated to `CAP-002 ("Hook Claude Code tool calls with sandboxed WASM plugins")` with widening note. Sibling-aligned. |
| MED-P09-002 | MED | VERIFIED | Story AC6 and Task 5 updated to 9 test functions; `test_bc_4_08_002b_platform_variants_in_sync` enumerated. |
| OBS-P08-001 | LOW | DEFERRED (carry-forward) | BC-4.08.001/002/003 `lifecycle_status: draft` sibling parity gap — pending intent verification. Non-blocking per pass-8 adjudication. Not re-raised this pass. |

Pass-9 CLOCK_RESET fix burst confirmed: both MED-P09-001 and MED-P09-002 closed. However, the pass-9 fix burst itself introduced 3 residual propagation gaps (pass-9 count-propagation discipline did not grep for old count value "8" across all artifact locations).

## Part B — New Findings

Fresh-context 30-axis sweep executed across:
- STORY-INDEX version column vs. story file frontmatter version
- Story File Structure Requirements test count vs. story body AC/Task counts
- VP-068 Feasibility Assessment math vs. story body test count
- PRD↔BC-INDEX count parity
- VP-068 source_bcs[] field membership
- BC-INDEX row vs. physical file alignment
- Story AC coverage vs. BC list
- Pass-9 count-propagation sweep completeness (grepped for "8" across artifact corpus)

### CRITICAL

None.

### HIGH

#### HIGH-P10-001: STORY-INDEX line 106 version column shows v2.3; story frontmatter is v2.4

- **Severity:** HIGH
- **Category:** spec-consistency
- **Location:** `.factory/stories/STORY-INDEX.md` line 106 (S-5.04 row, Version column)
- **Description:** STORY-INDEX version column for S-5.04 reads `2.3 (pass-1/2/4 fix bursts; ...)`. The story file frontmatter is `version: "2.4"` after the pass-9 fix burst. This is a recurrence of MED-P07-001 (same file, same column), which was closed at pass-8. The pass-9 fix burst updated the story to v2.4 but did not propagate the version update to STORY-INDEX. Severity elevated to HIGH (from MED-P07-001) due to recurrence of the same pattern after explicit remediation.
- **Evidence:** `STORY-INDEX.md` line 106 descriptor string begins `2.3`; `S-5.04-post-tool-use-failure.md` frontmatter `version: "2.4"`.
- **Proposed Fix:** Update STORY-INDEX line 106 version column to `2.4 (pass-1/2/4/9 fix bursts; BC-4.08.001-003 + VP-068; CAP-002 sibling-aligned; 1000-char truncation; tool-failure-hooks crate; 9 VP-068 tests incl. platform-variant sync)`.
- **Root cause:** Pass-9 fix burst lacked a STORY-INDEX propagation step. Count-propagation discipline (S-7.02) requires grepping for the old version string after every story version bump.
- **Blocking:** Yes — STORY-INDEX is the canonical index read by orchestrator at session start.

### MEDIUM

#### MED-P10-001: Story File Structure Requirements line 182 still reads "(8 test functions)"

- **Severity:** MED
- **Category:** spec-completeness
- **Location:** `.factory/stories/S-5.04-post-tool-use-failure.md` line 182 (File Structure Requirements table)
- **Description:** The File Structure Requirements table row for `integration_test.rs` reads "VP-068 integration test (8 test functions)". The pass-9 fix burst updated AC6 and Task 5 to 9 test functions, but the File Structure Requirements table (a third location in the same file) retains the old count. This is a within-document propagation gap.
- **Evidence:** Story line 182 Purpose column: `VP-068 integration test (8 test functions)`. Story AC6 and Task 5 (corrected in pass-9): both say 9.
- **Proposed Fix:** Update line 182 Purpose column to `VP-068 integration test (9 test functions)`.
- **Root cause:** Pass-9 fix burst grep scope covered AC6 and Task 5 but not the File Structure Requirements table.
- **Blocking:** Yes — File Structure Requirements is read by implementer as the file manifest; count mismatch creates confusion.

#### MED-P10-002: VP-068 line 485 Feasibility Assessment math implies 10 total (9 + 1 platform-variant)

- **Severity:** MED
- **Category:** spec-consistency
- **Location:** `.factory/specs/verification-properties/VP-068.md` line 485 (Feasibility Assessment table, Input space size row)
- **Description:** The Feasibility Assessment states "Finite — 3 BCs × 2-3 scenarios each = ~9 discrete test cases + 1 platform-variant sync check". The additive phrasing "9 + 1" implies 10 total test functions, but the Proof Harness Skeleton explicitly enumerates 9 functions — the platform-variant sync test is the 9th function, not an additional one beyond 9. This is the inverse of MED-P09-002: MED-P09-002 fixed AC6/Task 5 to say 9; VP-068 feasibility math was not simultaneously corrected, leaving it inconsistent.
- **Evidence:** VP-068 Proof Harness Skeleton: 9 functions total (test_bc_4_08_001_emits_tool_error_event, test_bc_4_08_001_tool_name_unknown_fallback, test_bc_4_08_001_error_message_truncated_at_1000_chars, test_bc_4_08_001_exactly_1000_chars_no_truncation, test_bc_4_08_001_ten_field_wire_payload, test_bc_4_08_001_reserved_fields_not_set_by_plugin, test_bc_4_08_002_hooks_json_template_entry, test_bc_4_08_003_hooks_registry_toml_entry, test_bc_4_08_002b_platform_variants_in_sync). Line 485 phrasing implies 10.
- **Proposed Fix:** Update line 485 to "Finite — 9 test scenarios (8 BC-direct + 1 platform-variant sync check; platform-variant test is the 9th function in the harness, not additional)".
- **Root cause:** Pass-1 HIGH-P01-006 added the platform-variant test as "1 additional" in the feasibility math, but the Proof Harness Skeleton counts it as the 9th of 9. The math was never reconciled when AC6/Task 5 were fixed to 9 in pass-9.
- **Blocking:** Yes — fresh-context read of VP-068 feasibility math contradicts the Proof Harness Skeleton enumeration.

### LOW

None.

### OBS (Observation — below LOW threshold)

#### OBS-P10-001: Count-propagation discipline (S-7.02) missing explicit "grep for old count value" step

- **Severity:** OBS
- **Category:** process-gap
- **Location:** Factory process — adversarial fix burst protocol
- **Description:** All three MED/HIGH findings in this pass are pass-9 propagation gaps that a grep for the old count value "8" across the artifact corpus would have caught before sealing the pass-9 fix burst. S-7.02 (Defensive Sweep Discipline) requires sweeping for old count values before declaring a count-changing update complete, but the current fix-burst workflow does not enforce this for test-function counts (only for BC/VP/story numeric counts). The pattern has now recurred across passes 7, 9, and 10.
- **Proposed action:** Codify "grep for old test-function count string across all story locations (AC, Task, File Structure Requirements, VP Feasibility)" as a mandatory step in the fix-burst checklist whenever a test-count changes. Candidate for lessons.md.
- **Blocking:** No.

## Summary

| Severity | Count |
|----------|-------|
| CRITICAL | 0 |
| HIGH | 1 (HIGH-P10-001 — blocking) |
| MEDIUM | 2 (MED-P10-001, MED-P10-002 — both blocking) |
| LOW | 0 |
| OBS | 1 (OBS-P10-001 — process-gap, non-blocking) |

**Overall Assessment:** CLOCK_RESET — 1 HIGH + 2 MED findings, all pass-9 count-propagation gaps.
**Convergence:** Clock reset to 0_of_3. Pass-11 expectation: CLEAN_PASS_1_OF_3 after fix burst closes HIGH-P10-001 + MED-P10-001 + MED-P10-002.
**Readiness:** Fix burst required before pass-11 dispatch.

## Fix Verification Pre-check (for pass-11 adversary)

| Finding | Expected Evidence |
|---------|-----------------|
| HIGH-P10-001 | STORY-INDEX line 106 S-5.04 version column reads `2.4 (pass-1/2/4/9 fix bursts; ...)` or higher |
| MED-P10-001 | Story File Structure Requirements table integration_test.rs row reads "(9 test functions)" |
| MED-P10-002 | VP-068 line 485 Input space size reads "9 test scenarios (8 BC-direct + 1 platform-variant sync check; ...)" with no additive "+" phrasing |

## Novelty Assessment

| Field | Value |
|-------|-------|
| New patterns | OBS-P10-001 count-value grep step missing from fix-burst checklist |
| Recurrences | HIGH-P10-001 is recurrence of MED-P07-001 (STORY-INDEX version column); MED-P10-001/002 are recurrences of MED-P09-002 propagation gap pattern |
| Converging | Yes — all findings are mechanical propagation gaps, not spec logic errors |
