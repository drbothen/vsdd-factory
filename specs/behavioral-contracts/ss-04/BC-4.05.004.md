---
document_type: behavioral-contract
level: L3
version: "2.1"
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
modified: [v1.0-pass-1, v1.0-pass-2, v2.0-async-semantics-F2-2026-05-07]
last_amended: "2026-05-07 (v1.0-feature-plugin-async-semantics-pass-1 cycle F2; see ADR-019)"
deprecated: null
deprecated_by: null
replacement: null
retired: null
removed: null
removal_reason: null
---

# BC-4.05.004: hooks.json.template registers SessionEnd event with `command` field routing to dispatcher binary; once:true and synchronous envelope (async:true removed per ADR-019)

## Description

The shipped `plugins/vsdd-factory/hooks/hooks.json.template` must contain a `SessionEnd` entry in its `hooks` object whose `command` field routes to the **dispatcher binary** (`${CLAUDE_PLUGIN_ROOT}/hooks/dispatcher/bin/{{PLATFORM}}/factory-dispatcher{{EXE_SUFFIX}}`), not directly to a WASM plugin filename. This is Layer 1 of the dual-routing-tables pattern per ADR-011: `hooks.json.template` is the Claude Code harness wiring document and knows only about the dispatcher binary. The dispatcher, once invoked, reads `hooks-registry.toml` (BC-4.05.005 — Layer 2) to route the `SessionEnd` event to `session-end-telemetry.wasm`. The `once: true` flag is set on the `SessionEnd` hook entry. Per ADR-019, `async: true` is REMOVED — the Claude Code envelope is uniformly synchronous for all event types. This mirrors BC-4.04.004 (amended), applied to `SessionEnd`.

## Preconditions

1. `plugins/vsdd-factory/hooks/hooks.json.template` is the Claude Code harness wiring source of truth (per ADR-011 dual-hook-routing-tables; Layer 1 of the two-layer routing pattern).
2. The dispatcher binary is present in the platform-specific path at `${CLAUDE_PLUGIN_ROOT}/hooks/dispatcher/bin/{{PLATFORM}}/factory-dispatcher{{EXE_SUFFIX}}` at activation time.

## Postconditions

1. `hooks.json.template` contains a `SessionEnd` key in its top-level `hooks` object.
2. The `SessionEnd` entry's `command` field in the nested `hooks` array references the dispatcher binary path: `${CLAUDE_PLUGIN_ROOT}/hooks/dispatcher/bin/{{PLATFORM}}/factory-dispatcher{{EXE_SUFFIX}}`. It does NOT reference `session-end-telemetry.wasm` or any other WASM plugin filename (per ADR-011 layer separation).
3. The `SessionEnd` hook entry has `once: true`. The `async` key is ABSENT from the entry (per ADR-019: all hook envelopes are synchronous; async classification belongs in `hooks-registry.toml`). The `session-end-telemetry` plugin is classified `async = true` in `hooks-registry.toml` (BC-7.06.001 Invariant 6).
4. The entry follows the array-of-objects schema: `template["hooks"]["SessionEnd"]` is an array; each element is an object with a nested `hooks` array; each nested entry has `type = "command"` and `command` = dispatcher binary path.
5. The `SessionEnd` hook entry has `timeout: 10000`. This is the Claude Code harness timeout (ms). For `SessionEnd`, the dispatcher timeout is 5000ms (BC-4.05.005 Postcondition 4); the harness timeout of 10000ms provides adequate headroom above the dispatcher budget, preserving the timeout hierarchy: dispatcher timeout (5000ms) < harness timeout (10000ms).

## Invariants

1. The `SessionEnd` entry in `hooks.json.template` MUST NEVER reference WASM plugin filenames (e.g., `session-end-telemetry.wasm`) — per ADR-011 layer separation, WASM plugin references belong exclusively in `hooks-registry.toml` (BC-4.05.005).
2. The `SessionEnd` entry must remain present in `hooks.json.template` through all v1.0 releases — removal requires a deprecation pass.
3. The harness `timeout` (10000ms) MUST be ≥ the dispatcher per-call budget (`timeout_ms = 5000` in BC-4.05.005) per ADR-011 timeout-hierarchy invariant. For `SessionEnd` (no subprocess), the hierarchy is simpler: dispatcher timeout (5000ms) < harness timeout (10000ms).

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Dispatcher binary path platform substitution fails (e.g., `{{PLATFORM}}` placeholder not substituted during activation) | Activation system falls back to the previous generated `hooks.json` or reports activation error; the unsubstituted template path is never written to the live `hooks.json` |
| EC-002 | `hooks.json.template` is missing the `SessionEnd` key entirely | No `SessionEnd` routing exists in the Claude Code harness; the dispatcher is never invoked for SessionEnd events; `session.ended` events are never emitted |
| EC-003 | Per-platform variant file (`hooks.json.darwin-arm64`, `hooks.json.darwin-x64`, `hooks.json.linux-arm64`, `hooks.json.linux-x64`, `hooks.json.windows-x64`) is out of sync with `hooks.json.template` | Activation regeneration required; the platform variant is the committed artifact used by the activation skill (hooks.json is gitignored) |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Parse `hooks.json.template`; inspect `hooks.SessionEnd[0].hooks[0]` | `command` field contains `factory-dispatcher`; `once: true` present; **`async` key ABSENT** (per ADR-019); no reference to `.wasm` filename in `command` field | happy-path |
| Parse `hooks.json.template`; inspect `hooks.SessionEnd[0].hooks[0].command` | Value matches pattern `${CLAUDE_PLUGIN_ROOT}/hooks/dispatcher/bin/{{PLATFORM}}/factory-dispatcher{{EXE_SUFFIX}}` (placeholder-unsubstituted form) | happy-path (dispatcher binary routing) |
| Parse `hooks.json.template`; inspect `hooks.SessionEnd[0].hooks[0].timeout` | Value equals `10000` (harness timeout ms; must exceed dispatcher per-call budget of 5000ms per ADR-011 timeout hierarchy) | happy-path (timeout hierarchy) |
| `hooks.json.template` is missing `SessionEnd` key | No SessionEnd entry in `template["hooks"]`; dispatcher never invoked for SessionEnd events | error (missing key) |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-066 | Session-End Plugin Surface Invariant — All BC-4.05.* Postconditions Hold in Integration Test | integration |

## Related BCs

- **BC-4.05.001** — enables (this registration is the trigger that causes BC-4.05.001 to execute)
- **BC-4.05.003** — enables (this registration provides the Layer 1 `once: true` directive that is the source of BC-4.05.003's idempotency guarantee)
- **BC-4.05.005** — counterpart (this is Layer 1; BC-4.05.005 is Layer 2; both must exist for full routing)
- **BC-4.04.004** — structural analog (SessionStart Layer 1 registration; BC-4.05.004 mirrors it for SessionEnd)
- **BC-1.02.005** — depends on (dispatcher envelope parsing handles the `SessionEnd` event type routed via this entry)

## Architecture Anchors

- SS-07 — `plugins/vsdd-factory/hooks/hooks.json.template` (harness wiring source of truth; owned by SS-07 Hook Bash Layer per ADR-011)
- SS-04 — behavioral contract for the SessionEnd routing semantics within `hooks.json.template` (routing layer ownership per F-8 ruling applied to SessionEnd)
- SS-09 — activation skill generates the live `hooks.json` (and per-platform variants) from `hooks.json.template` at activation time; the template itself is committed and not gitignored

## Story Anchor

S-5.02

## VP Anchors

VP-066

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-002 |
| Capability Anchor Justification | CAP-002 ("Hook Claude Code tool calls with sandboxed WASM plugins") per capabilities.md §CAP-002 |
| L2 Domain Invariants | DI-015 (per-project activation required — hooks.json.template is the activation surface that must be present for dispatcher invocation) |
| Architecture Module | SS-07 — `plugins/vsdd-factory/hooks/hooks.json.template` (harness wiring); SS-04 contracts SessionEnd routing semantics within this file |
| Stories | S-5.02 |
| Functional Requirement | FR-046 |

## Amendment 2026-05-07 (v2.1 — F2 pass-2 fix burst)

Addresses adversary pass-2 finding F-P2-003.

**F-P2-003 (Stale cross-reference)**: Postcondition 3 cited "BC-7.06.001 Postcondition 7" for the `session-end-telemetry` async classification. During pass-1, the classification table was promoted from Postcondition 7 to Invariant 6. Updated to "BC-7.06.001 Invariant 6".

## Amendment 2026-05-07 (v2.0 — F2 pass-1 fix burst)

**Cycle:** v1.0-feature-plugin-async-semantics-pass-1 (F2). **ADR:** ADR-019.

**Delta:** `async: true` removed from the `SessionEnd` hook entry in `hooks.json.template`. Mirrors BC-4.04.004 Amendment 2026-05-07 (SessionStart). Per ADR-019, all Claude Code hook envelopes are uniformly synchronous; async classification for `session-end-telemetry` lives in `hooks-registry.toml` (BC-7.06.001).

**User decision:** "Every Claude Code hook event must be sync at the envelope. No per-event carve-outs." SessionEnd is not exempt.

**Changes:** H1 title updated; Description updated; Postcondition 3 updated (`async` key absent); Canonical Test Vector updated.
