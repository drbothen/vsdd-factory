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
extracted_from: ".factory/phase-0-ingestion/pass-3-behavioral-contracts-deep-r1.md:398"
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

# Behavioral Contract BC-2.02.001: Plugin-author API surface is `vsdd_hook_sdk::host::*`; raw FFI is private (`mod ffi;`)

## Description

The SDK exposes plugin-author APIs only via `vsdd_hook_sdk::host::*` (re-exports include `host::log`, `host::log_info/log_warn/log_error`, `host::emit_event`, context getters, `host::env`, `host::read_file`, `host::exec_subprocess`, plus types `LogLevel`, `HostError`, `SubprocessResult`). The `ffi` module is declared `mod ffi;` (private). Plugin authors who reach into FFI directly bypass the type-safe wrappers.

## Preconditions

1. A plugin author writes against the SDK.

## Postconditions

1. Public re-exports include the listed `host::*` items.
2. `ffi` is private (`mod ffi;` not `pub mod ffi;`).

## Invariants

1. The plugin-author surface is exactly `host::*`; FFI is internal.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Plugin attempts to use `vsdd_hook_sdk::ffi` | Compile error (private module) |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Plugin imports `vsdd_hook_sdk::host::log_info` | Compiles | happy-path |
| Plugin imports `vsdd_hook_sdk::ffi::*` | Compile error: private module | error |
| TBD | TBD | edge-case |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| (TBD — to be assigned in Phase 1.6b) | | |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-009 |
| L2 Domain Invariants | TBD |
| Architecture Module | SS-02 — `crates/hook-sdk/src/lib.rs`, `crates/hook-sdk/src/host.rs` |
| Stories | S-1.03 |

### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `crates/hook-sdk/src/lib.rs:37-47` (mod ffi; pub mod host;); `crates/hook-sdk/src/host.rs:1-10` (docstring) |
| **Confidence** | HIGH |
| **Extraction Date** | 2026-04-25 |
| **Extracted from** | `.factory/phase-0-ingestion/pass-3-behavioral-contracts-deep-r1.md` line `398` |

#### Evidence Types Used

- type constraint (visibility modifier)
- documentation (docstring)

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
