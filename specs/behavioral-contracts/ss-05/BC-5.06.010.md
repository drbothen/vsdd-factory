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
extracted_from: .factory/phase-0-ingestion/pass-3-deep-agents.md:1061
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

# Behavioral Contract BC-5.06.010: product-owner: subsystem ID from ARCH-INDEX, never names

## Description

BC frontmatter `subsystem:` field MUST be SS-NN format from ARCH-INDEX Subsystem
Registry. Names (e.g., "Sensor Adapters") are forbidden — only IDs.
Pre-architecture: use `SS-TBD` placeholder.

## Preconditions

1. product-owner setting BC frontmatter `subsystem:` field.

## Postconditions

1. BC frontmatter `subsystem:` matches regex `^SS-(TBD|\d+)$`.

## Invariants

1. Subsystem references are by ID, never name.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Pre-architecture phase | Use `SS-TBD` |
| EC-002 | Architecture renames a subsystem | BCs unchanged (still by ID) |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| `subsystem: SS-05` | Accepted | happy-path |
| `subsystem: "Pipeline Orchestration"` | Rejected | error |
| `subsystem: SS-TBD` | Accepted (pre-architecture) | edge-case |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-TBD | Every BC frontmatter `subsystem:` matches the canonical regex | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD |
| L2 Domain Invariants | DI-TBD |
| Architecture Module | `plugins/vsdd-factory/agents/product-owner.md` |
| Stories | TBD |

## Related BCs (Recommended)

- BC-5.06.005 — composes with (BC numbering)

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
| **Path** | `plugins/vsdd-factory/agents/product-owner.md:108-113` |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |

#### Evidence Types Used

- documentation: explicit Subsystem ID Validation rule

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | reads (ARCH-INDEX) + writes (BC frontmatter) |
| **Global state access** | none |
| **Deterministic** | yes |
| **Thread safety** | unknown |
| **Overall classification** | pure |

#### Refactoring Notes

No refactoring needed.
