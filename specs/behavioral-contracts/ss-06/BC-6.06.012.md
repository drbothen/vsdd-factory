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
extracted_from: ".factory/phase-0-ingestion/pass-3-deep-skills-batch-1.md#L649"
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

# Behavioral Contract BC-6.06.012: compact-state: post-compaction STATE.md <200 lines + verify

> Source: `pass-3-deep-skills-batch-1.md` line 649 (was `BC-AUDIT-260`)
> Subsystem: SS-06 — Skill Catalog
> Section: State and convergence skills

## Description

compact-state: post-compaction STATE.md <200 lines + verify. Rewrites STATE.md keeping only: frontmatter (sans adversary_pass_*), Project Metadata, Phase Progress, last-5 Current Phase Steps, Decisions Log, Skip Log, open Blocking Issues, Phase Numbering Reconciliation if present, latest Session Resume Checkpoint only. Replaces extracted sections with pointer block. Verifies <200 lines.

## Preconditions

1. After extraction in Step 4-5

## Postconditions

1. Final STATE.md line count < 200 AND all 9 retained sections present AND adversary_pass_* fields removed from frontmatter.

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
| (TBD — to be assigned in Phase 1.6c) | TBD — assertion derived from acceptance: "Final STATE.md line count < 200 AND all 9 retained sections present AND adversary_pass_* fields removed from frontmatter." | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | TBD |
| L2 Domain Invariants | TBD |
| Architecture Module | plugins/vsdd-factory/skills/compact-state/SKILL.md |
| Stories | TBD |

## Related BCs (Recommended)

- TBD — link to sibling BCs in same skill (cross-reference once full SS-06 catalog written)

## Architecture Anchors (Recommended)

- `architecture/ss-06-skill-catalog.md#compact-state-post-compaction-state.md-<200-lines-+-verify` — TBD anchor

## Story Anchor (Recommended)

TBD — assigned by story-writer

## VP Anchors (Recommended)

- (TBD — to be assigned in Phase 1.6c)

---

### Brownfield-Specific Sections

#### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `plugins/vsdd-factory/skills/compact-state/SKILL.md` |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |
| **Source line(s) within skill** | 76-110 |

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
#### BC-AUDIT-260 — compact-state: post-compaction STATE.md <200 lines + verify

**Skill:** `plugins/vsdd-factory/skills/compact-state/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 76-110
**Trigger:** After extraction in Step 4-5
**Behavior:** Rewrites STATE.md keeping only: frontmatter (sans adversary_pass_*), Project Metadata, Phase Progress, last-5 Current Phase Steps, Decisions Log, Skip Log, open Blocking Issues, Phase Numbering Reconciliation if present, latest Session Resume Checkpoint only. Replaces extracted sections with pointer block. Verifies <200 lines.
**Acceptance:** Final STATE.md line count < 200 AND all 9 retained sections present AND adversary_pass_* fields removed from frontmatter.
```
