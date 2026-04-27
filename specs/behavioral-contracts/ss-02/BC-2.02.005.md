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
extracted_from: ".factory/phase-0-ingestion/pass-3-behavioral-contracts-deep-r1.md:426"
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

# Behavioral Contract BC-2.02.005: SDK-side `read_string` re-call protocol — host returns required size; SDK reallocates and re-calls

## Description

When the host writes more bytes than the SDK's `out_cap`, the host returns the required size; the SDK resizes its buffer to that size and re-calls the host fn. This 2-call protocol is used for `session_id`, `dispatcher_trace_id`, `plugin_root`, `plugin_version`, and `cwd`.

## Preconditions

1. Host writes more bytes than the initial `out_cap`.

## Postconditions

1. Host returns the required buffer size.
2. SDK resizes its buffer to that size and re-calls the host fn.
3. Second call succeeds with the larger buffer.

## Invariants

1. Context getters always return the current value via at most 2 host calls.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | First call's out_cap is sufficient | Returns immediately on first call |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Context getter with insufficient initial out_cap | SDK resizes and re-calls; final result complete | happy-path |
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
| Architecture Module | SS-02 — `crates/hook-sdk/src/host.rs` (read_string helper) |
| Stories | S-1.03 |

### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `crates/hook-sdk/src/host.rs:108-125` (read_string helper) |
| **Confidence** | HIGH (logic explicit; test coverage indirect) |
| **Extraction Date** | 2026-04-25 |
| **Extracted from** | `.factory/phase-0-ingestion/pass-3-behavioral-contracts-deep-r1.md` line `426` |

#### Evidence Types Used

- assertion (read_string helper logic)
- inferred (host stubs return 0 on non-wasm so 2-call path is exercised manually)

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
