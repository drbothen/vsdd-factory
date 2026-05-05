# Adversarial Review — E-9 v1.13 Combined Burst (D-251) — Pass 11 (CONVERGENCE CANDIDATE → SUBSTANTIVE)

**Date:** 2026-05-05
**Commit reviewed:** 088f46d (v1.13; unchanged)
**Cumulative surface:** v1.7..v1.13 (4 files + open-questions.md)
**Verdict:** SUBSTANTIVE
**ADR-013 clock at end of pass:** 2_of_3 → 0_of_3 RESET (convergence blocked)
**Pass methodology angle:** Numerical/quantitative consistency audit — every numeric claim (byte budgets, percentages, line counts, hook counts, ms latencies, MB thresholds, ADR line ranges) cross-validated for internal consistency and external accuracy across the 4 amendment files plus ADR-014/ADR-015. NEW per TD-VSDD-057.

## Summary

Pass-11 attacked along a numerical-consistency angle not used in passes 1-10. Five non-trivial numerics checked: (a) 23 hook count, (b) 5-of-23 block-mode count, (c) bundle budget thresholds, (d) cold-start latency numerics, (e) ADR-015 D-15.x line-range citations. **One HIGH finding** identified — contradiction between AC-3 (~14MB target) and ADR-014 line 45 + perf-baseline-w16.md actual value (643686 bytes ≈ 644KB). The "~14MB" figure has been formally superseded in ADR-014 yet survives in AC-3 — numerical mis-anchor that would mislead implementer reading AC-3 alone into measuring against obsolete target by 3 orders of magnitude. **One MED finding** — fix-burst-internal nomenclature leaks into open-questions.md (escalated from L-P9-001 LOW deferral due to TD-VSDD-063 codification).

Other numerics validated cleanly: 23 hook count consistent across all 3 architecture files; 5-of-23 block-mode count consistent; ADR-015 line ranges all cite real targets; cold-start numerics (500ms gate, 642.6ms measured, 706.9ms = 1.10×642.6) arithmetically self-consistent; per-plugin sum 8,549,146 verified; median 205160 verified at index 9 of 17; soft cap 643686 = 321843×2 verified.

## Findings

### HIGH

**H-P11-001 [HIGH]: AC-3 cites superseded "~14MB" advisory soft cap target; contradicts ADR-014 + perf-baseline-w16.md actual values**

- File: E-9 epic line 368 (AC-3). Cite: "advisory soft cap ≤ 100% cumulative growth at end of W-17 (~14MB)"
- Contradicting source 1 — ADR-014 line 45: "the prior ~14MB target derived from research §Q3's 7.2MB projection is **superseded — that figure was a projection, not a measurement**."
- Contradicting source 2 — perf-baseline-w16.md line 163: `w16_advisory_bundle_soft_cap_bytes | 643686 (= 321843 × 2)`. 643686 bytes ≈ **644 KB**, three orders of magnitude smaller than ~14MB.
- Contradicting source 3 — perf-baseline-w16.md line 175 Formula: `advisory_soft_cap = pre-W-15-baseline × 2 = 321843 × 2 = 643686 bytes`. The divisor for "100% growth" is 321,843 bytes (rc.1 baseline), NOT the rc.4 ~7.2MB cumulative. AC-3's "(~14MB)" parenthetical implies a 7.2MB × 2 calculation that ADR-014 explicitly retired.
- Why HIGH: AC-3 is the contractual gate. Implementer of any S-9.0N batch story computing "bundle delta vs ~14MB ceiling" would compute against wrong denominator (3 orders of magnitude off). Per ADR-014 line 45, ~14MB is **explicitly superseded**. This is exactly the propagation gap H-P11-001 documents.
- Note: This AC-3 was the target of v1.13's M-P8-001 fix in perf-baseline-w16.md line 156 (`E-9 D-9.4 → E-9 AC-3`). The fix re-anchored to AC-3 — but AC-3 itself contains an obsolete numerical citation. The fix's intent (point at gate-model authority) was right; the destination contains stale numbers.
- Recommended fix: Replace "(~14MB)" in AC-3 with reference to perf-baseline `w16_advisory_bundle_soft_cap_bytes = 643,686 bytes per ADR-014 R-8.09 Amendment`.

### MED

**M-P11-001 [MED]: Fix-burst-internal nomenclature leaks into permanent specs document open-questions.md**

- File: `.factory/specs/open-questions.md` line 20: `**Source:** D-247 pass-6 finding M-P6-002 (b04843d cycle)`
- Why MED: v1.12's TD-VSDD-063 explicitly codified "fix-burst-internal-nomenclature-leakage check". audit-w16.md / gap-analysis / perf-baseline bodies were swept clean. open-questions.md was filed in parallel by state-manager during D-248 and **carries the leak**. Per TD-VSDD-063's intent, this should resolve to neutral wording.
- (Note: This is L-P9-001 escalated. Pass-9 deferred as LOW per "TD-VSDD-063 recently-codified, scope ambiguity expected". Pass-11 escalates to MED since TD-VSDD-066 has now extended scan scope to register-class permanent specs.)
- Recommended fix: "Source: gap-analysis-w16-subprocess.md §'How ADR-015 affects the telemetry gap' (M-1 closure forward-pointer to OQ-W16-001)".
- Severity rationale: Single occurrence in single file; per TD-VSDD-063 the convention is MED floor, not HIGH. Hygiene defect.

### LOW

(none)

## Out-of-scope-but-noted

- Sampling Variance 706.9ms threshold in perf-baseline-w16.md line 322: 642.6 × 1.10 = 706.86 → 706.9. Verified.
- 17-plugin median calc lines 182-187: index 9 of 17 sorted = 205160 (session-end-telemetry). Verified.
- per_plugin sum lines 96-114: manually verified 8,549,146. Matches stated total.
- ADR-015 D-15.4 trace propagation cited at E-9 line 300, audit-w16 line 36, gap-analysis line 334. Verified at ADR-015 line 401-419.

## Process-gaps

- [process-gap]: TD-VSDD-063 (fix-burst-internal-nomenclature scan) was applied to audit-w16/gap-analysis/perf-baseline (per v1.12 closure check), but NOT extended to open-questions.md filed in same burst. Future TD-VSDD-063 sweeps must include all permanent specs touching amendment surface, including parallel-burst artifacts authored by state-manager. (TD-VSDD-066 codified at D-252 addresses this scope extension.)
- [process-gap]: Numerical-consistency angle (this pass-11) revealed AC-numbered values ("~14MB" in AC-3) were never cross-validated against underlying measurement source. No prior pass enumerated/verified every numeric claim across 4 amendment files. A `numeric-cross-anchor` review axis should be added to adversary checklist, complementing existing `decision-ID outbound semantic-anchor check` from TD-VSDD-065. File as TD-VSDD-067.

## Convention checks

- Frontmatter `version:` matches latest non-reserved row (1.13): PASS
- v1.7-v1.13 summary rows intact (POLICY 1): PASS
- v1.14 preemptive reserved row: PASS
- v1.13 H3 section present: PASS
- v1.7-v1.12 block prose preserved as authored (POLICY 1 append-only): PASS
- No "Lines: X → Y" footer at v1.7-v1.13: PASS
- H3 version count matches summary table: PASS
- audit-w16.md line 38 sibling-wording-template consistency: PASS
- No fix-burst-internal IDs leak into permanent specs body: **FAIL** — `D-247 pass-6 finding M-P6-002` in open-questions.md line 20
- Outbound decision-ID anchors semantically compatible (TD-VSDD-065): PASS (perf-baseline line 156 `E-9 AC-3` re-anchor verified — AC-3 IS the gate-model AC; AC-3 *content* contains obsolete "~14MB" but anchor target is correct)

## Angle-specific outputs (numerical consistency audit)

Verified numerics: 23 hook count, 5-of-23 block-mode, 500ms cold-start gate, 642.6ms measured, 30MB hard kill-switch, 8,549,146-byte sum, 643,686 = 321,843 × 2, ADR-015 line citations all verified.

Failed numeric: AC-3 "~14MB" advisory soft cap target — contradicted by ADR-014 line 45 ("superseded") and perf-baseline-w16.md line 163 (643686 bytes ≈ 644 KB). FAIL → H-P11-001.

Coverage note: audit-w16.md Section 5 R-W16-003 prose at lines 480-485 is the historical origin of the ~14MB figure that AC-3 inherited. ADR-014 line 45 explicitly retired this projection in favor of rc.1 × 2 = 643,686 model. AC-3's "(~14MB)" survived the ADR-014 retirement — exactly the propagation gap H-P11-001 documents.
