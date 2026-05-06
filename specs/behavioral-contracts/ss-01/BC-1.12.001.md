---
document_type: behavioral-contract
level: L3
version: "1.1"
status: draft
producer: product-owner
timestamp: 2026-05-06T00:00:00Z
phase: 1a
inputs:
  - .factory/specs/architecture/decisions/ADR-015-single-stream-otel-schema.md
  - .factory/stories/epics/E-10-single-stream-otel-event-emission.md
input-hash: "[pending-recompute]"
traces_to: ADR-015-single-stream-otel-schema.md
origin: greenfield
subsystem: "SS-01"
capability: "CAP-029"
lifecycle_status: active
introduced: v1.1.0
modified: []
deprecated: null
deprecated_by: null
replacement: null
retired: null
removed: null
removal_reason: null
---

# Behavioral Contract BC-1.12.001: factory-dispatcher::host::emit_event::single_stream_filesink_routing — host::emit_event writes all events exclusively to events-YYYY-MM-DD.jsonl via FileSink; Router/SinkRegistry/DlqWriter retired from production path

## Description

After ADR-015 D-15.1, every event emitted by `host::emit_event` — whether a
plugin-emitted domain event or a dispatcher-lifecycle event — MUST be written
to a single physical file: `.factory/logs/events-YYYY-MM-DD.jsonl` via
`FileSink`. The `Router`, `SinkRegistry`, and `DlqWriter` types are NOT called
from the production path. No event reaches `dispatcher-internal-YYYY-MM-DD.jsonl`
via `host::emit_event` (that file is reserved for the `VSDD_DEBUG_LOG=1`-gated
debug stream per BC-1.12.002 and the FileSink write-failure fallback per BC-1.11.002).

This is a future-implementation contract. The current source (pre-E-10 Wave 1)
routes events through `dispatcher-internal-*.jsonl` via `InternalLog` (see
BC-1.05.036 Postcondition 4 bifurcation). BC-1.12.001 specifies the post-Wave-1
spec-frame state that S-10.02 must implement. All Canonical Test Vectors in this
BC are future-implementation witnesses designed to distinguish a correct
single-stream implementation from a reasonable-but-wrong "still routes to
internal log" misimplementation.

## Preconditions

1. The dispatcher process is running and has completed startup initialization
   (Resource attributes stamped per BC-1.12.003; `service.instance.id` UUID generated).
2. A plugin calls `host::emit_event` with a valid event payload, OR the dispatcher
   itself triggers a lifecycle event emission.
3. `FileSink` is initialized and pointing at `.factory/logs/events-YYYY-MM-DD.jsonl`
   (path template resolved at startup using today's UTC date).
4. The invocation is a normal (non-failure) code path — no prior `FileSink::write`
   error has occurred for this event.

## Postconditions

1. The event is written to `.factory/logs/events-YYYY-MM-DD.jsonl` by `FileSink::write`
   (the sole writer after D-15.1). The event is serialized as a JSONL record
   (a single JSON object followed by `\n`) appended to the file.
2. `Router::submit` is NOT called. `SinkRegistry` dispatch is NOT called.
   `DlqWriter` is NOT called. These code paths are excluded from `Cargo.toml`
   `default-members` (deprecated Wave 1) and physically removed (retired Wave 5).
   **Future-implementation witness:** A misimplementation that still calls
   `Router::submit` after D-15.1 will cause events to route to the wrong destination
   or fail silently if the Router is wired but the downstream sinks are not configured.
   The distinguishing test: emit one event via `host::emit_event` and assert it
   appears in `events-*.jsonl` (NOT in `dispatcher-internal-*.jsonl` absent a
   `VSDD_DEBUG_LOG=1` flag).
3. `dispatcher-internal-YYYY-MM-DD.jsonl` does NOT receive this event unless the
   `VSDD_DEBUG_LOG=1` environment variable is set (see BC-1.12.002) or unless
   `FileSink::write` returns an error (fallback path per BC-1.11.002).
   **SOUL #4 acknowledgment (BC-SOUL4-coverage per TD-VSDD-092):** The pre-Wave-1
   `HostContext::emit_internal` function discards IO errors from `log.write` via a
   best-effort pattern — this discard is acknowledged and intentional per
   BC-1.05.036 Postcondition 6 (stable anchor per TD-VSDD-091; line numbers are not
   authoritative — use the function/method name as the canonical reference.) After the
   FileSink rewire, the `emit_event` → `FileSink::write` path uses the write-failure
   cascade per BC-1.11.002 (NOT silent discard) — the cascade writes to the fallback
   file and emits a stderr warning. Silent discard is no longer the failure mode for
   FileSink::write IO errors on the critical path. The pre-existing in-memory
   events-queue Mutex-poison silent-drop (acknowledged per BC-1.05.036 EC-011 /
   OQ-W16-004) is preserved as a documented known-limitation outside the scope of
   ADR-015 D-15.1.
4. The OTel Collector filelog receiver pointed at `events-*.jsonl` receives
   all events — both domain events from plugins AND lifecycle events from the
   dispatcher — via its normal file-tailing mechanism. No consumer-side
   reconfiguration is required to receive both event categories; consumers apply
   `event.category` filters to separate them.
5. The file `.factory/logs/events-YYYY-MM-DD.jsonl` is created on first write of
   the day if it does not already exist. The path template uses the UTC date at the
   time of the first event write.

## Invariants

1. `host::emit_event` has exactly one write destination for the primary (non-failure)
   path: `FileSink` writing to `events-YYYY-MM-DD.jsonl`. No event is split across
   multiple primary destinations.
2. All event categories (`lifecycle`, `domain`, `audit`, `error`, `unknown`) are
   written to the same file. Consumer-side category discrimination via `event.category`
   attribute (per D-15.2 registry — see BC-1.12.004) is the ONLY supported
   fan-out mechanism.
3. `Router`, `SinkRegistry`, and `DlqWriter` are NOT on the production hot path
   for any event emission after Wave 1. Their absence from `default-members` (deprecated)
   and eventual physical deletion (retired Wave 5) enforces this invariant.

## Related BCs

- BC-1.11.002 — FileSink partial-write recovery and write-failure cascade (composes with:
  this BC describes the primary-path emit; BC-1.11.002 describes the failure-path fallback)
- BC-1.12.002 — `VSDD_DEBUG_LOG` gate for `dispatcher-internal-*.jsonl` (composes with:
  this BC defines what does NOT go to the debug file on the primary path; BC-1.12.002
  defines what DOES go there)
- BC-1.12.003 — Resource attribute stamping (depends on: Resource attributes must be
  stamped before the first `host::emit_event` call; they are attached to every event
  written by this BC)
- BC-1.05.036 — `host.exec_subprocess.completed` event (sibling: that event is emitted
  via `ctx.emit_internal` which is the pre-Wave-1 path; after Wave 1 rewire, it must also
  write through `FileSink` to `events-*.jsonl`)

## Architecture Anchors

- `crates/factory-dispatcher/src/main.rs` — `host::emit_event` call site; FileSink wiring
  added here in Wave 1 (S-10.02)
- `crates/sink-file/src/lib.rs` — `FileSink::write` implementation (the single writer)
- `factory-dispatcher::sinks::Sink` trait dispatch surface — the open integration point that
  ADR-015 resolves; `Router::submit` is NOT wired post-Wave-1 (stable anchor per TD-VSDD-091;
  line numbers are not authoritative — use the function/method name as the canonical reference.)
  See also BC-1.05.036 for the pre-Wave-1 `emit_internal` path and Mutex-poison silent-drop
  acknowledgment.
- ADR-015 D-15.1 — policy decision for single physical stream

## Story Anchor

S-10.02 (Wave 1: FileSink single-stream wiring)

## VP Anchors

(TBD — to be assigned after S-10.02 story authoring)

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | First event of the day emitted; `events-YYYY-MM-DD.jsonl` does not yet exist | `FileSink` creates the file on first write; event is the first line; file path uses today's UTC date |
| EC-002 | `events-YYYY-MM-DD.jsonl` already exists from earlier today | `FileSink` appends to the existing file; no data from prior sessions is overwritten |
| EC-003 | `FileSink::write` returns `Err(...)` (e.g., disk full, permissions denied) | Write-failure cascade per BC-1.11.002: fallback write to `dispatcher-internal-*.jsonl` unconditionally; stderr warning emitted; primary failure does NOT change dispatcher exit code |
| EC-004 | `VSDD_DEBUG_LOG=1` is set in the environment | Event is ALSO written to `dispatcher-internal-*.jsonl` (the debug stream is additive — it does NOT replace `events-*.jsonl` as the primary destination); `events-*.jsonl` still receives the event. **Note:** the debug stream is per BC-1.12.002. |
| EC-005 | Dispatcher is run with `Router` or `SinkRegistry` crates present in the workspace (pre-Wave-5 state) | `Router` and `SinkRegistry` code exists on disk but is NOT called from `host::emit_event`. Their presence does not affect the single-stream routing invariant. No event reaches those code paths. |
| EC-006 | Plugin emits a lifecycle event (e.g., `vsdd.dispatcher.started.v1`) via `host::emit_event` | Same FileSink routing applies; lifecycle events are NOT filtered to a separate file. Consumer uses `event.category = "lifecycle"` filter to discriminate. |
| EC-007 | Plugin emits an event with `event.name` prefix NOT in the ADR-015 D-15.2 registry (e.g., `custom.unregistered.event.v1`) | Event is still written to `events-*.jsonl`; `event.category` is stamped `"unknown"` by the host-side category registry (per BC-1.12.004). The unknown categorization does NOT prevent FileSink write. |
| EC-008 | `HookResult::Block` is returned by a plugin | Per ADR-015 D-15.3, the dispatcher emits `vsdd.block.plugin_blocked.v1` with `outcome=blocked` before exiting. This audit event is written to `events-*.jsonl` via the same single-stream path. **Future-implementation witness:** a pre-Wave-1 implementation has no block-path event; the distinguishing test is that `vsdd.block.plugin_blocked.v1` appears in `events-*.jsonl` after a plugin returns `HookResult::Block`. |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Plugin emits one domain event via `host::emit_event`; `VSDD_DEBUG_LOG` unset; no disk errors | Event appears in `.factory/logs/events-YYYY-MM-DD.jsonl` as a valid JSONL line; event does NOT appear in `dispatcher-internal-*.jsonl` (file absent or unchanged) | happy-path-future-witness |
| **Misimplementation distinguisher:** Implementation still routes via `Router::submit` (pre-D-15.1 code path) | Event does NOT appear in `events-*.jsonl`; Router dispatch fails silently or routes to `dispatcher-internal-*.jsonl` only. Test MUST assert `events-*.jsonl` contains the emitted event to distinguish correct from incorrect. | misimplementation-witness |
| Plugin emits domain event; filesystem write succeeds | `events-*.jsonl` contains valid JSON object followed by `\n`; JSON parses without error; `event.name`, `timestamp`, `event.id` fields present | happy-path |
| Plugin emits domain event; `VSDD_DEBUG_LOG=1` | Event appears in BOTH `events-*.jsonl` AND `dispatcher-internal-*.jsonl` | debug-additive-write |
| `FileSink::write` returns `Err(disk full)` | Fallback write to `dispatcher-internal-*.jsonl` (unconditional, regardless of `VSDD_DEBUG_LOG`); stderr warning of form `[vsdd-dispatcher] WARN: FileSink write failed for events-YYYY-MM-DD.jsonl (<error>); event written to dispatcher-internal-YYYY-MM-DD.jsonl as fallback.`; exit code UNCHANGED | write-failure-cascade (per BC-1.11.002) |
| Dispatcher lifecycle event (`vsdd.dispatcher.started.v1`) emitted at startup | Event appears in `events-*.jsonl`; `event.category = "lifecycle"` (per ADR-015 D-15.2 registry, `vsdd.dispatcher.*` prefix maps to `lifecycle`) | lifecycle-to-single-stream |
| Plugin returns `HookResult::Block`; Wave 1 block-audit implemented | `vsdd.block.plugin_blocked.v1` event appears in `events-*.jsonl` with `outcome=blocked`, `plugin.name`, `hook.tool_name`; no event appears unless plugin returns Block | block-path-audit-trail |
| Event with unregistered prefix `custom.foo.bar.v1` emitted | Event written to `events-*.jsonl`; `event.category = "unknown"` (ADR-015 D-15.2.b unrecognized prefix rule) | unknown-category-routing |
| `Router` crate present in workspace; `Router::submit` NOT called from `host::emit_event` | Cargo build succeeds; `Router` code unused; events route only to `FileSink`; no Router-related errors | deprecated-but-not-deleted |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| (TBD — to be assigned after S-10.02 authoring) | All events from `host::emit_event` appear in `events-*.jsonl` | integration test: emit N events; assert N lines in `events-*.jsonl` |
| (TBD) | `dispatcher-internal-*.jsonl` receives 0 events on primary path when `VSDD_DEBUG_LOG` unset | integration test: run dispatcher without `VSDD_DEBUG_LOG`; assert `dispatcher-internal-*.jsonl` absent or empty after N emissions |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-029 ("Emit structured events to a single observability stream (file path)") per capabilities.md §CAP-029 |
| Capability Anchor Justification | CAP-029 ("Emit structured events to a single observability stream (file path)") per capabilities.md §CAP-029. This BC specifies the single-stream FileSink wiring that removes Router/SinkRegistry/DlqWriter from the production path — the exact architectural outcome that CAP-029 describes in its first paragraph: "The dispatcher writes every user-facing domain event as a JSONL record to a single `events-YYYY-MM-DD.jsonl` file via FileSink. Router, SinkRegistry, and DlqWriter are retired." BC-1.12.001 is the behavioral contract that makes that CAP-029 outcome verifiable. |
| L2 Domain Invariants | DI-011 (superseded by ADR-015 D-15.1 — single-sink eliminates submit-must-not-block; this BC is the post-supersession replacement); DI-012 (superseded by ADR-015 D-15.1 — single-sink eliminates per-sink isolation; this BC governs the single-sink behavior that replaces both DIs) |
| Architecture Module | SS-01 — `crates/factory-dispatcher/src/main.rs` (emit_event call site), `crates/sink-file/src/lib.rs` (FileSink) |
| Stories | S-10.02 (Wave 1 FileSink single-stream wiring) |
| Epic | E-10 (Single-stream OTel-aligned event emission) |
| ADR | ADR-015 D-15.1 (single physical stream policy) |
| OQ Resolved | OQ-W16-003 (FileSink write-failure cascade — fallback path is BC-1.11.002; this BC covers primary-path routing) |

### Purity Classification

| Property | Assessment |
|----------|-----------|
| I/O operations | YES — file append write via FileSink |
| Global state access | YES — reads dispatcher-level FileSink instance and Resource attribute context |
| Deterministic | YES given fixed filesystem state and fixed date |
| Thread safety | YES — FileSink is not shared across concurrent threads in the single-stream model (single-threaded dispatcher per ADR-008) |
| Overall classification | Effectful shell (file I/O with defined failure contract per BC-1.11.002) |

### TD-VSDD-092 (BC-SOUL4-coverage) Verification

Source-walk for silent-discard patterns in the `host::emit_event` → `FileSink::write` future implementation path:

- The pre-Wave-1 `HostContext::emit_internal` function uses
  `if let Ok(mut events) = self.events.lock()` which silently drops on Mutex poison
  (EC-007 of BC-1.05.036 / EC-011). This is acknowledged in BC-1.05.036 and is NOT
  changed by BC-1.12.001 — the FileSink rewire adds a primary write path; the
  Mutex-guarded in-memory queue remains best-effort. (Stable anchor per TD-VSDD-091;
  use function name `HostContext::emit_internal` not line numbers as the canonical reference.)
- The `FileSink::write` failure path must NOT use `let _ =` pattern (silent discard is
  prohibited by BC-1.11.002 Invariant 3). The implementation MUST propagate write errors
  to the fallback cascade. This BC explicitly designates Postcondition 3 as the SOUL #4
  guard: the implementation is required to use write-failure cascade, NOT silent discard.
- No additional `let _ =` patterns are expected in the `host::emit_event` path for the
  FileSink routing change.
