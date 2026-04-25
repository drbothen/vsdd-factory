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
input-hash: "d413cae"
traces_to: .factory/specs/behavioral-contracts/bc-id-mapping.md
origin: brownfield
extracted_from: "pass-3-behavioral-contracts-deep-r1.md:536"
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

# Behavioral Contract BC-3.03.007: Shutdown drains and joins the worker thread; idempotent post-`accepts` rejection

> Section: OTLP gRPC sink batching and lifecycle
> Source BC (audit ID): BC-AUDIT-143

## Description

`shared.shutdown.store(true, Release)`; sender lifted out of its mutex (channel closes when last sender drops); worker `JoinHandle` taken and `join()`-ed. After shutdown, `Sink::accepts` returns false (early return on `shared.shutdown.load(Acquire)`); `Sink::submit` is a no-op when the sender is None. `Drop` trampolines to `shutdown()` if the worker is still active — this guarantees the worker drains pending events before the producer's main exits.

## Preconditions

1. Drop-or-explicit-shutdown.

## Postconditions

1. `shared.shutdown.store(true, Release)`; sender lifted out of its mutex (channel closes when last sender drops); worker `JoinHandle` taken and `join()`-ed. After shutdown, `Sink::accepts` returns false (early return on `shared.shutdown.load(Acquire)`); `Sink::submit` is a no-op when the sender is None. `Drop` trampolines to `shutdown()` if the worker is still active — this guarantees the worker drains pending events before the producer's main exits.

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
| (TBD — to be assigned in Phase 1.6c) | TBD — Phase 1.6b will identify formal verification properties | manual |

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
| **Original audit ID** | BC-AUDIT-143 |
| **Pass-3 source** | `pass-3-behavioral-contracts-deep-r1.md:536` |
| **Extraction Date** | 2026-04-25 |

**Evidence (from pass-3):**

> `sink-otel-grpc/src/lib.rs:440-461` (shutdown impl + Drop); `:392-403` (accepts checks shutdown flag); `:406-419` (submit checks accepts via early return).

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

