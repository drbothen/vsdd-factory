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
extracted_from: ".factory/phase-0-ingestion/pass-3-behavioral-contracts-deep-r1.md:405"
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

# Behavioral Contract BC-2.02.002: Bounded host calls are mandatory — `read_file` and `exec_subprocess` REQUIRE `timeout_ms` and a byte cap

## Description

`host::read_file` and `host::exec_subprocess` carry mandatory `max_bytes` (or `max_output_bytes`) and `timeout_ms` parameters at the API level. The caller cannot opt out of bounds at the type level. The dispatcher enforces the bounds again at runtime (defense in depth).

## Preconditions

1. A plugin author calls `host::read_file` or `host::exec_subprocess`.

## Postconditions

1. Both functions require `max_bytes: u32` (or `max_output_bytes: u32`).
2. Both functions require `timeout_ms: u32`.
3. The dispatcher enforces these bounds at runtime in addition to the SDK signature.

## Invariants

1. No unbounded host call exists in the SDK surface.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | (No edge cases captured in Phase 0 extraction; to be added in Phase 1.5/test-writer pass) | TBD |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| `host::read_file(path, 1024, 5000)` | Compiles; runtime enforces bounds | happy-path |
| `host::read_file(path)` (missing args) | Compile error | error |
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
| Architecture Module | SS-02 — `crates/hook-sdk/src/host.rs` |
| Stories | TBD (re-anchor in Phase 1.8 from S-N.MM stories) |

### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `crates/hook-sdk/src/host.rs:184-205` (read_file), :215-256 (exec_subprocess), :1-10 (docstring) |
| **Confidence** | HIGH |
| **Extraction Date** | 2026-04-25 |
| **Extracted from** | `.factory/phase-0-ingestion/pass-3-behavioral-contracts-deep-r1.md` line `405` |

#### Evidence Types Used

- type constraint (function signatures)
- documentation (module docstring)

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
