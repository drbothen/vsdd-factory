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
extracted_from: .factory/phase-0-ingestion/pass-3-deep-agents.md:1259
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

# Behavioral Contract BC-5.05.015: spec-steward: never modifies spec content — governance only

## Description

The spec-steward writes governance artifacts (spec-versions.md, traceability-matrix.md,
spec-changelog.md, drift-reports/) but MUST NOT modify content of PRD, BCs, VPs,
architecture, or stories.

## Preconditions

1. spec-steward dispatched.

## Postconditions

1. Git diff after spec-steward runs touches only `.factory/spec-versions.md`,
   `traceability-matrix.md`, `spec-changelog.md`, `drift-reports/`.
2. Never edits `.factory/specs/` content files.

## Invariants

1. Governance is read-from-content + write-to-governance only.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Spec-steward suggests a content change | Returns suggestion as text; never edits content |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| spec-steward run | Diff confined to governance artifacts | happy-path |
| Attempt to edit BC file | Self-blocked | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-TBD | Git diff after spec-steward runs has zero entries in `.factory/specs/` | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD |
| L2 Domain Invariants | DI-TBD |
| Architecture Module | `plugins/vsdd-factory/agents/spec-steward.md` |
| Stories | TBD |

## Related BCs (Recommended)

- BC-5.05.016 — composes with (version bump enforcement)
- BC-5.05.017 — composes with (locked VP enforcement)
- BC-5.05.018 — composes with (append-only IDs)

## Architecture Anchors (Recommended)

- `architecture/SS-05-orchestration.md#spec-steward`

## Story Anchor (Recommended)

TBD

## VP Anchors (Recommended)

- VP-TBD

---

### Brownfield-Specific Sections

#### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `plugins/vsdd-factory/agents/spec-steward.md:28, 230` |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |

#### Evidence Types Used

- documentation: explicit governance-only rule

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | reads + writes (governance only) |
| **Global state access** | none |
| **Deterministic** | yes |
| **Thread safety** | unknown |
| **Overall classification** | effectful shell |

#### Refactoring Notes

No refactoring needed.
