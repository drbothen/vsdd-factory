# Adversarial Review — E-9 v1.7 Amendment (D-236) — Pass 1

**Date:** 2026-05-05
**Commit reviewed:** d9f2c86
**Files reviewed:** 4 (E-9 epic, gap-analysis-w16-subprocess.md, perf-baseline-w16.md, audit-w16.md)
**Verdict:** NITPICK_ONLY
**ADR-013 clock:** 0_of_3 → 1_of_3 (advance)

## Summary

The d9f2c86 amendment is a tightly scoped metadata-only absorption of ADR-015 awareness across 4 files. It correctly cites D-15.1/D-15.2/D-15.3/D-15.4 with semantic accuracy verified against ADR-015's actual text, introduces zero new BCs/VPs/FRs, preserves append-only changelog discipline, and explicitly flags both D-238 anomalies (`internal.capability_denied` missing `vsdd.` prefix and missing `vsdd.host.*` registry entry for `host.exec_subprocess.completed`) for downstream resolution. No HIGH or MED findings; 3 LOW findings worth recording but not blocking.

## Findings (severity-ordered)

### HIGH
none

### MED
none

### LOW

**LOW-1 (pending intent verification): Architecture files retain `version: "1.0"` despite v1.7 amendment content.**
- File:line evidence: `gap-analysis-w16-subprocess.md:4` (`version: "1.0"`), `audit-w16.md:5` (`version: "1.0"`), `perf-baseline-w16.md:6` (`version: "1.0"`).
- Rationale: All three architecture files received substantive amendment sections (gap-analysis "Post-Audit Amendment: ADR-015 Awareness", audit-w16 "Post-Audit Amendment", perf-baseline "ADR-015 Emit Overhead — N/A"), yet none bump the frontmatter `version`. The E-9 epic file was correctly bumped v1.6 → v1.7. The architect appears to have intentionally treated these as "in-place annotation" rather than versioned amendment. Cannot be adjudicated by adversary; depends on architect intent.
- Confidence: MEDIUM. Could be intentional convention (annotate-in-place for arch docs vs. version-bump for epics) or a propagation gap. Tagged `(pending intent verification)` per rule.

**LOW-2: `gap-analysis-w16-subprocess.md` Section 5 Gap 2 still cites event name without `vsdd.` prefix in pre-amendment body.**
- File:line evidence: `gap-analysis-w16-subprocess.md:215` (cites `host.exec_subprocess.completed` event without `vsdd.` namespace prefix); also Section 5 Gap 2 summary at line 301: `A "host.exec_subprocess.completed" event closes the observability gap`.
- Rationale: The new "Post-Audit Amendment" section at lines 307-348 correctly identifies and flags this anomaly (line 322-324). However, the pre-existing Section 5 / Section "Summary: Top 3 Gaps" prose still uses the non-conforming name. The amendment surfaces the issue but does not edit the originating prose. This is consistent with the v1.7 amendment scope (metadata-only, append-only annotations) but readers may grep the file and find the unprefixed name without the amendment context.
- Confidence: HIGH evidence; LOW severity because the amendment correctly flags the anomaly and the original prose is in scope-frozen body.

**LOW-3: `event.host_overrides` is included in ADR-015 D-15.2 but the v1.7 amendment block in E-9 D-9.2 does not enumerate it among plugin obligations.**
- File:line evidence: ADR-015 lines 274, 360-362 specify `event.host_overrides` as a host-stamped optional field that surfaces inline on the plugin's domain event when overrides occur. E-9 D-9.2 amendment block (epic file lines 282-303) enumerates `event.name`, `outcome` enum, `VSDD_TRACE_ID`/`VSDD_PARENT_SPAN_ID`, host enrichment generally — but does not call out that plugin authors will see `event.host_overrides` on their own emissions if they incorrectly stamp host-owned fields.
- Rationale: This is informational visibility for plugin authors during S-9.01..S-9.07 development. Omitting it does not create a contract violation (the host stamps it regardless of plugin awareness), but a story-writer authoring per this amendment could miss writing an AC for that test surface. The amendment's "Story-writer MUST incorporate ADR-015 compliance ACs" instruction is broad enough to cover this implicitly.
- Confidence: MEDIUM evidence; LOW severity because the amendment's high-level pointer to ADR-015 is enforceable when story-writer reads ADR-015 for AC authoring.

## Out-of-scope-but-noted

- **`input-hash: "37151a4"` in E-9 frontmatter (line 28) was not recomputed despite ADR-015 being added to the `inputs` array (line 23).** The validate-input-hash hook should detect this drift if/when the file is next written. The pre-existing input-hash field carries `[pending-recompute]` semantics in some workflow patterns (per v1.2 F-P2-010), but here it is a literal SHA. Out of scope: state-manager normally recomputes after architect commits, and D-238 was sealed. Adversary cannot adjudicate whether 37151a4 reflects post-amendment inputs or is stale; the hook will catch it on next write.
- **ADR-015 D-15.2.b says unrecognized prefixes resolve to `event.category = "unknown"`, NOT a hard failure.** The audit-w16.md amendment notes block-mode hooks "MUST emit `outcome = "blocked"`" (correct per D-15.3), which is a positive obligation. Nothing in scope flags it as drifted. Noting only for completeness — adversary verified semantic alignment.
- The E-9 changelog summary table row for v1.7 (line 466) reads "D-236 amendment — absorb ADR-015 single-stream OTel contract awareness before Burst 2 story authoring." This matches the v1.7 detail section header (line 661). Consistent.

## Process-gaps

none in this amendment surface. (The 4-file amendment process worked as designed: architect amended; state-manager sealed; adversary verifies. No prompt/hook/workflow gap surfaced.)

## Convention checks

- No "Lines: X → Y" footer in v1.6 or v1.7 sections: PASS
- Summary table v1.7 + v1.8 preemptive: PASS
- Changelog summary table v1.7 row: PASS
- H3 version count matches summary table: PASS

## Anomaly verification

- (a) `internal.capability_denied` lacks `vsdd.` prefix flagged in amendment? **PASS** — `gap-analysis-w16-subprocess.md:336-340` explicitly flags conformance issue for SS-01 implementers to address in E-10 Wave 1 or 2.
- (b) `host.exec_subprocess.completed` lacks `vsdd.host.*` registry entry flagged in amendment? **PASS** — `gap-analysis-w16-subprocess.md:322-324` explicitly flags need for canonical prefix confirmation with dispatcher team. (Verified ADR-015 lines 319-332 registry table contains no `vsdd.host.*` entry.)
