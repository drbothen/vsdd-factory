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
extracted_from: ".factory/phase-0-ingestion/pass-3-behavioral-contracts.md:204"
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

# Behavioral Contract BC-1.05.007: env host fn denies env var not on allow-list

## Description

A plugin calling `vsdd::env(name)` for a name not in `Capabilities.env_allow` receives CAPABILITY_DENIED. A denial event is emitted with `reason = "env_not_on_allow_list"` and the offending variable name.

## Preconditions

1. Plugin invokes `vsdd::env(name)`.
2. `name` is not in `Capabilities.env_allow`.

## Postconditions

1. Returns CAPABILITY_DENIED.
2. Emits denial event with `reason = "env_not_on_allow_list"`, `variable = name`.

## Invariants

1. env access is allow-list-gated; deny-by-default.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Name on allow-list but unset in env_view | Returns 0 (zero bytes); see BC-1.05.008 |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| `vsdd::env("SECRET")` with env_allow=["PATH"] | DENIED, denial event emitted | error |
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
| Architecture Module | SS-01 — `crates/factory-dispatcher/src/host/env.rs`, `invoke.rs` env wrapper |
| Stories | TBD (re-anchor in Phase 1.8 from S-N.MM stories) |

### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `invoke.rs` StoreData env wrapper + `host/env.rs` |
| **Confidence** | HIGH |
| **Extraction Date** | 2026-04-25 |
| **Extracted from** | `.factory/phase-0-ingestion/pass-3-behavioral-contracts.md` line `204` |

#### Evidence Types Used

- guard clause (env wrapper)

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
