---
document_type: adversarial-review
level: ops
version: "1.0"
status: complete
producer: vsdd-factory:adversary
timestamp: 2026-08-20T00:00:00Z
phase: pre-TDD
inputs:
  - .factory/cycles/v1.0-brownfield-backfill/adv-s21.19-local-pass-1.md
  - .factory/stories/S-21.19-executor-decision-function-core.md
  - .factory/specs/behavioral-contracts/ss-01/BC-1.03.017.md
  - .factory/specs/architecture/decisions/ADR-044-split-topology-enforcement-flip-capstone-ownership.md
input-hash: "ea30c8d"
traces_to: S-21.19-executor-decision-function-core.md
pass: 2
cascade: S-21.19-local
previous_review: .factory/cycles/v1.0-brownfield-backfill/adv-s21.19-local-pass-1.md
---

# Adversarial Review — S-21.19 (LOCAL pre-TDD cascade, pass 2) — NOT-CLEAN

Artifacts reviewed: story `S-21.19-executor-decision-function-core.md` v1.1 (input-hash `e6f82f2`);
`BC-1.03.017.md` v1.18 (pre-remediation state at review time); `S-21.24-capstone-gated-flip-completion-regression.md`
v1.1; `ADR-044-split-topology-enforcement-flip-capstone-ownership.md` (RATIFIED 2026-08-20). Rubric:
full `.factory/policies.yaml` (POLICY 1-22). This is pass 2 of the S-21.19 LOCAL pre-TDD cascade,
reviewing the ADR-044 capstone-owned-flip remediation (D-1058) applied against pass 1's BLOCKER
finding F-S2119-P1-001.

## Verdict: NOT-CLEAN

2 MEDIUM findings. LOCAL streak 0/3 (a NOT-CLEAN pass resets the streak per BC-5.39.001; pass 1's
BLOCKER resolution does not itself count as a clean pass).

## Part A — Fix Verification (pass 1 BLOCKER)

F-S2119-P1-001 (split severs enforcement-flip↔annotation atomicity): **VERIFIED FIXED**. ADR-044's
capstone-owned-flip topology (S-21.19 authors the dormant 3-arg extension only; S-21.24 Task 0
performs the wiring strictly after all five plugin-annotation tasks) is correctly reflected in both
story files as of v1.1 — S-21.19 v1.1's Task 5 no longer carries a live-wiring clause, and S-21.24
v1.1's new Task 0 sequences the flip after the annotation tasks. No re-introduction of the
fail-open window found. This pass's findings are new, surfaced by reviewing the v1.1 bundle plus
its cited BC and ADR content, not a re-opening of F-S2119-P1-001.

## Part B — New Findings

### MEDIUM

#### F-S2119-P2-001: BC-1.03.017 Invariant 7 literally contradicts ADR-044's own compliant state
- **Severity:** MEDIUM
- **Category:** contradictions / spec-fidelity
- **Location:** BC-1.03.017 v1.18 Invariant 7 vs. ADR-044 (capstone-owned flip) vs. S-21.19 v1.1's
  own now-compliant merge state
- **Description:** ADR-044 declares S-21.19's post-remediation merge state SAFE: the extended
  3-arg `plugin_fail_closed` function and `PluginOutcome.failure_policy` field exist in the
  merged commit, but are not wired into any real block-decision call site, and the five targeted
  plugins remain fail-open until S-21.24's Task 0 flip. BC-1.03.017 v1.18 Invariant 7, however,
  still reads literally as "Any CI-passing commit that contains the extended function while any of
  these five plugins remains at `failure_policy = fail-open` ... is a CWE-636 regression" — read
  literally, S-21.19's OWN compliant merge trips this sentence, directly contradicting ADR-044.
  Invariant 7 also conflates "decision-function change (authoring — S-21.19, inert)" with
  "executor flip (wiring — S-21.24 Task 0)," the exact distinction ADR-044 exists to draw. PC11
  was already updated (v1.3-v1.5, prior burst) to the correct wiring-keyed form, so this is an
  un-swept sibling site, not a fresh architectural gap.
- **Evidence:** BC-1.03.017 v1.18 Invariant 7 prose (pre-remediation): "Any CI-passing commit that
  contains the extended function while any of these five plugins remains at `failure_policy =
  fail-open` ... is a CWE-636 regression" and "The decision-function change and the annotations
  MUST be co-committed ... or ordered annotate-before-flip" — both clauses key on "contains"/
  "decision-function change," not on wiring/enforcement-active status, contradicting PC11's
  already-corrected wiring-keyed trigger.
- **Proposed Fix:** Rewrite Invariant 7 to key the CWE-636 regression trigger on the function being
  WIRED INTO / IN EFFECT in the block-decision chain (enforcement-active per PC11's signal),
  explicitly disambiguating authoring (inert, S-21.19, NOT prohibited) from wiring (enforcement-
  active flip, S-21.24 Task 0, IS what must be atomic-with/after the five annotations). Add
  ADR-044 to BC-1.03.017's `inputs:` and Traceability ADR row.
- **Status:** RESOLVED this burst (D-1060) — product-owner rewrote Invariant 7 (BC-1.03.017 v1.19)
  keyed on the wiring/enforcement-active signal, matching PC11; ADR-044 added to inputs and
  Traceability. See Disposition.

#### F-S2119-P2-002: AC-009 red-first requirement conflicts with green-trunk CI given the deferred flip
- **Severity:** MEDIUM
- **Category:** test-design / CI-topology conflict
- **Location:** S-21.19 v1.1 AC-009 (enforcement-gate assertion)
- **Description:** AC-009 asserts the enforcement-active behavior that, under ADR-044's
  capstone-owned-flip topology, is not wired until S-21.24 Task 0 (wave 8). Standard TDD discipline
  requires AC-009's test to be authored red-first (failing before the code under test exists) and
  land GREEN on `develop` at merge. But under the deferred-flip topology, the enforcement-active
  behavior AC-009 asserts genuinely does not exist yet at S-21.19's own merge point (wave 6) —
  making it structurally impossible for AC-009's test to be both red-first-authored AND
  green-on-`develop`-at-S-21.19's-own-merge without either (a) prematurely wiring the flip inside
  S-21.19 (reintroducing the F-S2119-P1-001 fail-open window ADR-044 was written to close), or (b)
  landing a genuinely-failing (red) test on `develop`, which the project's green-trunk CI
  discipline forbids.
- **Evidence:** S-21.19 v1.1 AC-009 as authored has no mechanism to distinguish "not-yet-wired,
  correctly dormant" from "wired and enforcement passing" — a naive implementation would either
  fail on `develop` (red-trunk violation) or require the very live-wiring the split was
  restructured to avoid.
- **Proposed Fix:** Mark AC-009's enforcement-behavior test
  `#[ignore = "enforcement gate; enabled at S-21.24 Task 0 flip"]` so it compiles and is skipped
  (not red, not falsely green) on `develop` between S-21.19's merge and S-21.24's flip; add a
  compile-safe fs-source-scan cross-assertion that verifies the dormant extension/field exist and
  are NOT yet referenced at the real block-decision call site, giving AC-009 real assertion content
  at S-21.19's own merge point without requiring the flip. S-21.24 Task 5 removes the `#[ignore]`
  gate once its own Task 0 performs the wiring.
- **Status:** RESOLVED this burst (D-1060) — story-writer added the `#[ignore]` attribute +
  compile-safe cross-assertion to S-21.19 v1.2 AC-009, and added the corresponding un-ignore step
  to S-21.24 v1.2 Task 5. See Disposition.

## Disposition

Both findings routed per the Agent Routing Table: F-S2119-P2-001 is BC content (product-owner's
domain), F-S2119-P2-002 is story/test-design content (story-writer's domain). Product-owner
rewrote BC-1.03.017 Invariant 7 (v1.18→v1.19), re-keying the CWE-636 trigger on wiring/enforcement-
active status rather than mere presence/"contains," and added ADR-044 to `inputs:` and the
Traceability ADR row — Invariant 7 is now internally consistent with PC11's already-corrected form
and with ADR-044's authoring-vs-wiring split. Story-writer applied the AC-009 `#[ignore]` gate +
compile-safe cross-assertion to S-21.19 (v1.1→v1.2) and the matching un-ignore step to S-21.24
(v1.1→v1.2), resolving the red-first/green-trunk conflict without reintroducing live wiring inside
S-21.19. Both fixes same-burst (D-1060).

## Summary

| Severity | Count |
|----------|-------|
| BLOCKER  | 0     |
| HIGH     | 0     |
| MEDIUM   | 2     |
| LOW      | 0     |

**Overall Assessment:** block (pre-remediation, v1.1 bundle) — RESOLVED same burst via product-owner
BC-1.03.017 v1.19 + story-writer S-21.19 v1.2 / S-21.24 v1.2 remediation (D-1060).
**Convergence:** findings remain — iterate. LOCAL streak 0/3 (resolving MEDIUM findings does not
itself advance the streak per BC-5.39.001; pass 3 required against the v1.2 bundle to confirm
CLEAN).
**Readiness:** requires re-review (pass 3, against S-21.19 v1.2 + S-21.24 v1.2 + BC-1.03.017 v1.19)
before TDD dispatch.

## Novelty Assessment

| Field | Value |
|-------|-------|
| **Pass** | 2 |
| **New findings** | 2 |
| **Duplicate/variant findings** | 0 |
| **Novelty score** | 1.0 (2/2) |
| **Median severity** | 2.0 (MEDIUM) |
| **Trajectory** | 1 → 2 |
| **Verdict** | FINDINGS_REMAIN — resolved this burst (D-1060); pass 3 required to confirm CLEAN against the remediated bundle. |
