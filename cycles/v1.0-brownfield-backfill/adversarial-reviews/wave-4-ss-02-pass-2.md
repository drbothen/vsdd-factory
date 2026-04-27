---
document_type: adversarial-review
level: ops
version: "1.0"
status: complete
producer: adversary
timestamp: 2026-04-26T00:00:00Z
phase: 2-re-anchor
inputs:
  - .factory/stories/S-1.03-hook-sdk-crate.md
  - .factory/stories/S-2.05-hook-sdk-publish.md
  - .factory/stories/S-3.01-port-capture-commit-activity.md
  - .factory/stories/S-3.02-port-capture-pr-activity.md
  - .factory/stories/S-3.03-port-block-ai-attribution.md
  - .factory/specs/prd.md
  - .factory/specs/domain-spec/capabilities.md
  - .factory/specs/architecture/ARCH-INDEX.md
  - .factory/specs/behavioral-contracts/BC-INDEX.md
  - .factory/specs/verification-properties/VP-INDEX.md
  - .factory/specs/verification-properties/VP-038.md
input-hash: "d7ecc0d"
traces_to: ".factory/specs/prd.md#FR-009"
cycle: v1.0-brownfield-backfill
sub_cycle: wave-4-ss-02-re-anchor
pass: 2
previous_review: wave-4-ss-02-pass-1.md
po_commit_reviewed: d7ecc0d
verdict: FINDINGS_REMAIN
finding_count: 1
convergence_step: 0_of_3
---

# Adversarial Review — Wave 4 SS-02 Re-anchor — Pass 2

## Finding ID Convention

Finding IDs use the format: `ADV-W4SS02-P2-<SEV>-<SEQ>`.

## Part A — Closure Verification of Pass-1 Findings

| Pass-1 Finding | Status | Evidence |
|----------------|--------|----------|
| CRIT-001 — VP-038 body Stories dropped S-3.03 | CLOSED | VP-038.md:95 dual-anchored: S-3.03 (Wave 3) + S-1.03 (Wave 4) |
| HIGH-001 — 22 SS-02 BC files retain CAP-TBD | CLOSED (sampled 11 of 22) | BC-2.01.001/002/003, BC-2.02.001/002/005/010, BC-2.04.001/005, BC-2.05.001/003 all show CAP-009 + S-1.03. BC-2.01.002.md:74 dual-anchored S-1.03, S-3.03 |
| HIGH-002 — S-3.01/02/03 missing S-1.03 in depends_on | CLOSED | All three depends_on now include S-1.03 |
| HIGH-003 — VP-INDEX §Story Anchors VP-038 row lost S-3.03 | CLOSED | VP-INDEX.md:145-146 split into 2 rows |
| MED-001 — AC-006 missing VP-025 trace | CLOSED | S-1.03.md:119 appends VP-025 trace |
| MED-002 — status=merged vs status=partial ambiguity | CLOSED | S-1.03 disambiguation note inserted |
| MED-003 — S-2.05 CAP-009 partial-coverage gap | CLOSED | S-2.05.md:138-140 explicit partial-coverage prose |

**Summary:** 7 of 7 pass-1 findings closed.

## Part B — New Findings (1 total: 0 CRIT, 1 HIGH, 0 MED, 0 LOW)

### ADV-W4SS02-P2-HIGH-001 [HIGH] — BC-INDEX row for BC-2.01.002 missed S-3.03 dual-anchor

**Severity:** HIGH (POLICY 1 + POLICY 8 sibling-propagation gap)
**Confidence:** HIGH
**Files:** `.factory/specs/behavioral-contracts/BC-INDEX.md:147`

**Evidence:**
- VP-038.md:95 → dual-anchored S-3.03 + S-1.03 ✓
- BC-2.01.002.md:74 → `Stories | S-1.03, S-3.03` ✓
- VP-INDEX.md:145-146 → 2 rows ✓
- BC-INDEX.md:147 → `| BC-2.01.002 | ... | CAP-009 | S-1.03 |` ✗ MISSING S-3.03

The pass-1 CRIT-001/HIGH-003 fix burst restored the dual anchor in 3 of 4 sibling artifacts but missed the BC-INDEX index row. Same POLICY 1 violation class.

**Fix:** BC-INDEX.md:147 Stories column `S-1.03` → `S-1.03, S-3.03`.

## Observations

- All 7 pass-1 findings closed at primary site (100% closure rate).
- Token Budget BC count consistency CLEAN (S-1.03 BC count=22 matches frontmatter array length and body BC table row count; VP count=7 matches).
- PRD §2.2 FR-009 ↔ S-1.03 BC list coherence CLEAN (22 SS-02 BCs aligned).
- CAP-009 verbatim quote axis CLEAN (S-1.03 + S-2.05 + S-3.03 all match capabilities.md).
- VP-INDEX self-consistency CLEAN (total_vps=64, §Story Anchors now 8 rows after VP-038 split).
- F-104 partial-fix-regression spot-check CLEAN (sampled 11 of 22 BC files).
- Wave 3 sibling regression check CLEAN (S-3.03.bcs preserved; BC body dual-anchored).
- POLICY 1 sweep extended to VP-044/045 — no prior anchors lost.
- Bidirectional dep symmetry CLEAN.

## Sweep Results — Per-Axis

| Policy | Status | Delta from Pass-1 |
|--------|--------|-------------------|
| POLICY 1 (append-only) | partial violation (HIGH-001 BC-INDEX row) | improved |
| POLICY 4 (semantic anchoring) | CLEAN | improved |
| POLICY 5 (creators_justify_anchors) | CLEAN | improved |
| POLICY 6 (subsystem-registry verbatim) | CLEAN | unchanged |
| POLICY 7 (BC H1 source-of-truth) | CLEAN | unchanged |
| POLICY 8 (frontmatter↔body↔index propagation) | partial violation (HIGH-001) | improved |
| POLICY 9 (VP-INDEX coherence) | CLEAN | improved |
| POLICY 10 (demo evidence) | N/A | unchanged |
| Bidirectional dep symmetry | CLEAN | improved |
| F-104 stretch-anchor | CLEAN | unchanged |
| Status field appropriateness | CLEAN | improved |
| Token Budget BC count (NEW axis) | CLEAN | new |
| FR-009 BC list coverage (NEW axis) | CLEAN | new |
| CAP-009 verbatim quote (NEW axis) | CLEAN | new |

## Summary

| Severity | Count |
|----------|-------|
| CRITICAL | 0 |
| HIGH | 1 |
| MEDIUM | 0 |
| LOW | 0 |

**Overall Assessment:** pass-with-findings (single propagation gap)
**Convergence:** findings remain — iterate (1 HIGH blocks per BC-5.04.003)
**Readiness:** requires revision (single-line edit)

## Novelty Assessment

| Field | Value |
|-------|-------|
| **Pass** | 2 |
| **New findings count** | 1 |
| **Duplicate count** | 0 (HIGH-001 is novel sibling-propagation gap) |
| **Novelty score** | 1.0 |
| **Median severity** | HIGH |
| **Severity distribution** | 0 CRIT, 1 HIGH, 0 MED, 0 LOW |
| **Trajectory** | pass-1=7 → pass-2=1 (86% reduction) |
| **Verdict** | FINDINGS_REMAIN |

## Convergence Status

**0 of 3.** 1 HIGH blocks. Trajectory strongly positive. Pass-3 should advance to 1_of_3 after BC-INDEX:147 fix.

## Trajectory Baseline

| Pass | Total | CRIT | HIGH | MED | LOW |
|------|-------|------|------|-----|-----|
| 1 | 7 | 1 | 3 | 3 | 0 |
| 2 | 1 | 0 | 1 | 0 | 0 |
