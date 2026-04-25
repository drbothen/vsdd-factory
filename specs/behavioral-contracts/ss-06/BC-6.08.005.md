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
extracted_from: ".factory/phase-0-ingestion/pass-3-deep-skills-batch-1.md#L1266"
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

# Behavioral Contract BC-6.08.005: design-drift-detection: skill identity (Sweep 10, UI products only)

> Source: `pass-3-deep-skills-batch-1.md` line 1266 (was `BC-AUDIT-321`)
> Subsystem: SS-06 — Skill Catalog
> Section: Demo, UX, and design skills

## Description

design-drift-detection: skill identity (Sweep 10, UI products only). Detects design system drift: token overrides (hardcoded values), component misuse (invalid prop combos), pattern violations, emergent patterns (new clusters), style inconsistency. Sweep 10 alongside existing 9 sweeps in maintenance.lobster. Primary agent: ux-designer; supporting: consistency-validator.

## Preconditions

1. Weekly during maintenance sweep, condition `state.has_ui == true`

## Postconditions

1. Frontmatter declares condition `state.has_ui == true`; produces design-drift.md.

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
| (TBD — to be assigned in Phase 1.6c) | TBD — assertion derived from acceptance: "Frontmatter declares condition `state.has_ui == true`; produces design-drift.md." | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | TBD |
| L2 Domain Invariants | TBD |
| Architecture Module | plugins/vsdd-factory/skills/design-drift-detection/SKILL.md |
| Stories | TBD |

## Related BCs (Recommended)

- TBD — link to sibling BCs in same skill (cross-reference once full SS-06 catalog written)

## Architecture Anchors (Recommended)

- `architecture/ss-06-skill-catalog.md#design-drift-detection-skill-identity-(sweep-10,-ui-products-only)` — TBD anchor

## Story Anchor (Recommended)

TBD — assigned by story-writer

## VP Anchors (Recommended)

- (TBD — to be assigned in Phase 1.6c)

---

### Brownfield-Specific Sections

#### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `plugins/vsdd-factory/skills/design-drift-detection/SKILL.md` |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |
| **Source line(s) within skill** | 1-22 |

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
#### BC-AUDIT-321 — design-drift-detection: skill identity (Sweep 10, UI products only)

**Skill:** `plugins/vsdd-factory/skills/design-drift-detection/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 1-22
**Trigger:** Weekly during maintenance sweep, condition `state.has_ui == true`
**Behavior:** Detects design system drift: token overrides (hardcoded values), component misuse (invalid prop combos), pattern violations, emergent patterns (new clusters), style inconsistency. Sweep 10 alongside existing 9 sweeps in maintenance.lobster. Primary agent: ux-designer; supporting: consistency-validator.
**Acceptance:** Frontmatter declares condition `state.has_ui == true`; produces design-drift.md.
```
