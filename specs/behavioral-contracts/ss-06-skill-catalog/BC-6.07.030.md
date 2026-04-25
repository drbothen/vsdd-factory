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
extracted_from: ".factory/phase-0-ingestion/pass-3-deep-skills-batch-1.md#L1062"
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

# Behavioral Contract BC-6.07.030: create-story: forbidden dependencies + version pin enforcement

> Source: `pass-3-deep-skills-batch-1.md` line 1062 (was `BC-AUDIT-301`)
> Subsystem: SS-06 — Skill Catalog
> Section: Spec creation and validation skills

## Description

create-story: forbidden dependencies + version pin enforcement. Story MUST include verbatim external dependency table from `dependency-graph.md` in "Library & Framework Requirements". MUST include "Forbidden Dependencies" section listing crates/packages that must NOT appear in module's dependency graph (compile-time enforcement). Error codes MUST come from `prd-supplements/error-taxonomy.md` (new codes flagged "NEW — add E-xxx-NNN to taxonomy").

## Preconditions

1. Story finalization

## Postconditions

1. Story has Library Requirements with verbatim version pins, Forbidden Dependencies section, and error codes from existing taxonomy or flagged NEW.

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
| VP-001 | TBD — assertion derived from acceptance: "Story has Library Requirements with verbatim version pins, Forbidden Dependencies section, and error codes from existing taxonomy or flagged NEW." | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | TBD |
| L2 Domain Invariants | TBD |
| Architecture Module | plugins/vsdd-factory/skills/create-story/SKILL.md |
| Stories | TBD |

## Related BCs (Recommended)

- TBD — link to sibling BCs in same skill (cross-reference once full SS-06 catalog written)

## Architecture Anchors (Recommended)

- `architecture/ss-06-skill-catalog.md#create-story-forbidden-dependencies-+-version-pin-enforcement` — TBD anchor

## Story Anchor (Recommended)

TBD — assigned by story-writer

## VP Anchors (Recommended)

- VP-001 — TBD

---

### Brownfield-Specific Sections

#### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `plugins/vsdd-factory/skills/create-story/SKILL.md` |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |
| **Source line(s) within skill** | 100-112 |

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
#### BC-AUDIT-301 — create-story: forbidden dependencies + version pin enforcement

**Skill:** `plugins/vsdd-factory/skills/create-story/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 100-112
**Trigger:** Story finalization
**Behavior:** Story MUST include verbatim external dependency table from `dependency-graph.md` in "Library & Framework Requirements". MUST include "Forbidden Dependencies" section listing crates/packages that must NOT appear in module's dependency graph (compile-time enforcement). Error codes MUST come from `prd-supplements/error-taxonomy.md` (new codes flagged "NEW — add E-xxx-NNN to taxonomy").
**Acceptance:** Story has Library Requirements with verbatim version pins, Forbidden Dependencies section, and error codes from existing taxonomy or flagged NEW.
```
