---
document_type: behavioral-contract
level: L3
version: "1.0"
status: draft
producer: codebase-analyzer
timestamp: 2026-04-25T00:00:00
phase: "1.4b"
inputs:
  - .factory/specs/behavioral-contracts/bc-id-mapping.md
  - .factory/phase-0-ingestion/pass-3-deep-rust-tests.md
input-hash: "4f158c9"
traces_to: .factory/specs/behavioral-contracts/bc-id-mapping.md
origin: brownfield
extracted_from: "pass-3-deep-rust-tests.md:1035"
subsystem: "SS-03"
capability: "TBD"
lifecycle_status: active
introduced: v1.0.0-beta.4
modified: []
deprecated: null
deprecated_by: null
replacement: null
retired: null
removed: null
removal_reason: null
---

# Behavioral Contract BC-3.03.008: sink-otel-grpc::event_to_log_record_maps_reserved_fields: SinkEvent → LogRecord lifts type→bo

> Section: OTLP gRPC sink batching and lifecycle
> Source BC (audit ID): BC-AUDIT-2384

## Description

Given Event with `type=plugin.invoked`, `ts_epoch=1_777_003_425_000`, dispatcher_trace_id, session_id, plugin_name, plugin_version. When `event_to_log_record(event)`.. Then Record body == "plugin.invoked"; time_unix_nano == 1_777_003_425_000 * 1_000_000; observed_time_unix_nano == time_unix_nano; attributes contain dispatcher_trace_id, session_id, plugin_name, plugin_version; type and ts_epoch are NOT also attributes.

## Preconditions

1. Event with `type=plugin.invoked`, `ts_epoch=1_777_003_425_000`, dispatcher_trace_id, session_id, plugin_name, plugin_version.

## Postconditions

1. Record body == "plugin.invoked"; time_unix_nano == 1_777_003_425_000 * 1_000_000; observed_time_unix_nano == time_unix_nano; attributes contain dispatcher_trace_id, session_id, plugin_name, plugin_version; type and ts_epoch are NOT also attributes.

## Invariants

1. OTLP wire shape is correct out of the box; reserved fields don't leak as attributes.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | TBD — Phase 1.6b will enumerate edge cases from source review | TBD |

## Canonical Test Vectors

> Test vectors will be pulled from the pinned test case in Phase 1.6b.

| Input | Expected Output | Category |
|-------|----------------|----------|
| Event with `type=plugin.invoked`, `ts_epoch=1_777_003_425_000`, dispatcher_trace_id, session_id, plugin_name, plugin_ver | Record body == "plugin.invoked"; time_unix_nano == 1_777_003_425_000 * 1_000_000; observed_time_unix_nano == time_unix_n | happy-path |
| TBD | TBD | edge-case |
| TBD | TBD | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-001 | Given Event with `type=plugin.invoked`, `ts_epoch=1_777_003_425_000`, dispatcher_trace_id, session_id, plugin_name, plug | manual (existing test: `crates/sink-otel-grpc/src/lib.rs::tests::event_to_log_record_maps_reserved_fields`) |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | TBD (subsystem L2 spec pending) |
| L2 Domain Invariants | TBD |
| Architecture Module | SS-03 (OTLP gRPC sink batching and lifecycle) |
| Stories | TBD (Phase 2 story-writer pass) |

## Related BCs (Recommended)

- TBD — cross-references will be filled in Phase 1.6b after all per-BC files exist.

## Architecture Anchors (Recommended)

- `architecture/SS-03-observability-sinks.md` — section: OTLP gRPC sink batching and lifecycle

---

### Brownfield-Specific Sections

> This BC was extracted during Phase 0d brownfield ingestion (BC-AUDIT pass) and migrated to canonical one-per-file BC-S.SS.NNN format in Phase 1.4b.

#### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `crates/sink-otel-grpc/src/lib.rs` |
| **Source line(s)** | 810–847 |
| **Confidence** | high |
| **Original audit ID** | BC-AUDIT-2384 |
| **Pass-3 source** | `pass-3-deep-rust-tests.md:1035` |
| **Extraction Date** | 2026-04-25 |
| **Pinned test** | ``crates/sink-otel-grpc/src/lib.rs::tests::event_to_log_record_maps_reserved_fields`` |
| **Test type** | unit |

#### Evidence Types Used

- **assertion**: pinned by Rust unit/integration test

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | TBD — Phase 1.6b will classify |
| **Global state access** | TBD |
| **Deterministic** | TBD |
| **Thread safety** | TBD |
| **Overall classification** | TBD |

#### Refactoring Notes

TBD — Phase 1.6b will produce refactoring guidance.

