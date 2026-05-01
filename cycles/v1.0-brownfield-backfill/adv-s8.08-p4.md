---
document_type: adversarial-review
level: ops
version: "1.0"
status: complete
producer: adversary
timestamp: 2026-05-01T00:00:00Z
phase: 2
inputs:
  - .factory/stories/S-8.08-native-port-track-agent-start.md
  - .factory/cycles/v1.0-brownfield-backfill/adv-s8.08-p3.md
  - crates/hook-sdk/src/host.rs
  - .factory/specs/behavioral-contracts/ss-07/BC-7.03.079.md
  - .factory/stories/S-8.03-native-port-track-agent-stop.md
  - crates/hook-sdk/src/result.rs
input-hash: "e90faab"
traces_to: prd.md
pass: p4
previous_review: adv-s8.08-p3.md
target: story
target_file: .factory/stories/S-8.08-native-port-track-agent-start.md
verdict: NITPICK_ONLY
clock: 1_of_3
findings_critical: 0
findings_high: 0
findings_medium: 0
findings_low: 3
findings_nit: 0
---

# Adversarial Review: S-8.08 v1.3 (Pass 4)

## Finding ID Convention

Finding IDs use the format: `F-S808-P4-<SEQ>`

- `F`: Fixed prefix
- `S808`: Story identifier
- `P4`: Pass number
- `<SEQ>`: Three-digit sequence

## Part A — Fix Verification (4/4 closed)

| ID | Previous Severity | Status | Notes |
|----|-------------------|--------|-------|
| F-S808-P3-001 agent_id = host::session_id() | HIGH | VERIFIED | Goal line 146, AC-002a line 173, T-3 line 327 all cite host::session_id() |
| F-S808-P3-002 HookResult::Continue silent path | HIGH | VERIFIED | T-3 lines 317-318 show HookResult::Continue return on missing agent_type |
| F-S808-P3-003 BC trace AC-006 included | MEDIUM | VERIFIED | BC trace row for AC-006 present at line 113 |
| F-S808-P3-004 T-5 sink-file fixture | LOW | VERIFIED | T-5 fixture with sink-file path at lines 348-358 |

All 4 pass-3 findings confirmed CLOSED. No regressions.

## Part B — New Findings (3)

### LOW

#### F-S808-P4-001: Sibling S-8.03 does not emit agent_id; justification for new field absent (pending intent)

- **Severity:** LOW
- **Category:** spec-fidelity
- **Location:** S-8.08 Goal section and AC-002a
- **Description:** S-8.08 v1.3 introduces `agent_id` sourced from `host::session_id()`. Sibling S-8.03 (track-agent-stop) emits an analogous event without an agent_id field. The asymmetry may be intentional (S-8.08 adding a new telemetry field that S-8.03 lacks by design), an oversight (S-8.03 should also have agent_id), or a BC-misread vestige (agent_id was specified in a BC that doesn't require it). The story does not explain the asymmetry. An implementer reviewing both stories in parallel will be confused by the inconsistency. Pending orchestrator intent verification.
- **Proposed Fix:** Add a single sentence to the Goal section: "Note: S-8.03 (track-agent-stop) intentionally omits agent_id [OR: S-8.03 will be updated to include agent_id in a follow-on story]."

#### F-S808-P4-002: BC-7.03.079 registry line range mismatch between BC source and story citation

- **Severity:** LOW
- **Category:** spec-fidelity
- **Location:** S-8.08 BC trace table (T-6 row) and BC-7.03.079.md source
- **Description:** The story's T-6 (BC trace table) cites hooks-registry.toml at lines 763-781 for BC-7.03.079. The BC source file (BC-7.03.079.md:36) cites the same registry entry at lines 698-716. The registry has expanded since BC-7.03.079 was authored, moving the entry's line range. The BC source-of-truth (BC-7.03.079.md) is stale with respect to the current registry. The story's line range (763-781) appears to reflect a more recent registry read, but the BC file has not been updated to match.
- **Proposed Fix:** Update BC-7.03.079.md:36 registry line citation from 698-716 to the current range. The story's citation (763-781) should be taken as more recent; verify by running `grep -n "track-agent-start\|PreToolUseAgentStart" plugins/vsdd-factory/hooks-registry.toml`.

#### F-S808-P4-003: AC-002a bats fixture references track-agent-stop which does not emit agent_id

- **Severity:** LOW
- **Category:** spec-fidelity
- **Location:** S-8.08 AC-002a
- **Description:** AC-002a includes a bats fixture that reads from a track-agent-stop event sink file to verify agent_id is present. However, S-8.03 (track-agent-stop) does not emit agent_id. The fixture will always fail in a realistic test environment where both hooks are deployed, because the track-agent-stop sink file will not contain an agent_id field. T-5 handles this with a conditional fallback, but AC-002a wording is aspirational and inconsistent with T-5's actual test logic.
- **Proposed Fix:** Align AC-002a wording to match T-5's conditional fallback: "AC-002a: agent_id field present in track-agent-start sink output [verified via T-5 fixture using track-agent-start event directly, not cross-referencing track-agent-stop]."

## Sniff Verifications

| Check | Result |
|-------|--------|
| agent_id = host::session_id() consistent | CONFIRMED at Goal:146/AC-002a:173/T-3:327 |
| HookResult::Continue on missing agent_type | CONFIRMED at T-3:317-318 |
| BC-7.03.079 invariant 1 verbatim | CONFIRMED char-by-char against BC:45 |
| Sibling S-8.03 cross-check (agent_id asymmetry) | CONFIRMED asymmetry present; F-S808-P4-001 flagged |
| vsdd-hook-sdk path `../../hook-sdk` | PASS |
| emit_event slice-of-tuples form | PASS |
| wasm32-wasip1 target | PASS |

## Summary

| Severity | Count |
|----------|-------|
| CRITICAL | 0 |
| HIGH | 0 |
| MEDIUM | 0 |
| LOW | 3 |
| NIT | 0 |

**Overall Assessment:** nitpick only — advance clock
**Convergence:** converging (monotonic decay 12→11→4→3)
**Readiness:** spec ready for status=ready pending PO review and OQ-A1 closure

## Verdict

**NITPICK_ONLY** — 0 CRITICAL, 0 HIGH, 0 MED, 3 LOW, 0 NIT. Clock advances 0/3 → 1/3 per ADR-013 (first NITPICK_ONLY pass after pass-3 SUBSTANTIVE). Spec ready for status=ready pending PO review of agent_id asymmetry and BC-7.03.079 registry line update.

## Trajectory

| Pass | H | M | L | NIT | Total |
|------|---|---|---|-----|-------|
| p1 | 4 | 5 | 2 | 1 | 12 |
| p2 | 2 | 5 | 2 | 0 | 9 |
| p3 | 2 | 1 | 1 | 0 | 4 |
| p4 | 0 | 0 | 3 | 0 | 3 |

Monotonic decay confirmed across all four passes.

## Novelty Assessment

| Field | Value |
|-------|-------|
| **Pass** | 4 |
| **New findings** | 3 |
| **Closures** | 4 |
| **Novelty score** | 1.0 (3/3 novel; 1 pending-intent) |
| **Median severity** | LOW |
| **Trajectory** | 12→9→4→3 |
| **Verdict** | CONVERGING — clock 0/3 → 1/3 |
