---
document_type: adversarial-review
level: ops
version: "1.0"
status: complete
producer: adversary
timestamp: 2026-04-26T23:55:00Z
phase: 2-re-anchor
inputs:
  - .factory/stories/S-1.08-sink-file-driver.md
  - .factory/stories/S-1.09-sink-otel-grpc-driver.md
  - .factory/stories/S-4.01-sink-http-driver.md
  - .factory/stories/S-4.02-sink-datadog-driver.md
  - .factory/stories/S-4.03-sink-honeycomb-driver.md
  - .factory/stories/S-4.04-retry-circuit-breaker.md
  - .factory/stories/S-4.05-dead-letter-queue.md
  - .factory/stories/S-4.06-routing-tag-enrichment.md
  - .factory/stories/S-4.07-observability-integration-tests.md
  - .factory/specs/prd.md
input-hash: "8388f56"
traces_to: ".factory/specs/prd.md"
cycle: v1.0-brownfield-backfill
sub_cycle: wave-2-ss-03-re-anchor
pass: 3
verdict: FINDINGS_REMAIN
finding_count: 3
convergence_step: 0_of_3
po_commit_reviewed: 443c8ba
previous_review: wave-2-ss-03-pass-2.md
---

# Adversarial Review — Wave 2 SS-03 Re-anchor — Pass 3

## Finding ID Convention

Pass-3 findings use `F-2NN`.

## Part A — F-012 Fix Verification

**VERIFIED CLEAN** at commit `443c8ba`. S-4.06 AC#2/#3 traces correctly cite v1.1 candidate `BC-3.NN.NNN-tag-enrichment-wired-in-dispatch` + acknowledge BC-3.02.010/011 as Related Contracts siblings. No remaining frontmatter↔AC mismatches in S-4.06.

## Part B — New Findings (3 MEDIUM)

### F-013 [MED] — S-4.05 AC#2 cites BC-3.03.002 not in frontmatter (sibling-not-updated regression of F-012 fix pattern)

**Affected:** S-4.05 line 70-71 (AC#2)

AC#2: "Events written to DLQ when all retries exhausted (traces to BC-3.03.002 postcondition 1: self-healing reconnect-on-error is the base error path; DLQ write on retry exhaustion is uncontracted — v1.1 BC candidate ...)"

But S-4.05 frontmatter `behavioral_contracts:` (lines 20-23) only lists BC-3.02.013 + BC-3.02.015. BC-3.03.002 is NOT in frontmatter. No "Related Contracts (informational, not authoritative)" section exists in S-4.05 (unlike S-4.06 which got one in F-012 fix).

**Pattern recurrence:** This is the **3rd instance** of "frontmatter↔AC drift on demoted/missing BCs":
- F-006 (pass-1): S-4.06 had BC-3.02.010/011 anchored but file-sink-scoped → demoted from frontmatter
- F-012 (pass-2): S-4.06 AC#2/#3 still cited the demoted BCs → re-traced
- **F-013 (pass-3): S-4.05 same pattern with BC-3.03.002**

**Remediation:** Apply S-4.06's F-012 fix pattern to S-4.05:
1. Rewrite AC#2 trace: "(traces transitively to BC-3.03.002 postcondition 1 — otel-grpc-scoped sibling pattern listed under Related Contracts; DLQ write on retry exhaustion is uncontracted — v1.1 BC candidate `BC-3.NN.NNN-dlq-write-on-retry-exhaustion`. See v1.1 BC Candidates table.)"
2. Add "Related Contracts (informational, not authoritative)" section to S-4.05 body listing BC-3.03.002 as otel-grpc-scoped sibling pattern.

### F-014 [MED] — S-4.07 frontmatter declares BC-3.01.002 but no AC traces to it

**Affected:** S-4.07 line 22 (frontmatter), lines 95-114 (no AC traces)

S-4.07 frontmatter line 22 includes BC-3.01.002. Body BC table line 121 has it with rationale "Integration tests include configs with unknown sink types." But no AC explicitly traces to it (verified by reading all 10 ACs).

This violates Story Frontmatter-Body Coherence axis: every BC in `behavioral_contracts:` should be referenced by at least one AC.

**Remediation:** Either:
1. Add an AC: "Test: unknown sink type in config does not fail load (traces to BC-3.01.002 postcondition 1: unknown sink type warns to stderr but does not fail config load)"
2. Demote BC-3.01.002 from frontmatter to "Related Contracts (informational)" section if it's only transitively exercised.

### F-015 [MED] — PRD §2.3 FR-044 prose arithmetic error: says "4 v1.1 BC candidates" but enumerates 8

**Affected:** `.factory/specs/prd.md` line 340

FR-044 §2.3 prose: "v1.1 BC candidates pending: 4 (S-4.04: retry policy, CB state machine, CB event emission, retry isolation; S-4.05: DLQ on exhaustion, DLQ on overflow, DLQ event shape, DLQ disk-full no-crash)."

Counting: S-4.04 = 4 + S-4.05 = 4 = **8 total**. Matrix row line 1074 confirms: "8 pending."

**Remediation:** Change `pending: 4` to `pending: 8` in line 340.

### LOW Observation (not a finding)

S-1.08 frontmatter declares BC-3.02.002, BC-3.02.003, BC-3.02.004 (template name_only / project_basename / all_placeholders unit-test BCs). Body BC table lists with rationale, but no individual AC explicitly traces to them. May be intentionally subsumed by BC-3.01.006's abstract trace. Pending intent verification — not a regression.

## Part C — Self-Validation

3 rounds of refinement. 2 candidates withdrawn:
- BC-3.01.008 in S-4.05 body Note (correct F-006/F-012-pattern usage)
- S-1.08 placeholder unit-test BCs missing AC traces (demoted to LOW observation pending intent)

3 substantive findings retained.

## Summary

| Severity | Count |
|----------|-------|
| CRITICAL | 0 |
| HIGH | 0 |
| MEDIUM | 3 |
| LOW | 1 (observation) |

**Overall Assessment:** pass-with-findings
**Convergence:** findings remain — iterate
**Readiness:** requires revision

## Novelty Assessment

| Field | Value |
|---|---|
| **Pass** | 3 |
| **New findings count** | 3 |
| **Duplicate count** | 0 (F-013 is recurrence of pattern but not duplicate of any specific prior finding) |
| **Novelty score** | 1.0 |
| **Median severity** | MEDIUM |
| **Severity distribution** | 0 CRIT, 0 HIGH, 3 MED, 1 LOW (observation) |
| **Trajectory** | 11 → 1 → 3 (count rebounded; severity continues to decline) |
| **Verdict** | FINDINGS_REMAIN |

**Pattern flag:** F-013 is the 3rd recurrence of "frontmatter↔AC drift on demoted/missing BCs" — orchestrator should run comprehensive sibling-grep sweep this fix burst to break the cycle.

## Convergence Status

**0 of 3** consecutive NITPICK-only passes. Counter resets — pass-3 is not clean.

## Findings by Axis

| Axis | Pass-3 Findings |
|---|---|
| Story Frontmatter-Body Coherence | F-013 (S-4.05 AC→missing frontmatter); F-014 (S-4.07 frontmatter→missing AC) |
| Partial-Fix Regression Discipline | F-013 (sibling-not-updated 3rd recurrence) |
| PRD self-consistency (arithmetic) | F-015 |

## Trajectory

| Pass | Findings | Severity Mix |
|---|---|---|
| Pass-1 | 11 | 2C/4H/4M/1L |
| Pass-2 | 1 | 1H |
| Pass-3 | 3 | 3M |

Severity declining; sibling-pattern recurrence is the dominant residual theme. Recommend comprehensive grep sweep this fix burst.

## Observations

- **[process-gap]** Pattern recurrence F-006 → F-012 → F-013 (3rd instance) — partial-fix-discipline sibling-sweep gate not run during F-006 or F-012 fixes. Codify into PO fix-burst checklist: "When demoting a BC from frontmatter or applying Related Contracts pattern, grep all sibling stories for `traces to BC-X` where BC-X is no longer in that story's frontmatter."
- **[low]** PRD §2.3 prose drift (F-015) shows the FR-044 addition wasn't fully proofread for arithmetic consistency with the matrix row.
