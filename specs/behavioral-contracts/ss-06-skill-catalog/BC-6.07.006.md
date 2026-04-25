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
extracted_from: ".factory/phase-0-ingestion/pass-3-deep-skills-batch-1.md#L742"
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

# Behavioral Contract BC-6.07.006: consistency-validation: 36 numbered rules executed in order

> Source: `pass-3-deep-skills-batch-1.md` line 742 (was `BC-AUDIT-269`)
> Subsystem: SS-06 — Skill Catalog
> Section: Spec creation and validation skills

## Description

consistency-validation: 36 numbered rules executed in order. Runs Rules 0-36 in order: spec format (DF-020), PRD→Epic, Epic→Story, Story→Architecture, Story→UX, AC testability, VP coverage, Dependency acyclicity, Data model consistency, Performance target alignment, Purity boundary consistency, Semantic drift detection, Token budget, Upstream traceability chain, Downstream traceability completeness, L1→L2/L2→L3/L3→L4/L1→L4 chain, BC-to-Story, AC-to-BC, VP registry completeness, Design system token compliance, Component contract compliance, UI traceability, BC clause reverse coverage, EC+E error reverse coverage, NFR-to-Story reverse coverage, Holdout-BC-AC alignment, UI state completeness, PRD scope+differentiator enforcement, PRD RTM completeness, Frontmatter cross-reference integrity, BC lifecycle field coherence, FM-NNN to holdout coverage, L2 sharding integrity, UX sharding integrity (UI products only).

## Preconditions

1. Skill invocation

## Postconditions

1. Output table contains exactly 36 numbered rule rows with Status (PASS/FAIL) and violation lists.

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
| VP-001 | TBD — assertion derived from acceptance: "Output table contains exactly 36 numbered rule rows with Status (PASS/FAIL) and violation lists." | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | TBD |
| L2 Domain Invariants | TBD |
| Architecture Module | plugins/vsdd-factory/skills/consistency-validation/SKILL.md |
| Stories | TBD |

## Related BCs (Recommended)

- TBD — link to sibling BCs in same skill (cross-reference once full SS-06 catalog written)

## Architecture Anchors (Recommended)

- `architecture/ss-06-skill-catalog.md#consistency-validation-36-numbered-rules-executed-in-order` — TBD anchor

## Story Anchor (Recommended)

TBD — assigned by story-writer

## VP Anchors (Recommended)

- VP-001 — TBD

---

### Brownfield-Specific Sections

#### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `plugins/vsdd-factory/skills/consistency-validation/SKILL.md` |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |
| **Source line(s) within skill** | 21-289 |

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
#### BC-AUDIT-269 — consistency-validation: 36 numbered rules executed in order

**Skill:** `plugins/vsdd-factory/skills/consistency-validation/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 21-289
**Trigger:** Skill invocation
**Behavior:** Runs Rules 0-36 in order: spec format (DF-020), PRD→Epic, Epic→Story, Story→Architecture, Story→UX, AC testability, VP coverage, Dependency acyclicity, Data model consistency, Performance target alignment, Purity boundary consistency, Semantic drift detection, Token budget, Upstream traceability chain, Downstream traceability completeness, L1→L2/L2→L3/L3→L4/L1→L4 chain, BC-to-Story, AC-to-BC, VP registry completeness, Design system token compliance, Component contract compliance, UI traceability, BC clause reverse coverage, EC+E error reverse coverage, NFR-to-Story reverse coverage, Holdout-BC-AC alignment, UI state completeness, PRD scope+differentiator enforcement, PRD RTM completeness, Frontmatter cross-reference integrity, BC lifecycle field coherence, FM-NNN to holdout coverage, L2 sharding integrity, UX sharding integrity (UI products only).
**Acceptance:** Output table contains exactly 36 numbered rule rows with Status (PASS/FAIL) and violation lists.
```
