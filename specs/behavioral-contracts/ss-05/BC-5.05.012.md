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
extracted_from: .factory/phase-0-ingestion/pass-3-deep-agents.md:1229
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

# Behavioral Contract BC-5.05.012: spec-reviewer: SR-NNN ID space, distinct from ADV-NNN and CR-NNN

## Description

Spec-reviewer findings use the SR-NNN namespace, distinct from adversary's
ADV-NNN and code-reviewer's CR-NNN.

## Preconditions

1. spec-reviewer producing a finding.

## Postconditions

1. Finding ID uses SR-NNN format.
2. No SR-NNN ID collides with an existing ADV-NNN or CR-NNN ID anywhere in the project.

## Invariants

1. ID namespaces are disjoint.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | New SR-NNN reuses an existing ID number | Still distinct because of prefix |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| SR-001 finding | Distinct from ADV-001 and CR-001 | happy-path |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-TBD | No SR-NNN collides with ADV-NNN or CR-NNN | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD |
| L2 Domain Invariants | DI-TBD |
| Architecture Module | `plugins/vsdd-factory/agents/spec-reviewer.md` |
| Stories | TBD |

## Related BCs (Recommended)

- BC-5.04.007 — composes with (spec-reviewer never re-reports adversary findings)

## Architecture Anchors (Recommended)

- `architecture/SS-05-orchestration.md#spec-reviewer`

## Story Anchor (Recommended)

TBD

## VP Anchors (Recommended)

- VP-TBD

---

### Brownfield-Specific Sections

#### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `plugins/vsdd-factory/agents/spec-reviewer.md:53-54, 130-138` |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |

#### Evidence Types Used

- documentation: explicit SR-NNN namespace rule

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
