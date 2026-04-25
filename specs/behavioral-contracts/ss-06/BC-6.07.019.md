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
extracted_from: ".factory/phase-0-ingestion/pass-3-deep-skills-batch-1.md#L951"
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

# Behavioral Contract BC-6.07.019: create-domain-spec: 3-pass extraction (structural + behavioral + context)

> Source: `pass-3-deep-skills-batch-1.md` line 951 (was `BC-AUDIT-290`)
> Subsystem: SS-06 — Skill Catalog
> Section: Spec creation and validation skills

## Description

create-domain-spec: 3-pass extraction (structural + behavioral + context). Pass 1 (Structural): nouns — entities, relationships, value objects, ubiquitous language. Pass 2 (Behavioral): verbs — processes, domain events, business rules/invariants, state machines. Pass 3 (Context): bounded contexts. Brownfield enhancement: read `.factory/semport/` Pass 2/3 if present and validate with user rather than ask from scratch.

## Preconditions

1. Discovery process

## Postconditions

1. All 3 passes covered with sub-pass deliverables; brownfield path checks semport.

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
| (TBD — to be assigned in Phase 1.6c) | TBD — assertion derived from acceptance: "All 3 passes covered with sub-pass deliverables; brownfield path checks semport." | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | TBD |
| L2 Domain Invariants | TBD |
| Architecture Module | plugins/vsdd-factory/skills/create-domain-spec/SKILL.md |
| Stories | TBD |

## Related BCs (Recommended)

- TBD — link to sibling BCs in same skill (cross-reference once full SS-06 catalog written)

## Architecture Anchors (Recommended)

- `architecture/ss-06-skill-catalog.md#create-domain-spec-3-pass-extraction-(structural-+-behavioral-+-context)` — TBD anchor

## Story Anchor (Recommended)

TBD — assigned by story-writer

## VP Anchors (Recommended)

- (TBD — to be assigned in Phase 1.6c)

---

### Brownfield-Specific Sections

#### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `plugins/vsdd-factory/skills/create-domain-spec/SKILL.md` |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |
| **Source line(s) within skill** | 26-83 |

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
#### BC-AUDIT-290 — create-domain-spec: 3-pass extraction (structural + behavioral + context)

**Skill:** `plugins/vsdd-factory/skills/create-domain-spec/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 26-83
**Trigger:** Discovery process
**Behavior:** Pass 1 (Structural): nouns — entities, relationships, value objects, ubiquitous language. Pass 2 (Behavioral): verbs — processes, domain events, business rules/invariants, state machines. Pass 3 (Context): bounded contexts. Brownfield enhancement: read `.factory/semport/` Pass 2/3 if present and validate with user rather than ask from scratch.
**Acceptance:** All 3 passes covered with sub-pass deliverables; brownfield path checks semport.
```
