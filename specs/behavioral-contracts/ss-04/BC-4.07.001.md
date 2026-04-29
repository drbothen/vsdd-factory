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

# BC-4.07.001: worktree-hooks plugin emits worktree.created event with {worktree_path, worktree_name} on WorktreeCreate event

## Description

When the dispatcher routes a `WorktreeCreate` event to the `worktree-hooks.wasm` plugin via the `hooks.json.template` + `hooks-registry.toml` dual-layer registration, the plugin emits a `worktree.created` event via the `emit_event` host function. Two fields are set by the plugin: `worktree_path` and `worktree_name`, sourced from the incoming `WorktreeCreate` envelope. Eight additional fields are reserved and NOT settable by the plugin (RESERVED_FIELDS), set by the host in three sub-groups: (a) 4 host-enriched from HostContext by `emit_event`: `dispatcher_trace_id`, `session_id`, `plugin_name`, `plugin_version`; (b) 3 enriched by `emit_event` from `InternalEvent::now()`: `ts`, `ts_epoch`, `schema_version`; (c) 1 set at construction from the `emit_event` type argument: `type`. Per HOST_ABI.md (authoritative production contract). Total fields on wire: 10. The plugin performs NO filesystem writes, NO subprocess invocations, and requires ZERO declared capabilities — it reads all required data from the incoming envelope. This is the Option A (zero-capability) scoping decision applied to WorktreeCreate.

## Scoping Decision: Option A (Zero-Capability) — Rationale

**Option A (SELECTED):** The plugin emits `worktree.created` ONLY. No filesystem auto-registration, no subprocess invocation. The observability stack consumes the event at runtime — no plugin-side filesystem mutation required. This mirrors the S-5.02 zero-capability pattern (BC-4.05.001–005) exactly.

**Option B (REJECTED):** Use `exec_subprocess` to invoke a config-write CLI tool (`factory-worktree-register`). Adds capability scope creep; hard to test deterministically; requires a new CLI binary. Violates S-5.01 lesson 3 (avoid bespoke host fns / shell-out tricks). Rejected.

**Option C (REJECTED):** Add a new `write_file` host fn to vsdd-hook-sdk. HOST_ABI bump (major). Out of scope for v1.0 (release-gate calendar). Verified at `crates/hook-sdk/HOST_ABI.md`: available host fns are `log`, `emit_event` (always), `read_file` (capability-gated), `exec_subprocess` (capability-gated), and zero-capability info getters (`session_id`, `dispatcher_trace_id`, `plugin_root`, `plugin_version`, `cwd`, `env`). No `write_file` host fn exists. Rejected.

**Deferred to v1.1:** BC-4.07.005 (worktree-config-write — filesystem sink config auto-generation via a future `write_file` host fn). Filesystem-write capability deferred to v1.1 — no CAP ID allocated yet; v1.1 capability registry expansion required (provisional name: `CAP-NNN-filesystem-write` to be assigned in v1.1).

## Preconditions

1. A `WorktreeCreate` event has arrived at the dispatcher.
2. `hooks.json` (generated from `hooks.json.template`) contains a `WorktreeCreate` entry routing to the dispatcher binary, which then routes to `worktree-hooks.wasm` via `hooks-registry.toml`.
3. The `worktree-hooks.wasm` plugin is loaded in the dispatcher's `PluginCache`.
4. The incoming `WorktreeCreate` envelope contains `worktree_path` (absolute path string) and optionally `worktree_name` (human-readable identifier string).

## Postconditions

1. The plugin invokes `emit_event` exactly once with `event_name = "worktree.created"`.
2. The emitted payload contains all required fields. Fields are categorized by who sets them:

   **Plugin-set fields (2 fields — the plugin sets these via `emit_event` key/value pairs):**
   - `worktree_path` (string): absolute path to the newly created worktree, sourced from the envelope's `worktree_path` field. If absent from the envelope, `worktree_path = ""` (empty string default per PC-3). Value is always a string on the wire (per `emit_event.rs:49` string coercion).
   - `worktree_name` (string): human-readable identifier for the worktree, sourced from the envelope's `worktree_name` field. If absent from the envelope, `worktree_name = ""` (empty string default). Value is always a string on the wire.

   **Host-enriched fields (4 fields — set by `emit_event` host fn from `HostContext`, NOT by the plugin):** `dispatcher_trace_id`, `session_id`, `plugin_name`, `plugin_version`. These are part of `RESERVED_FIELDS` and are silently dropped if the plugin attempts to set them. Each is a non-empty string per BC-1.05.012 unconditional enrichment.

   **Host-enriched fields from `InternalEvent::now()` (3 fields):** `ts`, `ts_epoch`, `schema_version`. Set by `emit_event` internally via `InternalEvent::now()`. Part of `RESERVED_FIELDS`; plugin attempts to set them are silently dropped.

   **Construction-time field (1 field):** `type`. Set from the `emit_event` type argument (`SinkEvent::new` with the `type` argument). `type` MUST equal `"worktree.created"`. Part of `RESERVED_FIELDS`; plugin attempt to set it is silently dropped.

   **Authoritative source for RESERVED_FIELDS split:** HOST_ABI.md §emit_event. The 8 RESERVED_FIELDS = 4 HostContext-enriched + 3 InternalEvent::now() + 1 type-argument.

   **Wire format note:** All plugin-set field values are strings on the wire (`emit_event.rs:49` coerces all plugin-supplied values to `Value::String`). Downstream consumers MUST parse string values back to their semantic types.

   **Total wire fields: 10** (2 plugin-set + 4 host-enriched + 4 construction-time). This is the minimum payload for a worktree lifecycle event. SessionStart emits 14 (6+4+4); SessionEnd emits 11 (3+4+4); WorktreeCreate emits 10 (2+4+4); WorktreeRemove emits 9 (1+4+4).

3. The plugin returns `HookResult::Ok` (exit code 0) to the dispatcher.

## Invariants

1. The plugin performs NO filesystem writes. There is no `write_file` host fn in HOST_ABI v1.0. Any filesystem configuration for the new worktree MUST be deferred to v1.1 (BC-4.07.005 candidate) or handled by the observability stack consuming the `worktree.created` event.
2. The plugin performs NO subprocess invocations. `exec_subprocess` is NOT declared in BC-4.07.004's `hooks-registry.toml` entry. Any invocation attempt would receive `CAPABILITY_DENIED` from the host fn dispatch table — deny-by-default enforced by BC-1.05.001 (exec_subprocess denied when no exec_subprocess capability declared) and BC-1.05.021 (read_file denied when no Capabilities.read_file block).
3. `worktree_path` is never absent from the emitted payload — it defaults to `""` when the envelope field is absent. This ensures consumers can always inspect the field without null-checks.
4. `worktree_name` is never absent from the emitted payload — it defaults to `""` when the envelope field is absent.
5. The `worktree.created` event-name literal is immutable and reserved per PRD FR-046; renaming requires a new BC.
6. `emit_event` is called before the plugin function returns.
7. The plugin is unconditionally stateless — it maintains no in-process state across invocations (idempotency contract, per EC-001).

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | WorktreeCreate event fires multiple times for the same worktree_path (e.g., on Claude Code reconnect after disconnect) | Plugin is unconditionally stateless; emits `worktree.created` on every invocation it receives. `once: false` (or absent) in `hooks.json.template` registration (BC-4.07.003) means Layer 1 does NOT deduplicate — this is intentional. Multiple `worktree.created` events for the same path are operator-observable. This differs from SessionStart/SessionEnd which use `once: true`. |
| EC-002 | `worktree_path` is absent from the `WorktreeCreate` envelope | `worktree_path = ""` in the emitted `worktree.created` event; plugin does not abort; emits normally. Consumer is responsible for handling empty `worktree_path`. |
| EC-003 | `worktree_name` is absent from the `WorktreeCreate` envelope | `worktree_name = ""` in the emitted `worktree.created` event; plugin does not abort; emits normally. |
| EC-004 | Both `worktree_path` and `worktree_name` are absent from the envelope | Both fields are `""` in the emitted event; plugin emits normally; consumer must handle empty values. |
| EC-005 | `session_id` is missing or empty in the `WorktreeCreate` envelope | BC-1.02.005 lifecycle-tolerance sets `HostContext.session_id = "unknown"`; `emit_event` auto-enriches the event with this value; plugin is unconditionally stateless; emits normally. |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| `WorktreeCreate` envelope with `worktree_path = "/workspace/feat-branch"`, `worktree_name = "feat-branch"`, `session_id = "wt-sess-001"`, dispatcher routes to worktree-hooks.wasm | `worktree.created` emitted once; `worktree_path = "/workspace/feat-branch"` (string on wire); `worktree_name = "feat-branch"` (string on wire); `session_id = "wt-sess-001"` (host-enriched); `dispatcher_trace_id` non-empty string (host-enriched); `plugin_name` non-empty string (host-enriched); `plugin_version` non-empty string (host-enriched); `type = "worktree.created"` (construction-time); total 10 fields; `exec_subprocess` CountingMock invocation_count == 0 | happy-path |
| `WorktreeCreate` envelope with `worktree_path` absent, `worktree_name` absent | `worktree.created` emitted once; `worktree_path = ""`, `worktree_name = ""`; host-enriched and construction-time fields present normally | edge-case (both absent) |
| `WorktreeCreate` envelope with `session_id = ""` (empty) | `worktree.created` emitted once; `session_id = "unknown"` (BC-1.02.005 sentinel); other fields emitted normally | edge-case (missing session_id) |
| Two consecutive `WorktreeCreate` events with same `worktree_path` (reconnect simulation) | Two `worktree.created` events emitted (once:false — no Layer 1 dedup for WorktreeCreate); each event has correct 10-field payload | edge-case (idempotent re-fire, EC-001) |

## Notes

**Structural note — single worktree-hooks.wasm crate for both events:** A single `crates/hook-plugins/worktree-hooks` crate handles both `WorktreeCreate` (this BC) and `WorktreeRemove` (BC-4.07.002) via internal dispatch on `event_name` field (the HookPayload envelope field per HOST_ABI.md and BC-1.02.001/002). Two separate `[[hooks]]` entries in `hooks-registry.toml` (BC-4.07.004) both point to `hook-plugins/worktree-hooks.wasm`. This reduces binary count (one .wasm instead of two) while maintaining clean dispatch-path separation internally.

**Option A scoping note (proactive, mirrors S-5.01 pass-4 architectural reversal):** The v1.0 legacy story wanted filesystem auto-registration (writing sink config for the new worktree). This BC explicitly excludes that behavior because HOST_ABI v1.0 has no `write_file` host fn. This is the same class of decision as S-5.01 pass-4 reversal (over-engineered dispatcher-side dedup dropped in favor of Layer 1 `once:true`), but applied proactively here before adversarial review rather than as a reaction to it. BC-4.07.005 is the deferred candidate for filesystem config write when `write_file` becomes available in HOST_ABI v1.1.

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-067 | Worktree Hook Plugin Surface Invariant — All BC-4.07.* Postconditions Hold in Integration Test | integration |

## Related BCs

- **BC-4.07.002** — composes with (worktree-hooks.wasm handles WorktreeRemove; both are internal dispatch paths in the same plugin; the plugin dispatch logic selects BC-4.07.001 path vs. BC-4.07.002 path based on event_name)
- **BC-4.07.003** — depends on (hooks.json.template WorktreeCreate registration triggers this plugin via dispatcher routing)
- **BC-4.07.004** — depends on (hooks-registry.toml WorktreeCreate routing entry provides dispatcher-side routing to worktree-hooks.wasm)
- **BC-1.02.005** — depends on (dispatcher envelope parsing delivers `session_id` to this plugin via HostContext)
- **BC-1.05.012** — depends on (emit_event host fn auto-enriches with host-enriched fields including session_id, dispatcher_trace_id, plugin_name, plugin_version)
- **BC-4.05.001** — structural analog (SessionEnd event emission pattern; WorktreeCreate is simpler with only 2 plugin-set fields vs. 3 for SessionEnd)
- **BC-4.04.001** — structural analog (SessionStart event emission pattern; canonical reference for the Tier F BC family shape)

## Architecture Anchors

- SS-04 — `crates/hook-plugins/worktree-hooks/src/lib.rs` (plugin `on_hook` entry point + `emit_event` call for WorktreeCreate path)
- SS-01 — dispatcher routes `WorktreeCreate` to `worktree-hooks.wasm` per `hooks-registry.toml` routing entry (BC-4.07.004)

## Story Anchor

S-5.03

## VP Anchors

VP-067

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-002 |
| Capability Anchor Justification | CAP-002 ("Hook Claude Code tool calls with sandboxed WASM plugins") per capabilities.md §CAP-002 |
| L2 Domain Invariants | DI-007 **REMOVED** (DI-007 is "Dispatcher self-telemetry is always-on" — scoped to dispatcher-internal-YYYY-MM-DD.jsonl and enforced by SS-03 internal_log.rs; it does NOT govern plugin-emitted events. Replaced by: no current DI for plugin event emission unconditionally; this is a v1.1 candidate per PRD §S-5.03 flag. Plugin-emitted events are always-on by convention, but no DI formalizes this at v1.0.); DI-017 (dispatcher_trace_id on every emitted event — automatically enriched by emit_event host fn from HostContext; not the plugin's responsibility to set); BC-1.02.005 (lifecycle-tolerant envelope parsing populates HostContext.session_id used by emit_event auto-enrichment; "unknown" sentinel set at envelope-parse layer, not by the plugin) |
| Architecture Module | SS-04 — `crates/hook-plugins/worktree-hooks/src/lib.rs` |
| Stories | S-5.03 |
| Functional Requirement | FR-046 |

## Changelog

| Version | Date | Author | Change |
|---------|------|--------|--------|
| v1.1 | 2026-04-28 | product-owner | Pass-1 fix burst ADV-S5.03-P01: (CRIT-001) CAP-003 parenthetical removed — filesystem-write capability deferred to v1.1 with no CAP ID; (CRIT-002) BC-1.05.022 deny-by-default re-anchored to correct pair BC-1.05.001+BC-1.05.021; (CRIT-003) event_type → event_name (HookPayload envelope field per HOST_ABI.md); (HIGH-003) RESERVED_FIELDS split corrected from 4-vs-4 to 4-vs-3-vs-1 per HOST_ABI.md §emit_event (authoritative production contract); (HIGH-004) DI-007 removed — DI-007 is dispatcher self-telemetry (SS-03 internal_log.rs scope), not plugin event emission; replaced with "no current DI for plugin event emission; v1.1 candidate" annotation |
| v1.0 | 2026-04-28 | product-owner | Initial creation (S-5.03 foundation burst) |
