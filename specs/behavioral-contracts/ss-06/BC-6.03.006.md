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
extracted_from: ".factory/phase-0-ingestion/pass-3-deep-skills-batch-1.md#L110"
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

# Behavioral Contract BC-6.03.006: activate: applies per-platform variant via apply-platform.sh

> Source: `pass-3-deep-skills-batch-1.md` line 110 (was `BC-AUDIT-205`)
> Subsystem: SS-06 — Skill Catalog
> Section: Activation and deactivation skills

## Description

activate: applies per-platform variant via apply-platform.sh. Skill MUST run `apply-platform.sh <platform>` which copies `hooks.json.<platform>` to `hooks.json` and verifies dispatcher binary at `hooks/dispatcher/bin/<platform>/factory-dispatcher[.exe]`. Exit codes 0/1/2/3/4 are surfaced to user with helper's stderr verbatim.

## Preconditions

1. Successful detection + write of activation block

## Postconditions

1. Helper's stderr is shown verbatim; binary-missing case (exit 2) is documented as expected on fresh install pre-S-2.4.

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
| (TBD — to be assigned in Phase 1.6c) | TBD — assertion derived from acceptance: "Helper's stderr is shown verbatim; binary-missing case (exit 2) is documented as expected on fresh install pre-S-2.4." | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | TBD |
| L2 Domain Invariants | TBD |
| Architecture Module | plugins/vsdd-factory/skills/activate/SKILL.md |
| Stories | TBD |

## Related BCs (Recommended)

- TBD — link to sibling BCs in same skill (cross-reference once full SS-06 catalog written)

## Architecture Anchors (Recommended)

- `architecture/ss-06-skill-catalog.md#activate-applies-per-platform-variant-via-apply-platform.sh` — TBD anchor

## Story Anchor (Recommended)

TBD — assigned by story-writer

## VP Anchors (Recommended)

- (TBD — to be assigned in Phase 1.6c)

---

### Brownfield-Specific Sections

#### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `plugins/vsdd-factory/skills/activate/SKILL.md` |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |
| **Source line(s) within skill** | 42-48 |

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
#### BC-AUDIT-205 — activate: applies per-platform variant via apply-platform.sh

**Skill:** `plugins/vsdd-factory/skills/activate/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 42-48
**Trigger:** Successful detection + write of activation block
**Behavior:** Skill MUST run `apply-platform.sh <platform>` which copies `hooks.json.<platform>` to `hooks.json` and verifies dispatcher binary at `hooks/dispatcher/bin/<platform>/factory-dispatcher[.exe]`. Exit codes 0/1/2/3/4 are surfaced to user with helper's stderr verbatim.
**Acceptance:** Helper's stderr is shown verbatim; binary-missing case (exit 2) is documented as expected on fresh install pre-S-2.4.
```
