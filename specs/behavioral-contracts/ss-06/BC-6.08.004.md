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
extracted_from: ".factory/phase-0-ingestion/pass-3-deep-skills-batch-1.md#L1253"
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

# Behavioral Contract BC-6.08.004: demo-recording: every AC has user-observable behavior covered + visual review

> Source: `pass-3-deep-skills-batch-1.md` line 1253 (was `BC-AUDIT-320`)
> Subsystem: SS-06 — Skill Catalog
> Section: Demo, UX, and design skills

## Description

demo-recording: every AC has user-observable behavior covered + visual review. Every AC with user-observable behavior has a recording. Recordings under 2MB/5MB. Full user journey demo for happy path. Evidence report links all recordings. PR description snippet generated. ffmpeg post-processed (optimized, trimmed, labeled). Visual reviewer (DF-018) analyzes all recordings and writes findings to `.factory/demo-evidence/visual-review.md`.

## Preconditions

1. Quality gate

## Postconditions

1. All 7 quality-gate items pass; visual-review.md exists.

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
| (TBD — to be assigned in Phase 1.6c) | TBD — assertion derived from acceptance: "All 7 quality-gate items pass; visual-review.md exists." | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | TBD |
| L2 Domain Invariants | TBD |
| Architecture Module | plugins/vsdd-factory/skills/demo-recording/SKILL.md |
| Stories | TBD |

## Related BCs (Recommended)

- TBD — link to sibling BCs in same skill (cross-reference once full SS-06 catalog written)

## Architecture Anchors (Recommended)

- `architecture/ss-06-skill-catalog.md#demo-recording-every-ac-has-user-observable-behavior-covered-+-visual-review` — TBD anchor

## Story Anchor (Recommended)

TBD — assigned by story-writer

## VP Anchors (Recommended)

- (TBD — to be assigned in Phase 1.6c)

---

### Brownfield-Specific Sections

#### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `plugins/vsdd-factory/skills/demo-recording/SKILL.md` |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |
| **Source line(s) within skill** | 266-298 |

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
#### BC-AUDIT-320 — demo-recording: every AC has user-observable behavior covered + visual review

**Skill:** `plugins/vsdd-factory/skills/demo-recording/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 266-298
**Trigger:** Quality gate
**Behavior:** Every AC with user-observable behavior has a recording. Recordings under 2MB/5MB. Full user journey demo for happy path. Evidence report links all recordings. PR description snippet generated. ffmpeg post-processed (optimized, trimmed, labeled). Visual reviewer (DF-018) analyzes all recordings and writes findings to `.factory/demo-evidence/visual-review.md`.
**Acceptance:** All 7 quality-gate items pass; visual-review.md exists.
```
