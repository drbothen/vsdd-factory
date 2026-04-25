---
document_type: behavioral-contract
level: L3
version: "1.1"
status: draft
producer: phase-1-4b-bc-extractor
timestamp: 2026-04-25T00:00:00
phase: 1.4b
inputs:
  - .factory/phase-0-ingestion/pass-3-deep-skills-batch-1.md
  - .factory/specs/behavioral-contracts/bc-id-mapping.md
input-hash: "TBD"
traces_to: .factory/specs/architecture/ARCH-INDEX.md
origin: brownfield
extracted_from: ".factory/phase-0-ingestion/pass-3-deep-skills-batch-1.md#L1164"
subsystem: SS-06
capability: "TBD"
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

# Behavioral Contract BC-6.07.034: decompose-stories: 5 named output artifacts + holdout scenarios

> Source: `pass-3-deep-skills-batch-1.md` line 1164 (was `BC-AUDIT-311`)
> Subsystem: SS-06 — Skill Catalog
> Section: Spec creation and validation skills

## Description

decompose-stories: 5 named output artifacts + holdout scenarios. Writes `.factory/stories/epics.md`, per-story files `.factory/stories/STORY-NNN.md` with wave field, `.factory/stories/dependency-graph.md`, `.factory/cycles/<current>/wave-schedule.md`, `.factory/stories/STORY-INDEX.md`, `.factory/stories/sprint-state.yaml`, plus holdout scenarios in `.factory/holdout-scenarios/wave-scenarios/` and `HS-INDEX.md`.

## Preconditions

1. Output writing

## Postconditions

1. All listed artifacts exist post-decomposition.

## Invariants

1. TBD — derive from skill SKILL.md frontmatter and acceptance criteria.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | TBD | TBD |

## Canonical Test Vectors

> Golden-file test inputs and expected outputs.

| Input | Expected Output | Category |
|-------|----------------|----------|
| TBD — happy path from skill acceptance | TBD | happy-path |
| TBD — edge case | TBD | edge-case |
| TBD — error case | TBD | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| (TBD — to be assigned in Phase 1.6c) | TBD — assertion derived from acceptance: "All listed artifacts exist post-decomposition." | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | TBD |
| L2 Domain Invariants | TBD |
| Architecture Module | plugins/vsdd-factory/skills/decompose-stories/SKILL.md |
| Stories | TBD |

## Related BCs (Recommended)

- TBD — link to sibling BCs in same skill (cross-reference once full SS-06 catalog written)

## Architecture Anchors (Recommended)

- `architecture/ss-06-skill-catalog.md#decompose-stories-5-named-output-artifacts-+-holdout-scenarios` — TBD anchor

## Story Anchor (Recommended)

TBD — assigned by story-writer

## VP Anchors (Recommended)

- (TBD — to be assigned in Phase 1.6c)

---

### Brownfield-Specific Sections

#### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `plugins/vsdd-factory/skills/decompose-stories/SKILL.md` |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |
| **Source line(s) within skill** | 71-152 |

#### Evidence Types Used

- documentation: stated in SKILL.md frontmatter and prose
- inferred: behavior derived from skill acceptance criteria

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | TBD |
| **Global state access** | TBD |
| **Deterministic** | TBD |
| **Thread safety** | TBD |
| **Overall classification** | TBD |

#### Refactoring Notes

TBD — assess once architecture mapping is complete.

#### Source Excerpt (verbatim)

```text
#### BC-AUDIT-311 — decompose-stories: 5 named output artifacts + holdout scenarios

**Skill:** `plugins/vsdd-factory/skills/decompose-stories/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 71-152
**Trigger:** Output writing
**Behavior:** Writes `.factory/stories/epics.md`, per-story files `.factory/stories/STORY-NNN.md` with wave field, `.factory/stories/dependency-graph.md`, `.factory/cycles/<current>/wave-schedule.md`, `.factory/stories/STORY-INDEX.md`, `.factory/stories/sprint-state.yaml`, plus holdout scenarios in `.factory/holdout-scenarios/wave-scenarios/` and `HS-INDEX.md`.
**Acceptance:** All listed artifacts exist post-decomposition.
```
