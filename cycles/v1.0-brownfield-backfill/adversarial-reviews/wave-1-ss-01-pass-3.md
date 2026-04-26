---
document_type: adversarial-review
level: ops
version: "1.0"
status: complete
producer: adversary
timestamp: 2026-04-26T20:30:00Z
phase: 2-re-anchor
inputs:
  - .factory/stories/S-1.01-cargo-workspace-setup.md
  - .factory/stories/S-1.02-dispatcher-core.md
  - .factory/stories/S-1.04-host-function-surface.md
  - .factory/stories/S-1.05-wasmtime-integration.md
  - .factory/stories/S-1.06-tokio-parallel-tier-execution.md
  - .factory/stories/S-1.07-dispatcher-internal-log.md
  - .factory/stories/S-3.04-emit-event-host-function.md
input-hash: "3471ea6"
traces_to: ""
cycle: v1.0-brownfield-backfill
sub_cycle: wave-1-ss-01-re-anchor
pass: 3
previous_review: wave-1-ss-01-pass-2.md
verdict: FINDINGS_REMAIN
finding_count: 3
convergence_step: 0_of_3
po_commit_reviewed: 9a00ee3
---

# Adversarial Review — Wave 1 SS-01 Re-anchor — Pass 3

## Finding ID Convention

Pass-3 findings use `F-2NN` numbering.

## Part A — Pass-2 Fix Verification

| Finding | Pass-2 Severity | Fix Applied? | Evidence |
|---|---|---|---|
| F-101 | MED | YES | S-1.06 lines 73-75 (AC#3 disclaimer) + lines 131-135 (v1.1 BC candidate `BC-1.03.NNN-tier-skip-block`) |
| F-102 | MED | YES | capabilities.md line 39 reads `Subsystems: SS-01, SS-03, SS-10.` |
| F-103 | LOW | YES | S-1.04 line 100-101 (AC-007 OOB → BC-1.05.031); line 102-103 (AC-007b output-cap → BC-1.05.005) |
| F-104 | LOW | YES | S-1.04 lines 205-212 HTML comment documents semantic-faithful convention |

**Summary:** 4 of 4 fully fixed. No regressions.

## Part B — New Findings

### CAP Sweep Verification

PO claimed full 28-CAP sweep found no other gaps. Spot-check verification:

| CAP | Subsystems | Verdict |
|---|---|---|
| CAP-007 | SS-09, SS-01 | OK |
| CAP-008 | SS-01, SS-04, SS-07 | OK |
| CAP-027 | SS-07, SS-10 | LOW (pending intent verification — bin/emit-event source not inspected; if it routes through SS-01 internal_log, SS-01 should be added) |

PO sweep claim verified for CAP-007/008. CAP-027 plausible but cannot adjudicate without inspecting bin/emit-event implementation.

### Fresh-Context on Less-Audited Stories

### F-201 [MED] — S-1.02 AC#2 (`dispatcher_trace_id` UUID v4) traced to unrelated BCs

**Affected:** `.factory/stories/S-1.02-dispatcher-core.md` line 91-92

AC#2 claim: "`dispatcher_trace_id` generated per invocation (UUID v4)". Traces to BC-1.08.004 (cwd resolution) and BC-1.08.005 (plugin_root from CLAUDE_PLUGIN_ROOT).

Neither BC contracts UUID v4 trace_id generation. The dispatcher_trace_id flows through BC-1.06.007 (events with trace_id) and VP-017 (trace_id present on events) but no SS-01 BC contracts that the value is **generated as UUID v4 per invocation**.

Same pattern as pass-1 F-006 (lifecycle events) and pass-2 F-101 (tier-skip): AC primary topic uncontracted.

**Remediation:** v1.1 BC candidate "dispatcher generates UUID v4 for dispatcher_trace_id per invocation" + honest partial-coverage disclaimer.

### F-202 [MED] — S-1.07 AC#2 (log path from `CLAUDE_PLUGIN_ROOT`) traced to unrelated BCs

**Affected:** `.factory/stories/S-1.07-dispatcher-internal-log.md` line 74-75

AC#2 claim: "Log file path derived from `CLAUDE_PLUGIN_ROOT` env var". Traces to BC-1.06.003 (parent dirs auto-created) and BC-1.06.005 (prune_old no-op when missing).

Neither BC contracts CLAUDE_PLUGIN_ROOT-as-source. No BC-1.06.* covers log path source derivation.

**Remediation:** v1.1 BC candidate "InternalLogger derives log dir from CLAUDE_PLUGIN_ROOT env var" + disclaimer.

### F-203 [LOW, pending intent] — S-3.04 AC#3 (bin/emit-event deprecation) traces to RESERVED_FIELDS BC

**Affected:** `.factory/stories/S-3.04-emit-event-host-function.md` line 86-87

AC#3 claim: "`bin/emit-event` shell tool deprecated; existing callers migrated". Traces to BC-1.05.018 (RESERVED_FIELDS authoritative).

BC-1.05.018 is about reserved-field naming, not deprecation. AC#3 is openly marked `pending` in Partial Status table — bookkeeping stretch is acceptable.

**Remediation:** Optional. Either drop the BC trace or add v1.1 BC candidate for bin/emit-event retirement.

### Other Spot-Checks (all OK)

S-1.05 AC#1/2/10, S-1.07 AC#1/3, S-3.04 AC#1/5, S-1.02 AC#1/3 all verified semantic-faithful per F-104 convention.

## Part D — Self-Validation

All 3 findings re-grep'd + re-verified. No withdrawals. F-203 demoted from MED candidate to LOW due to Partial Status disclosure carrying load-bearing honesty.

## Observations

**[process-gap]** Recurring pattern detected (4 instances now: F-006, F-101, F-201, F-202): AC topics describing real implementation behavior with no SS-01 BC contract; trace cites adjacent BCs with mismatched subjects. Recommend pre-pass-4 reconciliation: PO scans ALL 7 wave-1 stories for AC topics whose cited BCs don't share subject, converts each to v1.1 BC candidate or honest disclaimer.

**[low priority]** CAP-027 sweep cannot be fully verified without reading `plugins/vsdd-factory/bin/emit-event` source.

## Novelty Assessment

| Field | Value |
|---|---|
| **Pass** | 3 |
| **New findings count** | 3 |
| **Duplicate count** | 0 (no pass-2 finding regressed) |
| **Novelty score** | 1.0 (3 of 3 are story-novel) |
| **Median severity** | MEDIUM |
| **Severity distribution** | 0 CRIT, 0 HIGH, 2 MED, 1 LOW |
| **Trajectory** | 10 → 4 → 3 (70% reduction from pass-1; no HIGHs since pass-1) |
| **Verdict** | FINDINGS_REMAIN |

## Convergence Status

**0 of 3** consecutive NITPICK-only passes per ADR-013.

Trajectory healthy. The recurring pattern (F-006, F-101, F-201, F-202) suggests a comprehensive AC-topic sweep before pass-4 would land NITPICK-only.

## Findings by Axis

| Axis | Findings |
|---|---|
| A — BC Existence | (none) ✓ |
| B — Semantic Anchoring | F-201, F-202, F-203 |
| C — Coverage Completeness | F-201, F-202 |
| D — AC↔BC Bidirectional | F-201, F-202, F-203 |
| E — Capability Justification | (none) ✓ |
| F — Subsystem/FR Hygiene | (none) ✓ |
| G — VP Soundness | (none) ✓ |
| H — CAP Choice | (none) ✓ |
| I — Spec-First Gate | (none) ✓ |
| J — POLICY 1 Reuse | (none) ✓ |
| K — Edge Cases | (none) ✓ |
| L — Bookkeeping | F-203 |

## Summary

Pass-3 verdict: FINDINGS_REMAIN (3 findings; 2 MED + 1 LOW).
Convergence: 0 of 3.
Trajectory: 10 → 4 → 3.
Recurring [process-gap] pattern: 4 instances of AC-topic-uncontracted-by-cited-BC. Recommend comprehensive sweep before pass-4.
