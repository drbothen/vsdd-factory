---
document_type: behavioral-contract
level: L3
version: "1.1"
status: draft
producer: product-owner
timestamp: 2026-05-06T00:00:00Z
phase: 1b
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

# Behavioral Contract BC-1.12.006: factory-dispatcher::host::block_path_audit — vsdd.block.plugin_blocked.v1 audit event emitted with outcome=blocked, plugin.name, hook.tool_name when plugin returns HookResult::Block (ADR-015 D-15.3)

## Description

Prior to ADR-015, when a plugin returned `HookResult::Block`, the dispatcher
exited with code 2 (the block exit code) but emitted NO observable event. The
block path had no audit trail. A security team could not determine after the
fact which plugin had blocked which tool call, or when.

ADR-015 D-15.3 closes this gap: when a plugin returns `HookResult::Block`, the
dispatcher MUST emit a `vsdd.block.plugin_blocked.v1` event to `events-*.jsonl`
before exiting. This event carries `outcome = "blocked"`, `plugin.name`, and
`hook.tool_name` at minimum.

This BC specifies the block-path audit event contract: what is emitted, when,
what fields it must carry, and what happens if the emission itself fails.

This is a future-implementation contract for S-10.04 (Wave 1). All Canonical
Test Vectors describe post-Wave-1 behavior.

## Preconditions

1. The dispatcher is processing a plugin invocation for a PreToolUse hook.
2. The plugin has completed execution and returned `HookResult::Block`.
3. The dispatcher has not yet exited (exit code 2 is deferred until after the
   audit event is written).
4. `FileSink` is initialized (post-Wave-1 single-stream state).

## Postconditions

1. Before the dispatcher exits with exit code 2, it MUST emit a
   `vsdd.block.plugin_blocked.v1` event to `events-*.jsonl` via the normal
   FileSink path (per BC-1.12.001).
2. The emitted event MUST carry at minimum the following fields with the
   specified values:

   | Field | Required Value |
   |-------|----------------|
   | `event.name` | `"vsdd.block.plugin_blocked.v1"` |
   | `event.category` | `"audit"` (derived from `vsdd.block.*` prefix in compile-time registry per BC-1.12.004 D-15.2.b) |
   | `outcome` | `"blocked"` |
   | `plugin.name` | plugin identifier from hooks-registry.toml (the plugin that returned `Block`) |
   | `hook.tool_name` | the tool name from the Claude envelope that triggered this hook invocation |

3. All host-stamped per-event fields (per BC-1.12.004 Postcondition 1) are also
   stamped on this event: `timestamp`, `event.id`, `event.source`, `trace_id`,
   `span_id`, `parent_span_id`, `plugin.invocation_id`, `session.id`, etc.
4. All 15 Resource attributes (per BC-1.12.003) are present on the event.
5. The event is written to `events-*.jsonl` BEFORE the dispatcher exits. Exit
   code 2 is not returned to Claude Code until the event write completes (or
   the write-failure fallback is exhausted per BC-1.11.002).
6. If `FileSink::write` fails on the block-audit event: the standard
   write-failure cascade per BC-1.11.002 applies — fallback write to
   `dispatcher-internal-*.jsonl` unconditionally; stderr warning; exit code
   unchanged (still 2). The block is reported and the dispatcher exits normally.
7. **Future-implementation witness:** A pre-Wave-1 implementation returns exit
   code 2 without emitting any event. The distinguishing test: run a plugin
   that returns `HookResult::Block`; assert `vsdd.block.plugin_blocked.v1`
   appears in `events-*.jsonl`.

## Invariants

1. `vsdd.block.plugin_blocked.v1` is emitted for EVERY `HookResult::Block`
   return, with no exceptions. There is no "quiet block" path.
2. The `vsdd.block.*` prefix maps to `event.category = "audit"` in the
   compile-time registry (per ADR-015 D-15.2 registry table and BC-1.12.004).
   This categorization ensures block events appear in audit dashboards, NOT
   domain dashboards.
3. The dispatcher exit code (2) is UNCHANGED by the audit event emission
   result. A FileSink failure does not change the exit code; the block is still
   enforced. The audit trail best-effort does not weaken the block enforcement.
4. The emit-before-exit ordering invariant: the audit event write (or its
   fallback cascade) must complete before the dispatcher process returns exit
   code 2 to its parent process.

## Related BCs

- BC-1.12.001 — Single-stream FileSink routing (composes with: block audit
  event routes through the same FileSink path)
- BC-1.12.004 — Per-event host-stamping and `event.category` registry (depends
  on: `event.category = "audit"` derived from `vsdd.block.*` prefix; all
  per-event fields stamped)
- BC-1.11.002 — FileSink write-failure cascade (composes with: if FileSink
  write fails on the block-audit event, the fallback cascade applies)
- BC-1.08.001 — `HookResult::Block` exit-code semantics (sibling: exit code 2
  for block returns is governed there; this BC governs the pre-exit audit event)

## Architecture Anchors

- `crates/factory-dispatcher/src/main.rs` — the `HookResult::Block` branch in
  the plugin-invocation result handler; audit event emission added here in
  S-10.04 before the `std::process::exit(2)` call
- `crates/factory-dispatcher/src/host/emit_event.rs` — standard per-event
  stamping path (block-audit event goes through this same path)
- ADR-015 D-15.3 — "When a plugin returns `HookResult::Block`, the dispatcher
  emits a `vsdd.block.plugin_blocked.v1` event with `outcome=blocked`,
  `plugin.name`, and `hook.tool_name` before exiting. Block path now has an
  audit trail."

## Story Anchor

S-10.04 (Wave 1: Trace propagation + lifecycle event types; block-path audit
trail is a D-15.3 lifecycle event type deliverable)

## VP Anchors

(TBD — to be assigned after S-10.04 story authoring)

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Plugin returns `HookResult::Block` on a `Bash` tool call | `vsdd.block.plugin_blocked.v1` emitted with `hook.tool_name = "Bash"`, `outcome = "blocked"`, `plugin.name` set; exit code 2; event in `events-*.jsonl` before exit |
| EC-002 | Plugin returns `HookResult::Block` on a `Write` tool call | Same as EC-001; `hook.tool_name = "Write"` |
| EC-003 | `FileSink::write` fails when writing the block-audit event | Fallback write to `dispatcher-internal-*.jsonl` per BC-1.11.002; stderr warning; dispatcher still exits with code 2; block is still enforced |
| EC-004 | Plugin returns `HookResult::Allow` (not Block) | No `vsdd.block.plugin_blocked.v1` event emitted; normal execution path |
| EC-005 | Multiple plugins invoked in sequence; second plugin returns `HookResult::Block` | `vsdd.block.plugin_blocked.v1` emitted for the SECOND plugin; the first plugin's outcome events are unaffected; `plugin.name` identifies the blocking plugin |
| EC-006 | Plugin returns `HookResult::Block`; `VSDD_DEBUG_LOG=1` is set | Block-audit event written to `events-*.jsonl` (primary stream) AND to `dispatcher-internal-*.jsonl` (debug-supplementary write per BC-1.12.002) |
| EC-007 | Pre-Wave-1 dispatcher (no audit event) vs post-Wave-1 | **Future-implementation witness:** Test MUST assert `vsdd.block.plugin_blocked.v1` event appears in `events-*.jsonl` after a plugin returns Block. A pre-Wave-1 implementation that only exits with code 2 will fail this assertion — distinguishes correct from incorrect. |
| EC-008 | Block-audit event emission races with dispatcher exit (timing concern) | The emit-before-exit invariant (Invariant 4) ensures the FileSink write call returns before exit code 2 is returned. No race: `emit_event` is synchronous in the single-threaded dispatcher model (per ADR-008). |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Plugin returns `HookResult::Block` on `Bash` tool | `vsdd.block.plugin_blocked.v1` in `events-*.jsonl` with `event.category = "audit"`, `outcome = "blocked"`, `plugin.name` set, `hook.tool_name = "Bash"`; dispatcher exits with code 2 | happy-path-block-audit |
| Plugin returns `HookResult::Allow` | No `vsdd.block.plugin_blocked.v1` in `events-*.jsonl`; dispatcher exits with code 0 | no-block-no-event |
| `FileSink::write` fails on block-audit event | Fallback write to `dispatcher-internal-*.jsonl`; stderr warning of form `[vsdd-dispatcher] WARN: FileSink write failed...`; exit code still 2 | write-failure-block-audit |
| **Misimplementation distinguisher:** dispatcher exits with code 2 without emitting event | Test MUST assert `vsdd.block.plugin_blocked.v1` appears in `events-*.jsonl`. Pre-Wave-1 behavior (code 2 only, no event) fails this assertion. | misimplementation-witness-no-block-event |
| Block-audit event fields inspection | Event has all 15 Resource fields (per BC-1.12.003) + all per-event host-stamped fields (per BC-1.12.004) + `event.category = "audit"` | fields-completeness |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| (TBD — Phase 1.6b) | `HookResult::Block` always produces `vsdd.block.plugin_blocked.v1` in `events-*.jsonl` | integration test: run plugin returning `Block`; assert event present |
| (TBD) | `event.category = "audit"` on block-audit events | unit test: verify registry lookup for `vsdd.block.*` prefix returns `"audit"` |
| (TBD) | Emit-before-exit invariant: event written before exit code 2 returned | integration test: verify `events-*.jsonl` has block-audit event; process exit code = 2 |
| (TBD) | `outcome = "blocked"` and `plugin.name` present on block-audit event | unit test: assert field values on emitted event |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-029 ("Emit structured events to a single observability stream (file path)") per capabilities.md §CAP-029 |
| Capability Anchor Justification | CAP-029 ("Emit structured events to a single observability stream (file path)") per capabilities.md §CAP-029. BC-1.12.006 governs single-stream emission of `vsdd.block.plugin_blocked.v1` audit events when a plugin returns HookResult::Block. Per CAP-029, every dispatched hook event appears as a parseable JSONL line on the single observability stream — this BC specifies the audit-event subset of CAP-029's surface. The BC's Postconditions 1–7 are entirely about FileSink emission path, field stamping, registry-derived `event.category=audit`, and emit-before-exit ordering — all single-stream-emission concerns that fall squarely within CAP-029's capability boundary. |
| Secondary Capability Reference | CAP-008 ("Gate tool calls with pre-execution behavioral checks (PreToolUse hooks)") per capabilities.md §CAP-008. CAP-008 is the upstream gating capability whose decisions this BC's audit events observe. CAP-008 produces the `HookResult::Block` decision; CAP-029 (via this BC) emits the audit trail of that decision on the single observability stream. The two capabilities together close the auditability loop for PreToolUse hook gating: CAP-008 enforces the block; BC-1.12.006 (under CAP-029) makes the enforcement observable and auditable. |
| L2 Domain Invariants | TBD |
| Architecture Module | SS-01 — `crates/factory-dispatcher/src/main.rs` (`HookResult::Block` branch; audit event emission before exit) |
| Stories | S-10.04 (Wave 1: Trace propagation + lifecycle event types; block audit trail) |
| Epic | E-10 (Single-stream OTel-aligned event emission) |
| ADR | ADR-015 D-15.3 ("When a plugin returns `HookResult::Block`, the dispatcher emits a `vsdd.block.plugin_blocked.v1` event with `outcome=blocked`, `plugin.name`, and `hook.tool_name` before exiting. Block path now has an audit trail.") |

### Purity Classification

| Property | Assessment |
|----------|-----------|
| I/O operations | YES — FileSink write to `events-*.jsonl`; optional fallback write; stderr warning |
| Global state access | YES — reads HostContext (plugin.name, hook.tool_name, session context) |
| Deterministic | YES given fixed plugin invocation context |
| Thread safety | YES — single-threaded dispatcher (per ADR-008) |
| Overall classification | Effectful shell (file write + process exit; emit-before-exit ordering invariant is the key correctness constraint) |

### TD-VSDD-092 (BC-SOUL4-coverage) Verification

Block-path audit emission source-walk:

- The `HookResult::Block` branch in `main.rs` currently calls
  `std::process::exit(2)` directly (pre-Wave-1 behavior). The Wave 1
  implementation MUST insert the `emit_event` call BEFORE `std::process::exit(2)`.
  No `let _ =` discard permitted on the `emit_event` call at this site.
- `FileSink::write` failure on the block-audit event: the failure cascade per
  BC-1.11.002 is a synchronous call chain that must complete before
  `std::process::exit(2)`. Invariant 4 (emit-before-exit) is the SOUL #4 guard:
  an implementation that `let _ = emit_event(...)` and then exits may silently
  discard a block-audit event if the FileSink is slow. This is prohibited.
