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
extracted_from: ".factory/phase-0-ingestion/pass-3-behavioral-contracts.md:104"
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

# Behavioral Contract BC-1.03.005: stderr captured per plugin and truncated at 4 KiB with marker

## Description

When a plugin emits more than 4096 bytes to stderr, the dispatcher truncates the captured stderr at 4 KiB and appends a marker `…(stderr truncated)`. The truncated stderr is included in the plugin's lifecycle event payload.

## Preconditions

1. Plugin emits more than 4096 bytes to stderr.

## Postconditions

1. `PluginResult.stderr` length <= 4096 bytes plus the truncation marker.
2. Marker text is `…(stderr truncated)`.
3. The lifecycle event payload includes the truncated stderr.

## Invariants

1. STDERR_CAP_BYTES = 4096 enforced.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | stderr exactly 4096 bytes | No truncation; no marker |
| EC-002 | stderr 4097 bytes | Truncated to 4096 + marker |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Plugin emits >4 KiB to stderr | `stderr <= 4096 bytes + "…(stderr truncated)"` marker | edge-case |
| TBD | TBD | happy-path |
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
| Architecture Module | SS-01 — `crates/factory-dispatcher/src/invoke.rs` (`stderr_to_string`, STDERR_CAP_BYTES) |
| Stories | TBD (re-anchor in Phase 1.8 from S-N.MM stories) |

### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `invoke.rs::stderr_to_string`; STDERR_CAP_BYTES constant; CHANGELOG v1.0.0-beta.4 entry |
| **Confidence** | HIGH (constant + truncation logic + CHANGELOG provenance) |
| **Extraction Date** | 2026-04-25 |
| **Extracted from** | `.factory/phase-0-ingestion/pass-3-behavioral-contracts.md` line `104` |

#### Evidence Types Used

- type constraint (constant)
- assertion (truncation logic)
- documentation (CHANGELOG)

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
