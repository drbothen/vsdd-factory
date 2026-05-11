---
document_type: adversarial-review
level: ops
version: "1.0"
status: complete
producer: adversary
timestamp: 2026-05-11T00:00:00
phase: F5
inputs:
  - .factory/specs/behavioral-contracts/BC-INDEX.md
  - .factory/specs/behavioral-contracts/ss-07/BC-7.03.091.md
  - .factory/specs/behavioral-contracts/ss-07/BC-7.03.092.md
  - .factory/stories/epics/E-14-engine-discipline-pass-2.md
  - .factory/stories/STORY-INDEX.md
  - .factory/cycles/v1.0-feature-engine-discipline-pass-1/decision-log.md
  - .github/workflows/ci.yml
  - plugins/vsdd-factory/tests/resolver-integration.bats
input-hash: "[pending-recompute]"
traces_to: prd.md
project: vsdd-factory
mode: feature
current_step: F5-adversarial-pass-7
current_cycle: v1.0-feature-engine-discipline-pass-1
pass: 7
previous_review: adv-cycle-pass-6.md
prior-pass-classification: CRITICAL
prior-findings-count: 7
verdict: LOW
findings_count: { critical: 0, high: 0, medium: 2, low: 3, nitpick: 0 }
observations: 5
deferred: 0
process_gap_count: 2
convergence_reached: false
---

# Adversarial Review — F5 Pass-7 (Cycle-Level, Fresh Context)

Fresh-context audit of the F5 fix burst applied after pass-6 CRITICAL verdict. All 7 F-P6 findings were reported as remediated or closed. This pass verifies those closure claims and conducts a clean sweep across all affected files.

## Finding ID Convention

Finding IDs in this cycle use the shorthand format `F-P7-NNN` (cycle-scoped pass-7 findings). Formal ADV IDs follow: `ADV-<CYCLE>-P<PASS>-<SEV>-<SEQ>` per the factory standard. The cycle prefix is omitted per legacy convention established in passes 1–6.

## Part A — Fix Verification (Pass-6 Closure Summary)

| Pass-6 ID | Status | Evidence |
|---|---|---|
| F-P6-001 (CRIT) — ci.yml WASM staging step mis-ordered | **CLOSED** | CI run 25651192161 (D-380): staging step repositioned before perf-baseline.bats; CI green on PR branch. Third generation of this defect class; closed with mandatory CI evidence per D-379. |
| F-P6-002 (CRIT) — D-378 closure overclaimed; S-14.06/07/08 absent | **CLOSED** | S-14.06, S-14.07, S-14.08, S-14.09 authored and registered in STORY-INDEX. D-378 amended with concrete story IDs. |
| F-P6-003 (HIGH) — CI-green-signal rule not codified in decision-log | **CLOSED** | D-379 authored in cycle decision-log: CRITICAL CI-class findings MUST demonstrate CI green on PR branch before closure; no self-certification. |
| F-P6-004 (HIGH) — forensic markers proliferating; no cleanup story | **CLOSED** | S-14.09 filed: "Forensic marker cleanup" with ACs to remove pass-specific F-P[N]-NNN markers and add CI lint rule. Story registered in STORY-INDEX. |
| F-P6-005 (HIGH) — BC-7.03.091:77 and BC-7.03.092:79-80 VP-TBD | **CLOSED-WITH-CAVEAT** | VP-076 extended with sub-property covering BC-7.03.091/092 scope; VP-TBD entries replaced with VP-076 in BC body files. However, the BC-INDEX Capability column for these two rows (lines 1782-1783) still reads TBD. The F-P5-002/F-P6-005 fix propagated to BC body files but did not propagate to the BC-INDEX sibling file. See F-P7-001. |
| F-P6-006 (MED) — BC-7.03.091:94 and BC-7.03.092:97 path casing `ss-07` vs `SS-07` | **CLOSED** | Both BC files updated: `architecture/ss-07-hook-bash.md` → `architecture/SS-07-hook-bash.md`. Verified path exists. |
| F-P6-007 (MED) — F-P5-008 CI-green precondition violated by pass-5 burst | **CLOSED-BY-PATTERN** | Addressed structurally by D-379 (F-P6-003 fix) and F-P6-001 fix. Pattern-level finding; no additional artifact change required. |
| F-P6-008 (obs) — timeout comment arithmetic inconsistency | **PARTIAL** | Wording shifted from "catches ≥15% reductions" to "catches ≥25%+ reductions" in resolver-integration.bats:594-595. The 8000ms assertion against a 9000ms deadline actually catches ≥13.3% reductions (8000/9000 = 88.9%, so reduction is caught at 11.1%+; or relative to nominal 9000ms, catching ≥11.1% degradations). The arithmetic remains inconsistent — now understating sensitivity rather than overstating it. See F-P7-005. |

## Part B — New Findings

### MEDIUM

#### F-P7-001 [MEDIUM]: BC-INDEX lines 1782-1783 Capability column still TBD for BC-7.03.091 and BC-7.03.092; F-P5-002/F-P6-005 fix did not propagate to BC-INDEX sibling file

- **Severity:** MEDIUM
- **Category:** spec-fidelity / sibling-file gap
- **Location:** `.factory/specs/behavioral-contracts/BC-INDEX.md:1782-1783`
- **Description:** F-P5-002 and F-P6-005 fix bursts amended BC-7.03.091 and BC-7.03.092 body files to resolve VP-TBD entries. The BC-INDEX table rows for these two BCs (lines 1782-1783) carry a Capability column value of `TBD`. The correct value is `CAP-008` (warn-pending-wave-gate is the primary capability for hook-plugins registering as pipeline gate advisors). This is a textbook S-7.01 sibling-file propagation gap: the BC body was updated but the BC-INDEX row was not. Drift detection and traceability tools that read BC-INDEX will report these two BCs as unanchored to any capability, creating a false gap in the capability coverage matrix.
- **Evidence:** BC-INDEX:1782 — `| [BC-7.03.091](ss-07/BC-7.03.091.md) | warn-pending-wave-gate: identity & registry binding | draft | TBD | S-8.07 |`. BC-INDEX:1783 — `| [BC-7.03.092](ss-07/BC-7.03.092.md) | warn-pending-wave-gate: stderr warning when any wave has gate_status: pending | draft | TBD | S-8.07 |`. Neighboring BCs in the warn-pending-wave-gate section reference `TBD` for Capability, consistent with the pattern that this sub-section was never swept during the capability propagation pass.
- **Proposed Fix:** Update BC-INDEX lines 1782-1783: replace `TBD` in the Capability column with `CAP-008`. Bump BC-INDEX version v1.63 → v1.64. Add CHANGELOG entry citing F-P5-002/F-P6-005 propagation and L-P28-001 sibling-file sweep obligation.

#### F-P7-002 [MEDIUM]: E-14 epic `story_count: 5` stale — should be 9; S-14.06/07/08/09 added by F-P6-002/F-P6-004 fix bursts but epic body and frontmatter not updated

- **Severity:** MEDIUM
- **Category:** spec-fidelity / sibling-file gap
- **Location:** `.factory/stories/epics/E-14-engine-discipline-pass-2.md:12`
- **Description:** The F-P6-002 fix burst authored S-14.06, S-14.07, S-14.08, and F-P6-004 fix burst authored S-14.09 — all under epic E-14. The STORY-INDEX was updated to register these four new stories. However, the E-14 epic file's frontmatter `story_count: 5` was not updated to 9, and the epic body "Stories Planned" table and description text still reference only S-14.01 through S-14.05. A reader loading only the epic file (the canonical planning artifact for E-14) has no visibility into S-14.06/07/08/09. This is the same sibling-file gap class as F-P7-001: STORY-INDEX updated, parent epic not.
- **Evidence:** E-14 epic frontmatter line 12: `story_count: 5`. STORY-INDEX contains entries for S-14.01 through S-14.09 under E-14. E-14 "Stories Planned" table rows: 5 (S-14.01..S-14.05 only). No S-14.06/07/08/09 entries in epic description or stories table.
- **Proposed Fix:** Update E-14 epic: (1) `story_count: 5` → `story_count: 9`, (2) bump version v1.0 → v1.1, (3) update `last_amended:` to 2026-05-11 referencing F-P6-002/F-P6-004 fix bursts, (4) add S-14.06, S-14.07, S-14.08, S-14.09 rows to the "Stories Planned" table with brief titles and metadata, (5) update description text to reference the four new stories. Also add a note clarifying that `cycle: v1.0-feature-engine-discipline-pass-2` in story frontmatter is a forward reference (see F-P7-004).

### LOW

#### F-P7-003 [LOW]: resolver-integration.bats line 461 header comment and line 471 test name still cite "3000ms" but the assertion bound is now 8000ms (commit ae4778c4 raised it)

- **Severity:** LOW
- **Category:** code-clarity
- **Location:** `plugins/vsdd-factory/tests/resolver-integration.bats:461` (header comment); `plugins/vsdd-factory/tests/resolver-integration.bats:471` (test name)
- **Description:** The timeout calibration fix (F-P5-007, updated in subsequent passes) raised the assertion bound from a lower value to 8000ms. The surrounding cosmetic surfaces — the section header comment and the `@test` name — were not updated in commit ae4778c4. A reader sees "3000ms" in the comment and test name but "8000ms" in the assertion body, creating an internal inconsistency in the test file. The assertion itself is correct; only the cosmetic strings are stale.
- **Evidence:** resolver-integration.bats:461 header comment cites "3000ms". resolver-integration.bats:471 `@test` name cites "3000ms". resolver-integration.bats assertion bound: 8000ms.
- **Proposed Fix:** Update the header comment at line 461 and the `@test` name at line 471 to cite "8000ms". Feature-branch fix — not a factory-artifacts change. Defer to fix commit on the feature/engine-discipline branch.

#### F-P7-004 [LOW]: All 9 E-14 stories carry `cycle: v1.0-feature-engine-discipline-pass-2` — cycle directory does not exist; forward reference without documentation

- **Severity:** LOW
- **Category:** spec-clarity / forward-reference
- **Location:** `.factory/stories/S-14.01` through `S-14.09` frontmatter `cycle:` field
- **Description:** All nine E-14 stories reference `cycle: v1.0-feature-engine-discipline-pass-2` in their frontmatter. No directory named `v1.0-feature-engine-discipline-pass-2` exists under `.factory/cycles/`. This is a forward reference to a cycle that will be opened when E-14 work begins. The reference is intentional planning-tier convention — stories are assigned to their delivery cycle at authoring time, before the cycle directory is opened. However, nothing in the epic file or the story files documents this convention, creating a potential confusion point for agents and humans reading the frontmatter who may interpret a missing cycle directory as a broken reference or a data entry error.
- **Evidence:** S-14.01..S-14.09 frontmatter: `cycle: v1.0-feature-engine-discipline-pass-2`. `.factory/cycles/` directory listing: no `v1.0-feature-engine-discipline-pass-2` entry.
- **Proposed Fix:** Add a note in the E-14 epic body explaining: all E-14 stories' `cycle:` frontmatter is a forward reference to a planned-but-unopened cycle (`v1.0-feature-engine-discipline-pass-2`). The cycle directory will be opened by the state-manager when E-14 work begins; this is accepted planning-tier convention. No behavioral defect — documentation only. Factory-artifacts fix (epic body update).

#### F-P7-005 [LOW]: resolver-integration.bats:594-595 timeout rationale understates sensitivity — "25%+" claimed; test actually catches ≥11.1%+ reductions from nominal

- **Severity:** LOW
- **Category:** code-clarity / arithmetic inconsistency
- **Location:** `plugins/vsdd-factory/tests/resolver-integration.bats:594-595`
- **Description:** F-P6-008 identified that the timeout comment overclaimed "catches ≥15% reductions" when the arithmetic showed 13.3%. The fix updated the comment to "25%+". The bound is 8000ms against a nominal deadline of 9000ms. A regression is caught when actual duration exceeds 8000ms. At 8000ms, the reduction from 9000ms nominal is 1000ms = 11.1% degradation. The claim "25%+" is now an understatement (more conservative claim than actual sensitivity), which is safer than overclaiming but still arithmetically wrong. The comment misrepresents what the assertion measures to anyone using it as documentation.
- **Evidence:** resolver-integration.bats:594-595 — comment states "catches ≥25%+ reductions." Arithmetic: 8000/9000 = 88.9% pass threshold; regression caught when actual time degrades by ≥11.1% from nominal. "25%+" is incorrect.
- **Proposed Fix:** Update the comment to state "catches regressions where load time exceeds 8000ms (≥11% degradation from nominal 9000ms)" or equivalent accurate statement. Feature-branch fix — not a factory-artifacts change. Defer to fix commit on the feature/engine-discipline branch.

## Observations

- **F-P7-006 [POSITIVE]:** D-379 + D-380 broke the 5-pass false-green CI pattern. The mandatory CI-green-signal rule (D-379) was enforced for the first time in this cycle's fix history: D-380 cites CI run 25651192161 as the closure evidence for F-P6-001. This is the correct operational behavior — a CRITICAL CI-class finding was verified with actual CI evidence, not self-certification. The rule is now operationally active, not just textually present.

- **F-P7-007 [process-gap]:** D-379 codifies the CI-green-signal rule for CRITICAL CI-class findings but does not define what constitutes a "CI-class finding." The current text relies on exemplars (coverage, staging, CI step invocation, bats runner configuration). An ambiguous finding at the CRITICAL/HIGH boundary could be misclassified as non-CI-class and escape the mandatory CI evidence requirement. Recommend adding a definitional sentence: "A finding is CI-class if a failure in the referenced artifact would cause CI to fail or produce a false-green signal." Low risk now; enforcement ambiguity could surface in future cycles with different defect patterns.

- **F-P7-008 [process-gap]:** The sibling-file propagation gap that produced F-P7-001 and F-P7-002 follows the same pattern as F-P6-005 (BC body updated, BC-INDEX not) and F-P6-002 (STORY-INDEX updated, epic not). The F-P6 fix burst introduced the S-14.06..S-14.09 stories and registered them in STORY-INDEX but did not sweep the E-14 epic file. This is the second consecutive pass where a fix burst propagated to an index but not to the parent artifact. The S-7.02 defensive sweep discipline requires sweeping sibling files before declaring a count-or-reference change complete. Recommend adding "parent epic file" to the S-7.02 sweep checklist for any story additions.

- **F-P7-009 [POSITIVE]:** No new F-P6-NNN or F-P7-NNN forensic markers were added to production source files or test files in the fix burst. The D-379/D-380 fix burst and the story-authoring burst stayed within spec/planning artifacts and CI config. The S-14.09 story file correctly defers forensic-marker cleanup to a story rather than attempting inline cleanup in the same burst. The discipline from pass-6's F-P6-004 finding is holding for this burst.

- **F-P7-010 [nitpick]:** All 9 E-14 stories carry `input-hash: "TBD"` or `input-hash: "[pending-recompute]"` in their frontmatter. This is expected for draft stories that have not yet passed through the S-14.03 pre-F5 lint gate (which does not yet exist — S-14.03 is itself a story in E-14). The placeholder hashes will be caught and enforced by S-14.03 when implemented. No action required before that gate.

## Summary

| Severity | Count |
|----------|-------|
| CRITICAL | 0 |
| HIGH | 0 |
| MEDIUM | 2 |
| LOW | 3 |
| Process-gap | 2 |

**Overall Assessment:** LOW — severity floor descended from CRITICAL (6 consecutive passes) to LOW
**Convergence:** findings remain — iterate
**Readiness:** requires revision (2 factory-artifacts fixes addressable in next burst)

## Novelty Assessment

| Field | Value |
|-------|-------|
| **Pass** | 7 |
| **New findings** | 5 |
| **Duplicate/variant findings** | 0 |
| **Novelty score** | 1.0 (5/5) |
| **Median severity** | LOW |
| **Trajectory** | 29→11→9→9→8→7→**5** |
| **Verdict** | FINDINGS_REMAIN |

## Convergence Trajectory

| Pass | Classification | Critical | High | Medium | Low | Total |
|------|---------------|----------|------|--------|-----|-------|
| P1 | CRITICAL | 2 | 10 | 12 | 5 | 29 |
| P2 | CRITICAL | 2 | 4 | 5 | 0 | 11 |
| P3 | CRITICAL | 2 | 4 | 3 | 0 | 9 |
| P4 | CRITICAL | 2 | 4 | 3 | 0 | 9 |
| P5 | CRITICAL | 1 | 3 | 3 | 1 | 8 |
| P6 | CRITICAL | 2 | 3 | 2 | 0 | 7 |
| P7 | LOW | 0 | 0 | 2 | 3 | **5** |

Severity floor descended from CRITICAL (6 consecutive passes) to LOW in P7. First pass with zero CRITICAL or HIGH findings. The D-379 CI-green-signal enforcement (D-380 evidence for F-P6-001) broke the self-certification pattern that was reproducing same-class CI defects across generations. Remaining findings (F-P7-001/002) are factory-artifacts sibling-file gaps addressable in one burst; F-P7-003/005 are feature-branch cosmetic fixes.

## Top 5 Most Important Findings (F5 pass-7 fix burst drivers)

1. **MEDIUM F-P7-001** — BC-INDEX lines 1782-1783 Capability column TBD → CAP-008; factory-artifacts fix (BC-INDEX sibling sweep)
2. **MEDIUM F-P7-002** — E-14 epic story_count: 5 → 9; add S-14.06..S-14.09 to Stories Planned table; factory-artifacts fix (epic body update)
3. **LOW F-P7-004** — Add forward-reference convention note to E-14 epic for `cycle: v1.0-feature-engine-discipline-pass-2`; factory-artifacts fix (documentation)
4. **LOW F-P7-003** — resolver-integration.bats comment + test name still cite "3000ms"; feature-branch cosmetic fix
5. **LOW F-P7-005** — resolver-integration.bats:594-595 timeout rationale "25%+" → accurate arithmetic statement; feature-branch cosmetic fix

## Recommendation

Continue F5. Pass-8 target: NITPICK_ONLY (streak 1 of 3). Factory-artifacts fixes (F-P7-001, F-P7-002, F-P7-004) are addressable in a single burst. Feature-branch fixes (F-P7-003, F-P7-005) should be applied in the same commit window.

Two process observations (F-P7-007: D-379 definitional gap; F-P7-008: sibling-file checklist gap) are low-risk; address in next process-codification burst or defer to S-14 stories if appropriate.

`convergence_reached`: false. Verdict LOW. Need 3 consecutive NITPICK_ONLY cycle-level passes for cycle convergence. Pass-8 is the first opportunity to start the streak.

## Process-Gap Findings (2)

F-P7-007 (D-379 CI-class definition ambiguity — enforceability gap for boundary cases), F-P7-008 (sibling-file checklist omits parent epic file for story additions — caused F-P7-001 and F-P7-002; second consecutive pass with this gap class).
