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
extracted_from: ".factory/phase-0-ingestion/pass-3-deep-rust-tests.md:688"
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

# Behavioral Contract BC-2.02.007: hook-sdk::host::encode_fields_uses_length_prefix — encode_fields([(k,v)]) lays out key_len|key|value_len|value LE-prefixed

## Description

`encode_fields(&[(k,v), ...])` lays out the buffer as `key_len(u32_LE) | key | value_len(u32_LE) | value` for each pair. Test inspects buffer byte-by-byte to confirm exact layout. Byte-compatible with the dispatcher's `decode_fields` (paired BC-1.05.013).

## Preconditions

1. Caller invokes `encode_fields(&[("k","vv"), ("aa","b")])`.

## Postconditions

1. Total length matches `4+1 + 4+2 + 4+2 + 4+1` for the sample input.
2. `buf[0..4] == 1u32_LE`; `buf[4] == b'k'`.
3. `buf[5..9] == 2u32_LE`; `buf[9..11] == b"vv"`.

## Invariants

1. Layout is little-endian length-prefix per pair, no separators.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Empty input | Empty buffer |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| `encode_fields(&[("k","vv"), ("aa","b")])` | Length-prefixed buffer matching layout above | happy-path |
| TBD | TBD | edge-case |
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
| Architecture Module | SS-02 — `crates/hook-sdk/src/host.rs` |
| Stories | TBD (re-anchor in Phase 1.8 from S-N.MM stories) |

### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `crates/hook-sdk/src/host.rs::tests::encode_fields_uses_length_prefix` (lines 317–328) |
| **Confidence** | HIGH |
| **Extraction Date** | 2026-04-25 |
| **Extracted from** | `.factory/phase-0-ingestion/pass-3-deep-rust-tests.md` line `688` |

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
