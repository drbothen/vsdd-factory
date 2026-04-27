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
extracted_from: ".factory/phase-0-ingestion/pass-3-behavioral-contracts.md:536"
subsystem: "SS-01"
capability: "CAP-002"
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

# Behavioral Contract BC-1.08.001: dispatcher exits 0 on registry/payload/engine errors (non-blocking)

## Description

For any startup-side error (registry, payload, or engine), the dispatcher emits an `internal.dispatcher_error` event and the process exits with code 0. The dispatcher does NOT block Claude Code on its own internal failures.

## Preconditions

1. A startup-side error occurs (registry, payload, or engine).

## Postconditions

1. `internal.dispatcher_error` event is emitted.
2. Process exits with code 0 (does not block Claude Code).

## Invariants

1. Dispatcher errors are non-blocking; the harness flow continues.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | (No edge cases captured in Phase 0 extraction; to be added in Phase 1.5/test-writer pass) | TBD |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Bad registry on startup | exit 0; `internal.dispatcher_error` emitted | error |
| TBD | TBD | happy-path |
| TBD | TBD | edge-case |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| (TBD — to be assigned in Phase 1.6b) | | |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-002 ("Hook Claude Code tool calls with sandboxed WASM plugins") per capabilities.md §CAP-002 |
| Capability Anchor Justification | CAP-002 ("Hook Claude Code tool calls with sandboxed WASM plugins") per capabilities.md §CAP-002 — this BC contracts the dispatcher's fail-safe non-blocking behavior on startup errors, which is a core invariant for the WASM dispatcher to never block Claude Code on its own internal failures |
| L2 Domain Invariants | TBD |
| Architecture Module | SS-01 — `crates/factory-dispatcher/src/main.rs` |
| Stories | S-2.07 (Wave 9 SS-01 straggler re-anchor) |

### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `main.rs::run` error branches all return Ok(0); `emit_dispatcher_error` writes the event |
| **Confidence** | HIGH |
| **Extraction Date** | 2026-04-25 |
| **Extracted from** | `.factory/phase-0-ingestion/pass-3-behavioral-contracts.md` line `536` |

#### Evidence Types Used

- assertion (error-branch return values)

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
