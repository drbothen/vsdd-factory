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
input-hash: "c2d3738"
traces_to: .factory/specs/behavioral-contracts/bc-id-mapping.md
origin: brownfield
extracted_from: "pass-3-deep-rust-tests.md:967"
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

# Behavioral Contract BC-3.02.011: sink-file::tag_enrichment_does_not_overwrite_producer_fields: tag with key="type" does NOT clobbe

> Section: File sink behavior
> Source BC (audit ID): BC-AUDIT-2378

## Description

Given Sink configured with `tags = {type: "stomped"}`. Event with `type="commit.made"` submitted. When Flushed.. Then File line has `type == "commit.made"` — producer field WINS over tag-key collision.

## Preconditions

1. Sink configured with `tags = {type: "stomped"}`. Event with `type="commit.made"` submitted.

## Postconditions

1. File line has `type == "commit.made"` — producer field WINS over tag-key collision.

## Invariants

1. Tag enrichment is non-destructive; producer-supplied `type` is authoritative.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | TBD — Phase 1.6b will enumerate edge cases from source review | TBD |

## Canonical Test Vectors

> Test vectors will be pulled from the pinned test case in Phase 1.6b.

| Input | Expected Output | Category |
|-------|----------------|----------|
| Sink configured with `tags = {type: "stomped"}`. Event with `type="commit.made"` submitted. | File line has `type == "commit.made"` — producer field WINS over tag-key collision. | happy-path |
| TBD | TBD | edge-case |
| TBD | TBD | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| (TBD — to be assigned in Phase 1.6c) | Given Sink configured with `tags = {type: "stomped"}`. Event with `type="commit.made"` submitted. When Flushed.. Then Fi | manual (existing test: `crates/sink-file/src/lib.rs::tests::tag_enrichment_does_not_overwrite_producer_fields`) |

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
| **Source line(s)** | 738–759 |
| **Confidence** | high |
| **Original audit ID** | BC-AUDIT-2378 |
| **Pass-3 source** | `pass-3-deep-rust-tests.md:967` |
| **Extraction Date** | 2026-04-25 |
| **Pinned test** | ``crates/sink-file/src/lib.rs::tests::tag_enrichment_does_not_overwrite_producer_fields`` |
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

