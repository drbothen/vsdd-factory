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
extracted_from: ".factory/phase-0-ingestion/pass-3-behavioral-contracts.md:222"
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

# Behavioral Contract BC-1.05.010: Context getters (session_id, dispatcher_trace_id, plugin_root, plugin_version, cwd) always return current value

## Description

Per-invocation context getters (`session_id`, `dispatcher_trace_id`, `plugin_root`, `plugin_version`, `cwd`) always return the host-context-stored value as bytes. If the caller's `out_cap` is 0, the host returns the required size for the second-call protocol.

## Preconditions

1. Plugin invokes any context getter.

## Postconditions

1. Returns the current host-context value as bytes.
2. If `out_cap == 0`, returns the required buffer size instead.

## Invariants

1. Context getters are deterministic and reflect the dispatcher's current invocation context.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | out_cap = 0 | Returns required size, no bytes written |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| `vsdd::session_id(buf)` with sufficient out_cap | Writes the session_id; returns its length | happy-path |
| `vsdd::session_id(buf)` with out_cap=0 | Returns required size | edge-case |
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
| Architecture Module | SS-01 — `crates/factory-dispatcher/src/invoke.rs::context_reader`, `host/context_fns.rs::register` |
| Stories | TBD (re-anchor in Phase 1.8 from S-N.MM stories) |

### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `invoke.rs::context_reader`; `host/context_fns.rs::register` |
| **Confidence** | HIGH |
| **Extraction Date** | 2026-04-25 |
| **Extracted from** | `.factory/phase-0-ingestion/pass-3-behavioral-contracts.md` line `222` |

#### Evidence Types Used

- assertion (context_reader logic)

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
