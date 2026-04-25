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
extracted_from: ".factory/phase-0-ingestion/pass-3-deep-skills-batch-1.md#L288"
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

# Behavioral Contract BC-6.14.002: artifact-detection: 5-tier readiness classification (L0-L4)

> Source: `pass-3-deep-skills-batch-1.md` line 288 (was `BC-AUDIT-223`)
> Subsystem: SS-06 — Skill Catalog
> Section: Artifact and verification skills

## Description

artifact-detection: 5-tier readiness classification (L0-L4). Classifies project as L0 (nothing) → Collaborative Discovery; L1 (brief) → spec crystallization; L2 (PRD) → remaining crystallization; L3 (PRD+arch+L3 BCs) → story decomposition; L4 (PRD+arch+L3+L4+stories) → implementation.

## Preconditions

1. After artifact scan complete

## Postconditions

1. Output declares exactly one of {L0, L1, L2, L3, L4} per the artifact set found.

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
| (TBD — to be assigned in Phase 1.6c) | TBD — assertion derived from acceptance: "Output declares exactly one of {L0, L1, L2, L3, L4} per the artifact set found." | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | TBD |
| L2 Domain Invariants | TBD |
| Architecture Module | plugins/vsdd-factory/skills/artifact-detection/SKILL.md |
| Stories | TBD |

## Related BCs (Recommended)

- TBD — link to sibling BCs in same skill (cross-reference once full SS-06 catalog written)

## Architecture Anchors (Recommended)

- `architecture/ss-06-skill-catalog.md#artifact-detection-5-tier-readiness-classification-(l0-l4)` — TBD anchor

## Story Anchor (Recommended)

TBD — assigned by story-writer

## VP Anchors (Recommended)

- (TBD — to be assigned in Phase 1.6c)

---

### Brownfield-Specific Sections

#### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `plugins/vsdd-factory/skills/artifact-detection/SKILL.md` |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |
| **Source line(s) within skill** | 60-72 |

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
#### BC-AUDIT-223 — artifact-detection: 5-tier readiness classification (L0-L4)

**Skill:** `plugins/vsdd-factory/skills/artifact-detection/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 60-72
**Trigger:** After artifact scan complete
**Behavior:** Classifies project as L0 (nothing) → Collaborative Discovery; L1 (brief) → spec crystallization; L2 (PRD) → remaining crystallization; L3 (PRD+arch+L3 BCs) → story decomposition; L4 (PRD+arch+L3+L4+stories) → implementation.
**Acceptance:** Output declares exactly one of {L0, L1, L2, L3, L4} per the artifact set found.
```
