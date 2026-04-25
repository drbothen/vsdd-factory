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
extracted_from: .factory/phase-0-ingestion/pass-3-deep-agents.md:201
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

# Behavioral Contract BC-5.06.003: business-analyst: include all template sections (mark N/A with justification)

## Description

The agent MUST include every template section even when not applicable;
"N/A — justification" is required where a section doesn't apply. Omission is forbidden.

## Preconditions

1. business-analyst authoring an L2 section file.

## Postconditions

1. Every section header from `templates/L2-domain-spec-template.md` appears in the output.
2. Sections marked N/A include a non-empty justification line.

## Invariants

1. No silent omission of template sections.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Section truly N/A | Include header + "N/A — justification" |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Section file with all template headers + justifications | Accepted | happy-path |
| Section file missing one template header | Rejected | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-TBD | Every template section header is present in the output | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD |
| L2 Domain Invariants | DI-TBD |
| Architecture Module | `plugins/vsdd-factory/agents/business-analyst.md` |
| Stories | TBD |

## Related BCs (Recommended)

- BC-5.06.002 — composes with (sharded L2 output)

## Architecture Anchors (Recommended)

- `architecture/SS-05-orchestration.md#business-analyst`

## Story Anchor (Recommended)

TBD

## VP Anchors (Recommended)

- VP-TBD

---

### Brownfield-Specific Sections

#### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `plugins/vsdd-factory/agents/business-analyst.md:31, 49` |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |

#### Evidence Types Used

- documentation: explicit "all sections" rule

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | reads + writes |
| **Global state access** | none |
| **Deterministic** | yes |
| **Thread safety** | unknown |
| **Overall classification** | effectful shell |

#### Refactoring Notes

No refactoring needed.
