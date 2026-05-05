# Adversarial Review — E-9 v1.18 OQ-Propagation Burst (D-258) — Pass 16

**Date:** 2026-05-05
**Commit reviewed:** e619082 (v1.17 → v1.18)
**Cumulative surface:** v1.7..v1.18 (4 files + open-questions.md)
**Verdict:** NITPICK_ONLY
**ADR-013 clock at end of pass:** 0_of_3 → 1_of_3 advance
**Pass methodology angle:** Hooks-batch traceability deep-dive + failure-mode catalog spot-check. NEW per TD-VSDD-057. Enumerates each of the 23 hooks across all three primary docs and confirms batch-membership consistency, then spot-checks failure-mode coverage handlers vs ADR-015 D-15.x clauses.

## Summary

Cumulative v1.7..v1.18 surface holds together. Hook traceability across E-9 Hook Inventory (lines 118-174), audit-w16 Section 4 batch table (lines 364-372), and audit-w16 amendment §"Batches that emit OTel events" (lines 33-38) is internally consistent: 4+4+3+3+3+3+3 = 23 hooks across B-1..B-7. Failure-mode catalog (block-mode dispatch, exec_subprocess denial path, unrecognized-prefix fallback, trace propagation) handlers aligned with ADR-015 D-15.1/D-15.2.b/D-15.3/D-15.4. The v1.18 OQ-W16-001 row append (E-9 line 386) parses as well-formed 4-column table row matching Open Questions header schema. No HIGH or MED findings. Three LOW observations: cosmetic only.

## Findings

### HIGH
None.

### MED
None.

### LOW

**L-P16-001 [LOW] (confidence MEDIUM): "Section 5, Gap 2" reference in gap-analysis amendment heading is non-self-disambiguating without context**
- File: gap-analysis-w16-subprocess.md line 313 H3 heading: `### How ADR-015 affects the telemetry gap (Section 5, Gap 2)`.
- Section 5 (line 194) has THREE numbered subsections: "Gaps where exec_subprocess is sufficient" (line 196, 8 items), "Gaps where exec_subprocess needs minor extension" (line 209, 3 items), "Gaps where exec_subprocess is fundamentally insufficient" (line 221, 3 items). Each has its own item list numbered from 1. Reference "Section 5, Gap 2" therefore matches THREE possible items.
- Mitigating: heading itself clarifies "the telemetry gap", disambiguates by topic.
- Severity rationale: LOW because heading topic disambiguates; nobody reading the document is misled.

**L-P16-002 [LOW] (confidence HIGH, pending intent verification): B-1 row in audit-w16 amendment lacks event-type parenthetical present in B-2/B-3/B-6/B-7 rows**
- File: audit-w16.md line 35 (B-1): no `(PostToolUse:Edit|Write, on_error=block)` parenthetical, while line 36 (B-7) carries `(PreToolUse:Agent, on_error=block)`, line 37 (B-3) carries `(PreToolUse:Agent, on_error=block)`, line 38 (B-2/B-6) carries `(PostToolUse:Edit|Write, on_error=block)`.
- Already raised and DEFERRED as L-P7-002 per v1.12 changelog block (E-9 lines 914-917). Re-raised on fresh pass.
- Severity rationale: LOW because sentence semantics complete; cosmetic only.

**L-P16-003 [LOW] (confidence HIGH): Adjacent redundant "Amendment 2026-05-03" tokens in perf-baseline §"W-16 Gate Model"**
- File: perf-baseline-w16.md line 154 H2: `## W-16 Gate Model (ADR-014 R-8.09 Amendment 2026-05-03)`. Line 156 (immediately following): `Reference: E-9 AC-3 + ADR-014 R-8.09 (Amendment 2026-05-03).`
- "Amendment 2026-05-03" appears twice in adjacent sentences spanning H2 + first body line.
- Severity rationale: LOW — readability only; no semantic defect.

## Out-of-scope-but-noted

- audit-w16 §R-W16-003 (lines 482-484) projects ~7.2 MB pre-W-16 baseline; perf-baseline measurement = 8549146 bytes (~8.55 MB). Audit-w16 was authored 2026-05-03 (audit phase, pre-measurement); not appropriate to amend per POLICY 1 append-only. Out-of-scope for v1.7..v1.18 amendment surface.
- OQ-W16-001 acceptance criterion (a) wording asymmetry between open-questions.md and E-9 paraphrase. Both semantically equivalent; acceptable paraphrase.

## Process-gaps

(none new this pass)

## Convention checks

- Frontmatter `version:` matches latest non-reserved row (1.18): PASS
- v1.7-v1.18 summary rows intact (POLICY 1): PASS
- v1.19 preemptive reserved row: PASS (line 482)
- v1.18 H3 section present: PASS (line 1085)
- v1.7-v1.17 block prose preserved as authored (POLICY 1 append-only): PASS
- No "Lines: X → Y" footer at v1.7-v1.18: PASS
- H3 version count matches summary table: PASS (12 H3 blocks for v1.7..v1.18; 12 summary rows for v1.7..v1.18)
- audit-w16.md line 38 sibling-wording-template consistency: PASS
- No fix-burst-internal IDs leak into permanent specs body: PASS
- Outbound decision-ID anchors semantically compatible: PASS
- Citation line numbers accurate: PASS
- OQ-W16-001 propagated to E-9 Open Questions table (TD-VSDD-071): PASS

## Angle-specific outputs

Hook traceability matrix (23 hooks × 3 docs): all 23 hooks traceable across E-9 + audit-w16 + amendment block with consistent batch assignments. Total: 4+4+3+3+3+3+3 = 23. All 5 block-mode hooks (factory-path-root B-1, input-hash B-2, pr-merge-prerequisites B-3, template-compliance B-6, wave-gate-prerequisite B-7) identified consistently across E-9 line 178-181 + AC-8 + audit-w16 lines 35-38.

Failure-mode handler audit: all 6 failure modes (block, unrecognized-prefix, denial, success-path, trace-propagation, host-field-override) trace to ADR-015 D-15.x clauses with correct semantics. No drift between proposed handler and ADR-015 contract.
