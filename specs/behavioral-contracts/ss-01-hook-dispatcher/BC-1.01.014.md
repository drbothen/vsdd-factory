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
extracted_from: ".factory/phase-0-ingestion/pass-3-deep-rust-tests.md:178"
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

# Behavioral Contract BC-1.01.014: factory-dispatcher::registry::load_returns_not_found_for_missing_path — missing registry file produces RegistryError::NotFound

## Description

`Registry::load(path)` for a non-existent path returns `Err(RegistryError::NotFound(_))`. This is a distinct variant from generic `Io` so callers can produce a precise diagnostic for "registry file not found".

## Preconditions

1. The supplied path does not exist on disk.

## Postconditions

1. `Registry::load(path)` returns `Err(RegistryError::NotFound(_))`.

## Invariants

1. Missing-file errors are distinguishable from other I/O errors.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Path exists but unreadable | TBD (different `Io` variant; not pinned by source) |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| `Registry::load("/nonexistent/registry.toml")` | `Err(RegistryError::NotFound(_))` | error |
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
| Architecture Module | SS-01 — `crates/factory-dispatcher/src/registry.rs` |
| Stories | TBD (re-anchor in Phase 1.8 from S-N.MM stories) |

### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `crates/factory-dispatcher/src/registry.rs::tests::load_returns_not_found_for_missing_path` (lines 502–506) |
| **Confidence** | HIGH |
| **Extraction Date** | 2026-04-25 |
| **Extracted from** | `.factory/phase-0-ingestion/pass-3-deep-rust-tests.md` line `178` |

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
