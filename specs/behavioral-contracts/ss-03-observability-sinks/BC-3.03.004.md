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
  - .factory/phase-0-ingestion/pass-3-behavioral-contracts-deep-r1.md
input-hash: "c0b89fd"
traces_to: .factory/specs/behavioral-contracts/bc-id-mapping.md
origin: brownfield
extracted_from: "pass-3-behavioral-contracts-deep-r1.md:513"
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

# Behavioral Contract BC-3.03.004: Worker thread owns its own current_thread tokio runtime on a dedicated OS thread

> Section: OTLP gRPC sink batching and lifecycle
> Source BC (audit ID): BC-AUDIT-140

## Description

`std::thread::Builder::new().name(format!("sink-otel-grpc:{name}")).spawn(move || { let runtime = Builder::new_current_thread().enable_all().build()?; runtime.block_on(worker_loop(...)) })`. Failure to build the runtime records a single `SinkFailure { reason: "failed to build tokio runtime: …" }` and exits the thread cleanly. The dispatcher's synchronous main never sees a tokio dependency. Pass-6 DRIFT-003 documents the design intent to migrate to a shared dispatcher runtime once S-1.6 lands; the migration has not happened (DRIFT-003 still active).

## Preconditions

1. Sink is constructed.

## Postconditions

1. `std::thread::Builder::new().name(format!("sink-otel-grpc:{name}")).spawn(move || { let runtime = Builder::new_current_thread().enable_all().build()?; runtime.block_on(worker_loop(...)) })`. Failure to build the runtime records a single `SinkFailure { reason: "failed to build tokio runtime: …" }` and exits the thread cleanly. The dispatcher's synchronous main never sees a tokio dependency. Pass-6 DRIFT-003 documents the design intent to migrate to a shared dispatcher runtime once S-1.6 lands; the migration has not happened (DRIFT-003 still active).

## Invariants

1. TBD — invariants not explicitly stated in source pass-3 entry.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | TBD — Phase 1.6b will enumerate edge cases from source review | TBD |

## Canonical Test Vectors

> Test vectors will be pulled from the pinned test case in Phase 1.6b.

| Input | Expected Output | Category |
|-------|----------------|----------|
| TBD | TBD | happy-path |
| TBD | TBD | edge-case |
| TBD | TBD | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-001 | TBD — Phase 1.6b will identify formal verification properties | manual |

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
| **Path** | `crates/factory-dispatcher/src/lib.rs` |
| **Source line(s)** | TBD |
| **Confidence** | high |
| **Original audit ID** | BC-AUDIT-140 |
| **Pass-3 source** | `pass-3-behavioral-contracts-deep-r1.md:513` |
| **Extraction Date** | 2026-04-25 |

**Evidence (from pass-3):**

> `sink-otel-grpc/src/lib.rs:307-335` (worker thread spawn); `:309-333` (runtime build and block_on).

#### Evidence Types Used

- **inferred**: from pass-3 narrative; no explicit assertion captured

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

