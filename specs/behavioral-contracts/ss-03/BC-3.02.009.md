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
input-hash: "21367c1"
traces_to: .factory/specs/behavioral-contracts/bc-id-mapping.md
origin: brownfield
extracted_from: "pass-3-deep-rust-tests.md:945"
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

# Behavioral Contract BC-3.02.009: sink-file::routing_filter_drops_excluded_events: allow=["commit.made"] passes 2 of 3 events through

> Section: File sink behavior
> Source BC (audit ID): BC-AUDIT-2376

## Description

Given Sink with allow=["commit.made"]. 3 events submitted: commit.made, plugin.invoked, commit.made. When flush().. Then File has 2 lines, both `type == "commit.made"`. Filter applied at the sink level.

## Preconditions

1. Sink with allow=["commit.made"]. 3 events submitted: commit.made, plugin.invoked, commit.made.

## Postconditions

1. File has 2 lines, both `type == "commit.made"`. Filter applied at the sink level.

## Invariants

1. Per-sink filtering works at the FileSink layer, not just the registry layer.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | TBD — Phase 1.6b will enumerate edge cases from source review | TBD |

## Canonical Test Vectors

> Test vectors will be pulled from the pinned test case in Phase 1.6b.

| Input | Expected Output | Category |
|-------|----------------|----------|
| Sink with allow=["commit.made"]. 3 events submitted: commit.made, plugin.invoked, commit.made. | File has 2 lines, both `type == "commit.made"`. Filter applied at the sink level. | happy-path |
| TBD | TBD | edge-case |
| TBD | TBD | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-001 | Given Sink with allow=["commit.made"]. 3 events submitted: commit.made, plugin.invoked, commit.made. When flush().. Then | manual (existing test: `crates/sink-file/src/lib.rs::tests::routing_filter_drops_excluded_events`) |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | TBD (subsystem L2 spec pending) |
| L2 Domain Invariants | TBD |
| Architecture Module | SS-03 (File sink behavior) |
| Stories | TBD (Phase 2 story-writer pass) |

## Related BCs (Recommended)

- TBD — cross-references will be filled in Phase 1.6b after all per-BC files exist.

## Architecture Anchors (Recommended)

- `architecture/SS-03-observability-sinks.md` — section: File sink behavior

---

### Brownfield-Specific Sections

> This BC was extracted during Phase 0d brownfield ingestion (BC-AUDIT pass) and migrated to canonical one-per-file BC-S.SS.NNN format in Phase 1.4b.

#### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `crates/sink-file/src/lib.rs` |
| **Source line(s)** | 682–710 |
| **Confidence** | high |
| **Original audit ID** | BC-AUDIT-2376 |
| **Pass-3 source** | `pass-3-deep-rust-tests.md:945` |
| **Extraction Date** | 2026-04-25 |
| **Pinned test** | ``crates/sink-file/src/lib.rs::tests::routing_filter_drops_excluded_events`` |
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

