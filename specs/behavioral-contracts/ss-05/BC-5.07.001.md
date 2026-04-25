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
extracted_from: .factory/phase-0-ingestion/pass-3-deep-agents.md:223
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

# Behavioral Contract BC-5.07.001: code-reviewer: cannot see adversarial reviews (information wall)

## Description

The code-reviewer is a SECONDARY reviewer providing cognitive diversity (different
model family from Builder + Adversary). It MUST NOT load any file under
`.factory/cycles/**/adversarial-reviews/`. If information is needed from behind
the wall, it must be derived independently from the artifacts the reviewer can see.

## Preconditions

1. code-reviewer dispatched.

## Postconditions

1. Tool-call audit shows zero Read calls against `.factory/cycles/**/adversarial-reviews/`.

## Invariants

1. Cognitive-diversity wall is structural.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Reviewer needs adversary's known-issue list | Derive independently |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| code-reviewer session | Zero Reads on adversarial-reviews/ | happy-path |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-TBD | Tool-call audit confirms info wall | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD |
| L2 Domain Invariants | DI-TBD |
| Architecture Module | `plugins/vsdd-factory/agents/code-reviewer.md` |
| Stories | TBD |

## Related BCs (Recommended)

- BC-5.07.002 — composes with (6-category classification)
- BC-5.07.003 — composes with (no re-report)

## Architecture Anchors (Recommended)

- `architecture/SS-05-orchestration.md#code-reviewer`

## Story Anchor (Recommended)

TBD

## VP Anchors (Recommended)

- VP-TBD

---

### Brownfield-Specific Sections

#### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `plugins/vsdd-factory/agents/code-reviewer.md:117-131` |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |

#### Evidence Types Used

- documentation: explicit Information Asymmetry Wall

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
