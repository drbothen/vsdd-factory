# Adversarial Review — S-5.05 Pass 1 (Wave 14)

**Reviewer:** adversary
**Artifact:** `.factory/stories/S-5.05-migration-guide.md` v1.4 (foundation burst commit 60038e9 on factory-artifacts)
**Date:** 2026-04-29

## Critical Findings
(None)

## Important Findings (MEDIUM)

### F-S5.05-P01-001 — MED — S-5.07 vs S-5.7 reference inconsistency
Story uses canonical `S-5.07` in frontmatter `blocks:` and body `**Blocks:**` line, but legacy short-form `S-5.7` in narrative (lines 50, 98, 112, 164, 182). STORY-INDEX uses canonical S-5.07. v1.0-index.md L41 uses legacy "S-5.7" (skeleton's choice). Story should use canonical except when literally quoting skeleton. Also line 164 task 9 uses `S-2.7` (should be `S-2.07`).
Policy: POLICY 1, POLICY 7 (story-id level).

### F-S5.05-P01-002 — MED — Architecture Compliance Rule mislabels BC-8.26.006 as "section-completeness"
Line 184: `BC-8.26.006 (section-completeness)` for the human-review rule. BC-8.26.006's H1 is the "user-facing-docs deliverable covers what-it-is/install/quickstart/..." rule — does NOT contract human-reviewer-qualification. AC-9's BC trace already correctly tags `[process-gap]`.
Policy: POLICY 7 (BC H1 source-of-truth) + POLICY 4 (semantic anchoring integrity).

### F-S5.05-P01-003 — MED — Edge Case EC-001 (custom hooks.json entries) has no AC, Task, or section coverage
Lines 130-133: EC-001 about custom hooks.json migration has no matching AC entry or Task entry. Skeleton has no section about it.
Policy: POLICY 4 (semantic anchoring — Edge Cases declared but uncovered by ACs is coverage gap).

### F-S5.05-P01-004 — MED — STORY-INDEX shows S-4.08 still depends on S-5.05 retroactively
STORY-INDEX line 96: S-4.08 status `done` but Depends On still includes `S-5.05`. v1.4 story changelog claims S-4.08 removed from S-5.05 blocks (saying "S-4.08 merged"). But S-4.08's `depends_on: S-5.05` reflects rc.1 RELEASE readiness (S-4.08 spec merged but rc.1 release event not yet happened). The v1.4 burst applied a wrong correction — S-5.05 still blocks rc.1.
Policy: POLICY 3 (state_manager_runs_last — bidirectional dep cleanup must propagate same-burst); POLICY 8 spirit.

### F-S5.05-P01-005 — MED (pending intent) — Wave field discrepancy: frontmatter wave: 16, STORY-INDEX/cycle dir wave 14
Story frontmatter line 24 `wave: 16`, but Wave 14 burst made the v1.3→v1.4 update (cycle dir says wave-14). Spec gives no guidance on whether `wave:` = ship wave vs burst sequence.
Policy: POLICY 4 (semantic anchoring — wave field semantics ambiguous).

### F-S5.05-P01-006 — MED (pending intent) — AC-7 threshold "5 common issues" exceeds skeleton hint of 4 examples
Story AC-7 demands ≥5 troubleshooting issues; skeleton TODO comment lists 4 examples; Task 11 lists 5 (adding "Windows git-bash" not in skeleton). Implementer reading just AC + skeleton might ship 4.
Policy: POLICY 5 (creators_justify_anchors — content thresholds against source-of-truth).

## Observations (LOW)

### F-S5.05-P01-007 — LOW — README links migrating-from-0.79.md as "finalized in S-5.5" (legacy short-form)
README L264 description text becomes stale after S-5.05 ships. No Task to update.

### F-S5.05-P01-008 — LOW — Token Budget estimate likely undercounts
Story 204 lines (~2,500-3,000 tokens), skeleton 169 lines (~2,000-2,500 tokens). "~900 + ~500 = ~1,600" is wrong; closer to ~5,000.

### F-S5.05-P01-009 — LOW [process-gap] — Orchestrator brief said "14 TODO blocks" but actual is 10
Story body correctly cites 10. Brief math (14+2+1=17 ≠ 15) inconsistent. Process-gap not story defect.

### F-S5.05-P01-010 — LOW — F-204 cite has no path
Line 63 cites "Wave 7 F-204 cross-wave-complementary anchor pattern" without a sanctioning artifact link.

## Verdict
`VERDICT: SUBSTANTIVE` — fix burst required.
`CRIT=0 HIGH=0 MED=6 LOW=4 NIT=0`
