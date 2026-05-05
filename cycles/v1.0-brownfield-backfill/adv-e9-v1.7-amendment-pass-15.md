# Adversarial Review — E-9 v1.17 Minimal Burst (D-257) — Pass 15

**Date:** 2026-05-05
**Commit reviewed:** 12ca13f (v1.16 → v1.17)
**Cumulative surface:** v1.7..v1.17 (4 files + open-questions.md)
**Verdict:** SUBSTANTIVE
**ADR-013 clock at end of pass:** 0_of_3 reset
**Pass methodology angle:** Discoverability audit — story-writer reading-graph traversal + bidirectional anchor verification + symmetry audit. Treats story-writer authoring S-9.01..S-9.07 as primary reader; checks that all OQs/anchors/forward-pointers needed are reachable from natural entry points. NEW per TD-VSDD-057.

## Summary

Discoverability audit on the v1.17 surface. Most navigation paths PASS (E-9→gap-analysis Section 7; E-9→ADR-014 Amendment; E-9 AC-3→perf-baseline soft cap; gap-analysis→OQ-W16-001 bidirectional). However, OQ-W16-001 is invisible from E-9 epic's Open Questions table (the canonical discoverability hub for OQ register entries gating E-9 stories). Story-writer authoring S-9.07 reading E-9 §Open Questions misses the binary-choice gate; recovery only via gap-analysis transitive path. MED severity — single-row-append fix.

## Findings

### HIGH
None.

### MED

**M-P15-001 [MED]: OQ-W16-001 absent from E-9 epic Open Questions table**
- File: E-9 epic Open Questions table at lines 379-385.
- Evidence: table enumerates OQ-1, OQ-2, OQ-3. OQ-W16-001 NOT listed. Was filed in v1.11/D-248 to `.factory/specs/open-questions.md:18` and referenced inside E-9 ONLY in changelog H3 entries (lines 473, 833, 835, 838, 870 — all historical per POLICY 1).
- Impact: story-writer authoring S-9.07 reads E-9 §Open Questions to enumerate unresolved gates; OQ-W16-001 is the binary-choice gate for `vsdd.host.*` registry-prefix decision (acceptance: (a) ADR-015 amended OR (b) `vsdd.dispatcher.subprocess_completed.v1` exact). Per gap-analysis:328 "SS-01 implementer or E-10 Wave 1 architect MUST close OQ-W16-001 before host-emit-fix story merges." If story-writer never visits gap-analysis, OQ-W16-001 invisible.
- Bidirectional anchor (gap-analysis line 325 ↔ open-questions.md line 20) exists but E-9 epic Open Questions table is the discoverability hub and OQ is missing.
- Fix: append row to E-9 Open Questions table.

### LOW

**L-P15-001 [LOW] (pending intent verification): audit-w16.md frontmatter `version: "1.0"` despite extensive ADR-015 amendment**
- Frontmatter says `version: "1.0"`, `timestamp: 2026-05-03`. Body has Post-Audit Amendment ADR-015 Awareness across 5 burst applications (v1.7, v1.10, v1.11, v1.12, v1.16).
- D-239 codified annotate-in-place convention. Pass-13 already deferred (M-P6-001 + M-P5-002). Re-noting; not raising.

**L-P15-002 [LOW] (pending intent verification): audit-w16.md line 165 "D-2 Option C" outbound decision-ID anchor non-resolving in current corpus**
- Pre-amendment audit-time prose. Already SKIPPED in v1.17 changelog L-P14-001. Sibling D-2 Option C anchors do not propagate into amendment-scope content (none found via grep).

## Process-gaps

- [process-gap PG-P15-001]: Discoverability hub asymmetry — E-9 epic Open Questions table is canonical hub for OQ entries gating E-9 stories, but OQ-W16-001 was added to `open-questions.md` and gap-analysis prose without corresponding row append to E-9 epic. Recommend hook/process-step: when an OQ is filed citing an E-N epic as scope-owner, verify epic body's Open Questions table contains corresponding row. Class adjacent to TD-VSDD-065/070 (outbound semantic-anchor) but specifically for **OQ-table propagation**. File as TD-VSDD-071.

## Convention checks

- Frontmatter `version:` matches latest non-reserved row (1.17): PASS
- v1.7-v1.17 summary rows intact (POLICY 1): PASS
- v1.18 preemptive reserved row: PASS (line 480)
- v1.17 H3 section present: PASS (lines 1065-1081)
- v1.7-v1.16 block prose preserved as authored (POLICY 1 append-only): PASS
- No "Lines: X → Y" footer at v1.7-v1.17: PASS
- H3 version count matches summary table: PASS
- audit-w16.md line 38 sibling-wording-template consistency: PASS
- No fix-burst-internal IDs leak into permanent specs body: PASS
- Outbound decision-ID anchors semantically compatible (in-text + section-heading): PASS for amendment-scope; one out-of-scope residual at audit-w16:165 noted as L-P15-002
- Citation line numbers accurate: PASS

## Angle-specific outputs

Reading-graph traversal: 12 navigation paths checked; 11 PASS, 1 FAIL (E-9 epic Open Questions → OQ-W16-001 missing). Bidirectional anchor verification: OQ-W16-001 ↔ gap-analysis:325 PASS; E-9 ↔ gap-analysis Section 7 PASS; AC-3 ↔ perf-baseline soft cap PASS. Asymmetric: E-9 epic body → OQ-W16-001 missing forward-pointer (M-P15-001).

Symmetry audit: AC-3 thresholds (≤500ms cold start, ≤30MB hard kill, 643686 bytes soft cap) all symmetric with perf-baseline-w16.md formulas/values. AC-8 5 block-mode hooks symmetric across E-9 + audit-w16 + hooks-registry references. v1.0.0-rc.1 ↔ pre-W-15-baseline equivalence noted in perf-baseline:147.
