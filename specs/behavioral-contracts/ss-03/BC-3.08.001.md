---
document_type: behavioral-contract
level: L3
version: "1.19"
last_amended: 2026-07-07
status: draft
producer: product-owner
timestamp: 2026-05-07T00:00:00Z
phase: F2
inputs:
  - .factory/cycles/v1.0-feature-plugin-async-semantics-pass-1/adversary-pass-1.md
  - .factory/specs/behavioral-contracts/ss-01/BC-1.14.001.md
  - .factory/specs/behavioral-contracts/ss-07/BC-7.06.001.md
input-hash: "6549a11"
traces_to: .factory/specs/prd.md
origin: greenfield
extracted_from: null
subsystem: "SS-03"
capability: "CAP-003"
lifecycle_status: active
introduced: v1.0-feature-plugin-async-semantics-pass-1
modified:
  - "2026-07-06 (v1.15)"
  - "2026-07-06 (v1.16)"
  - "2026-07-06 (v1.17)"
  - "2026-07-07 (v1.18)"
  - "2026-07-07 (v1.19)"
deprecated: null
deprecated_by: null
replacement: null
retired: null
removed: null
removal_reason: null
---

# BC-3.08.001: dispatcher async-semantics event types are catalogued and emitted via FileSink — `plugin.async_block_discarded`, `dispatcher.schema_mismatch`, `dispatcher.registry_invalid`, `plugin.timeout` (async path), `plugin.abandoned`, `plugin.completed` (async path)

## Description

ADR-019 F2 introduces four new event-type strings as part of the async-semantics feature; `plugin.abandoned` is added by the F5 E-19 pass-1 fix burst (F-P1-013) to cover the async drain-window expiry path; `plugin.completed` (async path) is added by the F5 E-19 pass-5 fix burst (F-P5-003) to cover async plugins that complete within the drain window. These events are referenced in BC-1.14.001 and BC-7.06.001 but require SS-03 catalog authority to define their payload schemas and wire format. Each event is a JSON line written to `events-*.jsonl` via the standard FileSink path. This BC provides the catalog entry for all six, establishing the authoritative field set, wire format, and sink-fan-out obligation per DI-007 (amended: opt-in debug stream) and the VP-028 sink-fan-out invariant.

## Preconditions

1. Dispatcher is running with `schema_version = 2` registry (BC-7.06.001).
2. FileSink is initialized and the `events-YYYY-MM-DD.jsonl` file is writable.
3. The triggering condition for each event type has occurred (see Postconditions for per-event triggers).

## Common Fields

All five event types carry the following dispatcher-owned fields on the wire. These fields are injected by the host (see `emit_event.rs` enrichment path) and are never supplied by plugins (they are RESERVED_FIELDS — see §Implementation Notes):

| Field | Type | Description |
|-------|------|-------------|
| `trace_id` | UUID v4 string | Trace correlation value from the invoking hook envelope (DI-017). Canonical wire-format name; `dispatcher_trace_id` must NOT appear on wire (Invariant 5). |
| `session_id` | UUID v4 string | Claude Code session identifier from the hook envelope context (`ctx.session_id`). Present on all five event types (O-P15-001). |
| `plugin_name` | string | Name of the plugin registry entry, injected by the host. Present on plugin-context events (1, 4, 5, and 6) only; absent from dispatcher-startup events (2 + 3) which have no plugin context. |
| `ts` | string | Emission timestamp (internal format). |
| `ts_epoch` | integer | Emission timestamp as Unix epoch milliseconds. |
| `schema_version` | integer | Registry schema version at emission time. |
| `type` | string | The event type string (e.g. `"plugin.async_block_discarded"`). |

The §Common Fields appear on the wire for ALL six event types except where noted. Wire-format examples in §Postconditions show:
- **Plugin-context events (1, 4, and 5):** `plugin_name` explicitly shown (these are plugin-instance events). `plugin_version` is NOT emitted by Events 1, 4, and 5 — the original BC-3.08.001 emit functions for these events (in `crates/factory-dispatcher/src/host/emit_event.rs`) do not call `with_plugin_version()`.
- **Event 6 (`plugin.completed` async path):** mirrors the sync-path `emit_lifecycle` call chain in `crates/factory-dispatcher/src/executor.rs`, which includes `with_plugin_version()`. Both `plugin_name` and `plugin_version` are explicitly shown in the Event 6 wire example.
- **Dispatcher-startup events (2 + 3):** `plugin_name` OMITTED from examples (no plugin context at dispatcher startup).
- All six event types: `trace_id` + `session_id` explicitly shown (verified by VP-079 payload conformance).
- Common fields shown only in summary: `ts`, `ts_epoch`, `schema_version` (always emitted; not in examples for readability).

## Postconditions

### Event 1: `plugin.async_block_discarded`

**Trigger**: An async group plugin (one with `async = true` in the registry) returns exit code 2 (which would be a block verdict if it were in the sync group). Because async plugins cannot have `on_error = "block"` (BC-1.14.001 Invariant 4), the block intent is structurally invalid and is discarded rather than reaching Claude Code.

**Wire format** (JSON line in `events-*.jsonl`):

```json
{
  "type": "plugin.async_block_discarded",
  "trace_id": "<uuid-v4>",
  "session_id": "<uuid-v4>",
  "plugin_name": "<string — registry entry name>",
  "exit_code": 2,
  "timestamp": "<ISO-8601>",
  "reason": "async_plugin_block_verdict_discarded"
}
```

**Mandatory fields**: `type`, `trace_id`, `session_id`, `plugin_name`, `exit_code`, `timestamp`, `reason`.

### Event 2: `dispatcher.schema_mismatch`

**Trigger**: Dispatcher loads `hooks-registry.toml` and finds `schema_version != 2` (e.g., `schema_version = 1` or any unknown value). Dispatcher hard-errors and emits this event before exiting.

**Wire format** (JSON line in `events-*.jsonl`):

```json
{
  "type": "dispatcher.schema_mismatch",
  "trace_id": "<uuid-v4>",
  "session_id": "<uuid-v4>",
  "found_version": <integer or null>,
  "expected_version": 2,
  "timestamp": "<ISO-8601>",
  "error_code": "E-REG-001"
}
```

**Mandatory fields**: `type`, `trace_id`, `session_id`, `found_version`, `expected_version`, `timestamp`, `error_code`.

### Event 3: `dispatcher.registry_invalid`

**Trigger**: Dispatcher's `registry.rs::validate()` detects a registry invariant violation. Two distinct violation conditions trigger this event, each with its own `error_code`:

| `error_code` | `violation` | Triggering condition |
|---|---|---|
| `E-REG-002` | `async_block_conflict` | A registry entry has both `on_error = "block"` AND `async = true` simultaneously |
| `E-REG-003` | `duplicate_hook_registration` | Two or more registry entries share the same hook name (duplicate registration) |

Dispatcher emits this event and hard-errors (non-zero exit) for either condition.

**Wire format — E-REG-002 variant** (JSON line in `events-*.jsonl`):

```json
{
  "type": "dispatcher.registry_invalid",
  "trace_id": "<uuid-v4>",
  "session_id": "<uuid-v4>",
  "offending_plugin": "<string — name of the plugin entry that violates the invariant>",
  "violation": "async_block_conflict",
  "timestamp": "<ISO-8601>",
  "error_code": "E-REG-002"
}
```

**Wire format — E-REG-003 variant** (JSON line in `events-*.jsonl`):

```json
{
  "type": "dispatcher.registry_invalid",
  "trace_id": "<uuid-v4>",
  "session_id": "<uuid-v4>",
  "offending_plugin": "<name>",
  "offending_event": "<event>",
  "offending_tool": "<tool regex string or null>",
  "violation": "duplicate_hook_registration",
  "error_code": "E-REG-003",
  "timestamp": "<ISO-8601>"
}
```

**Mandatory fields for E-REG-003**: `type`, `trace_id`, `session_id`, `offending_plugin` (string, required), `offending_event` (string, required), `offending_tool` (string or null, required — null when the duplicating entry has no `tool` filter), `violation`, `error_code`, `timestamp`.

**E-REG-002 vs E-REG-003 field asymmetry (F-P14-001):** E-REG-002 (`AsyncBlockConflict`) does NOT include `offending_event` or `offending_tool` in its payload — the violation is intra-entry (a single entry simultaneously has `on_error = "block"` and `async = true`; no second entry is involved). E-REG-003 (`DuplicateEntry`) DOES include `offending_event` and `offending_tool` because the violation is inter-entry: the specific `(name, event, tool)` tuple uniquely identifies which entry is the duplicator. The data is already present in `RegistryError::DuplicateEntry { name, event, tool }` and MUST be propagated to the event payload. Implementations MUST propagate all three tuple fields; omitting `offending_event` or `offending_tool` is a BC violation.

The `error_code` field is an enum with exactly two valid values: `"E-REG-002"` and `"E-REG-003"`. The `violation` field value is determined by the `error_code` per the table above — no other combinations are valid.

### Event 4: `plugin.timeout` (async path)

**Note**: `plugin.timeout` is emitted for both sync and async plugin timeouts. The sync-path behavior is governed by BC-1.14.001. This entry covers the **async-path variant** only: when an async group plugin exceeds its `timeout_ms` and is terminated.

**Wire format** (JSON line in `events-*.jsonl`):

```json
{
  "type": "plugin.timeout",
  "trace_id": "<uuid-v4>",
  "session_id": "<uuid-v4>",
  "plugin_name": "<string>",
  "execution_group": "async",
  "timeout_ms": <integer>,
  "timestamp": "<ISO-8601>"
}
```

**Mandatory fields**: `type`, `trace_id`, `session_id`, `plugin_name`, `execution_group`, `timeout_ms`, `timestamp`.

### Event 5: `plugin.abandoned`

**Trigger**: The async drain timer fires (`tokio::select!` timer arm in `crates/factory-dispatcher/src/main.rs`, EC-011) while the plugin's forwarding task has not yet delivered a result to the drain channel. The plugin was dispatched but did not complete within the `ASYNC_DRAIN_WINDOW_MS` budget (DI-019). One `plugin.abandoned` event is emitted per in-flight plugin at drain expiry.

**Wire format** (JSON line in `events-*.jsonl`):

```json
{
  "type": "plugin.abandoned",
  "trace_id": "<uuid-v4>",
  "session_id": "<uuid-v4>",
  "plugin_name": "<string>",
  "entry_index": <u32>,
  "drain_window_ms": <integer>,
  "timestamp": "<ISO-8601>"
}
```

**Mandatory fields**: `type`, `trace_id`, `session_id`, `plugin_name`, `entry_index`, `drain_window_ms`, `timestamp`.

**`entry_index` semantics**: The ordinal position (0-based, from `enumerate()`) of this plugin's registry entry in the async partition at the time of dispatch. The `plugin_name` field in `plugin.abandoned` events is the registry entry `name` field verbatim — it is not derived from the WASM binary path or a logical plugin grouping label. The registry schema does NOT enforce `name` uniqueness across entries: it is possible to register two entries with identical `name` values for different event types or tool filters. If both such entries are in-flight at drain time, both emit `plugin.abandoned` events with the same `plugin_name`, making name-only keying ambiguous. Name-only keying collapses those distinct invocations; consumers need the `(plugin_name, entry_index)` tuple to unambiguously identify which registry entry was abandoned. (Note on the production registry: the two `verify-factory-lock` entries carry DIFFERENT `name` values — `verify-factory-lock` and `verify-factory-lock-bash` respectively — and would therefore produce distinct `plugin_name` values in any `plugin.abandoned` events; `entry_index` disambiguation is a schema-level invariant for future-proof consumers of any registry that does not enforce `name` uniqueness.)

**Schema-level defense, not a runtime dispatch gate (F-P7-007):** The concurrent-same-`plugin_name` scenario has no production dispatch path — the production registry has no entries sharing the same `name` value AND overlapping tool-filter patterns that would co-occur in the same async partition invocation (confirmed: `registry.rs` enforces `(name, event, tool)` tuple uniqueness; two same-named entries require different `event` or `tool` values, and co-occurrence in a single async partition dispatch requires both entries to match the same event+tool pair simultaneously, which no production entry pair does). `entry_index` is a **schema-level defense**: its correctness is verified by serialization/property tests over the `plugin.abandoned` event struct — asserting that the 0-based ordinal from `enumerate()` of the async partition is correctly marshalled into the `entry_index` field — not by a runtime concurrent-dispatch fixture. The runtime path (single unique `plugin_name` per async partition in any production dispatch) is covered by normal integration tests.

**`drain_window_ms` semantics**: The effective drain window value at the time the timer fired — `ASYNC_DRAIN_WINDOW_MS` in release builds, or the debug-override value from `VSDD_ASYNC_DRAIN_WINDOW_MS` in debug builds (SEC-003). This is the dispatcher-level drain window, distinct from the per-plugin `timeout_ms` carried by `plugin.timeout` events. Both may apply to the same plugin: `plugin.timeout` fires when the plugin exceeds its per-plugin budget; `plugin.abandoned` fires when the drain window expires regardless of per-plugin timeout status.

**Abandoned-vs-late-completion semantics (F-P1-013)**: See Invariant 6.

**Sink destination**: All six events are routed to `events-*.jsonl` via FileSink. They are NOT routed to the dispatcher-internal debug stream (which is opt-in per DI-007 amended). The VP-028 sink-fan-out invariant applies: if multiple sinks are configured, all six events must fan out to all applicable sinks.

### Event 6: `plugin.completed` (async path)

**Note**: `plugin.completed` is emitted for both sync and async plugin completions. The sync-path behavior is governed by BC-1.14.001 and the internal log. This entry covers the **async-path variant** only: when an async group plugin completes within the drain window with a non-block exit code.

**Trigger**: An async group plugin's result arrives on the drain channel receiver (`rx`) before the `tokio::select!` timer arm fires (EC-011), and the result is `PluginResult::Ok` with a non-block exit code. The plugin completed within the `ASYNC_DRAIN_WINDOW_MS` budget (DI-019) without timing out or being abandoned.

**Wire format** (JSON line in `events-*.jsonl`):

```json
{
  "type": "plugin.completed",
  "trace_id": "<uuid-v4>",
  "session_id": "<uuid-v4>",
  "plugin_name": "<string — registry entry name>",
  "plugin_version": "<string>",
  "entry_index": <u32>,
  "exit_code": <integer>,
  "elapsed_ms": <integer>,
  "fuel_consumed": <integer>
}
```

`stderr` is present only when non-empty (matching sync-path behavior per `extra_fields.retain(|(k, v)| k != "stderr" || ...)` in `crates/factory-dispatcher/src/executor.rs`).

**Mandatory fields**: `type`, `trace_id`, `session_id`, `plugin_name`, `plugin_version`, `entry_index`, `exit_code`, `elapsed_ms`, `fuel_consumed`.

**`entry_index` semantics**: Mirrors Event 5 (`plugin.abandoned`) — the ordinal position (0-based, from `enumerate()`) of this plugin's registry entry in the async partition at the time of dispatch. The `(plugin_name, entry_index)` tuple unambiguously identifies which registry entry completed, enabling correlation with the corresponding `plugin.invoked` event and exclusion under Invariant 6. See Event 5 `entry_index` semantics paragraph for the full disambiguation rationale. The same schema-level defense applies: correctness of `entry_index` in `plugin.completed` events is verified by property/serialization tests over the event struct, not by a runtime concurrent-dispatch fixture (F-P7-007).

**Invariant 6 interplay**: `plugin.completed` (async path) and `plugin.abandoned` are mutually exclusive for any given `(trace_id, plugin_name, entry_index)` tuple (Invariant 6). When the drain timer fires, the `rx` channel receiver is dropped, precluding completion delivery for abandoned plugins. Conversely, a plugin that delivers its result to `rx` before the timer arm fires cannot subsequently emit `plugin.abandoned` for the same invocation.

## Invariants

1. **`trace_id` is mandatory on all six event types**: Per DI-017, every emitted event carries the UUID v4 from the invoking hook envelope. These six events are no exception.
2. **Events are write-once, no retry**: These are diagnostic events; partial emission is acceptable (emit-then-crash). They are never retried on FileSink write failure.
3. **Events do not affect dispatcher exit code**: All six are observability-only. `plugin.async_block_discarded`, `plugin.timeout (async)`, `plugin.abandoned`, and `plugin.completed` (async path) are logged and forgotten. `dispatcher.schema_mismatch` and `dispatcher.registry_invalid` accompany a hard exit (non-zero) but the event itself does not cause the exit — the validation failure does.
4. **`plugin.async_block_discarded` reason field is the literal string `"async_plugin_block_verdict_discarded"`**: Not an error code; a diagnostic reason string for human-readable log inspection.
5. **`trace_id` is the exclusive wire-format field name for the trace correlation value**: The dispatcher's structured-event wire format uses field name `trace_id` exclusively. The legacy field name `dispatcher_trace_id` MUST NOT appear in the serialized wire output. Plugins MUST NOT emit a `trace_id` field via `with_field()` — `trace_id` is reserved for the dispatcher (see §Implementation Notes). Reference: DI-017 (amended per F-P1-007).
6. **`plugin.abandoned` and `plugin.completed` (async path) are mutually exclusive terminal outcomes per invocation**: When the async drain timer fires (EC-011), `plugin.abandoned` is the last observable event for each in-flight plugin in this invocation. No `plugin.completed` event fires after `plugin.abandoned` for the same `trace_id` + `plugin_name` + `entry_index` tuple. Conversely, a plugin that emits `plugin.completed` (async path — Event 6) within the drain window cannot subsequently emit `plugin.abandoned` for the same tuple; the drain timer fires only for plugins still in-flight at expiry. Rationale for semantics option (a) abort-at-drain (F-P1-013): the dispatcher exits shortly after the drain window expires; in-flight Tokio tasks that complete after the `break` have no live FileSink to emit from. The `rx` channel receiver is dropped at drain timer fire, so any late result send is silently discarded. Option (b) (both events may fire) is structurally impossible under the current single-process lifecycle — the process exits before abandoned tasks can complete and write to a live sink. Option (c) (suppress `plugin.completed` after `plugin.abandoned`) is mechanically equivalent to (a) without benefit; no suppression logic is needed when the emission path is already closed. **Schema-level note (F-P7-007):** The `(trace_id, plugin_name, entry_index)` mutual-exclusivity key is a schema-level predicate — its correctness as a disambiguation key is verifiable by property/serialization tests over the event structs (asserting both events carry the same tuple fields with consistent types and ordinal derivation), not by a runtime concurrent-dispatch fixture that exercises two same-named entries simultaneously.

## Implementation Notes

### RESERVED_FIELDS and `trace_id` (F-P1-007, O-P15-003)

Implementations MUST add `trace_id` to the host-side reserved-fields filter (e.g., `RESERVED_FIELDS` in `crates/factory-dispatcher/src/host/emit_event.rs`). This prevents plugins from spoofing the dispatcher's trace correlation value via `with_field("trace_id", ...)`.

The reserved-fields filter MUST also retain `dispatcher_trace_id` for backward defense (defense-in-depth): even though the dispatcher no longer emits `dispatcher_trace_id` on the wire, plugins must not be allowed to inject it. Both names are reserved regardless of which name the dispatcher currently uses as the canonical field.

**Full `RESERVED_FIELDS` enumeration** (O-P15-003 — authoritative source: the `RESERVED_FIELDS` constant in `crates/factory-dispatcher/src/host/emit_event.rs`):

| Field name | Reason reserved |
|------------|----------------|
| `trace_id` | Canonical wire-format trace correlation value; dispatcher-owned per DI-017 and Invariant 5 of this BC (added by F-P1-007) |
| `dispatcher_trace_id` | Legacy field name; retained for defense-in-depth per F-P1-007; dispatcher no longer emits this on the wire but plugins must not inject it |
| `session_id` | Claude Code session identifier; dispatcher-owned; injected from `ctx.session_id` (see §Common Fields) |
| `plugin_name` | Plugin identity; dispatcher-owned; injected from `ctx.plugin_name` |
| `plugin_version` | Plugin version; dispatcher-owned; injected from `ctx.plugin_version` |
| `ts` | Emission timestamp (internal format); dispatcher-owned |
| `ts_epoch` | Emission timestamp as Unix epoch milliseconds; dispatcher-owned |
| `schema_version` | Registry schema version; dispatcher-owned |
| `type` | Event type string; provided by the emitting call site (not the plugin's field buffer); reserved to prevent shadowing |

Plugins that attempt to set any of these fields via `with_field()` MUST have the field silently stripped by the host-side filter before serialization. The full set is tested by the `reserved_fields_rejected` integration test in `crates/factory-dispatcher/src/host/emit_event.rs`.

For canonical HOST_ABI documentation of which fields the dispatcher enriches automatically, see `crates/hook-sdk/HOST_ABI.md` §`emit_event`. Note: HOST_ABI.md uses the legacy name `dispatcher_trace_id` in its enrichment description (§`emit_event` enrichment description; source-line carve-out per TD-VSDD-091: line 267 is unstable, stable anchor is §`emit_event` section) — the actual wire-format name is `trace_id` per BC-3.08.001 Invariant 5. RESERVED_FIELDS in `emit_event.rs` is the authoritative implementation reference.

## Error Paths

| Condition | Behavior |
|-----------|----------|
| FileSink write fails during event emission | Emission silently dropped; no retry; dispatcher continues (or exits if the triggering error requires exit) |
| `trace_id` unavailable at emission time | Event emitted with `trace_id: null`; this is a last-resort fallback; per DI-017 all events should have trace_id |

## Related BCs

- BC-1.14.001 — events `plugin.async_block_discarded` and `plugin.timeout` (async) originate from the dispatch loop defined there; this BC provides the SS-03 catalog authority
- BC-7.06.001 — events `dispatcher.schema_mismatch` and `dispatcher.registry_invalid` originate from the registry validation path defined there; this BC provides the SS-03 catalog authority
- BC-3.07.002 — sibling: `internal.sink_error` event catalog; same pattern (SS-03 catalogues events emitted by other subsystems)

## Architecture Anchors

- `crates/factory-dispatcher/src/main.rs` (call sites) + `crates/factory-dispatcher/src/host/emit_event.rs` (emit fns) — async block discard path; timeout termination path; `plugin.abandoned` emission path (drain timer arm, EC-011 break); `plugin.completed` (async path) emission path (drain result arm, mirroring sync `emit_lifecycle` in `crates/factory-dispatcher/src/executor.rs`)
- `crates/factory-dispatcher/src/registry.rs` — schema_mismatch and registry_invalid emission sites
- `crates/sink-core/src/` — FileSink fan-out path for all five event types
- VP-028 — sink fan-out invariant verification

## Story Anchor

TBD — single story per ADR-019 §6 (no phased rollout, user decision 2026-05-07)

## VP Anchors

- VP-079 — Payload schema conformance for all six event types including `plugin.abandoned` and `plugin.completed` (async path): each mandatory field is
  present, non-null, and the `type` string matches the catalogued value; verified via
  fault-injection integration test per event-type triggering scenario (integration method, bats)
- VP-028 — Sink fan-out invariant: once emitted, all four event types reach every
  configured accepting sink (independent of VP-079's payload conformance check)

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Async plugin returns exit code 2 (block_intent false by definition — on_error != block) | `plugin.async_block_discarded` emitted with `reason: "async_plugin_block_verdict_discarded"`; dispatcher exit code unchanged |
| EC-002 | Registry with schema_version = 1 loaded | `dispatcher.schema_mismatch` emitted with `found_version: 1`, `expected_version: 2`, `error_code: "E-REG-001"` |
| EC-003 | Registry with schema_version = null (malformed TOML) | `dispatcher.schema_mismatch` emitted with `found_version: null`; dispatcher hard-errors |
| EC-004a | Registry entry has on_error=block AND async=true (AsyncBlockConflict) | `dispatcher.registry_invalid` emitted with `error_code: "E-REG-002"`, `violation: "async_block_conflict"`, `offending_plugin` named; dispatcher refuses to start |
| EC-004b | Two or more registry entries share the same `(name, event, tool)` tuple (DuplicateEntry) | `dispatcher.registry_invalid` emitted with `error_code: "E-REG-003"`, `violation: "duplicate_hook_registration"`, `offending_plugin`/`offending_event`/`offending_tool` set to the duplicating entry's tuple; dispatcher refuses to start |
| EC-005 | Async plugin times out | `plugin.timeout` emitted with `execution_group: "async"`; plugin process terminated; dispatcher exit code unaffected |
| EC-006 | Multiple async plugins time out in same invocation | One `plugin.timeout` event per timed-out plugin (not a single batch event) |
| EC-007 | Drain timer fires with N async plugins still in-flight | N `plugin.abandoned` events emitted (one per in-flight plugin), each with `drain_window_ms` set to the effective drain window value and `entry_index` identifying the registry partition position; no `plugin.completed` events follow for the abandoned plugins in this invocation (Invariant 6) |
| EC-008 | Async plugin completes within drain window with exit code 0 (non-block) | `plugin.completed` (async path) emitted with all mandatory fields present including `entry_index`; no `plugin.abandoned` follows for the same `(trace_id, plugin_name, entry_index)` tuple; dispatcher exit code unchanged (Invariant 3 and Invariant 6) |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Async plugin exits 2 | `plugin.async_block_discarded` event in events-*.jsonl; dispatcher exit 0 | async-block-discard |
| Registry schema_version=1 loaded | `dispatcher.schema_mismatch` event in events-*.jsonl; dispatcher exits non-zero | schema-mismatch |
| Registry entry on_error=block + async=true | `dispatcher.registry_invalid` event in events-*.jsonl; `error_code: "E-REG-002"`, `violation: "async_block_conflict"`; dispatcher refuses to start | registry-invalid-E-REG-002 |
| Registry with duplicate hook name entries | `dispatcher.registry_invalid` event in events-*.jsonl; `error_code: "E-REG-003"`, `violation: "duplicate_hook_registration"`; dispatcher refuses to start | registry-invalid-E-REG-003 |
| Async plugin times out (timeout_ms exceeded) | `plugin.timeout` with `execution_group: "async"` in events-*.jsonl; no impact on dispatcher exit | async-timeout |
| All four original events emitted; FileSink running | All four appear as JSON lines in events-YYYY-MM-DD.jsonl; `trace_id` present on all | fan-out-happy-path |
| Async plugin exits 0 within drain window (non-block) | `plugin.completed` event in events-*.jsonl; `type = "plugin.completed"`; `entry_index` present and correct; `plugin_version` present; no `plugin.abandoned` for same `(trace_id, plugin_name, entry_index)` tuple; dispatcher exit code unchanged | async-completed |
| All async plugins complete before drain timer | No `plugin.abandoned` events in events-*.jsonl | abandoned-none |
| One async plugin still in-flight at drain timer expiry | `plugin.abandoned` event with `drain_window_ms` set, `plugin_name` correct, `entry_index` correct, `trace_id` + `session_id` present; no `plugin.completed` follows for that plugin (Invariant 6) | abandoned-one |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-028 | Sink fan-out invariant — all events reach all configured sinks | integration |
| VP-079 | Payload schema conformance for all six event types including `plugin.abandoned` and `plugin.completed` (async path) — mandatory fields present, non-null, type string correct | integration |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-003 ("Stream observability events to multiple configurable sinks") per capabilities.md §CAP-003 |
| Capability Anchor Justification | CAP-003 ("Stream observability events to multiple configurable sinks") per capabilities.md §CAP-003 — these four event types are observability events that operators and the VSDD engine consume to diagnose async plugin behavior; cataloguing them here fulfills the "stream observability events" promise by defining the wire format and sink-fan-out obligation |
| L2 Domain Invariants | DI-017 — `trace_id` present on every emitted event; all four event types must carry `trace_id`; Invariant 5 of this BC enforces DI-017's requirement that `trace_id` be the canonical wire-field name (not `dispatcher_trace_id`); DI-019 — `ASYNC_DRAIN_WINDOW_MS` (the `plugin.timeout` async path and `plugin.async_block_discarded` events are emitted by tasks running within the drain window bounded by DI-019; VP-079 fixture timing for these events must account for the DI-019 drain window value) |
| Architecture Module | SS-03 — `crates/sink-core/` (event routing); SS-01 — `crates/factory-dispatcher/src/main.rs` + `crates/factory-dispatcher/src/host/emit_event.rs` (emission sites); SS-01 — `crates/factory-dispatcher/src/registry.rs` (schema_mismatch + registry_invalid emission sites). Note: SS-07 owns `plugins/vsdd-factory/hooks-registry.toml` (the file format) but the emission sites in registry.rs are SS-01 Rust modules per ARCH-INDEX. |
| ADR | ADR-019 — Async Semantics at Registry Layer; introduces the conditions that trigger these four events |
| Stories | S-15.01 (single story per ADR-019 §6); S-19.05 (Event 6 — async plugin.completed telemetry) |
| Cycle | v1.0-feature-plugin-async-semantics-pass-1 (F2) |

### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | Adversary pass-1 F-P1-008: BC-1.14.001 + BC-7.06.001 introduce ≥3 new event-type strings without SS-03 catalog amendment; sink consumers may silently drop these events; VP-028 sink-fan-out invariant bypassed |
| **Confidence** | HIGH — all four event types are referenced in F2 BCs but were missing SS-03 catalog authority |
| **Extraction Date** | 2026-05-07 |

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | Event emission: writes to events-*.jsonl via FileSink (filesystem I/O). |
| **Global state access** | FileSink holds shared write state (file handle). |
| **Deterministic** | Event content is deterministic given same inputs; file timestamps vary. |
| **Thread safety** | FileSink is designed for concurrent writes (per BC-3.x contracts). |
| **Overall classification** | Effectful (filesystem I/O); emission is fire-and-once (no retry). |

## Amendment 2026-07-06 (v1.16 → v1.17 — F-P3-013: `entry_index` semantics paragraph corrected; derivation rule stated explicitly; verify-factory-lock example replaced)

**Driver:** Adversary finding F-P3-013 (E-19 pass-3) — the `entry_index` semantics paragraph in Event 5 (`plugin.abandoned`) claimed that "the registry idiom permits multiple entries per `plugin_name`" and cited `verify-factory-lock` as an example (one entry for `Edit|Write|MultiEdit|Agent`, one for `Bash`). Ground truth: those two production registry entries carry DIFFERENT `name` field values (`verify-factory-lock` and `verify-factory-lock-bash`), so they produce distinct `plugin_name` values in any `plugin.abandoned` events and do NOT constitute a name-duplication example. The paragraph also failed to state the derivation rule for `plugin_name` explicitly.

**Changes made:**

1. **Event 5 `entry_index` semantics paragraph** (F-P3-013): Replaced the erroneous `verify-factory-lock` duplication example with: (a) explicit derivation rule — `plugin_name` in `plugin.abandoned` events is the registry entry `name` field verbatim; (b) the registry schema does NOT enforce `name` uniqueness across entries, making name-only keying potentially ambiguous for future registries; (c) a parenthetical note that the production `verify-factory-lock` / `verify-factory-lock-bash` pair carries DIFFERENT names and does not require `entry_index` disambiguation today, but the mechanism exists as a schema-level invariant for future-proof consumers.
2. **Invariant 6** (unchanged): Terminal-semantics key remains `trace_id + plugin_name + entry_index` — no change to the disambiguation tuple.
3. **Frontmatter**: `version: "1.16"` → `"1.17"`; `modified[]` entry added.

**POLICY 1 verification:** All prior content preserved verbatim except the `entry_index` semantics paragraph replacement above.
**POLICY 7 verification:** H1 heading unchanged.
**TD-031 verification:** No new line-number citations introduced.

---

## Amendment 2026-07-06 (v1.15 → v1.16 — F-P2-008: `entry_index` field for `plugin.abandoned`; Invariant 6 tuple key extended)

**Driver:** Adversary finding F-P2-008 (E-19 pass-2) — the `plugin.abandoned` event schema used `plugin_name` alone as the per-entry identifier, but the registry idiom permits multiple entries per `plugin_name` (e.g., `verify-factory-lock` has two entries). Consumers disambiguating abandoned plugins by `(trace_id, plugin_name)` would collapse two distinct in-flight invocations into one signal, making root-cause analysis impossible when both entries abandon in the same drain window.

**Changes made:**

1. **Event 5 wire format** (F-P2-008): `entry_index: <u32>` field added between `plugin_name` and `drain_window_ms`. Mandatory fields list updated to include `entry_index`.
2. **Event 5 `entry_index` semantics paragraph** (F-P2-008): Added after mandatory fields. States: 0-based ordinal from `enumerate()` of the async partition at spawn time; registry allows multiple entries per `plugin_name`; consumers need `(plugin_name, entry_index)` tuple for unambiguous identification.
3. **Invariant 6** (F-P2-008): Terminal-semantics key extended from `trace_id + plugin_name` pair to `trace_id + plugin_name + entry_index` tuple.
4. **EC-007** (F-P2-008): Per-event description updated to mention `entry_index` alongside `drain_window_ms`.
5. **Canonical Test Vectors `abandoned-one` row** (F-P2-008): Updated to assert `entry_index` correct.
6. **Frontmatter** (F-P2-008): `version` bumped `"1.15"` → `"1.16"`; `modified[]` entry added; `last_amended` date retained 2026-07-06.

**POLICY 1 verification:** All prior content preserved verbatim except the six changes listed above.
**POLICY 7 verification:** H1 heading unchanged (entry_index is a field extension, not a new event type).
**TD-031 verification:** No new line-number citations introduced.

---

## Amendment 2026-07-06 (v1.14 → v1.15 — F-P1-013: `plugin.abandoned` event catalog + drain-terminal semantics codified)

**Driver:** Adversary finding F-P1-013 (E-19 pass-1) — the async drain timer arm in `crates/factory-dispatcher/src/main.rs` fires with plugins still in-flight (EC-011) but no SS-03 catalog event covered this condition. Telemetry consumers had no observable signal for abandoned plugins; the abandoned-vs-late-completion race semantics were unspecified, creating implementation ambiguity.

**Semantics decision: option (a) abort-at-drain — abandoned is terminal.**

Three options were evaluated (F-P1-013):

- **(a) abort-at-drain**: `plugin.abandoned` is terminal for this invocation. No `plugin.completed` fires after it. **Selected.**
- **(b) both events may fire**: `plugin.abandoned` then `plugin.completed` (late completion), correlated by `trace_id + plugin_name`. **Rejected.**
- **(c) suppress `plugin.completed`**: Track abandoned set; suppress late completions. **Rejected.**

Option (b) rejected: the dispatcher process exits within milliseconds of the drain timer firing. In Tokio, dropping a `JoinHandle` does not cancel the underlying task — the spawned tasks continue — but the `tokio::main` runtime shutdown drops all pending tasks before they can write to a live FileSink. Additionally, the `rx` channel receiver is dropped at drain timer `break`, so any late result send on `tx_for_task` is silently discarded even in the narrow pre-exit window. No emission path exists for late completions. Option (c) rejected: it is mechanically equivalent to (a) without benefit — the emission path is already closed, so suppression logic would be dead code.

**Changes made:**

1. Frontmatter: `version` bumped `"1.14"` → `"1.15"`; `last_amended` updated to 2026-07-06; `modified[]` array entry added.
2. H1 title: `, \`plugin.abandoned\`` appended to the event list.
3. §Description: "four new event-type strings" replaced with "four new event-type strings (plus `plugin.abandoned` added by F5 E-19 pass-1)"; "catalog entry for all four" → "all five".
4. §Common Fields: table `session_id` row updated "all four" → "all five"; `plugin_name` row updated "(1 + 4)" → "(1, 4, and 5)"; closing paragraph updated — "Plugin-context events (1 + 4)" → "(1, 4, and 5)"; "none of the four" → "none of the five"; "All four event types" → "All five event types".
5. §Postconditions: Event 5 (`plugin.abandoned`) section added after Event 4, including trigger, wire format, mandatory fields, `drain_window_ms` semantics, and forward-reference to Invariant 6. "Sink destination" paragraph: "All four events" → "All five events".
6. §Invariants: Invariant 6 added — `plugin.abandoned` is terminal (option a), with full rationale for rejecting options (b) and (c).
7. §Edge Cases: EC-007 added (drain timer fires with N in-flight plugins).
8. §Canonical Test Vectors: `abandoned-none` and `abandoned-one` rows added.
9. §VP Anchors: VP-079 scope updated to "all five event types including `plugin.abandoned`".
10. §Verification Properties table: VP-079 row scope updated to "all five event types including `plugin.abandoned`".
11. §Architecture Anchors: first bullet extended with `plugin.abandoned` drain timer arm; FileSink bullet "four event types" → "five event types".

**POLICY 1 verification:** All prior content preserved verbatim except the changes listed above.
**POLICY 7 verification:** H1 heading updated to include `plugin.abandoned`; BC-INDEX row updated atomically (BC-INDEX v3.58→v3.59).
**TD-031 verification:** No `main.rs:[0-9]+` line-number citations introduced; stable function/section anchors used throughout (drain timer arm identified by `tokio::select!` section and EC-011 label).

---

## Amendment 2026-05-09 (v1.13 → v1.14 — F5 fix-burst-35 F-P36-001: Traceability Stories TBD→S-15.01)

**F-P36-001 (BC body vs BC-INDEX Stories drift):** Traceability `Stories` row updated from `TBD — single story per ADR-019 §6 (no phased rollout, user decision 2026-05-07)` to `S-15.01 (single story per ADR-019 §6)`. BC-INDEX row (v1.28) already listed S-15.01; source body was pre-F3. F3 story decomposition (PR #106 merged 2026-05-07) is canonical.

## Amendment 2026-05-09 (v1.12 → v1.13 — F-P25-003: emission-site anchors corrected; F-P25-006: duplicate last_amended removed)

**Drivers:**
- **F-P25-003** — §Architecture Anchors bullet 1 cited `engine.rs` as the async block discard path / timeout termination path. Post-merge, the call sites are in `main.rs` (lines 46/423/550) and the emit function registrations are in `host/emit_event.rs`. The §Traceability Architecture Module row similarly cited `engine.rs (emission sites)`. Both corrected to `main.rs + host/emit_event.rs (emission sites)`.
- **F-P25-006** — Frontmatter contained duplicate `last_amended:` fields (lines 5 and 9 both had `last_amended: 2026-05-08`). The duplicate (line 9) was removed; only line 5 retained.

**Changes made:**
1. **§Architecture Anchors bullet 1** (F-P25-003): `crates/factory-dispatcher/src/engine.rs` → `crates/factory-dispatcher/src/main.rs (call sites) + crates/factory-dispatcher/src/host/emit_event.rs (emit fns)`
2. **§Traceability Architecture Module row** (F-P25-003): `SS-01 — crates/factory-dispatcher/src/engine.rs (emission sites)` → `SS-01 — crates/factory-dispatcher/src/main.rs + crates/factory-dispatcher/src/host/emit_event.rs (emission sites)`
3. **Frontmatter** (F-P25-006): duplicate `last_amended:` field removed; `version: "1.12"` → `"1.13"`, `last_amended:` updated to 2026-05-09.

**POLICY 1 verification:** All prior content preserved verbatim.
**POLICY 7 verification:** H1 heading unchanged.
**TD-031 verification:** No line-number citations introduced.

---

## Amendment 2026-05-08 (v1.10 → v1.11 — F-P17-002: §Common Fields plugin_version removed)

**Driver:** **F-P17-002** — §Common Fields table listed `plugin_version` as a field present on all four event types. This was incorrect. None of the four BC-3.08.001 emit functions (`emit_dispatcher_schema_mismatch`, `emit_dispatcher_registry_invalid`, `emit_plugin_async_block_discarded`, `emit_plugin_timeout_async`) in `crates/factory-dispatcher/src/host/emit_event.rs` call `with_plugin_version()`. Only the generic plugin `emit_event` host function (which handles arbitrary plugin-emitted events) enriches with `plugin_version` — and that function is not used by any of the four structured events catalogued in this BC.

The v1.10 amendment (F-P16-005) introduced this error: it rewrote the §Common Fields closing paragraph to state `plugin_name + plugin_version explicitly shown` for Events 1+4. The actual wire examples for Events 1 and 4 show only `plugin_name`; neither shows `plugin_version`. The bats tests do not assert `plugin_version`. The v1.10 amendment paragraph was therefore internally inconsistent with the wire-format examples it purported to describe.

**POLICY 4 verification:** `grep -n "with_plugin_version" crates/factory-dispatcher/src/host/emit_event.rs` finds only one call site: inside the generic `emit_event` host function registration (line 46). None of the four named emit functions for BC-3.08.001 events call `with_plugin_version`.

**Changes made:**

1. **§Common Fields table** (F-P17-002): `plugin_version` row removed. `plugin_name` row description updated to note it is present on plugin-context events (1 + 4) only; absent from dispatcher-startup events (2 + 3).
2. **§Common Fields closing paragraph** (F-P17-002): Corrected to state `plugin_name` only (not `plugin_name + plugin_version`) for Events 1+4; added explicit note that `plugin_version` is NOT emitted by any of the four BC-3.08.001 emit functions.
3. **Frontmatter version:** `"1.10"` → `"1.11"`.

**POLICY 1 verification:** All prior content preserved verbatim except the two §Common Fields changes above. No event IDs renumbered. No wire-format examples changed (they never showed `plugin_version` — this removes the erroneous table row that contradicted them).

**POLICY 7 verification:** H1 heading unchanged.

**TD-031 verification:** No `emit_event.rs:[0-9]+` or `main.rs:[0-9]+` patterns introduced.

**Sync notes for story-writer:** No story task or AC changes required — the wire examples were already correct; only the §Common Fields prose is corrected to match them.

**Sync notes for test-writer:** No bats changes required. Bats tests never asserted `plugin_version` for these events; this amendment aligns the spec with the existing correct test behavior.

---

## Amendment 2026-05-08 (v1.9 → v1.10 — F-P16-002 + F-P16-005: emit_event.rs line citations migrated to stable symbol anchors; §Common Fields paragraph rewritten)

**Drivers:**
- **F-P16-002** — The v1.9 amendment narrative cited specific line numbers within `emit_event.rs` (for the four `.with_session_id` call sites and for the `reserved_fields_rejected` test). Per TD-031, `emit_event.rs:[0-9]+` patterns are prohibited in body text because line numbers drift as code evolves. The §Implementation Notes section similarly cited `emit_event.rs` lines 62-78 for the `RESERVED_FIELDS` constant.
- **F-P16-005** — §Common Fields closing paragraph stated that `plugin_name` and `plugin_version` were "omitted from examples for readability" across all four events. This contradicted Events 1 and 4 wire examples, which DO include `plugin_name` explicitly. The paragraph failed to differentiate between plugin-context events (1 + 4, which carry plugin identity) and dispatcher-startup events (2 + 3, which have no plugin context at startup).

**Changes made:**

1. **v1.9 amendment narrative — O-P15-001 driver bullet** (F-P16-002): Replaced `"lines 162, 193, 243, 289"` with the stable symbol anchor: `"all four emit_dispatcher_* and emit_plugin_*_async functions in crates/factory-dispatcher/src/host/emit_event.rs"`.

2. **§Implementation Notes — RESERVED_FIELDS table heading** (F-P16-002): Replaced `"emit_event.rs lines 62-78"` with `"the RESERVED_FIELDS constant in crates/factory-dispatcher/src/host/emit_event.rs"`.

3. **§Implementation Notes — reserved_fields_rejected sentence** (F-P16-002): Replaced `"(test at line 348)"` with the stable description: `"the reserved_fields_rejected integration test in crates/factory-dispatcher/src/host/emit_event.rs"`.

4. **§Common Fields closing paragraph** (F-P16-005): Rewritten to differentiate plugin-context events (1 + 4, `plugin_name` + `plugin_version` explicitly shown) from dispatcher-startup events (2 + 3, those fields omitted from examples), while clarifying that `trace_id` + `session_id` appear in all four examples per VP-079 and that `ts`/`ts_epoch`/`schema_version` are always emitted but not shown for readability.

5. **Frontmatter:** `version: "1.9"` → `"1.10"`.

**POLICY 1 verification:** All prior content preserved verbatim. No event IDs renumbered. No wire-format examples changed.

**POLICY 7 verification:** H1 heading unchanged.

**TD-031 verification:** No `emit_event.rs:[0-9]+` or `main.rs:[0-9]+` patterns remain in body text.

**Sync notes for story-writer:** No story task or AC changes required — these are spec-narrative and §Common Fields prose fixes only. No mandatory field lists were changed.

---

## Amendment 2026-05-08 (v1.8 → v1.9 — O-P15-001: session_id on wire for all four event types; O-P15-003: RESERVED_FIELDS full enumeration)

**Drivers:**
- **O-P15-001** — Wire-format examples for Events 1, 2, 3-E-REG-002, and 4 did not show the `session_id` field. All four `emit_dispatcher_*` and `emit_plugin_*_async` functions in `crates/factory-dispatcher/src/host/emit_event.rs` call `.with_session_id(&ctx.session_id)`, so `session_id` IS on the wire for all four event types. Only the E-REG-003 example (introduced in v1.8) showed `session_id`. This was a spec omission, not an implementation gap.
- **O-P15-003** — §Implementation Notes discussed only `trace_id` and `dispatcher_trace_id` from the reserved-fields filter. The full `RESERVED_FIELDS` constant in `crates/factory-dispatcher/src/host/emit_event.rs` contains 9 fields; the remaining 7 (`session_id`, `plugin_name`, `plugin_version`, `ts`, `ts_epoch`, `schema_version`, `type`) were invisible to spec readers.

**Changes made:**

1. **§Common Fields section added** (O-P15-001, O-P15-003): New section between Preconditions and Postconditions. Enumerates all 8 dispatcher-owned fields present on every event (`trace_id`, `session_id`, `plugin_name`, `plugin_version`, `ts`, `ts_epoch`, `schema_version`, `type`). States that `session_id` and `trace_id` appear explicitly in wire-format examples because VP-079 verifies them; remaining fields present but omitted from examples for readability.

2. **Event 1 (`plugin.async_block_discarded`) wire-format example** (O-P15-001): `session_id` field added between `trace_id` and `plugin_name`. Mandatory fields paragraph updated: `session_id` added.

3. **Event 2 (`dispatcher.schema_mismatch`) wire-format example** (O-P15-001): `session_id` field added between `trace_id` and `found_version`. Mandatory fields paragraph updated: `session_id` added.

4. **Event 3 E-REG-002 (`dispatcher.registry_invalid`) wire-format example** (O-P15-001): `session_id` field added between `trace_id` and `offending_plugin`. (E-REG-003 example already showed `session_id` per v1.8; unchanged.)

5. **Event 4 (`plugin.timeout` async path) wire-format example** (O-P15-001): `session_id` field added between `trace_id` and `plugin_name`. Mandatory fields paragraph updated: `session_id` added.

6. **§Implementation Notes — RESERVED_FIELDS subsection extended** (O-P15-003): Subsection heading updated to cite O-P15-003. Full 9-field enumeration table added. Cross-reference to HOST_ABI.md and implementation test added.

7. **Frontmatter:** `version: "1.8"` → `"1.9"`.

**POLICY 1 verification:** All prior content preserved verbatim. No event IDs renumbered. E-REG-003 wire-format example unchanged.

**POLICY 7 verification:** H1 heading unchanged.

**POLICY 12 (TV emitter consistency):** Canonical Test Vectors table rows assert `trace_id present on all` (fan-out-happy-path row) — this coverage already included `session_id` implicitly via the Common Fields definition. No TV row changes required: VP-079's payload conformance check now has the §Common Fields section as the authoritative field list, and the test vectors continue to exercise the same triggering scenarios.

**Sync notes for story-writer:** BC-3.08.001 v1.9 changes are spec-only (wire examples and notes); no story task changes required. If any story task currently references "mandatory fields" from the pre-v1.9 wire examples, those task bodies should be updated to include `session_id` in each event's field list.

**Sync notes for test-writer:** Bats tests that assert `plugin.async_block_discarded`, `dispatcher.schema_mismatch`, `dispatcher.registry_invalid` (E-REG-002 path), and `plugin.timeout` events MUST be updated to assert `session_id` is present and non-null in the emitted JSON. The E-REG-003 path already required `session_id` per v1.8 — only the other three event types need test updates.

---

## Amendment 2026-05-08 (v1.7 → v1.8 — F-P14-001 Path B: E-REG-003 wire schema extended with offending_event + offending_tool)

**Driver:** F-P14-001 Path B — Cross-BC contradiction since fix-burst-7: BC-7.06.001 v1.8 §E-REG-NNN Error Code Table declares the authoritative E-REG-003 `dispatcher.registry_invalid` payload as including `offending_plugin`, `offending_event`, `offending_tool`, `violation`, `timestamp`, `error_code`. BC-3.08.001 v1.7 Event 3 E-REG-003 wire-format example only enumerated `offending_plugin`, omitting `offending_event` and `offending_tool`. The data already exists in `RegistryError::DuplicateEntry { name, event, tool }` — the divergence was a spec-level omission, not an implementation gap.

**User-decided resolution:** Path B — extend BC-3.08.001 wire schema to match BC-7.06.001's authoritative enrichment. (Path A would have stripped the fields from BC-7.06.001 v1.8; Path B was chosen because the extra fields carry diagnostic value and are already present in the Rust error variant.)

**Changes made:**

1. **Event 3 E-REG-003 wire-format example extended** (F-P14-001): JSON example now includes `session_id`, `offending_event` (string, required), and `offending_tool` (string or null, required). Field ordering normalized to match BC-7.06.001 v1.8 authoritative schema.

2. **Mandatory fields paragraph for E-REG-003 updated**: `offending_event` (string, required) and `offending_tool` (string or null, required) added to the mandatory fields list. `session_id` added. Null semantics for `offending_tool` clarified (null when duplicating entry has no `tool` filter).

3. **E-REG-002 vs E-REG-003 asymmetry note added** (F-P14-001): Explains why E-REG-002 does NOT carry `offending_event`/`offending_tool` (intra-entry violation) while E-REG-003 does (inter-entry violation — the tuple identifies the duplicating entry). Cites implementation anchor: `RegistryError::DuplicateEntry { name, event, tool }`.

4. **EC-004b updated**: "offending_plugin set to first duplicate entry name" replaced with "offending_plugin/offending_event/offending_tool set to the duplicating entry's tuple" (F-P14-001 POLICY 12 TV emitter consistency).

5. **Frontmatter:** `version: "1.7"` → `"1.8"`.

**Sync notes for implementer**: The event emission site for E-REG-003 in `factory_dispatcher::main::run` (or `registry.rs::validate()`) must accept the full `RegistryError::DuplicateEntry { name, event, tool }` destructure and propagate all three fields to the structured event. The emit call signature must accept `event: &str` and `tool: Option<&str>` (already present in the `DuplicateEntry` variant). No new fields are required in `RegistryError` itself — only the event emission path needs updating to include `offending_event` and `offending_tool` alongside the existing `offending_plugin`.

**Sync notes for test-writer**: Bats S8 tests that assert `dispatcher.registry_invalid` for E-REG-003 MUST be updated to assert all three fields: `offending_plugin`, `offending_event`, and `offending_tool`. A test vector that asserts only `offending_plugin` is now insufficient. The `offending_tool` field must be `null` when the duplicating entry has no `tool` filter and a regex string otherwise.

**POLICY 1 verification:** All prior content preserved verbatim. E-REG-002 wire-format example unchanged (asymmetry note explains why it remains field-minimal). No event IDs renumbered.

**POLICY 7 verification:** H1 heading unchanged.

**F-P14-001 cross-reference:** BC-7.06.001 v1.8 §Sibling BC-3.08.001 cross-reference at line 204 referenced "BC-3.08.001 v1.7 lines 107-117" as the SS-03 catalog mirror. That note is now superseded: BC-3.08.001 v1.8 Event 3 (E-REG-003 wire format section) is the updated SS-03 catalog mirror, and the two BCs are now consistent.

---

## Amendment 2026-05-08 (v1.7 — O-P10-001: phase frontmatter corrected F8 → F2)

**Driver:** O-P10-001 metadata defect — frontmatter field `phase:` contained `F8`, a value that appears nowhere else in the F-series BC corpus. All sibling BCs produced in the same feature cycle (BC-7.06.001, BC-1.14.001, BC-9.01.006) declare `phase: F2`. This was a typo introduced when the frontmatter was last written.

**Change:** `phase: F8` → `phase: F2` (line 9). No body content altered. No version bump — pure metadata correction per project convention for frontmatter-only fixes.

---

## Amendment 2026-05-08 (v1.6 → v1.7 — F-P8-001 sibling: Event 3 E-REG-003 added; violation string canonicalized)

**Driver:** F-P8-001 PO sync — BC-7.06.001 v1.6 (amended in the same burst) establishes that `dispatcher.registry_invalid` has two valid error codes: `E-REG-002` (async block conflict) and `E-REG-003` (duplicate hook registration). BC-3.08.001 v1.6 only enumerated E-REG-002 in Event 3, omitting E-REG-003 entirely. Additionally, the canonical violation string for E-REG-002 was normalized in BC-7.06.001 from the legacy value `"on_error_block_with_async_true"` to `"async_block_conflict"`; BC-3.08.001 v1.6 still carried the legacy string.

**Changes made:**

1. **Event 3 wire-format section expanded** — trigger description now states both violation conditions. A two-row enum table lists `E-REG-002 / async_block_conflict` and `E-REG-003 / duplicate_hook_registration` with their triggering conditions. Two wire-format examples are provided (one per error code). The mandatory fields paragraph clarifies that `error_code` is an enum with exactly these two valid values and that `violation` is determined by `error_code`.

2. **Canonical violation string normalized** — `"on_error_block_with_async_true"` replaced by `"async_block_conflict"` throughout Event 3 (wire format example, EC table, test vectors). This matches BC-7.06.001 v1.6 as the canonical authority. **Bats tests for S8 and any test file asserting `"on_error_block_with_async_true"` MUST be updated to `"async_block_conflict"` before delivery.**

3. **Edge Cases table** — EC-004 split into EC-004a (AsyncBlockConflict / E-REG-002) and EC-004b (DuplicateEntry / E-REG-003).

4. **Canonical Test Vectors table** — the single `registry-invalid` row split into `registry-invalid-E-REG-002` and `registry-invalid-E-REG-003` rows.

5. **Frontmatter** — `version:` bumped `"1.6"` → `"1.7"`.

No other postconditions, invariants, or verification properties were modified. POLICY 1 (append-only, no event renumbering) and POLICY 7 (H1 unchanged) observed.

## Amendment 2026-05-08 (v1.5 → v1.6 — F5 fix-burst-2 F-P2-015: last_amended frontmatter format normalized)

**Driver:** F5 pass-2 finding F-P2-015 — the `last_amended:` frontmatter field contained an embedded parenthetical annotation `(v1.5 — F5 pass-1 fix-burst F-P1-007)` appended to the date string. This non-standard format may break date-parsing tooling that expects a bare ISO-8601 date value in this field.

**Changes made:**
- Frontmatter `last_amended:` cleaned to bare date `2026-05-08` (parenthetical removed).
- Parenthetical content ("v1.5 — F5 pass-1 fix-burst F-P1-007") is preserved in the §Amendment 2026-05-08 (v1.4 → v1.5) changelog entry below, where it already appeared as the section title.
- Frontmatter `version:` bumped `"1.5"` → `"1.6"`.

No behavioral, wire-format, invariant, or test-vector content was changed. This is a frontmatter hygiene fix only.

## Amendment 2026-05-08 (v1.4 → v1.5 — F5 pass-1 fix-burst F-P1-007)

Addresses adversary F5-pass-1 finding F-P1-007 (trace_id duality on wire).

**F-P1-007 (trace_id is the canonical wire-field name)**: The implementation in `crates/factory-dispatcher/src/host/emit_event.rs` (the `with_trace_id` call path) emitted both `dispatcher_trace_id` (via `with_trace_id(...)`) and `trace_id` (via `with_field("trace_id", ...)`) on the wire. This created an ABI inconsistency: sink consumers may parse one or the other; RESERVED_FIELDS included `dispatcher_trace_id` but not `trace_id`, allowing plugins to spoof the trace correlation field.

User-approved resolution (option b): code emits only `trace_id`; `dispatcher_trace_id` removed from wire output; `trace_id` added to RESERVED_FIELDS.

The wire format examples in §Postconditions (Events 1-4) already showed `trace_id` only — no change needed there. The spec was correct; the implementation deviated.

**Changes made:**
- Frontmatter `version:` bumped to `"1.5"`; `last_amended:` updated to `2026-05-08`
- Invariant 5 added: `trace_id` is the exclusive wire-format field name; `dispatcher_trace_id` MUST NOT appear on wire; plugins MUST NOT emit `trace_id` via `with_field()` (it is dispatcher-reserved)
- `## Implementation Notes` section added with `### RESERVED_FIELDS and trace_id` subsection: explicit guidance that `trace_id` must be added to RESERVED_FIELDS; `dispatcher_trace_id` retained in RESERVED_FIELDS for defense-in-depth; plugins attempting to set either field are silently stripped
- Traceability L2 Domain Invariants: DI-017 entry expanded to note that Invariant 5 enforces the canonical wire-field-name requirement

**Canonical Test Vectors**: All existing test vectors show `trace_id` only (no `dispatcher_trace_id`); no changes required. Wire format examples in Postconditions §Events 1-4 already compliant.

## Amendment 2026-05-07 (v1.3 → v1.4 — F2 pass-10 NIT-P10-001 cleanup; sibling fix to F-P7-004)

**Redundant `(per DI-019)` parenthetical removed from Traceability L2 Domain Invariants cell.** The DI-019 entry in the cell already began with `DI-019 —` prefix, making the trailing `(per DI-019)` redundant. Pass-7 F-P7-004 fixed exactly this pattern in the sibling BC-1.14.001 (v1.5→v1.6) but the parallel fix in BC-3.08.001 was not applied at that time.

This is a cosmetic-only change. No postconditions, wire formats, invariants, test vectors, or verification properties were modified.

## Amendment 2026-05-07 (v1.2 → v1.3 — F2 pass-6 F-P6-003: inline literal removed; sibling-fix to F-P4-005)

**DI-019 inline literal removed from Traceability L2 Domain Invariants cell.** The text `ASYNC_DRAIN_WINDOW_MS = 100 ms` was an inlined value literal that violated the DI-019 canonical-value rule (same pattern removed from BC-1.14.001 v1.4 → v1.5 in the F2 pass-4 burst). Replaced with `ASYNC_DRAIN_WINDOW_MS (per DI-019)` — DI-019 is the single source of truth for the numeric value.

This is a traceability-text-only change. No postconditions, wire formats, invariants, test vectors, or verification properties were modified.

## Amendment 2026-05-07 (v1.1 → v1.2 — F2 pass-3 user-correction: DI-019 cross-reference added)

**DI-019 traceability cross-reference added** per user-directed structural correction (same burst as BC-1.14.001 v1.3 → v1.4 and invariants.md v1.4 → v1.5).

DI-019 (`ASYNC_DRAIN_WINDOW_MS = 100 ms`) was lifted from BC-1.14.001 v1.3's inline "Constant Definitions" table to a domain invariant. Two of the four event types catalogued in this BC (`plugin.timeout` async path and `plugin.async_block_discarded`) are emitted by async tasks running within the drain window bounded by DI-019. The Traceability L2 Domain Invariants field now cites DI-019 alongside DI-017 to make this dependency explicit.

This is a traceability-only change. No postconditions, wire formats, invariants, or test vectors were modified.

**Architect obligation:** VP-079 fixture timing (which verifies that these events reach FileSink before dispatcher exit) must anchor to DI-019 for the drain window budget. This is unchanged from the VP-079 obligation noted in BC-1.14.001 v1.3.

## Amendment 2026-05-07 (v1.0 → v1.1 — F2 pass-2 fix burst)

Addresses adversary pass-2 finding F-P2-010.

**F-P2-010 (Architecture Module misclassification)**: Traceability Architecture Module field previously listed "SS-07 — `crates/factory-dispatcher/src/registry.rs`" for the schema_mismatch + registry_invalid emission sites. Per ARCH-INDEX, `crates/factory-dispatcher/src/registry.rs` is owned by SS-01 (the SS-01 row in ARCH-INDEX explicitly lists `{main,registry,routing,executor,invoke,engine,plugin_loader,payload}.rs`). SS-07 owns `plugins/vsdd-factory/hooks/*.sh` and `hooks-registry.toml` (the file format) — not `registry.rs` (the Rust module that reads it). Updated to "SS-01 — `crates/factory-dispatcher/src/registry.rs`" with a clarifying note that SS-07 still owns the TOML file format. This is a POLICY 6 (architecture_is_subsystem_name_source_of_truth) HIGH severity fix.

## Amendment 2026-05-08 (v1.11 → v1.12 — F-P23-002: cross-subsystem source-line-cite migrated to stable symbol anchor)

**Driver:** F-P23-002 pass-23 cross-subsystem corpus sweep (per L-P20-001 / L-P22-001 broadest scope mandate) — §Implementation Notes §RESERVED_FIELDS note cited `HOST_ABI.md (line 267)`. This references a line number in `crates/hook-sdk/HOST_ABI.md` which drifts as the doc evolves. Per TD-VSDD-091, source-file line cites must migrate to stable symbol anchors. The §`emit_event` section is the stable anchor.

**Changes made:**
- §Implementation Notes §RESERVED_FIELDS cross-reference note: `(line 267)` → `(§\`emit_event\` enrichment description; source-line carve-out per TD-VSDD-091: line 267 is unstable, stable anchor is §\`emit_event\` section)`.
- Frontmatter `version:` bumped `"1.11"` → `"1.12"`.

**Changelog:**

| v1.18 | 2026-07-07 | product-owner | E-19 pass-5 PO fix burst (F-P5-003): Event 6 `plugin.completed` (async path) added to catalog — async-path variant for async plugins that complete within the drain window. Wire schema mirrors sync-path `emit_lifecycle` in `crates/factory-dispatcher/src/executor.rs` (`trace_id`, `session_id`, `plugin_name`, `plugin_version`, `elapsed_ms`, `fuel_consumed`, `exit_code`, `stderr` conditional) plus `entry_index: u32` for Invariant 6 tuple parity with Event 5. Invariant 6 updated to state mutual exclusivity: abandoned↔completed are terminal alternatives for any (trace_id, plugin_name, entry_index) tuple. Invariants 1 and 3 count corrected four→six. §Common Fields updated: plugin_name present on events (1,4,5,6); plugin_version note split — Events 1/4/5 do NOT emit plugin_version (original emit_event.rs fns); Event 6 DOES (mirrors executor.rs emit_lifecycle). §Description, §Sink destination, §EC-008, §async-completed test vector, §VP Anchors, §Verification Properties, §Architecture Anchors, §Traceability Stories updated. BC-INDEX v3.70→v3.71. |
| v1.17 | 2026-07-06 | product-owner | E-19 pass-3 PO finalization (F-P3-013): `entry_index` semantics paragraph in Event 5 corrected — derivation rule stated explicitly: `plugin_name` in `plugin.abandoned` events = registry entry `name` field verbatim. Example replaced: the two production `verify-factory-lock` entries carry DIFFERENT `name` values (`verify-factory-lock` vs `verify-factory-lock-bash`) so they do NOT constitute a name-duplication example. Paragraph now cites the schema-level invariant: registry does NOT enforce `name` uniqueness, so future registries may have duplicate `name` entries that make name-only keying ambiguous — `entry_index` is the disambiguation mechanism. Invariant 6 terminal-semantics key (`trace_id + plugin_name + entry_index`) UNCHANGED. |
| v1.16 | 2026-07-06 | product-owner | F-P2-008 fix burst (product-owner): Event 5 `plugin.abandoned` schema extended with mandatory field `entry_index: u32` (ordinal position of registry entry in async partition at spawn time; enumerate() order; disambiguates multiple entries per plugin_name — e.g., verify-factory-lock has 2 entries). Mandatory fields list updated. Invariant 6 terminal-semantics key extended: `trace_id+plugin_name` → `trace_id+plugin_name+entry_index`. EC-007 and abandoned-one test vector updated. Closes F-P2-008. BC-INDEX v3.60→v3.61. |
| v1.15 | 2026-07-06 | product-owner | F-P1-013: Event 5 `plugin.abandoned` added to catalog; drain-terminal semantics codified (option a); Invariant 6; EC-007; abandoned-none/one test vectors; VP-079 scope updated; §Architecture Anchors updated. Closes F-P1-013. |
| v1.14 | 2026-05-09 | state-manager | F-P36-001: Traceability Stories row updated TBD → S-15.01 (F3 story decomposition propagation). |
| v1.13 | 2026-05-09 | implementer | F-P25-003: §Architecture Anchors bullet 1 corrected to main.rs + host/emit_event.rs (emission sites); §Traceability Architecture Module row corrected. F-P25-006: duplicate last_amended frontmatter field removed. |
| v1.12 | 2026-05-08 | state-manager | F-P23-002 cross-subsystem sweep: HOST_ABI.md line cite migrated to stable §`emit_event` section anchor per TD-VSDD-091. |

---

## Amendment 2026-07-07 (v1.17 → v1.18 — F-P5-003: `plugin.completed` (async path) Event 6 catalog entry added)

**Driver:** Adversary finding F-P5-003 (E-19 pass-5) — S-19.05 emits a new `plugin.completed` event for async plugins that complete within the drain window, but BC-3.08.001 v1.17 catalogues no such event. The story's AC-001 BC Trace cited "BC-3.08.001 v1.17 (Event 3 schema)" — an error on two counts: Event 3 is `dispatcher.registry_invalid`, and `plugin.completed` was absent from the catalog entirely. The story's Architecture Mapping stated "plugin.completed already exists in the sync path; confirm async variant uses same schema" without citing an owning BC. Ground-truth search (`grep -rl "plugin.completed" .factory/specs/behavioral-contracts/`) found only BC-3.02.008 (test fixture references) and BC-3.08.001 (Invariant 6 reference) — neither owned the wire schema for the async variant.

**Schema derivation:** Field set derived from `crates/factory-dispatcher/src/executor.rs::emit_lifecycle` for `PluginResult::Ok { exit_code, elapsed_ms, fuel_consumed, stderr }`: `trace_id`, `session_id`, `plugin_name`, `plugin_version`, `elapsed_ms`, `fuel_consumed`, `exit_code`, `stderr` (conditional on non-empty). `entry_index: u32` added for Invariant 6 tuple parity: the abandoned↔completed mutual-exclusion check requires the same `(trace_id, plugin_name, entry_index)` key on both events.

**Option chosen:** Option (a) — amend BC-3.08.001 v1.17→v1.18 to add `plugin.completed` (async path) as Event 6. No other existing BC owns this wire schema; BC-3.08.001 is the SS-03 catalog authority for async-semantics events; Event 6 is the natural complement to Event 5 under Invariant 6.

**Changes made:**

1. **Frontmatter** (F-P5-003): `version` bumped `"1.17"` → `"1.18"`; `last_amended` updated to 2026-07-07; `modified[]` entry added.
2. **H1 title** (F-P5-003): `, \`plugin.completed\` (async path)` appended to event list. (POLICY 7: H1 updated; BC-INDEX row mirrors H1.)
3. **§Description** (F-P5-003): `plugin.completed` (async path) cite added for F-P5-003 fix burst; "five" → "six" in catalog-entry count.
4. **§Common Fields** (F-P5-003): `plugin_name` row updated — present on events (1, 4, 5, and 6). Closing paragraph split: Events 1/4/5 do NOT emit `plugin_version` (original emit_event.rs fns); Event 6 DOES (mirrors executor.rs emit_lifecycle). "All five" → "All six" in trace_id+session_id line.
5. **§Sink destination paragraph** (F-P5-003): "All five events" → "All six events".
6. **Event 6 postcondition section added** (F-P5-003): trigger, wire format (JSON), mandatory fields list, `entry_index` semantics, Invariant 6 interplay note.
7. **Invariant 1** (backfill — stale since v1.15): "all four event types" → "all six event types".
8. **Invariant 3** (backfill — stale since v1.15): "All four" → "All six"; enumeration extended to include `plugin.abandoned` and `plugin.completed` (async path).
9. **Invariant 6** (F-P5-003): Updated to explicitly state mutual exclusivity in both directions: `plugin.abandoned` and `plugin.completed` (async path) are terminal alternatives; reference to Event 6 added.
10. **EC-008** (F-P5-003): Edge case for async plugin completing within drain window.
11. **§Canonical Test Vectors — `async-completed` row** (F-P5-003): Test vector for async plugin exit 0 within drain window.
12. **§VP Anchors** (F-P5-003): VP-079 scope updated to "all six event types including `plugin.abandoned` and `plugin.completed` (async path)".
13. **§Verification Properties table** (F-P5-003): VP-079 row scope updated.
14. **§Architecture Anchors** (F-P5-003): First bullet extended with `plugin.completed` (async path) emission path anchor.
15. **§Traceability Stories** (F-P5-003): S-19.05 added as anchor story for Event 6.

**POLICY 1 verification:** All prior content preserved verbatim except the changes listed above. No event IDs renumbered. Events 1–5 wire-format examples unchanged.
**POLICY 7 verification:** H1 heading updated to include `plugin.completed` (async path); BC-INDEX row mirrors H1 (BC-INDEX v3.71).
**TD-031 verification:** No `executor.rs:[0-9]+` or `main.rs:[0-9]+` line-number citations introduced; stable function/section anchors used throughout.

**VP-079 staleness flag (architect routing required):** VP-079 currently enumerates "five event types" and its mandatory-fields table does not include `plugin.completed` (async path). Adding Event 6 makes VP-079 stale on: (a) event-count prose ("five" → "six"); (b) mandatory-fields table (missing Event 6 row); (c) production caller-site list (missing the async drain result arm). Orchestrator should route VP-079 amendment to `vsdd-factory:architect`.

**VP-100 cite staleness flag (architect routing required):** VP-100 `source_bc:` field cites "BC-3.08.001 v1.16 Invariant 6" and `input-hash: "1072e05"` will be stale once this amendment is persisted. The conceptual correctness of VP-100 is unaffected (Invariant 6 strengthened, not changed); only the version pin and input-hash need updating. Orchestrator should route VP-100 frontmatter update to `vsdd-factory:architect`.

---

## Amendment 2026-07-07 (v1.18 → v1.19 — F-P7-007: `entry_index` clarified as schema-level defense, not runtime dispatch gate)

**Driver:** Adversary finding F-P7-007 (E-19 pass-7) — the `entry_index` semantics paragraphs in Event 5 and Event 6, and Invariant 6, described `entry_index` disambiguation for the concurrent-same-`plugin_name` scenario without clarifying that this scenario has no production dispatch path and that `entry_index` correctness is a schema-level property verified by serialization/property tests, not a runtime concurrent-dispatch fixture. S-19.05's AC-002 concurrent clause implied a runtime test fixture path that does not exist.

**Ruling: schema-level defense.** The dispatcher's `registry.rs` enforces `(name, event, tool)` tuple uniqueness (E-REG-003). Two same-named entries require different `event` or `tool` values; co-occurring in a single async partition dispatch requires both to match the same event+tool invocation simultaneously — no production registry entry pair does this. `entry_index` is the natural 0-based ordinal from `enumerate()` of the async partition slice, emitted by the spawn loop without any additional runtime gate. Its correctness as a consumer disambiguation key is a schema-level invariant: the only thing that needs verifying is that the ordinal is correctly marshalled from the spawn loop's `enumerate()` into the event payload — a property test over the struct. No runtime concurrent-dispatch fixture exercises two same-named entries because that scenario is not reachable in any registered production plugin pair.

**S-19.05 AC-002 concurrent clause ruling for story-writer:** Replace any runtime concurrent-dispatch fixture for same-`plugin_name` disambiguation with: "Property test over the `plugin.completed` and `plugin.abandoned` event structs asserting (a) `entry_index` field type is `u32`, (b) the ordinal value in the emitted event equals the `enumerate()` index of the entry in the async partition slice, verified by constructing a synthetic partition of N entries and asserting each emitted event's `entry_index` matches its position."

**Changes made:**

1. **Frontmatter** (F-P7-007): `version` bumped `"1.18"` → `"1.19"`; `last_amended` updated to 2026-07-07; `modified[]` entry added.
2. **Event 5 `entry_index` semantics paragraph** (F-P7-007): Added explicit schema-level-defense paragraph after the existing parenthetical — states the concurrent-same-`plugin_name` scenario has no production dispatch path; `entry_index` is a schema-level defense verified by serialization/property tests, not a runtime concurrent-dispatch fixture; runtime path covered by normal integration tests.
3. **Event 6 `entry_index` semantics paragraph** (F-P7-007): Added final sentence mirroring the schema-level defense principle for `plugin.completed`.
4. **Invariant 6** (F-P7-007): Added schema-level note at end — the `(trace_id, plugin_name, entry_index)` mutual-exclusivity key is a schema-level predicate verifiable by property/serialization tests, not a runtime concurrent-dispatch fixture.

**POLICY 1 verification:** All prior content preserved verbatim except the four additions above. No event IDs renumbered. No wire-format examples changed.
**POLICY 7 verification:** H1 heading unchanged (no new event type; this is a semantics clarification).
**TD-031 verification:** No `registry.rs:[0-9]+` line-number citations introduced; stable function/symbol anchors used (`registry.rs` validation function, `(name, event, tool)` uniqueness invariant, E-REG-003 error code).
