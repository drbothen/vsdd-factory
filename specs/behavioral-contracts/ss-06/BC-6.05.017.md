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
extracted_from: ".factory/phase-0-ingestion/pass-3-deep-skills-batch-1.md#L1346"
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

# Behavioral Contract BC-6.05.017: discovery-engine: 7-dimension scoring with weights summing to 1.00

> Source: `pass-3-deep-skills-batch-1.md` line 1346 (was `BC-AUDIT-329`)
> Subsystem: SS-06 — Skill Catalog
> Section: Brownfield, discovery, research skills

## Description

discovery-engine: 7-dimension scoring with weights summing to 1.00. Scores ideas on 7 dimensions: Value (0.25), Feasibility (0.15), Alignment (0.15), Novelty (0.10), Time-Criticality (0.10), Effort (0.10), Evidence Strength (0.15). Weights sum to 1.00. Evidence Strength rubric: 0.0-0.2 speculation; 0.3-0.5 market research only; 0.5-0.6 + one customer signal; 0.6-0.8 multiple signals; 0.8-0.9 all sources; 0.9-1.0 + revenue impact.

## Preconditions

1. Idea evaluation

## Postconditions

1. Each idea has all 7 dimension scores and a composite from weighted average.

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
| VP-001 | TBD — assertion derived from acceptance: "Each idea has all 7 dimension scores and a composite from weighted average." | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | TBD |
| L2 Domain Invariants | TBD |
| Architecture Module | plugins/vsdd-factory/skills/discovery-engine/SKILL.md |
| Stories | TBD |

## Related BCs (Recommended)

- TBD — link to sibling BCs in same skill (cross-reference once full SS-06 catalog written)

## Architecture Anchors (Recommended)

- `architecture/ss-06-skill-catalog.md#discovery-engine-7-dimension-scoring-with-weights-summing-to-1.00` — TBD anchor

## Story Anchor (Recommended)

TBD — assigned by story-writer

## VP Anchors (Recommended)

- VP-001 — TBD

---

### Brownfield-Specific Sections

#### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `plugins/vsdd-factory/skills/discovery-engine/SKILL.md` |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |
| **Source line(s) within skill** | 226-258 |

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
#### BC-AUDIT-329 — discovery-engine: 7-dimension scoring with weights summing to 1.00

**Skill:** `plugins/vsdd-factory/skills/discovery-engine/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 226-258
**Trigger:** Idea evaluation
**Behavior:** Scores ideas on 7 dimensions: Value (0.25), Feasibility (0.15), Alignment (0.15), Novelty (0.10), Time-Criticality (0.10), Effort (0.10), Evidence Strength (0.15). Weights sum to 1.00. Evidence Strength rubric: 0.0-0.2 speculation; 0.3-0.5 market research only; 0.5-0.6 + one customer signal; 0.6-0.8 multiple signals; 0.8-0.9 all sources; 0.9-1.0 + revenue impact.
**Acceptance:** Each idea has all 7 dimension scores and a composite from weighted average.
```
