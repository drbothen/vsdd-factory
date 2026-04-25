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
extracted_from: ".factory/phase-0-ingestion/pass-3-deep-skills-batch-1.md#L640"
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

# Behavioral Contract BC-6.06.011: compact-state: never-deletes safety guarantee

> Source: `pass-3-deep-skills-batch-1.md` line 640 (was `BC-AUDIT-259`)
> Subsystem: SS-06 — Skill Catalog
> Section: State and convergence skills

## Description

compact-state: never-deletes safety guarantee. Skill ONLY moves content between files — never deletes. All extracted content is written to cycle files BEFORE removed from STATE.md. If any write fails, abort without modifying STATE.md. Git commit captures slim STATE.md and new cycle files atomically.

## Preconditions

1. Any extraction operation

## Postconditions

1. Sum of extracted content + remaining STATE.md = original content (no loss); abort path tested.

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
| VP-001 | TBD — assertion derived from acceptance: "Sum of extracted content + remaining STATE.md = original content (no loss); abort path tested." | manual |

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

- `architecture/ss-06-skill-catalog.md#compact-state-never-deletes-safety-guarantee` — TBD anchor

## Story Anchor (Recommended)

TBD — assigned by story-writer

## VP Anchors (Recommended)

- VP-001 — TBD

---

### Brownfield-Specific Sections

#### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `plugins/vsdd-factory/skills/compact-state/SKILL.md` |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |
| **Source line(s) within skill** | 135-141 |

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
#### BC-AUDIT-259 — compact-state: never-deletes safety guarantee

**Skill:** `plugins/vsdd-factory/skills/compact-state/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 135-141
**Trigger:** Any extraction operation
**Behavior:** Skill ONLY moves content between files — never deletes. All extracted content is written to cycle files BEFORE removed from STATE.md. If any write fails, abort without modifying STATE.md. Git commit captures slim STATE.md and new cycle files atomically.
**Acceptance:** Sum of extracted content + remaining STATE.md = original content (no loss); abort path tested.
```
