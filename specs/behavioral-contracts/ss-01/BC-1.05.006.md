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
extracted_from: ".factory/phase-0-ingestion/pass-3-behavioral-contracts.md:198"
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

# Behavioral Contract BC-1.05.006: exec_subprocess result envelope is i32_LE then u32_LE_stdout_len then stdout then u32_LE_stderr_len then stderr

## Description

After a successful subprocess execution, `encode_envelope` produces the byte layout `i32_LE` (exit_code) followed by `u32_LE` (stdout length) then the stdout bytes, then `u32_LE` (stderr length) then the stderr bytes. The SDK's `SubprocessResult::decode` mirrors this layout exactly.

## Preconditions

1. A subprocess execution completes successfully.

## Postconditions

1. Encoded envelope follows the exact layout: i32_LE exit_code, u32_LE stdout_len, stdout bytes, u32_LE stderr_len, stderr bytes.
2. SDK-side `SubprocessResult::decode` parses this layout.

## Invariants

1. Envelope byte layout is wire-stable across crate versions.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Empty stdout / stderr | length prefix = 0 with no following bytes |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| `exit_code=0, stdout="ok", stderr=""` | bytes `[0,0,0,0, 2,0,0,0, "ok", 0,0,0,0]` | happy-path |
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
| Architecture Module | SS-01 — `crates/factory-dispatcher/src/host/exec_subprocess.rs` (`encode_envelope`); cross-cuts SS-02 SDK decoder |
| Stories | TBD (re-anchor in Phase 1.8 from S-N.MM stories) |

### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `host/exec_subprocess.rs::encode_envelope` |
| **Confidence** | HIGH |
| **Extraction Date** | 2026-04-25 |
| **Extracted from** | `.factory/phase-0-ingestion/pass-3-behavioral-contracts.md` line `198` |

#### Evidence Types Used

- assertion (encode_envelope implementation)
- type constraint (byte layout)

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
