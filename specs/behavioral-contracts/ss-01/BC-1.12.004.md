---
document_type: behavioral-contract
level: L3
version: "1.4"
status: draft
producer: product-owner
timestamp: 2026-05-08T00:00:00Z
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

# Behavioral Contract BC-1.12.004: factory-dispatcher::emit_event::per_event_host_stamping_and_internal_bifurcation — per-event OTel fields stamped at emit time; emit_internal Some/None bifurcation post-FileSink-rewire; event.category derived from compile-time registry

## Description

Per ADR-015 D-15.2, the dispatcher stamps two categories of fields at emit time:

1. **Per-event identity fields** — stamped by the host at `emit_event` time, BEFORE
   plugin domain fields are merged. These include `timestamp`, `event.id`, `event.name`
   (validated/normalized), `event.category` (derived from registry), `event.source`,
   `severity_number`, `severity_text`, `trace_id`, `span_id`, `parent_span_id`, and
   the plugin context fields (`plugin.name`, `plugin.version`, `plugin.invocation_id`,
   `hook.tool_name`, `hook.event_name`).

2. **`emit_internal` Some/None bifurcation under FileSink rewire** — after S-10.02
   rewires `host::emit_event` to write directly to `FileSink`, the pre-Wave-1
   `internal_log.is_some()` conditional in `HostContext::emit_internal` behavior changes:
   `InternalLog` is no longer the primary write target; `FileSink` is. The bifurcation
   behavior for test contexts (`internal_log: None`) is preserved but its semantics
   shift. (Stable anchor per TD-VSDD-091; use function name `HostContext::emit_internal`
   not line numbers as the canonical reference.)

3. **`event.category` compile-time registry** — the category is derived from the
   `event.name` prefix against a static compile-time registry table (D-15.2.a).
   Unrecognized prefixes resolve to `"unknown"` (NOT `"domain"` — D-15.2.b).

This BC governs the `emit_internal` Some/None bifurcation pattern that is the
central dispatch-path abstraction throughout SS-01, and specifies the per-event
host-stamping contract as it must be implemented in Wave 1 (S-10.02 + S-10.03).
The sibling BC-1.05.036 Postcondition 4 describes the pre-Wave-1 bifurcation;
this BC describes the post-Wave-1 spec-frame state.

All Canonical Test Vectors are future-implementation witnesses.

## Preconditions

1. Dispatcher startup is complete: Resource attributes stamped per BC-1.12.003,
   `service.instance.id` generated, `trace_id` established (from `VSDD_TRACE_ID` env
   or per-invocation UUID).
2. A plugin call to `host::emit_event` is in progress, OR the dispatcher is emitting
   a lifecycle event internally.
3. The `FileSink` instance is initialized (post-Wave-1 state; pre-Wave-1 state where
   `internal_log: Some(...)` is the primary route is the INTERIM state not governed by
   this BC).

## Postconditions

1. **Per-event host-stamped fields** — at `emit_event` time, the host stamps the
   following fields on EVERY event, before merging plugin domain fields. Plugin-supplied
   values for these fields are overridden (D-15.3 — host wins; visible via
   `event.host_overrides` per BC-1.12.005 Phase 1b):

   | Field | Value stamped by host |
   |-------|----------------------|
   | `timestamp` | RFC 3339 nanosecond-precision UTC (from `std::time::SystemTime::now()`) |
   | `observed_timestamp` | Same as `timestamp` (local machine; no network time adjustment) |
   | `event.id` | UUIDv4 generated per emission via `uuid::Uuid::new_v4()` |
   | `event.category` | Derived from compile-time registry (see Postcondition 2) |
   | `event.source` | `"dispatcher"` for lifecycle events; `"plugin:<plugin.name>"` for plugin-emitted events |
   | `severity_number` | OTel severity integer: `9` (INFO), `13` (WARN), `17` (ERROR) |
   | `severity_text` | `"INFO"` \| `"WARN"` \| `"ERROR"` |
   | `trace_id` | Inherited from `VSDD_TRACE_ID` env if set at dispatcher start; else per-invocation UUID |
   | `span_id` | UUIDv4 per plugin invocation (the invoking plugin's `plugin.invocation_id`) |
   | `parent_span_id` | The dispatcher's own span for this invocation |
   | `session.id` | From Claude envelope (`session_id` field in hook payload) |
   | `session.previous_id` | From Claude envelope (`parent_session_id` if present) |
   | `project.id` | From startup Resource context (SHA-256 of `vcs.repository.url.full`) |
   | `project.path` | `CLAUDE_PROJECT_DIR` env var |
   | `project.name` | Basename of `CLAUDE_PROJECT_DIR` |
   | `vcs.ref.head.name` | Branch name from `git rev-parse --abbrev-ref HEAD` |
   | `vcs.ref.head.revision` | Current commit SHA from `git rev-parse HEAD` |
   | `vcs.ref.head.type` | `"branch"` \| `"tag"` \| `"detached"` |
   | `hook.tool_name` | From Claude envelope (auto-derived) |
   | `hook.event_name` | From Claude envelope (auto-derived) |
   | `plugin.name` | Plugin identifier from hooks-registry.toml |
   | `plugin.version` | The plugin's own Cargo package version (NOT the dispatcher's version) |
   | `plugin.invocation_id` | UUIDv4 per plugin invocation |
   | `outcome` | Canonical enum: `success` \| `failure` \| `error` \| `timeout` \| `skipped` \| `blocked` |

   **Source-of-truth verification (TD-VSDD-093):** `severity_number = 9` for INFO is per
   OTel Log Data Model (SEVERITY_NUMBER_INFO = 9). `outcome` canonical values
   (`success | failure | error | timeout | skipped | blocked`) are per ADR-015 D-15.2
   per-event attributes table. `plugin.version` is the plugin's own Cargo version, NOT
   `env!("CARGO_PKG_VERSION")` — ADR-015 Context identifies this as a known bug at
   `factory-dispatcher::main::plugin_version_stamp_call_site` (the expression
   `plugin_version = env!("CARGO_PKG_VERSION")` that stamps dispatcher version instead
   of plugin version). (Stable anchor per TD-VSDD-091; line numbers are not authoritative
   — use the function/method name as the canonical reference.)

2. **`event.category` compile-time registry (D-15.2.a + D-15.2.b):** The host derives
   `event.category` from the `event.name` prefix using the static registry table below.
   Plugin authors do NOT set this field.

   | Prefix | `event.category` |
   |--------|-----------------|
   | `vsdd.dispatcher.*` | `lifecycle` |
   | `vsdd.plugin.*` | `lifecycle` |
   | `vsdd.commit.*` | `domain` |
   | `vsdd.pr.*` | `domain` |
   | `vsdd.session.*` | `domain` |
   | `vsdd.agent.*` | `domain` |
   | `vsdd.worktree.*` | `domain` |
   | `vsdd.hook.*` | `domain` |
   | `vsdd.tool.*` | `domain` |
   | `vsdd.block.*` | `audit` |
   | `vsdd.capability.denied.*` | `audit` |
   | `vsdd.error.*` | `error` |
   | `vsdd.internal.*` | `lifecycle` |
   | *(unrecognized prefix)* | `unknown` |

   **D-15.2.a:** Registry is maintained in dispatcher source code (compile-time stable).
   Operator-extensible config-file registration is explicitly deferred.
   **D-15.2.b:** Unrecognized prefixes → `"unknown"` (NOT `"domain"`). This is intentional:
   it allows `unknown_category_events` Grafana alert (Wave 3 AC-2) to catch uncategorized
   events rather than silently absorbing them into domain aggregates.

3. **`emit_internal` Some/None bifurcation post-rewire:**
   - **Production path (FileSink wired):** `host::emit_event` writes through `FileSink`
     to `events-*.jsonl` as the primary destination (per BC-1.12.001). The `internal_log`
     field in `HostContext` is repurposed: when `VSDD_DEBUG_LOG=1`, the event is ALSO
     written to `dispatcher-internal-*.jsonl` via `InternalLog` (per BC-1.12.002).
   - **Test context path (no FileSink, `internal_log: None`):** When `HostContext` is
     constructed in a test context via `HostContext::new` (which sets `internal_log: None`),
     the `InternalLog` write is skipped entirely (the same pre-Wave-1 behavior). Test contexts
     assert events through the in-memory `events` queue via `drain_events()`. This is
     intentional and documented (BC-1.05.036 Postcondition 4 established this pattern).
     Post-Wave-1, the FileSink is also absent from test contexts unless explicitly injected —
     test behavior through the `events` queue is preserved.
   - **SOUL #4 acknowledgment:** The `if let Ok(mut events) = self.events.lock()` pattern
     in `HostContext::events::lock_push` silently drops on Mutex poison (acknowledged in
     BC-1.05.036 EC-011 / OQ-W16-004). See also BC-1.05.036 EC-011. (Stable anchor per
     TD-VSDD-091; use the expression name `HostContext::events::lock_push` not line
     numbers as the canonical reference.) This known limitation is NOT changed by
     BC-1.12.004 — the FileSink rewire adds a new primary path; the in-memory queue
     remains best-effort for test contexts. The known asymmetry (silent-drop on poison
     vs panic on drain) persists.
   - **Per-event `event.schema_url` (D-15.2.d):** Each event carries an `event.schema_url`
     attribute identifying the schema version of that specific event family
     (e.g., `"https://vsdd-factory.dev/schemas/events/v2/commit.made"`). This is DISTINCT
     from the process-level Resource `schema_url`. The per-event attribute is
     INFORMATIONAL-ONLY (forward-discovery hint). Consumers MUST route by `event.name`
     suffix (`.vN`); they MUST NOT require `event.schema_url` to be dereferenceable.

4. **`plugin.version` fix (content-defect bug per ADR-015 Context):** After Wave 1, the
   host stamps `plugin.version` with the plugin's OWN Cargo package version, NOT with
   `env!("CARGO_PKG_VERSION")` (the dispatcher's version). The bug at `main.rs::run` is
   fixed in S-10.02. A plugin compiled as version `0.3.1` must have `plugin.version = "0.3.1"`
   in its emitted events, not the dispatcher's version.

## Invariants

1. `event.id` is a fresh UUIDv4 per emission. Two distinct `emit_event` calls MUST NOT
   share an `event.id` (barring UUID collision, probability negligible at vsdd-factory scale).
2. `event.category` is ALWAYS one of: `lifecycle`, `domain`, `audit`, `error`, `unknown`.
   There is no other valid value. Plugin-supplied `event.category` is overridden.
3. `plugin.version` reflects the plugin's Cargo package version, NOT the dispatcher's.
4. Host-stamped fields win unconditionally over plugin-supplied values for the same field
   (D-15.3). Plugin domain fields are merged AFTER host stamping.
5. The `event.category = "unknown"` value for unrecognized prefixes is deterministic and
   testable — any event with an unregistered prefix produces `"unknown"`, not `"domain"`.

## Related BCs

- BC-1.12.001 — Single-stream FileSink routing (depends on: per-event stamps from this BC
  are attached to every event BEFORE FileSink write)
- BC-1.12.003 — Resource attribute startup stamping (composes with: Resource fields from
  startup are merged with per-event fields from this BC to form the complete event record)
- BC-1.05.036 — `host.exec_subprocess.completed` event (depends on: the `emit_internal`
  Some/None bifurcation described in BC-1.05.036 Postcondition 4 is the pre-Wave-1
  behavior; this BC describes the post-Wave-1 repurposing of that bifurcation)
- BC-1.11.001 — VSDD_TRACE_ID injection (sibling: `trace_id` in per-event fields comes
  from the same startup context where VSDD_TRACE_ID is established)

## Architecture Anchors

- `HostContext::emit_internal` (in `crates/factory-dispatcher/src/host/mod.rs`) — current
  `emit_internal` bifurcation (pre-Wave-1: `internal_log.is_some()` routes to
  `InternalLog::write`; post-Wave-1: routes to `FileSink` as primary, `InternalLog` as
  debug-supplementary when `VSDD_DEBUG_LOG=1`). Stable anchor per TD-VSDD-091; line numbers
  are not authoritative — use the function name as the canonical reference. See BC-1.05.036
  EC-011 for the Mutex-poison silent-drop acknowledgment.
- `factory-dispatcher::main::plugin_version_stamp_call_site` (expression
  `plugin_version = env!("CARGO_PKG_VERSION")` in `crates/factory-dispatcher/src/main.rs`)
  — the bug cited in ADR-015 Context; fixed in S-10.02 to use actual plugin version.
  Stable anchor per TD-VSDD-091; line numbers are not authoritative.
- ADR-015 D-15.2 per-event attributes table — the authoritative list of all per-event fields
  and value sources
- ADR-015 D-15.2.a — compile-time registry ownership decision
- ADR-015 D-15.2.b — unrecognized prefix → `"unknown"` decision
- ADR-015 D-15.2.d — per-event `event.schema_url` informational-only semantics

## Story Anchor

S-10.02 (Wave 1: FileSink single-stream wiring + per-event stamping + `plugin.version` fix)
S-10.03 (Wave 1: Resource-attribute enrichment, which provides the startup context this
BC's per-event stamping draws from)
S-10.04 (Wave 1: Trace propagation + lifecycle event types — registry entries for vsdd.internal.* lifecycle event names per BC-1.12.004 PC2)

## VP Anchors

(TBD — to be assigned after S-10.02/S-10.03 story authoring)

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Plugin emits event with `event.name = "vsdd.commit.made.v1"` | `event.category = "domain"` (prefix `vsdd.commit.*` maps to `domain` per registry) |
| EC-002 | Plugin emits event with `event.name = "vsdd.block.plugin_blocked.v1"` | `event.category = "audit"` (prefix `vsdd.block.*` maps to `audit`) |
| EC-003 | Plugin emits event with `event.name = "my.custom.event.v1"` (unregistered prefix) | `event.category = "unknown"` (D-15.2.b); event still written to `events-*.jsonl`; triggers `unknown_category_events` Grafana alert in Wave 3 |
| EC-004 | Plugin supplies `event.category = "domain"` in its domain fields | Host overrides this with the registry-derived value; plugin-supplied `event.category` is discarded; `event.host_overrides: ["event.category"]` is stamped (per D-15.3 host-field-override visibility — Phase 1b, BC-1.12.005) |
| EC-005 | Plugin supplies `plugin.version = "1.0.0"` (potentially wrong if it matches dispatcher version) | After Wave 1 fix, host stamps correct plugin version from `plugin.manifest` or registry; plugin-supplied value for `plugin.version` is treated as a plugin domain field and subject to override if it conflicts with host-stamped value. **Note:** if the plugin legitimately knows its own version, the host stamped value should match. Mismatch signals a legacy plugin. |
| EC-006 | `VSDD_TRACE_ID` is set in environment at dispatcher startup | `trace_id` = inherited value; same `trace_id` on all events in this invocation; propagated to `exec_subprocess` environments per BC-1.11.001 |
| EC-007 | `VSDD_TRACE_ID` is NOT set | `trace_id` = fresh UUIDv4 generated at dispatcher startup; used for all events in this invocation |
| EC-008 | Plugin emits event in test context (`HostContext::new` with `internal_log: None`); no FileSink injected | Event pushed to in-memory `events` queue via `events.lock().push(event)` at `host/mod.rs::HostContext::emit_internal`; no file written; observable via `drain_events()`. FileSink NOT involved (test context has no FileSink). Pre-Wave-1 and post-Wave-1 test behavior is identical for this path. |
| EC-009 | `event.name` format does not include `.vN` suffix (e.g., `"pr.created"` old-style name) | Event is written with the supplied `event.name`; `event.category` is derived from prefix match. `"pr.created"` prefix `"pr"` is NOT in the registry → `event.category = "unknown"`. Plugins using old-style names without `vsdd.` prefix will appear in `unknown` category. Migration to Wave 2 reverse-DNS names is expected. |
| EC-010 | Multiple events emitted in same `emit_event` call (hypothetical batch) | Each event gets its own fresh `event.id` UUIDv4. Two events with the same `event.id` is a postcondition violation. |
| EC-011 | **`plugin.version` fix sentinel:** plugin at `crates/hook-plugins/capture-pr-activity/` compiled as version `0.2.0`; pre-Wave-1 dispatcher stamps `plugin_version = dispatcher_version` | Post-Wave-1: `plugin.version = "0.2.0"` (actual plugin Cargo version); NOT the dispatcher's version. **Future-implementation witness:** assert `plugin.version != dispatcher.service.version` for any plugin that has a different semver than the dispatcher (which is the normal case). If a misimplementation still stamps dispatcher version, `plugin.version = "0.2.0"` test assertion fails. |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Plugin emits `event.name = "vsdd.commit.made.v1"` | Event in `events-*.jsonl` has `event.category = "domain"` | category-registry-domain |
| Plugin emits `event.name = "vsdd.block.plugin_blocked.v1"` | `event.category = "audit"` | category-registry-audit |
| Plugin emits `event.name = "vsdd.dispatcher.started.v1"` | `event.category = "lifecycle"` | category-registry-lifecycle |
| Plugin emits `event.name = "vsdd.error.plugin_panicked.v1"` | `event.category = "error"` | category-registry-error |
| Plugin emits `event.name = "my.custom.unregistered.v1"` | `event.category = "unknown"` (D-15.2.b) | category-registry-unknown |
| **Misimplementation distinguisher:** implementation returns `"domain"` for unrecognized prefix | Test MUST assert `event.category = "unknown"` for unrecognized prefix. A misimplementation that defaults to `"domain"` silently absorbs unknown events into domain dashboards. | misimplementation-witness-category-default |
| Two consecutive `emit_event` calls | Two events with distinct `event.id` UUIDv4 values | event-id-uniqueness |
| Plugin emits event with `event.category = "domain"` in payload | Host overrides; emitted event has `event.category` from registry (NOT the plugin-supplied value); `event.host_overrides: ["event.category"]` present (post-Wave-1 D-15.3 implementation per Phase 1b BC-1.12.005) | host-override-category |
| `VSDD_TRACE_ID=abc123` in env; plugin emits event | `trace_id = "abc123"` in emitted event | trace-id-inheritance |
| `VSDD_TRACE_ID` unset; plugin emits event | `trace_id` is a valid UUIDv4 (36-char format `xxxxxxxx-xxxx-4xxx-xxxx-xxxxxxxxxxxx`); same value across all events in this invocation | trace-id-generated |
| Post-Wave-1; plugin compiled as v0.2.0; dispatcher is v1.1.0 | `plugin.version = "0.2.0"` (NOT `"1.1.0"`); `service.version = "1.1.0"` | plugin-version-fix-sentinel |
| Test context: `HostContext::new()` (no FileSink, `internal_log: None`); emit event | Event in `drain_events()` in-memory queue; no file written; `event.id` and `event.category` still stamped correctly (host stamping happens before queue push, regardless of FileSink presence) | test-context-no-filesink |
| `event.schema_url` field on emitted event | Present with value of form `"https://vsdd-factory.dev/schemas/events/v2/<event-family>"` (informational; not required to be dereferenceable) | per-event-schema-url |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| (TBD) | `event.category` correctly derived for all 14 registered prefixes + `"unknown"` | unit test: tabular test over all registry entries + one unregistered prefix |
| (TBD) | `event.id` is unique per emission | property-based test: N emissions → N distinct UUIDs |
| (TBD) | `plugin.version` reflects plugin's own version, not dispatcher's | integration test: compile plugin with known version; assert emitted event `plugin.version` |
| (TBD) | Host-supplied `event.category` overrides plugin-supplied value | unit test: emit event with plugin-supplied `event.category = "domain"` for a `vsdd.block.*` prefix; assert result is `"audit"` |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-029 |
| Capability Anchor Justification | CAP-029 ("Emit structured events to a single observability stream (file path)") per capabilities.md §CAP-029. This BC governs the compile-time event-name registry (D-15.2.a), per-event host stamping, and the `dispatcher_version` stamping fix — all of which determine the content correctness of every event written to the single `events-*.jsonl` stream that CAP-029 defines. The registry ensures event.category is accurately derived before each record reaches FileSink; without it, the stream's semantic structure (which events are lifecycle vs domain vs audit) would be undefined. |
| L2 Domain Invariants | DI-017 (renamed by ADR-015 v1.7 from dispatcher_trace_id → trace_id; this BC stamps `trace_id` at emit time as a per-event host-owned identity field; every event in the single stream must carry `trace_id` — see Postcondition 1 trace_id row) |
| Architecture Module | SS-01 — `crates/factory-dispatcher/src/host/mod.rs` (`HostContext::emit_internal` bifurcation, per-event stamping), `factory-dispatcher::main::plugin_version_stamp_call_site` (plugin.version bug fix) |
| Stories | S-10.02 (FileSink wiring + per-event stamping + plugin.version fix), S-10.03 (Resource context consumed here), S-10.04 (Trace propagation + lifecycle event types) |
| Epic | E-10 (Single-stream OTel-aligned event emission) |
| ADR | ADR-015 D-15.2 (per-event attributes table); ADR-015 D-15.2.a (compile-time registry); ADR-015 D-15.2.b (unknown prefix default); ADR-015 D-15.2.d (per-event `event.schema_url`); ADR-015 D-15.3 (host fields win) |
| Content-defect bug | `plugin_version = env!("CARGO_PKG_VERSION")` at `factory-dispatcher::main::plugin_version_stamp_call_site` — fixed in S-10.02 per ADR-015 Context "Field schema is critically incomplete" (stable anchor per TD-VSDD-091) |

### Purity Classification

| Property | Assessment |
|----------|-----------|
| I/O operations | YES — file write via FileSink; git commands for `vcs.ref.head.*` fields |
| Global state access | YES — reads startup `ResourceContext`; reads `HostContext` for session/plugin context |
| Deterministic | MOSTLY: `event.category` derivation and most fields are deterministic; `event.id` and `timestamp` are intentionally non-deterministic (fresh per emission) |
| Thread safety | YES — per BC-1.08.006 and ADR-008 current-thread runtime; no concurrent emit calls |
| Overall classification | Effectful shell (file I/O + compile-time registry lookup + deterministic field assembly with intentionally non-deterministic UUIDs/timestamps) |

### TD-VSDD-092 (BC-SOUL4-coverage) Verification

Source-walk for silent-discard patterns in per-event stamping and `emit_internal` bifurcation:

- `HostContext::events::lock_push` (the `events.lock().push(event)` expression in
  `HostContext::emit_internal`): uses `if let Ok(mut events) = self.events.lock()`
  which silently drops on Mutex poison. Acknowledged in EC-008 (test-context path) and as
  known-limitation per BC-1.05.036 EC-011 / OQ-W16-004. (Stable anchor per TD-VSDD-091;
  use the expression name `HostContext::events::lock_push` not line numbers.) This is the
  ONLY known silent-discard in this path; it is documented, not new.
- Per-event field computation: `uuid::Uuid::new_v4()` for `event.id` is infallible (returns
  a value, no error path). `std::time::SystemTime::now()` for `timestamp` is infallible.
  No `let _ =` patterns expected or permitted for these computations.
- `event.category` registry lookup: compile-time static table; lookup is infallible.
  No error path exists; no silent-discard risk.
- Plugin field override: when the host overrides a plugin-supplied field, the override
  MUST be visible via `event.host_overrides` (Phase 1b, BC-1.12.005). Silent override
  without `event.host_overrides` annotation is a SOUL #4 violation for the plugin author
  (their field is discarded without signal). Phase 1b BC-1.12.005 covers this; it is
  flagged here for Phase 1b authoring awareness.

## Changelog

| Version | Date | Change |
|---------|------|--------|
| v1.0 | 2026-05-06 | Initial authoring (D-315). Per-event host stamping, emit_internal Some/None bifurcation, event.category compile-time registry per ADR-015 D-15.2/D-15.2.a/D-15.2.b. |
| v1.1 | 2026-05-06 | D-315/D-316 amendments — cap-anchor justification, edge case sharpening. |
| v1.2 | 2026-05-06 | D-319 — F-3 fix: Story Anchor + Stories cell extended with S-10.04 (POLICY 8 reverse-direction drift from D-316 closed). |
| v1.4 | 2026-05-08 | TD-VSDD-091 Chunk 4 — migrated 2 line citations to stable symbol anchors: `main.rs:143` → `main.rs::run`; `host/mod.rs:113-115` → `host/mod.rs::HostContext::emit_internal`. |
| v1.3 | 2026-05-06 | D-325 — F-7 sweep: L2 Capability cell paraphrase removed — cell now just `CAP-029`. F-14 sweep: Architecture Anchors already carry `Stable anchor per TD-VSDD-091` on code symbol references (`HostContext::emit_internal`, `factory-dispatcher::main::plugin_version_stamp_call_site`); stable-anchor discipline confirmed applied. |
