---
document_type: behavioral-contract
level: L3
version: "1.0"
status: draft
producer: codebase-analyzer
timestamp: 2026-04-25T00:00:00
phase: 1.4b
inputs: [bc-id-mapping.md, pass-3-behavioral-contracts-deep-r1.md]
input-hash: "[pending-recompute]"
traces_to: bc-id-mapping.md
origin: brownfield
extracted_from: ".factory/phase-0-ingestion/pass-3-behavioral-contracts-deep-r1.md:412"
subsystem: "SS-02"
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

# Behavioral Contract BC-2.02.003: HostError code mapping: -1 = CapabilityDenied, -2 = Timeout, -3 = OutputTooLarge, -4 = InvalidArgument, other negative = Other(i32)

## Description

The SDK's `HostError::from_code(code)` converts a negative i32 returned from a host call into a typed variant. Mapping is 1:1 with the dispatcher-side `codes::*` constants — a paired stability invariant tested both sides.

## Preconditions

1. A host call returns a negative i32.

## Postconditions

1. -1 → `HostError::CapabilityDenied`.
2. -2 → `HostError::Timeout`.
3. -3 → `HostError::OutputTooLarge`.
4. -4 → `HostError::InvalidArgument`.
5. Any other negative value → `HostError::Other(i32)`.

## Invariants

1. The mapping matches `factory-dispatcher/src/host/mod.rs::codes::*` byte-for-byte (cross-crate paired invariant).

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | code = -99 | `HostError::Other(-99)` |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| `from_code(-1)` | CapabilityDenied | happy-path |
| `from_code(-99)` | `Other(-99)` | edge-case |
| TBD | TBD | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| (TBD — to be assigned in Phase 1.6b) | | |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | TBD (anchor in Phase 1.5) |
| L2 Domain Invariants | TBD |
| Architecture Module | SS-02 — `crates/hook-sdk/src/host.rs`; cross-cuts SS-01 `crates/factory-dispatcher/src/host/mod.rs::codes` |
| Stories | TBD (re-anchor in Phase 1.8 from S-N.MM stories) |

### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `crates/hook-sdk/src/host.rs:81-106` (HostError + from_code), :339-345 (test) |
| **Confidence** | HIGH (compile-time-stable constants tested both sides) |
| **Extraction Date** | 2026-04-25 |
| **Extracted from** | `.factory/phase-0-ingestion/pass-3-behavioral-contracts-deep-r1.md` line `412` |

#### Evidence Types Used

- type constraint (enum variants and constants)
- assertion (test `host_error_code_mapping`)

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
