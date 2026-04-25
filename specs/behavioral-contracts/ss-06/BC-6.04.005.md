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
extracted_from: ".factory/phase-0-ingestion/pass-3-deep-skills-batch-1.md#L159"
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

# Behavioral Contract BC-6.04.005: adversarial-review: policy rubric auto-loading from policies.yaml

> Source: `pass-3-deep-skills-batch-1.md` line 159 (was `BC-AUDIT-210`)
> Subsystem: SS-06 — Skill Catalog
> Section: Adversarial and review skills

## Description

adversarial-review: policy rubric auto-loading from policies.yaml. Skill MUST read `.factory/policies.yaml`, format each policy as a rubric block (id, name, description, severity, scope, verification steps), and append all blocks under `## Project Policy Rubric` heading in adversary's task prompt. Adversary executes verification steps and reports compliance per-policy.

## Preconditions

1. Before dispatching adversary agent

## Postconditions

1. Adversary task prompt contains a `## Project Policy Rubric` section with one block per policy in policies.yaml.

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
| (TBD — to be assigned in Phase 1.6c) | TBD — assertion derived from acceptance: "Adversary task prompt contains a `## Project Policy Rubric` section with one block per policy in policies.yaml." | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | TBD |
| L2 Domain Invariants | TBD |
| Architecture Module | plugins/vsdd-factory/skills/adversarial-review/SKILL.md |
| Stories | TBD |

## Related BCs (Recommended)

- TBD — link to sibling BCs in same skill (cross-reference once full SS-06 catalog written)

## Architecture Anchors (Recommended)

- `architecture/ss-06-skill-catalog.md#adversarial-review-policy-rubric-auto-loading-from-policies.yaml` — TBD anchor

## Story Anchor (Recommended)

TBD — assigned by story-writer

## VP Anchors (Recommended)

- (TBD — to be assigned in Phase 1.6c)

---

### Brownfield-Specific Sections

#### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `plugins/vsdd-factory/skills/adversarial-review/SKILL.md` |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |
| **Source line(s) within skill** | 106-125 |

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
#### BC-AUDIT-210 — adversarial-review: policy rubric auto-loading from policies.yaml

**Skill:** `plugins/vsdd-factory/skills/adversarial-review/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 106-125
**Trigger:** Before dispatching adversary agent
**Behavior:** Skill MUST read `.factory/policies.yaml`, format each policy as a rubric block (id, name, description, severity, scope, verification steps), and append all blocks under `## Project Policy Rubric` heading in adversary's task prompt. Adversary executes verification steps and reports compliance per-policy.
**Acceptance:** Adversary task prompt contains a `## Project Policy Rubric` section with one block per policy in policies.yaml.
```
