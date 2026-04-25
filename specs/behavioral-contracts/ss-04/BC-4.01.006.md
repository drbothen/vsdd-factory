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
extracted_from: ".factory/phase-0-ingestion/pass-3-deep-rust-tests.md:642"
subsystem: "SS-04"
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

# Behavioral Contract BC-4.01.006: hook-plugins::legacy-bash-adapter::passes_payload_bytes_to_bash_with_plugin_config_stripped — re-serialized payload reaches bash with plugin_config=null while preserving event_name + dispatcher_trace_id

## Description

The unit test `passes_payload_bytes_to_bash_with_plugin_config_stripped` proves that when `adapter_logic` runs with a payload carrying `plugin_config = {script_path:"echo.sh", extra:1}`, the bytes piped to bash parse as JSON with `plugin_config: null` while preserving `event_name == "PostToolUse"` and `dispatcher_trace_id == "trace-1"`. Pins BC-AUDIT-055 / BC-AUDIT-134 with a real captured-bytes assertion.

## Preconditions

1. Payload with `plugin_config = {script_path:"echo.sh", extra:1}` provided to `adapter_logic`.
2. A runner closure captures the bytes piped to bash.

## Postconditions

1. Captured bytes parse as JSON with `plugin_config: null`.
2. `event_name == "PostToolUse"` is preserved.
3. `dispatcher_trace_id == "trace-1"` is preserved.

## Invariants

1. Bash hooks see no `plugin_config` (predates the field) but receive full upstream context.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | (No edge cases captured in Phase 0 extraction; to be added in Phase 1.5/test-writer pass) | TBD |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Payload `{event_name: "PostToolUse", dispatcher_trace_id: "trace-1", plugin_config: {script_path: "echo.sh", extra: 1}}` | Bytes piped to bash parse with `plugin_config: null`, event_name + dispatcher_trace_id preserved | happy-path |
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
| Architecture Module | SS-04 — `crates/hook-plugins/legacy-bash-adapter/src/lib.rs` (test) |
| Stories | TBD (re-anchor in Phase 1.8 from S-N.MM stories) |

### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `crates/hook-plugins/legacy-bash-adapter/src/lib.rs::tests::passes_payload_bytes_to_bash_with_plugin_config_stripped` (lines 298–331) |
| **Confidence** | HIGH |
| **Extraction Date** | 2026-04-25 |
| **Extracted from** | `.factory/phase-0-ingestion/pass-3-deep-rust-tests.md` line `642` |

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
