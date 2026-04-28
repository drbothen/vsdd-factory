---
document_type: behavioral-contract
level: L3
version: "1.0"
status: draft
producer: product-owner
timestamp: 2026-04-28T00:00:00
phase: 1a
inputs:
  - .factory/stories/S-5.02-session-end-hook.md
  - .factory/specs/domain-spec/capabilities.md
input-hash: "d5ae7e4"
traces_to: .factory/specs/prd.md#FR-046
origin: greenfield
extracted_from: null
subsystem: "SS-04"
capability: "CAP-002"
lifecycle_status: active
introduced: v1.0.0-rc.1
modified: [v1.0-pass-1]
deprecated: null
deprecated_by: null
replacement: null
retired: null
removed: null
removal_reason: null
---

# BC-4.05.001: session-end plugin emits session.ended event with session telemetry on SessionEnd event

## Description

When the dispatcher routes a `SessionEnd` event to the `session-end-telemetry.wasm` plugin via the `hooks.json.template` registration, the plugin emits a `session.ended` event via the `emit_event` host function. Three fields are set by the plugin: `duration_ms`, `tool_call_count`, and `timestamp`. These are computed from the incoming envelope's `session_start_ts` and `tool_call_count` fields; if either is absent, the plugin substitutes `"0"`. Four additional fields are automatically injected by the `emit_event` host fn from `HostContext` (per BC-1.05.012 enrichment; the plugin does not set these): `dispatcher_trace_id`, `session_id`, `plugin_name`, `plugin_version`. Four construction-time fields are set by `InternalEvent::now()`: `ts`, `ts_epoch`, `schema_version`, `type`. Total fields on wire: 11. The `session.ended` event-name literal is reserved per PRD FR-046.

## Preconditions

1. A `SessionEnd` event has arrived at the dispatcher.
2. `hooks.json` (generated from `hooks.json.template`) contains a `SessionEnd` entry routing to the dispatcher binary, which then routes to `session-end-telemetry.wasm` via `hooks-registry.toml`.
3. The `session-end-telemetry.wasm` plugin is loaded in the dispatcher's `PluginCache`.

## Postconditions

1. The plugin invokes `emit_event` exactly once with `event_name = "session.ended"`.
2. The emitted payload contains all required fields. Fields are categorized by who sets them:

   **Plugin-set fields (3 fields — the plugin sets these via `emit_event` key/value pairs):**
   - `duration_ms` (string per wire format, per `emit_event.rs:49` coercion): integer milliseconds since the SessionStart that opened this session, computed from the envelope's `session_start_ts` field. If `session_start_ts` is absent from the envelope, `duration_ms = "0"`. Value is always a non-negative integer represented as a decimal string.
   - `tool_call_count` (string per wire format): integer count of tool invocations during the session, sourced from the envelope's `tool_call_count` field. If `tool_call_count` is absent from the envelope, `tool_call_count = "0"`. Value is always a non-negative integer represented as a decimal string.
   - `timestamp` (ISO-8601 UTC with millisecond precision and `Z` suffix; regex: `^\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}\.\d{3}Z$`; example: `2026-04-28T12:34:56.789Z`) — the plugin's own emission timestamp, not the session-end time from the envelope.

   **Notes:**

   **Field provenance — 4+4+3 split (SessionEnd analog of F-P7-02):** The 11 fields on the wire come from three distinct sources:

   - **Host-enriched fields (4 fields — set by `emit_event` host fn from `HostContext`, NOT by the plugin):** `dispatcher_trace_id`, `session_id`, `plugin_name`, `plugin_version`. These are part of `RESERVED_FIELDS` and would be silently dropped if the plugin attempted to set them. `session_id` originates from the incoming `SessionEnd` envelope parsed by BC-1.02.005 lifecycle-tolerant envelope parsing; when missing or empty, BC-1.02.005 sets `HostContext.session_id = "unknown"`.

   - **Construction-time fields (4 fields — set by `InternalEvent::now()`, NOT by the plugin or `emit_event` enrichment):** `ts` (current UTC time), `ts_epoch` (current Unix timestamp), `schema_version` (struct constant), `type` (the plugin-supplied `event_name` argument — `"session.ended"` in this case). Also part of `RESERVED_FIELDS`; plugin attempts to set them are silently dropped.

   - **Plugin-set fields (3 fields listed above):** set by the plugin via `emit_event` key/value pairs and pass through the non-reserved field path in `emit_event.rs`.

   **RESERVED_FIELDS — plugin must NOT set (8 total):**
   4 host-enriched (auto-injected by `emit_event` from `HostContext`): `dispatcher_trace_id`, `session_id`, `plugin_name`, `plugin_version`.
   4 construction-time (set by `InternalEvent::now()`): `ts`, `ts_epoch`, `schema_version`, `type`.
   Any plugin attempt to set a RESERVED_FIELD is silently dropped by `emit_event.rs`.

   **Wire format — all plugin-set field values are strings (same as F-P6-05 for SessionStart):** The `emit_event` host fn coerces all plugin-supplied values to JSON strings on the wire (`emit_event.rs:49`). `duration_ms` and `tool_call_count` are integer semantics but arrive as decimal strings. Downstream consumers MUST parse string values back to their semantic types.

   **SessionEnd vs. SessionStart payload size:** SessionStart emits 14 fields (6 plugin-set + 4 host-enriched + 4 construction-time). SessionEnd emits 11 fields (3 plugin-set + 4 host-enriched + 4 construction-time). The reduced count reflects the absence of `factory_version`, `plugin_count`, `activated_platform`, `factory_health`, and `tool_deps` fields — SessionEnd is intentionally lightweight.

3. `session_id` in the emitted event matches the `session_id` that BC-1.02.005 lifecycle-tolerant envelope parsing populated into `HostContext.session_id` from the incoming `SessionEnd` envelope — auto-enriched by the `emit_event` host fn, not set by the plugin.
4. The plugin returns `HookResult::Ok` (exit code 0) to the dispatcher.

## Invariants

1. `session_id` on the emitted event reflects the value BC-1.02.005 envelope parsing placed into `HostContext.session_id` — preserved verbatim from the envelope (or `"unknown"` if missing/empty), never transformed, truncated, or replaced by the plugin (the plugin does not set it; `emit_event` auto-enriches it from HostContext).
2. The `session.ended` event-name literal is immutable and reserved per PRD FR-046; renaming requires a new BC.
3. `emit_event` is called before the plugin function returns.
4. `duration_ms` and `tool_call_count` are never absent or null in the emitted payload — they default to `"0"` when the corresponding envelope fields are absent.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | `session_start_ts` is absent from the `SessionEnd` envelope | `duration_ms = "0"` in the emitted `session.ended` event; all other fields emitted normally; plugin does not abort |
| EC-002 | `tool_call_count` is absent from the `SessionEnd` envelope | `tool_call_count = "0"` in the emitted `session.ended` event; all other fields emitted normally; plugin does not abort |
| EC-003 | Both `session_start_ts` and `tool_call_count` are absent from the envelope | `duration_ms = "0"`, `tool_call_count = "0"`; `timestamp` is the plugin's own emission time; plugin emits normally |
| EC-004 | `session_id` is missing or empty string in the `SessionEnd` envelope | BC-1.02.005 lifecycle-tolerance sets `HostContext.session_id = "unknown"`; `emit_event` auto-enriches the event with this value; plugin is unconditionally stateless per BC-4.05.003; emits normally |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| `SessionEnd` envelope with `session_id = "sess-abc-123"`, `session_start_ts = "2026-04-28T12:00:00.000Z"`, `tool_call_count = 42`, dispatcher routes to session-end-telemetry.wasm | `session.ended` emitted once; `duration_ms` is a non-negative decimal string representing elapsed ms since session_start_ts; `tool_call_count = "42"` (string on wire per emit_event.rs:49); `timestamp` matches regex `^\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}\.\d{3}Z$`; host-enriched fields present: `session_id = "sess-abc-123"`, `dispatcher_trace_id` non-empty string, `plugin_name` non-empty string, `plugin_version` non-empty string | happy-path |
| `SessionEnd` envelope with `session_id = "sess-def-456"`, `session_start_ts` absent, `tool_call_count` absent | `session.ended` emitted once; `duration_ms = "0"`, `tool_call_count = "0"`, `timestamp` present with correct format; `session_id = "sess-def-456"` (host-enriched) | edge-case (both absent) |
| `SessionEnd` envelope with `session_id = ""` (empty string) | `session.ended` emitted once; `session_id = "unknown"` (BC-1.02.005 sets "unknown" sentinel; emit_event auto-enriches); `duration_ms = "0"` if session_start_ts absent; plugin does not abort | edge-case (missing session_id) |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-066 | Session-End Plugin Surface Invariant — All BC-4.05.* Postconditions Hold in Integration Test | integration |

## Related BCs

- **BC-4.05.002** — composes with (no subprocess invocation; plugin completes synchronously before emitting)
- **BC-4.05.003** — composes with (plugin emits unconditionally per invocation; Layer 1 once-discipline per BC-4.05.004 ensures single emission per session)
- **BC-4.05.004** — depends on (hooks.json.template registration triggers this plugin; Layer 1 `once: true` directive is the upstream once-per-session guarantee)
- **BC-4.05.005** — depends on (hooks-registry.toml registration provides dispatcher-side routing to this plugin)
- **BC-1.02.005** — depends on (dispatcher envelope parsing delivers `session_id` to this plugin)
- **BC-1.05.012** — depends on (emit_event host fn auto-enriches with host-enriched fields including session_id, dispatcher_trace_id, plugin_name, plugin_version)
- **BC-4.04.001** — structural analog (SessionStart counterpart; BC-4.05.001 mirrors BC-4.04.001 pattern adapted for SessionEnd semantics)

## Architecture Anchors

- SS-04 — `crates/hook-plugins/session-end-telemetry/src/lib.rs` (plugin `on_hook` entry point + `emit_event` call)
- SS-01 — dispatcher routes `SessionEnd` to `session-end-telemetry.wasm` per `hooks-registry.toml` routing entry (BC-4.05.005)

## Story Anchor

S-5.02

## VP Anchors

VP-066

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-002 |
| Capability Anchor Justification | CAP-002 ("Hook Claude Code tool calls with sandboxed WASM plugins") per capabilities.md §CAP-002 |
| L2 Domain Invariants | DI-007 (always-on self-telemetry — session.ended is part of the always-on telemetry surface; emitted unconditionally per invocation); DI-017 (dispatcher_trace_id on every emitted event — automatically enriched by emit_event host fn from HostContext; not the plugin's responsibility to set); BC-1.02.005 (lifecycle-tolerant envelope parsing populates HostContext.session_id used by emit_event auto-enrichment; "unknown" sentinel set at envelope-parse layer, not by the plugin) |
| Architecture Module | SS-04 — `crates/hook-plugins/session-end-telemetry/src/lib.rs` |
| Stories | S-5.02 |
| Functional Requirement | FR-046 |
