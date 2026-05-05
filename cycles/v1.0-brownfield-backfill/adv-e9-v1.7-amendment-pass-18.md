# Adversarial Review — E-9 v1.19 Sibling-Residue Burst (D-260) — Pass 18

**Date:** 2026-05-05
**Commit reviewed:** 7925c62 (v1.18 → v1.19)
**Cumulative surface:** v1.7..v1.19 (4 files + open-questions.md)
**Verdict:** SUBSTANTIVE
**ADR-013 clock at end of pass:** 0_of_3 (was 0_of_3 entering; remains 0 — SUBSTANTIVE non-advance)
**Pass methodology angle:** Frontmatter consistency audit across all 5 in-scope files (schema fields, value types, version-vs-body alignment, references list completeness, last_amended date alignment). NEW per TD-VSDD-057.

## Summary

Examined frontmatter of all 5 in-scope files as a coherent set: do they accurately describe each file's *current* state after v1.19? Found one MED finding: 4 of 5 files (audit-w16, gap-analysis, perf-baseline, open-questions) have `version: "1.0"` and no `last_amended:` despite material body amendments across v1.7..v1.19. This is the 5th re-flag of the same convention question (M-P5-002 → M-P6-001 → L-P14-002 → L-P15-001 → M-P18-001). S-7.02 recurrence threshold met (3+); recommend DEFINITIVE resolution. Plus one LOW: perf-baseline references entry missing `(research)` source-tag matching v1.16 L-P13-001 AC-3 closure.

## Findings

### HIGH
None.

### MED

**M-P18-001 [MED] [process-gap recurrence]: 4 of 5 in-scope files have stale `version: "1.0"` frontmatter despite material body amendments — 5th re-flag of same convention question**
- Files: gap-analysis-w16-subprocess.md (v1.7+v1.8+v1.11 body changes), audit-w16.md (v1.7+v1.10+v1.11+v1.12+v1.13 body changes), perf-baseline-w16.md (v1.8+v1.13+v1.15+v1.17+v1.19 body changes), open-questions.md (v1.14+v1.15+v1.16 body changes).
- Body amendment status not reflected in frontmatter `version:` or `last_amended:` fields.
- Recurrence: M-P5-002 (D-245) → M-P6-001 (D-247) → L-P14-002 (D-257) → L-P15-001 (D-258) → M-P18-001 (this pass).
- Per S-7.02 lessons-codification rule: 3+ recurrences qualify for definitive codification. We are at 5+ recurrences without resolution.
- Recommendation: codify TD-VSDD-073 with EXPLICIT decision (option A: add `last_amended:` to arch-doc convention; option B: exempt arch-doc `version:` from frontmatter-coherence checks). Stop deferring.

### LOW

**L-P18-001 [LOW]: perf-baseline-w16.md references entry line 13 cites ADR-014 amendment without `(research)` source-tag**
- File: perf-baseline-w16.md line 13: `- ADR-014 R-8.09 revised (Amendment 2026-05-03)`
- Source-of-truth: ADR-014 line 38: `## Amendment 2026-05-03: R-8.09 ceiling model revised (research)`
- v1.16 L-P13-001 (D-256) restored `(research)` source-tag in AC-3 line 368. Same restoration not applied to perf-baseline frontmatter references entry.
- LOW (semantically equivalent reference; cosmetic alignment with v1.16 fix).

## Out-of-scope-but-noted

- audit-w16.md Section 5 R-W16-003 historical projection (~7.2 MB / 11.8-14.1 MB): pre-amendment audit-time prose; POLICY 1 immutable.
- audit-w16.md line 165 "D-2 Option C": deferred per L-P14-001 / L-P15-002.

## Process-gaps

- [process-gap PG-P18-001]: Frontmatter-vs-body amendment-convention drift recurrence pattern. 5 distinct adversarial findings across passes 5/6/14/15/18 surfaced same pattern. Each DEFERRED individually citing D-239. S-7.02 threshold (3+ recurrences) met. Recommend codifying TD-VSDD-073 to definitively resolve.

## Convention checks

- Frontmatter `version:` matches latest non-reserved row (1.19): PASS (E-9 only)
- v1.7-v1.19 summary rows intact (POLICY 1): PASS
- v1.20 preemptive reserved row: PASS (line 483)
- v1.19 H3 section present: PASS (line 1104)
- v1.7-v1.18 block prose preserved as authored (POLICY 1 append-only): PASS
- No "Lines: X → Y" footer at v1.7-v1.19: PASS
- H3 version count matches summary table: PASS
- audit-w16.md line 38 sibling-wording-template consistency: PASS
- No fix-burst-internal IDs leak into permanent specs body: PASS
- Outbound decision-ID anchors semantically compatible: PASS
- Citation line numbers accurate: PASS
- OQ-W16-001 propagated to E-9 Open Questions table: PASS
- No retired-figure residue in non-changelog body (TD-VSDD-072): PASS

## Angle-specific outputs

Frontmatter consistency audit results — 4 of 5 files have version: "1.0" despite material body amendments. Only E-9 epic carries last_amended:. Only perf-baseline-w16.md has populated references list. Convention is partially codified (D-239 annotate-in-place for body) but frontmatter coherence is unspecified. 5+ adversarial passes have re-surfaced this; codification overdue.
