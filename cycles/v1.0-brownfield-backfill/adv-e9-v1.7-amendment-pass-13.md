# Adversarial Review — E-9 v1.15 Combined Burst (D-255) — Pass 13

**Date:** 2026-05-05
**Commit reviewed:** 7b48031 (v1.14 → v1.15)
**Cumulative surface:** v1.7..v1.15 (4 files + open-questions.md)
**Verdict:** SUBSTANTIVE
**ADR-013 clock at end of pass:** 0_of_3 reset
**Pass methodology angle:** Outbound decision-ID exhaustive enumeration — enumerated every decision-ID-style anchor (ADR-NNN D-NN.x, R-W16-NNN, R-8.NN, AC-N, BC-S.SS.NNN, S-N.NN, OQ-NNN, line citations) across all 5 in-scope files. Distinct from pass-8 (single-axis spot-check) and pass-10 (producer-side D-15.x enumeration). NEW per TD-VSDD-057.

## Summary

Audited 5 files against ADR-015, ADR-014, and self-references. ~80 anchors enumerated; 1 line-anchor failed (introduced by H-P12-001 fix in v1.15 itself — same defect class fix was meant to close). One MED-severity sibling-template inconsistency in audit-w16.md noted. AC-3 prose form internally clean; ADR-014 amendment-title quote elides `(research)` parenthetical (LOW). All append-only invariants hold. Recursive-scrub left no fix-burst-internal tokens in permanent specs body, but caught only forbidden tokens — not line-number accuracy.

## Findings

### HIGH
None.

### MED

**M-P13-001 [MED] — open-questions.md line 20 cites "gap-analysis line 326" but quoted text is on line 325 (off-by-one anchor introduced by v1.15 fix)**
- File: open-questions.md line 20: `see also gap-analysis line 326 ("Resolution tracked in **OQ-W16-001**")`
- Actual: gap-analysis-w16-subprocess.md line 325 contains `Resolution tracked in **OQ-W16-001**`. Line 326 is `(`.factory/specs/open-questions.md`): either (a) `vsdd.host.*` is added...`.
- Severity: same anchor-grounding class TD-VSDD-068 was supposed to enforce. Recursive-scrub checks for forbidden tokens but not line-number accuracy of newly added citations. MED because: substring still resolves via grep-search; only line-number annotation wrong.
- Fix: `gap-analysis line 326` → `gap-analysis line 325`.

### LOW

**L-P13-001 [LOW] — AC-3 quotes ADR-014 amendment title without `(research)` source-tag**
- E-9 line 368: `per ADR-014 Amendment 2026-05-03 "R-8.09 ceiling model revised"`
- Actual ADR-014 line 38: `## Amendment 2026-05-03: R-8.09 ceiling model revised (research)`
- Severity: LOW. Substantive title faithful; `(research)` is meta-annotation. Acceptable but cite-fidelity miss.

**L-P13-002 [LOW] — audit-w16.md line 36 uses backticks around `PreToolUse:Agent` while sibling lines 37+38 don't (sibling-template inconsistency)**
- Line 36 (B-7): `(\`PreToolUse:Agent\`, \`on_error=block\`)` — backticks
- Line 37 (B-3): `(PreToolUse:Agent, on_error=block)` — no backticks
- Line 38 (B-2/B-6): `(PostToolUse:Edit|Write, on_error=block)` — no backticks
- Pass-12 normalized line 38 against line 37; line 36 left as outlier. Pending intent verification.

## Process-gaps

- [process-gap PG-P13-001] TD-VSDD-068 recursive-scrub command checks forbidden tokens but does not validate line-number accuracy of newly added cross-document citations. M-P13-001 is the class this gap creates: fix replaced one bad anchor with another bad anchor (off-by-one). Recommend extending TD-VSDD-068 (or filing as TD-VSDD-069): when a citation of form `<filename> line N (\"<quoted text>\")` is added in a fix burst, scrub MUST grep `<quoted text>` in `<filename>` and confirm exactly one match at line `N`.

## Convention checks
- Frontmatter `version:` matches latest non-reserved row (1.15): PASS
- v1.7-v1.15 summary rows intact (POLICY 1): PASS
- v1.16 preemptive reserved row: PASS
- v1.15 H3 section present: PASS
- v1.7-v1.14 block prose preserved as authored (POLICY 1 append-only): PASS
- No "Lines: X → Y" footer at v1.7-v1.15: PASS
- H3 version count matches summary table: PASS
- audit-w16.md line 38 sibling-wording-template consistency: PASS vs line 37; FAIL vs line 36 (L-P13-002)
- No fix-burst-internal IDs leak into permanent specs body: PASS
- Outbound decision-ID anchors semantically compatible (TD-VSDD-065): MOSTLY PASS — 1 off-by-one (M-P13-001)
- AC-3 canonical prose form with no-comma byte count: PASS

## Angle-specific outputs

35 anchors verified directly; ~45 more spot-checked via grep. Coverage of ADR-015 D-15.1/D-15.2/D-15.3/D-15.4 anchors PASS across all 5 files. ADR-014 R-8.09 amendment cite PASS at all sites except `(research)` tag elision (L-P13-001). Single defect: M-P13-001 off-by-one line cite.
