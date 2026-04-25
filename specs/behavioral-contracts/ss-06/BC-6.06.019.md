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
extracted_from: ".factory/phase-0-ingestion/pass-3-deep-skills-batch-1.md#L840"
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

# Behavioral Contract BC-6.06.019: convergence-tracking: tier-based mutation kill rate thresholds

> Source: `pass-3-deep-skills-batch-1.md` line 840 (was `BC-AUDIT-279`)
> Subsystem: SS-06 — Skill Catalog
> Section: State and convergence skills

## Description

convergence-tracking: tier-based mutation kill rate thresholds. Per-module kill rate excludes equivalent mutants and compares against tier thresholds: CRITICAL ≥95%, HIGH ≥90%, MEDIUM ≥80%, LOW ≥70%. Survivors classified equivalent/dead-code/insufficient-assertions/complex-logic. CONVERGED requires all modules meet tier target AND survivors >80% equivalent/dead AND all invariants have property tests.

## Preconditions

1. Dimension 2 computation

## Postconditions

1. Each module's verdict cites its tier target and actual kill rate.

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
| VP-001 | TBD — assertion derived from acceptance: "Each module's verdict cites its tier target and actual kill rate." | manual |

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

- `architecture/ss-06-skill-catalog.md#convergence-tracking-tier-based-mutation-kill-rate-thresholds` — TBD anchor

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
| **Source line(s) within skill** | 51-60 |

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
#### BC-AUDIT-279 — convergence-tracking: tier-based mutation kill rate thresholds

**Skill:** `plugins/vsdd-factory/skills/convergence-tracking/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 51-60
**Trigger:** Dimension 2 computation
**Behavior:** Per-module kill rate excludes equivalent mutants and compares against tier thresholds: CRITICAL ≥95%, HIGH ≥90%, MEDIUM ≥80%, LOW ≥70%. Survivors classified equivalent/dead-code/insufficient-assertions/complex-logic. CONVERGED requires all modules meet tier target AND survivors >80% equivalent/dead AND all invariants have property tests.
**Acceptance:** Each module's verdict cites its tier target and actual kill rate.
```
