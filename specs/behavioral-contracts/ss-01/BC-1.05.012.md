---
document_type: behavioral-contract
level: L3
version: "1.0"
status: draft
producer: codebase-analyzer
timestamp: 2026-04-25T00:00:00
phase: 1.4b
inputs: [bc-id-mapping.md, pass-3-behavioral-contracts.md]
input-hash: "ff7795e"
traces_to: bc-id-mapping.md
origin: brownfield
extracted_from: ".factory/phase-0-ingestion/pass-3-behavioral-contracts.md:234"
subsystem: "SS-01"
capability: "CAP-TBD"
lifecycle_status: active
introduced: v1.0.0-beta.4
modified: [v1.0-pass-7, v1.0-pass-8]
deprecated: null
deprecated_by: null
replacement: null
retired: null
removed: null
removal_reason: null
---

# Behavioral Contract BC-1.05.012: emit_event enriches every emitted event with host-owned identity fields and filters reserved field names from plugin payload

## Description

The `emit_event` host fn has two inseparable halves that operate on every call:

**Enrichment half:** On every `emit_event` call, the host fn unconditionally enriches the event with four host-owned identity fields sourced from `HostContext`: `dispatcher_trace_id` (via `.with_trace_id`), `session_id` (via `.with_session_id`), `plugin_name` (via `.with_plugin_name`), and `plugin_version` (via `.with_plugin_version`). These values are populated by the dispatcher's routing layer before the plugin is invoked. The plugin has no responsibility to set these fields.

**Filter half:** When a plugin emits an event with fields that include any of the eight reserved names (`dispatcher_trace_id`, `session_id`, `plugin_name`, `plugin_version`, `ts`, `ts_epoch`, `schema_version`, `type`), those reserved fields are silently dropped from the plugin's submitted payload before the event is stored. This prevents plugins from spoofing host-owned or construction-time fields.

The four remaining reserved fields (`ts`, `ts_epoch`, `schema_version`, `type`) are set at event construction time by `InternalEvent::now()` — not by the `emit_event` enrichment path — but are equally protected by the filter.

Together, the enrichment and filter guarantee that all eight RESERVED_FIELDS always carry authoritative dispatcher-or-construction-time values, regardless of what the plugin submits.

## Preconditions

1. The `emit_event` host fn is called by a plugin with any payload (including an empty one).

## Postconditions

1. The emitted event unconditionally contains `dispatcher_trace_id`, `session_id`, `plugin_name`, and `plugin_version` sourced from `HostContext` — set by the enrichment path regardless of whether the plugin provided those fields.
2. Any reserved field names (`dispatcher_trace_id`, `session_id`, `plugin_name`, `plugin_version`, `ts`, `ts_epoch`, `schema_version`, `type`) present in the plugin's submitted payload are dropped before the event is stored.
3. All non-reserved fields in the plugin's submitted payload pass through unchanged.

## Invariants

1. Reserved field set is closed: `{dispatcher_trace_id, session_id, plugin_name, plugin_version, ts, ts_epoch, schema_version, type}`.
2. Plugins cannot spoof host-owned or construction-time fields.
3. `dispatcher_trace_id`, `session_id`, `plugin_name`, `plugin_version` are unconditionally present on every emitted event (sourced from `HostContext` via `.with_X(&str)` calls in emit_event.rs:38-42). Non-empty guarantee is upstream-BC-conditional: BC-1.02.005 lifecycle-tolerance ensures non-empty `session_id` for SessionStart envelopes (sets 'unknown' sentinel if missing); the dispatcher routing layer is responsible for populating `dispatcher_trace_id`, `plugin_name`, `plugin_version` (no current BC enforces non-empty for these — v1.1 candidate to lift to dispatcher-routing-layer BCs).

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Plugin emits non-reserved field | Passes through unchanged |
| EC-002 | Plugin emits no fields (empty payload) | Enrichment still injects all four HostContext fields; event is stored with only construction-time + host-enriched fields |
| EC-003 | Plugin attempts to set `session_id` to a custom value | Plugin's `session_id` value is dropped by the filter; HostContext's `session_id` is set by the enrichment path |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Plugin emits `{type:"my.evt", session_id:"x"}` | Event has `session_id` from HostContext (enrichment), plugin's `session_id:"x"` is dropped (filter), `dispatcher_trace_id` is present from HostContext | happy-path |
| Plugin emits `{commit_sha: "abc"}` | Field passes through unchanged; host-enriched fields also present | edge-case |
| Plugin emits empty payload `{}` | Event stored with HostContext-enriched fields only (plus construction-time fields from `InternalEvent::now`) | edge-case |

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
