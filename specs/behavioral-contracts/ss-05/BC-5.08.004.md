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
extracted_from: .factory/phase-0-ingestion/pass-3-deep-agents.md:369
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

# Behavioral Contract BC-5.08.004: data-engineer: every schema traces to a BC-NNN data contract

## Description

Every schema artifact MUST trace to one or more BC-S.SS.NNN data structure
contracts. Schema docs use canonical frontmatter with `traces_to:` populated.

## Preconditions

1. data-engineer authoring or modifying a schema.

## Postconditions

1. Every schema file's frontmatter has a `traces_to:` field listing at least one BC-S.SS.NNN.

## Invariants

1. No untraced schemas.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Schema covers multiple BCs | Multiple BC IDs in traces_to |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Schema with `traces_to: [BC-2.01.001]` | Accepted | happy-path |
| Schema without traces_to | Rejected | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-TBD | Every schema file has non-empty traces_to | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD |
| L2 Domain Invariants | DI-TBD |
| Architecture Module | `plugins/vsdd-factory/agents/data-engineer.md` |
| Stories | TBD |

## Related BCs (Recommended)

- BC-5.08.002 — composes with (privacy classification)

## Architecture Anchors (Recommended)

- `architecture/SS-05-orchestration.md#data-engineer`

## Story Anchor (Recommended)

TBD

## VP Anchors (Recommended)

- VP-TBD

---

### Brownfield-Specific Sections

#### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `plugins/vsdd-factory/agents/data-engineer.md:48, 108-110` |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |

#### Evidence Types Used

- documentation: explicit BC-NNN Schema Tracing rule

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | writes (schema files) |
| **Global state access** | none |
| **Deterministic** | yes |
| **Thread safety** | unknown |
| **Overall classification** | pure |

#### Refactoring Notes

No refactoring needed.
