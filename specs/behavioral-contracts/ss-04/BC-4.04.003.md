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

# BC-4.04.003: session-start plugin is idempotent on duplicate SessionStart events within the same session_id

## Description

If the session-start plugin receives a second `SessionStart` event carrying the same `session_id` as one it has already processed within the current dispatcher process lifetime, it does NOT re-emit `session.started`. The deduplication is maintained in-process via a seen-sessions set within the plugin's in-memory state, which persists for the lifetime of the dispatcher process (and therefore the plugin's WASM instance). This prevents duplicate telemetry rows when Claude Code occasionally fires redundant lifecycle events.

## Preconditions

1. The plugin has previously received and processed a `SessionStart` event with `session_id = X`.
2. A second `SessionStart` event arrives at the same plugin instance with `session_id = X`.

## Postconditions

1. No additional `session.started` event is emitted for the duplicate `SessionStart`.
2. The plugin returns `HookResult::Ok` (exit code 0) — the duplicate is silently dropped, not an error.
3. The in-memory seen-sessions set still contains `session_id = X` after the second invocation.

## Invariants

1. At most one `session.started` event is emitted per `session_id` (where `session_id` ≠ `"unknown"`) per dispatcher process lifetime.
2. Deduplication state is in-process only — it does not persist across dispatcher restarts.
3. Deduplication state must be thread-safe (`Mutex<HashSet<String>>` or equivalent). Concurrent `SessionStart` events (possible per BC-1.03.008 concurrent-execution context) must not cause race conditions in the seen-sessions set.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Dispatcher restarts between two `SessionStart` events with the same `session_id` | Dedup state is lost on restart; the second event is treated as a new session and `session.started` is emitted. This is acceptable and documented — cross-process deduplication is out of scope. |
| EC-002 | Two different `session_id` values arrive in rapid succession | Both emit `session.started` independently; dedup set grows to contain both IDs |
| EC-003 | `session_id = "unknown"` sentinel appears in two separate `SessionStart` events (both envelopes had missing session_id) | Dedup is SKIPPED for `session_id = "unknown"`; both events emit `session.started`. Operator sees two events. This sentinel is excluded from the dedup set intentionally. |
| EC-004 | Dedup `HashSet` reaches 1000 entries | No eviction in v1.0 (acceptable upper bound; expected sessions per dispatcher process lifetime ≤ 1000, set memory ≤ 64 KB). v1.1 candidate for LRU eviction at higher thresholds. |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| First `SessionStart` with `session_id = "sess-001"` → second `SessionStart` with `session_id = "sess-001"` (same process) | `session.started` emitted exactly once total | happy-path (idempotency) |
| `SessionStart` with `session_id = "sess-001"` → `SessionStart` with `session_id = "sess-002"` | `session.started` emitted twice (one per unique session_id) | edge-case |
| Simulated dispatcher restart: `SessionStart` `sess-001` processed, plugin re-initialized, `SessionStart` `sess-001` received again | `session.started` emitted on second receipt (new process lifetime, dedup state absent) | edge-case (restart boundary) |
| Two `SessionStart` events both with missing `session_id` (both mapped to `"unknown"`) in same process lifetime | `session.started` emitted TWICE (dedup skipped for `"unknown"` sentinel); operator sees both events | edge-case (unknown sentinel bypass) |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-065 | Session-Start Plugin Surface Invariant — All BC-4.04.* Postconditions Hold in Integration Test | integration |

## Related BCs

- **BC-4.04.001** — composes with (idempotency guard wraps the `emit_event` call in BC-4.04.001)
- **BC-4.04.002** — composes with (idempotency guard also suppresses redundant `exec_subprocess` calls for duplicate events)
- **BC-1.03.008** — depends on (concurrent-execution context motivates the Mutex thread-safety requirement for the dedup HashSet)

## Architecture Anchors

- SS-04 — `crates/hook-plugins/session-start-telemetry/src/lib.rs` (in-memory `HashSet<String>` for seen session_ids; checked before `emit_event`)

## Story Anchor

S-5.01

## VP Anchors

VP-065

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-002 |
| Capability Anchor Justification | CAP-002 ("Hook Claude Code tool calls with sandboxed WASM plugins") per capabilities.md §CAP-002 |
| L2 Domain Invariants | DI-001 (tier sequential execution — concurrent SessionStart arrivals within a tier must not break dedup atomicity); DI-002 (plugin crash does not block siblings — dedup state uses Mutex to avoid blocking) |
| Architecture Module | SS-04 — `crates/hook-plugins/session-start-telemetry/src/lib.rs` |
| Stories | S-5.01 |
| Functional Requirement | FR-046 |
