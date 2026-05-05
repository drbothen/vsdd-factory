# Adversarial Review — E-9 v1.16 Last-Mile Burst (D-256) — Pass 14

**Date:** 2026-05-05
**Commit reviewed:** f43f0f6 (v1.15 → v1.16)
**Cumulative surface:** v1.7..v1.16 (4 files + open-questions.md)
**Verdict:** SUBSTANTIVE
**ADR-013 clock at end of pass:** 0_of_3 reset
**Pass methodology angle:** AC chain audit + section-heading semantic anchor verification — every AC in E-9 (AC-1..AC-10), trace each to its measurement source / gate value / target artifact, and verify section-level semantic anchors in cross-doc references AC chain depends on. NEW per TD-VSDD-057.

## Summary

Verified AC-1..AC-10 traceability and v1.16 fix-burst citations. Numerical and citation integrity solid: AC-3 numerics (643686 = 321843 × 2), AC-8 block-mode list (5 hooks), open-questions.md line 20 → gap-analysis line 325, ADR-014 amendment title quote, ADR-015 line citations all PASS.

One semantic-anchor finding at section-heading level: perf-baseline-w16.md line 154 H2 carries "Option C" descriptor that does not resolve to any ADR-014 R-8.09 option taxonomy. Pass-8 corrected line 156 inline cite but didn't scrub the section heading itself. Residual from same outbound-decision-ID class. MED severity.

## Findings

### HIGH
None.

### MED

**M-P14-001 [MED] — Outbound mis-anchor at section-heading level: perf-baseline-w16.md line 154 H2 cites "Option C" but ADR-014 R-8.09 Revised has no Options A/B/C taxonomy**
- File: perf-baseline-w16.md line 154
- Cite: `## W-16 Gate Model (ADR-014 R-8.09 Revised — Option C)`
- Verification: grep "Option C" ADR-014 — only match is line 173 ("D-6 Option A") referencing write_file precedent, not R-8.09. ADR-014 lines 38-53 (R-8.09 Amendment) describe one revised decision (latency-primary + advisory + kill-switch) with no enumerated options.
- Same outbound-decision-ID class pass-8 closed for line 156. Pass-8 fixed in-section reference but not H2 heading.
- Story-writer reading section sees `R-8.09 Revised — Option C` and would search ADR-014 for non-existent "Option C" structure.
- Fix: drop `— Option C`. Replace with `## W-16 Gate Model (ADR-014 R-8.09 Amendment 2026-05-03)`.

### LOW

**L-P14-001 [LOW] — audit-w16.md line 165 cites "D-2 Option C" — possibly mis-anchored to E-8 D-2 (BC-anchor decision)**
- audit-w16.md line 165: `they considered port-as-is versus fix. They chose port-as-is (D-2 Option C)`
- "D-2 Option C" is E-8 D-2 (BC-anchor strategy). The W-15 OQ-001 "preserved bash quirk" decision is unrelated to BC-anchor strategy.
- May be intentional shorthand. Pending intent verification.

**L-P14-002 [LOW] — perf-baseline-w16.md frontmatter `producer: implementer` does not reflect architect amendments**
- File: perf-baseline-w16.md line 8: `producer: implementer`
- E-9 v1.8 changelog M-3 closure done by architect.
- The `producer` field could canonically reflect original author with amendments tracked elsewhere. Pending intent verification.

## Process-gaps

- [process-gap PG-P14-001] TD-VSDD-065 (decision-ID outbound semantic-anchor check) was applied at in-line-citation level by pass-8 + pass-13 but not at section-heading level. Pass-14 found "Option C" in H2 heading where section anchors to non-existent structure. Codification: extend TD-VSDD-065 scope from "in-text decision IDs" to "all section/subsection headings that name an external authority's decision/option/choice." File as TD-VSDD-070.

## Convention checks

- Frontmatter `version:` matches latest non-reserved row (1.16): PASS
- v1.7-v1.16 summary rows intact (POLICY 1): PASS
- v1.17 preemptive reserved row: PASS (line 479)
- v1.16 H3 section present: PASS (line 1028)
- v1.7-v1.15 block prose preserved as authored (POLICY 1 append-only): PASS
- No "Lines: X → Y" footer at v1.7-v1.16: PASS
- H3 version count matches summary table: PASS
- audit-w16.md line 38 sibling-wording-template consistency: PASS
- No fix-burst-internal IDs leak into permanent specs body: PASS
- Outbound decision-ID anchors semantically compatible (TD-VSDD-065): **FAIL** — perf-baseline-w16.md line 154 H2 "Option C" non-resolving (M-P14-001)
- Citation line numbers accurate (TD-VSDD-069): PASS

## Angle-specific outputs

AC chain audit: 10 ACs all traced to measurement sources / target artifacts. AC-3 numerics (643686 = 321843 × 2) verified. AC-8 block-mode list (5 hooks at hooks-registry.toml lines 231/291/471/774/794) verified. AC-3 ADR-014 amendment title quote `"R-8.09 ceiling model revised (research)"` matches ADR-014 line 38 exactly post-v1.16 fix.

Section-heading semantic-anchor verification: 1 FAIL (M-P14-001 "Option C") + 1 LOW pending (L-P14-001 "D-2 Option C") out of ~12 H2/H3 headings checked.

Numerical cross-anchor: 643686 = 321843 × 2 PASS; 23 hooks PASS; 5 block-mode PASS; 642.6ms p95 + 706.9ms threshold PASS; median 205160 PASS; 8549146 sum PASS.
