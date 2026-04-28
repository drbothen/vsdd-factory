---
document_type: behavioral-contract
level: L3
version: "1.0"
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
modified: [v1.0-pass-1]
deprecated: null
deprecated_by: null
replacement: null
retired: null
removed: null
removal_reason: null
---

# BC-4.05.002: session-end plugin emits without subprocess invocation; fast-path completion

## Description

Unlike the session-start-telemetry plugin (which invokes `factory-health --brief` via `exec_subprocess`), the `session-end-telemetry` plugin has NO subprocess dependencies. All data needed to populate the `session.ended` payload (`duration_ms`, `tool_call_count`) is read directly from the incoming envelope. The plugin completes its `emit_event` call synchronously without invoking `vsdd::exec_subprocess` at any point, and completes well within the 5000ms dispatcher timeout.

## Preconditions

1. A `SessionEnd` event has been routed to the session-end plugin (BC-4.05.004 + BC-4.05.005).
2. The `exec_subprocess` host function is available in the plugin's WASM runtime context (inherited from the dispatcher's host-fn table), but the plugin does NOT call it.

## Postconditions

1. The plugin does NOT invoke `vsdd::exec_subprocess` for any reason during `SessionEnd` handling.
2. The plugin completes its entire `on_hook` execution (envelope parse + `emit_event` call + return) within the dispatcher `timeout_ms = 5000` (per BC-4.05.005 Postcondition 4). Because no subprocess is involved, this completion time is bounded by WASM execution overhead and the `emit_event` channel write — typically well under 100ms.
3. The plugin returns `HookResult::Ok` (exit code 0) to the dispatcher.

## Invariants

1. `exec_subprocess` capability is NOT declared in BC-4.05.005 hooks-registry.toml entry. Deny-by-default per BC-1.05.022 (capability sandbox): undeclared capabilities are refused by the host fn dispatch table. If the plugin were to call `exec_subprocess` despite Invariant 1, it would receive `CAPABILITY_DENIED` from the host fn — not a runtime panic.
2. Plugin invocation latency for SessionEnd is bounded at dispatcher `timeout_ms = 5000`. No subprocess wait is included in this budget.
3. The 5000ms dispatcher timeout for SessionEnd is strictly less than the 10000ms Claude Code harness timeout (BC-4.05.004 Postcondition 5), preserving the timeout hierarchy: dispatcher timeout (5000ms) < harness timeout (10000ms).

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Plugin code contains a mistaken `exec_subprocess` call (developer error in implementation) | Host fn dispatch returns `CAPABILITY_DENIED` immediately (capability not declared per BC-4.05.005 Postcondition 5–6); plugin handles error gracefully and still emits `session.ended`; no crash |
| EC-002 | Plugin invocation exceeds 5000ms (e.g., pathological contention on the `emit_event` channel) | Dispatcher epoch interruption fires at `timeout_ms`; SessionEnd event is lost (acceptable v1.0 — subsequent SessionStart will repopulate session context); dispatcher logs timeout; returns non-zero to harness |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| `SessionEnd` envelope dispatched to session-end-telemetry.wasm; CountingMock wraps exec_subprocess host fn | `session.ended` emitted once; `exec_subprocess` CountingMock invocation count == 0 (plugin never calls it); plugin returns `HookResult::Ok` | happy-path (no subprocess) |
| `SessionEnd` envelope dispatched; plugin invocation measured against 5000ms budget | Plugin returns before 5000ms wall-clock elapsed; `session.ended` present in file sink | happy-path (fast-path timing) |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-066 | Session-End Plugin Surface Invariant — All BC-4.05.* Postconditions Hold in Integration Test | integration |

## Related BCs

- **BC-4.05.001** — composes with (the fast-path emission this BC specifies produces the `session.ended` event defined in BC-4.05.001)
- **BC-4.05.005** — depends on (hooks-registry.toml entry omits `exec_subprocess` capability table; absence is the deny-by-default gate)
- **BC-4.04.002** — structural contrast (SessionStart analog that DOES invoke subprocess; BC-4.05.002 deliberately omits that step)

## Architecture Anchors

- SS-04 — `crates/hook-plugins/session-end-telemetry/src/lib.rs` (plugin `on_hook` body — no `exec_subprocess` call present)

## Story Anchor

S-5.02

## VP Anchors

VP-066

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-002 |
| Capability Anchor Justification | CAP-002 ("Hook Claude Code tool calls with sandboxed WASM plugins") per capabilities.md §CAP-002 |
| L2 Domain Invariants | DI-004 (capability denial emits audit event — if exec_subprocess were attempted without capability declaration, CAPABILITY_DENIED is returned; no audit event needed here since the capability is never declared and the plugin never calls it); DI-011 (sink submit must not block — no subprocess wait in SessionEnd path means the only latency source is the emit_event channel write, well within the 5000ms budget) |
| Architecture Module | SS-04 — `crates/hook-plugins/session-end-telemetry/src/lib.rs` |
| Stories | S-5.02 |
| Functional Requirement | FR-046 |
