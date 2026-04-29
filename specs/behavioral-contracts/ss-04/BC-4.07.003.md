---
document_type: behavioral-contract
level: L3
version: "1.0"
status: draft
producer: product-owner
timestamp: 2026-04-28T00:00:00
phase: 1a
inputs:
  - .factory/stories/S-5.03-worktree-hooks.md
  - .factory/specs/domain-spec/capabilities.md
input-hash: "0b97a0a"
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

# BC-4.07.003: hooks.json.template registers WorktreeCreate and WorktreeRemove events with `command` field routing to dispatcher binary; once:false (can re-fire); async:true; timeout:10000

## Description

The shipped `plugins/vsdd-factory/hooks/hooks.json.template` must contain both a `WorktreeCreate` entry and a `WorktreeRemove` entry in its `hooks` object. Each entry's `command` field routes to the **dispatcher binary** (`${CLAUDE_PLUGIN_ROOT}/hooks/dispatcher/bin/{{PLATFORM}}/factory-dispatcher{{EXE_SUFFIX}}`), not directly to a WASM plugin filename. This is Layer 1 of the dual-routing-tables pattern per ADR-011. Critically, worktree events MUST NOT carry `once: true` — worktree events can re-fire on Claude Code reconnect after disconnect, and once-per-session suppression would silently drop legitimate events. This is the key behavioral difference from SessionStart/SessionEnd (which use `once: true`). Both entries have `async: true` and `timeout: 10000`. Per-platform variants (`hooks.json.darwin-arm64`, etc.) must be kept in sync with the template (EC-003).

## Preconditions

1. `plugins/vsdd-factory/hooks/hooks.json.template` is the Claude Code harness wiring source of truth (per ADR-011 dual-hook-routing-tables; Layer 1 of the two-layer routing pattern).
2. The dispatcher binary is present in the platform-specific path at `${CLAUDE_PLUGIN_ROOT}/hooks/dispatcher/bin/{{PLATFORM}}/factory-dispatcher{{EXE_SUFFIX}}` at activation time.

## Postconditions

1. `hooks.json.template` contains a `WorktreeCreate` key in its top-level `hooks` object.
2. `hooks.json.template` contains a `WorktreeRemove` key in its top-level `hooks` object.
3. Each entry's `command` field references the dispatcher binary path: `${CLAUDE_PLUGIN_ROOT}/hooks/dispatcher/bin/{{PLATFORM}}/factory-dispatcher{{EXE_SUFFIX}}`. Neither entry references `worktree-hooks.wasm` or any WASM plugin filename directly (per ADR-011 layer separation).
4. Neither `WorktreeCreate` nor `WorktreeRemove` hook entries carry `once: true`. Worktree events can re-fire (e.g., on Claude Code reconnect after disconnect). Omitting `once` is equivalent to `once: false` under Claude Code's default semantics. This is a deliberate departure from the SessionStart/SessionEnd pattern.
5. Both `WorktreeCreate` and `WorktreeRemove` hook entries have `async: true`.
6. Both entries have `timeout: 10000`. This is the Claude Code harness timeout (ms). The dispatcher's per-call budget is `timeout_ms = 5000` (BC-4.07.004 Postcondition 3), preserving the timeout hierarchy: dispatcher timeout (5000ms) < harness timeout (10000ms).
7. Each entry follows the array-of-objects schema: `template["hooks"]["WorktreeCreate"]` is an array; each element is an object with a nested `hooks` array; each nested entry has `type = "command"` and `command` = dispatcher binary path. Same for `WorktreeRemove`.

## Invariants

1. Neither `WorktreeCreate` nor `WorktreeRemove` entries in `hooks.json.template` MUST NEVER carry `once: true`. If `once: true` were added, Claude Code would suppress re-fires after the first occurrence per session — silently dropping `WorktreeCreate` events on reconnect and making it appear that new worktrees were not created. This is the primary behavioral difference from session lifecycle events.
2. Neither entry must ever reference WASM plugin filenames — per ADR-011 layer separation, WASM plugin references belong exclusively in `hooks-registry.toml` (BC-4.07.004).
3. Both `WorktreeCreate` and `WorktreeRemove` entries must remain present in `hooks.json.template` through all v1.0 releases — removal requires a deprecation pass.
4. The harness `timeout` (10000ms) MUST be ≥ the dispatcher per-call budget (`timeout_ms = 5000` in BC-4.07.004) per ADR-011 timeout-hierarchy invariant.
5. Per-platform variant files must be regenerated from `hooks.json.template` whenever the template changes (EC-003).

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Dispatcher binary path platform substitution fails (placeholder not substituted during activation) | Activation system falls back or reports activation error; unsubstituted template path never written to live `hooks.json` |
| EC-002 | `hooks.json.template` is missing the `WorktreeCreate` or `WorktreeRemove` key entirely | No routing exists in the Claude Code harness for the missing event; dispatcher never invoked; corresponding `worktree.*` events never emitted |
| EC-003 | Per-platform variant file (`hooks.json.darwin-arm64`, `hooks.json.darwin-x64`, `hooks.json.linux-arm64`, `hooks.json.linux-x64`, `hooks.json.windows-x64`) is out of sync with `hooks.json.template` | Activation regeneration required; the platform variant is the committed artifact used by the activation skill |
| EC-004 | `once: true` accidentally added to the `WorktreeCreate` entry | Claude Code suppresses re-fires after first WorktreeCreate per session; subsequent `WorktreeCreate` events (e.g., from reconnect) are silently dropped; `worktree.created` events missed; VP-067 `test_bc_4_07_003_hooks_json_template_no_once_true` assertion fails at test time |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Parse `hooks.json.template`; inspect `hooks.WorktreeCreate[0].hooks[0]` | `command` field contains `factory-dispatcher`; NO `once` key present (or `once: false`); `async: true` present; no reference to `.wasm` filename in `command` field | happy-path (WorktreeCreate) |
| Parse `hooks.json.template`; inspect `hooks.WorktreeRemove[0].hooks[0]` | `command` field contains `factory-dispatcher`; NO `once` key present (or `once: false`); `async: true` present; no reference to `.wasm` filename in `command` field | happy-path (WorktreeRemove) |
| Parse `hooks.json.template`; inspect `hooks.WorktreeCreate[0].hooks[0].timeout` | Value equals `10000` (harness timeout ms) | happy-path (timeout hierarchy) |
| `hooks.json.template` is missing `WorktreeCreate` key | No WorktreeCreate entry in `template["hooks"]`; dispatcher never invoked for WorktreeCreate events | error (missing key) |
| `hooks.json.template` has `once: true` on `WorktreeCreate` entry | VP-067 assertion `once != true` fails; worktree re-fire semantics violated | error (once:true) |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-067 | Worktree Hook Plugin Surface Invariant — All BC-4.07.* Postconditions Hold in Integration Test | integration |

## Related BCs

- **BC-4.07.001** — enables (this WorktreeCreate registration is the trigger that causes BC-4.07.001 to execute)
- **BC-4.07.002** — enables (this WorktreeRemove registration is the trigger that causes BC-4.07.002 to execute)
- **BC-4.07.004** — counterpart (this is Layer 1; BC-4.07.004 is Layer 2; both must exist for full routing of both worktree events)
- **BC-4.05.004** — structural analog (SessionEnd Layer 1 registration; BC-4.07.003 differs in `once: false` vs. `once: true` — this is the critical behavioral distinction)
- **BC-4.04.004** — structural analog (SessionStart Layer 1 registration; same `once: true` vs. `once: false` distinction applies)
- **BC-1.02.005** — depends on (dispatcher envelope parsing handles both `WorktreeCreate` and `WorktreeRemove` event types routed via these entries)

## Architecture Anchors

- SS-07 — `plugins/vsdd-factory/hooks/hooks.json.template` (harness wiring source of truth; owned by SS-07 Hook Bash Layer per ADR-011)
- SS-04 — behavioral contract for WorktreeCreate/WorktreeRemove routing semantics within `hooks.json.template` (routing layer ownership per F-8 ruling applied to worktree events)
- SS-09 — activation skill generates the live `hooks.json` (and per-platform variants) from `hooks.json.template` at activation time

## Story Anchor

S-5.03

## VP Anchors

VP-067

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-002 |
| Capability Anchor Justification | CAP-002 ("Hook Claude Code tool calls with sandboxed WASM plugins") per capabilities.md §CAP-002 |
| L2 Domain Invariants | DI-015 (per-project activation required — hooks.json.template is the activation surface that must be present for dispatcher invocation; both WorktreeCreate and WorktreeRemove entries must be present) |
| Architecture Module | SS-07 — `plugins/vsdd-factory/hooks/hooks.json.template` (harness wiring); SS-04 contracts WorktreeCreate/WorktreeRemove routing semantics within this file |
| Stories | S-5.03 |
| Functional Requirement | FR-046 |
