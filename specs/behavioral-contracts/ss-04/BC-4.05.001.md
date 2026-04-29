---
document_type: behavioral-contract
level: L3
version: "v1.2"
status: draft
producer: product-owner
timestamp: 2026-04-28T00:00:00
phase: 1a
inputs:
  - .factory/stories/S-5.02-session-end-hook.md
  - .factory/specs/domain-spec/capabilities.md
input-hash: "f2f67a5"
traces_to: .factory/specs/prd.md#FR-046
origin: greenfield
extracted_from: null
subsystem: "SS-04"
capability: "CAP-002"
lifecycle_status: active
introduced: v1.0.0-rc.1
modified: [v1.0-pass-1, v1.0-pass-2, v1.0-pass-3, v1.0-pass-4, v1.0-pass-5, v1.1-adv-s5.03-p01-sibling-sweep, v1.2-adv-s5.03-p04]
deprecated: null
deprecated_by: null
replacement: null
retired: null
removed: null
removal_reason: null
---

# BC-4.05.001: session-end plugin emits session.ended event with session telemetry on SessionEnd event

## Description

When the dispatcher routes a `SessionEnd` event to the `session-end-telemetry.wasm` plugin via the `hooks.json.template` registration, the plugin emits a `session.ended` event via the `emit_event` host function. Three fields are set by the plugin: `duration_ms`, `tool_call_count`, and `timestamp`. These are computed from the incoming envelope's `session_start_ts` and `tool_call_count` fields; if either is absent, the plugin substitutes `"0"`. `duration_ms = "0"` also applies when `session_start_ts` is present but in the future relative to `now_ms` (clock skew safeguard — a future timestamp yields a negative elapsed duration, which is clamped to `"0"`). Four additional fields are automatically injected by the `emit_event` host fn from `HostContext` (per BC-1.05.012 enrichment; the plugin does not set these): `dispatcher_trace_id`, `session_id`, `plugin_name`, `plugin_version`. Four construction-time fields are set by the dispatcher between the plugin's `emit_event` call and the final wire format (implementation provenance is opaque from the spec layer): `ts`, `ts_epoch`, `schema_version`, `type`. Total fields on wire: 11. The `session.ended` event-name literal is reserved per PRD FR-046.

## Preconditions

1. A `SessionEnd` event has arrived at the dispatcher.
2. `hooks.json` (generated from `hooks.json.template`) contains a `SessionEnd` entry routing to the dispatcher binary, which then routes to `session-end-telemetry.wasm` via `hooks-registry.toml`.
3. The `session-end-telemetry.wasm` plugin is loaded in the dispatcher's `PluginCache`.

## Postconditions

1. The plugin invokes `emit_event` exactly once with `event_name = "session.ended"`.
2. The emitted payload contains all required fields. Fields are categorized by who sets them:

   **Plugin-set fields (3 fields — the plugin sets these via `emit_event` key/value pairs):**
   - `duration_ms` (string per wire format, per `emit_event.rs:49` coercion): integer milliseconds since the SessionStart that opened this session, computed from the envelope's `session_start_ts` field. Specifically: `duration_ms = now_ms - session_start_ts_ms` where `now_ms` is the plugin's emission instant (the same instant as the `timestamp` field). `duration_ms = "0"` when: (a) `session_start_ts` is absent from the envelope; OR (b) `session_start_ts` is in the future relative to `now_ms` (clock-skew clamp — a future timestamp yields a negative elapsed duration, which the plugin clamps to `"0"` rather than emitting a negative value); OR (c) `session_start_ts` is present as a JSON string but does not parse as ISO-8601 (treat-as-absent default; non-string-type envelope values are out of scope for this BC and deferred to BC-1.02.005 v1.1 envelope-value type validation extension). Value is always a non-negative integer represented as a decimal string.
   - `tool_call_count` (string per wire format): integer count of tool invocations during the session, sourced from the envelope's `tool_call_count` field. If `tool_call_count` is absent from the envelope, `tool_call_count = "0"`. Value is always a non-negative integer represented as a decimal string.
   - `timestamp` (ISO-8601 UTC with millisecond precision and `Z` suffix; regex: `^\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}\.\d{3}Z$`; example: `2026-04-28T12:34:56.789Z`) — the plugin's own emission timestamp, not the session-end time from the envelope.

   **Notes:**

   **Field provenance — 4+4+3 split (SessionEnd analog of F-P7-02):** The 11 fields on the wire come from three distinct sources:

   - **Host-enriched fields (4 fields — set by `emit_event` host fn from `HostContext`, NOT by the plugin):** `dispatcher_trace_id`, `session_id`, `plugin_name`, `plugin_version`. These are part of `RESERVED_FIELDS` and would be silently dropped if the plugin attempted to set them. `session_id` originates from the incoming `SessionEnd` envelope parsed by BC-1.02.005 lifecycle-tolerant envelope parsing; when missing or empty, BC-1.02.005 sets `HostContext.session_id = "unknown"`.

   - **Construction-time fields (4 fields — set by the dispatcher between the plugin's `emit_event` call and the final wire format; the plugin must NOT set them — implementation provenance is opaque from the spec layer):** `ts` (current UTC time), `ts_epoch` (current Unix timestamp), `schema_version` (struct constant), `type` (the plugin-supplied `event_name` argument — `"session.ended"` in this case). Also part of `RESERVED_FIELDS`; plugin attempts to set them are silently dropped.

   - **Plugin-set fields (3 fields listed above):** set by the plugin via `emit_event` key/value pairs and pass through the non-reserved field path in `emit_event.rs`.

   **RESERVED_FIELDS — plugin must NOT set (8 total):**
   4 host-enriched (auto-injected by `emit_event` from `HostContext`): `dispatcher_trace_id`, `session_id`, `plugin_name`, `plugin_version`.
   4 construction-time (set by the dispatcher between the plugin's `emit_event` call and the final wire format; implementation provenance is opaque from the spec layer): `ts`, `ts_epoch`, `schema_version`, `type`.
   Any plugin attempt to set a RESERVED_FIELD is silently dropped by `emit_event.rs`.

   **Wire format — all plugin-set field values are strings (same as F-P6-05 for SessionStart):** The `emit_event` host fn coerces all plugin-supplied values to JSON strings on the wire (`emit_event.rs:49`). `duration_ms` and `tool_call_count` are integer semantics but arrive as decimal strings. Downstream consumers MUST parse string values back to their semantic types.

   **SessionEnd vs. SessionStart payload size:** SessionStart emits 14 fields (6 plugin-set + 4 host-enriched + 4 construction-time). SessionEnd emits 11 fields (3 plugin-set + 4 host-enriched + 4 construction-time). The reduced count reflects the absence of `factory_version`, `plugin_count`, `activated_platform`, `factory_health`, and `tool_deps` fields — SessionEnd is intentionally lightweight.

3. `session_id` in the emitted event matches the `session_id` that BC-1.02.005 lifecycle-tolerant envelope parsing populated into `HostContext.session_id` from the incoming `SessionEnd` envelope — auto-enriched by the `emit_event` host fn, not set by the plugin.
4. The plugin returns `HookResult::Ok` (exit code 0) to the dispatcher.

## Invariants

1. `session_id` on the emitted event reflects the value BC-1.02.005 envelope parsing placed into `HostContext.session_id` — preserved verbatim from the envelope (or `"unknown"` if missing/empty), never transformed, truncated, or replaced by the plugin (the plugin does not set it; `emit_event` auto-enriches it from HostContext).
2. The `session.ended` event-name literal is immutable and reserved per PRD FR-046; renaming requires a new BC.
3. `emit_event` is called before the plugin function returns.
4. `duration_ms` is never absent or null in the emitted payload — it defaults to `"0"` when `session_start_ts` is absent OR in the future OR unparseable as ISO-8601 per PC-2 branches (a)/(b)/(c). Branch (c) applies only to string-typed envelope values; non-string-type handling is a v1.1 candidate (deferred to BC-1.02.005 envelope-value type validation extension).
5. `tool_call_count` is never absent or null in the emitted payload — it defaults to `"0"` when the envelope's `tool_call_count` field is absent per EC-002. Parse-failure handling for non-numeric `tool_call_count` values is a v1.1 candidate (deferred to BC-1.02.005 envelope-value type validation extension).

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001a | `session_start_ts` is absent from the `SessionEnd` envelope | `duration_ms = "0"` in the emitted `session.ended` event; all other fields emitted normally; plugin does not abort |
| EC-001b | `session_start_ts` is present in the `SessionEnd` envelope but is in the future relative to `now_ms` (clock skew / test fixture / adversarial input) | `duration_ms = "0"` (negative elapsed time clamped to zero); all other fields emitted normally; plugin does not abort. This is the clock-skew safeguard that prevents negative duration values on the wire. |
| EC-001c | `session_start_ts` is present as a JSON string but unparseable as ISO-8601 (e.g., `"garbage"`, empty string) | Plugin treats unparseable string as functionally absent; `duration_ms = "0"` (defers to PC-2 branch (a) absent-field default per EC-001c); all other fields emitted normally; plugin does not abort. NOTE: Non-string envelope types (JSON number, null, object) are out of scope; v1.1 candidate to lift to BC-1.02.005. |
| EC-002 | `tool_call_count` is absent from the `SessionEnd` envelope | `tool_call_count = "0"` in the emitted `session.ended` event; all other fields emitted normally; plugin does not abort |
| EC-003 | Both `session_start_ts` and `tool_call_count` are absent from the envelope | `duration_ms = "0"`, `tool_call_count = "0"`; `timestamp` is the plugin's own emission time; plugin emits normally |
| EC-004 | `session_id` is missing or empty string in the `SessionEnd` envelope | BC-1.02.005 lifecycle-tolerance sets `HostContext.session_id = "unknown"`; `emit_event` auto-enriches the event with this value; plugin is unconditionally stateless per BC-4.05.003; emits normally. Note: `duration_ms` and `tool_call_count` follow EC-001/EC-002 independently of `session_id` presence — they read different envelope fields (`session_start_ts` and `tool_call_count` respectively) and are unaffected by `session_id` being missing or empty. |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| `SessionEnd` envelope with `session_id = "sess-abc-123"`, `session_start_ts = "2026-04-28T12:00:00.000Z"`, `tool_call_count = 42`, dispatcher routes to session-end-telemetry.wasm | `session.ended` emitted once; `duration_ms` is a non-negative decimal string representing elapsed ms since session_start_ts; `tool_call_count = "42"` (string on wire per emit_event.rs:49); `timestamp` matches regex `^\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}\.\d{3}Z$`; host-enriched fields present: `session_id = "sess-abc-123"`, `dispatcher_trace_id` non-empty string, `plugin_name` non-empty string, `plugin_version` non-empty string | happy-path |
| `SessionEnd` envelope with `session_id = "sess-def-456"`, `session_start_ts` absent, `tool_call_count` absent | `session.ended` emitted once; `duration_ms = "0"`, `tool_call_count = "0"`, `timestamp` present with correct format; `session_id = "sess-def-456"` (host-enriched) | edge-case (both absent) |
| `SessionEnd` envelope with `session_id = ""` (empty string) | `session.ended` emitted once; `session_id = "unknown"` (BC-1.02.005 sets "unknown" sentinel; emit_event auto-enriches); `duration_ms = "0"` if session_start_ts absent; plugin does not abort | edge-case (missing session_id) |
| `SessionEnd` envelope with `session_start_ts` set to a timestamp 60 seconds in the future relative to dispatch time | `session.ended` emitted once; `duration_ms = "0"` (clock-skew safeguard: negative elapsed time clamped to zero per EC-001b); all other fields emitted normally | edge-case (future session_start_ts / clock skew) |

## Notes

**Malformed-envelope handling (scope boundary):** This BC handles only field-presence cases (EC-001a, EC-001b, EC-002, EC-003, EC-004) and the parse-failure case for `session_start_ts` (EC-001c). Envelope malformation (non-JSON envelope, etc.) is handled upstream by BC-1.02.005 lifecycle-tolerant envelope parsing before this plugin is ever invoked. If BC-1.02.005 admits the envelope (i.e., it parses as valid JSON with a recognized event type), this BC handles field-presence cases and `session_start_ts` parse failures.

**Parse-failure semantics for `session_start_ts` (v1.0 — string-only scope):** If `session_start_ts` is present as a JSON string but unparseable as ISO-8601 (e.g., `"garbage"`, empty string), the plugin treats it as functionally absent and emits `duration_ms = "0"` (defers to PC-2 branch (c) string-only parse-failure clause). Non-string-type envelope values (JSON number, null, object) are out of scope for this BC; handling is a v1.1 candidate to lift to BC-1.02.005 envelope-value type validation extension.

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
| L2 Domain Invariants | DI-007 **REMOVED** (retroactive sibling-sweep fix from S-5.03 ADV-S5.03-P01: DI-007 is "Dispatcher self-telemetry is always-on" — scoped to dispatcher-internal-YYYY-MM-DD.jsonl and SS-03 internal_log.rs; does NOT govern plugin-emitted events. No current DI for plugin event emission unconditionally; v1.1 candidate.); DI-017 (dispatcher_trace_id on every emitted event — automatically enriched by emit_event host fn from HostContext; not the plugin's responsibility to set); BC-1.02.005 (lifecycle-tolerant envelope parsing populates HostContext.session_id used by emit_event auto-enrichment; "unknown" sentinel set at envelope-parse layer, not by the plugin) |
| Architecture Module | SS-04 — `crates/hook-plugins/session-end-telemetry/src/lib.rs` |
| Stories | S-5.02 |
| Functional Requirement | FR-046 |

## Changelog

| Version | Date | Author | Change |
|---------|------|--------|--------|
| v1.2 | 2026-04-28 | product-owner | ADV-S5.03-P04 sibling-sweep MED-P04-006 — abstract construction-time framing propagated from VP-067 v1.2 (MED-P03-001 closure). "set by `InternalEvent::now()`" concrete attribution replaced with "set by the dispatcher between the plugin's `emit_event` call and the final wire format; the plugin must NOT set them — implementation provenance is opaque from the spec layer" in Postconditions §2 Construction-time fields description (line 57) and RESERVED_FIELDS inline note (line 63). Third retroactive edit to S-5.02 BCs in S-5.03 cycle. |
| v1.1 | 2026-04-28 | product-owner | Retroactive sibling-sweep fix from S-5.03 ADV-S5.03-P01: (HIGH-004 sweep) DI-007 removed from Traceability — DI-007 is dispatcher self-telemetry (SS-03 scope), not plugin-emitted event emission; replaced with "no current DI; v1.1 candidate" annotation; S-5.02 story body NOT bumped per bc_array_changes_propagate_to_body_and_acs policy. Sibling-sweep findings considered: HIGH-004 (DI-007 removal) — APPLIED; HIGH-003 (4+3+1 RESERVED_FIELDS split) — NOT APPLICABLE (BC-4.05.001 already uses 4+4 grouping per "Field provenance — 4+4+3 split"; HIGH-003 was reverted in S-5.03 P02 confirming 4+4 is canonical). |
| v1.0 | 2026-04-27 | product-owner | Final state after S-5.02 convergence passes (v1.0-pass-1 through v1.0-pass-5) |
