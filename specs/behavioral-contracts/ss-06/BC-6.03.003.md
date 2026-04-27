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
input-hash: "8fa4609"
traces_to: .factory/specs/architecture/ARCH-INDEX.md
origin: brownfield
extracted_from: ".factory/phase-0-ingestion/pass-3-deep-skills-batch-1.md#L83"
subsystem: SS-06
capability: "CAP-007"
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

# Behavioral Contract BC-6.03.003: activate: drift warning on host change

> Source: `pass-3-deep-skills-batch-1.md` line 83 (was `BC-AUDIT-202`)
> Subsystem: SS-06 — Skill Catalog
> Section: Activation and deactivation skills

## Description

activate: drift warning on host change. Skill MUST surface a warning that quotes both the persisted platform and the current host platform; activation continues after warning and the new platform overwrites the persisted value.

## Preconditions

1. `.vsdd-factory.activated_platform` exists in settings.local.json AND does not match current detected platform

## Postconditions

1. Warning string contains both `<persisted>` and `<current>` strings; settings.local.json is updated post-warning.

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
| (TBD — to be assigned in Phase 1.6c) | TBD — assertion derived from acceptance: "Warning string contains both `<persisted>` and `<current>` strings; settings.local.json is updated post-warning." | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-007 ("Deploy and activate the plugin on any supported platform") per capabilities.md §CAP-007 |
| L2 Domain Invariants | TBD |
| Architecture Module | plugins/vsdd-factory/skills/activate/SKILL.md |
| Stories | S-2.06 |

## Related BCs (Recommended)

- TBD — link to sibling BCs in same skill (cross-reference once full SS-06 catalog written)

## Architecture Anchors (Recommended)

- `architecture/ss-06-skill-catalog.md#activate-drift-warning-on-host-change` — TBD anchor

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
| **Source line(s) within skill** | 21-26 |

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
#### BC-AUDIT-202 — activate: drift warning on host change

**Skill:** `plugins/vsdd-factory/skills/activate/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 21-26
**Trigger:** `.vsdd-factory.activated_platform` exists in settings.local.json AND does not match current detected platform
**Behavior:** Skill MUST surface a warning that quotes both the persisted platform and the current host platform; activation continues after warning and the new platform overwrites the persisted value.
**Acceptance:** Warning string contains both `<persisted>` and `<current>` strings; settings.local.json is updated post-warning.
```
