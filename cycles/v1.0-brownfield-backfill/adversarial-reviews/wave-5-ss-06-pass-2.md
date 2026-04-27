---
document_type: adversarial-review
level: ops
version: "1.0"
status: complete
producer: adversary
timestamp: 2026-04-26T00:00:00Z
phase: 2-re-anchor
inputs:
  - .factory/stories/S-0.03-activation-skill-platform-detection.md
  - .factory/stories/S-2.06-activation-skill-integration.md
  - .factory/specs/prd.md
  - .factory/specs/domain-spec/capabilities.md
  - .factory/specs/domain-spec/invariants.md
  - .factory/specs/behavioral-contracts/BC-INDEX.md
  - .factory/specs/behavioral-contracts/ss-09/BC-9.01.001.md
  - .factory/specs/behavioral-contracts/ss-09/BC-9.01.005.md
  - .factory/specs/verification-properties/VP-INDEX.md
  - .factory/specs/verification-properties/VP-015.md
input-hash: "a20a973"
traces_to: ".factory/specs/prd.md#FR-037"
cycle: v1.0-brownfield-backfill
sub_cycle: wave-5-ss-06-re-anchor
pass: 2
previous_review: wave-5-ss-06-pass-1.md
po_commit_reviewed: a20a973
verdict: FINDINGS_REMAIN
finding_count: 7
convergence_step: 0_of_3
---

# Adversarial Review — Wave 5 SS-06 Re-anchor — Pass 2

## Finding ID Convention

Finding IDs use the format: `ADV-W5SS06-P2-<SEV>-NNN`.

## Part A — Fix Verification (Pass-1 closures)

| Pass-1 Finding | Status |
|----------------|--------|
| CRIT-001 (PRD §FR-037 BC titles desync) | CLOSED — all 5 titles match BC H1s verbatim; scope note documents dual-scope |
| CRIT-002 (BC-9.01.002 in S-0.03) | CLOSED — frontmatter + body + Token Budget = 3 BCs |
| HIGH-001 (BC-9.01.001/003 in S-2.06) | CLOSED — frontmatter + body + Token Budget = 9 BCs |
| HIGH-002 + MED-004 (CAP-007 SS-01) | CLOSED — capabilities.md + PRD §8 verbatim match (SS-06, SS-09) |
| HIGH-003 (CAP-028 + DI-015) | PARTIAL — see new HIGH-002 below; DI-015 cited by BC-9.01.004/005; CAP-028 dropped |
| HIGH-004 (VP-015 missing) | PARTIAL — see new CRIT-001 below; VP-015 added to S-2.06 + VP-INDEX but VP-015.md internal anchors stale |

5 fully closed, 2 partial closures introduced new gaps.

## Part B — New Findings (7 total: 2 CRIT, 2 HIGH, 2 MED, 1 LOW)

### ADV-W5SS06-P2-CRIT-001 — VP-015.md frontmatter+body still anchor to release-tooling BC-9.01.001

**Severity:** CRITICAL (POLICY 4 + POLICY 7 + POLICY 9)
**Files:** VP-015.md:14 (source_bc), :35 (bcs), :49 (Source Contract), :89 (Traceability)

**Evidence:** VP-015.md declares source_bc=BC-9.01.001 in 4 locations; BC-9.01.001 H1 is "bump-version.sh accepts semver prerelease format" (release-tooling). VP-015 ("Per-Project Activation Required") should anchor BC-9.01.005 (hooks.json gitignored — gate artifact, cites DI-015) + BC-9.01.004 (5-platform CI — gate prerequisite).

**Fix:** Update VP-015.md `source_bc: BC-9.01.005`; `bcs: [BC-9.01.004, BC-9.01.005]`; rewrite Source Contract + Traceability §BCs.

### ADV-W5SS06-P2-CRIT-002 — PRD §8 CAP-007 row asserts CAP-007 BC range BC-9.01.001-005 when only 004/005 carry CAP-007

**Severity:** CRITICAL (POLICY 5 + POLICY 7)
**Files:** prd.md:1095; BC-INDEX.md:1913-1917; BC-9.01.001/002/003.md:15 (CAP-TBD)

**Evidence:** PRD §8:1095 still labels "BC-9.01.001-005 (release/CI)". Post pass-1 reversion, only BC-9.01.004/005 are CAP-007. Same desync class as pass-1 CRIT-001 but in §8 instead of §FR-037.

**Fix:** PRD §8:1095 BC range → "BC-9.01.004-005 (CI matrix + hooks.json gitignore — activation-gate prerequisites)". Drop "(release/CI)" label.

### ADV-W5SS06-P2-HIGH-001 — PRD §FR-037 Status field overstates "shipped" coverage

**Severity:** HIGH (POLICY 7)
**Files:** prd.md:723 + scope note 725

**Evidence:** Line 723 says "Status: shipped (S-0.03, S-0.04, S-2.06)". Scope note 725 explicitly states BC-9.01.001/002/003 are release-pipeline, NOT shipped via activate-skill stories. Top-to-bottom contradiction.

**Fix:** Refine to: "Status: shipped for BC-9.01.004/005 (S-0.03, S-0.04, S-2.06); BC-9.01.001-003 ship via release tooling (no v1.0 story anchor — pending re-anchor to release-pipeline story)."

### ADV-W5SS06-P2-HIGH-002 — DI-015 BC range "BC-9" loose; refine to specific cited BCs

**Severity:** HIGH (POLICY 2)
**Files:** invariants.md:107 + BC-9.01.001-005.md L2 Domain Invariants cells

**Evidence:** invariants.md:107 says "BC range: BC-9". Only BC-9.01.004/005 cite DI-015 post pass-1 fix. Bidirectional symmetry per POLICY 2 step 4 ambiguous.

**Fix:** invariants.md:107 → "BC range: BC-9.01.004 (CI matrix), BC-9.01.005 (hooks.json gitignore — gate artifact)."

### ADV-W5SS06-P2-MED-001 — VP-015 Source Contract description fabricated

**Severity:** MEDIUM (POLICY 5; subsumed by CRIT-001 prose drift)
**Files:** VP-015.md:47-50

**Evidence:** VP-015 §Source Contract says "BC-9.01.001 — Per-project activation gates dispatcher use" but BC-9.01.001 actual postconditions are "Version bump succeeds. CHANGELOG retains monotonicity." Fabricated description.

**Fix:** Subsumed by CRIT-001 (a) — when source_bc points to BC-9.01.005, this section rewrites correctly.

### ADV-W5SS06-P2-MED-002 [process-gap] — Pass-1 fix burst lacked bc-anchor-sweep step

**Severity:** MEDIUM (process-gap)
**Files:** N/A

**Evidence:** Pass-1 fix burst correctly updated 9 artifact locations (PRD §FR-037 titles, story frontmatters, BC-INDEX, BC files frontmatter, capabilities.md, VP-INDEX) but missed VP-015.md frontmatter + PRD §8 BC range + PRD §FR-037 Status field + invariants.md BC range. Same defect class recurring.

**Fix:** Codify `bc-anchor-sweep` checklist step: "When BC capability/title changes, grep for the BC ID across `.factory/specs/` and validate every citation." Document in `rules/lessons-codification.md` or add lint hook.

### ADV-W5SS06-P2-LOW-001 — Manual-VP-vs-merged-story semantics (pending intent verification)

**Severity:** LOW (pending intent verification)
**Files:** VP-015.md:92

**Evidence:** VP-015 proof_method=manual; S-2.06 status=merged. Lifecycle question: is verification stale post-merge?

**Fix:** Add note "Manual verification last executed at [date]; re-execute on each release-bearing build." or document manual-VP convention.

## Sweep Results — Per-Axis

| Policy | Status |
|--------|--------|
| POLICY 1 (append-only) | CLEAN |
| POLICY 2 (lift_invariants_to_bcs) | partial (HIGH-002) |
| POLICY 4 (semantic_anchoring_integrity) | VIOLATION (CRIT-001) |
| POLICY 5 (creators_justify_anchors) | VIOLATION (CRIT-002, MED-001) |
| POLICY 6 (subsystem-registry verbatim) | CLEAN |
| POLICY 7 (BC H1 source-of-truth) | VIOLATION (CRIT-001, CRIT-002, HIGH-001) |
| POLICY 8 (frontmatter↔body↔ACs) | CLEAN |
| POLICY 9 (vp_index_is_vp_catalog_source_of_truth) | partial (CRIT-001) |

### POLICY 7 BC H1 verbatim sweep

VP-015.md:49 fabricates "Per-project activation gates dispatcher use" for BC-9.01.001. VIOLATION.

### CAP→PRD §8 propagation

Subsystems consistent (HIGH-002 closed). BC range column stale (CRIT-002).

## Summary

| Severity | Count |
|----------|-------|
| CRITICAL | 2 |
| HIGH | 2 |
| MEDIUM | 2 |
| LOW | 1 |

**Overall Assessment:** SUBSTANTIVE — fix burst closed 5 of 11 cleanly but introduced 2 fresh CRITICALs (downstream-artifact desync class, recurring).

## Convergence

**Convergence step: 0_of_3.** Verdict: FINDINGS_REMAIN.

## Novelty Assessment

| Field | Value |
|-------|-------|
| **Pass** | 2 |
| **New findings count** | 7 |
| **Carryover from pass-1** | 0 |
| **Same defect class as pass-1** | YES (BC-anchor-sweep gap) |
| **Novelty score** | 0.7 |
| **Median severity** | HIGH |
| **Severity distribution** | 2 CRIT, 2 HIGH, 2 MED, 1 LOW |
| **Trajectory** | pass-1=11 → pass-2=7 (-36%) |
| **Verdict** | FINDINGS_REMAIN |

## Trajectory

| Pass | Total | CRIT | HIGH | MED | LOW |
|------|-------|------|------|-----|-----|
| 1 | 11 | 2 | 4 | 4 | 3 |
| 2 | 7 | 2 | 2 | 2 | 1 |

## Verdict

**FINDINGS_REMAIN.** 2 CRITICAL findings block convergence; 4 substantive fixes recommended for pass-2 fix burst.
