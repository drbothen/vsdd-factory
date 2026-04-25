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
extracted_from: ".factory/phase-0-ingestion/pass-3-deep-skills-batch-1.md#L849"
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

# Behavioral Contract BC-6.06.020: convergence-tracking: convergence index formula (CI(i))

> Source: `pass-3-deep-skills-batch-1.md` line 849 (was `BC-AUDIT-280`)
> Subsystem: SS-06 — Skill Catalog
> Section: State and convergence skills

## Description

convergence-tracking: convergence index formula (CI(i)). CI(i) = (Novelty(i) * (1 - AvgSimilarity) * (6 - MedianSeverity)) / Cost(i). CONVERGED if verification rate < 60% OR (projected findings < 0.5 AND CI < 0.3 declining for 3+ iterations).

## Preconditions

1. Dimension 3 (Implementation) computation

## Postconditions

1. CI value is computed via the documented formula and threshold check is applied as documented.

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
| VP-001 | TBD — assertion derived from acceptance: "CI value is computed via the documented formula and threshold check is applied as documented." | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | TBD |
| L2 Domain Invariants | TBD |
| Architecture Module | plugins/vsdd-factory/skills/convergence-tracking/SKILL.md |
| Stories | TBD |

## Related BCs (Recommended)

- TBD — link to sibling BCs in same skill (cross-reference once full SS-06 catalog written)

## Architecture Anchors (Recommended)

- `architecture/ss-06-skill-catalog.md#convergence-tracking-convergence-index-formula-(ci(i))` — TBD anchor

## Story Anchor (Recommended)

TBD — assigned by story-writer

## VP Anchors (Recommended)

- VP-001 — TBD

---

### Brownfield-Specific Sections

#### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `plugins/vsdd-factory/skills/convergence-tracking/SKILL.md` |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |
| **Source line(s) within skill** | 62-73 |

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
#### BC-AUDIT-280 — convergence-tracking: convergence index formula (CI(i))

**Skill:** `plugins/vsdd-factory/skills/convergence-tracking/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 62-73
**Trigger:** Dimension 3 (Implementation) computation
**Behavior:** CI(i) = (Novelty(i) * (1 - AvgSimilarity) * (6 - MedianSeverity)) / Cost(i). CONVERGED if verification rate < 60% OR (projected findings < 0.5 AND CI < 0.3 declining for 3+ iterations).
**Acceptance:** CI value is computed via the documented formula and threshold check is applied as documented.
```
