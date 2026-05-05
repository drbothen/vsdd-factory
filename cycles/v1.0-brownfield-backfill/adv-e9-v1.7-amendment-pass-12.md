# Adversarial Review — E-9 v1.14 Combined Burst (D-254) — Pass 12

**Date:** 2026-05-05
**Commit reviewed:** 3d891c6 (v1.13 → v1.14)
**Cumulative surface:** v1.7..v1.14 (4 files + open-questions.md)
**Verdict:** SUBSTANTIVE
**ADR-013 clock at end of pass:** 0_of_3 reset
**Pass methodology angle:** v1.14 diff-only line-by-line + AC-3 sibling-style audit (hybrid; NEW per TD-VSDD-057)

## Summary

Pass-12 attacked the v1.14 burst surgically. Numerical re-anchor in AC-3 is correct (643686 verifies against perf-baseline:163). However the M-P11-001 nomenclature scrub of open-questions.md line 20 introduced a NEW fix-burst-internal token (`M-1 closure`) that does not resolve in the named target document — exactly the class M-P11-001 was supposed to fix. AC-3 fix introduced 3 structural defects (non-canonical ADR-014 label; pseudo-code vs prose-form sibling style; comma byte-formatting).

## Findings

### HIGH

**H-P12-001 [HIGH]: open-questions.md line 20 reintroduces fix-burst-internal nomenclature; forward-pointer is unresolvable**
- File: open-questions.md line 20: `**Source:** gap-analysis-w16-subprocess.md §"How ADR-015 affects the telemetry gap" (M-1 closure forward-pointer to OQ-W16-001)`
- "M-1 closure" is fix-burst-internal nomenclature (refers to E-9 changelog v1.8 H3 closure entry M-1). Same character as `D-247`, `M-P6-002`, `pass-6 finding`, SHA `b04843d` that M-P11-001 just removed from this exact line. v1.14 closure replaced one set of fix-burst tokens with another. TD-VSDD-063 + TD-VSDD-066 cited as rationale but the scrub failed to apply its own rule.
- Grep `gap-analysis-w16-subprocess.md` for "M-1": zero matches. Forward-pointer unresolvable.

### MED

**M-P12-001 [MED]: AC-3 v1.14 parenthetical introduces non-canonical "ADR-014 R-8.09 Amendment" label**
- File: E-9 epic line 368 cite `... 643,686 bytes per ADR-014 R-8.09 Amendment) ...`
- ADR-014's actual H2 label is `## Amendment 2026-05-03: R-8.09 ceiling model revised (research)` (line 38). Non-canonical inversion + drops `2026-05-03` date that disambiguates from second ADR-014 amendment (`Amendment 2026-05-03: D-9.2 withdrawn (gap analysis)` at line 13).

**M-P12-002 [MED]: AC-3 v1.14 parenthetical structurally diverges from sibling-AC parenthetical style**
- AC-3 v1.14: `(soft_cap = perf-baseline-w16.md w16_advisory_bundle_soft_cap_bytes = 643,686 bytes per ADR-014 R-8.09 Amendment)` — pseudo-code form. Sibling ACs use prose: `(hard gate, inherited from S-9.00 / E-8 AC-7b)`.

**M-P12-003 [MED]: AC-3 byte-count formatting inconsistent with source field**
- AC-3 `643,686 bytes` (comma) vs perf-baseline-w16.md:163 `643686` (no comma). Numerically equivalent, mechanically different.

### LOW

**L-P12-001 [LOW]: open-questions.md retains version-internal "E-9 v1.10 amendment" reference in body**
- Line 25 (Question prose): `E-9 v1.10 amendment proposes...`. Same TD-VSDD-066 class as M-P11-001 but in Question field instead of Source. Pending intent verification.

## Process-gaps

- **[process-gap PG-P12-001]** TD-VSDD-063 / TD-VSDD-066 enforcement gap: scrub-rule says "remove fix-burst-internal IDs" but no automated check that the *replacement* text is also free of such IDs. M-P11-001 closed by replacing one set of internal IDs with another. Hook needs a recursive-scrub check: re-grep for forbidden pattern set after replacement. File as TD-VSDD-068.

## Convention checks

- Frontmatter `version:` matches latest non-reserved row (1.14): PASS
- v1.7-v1.14 summary rows intact (POLICY 1): PASS
- v1.15 preemptive reserved row: PASS
- v1.14 H3 section present: PASS (line 974)
- v1.7-v1.13 block prose preserved as authored (POLICY 1 append-only): PASS
- No "Lines: X → Y" footer at v1.7-v1.14: PASS
- H3 version count matches summary table: PASS
- audit-w16.md line 38 sibling-wording-template consistency: PASS
- No fix-burst-internal IDs leak into permanent specs body: **FAIL** (open-questions.md line 20 contains `M-1 closure` — see H-P12-001)
- Outbound decision-ID anchors semantically compatible (TD-VSDD-065): MIXED (perf-baseline:156 anchor correct; AC-3 internal anchor non-canonical per M-P12-001)
- AC-3 numerical-cross-anchor (643686 bytes matches perf-baseline source): PARTIAL (numerically PASS; byte-formatting FAIL — M-P12-003)

## Angle-specific outputs

v1.14 diff-only line-by-line:
- Fix 1 (AC-3 line 368): 3 new defects (M-P12-001 non-canonical label; M-P12-002 sibling-style divergence; M-P12-003 comma formatting). Numerical PASS; structural FAIL.
- Fix 2 (open-questions.md line 20): 1 new defect (H-P12-001 fix-burst-internal nomenclature reintroduced + unresolvable forward-pointer). Stated success criterion FAIL.
- v1.14 H3 (E-9 lines 974-990): structurally clean.

AC self-consistency: AC-1..AC-10 all internally well-formed except AC-3, which now has 3 fresh structural inconsistencies introduced by Fix 1.
