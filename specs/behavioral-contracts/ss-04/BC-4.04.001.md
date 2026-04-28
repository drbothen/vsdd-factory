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
input-hash: "20ed836"
traces_to: .factory/specs/prd.md#FR-046
origin: greenfield
extracted_from: null
subsystem: "SS-04"
capability: "CAP-002"
lifecycle_status: active
introduced: v1.0.0-rc.1
modified: [v1.0-pass-1, v1.0-pass-2]
deprecated: null
deprecated_by: null
replacement: null
retired: null
removed: null
removal_reason: null
---

# BC-4.04.001: session-start plugin emits session.started event with session telemetry on SessionStart event

## Description

When the dispatcher routes a `SessionStart` event to the `session-start-telemetry.wasm` plugin via the `hooks.json.template` registration, the plugin emits a `session.started` event via the `emit_event` host function. The emitted event contains the canonical session telemetry payload: `session_id`, `factory_version`, `plugin_count`, `activated_platform`, `factory_health`, `tool_deps`, and `timestamp`. The `session.started` event-name literal is reserved per PRD FR-046.

## Preconditions

1. A `SessionStart` event has arrived at the dispatcher.
2. `hooks.json` (generated from `hooks.json.template`) contains a `SessionStart` entry routing to `session-start-telemetry.wasm`.
3. The `session-start-telemetry.wasm` plugin is loaded in the dispatcher's `PluginCache`.

## Postconditions

1. The plugin invokes `emit_event` exactly once with `event_name = "session.started"`.
2. The emitted payload contains all required fields:
   - `session_id` (string) — value from the incoming envelope or `"unknown"` if missing
   - `factory_version` (string) — compile-time `env!("CARGO_PKG_VERSION")` of the host vsdd-factory plugin crate
   - `plugin_count` (integer ≥ 0) — count of WASM plugins loaded in the dispatcher's `PluginCache` at the time of this `SessionStart` event (canonical source: SS-01 plugin cache)
   - `activated_platform` (string) — platform identifier (e.g., `"darwin-arm64"`, `"linux-x86_64"`) obtained via the `vsdd::activated_platform()` host function per BC-1.10.001 (the dispatcher mediates the read from `.claude/settings.local.json`; no `read_file` capability required)
   - `factory_health` (one of `"healthy"`, `"warnings"`, `"errors"`, `"unknown"`)
   - `tool_deps` (`object | null`) — keys restricted to v1.0 whitelist `["git", "jq", "yq", "rustc", "cargo"]`; values are version strings (max 64 chars each); total payload ≤ 512 bytes
   - `timestamp` (ISO-8601 UTC with millisecond precision and `Z` suffix; regex: `^\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}\.\d{3}Z$`; example: `2026-04-28T12:34:56.789Z`)
3. `session_id` in the emitted event matches the `session_id` from the incoming `SessionStart` envelope verbatim.
4. The plugin returns `HookResult::Ok` (exit code 0) to the dispatcher.

## Invariants

1. `session_id` is preserved verbatim from the `SessionStart` envelope — never transformed, truncated, or replaced.
2. The `session.started` event-name literal is immutable and reserved per PRD FR-046; renaming requires a new BC.
3. `emit_event` is called before the plugin function returns.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | `session_id` is missing or empty string in the `SessionStart` envelope | Plugin emits `session.started` with `session_id = "unknown"`; dedup is skipped for this sentinel value (two sessions both with missing session_id both emit); does not abort |
| EC-002 | `activated_platform` host fn returns `"unknown"` (file missing, key absent, value not a string — per BC-1.10.001 error paths) | Plugin emits with `activated_platform = "unknown"`; does not abort |
| EC-003 | `tool_deps` payload exceeds 512-byte size budget | Keys are dropped in iteration order over the v1.0 whitelist `["git", "jq", "yq", "rustc", "cargo"]`, dropping from the END of that list first (i.e., `cargo` is dropped first, then `rustc`, then `yq`, etc.), until total payload ≤ 512 bytes. Canonical Test Vector: all 5 tools present with version strings totaling 600 bytes → `cargo` is dropped first; resulting payload ≤ 512 bytes; `session.started` emitted with truncated `tool_deps`. |
| EC-004 | `tool_deps` detection fails (timeout or permission error) | Plugin emits with `tool_deps = null`; does not abort |
| EC-005 | Host crate `factory_version` from `env!("CARGO_PKG_VERSION")` differs from any registry record (version drift between crates in the workspace) | Plugin emits `factory_version` from compile-time constant — this is the host crate's canonical identity. No error; the compile-time value is the authoritative version. Registry drift is an operator concern, not a plugin error. |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| `SessionStart` envelope with `session_id = "sess-abc-123"`, all runtime reads succeed | `session.started` emitted once; payload has `session_id = "sess-abc-123"`, `factory_health` in `{"healthy","warnings","errors","unknown"}`, all required fields present, `timestamp` matches regex `^\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}\.\d{3}Z$` | happy-path |
| `SessionStart` envelope with `session_id = ""` (empty string) | `session.started` emitted once; payload has `session_id = "unknown"`; dedup NOT applied for this record | edge-case |
| `SessionStart` envelope with `session_id = "sess-xyz"`, `activated_platform` read returns error (missing key) | `session.started` emitted once; payload has `activated_platform = "unknown"`, `session_id = "sess-xyz"` | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-065 | Session-Start Plugin Surface Invariant — All BC-4.04.* Postconditions Hold in Integration Test | integration |

## Related BCs

- **BC-4.04.002** — composes with (factory-health subprocess populates `factory_health` field emitted here)
- **BC-4.04.003** — composes with (idempotency guard prevents duplicate `session.started` emission)
- **BC-4.04.004** — depends on (hooks.json.template registration triggers this plugin)
- **BC-4.04.005** — depends on (hooks-registry.toml registration provides dispatcher-side routing to this plugin)
- **BC-1.02.005** — depends on (dispatcher envelope parsing delivers `session_id` to this plugin)
- **BC-1.10.001** — depends on (`vsdd::activated_platform()` host fn mediates the `.claude/settings.local.json` read; replaces direct file-path read and BC-6.01.005 dependency)

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
| L2 Domain Invariants | DI-004 (capability denial emits audit event — applies to exec_subprocess capability gate for factory-health; the activated_platform host fn per BC-1.10.001 is specifically designed to avoid requiring read_file capability, avoiding a DI-004 trigger for that read); DI-007 (always-on self-telemetry — session.started is part of the always-on telemetry surface); DI-017 (dispatcher_trace_id on every emitted event — session.started carries trace_id from the incoming SessionStart envelope) |
| Architecture Module | SS-04 — `crates/hook-plugins/session-start-telemetry/src/lib.rs` |
| Stories | S-5.01 |
| Functional Requirement | FR-046 |
