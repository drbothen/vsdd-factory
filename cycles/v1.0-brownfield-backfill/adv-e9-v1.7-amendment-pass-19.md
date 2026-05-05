# Adversarial Review — E-9 v1.20 Convention-Closure Burst (D-261) — Pass 19

**Date:** 2026-05-05
**Commit reviewed:** edb340a (v1.19 → v1.20)
**Cumulative surface:** v1.7..v1.20 (4 files + open-questions.md)
**Verdict:** NITPICK_ONLY
**ADR-013 clock at end of pass:** 0_of_3 → 1_of_3 advance
**Pass methodology angle:** Diff-only line-by-line of v1.20 (edb340a) — fresh-context audit of the 4 frontmatter `last_amended:` insertions + perf-baseline references update + v1.20 H3 prose + v1.20 summary-table row, plus markdown-syntax sweep. NEW per TD-VSDD-057.

## Summary

v1.20 burst is a tight 2-fix surface (M-P18-001 frontmatter additions + L-P18-001 perf-baseline references restoration). All four arch-doc-class files carry `last_amended: 2026-05-05` in frontmatter at expected position adjacent to `timestamp:`. E-9 frontmatter `version: "1.20"` matches latest non-reserved row. perf-baseline-w16.md L14 references entry matches ADR-014 H2 title canonical form with `(research)` source-tag. No body-content drift; v1.21 reserved row preserved; v1.7-v1.19 H3 prose intact. Markdown structure clean across all 5 files. No HIGH or MED findings.

## Findings

### HIGH
None.

### MED
None.

### LOW

**L-P19-001 [LOW]: open-questions.md line 21 citation off-by-one to gap-analysis line 325 vs actual 326**
- File: open-questions.md line 21: `see also gap-analysis line 325 ("Resolution tracked in **OQ-W16-001**") for the bidirectional anchor.`
- Reality: phrase "Resolution tracked in **OQ-W16-001**" in gap-analysis-w16-subprocess.md begins on line 326 (or spans 325→326). Possibly line shift since v1.15/v1.16 grep verification.
- Severity: LOW (line off-by-one in "see also" pointer; bidirectional anchor still resolves visually).

**L-P19-002 [LOW]: v1.20 H3 prose recurrence-chain composition imprecise**
- File: E-9 line 1150 (v1.20 H3 prose): "5th re-flag of the same convention question (M-P5-002 → M-P6-001 → L-P14-002 → L-P15-001 → M-P18-001)".
- Reality: chain mixes two related-but-distinct convention sub-questions. L-P14-002 (line 1079) is about `producer:` field semantics, not `last_amended:` field absence. Strict recurrence count of closed convention question is 4 (M-P5-002, M-P6-001, L-P15-001, M-P18-001); L-P14-002 is sibling convention question deferred with same D-239 rationale but not strictly same sub-question.
- Impact: S-7.02 threshold (3+) comfortably met regardless; closure rationale stands. Descriptive imprecision, not closure defect.

## Out-of-scope-but-noted

- audit-w16 lines 479-492 retain "R-8.09 25% growth ceiling" framing without inline pointer to post-audit ADR-014 R-8.09 revised (per D-239 annotate-in-place; amendment lives at top of doc).
- v1.20 H3 line 1152 calls all 4 in-scope files "amendment-touched arch-doc-class" but open-questions.md has `level: ops` and `document_type: open-questions-register`. Loose category; convention extension is intentional.

## Process-gaps

(none new this pass)

## Convention checks

- Frontmatter `version:` matches latest non-reserved row (1.20): PASS
- Arch-doc-class files have `last_amended: 2026-05-05` (TD-VSDD-073): PASS (all 4 verified)
- v1.7-v1.20 summary rows intact (POLICY 1): PASS
- v1.21 preemptive reserved row: PASS (line 484)
- v1.20 H3 section present: PASS (line 1146)
- v1.7-v1.19 block prose preserved as authored (POLICY 1 append-only): PASS
- No "Lines: X → Y" footer at v1.7-v1.20: PASS
- H3 version count matches summary table: PASS
- audit-w16.md line 38 sibling-wording-template consistency: PASS
- No fix-burst-internal IDs leak into permanent specs body: PASS
- Outbound decision-ID anchors semantically compatible: PASS
- Citation line numbers accurate: MINOR (L-P19-001 off-by-one)
- OQ-W16-001 propagated to E-9 Open Questions table: PASS
- No retired-figure residue in non-changelog body (TD-VSDD-072): PASS

## Angle-specific outputs

Diff-only line-by-line v1.20 (edb340a) audit: 9 changes verified — gap-analysis L8 last_amended ✓, audit-w16 L9 last_amended ✓, perf-baseline L10 last_amended ✓, open-questions L8 last_amended ✓, perf-baseline L14 references entry with `(research)` ✓, E-9 frontmatter L4 v1.20 ✓, E-9 changelog summary L483 v1.20 row ✓, E-9 changelog summary L484 v1.21 reserved ✓, E-9 H3 v1.20 section L1146-1170 well-formed (1 descriptive imprecision per L-P19-002).

Markdown structure sweep: frontmatter delimiters balanced (5/5 files); tables properly closed; no code-block fence imbalance; bullet nesting consistent across H3 sections.
