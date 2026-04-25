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
extracted_from: .factory/phase-0-ingestion/pass-3-deep-agents.md:1473
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

# Behavioral Contract BC-5.03.011: ux-designer: every screen traces to a PRD requirement

## Description

Every SCR-NNN screen file MUST trace to a specific PRD section/BC. UI elements
without spec justification are forbidden.

## Preconditions

1. ux-designer is authoring a screen file.

## Postconditions

1. Screen file frontmatter has `traces_to:` listing a PRD section or BC.
2. Every UI element in the screen has spec justification.

## Invariants

1. No invented screens — every screen grounded in PRD.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Screen lacks PRD trace | Rejected |
| EC-002 | UI element with no spec justification | Rejected |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Screen with `traces_to: PRD§4.2` and per-element justification | Accepted | happy-path |
| Screen without `traces_to` | Rejected | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-TBD | Every screen file has a non-empty traces_to | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD |
| L2 Domain Invariants | DI-TBD |
| Architecture Module | `plugins/vsdd-factory/agents/ux-designer.md` |
| Stories | TBD |

## Related BCs (Recommended)

- BC-5.03.013 — composes with (sharded UX output)

## Architecture Anchors (Recommended)

- `architecture/SS-05-orchestration.md#ux-designer`

## Story Anchor (Recommended)

TBD

## VP Anchors (Recommended)

- VP-TBD

---

### Brownfield-Specific Sections

#### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `plugins/vsdd-factory/agents/ux-designer.md:95, 119, 218` |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |

#### Evidence Types Used

- documentation: explicit traces-to rule

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | writes (screen files) |
| **Global state access** | reads PRD |
| **Deterministic** | yes |
| **Thread safety** | unknown |
| **Overall classification** | effectful shell |

#### Refactoring Notes

No refactoring needed.
