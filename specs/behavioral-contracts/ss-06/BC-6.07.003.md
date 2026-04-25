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
extracted_from: ".factory/phase-0-ingestion/pass-3-deep-skills-batch-1.md#L711"
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

# Behavioral Contract BC-6.07.003: conform-to-template: user approval gate before write

> Source: `pass-3-deep-skills-batch-1.md` line 711 (was `BC-AUDIT-266`)
> Subsystem: SS-06 — Skill Catalog
> Section: Spec creation and validation skills

## Description

conform-to-template: user approval gate before write. Presents diff (frontmatter additions, section additions, manual fixes needed). Waits for `yes / no / edit` approval. On `no`, stops without changes. On `edit`, allows user to modify plan. On `yes`, creates backup, adds frontmatter fields before closing `---`, adds `[TODO: populate this section per template]` placeholders.

## Preconditions

1. After plan computed

## Postconditions

1. No filesystem mutation observable until explicit `yes`; placeholder format matches the template literal `[TODO: populate this section per template]`.

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
| (TBD — to be assigned in Phase 1.6c) | TBD — assertion derived from acceptance: "No filesystem mutation observable until explicit `yes`; placeholder format matches the template literal `[TODO: populate this section per template]`." | manual |

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

- `architecture/ss-06-skill-catalog.md#conform-to-template-user-approval-gate-before-write` — TBD anchor

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
| **Source line(s) within skill** | 80-115 |

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
#### BC-AUDIT-266 — conform-to-template: user approval gate before write

**Skill:** `plugins/vsdd-factory/skills/conform-to-template/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 80-115
**Trigger:** After plan computed
**Behavior:** Presents diff (frontmatter additions, section additions, manual fixes needed). Waits for `yes / no / edit` approval. On `no`, stops without changes. On `edit`, allows user to modify plan. On `yes`, creates backup, adds frontmatter fields before closing `---`, adds `[TODO: populate this section per template]` placeholders.
**Acceptance:** No filesystem mutation observable until explicit `yes`; placeholder format matches the template literal `[TODO: populate this section per template]`.
```
