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
extracted_from: ".factory/phase-0-ingestion/pass-3-deep-skills-batch-1.md#L368"
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

# Behavioral Contract BC-6.05.001: brownfield-ingest: skill identity (broad-then-converge protocol)

> Source: `pass-3-deep-skills-batch-1.md` line 368 (was `BC-AUDIT-231`)
> Subsystem: SS-06 — Skill Catalog
> Section: Brownfield, discovery, research skills

## Description

brownfield-ingest: skill identity (broad-then-converge protocol). Analyzes existing codebase via 6 broad passes (0-6) followed by iterative deepening on every pass until novelty decays to NITPICK. Produces complete semantic understanding feeding spec crystallization.

## Preconditions

1. User invokes `/brownfield-ingest <path-or-url> [--resume]`

## Postconditions

1. Frontmatter declares `name: brownfield-ingest`; argument-hint matches `[codebase-path] [--resume]`.

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
| VP-001 | TBD — assertion derived from acceptance: "Frontmatter declares `name: brownfield-ingest`; argument-hint matches `[codebase-path] [--resume]`." | manual |

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

- `architecture/ss-06-skill-catalog.md#brownfield-ingest-skill-identity-(broad-then-converge-protocol)` — TBD anchor

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
| **Source line(s) within skill** | 1-9 |

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
#### BC-AUDIT-231 — brownfield-ingest: skill identity (broad-then-converge protocol)

**Skill:** `plugins/vsdd-factory/skills/brownfield-ingest/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 1-9
**Trigger:** User invokes `/brownfield-ingest <path-or-url> [--resume]`
**Behavior:** Analyzes existing codebase via 6 broad passes (0-6) followed by iterative deepening on every pass until novelty decays to NITPICK. Produces complete semantic understanding feeding spec crystallization.
**Acceptance:** Frontmatter declares `name: brownfield-ingest`; argument-hint matches `[codebase-path] [--resume]`.
```
