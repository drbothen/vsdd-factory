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
extracted_from: ".factory/phase-0-ingestion/pass-3-behavioral-contracts-deep-r1.md:466"
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

# Behavioral Contract BC-4.02.004: Adapter strips plugin_config to Null before piping to bash — bash hooks predate the field

## Description

After resolving `script_path`, the adapter clones the payload, sets `bash_payload.plugin_config = Value::Null`, and serializes that clone for bash. The original `payload.plugin_config` is preserved on the adapter's stack but never reaches the bash hook. Test `passes_payload_bytes_to_bash_with_plugin_config_stripped` round-trips the JSON and asserts `plugin_config: null` while preserving `event_name` and `dispatcher_trace_id`.

## Preconditions

1. Adapter has resolved `script_path` and is about to pipe payload bytes to bash.

## Postconditions

1. The bytes piped to bash deserialize to a payload with `plugin_config: null`.
2. `event_name` and `dispatcher_trace_id` (and other top-level fields) remain populated.
3. Adapter's local `payload.plugin_config` is preserved (used by the adapter itself).

## Invariants

1. Bash sees `plugin_config: null` regardless of the adapter's view.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | (No edge cases captured in Phase 0 extraction; to be added in Phase 1.5/test-writer pass) | TBD |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Payload `{event_name, dispatcher_trace_id, plugin_config:{...}}` | Bytes piped to bash parse with `plugin_config: null`, event_name + dispatcher_trace_id preserved | happy-path |
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
| Architecture Module | SS-04 — `crates/hook-plugins/legacy-bash-adapter/src/lib.rs` (clone + null + serialize) |
| Stories | TBD (re-anchor in Phase 1.8 from S-N.MM stories) |

### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `crates/hook-plugins/legacy-bash-adapter/src/lib.rs:81-90` (clone + null + serialize); :298-331 (test) |
| **Confidence** | HIGH |
| **Extraction Date** | 2026-04-25 |
| **Extracted from** | `.factory/phase-0-ingestion/pass-3-behavioral-contracts-deep-r1.md` line `466` |

#### Evidence Types Used

- assertion (unit test round-trips JSON)

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
