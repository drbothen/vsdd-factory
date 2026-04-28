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

# BC-4.04.003: session-start plugin is idempotent on duplicate SessionStart events within the same session_id

## Description

If the dispatcher receives a second `SessionStart` event carrying the same `session_id` as one it has already dispatched within the current process lifetime, it suppresses the invocation before reaching the plugin — the plugin is never called for the duplicate. Deduplication is a routing-layer concern enforced by the dispatcher's per-process `Mutex<HashSet<(event_name, session_id)>>` tracker per BC-1.10.002. The plugin itself is unconditionally stateless across invocations: it emits `session.started` on every invocation and relies on the dispatcher to ensure it is only invoked once per `(SessionStart, session_id)` pair. This prevents duplicate telemetry rows when Claude Code occasionally fires redundant lifecycle events. (BC-1.10.002 supersedes the original plugin-side `Mutex<HashSet<String>>` design specified in this contract's Invariant 3.)

## Preconditions

1. The dispatcher has already routed a `SessionStart` event with `session_id = X` to the plugin in this process lifetime, inserting `(SessionStart, X)` into the dispatcher dedup tracker per BC-1.10.002.
2. A second `SessionStart` event with `session_id = X` arrives at the dispatcher.

## Postconditions

1. No additional `session.started` event is emitted for the duplicate `SessionStart` — the dispatcher suppresses the second routing before the plugin is invoked.
2. The dispatcher returns a synthetic "no plugins matched" result (exit code 0, no block_intent, no events emitted) per BC-1.10.002 Postcondition 2b.
3. The dispatcher's dedup tracker still contains the `(SessionStart, X)` pair after the second arrival — the pair is not evicted on a suppressed invocation.

## Invariants

1. At most one `session.started` event is emitted per `session_id` (where `session_id` ≠ `"unknown"`) per dispatcher process lifetime. This guarantee is enforced at the dispatcher routing layer per BC-1.10.002, not in the plugin.
2. Deduplication state is in-process only — it does not persist across dispatcher restarts.
3. Dedup is enforced at the dispatcher routing layer per BC-1.10.002, not in the plugin. The plugin is unconditionally stateless across invocations: it does not maintain a `Mutex<HashSet<String>>` or any other seen-sessions set internally.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Dispatcher restarts between two `SessionStart` events with the same `session_id` | Dedup state is lost on restart; the second event is treated as a new session and `session.started` is emitted. This is acceptable and documented — cross-process deduplication is out of scope. |
| EC-003 | `session_id = "unknown"` sentinel appears in two separate `SessionStart` events (both envelopes had missing session_id) | Dedup is SKIPPED for `session_id = "unknown"` — confirmed at the dispatcher layer per BC-1.10.002 EC-003. Both events route to the plugin; both emit `session.started`. Operator sees two events. |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Dispatcher receives first `SessionStart` with `session_id = "sess-001"` (same process) | Dispatcher routes to plugin; plugin invoked; `session.started` emitted; `(SessionStart, sess-001)` inserted into dispatcher tracker (per BC-1.10.002) | happy-path |
| Dispatcher receives second `SessionStart` with `session_id = "sess-001"` (same process) | Dispatcher suppresses routing (pair already in tracker); plugin NOT invoked; no `session.started` emitted; `exec_subprocess` invocation count = 0 for second event | happy-path (dispatcher-side idempotency) |
| Dispatcher receives two `SessionStart` events both with missing `session_id` (mapped to `"unknown"`) | Dispatcher does NOT dedup "unknown" (per BC-1.10.002 EC-003); both route to plugin; plugin invoked twice; `session.started` emitted twice; `exec_subprocess` invocation count = 2 | edge-case (unknown sentinel bypass) |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-065 | Session-Start Plugin Surface Invariant — All BC-4.04.* Postconditions Hold in Integration Test | integration |

## Related BCs

- **BC-4.04.001** — composes with (idempotency guard wraps the `emit_event` call in BC-4.04.001)
- **BC-4.04.002** — composes with (idempotency guard also suppresses redundant `exec_subprocess` calls for duplicate events)
- **BC-1.03.008** — depends on (concurrent-execution context motivates the Mutex thread-safety requirement for the dedup HashSet at the dispatcher layer)
- **BC-1.10.002** — supersedes plugin-side dedup (dispatcher-side `Mutex<HashSet<(event_name, session_id)>>` replaces the plugin-side `Mutex<HashSet<String>>` originally specified in this BC's Invariant 3)

## Architecture Anchors

- SS-04 — `crates/hook-plugins/session-start-telemetry/src/lib.rs` (plugin is stateless; no in-process HashSet; emits `session.started` unconditionally on each invocation)
- SS-01 — `crates/factory-dispatcher/src/dispatch.rs` or `router.rs` (dispatcher-side dedup tracker per BC-1.10.002; suppresses plugin invocation before it reaches SS-04 plugin)

## Story Anchor

S-5.01

## VP Anchors

VP-065

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-002 |
| Capability Anchor Justification | CAP-002 ("Hook Claude Code tool calls with sandboxed WASM plugins") per capabilities.md §CAP-002 |
| L2 Domain Invariants | DI-001 (tier sequential execution — concurrent SessionStart arrivals within a tier must not break dedup atomicity; now enforced by dispatcher-layer Mutex per BC-1.10.002); DI-002 (plugin crash does not block siblings — dedup operates before plugin invocation per BC-1.10.002, so a crashed plugin does not prevent tracker update) |
| Superseded by | BC-1.10.002 supersedes plugin-side dedup (Invariant 3 formerly required in-plugin Mutex<HashSet<String>>; dispatcher-side dedup replaces it) |
| Architecture Module | SS-04 — `crates/hook-plugins/session-start-telemetry/src/lib.rs` (stateless plugin); SS-01 — dispatcher routing (enforces dedup) |
| Stories | S-5.01 |
| Functional Requirement | FR-046 |
