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
extracted_from: ".factory/phase-0-ingestion/pass-3-deep-skills-batch-1.md#L422"
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

# Behavioral Contract BC-6.05.007: brownfield-ingest: Phase B.5 coverage audit is mandatory

> Source: `pass-3-deep-skills-batch-1.md` line 422 (was `BC-AUDIT-237`)
> Subsystem: SS-06 — Skill Catalog
> Section: Brownfield, discovery, research skills

## Description

brownfield-ingest: Phase B.5 coverage audit is mandatory. Must run grep-driven (not agent-judgment) coverage audit. Inventory source tree, grep deep files for package/subsystem references, flag zero/minimal hits as blind spots. One agent per project. Loop until no substantive gaps remain. Empirically every secrets-corpus repo had blind spots after 19-62 rounds — B.5 is the only check that catches them.

## Preconditions

1. After all passes reach NITPICK

## Postconditions

1. `<project>-coverage-audit.md` exists; subsequent passes have either PASS verdict or surgical mini-rounds (`-phase-b5-tr-N.md`) that close gaps.

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
| VP-001 | TBD — assertion derived from acceptance: "`<project>-coverage-audit.md` exists; subsequent passes have either PASS verdict or surgical mini-rounds (`-phase-b5-tr-N.md`) that close gaps." | manual |

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

- `architecture/ss-06-skill-catalog.md#brownfield-ingest-phase-b.5-coverage-audit-is-mandatory` — TBD anchor

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
| **Source line(s) within skill** | 264-303 |

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
#### BC-AUDIT-237 — brownfield-ingest: Phase B.5 coverage audit is mandatory

**Skill:** `plugins/vsdd-factory/skills/brownfield-ingest/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 264-303
**Trigger:** After all passes reach NITPICK
**Behavior:** Must run grep-driven (not agent-judgment) coverage audit. Inventory source tree, grep deep files for package/subsystem references, flag zero/minimal hits as blind spots. One agent per project. Loop until no substantive gaps remain. Empirically every secrets-corpus repo had blind spots after 19-62 rounds — B.5 is the only check that catches them.
**Acceptance:** `<project>-coverage-audit.md` exists; subsequent passes have either PASS verdict or surgical mini-rounds (`-phase-b5-tr-N.md`) that close gaps.
```
