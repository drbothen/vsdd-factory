---
document_type: adversarial-review
level: ops
version: "1.0"
status: complete
producer: adversary
timestamp: 2026-04-27T00:00:00Z
phase: 2-re-anchor
inputs:
  - .factory/stories/S-0.02-release-workflow-prerelease.md
  - .factory/stories/S-4.08-rc1-release-gate.md
  - .factory/stories/S-5.07-v1.0-release-gate.md
  - .factory/stories/S-2.08-beta1-release-gate.md
  - .factory/stories/S-0.01-bump-version-prerelease.md
  - .factory/stories/STORY-INDEX.md
  - .factory/specs/behavioral-contracts/ss-09/BC-9.01.001.md
  - .factory/specs/behavioral-contracts/ss-09/BC-9.01.003.md
  - .factory/specs/behavioral-contracts/BC-INDEX.md
  - .factory/specs/architecture/ARCH-INDEX.md
  - .factory/specs/architecture/SS-09-config-activation.md
  - .factory/specs/architecture/SS-10-cli-tools.md
  - .factory/specs/domain-spec/capabilities.md
  - .factory/specs/prd.md
  - .factory/policies.yaml
  - .factory/cycles/v1.0-brownfield-backfill/adversarial-reviews/wave-7-ss-10-pass-1.md
input-hash: "a0c44d4"
traces_to: ".factory/specs/prd.md"
cycle: v1.0-brownfield-backfill
sub_cycle: wave-7-ss-10-re-anchor
pass: 2
verdict: FINDINGS_REMAIN
finding_count: 4
convergence_step: 0_of_3
po_commit_reviewed: 5ffa93d
previous_review: wave-7-ss-10-pass-1.md
---

# Adversarial Review — Wave 7 SS-10 Re-anchor — Pass 2

## Finding ID Convention

Pass-2 findings use F-101..F-104.

## Part A — Pass-1 Closure Verification

5 of 5 pass-1 findings verified CLOSED at PO 5ffa93d + state-manager b2c1a3d. F-001/F-002/F-003/F-004/F-005 all intact; no regressions detected.

## Part B — New Findings (4 total: 0 CRIT, 1 HIGH, 2 MED, 1 LOW)

### F-101 [HIGH] — POLICY 7 violation: BC-9.01.001 H1 prerelease-only scope vs S-5.07 AC-9 stable-format anchor

BC-9.01.001 H1 reads "bump-version.sh accepts semver prerelease format" but S-5.07 AC-9 anchors stable-format `1.0.0` invocation. POLICY 7 (BC H1 source-of-truth) requires enrichment to go INTO H1, not downstream-only acknowledgment.

**Fix (option 1):** Enrich BC-9.01.001 H1 to cover stable+prerelease scope; sync to BC-INDEX + body BC tables across 5 stories.

### F-102 [MED] — AC classification correctness: S-0.02 AC-1/AC-2 mis-classified as direct BC exercise

S-0.02 AC-1 + AC-2 exercise Release.yml's prerelease:true flag emission, NOT BC-9.01.001/003 directly. Disclosure section claim contradicts BC scope.

**Fix:** Re-classify as [process-gap]; add v1.1 candidate BC-10.13.012 covering both prerelease:true and false branches.

### F-103 [MED] — S-5.07 disclosure section maps "stable-format scope" to BC-10.13.011 (mis-mapping)

BC-10.13.011 is for marketplace default-switch, not stable-format scope.

**Fix (post-F-101):** F-101 H1 enrichment closes the stable-format gap; remove the stretch paragraph from S-5.07 disclosure.

### F-104 [LOW] — Bidirectional dep symmetry gap: S-0.01.blocks ↔ S-4.08/S-5.07.depends_on missing S-0.01

**Fix:** Prepend S-0.01 to S-4.08/S-5.07 depends_on (gates need bump-version.sh available).

## Sibling Sweep Results

- F-001 disclosure shape evolves Wave 5 pattern with explicit "BC-subsystem != story.subsystems[]" naming — CLEAN
- F-002 sibling sweep (other CAP surfaces): no Wave 7 collateral drift on CAP-024/026/027 — CLEAN
- F-003 sibling sweep (shared-ownership files): scripts/bump-version.sh + Release.yml only — no further candidates
- AC contractor-vs-gate classification: 16 of 18 ACs correctly classified; 2 mis-classified (F-102)

## CAP Subsystem Drift Sweep — F-002 RESOLVED, no new drift

CAP-028 secondary-SS disclosure HTML comments at capabilities.md:86 + prd.md:1123 verified.

## Summary

| Severity | Count |
|----------|-------|
| CRITICAL | 0 |
| HIGH | 1 |
| MEDIUM | 2 |
| LOW | 1 |

**Overall Assessment:** pass-with-findings (HIGH ceiling — POLICY 7 violation)
**Convergence:** findings remain — iterate
**Readiness:** requires revision

## Novelty Assessment

| Field | Value |
|---|---|
| **Pass** | 2 |
| **New findings count** | 4 |
| **Duplicate count** | 0 |
| **Novelty score** | 1.0 |
| **Median severity** | MED |
| **Severity distribution** | 0 CRIT, 1 HIGH, 2 MED, 1 LOW |
| **Trajectory** | 5 → 4 (decay; HIGH ceiling unchanged) |
| **Verdict** | FINDINGS_REMAIN |

## Convergence Status

**0 of 3.** F-101 HIGH POLICY 7 blocks per BC-5.04.003.

## Findings by Axis

| Axis | Findings |
|---|---|
| POLICY 4 (semantic anchoring stretch) | F-102, F-103 |
| POLICY 7 (BC H1 source-of-truth) | F-101 |
| Same-burst sibling propagation | F-104 |
| F-001 disclosure-correctness | F-102, F-103 |
| Bidirectional dep symmetry | F-104 |

## Trajectory Baseline

Pass-1=5 → pass-2=4 (-1 finding; HIGH count stable; severity content shifted from disclosure-absence to deeper semantic-scope).

## Verdict

**FINDINGS_REMAIN.** PO fix burst applied at 0f2d432: F-101 H1 enrichment + sibling sync (BC-INDEX, 5 stories, PRD §FR-037 + §8); F-102 re-classification + BC-10.13.012; F-103 disclosure update; F-104 dep symmetry. State-manager handles F-104 STORY-INDEX + persist this pass.
