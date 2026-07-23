---
document_type: behavioral-contract
level: L3
version: "1.1"
status: draft
producer: codebase-analyzer
timestamp: 2026-04-25T00:00:00
phase: 1.4b
inputs: [.factory/phase-0-ingestion/pass-3-deep-agents.md]
input-hash: "595f07d"
traces_to: .factory/specs/architecture/ARCH-INDEX.md
origin: brownfield
extracted_from: .factory/phase-0-ingestion/pass-3-deep-agents.md:1021
subsystem: SS-05
capability: CAP-TBD
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

# Behavioral Contract BC-5.06.005: product-owner: BC-S.SS.NNN numbering scheme

## Description

Every BC ID MUST follow `BC-S.SS.NNN` format where S = PRD section number,
SS = subsection (0-99, mapped to L2 CAP-NNN), NNN = sequential 001-999. BCs
are grouped by L2 domain subsystems, not implementation modules.

## Preconditions

1. product-owner authoring or numbering BCs.

## Postconditions

1. All BC files match regex `^BC-\d+\.\d{1,2}\.\d{3}\.md$`.
2. BC-INDEX rows comply with the numbering scheme.

## Invariants

1. Numbering is canonical and append-only.
2. Grouping is by L2 domain subsystem, not by implementation module.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | BC-1.0.001 (single-digit subsection) | Accepted (regex permits 1-2 digit SS) |
| EC-002 | BC-1.001.0001 (4-digit NNN) | Rejected |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| BC-2.03.045.md | Accepted | happy-path |
| BC-2-3-45.md | Rejected | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-TBD | Every BC filename matches the canonical regex | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD |
| L2 Domain Invariants | DI-TBD |
| Architecture Module | `plugins/vsdd-factory/agents/product-owner.md` |
| Stories | TBD |

## Related BCs (Recommended)

- BC-5.06.006 — composes with (BC H1 title authority)
- BC-5.06.010 — composes with (subsystem ID from ARCH-INDEX)

## Architecture Anchors (Recommended)

- `architecture/SS-05-orchestration.md#product-owner`

## Story Anchor (Recommended)

TBD

## VP Anchors (Recommended)

- VP-TBD

---

### Brownfield-Specific Sections

#### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `plugins/vsdd-factory/agents/product-owner.md:30, 70-79` |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |

#### Evidence Types Used

- documentation: explicit BC Numbering Rules section

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | none (naming) |
| **Global state access** | none |
| **Deterministic** | yes |
| **Thread safety** | unknown |
| **Overall classification** | pure |

#### Refactoring Notes

No refactoring needed.
