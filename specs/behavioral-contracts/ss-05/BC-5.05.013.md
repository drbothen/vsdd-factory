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
extracted_from: .factory/phase-0-ingestion/pass-3-deep-agents.md:1237
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

# Behavioral Contract BC-5.05.013: spec-reviewer: cannot see implementation details (information wall)

## Description

The spec-reviewer reviews specs and stories only. It MUST NOT load implementation
logs, red-gate logs, or any code-delivery artifacts.

## Preconditions

1. spec-reviewer dispatched.

## Postconditions

1. Tool-call audit shows zero Read against `.factory/cycles/**/implementation/`.
2. Zero Read against `red-gate-log*`.

## Invariants

1. Spec-reviewer judgment must be independent of implementation reasoning.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Agent attempts to Read implementation log | Read denied |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Spec review session | Zero Reads on implementation/ or red-gate-log | happy-path |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-TBD | Tool-call audit confirms info wall | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD |
| L2 Domain Invariants | DI-TBD |
| Architecture Module | `plugins/vsdd-factory/agents/spec-reviewer.md` |
| Stories | TBD |

## Related BCs (Recommended)

- BC-5.04.007 — composes with (spec-reviewer never re-reports adversary)
- BC-5.05.014 — composes with (6-category taxonomy)

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
| **Path** | `plugins/vsdd-factory/agents/spec-reviewer.md:167-174` |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |

#### Evidence Types Used

- documentation: explicit Information Asymmetry Wall section

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | reads (allowed paths) |
| **Global state access** | none |
| **Deterministic** | yes |
| **Thread safety** | unknown |
| **Overall classification** | effectful shell |

#### Refactoring Notes

No refactoring needed.
