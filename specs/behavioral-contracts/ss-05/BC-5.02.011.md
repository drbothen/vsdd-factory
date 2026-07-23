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
extracted_from: .factory/phase-0-ingestion/pass-3-deep-agents.md:853
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

# Behavioral Contract BC-5.02.011: orchestrator: split bursts of >8 artifacts into create + integrate sub-bursts

## Description

When a single dispatch would create more than 8 artifacts (stories, BCs, etc.),
the orchestrator MUST split into Sub-burst A (Create) and Sub-burst B (Integrate).
121k+ token transcripts degrade output quality.

## Preconditions

1. A dispatch to story-writer or product-owner would create >8 artifacts in one burst.

## Postconditions

1. The orchestrator splits the work into two sequential sub-bursts:
   - Sub-burst A creates the files.
   - Sub-burst B updates indexes and cross-references.
2. The dispatch log shows two sequential dispatches per >8-artifact burst.

## Invariants

1. The 8-artifact threshold is the documented limit.
2. Single-burst creation of >8 artifacts is forbidden due to context overflow.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Burst with exactly 8 artifacts | Single burst is permitted |
| EC-002 | Burst with 9 artifacts | Split required |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Story-writer dispatch creating 12 stories | Split into 8 create + 4 integrate (or similar) | happy-path |
| Story-writer dispatch creating 7 stories | Single burst | happy-path |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-TBD | Burst log shows ≤8 artifacts per dispatch | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD |
| L2 Domain Invariants | DI-TBD |
| Architecture Module | `plugins/vsdd-factory/agents/orchestrator/orchestrator.md` |
| Stories | TBD |

## Related BCs (Recommended)

- BC-5.02.005 — composes with (state-manager last in burst)
- BC-5.06.011 — composes with (story-writer one-file-per-story)

## Architecture Anchors (Recommended)

- `architecture/SS-05-orchestration.md#burst-sizing`

## Story Anchor (Recommended)

TBD

## VP Anchors (Recommended)

- VP-TBD

---

### Brownfield-Specific Sections

#### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `plugins/vsdd-factory/agents/orchestrator/orchestrator.md:129` |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |

#### Evidence Types Used

- documentation: explicit split rule in orchestrator agent body

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | none |
| **Global state access** | reads dispatch artifact count |
| **Deterministic** | yes |
| **Thread safety** | unknown |
| **Overall classification** | pure (numerical guard) |

#### Refactoring Notes

No refactoring needed.
