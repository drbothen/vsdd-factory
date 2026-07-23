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
extracted_from: .factory/phase-0-ingestion/pass-3-deep-agents.md:1359
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

# Behavioral Contract BC-5.06.013: story-writer: no story exceeds 13 points or 20-30% agent context window

## Description

Story sizing constraints: max 13 story points; max 20-30% of implementing agent's
context window. Stories exceeding either constraint MUST be split.

## Preconditions

1. story-writer authoring a story.

## Postconditions

1. STORY-INDEX.md shows no story with `points > 13`.
2. Every story's Token Budget Estimate ≤ 30% of target agent context.

## Invariants

1. Both bounds (points and tokens) are mandatory.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Story near 30% context | Split into smaller stories |
| EC-002 | Story is 13 points exactly | Accepted |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Story with 8 points and 22% context | Accepted | happy-path |
| Story with 14 points | Rejected (split) | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-TBD | Every story has points ≤ 13 and context ≤ 30% | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD |
| L2 Domain Invariants | DI-TBD |
| Architecture Module | `plugins/vsdd-factory/agents/story-writer.md` |
| Stories | TBD |

## Related BCs (Recommended)

- BC-5.06.011 — composes with (one-file-per-story)

## Architecture Anchors (Recommended)

- `architecture/SS-05-orchestration.md#story-writer`

## Story Anchor (Recommended)

TBD

## VP Anchors (Recommended)

- VP-TBD

---

### Brownfield-Specific Sections

#### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `plugins/vsdd-factory/agents/story-writer.md:56, 221, 226` |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |

#### Evidence Types Used

- documentation: explicit sizing rules

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | writes |
| **Global state access** | none |
| **Deterministic** | yes |
| **Thread safety** | unknown |
| **Overall classification** | pure (sizing check) |

#### Refactoring Notes

No refactoring needed.
