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
extracted_from: .factory/phase-0-ingestion/pass-3-deep-agents.md:131
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

# Behavioral Contract BC-5.05.001: architect: every module gets a purity boundary classification

## Description

Every module declared in `module-decomposition.md` MUST be classified as either
Pure Core (deterministic, no I/O) or Effectful Shell. The purity boundary MUST
be drawn before implementation design is finalized.

## Preconditions

1. architect authoring or updating `module-decomposition.md`.

## Postconditions

1. Every module row has a non-empty `purity:` column with value in {pure, effectful}.
2. `purity-boundary-map.md` exists.
3. Purity boundary is drawn before implementation design is finalized.

## Invariants

1. No module is unclassified.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Module with mixed pure + effectful | Classified `effectful`; refactor to extract pure core |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Module-decomposition with all rows classified | Accepted | happy-path |
| Module without purity column | Rejected | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-TBD | Every module-decomposition.md row has purity ∈ {pure, effectful} | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD |
| L2 Domain Invariants | DI-TBD |
| Architecture Module | `plugins/vsdd-factory/agents/architect.md` |
| Stories | TBD |

## Related BCs (Recommended)

- BC-5.07.023 — composes with (formal-verifier purity boundary audit)
- BC-5.07.031 — composes with (implementer respects purity boundary)

## Architecture Anchors (Recommended)

- `architecture/SS-05-orchestration.md#architect`

## Story Anchor (Recommended)

TBD

## VP Anchors (Recommended)

- VP-TBD

---

### Brownfield-Specific Sections

#### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `plugins/vsdd-factory/agents/architect.md:41, 396` |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |

#### Evidence Types Used

- documentation: explicit purity rule

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | writes (architecture docs) |
| **Global state access** | none |
| **Deterministic** | yes |
| **Thread safety** | unknown |
| **Overall classification** | effectful shell |

#### Refactoring Notes

No refactoring needed.
