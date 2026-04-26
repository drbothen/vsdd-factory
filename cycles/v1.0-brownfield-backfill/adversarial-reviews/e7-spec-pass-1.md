---
document_type: adversarial-review-pass
pass: 1
scope: e7-spec
cycle: v1.0-brownfield-backfill
date: 2026-04-26
reviewer: adversary
status: MINOR
verdict: MINOR
novelty_score: MINOR
finding_count: 12
trajectory: "12 (initial)"
convergence: FINDINGS_REMAIN
artifacts_reviewed:
  bcs: [BC-5.36.001..007, BC-5.37.001..002, BC-7.05.001..003, BC-8.28.001..002, BC-9.02.001]
  vps: [VP-061, VP-062]
  frs: [FR-042]
  stories: [S-7.01, S-7.02]
  epics: [E-7]
---

# Adversarial Review — Pass 1 (E-7 Process Codification)

## Verdict

**MINOR** — Spec is in good shape. Story-writer and PO actively applied S-6.01 lessons (Capability Anchor Justification verbatim across all 15 BCs, frontmatter ↔ body bidirectional, ECs populated). 12 findings: 6 HIGH/MEDIUM fix-then-re-pass + 6 LOW + 6 non-blocking observations.

**The headline finding (F-003) is meta dogfood failure:** PRD body still cites `1,863-BC catalog` while the very BCs being added codify defensive sweep tooling to catch this drift class. State-manager swept BC-INDEX/VP-INDEX/ARCH-INDEX/STATE.md but missed PRD body — exactly the gap BC-5.37.001 + BC-7.05.001 are designed to prevent. The spec is failing its own meta-rule before delivery.

## High/Medium Findings (block convergence; need fix)

### F-001 — BC-9.02.001 mis-anchored to SS-09 but Architecture Module is owned by SS-07 (HIGH, semantic-anchoring)

**Artifact:** `.factory/specs/behavioral-contracts/ss-09/BC-9.02.001.md`
**Issue:** ARCH-INDEX SS-07 row owns `hooks-registry.toml`. SS-09 row does not. BC frontmatter says `subsystem: SS-09`. POLICY 6 violation. Story S-7.02 §subsystem justifications admits the contradiction.
**Fix:** Move BC to `ss-07/BC-7.05.004.md` (renumber, append-only within SS-07). Update BC-INDEX, S-7.02 frontmatter+body, FR-042, VP-062, BC-7.05.001 Related BCs, ARCH-INDEX SS-07/SS-09 counts (195→196 / 6→5).

### F-002 — VP-061 type drift: file says `invariant`, VP-INDEX says `safety` (HIGH, vp-index)

**Issue:** Property statement is presence-checking → naturally `invariant`. VP-INDEX line 113 has `safety`. POLICY 9 violation.
**Fix:** Update VP-INDEX line 113 type column to `invariant`.

### F-003 — PRD count drift: 1,863 BCs / 41 FRs everywhere; should be 1,878 / 42 (HIGH, propagation, **meta-dogfood-failure**)

**Issue:** PRD lines 38, 45, 79, 129, 489, 617, 645, 664, etc. all carry stale counts. The new BC-5.37.001 + BC-7.05.001 EXIST to catch exactly this. State-manager swept indexes but not PRD body.
**Fix:** Update PRD to 1,878 BCs / 42 FRs. SS-05 → 636, SS-07 → 195/196 (post-F-001), SS-08 → 217, SS-09 → 6/5 (post-F-001).

### F-004 — STATE.md story count stale: 41 vs 44; FR count 41 vs 42 (HIGH, propagation)

**Issue:** STATE.md line 59 says "41 FRs"; line 95 says story count = 41; lines 117-119 omit 3 ready stories.
**Fix:** Update STATE.md FR/story counts; add Ready (3) row.

### F-005 — VP-062 covers 8 BCs across 4 subsystems and 4 distinct deliverables (HIGH/MEDIUM, vp-multi-bc)

**Issue:** Source Contract bundles state-manager prompt + count-prop hook + template-compliance VP check + lessons rule + hooks-registry entry under one VP. Each is a separate proof concern.
**Fix:** Either split (VP-062..VP-065) or rename to "S-7.02 Process-Codification Surface Invariant" acknowledging multi-axis scope. Recommend split.

### F-006 — BC-9.02.001 EC-002 specifies a guessed regex format for hooks-registry.toml (HIGH/MEDIUM, structural)

**Issue:** EC-002's regex pattern may not match actual hooks-registry.toml schema. Implementer would need to read existing entries to verify.
**Fix:** Drop the regex string OR quote actual existing entry's file-matching syntax with proper escaping.

## Lower-Severity Findings (MEDIUM, fix before convergence)

### F-007 — VP-062 module field too narrow for cross-subsystem scope (MEDIUM)
Single `module:` points at count-prop hook only; harness covers 4 subsystems. Add `additional_modules:` array or split.

### F-008 — VP-061/062 inputs[] omits target agent files (MEDIUM)
VP-061 should list `agents/{story-writer,product-owner,adversary}.md` in inputs. VP-062 should list state-manager.md + count-prop hook + template-compliance + lessons-codification.md + hooks-registry.toml.

### F-009 — BC-5.36.005 EC-002 self-contradictory on "intentionally different sibling" (MEDIUM)
Adversary can't adjudicate intent. EC should clarify: detection role only; orchestrator/human adjudicates.

### F-010 — Story-writer Spec-First Gate's literal `[BC-TBD]` not blocked by Postcondition 1 (MEDIUM)
Postcondition checks "empty or absent"; `[BC-TBD]` is non-empty. Strengthen to require canonical BC-NNN pattern match.

### F-011 — BC-7.05.001 vs BC-7.05.003 exit-code inconsistency (MEDIUM)
Count-prop = exit 2; template-compliance VP check = exit 1. Reconcile per VP-022 dispatcher exit-code semantics.

### F-012 — BC-8.28.002 Architecture Module mis-anchors to lessons-codification.md (MEDIUM, semantic-anchoring)
This BC governs the orchestrator cycle-closing checklist; module should point at the orchestrator skill/prompt, not the rule file the checklist references.

## Observations (non-blocking)

- **O-01:** 200ms timing for count-prop hook lacks CI escape hatch — recommend EC for >1000ms PERFORMANCE WARNING.
- **O-02:** BC-7.05.002 EC-002 missing-file case has no test vector.
- **O-03:** S-7.02 has 15 tasks (above typical 8-12 cap); cluster decision was deliberate.
- **O-04:** All 15 BCs say `L2 Domain Invariants | none` — could plausibly trace to DI-014 or new DI-018/019 (governance invariants). Recommend domain-spec review.
- **O-05:** VP-061 proof harness uses `git diff HEAD~1` which fails on first commit; recommend branch-aware diff.
- **O-06:** BC-5.37.001 EC-001 doesn't handle historical-context preambles in changelogs — recommend `previously had`/`as of`/`historical:` skip rule.

## Coverage Assessment

All 15 BCs read in full. All 2 VPs read in full. PRD §FR-042 + counts at 6 sites read. S-7.01, S-7.02, E-7 read in full. Cross-references verified: BC-INDEX, VP-INDEX, ARCH-INDEX, STORY-INDEX, STATE.md, capabilities.md.

## Policy Compliance

| Policy | Status |
|--------|--------|
| 1 (append_only_numbering) | ✅ PASS |
| 2 (lift_invariants_to_bcs) | ⚠️ WARN (O-04) |
| 3 (state_manager_runs_last) | n/a (delivery-phase) |
| 4 (semantic_anchoring_integrity) | ❌ FAIL (F-001, F-012) |
| 5 (creators_justify_anchors) | ✅ PASS (verbatim CAP-001 across all 15) |
| 6 (architecture_is_subsystem_name_source_of_truth) | ❌ FAIL (F-001) |
| 7 (bc_h1_is_title_source_of_truth) | ✅ PASS |
| 8 (bc_array_changes_propagate_to_body_and_acs) | ✅ PASS |
| 9 (vp_index_is_vp_catalog_source_of_truth) | ❌ FAIL (F-002) |
| 10 (demo_evidence_story_scoped) | n/a (spec-only) |

## Novelty Assessment

| Field | Value |
|-------|-------|
| **Pass** | 1 |
| **New findings** | 12 |
| **Duplicate/variant findings** | 0 (this is pass 1) |
| **Novelty score** | MINOR |
| **Median severity** | MEDIUM |
| **Trajectory** | 12 (initial) |
| **Verdict** | FINDINGS_REMAIN |

## Adversary Notes

S-6.01 lessons applied successfully — the calibration prediction was correct (MINOR, not MAJOR). The major value pass-1 added: catching the meta dogfood failure in F-003 (PRD count drift not swept) and F-001 (subsystem mis-anchor).

**Recommended pass-1-fix order:**
1. F-001 BC renumber (largest blast radius, do first to avoid 2 sweeps)
2. F-002 VP-061 type
3. F-003 + F-004 PRD/STATE count drift (after F-001 lands; corpus sweep)
4. F-012 BC-8.28.002 module (orchestrator skill identification needed)
5. F-009..F-011 BC textual fixes
6. F-005, F-007, F-008 VP refinements
