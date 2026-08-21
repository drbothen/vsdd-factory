---
document_type: adversarial-review
level: ops
version: "1.0"
status: complete
producer: vsdd-factory:adversary
timestamp: 2026-08-20T00:00:00Z
phase: pre-TDD
inputs:
  - .factory/cycles/v1.0-brownfield-backfill/adv-s21.19-local-pass-2.md
  - .factory/stories/S-21.19-executor-decision-function-core.md
  - .factory/stories/S-21.24-capstone-gated-flip-completion-regression.md
  - .factory/specs/behavioral-contracts/ss-01/BC-1.03.017.md
  - .factory/specs/architecture/decisions/ADR-044-split-topology-enforcement-flip-capstone-ownership.md
input-hash: "ba31776"
traces_to: S-21.19-executor-decision-function-core.md
pass: 3
cascade: S-21.19-local
previous_review: .factory/cycles/v1.0-brownfield-backfill/adv-s21.19-local-pass-2.md
---

# Adversarial Review — S-21.19 (LOCAL pre-TDD cascade, pass 3) — NOT-CLEAN

Artifacts reviewed: story `S-21.19-executor-decision-function-core.md` v1.2 (input-hash `e6f82f2`);
`BC-1.03.017.md` v1.19; `S-21.24-capstone-gated-flip-completion-regression.md` v1.2;
`ADR-044-split-topology-enforcement-flip-capstone-ownership.md` (RATIFIED). Rubric: full
`.factory/policies.yaml` (POLICY 1-22). This is pass 3 of the S-21.19 LOCAL pre-TDD cascade,
reviewing the D-1060 pass-2 remediation (BC-1.03.017 Invariant 7 wiring re-key + AC-009
`#[ignore]` gate) applied against pass 2's two MEDIUM findings.

## Verdict: NOT-CLEAN

1 MEDIUM finding + 1 LOW finding. LOCAL streak 0/3 (a NOT-CLEAN pass resets the streak per
BC-5.39.001; pass 2's MEDIUM resolutions do not themselves count as a clean pass).

## Part A — Fix Verification (pass 2 findings)

**F-S2119-P2-001** (BC-1.03.017 Invariant 7 wiring-key contradiction): **VERIFIED FIXED**.
BC-1.03.017 v1.19 Invariant 7 now keys the CWE-636 regression trigger on the extended function
being WIRED INTO / enforcement-active in the real block-decision chain, not on mere presence —
disambiguating authoring (S-21.19, inert) from wiring (S-21.24 Task 0). ADR-044 is present in
`inputs:` and the Traceability ADR row. No re-contradiction found.

**F-S2119-P2-002** (AC-009 red-first vs green-trunk conflict): **VERIFIED FIXED**. S-21.19 v1.2
Task 8 carries the `#[ignore = "enforcement gate; enabled at S-21.24 Task 0 flip"]` attribute on
AC-009's enforcement-behavior test, plus the compile-safe fs-source-scan cross-assertion
confirming the dormant extension/field exist and are not yet referenced at the real block-decision
call site. S-21.24 v1.2 Task 5 carries the matching un-ignore step. The red-first/green-trunk
conflict is structurally resolved for both S-21.19's own merge point and S-21.24's flip point.

Both prior structural-fix sets — the ADR-044 capstone-owned-flip split (D-1058, pass 1) and the
BC-1.03.017 Invariant 7 wiring re-key + AC-009 `#[ignore]` gate (D-1060, pass 2) — CONFIRMED HELD
at this pass. This pass's findings are new, surfaced by reviewing the v1.2 bundle's own body
content for residual cite drift and cross-story parity, not a re-opening of any prior finding.

## Part B — New Findings

### MEDIUM

#### F-S2119-P3-001: Task 2's AC-012 test cite retained the stale BC-1.03.017 v1.18 version string
- **Severity:** MEDIUM
- **Category:** stale cross-reference / sibling-site sweep gap (TD-VSDD-060)
- **Location:** S-21.19 v1.2, Task 2 (AC-012 early-authoring instruction), the
  `test_no_on_error_block_without_fail_closed_when_3arg_executor` cite
- **Description:** Pass 2's fix burst (D-1060) re-anchored BC-1.03.017 v1.18 → v1.19 across the
  story and its own v1.2 changelog claimed "all 18 body cites re-anchored." Task 2's cite of
  `BC-1.03.017 v1.18 PC11` in the AC-012 early-authoring instruction was not among the sites swept
  — a live cite left pointing at the pre-remediation Invariant-7-keyed version, one line above the
  Task 2 body that otherwise correctly describes wiring-keyed behavior. This is a sibling-site
  sweep miss (TD-VSDD-060): the pass-2 burst changed the frontmatter cite and most body cites but
  missed this one occurrence.
- **Evidence:** Whitespace-normalized/backtick-tolerant detector run against the pre-fix v1.2 body:
  `tr '\n' ' ' | grep -oE 'BC-1\.03\.017[^v]{0,80}v1\.[0-9]+' | sort | uniq -c` returned 4 live
  `v1.18` occurrences (1 the Task-2 cite; 3 embedded in the v1.2 changelog's own "v1.18 → v1.19"
  narrative text, which are historical and exempt) alongside 18 already-correct `v1.19` cites —
  confirming the v1.2 changelog's "all 18 body cites re-anchored" claim was off by one (18 of 19
  total live cites were actually swept).
- **Proposed Fix:** Sweep the Task 2 cite `BC-1.03.017 v1.18 PC11` → `BC-1.03.017 v1.19 PC11`.
  Independently re-verify `BC-1.01.016` throughout (expected already correctly anchored at v1.3).
- **Status:** RESOLVED this burst (D-1061) — story-writer swept the Task 2 cite to
  `BC-1.03.017 v1.19 PC11` (S-21.19 v1.3). Re-run of the same detector post-fix shows 0 live
  `v1.18` residuals (only the two historical changelog rows — v1.2, v1.3 — narrating the sweep
  itself retain the substring, which is exempt). `BC-1.01.016` independently confirmed already
  correctly anchored at v1.3 throughout, no drift found. True count now 19/19 live
  `BC-1.03.017` cites swept.

### LOW

#### F-S2119-P3-002: `blocks:` frontmatter missing bidirectional parity with S-21.24's `depends_on:`
- **Severity:** LOW
- **Category:** cross-document consistency / DAG parity
- **Location:** S-21.19 v1.2 frontmatter `blocks: [S-21.20, S-21.21, S-21.22, S-21.23]`
- **Description:** S-21.24's `depends_on:` array directly lists S-21.19 (S-21.24 Task 0 consumes
  S-21.19's dormant 3-arg `plugin_fail_closed` extension per ADR-044), but S-21.19's `blocks:`
  array did not list S-21.24 — an asymmetric depends_on/blocks pair. The STORY-INDEX D-1057
  sub-schedule DAG narrative also only shows the transitive path
  S-21.19→{S-21.20,S-21.21,S-21.22,S-21.23}→S-21.24, not the direct S-21.19→S-21.24 edge that the
  frontmatter asymmetry, once fixed, implies.
- **Evidence:** S-21.24 frontmatter `depends_on: [S-21.19, S-21.20, S-21.21, S-21.22, S-21.23]`
  (confirmed present); S-21.19 frontmatter `blocks:` lacked the reciprocal `S-21.24` entry.
- **Proposed Fix:** Add `S-21.24` to S-21.19's `blocks:` array (bidirectional parity; no cycle —
  19→24 is the same direction as the existing transitive path through S-21.20-23). State-manager
  to add the corresponding direct edge to the STORY-INDEX D-1057 sub-schedule DAG narrative.
- **Status:** RESOLVED this burst (D-1061) — story-writer added `S-21.24` to S-21.19 v1.3's
  `blocks:` array (now `[S-21.20, S-21.21, S-21.22, S-21.23, S-21.24]`); state-manager added the
  direct S-21.19→S-21.24 edge to the STORY-INDEX D-1057 sub-schedule DAG (verified acyclic).

## Disposition

Both findings routed per the Agent Routing Table: F-S2119-P3-001 is story content (story-writer's
domain — a cite sweep, not a BC content change), F-S2119-P3-002 is story frontmatter + STORY-INDEX
content (story-writer for the story's own `blocks:` array, state-manager for the STORY-INDEX DAG
narrative and index-parity bookkeeping). Story-writer swept the stale Task 2 cite and added the
`blocks:` entry in a single S-21.19 v1.2→v1.3 burst; state-manager added the direct DAG edge and
extended the STORY-INDEX POLICY 18 blockquote input-hash enumeration to cover the split stories
(a related LOW process-gap both this pass and the S-21.25 pass-3 review independently flagged).
Both fixes same-burst (D-1061).

## Summary

| Severity | Count |
|----------|-------|
| BLOCKER  | 0     |
| HIGH     | 0     |
| MEDIUM   | 1     |
| LOW      | 1     |

**Overall Assessment:** block (pre-remediation, v1.2 bundle) — RESOLVED same burst via
story-writer S-21.19 v1.3 + state-manager STORY-INDEX DAG/blockquote remediation (D-1061).
**Convergence:** findings remain — iterate. LOCAL streak 0/3 (resolving the MEDIUM+LOW findings
does not itself advance the streak per BC-5.39.001; pass 4 required against the v1.3 bundle to
confirm CLEAN).
**Readiness:** requires re-review (pass 4, against S-21.19 v1.3 + BC-1.03.017 v1.19) before TDD
dispatch.

## Novelty Assessment

| Field | Value |
|-------|-------|
| **Pass** | 3 |
| **New findings** | 2 |
| **Duplicate/variant findings** | 0 |
| **Novelty score** | 1.0 (2/2) |
| **Median severity** | 1.5 (MEDIUM/LOW midpoint) |
| **Trajectory** | 1 → 2 → 2 |
| **Verdict** | FINDINGS_REMAIN — resolved this burst (D-1061); pass 4 required to confirm CLEAN against the remediated bundle. |
