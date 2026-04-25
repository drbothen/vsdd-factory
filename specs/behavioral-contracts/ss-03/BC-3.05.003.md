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
input-hash: "33c3853"
traces_to: .factory/specs/behavioral-contracts/bc-id-mapping.md
origin: brownfield
extracted_from: "pass-3-deep-rust-tests.md:618"
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

# Behavioral Contract BC-3.05.003: factory-dispatcher::sinks_otel_grpc (integration)::ten_events_arrive_with_correct_attribute_mappi

> Section: Sink integration via dispatcher
> Source BC (audit ID): BC-AUDIT-2347

## Description

Given Mock LogsService bound on 127.0.0.1:0; sink configured with batch.size=100, batch.interval=60s; 10 events submitted. When sink.flush() then mock server's snapshot is taken.. Then 10 records arrive; first record body == "plugin.invoked"; time_unix_nano == 1_777_003_425_000 * 1_000_000 (ms→ns); attributes contain dispatcher_trace_id, session_id, plugin_name, plugin_version, seq; type and ts_epoch are NOT attributes (they were lifted to body and time_unix_nano).

## Preconditions

1. Mock LogsService bound on 127.0.0.1:0; sink configured with batch.size=100, batch.interval=60s; 10 events submitted.

## Postconditions

1. 10 records arrive; first record body == "plugin.invoked"; time_unix_nano == 1_777_003_425_000 * 1_000_000 (ms→ns); attributes contain dispatcher_trace_id, session_id, plugin_name, plugin_version, seq; type and ts_epoch are NOT attributes (they were lifted to body and time_unix_nano).

## Invariants

1. OTLP wire-shape is byte-compatible with operator dashboards (Grafana / Tempo / Loki) without translation.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | TBD — Phase 1.6b will enumerate edge cases from source review | TBD |

## Canonical Test Vectors

> Test vectors will be pulled from the pinned test case in Phase 1.6b.

| Input | Expected Output | Category |
|-------|----------------|----------|
| Mock LogsService bound on 127.0.0.1:0; sink configured with batch.size=100, batch.interval=60s; 10 events submitted. | 10 records arrive; first record body == "plugin.invoked"; time_unix_nano == 1_777_003_425_000 * 1_000_000 (ms→ns); attri | happy-path |
| TBD | TBD | edge-case |
| TBD | TBD | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| (TBD — to be assigned in Phase 1.6c) | Given Mock LogsService bound on 127.0.0.1:0; sink configured with batch.size=100, batch.interval=60s; 10 events submitte | manual (existing test: `crates/factory-dispatcher/tests/sinks_otel_grpc.rs::ten_events_arrive_with_correct_attribute_mapping`) |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | TBD (subsystem L2 spec pending) |
| L2 Domain Invariants | TBD |
| Architecture Module | SS-03 (Sink integration via dispatcher) |
| Stories | TBD (Phase 2 story-writer pass) |

## Related BCs (Recommended)

- TBD — cross-references will be filled in Phase 1.6b after all per-BC files exist.

## Architecture Anchors (Recommended)

- `architecture/SS-03-observability-sinks.md` — section: Sink integration via dispatcher

---

### Brownfield-Specific Sections

> This BC was extracted during Phase 0d brownfield ingestion (BC-AUDIT pass) and migrated to canonical one-per-file BC-S.SS.NNN format in Phase 1.4b.

#### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `crates/factory-dispatcher/tests/sinks_otel_grpc.rs` |
| **Source line(s)** | 198–251 |
| **Confidence** | high |
| **Original audit ID** | BC-AUDIT-2347 |
| **Pass-3 source** | `pass-3-deep-rust-tests.md:618` |
| **Extraction Date** | 2026-04-25 |
| **Pinned test** | ``crates/factory-dispatcher/tests/sinks_otel_grpc.rs::ten_events_arrive_with_correct_attribute_mapping`` |
| **Test type** | integration (real gRPC server) |

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

