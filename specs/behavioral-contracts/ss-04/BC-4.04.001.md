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
input-hash: "0b2120e"
traces_to: .factory/specs/prd.md#FR-046
origin: greenfield
extracted_from: null
subsystem: "SS-04"
capability: "CAP-002"
lifecycle_status: active
introduced: v1.0.0-rc.1
modified: [v1.0-pass-1, v1.0-pass-2, v1.0-pass-4]
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
   - `factory_version` (string) — compile-time `env!("CARGO_PKG_VERSION")` from `crates/hook-plugins/session-start-telemetry/Cargo.toml`. This is the session-start-telemetry plugin's own crate version, which serves as a proxy for the factory version since the plugin is shipped with the factory binary. If factory and plugin versions diverge (e.g., a mismatched deployment), the plugin's compile-time version is reported — this is the authoritative value and the mismatch is an operator concern, not a plugin error.
   - `plugin_count` (integer ≥ 0) — count of WASM plugins loaded in the dispatcher's `PluginCache` at the time of this `SessionStart` event (canonical source: SS-01 plugin cache)
   - `activated_platform` (string) — platform identifier (e.g., `"darwin-arm64"`, `"linux-x86_64"`) read from `.claude/settings.local.json` key `vsdd-factory.activated_platform` via the existing `read_file` host fn (`crates/factory-dispatcher/src/host/read_file.rs`). The plugin declares `[hooks.capabilities.read_file]` with `path_allow = [".claude/settings.local.json"]` per BC-4.04.005. Failure modes (file missing, parse error, key absent, capability denied) → `activated_platform = "unknown"` (fail-open)
   - `factory_health` (one of `"healthy"`, `"warnings"`, `"errors"`, `"unknown"`)
   - `tool_deps` (`object | null`) — keys restricted to v1.0 whitelist `["git", "jq", "yq", "rustc", "cargo"]`; values are version strings (max 64 chars each); total payload ≤ 512 bytes measured as the JSON-serialized `tool_deps` object with no whitespace and lexicographically sorted keys (canonical `serde_json` default serialization); if the serialized form exceeds 512 bytes, keys are evicted in reverse-whitelist order (see EC-003)
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
| EC-002 | `activated_platform` read fails via `read_file` host fn | Any of the following map to `activated_platform = "unknown"` (fail-open): (a) `.claude/settings.local.json` is missing or unreadable; (b) file exists but `vsdd-factory.activated_platform` key is absent; (c) key value is not a string (e.g., integer, object); (d) `read_file` capability denied (capability not declared or `path_allow` does not cover the path, per DI-004). Plugin emits `session.started` with `activated_platform = "unknown"`; does not abort. |
| EC-003 | `tool_deps` payload exceeds 512-byte size budget | **Canonical encoding:** "Total payload" is the JSON-serialized `tool_deps` object as it appears in the emitted `session.started` event after `serde_json` serialization with default settings (no whitespace, keys in lexicographic/sorted order for determinism). Example with 5 tools at normal version string lengths: `{"cargo":"1.78.0","git":"2.42.0","jq":"1.7","rustc":"1.78.0","yq":"4.40.5"}` ≈ 71 bytes. The 512-byte budget applies to this serialized form. Keys are dropped in reverse iteration order over the v1.0 whitelist `["git", "jq", "yq", "rustc", "cargo"]`, dropping from the END of that list first (i.e., `cargo` is dropped first, then `rustc`, then `yq`, etc.), until the serialized `tool_deps` object is ≤ 512 bytes. **Adversarial Test Vector (near-budget):** all 5 tools present, each with a 64-byte version string (per-value maximum) → serialized form ≈ `{"cargo":"<64 chars>","git":"<64 chars>","jq":"<64 chars>","rustc":"<64 chars>","yq":"<64 chars>"}` ≈ 387 bytes — within budget with all 5 keys. To trigger eviction, use 6 or more tools or version strings exceeding 64 chars; for the eviction path test, inject a synthetic 6th key with a 64-byte value so the serialized form exceeds 512 bytes, confirm `cargo` is evicted first, confirm resulting serialized form ≤ 512 bytes, confirm `session.started` is emitted with truncated `tool_deps`. |
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
| L2 Domain Invariants | DI-004 (capability denial emits audit event — applies to both exec_subprocess capability gate for factory-health AND read_file capability gate for activated_platform; capability denied for either maps to the respective "unknown" fallback per fail-open design); DI-007 (always-on self-telemetry — session.started is part of the always-on telemetry surface); DI-017 (dispatcher_trace_id on every emitted event — session.started carries trace_id from the incoming SessionStart envelope) |
| Architecture Module | SS-04 — `crates/hook-plugins/session-start-telemetry/src/lib.rs` |
| Stories | S-5.01 |
| Functional Requirement | FR-046 |
