# Adversarial Review — E-9 v1.13 Combined Burst (D-251) — Pass 9

**Date:** 2026-05-05
**Commit reviewed:** 088f46d (v1.12 → v1.13)
**Cumulative surface:** v1.7..v1.13 (4 files + open-questions.md)
**Verdict:** NITPICK_ONLY
**ADR-013 clock at end of pass:** 0_of_3 → 1_of_3 advance
**Pass methodology angle:** POLICY 6 deep-dive (SS-NN canonical-equality audit across all in-scope files) + OQ-W16-001 lifecycle audit. NEW per TD-VSDD-057. For every SS-NN reference across the 5 in-scope files, verify exact case-, hyphen-, and name-equality against ARCH-INDEX Subsystem Registry. Verify OQ-W16-001 register entry's structural integrity, owner sensibility, acceptance-criterion binarity, ADR-015 D-15.2 registry consistency.

## Summary

Pass-9 ran a fresh canonical-name audit on all SS-NN references in the v1.7..v1.13 surface plus a structural-integrity sweep on OQ-W16-001. **All 42 SS-NN occurrences across 5 files map exactly to ARCH-INDEX Subsystem Registry.** No case-mismatches, hyphen-vs-space drift, or fabricated subsystem IDs. **OQ-W16-001 has correctly-anchored owner (SS-01), well-formed binary acceptance criterion, valid fallback `vsdd.dispatcher.subprocess_completed.v1` mapping to lifecycle category per ADR-015 line 319.** Numerical-consistency cross-check (median 205160, soft cap 643686, per-plugin cap 615480, p95 706.9, per-plugin sum 8549146, grand_total 20955111) all reconcile. The v1.13 line-156 fix is clean. Frontmatter v1.13 = latest non-reserved row. POLICY 1 append-only intact.

**One LOW process-gap finding:** OQ-W16-001 (open-questions.md line 20) `Source:` field carries raw fix-burst-internal ID `M-P6-002`. TD-VSDD-063 was codified at v1.12 specifically to prevent this leak class — but its scan scope was architect docs (audit-w16, gap-analysis, perf-baseline) and missed open-questions register, a peer permanent-spec surface with same leak susceptibility.

## Findings

### HIGH
None.

### MED
None.

### LOW

**L-P9-001 [LOW] [process-gap] (pending intent verification): OQ-W16-001 `Source:` field leaks fix-burst-internal ID into permanent OQ register**

- File: `/Users/jmagady/Dev/vsdd-factory/.factory/specs/open-questions.md` line 20: `**Source:** D-247 pass-6 finding M-P6-002 (b04843d cycle)`
- Pattern signal: SAME LEAK CLASS that TD-VSDD-063 was created to prevent, recurring in a peer permanent-spec file. Per Sibling-files Partial-Fix Regression Discipline S-7.01: if a fix applied to architect docs in a subsystem, the same pattern should be checked in sibling permanent-spec files of the same role.
- Fix: Either (a) reword line 20 `**Source:** D-247 pass-6 (b04843d cycle)` (drop the `finding M-P6-002` token); or (b) extend TD-VSDD-063 scan scope to include `.factory/specs/open-questions.md` and any future register-class permanent-spec files, then re-run scan and apply (a). 
- Severity rationale: LOW because (i) single-line cosmetic blemish in single file; (ii) TD-VSDD-063 is recently-codified and scope ambiguity at boundary expected; (iii) leak in `**Source:**` traceability field, not normative OQ prose; (iv) OQ acceptance criterion and resolution path unaffected.
- (pending intent verification) per S-7.01 — architect may have wanted inbound traceability anchor to survive.

## Out-of-scope-but-noted

- audit-w16.md line 480-481 has per-plugin sum `~7.2 MB` arithmetically yielding ~8.2 MB. From original W-16 audit (v1.0, 2026-05-03) — pre-amendment, not in v1.7..v1.13 fix scope. Author's intent likely "approximate W-15 baseline projection"; actual measurement now lives in perf-baseline-w16.md (8549146 bytes ≈ 8.5 MB). NOT a finding for this pass.

## Process-gaps

- See L-P9-001. Recommended codification: extend TD-VSDD-063 pre-commit scan scope from architect docs to `.factory/specs/**` (open-questions register, future similar register-class artifacts). File as TD-VSDD-066.

## Convention checks

- Frontmatter `version:` matches latest non-reserved row (1.13): PASS
- v1.7-v1.13 summary rows intact (POLICY 1): PASS
- v1.14 preemptive reserved row: PASS
- v1.13 H3 section present: PASS (line 944)
- v1.7-v1.12 block prose preserved as authored (POLICY 1 append-only): PASS — including v1.10's explicit preservation of v1.8 historical "Wave 3 AC-3" wording per D-245
- No "Lines: X → Y" footer at v1.7-v1.13: PASS
- H3 version count matches summary table: PASS
- audit-w16.md line 38 sibling-wording-template consistency: PASS
- No fix-burst-internal IDs leak into permanent specs body: FAIL (open-questions.md line 20 — see L-P9-001)
- Outbound decision-ID anchors semantically compatible (TD-VSDD-065): PASS

## Angle-specific outputs

POLICY 6 deep-dive: 42 SS-NN occurrences total across 5 files. Zero canonical-name drift, zero case mismatches, zero hyphen-vs-space variations. SS-01 (Hook Dispatcher Core), SS-02 (Hook SDK and Plugin ABI), SS-04 (Plugin Ecosystem), SS-07 (Hook Bash Layer) — all canonical names match ARCH-INDEX lines 74-83 exactly.

OQ-W16-001 lifecycle audit: heading hierarchy correct (H2 under H1); Status OPEN valid; Owner SS-01 anchors correctly (exec_subprocess.rs lives in crates/factory-dispatcher/src/host/, SS-01 BC-1); Filed 2026-05-05; Question well-posed (cites ADR-015 lines 317-332); Acceptance criterion binary (a)/(b) well-formed; fallback `vsdd.dispatcher.subprocess_completed.v1` correctly maps to lifecycle per ADR-015 line 319 (registry table); Why-this-matters cites D-15.2.b + Wave 3 AC-2 (line 634); Resolution path concrete; Cross-doc sync with gap-analysis line 326 verified bidirectional. Structurally sound and content-correct.
