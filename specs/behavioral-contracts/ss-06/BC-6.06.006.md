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
extracted_from: ".factory/phase-0-ingestion/pass-3-deep-skills-batch-1.md#L493"
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

# Behavioral Contract BC-6.06.006: check-state-health: 7 numbered checks executed in order

> Source: `pass-3-deep-skills-batch-1.md` line 493 (was `BC-AUDIT-244`)
> Subsystem: SS-06 — Skill Catalog
> Section: State and convergence skills

## Description

check-state-health: 7 numbered checks executed in order. Runs (1) Existence; (2) Frontmatter validation against fixed schema; (3) Size check (≤200 HEALTHY, 201-500 WARNING, 501+ NEEDS-COMPACT); (4) Phase numbering (no Phase 3.5/4.x adversar/5.x formal/6.x converg); (5) Structure compliance (6 required sections); (6) Content routing compliance (5 antipatterns); (7) Convergence counter format `N of 3`.

## Preconditions

1. Skill invocation

## Postconditions

1. Output table has exactly 7 numbered rows.

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
| (TBD — to be assigned in Phase 1.6c) | TBD — assertion derived from acceptance: "Output table has exactly 7 numbered rows." | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | TBD |
| L2 Domain Invariants | TBD |
| Architecture Module | plugins/vsdd-factory/skills/check-state-health/SKILL.md |
| Stories | TBD |

## Related BCs (Recommended)

- TBD — link to sibling BCs in same skill (cross-reference once full SS-06 catalog written)

## Architecture Anchors (Recommended)

- `architecture/ss-06-skill-catalog.md#check-state-health-7-numbered-checks-executed-in-order` — TBD anchor

## Story Anchor (Recommended)

TBD — assigned by story-writer

## VP Anchors (Recommended)

- (TBD — to be assigned in Phase 1.6c)

---

### Brownfield-Specific Sections

#### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `plugins/vsdd-factory/skills/check-state-health/SKILL.md` |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |
| **Source line(s) within skill** | 18-93 |

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
#### BC-AUDIT-244 — check-state-health: 7 numbered checks executed in order

**Skill:** `plugins/vsdd-factory/skills/check-state-health/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 18-93
**Trigger:** Skill invocation
**Behavior:** Runs (1) Existence; (2) Frontmatter validation against fixed schema; (3) Size check (≤200 HEALTHY, 201-500 WARNING, 501+ NEEDS-COMPACT); (4) Phase numbering (no Phase 3.5/4.x adversar/5.x formal/6.x converg); (5) Structure compliance (6 required sections); (6) Content routing compliance (5 antipatterns); (7) Convergence counter format `N of 3`.
**Acceptance:** Output table has exactly 7 numbered rows.
```
