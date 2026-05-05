# Adversarial Review — E-9 v1.7 Amendment (D-236) — Pass 2

**Date:** 2026-05-05
**Commit reviewed:** d9f2c86
**Files reviewed:** 4 (E-9 epic, gap-analysis-w16-subprocess.md, perf-baseline-w16.md, audit-w16.md)
**Verdict:** NITPICK_ONLY
**ADR-013 clock at end of pass:** 2_of_3 advance
**Pass methodology angle:** Reverse-trace ADR-015 D-15.1/D-15.2/D-15.3/D-15.4 obligations FROM the ADR text TO their landing sites in the 4 amendment files; arithmetic check on changelog summary table (row count, H3 count, reserved-row exclusion); intra-amendment internal-consistency check (does any single amendment block contradict itself?).

## Summary
The d9f2c86 amendment correctly absorbs ADR-015 awareness into all 4 in-scope files. ADR-015 cross-refs (D-15.1 single-stream, D-15.2 reverse-DNS + outcome enum, D-15.3 enrichment + block-path, D-15.4 trace injection) are semantically accurate against the ADR-015 source. The two D-238-flagged anomalies (`internal.capability_denied` missing `vsdd.` prefix; `vsdd.host.*` not in registry) are explicitly flagged in the amendment text — the anomalies are surfaced, not papered over. Changelog convention checks (no Lines: footer at v1.6/v1.7; preemptive v1.8 row; H3 count vs. summary table) all PASS. Two LOW observations on amendment phrasing precision; no HIGH or MED defects found.

## Findings (severity-ordered)

### HIGH
none

### MED
none

### LOW

**LOW-1 — Internal tension in gap-analysis amendment between "MUST" and "pending confirmation"**
File: `/Users/jmagady/Dev/vsdd-factory/.factory/architecture/gap-analysis-w16-subprocess.md` lines 320–324
Confidence: HIGH (textual contradiction within a single amendment block)
The amendment says: "The event MUST use `event.name = \"vsdd.host.exec_subprocess.completed.v1\"`" — and then the very next clause says "a `vsdd.host.*` prefix would need a registry entry — story-writer or SS-01 implementer must confirm the canonical prefix for this event family with the dispatcher team." A "MUST use X where X is not yet a confirmed canonical prefix" reads as a soft-MUST. Suggested phrasing: "SHOULD tentatively use `vsdd.host.exec_subprocess.completed.v1` pending registry-prefix confirmation per D-15.2." This is a polish nit, not a substantive defect — the contradiction resolves to "use this name as a placeholder pending dispatcher-team confirmation," which the surrounding prose makes clear in context.

**LOW-2 — perf-baseline amendment closes an open prediction without a measurement gate**
File: `/Users/jmagady/Dev/vsdd-factory/.factory/architecture/perf-baseline-w16.md` lines 351–355
Confidence: MEDIUM
The amendment asserts ADR-015 FileSink emit overhead is "negligible (sub-millisecond I/O)" and downstream waves "should NOT attribute emit-path overhead to ADR-015 unless profiling evidence suggests otherwise." This is a prediction, not a measured baseline. At 23 plugins × N events per invocation, cumulative emit overhead (file open + append + close per event, with possible fsync) could become measurable enough to matter against the existing 642.6ms cold-start (already over 500ms gate). The amendment's "negligible per ADR-015 D-15.1 rationale" is correct in expectation but lacks a downstream measurement obligation (e.g., "S-9.01 MUST measure per-emit overhead and confirm < 1ms per event"). Polish nit only — the rationale citation is accurate against ADR-015 lines 433–440; a stronger amendment would add a forward-pointer task. (pending intent verification — the architect may have intentionally deferred measurement to the implementer-author workflow, not the amendment.)

## Out-of-scope-but-noted

- v1.5 changelog body at line 648 retains the `Lines: v1.4 (~614L) → v1.5 (~622L; +8L)` footer. This is INSIDE the v1.5 section authored at pass-6 BEFORE the v1.6 convention drop. Per POLICY 1 append-only, v1.5 history is intentionally preserved as authored. NOT a v1.7 finding.
- Frontmatter `producer: story-writer` (line 15) was not updated to reflect that v1.7 was authored by `architect` (per changelog row line 466). Field appears to denote original-author identity; amendments are tracked in changelog. Out-of-scope-but-noted as [process-gap] candidate.

## Process-gaps

none

## Convention checks

- **No "Lines: X → Y" footer in v1.7 section:** PASS — v1.7 section (lines 661–683) contains NO `Lines:` footer.
- **No "Lines: X → Y" footer in v1.6 section:** PASS — v1.6 section (lines 650–659) contains NO `Lines:` footer.
- **Summary table v1.7 + preemptive v1.8:** PASS — line 466 v1.7 row + line 467 v1.8 reserved row.
- **Changelog summary table v1.7 row:** PASS.
- **H3 version count matches summary table (excluding reserved):** PASS — 7 H3 sections; summary table 9 rows minus v1.0 (initial) minus v1.8 (reserved) = 7 expected; 7 = 7.

## Anomaly verification

- **(a) `internal.capability_denied` lacks `vsdd.` prefix flagged?** PASS — gap-analysis-w16-subprocess.md lines 339–340.
- **(b) `host.exec_subprocess.completed` lacks `vsdd.host.*` registry entry flagged?** PASS — gap-analysis-w16-subprocess.md lines 321–324.

## Numerical/arithmetic verifications

1. D-15.2 outcome enum copy fidelity: 6 values match (`success | failure | error | timeout | skipped | blocked`). E-9 v1.7 line 295 ↔ ADR-015 line 270.
2. D-15.2 prefix registry coverage: `vsdd.hook.*` MATCH (line 326); `vsdd.host.*` NOT IN REGISTRY (correctly flagged); `vsdd.internal.*` MATCH (line 331).
3. Changelog summary 9 rows − v1.0 (no H3) − v1.8 (reserved) = 7 expected H3 sections; 7 found at lines 469, 527, 599, 628, 641, 650, 661.
4. D-15.4 trace-vars naming: `VSDD_TRACE_ID` + `VSDD_PARENT_SPAN_ID` MATCH (E-9 v1.7 line 297 ↔ ADR-015 line 408).
5. Block-mode batch attribution: validate-input-hash B-2 (E-9 line 134) + validate-template-compliance B-6 (E-9 line 167) MATCH audit-w16 line 38.
