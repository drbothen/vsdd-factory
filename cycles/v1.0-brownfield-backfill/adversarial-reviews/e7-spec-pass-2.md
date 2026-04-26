---
document_type: adversarial-review-pass
pass: 2
scope: e7-spec
cycle: v1.0-brownfield-backfill
date: 2026-04-26
reviewer: adversary
status: MINOR
verdict: MINOR
novelty_score: NITPICK
finding_count: 5
trajectory: "12 → 5"
convergence: FINDINGS_REMAIN
artifacts_reviewed:
  bcs: [BC-5.36.001..007, BC-5.37.001..002, BC-7.05.001..004, BC-8.28.001..002]
  vps: [VP-061, VP-062]
  frs: [FR-042]
  stories: [S-7.01, S-7.02]
  epics: [E-7]
---

# Adversarial Review — Pass 2 (E-7 Process Codification)

## Verdict

**MINOR** (advancing toward 3-NITPICK convergence). Pass-1 fix application largely correct: F-001 BC renumber landed cleanly across 6 surfaces; F-002, F-003, F-004 swept fully; F-006 EC regex now plausible; F-009/F-010/F-011 textual fixes verified. F-005, F-007, F-008 partially landed — VP file rename + multi-axis note + additional_modules + expanded inputs all present, but VP-INDEX line 114 title still stale (F-013) and E-7 epic body has 3 sibling-file propagation gaps (F-014, F-015, F-017).

5 new findings: 1 HIGH (F-013 VP-INDEX title sync), 2 MEDIUM (F-014 epic body BC-TBD, F-015 epic frontmatter prd_frs empty), 2 LOW (F-016 speculative orchestrator path, F-017 epic stories status). Trajectory 12 → 5 is healthy.

The pass-1 fix surface itself dogfooded the partial-fix-regression risk that BC-5.36.005 + BC-5.36.006 codify. The adversary catching VP-INDEX title drift and epic body drift IS the partial-fix axis working as designed.

## Part A — Pass-1 Fix Verification

| Finding | Sev | Status | Evidence |
|---------|-----|--------|----------|
| F-001 | HIGH | ✅ | BC-7.05.004.md exists; BC-INDEX SS-07=196, SS-09=5; ARCH-INDEX same; VP-062, S-7.02, PRD all reference BC-7.05.004; defensive sweep BC-9.02.001 = 0 hits |
| F-002 | HIGH | ✅ | VP-INDEX line 113 VP-061 type = `invariant` |
| F-003 | HIGH | ✅ | PRD: 1,878 everywhere; 42 logical FRs; SS-05=636, SS-07=196, SS-08=217; defensive sweep clean |
| F-004 | HIGH | ✅ | STATE.md FR=42, story=44, Ready (3) row added |
| F-005 | HIGH/MED | ⚠️ PARTIAL | VP-062 file H1 + multi-axis note correct; VP-INDEX line 114 still stale → see F-013 |
| F-006 | HIGH/MED | ✅ | BC-7.05.004 EC-002 directs implementer to inspect existing format |
| F-007 | MED | ✅ | additional_modules[] on VP-061 (3 agents) + VP-062 (4 modules); schema gap noted as O-07 |
| F-008 | MED | ✅ | inputs[] expanded on both VPs |
| F-009 | MED | ✅ | BC-5.36.005 EC-002 reworded for adversary detection-only role |
| F-010 | MED | ✅ | BC-5.36.001 Postcondition 1 + Invariant 1 + EC-002 + Test Vector all reference canonical pattern |
| F-011 | MED | ✅ | BC-7.05.003 exit codes 1 → 2 throughout |
| F-012 | MED | ⚠️ | Path updated with TBD note; speculative path may be wrong → see F-016 |

## Part B — Defensive Sweep Results

| Sweep | Expected | Result |
|-------|----------|--------|
| `1,?863` in PRD | 0 hits | ✅ 0 hits |
| `41 logical FRs` / `41.*FRs` in PRD | 0 hits | ✅ 0 hits |
| `BC-9.02.001` in specs/stories | 0 hits | ✅ 0 hits |

All three defensive sweeps clean — pass-1 propagation discipline applied correctly to enumerated surfaces.

## Part C — New Findings

### F-013 — VP-INDEX VP-062 title stale; F-005 rename did not propagate (HIGH)

- **Confidence:** HIGH
- **Artifact:** `.factory/specs/verification-properties/VP-INDEX.md` line 114
- **Evidence:** VP-062.md H1 = "S-7.02 Process-Codification Surface Invariant — All Codification Artifacts Are Present and Coherent". VP-INDEX line 114 = "Count Propagation Invariant — validate-count-propagation.sh Flags Drift and Is Correctly Registered" (stale).
- **POLICY 9 violation:** vp_index_is_vp_catalog_source_of_truth.
- **Fix:** Update VP-INDEX line 114 title cell.

### F-014 — E-7 epic body BC table still has BC-TBD placeholders (MEDIUM)

- **Confidence:** HIGH
- **Artifact:** `.factory/stories/epics/E-7-process-codification.md` lines 84-96
- **Evidence:** 7 BC-TBD rows in body table; "BCs are pending product-owner authorship" disclaimer at line 84-86 — both stale.
- **POLICY 8 violation (epic-level analog):** bc_array_changes_propagate_to_body.
- **Fix:** Replace 7 BC-TBD rows with real BCs (BC-5.36.001-007, BC-5.37.001-002, BC-7.05.001-004, BC-8.28.001-002); remove disclaimer.

### F-015 — E-7 frontmatter `prd_frs: []` empty though FR-042 anchored (MEDIUM)

- **Confidence:** HIGH
- **Artifact:** `.factory/stories/epics/E-7-process-codification.md` line 6
- **Evidence:** PRD FR-042 row maps to E-7. Both stories cite FR-042. E-7 frontmatter `prd_frs: []` is empty.
- **Fix:** `prd_frs: []` → `prd_frs: [FR-042]`.

### F-016 — BC-8.28.002 Architecture Module path speculative (LOW)

- **Confidence:** MEDIUM
- **Artifact:** `.factory/specs/behavioral-contracts/ss-08/BC-8.28.002.md` line 82
- **Evidence:** Path `agents/orchestrator/orchestrator.md` is nested. ARCH-INDEX SS-05 implies flat directory. Likely `agents/orchestrator.md` or `skills/orchestrator/SKILL.md`.
- **Fix:** Either confirm path by inspection or replace with explicit "TBD: orchestrator skill location" without speculative claim.

### F-017 — E-7 Stories table status column says "draft" but stories are ready (LOW)

- **Confidence:** HIGH
- **Artifact:** `.factory/stories/epics/E-7-process-codification.md` lines 113-114
- **Evidence:** S-7.01 + S-7.02 frontmatter `status: ready`; epic body table shows "draft".
- **Fix:** Update status column to "ready" for both rows.

## Coverage Assessment

All 15 BCs read in full. Both VPs read. PRD §FR-042 + count surfaces verified. S-7.01, S-7.02, E-7 read in full. ARCH-INDEX, BC-INDEX SS-07/SS-09, VP-INDEX, STATE.md verified. Defensive sweeps clean.

## Policy Compliance

| Policy | Status |
|--------|--------|
| 1 (append_only_numbering) | ✅ PASS |
| 2 (lift_invariants_to_bcs) | ⚠️ WARN (carried O-04) |
| 3 (state_manager_runs_last) | n/a |
| 4 (semantic_anchoring_integrity) | ⚠️ WARN (F-016) |
| 5 (creators_justify_anchors) | ✅ PASS |
| 6 (architecture_is_subsystem_name_source_of_truth) | ✅ PASS |
| 7 (bc_h1_is_title_source_of_truth) | ✅ PASS |
| 8 (bc_array_changes_propagate_to_body_and_acs) | ⚠️ WARN (F-014, F-017 epic-body) |
| 9 (vp_index_is_vp_catalog_source_of_truth) | ❌ FAIL (F-013) |
| 10 (demo_evidence_story_scoped) | n/a |

## Observations

- **O-07 (new):** `additional_modules:` field added in F-007 not documented in L4-verification-property-template.md. Recommend appending schema note. Non-blocking.
- O-01..O-06 carried from pass-1.

## Novelty Assessment

| Field | Value |
|-------|-------|
| **Pass** | 2 |
| **New findings** | 5 (F-013..F-017) |
| **Duplicate/variant findings** | 0 |
| **Novelty score** | NITPICK |
| **Median severity** | MEDIUM |
| **Trajectory** | 12 → 5 |
| **Verdict** | FINDINGS_REMAIN |

Trajectory 12 → 5 is healthy convergence. After F-013..F-017 land, pass-3 should reach NITPICK (clean) and start the 3-pass convergence run.
