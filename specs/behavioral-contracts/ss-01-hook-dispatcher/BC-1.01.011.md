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
extracted_from: ".factory/phase-0-ingestion/pass-3-deep-rust-tests.md:145"
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

# Behavioral Contract BC-1.01.011: factory-dispatcher::registry::rejects_unknown_on_error_value — on_error="shout" (or any non-{block,continue}) fails parse

## Description

A TOML hook stanza with `on_error` set to anything other than `block` or `continue` (e.g., `"shout"`) fails parse. Serde's enum-variant rejection catches typos at parse time rather than silently defaulting.

## Preconditions

1. TOML hook stanza has `on_error = "<unknown>"`.

## Postconditions

1. `Registry::parse_str` returns `Err`.
2. The error is produced by serde at parse time, before any registry validation.

## Invariants

1. `on_error` typos never silently default to `Continue`.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | `on_error = "shout"` | Err |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| `on_error = "shout"` | Err | error |
| `on_error = "block"` | Ok | happy-path |
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
| **Path** | `crates/factory-dispatcher/src/registry.rs::tests::rejects_unknown_on_error_value` (lines 438–450) |
| **Confidence** | HIGH |
| **Extraction Date** | 2026-04-25 |
| **Extracted from** | `.factory/phase-0-ingestion/pass-3-deep-rust-tests.md` line `145` |

#### Evidence Types Used

- type constraint (serde enum)
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
