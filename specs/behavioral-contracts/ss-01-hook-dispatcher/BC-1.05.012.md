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
extracted_from: ".factory/phase-0-ingestion/pass-3-behavioral-contracts.md:234"
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

# Behavioral Contract BC-1.05.012: emit_event filters out reserved field names from plugin payload

## Description

When a plugin emits an event with fields that include any of the reserved names (`dispatcher_trace_id`, `session_id`, `plugin_name`, `plugin_version`, `ts`, `ts_epoch`, `schema_version`, `type`), those reserved fields are dropped from the event. The dispatcher writes its authoritative values instead.

## Preconditions

1. Plugin emits an event whose fields include at least one reserved name.

## Postconditions

1. Reserved fields are dropped from the plugin's submitted payload.
2. The dispatcher's authoritative values for those fields are written instead.

## Invariants

1. Reserved field set is closed: `{dispatcher_trace_id, session_id, plugin_name, plugin_version, ts, ts_epoch, schema_version, type}`.
2. Plugins cannot spoof correlation fields.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Plugin emits non-reserved field | Passes through unchanged |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Plugin emits `{type:"my.evt", session_id:"x"}` | Event has dispatcher's session_id, plugin's `type` accepted only via dispatcher's authoritative value | happy-path |
| Plugin emits `{commit_sha: "abc"}` | Field passes through unchanged | edge-case |
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
| Architecture Module | SS-01 — `crates/factory-dispatcher/src/invoke.rs` (emit_event arm + reserved-name list) |
| Stories | TBD (re-anchor in Phase 1.8 from S-N.MM stories) |

### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `invoke.rs:emit_event arm` + reserved-name list |
| **Confidence** | HIGH |
| **Extraction Date** | 2026-04-25 |
| **Extracted from** | `.factory/phase-0-ingestion/pass-3-behavioral-contracts.md` line `234` |

#### Evidence Types Used

- guard clause (reserved-name filter)

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
