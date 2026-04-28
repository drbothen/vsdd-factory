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
modified: [v1.0-pass-1, v1.0-pass-2]
deprecated: null
deprecated_by: null
replacement: null
retired: null
removed: null
removal_reason: null
---

# BC-4.04.003: session-start plugin is idempotent on duplicate SessionStart events within the same session_id

## Description

SessionStart idempotency is enforced upstream at Layer 1 by Claude Code's `once: true` directive in `hooks.json.template` (per BC-4.04.004 invariant 1). The Claude Code harness fires `SessionStart` to the dispatcher exactly once per session; the dispatcher receives at most one invocation per session.

The session-start-telemetry plugin is unconditionally stateless across invocations: it does NOT maintain dedup state (no `Mutex<HashSet<String>>` or equivalent). It emits `session.started` on every invocation. If Layer 1 once-discipline is bypassed (e.g., manual `SessionStart` event injection during testing or a harness regression), the plugin emits multiple `session.started` events — this is operator-observable and acceptable at the plugin layer. The plugin does not attempt to defend against a Layer 1 failure.

Pass-1 specified plugin-side `Mutex<HashSet<String>>` (formerly Invariant 3). Pass-2 superseded that with dispatcher-side dedup per BC-1.10.002. Pass-4 retired BC-1.10.002 as over-engineering and reverted idempotency to Layer 1 delegation (this contract).

## Preconditions

1. Claude Code's `once: true` directive in `hooks.json.template` (BC-4.04.004 invariant 1) is in effect — the harness fires `SessionStart` to the dispatcher at most once per session.
2. A `SessionStart` event arrives at the dispatcher.

## Postconditions

1. The plugin emits exactly one `session.started` event per `SessionStart` invocation it receives. Under normal Layer 1 once-discipline, this means exactly one `session.started` per session.
2. If Layer 1 once-discipline is bypassed (e.g., manual injection), the plugin emits one `session.started` per invocation. This is operator-observable behavior, not a defect at the plugin layer.

## Invariants

1. The plugin is unconditionally stateless across invocations: it does not maintain a `Mutex<HashSet<String>>` or any other seen-sessions set. It emits `session.started` on every invocation it receives.
2. Once-per-session guarantee is delegated entirely to BC-4.04.004 invariant 1 (Layer 1 `once: true` directive). The dispatcher does not enforce per-event dedup (BC-1.10.002 retired in pass-4).
3. The plugin does not attempt to defend against Layer 1 failures — if the harness misfires and invokes the dispatcher twice for the same session, the plugin emits twice. Operator-observable; not a defect at this layer.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Two `SessionStart` events with the same `session_id` arrive at the dispatcher (Layer 1 once-discipline bypassed, e.g., by manual injection or harness regression) | Plugin is invoked for each arrival and emits `session.started` each time. Operator sees multiple events. This is acceptable at the plugin layer — once-discipline failure is a Layer 1 concern. |
| EC-002 | (Retired in pass-2 — in-plugin `Mutex<HashSet<String>>` dedup superseded by BC-1.10.002; BC-1.10.002 itself retired in pass-4; ID preserved per POLICY 1 append-only-numbering) | N/A — superseded and retired |
| EC-003 | `session_id = "unknown"` (missing session_id in envelope per BC-1.02.005 lifecycle-tolerance) | Plugin emits `session.started` with `session_id = "unknown"` per BC-4.04.001 EC-001. If Layer 1 misfires for this sentinel, the plugin emits multiple times — acceptable per Invariant 3. |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Normal operation: Layer 1 `once: true` fires exactly one `SessionStart` per session | Plugin invoked once; `session.started` emitted once with correct payload (per BC-4.04.001) | happy-path |
| Layer 1 bypass (manual injection): same `session_id` fired twice | Plugin invoked twice; `session.started` emitted twice; operator-observable but not a plugin defect | edge-case (Layer 1 bypass) |
| Missing `session_id` in envelope (mapped to `"unknown"` per BC-1.02.005) | Plugin emits `session.started` with `session_id = "unknown"`; single emission under Layer 1 once-discipline | edge-case |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-065 | Session-Start Plugin Surface Invariant — All BC-4.04.* Postconditions Hold in Integration Test | integration |

## Related BCs

- **BC-4.04.001** — composes with (plugin emits `session.started` per BC-4.04.001 on each invocation it receives)
- **BC-4.04.002** — composes with (plugin invokes factory-health subprocess per BC-4.04.002 on each invocation it receives)
- **BC-4.04.004** — depends on (Layer 1 `once: true` directive is the upstream guarantee that the plugin is invoked at most once per session under normal operation; this is the source of the idempotency guarantee)
- **BC-1.10.002** — retired in pass-4 (dispatcher-side dedup replaced by Layer 1 once-discipline; ID retained per POLICY 1)

## Architecture Anchors

- SS-04 — `crates/hook-plugins/session-start-telemetry/src/lib.rs` (plugin is unconditionally stateless; no in-process dedup state; emits `session.started` on each invocation)
- SS-09 — `plugins/vsdd-factory/hooks/hooks.json.template` (Layer 1 `once: true` directive; the upstream source of the once-per-session guarantee per BC-4.04.004)

## Story Anchor

S-5.01

## VP Anchors

VP-065

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-002 |
| Capability Anchor Justification | CAP-002 ("Hook Claude Code tool calls with sandboxed WASM plugins") per capabilities.md §CAP-002 |
| L2 Domain Invariants | DI-007 (always-on telemetry — `session.started` is emitted unconditionally per invocation; once-discipline ensures one emission per session under normal Layer 1 operation) |
| Architecture Module | SS-04 — `crates/hook-plugins/session-start-telemetry/src/lib.rs` (stateless plugin; emits unconditionally per invocation) |
| Stories | S-5.01 |
| Functional Requirement | FR-046 |
