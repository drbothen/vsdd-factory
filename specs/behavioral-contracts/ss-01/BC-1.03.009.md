---
document_type: behavioral-contract
level: L3
version: "1.0"
status: draft
producer: codebase-analyzer
timestamp: 2026-04-25T00:00:00
phase: 1.4b
inputs: [bc-id-mapping.md, pass-3-behavioral-contracts.md]
input-hash: "[pending-recompute]"
traces_to: bc-id-mapping.md
origin: brownfield
extracted_from: ".factory/phase-0-ingestion/pass-3-behavioral-contracts.md:128"
subsystem: "SS-01"
capability: "CAP-TBD"
lifecycle_status: active
introduced: v1.0.0-beta.4
modified: []
deprecated: null
deprecated_by: null
replacement: null
retired: null
removed: null
removal_reason: null
---

# Behavioral Contract BC-1.03.009: `block_intent` set only when on_error=block AND plugin asks to block

## Description

`TierExecutionSummary.block_intent` is set to true only if a plugin returns `HookResult::Block { reason }` (stdout contains `"outcome":"block"`) AND its registry entry has `on_error = "block"`. Remaining plugins in the tier still fire; the final `exit_code = 2` if any block_intent is recorded.

## Preconditions

1. A plugin returns Block.
2. Its registry entry has `on_error = "block"`.

## Postconditions

1. `TierExecutionSummary.block_intent = true`.
2. Remaining plugins in the tier still execute.
3. Final `exit_code = 2`.

## Invariants

1. Block intent requires both plugin assertion AND registry policy.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Plugin returns Block but registry has `on_error = "continue"` | block_intent stays false |
| EC-002 | Plugin returns Continue / crashes / times out | block_intent stays false |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Plugin returns Block, on_error=block | block_intent=true, exit_code=2 | happy-path |
| Plugin returns Block, on_error=continue | block_intent=false | edge-case |
| Plugin crashes, on_error=block | block_intent=false | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| (TBD — to be assigned in Phase 1.6b) | | |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | TBD (anchor in Phase 1.5) |
| L2 Domain Invariants | TBD |
| Architecture Module | SS-01 — `crates/factory-dispatcher/src/executor.rs` |
| Stories | TBD (re-anchor in Phase 1.8 from S-N.MM stories) |

### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `executor.rs::tests::plugin_requests_block_detects_tagged_json` + plugin_requests_block_false_for_continue/crash/timeout |
| **Confidence** | HIGH (per Q3 design resolution) |
| **Extraction Date** | 2026-04-25 |
| **Extracted from** | `.factory/phase-0-ingestion/pass-3-behavioral-contracts.md` line `128` |

#### Evidence Types Used

- assertion (4 unit tests covering every branch)

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | TBD (Phase 1.6b will refine) |
| **Global state access** | TBD |
| **Deterministic** | TBD |
| **Thread safety** | TBD |
| **Overall classification** | TBD |

#### Refactoring Notes

(TBD — to be assessed in Phase 1.6b verification properties pass)
