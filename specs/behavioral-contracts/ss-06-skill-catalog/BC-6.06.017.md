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
extracted_from: ".factory/phase-0-ingestion/pass-3-deep-skills-batch-1.md#L822"
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

# Behavioral Contract BC-6.06.017: convergence-tracking: skill identity (quantitative metrics-driven)

> Source: `pass-3-deep-skills-batch-1.md` line 822 (was `BC-AUDIT-277`)
> Subsystem: SS-06 — Skill Catalog
> Section: State and convergence skills

## Description

convergence-tracking: skill identity (quantitative metrics-driven). Computes quantitative convergence metrics across adversarial passes, mutation testing runs, formal verification results. Produces CONVERGED/NOT_CONVERGED with supporting metrics. Reads adversarial review reports, mutation results, Kani results, fuzz results, Semgrep results, module criticality, cost log.

## Preconditions

1. Convergence assessment

## Postconditions

1. Skill produces objective numerical assessment, not narrative judgment.

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
| VP-001 | TBD — assertion derived from acceptance: "Skill produces objective numerical assessment, not narrative judgment." | manual |

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

- `architecture/ss-06-skill-catalog.md#convergence-tracking-skill-identity-(quantitative-metrics-driven)` — TBD anchor

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
| **Source line(s) within skill** | 1-12 |

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
#### BC-AUDIT-277 — convergence-tracking: skill identity (quantitative metrics-driven)

**Skill:** `plugins/vsdd-factory/skills/convergence-tracking/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 1-12
**Trigger:** Convergence assessment
**Behavior:** Computes quantitative convergence metrics across adversarial passes, mutation testing runs, formal verification results. Produces CONVERGED/NOT_CONVERGED with supporting metrics. Reads adversarial review reports, mutation results, Kani results, fuzz results, Semgrep results, module criticality, cost log.
**Acceptance:** Skill produces objective numerical assessment, not narrative judgment.
```
