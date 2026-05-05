# Adversarial Review — E-9 v1.23 Sibling-Sweep Burst (D-265) — Pass 23

**Date:** 2026-05-05
**Commit reviewed:** 721d2eb (v1.22 → v1.23)
**Cumulative surface:** v1.7..v1.23 (4 files + open-questions.md + 2 BCs)
**Verdict:** NITPICK_ONLY
**ADR-013 clock at end of pass:** 0_of_3 → 1_of_3 advance
**Pass methodology angle:** Cross-pass-fix-history audit — read every v1.7..v1.23 H3 changelog section as a normative claim about artifact state, then verify each cited line/wording/file against current artifacts. ~85 normative H3 claims across 17 sections audited. Distinct from passes 1-22 (which audited artifact-state without re-validating H3 narrative accuracy). NEW per TD-VSDD-057.

## Summary

Verified all v1.23 fix-claims (H-P22-001 BC-1.05.036 sibling alignment at lines 61-62/86/97; H-P22-002 BC-1.05.035 Postcondition 4 INTERIM qualifier; M-P22-001 Postcondition 1 success-path scoping; M-P22-002 OQ-W16-001 acceptance (a) AND-link; M-P22-003 precedence ladder) against actual artifact state. All 5 fixes land correctly. ADR-015 D-15.2 registry line 329, Wave 3 AC-2 line 634, source-code error codes -2/-3 at mod.rs:181-182, gap-analysis line 326 anchor — all verified.

All convention checks PASS. v1.7..v1.22 historical changelog blocks preserved verbatim per POLICY 1. v1.23 H3 present, no footer, v1.24 reserved row present. No retired-figure residue in non-changelog body. No fabricated content in v1.23 H3.

Two LOW observations on H3 narrative-accuracy meta — neither is content defect; both are POLICY 1-locked historical artifacts that cannot be remediated.

## Findings

### HIGH
None.

### MED
None.

### LOW

**L-P23-001 [LOW]: v1.23 H3 Fix 1 narrative conflates v1.21+v1.22 history**
- E-9:1262: "v1.22 correctly updated Postcondition 5 to state TIMEOUT/OUTPUT_TOO_LARGE return error codes WITHOUT emitting any event."
- Actual: Postcondition 5's no-event semantics FIRST authored v1.21 (D-263 L-P20-002 line 1185 with fabricated -7/-8 codes); v1.22 (H-P21-001) corrected codes only.
- POLICY 1 immutable; cannot be remediated. Cosmetic narrative imprecision only.

**L-P23-002 [LOW]: v1.20 H3 5-element recurrence chain conflates 4 distinct convention questions**
- E-9:1153: "5th re-flag of the same convention question (M-P5-002 → M-P6-001 → L-P14-002 → L-P15-001 → M-P18-001)"
- Actual: each was a different frontmatter convention question (version/references/producer/last_amended).
- POLICY 1 immutable; S-7.02 threshold valid at meta-level (all four about arch-doc frontmatter convention).

## Out-of-scope-but-noted

None.

## Process-gaps

None.

## Convention checks (all PASS)

- Frontmatter `version:` matches latest non-reserved row (1.23): PASS
- Arch-doc-class files have `last_amended: 2026-05-05`: PASS
- BCs have `last_amended: 2026-05-05`: PASS
- BC-1.05.035 + BC-1.05.036 ADR-015 awareness clauses: PASS
- BC-1.05.036 error-path event reality consistent across §Postcondition 5 + §Related BCs + §EC-004: PASS
- BC-1.05.036 error codes -2/-3 match source code: PASS
- BC-1.05.035 Postcondition 4 INTERIM qualifier: PASS
- BC-1.05.035 precedence ladder: PASS
- OQ-W16-001 acceptance (a) AND-links registry + canonical name: PASS
- open-questions.md line 21 cites correct gap-analysis line: PASS
- v1.7-v1.23 summary rows intact (POLICY 1): PASS
- v1.24 preemptive reserved row: PASS (line 487)
- v1.23 H3 section present: PASS (line 1254)
- v1.7-v1.22 block prose preserved as authored (POLICY 1 append-only): PASS
- No "Lines: X → Y" footer at v1.7-v1.23: PASS
- H3 version count matches summary table: PASS (17 H3 sections v1.7-v1.23)
- audit-w16.md sibling-wording-template consistency: PASS
- No fix-burst-internal IDs leak into permanent specs body: PASS
- Outbound decision-ID anchors semantically compatible: PASS
- Citation line numbers accurate: PASS
- No retired-figure residue in non-changelog body: PASS

## Angle-specific outputs

Cross-pass-fix-history audit results: 17 H3 sections × ~5 average claims = ~85 normative claims audited. All fix-claims verified against current artifact state. The v1.21 → v1.22 → v1.23 chain illustrates the TD-VSDD-074 → TD-VSDD-075 → TD-VSDD-076 lessons building on each other. POLICY 1 retention of fabricated values in v1.21 H3 correctly handled with explicit historical-record notes in v1.22. No contradictions detected between H3 narrative claims and current artifact state.
