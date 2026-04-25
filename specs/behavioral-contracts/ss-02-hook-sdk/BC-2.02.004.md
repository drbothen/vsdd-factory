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
extracted_from: ".factory/phase-0-ingestion/pass-3-behavioral-contracts-deep-r1.md:419"
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

# Behavioral Contract BC-2.02.004: SubprocessResult envelope decoding is paranoid — rejects truncated input rather than panicking

## Description

`decode_subprocess_result` performs explicit `len < N` checks before each slice, returning `None` on truncation rather than panicking. Callers map `None` to `HostError::Other(-99)`.

## Preconditions

1. Host returns possibly malformed envelope bytes (e.g., truncated by a buffer cap).

## Postconditions

1. On truncation, decoder returns `None`.
2. Caller maps `None` → `HostError::Other(-99)`.
3. No panic occurs even on adversarial input.

## Invariants

1. Decoder is panic-free across all byte inputs.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Empty buffer | None |
| EC-002 | Buffer truncated mid-stream length | None |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Truncated envelope | None → caller maps to `HostError::Other(-99)` | error |
| TBD | TBD | happy-path |
| TBD | TBD | edge-case |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| (TBD — to be assigned in Phase 1.6b) | | |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | TBD (anchor in Phase 1.5) |
| L2 Domain Invariants | TBD |
| Architecture Module | SS-02 — `crates/hook-sdk/src/host.rs` (`decode_subprocess_result`) |
| Stories | TBD (re-anchor in Phase 1.8 from S-N.MM stories) |

### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `crates/hook-sdk/src/host.rs:272-295` (decoder), :255 (caller), :362-365 (test) |
| **Confidence** | HIGH |
| **Extraction Date** | 2026-04-25 |
| **Extracted from** | `.factory/phase-0-ingestion/pass-3-behavioral-contracts-deep-r1.md` line `419` |

#### Evidence Types Used

- guard clause (explicit length checks)
- assertion (test `decode_subprocess_result_rejects_truncated`)

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
