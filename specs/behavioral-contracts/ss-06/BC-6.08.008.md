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
extracted_from: ".factory/phase-0-ingestion/pass-3-deep-skills-batch-1.md#L1297"
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

# Behavioral Contract BC-6.08.008: design-system-bootstrap: skill identity (greenfield + brownfield + feature)

> Source: `pass-3-deep-skills-batch-1.md` line 1297 (was `BC-AUDIT-324`)
> Subsystem: SS-06 — Skill Catalog
> Section: Demo, UX, and design skills

## Description

design-system-bootstrap: skill identity (greenfield + brownfield + feature). Bootstraps design system. Greenfield: from product brief + brand guidelines. Brownfield: extracts from existing codebase. Produces `.factory/design-system/` with tokens, components, patterns, constraints. Human review gate before downstream use. Primary: ux-designer; supporting: architect, codebase-analyzer, accessibility-auditor.

## Preconditions

1. Greenfield Phase 1 (after spec crystallization, before story decomp); Brownfield Phase 0 (during ingestion); Feature F2 (only if no design system); condition `feature_type in ['ui', 'full-stack']`

## Postconditions

1. `.factory/design-system/` directory exists with the 4-section structure (tokens/components/patterns/constraints.yaml).

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
| VP-001 | TBD — assertion derived from acceptance: "`.factory/design-system/` directory exists with the 4-section structure (tokens/components/patterns/constraints.yaml)." | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | TBD |
| L2 Domain Invariants | TBD |
| Architecture Module | plugins/vsdd-factory/skills/design-system-bootstrap/SKILL.md |
| Stories | TBD |

## Related BCs (Recommended)

- TBD — link to sibling BCs in same skill (cross-reference once full SS-06 catalog written)

## Architecture Anchors (Recommended)

- `architecture/ss-06-skill-catalog.md#design-system-bootstrap-skill-identity-(greenfield-+-brownfield-+-feature)` — TBD anchor

## Story Anchor (Recommended)

TBD — assigned by story-writer

## VP Anchors (Recommended)

- VP-001 — TBD

---

### Brownfield-Specific Sections

#### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `plugins/vsdd-factory/skills/design-system-bootstrap/SKILL.md` |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |
| **Source line(s) within skill** | 1-26 |

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
#### BC-AUDIT-324 — design-system-bootstrap: skill identity (greenfield + brownfield + feature)

**Skill:** `plugins/vsdd-factory/skills/design-system-bootstrap/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 1-26
**Trigger:** Greenfield Phase 1 (after spec crystallization, before story decomp); Brownfield Phase 0 (during ingestion); Feature F2 (only if no design system); condition `feature_type in ['ui', 'full-stack']`
**Behavior:** Bootstraps design system. Greenfield: from product brief + brand guidelines. Brownfield: extracts from existing codebase. Produces `.factory/design-system/` with tokens, components, patterns, constraints. Human review gate before downstream use. Primary: ux-designer; supporting: architect, codebase-analyzer, accessibility-auditor.
**Acceptance:** `.factory/design-system/` directory exists with the 4-section structure (tokens/components/patterns/constraints.yaml).
```
