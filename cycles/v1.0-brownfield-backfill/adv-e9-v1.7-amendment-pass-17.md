# Adversarial Review — E-9 v1.18 OQ-Propagation Burst (D-258) — Pass 17

**Date:** 2026-05-05
**Commit reviewed:** e619082 (v1.18; unchanged from pass-16)
**Cumulative surface:** v1.7..v1.18 (4 files + open-questions.md)
**Verdict:** SUBSTANTIVE
**ADR-013 clock at end of pass:** 1_of_3 → 0_of_3 reset
**Pass methodology angle:** Linguistic uniformity / numerical-claim cross-table walk + section-heading semantic-anchor scan. NEW per TD-VSDD-057. Combines (a) sibling-table walk WITHIN a single body (Risk table vs AC table) which prior passes did not do, and (b) section-heading-vs-body-data factual consistency (extending TD-VSDD-070 to release-tag descriptors).

## Summary

Pass-17 walked numerical claims across sibling tables in the same body (Risk table vs AC table in E-9), and checked section-heading-vs-body data consistency across the perf-baseline file. Two HIGH findings: a sibling-row regression of the H-P11-001 fix (R-W16-003 mitigation column retains "~14MB" while AC-3 was scrubbed in v1.14), and an inaccurate H2 section heading "post-rc.4" in perf-baseline-w16.md that contradicts the dec5361 (v1.0.0-rc.11) measurement tag in the body. One MED echo of #2 in OQ-1 prose. Pattern: "fix applied to primary, sibling not updated" per S-7.01 Partial-Fix Regression Discipline. Body-grep gap in v1.14 fix-burst protocol.

## Findings

### HIGH

**H-P17-001 [HIGH]: R-W16-003 mitigation column at E-9 line 353 retains superseded "~14MB" advisory soft cap; contradicts AC-3 by ~21x**
- File: E-9 epic line 353. R-W16-003 mitigation cell: "Advisory soft cap: cumulative ≤100% growth (~14MB) at end of W-17."
- Contradiction: AC-3 at line 368 says "advisory soft cap target = 643686 bytes". Same epic body. Same advisory-soft-cap subject. ~14MB ≈ 14,000,000 bytes vs 643,686 bytes — 21× divergence.
- Authority: ADR-014 line 45 explicitly retires the ~14MB projection: "the prior ~14MB target derived from research §Q3's 7.2MB projection is superseded — that figure was a projection, not a measurement". Perf-baseline:163 pins `w16_advisory_bundle_soft_cap_bytes | 643686`.
- Class: Sibling-row regression of v1.14 / D-254 / H-P11-001 fix. Body-grep would have caught it.

**H-P17-002 [HIGH]: perf-baseline-w16.md H2 heading line 33 "(post-rc.4, pre-Tier 2)" contradicts measured release_tag_sha at line 41 (v1.0.0-rc.11)**
- File: perf-baseline-w16.md line 33: `## W-16 Bundle Baseline (post-rc.4, pre-Tier 2)`. Line 41: `release_tag_sha | dec5361 (v1.0.0-rc.11)`.
- Original framing was rc.4 when v1.6 authored 2026-05-03; measurement retaken at rc.11 on 2026-05-05. Heading parenthetical not updated when measurement retaken.
- Class: TD-VSDD-070 (section-heading semantic-anchor) extended to release-tag descriptors.

### MED

**M-P17-001 [MED]: E-9 OQ-1 line 383 "post-rc.4 baseline" anchor stale (sibling propagation of H-P17-002)**
- File: E-9 line 383. OQ-1 question prose: "what % growth is acceptable for 23 new plugins over the post-rc.4 baseline?"
- S-9.00 perf-baseline (resolver) measured at rc.11. OQ-1 question anchors rc.4 vs resolver rc.11. Question text and resolver disagree on baseline epoch.

### LOW

**L-P17-001 [LOW]: verify-sha-currency.sh backtick inconsistency**
- E-9 lines 39 (backticks), 220 (backticks), 300 (no backticks). Stylistic only.

## Out-of-scope-but-noted

- audit-w16.md frontmatter version drift (deferred per D-239).
- audit-w16.md line 165 "D-2 Option C" (deferred per L-P14-001 / L-P15-002).
- audit-w16.md Section 5 R-W16-003 historical "~7.2 MB" projection — pre-amendment audit-time prose, POLICY 1 immutable.

## Process-gaps

- [process-gap PG-P17-001]: Body-grep gap in fix-burst protocol. v1.14 / D-254 fix-burst that sealed H-P11-001 in AC-3 line 368 did not run a body-wide grep for retired figure to detect sibling rows. Adding such a body-grep to TD-VSDD-068 recursive-scrub regex set would have caught H-P17-001 at v1.14 close. Recommend extending TD-VSDD-068 with "retired-figure-residue" grep when a fix burst replaces a retired numeric value: every value the fix retires should be grepped across the file body to detect un-scrubbed residue. File as TD-VSDD-072.

## Convention checks

- Frontmatter `version:` matches latest non-reserved row (1.18): PASS
- v1.7-v1.18 summary rows intact (POLICY 1): PASS
- v1.19 preemptive reserved row: PASS (line 482)
- v1.18 H3 section present: PASS
- v1.7-v1.17 block prose preserved as authored (POLICY 1 append-only): PASS
- No "Lines: X → Y" footer at v1.7-v1.18: PASS
- H3 version count matches summary table: PASS
- audit-w16.md line 38 sibling-wording-template consistency: PASS
- No fix-burst-internal IDs leak into permanent specs body: PASS
- Outbound decision-ID anchors semantically compatible: MIXED — H-P17-002 section-heading mismatch
- Citation line numbers accurate: PASS
- OQ-W16-001 propagated to E-9 Open Questions table: PASS

## Angle-specific outputs

Linguistic uniformity audit:
- "block-mode" usage uniform across audit-w16 + E-9. PASS.
- "subprocess" uniform across gap-analysis. PASS.
- "native WASM" uniform. PASS.
- "verify-sha-currency.sh" backtick treatment inconsistent (L-P17-001).
- "post-rc.X" stale at perf-baseline:33 + E-9:383 (H-P17-002 + M-P17-001).

Numerical-claim cross-table walk:
- E-9 R-W16-003 line 353 "~14MB" vs E-9 AC-3 line 368 "643686 bytes" — CONTRADICTION (H-P17-001).
- E-9 AC-3 vs perf-baseline:163 vs ADR-014:45 — three-way symmetric. PASS.
- E-9 AC-3 hard kill-switch vs perf-baseline:164 — symmetric. PASS.

Section-heading semantic-anchor scan (per TD-VSDD-070):
- perf-baseline:33 H2 "(post-rc.4)" — STALE (H-P17-002).
- perf-baseline:154 H2 — fixed in v1.17. PASS.
- E-9 H1/H2/H3 — no other stale anchors identified.
