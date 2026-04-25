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
extracted_from: ".factory/phase-0-ingestion/pass-3-deep-skills-batch-1.md#L582"
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

# Behavioral Contract BC-6.04.015: code-delivery: per-AC demo evidence with both success and error paths

> Source: `pass-3-deep-skills-batch-1.md` line 582 (was `BC-AUDIT-253`)
> Subsystem: SS-06 — Skill Catalog
> Section: Adversarial and review skills

## Description

code-delivery: per-AC demo evidence with both success and error paths. For each AC, create recording script from template, execute, verify output. Record BOTH success AND error paths for each AC. Generate `docs/demo-evidence/<STORY-ID>/evidence-report.md`. Demo artifacts go to feature branch (per-story subfolder), NOT `.factory/`. Gate: at least 1 recording (`.gif`/`.webm`, NOT `.txt`) per AC, both paths.

## Preconditions

1. Step 2 (per-story demo recording in worktree)

## Postconditions

1. Per-story subfolder exists with evidence-report.md and at least 1 binary recording per AC covering both success and error.

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
| VP-001 | TBD — assertion derived from acceptance: "Per-story subfolder exists with evidence-report.md and at least 1 binary recording per AC covering both success and error." | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | TBD |
| L2 Domain Invariants | TBD |
| Architecture Module | plugins/vsdd-factory/skills/code-delivery/SKILL.md |
| Stories | TBD |

## Related BCs (Recommended)

- TBD — link to sibling BCs in same skill (cross-reference once full SS-06 catalog written)

## Architecture Anchors (Recommended)

- `architecture/ss-06-skill-catalog.md#code-delivery-per-ac-demo-evidence-with-both-success-and-error-paths` — TBD anchor

## Story Anchor (Recommended)

TBD — assigned by story-writer

## VP Anchors (Recommended)

- VP-001 — TBD

---

### Brownfield-Specific Sections

#### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `plugins/vsdd-factory/skills/code-delivery/SKILL.md` |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |
| **Source line(s) within skill** | 38-63 |

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
#### BC-AUDIT-253 — code-delivery: per-AC demo evidence with both success and error paths

**Skill:** `plugins/vsdd-factory/skills/code-delivery/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 38-63
**Trigger:** Step 2 (per-story demo recording in worktree)
**Behavior:** For each AC, create recording script from template, execute, verify output. Record BOTH success AND error paths for each AC. Generate `docs/demo-evidence/<STORY-ID>/evidence-report.md`. Demo artifacts go to feature branch (per-story subfolder), NOT `.factory/`. Gate: at least 1 recording (`.gif`/`.webm`, NOT `.txt`) per AC, both paths.
**Acceptance:** Per-story subfolder exists with evidence-report.md and at least 1 binary recording per AC covering both success and error.
```
