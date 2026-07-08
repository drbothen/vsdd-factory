# Adversarial Review — E-19 Pass 25 (post-D-778 delta; perimeter = BC-2.02.011 v1.5 + S-19.01 v1.14 + S-19.03 v1.13 + S-19.07 v1.9)

**Perimeter:** E-19 epic + S-19.01..S-19.07 + STORY-INDEX E-19 section + ADR-030 v1.3 + BC-5.42.001 v1.3 + BC-2.02.011 v1.5 + BC-4.13.001 v1.9 + BC-1.17.001 v1.3
**Reviewer:** fresh-context adversary (Iron Law; rubric = policies.yaml v1.4.1)
**Date:** 2026-07-08
**Verdict:** NOT-CLEAN — BLOCKER 0 / HIGH 1 / MEDIUM 0 / LOW 1 (2 items: 1 finding + 1 observation)
**Streak:** 0/3 RESET (streak was 0/3 entering pass-25 per D-778; severity floor decaying: 4→3→4→2 across passes 22-25)
**Model family:** Claude Opus 4.7
**Delta artifact versions verified:** BC-2.02.011 v1.5 (was v1.4); S-19.01 v1.14 (was v1.13); S-19.03 v1.13 (was v1.12).

## Part A — D-778 Delta Verification + New Findings

### Amendment 1 — BC-2.02.011 v1.4 → v1.5 (§Traceability bidirectional parity: S-19.03 added)

§Traceability §Stories subsection now lists both S-8.10 and S-19.03 ✓. §Story Anchor subsection notes S-19.03 AC-006 as extension ✓. §Refactoring Notes names S-19.03 as dependent sweep target ✓. Traceability-only amendment; no behavioral content changed ✓. POLICY 14 5-leg parity confirmed ✓.

New finding identified on BC-2.02.011 v1.5: NONE. §Traceability repair complete and correct.

### Amendment 2 — S-19.01 v1.13 → v1.14 (stale "or new hook" disjunction retired)

Stale "or new hook" disjunction in §Red Gate Tests retired ✓. Extend-only wording confirmed consistent with D-f adjudication and E-19 wave schedule ✓. Input-hash 358f3e2 unchanged ✓. POLICY 14 5-leg parity ✓.

No new findings on S-19.01 v1.14.

### Amendment 3 — S-19.03 v1.12 → v1.13 (BC-2.02.011 v1.5 cite sweep)

BC-2.02.011 cite sweep confirmed at Token Budget and AC traceability. Input-hash 8d1225d ✓. POLICY 14 5-leg parity ✓.

**F-P25-001 HIGH — S-19.03 §Behavioral Contracts table Version cell cites BC-2.02.011 v1.4 (stale; should be v1.5).**

S-19.03 v1.13 performs BC-2.02.011 version cite sweep at Token Budget and AC traceability. However, the §Behavioral Contracts body table — the inline table listing each BC with its current version — still carries `v1.4` in the Version column for BC-2.02.011. The D-778 sweep updated the footer (line 702 BC coverage citation) and the Token Budget row, but the body BC-table Version cell was not included in the predicate. This is a pass-24 partial-sweep escape: the sweep was substantively correct at 15+ sites, but the body table Version cell is a distinct cite location that requires explicit inclusion.

Evidence: S-19.03 §Behavioral Contracts table contains a row `| BC-2.02.011 | ... | v1.4 | ...` while BC-2.02.011 is at v1.5 after the D-778 PO fix burst.

**Locus:** S-19.03 §Behavioral Contracts body table, BC-2.02.011 row, Version cell.
**Routing:** story-writer.
**Fix:** S-19.03 v1.13→v1.14: BC-2.02.011 Version cell in body BC-table: `v1.4` → `v1.5`. Whole-file predicate re-run. Input-hash unchanged (Version cell edit is a cite update, not content change — same functional content, one cite string updated). POLICY 14 5-leg parity applied.

### Full E-19 Epic and Story Suite Review

No additional findings on S-19.02, S-19.04, S-19.05, S-19.06, ADR-030 v1.3, BC-4.13.001 v1.9, BC-5.42.001 v1.3, BC-1.17.001 v1.3, or E-19 epic v1.16.

## Observations

**O-P25-001 [LOW; spec-hygiene] — S-19.07 contains three "shipped" present-perfect tense sites that imply the story has already delivered, contradicting pending-merge status.**

S-19.07 v1.9 contains prose phrasing such as "has been migrated", "has been removed", and "is now using" (or equivalent shipped-tense constructions) in narrative sections describing the migration outcome. Per BC-4.13.001 v1.9 tense convention (normative ACs/Invariants use present-tense imperative), these past-tense/present-perfect constructions in story prose imply completion rather than pending-merge specification. Three distinct sites carry this pattern.

This is the same tense-class finding as F-P23-002 (BC-4.13.001 v1.8 tense; fixed D-777) but in story narrative prose rather than BC normative text. The human production-grade directive (D-778 precedent for O-P24-001/002: "fix-not-accept") applies.

**Routing:** story-writer.
**Fix:** S-19.07 v1.9→v1.10: align 3 "shipped" tense sites to pending-merge conditional phrasing consistent with BC-4.13.001 v1.9 tense class. Input-hash unchanged (tense-only, no behavioral content changed). POLICY 14 5-leg parity applied.

## Part B — Dimensions

| Dimension | Status |
|-----------|--------|
| Dim-1: BC/VP coverage | PASS — BC-2.02.011 §Traceability now bidirectionally consistent after D-778 fix |
| Dim-2: AC gate execution | Not re-run (frozen stories for pass-25 delta; no gate changes in D-778 delta) |
| Dim-3: POLICY 14 5-leg parity | PASS on all 3 D-778 delta amendments |
| Dim-4: POLICY 8 BC propagation | PASS |
| Dim-5: Input-hash consistency | PASS — S-19.03 8d1225d, S-19.07 01bed1d match post-D-778 compute |
| Dim-6: STORY-INDEX BC-coverage | PARTIAL — F-P25-001 requires S-19.03 body BC-table Version cell update |
| Dim-7: ADR/BC interface parity | PASS (ADR-030 v1.3 canonical TOML shape correct per D-777) |

## Summary

| Severity | Count |
|----------|-------|
| BLOCKER | 0 |
| HIGH | 1 |
| MEDIUM | 0 |
| LOW | 1 (observation) |
| Observations | 1 |

**Severity floor decaying.** Trajectory (pass count): pass-22 B1/H1/M2/L0=4 → pass-23 B0/H1/M2/L0=3 → pass-24 B0/H0/M2/L2=4 → pass-25 B0/H1/M0/L1=2. Package is one edit from CLEAN: single partial-sweep escape (body BC-table Version cell) + tense residual (3 prose sites). D-779 fix burst applied (SW S-19.03 v1.14 + SW S-19.07 v1.10; SM STORY-INDEX v4.157 + governance).

**Overall Assessment:** NOT-CLEAN — B0/H1/M0/L1. Streak 0/3. NEXT: pass-26.

## Novelty Assessment

| Field | Value |
|-------|-------|
| Pass | 25 |
| New findings | 1 |
| New observations | 1 |
| Duplicate/variant findings | 0 |
| Novelty score | LOW — "package is one edit from CLEAN"; pass-24 sweep substantively correct across 15+ sites, one cell escaped |
| Median severity | HIGH (finding); LOW (observation) |
| Verdict | NOT-CLEAN — streak 0/3; pass-26 NEXT |

## Coverage Attestation

Artifacts read in full: BC-2.02.011 v1.5 (1-end); BC-4.13.001 v1.9 (1-end); S-19.01 v1.14 (1-end); S-19.03 v1.13 (1-end); S-19.07 v1.9 (1-end); STORY-INDEX E-19 section (680-710).
Spot-checked (no changes): ADR-030 v1.3; BC-5.42.001 v1.3; BC-1.17.001 v1.3; S-19.02 v1.10; S-19.04 v1.11; S-19.05 v1.13; S-19.06 v1.14; E-19 epic v1.16.
Not read (Iron Law): decision-log; burst-log; lessons.md; STATE.md; checkpoints; adv passes 1-24; fix-burst records.

## Fix Burst Closure (D-779)

**Leg 1 — Story-writer (S-19.03):** S-19.03 v1.13→v1.14. BC-2.02.011 Version cell in body BC-table updated v1.4→v1.5. Whole-file predicate re-run confirmed no additional stale v1.4 cites. Input-hash 8d1225d unchanged (cite-only update; no content change). POLICY 14 5-leg parity applied. **CLOSED F-P25-001.**

**Leg 2 — Story-writer (S-19.07):** S-19.07 v1.9→v1.10. Three "shipped" tense sites aligned to pending-merge conditional phrasing per BC-4.13.001 v1.9 tense class. Human-directed fix-not-accept per production-grade default. Input-hash 01bed1d unchanged (tense-only; no behavioral content changed). POLICY 14 5-leg parity applied. **CLOSED O-P25-001.**

**Leg 3 — State-manager:** STORY-INDEX v4.156→v4.157: S-19.03 row v1.13→v1.14 (input-hash 8d1225d unchanged); S-19.07 row v1.9→v1.10 (input-hash 01bed1d unchanged); frontmatter last_amended prepended. BC-INDEX v3.80 UNCHANGED. VP-INDEX v2.53 UNCHANGED. ARCH-INDEX v2.94 UNCHANGED. Governance-lean burst (no BC/ADR legs).

4-index after D-779: BC-INDEX v3.80 UNCHANGED / VP-INDEX v2.53 UNCHANGED / STORY-INDEX v4.157 / ARCH-INDEX v2.94 UNCHANGED. Streak 0/3. **NEXT: pass-26.**
