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
input-hash: "21869ea"
traces_to: .factory/specs/behavioral-contracts/bc-id-mapping.md
origin: brownfield
extracted_from: "pass-3-deep-rust-tests.md:811"
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

# Behavioral Contract BC-3.06.003: sink-core::sink_event_event_type_missing_returns_none: no "type" field → event_type() returns None

> Section: sink-core base traits and submit/flush
> Source BC (audit ID): BC-AUDIT-2364

## Description

Given Empty SinkEvent. When `event_type()`.. Then Returns None.

## Preconditions

1. Empty SinkEvent.

## Postconditions

1. Returns None.

## Invariants

1. Producer-bug case (forgot to set `type`) is observable, not implicit.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | TBD — Phase 1.6b will enumerate edge cases from source review | TBD |

## Canonical Test Vectors

> Test vectors will be pulled from the pinned test case in Phase 1.6b.

| Input | Expected Output | Category |
|-------|----------------|----------|
| Empty SinkEvent. | Returns None. | happy-path |
| TBD | TBD | edge-case |
| TBD | TBD | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-001 | Given Empty SinkEvent. When `event_type()`.. Then Returns None. | manual (existing test: `crates/sink-core/src/lib.rs::tests::sink_event_event_type_missing_returns_none`) |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | TBD (subsystem L2 spec pending) |
| L2 Domain Invariants | TBD |
| Architecture Module | SS-03 (sink-core base traits and submit/flush) |
| Stories | TBD (Phase 2 story-writer pass) |

## Related BCs (Recommended)

- TBD — cross-references will be filled in Phase 1.6b after all per-BC files exist.

## Architecture Anchors (Recommended)

- `architecture/SS-03-observability-sinks.md` — section: sink-core base traits and submit/flush

---

### Brownfield-Specific Sections

> This BC was extracted during Phase 0d brownfield ingestion (BC-AUDIT pass) and migrated to canonical one-per-file BC-S.SS.NNN format in Phase 1.4b.

#### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `crates/sink-core/src/lib.rs` |
| **Source line(s)** | 330–334 |
| **Confidence** | high |
| **Original audit ID** | BC-AUDIT-2364 |
| **Pass-3 source** | `pass-3-deep-rust-tests.md:811` |
| **Extraction Date** | 2026-04-25 |
| **Pinned test** | ``crates/sink-core/src/lib.rs::tests::sink_event_event_type_missing_returns_none`` |
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

