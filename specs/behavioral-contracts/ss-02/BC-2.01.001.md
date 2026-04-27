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
extracted_from: ".factory/phase-0-ingestion/pass-3-behavioral-contracts.md:336"
subsystem: "SS-02"
capability: "CAP-009"
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

# Behavioral Contract BC-2.01.001: HookResult serialization is tagged with `outcome` field

## Description

`HookResult` JSON serialization carries an explicit `outcome` discriminator. `Continue` → `{"outcome":"continue"}`; `Block { reason }` → `{"outcome":"block","reason":...}`; `Error { message }` → `{"outcome":"error","message":...}`. The dispatcher's tier-execution logic depends on this tag to detect block intent.

## Preconditions

1. A `HookResult` variant is being serialized to JSON.

## Postconditions

1. `Continue` serializes to `{"outcome":"continue"}`.
2. `Block { reason }` serializes to `{"outcome":"block","reason":...}`.
3. `Error { message }` serializes to `{"outcome":"error","message":...}`.

## Invariants

1. The `outcome` tag is the canonical discriminator; round-trip preserves variant identity.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | (No edge cases captured in Phase 0 extraction; to be added in Phase 1.5/test-writer pass) | TBD |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| `HookResult::Continue` | `{"outcome":"continue"}` | happy-path |
| `HookResult::Block { reason: "x" }` | `{"outcome":"block","reason":"x"}` | edge-case |
| `HookResult::Error { message: "boom" }` | `{"outcome":"error","message":"boom"}` | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| (TBD — to be assigned in Phase 1.6b) | | |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-009 |
| L2 Domain Invariants | TBD |
| Architecture Module | SS-02 — `crates/hook-sdk/src/result.rs` |
| Stories | S-1.03 |

### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `crates/hook-sdk/src/result.rs` (4 unit tests in tests::{continue_serializes_with_outcome_tag, block_serializes_with_reason, error_serializes_with_message, round_trip_block}) |
| **Confidence** | HIGH |
| **Extraction Date** | 2026-04-25 |
| **Extracted from** | `.factory/phase-0-ingestion/pass-3-behavioral-contracts.md` line `336` |

#### Evidence Types Used

- assertion (4 unit tests)

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
