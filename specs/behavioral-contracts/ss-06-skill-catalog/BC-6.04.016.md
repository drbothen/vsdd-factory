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
extracted_from: ".factory/phase-0-ingestion/pass-3-deep-skills-batch-1.md#L591"
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

# Behavioral Contract BC-6.04.016: code-delivery: 4-model-family review (4th model in pr-reviewer)

> Source: `pass-3-deep-skills-batch-1.md` line 591 (was `BC-AUDIT-254`)
> Subsystem: SS-06 — Skill Catalog
> Section: Adversarial and review skills

## Description

code-delivery: 4-model-family review (4th model in pr-reviewer). pr-reviewer reviews PR diff with fresh context using a different model family from implementer, adversary, code-reviewer (the 4th family). Reviews 8 checklist items. Verdict: APPROVE / REQUEST_CHANGES / COMMENT. Findings posted as inline PR comments via github-ops AND written to `.factory/code-delivery/STORY-NNN/pr-review.md`.

## Preconditions

1. Step 5 after PR creation

## Postconditions

1. pr-reviewer model differs from implementer/adversary/code-reviewer model AND output covers all 8 checklist items.

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
| VP-001 | TBD — assertion derived from acceptance: "pr-reviewer model differs from implementer/adversary/code-reviewer model AND output covers all 8 checklist items." | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | TBD |
| L2 Domain Invariants | TBD |
| Architecture Module | plugins/vsdd-factory/skills/code-delivery/SKILL.md |
| Stories | TBD |

## Related BCs (Recommended)

- TBD — link to sibling BCs in same skill (cross-reference once full SS-06 catalog written)

## Architecture Anchors (Recommended)

- `architecture/ss-06-skill-catalog.md#code-delivery-4-model-family-review-(4th-model-in-pr-reviewer)` — TBD anchor

## Story Anchor (Recommended)

TBD — assigned by story-writer

## VP Anchors (Recommended)

- VP-001 — TBD

---

### Brownfield-Specific Sections

#### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `plugins/vsdd-factory/skills/code-delivery/SKILL.md` |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |
| **Source line(s) within skill** | 99-117 |

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
#### BC-AUDIT-254 — code-delivery: 4-model-family review (4th model in pr-reviewer)

**Skill:** `plugins/vsdd-factory/skills/code-delivery/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 99-117
**Trigger:** Step 5 after PR creation
**Behavior:** pr-reviewer reviews PR diff with fresh context using a different model family from implementer, adversary, code-reviewer (the 4th family). Reviews 8 checklist items. Verdict: APPROVE / REQUEST_CHANGES / COMMENT. Findings posted as inline PR comments via github-ops AND written to `.factory/code-delivery/STORY-NNN/pr-review.md`.
**Acceptance:** pr-reviewer model differs from implementer/adversary/code-reviewer model AND output covers all 8 checklist items.
```
