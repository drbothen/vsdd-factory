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
extracted_from: ".factory/phase-0-ingestion/pass-3-deep-skills-batch-1.md#L431"
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

# Behavioral Contract BC-6.05.008: brownfield-ingest: Phase B.6 extraction validation with behavioral/metric split

> Source: `pass-3-deep-skills-batch-1.md` line 431 (was `BC-AUDIT-238`)
> Subsystem: SS-06 — Skill Catalog
> Section: Brownfield, discovery, research skills

## Description

brownfield-ingest: Phase B.6 extraction validation with behavioral/metric split. validate-extraction agent splits work into Phase 1 (behavioral verification using judgment — sample contracts/entities/relationships against source) and Phase 2 (metric verification using arithmetic — recount every numeric claim). Two tables, one per phase. Up to 3 refinement iterations.

## Preconditions

1. After B.5 passes

## Postconditions

1. `<project>-extraction-validation.md` exists with two tables (behavioral + metric); metric table compares claimed vs recounted values; PASS/FAIL verdict rendered after ≤3 iterations.

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
| VP-001 | TBD — assertion derived from acceptance: "`<project>-extraction-validation.md` exists with two tables (behavioral + metric); metric table compares claimed vs recounted values; PASS/FAIL verdict rendered after ≤3 iterations." | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | TBD |
| L2 Domain Invariants | TBD |
| Architecture Module | plugins/vsdd-factory/skills/brownfield-ingest/SKILL.md |
| Stories | TBD |

## Related BCs (Recommended)

- TBD — link to sibling BCs in same skill (cross-reference once full SS-06 catalog written)

## Architecture Anchors (Recommended)

- `architecture/ss-06-skill-catalog.md#brownfield-ingest-phase-b.6-extraction-validation-with-behavioral/metric-split` — TBD anchor

## Story Anchor (Recommended)

TBD — assigned by story-writer

## VP Anchors (Recommended)

- VP-001 — TBD

---

### Brownfield-Specific Sections

#### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `plugins/vsdd-factory/skills/brownfield-ingest/SKILL.md` |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |
| **Source line(s) within skill** | 305-359 |

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
#### BC-AUDIT-238 — brownfield-ingest: Phase B.6 extraction validation with behavioral/metric split

**Skill:** `plugins/vsdd-factory/skills/brownfield-ingest/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 305-359
**Trigger:** After B.5 passes
**Behavior:** validate-extraction agent splits work into Phase 1 (behavioral verification using judgment — sample contracts/entities/relationships against source) and Phase 2 (metric verification using arithmetic — recount every numeric claim). Two tables, one per phase. Up to 3 refinement iterations.
**Acceptance:** `<project>-extraction-validation.md` exists with two tables (behavioral + metric); metric table compares claimed vs recounted values; PASS/FAIL verdict rendered after ≤3 iterations.
```
