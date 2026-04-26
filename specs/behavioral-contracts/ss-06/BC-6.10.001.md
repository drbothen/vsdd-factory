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
input-hash: "f63eb74"
traces_to: .factory/specs/architecture/ARCH-INDEX.md
origin: brownfield
extracted_from: ".factory/phase-0-ingestion/pass-3-deep-skills-batch-1.md#L1177"
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

# Behavioral Contract BC-6.10.001: deliver-story: skill identity (dispatcher, not implementer)

> Source: `pass-3-deep-skills-batch-1.md` line 1177 (was `BC-AUDIT-312`)
> Subsystem: SS-06 — Skill Catalog
> Section: Story delivery skills

## Description

deliver-story: skill identity (dispatcher, not implementer). Skill is a DISPATCHER — does not write code, write tests, create worktrees, or open PRs directly. Reads canonical workflow from `agents/orchestrator/per-story-delivery.md` and delegates each step to a fresh specialist subagent. Single-context delivery is a correctness bug. `allowed-tools: Read, Bash, Glob, Grep, AskUserQuestion, Task`.

## Preconditions

1. User invokes `/deliver-story STORY-NNN`

## Postconditions

1. Skill's allowed-tools does NOT include Write/Edit; uses Task tool to dispatch specialists.

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
| (TBD — to be assigned in Phase 1.6c) | TBD — assertion derived from acceptance: "Skill's allowed-tools does NOT include Write/Edit; uses Task tool to dispatch specialists." | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | TBD |
| L2 Domain Invariants | TBD |
| Architecture Module | SS-06 (Skill Catalog) — plugins/vsdd-factory/skills/deliver-story/SKILL.md |
| Stories | TBD |

## Related BCs (Recommended)

- TBD — link to sibling BCs in same skill (cross-reference once full SS-06 catalog written)

## Architecture Anchors (Recommended)

- `architecture/ss-06-skill-catalog.md#deliver-story-skill-identity-(dispatcher,-not-implementer)` — TBD anchor

## Story Anchor (Recommended)

TBD — assigned by story-writer

## VP Anchors (Recommended)

- (TBD — to be assigned in Phase 1.6c)

---

### Brownfield-Specific Sections

#### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `plugins/vsdd-factory/skills/deliver-story/SKILL.md` |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |
| **Source line(s) within skill** | 1-15 |

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
#### BC-AUDIT-312 — deliver-story: skill identity (dispatcher, not implementer)

**Skill:** `plugins/vsdd-factory/skills/deliver-story/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 1-15
**Trigger:** User invokes `/deliver-story STORY-NNN`
**Behavior:** Skill is a DISPATCHER — does not write code, write tests, create worktrees, or open PRs directly. Reads canonical workflow from `agents/orchestrator/per-story-delivery.md` and delegates each step to a fresh specialist subagent. Single-context delivery is a correctness bug. `allowed-tools: Read, Bash, Glob, Grep, AskUserQuestion, Task`.
**Acceptance:** Skill's allowed-tools does NOT include Write/Edit; uses Task tool to dispatch specialists.
```
