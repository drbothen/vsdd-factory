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
input-hash: "c083e63"
traces_to: .factory/specs/behavioral-contracts/bc-id-mapping.md
origin: brownfield
extracted_from: "pass-3-deep-rust-tests.md:789"
subsystem: "SS-03"
capability: "CAP-003"
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

# Behavioral Contract BC-3.06.001: sink-core::routing_filter_default_accepts_everything: empty allow + empty deny → every event pa

> Section: sink-core base traits and submit/flush
> Source BC (audit ID): BC-AUDIT-2362

## Description

Given `RoutingFilter::default()` (both lists empty). When `accepts(<any-name>)` is called.. Then Always true. Pass-through default is preserved when no filter is configured. (Companion to BC-AUDIT-044.)

## Preconditions

1. `RoutingFilter::default()` (both lists empty).

## Postconditions

1. Always true. Pass-through default is preserved when no filter is configured. (Companion to BC-AUDIT-044.)

## Invariants

1. Operators omitting routing_filter get a no-op pass-through.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | TBD — Phase 1.6b will enumerate edge cases from source review | TBD |

## Canonical Test Vectors

> Test vectors will be pulled from the pinned test case in Phase 1.6b.

| Input | Expected Output | Category |
|-------|----------------|----------|
| `RoutingFilter::default()` (both lists empty). | Always true. Pass-through default is preserved when no filter is configured. (Companion to BC-AUDIT-044.) | happy-path |
| TBD | TBD | edge-case |
| TBD | TBD | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| (TBD — to be assigned in Phase 1.6c) | Given `RoutingFilter::default()` (both lists empty). When `accepts(<any-name>)` is called.. Then Always true. Pass-throu | manual (existing test: `crates/sink-core/src/router_filter.rs::tests::routing_filter_default_accepts_everything`) |

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
| **Source line(s)** | 249–255 |
| **Confidence** | high |
| **Original audit ID** | BC-AUDIT-2362 |
| **Pass-3 source** | `pass-3-deep-rust-tests.md:789` |
| **Extraction Date** | 2026-04-25 |
| **Pinned test** | ``crates/sink-core/src/router_filter.rs::tests::routing_filter_default_accepts_everything`` |
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

