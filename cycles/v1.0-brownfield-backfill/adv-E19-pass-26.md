# Adversarial Review — E-19 Pass 26 (post-D-779 delta; perimeter = S-19.03 v1.14 + S-19.07 v1.10)

**Perimeter:** E-19 epic + S-19.01..S-19.07 + STORY-INDEX E-19 section + ADR-030 v1.3 + BC-5.42.001 v1.3 + BC-2.02.011 v1.5 + BC-4.13.001 v1.9 + BC-1.17.001 v1.3
**Reviewer:** fresh-context adversary (Iron Law; rubric = policies.yaml v1.4.1)
**Date:** 2026-07-08
**Verdict:** NOT-CLEAN — BLOCKER 0 / HIGH 0 / MEDIUM 1 / LOW 1 (2 items: 1 finding + 1 finding-class observation)
**Streak:** 0/3 RESET (streak was 0/3 entering pass-26 per D-779; severity floor: 4→3→4→2→2 across passes 22-26)
**Model family:** Claude Opus 4.7
**Delta artifact versions verified:** S-19.03 v1.14 (was v1.13); S-19.07 v1.10 (was v1.9).

## Part A — D-779 Delta Verification + New Findings

### Amendment 1 — S-19.03 v1.13 → v1.14 (BC-2.02.011 body BC-table Version cell v1.4→v1.5)

BC-2.02.011 Version cell in §Behavioral Contracts body table now reads v1.5 ✓. Token Budget row already carried v1.5 from D-778 ✓. Whole-file predicate re-run in D-779 confirmed zero stale v1.4 cites ✓. Input-hash 8d1225d unchanged ✓. POLICY 14 5-leg parity confirmed ✓.

No new findings on S-19.03 v1.14.

### Amendment 2 — S-19.07 v1.9 → v1.10 (3 "shipped" tense sites aligned)

Three present-perfect tense sites aligned to pending-merge conditional phrasing ✓. Tense-only change; no behavioral content altered ✓. Input-hash 01bed1d unchanged ✓. POLICY 14 5-leg parity confirmed ✓.

**F-P26-002 LOW — S-19.07 retains "already works" / "correct and operational" rhetorical framing implying Phase-B is delivered.**

S-19.07 v1.10, while correctly removing the D-779 "shipped" tense sites, retains additional phrases in narrative sections that carry a delivered-state rhetorical frame: constructions such as "already works" (describing the read_prefix path as currently operational) and "correct and operational" (describing the resulting behavior as present-tense fact rather than post-merge specification). These are distinct from the three "shipped" sites fixed in D-779 — they use different phrasing patterns (declarative present-tense affirmatives describing a state that will only be true post-merge, rather than explicit present-perfect constructions).

The finding was escalated to the orchestrator for adjudication because "already works" touches both tense convention and could be read as an architecture statement (the read_prefix path is correct per ADR-025). Orchestrator ruling: tense class, not architecture — the claim is factually correct about the intended design but uses wrong temporal framing for a pending-merge story. Fix: convert to design-tense / merge-conditional.

**Locus:** S-19.07 narrative prose, ≥2 sites using "already works" / "correct and operational" or equivalent declarative present-tense delivered framing.
**Routing:** story-writer.
**Fix:** S-19.07 v1.10→v1.11: convert rhetorical delivered-frame phrases to design-tense / merge-conditional per BC-4.13.001 v1.9 tense class. Whole-file tense predicate run. Input-hash unchanged (tense-only; no behavioral content changed). POLICY 14 5-leg parity applied.

### Full E-19 Epic and Story Suite Review

**F-P26-001 MEDIUM — S-19.02 §Narrative contains a "Phase-A is complete" present-tense assertion implying Phase-A has already shipped.**

S-19.02 v1.10 §Narrative (or equivalent preamble section) contains a sentence asserting that "Phase-A is complete" or "Phase-A has been completed" as a current-state fact. This is a pending-merge story — Phase-A will be complete when this story merges (i.e., when the implementation lands); it is not complete in any production build prior to that merge. The assertion uses present-perfect/present-tense construction that contradicts the story's draft/pending-merge status and the tense convention established by BC-4.13.001 v1.9.

This is the same tense-class issue as F-P23-002 (fixed D-777) and O-P25-001 (fixed D-779) but in S-19.02 rather than S-19.07. The sibling sweep conducted in D-779 covered S-19.07 tense residuals; S-19.02 was spot-checked with a narrower predicate that did not catch this site.

**Locus:** S-19.02 §Narrative (or preamble), "Phase-A is complete" assertion.
**Routing:** story-writer.
**Fix:** S-19.02 v1.10→v1.11: convert "Phase-A is complete" to merge-conditional form ("Phase-A of BC-4.13.001 will be complete when this story merges" or equivalent). Whole-file tense predicate run to confirm no additional present-tense delivered-state assertions. Input-hash unchanged (tense-only; no behavioral content changed). POLICY 14 5-leg parity applied.

No additional findings on S-19.01 v1.14, S-19.04 v1.11, S-19.05 v1.13, S-19.06 v1.14, ADR-030 v1.3, BC-4.13.001 v1.9, BC-5.42.001 v1.3, BC-1.17.001 v1.3, BC-2.02.011 v1.5, or E-19 epic v1.16.

## Observations

**O-P26-001 [LOW; housekeeping] — input-hash "[pending-recompute]" placeholder in 3 draft BCs (BC-5.42.001, BC-2.07.001, BC-1.17.001).**

Three draft behavioral contracts authored during the E-19 cascade carry `input-hash: "[pending-recompute]"` in their frontmatter rather than a computed 7-char hash value. This placeholder pattern appears systematic across the BC corpus for newly-authored draft BCs (the compute-input-hash tool requires the BC's declared inputs to resolve before a hash can be produced; newly-authored draft BCs with forward-referencing inputs may not yet have resolvable input artifacts).

This observation is not novel: the "[pending-recompute]" convention has appeared in multiple prior passes without finding status. It is established corpus convention for draft BCs awaiting implementation. Not a POLICY 14 violation (input-hash is only mandatory-computed on story files per current convention). Do NOT recompute in this burst.

**Routing:** ACCEPTED-WITH-RECORD. Record in D-780 decision log. Confirm one-line in decision log that "[pending-recompute]" is the established draft-BC convention. No file edits required this burst.

**O-P26-002 [LOW; defensibility note] — S-19.07 deferral-gate grep cites only S-19.06 as prerequisite, not S-19.02.**

S-19.07's deferral-gate grep pattern (in the story's Architecture Compliance Rules or equivalent section) references only S-19.06 as the prerequisite merge condition (physical dependency: read_prefix FFI boundary must land first). S-19.02 is also a logical prerequisite (Phase-A byte-budget expansion must land before Phase-B migration removes the old path). The deferral gate does not check for S-19.02 merge.

Defensibility: S-19.02 appears in `depends_on: [S-19.02, S-19.06]` frontmatter, which is the authoritative dependency declaration. The deferral-gate grep targets the physical merge-order constraint (S-19.06 FFI). The logical constraint (S-19.02 byte budget) is captured in frontmatter and story dependency graph. Omitting S-19.02 from the gate grep is defensible as a scope boundary: the gate verifies the physical prerequisite; the logical prerequisite is enforced by the wave schedule (both S-19.02 and S-19.06 are W1/W2 before S-19.07's W3).

**Routing:** ACCEPTED-WITH-RECORD. Record in D-780 decision log. No file edits required.

## Part B — Dimensions

| Dimension | Status |
|-----------|--------|
| Dim-1: BC/VP coverage | PASS — BC-4.13.001 v1.9 tense class consistently applied to F-P26-001/F-P26-002 |
| Dim-2: AC gate execution | Not re-run (frozen stories for pass-26 delta; no gate changes in D-779 delta) |
| Dim-3: POLICY 14 5-leg parity | PASS on both D-779 delta amendments (S-19.03 v1.14/S-19.07 v1.10) |
| Dim-4: POLICY 8 BC propagation | PASS — no BC amendments in D-779 delta |
| Dim-5: Input-hash consistency | PASS — S-19.03 8d1225d, S-19.07 01bed1d match expected values; S-19.02 ccd11cf PASS |
| Dim-6: STORY-INDEX BC-coverage | PASS — STORY-INDEX v4.157 row versions match frontmatter at time of review |
| Dim-7: ADR/BC interface parity | PASS (ADR-030 v1.3 canonical TOML shape correct per D-777) |

## Summary

| Severity | Count |
|----------|-------|
| BLOCKER | 0 |
| HIGH | 0 |
| MEDIUM | 1 |
| LOW | 1 |
| Observations | 2 |

**Severity floor stable at 2 items.** Trajectory (pass count): pass-22 B1/H1/M2/L0=4 → pass-23 B0/H1/M2/L0=3 → pass-24 B0/H0/M2/L2=4 → pass-25 B0/H1/M0/L1=2 → pass-26 B0/H0/M1/L1=2. Package holds at 2 items; zero HIGH for two consecutive passes (pass-25 had 1 HIGH finding; pass-26 is first pass with zero HIGH and zero BLOCKER). Residual class: sibling tense propagation (same BC-4.13.001 v1.9 tense convention, different story/phrasing pattern each pass). D-780 fix burst applied (SW S-19.02 v1.11 F-P26-001 + SW S-19.07 v1.11 F-P26-002; SM STORY-INDEX v4.158).

**Overall Assessment:** NOT-CLEAN — B0/H0/M1/L1. Streak 0/3. NEXT: pass-27.

## Novelty Assessment

| Field | Value |
|-------|-------|
| Pass | 26 |
| New findings | 2 |
| New observations | 2 |
| Duplicate/variant findings | 0 (same tense class; different story/phrasing pattern — counted as new per class-distinct detection) |
| Novelty score | LOW — residual sibling tense propagation only; BC-4.13.001 v1.9 tense class finding in a different story (S-19.02) and different phrasing pattern in S-19.07 |
| Median severity | MEDIUM (F-P26-001); LOW (F-P26-002) |
| Verdict | NOT-CLEAN — streak 0/3; pass-27 NEXT |

## Coverage Attestation

Artifacts read in full: BC-4.13.001 v1.9 (1-end); S-19.02 v1.10 (1-end); S-19.03 v1.14 (1-end); S-19.07 v1.10 (1-end); STORY-INDEX E-19 section (680-710).
Spot-checked (no changes): ADR-030 v1.3; BC-5.42.001 v1.3; BC-2.02.011 v1.5; BC-1.17.001 v1.3; S-19.01 v1.14; S-19.04 v1.11; S-19.05 v1.13; S-19.06 v1.14; E-19 epic v1.16.
Not read (Iron Law): decision-log; burst-log; lessons.md; STATE.md; checkpoints; adv passes 1-25; fix-burst records.

## Fix Burst Closure (D-780)

**Leg 1 — Story-writer (S-19.02):** S-19.02 v1.10→v1.11. Phase-A-complete assertion converted to merge-conditional tense ("Phase-A of BC-4.13.001 will be complete when this story merges"). Whole-file tense predicate run confirmed no additional delivered-state assertions. Input-hash ccd11cf unchanged (tense-only update; no content change). POLICY 14 5-leg parity applied. **CLOSED F-P26-001.**

**Leg 2 — Story-writer (S-19.07):** S-19.07 v1.10→v1.11. "Already works" / "correct and operational" rhetorical delivered-frame converted to design-tense / merge-conditional per BC-4.13.001 v1.9 tense class. Whole-file tense predicate run confirmed. Input-hash 01bed1d unchanged (tense-only update; no content change). POLICY 14 5-leg parity applied. **CLOSED F-P26-002.**

**Leg 3 — State-manager:** STORY-INDEX v4.157→v4.158 (S-19.02 row v1.10→v1.11 ccd11cf; S-19.07 row v1.10→v1.11 01bed1d; frontmatter last_amended prepended). BC-INDEX v3.80 UNCHANGED. VP-INDEX v2.53 UNCHANGED. ARCH-INDEX v2.94 UNCHANGED.

**O-P26-001:** ACCEPTED-WITH-RECORD per orchestrator adjudication. "[pending-recompute]" is established draft-BC convention. Recorded in D-780 decision log. No file edits.

**O-P26-002:** ACCEPTED-WITH-RECORD per orchestrator adjudication. Deferral-gate physical-prerequisite scope boundary is defensible. Recorded in D-780 decision log. No file edits.
