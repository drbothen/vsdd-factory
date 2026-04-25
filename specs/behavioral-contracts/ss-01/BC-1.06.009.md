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
extracted_from: ".factory/phase-0-ingestion/pass-3-deep-rust-tests.md:563"
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

# Behavioral Contract BC-1.06.009: factory-dispatcher::internal_log (integration)::startup_flow_writes_parseable_jsonl — 4-event dispatcher startup flow round-trips through JSONL with correct envelope per event

## Description

Integration test: a fresh `InternalLog` at a nested non-existent dir. 4 events written: `dispatcher.started`, `plugin.loaded`, `plugin.invoked`, `internal.dispatcher_error`. Files are read back: exactly one rotated file; 4 lines; each carries the common envelope (schema_version, ts, ts_epoch, dispatcher_trace_id) plus event-specific extras (dispatcher_version, loaded_plugin_count, plugin_name, plugin_version, tool_name, message). `plugin.loaded` event has no `session_id` (omitted because not set).

## Preconditions

1. Fresh `InternalLog` at nested non-existent dir.
2. 4 specific events written.

## Postconditions

1. One rotated file produced.
2. 4 lines.
3. Each line carries the documented envelope plus per-event extras.
4. `plugin.loaded` lacks `session_id` (per omit-on-None).

## Invariants

1. Library re-export surface (InternalEvent, InternalLog, const event-name strings) matches unit-tested behavior end-to-end.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | (No edge cases captured in Phase 0 extraction; to be added in Phase 1.5/test-writer pass) | TBD |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| 4 startup events | 4-line JSONL file with documented envelope | happy-path |
| TBD | TBD | edge-case |
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
| Architecture Module | SS-01 — `crates/factory-dispatcher/tests/internal_log_integration.rs` |
| Stories | TBD (re-anchor in Phase 1.8 from S-N.MM stories) |

### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `crates/factory-dispatcher/tests/internal_log_integration.rs::startup_flow_writes_parseable_jsonl` (lines 29–131) |
| **Confidence** | HIGH |
| **Extraction Date** | 2026-04-25 |
| **Extracted from** | `.factory/phase-0-ingestion/pass-3-deep-rust-tests.md` line `563` |

#### Evidence Types Used

- assertion (integration test)

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
