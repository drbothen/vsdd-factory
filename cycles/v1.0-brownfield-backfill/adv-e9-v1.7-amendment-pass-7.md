# Adversarial Review — E-9 v1.11 Fix Burst (D-248) — Pass 7

**Date:** 2026-05-05
**Commit reviewed:** 0ccdf4f (v1.10 → v1.11) — cumulative surface v1.7..v1.11
**Files reviewed:** 4 (E-9 epic, gap-analysis-w16-subprocess.md, perf-baseline-w16.md, audit-w16.md) + open-questions.md (OQ-W16-001)
**Verdict:** SUBSTANTIVE
**ADR-013 clock at end of pass:** 0_of_3 (RESET — was already 0_of_3 entering pass-7)
**Pass methodology angle:** Cross-doc consistency / contradiction hunt (NEW per TD-VSDD-057). Verified agreement on block-mode hook IDs, batch-to-story mapping, ADR-015 D-15.x clauses, terminology, POLICY 6 SS-NN names, POLICY 1 append-only prose preservation across the 4 amendment files. Distinct from passes 1-6.

## Summary

Cross-doc audit finds the v1.11 H-P6-001 closure (audit-w16.md line 38 amendment) introduced 3 MED issues — all localized to that single line. (1) Self-contradictory parenthetical: "S-9.02–S-9.06" en-dash range includes S-9.03 but the row's batch list "B-2, B-4, B-5, B-6" excludes B-3/S-9.03 (on a separate row at line 37). (2) Fix-burst-internal terminology "H-1 option (b)" leaks into permanent architecture doc. (3) Asymmetric wording template: line 38 omits PostToolUse parenthetical present on sibling rows 36/37. OQ-W16-001 is well-formed and consistently referenced. ADR-015 D-15.x cross-citations all verify. POLICY 6 SS-NN names match ARCH-INDEX. POLICY 1 append-only PASS (v1.7-v1.10 H3 prose preserved). Notable: **0 HIGH findings** — first 0-HIGH SUBSTANTIVE pass in this convergence cycle; defects are increasingly cosmetic/structural at the polish boundary.

## Findings (severity-ordered)

### HIGH

(none)

### MED

**M-P7-001 [MED] — audit-w16.md line 38 parenthetical "S-9.02–S-9.06" en-dash range contradicts batch list "B-2, B-4, B-5, B-6"**
- File: audit-w16.md line 38.
- Evidence: cell header `| B-2, B-4, B-5, B-6 (S-9.02–S-9.06) | File-read validators | **Standard.** ...`. The en-dash range covers 5 stories (S-9.02..S-9.06) but the batch list is 4 batches excluding B-3 (S-9.03 covered separately on line 37). Internally inconsistent.
- Fix: Replace `(S-9.02–S-9.06)` with explicit list `(S-9.02, S-9.04, S-9.05, S-9.06)`.

**M-P7-002 [MED] — audit-w16.md line 38 introduces fix-burst-internal terminology "H-1 option (b)" without context**
- File: audit-w16.md line 38 (v1.11 appended sentence).
- Evidence: "Both validate-input-hash (B-2) and validate-template-compliance (B-6) follow H-1 option (b): plugins return HookResult::Block...". Lines 35/36/37 do NOT use "H-1 option (b)" — describe same mechanism in plain language. "H-1" appears nowhere else in audit-w16.md (zero grep occurrences). The term refers to pass-3 finding H-1 from review document — fix-burst-internal nomenclature.
- Fix: Replace `follow H-1 option (b):` with `are block-mode:` (matches lines 35/37 wording style) OR remove phrase — rest of sentence is self-sufficient.

**M-P7-003 [MED] — audit-w16.md line 38 omits PostToolUse event/on_error parenthetical present on sibling block-mode rows 36/37**
- Files: audit-w16.md line 38 vs line 36 (B-7), line 37 (B-3).
- Evidence: line 36 provides `(PreToolUse:Agent, on_error=block)`; line 37 provides `(PreToolUse:Agent, on_error=block)`; line 38 provides only `(B-2, block-mode)` and `(B-6, block-mode)` — no event/on_error parenthetical. Per E-9 line 134 + 166, validate-input-hash and validate-template-compliance are PostToolUse:Edit|Write block-mode. Event-type difference (PostToolUse vs PreToolUse) IS materially relevant for ADR-015 D-15.3.
- Fix: Append `(PostToolUse:Edit|Write, on_error=block)` to validate-input-hash and validate-template-compliance mentions on line 38.

### LOW

**L-P7-001 [LOW] — line 38 wording "per ADR-015 D-15.3" varies from peer rows**
- Lines 35/36/37 use bare "D-15.3"; line 38 uses "per ADR-015 D-15.3". Cosmetic drift.
- Fix (optional): `per ADR-015 D-15.3` → `per D-15.3`.

**L-P7-002 [LOW] — line 38 row label "Standard" is asymmetric (row contains 2 block-mode hooks treated as exceptions)**
- Lines 35/36 use "Highest"; line 37 uses "Medium" (block-mode rows). Line 38 uses "Standard" despite containing 2 block-mode hooks.
- Defensible: row groups by file-read pattern; block-mode aspect captured as exception. (pending intent verification)

**L-P7-003 [LOW] — gap-analysis line 17 "BC-1.05.001..034" range vs ARCH-INDEX**
- ARCH-INDEX cites SS-01 has 106 BCs; the "..034" upper bound is repeated in 7+ places. Verifying upper bound requires exhaustive BC enumeration. Out of pass-7 tooling scope. (pending intent verification)

## Out-of-scope-but-noted

- Frontmatter convention drift across 3 arch docs (deferred per M-P6-001/M-P5-002): re-noted; not raising again.
- gap-analysis line 342 cites ADR-015 line 329: VERIFIED.

## Process-gaps

- **[process-gap] PG-P7-001:** TD-VSDD-061 closure-claim enumeration check should extend to "consistent wording template across sibling block-mode entries" — pass-6 caught the B-2/B-6 "all 5 named" gap; pass-7 caught the B-2/B-6 "named but with asymmetric wording" gap. The enumeration was applied; the wording template was not.
- **[process-gap] PG-P7-002:** Fix-burst-internal nomenclature leakage (M-P7-002): no current process check prevents internal IDs (H-N, M-PN, F-PN) from leaking into permanent specs. Recommendation: pre-commit check or adversary axis to scan permanent specs for `H-\d`, `M-P\d`, `F-P\d` patterns; flag in non-changelog sections.

## Convention checks

- Frontmatter `version:` matches latest non-reserved row (1.11): PASS
- v1.7-v1.11 summary rows intact (POLICY 1): PASS
- v1.12 preemptive reserved row: PASS (line 474)
- v1.11 H3 section present: PASS (line 799)
- v1.7-v1.10 block prose preserved as authored (POLICY 1 append-only): PASS — including v1.8 defective "Wave 3 AC-3" wording preserved per M-P5-001
- No "Lines: X → Y" footer at v1.7-v1.11: PASS
- H3 version count matches summary table: PASS (11 H3s for v1.1..v1.11)
- Closure-claim enumeration (TD-VSDD-061): PASS (v1.11 H3 enumerates all 5 block-mode hooks)

## Angle-specific outputs (Cross-doc consistency / contradiction hunt)

Block-mode hooks 5/5 confirmed across E-9 + audit-w16 (table elided for brevity).
Batch-to-story mapping B-N → S-9.0N agrees across all 4 files at the row level. Anomaly: line 38 parenthetical `(S-9.02–S-9.06)` contradicts row batch list — see M-P7-001.
ADR-015 D-15.x clauses (D-15.1, D-15.2, D-15.2.b, D-15.3, D-15.4, vsdd.capability.denied.* line 329, Wave 3 AC-2 line 634): all citations verify across the 4 files.
POLICY 6 SS-NN names: all 12 reference sites match ARCH-INDEX exactly.
OQ-W16-001 file: exists, well-formed; binary acceptance criterion (a)/(b) clean; ADR-015 line 634 citation verified.
POLICY 1 append-only audit: v1.7/v1.8/v1.9/v1.10 H3 prose all PRESERVED since their authoring bursts.
