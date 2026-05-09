---
document_type: behavioral-contract
level: L3
version: "1.13"
last_amended: 2026-05-09
status: draft
producer: product-owner
timestamp: 2026-05-07T00:00:00Z
phase: F2
inputs:
  - .factory/cycles/v1.0-feature-plugin-async-semantics-pass-1/adversary-pass-1.md
  - .factory/specs/behavioral-contracts/ss-01/BC-1.14.001.md
  - .factory/specs/behavioral-contracts/ss-07/BC-7.06.001.md
input-hash: "[to-be-computed-by-state-manager]"
traces_to: .factory/specs/prd.md
origin: greenfield
extracted_from: null
subsystem: "SS-03"
capability: "CAP-003"
lifecycle_status: active
introduced: v1.0-feature-plugin-async-semantics-pass-1
modified: []
deprecated: null
deprecated_by: null
replacement: null
retired: null
removed: null
removal_reason: null
---

# BC-3.08.001: dispatcher async-semantics event types are catalogued and emitted via FileSink — `plugin.async_block_discarded`, `dispatcher.schema_mismatch`, `dispatcher.registry_invalid`, `plugin.timeout` (async path)

## Description

ADR-019 F2 introduces four new event-type strings as part of the async-semantics feature. These events are referenced in BC-1.14.001 and BC-7.06.001 but require SS-03 catalog authority to define their payload schemas and wire format. Each event is a JSON line written to `events-*.jsonl` via the standard FileSink path. This BC provides the catalog entry for all four, establishing the authoritative field set, wire format, and sink-fan-out obligation per DI-007 (amended: opt-in debug stream) and the VP-028 sink-fan-out invariant.

## Preconditions

1. Dispatcher is running with `schema_version = 2` registry (BC-7.06.001).
2. FileSink is initialized and the `events-YYYY-MM-DD.jsonl` file is writable.
3. The triggering condition for each event type has occurred (see Postconditions for per-event triggers).

## Common Fields

All four event types carry the following dispatcher-owned fields on the wire. These fields are injected by the host (see `emit_event.rs` enrichment path) and are never supplied by plugins (they are RESERVED_FIELDS — see §Implementation Notes):

| Field | Type | Description |
|-------|------|-------------|
| `trace_id` | UUID v4 string | Trace correlation value from the invoking hook envelope (DI-017). Canonical wire-format name; `dispatcher_trace_id` must NOT appear on wire (Invariant 5). |
| `session_id` | UUID v4 string | Claude Code session identifier from the hook envelope context (`ctx.session_id`). Present on all four event types (O-P15-001). |
| `plugin_name` | string | Name of the plugin registry entry, injected by the host. Present on plugin-context events (1 + 4) only; absent from dispatcher-startup events (2 + 3) which have no plugin context. |
| `ts` | string | Emission timestamp (internal format). |
| `ts_epoch` | integer | Emission timestamp as Unix epoch milliseconds. |
| `schema_version` | integer | Registry schema version at emission time. |
| `type` | string | The event type string (e.g. `"plugin.async_block_discarded"`). |

The §Common Fields appear on the wire for ALL four event types except where noted. Wire-format examples in §Postconditions show:
- **Plugin-context events (1 + 4):** `plugin_name` explicitly shown (these are plugin-instance events). `plugin_version` is NOT emitted by these events — none of the four BC-3.08.001 emit functions call `with_plugin_version()`.
- **Dispatcher-startup events (2 + 3):** `plugin_name` OMITTED from examples (no plugin context at dispatcher startup).
- All four event types: `trace_id` + `session_id` explicitly shown (verified by VP-079 payload conformance).
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

**Sink destination**: All four events are routed to `events-*.jsonl` via FileSink. They are NOT routed to the dispatcher-internal debug stream (which is opt-in per DI-007 amended). The VP-028 sink-fan-out invariant applies: if multiple sinks are configured, all four events must fan out to all applicable sinks.

## Invariants

1. **`trace_id` is mandatory on all four event types**: Per DI-017, every emitted event carries the UUID v4 from the invoking hook envelope. These four events are no exception.
2. **Events are write-once, no retry**: These are diagnostic events; partial emission is acceptable (emit-then-crash). They are never retried on FileSink write failure.
3. **Events do not affect dispatcher exit code**: All four are observability-only. `plugin.async_block_discarded` and `plugin.timeout (async)` are logged and forgotten. `dispatcher.schema_mismatch` and `dispatcher.registry_invalid` accompany a hard exit (non-zero) but the event itself does not cause the exit — the validation failure does.
4. **`plugin.async_block_discarded` reason field is the literal string `"async_plugin_block_verdict_discarded"`**: Not an error code; a diagnostic reason string for human-readable log inspection.
5. **`trace_id` is the exclusive wire-format field name for the trace correlation value**: The dispatcher's structured-event wire format uses field name `trace_id` exclusively. The legacy field name `dispatcher_trace_id` MUST NOT appear in the serialized wire output. Plugins MUST NOT emit a `trace_id` field via `with_field()` — `trace_id` is reserved for the dispatcher (see §Implementation Notes). Reference: DI-017 (amended per F-P1-007).

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

- `crates/factory-dispatcher/src/main.rs` (call sites) + `crates/factory-dispatcher/src/host/emit_event.rs` (emit fns) — async block discard path; timeout termination path
- `crates/factory-dispatcher/src/registry.rs` — schema_mismatch and registry_invalid emission sites
- `crates/sink-core/src/` — FileSink fan-out path for all four event types
- VP-028 — sink fan-out invariant verification

## Story Anchor

TBD — single story per ADR-019 §6 (no phased rollout, user decision 2026-05-07)

## VP Anchors

- VP-079 — Payload schema conformance for all four event types: each mandatory field is
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

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Async plugin exits 2 | `plugin.async_block_discarded` event in events-*.jsonl; dispatcher exit 0 | async-block-discard |
| Registry schema_version=1 loaded | `dispatcher.schema_mismatch` event in events-*.jsonl; dispatcher exits non-zero | schema-mismatch |
| Registry entry on_error=block + async=true | `dispatcher.registry_invalid` event in events-*.jsonl; `error_code: "E-REG-002"`, `violation: "async_block_conflict"`; dispatcher refuses to start | registry-invalid-E-REG-002 |
| Registry with duplicate hook name entries | `dispatcher.registry_invalid` event in events-*.jsonl; `error_code: "E-REG-003"`, `violation: "duplicate_hook_registration"`; dispatcher refuses to start | registry-invalid-E-REG-003 |
| Async plugin times out (timeout_ms exceeded) | `plugin.timeout` with `execution_group: "async"` in events-*.jsonl; no impact on dispatcher exit | async-timeout |
| All four events emitted; FileSink running | All four appear as JSON lines in events-YYYY-MM-DD.jsonl; `trace_id` present on all | fan-out-happy-path |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-028 | Sink fan-out invariant — all events reach all configured sinks | integration |
| VP-079 | Payload schema conformance for all four event types — mandatory fields present, non-null, type string correct | integration |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-003 ("Stream observability events to multiple configurable sinks") per capabilities.md §CAP-003 |
| Capability Anchor Justification | CAP-003 ("Stream observability events to multiple configurable sinks") per capabilities.md §CAP-003 — these four event types are observability events that operators and the VSDD engine consume to diagnose async plugin behavior; cataloguing them here fulfills the "stream observability events" promise by defining the wire format and sink-fan-out obligation |
| L2 Domain Invariants | DI-017 — `trace_id` present on every emitted event; all four event types must carry `trace_id`; Invariant 5 of this BC enforces DI-017's requirement that `trace_id` be the canonical wire-field name (not `dispatcher_trace_id`); DI-019 — `ASYNC_DRAIN_WINDOW_MS` (the `plugin.timeout` async path and `plugin.async_block_discarded` events are emitted by tasks running within the drain window bounded by DI-019; VP-079 fixture timing for these events must account for the DI-019 drain window value) |
| Architecture Module | SS-03 — `crates/sink-core/` (event routing); SS-01 — `crates/factory-dispatcher/src/main.rs` + `crates/factory-dispatcher/src/host/emit_event.rs` (emission sites); SS-01 — `crates/factory-dispatcher/src/registry.rs` (schema_mismatch + registry_invalid emission sites). Note: SS-07 owns `plugins/vsdd-factory/hooks-registry.toml` (the file format) but the emission sites in registry.rs are SS-01 Rust modules per ARCH-INDEX. |
| ADR | ADR-019 — Async Semantics at Registry Layer; introduces the conditions that trigger these four events |
| Stories | TBD — single story per ADR-019 §6 (no phased rollout, user decision 2026-05-07) |
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

| v1.13 | 2026-05-09 | implementer | F-P25-003: §Architecture Anchors bullet 1 corrected to main.rs + host/emit_event.rs (emission sites); §Traceability Architecture Module row corrected. F-P25-006: duplicate last_amended frontmatter field removed. |
| v1.12 | 2026-05-08 | state-manager | F-P23-002 cross-subsystem sweep: HOST_ABI.md line cite migrated to stable §`emit_event` section anchor per TD-VSDD-091. |
