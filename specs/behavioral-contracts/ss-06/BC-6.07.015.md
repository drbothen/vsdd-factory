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
extracted_from: ".factory/phase-0-ingestion/pass-3-deep-skills-batch-1.md#L911"
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

# Behavioral Contract BC-6.07.015: create-brief: factory-health prerequisite + research check

> Source: `pass-3-deep-skills-batch-1.md` line 911 (was `BC-AUDIT-286`)
> Subsystem: SS-06 — Skill Catalog
> Section: Spec creation and validation skills

## Description

create-brief: factory-health prerequisite + research check. Run `/factory-health`; if existing `.factory/specs/product-brief.md` exists ask "update vs start fresh"; check `.factory/specs/research/RESEARCH-INDEX.md` for prior domain research and read relevant reports — do NOT ask questions research already answers.

## Preconditions

1. Before discovery starts

## Postconditions

1. Factory-health invoked first; existing brief triggers update-vs-fresh prompt; research reports consulted before redundant questions.

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
| (TBD — to be assigned in Phase 1.6c) | TBD — assertion derived from acceptance: "Factory-health invoked first; existing brief triggers update-vs-fresh prompt; research reports consulted before redundant questions." | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | TBD |
| L2 Domain Invariants | TBD |
| Architecture Module | plugins/vsdd-factory/skills/create-brief/SKILL.md |
| Stories | TBD |

## Related BCs (Recommended)

- TBD — link to sibling BCs in same skill (cross-reference once full SS-06 catalog written)

## Architecture Anchors (Recommended)

- `architecture/ss-06-skill-catalog.md#create-brief-factory-health-prerequisite-+-research-check` — TBD anchor

## Story Anchor (Recommended)

TBD — assigned by story-writer

## VP Anchors (Recommended)

- (TBD — to be assigned in Phase 1.6c)

---

### Brownfield-Specific Sections

#### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `plugins/vsdd-factory/skills/create-brief/SKILL.md` |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |
| **Source line(s) within skill** | 22-26 |

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
#### BC-AUDIT-286 — create-brief: factory-health prerequisite + research check

**Skill:** `plugins/vsdd-factory/skills/create-brief/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 22-26
**Trigger:** Before discovery starts
**Behavior:** Run `/factory-health`; if existing `.factory/specs/product-brief.md` exists ask "update vs start fresh"; check `.factory/specs/research/RESEARCH-INDEX.md` for prior domain research and read relevant reports — do NOT ask questions research already answers.
**Acceptance:** Factory-health invoked first; existing brief triggers update-vs-fresh prompt; research reports consulted before redundant questions.
```
