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

# BC-4.05.003: session-end plugin is unconditionally stateless; idempotency delegated to Layer 1 once:true

## Description

The `session-end-telemetry` plugin maintains NO in-process state across invocations. It does not track which `session_id` values it has already processed. Idempotency for `SessionEnd` events — ensuring each session produces exactly one `session.ended` — is enforced upstream at Layer 1 by Claude Code's `once: true` directive in `hooks.json.template` (per BC-4.05.004 invariant 1). If Layer 1 fires multiple `SessionEnd` events with the same `session_id` (Layer 1 bypass; e.g., manual injection during testing or a harness regression), the plugin emits multiple `session.ended` events — operator-observable but not a defect at the plugin layer. This pattern mirrors BC-4.04.003 exactly, applied to SessionEnd semantics.

## Preconditions

1. Claude Code's `once: true` directive in `hooks.json.template` (BC-4.05.004 invariant 1) is in effect — the harness fires `SessionEnd` to the dispatcher at most once per session.
2. A `SessionEnd` event arrives at the dispatcher.

## Postconditions

1. The plugin retains no state between invocations: no `Mutex<HashSet<String>>`, no global invocation counters, no dedup tables.
2. The plugin processes each `SessionEnd` event independently and emits exactly one `session.ended` event per invocation it receives.
3. Under normal Layer 1 once-discipline, each session produces exactly one `session.ended` event.

## Invariants

1. The plugin is unconditionally stateless across invocations. State between calls is prohibited at the plugin layer.
2. Layer 1 `once: true` (BC-4.05.004 invariant 1) is the single source of idempotency for `SessionEnd` — the plugin does not attempt to enforce it.
3. The plugin does not attempt to defend against Layer 1 failures — if the harness misfires and invokes the dispatcher twice for the same session, the plugin emits twice. Operator-observable; not a defect at this layer.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Dispatcher restart between two `SessionEnd` events with the same `session_id` | Layer 1 `once: true` prevents duplicate fires under normal operation; plugin sees only one event; emits `session.ended` once |
| EC-002 | (Reserved; not assigned in this foundation burst.) Per the BC-4.04.003 structural template, EC-002 was retired in S-5.01's pass-4 reversal — the plugin-side `Mutex<HashSet>` dedup approach was superseded by Layer 1 `once: true` discipline. For S-5.02 we apply that lesson up-front: EC-002 was never drafted here because plugin-side dedup was never considered a candidate. The ID is intentionally skipped for parallel structural alignment with BC-4.04.003. Preserved per append-only-numbering policy. | N/A — reserved; ID skipped for structural alignment |
| EC-003 | `session_id = "unknown"` (per BC-1.02.005 lifecycle-tolerance, missing session_id in envelope) | Plugin emits `session.ended` with `session_id = "unknown"` per BC-4.05.001 EC-004; no special handling; plugin is unconditionally stateless |
| EC-004 | Two `SessionEnd` events with the same `session_id` arrive (Layer 1 bypass, e.g., manual injection) | Plugin invoked for each; emits `session.ended` each time; operator sees duplicate events; acceptable at plugin layer — Layer 1 once-discipline failure is a Layer 1 concern |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Normal operation: Layer 1 `once: true` fires exactly one `SessionEnd` per session | Plugin invoked once; `session.ended` emitted once with correct payload (per BC-4.05.001) | happy-path |
| Layer 1 bypass (manual injection): same `session_id` fired twice | Plugin invoked twice; `session.ended` emitted twice; operator-observable but not a plugin defect | edge-case (Layer 1 bypass) |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-066 | Session-End Plugin Surface Invariant — All BC-4.05.* Postconditions Hold in Integration Test | integration |

## Related BCs

- **BC-4.05.001** — composes with (plugin emits `session.ended` per BC-4.05.001 on each invocation it receives; statelessness ensures clean per-invocation semantics)
- **BC-4.05.004** — depends on (Layer 1 `once: true` directive is the upstream guarantee that the plugin is invoked at most once per session under normal operation)
- **BC-4.04.003** — structural analog (SessionStart idempotency contract; BC-4.05.003 mirrors it exactly for SessionEnd)

## Architecture Anchors

- SS-04 — `crates/hook-plugins/session-end-telemetry/src/lib.rs` (plugin is unconditionally stateless; no in-process dedup state; emits `session.ended` on each invocation)
- SS-09 — `plugins/vsdd-factory/hooks/hooks.json.template` (Layer 1 `once: true` directive; the upstream source of the once-per-session guarantee per BC-4.05.004)

## Story Anchor

S-5.02

## VP Anchors

VP-066

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-002 |
| Capability Anchor Justification | CAP-002 ("Hook Claude Code tool calls with sandboxed WASM plugins") per capabilities.md §CAP-002 |
| L2 Domain Invariants | DI-007 (always-on telemetry — `session.ended` is emitted unconditionally per invocation; once-discipline ensures one emission per session under normal Layer 1 operation) |
| Architecture Module | SS-04 — `crates/hook-plugins/session-end-telemetry/src/lib.rs` (stateless plugin; emits unconditionally per invocation) |
| Stories | S-5.02 |
| Functional Requirement | FR-046 |
