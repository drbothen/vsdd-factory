---
document_type: adversarial-review
level: ops
version: "1.0"
status: complete
producer: vsdd-factory:adversary
timestamp: 2026-08-20T00:00:00Z
phase: pre-TDD
inputs:
  - .factory/stories/S-21.19-executor-decision-function-core.md
  - .factory/specs/behavioral-contracts/ss-01/BC-1.03.017.md
  - .factory/planning/S-21.11-decomposition-plan.md
input-hash: "2ac0785"
traces_to: S-21.19-executor-decision-function-core.md
pass: 1
cascade: S-21.19-local
previous_review: null
---

# Adversarial Review — S-21.19 (LOCAL pre-TDD cascade, pass 1) — NOT-CLEAN

Artifacts reviewed: story `S-21.19-executor-decision-function-core.md` v1.0 (input-hash `a2dca8e`);
`BC-1.03.017.md` v1.18; `.factory/planning/S-21.11-decomposition-plan.md` (D-1057 6-seam split of
CONVERGED S-21.11 v2.11). Rubric: full `.factory/policies.yaml` (POLICY 1-22). This is the first
LOCAL pre-TDD adversarial pass against S-21.19 as an independently-mergeable split seam (the
seam did not exist as a standalone reviewable artifact before D-1057).

## Verdict: NOT-CLEAN
1 BLOCKER finding (streak-resetting). LOCAL streak 0/3 (this pass establishes the cascade).

## Finding ID Convention

Finding IDs for the S-21.19 LOCAL cascade use the format `F-S2119-P<PASS>-<SEQ>` (e.g.
`F-S2119-P1-001`), matching the sibling per-story LOCAL cascade convention already established
for S-21.09 and S-21.11 v2 (`F-S2111V2-P<PASS>-<SEQ>` via `adv-s21.11-v2-local-pass-N.md`).

## Part A — Fix Verification (pass >= 2 only)

N/A — this is pass 1 of the S-21.19 LOCAL cascade; there is no prior pass to verify fixes against.

## Part B — New Findings (or all findings for pass 1)

### BLOCKER

#### F-S2119-P1-001: Split severs the enforcement-flip↔annotation atomic unit (fail-open window, waves 6-8)
- **Severity:** BLOCKER
- **Category:** contradictions / spec-fidelity (cross-story topology defect)
- **Location:** S-21.19 v1.0 Task 5 ("ATOMICITY GATE" note) vs. BC-1.03.017 v1.18 Invariant 7 / PC8 / PC11
- **Description:** The 6-seam split of CONVERGED S-21.11 v2.11 (per D-1057 /
  `.factory/planning/S-21.11-decomposition-plan.md` §1/§3) severs an atomicity-critical unit. As
  authored (v1.0), S-21.19's Task 5 wires the extended 3-arg `plugin_fail_closed` function
  directly into `execute_tiers`/`execute_tier`'s real block-decision call site — i.e. S-21.19
  makes enforcement-active — while the annotation of the five `on_error=block` plugins that must
  be marked before enforcement goes live is owned by downstream seams S-21.21/S-21.23/S-21.24
  (waves 7-8). Because S-21.19 lands at wave 6, strictly before those annotation waves, a
  merged-to-`develop` state exists in which enforcement is active AND all five targeted plugins
  are still unannotated — a live CWE-636 fail-open window spanning waves 6 through 8 on every
  intervening `develop` commit. This directly contradicts BC-1.03.017 v1.18 Invariant 7 and
  PC8/PC11, which together establish a mechanical (not merely process) un-mergeability guarantee
  between "enforcement flip" and "all five plugins annotated."
- **Evidence:** S-21.19 v1.0's own Task 5 "ATOMICITY GATE (BC-1.03.017 v1.18 Invariant 7 + AC-012
  PC11 — fail-open-window hazard)" note claimed this hazard was "vacuously satisfied" by the
  split. That claim is independently FALSE: vacuous satisfaction would require the wiring to
  never exist without the annotations already present, but the split as authored puts the wiring
  in the EARLIER story (S-21.19, wave 6) and the annotations in the LATER stories (waves 7-8) —
  the exact inversion that trips the invariant, not a satisfaction of it.
- **Proposed Fix:** This is a BLOCKER, not a HIGH: it is not a local defect in S-21.19's own logic
  but a cross-story topology defect that makes the split, as approved, structurally incapable of
  preserving an invariant the pre-split unified S-21.11 v2.11 held by construction (3-CLEAN
  BC-5.39.001 convergence). TDD dispatch on S-21.19 v1.0 MUST NOT proceed until this is resolved
  at the architecture layer — a story-local fix cannot close a cross-story sequencing defect.
  Route to `vsdd-factory:architect` for a topology-level ruling on which story should own the
  enforcement-flip commit.
- **Status:** **RESOLVED this burst (D-1058)** — see Disposition below.

## Disposition

Routed to `vsdd-factory:architect` per the Agent Routing Table (cross-story architecture defect,
not a story-writer-fixable local gap). Architect adjudicated via **ADR-044** (Split-topology
flip-sequencing — capstone-owned enforcement-active wiring; extends ADR-039 §Decision 3;
RATIFIED 2026-08-20): the enforcement-active wiring commit moves to S-21.24 (the capstone, which
`depends_on` all five sibling seams by construction and therefore cannot land before every
plugin is annotated); S-21.19 retains only the dormant, standalone, unit-tested 3-arg extension
and `PluginOutcome.failure_policy` field — never wired into the real block-decision chain within
S-21.19 itself. `vsdd-factory:story-writer` then applied ADR-044's per-story change-list
(`.factory/planning/S-21.11-decomposition-plan.md` §8.4) same-burst: S-21.19 v1.0→v1.1 (points
9→7, input-hash a2dca8e→e6f82f2, Task 5 sheds the live-wiring clause and the false "vacuously
satisfied" note); S-21.24 v1.0→v1.1 (points 3→5, input-hash cbbc8dd→e3c75a4, new Task 0 performs
the wiring strictly after the five annotation tasks). AC-002 and AC-011 are each SPLIT: their
unit legs remain on S-21.19, their integration legs (bats fixture-registry dispatch; TC-12
`full_stack_plugin_invocation.rs` mirror) move to S-21.24 with the wiring they depend on. Zero
`depends_on`/`blocks`/wave-assignment change (topology-preserving per plan §8.3); AC partition
remains 43/43 (leg-ownership move, not an AC add or drop). This makes "annotate before flip" hold
by wave-schedule construction alone — no same-commit choreography is required, and no commit in
`develop`'s merged history can ever observe "enforcement-active AND any of the five plugins
unannotated."

## Summary

| Severity | Count |
|----------|-------|
| BLOCKER  | 1     |
| HIGH     | 0     |
| MEDIUM   | 0     |
| LOW      | 0     |

**Overall Assessment:** block (pre-remediation, v1.0) — RESOLVED same burst via ADR-044 + story-writer v1.1 remediation (D-1058).
**Convergence:** findings remain — iterate. LOCAL streak 0/3 (resolving a BLOCKER does not itself advance the streak per BC-5.39.001; pass 2 required against the v1.1 bundle to confirm CLEAN).
**Readiness:** requires re-review (pass 2, against S-21.19 v1.1 + S-21.24 v1.1 + ADR-044 v1.0) before TDD dispatch.

## Novelty Assessment

| Field | Value |
|-------|-------|
| **Pass** | 1 |
| **New findings** | 1 |
| **Duplicate/variant findings** | 0 |
| **Novelty score** | 1.0 (1/1) |
| **Median severity** | 5.0 (BLOCKER — single-finding pass) |
| **Trajectory** | 1 |
| **Verdict** | FINDINGS_REMAIN — resolved this burst (D-1058); pass 2 required to confirm CLEAN against the remediated bundle. |
