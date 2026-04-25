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
input-hash: "b47141b"
traces_to: .factory/specs/behavioral-contracts/bc-id-mapping.md
origin: brownfield
extracted_from: "pass-3-deep-rust-tests.md:901"
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

# Behavioral Contract BC-3.02.005: sink-file::template_no_project_yields_empty_basename: template uses {project} but None passed →

> Section: File sink behavior
> Source BC (audit ID): BC-AUDIT-2372

## Description

Given project_dir = None, template = `{project}events-{date}.jsonl`. When Resolved.. Then Returns `events-2026-04-24.jsonl` (project replaced with empty string, not error).

## Preconditions

1. project_dir = None, template = `{project}events-{date}.jsonl`.

## Postconditions

1. Returns `events-2026-04-24.jsonl` (project replaced with empty string, not error).

## Invariants

1. Optional project_dir gracefully degrades to empty rather than failing config load.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | TBD — Phase 1.6b will enumerate edge cases from source review | TBD |

## Canonical Test Vectors

> Test vectors will be pulled from the pinned test case in Phase 1.6b.

| Input | Expected Output | Category |
|-------|----------------|----------|
| project_dir = None, template = `{project}events-{date}.jsonl`. | Returns `events-2026-04-24.jsonl` (project replaced with empty string, not error). | happy-path |
| TBD | TBD | edge-case |
| TBD | TBD | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-001 | Given project_dir = None, template = `{project}events-{date}.jsonl`. When Resolved.. Then Returns `events-2026-04-24.jso | manual (existing test: `crates/sink-file/src/lib.rs::tests::template_no_project_yields_empty_basename`) |

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
| **Source line(s)** | 587–592 |
| **Confidence** | high |
| **Original audit ID** | BC-AUDIT-2372 |
| **Pass-3 source** | `pass-3-deep-rust-tests.md:901` |
| **Extraction Date** | 2026-04-25 |
| **Pinned test** | ``crates/sink-file/src/lib.rs::tests::template_no_project_yields_empty_basename`` |
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

