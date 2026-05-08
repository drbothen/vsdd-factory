---
document_type: behavioral-contract
level: L3
version: "1.7"
status: draft
producer: product-owner
timestamp: 2026-05-07T00:00:00Z
last_amended: 2026-05-08
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
  "offending_plugin": "<string — name of the first duplicate plugin entry>",
  "violation": "duplicate_hook_registration",
  "timestamp": "<ISO-8601>",
  "error_code": "E-REG-003"
}
```

**Mandatory fields**: `type`, `trace_id`, `offending_plugin`, `violation`, `timestamp`, `error_code`.

The `error_code` field is an enum with exactly two valid values: `"E-REG-002"` and `"E-REG-003"`. The `violation` field value is determined by the `error_code` per the table above — no other combinations are valid.

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
5. **`trace_id` is the exclusive wire-format field name for the trace correlation value**: The dispatcher's structured-event wire format uses field name `trace_id` exclusively. The legacy field name `dispatcher_trace_id` MUST NOT appear in the serialized wire output. Plugins MUST NOT emit a `trace_id` field via `with_field()` — `trace_id` is reserved for the dispatcher (see §Implementation Notes). Reference: DI-017 (amended per F-P1-007).

## Implementation Notes

### RESERVED_FIELDS and `trace_id` (F-P1-007)

Implementations MUST add `trace_id` to the host-side reserved-fields filter (e.g., `RESERVED_FIELDS` in `crates/factory-dispatcher/src/host/emit_event.rs`). This prevents plugins from spoofing the dispatcher's trace correlation value via `with_field("trace_id", ...)`.

The reserved-fields filter MUST also retain `dispatcher_trace_id` for backward defense (defense-in-depth): even though the dispatcher no longer emits `dispatcher_trace_id` on the wire, plugins must not be allowed to inject it. Both names are reserved regardless of which name the dispatcher currently uses as the canonical field.

Summary of the required `RESERVED_FIELDS` state after this amendment:
- `trace_id` — added by this amendment; canonical wire field name; dispatcher-owned
- `dispatcher_trace_id` — already present; legacy field; retained for defense-in-depth

Plugins that attempt to set either field via `with_field()` MUST have the field silently stripped by the host-side filter before serialization.

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
| EC-004a | Registry entry has on_error=block AND async=true (AsyncBlockConflict) | `dispatcher.registry_invalid` emitted with `error_code: "E-REG-002"`, `violation: "async_block_conflict"`, `offending_plugin` named; dispatcher refuses to start |
| EC-004b | Two or more registry entries share the same hook name (DuplicateEntry) | `dispatcher.registry_invalid` emitted with `error_code: "E-REG-003"`, `violation: "duplicate_hook_registration"`, `offending_plugin` set to first duplicate entry name; dispatcher refuses to start |
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
| Architecture Module | SS-03 — `crates/sink-core/` (event routing); SS-01 — `crates/factory-dispatcher/src/engine.rs` (emission sites); SS-01 — `crates/factory-dispatcher/src/registry.rs` (schema_mismatch + registry_invalid emission sites). Note: SS-07 owns `plugins/vsdd-factory/hooks-registry.toml` (the file format) but the emission sites in registry.rs are SS-01 Rust modules per ARCH-INDEX. |
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

**F-P1-007 (trace_id is the canonical wire-field name)**: The implementation at `crates/factory-dispatcher/src/host/emit_event.rs:150-157` emitted both `dispatcher_trace_id` (via `with_trace_id(...)`) and `trace_id` (via `with_field("trace_id", ...)`) on the wire. This created an ABI inconsistency: sink consumers may parse one or the other; RESERVED_FIELDS included `dispatcher_trace_id` but not `trace_id`, allowing plugins to spoof the trace correlation field.

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
