---
document_type: behavioral-contract
level: L3
version: "1.0"
status: draft
producer: architect
timestamp: 2026-05-06T00:00:00Z
phase: 1c
inputs:
  - .factory/specs/architecture/decisions/ADR-015-single-stream-otel-schema.md
  - .factory/stories/S-10.05-adr015-wave2-plugin-schema-migration.md
input-hash: "[pending-recompute]"
traces_to: ADR-015-single-stream-otel-schema.md
origin: spec-revision
subsystem: "SS-04"
capability: "CAP-009"
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

# Behavioral Contract BC-4.09.001: hook-plugins::event_naming::wave2_reverse_dns_event_name_migration_with_dual_emit — all native WASM plugins under crates/hook-plugins/ migrate event-name strings from legacy short-form to reverse-DNS .v1 canonical form; dual-emit shim active during Wave 2→Wave 3 window; legacy emission removed post-Wave-3

## Description

ADR-015 D-15.2 mandates that all event names emitted by the dispatcher ecosystem
follow the reverse-DNS + `.vN` canonical form (e.g.,
`vsdd.plugin.capture-pr-activity.pr_opened.v1`). Prior to Wave 2, native WASM
plugins under `crates/hook-plugins/*/` emitted legacy short-form event names
(e.g., `pr_opened`, `session.started`, `tool.error`).

This BC governs the **plugin-side migration contract** for all 11 native WASM
plugins during Wave 2: each plugin's source code is updated to emit the
reverse-DNS canonical event name. During the Wave 2 → Wave 3 dual-emit window,
plugins call `vsdd_hook_sdk::host::emit_pair` (per BC-1.11.003) to atomically
emit both the legacy short-form event AND the new reverse-DNS canonical event as
a correlated pair. This ensures consumers that have NOT yet migrated to the new
event names continue to receive the old names while the new names become
available in parallel.

At Wave 3 (shim removal), the legacy emission is removed from each plugin. From
Wave 3 onward, plugins call `emit_event` directly with only the reverse-DNS
canonical name. Post-Wave-3, any reference to the legacy short-form name in a
plugin's source code is a compile error (the SDK no longer exports the shim
path after Wave 3 cleanup per BC-1.11.003).

## Scope

This BC covers the 11 native WASM plugins under `crates/hook-plugins/`:
- `capture-pr-activity`
- `capture-commit-activity`
- `session-start-telemetry`
- `session-end-telemetry`
- `worktree-hooks`
- `tool-failure-hooks`
- Any additional plugins added to `crates/hook-plugins/` before Wave 2 lands.

It does NOT cover: bash hooks in `hooks/` (SS-07 scope), the dispatcher itself
(SS-01 scope), or SDK internals (SS-02 scope).

## Preconditions

1. Wave 2 SDK is released with `emit_pair` host binding available
   (BC-1.11.003 implemented; BC-2.06.001 MAJOR version bump complete).
2. Each plugin currently emits legacy short-form event names in its `on_hook`
   implementation.
3. The reverse-DNS canonical names for each plugin's events are defined in the
   compile-time event registry (per BC-1.12.004).

## Postconditions

### Wave 2 state (dual-emit window)

1. Each plugin's `on_hook` implementation replaces its direct `emit_event(legacy_name, ...)`
   call with a `emit_pair(legacy_name, legacy_fields, new_reverse_dns_name, new_fields)`
   call per BC-1.11.003's `emit_pair` API surface.
2. The new reverse-DNS canonical name for each event follows the pattern:
   `vsdd.plugin.<plugin-crate-name>.<event-concept>.v1`
   Example mappings:
   - `pr_opened` → `vsdd.plugin.capture-pr-activity.pr_opened.v1`
   - `session.started` → `vsdd.plugin.session-start-telemetry.session_started.v1`
   - `session.ended` → `vsdd.plugin.session-end-telemetry.session_ended.v1`
   - `tool.error` → `vsdd.plugin.tool-failure-hooks.tool_error.v1`
   - `worktree.created` → `vsdd.plugin.worktree-hooks.worktree_created.v1`
   - `worktree.removed` → `vsdd.plugin.worktree-hooks.worktree_removed.v1`
   - `commit.made` → `vsdd.plugin.capture-commit-activity.commit_made.v1`
   (Complete mapping table is defined in the Wave 2 story S-10.05 File Structure
   Requirements; this BC references the canonical pattern, not the exhaustive list.)
3. The `emit_pair` call atomically assigns a shared `event.correlation_id`,
   sets `old_event.event.deprecated_by = new_event.event.id`, and sets
   `new_event.event.replaces_deprecated_alias = old_event.event.id` — all
   per BC-1.11.003 host semantics. Plugin code does NOT set these correlation
   fields directly.
4. Both the legacy short-form event AND the new reverse-DNS event appear in
   `events-*.jsonl` as a correlated pair for every plugin invocation during
   Wave 2.

### Wave 3 state (post-shim-removal)

5. Each plugin's `on_hook` implementation contains ONLY `emit_event(new_reverse_dns_name, ...)`
   calls. The `emit_pair` call and any direct legacy-name references are removed.
6. If any plugin source still contains the legacy short-form name as a string
   literal in an `emit_event` call after Wave 3, the event-name registry
   validation at compile time (per BC-1.12.004) will emit a compile-time warning
   or error (the legacy name is no longer registered). This serves as the
   compile-error migration forcing function.

### Special cases (AC-005, AC-006 from S-10.05)

7. The `capture-pr-activity` plugin fixes `plugin_version` field: the emitted
   event payload MUST include `plugin_version` set to the plugin crate's
   `CARGO_PKG_VERSION` (compile-time constant), not a hardcoded or empty string.
8. The `capture-pr-activity` plugin adds `open_to_merge_seconds` field emission:
   when `pr.merged` event is emitted (or its reverse-DNS equivalent), the payload
   MUST include `open_to_merge_seconds` computed as the duration in seconds between
   PR open timestamp and PR merge timestamp, as a non-negative integer.

## Invariants

1. After Wave 2 lands, no plugin's `on_hook` implementation emits a legacy
   short-form event name via `emit_event` without also emitting the corresponding
   reverse-DNS canonical name via `emit_pair`. The dual-emit invariant holds for
   the entire Wave 2 → Wave 3 window.
2. After Wave 3 lands, no plugin's `on_hook` implementation contains a reference
   to any legacy short-form event name string. Legacy names are absent from all
   plugin source files under `crates/hook-plugins/`. This is verifiable by grep:
   `grep -rn '"pr_opened"\|"session\.started"\|"session\.ended"\|"tool\.error"\|"worktree\.created"\|"worktree\.removed"\|"commit\.made"' crates/hook-plugins/`
   returns zero matches in production source files (excluding test files that
   explicitly test the migration).
3. The `plugin_version` field (AC-005) is ALWAYS present in `capture-pr-activity`
   plugin emissions from Wave 2 onward. A missing or empty `plugin_version` field
   is a postcondition violation.
4. The `open_to_merge_seconds` field (AC-006) is ALWAYS present in
   `capture-pr-activity` `pr.merged` (and `vsdd.plugin.capture-pr-activity.pr_merged.v1`)
   emissions from Wave 2 onward.

## Related BCs

- BC-1.11.003 — emit_pair host helper (depends on: the Wave 2 dual-emit mechanism;
  this BC governs the plugin-side usage of that helper)
- BC-2.06.001 — vsdd-hook-sdk wave2 MAJOR semver bump (companion: the SDK version
  bump is the prerequisite for plugin authors to have access to `emit_pair`; this
  BC governs what the 11 native plugins DO with that new SDK version)
- BC-1.12.004 — per-event host-stamping and event.category registry (depends on:
  the reverse-DNS event names must be registered in the compile-time registry for
  correct `event.category` derivation)
- BC-1.12.009 — dual-emit pair identity contract (consumer side; the five-state
  classification this BC's dual-emit output is consumed by)
- BC-1.12.005 — host_field_override contract (sibling: if any plugin sets a
  host-owned field, the override semantics apply; plugin migration should remove
  such fields)

## Architecture Anchors

- `crates/hook-plugins/*/src/lib.rs` — the `on_hook` function in each plugin;
  the `emit_event` → `emit_pair` migration site for each of the 11 plugins
- `crates/hook-plugins/capture-pr-activity/src/lib.rs` — additional fixes:
  `plugin_version` field and `open_to_merge_seconds` computation
- `crates/hook-sdk/src/host.rs` — `emit_pair` ABI surface consumed by plugins
- ADR-015 D-15.2 — reverse-DNS event naming convention; canonical form mandate
- ADR-015 D-15.2.e — dual-emit pair identity contract (correlation fields;
  `emit_pair` enforces this on the host side)

## Story Anchor

S-10.05 (Wave 2: Plugin schema migration — reverse-DNS event name migration for
all 11 native WASM plugins; dual-emit shim via `emit_pair` per BC-1.11.003;
`plugin_version` fix per AC-005; `open_to_merge_seconds` addition per AC-006)

## VP Anchors

(TBD — to be assigned after S-10.05 story authoring completes Phase 1c)

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Plugin emits legacy name via `emit_event` in Wave 2 (missed migration) | One event in `events-*.jsonl` with legacy short-form name only; no `event.deprecated_by` / `event.replaces_deprecated_alias` cross-reference; consumer receives orphaned single event; migration gap flagged in audit |
| EC-002 | `emit_pair` fails on first write (old-name write fails) | New-name write NOT attempted per BC-1.11.003 Postcondition 5; no orphan created; failure cascade per BC-1.11.002; plugin receives error return |
| EC-003 | `emit_pair` fails on second write (new-name write fails) | `vsdd.internal.emit_pair_partial_failure.v1` emitted to debug file; old-name event is orphaned; stderr warning per BC-1.11.003 Postcondition 4; plugin receives error return |
| EC-004 | Post-Wave-3 plugin still references legacy name string | Compile-time event-name validation emits warning/error; legacy name not in registry; this is the migration forcing function |
| EC-005 | `capture-pr-activity` `pr.merged` event missing `open_to_merge_seconds` | Postcondition violation (Postcondition 8 / Invariant 4); test MUST assert field presence and non-negative integer type |
| EC-006 | Plugin emits `plugin_version = ""` (empty string) | Postcondition violation (Postcondition 7 / Invariant 3); test MUST assert `plugin_version` is a non-empty semver string matching `[0-9]+\.[0-9]+\.[0-9]+` |
| EC-007 | Dashboard query for `pr_opened` after Wave 3 shim removal | Zero results returned; consumer must migrate query to `vsdd.plugin.capture-pr-activity.pr_opened.v1`; this is an expected migration forcing function for consumers |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Wave 2: trigger `capture-pr-activity` on PR open event | Two events in `events-*.jsonl`: one with `event.name = "pr_opened"` (legacy) and one with `event.name = "vsdd.plugin.capture-pr-activity.pr_opened.v1"` (canonical); both share `event.correlation_id`; legacy event has `event.deprecated_by` set; canonical event has `event.replaces_deprecated_alias` set | wave2-dual-emit-pr-opened |
| Wave 2: trigger `session-start-telemetry` | Two events: `session.started` + `vsdd.plugin.session-start-telemetry.session_started.v1`; correlated pair | wave2-dual-emit-session-start |
| `capture-pr-activity` `pr.merged` event in Wave 2 | Canonical event includes `open_to_merge_seconds` as non-negative integer; `plugin_version` present and non-empty | pr-merged-open-to-merge-seconds |
| `capture-pr-activity` any event in Wave 2 | `plugin_version` field present in payload; value matches plugin `CARGO_PKG_VERSION` | plugin-version-field-present |
| Wave 3: trigger any plugin after shim removal | Single event with reverse-DNS canonical name only; no legacy short-form name; no `event.deprecated_by` cross-reference | wave3-single-emit-canonical-only |
| **Misimplementation distinguisher (Wave 2):** plugin emits only canonical name (skips dual-emit) | Test MUST assert BOTH events present in `events-*.jsonl` for the same invocation; single-event-only output fails this assertion | misimplementation-witness-no-dual-emit |
| **Misimplementation distinguisher (Wave 3):** legacy name still present in source | `grep -rn '"pr_opened"' crates/hook-plugins/capture-pr-activity/src/lib.rs` returns zero matches in production `on_hook` implementation | misimplementation-witness-legacy-name-survives |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| (TBD — Phase 1c) | All 11 plugins emit reverse-DNS canonical event name via `emit_pair` in Wave 2 | integration test: trigger each plugin; assert both legacy + canonical events present in `events-*.jsonl` with correlation fields |
| (TBD) | Post-Wave-3: zero legacy short-form names in plugin source | static analysis: grep across `crates/hook-plugins/*/src/lib.rs` for legacy name strings; assert zero matches |
| (TBD) | `capture-pr-activity` `plugin_version` always present and non-empty | unit test: verify `plugin_version` field in emitted payload equals `CARGO_PKG_VERSION` constant |
| (TBD) | `capture-pr-activity` `open_to_merge_seconds` present on `pr_merged` events | unit test: construct `pr.merged` scenario; assert `open_to_merge_seconds` present as non-negative integer |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-009 ("Author and publish WASM hook plugins using the Rust SDK") per capabilities.md §CAP-009 |
| Capability Anchor Justification | CAP-009 ("Author and publish WASM hook plugins using the Rust SDK") per capabilities.md §CAP-009. BC-4.09.001 governs the migration of the 11 native WASM plugins' event-emission code from legacy short-form names to reverse-DNS canonical names, using the Wave 2 `emit_pair` SDK host binding. CAP-009 describes the SDK as the interface through which plugin authors implement hook behavior: "The `vsdd-hook-sdk` crate provides the `#[hook]` macro, `HookPayload`, `HookResult`, and all `vsdd::*` host function bindings. A third-party plugin author can add a dependency and ship a `.wasm` without touching the dispatcher" (capabilities.md §CAP-009). This BC governs how those `vsdd::*` host function bindings — specifically `emit_pair` and `emit_event` — are used in the 11 first-party WASM plugins during the Wave 2 migration. The event-name migration (Postconditions 1–6) and plugin-specific fixes (Postconditions 7–8) are entirely within the plugin-authoring surface that CAP-009 defines. |
| Secondary Capability Reference | CAP-029 ("Emit structured events to a single observability stream (file path)") per capabilities.md §CAP-029. Plugin emissions ultimately appear on the single CAP-029 stream; the reverse-DNS canonical event names emitted by this BC appear in `events-*.jsonl` per CAP-029's single-stream guarantee. CAP-029 is a secondary reference here because the primary concern of this BC is the plugin-authoring migration (CAP-009 surface), not the dispatcher's FileSink routing (CAP-029 surface). |
| L2 Domain Invariants | No domain invariants directly enforced. The `emit_pair` atomicity and single-stream routing are governed by BC-1.11.003 and BC-1.12.001 respectively; this BC governs plugin-side migration behavior. |
| Architecture Module | SS-04 — `crates/hook-plugins/*/src/lib.rs` (all 11 plugin crates); `crates/hook-plugins/capture-pr-activity/src/lib.rs` (additional `plugin_version` + `open_to_merge_seconds` fixes) |
| Stories | S-10.05 (Wave 2: Plugin schema migration — reverse-DNS event name migration, dual-emit shim, plugin_version fix, open_to_merge_seconds addition) |
| Epic | E-10 (Single-stream OTel-aligned event emission) |
| ADR | ADR-015 D-15.2 (reverse-DNS event naming convention; `.v1` suffix mandate); ADR-015 D-15.2.e (dual-emit pair identity contract; `emit_pair` correlation fields) |

### Purity Classification

| Property | Assessment |
|----------|-----------|
| I/O operations | YES — `emit_pair` / `emit_event` host calls from within each plugin; these write to `events-*.jsonl` via the FileSink path |
| Global state access | YES — reads plugin-scoped state (plugin_version, invocation context); reads event payload from dispatcher |
| Deterministic | YES given fixed plugin invocation context; `open_to_merge_seconds` computation is deterministic for a given PR open/merge timestamp pair |
| Thread safety | YES — single-threaded plugin execution model per ADR-002 |
| Overall classification | Effectful shell (host function calls for event emission; input parsing is pure) |

### Token Budget

| Item | Estimate |
|------|---------|
| BC files (this BC) | ~1 BC |
| Story anchor | S-10.05 (Wave 2 plugin schema migration) |
| Subsystem | SS-04 |
