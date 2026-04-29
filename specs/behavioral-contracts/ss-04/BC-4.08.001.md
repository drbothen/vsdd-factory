---
document_type: behavioral-contract
level: L3
version: "v1.0"
status: active
producer: product-owner
timestamp: 2026-04-28T00:00:00
phase: 1a
inputs:
  - .factory/stories/S-5.04-post-tool-use-failure.md
  - .factory/specs/domain-spec/capabilities.md
input-hash: "80444f0"
traces_to: .factory/specs/prd.md#FR-046
origin: greenfield
extracted_from: null
subsystem: "SS-04"
capability: "CAP-013"
lifecycle_status: active
introduced: v1.0.0-rc.1
modified: []
deprecated: null
deprecated_by: null
replacement: null
retired: null
removed: null
removal_reason: null
---

# BC-4.08.001: tool-failure-hooks plugin emits tool.error event with {tool_name, error_message} on PostToolUseFailure event; tool_name="unknown" if absent; error_message truncated to 2000 chars; 10-field wire payload; RESERVED_FIELDS not set by plugin

## Description

When the dispatcher routes a `PostToolUseFailure` event to the `tool-failure-hooks.wasm` plugin via the `hooks.json.template` + `hooks-registry.toml` dual-layer registration, the plugin emits a `tool.error` event via the `emit_event` host function. Two fields are set by the plugin: `tool_name` (sourced from the incoming `PostToolUseFailure` envelope, defaulting to `"unknown"` if absent — EC-002) and `error_message` (sourced from the envelope, truncated to 2000 characters if longer — EC-001). Eight additional fields are reserved and NOT settable by the plugin (RESERVED_FIELDS), set by the host in two groups: (a) 4 host-enriched from `HostContext` by `emit_event`: `dispatcher_trace_id`, `session_id`, `plugin_name`, `plugin_version`; (b) 4 construction-time fields set by the dispatcher between plugin `emit_event` call and final wire format: `ts`, `ts_epoch`, `schema_version`, `type`. The plugin MUST NOT set any of the 8 RESERVED_FIELDS. Total fields on wire: 10 (2 plugin-set + 4 host-enriched + 4 construction-time). The plugin performs NO filesystem reads, NO subprocess invocations, and requires ZERO declared capabilities — all required data is read from the incoming envelope. This is the Option A (zero-capability) scoping decision applied to PostToolUseFailure, mirroring S-5.02 (BC-4.05.001–005) and S-5.03 (BC-4.07.001–004).

## Scoping Decision: Option A (Zero-Capability) — Rationale

**Option A (SELECTED):** The plugin emits `tool.error` ONLY. No filesystem reads, no subprocess invocation. The observability stack consumes the event at runtime. This mirrors the S-5.02 and S-5.03 zero-capability pattern exactly.

**Option B (REJECTED):** Use `exec_subprocess` to invoke an alerting CLI tool. Adds capability scope creep; hard to test deterministically; requires a new CLI binary. Violates S-5.01 lesson 3 (avoid shell-out tricks). Rejected.

**Option C (REJECTED):** Use `read_file` to load error context from a file. PostToolUseFailure envelope already contains all needed data (`tool_name`, `error_message`). No file read needed. Rejected.

**Note on `session_id` (S-5.04 legacy-story correction):** The legacy story (v1.2, line 78) listed `session_id` as a plugin-set field. This is incorrect. `session_id` is a RESERVED_FIELDS host-enriched field set by the `emit_event` host fn from `HostContext`. The plugin MUST NOT set it. Plugin-set fields are exactly 2: `tool_name` + `error_message`.

## Preconditions

1. A `PostToolUseFailure` event has arrived at the dispatcher.
2. `hooks.json` (generated from `hooks.json.template`) contains a `PostToolUseFailure` entry routing to the dispatcher binary, which then routes to `tool-failure-hooks.wasm` via `hooks-registry.toml`.
3. The `tool-failure-hooks.wasm` plugin is loaded in the dispatcher's `PluginCache`.
4. The incoming `PostToolUseFailure` envelope may contain `tool_name` (string identifying the failing tool) and `error_message` (string describing the failure). Both fields are optional in the envelope; plugin handles absence of either.

## Postconditions

1. The plugin invokes `emit_event` exactly once with `event_name = "tool.error"`.
2. The emitted payload contains all required fields. Fields are categorized by who sets them:

   **Plugin-set fields (2 fields — the plugin sets these via `emit_event` key/value pairs):**
   - `tool_name` (string): name of the failing tool, sourced from the envelope's `tool_name` field. If absent from the envelope, `tool_name = "unknown"` (fallback sentinel per EC-002). Value is always a string on the wire (per `emit_event.rs:49` string coercion).
   - `error_message` (string): description of the failure, sourced from the envelope's `error_message` field. If the envelope value exceeds 2000 characters, it is truncated to exactly 2000 characters before being set (EC-001). If absent from the envelope, `error_message = ""` (empty string default per EC-003). Value is always a string on the wire.

   **Host-enriched fields (4 fields — set by `emit_event` host fn from `HostContext`, NOT by the plugin):** `dispatcher_trace_id`, `session_id`, `plugin_name`, `plugin_version`. These are part of `RESERVED_FIELDS` and are silently dropped if the plugin attempts to set them. Each is a non-empty string per BC-1.05.012 unconditional enrichment. The plugin MUST NOT set `session_id` — it is host-enriched, not plugin-set (correction from legacy S-5.04 story v1.2 which incorrectly listed session_id as plugin-set).

   **Construction-time fields (4 fields — set by the dispatcher between plugin `emit_event` call and final wire format, NOT by the plugin):** `ts`, `ts_epoch`, `schema_version`, `type`. Part of `RESERVED_FIELDS`; plugin attempts to set them are silently dropped. `type` MUST equal `"tool.error"`. Implementation provenance for these 4 fields is opaque from the spec layer.

   **Wire format note:** All plugin-set field values are strings on the wire (`emit_event.rs:49` coerces all plugin-supplied values to `Value::String`). Downstream consumers MUST parse string values back to their semantic types.

   **Total wire fields: 10** (2 plugin-set + 4 host-enriched + 4 construction-time). PostToolUseFailure emits 10 fields — same count as WorktreeCreate (2+4+4); one more than WorktreeRemove (1+4+4); fewer than SessionStart (6+4+4) or SessionEnd (3+4+4).

3. The plugin returns `HookResult::Ok` (exit code 0) to the dispatcher.

## Invariants

1. The plugin performs NO filesystem reads. The envelope contains all required data. No `read_file` host fn is declared in BC-4.08.003's `hooks-registry.toml` entry. Any invocation attempt would receive `CAPABILITY_DENIED` from the host fn dispatch table (BC-1.05.021).
2. The plugin performs NO subprocess invocations. `exec_subprocess` is NOT declared in BC-4.08.003's `hooks-registry.toml` entry. Any invocation attempt would receive `CAPABILITY_DENIED` (BC-1.05.001).
3. `tool_name` is never absent from the emitted payload — it defaults to `"unknown"` when the envelope field is absent. This fallback sentinel ensures consumers can always inspect the field.
4. `error_message` is never absent from the emitted payload — it defaults to `""` when the envelope field is absent. When present but over 2000 characters, it is truncated to exactly 2000 characters.
5. The `tool.error` event-name literal is immutable and reserved per PRD FR-046.
6. `emit_event` is called before the plugin function returns.
7. The plugin is unconditionally stateless — it maintains no in-process state across invocations.
8. `session_id` MUST NOT be set by the plugin — it is RESERVED_FIELDS host-enriched. The legacy S-5.04 story (v1.2) incorrectly listed session_id as a plugin-set field; this BC is the authoritative correction.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | `error_message` field in the envelope exceeds 2000 characters | Plugin truncates `error_message` to exactly 2000 characters before emitting. The emitted `error_message` field on the wire is always ≤ 2000 characters. No error or warning — truncation is silent. |
| EC-002 | `tool_name` is absent from the `PostToolUseFailure` envelope | `tool_name = "unknown"` in the emitted `tool.error` event. The fallback sentinel `"unknown"` enables downstream consumers to identify events from unknown tool invocations. Plugin does not abort; emits normally. |
| EC-003 | `error_message` is absent from the `PostToolUseFailure` envelope | `error_message = ""` (empty string) in the emitted `tool.error` event. Plugin does not abort; emits normally. Consumer is responsible for handling empty `error_message`. |
| EC-004 | Both `tool_name` and `error_message` are absent from the envelope | `tool_name = "unknown"`, `error_message = ""` in the emitted event. Plugin emits normally; consumer must handle both sentinel values. |
| EC-005 | `error_message` is exactly 2000 characters long | No truncation. `error_message` emitted as-is (boundary condition — truncation applies strictly when length > 2000). |
| EC-006 | `session_id` is missing or empty in the `PostToolUseFailure` envelope | BC-1.02.005 lifecycle-tolerance sets `HostContext.session_id = "unknown"`; `emit_event` auto-enriches the event with this value; plugin emits normally. Plugin does NOT set session_id (RESERVED). |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| `PostToolUseFailure` envelope with `tool_name = "Bash"`, `error_message = "command not found"`, `session_id = "fail-sess-001"`, dispatcher routes to tool-failure-hooks.wasm | `tool.error` emitted once; `tool_name = "Bash"` (string on wire); `error_message = "command not found"` (string on wire); `session_id = "fail-sess-001"` (host-enriched, NOT plugin-set); `dispatcher_trace_id` non-empty string (host-enriched); `plugin_name` non-empty string (host-enriched); `plugin_version` non-empty string (host-enriched); `type = "tool.error"` (construction-time); total 10 fields; `exec_subprocess` CountingMock invocation_count == 0 | happy-path |
| `PostToolUseFailure` envelope with `tool_name` absent, `error_message = "timeout"` | `tool.error` emitted once; `tool_name = "unknown"` (fallback sentinel); `error_message = "timeout"` (string on wire); host-enriched and construction-time fields present normally | edge-case (missing tool_name, EC-002) |
| `PostToolUseFailure` envelope with `error_message` = 2500-character string | `tool.error` emitted once; `error_message` is exactly 2000 characters (truncated from 2500); all other fields present normally | edge-case (truncation, EC-001) |
| `PostToolUseFailure` envelope with `error_message` absent | `tool.error` emitted once; `error_message = ""` (empty string); `tool_name` present or `"unknown"`; plugin does not abort | edge-case (missing error_message, EC-003) |
| `PostToolUseFailure` envelope with `error_message` exactly 2000 characters | `tool.error` emitted once; `error_message` is exactly 2000 characters (no truncation); all other fields present normally | edge-case (boundary, EC-005) |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-068 | Tool-Failure Hook Plugin Surface Invariant — All BC-4.08.* Postconditions Hold in Integration Test | integration |

## Related BCs

- **BC-4.08.002** — depends on (hooks.json.template PostToolUseFailure registration triggers this plugin via dispatcher routing)
- **BC-4.08.003** — depends on (hooks-registry.toml PostToolUseFailure routing entry provides dispatcher-side routing to tool-failure-hooks.wasm)
- **BC-1.02.005** — depends on (dispatcher envelope parsing delivers `session_id` to this plugin via HostContext; "unknown" sentinel used when absent)
- **BC-1.05.012** — depends on (emit_event host fn auto-enriches with host-enriched fields including session_id, dispatcher_trace_id, plugin_name, plugin_version)
- **BC-4.05.001** — structural analog (SessionEnd event emission pattern; PostToolUseFailure is same field count: 2 plugin-set + 4 host-enriched + 4 construction-time = 10)
- **BC-4.07.001** — structural analog (WorktreeCreate event emission pattern; same 10-field count; same zero-capability profile)
- **BC-1.05.001** — depends on (exec_subprocess denied when no exec_subprocess capability declared — enforces zero-capability sandbox for this plugin)
- **BC-1.05.021** — depends on (read_file denied when no Capabilities.read_file block — enforces zero-capability sandbox for this plugin)

## Architecture Anchors

- SS-04 — `crates/hook-plugins/tool-failure-hooks/src/lib.rs` (plugin `on_hook` entry point + `emit_event` call for PostToolUseFailure path)
- SS-01 — dispatcher routes `PostToolUseFailure` to `tool-failure-hooks.wasm` per `hooks-registry.toml` routing entry (BC-4.08.003)

## Story Anchor

S-5.04

## VP Anchors

VP-068

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-013 |
| Capability Anchor Justification | CAP-013 ("Capture post-execution activity (PostToolUse hooks)") per capabilities.md §CAP-013. PostToolUseFailure is the failure-path variant of PostToolUse; CAP-013 explicitly covers "tool errors for audit and observability purposes" in its description, making it the correct anchor for this BC. |
| L2 Domain Invariants | DI-004 (capability denial — by declaring ZERO capabilities, deny-by-default ensures exec_subprocess and read_file are both denied; plugin never attempts to call them); DI-017 (dispatcher_trace_id on every emitted event — automatically enriched by emit_event host fn from HostContext; not the plugin's responsibility to set); BC-1.02.005 (lifecycle-tolerant envelope parsing populates HostContext.session_id used by emit_event auto-enrichment; "unknown" sentinel set at envelope-parse layer, not by the plugin) |
| Architecture Module | SS-04 — `crates/hook-plugins/tool-failure-hooks/src/lib.rs` |
| Stories | S-5.04 |
| Functional Requirement | FR-046 |

## Changelog

| Version | Date | Author | Change |
|---------|------|--------|--------|
| v1.0 | 2026-04-28 | product-owner | Initial creation (S-5.04 foundation burst). Promoted from v1.1 BC candidate BC-4.08.001 in legacy story. All S-5.01/02/03 lessons applied: 4+4 opaque RESERVED_FIELDS grouping; event_name (not event_type); ZERO declared capabilities (Option A); once key absent (mirrors S-5.03 pattern); session_id corrected to RESERVED (not plugin-set); 2-plugin-set-field count confirmed (tool_name + error_message). |
