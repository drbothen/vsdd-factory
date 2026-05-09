---
document_type: behavioral-contract
level: L3
version: "2.2"
status: draft
producer: product-owner
timestamp: 2026-04-28T00:00:00
phase: 1a
inputs:
  - .factory/stories/S-5.01-session-start-hook.md
  - .factory/specs/domain-spec/capabilities.md
input-hash: "5765182"
traces_to: .factory/specs/prd.md#FR-046
origin: greenfield
extracted_from: null
subsystem: "SS-04"
capability: "CAP-002"
lifecycle_status: active
introduced: v1.0.0-rc.1
modified: [v1.0-pass-1, v1.0-pass-2, v1.0-pass-3, v1.0-pass-5, v1.0-pass-2-S5.02-cross-story, v2.0-async-semantics-F2-2026-05-07]
last_amended: "2026-05-09 (v2.2 — F5 fix-burst-35: F-P36-001 Traceability Stories S-5.01→S-5.01, S-15.01)"
deprecated: null
deprecated_by: null
replacement: null
retired: null
removed: null
removal_reason: null
---

# BC-4.04.004: hooks.json.template registers SessionStart event with `command` field routing to dispatcher binary; once:true and synchronous envelope (async:true removed per ADR-019)

## Description

The shipped `plugins/vsdd-factory/hooks/hooks.json.template` must contain a `SessionStart` entry in its `hooks` object whose `command` field routes to the **dispatcher binary** (`${CLAUDE_PLUGIN_ROOT}/hooks/dispatcher/bin/{{PLATFORM}}/factory-dispatcher{{EXE_SUFFIX}}`), not directly to a WASM plugin filename. This is Layer 1 of the dual-routing-tables pattern per ADR-011: `hooks.json.template` is the Claude Code harness wiring document and knows only about the dispatcher binary. The dispatcher, once invoked, reads `hooks-registry.toml` (BC-4.04.005 — Layer 2) to route the SessionStart event to `session-start-telemetry.wasm`. The `once: true` flag is set on the SessionStart hook entry. Per ADR-019, `async: true` is REMOVED from this entry — the Claude Code envelope is uniformly synchronous for all event types. The template uses an array-of-objects schema: `hooks.SessionStart` is an array; each element has a nested `hooks` array whose entries have `type = "command"` and `command` pointing to the dispatcher binary.

## Preconditions

1. `plugins/vsdd-factory/hooks/hooks.json.template` is the Claude Code harness wiring source of truth (per ADR-011 dual-hook-routing-tables; Layer 1 of the two-layer routing pattern).
2. The dispatcher binary is present in the platform-specific path at `${CLAUDE_PLUGIN_ROOT}/hooks/dispatcher/bin/{{PLATFORM}}/factory-dispatcher{{EXE_SUFFIX}}` at activation time.

## Postconditions

1. `hooks.json.template` contains a `SessionStart` key in its top-level `hooks` object.
2. The `SessionStart` entry's `command` field in the nested `hooks` array references the dispatcher binary path: `${CLAUDE_PLUGIN_ROOT}/hooks/dispatcher/bin/{{PLATFORM}}/factory-dispatcher{{EXE_SUFFIX}}`. It does NOT reference `session-start-telemetry.wasm` or any other WASM plugin filename (per ADR-011 layer separation).
3. The `SessionStart` hook entry has `once: true`. The `async` key is ABSENT from the entry (per ADR-019: all hook envelopes are synchronous; async classification belongs in `hooks-registry.toml`, not `hooks.json`). The `session-start-telemetry` plugin is classified `async = true` in `hooks-registry.toml` (BC-7.06.001 Invariant 6), enabling it to run fire-and-forget at the dispatcher level while the envelope remains synchronous.
4. The entry follows the array-of-objects schema: `template["hooks"]["SessionStart"]` is an array; each element is an object with a nested `hooks` array; each nested entry has `type = "command"` and `command` = dispatcher binary path.
5. The `SessionStart` hook entry has `timeout: 10000`. This is the Claude Code harness timeout (ms) and bounds the entire dispatcher process invocation including subprocess wait.

## Invariants

1. The `SessionStart` entry in `hooks.json.template` MUST NEVER reference WASM plugin filenames (e.g., `session-start-telemetry.wasm`) — per ADR-011 layer separation, WASM plugin references belong exclusively in `hooks-registry.toml` (BC-4.04.005).
2. The `SessionStart` entry must remain present in `hooks.json.template` through all v1.0 releases — removal requires a deprecation pass.
3. The harness `timeout` (10000ms) MUST be ≥ the dispatcher per-call budget (`timeout_ms = 8000` in BC-4.04.005) per ADR-011 timeout-hierarchy invariant: subprocess timeout (5000ms, BC-4.04.002 Invariant 4) < dispatcher per-call budget (8000ms, BC-4.04.005 Postcondition 5) < harness timeout (10000ms). Lowering the harness timeout below 10000 breaks this hierarchy and causes Claude Code to kill the dispatcher process before the dispatcher's own timeout fires, preventing the fail-open `factory_health = "unknown"` path from completing.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Dispatcher binary path platform substitution fails (e.g., `{{PLATFORM}}` placeholder not substituted during activation) | Activation system falls back to the previous generated `hooks.json` or reports activation error; the unsubstituted template path is never written to the live `hooks.json` |
| EC-002 | `hooks.json.template` is missing the `SessionStart` key entirely | No `SessionStart` routing exists in the Claude Code harness; the dispatcher is never invoked for SessionStart events; `session.started` events are never emitted |
| EC-003 | Per-platform variant file (`hooks.json.darwin-arm64`, `hooks.json.darwin-x64`, `hooks.json.linux-arm64`, `hooks.json.linux-x64`, `hooks.json.windows-x64`) is out of sync with `hooks.json.template` | Activation regeneration required; the platform variant is the committed artifact used by the activation skill (hooks.json is gitignored) |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Parse `hooks.json.template`; inspect `hooks.SessionStart[0].hooks[0]` | `command` field contains `factory-dispatcher`; `once: true` present; **`async` key ABSENT** (per ADR-019); no reference to `.wasm` filename in `command` field | happy-path |
| Parse `hooks.json.template`; inspect `hooks.SessionStart[0].hooks[0].command` | Value matches pattern `${CLAUDE_PLUGIN_ROOT}/hooks/dispatcher/bin/{{PLATFORM}}/factory-dispatcher{{EXE_SUFFIX}}` (placeholder-unsubstituted form) | happy-path (dispatcher binary routing) |
| Parse `hooks.json.template`; inspect `hooks.SessionStart[0].hooks[0].timeout` | Value equals `10000` (harness timeout ms; must exceed dispatcher per-call budget of 8000ms per ADR-011 timeout hierarchy) | happy-path (timeout hierarchy) |
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
| Stories | S-5.01, S-15.01 |
| Functional Requirement | FR-046 |

## Amendment 2026-05-09 (v2.1 → v2.2 — F5 fix-burst-35 F-P36-001: Traceability Stories S-5.01→S-5.01, S-15.01)

**F-P36-001 (BC body vs BC-INDEX Stories drift):** Traceability `Stories` row updated from `S-5.01` to `S-5.01, S-15.01`. BC-INDEX row (v1.28) already listed S-5.01, S-15.01; source body was missing S-15.01 (pre-F3). F3 story decomposition (PR #106 merged 2026-05-07) is canonical.

## Amendment 2026-05-07 (v2.1 — F2 pass-2 fix burst)

Addresses adversary pass-2 finding F-P2-003.

**F-P2-003 (Stale cross-reference)**: Postcondition 3 cited "BC-7.06.001 Postcondition 7" for the `session-start-telemetry` async classification. During pass-1, the classification table was promoted from Postcondition 7 to Invariant 6. Updated to "BC-7.06.001 Invariant 6".

## Amendment 2026-05-07 (v2.0 — F2 pass-1 fix burst)

**Cycle:** v1.0-feature-plugin-async-semantics-pass-1 (F2). **ADR:** ADR-019.

**Delta:** `async: true` is removed from the `SessionStart` hook entry in `hooks.json.template`. Per ADR-019, all Claude Code hook envelopes are now uniformly synchronous. The previous `async: true` on SessionStart meant the dispatcher was invoked fire-and-forget, making any block verdict from session-start-telemetry invisible to Claude Code. Removing it makes the envelope synchronous; per-plugin async classification is now in `hooks-registry.toml` (BC-7.06.001), where `session-start-telemetry` is correctly classified as `async = true` in the dispatcher's partition model.

**User decision:** "Every Claude Code hook event must be sync at the envelope. No per-event carve-outs." This overrides F1's hedged framing that SessionStart/SessionEnd could remain async.

**Changes:**
- H1 title updated to reflect sync envelope.
- Description paragraph updated: `async: true` → removed; rationale for ADR-019 move added.
- Postcondition 3 updated: `once: true` retained; `async` key is now ABSENT.
- Canonical Test Vector updated: `async: true` present → `async` key ABSENT.
- Per-platform variant files (`hooks.json.darwin-arm64`, etc.) must be regenerated from updated template.
