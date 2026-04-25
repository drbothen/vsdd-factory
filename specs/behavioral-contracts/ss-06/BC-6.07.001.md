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
extracted_from: ".factory/phase-0-ingestion/pass-3-deep-skills-batch-1.md#L693"
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

# Behavioral Contract BC-6.07.001: conform-to-template: skill identity (additive only — never deletes)

> Source: `pass-3-deep-skills-batch-1.md` line 693 (was `BC-AUDIT-264`)
> Subsystem: SS-06 — Skill Catalog
> Section: Spec creation and validation skills

## Description

conform-to-template: skill identity (additive only — never deletes). Fixes structural gaps by adding missing frontmatter fields, sections, table structures from the template. Safety guarantees: never deletes, never modifies existing content, always shows changes before applying, creates backup `<filename>.backup-YYYY-MM-DD-HHMMSS`, single file per invocation.

## Preconditions

1. After validate-template-compliance reports WARN/FAIL, during template migration, or when manually-created artifacts need structure

## Postconditions

1. Argument parsing accepts `<file-path> [--template=<template-name>]`; backup file exists post-modification.

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
| (TBD — to be assigned in Phase 1.6c) | TBD — assertion derived from acceptance: "Argument parsing accepts `<file-path> [--template=<template-name>]`; backup file exists post-modification." | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | TBD |
| L2 Domain Invariants | TBD |
| Architecture Module | plugins/vsdd-factory/skills/conform-to-template/SKILL.md |
| Stories | TBD |

## Related BCs (Recommended)

- TBD — link to sibling BCs in same skill (cross-reference once full SS-06 catalog written)

## Architecture Anchors (Recommended)

- `architecture/ss-06-skill-catalog.md#conform-to-template-skill-identity-(additive-only-never-deletes)` — TBD anchor

## Story Anchor (Recommended)

TBD — assigned by story-writer

## VP Anchors (Recommended)

- (TBD — to be assigned in Phase 1.6c)

---

### Brownfield-Specific Sections

#### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `plugins/vsdd-factory/skills/conform-to-template/SKILL.md` |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |
| **Source line(s) within skill** | 1-25 |

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
#### BC-AUDIT-264 — conform-to-template: skill identity (additive only — never deletes)

**Skill:** `plugins/vsdd-factory/skills/conform-to-template/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 1-25
**Trigger:** After validate-template-compliance reports WARN/FAIL, during template migration, or when manually-created artifacts need structure
**Behavior:** Fixes structural gaps by adding missing frontmatter fields, sections, table structures from the template. Safety guarantees: never deletes, never modifies existing content, always shows changes before applying, creates backup `<filename>.backup-YYYY-MM-DD-HHMMSS`, single file per invocation.
**Acceptance:** Argument parsing accepts `<file-path> [--template=<template-name>]`; backup file exists post-modification.
```
