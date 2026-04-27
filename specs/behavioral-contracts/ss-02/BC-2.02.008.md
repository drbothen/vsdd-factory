---
document_type: behavioral-contract
level: L3
version: "1.0"
status: draft
producer: codebase-analyzer
timestamp: 2026-04-25T00:00:00
phase: 1.4b
inputs: [bc-id-mapping.md, pass-3-deep-rust-tests.md]
input-hash: "[pending-recompute]"
traces_to: bc-id-mapping.md
origin: brownfield
extracted_from: ".factory/phase-0-ingestion/pass-3-deep-rust-tests.md:699"
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

# Behavioral Contract BC-2.02.008: hook-sdk::host::encode_args_round_trip — encode_args matches the same length-prefix shape with no separator

## Description

`encode_args(&["a","bb","ccc"])` lays out the buffer with one little-endian length-prefix per arg followed by the arg bytes — same layout shape as `encode_fields` but pairs reduce to a single length-prefix per arg. SDK args encoder matches the dispatcher's `decode_args` byte-for-byte.

## Preconditions

1. Caller invokes `encode_args(&["a", "bb", "ccc"])`.

## Postconditions

1. Total length is `4+1 + 4+2 + 4+3`.
2. `buf[0..4] == 1u32_LE`, `buf[4] == b'a'`.
3. Same length-prefix shape as `encode_fields`, but pairs reduce to 1 length-prefix per arg.

## Invariants

1. Encoder is byte-compatible with `decode_args` (BC-1.05.030 / BC-AUDIT-2331).

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Empty arg list | Empty buffer |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| `encode_args(&["a","bb","ccc"])` | Length-prefixed buffer matching layout above | happy-path |
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
| Architecture Module | SS-02 — `crates/hook-sdk/src/host.rs` |
| Stories | S-1.03 |

### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `crates/hook-sdk/src/host.rs::tests::encode_args_round_trip` (lines 330–336) |
| **Confidence** | HIGH |
| **Extraction Date** | 2026-04-25 |
| **Extracted from** | `.factory/phase-0-ingestion/pass-3-deep-rust-tests.md` line `699` |

#### Evidence Types Used

- assertion (unit test)

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
