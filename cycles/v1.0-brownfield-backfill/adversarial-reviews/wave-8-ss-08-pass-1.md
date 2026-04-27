---
document_type: adversarial-review
level: ops
version: "1.0"
status: complete
producer: adversary
timestamp: 2026-04-27T00:00:00Z
phase: 2-re-anchor
inputs:
  - .factory/stories/S-0.05-docs-scaffolding.md
  - .factory/stories/S-5.05-migration-guide.md
  - .factory/stories/S-5.06-semver-commitment-docs.md
  - .factory/specs/behavioral-contracts/ss-08/BC-8.22.001.md
  - .factory/specs/behavioral-contracts/ss-08/BC-8.26.001.md
  - .factory/specs/behavioral-contracts/ss-08/BC-8.26.006.md
  - .factory/specs/behavioral-contracts/BC-INDEX.md
  - .factory/specs/domain-spec/capabilities.md
  - .factory/specs/architecture/ARCH-INDEX.md
  - .factory/specs/architecture/SS-08-templates-rules.md
  - .factory/specs/prd.md
  - .factory/stories/STORY-INDEX.md
  - .factory/STATE.md
  - .factory/policies.yaml
input-hash: 58aecb6
traces_to: ".factory/specs/prd.md"
cycle: v1.0-brownfield-backfill
sub_cycle: wave-8-ss-08-re-anchor
pass: 1
verdict: FINDINGS_REMAIN
finding_count: 9
convergence_step: 0_of_3
po_commit_reviewed: 21fb210
previous_review: null
---

# Adversarial Review — Wave 8 SS-08 Re-anchor — Pass 1

## Finding ID Convention

Pass-1 findings use F-001..F-009.

## Part B — New Findings (9 total: 0 CRIT, 2 HIGH, 4 MED, 3 LOW)

### F-001 [HIGH] — S-0.05 missing first AC bullet (POLICY 8 body corruption carry-over)

S-0.05 ACs section has orphan "introduction, SDK overview..." continuation without leading bullet. AC-1 missing.

### F-002 [HIGH] — Systematic POLICY 8 violation: 3 stories lack AC traces and F-204 exemption

All 3 Wave 8 stories have no per-AC BC trace cells AND no F-204 HTML comment exemption. 9 total trace gaps (3 stories × 3 BCs). Wave 7 sanctioned shape not propagated.

### F-003 [MED] — Uniform 3-BC anchor across 3 semantically distinct stories

S-0.05 (skeleton-only) anchored to BC-8.26.006 (user-facing-docs deliverable) — stretch since skeletons aren't deliverables.

### F-004 [MED] — `v1.1-BC-8.XX.NNN` candidate ID format deviates from precedent

Wave 1-7 used `BC-N.NN.NNN-slug`. Wave 8 used `v1.1-BC-8.XX.NNN` (literal XX placeholder). Format inconsistency.

### F-005 [MED] — STORY-INDEX missing Wave 8 SS-08 baseline summary block

Pattern inconsistency with Waves 1, 2, 7 (which have summary blocks).

### F-006 [MED] — Imprecise "All ACs" cells in body BC tables

Story body BC table `Covering AC` cells read "All ACs" — uncheckable hand-wave.

### F-007 [LOW pending intent] — `input-hash:""` selectively added to 3 BCs but not siblings

Sibling-propagation gap; pending intent (sweep all 218 SS-08 BCs vs revert).

### F-008 [LOW] — PRD HTML comment placed under §FR-043 instead of §FR-036

Wave 8 disclosure under wrong FR; should be under rules-family FR-036.

### F-009 [LOW pending intent] — capabilities.md CAP-014 has no Wave 8 inline comment

Soft mild drift; no CAP expansion needed but disclosure convention deviates.

## Stretch-Anchor Scrutiny

Uniform 3-BC anchor pattern verdict: MIXED. BC-8.22.001 + BC-8.26.001 are universal-applicability BCs (technically true but tautological as anchors). BC-8.26.006 is a precise fit for S-5.05/S-5.06 deliverables but a stretch for S-0.05 skeleton.

## Sibling Sweep Results

CLEAN: BC-INDEX SS-08 rows, BC frontmatter subsystem alignment, BC body Stories, BC capability=CAP-014, story BC table H1 verbatim, story bcs[] ↔ body table coherence, PRD §8 CAP-014 row, ARCH-INDEX, STATE.md.

FAIL: F-002 (story body BC table ↔ AC traces), F-008 (PRD §FR-036 inline disclosure), F-009 (capabilities.md inline), F-005 (STORY-INDEX), F-007 (input-hash uniformity).

## CAP Subsystem Drift Sweep

CAP-014 Subsystems: SS-05, SS-06, SS-08 (already includes SS-08). No CAP expansion needed.

## Summary

| Severity | Count |
|----------|-------|
| CRITICAL | 0 |
| HIGH | 2 |
| MEDIUM | 4 |
| LOW | 3 |

**Overall Assessment:** pass-with-findings (HIGH ceiling)
**Convergence:** findings remain — iterate
**Readiness:** requires revision

## Novelty Assessment

| Field | Value |
|---|---|
| **Pass** | 1 |
| **New findings count** | 9 |
| **Duplicate count** | 0 |
| **Novelty score** | 1.0 |
| **Median severity** | MED |
| **Severity distribution** | 0 CRIT, 2 HIGH, 4 MED, 3 LOW |
| **Trajectory** | starting baseline (9) |
| **Verdict** | FINDINGS_REMAIN |

## Convergence Status

**0 of 3.** F-001 + F-002 HIGH block.

## Findings by Axis

| Axis | Findings |
|---|---|
| POLICY 1 (format) | F-004 |
| POLICY 4 (semantic anchoring) | F-003, F-008 |
| POLICY 7 (BC H1 verbatim) | (clean) |
| POLICY 8 (bcs[] ↔ body ↔ ACs) | F-001, F-002, F-005, F-007 |
| Sibling propagation / disclosure | F-009 |

## Trajectory Baseline

Pass-1 baseline 9. Wave comparison: Wave 6=9, Wave 7=5. Wave 8 same band as Wave 6.

## Verdict

**FINDINGS_REMAIN.** PO fix burst applied at 21ea6d3 (F-001/002/003/004/006/008/009). State-manager handles F-005 + persist + F-007 deferred.
