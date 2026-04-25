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
extracted_from: ".factory/phase-0-ingestion/pass-3-deep-skills-batch-1.md#L1213"
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

# Behavioral Contract BC-6.10.005: deliver-story: context discipline mapping per specialist

> Source: `pass-3-deep-skills-batch-1.md` line 1213 (was `BC-AUDIT-316`)
> Subsystem: SS-06 — Skill Catalog
> Section: Story delivery skills

## Description

deliver-story: context discipline mapping per specialist. Pass only the minimum context to each specialist per a fixed mapping: devops-engineer → worktree protocol; test-writer (stubs) → story+dep-graph+api-surface+BCs; test-writer (tests) → story+api-surface+test-vectors+BCs; implementer → story+module-decomp+dep-graph+api-surface+BCs; demo-recorder → story+AC extract only; pr-manager → story ID + branch name + template path. If story ≥60% of agent context window, STOP and dispatch story-writer to split.

## Preconditions

1. Each Task dispatch

## Postconditions

1. Each dispatch payload matches the documented per-specialist mapping; no whole-story-file passes to demo-recorder/pr-manager.

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
| VP-001 | TBD — assertion derived from acceptance: "Each dispatch payload matches the documented per-specialist mapping; no whole-story-file passes to demo-recorder/pr-manager." | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | TBD |
| L2 Domain Invariants | TBD |
| Architecture Module | plugins/vsdd-factory/skills/deliver-story/SKILL.md |
| Stories | TBD |

## Related BCs (Recommended)

- TBD — link to sibling BCs in same skill (cross-reference once full SS-06 catalog written)

## Architecture Anchors (Recommended)

- `architecture/ss-06-skill-catalog.md#deliver-story-context-discipline-mapping-per-specialist` — TBD anchor

## Story Anchor (Recommended)

TBD — assigned by story-writer

## VP Anchors (Recommended)

- VP-001 — TBD

---

### Brownfield-Specific Sections

#### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `plugins/vsdd-factory/skills/deliver-story/SKILL.md` |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |
| **Source line(s) within skill** | 122-136 |

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
#### BC-AUDIT-316 — deliver-story: context discipline mapping per specialist

**Skill:** `plugins/vsdd-factory/skills/deliver-story/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 122-136
**Trigger:** Each Task dispatch
**Behavior:** Pass only the minimum context to each specialist per a fixed mapping: devops-engineer → worktree protocol; test-writer (stubs) → story+dep-graph+api-surface+BCs; test-writer (tests) → story+api-surface+test-vectors+BCs; implementer → story+module-decomp+dep-graph+api-surface+BCs; demo-recorder → story+AC extract only; pr-manager → story ID + branch name + template path. If story ≥60% of agent context window, STOP and dispatch story-writer to split.
**Acceptance:** Each dispatch payload matches the documented per-specialist mapping; no whole-story-file passes to demo-recorder/pr-manager.
```
