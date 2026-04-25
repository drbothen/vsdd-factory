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
extracted_from: ".factory/phase-0-ingestion/pass-3-deep-skills-batch-1.md#L769"
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

# Behavioral Contract BC-6.07.009: consistency-validation: NFR-to-Story severity by priority tier (Rule 27)

> Source: `pass-3-deep-skills-batch-1.md` line 769 (was `BC-AUDIT-272`)
> Subsystem: SS-06 — Skill Catalog
> Section: Spec creation and validation skills

## Description

consistency-validation: NFR-to-Story severity by priority tier (Rule 27). Every NFR-NNN must be referenced by at least one story. Severity by priority: P0 = Critical, P1 = Major, P2 = Minor. Greps story files and dependency-graph.md.

## Preconditions

1. Rule 27 execution

## Postconditions

1. Uncovered NFRs flagged with severity matching priority tier.

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
| VP-001 | TBD — assertion derived from acceptance: "Uncovered NFRs flagged with severity matching priority tier." | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | TBD |
| L2 Domain Invariants | TBD |
| Architecture Module | plugins/vsdd-factory/skills/consistency-validation/SKILL.md |
| Stories | TBD |

## Related BCs (Recommended)

- TBD — link to sibling BCs in same skill (cross-reference once full SS-06 catalog written)

## Architecture Anchors (Recommended)

- `architecture/ss-06-skill-catalog.md#consistency-validation-nfr-to-story-severity-by-priority-tier-(rule-27)` — TBD anchor

## Story Anchor (Recommended)

TBD — assigned by story-writer

## VP Anchors (Recommended)

- VP-001 — TBD

---

### Brownfield-Specific Sections

#### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `plugins/vsdd-factory/skills/consistency-validation/SKILL.md` |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |
| **Source line(s) within skill** | 201-205 |

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
#### BC-AUDIT-272 — consistency-validation: NFR-to-Story severity by priority tier (Rule 27)

**Skill:** `plugins/vsdd-factory/skills/consistency-validation/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 201-205
**Trigger:** Rule 27 execution
**Behavior:** Every NFR-NNN must be referenced by at least one story. Severity by priority: P0 = Critical, P1 = Major, P2 = Minor. Greps story files and dependency-graph.md.
**Acceptance:** Uncovered NFRs flagged with severity matching priority tier.
```
