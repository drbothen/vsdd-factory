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
extracted_from: .factory/phase-0-ingestion/pass-3-deep-agents.md:1481
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

# Behavioral Contract BC-5.03.012: ux-designer: every interaction has both success AND error paths

## Description

Every interaction flow in `flows/FLOW-NNN-*.md` MUST define both a success path
and at least one error path. One-path flows are rejected.

## Preconditions

1. ux-designer is authoring a flow file.

## Postconditions

1. Each flow file contains a `## Success Path` section AND a `## Error Path` section
   (or equivalent named subsections).

## Invariants

1. Single-path flows are rejected.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Flow with only success path | Rejected |
| EC-002 | Flow with multiple error paths | Accepted |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Flow with success + error sections | Accepted | happy-path |
| Flow with only success section | Rejected | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-TBD | Every flow file has both success and error sections | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD |
| L2 Domain Invariants | DI-TBD |
| Architecture Module | `plugins/vsdd-factory/agents/ux-designer.md` |
| Stories | TBD |

## Related BCs (Recommended)

- BC-5.03.013 — composes with (sharded UX)

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
| **Path** | `plugins/vsdd-factory/agents/ux-designer.md:96, 117` |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |

#### Evidence Types Used

- documentation: explicit dual-path rule

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | writes (flow files) |
| **Global state access** | none |
| **Deterministic** | yes |
| **Thread safety** | unknown |
| **Overall classification** | effectful shell |

#### Refactoring Notes

No refactoring needed.
