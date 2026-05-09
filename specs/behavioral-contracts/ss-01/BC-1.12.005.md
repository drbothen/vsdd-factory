---
document_type: behavioral-contract
level: L3
version: "1.4"
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

# Behavioral Contract BC-1.12.005: factory-dispatcher::host::emit_event::host_field_override_visibility — event.host_overrides field stamped on domain events + vsdd.internal.host_field_override.v1 lifecycle event emitted when host overrides plugin-supplied host-owned fields (ADR-015 D-15.3)

## Description

Per ADR-015 D-15.3, the host stamps all Resource attributes and per-event
identity fields. When a plugin supplies a value for a host-owned field, the
host-stamped value wins unconditionally. This override MUST NOT be silent.
This BC specifies the two-channel override-visibility contract: (1) the inline
`event.host_overrides` field stamped on the domain event itself, and (2) the
rate-limited `vsdd.internal.host_field_override.v1` lifecycle event emitted
for centralized alerting.

This is a future-implementation contract for S-10.02/S-10.03 (Wave 1).
All Canonical Test Vectors describe post-Wave-1 behavior.

## Preconditions

1. The dispatcher has completed startup and Resource attributes are stamped
   (per BC-1.12.003).
2. A plugin calls `host::emit_event` and its event payload includes a value
   for a field that the host also stamps (a host-owned field — any Resource
   attribute field or any per-event identity field listed in ADR-015 D-15.2).
3. The host-stamped value for that field is available at emit time.

## Postconditions

### Channel 1 — inline `event.host_overrides` on the domain event

1. The dispatcher MUST stamp the field `event.host_overrides` on the domain
   event as a string array listing every field name the host overrode in this
   specific emission. Example: `["plugin.version", "service.name"]`.
2. `event.host_overrides` is ABSENT (field not present) when the host overrides
   no fields on that event — i.e., it is absent on events where the plugin
   supplied no conflicting host-owned fields.
3. The overridden field's host-stamped value appears in the emitted event
   record. The plugin-supplied value for that field is discarded and does NOT
   appear in the emitted event.
4. `event.host_overrides` is itself a host-stamped field and cannot be
   overridden by the plugin.

### Channel 2 — `vsdd.internal.host_field_override.v1` lifecycle event

5. The dispatcher MUST emit a `vsdd.internal.host_field_override.v1` lifecycle
   event carrying the attribute `affected.plugin.name = <plugin.name>` when a
   host-owned field is overridden.
6. This lifecycle event is **rate-limited to one emission per unique
   `(plugin.name, field_name)` pair per dispatcher invocation**. Repeated
   overrides of the same `(plugin.name, field_name)` combination within one
   dispatcher invocation produce exactly one lifecycle notice, not one per
   emission.
7. The `vsdd.internal.host_field_override.v1` lifecycle event is written to
   `events-*.jsonl` via the normal single-stream FileSink path (per BC-1.12.001).
   Its `event.category` resolves to `"lifecycle"` per the `vsdd.internal.*`
   prefix in the compile-time registry (per BC-1.12.004).
8. The lifecycle event MUST carry at minimum: `event.name =
   "vsdd.internal.host_field_override.v1"`, `affected.plugin.name`, and
   `overridden.field.name`.

### Channel 3 — stderr warning

9. The dispatcher MUST emit a stderr warning of the form:
   `[vsdd-dispatcher] WARN: plugin '<plugin.name>' supplied host-owned field
   '<field_name>'; host-stamped value takes precedence.`
   This warning is per ADR-015 D-15.3 and makes the override visible in the
   operator's process stderr stream.
10. The stderr warning is NOT rate-limited — it fires once per overridden-field
    per emission. (The rate-limit applies only to the lifecycle event, not stderr.)

## Invariants

1. `event.host_overrides` is ALWAYS absent on events where no field was
   overridden, and ALWAYS present (non-empty array) on events where at least
   one field was overridden.
2. The `vsdd.internal.*` prefix maps to `"lifecycle"` category in the
   compile-time registry (per BC-1.12.004); host_field_override events do NOT
   pollute domain dashboards.
3. The rate-limit state is per-`(plugin.name, field_name)` pair and is reset
   on each dispatcher invocation. A plugin that overrides the same field in a
   subsequent dispatcher invocation receives a new lifecycle notice.
4. A plugin cannot suppress or spoof the `event.host_overrides` field: it is
   always the LAST host-stamp applied, after all domain-field merging.

## Related BCs

- BC-1.12.001 — Single-stream FileSink routing (composes with: lifecycle
  override notices route to the same `events-*.jsonl` stream)
- BC-1.12.003 — Resource attribute startup stamping (depends on: the host-owned
  fields this BC governs are the Resource attributes stamped in BC-1.12.003)
- BC-1.12.004 — Per-event host-stamping (composes with: this BC adds the
  `event.host_overrides` post-stamp step after per-event field merging)
- BC-1.11.003 — `emit_pair` host helper (related: EC-003 of BC-1.11.003
  documents the case where a plugin supplies `event.correlation_id` in
  `emit_pair`; host overrides it and stamps `event.host_overrides` per this BC)

## Architecture Anchors

- `crates/factory-dispatcher/src/host/emit_event.rs` — `event.host_overrides`
  stamping happens in the emit path after domain-field merge; the
  `vsdd.internal.host_field_override.v1` emission happens here too.
  [Stable anchor per TD-VSDD-091; line numbers are not authoritative — use the
  function/module name as the canonical reference.]
- ADR-015 D-15.3 — policy decision for host-field override visibility;
  two-channel approach (inline `event.host_overrides` + lifecycle event with
  `affected.plugin.name`)

## Story Anchor

S-10.02 (Wave 1: FileSink single-stream wiring + per-event stamping)
S-10.03 (Wave 1: Resource-attribute enrichment — establishes the host-owned
fields that this BC's override detection compares against)
S-10.04 (Wave 1: Trace propagation + lifecycle event types — host_field_override emission task implementation T-2/T-3/T-4 per BC-1.12.005 PC5)

## VP Anchors

(TBD — to be assigned after S-10.02/S-10.03 story authoring)

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Plugin supplies `service.name = "my-plugin"` in event payload | Host overrides with `"vsdd-factory"`; emitted event has `event.host_overrides: ["service.name"]`; `vsdd.internal.host_field_override.v1` emitted (once per invocation for this `(plugin.name, "service.name")` pair); stderr warning |
| EC-002 | Plugin supplies NO host-owned fields | `event.host_overrides` is absent from the emitted event; no lifecycle notice; no stderr warning |
| EC-003 | Plugin supplies `plugin.version = "99.0.0"` (wrong, legacy stamp) | Host overrides with correct plugin version from registry; `event.host_overrides: ["plugin.version"]`; stderr warning |
| EC-004 | Same plugin overrides same field (`service.name`) in 50 consecutive emissions in one invocation | Exactly ONE `vsdd.internal.host_field_override.v1` notice per `(plugin.name, "service.name")` pair per invocation; 50 domain events each have `event.host_overrides: ["service.name"]`; 50 stderr warnings (not rate-limited) |
| EC-005 | Plugin supplies multiple host-owned fields: `["service.name", "plugin.version", "trace_id"]` | All three are overridden; emitted event has `event.host_overrides: ["service.name", "plugin.version", "trace_id"]`; three separate `vsdd.internal.host_field_override.v1` notices (one per field name, rate-limited per `(plugin.name, field_name)` pair); three stderr warnings |
| EC-006 | Plugin supplies `event.category = "domain"` for a `vsdd.block.*` prefix event | Host overrides `event.category` with `"audit"` (registry lookup per BC-1.12.004); `event.host_overrides: ["event.category"]`; lifecycle notice + stderr warning |
| EC-007 | Plugin calls `emit_pair` (BC-1.11.003) and supplies `event.correlation_id` | `emit_pair` host helper overrides plugin-supplied `event.correlation_id` with its own generated UUID; `event.host_overrides: ["event.correlation_id"]` stamped on both events; lifecycle notice emitted |
| EC-008 | Lifecycle event meta-rate-limit clarification | The `vsdd.internal.host_field_override.v1` lifecycle event IS rate-limited per Postcondition 6: at most one emission per `(plugin.name, field_name)` per dispatcher invocation. There is NO SEPARATE meta-rate-limit on top of the per-pair rate-limit — the per-pair rate IS the only rate-limit. After the first override of a given `(plugin.name, field_name)` pair, all subsequent overrides of the same pair within the same dispatcher invocation are suppressed (no lifecycle event); the domain event still receives `event.host_overrides`, and the stderr warning still fires. |
| EC-009 | Post-Wave-1 implementation silently discards plugin-supplied host-owned fields without stamping `event.host_overrides` | **Future-implementation witness (SOUL #4):** This is the wrong behavior. The distinguishing test: emit an event with a known host-owned field set by the plugin; assert `event.host_overrides` contains that field name. A misimplementation that silently discards the field without stamping `event.host_overrides` violates this BC. |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Plugin emits event with `service.name = "my-plugin"` | Domain event in `events-*.jsonl` has `event.host_overrides: ["service.name"]`; `service.name = "vsdd-factory"` (host-stamped); `vsdd.internal.host_field_override.v1` also in `events-*.jsonl` with `affected.plugin.name`; stderr warning present | inline-override-visibility |
| Plugin emits event with NO host-owned fields | `event.host_overrides` absent from emitted event; no `vsdd.internal.host_field_override.v1` event | no-override-case |
| Same plugin overrides `trace_id` in 10 consecutive emissions, one invocation | 10 domain events each have `event.host_overrides: ["trace_id"]`; exactly 1 `vsdd.internal.host_field_override.v1` event (rate-limited per `(plugin.name, "trace_id")`); 10 stderr warnings | rate-limit-one-notice |
| **Misimplementation distinguisher:** override occurs but `event.host_overrides` absent | Test MUST assert `event.host_overrides` is present and non-empty. Silent override without `event.host_overrides` is a BC violation (SOUL #4 / TD-VSDD-092). | misimplementation-witness-silent-override |
| Plugin overrides 3 host-owned fields in one emission | Single domain event has `event.host_overrides` array with 3 entries; 3 lifecycle notices (one per `(plugin.name, field_name)` pair); 3 stderr warnings | multi-field-override |
| Two different plugins each override `service.name` in same invocation | Two `vsdd.internal.host_field_override.v1` events (one per `plugin.name`); each domain event from each plugin has `event.host_overrides: ["service.name"]` | two-plugins-same-field |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| (TBD — Phase 1.6b) | `event.host_overrides` present and non-empty on all events where host overrides occurred | unit test: inject plugin payload with host-owned field; assert `event.host_overrides` contains field name |
| (TBD) | Rate-limit: exactly one `vsdd.internal.host_field_override.v1` per `(plugin.name, field_name)` per invocation | integration test: emit N events with same override; count lifecycle events; assert = 1 |
| (TBD) | `event.host_overrides` absent when no override occurred | unit test: emit event with no host-owned fields; assert field absent |
| (TBD) | `vsdd.internal.host_field_override.v1` carries `affected.plugin.name` | unit test: assert lifecycle event has `affected.plugin.name = <plugin.name>` |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-029 |
| Capability Anchor Justification | CAP-029 ("Emit structured events to a single observability stream (file path)") per capabilities.md §CAP-029. This BC specifies the `event.host_overrides` field stamping and the `vsdd.internal.host_field_override.v1` lifecycle event that make host-field override activity observable within the single `events-*.jsonl` stream. CAP-029's single-stream model depends on the stream carrying semantically correct host-field values; the override-visibility contract governed by this BC ensures that any host-stamped field taking precedence over a plugin-supplied value leaves an auditable trace on the same stream — a host-field semantics integrity guarantee intrinsic to the single-stream design. |
| L2 Domain Invariants | (no domain invariants directly enforced; override-visibility behavior is fully specified by BC postconditions; the underlying host-field precedence policy is governed by ADR-015 D-15.3) |
| Architecture Module | SS-01 — `crates/factory-dispatcher/src/host/emit_event.rs` (override detection, `event.host_overrides` stamping, lifecycle notice emission) |
| Stories | S-10.02 (Wave 1 FileSink wiring + per-event stamping), S-10.03 (Resource-attribute enrichment), S-10.04 (Trace propagation + lifecycle event types) |
| Epic | E-10 (Single-stream OTel-aligned event emission) |
| ADR | ADR-015 D-15.3 (producer-side enrichment contract; host-field override visibility; two-channel approach) |

### Purity Classification

| Property | Assessment |
|----------|-----------|
| I/O operations | YES — writes domain event + lifecycle override event to FileSink; writes stderr warning |
| Global state access | YES — reads per-invocation rate-limit state for `(plugin.name, field_name)` pairs; reads startup ResourceContext |
| Deterministic | YES given fixed plugin payload and fixed host-stamped values |
| Thread safety | YES — single-threaded dispatcher model per ADR-008 |
| Overall classification | Effectful shell (per-emit field comparison + conditional lifecycle event emission + conditional stderr warning) |

### TD-VSDD-092 (BC-SOUL4-coverage) Verification

Per-emit override detection source-walk:

- Override comparison (plugin field vs host field): must be a direct comparison
  with no `let _ =` discard. The implementation MUST populate `event.host_overrides`
  whenever any override occurs — this is the primary SOUL #4 guard for this BC.
  EC-009 documents the distinguishing test for a misimplementation that discards
  silently.
- `vsdd.internal.host_field_override.v1` emission: must NOT use `let _ =
  emit_event(...)`. The lifecycle event is a first-class observability signal.
  Best-effort semantics apply at the FileSink level (per BC-1.11.002 failure
  cascade) but the EMISSION call itself must not be silently discarded at the
  call site.
- Rate-limit state: per-invocation `HashSet<(plugin_name, field_name)>` or
  equivalent. Must be checked BEFORE emitting the lifecycle event; after
  insertion, subsequent duplicates are suppressed. No `let _ =` on the
  rate-limiter check.

## Changelog

| Version | Date | Change |
|---------|------|--------|
| v1.0 | 2026-05-06 | Initial authoring (D-315). Two-channel override-visibility contract: event.host_overrides inline + vsdd.internal.host_field_override.v1 lifecycle event + stderr warning per ADR-015 D-15.3. |
| v1.1 | 2026-05-06 | D-315/D-316 amendments — cap-anchor justification, edge case clarifications. |
| v1.2 | 2026-05-06 | D-319 — F-3 fix: Story Anchor + Stories cell extended with S-10.04 (POLICY 8 reverse-direction drift from D-316 closed). F-8 fix: EC-008 prose rewritten — unambiguously states the lifecycle event IS rate-limited per Postcondition 6 with no separate meta-rate-limit; the prior contradicting "NOT rate-limited against itself" phrase removed. |
| v1.3 | 2026-05-06 | D-325 — F-7 sweep: L2 Capability cell paraphrase removed — cell now just `CAP-029`. F-14 sweep: stable-anchor disclaimer added to `crates/factory-dispatcher/src/host/emit_event.rs` Architecture Anchor. |
| v1.4 | 2026-05-09 | fix-burst-36 (F-P37-001): BC-INDEX row Stories cell S-10.02, S-10.03 → S-10.02, S-10.03, S-10.04. Bidirectional L-P28-001 sweep: source body already listed S-10.04 (Story Anchor + Stories row); index row was missing S-10.04. |
