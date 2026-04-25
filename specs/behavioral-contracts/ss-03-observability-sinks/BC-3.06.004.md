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
input-hash: "8e2efaa"
traces_to: .factory/specs/behavioral-contracts/bc-id-mapping.md
origin: brownfield
extracted_from: "pass-3-deep-rust-tests.md:822"
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

# Behavioral Contract BC-3.06.004: sink-core::sink_event_event_type_non_string_returns_none: "type" set to non-string Value → even

> Section: sink-core base traits and submit/flush
> Source BC (audit ID): BC-AUDIT-2365

## Description

Given `SinkEvent::new().insert("type", json!(42))`. When `event_type()`.. Then Returns None. (Type is a string-typed reserved field; numeric `type` is producer error.)

## Preconditions

1. `SinkEvent::new().insert("type", json!(42))`.

## Postconditions

1. Returns None. (Type is a string-typed reserved field; numeric `type` is producer error.)

## Invariants

1. Drivers can drop events with malformed type without panicking.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | TBD — Phase 1.6b will enumerate edge cases from source review | TBD |

## Canonical Test Vectors

> Test vectors will be pulled from the pinned test case in Phase 1.6b.

| Input | Expected Output | Category |
|-------|----------------|----------|
| `SinkEvent::new().insert("type", json!(42))`. | Returns None. (Type is a string-typed reserved field; numeric `type` is producer error.) | happy-path |
| TBD | TBD | edge-case |
| TBD | TBD | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-001 | Given `SinkEvent::new().insert("type", json!(42))`. When `event_type()`.. Then Returns None. (Type is a string-typed res | manual (existing test: `crates/sink-core/src/lib.rs::tests::sink_event_event_type_non_string_returns_none`) |

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
| **Source line(s)** | 336–340 |
| **Confidence** | high |
| **Original audit ID** | BC-AUDIT-2365 |
| **Pass-3 source** | `pass-3-deep-rust-tests.md:822` |
| **Extraction Date** | 2026-04-25 |
| **Pinned test** | ``crates/sink-core/src/lib.rs::tests::sink_event_event_type_non_string_returns_none`` |
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

