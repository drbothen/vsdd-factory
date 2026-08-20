---
document_type: adversarial-review
level: ops
version: "1.0"
status: complete
producer: vsdd-factory:adversary
timestamp: 2026-08-20T00:00:00Z
phase: 3
inputs:
  - .factory/stories/S-21.11-validator-exhaustion-fail-closed-calibration-and-enforcement.md
  - .factory/specs/behavioral-contracts/ss-01/BC-1.03.017.md
  - .factory/specs/behavioral-contracts/ss-01/BC-1.03.018.md
  - .factory/specs/architecture/decisions/ADR-039-validator-failure-policy-resource-exhaustion-fail-closed.md
input-hash: "da73a95"
traces_to: S-21.11-validator-exhaustion-fail-closed-calibration-and-enforcement.md
pass: 13
cascade: S-21.11-v2
previous_review: adv-s21.11-v2-local-pass-12.md
---

# Adversarial Review — S-21.11 v2 (LOCAL cascade, pass 13)

> **Persistence note (state-manager, this burst):** content below is transcribed verbatim from the
> orchestrator-relayed adversary dispatch per POLICY 22 (orchestrator-transcribed; the live
> adversary session ran in a separate context this burst resumes from), following the
> `adv-s21.11-v2-local-pass-N.md` convention established at pass-3.

Artifacts reviewed: story `S-21.11-validator-exhaustion-fail-closed-calibration-and-enforcement.md`
v2.10 (input-hash `97029a5`); `BC-1.03.017.md` v1.18; `BC-1.03.018.md` v1.1; `ADR-039` v1.13
(§Erratum E-005, `status: ratified`); factory-artifacts bundle HEAD `d5843f49` (D-1052 commit) — the
SAME bundle reviewed at pass-12, unchanged per the BC-5.39.001 3-CLEAN protocol. Rubric: full
`.factory/policies.yaml` (POLICY 1-22).

## Verdict: NOT-CLEAN

1 HIGH streak-resetting finding (F-S2111V2-P13-001), plus independent re-confirmation of pass-12's
3 non-resetting LOW/ADVISORY cosmetic observations (F-S2111V2-P12-001/002/003, still present and
unremediated — pass-12 deliberately deferred them). BC-5.39.001 streak **RESETS 1/3 → 0/3**.

## Finding ID Convention

Finding IDs for the S-21.11 v2 cascade use the format `F-S2111V2-P<PASS>-<SEQ>`, established at
pass-1 (F-S2111V2-P1-001) and continued through pass-12 (F-S2111V2-P12-001..003). This pass's 1
HIGH streak-resetting finding uses `F-S2111V2-P13-001`.

## Part A — Fix Verification (pass >= 2 only)

No BLOCKER/HIGH/MEDIUM findings were open from pass-12 to verify — pass-12 returned CLEAN. The 3
LOW/ADVISORY cosmetic observations pass-12 recorded (F-S2111V2-P12-001/002/003) were explicitly
DOCUMENTED-not-remediated at pass-12 (deferred to convergence-close per that pass's own
disposition); this pass independently re-confirms all 3 are still present, unchanged, and correctly
classified (F-S2111V2-P12-001 LOW; F-S2111V2-P12-002/003 ADVISORY) — no escalation in severity
found for any of the three.

## Part B — New Findings

### F-S2111V2-P13-001 (HIGH, streak-resetting)

**Location:** Story Task #19 (ATOMICITY GATE note) and Task #29 (test authoring), governing
AC-012.

**Finding:** AC-012's migration-window gate (`test_no_on_error_block_without_fail_closed_when_3arg_executor`,
BC-1.03.017 v1.18 PC11) is authored at Task #29 — Phase 4c, Node (E) — twelve tasks AFTER Task
#19's Node (D) executor decision-function extension and the Phase-4c fail-closed-flip Tasks
#26–#28 that this gate exists to catch. Task #19's own ATOMICITY GATE note asserts: "Committing
this task alone WILL cause AC-012's test to FAIL, blocking CI merge" — but at Task #19's execution
time, `test_no_on_error_block_without_fail_closed_when_3arg_executor` does not exist yet (it is not
authored until Task #29). A test that has not been written cannot fail and cannot block CI merge;
the note's own claim is FALSE at the moment it is asserted. Consequently the CWE-636 migration
window Task #19 opens (spanning Task #19 through the Tasks #27/#28 flip commits) is mechanically
ungated for the twelve intervening tasks — exactly the class of hazard AC-012 exists to prevent.

This is the un-swept structural sibling of pass-11's F-S2111V2-P11-002 (AC-009's red-first gate
was mis-pointed and authored AFTER the code changes it is meant to catch RED). The v2.10/pass-11
remediation burst fixed AC-009 alone via new Task #20a; it did not extend the same fix to AC-012,
which shares the identical authored-after-referenced structural defect. An exhaustive
authoring-task/referencing-task sweep across all 41 ACs was not performed at that burst — only the
three findings pass-11 explicitly named were fixed, leaving this un-swept sibling to surface here.

**Impact:** A structural TDD/atomicity-discipline violation on one of the story's two central
migration-window gates (AC-012, alongside AC-009), not a wording or cite defect — an implementer
following the story's task order as written would commit Task #19 believing (per its own
ATOMICITY GATE note) that a CI-blocking test exists to catch the open window, when no such test
exists yet. The CWE-636 regression the gate is designed to prevent is genuinely un-caught for the
duration of the window under the story's own task ordering.

## Re-confirmed non-resetting observations (carried forward from pass-12, unremediated)

- **F-S2111V2-P12-001 (LOW):** Task #22's "AC-014 through AC-021 (all nine BC-1.03.018 PC1–PC8
  behaviors …)" adjectival count is off-by-one — AC-014 through AC-021 inclusive is 8 ACs, not
  nine. The explicit enumeration itself remains correct and complete.
- **F-S2111V2-P12-002 (ADVISORY):** the story's Node (D) header "independent of B/C" phrasing
  remains slightly loose given Task #20a's soft Node-B prerequisite; task SEQUENCE itself is
  correct. Unchanged since pass-12.
- **F-S2111V2-P12-003 (ADVISORY):** the DAG Node-(D) illustrative box's omission of AC-009 (and,
  as of this pass, AC-012 pending its Task #16a relocation) from its caption remains a deliberate
  decision-function-behavior scoping choice, not a partial-propagation miss. Unchanged since
  pass-12.

## Summary

| Severity | Count |
|----------|-------|
| CRITICAL | 0 |
| HIGH | 1 |
| MEDIUM | 0 |
| LOW | 1 |
| ADVISORY | 2 |

**Overall Assessment:** not-clean, streak-resetting
**Convergence:** streak RESETS **1/3 → 0/3**. A fresh CLEAN pass-14 restarts the count from 0/3 per
BC-5.39.001.
**Readiness:** route F-S2111V2-P13-001 to story-writer for same-burst remediation (story-only
fix — no BC/ADR change required: relocate AC-012's authoring to a pre-flip task mirroring Task
#20a's insertion pattern for AC-009, and MANDATE an exhaustive AC-001..AC-041 task-ordering
sibling-sweep this time, not a single-instance patch, to confirm no third sibling remains). Fold
in F-S2111V2-P12-001 (LOW cosmetic) in the same burst since the bundle is being touched
(re-versioned) regardless this pass; F-S2111V2-P12-002/003 (ADVISORY, non-resetting, no action
required) remain deferred to convergence-close.

## Novelty Assessment

Novelty **HIGH** — F-S2111V2-P13-001 is not a new defect *class* (it is the same
authored-after-referenced/red-first-gate-ordering class F-S2111V2-P11-002 introduced at pass-11),
but its *recurrence as an un-swept sibling of a previously-fixed instance* is itself the novel and
more consequential signal: it demonstrates that the pass-11 remediation burst fixed the NAMED
instance (AC-009) without verifying no OTHER instance of the same structural class existed
elsewhere in the story's 41-AC task DAG. This is a generalization of the sibling-sweep-scope
lesson already established twice in this cascade for STRING-CITE sweeps (line-wrap at D-1047,
backtick-intervening at D-1051) — this pass extends the same lesson to TASK-ORDERING sweeps for
the first time. Recommend TD-VSDD-060 be read as covering same-class task-ordering defects, not
only literal string-cite patterns, and recommend the remediation burst perform (and record) an
exhaustive same-class sibling check as a standing requirement whenever a task-ordering/red-first
defect is fixed, not only when a version-cite defect is fixed.
