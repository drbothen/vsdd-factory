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
extracted_from: ".factory/phase-0-ingestion/pass-3-behavioral-contracts-deep-r1.md:433"
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

# Behavioral Contract BC-2.02.006: SDK ffi.rs uses `#[link(wasm_import_module = "vsdd")]` on wasm32 targets, host stubs on others

## Description

On `wasm32-wasip1` (production), `ffi.rs` declares an `unsafe extern "C"` block with `pub safe fn` items linked from the import module `vsdd`. On non-wasm32 targets (tests), a `host_stubs` module provides no-op stub fns so unit tests link. Capability-bearing host stubs return -1 (CapabilityDenied) on non-wasm so test paths see the expected error variant.

## Preconditions

1. Crate is being compiled for `wasm32-wasip1` (production) OR any other arch (tests).

## Postconditions

1. wasm32-wasip1: imports linked from `vsdd` module.
2. Non-wasm32: `host_stubs` module supplies no-op replacements.
3. Capability-bearing host stubs return -1 (CapabilityDenied) on non-wasm32.

## Invariants

1. SDK builds and unit-tests on any host arch even though host functions are wasm-only at runtime.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | (No edge cases captured in Phase 0 extraction; to be added in Phase 1.5/test-writer pass) | TBD |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Build SDK for wasm32-wasip1 | imports link to `vsdd` module | happy-path |
| Build SDK for x86_64-apple-darwin | host_stubs supply no-ops; capability stubs return -1 | edge-case |
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
| Architecture Module | SS-02 — `crates/hook-sdk/src/ffi.rs` |
| Stories | TBD (re-anchor in Phase 1.8 from S-N.MM stories) |

### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `crates/hook-sdk/src/ffi.rs:13-58` (wasm32 imports), :62-127 (non-wasm host_stubs) |
| **Confidence** | HIGH |
| **Extraction Date** | 2026-04-25 |
| **Extracted from** | `.factory/phase-0-ingestion/pass-3-behavioral-contracts-deep-r1.md` line `433` |

#### Evidence Types Used

- type constraint (cfg-gated #[link] attribute)
- assertion (host_stubs return -1 for capability-bearing fns)

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
