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

# BC-4.08.002: hooks.json.template registers PostToolUseFailure with `command` routing to dispatcher binary; once key ABSENT (fires per-failure); async:true; timeout:10000

## Description

The shipped `plugins/vsdd-factory/hooks/hooks.json.template` must contain a `PostToolUseFailure` entry in its `hooks` object. The entry's `command` field routes to the **dispatcher binary** (`${CLAUDE_PLUGIN_ROOT}/hooks/dispatcher/bin/{{PLATFORM}}/factory-dispatcher{{EXE_SUFFIX}}`), not directly to a WASM plugin filename. This is Layer 1 of the dual-routing-tables pattern per ADR-011. Critically, PostToolUseFailure MUST NOT carry `once: true` — PostToolUseFailure fires once per tool failure event from Claude Code, and suppression would silently drop legitimate tool error events. The `once` key MUST be absent (defensive omission, same pattern as WorktreeCreate/WorktreeRemove per BC-4.07.003). The entry has `async: true` and `timeout: 10000`. Per-platform variants (`hooks.json.darwin-arm64`, `hooks.json.darwin-x64`, `hooks.json.linux-arm64`, `hooks.json.linux-x64`, `hooks.json.windows-x64`) must be regenerated from the template via `scripts/generate-hooks-json.sh` after every template change (EC-003).

## Preconditions

1. `plugins/vsdd-factory/hooks/hooks.json.template` is the Claude Code harness wiring source of truth (per ADR-011 dual-hook-routing-tables; Layer 1 of the two-layer routing pattern).
2. The dispatcher binary is present in the platform-specific path at `${CLAUDE_PLUGIN_ROOT}/hooks/dispatcher/bin/{{PLATFORM}}/factory-dispatcher{{EXE_SUFFIX}}` at activation time.

## Postconditions

1. `hooks.json.template` contains a `PostToolUseFailure` key in its top-level `hooks` object.
2. The entry's `command` field references the dispatcher binary path: `${CLAUDE_PLUGIN_ROOT}/hooks/dispatcher/bin/{{PLATFORM}}/factory-dispatcher{{EXE_SUFFIX}}`. The entry does NOT reference `tool-failure-hooks.wasm` or any WASM plugin filename directly (per ADR-011 layer separation).
3. The `PostToolUseFailure` hook entry does NOT carry a `once` key at all. **The `once` key MUST be absent** (not `once: false`, not `once: true`, not `once: "false"` — key must not exist). PostToolUseFailure fires per-failure from Claude Code; the `once` key is inapplicable. Claude Code's default semantics treat absence as re-firable (fires for each occurrence); explicit `false` semantics are unspecified — defensive omission protects against future Claude Code parser changes. This is the same pattern as WorktreeCreate/WorktreeRemove (BC-4.07.003), and differs from SessionStart/SessionEnd (which use `once: true`).
4. The `PostToolUseFailure` hook entry has `async: true`.
5. The entry has `timeout: 10000`. This is the Claude Code harness timeout (ms). The dispatcher's per-call budget is `timeout_ms = 5000` (BC-4.08.003 Postcondition 3), preserving the timeout hierarchy: dispatcher timeout (5000ms) < harness timeout (10000ms).
6. The entry follows the array-of-objects schema: `template["hooks"]["PostToolUseFailure"]` is an array; each element is an object with a nested `hooks` array; each nested entry has `type = "command"` and `command` = dispatcher binary path.

## Invariants

1. The `PostToolUseFailure` entry in `hooks.json.template` MUST NEVER carry any `once` key (including `once: true`, `once: false`, or `once: "false"`). **The `once` key MUST be completely absent.** PostToolUseFailure fires per-failure; `once: true` would suppress all but the first failure per session — silently dropping subsequent error events. Defensive omission (key absent) is the canonical pattern for per-occurrence events.
2. The entry must never reference WASM plugin filenames — per ADR-011 layer separation, WASM plugin references belong exclusively in `hooks-registry.toml` (BC-4.08.003).
3. The `PostToolUseFailure` entry must remain present in `hooks.json.template` through all v1.0 releases — removal requires a deprecation pass.
4. The harness `timeout` (10000ms) MUST be ≥ the dispatcher per-call budget (`timeout_ms = 5000` in BC-4.08.003) per ADR-011 timeout-hierarchy invariant.
5. Per-platform variant files must be regenerated from `hooks.json.template` via `scripts/generate-hooks-json.sh` whenever the template changes (EC-003). Five variants must be kept in sync: `hooks.json.darwin-arm64`, `hooks.json.darwin-x64`, `hooks.json.linux-arm64`, `hooks.json.linux-x64`, `hooks.json.windows-x64`. Failure to regenerate causes activation failures on non-regenerated platforms.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Dispatcher binary path platform substitution fails (placeholder not substituted during activation) | Activation system falls back or reports activation error; unsubstituted template path never written to live `hooks.json` |
| EC-002 | `hooks.json.template` is missing the `PostToolUseFailure` key entirely | No routing exists in the Claude Code harness for PostToolUseFailure events; dispatcher never invoked; `tool.error` events never emitted; tool failure observability is dark |
| EC-003 | Per-platform variant file is out of sync with `hooks.json.template` after template edit | `scripts/generate-hooks-json.sh` must be run to regenerate all 5 platform variants. The platform variant is the committed artifact used by the activation skill. Skipping regeneration causes the affected platform to use stale routing. (S-5.03 PR-cycle-1 lesson — always regenerate all 5 variants after template edit.) |
| EC-004 | `once: true` accidentally added to the `PostToolUseFailure` entry | Claude Code fires the hook only for the first PostToolUseFailure per session; subsequent failures are silently dropped; `tool.error` events are missed; VP-068 `test_bc_4_08_002_hooks_json_template_no_once_key` assertion fails at test time |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Parse `hooks.json.template`; inspect `hooks.PostToolUseFailure[0].hooks[0]` | `command` field contains `factory-dispatcher`; `once` key MUST BE ABSENT (not `false`, not any value — key must not exist); `async: true` present; no reference to `.wasm` filename in `command` field | happy-path |
| Parse `hooks.json.template`; inspect `hooks.PostToolUseFailure[0].hooks[0].timeout` | Value equals `10000` (harness timeout ms) | happy-path (timeout hierarchy) |
| `hooks.json.template` is missing `PostToolUseFailure` key | No PostToolUseFailure entry in `template["hooks"]`; dispatcher never invoked for PostToolUseFailure events | error (missing key) |
| `hooks.json.template` has `once: true` on `PostToolUseFailure` entry | VP-068 assertion `once key absent` fails; per-failure semantics violated | error (once:true) |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-068 | Tool-Failure Hook Plugin Surface Invariant — All BC-4.08.* Postconditions Hold in Integration Test | integration |

## Related BCs

- **BC-4.08.001** — enables (this PostToolUseFailure registration is the trigger that causes BC-4.08.001 to execute)
- **BC-4.08.003** — counterpart (this is Layer 1; BC-4.08.003 is Layer 2; both must exist for full routing of PostToolUseFailure)
- **BC-4.07.003** — structural analog (WorktreeCreate/WorktreeRemove Layer 1 registration; BC-4.08.002 uses the same `once` key ABSENT pattern — per-occurrence events do not suppress)
- **BC-4.05.004** — structural contrast (SessionEnd Layer 1 registration; SessionEnd uses `once: true`; PostToolUseFailure omits `once` entirely — key distinction)
- **BC-4.04.004** — structural contrast (SessionStart Layer 1 registration; SessionStart uses `once: true`; PostToolUseFailure omits `once` entirely)
- **BC-1.02.005** — depends on (dispatcher envelope parsing handles `PostToolUseFailure` event type routed via this entry)

## Architecture Anchors

- SS-07 — `plugins/vsdd-factory/hooks/hooks.json.template` (harness wiring source of truth; owned by SS-07 Hook Bash Layer per ADR-011)
- SS-04 — behavioral contract for PostToolUseFailure routing semantics within `hooks.json.template`
- SS-09 — activation skill generates the live `hooks.json` (and per-platform variants) from `hooks.json.template` at activation time

## Story Anchor

S-5.04

## VP Anchors

VP-068

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-013 |
| Capability Anchor Justification | CAP-013 ("Capture post-execution activity (PostToolUse hooks)") per capabilities.md §CAP-013. The hooks.json.template registration is the Claude Code harness wiring that enables PostToolUse failure capture; CAP-013 explicitly covers "tool errors for audit and observability purposes". |
| L2 Domain Invariants | DI-015 (per-project activation required — hooks.json.template is the activation surface that must be present for dispatcher invocation; PostToolUseFailure entry must be present) |
| Architecture Module | SS-07 — `plugins/vsdd-factory/hooks/hooks.json.template` (harness wiring); SS-04 contracts PostToolUseFailure routing semantics within this file |
| Stories | S-5.04 |
| Functional Requirement | FR-046 |

## Changelog

| Version | Date | Author | Change |
|---------|------|--------|--------|
| v1.0 | 2026-04-28 | product-owner | Initial creation (S-5.04 foundation burst). Promoted from v1.1 BC candidate BC-4.08.002 in legacy story. `once` key ABSENT (not `once: false` — defensive omission per BC-4.07.003 pattern). Platform variant regeneration requirement (EC-003) explicitly documented per S-5.03 PR-cycle-1 lesson. |
