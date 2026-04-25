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
input-hash: "fa2c376"
traces_to: .factory/specs/behavioral-contracts/bc-id-mapping.md
origin: brownfield
extracted_from: "pass-3-deep-rust-tests.md:1046"
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

# Behavioral Contract BC-3.03.009: sink-otel-grpc::event_attributes_flatten_non_reserved_fields: non-reserved fields flatten to OTLP

> Section: OTLP gRPC sink batching and lifecycle
> Source BC (audit ID): BC-AUDIT-2385

## Description

Given Event with `sha=deadbeef` (string), `files_changed=7` (int), `dirty=true` (bool), `score=0.42` (float). When `event_to_log_record` → attribute map inspected.. Then `sha` is StringValue, `files_changed` is IntValue(7), `dirty` is BoolValue(true), `score` is DoubleValue(~0.42).

## Preconditions

1. Event with `sha=deadbeef` (string), `files_changed=7` (int), `dirty=true` (bool), `score=0.42` (float).

## Postconditions

1. `sha` is StringValue, `files_changed` is IntValue(7), `dirty` is BoolValue(true), `score` is DoubleValue(~0.42).

## Invariants

1. Non-reserved field types map to OTLP variants without lossy coercion.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | TBD — Phase 1.6b will enumerate edge cases from source review | TBD |

## Canonical Test Vectors

> Test vectors will be pulled from the pinned test case in Phase 1.6b.

| Input | Expected Output | Category |
|-------|----------------|----------|
| Event with `sha=deadbeef` (string), `files_changed=7` (int), `dirty=true` (bool), `score=0.42` (float). | `sha` is StringValue, `files_changed` is IntValue(7), `dirty` is BoolValue(true), `score` is DoubleValue(~0.42). | happy-path |
| TBD | TBD | edge-case |
| TBD | TBD | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-001 | Given Event with `sha=deadbeef` (string), `files_changed=7` (int), `dirty=true` (bool), `score=0.42` (float). When `even | manual (existing test: `crates/sink-otel-grpc/src/lib.rs::tests::event_attributes_flatten_non_reserved_fields`) |

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
| **Source line(s)** | 849–878 |
| **Confidence** | high |
| **Original audit ID** | BC-AUDIT-2385 |
| **Pass-3 source** | `pass-3-deep-rust-tests.md:1046` |
| **Extraction Date** | 2026-04-25 |
| **Pinned test** | ``crates/sink-otel-grpc/src/lib.rs::tests::event_attributes_flatten_non_reserved_fields`` |
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

