---
document_type: behavioral-contract
level: L3
version: "v1.1"
status: draft
producer: product-owner
timestamp: 2026-04-28T00:00:00
phase: 1a
inputs:
  - .factory/stories/S-5.03-worktree-hooks.md
  - .factory/specs/domain-spec/capabilities.md
input-hash: "0b97a0a"
traces_to: .factory/specs/prd.md#FR-046
origin: greenfield
extracted_from: null
subsystem: "SS-04"
capability: "CAP-002"
lifecycle_status: active
introduced: v1.0.0-rc.1
modified: [v1.1-adv-s5.03-p01]
deprecated: null
deprecated_by: null
replacement: null
retired: null
removed: null
removal_reason: null
---

# BC-4.07.002: worktree-hooks plugin emits worktree.removed event with {worktree_path} on WorktreeRemove event

## Description

When the dispatcher routes a `WorktreeRemove` event to the `worktree-hooks.wasm` plugin via the `hooks.json.template` + `hooks-registry.toml` dual-layer registration, the plugin emits a `worktree.removed` event via the `emit_event` host function. One field is set by the plugin: `worktree_path`, sourced from the incoming `WorktreeRemove` envelope. Eight additional fields are reserved and NOT settable by the plugin (RESERVED_FIELDS), set by the host in three sub-groups: (a) 4 host-enriched from HostContext by `emit_event`: `dispatcher_trace_id`, `session_id`, `plugin_name`, `plugin_version`; (b) 3 enriched by `emit_event` from `InternalEvent::now()`: `ts`, `ts_epoch`, `schema_version`; (c) 1 set at construction from the `emit_event` type argument: `type`. Per HOST_ABI.md (authoritative production contract). Total fields on wire: 9. The plugin performs NO filesystem writes, NO subprocess invocations, and requires ZERO declared capabilities (Option A zero-capability scoping — same as BC-4.07.001). WorktreeRemove is the cleanup complement to WorktreeCreate; the plugin emits the event regardless of whether the worktree was previously registered (unknown-worktree no-op per EC-002).

## Preconditions

1. A `WorktreeRemove` event has arrived at the dispatcher.
2. `hooks.json` (generated from `hooks.json.template`) contains a `WorktreeRemove` entry routing to the dispatcher binary, which then routes to `worktree-hooks.wasm` via `hooks-registry.toml`.
3. The `worktree-hooks.wasm` plugin is loaded in the dispatcher's `PluginCache`.
4. The incoming `WorktreeRemove` envelope contains `worktree_path` (absolute path string) identifying the removed worktree.

## Postconditions

1. The plugin invokes `emit_event` exactly once with `event_name = "worktree.removed"`.
2. The emitted payload contains all required fields. Fields are categorized by who sets them:

   **Plugin-set fields (1 field — the plugin sets this via `emit_event` key/value pair):**
   - `worktree_path` (string): absolute path to the removed worktree, sourced from the envelope's `worktree_path` field. If absent from the envelope, `worktree_path = ""` (empty string default). Value is always a string on the wire (per `emit_event.rs:49` string coercion).

   **Host-enriched fields (4 fields — set by `emit_event` host fn from `HostContext`, NOT by the plugin):** `dispatcher_trace_id`, `session_id`, `plugin_name`, `plugin_version`. Each is a non-empty string per BC-1.05.012 unconditional enrichment.

   **Host-enriched fields from `InternalEvent::now()` (3 fields):** `ts`, `ts_epoch`, `schema_version`. Set by `emit_event` internally. Part of `RESERVED_FIELDS`; plugin attempts to set them are silently dropped.

   **Construction-time field (1 field):** `type`. Set from the `emit_event` type argument. `type` MUST equal `"worktree.removed"`. Part of `RESERVED_FIELDS`.

   **Authoritative source for RESERVED_FIELDS split:** HOST_ABI.md §emit_event. The 8 RESERVED_FIELDS = 4 HostContext-enriched + 3 InternalEvent::now() + 1 type-argument.

   **Wire format note:** All plugin-set field values are strings on the wire (`emit_event.rs:49` coercion). Downstream consumers MUST parse string values back to their semantic types.

   **Total wire fields: 9** (1 plugin-set + 4 host-enriched + 4 construction-time). WorktreeRemove is the smallest lifecycle event payload in the Tier F family (one less field than WorktreeCreate, which has both `worktree_path` and `worktree_name`).

3. The plugin returns `HookResult::Ok` (exit code 0) to the dispatcher, regardless of whether the worktree identified by `worktree_path` was previously registered in any observability configuration (unknown-worktree no-op — EC-002).

## Invariants

1. The plugin performs NO filesystem writes. No `write_file` host fn exists in HOST_ABI v1.0. Any deregistration of sink config for the removed worktree is a v1.1 concern (BC-4.07.005 or a new BC-4.07.006 deregistration candidate).
2. The plugin performs NO subprocess invocations. `exec_subprocess` is NOT declared in BC-4.07.004's `hooks-registry.toml` entry.
3. `worktree_path` is never absent from the emitted payload — it defaults to `""` when the envelope field is absent.
4. The plugin emits `worktree.removed` unconditionally, regardless of whether it recognizes the `worktree_path` value. Unknown-worktree paths produce a valid event (EC-002 — no-op from the plugin's perspective).
5. The `worktree.removed` event-name literal is immutable and reserved per PRD FR-046.
6. `emit_event` is called before the plugin function returns.
7. The plugin is unconditionally stateless — it maintains no in-process state across invocations.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | WorktreeRemove event fires multiple times for the same worktree_path (e.g., reconnect scenario) | Plugin is unconditionally stateless; emits `worktree.removed` on every invocation it receives. `once: false` (or absent) in `hooks.json.template` means no Layer 1 dedup. Multiple `worktree.removed` events for the same path are operator-observable. |
| EC-002 | WorktreeRemove for a worktree_path not previously registered (unknown worktree) | Plugin emits `worktree.removed` event normally. The plugin has no registry of known worktrees — it cannot distinguish known from unknown paths. The observability stack consuming the event is responsible for handling unknown-path removal gracefully (log warning, no-op). This is NOT a plugin error condition. |
| EC-003 | `worktree_path` is absent from the `WorktreeRemove` envelope | `worktree_path = ""` in the emitted `worktree.removed` event; plugin does not abort; emits normally. Consumer must handle empty `worktree_path` on remove. |
| EC-004 | `session_id` is missing or empty in the `WorktreeRemove` envelope | BC-1.02.005 lifecycle-tolerance sets `HostContext.session_id = "unknown"`; `emit_event` auto-enriches the event with this value; plugin emits normally. |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| `WorktreeRemove` envelope with `worktree_path = "/workspace/feat-branch"`, `session_id = "wt-sess-001"`, dispatcher routes to worktree-hooks.wasm | `worktree.removed` emitted once; `worktree_path = "/workspace/feat-branch"` (string on wire); `session_id = "wt-sess-001"` (host-enriched); `dispatcher_trace_id` non-empty string; `plugin_name` non-empty string; `plugin_version` non-empty string; `type = "worktree.removed"`; total 9 fields; `exec_subprocess` CountingMock invocation_count == 0 | happy-path |
| `WorktreeRemove` envelope with `worktree_path = "/workspace/unknown-path"` (path never registered) | `worktree.removed` emitted once with `worktree_path = "/workspace/unknown-path"`; plugin does not error; returns `HookResult::Ok` | edge-case (unknown worktree, EC-002) |
| `WorktreeRemove` envelope with `worktree_path` absent | `worktree.removed` emitted once with `worktree_path = ""`; plugin does not abort | edge-case (missing field, EC-003) |
| Two consecutive `WorktreeRemove` events with same `worktree_path` | Two `worktree.removed` events emitted (no Layer 1 dedup); each event has correct 9-field payload | edge-case (idempotent re-fire, EC-001) |

## Notes

**Unknown-worktree no-op (EC-002):** The plugin has no notion of "registered worktrees." It does not read any configuration file (no `read_file` capability declared), so it cannot check whether a removed worktree was previously registered. It simply emits `worktree.removed` with whatever `worktree_path` the envelope provides. The observability stack downstream is responsible for interpreting this event gracefully when the path is unexpected.

**9-field payload vs. 10-field (WorktreeCreate):** WorktreeRemove omits `worktree_name` — the name is not needed on removal (the path is sufficient to identify which worktree was removed). This asymmetry is intentional: name information is relevant at creation time for labeling/routing; at removal time only the path is needed for deregistration.

**Option A scoping (same as BC-4.07.001):** All Option A/B/C rationale from BC-4.07.001 applies here verbatim. WorktreeRemove has no additional filesystem write requirements — it is a pure event emission.

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-067 | Worktree Hook Plugin Surface Invariant — All BC-4.07.* Postconditions Hold in Integration Test | integration |

## Related BCs

- **BC-4.07.001** — composes with (WorktreeCreate counterpart; both are internal dispatch paths in worktree-hooks.wasm; this BC is the Remove path, BC-4.07.001 is the Create path)
- **BC-4.07.003** — depends on (hooks.json.template WorktreeRemove registration triggers this plugin)
- **BC-4.07.004** — depends on (hooks-registry.toml WorktreeRemove routing entry provides dispatcher-side routing to worktree-hooks.wasm)
- **BC-1.02.005** — depends on (dispatcher envelope parsing delivers `session_id` to this plugin via HostContext)
- **BC-1.05.012** — depends on (emit_event host fn auto-enriches with host-enriched fields)
- **BC-4.05.002** — structural analog (SessionEnd no-subprocess pattern; WorktreeRemove follows the same zero-capability profile)

## Architecture Anchors

- SS-04 — `crates/hook-plugins/worktree-hooks/src/lib.rs` (plugin `on_hook` entry point + `emit_event` call for WorktreeRemove dispatch path)
- SS-01 — dispatcher routes `WorktreeRemove` to `worktree-hooks.wasm` per `hooks-registry.toml` routing entry (BC-4.07.004)

## Story Anchor

S-5.03

## VP Anchors

VP-067

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-002 |
| Capability Anchor Justification | CAP-002 ("Hook Claude Code tool calls with sandboxed WASM plugins") per capabilities.md §CAP-002 |
| L2 Domain Invariants | DI-007 **REMOVED** (DI-007 is "Dispatcher self-telemetry is always-on" — scoped to dispatcher-internal-YYYY-MM-DD.jsonl and SS-03 internal_log.rs; does NOT govern plugin-emitted events. No current DI for plugin event emission unconditionally; v1.1 candidate per PRD §S-5.03 flag.); DI-017 (dispatcher_trace_id on every emitted event — automatically enriched by emit_event host fn from HostContext) |
| Architecture Module | SS-04 — `crates/hook-plugins/worktree-hooks/src/lib.rs` |
| Stories | S-5.03 |
| Functional Requirement | FR-046 |

## Changelog

| Version | Date | Author | Change |
|---------|------|--------|--------|
| v1.1 | 2026-04-28 | product-owner | Pass-1 fix burst ADV-S5.03-P01: (HIGH-003) RESERVED_FIELDS split corrected from 4-vs-4 to 4-vs-3-vs-1 per HOST_ABI.md §emit_event; (HIGH-004) DI-007 removed — DI-007 is dispatcher self-telemetry scope (SS-03), not plugin event emission; replaced with "no current DI; v1.1 candidate" annotation |
| v1.0 | 2026-04-28 | product-owner | Initial creation (S-5.03 foundation burst) |
