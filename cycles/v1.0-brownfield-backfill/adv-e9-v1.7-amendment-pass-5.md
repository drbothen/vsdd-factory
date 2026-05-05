# Adversarial Review — E-9 v1.9 Minimal Fix Burst (D-244) — Pass 5

**Date:** 2026-05-05
**Commit reviewed:** 067379c (v1.8 → v1.9)
**Cumulative amendment surface:** v1.7 + v1.8 + v1.9 (4 files)
**Verdict:** SUBSTANTIVE
**ADR-013 clock at end of pass:** 0_of_3 reset
**Pass methodology angle:** Versioning/lifecycle propagation audit + frontmatter-body coherence audit (NEW per TD-VSDD-057). Walks epic frontmatter, changelog summary table, v1.9 changelog body, and cross-doc citations as a paired set. Asks: when the architect bumped v1.8→v1.9 in the body, did all metadata anchored to the version number propagate? Re-verifies the "minimal fix burst" diff for new defects.

## Summary

The v1.9 fix burst correctly identifies and resolves the H-P4-001 fabricated-AC-3 defect at both sites. Citations re-anchored to ADR-015 D-15.2 taxonomy registry are accurate (line 329 verified, lines 295–333 bracket the registry section). However, the burst introduces one HIGH structural defect: the epic frontmatter `version` field was not bumped from `"1.8"` to `"1.9"`, despite the body containing a complete v1.9 changelog section and the summary table v1.9 row. This is the third frontmatter-vs-summary-table drift in this epic's history (after F-P6-002 + F-P7-001) — recurrent pattern qualifying for codification per the lessons-codification rule.

## Findings

### HIGH

**H-P5-001 [HIGH] — Frontmatter version not bumped to 1.9**
- File: E-9 epic line 4. Frontmatter says `version: "1.8"`; body Changelog summary table line 471 contains v1.9 row; v1.9 H3 section exists at line 734.
- POLICY 1 violation: frontmatter must track latest changelog row. Tooling reading frontmatter (state-manager SHA recompute, input-hash, downstream cross-doc references) sees v1.8.
- Same regression class as F-P6-002 / F-P7-001 inverted: table has the row, frontmatter does not have the version. Recurrent pattern (3+).
- Blast radius: 1 file. Severity HIGH per S-7.01 partial-fix-regression rule.

### MED

**M-P5-001 [MED] — Self-citing change description points at wrong section; v1.9 burst edits v1.8 prose in-place violating POLICY 1 append-only**
- File: E-9 epic line 744. v1.9 changelog says: "Site 2 (E-9 changelog v1.8 M-2 closure entry): 'Wave 3 AC-3 queries...' rewritten to 'audit-category events are SIEM-queryable...'". Actual rewritten text now lives at lines 720-722 INSIDE the v1.8 block.
- POLICY 1 (append_only_numbering) requires append-only changelog. In-place edits to prior version's changelog body silently rewrite history.
- Correct mechanism: leave v1.8 prose as-was (with fabricated citation) and record correction ONLY in v1.9 block.

**M-P5-002 [MED] — gap-analysis frontmatter `version: "1.0"` inconsistent with body amendment-version annotations (v1.7)**
- File: gap-analysis-w16-subprocess.md frontmatter line 5 (`version: "1.0"`); body line 307 says "Post-Audit Amendment: ADR-015 Awareness (v1.7, 2026-05-05)".
- POLICY 1 / D-239 reconciliation issue: D-239 codified annotate-in-place for arch docs, but the body now carries amendment-version annotations (v1.7) the frontmatter does not reconcile. Either body drops version-annotations OR frontmatter reflects them. Inconsistent application.

**M-P5-003 [MED] — Cross-doc gap: audit-w16.md B-7 row missing block-mode treatment for validate-wave-gate-prerequisite**
- File: audit-w16.md amendment block. Lines 35-38 enumerate 4 block-mode hooks (factory-path-root B-1, pr-merge-prerequisites B-3, input-hash B-2, template-compliance B-6). The 5th block-mode hook validate-wave-gate-prerequisite (S-9.07) is mentioned at line 36 but NOT explicitly called out as block-mode with H-1 option (b) treatment.
- E-9 enumerates all 5 block-mode hooks correctly at lines 176-183; audit-w16.md amendment incomplete.
- H-1 option (b) propagation gap. Story-writer reading audit-w16.md alone misses block-path treatment for B-7.
- S-7.01 partial-fix-regression: blast radius 2 files.

### LOW

**L-P5-001 [LOW] — `event.category=audit` filter syntax inconsistency across 4 sites (3 different forms; with/without spaces, with/without quotes).** Cosmetic.

**L-P5-002 [LOW] — Reserved row v1.10 verified per D-232 convention.** No defect; acknowledged for completeness.

**L-P5-003 [LOW] — Block-mode callout E-9 lines 176-185 silent on emit-event behavior on block path; may be intentional (test-fixtures focus, not emit-semantics). (pending intent verification per S-7.01).**

## Out-of-scope-but-noted

- v1.6 body converged per pass-10 NITPICK x3 (2026-05-03). Not re-attacked.
- ARCH-INDEX/BC-INDEX/SS-01/SS-03 per D-236.

## Process-gaps

- **[process-gap PG-P5-001]:** No automated check ties frontmatter `version:` to latest non-reserved Changelog summary table row. Third recurrence (F-P6-002, F-P7-001, H-P5-001) qualifies for codification. Suggest a hook asserting `frontmatter.version == max(changelog_summary_table.version where date != '—')`.
- **[process-gap PG-P5-002]:** POLICY 1 silent on whether prose corrections within prior version blocks are allowed. Either explicitly forbid in-place edits (require corrections-only-in-new-version-block) OR allow with marker (e.g., `[corrected v1.9: ...]`). Currently undocumented and inconsistent.

## Convention checks

- v1.7 + v1.8 summary rows intact (POLICY 1): PASS (lines 469-470)
- v1.9 summary row appended: PASS (line 471)
- v1.10 reserved preemptive row: PASS (line 472)
- v1.9 H3 section present and consistent: PASS (line 734)
- No "Lines: X → Y" footer at v1.7-v1.9: PASS
- Frontmatter version pin matches latest non-reserved changelog row: **FAIL** (H-P5-001)
- H3 version count matches summary table count (excluding initial v1.0 + reserved future rows): PASS (10 entries non-reserved minus v1.0 = 9 H3 sections; 9 H3 found at v1.1..v1.9)

## Angle-specific outputs

Versioning/lifecycle propagation audit table (selected rows):

| Metadata anchor | Pre-v1.9 (v1.8) | Post-v1.9 expected | Post-v1.9 actual | PASS/FAIL |
|---|---|---|---|---|
| frontmatter.version | "1.8" | "1.9" | "1.8" | **FAIL** (H-P5-001) |
| frontmatter.last_amended | 2026-05-05 | 2026-05-05 | 2026-05-05 | PASS |
| frontmatter.status | in-review | in-review | in-review | PASS |
| Changelog summary table latest row | v1.8 | v1.9 | v1.9 | PASS |
| Reserved row | v1.9 (reserved) | v1.10 (reserved) | v1.10 (reserved) | PASS |
| Latest H3 section | ### v1.8 | ### v1.9 | ### v1.9 | PASS |
| Cross-doc gap-analysis frontmatter | v1.0 (per D-239) | v1.0 (per D-239) | v1.0 | PASS per D-239; M-P5-002 raises body-vs-frontmatter inconsistency |
| Cross-doc perf-baseline frontmatter `references:` | ADR-015 row appended (M-3) | unchanged | ADR-015 row at line 17 | PASS |

Diff-only re-verification of v1.9 (067379c) against ADR-015:
- gap-analysis line 343 cites lines 295-333: VERIFIED (registry begins line 295, ends line 332; ~tolerance OK).
- gap-analysis line 339 cites ADR-015 line 329 → `vsdd.capability.denied.* | audit`: VERIFIED.
- E-9 line 722: "audit-category events SIEM-queryable by event.category=audit filter (ADR-015 D-15.2 taxonomy registry)": VERIFIED.
- E-9 line 704: "Site 1: E-9 lines ~294-302": VERIFIED (brackets H-1-relevant subset of v1.7 block).
