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
extracted_from: ".factory/phase-0-ingestion/pass-3-behavioral-contracts.md:348"
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

# Behavioral Contract BC-2.01.003: HOST_ABI_VERSION is 1 in both crates

## Description

Both `factory-dispatcher::HOST_ABI_VERSION` and `vsdd_hook_sdk::HOST_ABI_VERSION` are declared as `pub const HOST_ABI_VERSION: u32 = 1;` in their respective `lib.rs`. A mismatch between dispatcher and SDK ABI version is a major-version event by the project's stability policy.

## Preconditions

1. The compiled artifacts of `factory-dispatcher` and `vsdd-hook-sdk` are linked or paired at runtime.

## Postconditions

1. `factory_dispatcher::HOST_ABI_VERSION == 1`.
2. `vsdd_hook_sdk::HOST_ABI_VERSION == 1`.
3. Both constants are equal.

## Invariants

1. ABI version equality is enforced cross-crate as a release-gating invariant.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Future bump to 2 in dispatcher only | Major-version mismatch; would require coordinated bump |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Compile both crates and assert equality | Both equal 1 | happy-path |
| TBD | TBD | edge-case |
| TBD | TBD | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| (TBD — to be assigned in Phase 1.6b) | | |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-009 |
| L2 Domain Invariants | TBD |
| Architecture Module | SS-01 + SS-02 — `crates/factory-dispatcher/src/lib.rs`, `crates/hook-sdk/src/lib.rs` |
| Stories | S-1.03 |

### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `crates/factory-dispatcher/src/lib.rs` and `crates/hook-sdk/src/lib.rs` (both declare `HOST_ABI_VERSION: u32 = 1`) |
| **Confidence** | HIGH |
| **Extraction Date** | 2026-04-25 |
| **Extracted from** | `.factory/phase-0-ingestion/pass-3-behavioral-contracts.md` line `348` |

#### Evidence Types Used

- type constraint (compile-time const)

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
