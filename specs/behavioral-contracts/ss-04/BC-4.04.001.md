---
document_type: behavioral-contract
level: L3
version: "1.0"
status: draft
producer: product-owner
timestamp: 2026-04-28T00:00:00
phase: 1a
inputs:
  - .factory/stories/S-5.01-session-start-hook.md
  - .factory/specs/domain-spec/capabilities.md
input-hash: "73d0f03"
traces_to: .factory/specs/prd.md#FR-046
origin: greenfield
extracted_from: null
subsystem: "SS-04"
capability: "CAP-002"
lifecycle_status: active
introduced: v1.0.0-rc.1
modified: [v1.0-pass-1, v1.0-pass-2, v1.0-pass-4, v1.0-pass-5, v1.0-pass-6, v1.0-pass-7, v1.0-pass-7-po, v1.0-pass-8]
deprecated: null
deprecated_by: null
replacement: null
retired: null
removed: null
removal_reason: null
---

# BC-4.04.001: session-start plugin emits session.started event with session telemetry on SessionStart event

## Description

When the dispatcher routes a `SessionStart` event to the `session-start-telemetry.wasm` plugin via the `hooks.json.template` registration, the plugin emits a `session.started` event via the `emit_event` host function. The emitted event contains the canonical session telemetry payload. Six fields are set by the plugin: `factory_version`, `plugin_count`, `activated_platform`, `factory_health`, `tool_deps`, and `timestamp`. Four fields are automatically injected by the `emit_event` host fn from `HostContext` (per BC-1.05.012 enrichment; the plugin does not set these): `dispatcher_trace_id`, `session_id`, `plugin_name`, `plugin_version`. The `session.started` event-name literal is reserved per PRD FR-046.

## Preconditions

1. A `SessionStart` event has arrived at the dispatcher.
2. `hooks.json` (generated from `hooks.json.template`) contains a `SessionStart` entry routing to `session-start-telemetry.wasm`.
3. The `session-start-telemetry.wasm` plugin is loaded in the dispatcher's `PluginCache`.

## Postconditions

1. The plugin invokes `emit_event` exactly once with `event_name = "session.started"`.
2. The emitted payload contains all required fields. Fields are categorized by who sets them:

   **Plugin-set fields (6 fields — the plugin sets these via `emit_event` key/value pairs):**
   - `factory_version` (string) — compile-time `env!("CARGO_PKG_VERSION")` from `crates/hook-plugins/session-start-telemetry/Cargo.toml`. This is the session-start-telemetry plugin's own crate version, which serves as a proxy for the factory binary version because the plugin is shipped as part of the factory binary release. If factory and plugin versions diverge (e.g., a mismatched deployment), the plugin's compile-time version is reported — this is the authoritative value and the mismatch is an operator concern, not a plugin error. **Relationship to `plugin_version`:** `factory_version` and the host-enriched `plugin_version` (injected by `emit_event` from `HostContext.plugin_version`) typically hold the same value in a standard release, because both reflect the plugin crate's `CARGO_PKG_VERSION`. This redundancy is intentional in v1.0: `factory_version` exists for downstream-consumer compatibility (consumers that expect a factory-binary-version field in the payload) and as an operator-visible drift indicator (if the two values diverge, the deployment is non-canonical). A future typed `emit_event_typed` ABI or a dedicated dispatcher-version field may eliminate this redundancy; tracked as a v1.1 candidate.
   - `plugin_count` (integer ≥ 0, semantic type) — count of WASM plugins loaded in the dispatcher's `PluginCache` at the time of this `SessionStart` event (canonical source: SS-01 plugin cache)
   - `activated_platform` (string) — platform identifier (e.g., `"darwin-arm64"`, `"linux-x64"`) read from `.claude/settings.local.json` key `vsdd-factory.activated_platform` via the `read_file` host fn by invoking `read_file(path = ".claude/settings.local.json", max_bytes = 65536, timeout_ms = 1000)` (`crates/factory-dispatcher/src/host/read_file.rs`). `max_bytes = 65536` (64 KB — generous for the small settings JSON file); `timeout_ms = 1000` (1 second — local file read; advisory only — see §Notes). The plugin declares `[hooks.capabilities.read_file]` with `path_allow = [".claude/settings.local.json"]` per BC-4.04.005. Failure modes (file missing, parse error, key absent, capability denied) → `activated_platform = "unknown"` (fail-open)
   - `factory_health` (one of `"healthy"`, `"warnings"`, `"errors"`, `"unknown"`)
   - `tool_deps` (`object | null`, semantic type) — keys restricted to v1.0 whitelist `["git", "jq", "yq", "rustc", "cargo"]`; values are version strings (max 64 chars each); total payload ≤ 512 bytes measured as the JSON-serialized `tool_deps` object with no whitespace and lexicographically sorted keys (canonical `serde_json` default serialization); if the serialized form exceeds 512 bytes, keys are evicted in reverse-whitelist order (see EC-003)
   - `timestamp` (ISO-8601 UTC with millisecond precision and `Z` suffix; regex: `^\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}\.\d{3}Z$`; example: `2026-04-28T12:34:56.789Z`)

   **Notes:**

   **Field provenance — 4+4+6 split (F-P7-02):** The 14 fields on the wire come from three distinct sources:

   - **Host-enriched fields (4 fields — set by `emit_event` host fn from `HostContext`, NOT by the plugin):** `dispatcher_trace_id`, `session_id`, `plugin_name`, `plugin_version`. The `emit_event` host fn calls `.with_trace_id`, `.with_session_id`, `.with_plugin_name`, `.with_plugin_version` from `HostContext` on every emitted event (per BC-1.05.012 enrichment half). These are part of `RESERVED_FIELDS` and would be silently dropped if the plugin attempted to set them. `session_id` specifically: the value originates from the incoming `SessionStart` envelope parsed by BC-1.02.005 lifecycle-tolerant envelope parsing, which populates `HostContext.session_id`. When the envelope's `session_id` is missing or empty, BC-1.02.005 sets `HostContext.session_id = "unknown"`; the `emit_event` host fn then auto-enriches the event with this value. The plugin does not set `session_id`.

   - **Construction-time fields (4 fields — set by `InternalEvent::now()`, NOT by the plugin or `emit_event` enrichment):** `ts` (current UTC time), `ts_epoch` (current Unix timestamp), `schema_version` (struct constant), `type` (the plugin-supplied `event_name` argument — `"session.started"` in this case). These are set at event construction before the `emit_event` enrichment loop. Also part of `RESERVED_FIELDS`; plugin attempts to set them are silently dropped.

   - **Plugin-set fields (6 fields listed above):** set by the plugin via `emit_event` key/value pairs and pass through the non-reserved field path in `emit_event.rs`.

   **`dispatcher_trace_id` and `session_id` auto-enrichment (F-P6-01, F-P7-02):** Both fields are NOT set by the plugin. They are automatically injected by the `emit_event` host fn from `HostContext`. The plugin's required-fields list does not include either — they are dispatcher-owned identity fields in `RESERVED_FIELDS` and would be silently dropped if the plugin attempted to set them.

   **`read_file` `timeout_ms` is advisory in v1.0 (F-P6-02):** The `timeout_ms = 1000` argument passed to `read_file` is accepted by the ABI for stability but is currently discarded (`read_file.rs:36 — let _ = timeout_ms;`). The effective timeout for the file read is bounded by the dispatcher's epoch budget (8000ms per BC-4.04.005 Postcondition 5) rather than the per-call `timeout_ms`. Load-time `timeout_ms` enforcement via epoch interruption is a v1.1 candidate per S-1.5 epoch-interruption refinement (noted in the host fn source comment).

   **Wire format — all field values are strings (F-P6-05):** The `emit_event` host fn coerces all field values to `JSON strings` on the wire (`emit_event.rs:49 — ev = ev.with_field(&k, Value::String(v))`). The semantic types listed above (`plugin_count` as integer, `tool_deps` as object) describe the INTENDED schema; on the wire all values are JSON strings (e.g., `plugin_count` arrives as `"12"`, `tool_deps` as `'{"git":"2.42.0"}'`). Downstream consumers (file sink readers, observability dashboards) MUST parse string values back to their semantic types. A typed `emit_event_typed` ABI is a v1.1 candidate.
3. `session_id` in the emitted event matches the `session_id` that BC-1.02.005 lifecycle-tolerant envelope parsing populated into `HostContext.session_id` from the incoming `SessionStart` envelope — auto-enriched by the `emit_event` host fn, not set by the plugin.
4. The plugin returns `HookResult::Ok` (exit code 0) to the dispatcher.

## Invariants

1. `session_id` on the emitted event reflects the value BC-1.02.005 envelope parsing placed into `HostContext.session_id` — preserved verbatim from the envelope (or `"unknown"` if missing/empty), never transformed, truncated, or replaced by the plugin (the plugin does not set it; `emit_event` auto-enriches it from HostContext).
2. The `session.started` event-name literal is immutable and reserved per PRD FR-046; renaming requires a new BC.
3. `emit_event` is called before the plugin function returns.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | `session_id` is missing or empty string in the `SessionStart` envelope | When the envelope's `session_id` is missing or empty, BC-1.02.005 lifecycle-tolerance sets `HostContext.session_id = "unknown"`; the `emit_event` host fn auto-enriches the event with this value. The plugin does not set `session_id`. Plugin is unconditionally stateless (no dedup at any layer per BC-4.04.003); two `SessionStart` events with `session_id = "unknown"` both emit independently. Does not abort. |
| EC-002 | `activated_platform` read fails via `read_file` host fn | Any of the following map to `activated_platform = "unknown"` (fail-open): (a) `.claude/settings.local.json` is missing or unreadable; (b) file exists but `vsdd-factory.activated_platform` key is absent; (c) key value is not a string (e.g., integer, object); (d) `read_file` capability denied (capability not declared or `path_allow` does not cover the path, per DI-004). Plugin emits `session.started` with `activated_platform = "unknown"`; does not abort. |
| EC-003 | `tool_deps` payload exceeds 512-byte size budget | **Canonical encoding:** "Total payload" is the JSON-serialized `tool_deps` object as it appears in the emitted `session.started` event after `serde_json` serialization with default settings (no whitespace, keys in lexicographic/sorted order for determinism). Example with 5 tools at normal version string lengths: `{"cargo":"1.78.0","git":"2.42.0","jq":"1.7","rustc":"1.78.0","yq":"4.40.5"}` ≈ 71 bytes. The 512-byte budget applies to this serialized form. **Eviction algorithm (F-P8-03):** Iterate the canonical whitelist `["git","jq","yq","rustc","cargo"]` IN REVERSE. For each key, if removing that key from the `tool_deps` map causes the JSON-serialized form (lexicographically sorted, no whitespace) to fit within 512 bytes, drop it and re-serialize. Continue until the budget is met. Concretely, eviction order: `cargo` first, then `rustc`, then `yq`, then `jq`, then `git`. The on-wire form is always lexicographically sorted by `serde_json` defaults regardless of eviction order. **Adversarial Test Vector (near-budget):** all 5 tools present, each with a 64-byte version string (per-value maximum) → serialized form ≈ `{"cargo":"<64 chars>","git":"<64 chars>","jq":"<64 chars>","rustc":"<64 chars>","yq":"<64 chars>"}` ≈ 387 bytes — within budget with all 5 keys. To trigger eviction, use 6 or more tools or version strings exceeding 64 chars; for the eviction path test, inject a synthetic 6th key with a 64-byte value so the serialized form exceeds 512 bytes, confirm `cargo` is evicted first, confirm resulting serialized form ≤ 512 bytes, confirm `session.started` is emitted with truncated `tool_deps`. |
| EC-004 | `tool_deps` detection fails (timeout or permission error) | Plugin emits with `tool_deps = null`; does not abort |
| EC-005 | Host crate `factory_version` from `env!("CARGO_PKG_VERSION")` differs from any registry record (version drift between crates in the workspace) | Plugin emits `factory_version` from compile-time constant — this is the host crate's canonical identity. No error; the compile-time value is the authoritative version. Registry drift is an operator concern, not a plugin error. |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| `SessionStart` envelope with `session_id = "sess-abc-123"`, `dispatcher_trace_id = "trace-abc-001"`, all runtime reads succeed | `session.started` emitted once; payload has `session_id = "sess-abc-123"` (auto-enriched from HostContext by `emit_event` host fn; plugin does not set this), `dispatcher_trace_id = "trace-abc-001"` (auto-enriched from HostContext by `emit_event` host fn; plugin does not set this), `factory_health` in `{"healthy","warnings","errors","unknown"}`, all required plugin-set fields present (`factory_version`, `plugin_count`, `activated_platform`, `factory_health`, `tool_deps`, `timestamp`), `timestamp` matches regex `^\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}\.\d{3}Z$` | happy-path |
| `SessionStart` envelope with `session_id = ""` (empty string) | `session.started` emitted once; payload has `session_id = "unknown"`; plugin is unconditionally stateless — no dedup at any layer (per BC-4.04.003); a second invocation with `session_id = "unknown"` also emits independently | edge-case |
| `SessionStart` envelope with `session_id = "sess-xyz"`, `activated_platform` read returns error (missing key) | `session.started` emitted once; payload has `activated_platform = "unknown"`, `session_id = "sess-xyz"` | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-065 | Session-Start Plugin Surface Invariant — All BC-4.04.* Postconditions Hold in Integration Test | integration |

## Related BCs

- **BC-4.04.002** — composes with (factory-health subprocess populates `factory_health` field emitted here)
- **BC-4.04.003** — composes with (plugin emits unconditionally per invocation; Layer 1 once-discipline per BC-4.04.004 ensures single emission per session)
- **BC-4.04.004** — depends on (hooks.json.template registration triggers this plugin; Layer 1 `once: true` directive is the upstream once-per-session guarantee)
- **BC-4.04.005** — depends on (hooks-registry.toml registration provides dispatcher-side routing to this plugin; read_file capability declaration in this entry enables the activated_platform read)
- **BC-1.02.005** — depends on (dispatcher envelope parsing delivers `session_id` to this plugin)
- **BC-1.10.001** — retired in pass-4 (bespoke `vsdd::activated_platform()` host fn replaced by canonical `read_file` host fn + `path_allow` pattern declared in BC-4.04.005)

## Architecture Anchors

- SS-04 — `crates/hook-plugins/session-start-telemetry/src/lib.rs` (plugin `on_hook` entry point + `emit_event` call)
- SS-01 — dispatcher routes `SessionStart` to `session-start-telemetry.wasm` per `hooks.json` registry entry

## Story Anchor

S-5.01

## VP Anchors

VP-065

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-002 |
| Capability Anchor Justification | CAP-002 ("Hook Claude Code tool calls with sandboxed WASM plugins") per capabilities.md §CAP-002 |
| L2 Domain Invariants | DI-004 (capability denial emits audit event — applies to both exec_subprocess capability gate for factory-health AND read_file capability gate for activated_platform; capability denied for either maps to the respective "unknown" fallback per fail-open design); DI-007 (always-on self-telemetry — session.started is part of the always-on telemetry surface); DI-017 (dispatcher_trace_id on every emitted event — automatically enriched by emit_event host fn from HostContext; not the plugin's responsibility to set; plugin does not include trace_id in its required-fields list); BC-1.02.005 (lifecycle-tolerant envelope parsing populates HostContext.session_id used by emit_event auto-enrichment; "unknown" sentinel set at envelope-parse layer, not by the plugin) |
| Architecture Module | SS-04 — `crates/hook-plugins/session-start-telemetry/src/lib.rs` |
| Stories | S-5.01 |
| Functional Requirement | FR-046 |
