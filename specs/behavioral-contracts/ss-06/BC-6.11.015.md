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
extracted_from: ".factory/phase-0-ingestion/pass-3-deep-skills-batch-1.md#L1643"
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

# Behavioral Contract BC-6.11.015: factory-worktree-health: workspace isolation guard (Step 0)

> Source: `pass-3-deep-skills-batch-1.md` line 1643 (was `BC-AUDIT-358`)
> Subsystem: SS-06 — Skill Catalog
> Section: Factory operations and dashboards skills

## Description

factory-worktree-health: workspace isolation guard (Step 0). Verifies cwd does NOT contain "dark-factory"; remote URL does NOT contain "dark-factory"; if `.factory/.git` exists, the gitdir path does NOT contain "dark-factory". Any failure → FATAL exit with documented fix. This prevents engine/project mix-ups.

## Preconditions

1. Before ANY git commands

## Postconditions

1. All 3 isolation checks pass before pipeline proceeds; failure produces FATAL with actionable fix.

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
| (TBD — to be assigned in Phase 1.6c) | TBD — assertion derived from acceptance: "All 3 isolation checks pass before pipeline proceeds; failure produces FATAL with actionable fix." | manual |

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

- `architecture/ss-06-skill-catalog.md#factory-worktree-health-workspace-isolation-guard-(step-0)` — TBD anchor

## Story Anchor (Recommended)

TBD — assigned by story-writer

## VP Anchors (Recommended)

- (TBD — to be assigned in Phase 1.6c)

---

### Brownfield-Specific Sections

#### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `plugins/vsdd-factory/skills/factory-worktree-health/SKILL.md` |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |
| **Source line(s) within skill** | 38-66 |

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
#### BC-AUDIT-358 — factory-worktree-health: workspace isolation guard (Step 0)

**Skill:** `plugins/vsdd-factory/skills/factory-worktree-health/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 38-66
**Trigger:** Before ANY git commands
**Behavior:** Verifies cwd does NOT contain "dark-factory"; remote URL does NOT contain "dark-factory"; if `.factory/.git` exists, the gitdir path does NOT contain "dark-factory". Any failure → FATAL exit with documented fix. This prevents engine/project mix-ups.
**Acceptance:** All 3 isolation checks pass before pipeline proceeds; failure produces FATAL with actionable fix.
```
