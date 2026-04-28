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
input-hash: "9865e16"
traces_to: .factory/specs/prd.md#FR-046
origin: greenfield
extracted_from: null
subsystem: "SS-04"
capability: "CAP-002"
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

# BC-4.04.004: hooks.json.template registers SessionStart event with once:true routing to session-start-telemetry.wasm plugin

## Description

The shipped `plugins/vsdd-factory/hooks/hooks.json.template` must contain a `SessionStart` entry that routes to `session-start-telemetry.wasm` with `once: true`. This template is the discoverable activation surface for the session-start plugin — without this entry, the dispatcher never invokes the plugin regardless of whether the WASM binary is present. The `once: true` flag ensures the plugin is invoked at most once per Claude Code session lifecycle, consistent with the idempotency contract in BC-4.04.003.

## Preconditions

1. `plugins/vsdd-factory/hooks/hooks.json.template` is the source-of-truth for hook event routing (per BC-1.01.001 registry schema).
2. The `session-start-telemetry.wasm` binary is present in the platform-specific plugin directory at activation time.

## Postconditions

1. `hooks.json.template` contains a `SessionStart` key in its hook registry.
2. The `SessionStart` entry references `session-start-telemetry.wasm` as the plugin target.
3. The `SessionStart` entry has `once: true` set.
4. The template is syntactically valid JSON that passes the schema validation defined in BC-1.01.001.

## Invariants

1. The `SessionStart` entry must remain present in `hooks.json.template` through all v1.0 releases — removal requires a deprecation pass.
2. `once: true` is immutable for `SessionStart` — changing to `once: false` requires a new BC and explicit justification.
3. The plugin filename `session-start-telemetry.wasm` is the canonical reference; any rename must propagate to this template entry in the same commit.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Plugin binary is renamed from `session-start-telemetry.wasm` to a new name | Template entry `session-start-telemetry.wasm` reference becomes stale; dispatcher logs "plugin not found"; fix requires a coordinated rename commit updating both the binary name and this template entry |
| EC-002 | Platform-specific binary path resolution differs between darwin-arm64 and windows-x64 | Template must use a platform-neutral relative path or use the activation system's path substitution variable; absolute paths in the template are invalid |
| EC-003 | `hooks.json.template` is rendered without the `SessionStart` entry (e.g., template variable substitution strips it) | Dispatcher never invokes session-start plugin; `session.started` events are never emitted; factory-health is never triggered on session start |
| EC-004 | `session-start-telemetry.wasm` binary is not present at expected path at runtime (e.g., partial activation, missing binary after platform switch) | Dispatcher emits `internal.dispatcher_error` event with `plugin_id = "session-start-telemetry"`; `SessionStart` event does not produce `session.started`; dispatcher exit code is unaffected (fail-open). See SS-01 missing-plugin handling for the dispatcher-side enforcement. |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Parse `hooks.json.template` and inspect registry | `SessionStart` key present; value references `session-start-telemetry.wasm`; `once: true` present | happy-path |
| Rename `session-start-telemetry.wasm` to `session-telemetry.wasm` without updating template | Dispatcher reports "plugin not found: session-start-telemetry.wasm" on SessionStart event | error (rename propagation failure) |
| Template rendered on windows-x64 platform | `SessionStart` entry still resolves to correct plugin binary path on that platform | edge-case |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-065 | Session-Start Plugin Surface Invariant — All BC-4.04.* Postconditions Hold in Integration Test | integration |

## Related BCs

- **BC-4.04.001** — enables (this registration is the trigger that causes BC-4.04.001 to execute)
- **BC-4.04.002** — enables (this registration is the trigger that causes BC-4.04.002 to execute)
- **BC-4.04.003** — enables (this registration is the trigger that causes BC-4.04.003 to execute)
- **BC-1.01.001** — depends on (registry schema validation applies to this template entry)
- **BC-1.02.005** — depends on (dispatcher envelope parsing handles the SessionStart event type routed via this entry)

## Architecture Anchors

- SS-04 — `plugins/vsdd-factory/hooks/hooks.json.template` (`SessionStart` registry entry)
- SS-09 — activation system path resolution for `session-start-telemetry.wasm` binary

## Story Anchor

S-5.01

## VP Anchors

VP-065

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-002 |
| Capability Anchor Justification | CAP-002 ("Hook Claude Code tool calls with sandboxed WASM plugins") per capabilities.md §CAP-002 |
| L2 Domain Invariants | DI-014 (schema version mismatch = hard error — hooks.json.template schema_version must remain compatible with registry validator); DI-015 (per-project activation required — hooks.json.template is the activation surface that must be present for dispatcher invocation) |
| Architecture Module | SS-04 — `plugins/vsdd-factory/hooks/hooks.json.template` |
| Stories | S-5.01 |
| Functional Requirement | FR-046 |
