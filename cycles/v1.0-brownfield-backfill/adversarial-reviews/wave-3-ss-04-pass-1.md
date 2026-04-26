---
document_type: adversarial-review
level: ops
version: "1.0"
status: complete
producer: adversary
timestamp: 2026-04-27T04:00:00Z
phase: 2-re-anchor
inputs:
  - .factory/stories/S-2.01-legacy-bash-adapter.md
  - .factory/stories/S-3.01-port-capture-commit-activity.md
  - .factory/stories/S-3.02-port-capture-pr-activity.md
  - .factory/stories/S-3.03-port-block-ai-attribution.md
  - .factory/stories/S-5.01-session-start-hook.md
  - .factory/stories/S-5.02-session-end-hook.md
  - .factory/stories/S-5.03-worktree-hooks.md
  - .factory/stories/S-5.04-post-tool-use-failure.md
  - .factory/specs/prd.md
  - .factory/specs/domain-spec/capabilities.md
input-hash: "30b337b"
traces_to: ".factory/specs/prd.md"
cycle: v1.0-brownfield-backfill
sub_cycle: wave-3-ss-04-re-anchor
pass: 1
verdict: FINDINGS_REMAIN
finding_count: 11
convergence_step: 0_of_3
po_commit_reviewed: b242d67
previous_review: null
---

# Adversarial Review — Wave 3 SS-04 Re-anchor — Pass 1

## Finding ID Convention

Pass-1 findings use `F-0NN`.

## Part B — New Findings (11 total: 0 CRIT, 4 HIGH, 4 MED, 3 LOW)

### F-001 [HIGH] — BC-4.03.001 stretch-anchored to 6 stories with non-overlapping plugin scope

S-3.01 has legitimate anchor (BC's H1 names S-3.01 as replacement). For S-3.02, S-5.01-04, BC-4.03.001 is purely structural-template metaphor — its actual postconditions don't contract those stories' work.

**Resolution paths:** (a) remove from 5 stories with empty `behavioral_contracts:` + v1.1 disclaimer, OR (b) sanction the Wave 2 F-007 pattern with explicit disclosure.

### F-002 [HIGH] — S-3.03 anchors legacy-bash-adapter BCs to native WASM plugin

S-3.03 produces `block-ai-attribution.wasm` (native), but anchors BC-4.02.002/003 (legacy-bash-adapter scope). HookResult exit codes for native plugins are governed by BC-2.01.002 (SS-02 SDK).

**Fix:** re-anchor to BC-2.01.002 + add SS-02 to subsystems frontmatter; v1.1 candidate for pure-core pattern detection.

### F-003 [HIGH] — FR-045 proposal has internally inconsistent scope

S-3.02 proposes FR-045 = "capture-pr-activity"; S-5.01/S-5.04 propose FR-045 = "lifecycle hook events". Two different scopes claiming same ID.

**Fix:** FR-045 = lifecycle events (S-5 scope, has DRIFT-006 backing). Renumber S-3.02 proposal to FR-046 or fold into FR-014.

### F-004 [HIGH] — S-3.03 traces_to FR-013 (legacy adapter) vs functionally FR-032 (PreToolUse gates)

S-3.03 implements PreToolUse blocking (FR-032/SS-07 conceptual home) but traces_to FR-013 (legacy bash adapter).

**Fix:** dual-anchor `functional_requirements: ["FR-013", "FR-032"]`.

### F-005 [MED] — Architecture Compliance Rules cite stale BC ID prefixes

S-3.01/S-3.02/S-3.03 cite BC prefixes ("BC-4.03", "BC-2.01", "BC-1.03", "BC-1.01") without canonical full IDs.

**Fix:** Update each row to BC-S.SS.NNN format.

### F-006 [MED] — S-5.01-04 frontmatter dual-subsystems [SS-01, SS-04] in "Wave 3 SS-04" cycle

PRD FR-007 lists subsystems "SS-01, SS-04" → frontmatter is internally coherent. But cycle name "SS-04 re-anchor" is misleading.

**Fix:** Document cycle scope expansion in STATE.md when Wave 3 closes; no story changes.

### F-007 [MED] — S-3.01 AC traces semantically vacuous

S-3.01 AC#5 ("wasm compiles") cites BC-4.03.001 postcondition 1 ("stub returns 0") — should cite postcondition 2 ("crate compiles"). AC#7 ("bats tests still pass") has no contracting BC.

**Fix:** Re-trace AC#5 to postcondition 2; mark AC#7 as `[process-gap]`.

### F-008 [MED] — S-3.01 body VP scoping mis-grouped

Body line 58 calls VP-043/044/045 all "legacy-bash-adapter scoped" — VP-043 is SS-07/SS-01.

**Fix:** Correct body text with per-VP scope.

### F-009 [LOW] — PRD §14 "43 FRs" vs §7 "44 FRs"

Pre-existing PRD count drift; out of Wave 3 scope. Reconcile when adding FR-045 to PRD.

### F-010 [LOW] — BC-INDEX SS-04 Stories column "TBD"

Pre-existing across all subsystems; accepted pattern (Waves 1+2 also did not fix).

### F-011 [LOW] — SS-04 architecture doc stale BC ID format

References "BC-4.001-BC-4.030" single-segment range; actual IDs are BC-4.NN.NNN three-segment. Out of Wave 3 PO scope.

## CAP Subsystem Drift Sweep — CLEAN

Wave 3 found NO CAP→SS-04 subsystem drift. CAP-002, CAP-008, CAP-013 all correctly include SS-04. Breaks Wave 1+2 recurring pattern.

## Summary

| Severity | Count |
|----------|-------|
| CRITICAL | 0 |
| HIGH | 4 |
| MEDIUM | 4 |
| LOW | 3 |

**Overall Assessment:** pass-with-findings
**Convergence:** findings remain — iterate
**Readiness:** requires revision

## Novelty Assessment

| Field | Value |
|---|---|
| **Pass** | 1 |
| **New findings count** | 11 |
| **Duplicate count** | 0 |
| **Novelty score** | 1.0 |
| **Median severity** | HIGH |
| **Severity distribution** | 0 CRIT, 4 HIGH, 4 MED, 3 LOW |
| **Trajectory** | starting baseline (11) |
| **Verdict** | FINDINGS_REMAIN |

## Convergence Status

**0 of 3.** 4 HIGH semantic-anchoring findings block convergence per BC-5.04.003.

## Findings by Axis

| Axis | Findings |
|---|---|
| Semantic Anchoring (B) | F-001, F-002 |
| FR/Subsystem Hygiene (F) | F-003, F-004, F-006 |
| AC↔BC Bidirectional (D) | F-007 |
| Bookkeeping (L) | F-005, F-008 |
| Pre-existing TD/out-of-scope | F-009, F-010, F-011 |

## Trajectory Baseline

Pass-1 = 11 findings. Wave comparison:
- Wave 1 pass-1: 10 (3H/4M/3L)
- Wave 2 pass-1: 11 (2C/4H/4M/1L)
- Wave 3 pass-1: 11 (4H/4M/3L) — heavier on HIGH semantic-anchoring than Wave 1, less critical than Wave 2

## Verdict

**FINDINGS_REMAIN.** 4 HIGH semantic anchoring findings require remediation. F-001 BC-4.03.001 stretch-anchor pattern needs orchestrator adjudication.
