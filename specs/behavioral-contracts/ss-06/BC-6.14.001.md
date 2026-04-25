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
extracted_from: ".factory/phase-0-ingestion/pass-3-deep-skills-batch-1.md#L279"
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

# Behavioral Contract BC-6.14.001: artifact-detection: skill identity contract (universal pipeline front-end)

> Source: `pass-3-deep-skills-batch-1.md` line 279 (was `BC-AUDIT-222`)
> Subsystem: SS-06 — Skill Catalog
> Section: Artifact and verification skills

## Description

artifact-detection: skill identity contract (universal pipeline front-end). Scans for existing planning artifacts (brief, PRD, architecture, UX spec, stories), validates quality, identifies gaps, routes to correct pipeline entry point. Replaces former first step "Receive Product Brief" — does not assume human arrives with a finished brief.

## Preconditions

1. START of every pipeline invocation, before spec crystallization

## Postconditions

1. Skill is the documented Step 0 of every pipeline mode; description names "universal front-end."

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
| (TBD — to be assigned in Phase 1.6c) | TBD — assertion derived from acceptance: "Skill is the documented Step 0 of every pipeline mode; description names "universal front-end."" | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | TBD |
| L2 Domain Invariants | TBD |
| Architecture Module | plugins/vsdd-factory/skills/artifact-detection/SKILL.md |
| Stories | TBD |

## Related BCs (Recommended)

- TBD — link to sibling BCs in same skill (cross-reference once full SS-06 catalog written)

## Architecture Anchors (Recommended)

- `architecture/ss-06-skill-catalog.md#artifact-detection-skill-identity-contract-(universal-pipeline-front-end)` — TBD anchor

## Story Anchor (Recommended)

TBD — assigned by story-writer

## VP Anchors (Recommended)

- (TBD — to be assigned in Phase 1.6c)

---

### Brownfield-Specific Sections

#### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `plugins/vsdd-factory/skills/artifact-detection/SKILL.md` |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |
| **Source line(s) within skill** | 1-17 |

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
#### BC-AUDIT-222 — artifact-detection: skill identity contract (universal pipeline front-end)

**Skill:** `plugins/vsdd-factory/skills/artifact-detection/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 1-17
**Trigger:** START of every pipeline invocation, before spec crystallization
**Behavior:** Scans for existing planning artifacts (brief, PRD, architecture, UX spec, stories), validates quality, identifies gaps, routes to correct pipeline entry point. Replaces former first step "Receive Product Brief" — does not assume human arrives with a finished brief.
**Acceptance:** Skill is the documented Step 0 of every pipeline mode; description names "universal front-end."
```
