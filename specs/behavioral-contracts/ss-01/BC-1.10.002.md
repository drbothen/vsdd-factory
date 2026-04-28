---
document_type: behavioral-contract
level: L3
version: "1.0"
status: draft
producer: architect
timestamp: 2026-04-28T00:00:00
phase: 1a
inputs:
  - .factory/specs/architecture/SS-01-hook-dispatcher.md
  - .factory/specs/behavioral-contracts/ss-04/BC-4.04.003.md
input-hash: "728da22"
traces_to: .factory/specs/prd.md#FR-046
origin: greenfield
extracted_from: null
subsystem: "SS-01"
capability: "CAP-002"
lifecycle_status: retired
introduced: v1.0.0-rc.1
modified: []
deprecated: null
deprecated_by: null
replacement: "Claude Code hooks.json.template Layer 1 once:true directive; BC-4.04.004 contracts the Layer 1 entry. Plugin idempotency becomes 'delegated to Layer 1' per BC-4.04.003 revised contract."
retired: 2026-04-28
removed: null
removal_reason: "Over-engineered foundation — Claude Code Layer 1 'once: true' directive in hooks.json.template enforces once-per-session firing at the harness layer. Dispatcher only ever receives one SessionStart invocation per session, making dispatcher-side dedup redundant. Pass-4 root-cause review found pass-2 created this BC without verifying the upstream once-discipline."
---

# BC-1.10.002: Dispatcher suppresses duplicate once:true events by tracking per-event-name + per-session_id in dispatcher memory

## Description

The dispatcher maintains a per-process deduplication tracker (a
`Mutex<HashSet<(event_name, session_id)>>`) for all `hooks-registry.toml` entries that
declare `once = true`. Before routing a `once = true` event to any plugin, the dispatcher
checks whether the `(event_name, session_id)` pair has already been dispatched in the current
process lifetime. If it has, the dispatcher skips invocation of all plugins registered for
that event and returns as if no plugins were registered. If it has not, the dispatcher
inserts the pair into the tracker and proceeds with normal routing.

This contract supersedes and replaces the original plugin-side dedup design described in
BC-4.04.003 Invariant 3 (`Mutex<HashSet<String>>` inside the WASM plugin). Moving dedup to
the dispatcher layer eliminates the wasmtime Store reuse dependency: WASM plugins are
stateless across invocations as standard wasmtime practice, and in-process plugin state
does not reliably persist across invocations unless the dispatcher explicitly reuses the
Store. Dispatcher-side dedup is architecturally correct and does not impose any Store reuse
requirement.

## Motivation (Architectural Ruling — S-5.01 Pass-2 Finding F-12)

BC-4.04.003 Invariant 3 originally placed the dedup `Mutex<HashSet<String>>` inside the
WASM plugin's in-process state. Two options were evaluated:

- **Option D1 — Mandate Store reuse for plugins declaring state_lifetime = "process":**
  Add a new registry field and require the dispatcher to reuse wasmtime Stores across
  invocations for those plugins. Rejected: non-standard wasmtime usage; introduces
  complex Store lifecycle management; dedup is not a plugin-domain concern.
- **Option D2 — Dispatcher-side dedup (this BC):** The dispatcher tracks once:true event
  suppression entirely within dispatcher memory. WASM plugins remain stateless. Accepted:
  architecturally correct layer; no Store reuse required; once:true semantics are a
  routing-level concern, not a plugin-level concern.

BC-4.04.003 is amended by this BC: the `Mutex<HashSet<String>>` in the plugin is removed.
The plugin becomes unconditionally stateless. Idempotency of `session.started` emission
is guaranteed by the dispatcher, not the plugin.

## Preconditions

1. The dispatcher has loaded `hooks-registry.toml` and identified one or more entries
   with `once = true`.
2. A `SessionStart` (or other `once = true`) event has arrived at the dispatcher with a
   populated `session_id` in the envelope.

## Postconditions

1. For the first `(event_name, session_id)` pair seen in this process lifetime:
   a. The pair is inserted into the dispatcher's dedup tracker.
   b. Routing proceeds normally — all matching plugins for the event are invoked.
2. For any subsequent `(event_name, session_id)` pair that already exists in the tracker:
   a. The dispatcher skips plugin invocation entirely.
   b. The dispatcher returns a synthetic "no plugins matched" result (exit code 0,
      no block_intent, no events emitted).
   c. The skip is logged at DEBUG level with `reason = "once:true duplicate suppressed"`.
3. Pairs with `session_id = "unknown"` are EXCLUDED from dedup: the dispatcher does NOT
   insert `(event_name, "unknown")` into the tracker, so every `"unknown"` session_id
   event is routed normally.
4. The dedup tracker is scoped to the dispatcher process lifetime — it is never persisted
   to disk and is reset on dispatcher restart.

## Invariants

1. At most one plugin invocation is performed per `(event_name, session_id)` pair per
   dispatcher process lifetime (for `once = true` entries).
2. The dedup tracker is thread-safe (`Mutex<HashSet<...>>` or equivalent).
3. The `"unknown"` sentinel session_id is never deduplicated — this matches BC-4.04.003
   EC-003 semantics, now enforced at the dispatcher layer.
4. The dedup tracker has no eviction policy in v1.0 (acceptable upper bound: expected
   sessions per process lifetime ≤ 1000; tracker memory ≤ 64 KB).
5. Non-`once = true` entries are never affected by the dedup tracker.
6. **Atomicity (TOCTOU prevention):** The check-and-insert operation MUST be atomic under
   the dedup Mutex. Implementations MUST use `HashSet::insert(pair)` (Rust), which returns
   a `bool` (true = was-not-present, false = was-already-present) inside a single critical
   section. The Mutex lock MUST NOT be released between the check-existing step and the
   insert-new step. The routing decision is determined by the boolean return value: if `true`
   (pair was not present), routing proceeds and the pair is now in the set; if `false`
   (pair was already present), routing is suppressed. Two concurrent `SessionStart` events
   with the same `session_id` will race on the Mutex — exactly one will see `true` and
   proceed; the other will see `false` and be suppressed.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Dispatcher restarts between two `SessionStart` events with same `session_id` | Dedup state is lost on restart; second event is routed normally (new process lifetime) |
| EC-002 | Two different `session_id` values for the same event_name arrive concurrently | Both are routed; both pairs inserted into tracker independently (lock acquired per check) |
| EC-003 | `session_id = "unknown"` in `once = true` event | NOT deduplicated; every `"unknown"` event routes to plugins normally |
| EC-004 | Tracker reaches 1000 entries | No eviction; tracker continues to grow. v1.1 candidate for LRU eviction |
| EC-005 | `once = true` entry with no `session_id` in envelope (field absent) | Maps to `"unknown"` sentinel; not deduplicated (per EC-003). **Precondition note:** EC-005 applies only when BC-1.02.005 lifecycle-tolerance permits a missing/empty `session_id` (e.g., `SessionStart` lifecycle events parse with `session_id` defaulting to `""` per BC-1.02.005, which the dispatcher routing layer maps to `"unknown"` before reaching the dedup check). For non-lifecycle events, BC-1.02.001 rejects empty `session_id` at payload parse time, before the envelope ever reaches the dispatcher routing layer where the dedup tracker operates — EC-005 is therefore unreachable for non-lifecycle events. |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| First `SessionStart` `session_id="sess-001"` → plugins invoked | Plugin invoked; `(SessionStart, sess-001)` inserted in tracker | happy-path |
| Second `SessionStart` `session_id="sess-001"` (same process) | Plugin skipped; tracker unchanged; exit 0 | happy-path (idempotency) |
| `SessionStart` `session_id="sess-001"` then `session_id="sess-002"` | Both invoked; both pairs in tracker | edge-case |
| `SessionStart` `session_id="unknown"` × 2 | Both invoked (no dedup for `"unknown"`) | edge-case |
| Dispatcher restart, then `SessionStart` `session_id="sess-001"` | Plugin invoked (fresh process, empty tracker) | edge-case (restart boundary) |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-065 | Session-Start Plugin Surface Invariant — integration test covers Postconditions 1a–1c (first-arrival insert and routing), 2a–2c (duplicate suppression, synthetic no-plugins result, DEBUG log), and EC-001 through EC-005 (restart boundary, concurrent distinct sessions, "unknown" bypass, tracker growth, absent session_id sentinel) | integration |

## Related BCs

- **BC-4.04.003** — supersedes in part (the in-plugin `Mutex<HashSet<String>>` in Invariant 3
  is replaced by this dispatcher-side tracker; plugin becomes stateless; idempotency guarantee
  is now provided by the dispatcher, not the plugin)
- **BC-1.09.002** — parallel (PluginCache thread-safety via Mutex; same pattern applied to
  dedup tracker)
- **BC-4.04.001** — outcome guaranteed by (once-per-session_id emission of `session.started`
  is now enforced here at the dispatcher layer)
- **BC-1.02.005** — depends on (envelope parsing delivers `session_id` to the dispatcher
  routing layer where the dedup tracker reads it; BC-1.02.005 lifecycle-tolerance permits
  `SessionStart` envelopes to have a missing `tool_name` and an empty/absent `session_id`,
  which the dispatcher routing layer maps to the `"unknown"` sentinel before reaching the
  dedup check — this is the upstream precondition for EC-005 "unknown" sentinel handling)

## Architecture Anchors

- SS-01 — `crates/factory-dispatcher/src/dispatch.rs` or `router.rs` (dedup tracker
  `Mutex<HashSet<(String, String)>>` initialized at dispatcher startup; checked and updated
  in the routing hot path before plugin invocation for `once = true` entries)
- SS-07 — `plugins/vsdd-factory/hooks-registry.toml` (`once = true` field on `[[hooks]]`
  entries; this field is the trigger that activates dispatcher-side dedup for that entry)

## Story Anchor

S-5.01

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-002 |
| Capability Anchor Justification | CAP-002 ("Hook Claude Code tool calls with sandboxed WASM plugins") — dispatcher routing semantics are part of the plugin execution infrastructure |
| L2 Domain Invariants | DI-001 (tier sequential execution — concurrent once:true arrivals must not race through dedup); DI-002 (plugin crash does not block siblings — dedup operates before plugin invocation, so a crashed plugin does not prevent tracker update) |
| Architecture Module | SS-01 — `crates/factory-dispatcher/src/` (dispatch/router module) |
| Stories | S-5.01 |
| Functional Requirement | FR-046 |
