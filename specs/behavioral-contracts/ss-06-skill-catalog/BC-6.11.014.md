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
extracted_from: ".factory/phase-0-ingestion/pass-3-deep-skills-batch-1.md#L1634"
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

# Behavioral Contract BC-6.11.014: factory-worktree-health: skill identity (blocking precondition)

> Source: `pass-3-deep-skills-batch-1.md` line 1634 (was `BC-AUDIT-357`)
> Subsystem: SS-06 — Skill Catalog
> Section: Factory operations and dashboards skills

## Description

factory-worktree-health: skill identity (blocking precondition). Validates factory worktree health on pipeline start. Checks `.factory/` (all modes) and `.factory-project/` (multi-repo only when project.yaml exists). Verifies remote branch existence, local worktree mount, sync state. Auto-repairs if possible. BLOCKING precondition — no pipeline work proceeds until this passes. Executor: devops-engineer (requires `exec` tool).

## Preconditions

1. Every pipeline start (greenfield, brownfield, feature, resume)

## Postconditions

1. Pipeline does not proceed when this skill reports FAIL.

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
| VP-001 | TBD — assertion derived from acceptance: "Pipeline does not proceed when this skill reports FAIL." | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | TBD |
| L2 Domain Invariants | TBD |
| Architecture Module | plugins/vsdd-factory/skills/factory-worktree-health/SKILL.md |
| Stories | TBD |

## Related BCs (Recommended)

- TBD — link to sibling BCs in same skill (cross-reference once full SS-06 catalog written)

## Architecture Anchors (Recommended)

- `architecture/ss-06-skill-catalog.md#factory-worktree-health-skill-identity-(blocking-precondition)` — TBD anchor

## Story Anchor (Recommended)

TBD — assigned by story-writer

## VP Anchors (Recommended)

- VP-001 — TBD

---

### Brownfield-Specific Sections

#### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `plugins/vsdd-factory/skills/factory-worktree-health/SKILL.md` |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |
| **Source line(s) within skill** | 1-19 |

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
#### BC-AUDIT-357 — factory-worktree-health: skill identity (blocking precondition)

**Skill:** `plugins/vsdd-factory/skills/factory-worktree-health/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 1-19
**Trigger:** Every pipeline start (greenfield, brownfield, feature, resume)
**Behavior:** Validates factory worktree health on pipeline start. Checks `.factory/` (all modes) and `.factory-project/` (multi-repo only when project.yaml exists). Verifies remote branch existence, local worktree mount, sync state. Auto-repairs if possible. BLOCKING precondition — no pipeline work proceeds until this passes. Executor: devops-engineer (requires `exec` tool).
**Acceptance:** Pipeline does not proceed when this skill reports FAIL.
```
