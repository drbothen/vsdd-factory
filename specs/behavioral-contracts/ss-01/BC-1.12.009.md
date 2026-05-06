---
document_type: behavioral-contract
level: L3
version: "1.0"
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
capability: "CAP-003"
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

# Behavioral Contract BC-1.12.009: factory-dispatcher::dual_emit::pair_identity_contract — event.correlation_id / event.deprecated_by / event.replaces_deprecated_alias field semantics; four-state event classification (paired-current / paired-deprecated / orphaned-current-half / orphaned-deprecated-half); consumer degradation rule for orphaned halves (ADR-015 D-15.2.e)

## Description

During the Wave 2 dual-emit migration window, each migrating event family
emits TWO events for one logical event: the OLD-name emission (legacy namespace)
and the NEW-name emission (reverse-DNS canonical form). ADR-015 D-15.2.e
specifies three correlation fields that allow any consumer to classify any
event's role in a dual-emit pair by inspecting the event record alone —
without consulting a separate registry or requiring knowledge of which event
families are in migration.

This BC governs the **consumer-side identity contract**: the field semantics,
the four-state classification taxonomy, and the degradation rule for orphaned
pair halves. It is the consumer's contract for reading `events-*.jsonl`.

**Scope boundary vs BC-1.11.003:** BC-1.11.003 specifies the `emit_pair`
HOST HELPER — the producer-side mechanism that assigns `event.correlation_id`,
`event.deprecated_by`, and `event.replaces_deprecated_alias` atomically.
BC-1.12.009 specifies the **consumer-side contract** — what these fields mean
to any reader of `events-*.jsonl`, how to classify events by their role in a
pair, and what behavior is required when a cross-reference cannot be resolved
(orphaned half). The two BCs are complementary: BC-1.11.003 = "how a correct
pair is produced"; BC-1.12.009 = "how a consumer interprets and handles any
event's correlation state, including malformed or orphaned pairs."

**Wave scope:** The three correlation fields are WAVE 2 MIGRATION WINDOW
FIELDS. They are PLUGIN-ASSERTED during Wave 2 (for plugins that have not yet
adopted `emit_pair`; `emit_pair` adopters have them host-assigned). After
Wave 3 shim removal, all three fields are absent on all new events — the same
state as a non-paired event. This BC specifies the four-state classification
that handles both the Wave 2 state and the post-Wave-3 state.

## Preconditions

1. Events-*.jsonl is being read by a consumer (Grafana dashboard, factory-query,
   factory-sla, or a custom consumer).
2. The consumer encounters an event and wishes to classify its role in any
   dual-emit pair.
3. The consumer has access to the event's JSON record.
4. For orphan-half detection (State 3/4 below), the consumer has access to
   at least the events within the same `trace_id` scope from `events-*.jsonl`.

## Postconditions

### Three correlation fields (D-15.2.e)

1. The three correlation fields are defined as follows:

   | Field | Set on | Meaning |
   |-------|--------|---------|
   | `event.correlation_id` | both halves of a dual-emit pair | Shared UUIDv4 linking the old-name and new-name emissions of the same logical event |
   | `event.deprecated_by` | old-name emission ONLY | `event.id` of the corresponding new-name (canonical) emission |
   | `event.replaces_deprecated_alias` | new-name emission ONLY | `event.id` of the corresponding old-name emission |

2. The three fields are ABSENT (field not present in the JSON record) on:
   - Non-paired events (single-namespace events that are not part of a dual-emit pair)
   - Post-Wave-3 events (after shim removal, all events are single-namespace)
   Both of these states are indistinguishable from a consumer's perspective by
   design — the absence of correlation fields means "this is a single-emission
   event, no pairing context."

3. The fields are PLUGIN-ASSERTED during Wave 2 (for legacy two-call shims).
   Plugins that use `emit_pair` (BC-1.11.003) have these fields HOST-ASSIGNED.
   Either way, the field semantics and consumer classification rules are identical.

### Four-state classification taxonomy

4. Any event in `events-*.jsonl` falls into exactly one of four states
   with respect to the dual-emit pair identity contract:

   **State 1 — Paired-Current (new-name emission, healthy pair):**
   - `event.replaces_deprecated_alias` is set (pointing to the old-name event's `event.id`)
   - `event.deprecated_by` is ABSENT
   - `event.correlation_id` is set
   - Both `event.replaces_deprecated_alias` target AND this event are present in
     `events-*.jsonl` within the same `trace_id` scope
   - Consumer classification: canonical new-name emission; use this event for
     aggregation; ignore the corresponding old-name emission to avoid double-counting

   **State 2 — Paired-Deprecated (old-name emission, healthy pair):**
   - `event.deprecated_by` is set (pointing to the new-name event's `event.id`)
   - `event.replaces_deprecated_alias` is ABSENT
   - `event.correlation_id` is set
   - Both `event.deprecated_by` target AND this event are present in `events-*.jsonl`
     within the same `trace_id` scope
   - Consumer classification: legacy old-name emission; present for backward-
     compatibility only; consumers building new aggregations SHOULD prefer the
     paired-current event; legacy consumers keyed on old names continue to match

   **State 3 — Orphaned-Deprecated-Half (old-name emission, no healthy pair):**
   - `event.deprecated_by` is set (pointing to a new-name event)
   - The `event.id` referenced by `event.deprecated_by` is NOT found in
     `events-*.jsonl` within the same `trace_id` scope
   - `event.correlation_id` MAY be set (if the producing plugin assigned it)
   - Consumer classification: orphaned old-name emission; the new-name counterpart
     was lost (FileSink failure or dispatcher crash during dual-emit)
   - **Degradation rule:** Treat as a single non-paired event for dedup purposes;
     apply single-event accounting; do NOT exclude from aggregation on the
     assumption the canonical event will arrive later

   **State 4 — Orphaned-Current-Half (new-name emission, no healthy pair):**
   - `event.replaces_deprecated_alias` is set (pointing to an old-name event)
   - The `event.id` referenced by `event.replaces_deprecated_alias` is NOT found
     in `events-*.jsonl` within the same `trace_id` scope
   - `event.correlation_id` MAY be set
   - Consumer classification: orphaned new-name emission; the old-name counterpart
     was lost (FileSink failure or dispatcher crash during dual-emit)
   - **Degradation rule:** Same as State 3 — treat as a single non-paired event
     for dedup purposes

   **Non-paired (baseline state, also the post-Wave-3 state):**
   - `event.deprecated_by`, `event.replaces_deprecated_alias`, and
     `event.correlation_id` are ALL ABSENT
   - Consumer classification: standard single-emission event; no pairing context

### Consumer degradation rule for orphaned halves

5. **Consumers MUST treat orphaned pair halves (State 3 or State 4) as non-paired
   for dedup purposes.** Specifically:
   - A consumer MUST NOT exclude an orphaned half from aggregation on the
     assumption the missing counterpart will arrive later (it will not — the
     orphan condition arises from a past FileSink failure or dispatcher crash).
   - A consumer MUST NOT double-count by including BOTH the orphaned half AND
     any late-arriving counterpart that the consumer discovers later. If a
     consumer initially processes an event as orphaned and later encounters what
     appears to be the missing counterpart (across a file boundary or a later
     read), the consumer MAY re-classify, but this is out of scope for v1.
   - The dedup contract is: orphaned halves are accounted as individual events.

### Orphan detection scope

6. **Orphan-half detection is bounded to events sharing the same `trace_id`.**
   A consumer MAY classify a cross-reference as unresolvable (orphaned) if the
   referenced `event.id` does not appear within the same `trace_id` scope in
   `events-*.jsonl`. Consumers MUST NOT perform a full-file anti-join (scanning
   all events across all trace_ids to resolve a cross-reference) — this is
   explicitly out of scope for v1 consumer implementations.

### Wave 2 aggregation requirement

7. **Consumers running aggregation queries during Wave 2 MUST filter to a single
   event-name namespace** to avoid double-counting from dual-emit. Concretely:
   either filter to OLD-name events only OR filter to NEW-name events only for
   any given aggregation. Using `event.replaces_deprecated_alias` as the dedup
   hint is the recommended approach:
   - Count only events where `event.replaces_deprecated_alias` is absent to get
     old-name events only (excludes all new-name emissions).
   - Count only events where `event.replaces_deprecated_alias` is present OR
     both cross-reference fields are absent (excludes old-name paired emissions)
     for new-name aggregation.
   This requirement is per ADR-015 Negative consequences (dual-emit double-count
   dedup contract).

### Post-Wave-3 field absence

8. After Wave 3 removes dual-emit shims, all three correlation fields are absent
   on all new events. A consumer that correctly handles the non-paired state
   (all three fields absent) handles post-Wave-3 events correctly without any
   code change. The four-state classifier degrades gracefully: States 1–4 are
   checked first; if no fields match any state, the event is non-paired.

## Invariants

1. `event.deprecated_by` and `event.replaces_deprecated_alias` are mutually
   exclusive on any single event — they cannot both be present on the same event
   record. `event.deprecated_by` is set only on old-name emissions;
   `event.replaces_deprecated_alias` is set only on new-name emissions.
2. If `event.correlation_id` is present on an event, then either
   `event.deprecated_by` OR `event.replaces_deprecated_alias` MUST also be
   present (a `correlation_id` without a directional field is malformed; this
   indicates a plugin bug in legacy two-call shim code).
3. The bidirectional cross-reference is SYMMETRIC when both halves are healthy:
   old-event.`event.deprecated_by` == new-event.`event.id` AND
   new-event.`event.replaces_deprecated_alias` == old-event.`event.id`.
   Asymmetric cross-references (one half's pointer does not match the other
   half's `event.id`) indicate a plugin-side UUID coordination bug in a legacy
   two-call shim.
4. A consumer MUST handle malformed cross-references (Invariant 2 violation,
   Invariant 3 asymmetry) as orphaned halves per the degradation rule
   (Postcondition 5) — do NOT error or crash; degrade to single-event accounting.

## Related BCs

- BC-1.11.003 — `emit_pair` host helper (producer-side complement: BC-1.11.003
  specifies how the host PRODUCES a correct pair; this BC specifies how a
  consumer CLASSIFIES any event in terms of pair state)
- BC-1.12.004 — Per-event host-stamping (sibling: the three correlation fields
  appear in the per-event attribute table in ADR-015 D-15.2; host-stamped
  fields win — correlation fields are plugin-asserted EXCEPT when `emit_pair`
  is used, in which case they are host-assigned; in both cases consumer semantics
  are identical per this BC)
- BC-1.12.001 — Single-stream FileSink routing (depends on: all four states
  are detectable only because all events — including both halves of a pair —
  appear in the same `events-*.jsonl` file)
- BC-1.11.002 — FileSink partial-write recovery (related: orphaned halves arise
  from FileSink failures during the second write of a legacy two-call shim;
  BC-1.11.002 governs the failure; this BC governs the consumer's response to
  the orphaned artifact)

## Architecture Anchors

- ADR-015 D-15.2.e — the authoritative five-state identity contract (non-paired,
  paired-old, paired-new, post-Wave-3 absent, orphaned-half); this BC normalizes
  it to the four-state consumer taxonomy used in dashboards and consumer code
- ADR-015 § Negative consequences — "Wave 2 dual-emit doubles event volume...
  Aggregation queries MUST filter to a single event-name namespace to avoid
  double-counting"
- `events-*.jsonl` — the physical stream where all four states are observable

## Story Anchor

S-10.05 (Wave 2: Plugin schema migration + dual-emit shims; this BC defines the
identity contract that Wave 2 shim authors MUST implement, and the consumer
contract that Wave 2 dashboard migrations MUST respect)

## VP Anchors

(TBD — to be assigned after S-10.05 story authoring)

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Healthy dual-emit pair: both old-name and new-name events present in same trace scope | old-name: State 2 (Paired-Deprecated); new-name: State 1 (Paired-Current); all cross-references resolve; symmetric |
| EC-002 | Only the old-name event is present in trace scope; new-name event absent (FileSink failure on second write) | old-name: State 3 (Orphaned-Deprecated-Half); consumer degrades to single-event accounting; does NOT wait for counterpart |
| EC-003 | Only the new-name event is present; old-name event absent (FileSink failure on first write of two-call shim is unusual; more commonly: dispatcher crash between writes) | new-name: State 4 (Orphaned-Current-Half); same degradation rule |
| EC-004 | All three correlation fields absent | Non-paired state; single-emission event; normal aggregation applies; Wave 3 post-shim state |
| EC-005 | Both `event.deprecated_by` AND `event.replaces_deprecated_alias` present on same event | Malformed (violates Invariant 1); consumer MUST treat as orphaned half per degradation rule (Postcondition 5); do not crash |
| EC-006 | `event.correlation_id` present but both directional fields absent | Malformed (violates Invariant 2); consumer treats as non-paired (correlation_id alone does not establish a pair); degrade to single-event |
| EC-007 | Cross-reference is asymmetric: old-event.`event.deprecated_by` does not equal new-event.`event.id` | Violates Invariant 3; plugin UUID coordination bug in legacy two-call shim; consumer treats both events as orphaned halves per degradation rule |
| EC-008 | Consumer queries aggregation during Wave 2 without filtering by event-name namespace | Double-counting risk: both old-name and new-name events are present; consumer MUST apply namespace filter per Postcondition 7 |
| EC-009 | Post-Wave-3: event emitted with all three fields absent | Correctly classified as non-paired; backward-compatible with both pre-Wave-2 single-emission events and post-Wave-3 single-emission events |
| EC-010 | Consumer attempts full-file anti-join to resolve orphan cross-references | This is OUT OF SCOPE per Postcondition 6. Consumers MUST NOT rely on full-file scanning; they MUST use trace_id scope. A consumer that full-file scans for orphan resolution is not compliant with this BC. |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Event with `event.deprecated_by = <uuid>` AND corresponding new-name event with `event.replaces_deprecated_alias = <same source event.id>` in same trace scope | Old-name: State 2 (Paired-Deprecated); New-name: State 1 (Paired-Current); both states classified correctly; no orphan | healthy-pair-both-states |
| Event with `event.deprecated_by = <uuid>` but no matching new-name event in same trace scope | State 3 (Orphaned-Deprecated-Half); consumer applies degradation rule — single-event accounting | orphaned-deprecated-half |
| Event with `event.replaces_deprecated_alias = <uuid>` but no matching old-name event in same trace scope | State 4 (Orphaned-Current-Half); degradation rule | orphaned-current-half |
| Event with none of the three correlation fields | Non-paired state; standard single-emission; no dedup logic triggered | non-paired-baseline |
| Event with both `event.deprecated_by` AND `event.replaces_deprecated_alias` present | Malformed; consumer degrades to single-event; no crash | malformed-both-fields |
| Wave 2 aggregation: `count(event.name = "vsdd.pr.created.v1")` during dual-emit window | Without filter: double-counts old-name events; WITH `event.replaces_deprecated_alias IS NOT NULL` filter: counts only new-name events; correct | wave2-aggregation-dedup |
| **Misimplementation distinguisher:** consumer excludes orphaned halves from aggregation | Orphaned halves represent real events that occurred; a consumer that silently drops them (assuming the counterpart will arrive) produces under-counted aggregates. Test MUST assert orphaned-half events ARE included in aggregation. | misimplementation-witness-orphan-excluded |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| (TBD — Phase 1.6b) | Four-state classification correctly applied for all five input variants (States 1–4 + non-paired) | unit test: tabular test with one event per state; assert correct classification |
| (TBD) | Orphaned halves included in single-event accounting | integration test: write orphaned-half to events-*.jsonl; assert consumer aggregation count includes it |
| (TBD) | Symmetric healthy pair: both halves classified correctly | integration test: write both halves; assert State 1 + State 2 classification |
| (TBD) | Malformed (both fields present) degrades gracefully | unit test: inject malformed event; assert no crash; assert single-event accounting |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-003 ("Stream observability events to multiple configurable sinks") per capabilities.md §CAP-003 |
| Capability Anchor Justification | CAP-003 ("Stream observability events to multiple configurable sinks") per capabilities.md §CAP-003. This BC governs the consumer-side semantics of the dual-emit pair identity fields in `events-*.jsonl` — the stream that CAP-003 defines. The four-state classification and orphan degradation rule are essential for consumers (Grafana dashboards, factory-query, factory-sla) to correctly interpret events during and after the Wave 2 migration window. Without this contract, consumers would silently double-count or under-count events, breaking the stream's semantic correctness. |
| L2 Domain Invariants | TBD |
| Architecture Module | SS-01 — consumer behavior specification for `events-*.jsonl` events; relevant production code lives in consumer tooling (factory-query, Grafana dashboard queries) and Wave 2 plugin shims |
| Stories | S-10.05 (Wave 2: Plugin schema migration + dual-emit shims; shim authors implement the producer side; consumer implementations must conform to this BC) |
| Epic | E-10 (Single-stream OTel-aligned event emission) |
| ADR | ADR-015 D-15.2.e (dual-emit pair identity contract — five-state classification reduced to four-state consumer taxonomy; orphaned-half detection scoped to `trace_id`); ADR-015 § Negative consequences (Wave 2 double-count dedup contract) |
| Related BC | BC-1.11.003 (producer-side complement: `emit_pair` host helper; this BC governs the consumer-side interpretation; the two BCs form the complete dual-emit contract) |

### Purity Classification

| Property | Assessment |
|----------|-----------|
| I/O operations | NO — this BC specifies consumer-side LOGIC for classifying event records; no I/O performed by the classifier itself (the events were already written to `events-*.jsonl` by the producer path) |
| Global state access | DEPENDS ON CONSUMER — orphan detection requires trace-scoped event lookup; this is a read-only access to `events-*.jsonl` content |
| Deterministic | YES — four-state classification is a pure function of the event record's field values and the set of `event.id` values in the same trace scope |
| Thread safety | YES — classification logic is read-only |
| Overall classification | Pure function (consumer classification logic; no side effects) |

### TD-VSDD-092 (BC-SOUL4-coverage) Verification

Consumer-side classification source-walk:

- The `event.replaces_deprecated_alias` and `event.deprecated_by` fields are
  OPTIONAL in the event record (absent = not set). Consumer code MUST use
  `Option<String>` or nullable JSON field access — NOT `.unwrap()` or direct
  field access that panics on absence. A panicking classifier for absent
  cross-reference fields would silently fail for all non-paired events.
- Orphan detection: the "target `event.id` not found in trace scope" lookup
  must NOT silently default to "found" if the lookup fails (e.g., trace index
  unavailable). If the consumer cannot determine presence, it MUST assume
  orphaned (the safe degradation direction per Postcondition 5).
- Malformed event handling (EC-005, EC-006, EC-007): the classifier MUST NOT
  use `unwrap()` or `panic!()` for malformed cross-references. Malformed events
  are plugin bugs; they must be handled gracefully (degraded to single-event).
  A `let _ = classify(event)` pattern at the aggregation call site would silently
  drop classification results — prohibited.
