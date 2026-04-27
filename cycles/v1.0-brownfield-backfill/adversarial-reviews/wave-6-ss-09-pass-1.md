---
document_type: adversarial-review
level: ops
version: "1.0"
status: complete
producer: adversary
timestamp: 2026-04-27T05:00:00Z
phase: 2-re-anchor
inputs:
  - .factory/stories/S-0.01-bump-version-prerelease.md
  - .factory/stories/S-0.04-hooks-json-template-generation.md
  - .factory/stories/S-2.02-registry-toml-generation.md
  - .factory/stories/S-2.03-ci-cross-platform-matrix.md
  - .factory/stories/S-2.04-release-binary-commit.md
  - .factory/stories/S-2.08-beta1-release-gate.md
  - .factory/specs/prd.md
  - .factory/specs/domain-spec/capabilities.md
  - .factory/specs/domain-spec/invariants.md
  - .factory/specs/behavioral-contracts/BC-INDEX.md
  - .factory/specs/behavioral-contracts/ss-09/BC-9.01.001.md
  - .factory/specs/behavioral-contracts/ss-09/BC-9.01.002.md
  - .factory/specs/behavioral-contracts/ss-09/BC-9.01.003.md
  - .factory/specs/behavioral-contracts/ss-09/BC-9.01.004.md
  - .factory/specs/behavioral-contracts/ss-09/BC-9.01.005.md
  - .factory/specs/behavioral-contracts/ss-01/BC-1.07.003.md
  - .factory/specs/behavioral-contracts/ss-01/BC-1.07.004.md
  - .factory/specs/verification-properties/VP-INDEX.md
  - .factory/specs/verification-properties/VP-015.md
  - .factory/specs/verification-properties/VP-049.md
  - .factory/specs/architecture/ARCH-INDEX.md
input-hash: "c7c3053"
traces_to: ".factory/specs/prd.md"
cycle: v1.0-brownfield-backfill
sub_cycle: wave-6-ss-09-re-anchor
pass: 1
verdict: FINDINGS_REMAIN
finding_count: 9
convergence_step: 0_of_3
po_commit_reviewed: 837aedc
previous_review: null
---

# Adversarial Review — Wave 6 SS-09 Re-anchor — Pass 1

## Finding ID Convention

Pass-1 findings use F-001..F-009.

## Part B — New Findings (9 total: 0 CRIT, 4 HIGH, 4 MED, 1 LOW)

### F-001 [HIGH] — VP-049 ↔ BC-1.07.003/004 bidirectional back-reference missing (POLICY 9)

VP-049 anchored S-2.02 + bcs:[BC-1.07.003, BC-1.07.004] but BC-1.07.003.md and BC-1.07.004.md still show Verification Properties: TBD and Stories: TBD. POLICY 9 step 4-5 violated. Same-burst propagation gap recurrence (Wave 5 pass-3 pattern).

**Fix:** update BC-1.07.003 + BC-1.07.004 Verification Properties tables to cite VP-049 + Stories field append S-2.02.

### F-002 [HIGH] — S-2.03 AC-4 traces BC-9.01.003 outside its bcs[] (POLICY 8)

S-2.03 frontmatter bcs:[BC-9.01.004] but AC-4 cites BC-9.01.003. Either append BC-9.01.003 to bcs[] (option a) or reword AC-4 trace as soft cross-reference to S-2.04's anchor (option b). AC-7 also traces BC-1.07 regression — needs `[process-gap]` markup.

### F-003 [HIGH] — S-2.08 has 3+ ACs tracing BCs outside bcs[] without `[process-gap]` (POLICY 8)

S-2.08 bcs:[BC-9.01.001-005] but AC-2 cites BC-1.07, AC-3 cites BC-1.07, AC-4 cites BC-4.01/BC-7.05 — all cross-subsystem references silently anchored. Either append cross-SS BCs to bcs[] (option a) or mark `[process-gap]` with v1.1 BC candidates (option b).

### F-004 [HIGH] — S-2.04 declares subsystems:[SS-09, SS-10] but no BC-10.x anchored (POLICY 4/POLICY 6) `(pending intent verification)`

S-2.04 target_module is `.github/workflows/Release.yml` (SS-09 per ARCH-INDEX). bcs:[BC-9.01.002, BC-9.01.003] both SS-09. SS-10 declaration may be stale; clarify intent.

### F-005 [MEDIUM] — CAP-028 SS-09-only Subsystems vs FR-029 SS-06+SS-09 (Wave 3 F-007 expansion candidate)

capabilities.md §CAP-028 Subsystems: SS-09. PRD §8 CAP-028: SS-09. PRD FR-029 row CAPs: CAP-007, CAP-028; Subsystems: SS-06, SS-09. Either expand CAP-028 to SS-06 (Wave 3 F-007 precedent) or remove CAP-028 from FR-029 (FR-029 already cites CAP-007).

### F-006 [MEDIUM] — BC-1.07.003 / BC-1.07.004 capability:CAP-TBD orphan

BC-1.07.003/004 frontmatter capability:CAP-TBD. VP-049 anchors them. STORY-INDEX deferred them from Wave 1. Wave 6 has the trigger to close — anchor to CAP-002 or new CAP.

### F-007 [MEDIUM] — BC-9.01.* status:draft vs PRD §FR-037 narrative "all 5 BCs anchored"

PRD line 723 says "all 5 BCs anchored" but BC frontmatter status:draft. Reword PRD to "anchored (status:draft pending verification)" or update BC status post-anchor. Note: status:draft is pervasive across 1,800+ BCs; pre-existing systemic.

### F-008 [MEDIUM] — VP-049 source_bc singular vs bcs:[BC-1.07.003, BC-1.07.004] (VP frontmatter coherence)

VP-049 source_bc:BC-1.07.003 but bcs:[BC-1.07.003, BC-1.07.004]. VP-015 has co-anchor pattern (line 49 source_bc + line 50 "Co-anchor:"). Add Co-anchor line for BC-1.07.004 in VP-049.

### F-009 [LOW] — VP-015 anchor justification S-2.03 partial-vehicle scope `(pending intent verification)`

S-2.03 bcs:[BC-9.01.004] only; VP-015 bcs:[BC-9.01.004, BC-9.01.005]. S-2.03 contributes 9.01.004 prerequisite but not 9.01.005. If anchor stories are a list, no defect; if single primary, S-2.06 was Wave 5 primary.

## Observations / Nitpicks

- **OBS-1 [process-gap recurrence]:** F-001 is second-wave instance of Wave 5 pass-3 SS-06 BC↔VP gap. Recommend codification per `rules/lessons-codification.md`.
- **OBS-2:** S-2.08 AC-2/AC-3 use BC range "BC-1.07" instead of specific BC IDs. Tighten AC traces.
- **OBS-3:** Adversary could not load S-0.04 + S-2.02 via Glob — orchestrator should spot-check axes 4 (POLICY 8) and 9 (CAP-028 anchor justification verbatim) for those two stories during fix burst.
- **OBS-4:** PRD §FR-037 narrative arithmetic verified correct (BC-9.01.004/005 → CAP-007 stories + BC-9.01.001-003 → CAP-028 stories).

## CAP Subsystem Drift Sweep — FOUND on CAP-028

CAP-028 SS-09-only vs FR-029 SS-06+SS-09 (see F-005).

## Summary

| Severity | Count |
|----------|-------|
| CRITICAL | 0 |
| HIGH | 4 |
| MEDIUM | 4 |
| LOW | 1 |

**Overall Assessment:** pass-with-findings
**Convergence:** findings remain — iterate
**Readiness:** requires revision

## Novelty Assessment

| Field | Value |
|---|---|
| **Pass** | 1 |
| **New findings count** | 9 |
| **Duplicate count** | 0 |
| **Novelty score** | 1.0 |
| **Median severity** | HIGH-MED boundary |
| **Severity distribution** | 0 CRIT, 4 HIGH, 4 MED, 1 LOW |
| **Trajectory** | starting baseline (9) |
| **Verdict** | FINDINGS_REMAIN |

## Convergence Status

**0 of 3.** 4 HIGH findings block convergence per BC-5.04.003. F-001 is the dominant blocker (recurring Wave 5 pattern).

## Findings by Axis

| Axis | Findings |
|---|---|
| POLICY 1 (append-only) | (clean) |
| POLICY 4 (semantic anchoring) | F-003, F-004, F-005, F-009 |
| POLICY 7 (BC H1 = title) | (clean — sampled BC-9.01.001-005) |
| POLICY 8 (bcs[] ↔ body ↔ ACs) | F-002, F-003 |
| POLICY 9 (VP↔BC bidirectional) | F-001, F-008 |
| CAP→PRD §8 propagation | F-005 |
| Bidirectional dep edges | (clean for sampled) |
| Subsystem coherence | (S-2.02 not reviewed; OBS-3) |
| CAP-028 anchor justification verbatim | (S-0.01/S-2.04/S-2.08 clean; S-0.04/S-2.02 not reviewed; OBS-3) |
| Same-burst sibling propagation | F-001 |
| Process-gap markers | F-003 |

## Trajectory Baseline

Pass-1 = 9 findings. Wave comparison:

- Wave 1 SS-01 pass-1: 10
- Wave 2 SS-03 pass-1: 11
- Wave 3 SS-04 pass-1: 11
- Wave 4 SS-02 pass-1: 7
- Wave 5 SS-06 pass-1: 11
- Wave 6 SS-09 pass-1: 9 — within band; smaller surface (5 BCs vs Wave 5's 5+1.07 cross). Expected fewer passes; 3-of-3 convergence plausible by pass 4-5.

## Verdict

**FINDINGS_REMAIN.** 4 HIGH semantic-anchoring + propagation findings. F-001 is the dominant blocker. Recommended next step: PO fix burst targeting F-001..F-004 first, then F-005..F-008 with intent-adjudication where flagged `(pending intent verification)`.
