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
extracted_from: ".factory/phase-0-ingestion/pass-3-deep-skills-batch-1.md#L462"
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

# Behavioral Contract BC-6.06.003: check-input-drift: mandatory resolve step after scan

> Source: `pass-3-deep-skills-batch-1.md` line 462 (was `BC-AUDIT-241`)
> Subsystem: SS-06 — Skill Catalog
> Section: State and convergence skills

## Description

check-input-drift: mandatory resolve step after scan. Skill MUST always run `--scan .factory --resolve`. Files with missing inputs cannot be hashed; the binary refuses to hash partial input sets because partial hash produces false MATCH. If UNRESOLVABLE>0, diagnose before proceeding. NOT skipping this step is mandatory.

## Preconditions

1. After Step 1 scan completes

## Postconditions

1. Resolve invocation always follows scan; UNRESOLVABLE>0 blocks proceeding without diagnosis.

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
| VP-001 | TBD — assertion derived from acceptance: "Resolve invocation always follows scan; UNRESOLVABLE>0 blocks proceeding without diagnosis." | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | TBD |
| L2 Domain Invariants | TBD |
| Architecture Module | plugins/vsdd-factory/skills/check-input-drift/SKILL.md |
| Stories | TBD |

## Related BCs (Recommended)

- TBD — link to sibling BCs in same skill (cross-reference once full SS-06 catalog written)

## Architecture Anchors (Recommended)

- `architecture/ss-06-skill-catalog.md#check-input-drift-mandatory-resolve-step-after-scan` — TBD anchor

## Story Anchor (Recommended)

TBD — assigned by story-writer

## VP Anchors (Recommended)

- VP-001 — TBD

---

### Brownfield-Specific Sections

#### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `plugins/vsdd-factory/skills/check-input-drift/SKILL.md` |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |
| **Source line(s) within skill** | 61-93 |

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
#### BC-AUDIT-241 — check-input-drift: mandatory resolve step after scan

**Skill:** `plugins/vsdd-factory/skills/check-input-drift/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 61-93
**Trigger:** After Step 1 scan completes
**Behavior:** Skill MUST always run `--scan .factory --resolve`. Files with missing inputs cannot be hashed; the binary refuses to hash partial input sets because partial hash produces false MATCH. If UNRESOLVABLE>0, diagnose before proceeding. NOT skipping this step is mandatory.
**Acceptance:** Resolve invocation always follows scan; UNRESOLVABLE>0 blocks proceeding without diagnosis.
```
