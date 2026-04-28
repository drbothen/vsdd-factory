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
input-hash: "2f50188"
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

# BC-4.04.004: hooks.json.template registers SessionStart event with `command` field routing to dispatcher binary; once:true and async:true

## Description

The shipped `plugins/vsdd-factory/hooks/hooks.json.template` must contain a `SessionStart` entry in its `hooks` object whose `command` field routes to the **dispatcher binary** (`${CLAUDE_PLUGIN_ROOT}/hooks/dispatcher/bin/{{PLATFORM}}/factory-dispatcher{{EXE_SUFFIX}}`), not directly to a WASM plugin filename. This is Layer 1 of the dual-routing-tables pattern per ADR-011: `hooks.json.template` is the Claude Code harness wiring document and knows only about the dispatcher binary. The dispatcher, once invoked, reads `hooks-registry.toml` (BC-4.04.005 — Layer 2) to route the SessionStart event to `session-start-telemetry.wasm`. The `once: true` and `async: true` flags are set on the SessionStart hook entry. The template uses an array-of-objects schema: `hooks.SessionStart` is an array; each element has a nested `hooks` array whose entries have `type = "command"` and `command` pointing to the dispatcher binary.

## Preconditions

1. `plugins/vsdd-factory/hooks/hooks.json.template` is the Claude Code harness wiring source of truth (per ADR-011 dual-hook-routing-tables; Layer 1 of the two-layer routing pattern).
2. The dispatcher binary is present in the platform-specific path at `${CLAUDE_PLUGIN_ROOT}/hooks/dispatcher/bin/{{PLATFORM}}/factory-dispatcher{{EXE_SUFFIX}}` at activation time.

## Postconditions

1. `hooks.json.template` contains a `SessionStart` key in its top-level `hooks` object.
2. The `SessionStart` entry's `command` field in the nested `hooks` array references the dispatcher binary path: `${CLAUDE_PLUGIN_ROOT}/hooks/dispatcher/bin/{{PLATFORM}}/factory-dispatcher{{EXE_SUFFIX}}`. It does NOT reference `session-start-telemetry.wasm` or any other WASM plugin filename (per ADR-011 layer separation).
3. The `SessionStart` hook entry has `once: true` AND `async: true` set.
4. The entry follows the array-of-objects schema: `template["hooks"]["SessionStart"]` is an array; each element is an object with a nested `hooks` array; each nested entry has `type = "command"` and `command` = dispatcher binary path.

## Invariants

1. The `SessionStart` entry in `hooks.json.template` MUST NEVER reference WASM plugin filenames (e.g., `session-start-telemetry.wasm`) — per ADR-011 layer separation, WASM plugin references belong exclusively in `hooks-registry.toml` (BC-4.04.005).
2. The `SessionStart` entry must remain present in `hooks.json.template` through all v1.0 releases — removal requires a deprecation pass.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Dispatcher binary path platform substitution fails (e.g., `{{PLATFORM}}` placeholder not substituted during activation) | Activation system falls back to the previous generated `hooks.json` or reports activation error; the unsubstituted template path is never written to the live `hooks.json` |
| EC-002 | `hooks.json.template` is missing the `SessionStart` key entirely | No `SessionStart` routing exists in the Claude Code harness; the dispatcher is never invoked for SessionStart events; `session.started` events are never emitted |
| EC-003 | Per-platform variant file (`.platform/darwin-arm64/hooks.json`) is out of sync with `hooks.json.template` | Activation regeneration required; the platform variant is the committed artifact used by the activation skill (hooks.json is gitignored) |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Parse `hooks.json.template`; inspect `hooks.SessionStart[0].hooks[0]` | `command` field contains `factory-dispatcher`; `once: true` present; `async: true` present; no reference to `.wasm` filename in `command` field | happy-path |
| Parse `hooks.json.template`; inspect `hooks.SessionStart[0].hooks[0].command` | Value matches pattern `${CLAUDE_PLUGIN_ROOT}/hooks/dispatcher/bin/{{PLATFORM}}/factory-dispatcher{{EXE_SUFFIX}}` (placeholder-unsubstituted form) | happy-path (dispatcher binary routing) |
| `hooks.json.template` is missing `SessionStart` key | No SessionStart entry in `template["hooks"]`; dispatcher never invoked for SessionStart events | error (missing key) |

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

- SS-07 — `plugins/vsdd-factory/hooks/hooks.json.template` (harness wiring source of truth; owned by SS-07 Hook Bash Layer per ADR-011)
- SS-04 — behavioral contract for the SessionStart routing semantics within `hooks.json.template` (routing layer ownership per F-8 ruling)
- SS-09 — activation skill generates the live `hooks.json` (and per-platform variants) from `hooks.json.template` at activation time; the template itself is committed and not gitignored

## Story Anchor

S-5.01

## VP Anchors

VP-065

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-002 |
| Capability Anchor Justification | CAP-002 ("Hook Claude Code tool calls with sandboxed WASM plugins") per capabilities.md §CAP-002 |
| L2 Domain Invariants | DI-015 (per-project activation required — hooks.json.template is the activation surface that must be present for dispatcher invocation) |
| Architecture Module | SS-07 — `plugins/vsdd-factory/hooks/hooks.json.template` (harness wiring); SS-04 contracts SessionStart routing semantics within this file |
| Stories | S-5.01 |
| Functional Requirement | FR-046 |
