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
extracted_from: ".factory/phase-0-ingestion/pass-3-deep-rust-tests.md:710"
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

# Behavioral Contract BC-2.02.009: hook-sdk::host::decode_subprocess_result_parses_envelope — SubprocessResult envelope `i32 | u32 | stdout | u32 | stderr` decodes correctly

## Description

`decode_subprocess_result` parses a hand-built envelope `7i32_LE | 3u32_LE | "out" | 2u32_LE | "er"` into `Some(SubprocessResult { exit_code: 7, stdout: b"out", stderr: b"er" })`. Mirrors the dispatcher's `encode_envelope` byte layout (paired BC-1.05.006 / BC-AUDIT-028).

## Preconditions

1. Buffer is the envelope `i32_LE | u32_LE_stdout_len | stdout | u32_LE_stderr_len | stderr`.

## Postconditions

1. Returns `Some(SubprocessResult { exit_code, stdout, stderr })` with the encoded fields.
2. Layout is byte-compatible with the dispatcher-side `encode_envelope`.

## Invariants

1. Envelope decoding is total over well-formed inputs.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Truncated envelope | None (covered by BC-2.02.004) |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| `7i32_LE \| 3u32_LE \| "out" \| 2u32_LE \| "er"` | `Some(SubprocessResult { exit_code: 7, stdout: b"out", stderr: b"er" })` | happy-path |
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
| **Path** | `crates/hook-sdk/src/host.rs::tests::decode_subprocess_result_parses_envelope` (lines 347–359) |
| **Confidence** | HIGH |
| **Extraction Date** | 2026-04-25 |
| **Extracted from** | `.factory/phase-0-ingestion/pass-3-deep-rust-tests.md` line `710` |

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
