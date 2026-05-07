---
document_type: behavioral-contract
level: L3
version: "1.0"
status: draft
producer: product-owner
timestamp: 2026-05-07T00:00:00Z
last_amended: null
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

## Postconditions

### Event 1: `plugin.async_block_discarded`

**Trigger**: An async group plugin (one with `async = true` in the registry) returns exit code 2 (which would be a block verdict if it were in the sync group). Because async plugins cannot have `on_error = "block"` (BC-1.14.001 Invariant 4), the block intent is structurally invalid and is discarded rather than reaching Claude Code.

**Wire format** (JSON line in `events-*.jsonl`):

```json
{
  "type": "plugin.async_block_discarded",
  "trace_id": "<uuid-v4>",
  "plugin_name": "<string — registry entry name>",
  "exit_code": 2,
  "timestamp": "<ISO-8601>",
  "reason": "async_plugin_block_verdict_discarded"
}
```

**Mandatory fields**: `type`, `trace_id`, `plugin_name`, `exit_code`, `timestamp`, `reason`.

### Event 2: `dispatcher.schema_mismatch`

**Trigger**: Dispatcher loads `hooks-registry.toml` and finds `schema_version != 2` (e.g., `schema_version = 1` or any unknown value). Dispatcher hard-errors and emits this event before exiting.

**Wire format** (JSON line in `events-*.jsonl`):

```json
{
  "type": "dispatcher.schema_mismatch",
  "trace_id": "<uuid-v4>",
  "found_version": <integer or null>,
  "expected_version": 2,
  "timestamp": "<ISO-8601>",
  "error_code": "E-REG-001"
}
```

**Mandatory fields**: `type`, `trace_id`, `found_version`, `expected_version`, `timestamp`, `error_code`.

### Event 3: `dispatcher.registry_invalid`

**Trigger**: Dispatcher's `registry.rs::validate()` finds a registry entry with both `on_error = "block"` AND `async = true`. Dispatcher emits this event and hard-errors with `E-REG-002`.

**Wire format** (JSON line in `events-*.jsonl`):

```json
{
  "type": "dispatcher.registry_invalid",
  "trace_id": "<uuid-v4>",
  "offending_plugin": "<string — name of the plugin entry that violates the invariant>",
  "violation": "on_error_block_with_async_true",
  "timestamp": "<ISO-8601>",
  "error_code": "E-REG-002"
}
```

**Mandatory fields**: `type`, `trace_id`, `offending_plugin`, `violation`, `timestamp`, `error_code`.

### Event 4: `plugin.timeout` (async path)

**Note**: `plugin.timeout` is emitted for both sync and async plugin timeouts. The sync-path behavior is governed by BC-1.14.001. This entry covers the **async-path variant** only: when an async group plugin exceeds its `timeout_ms` and is terminated.

**Wire format** (JSON line in `events-*.jsonl`):

```json
{
  "type": "plugin.timeout",
  "trace_id": "<uuid-v4>",
  "plugin_name": "<string>",
  "execution_group": "async",
  "timeout_ms": <integer>,
  "timestamp": "<ISO-8601>"
}
```

**Mandatory fields**: `type`, `trace_id`, `plugin_name`, `execution_group`, `timeout_ms`, `timestamp`.

**Sink destination**: All four events are routed to `events-*.jsonl` via FileSink. They are NOT routed to the dispatcher-internal debug stream (which is opt-in per DI-007 amended). The VP-028 sink-fan-out invariant applies: if multiple sinks are configured, all four events must fan out to all applicable sinks.

## Invariants

1. **`trace_id` is mandatory on all four event types**: Per DI-017, every emitted event carries the UUID v4 from the invoking hook envelope. These four events are no exception.
2. **Events are write-once, no retry**: These are diagnostic events; partial emission is acceptable (emit-then-crash). They are never retried on FileSink write failure.
3. **Events do not affect dispatcher exit code**: All four are observability-only. `plugin.async_block_discarded` and `plugin.timeout (async)` are logged and forgotten. `dispatcher.schema_mismatch` and `dispatcher.registry_invalid` accompany a hard exit (non-zero) but the event itself does not cause the exit — the validation failure does.
4. **`plugin.async_block_discarded` reason field is the literal string `"async_plugin_block_verdict_discarded"`**: Not an error code; a diagnostic reason string for human-readable log inspection.

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

- `crates/factory-dispatcher/src/engine.rs` — async block discard path; timeout termination path
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
| EC-004 | Registry entry has on_error=block AND async=true | `dispatcher.registry_invalid` emitted with `offending_plugin` named; dispatcher refuses to start |
| EC-005 | Async plugin times out | `plugin.timeout` emitted with `execution_group: "async"`; plugin process terminated; dispatcher exit code unaffected |
| EC-006 | Multiple async plugins time out in same invocation | One `plugin.timeout` event per timed-out plugin (not a single batch event) |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Async plugin exits 2 | `plugin.async_block_discarded` event in events-*.jsonl; dispatcher exit 0 | async-block-discard |
| Registry schema_version=1 loaded | `dispatcher.schema_mismatch` event in events-*.jsonl; dispatcher exits non-zero | schema-mismatch |
| Registry entry on_error=block + async=true | `dispatcher.registry_invalid` event in events-*.jsonl; dispatcher refuses to start | registry-invalid |
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
| L2 Domain Invariants | DI-017 — `trace_id` present on every emitted event; all four event types must carry `trace_id` |
| Architecture Module | SS-03 — `crates/sink-core/` (event routing); SS-01 — `crates/factory-dispatcher/src/engine.rs` (emission sites); SS-07 — `crates/factory-dispatcher/src/registry.rs` (schema_mismatch + registry_invalid emission sites) |
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
