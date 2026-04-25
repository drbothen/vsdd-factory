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
extracted_from: ".factory/phase-0-ingestion/pass-3-deep-skills-batch-1.md#L1355"
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

# Behavioral Contract BC-6.05.018: discovery-engine: routing thresholds (auto-brief vs backlog vs registry vs urgent)

> Source: `pass-3-deep-skills-batch-1.md` line 1355 (was `BC-AUDIT-330`)
> Subsystem: SS-06 — Skill Catalog
> Section: Brownfield, discovery, research skills

## Description

discovery-engine: routing thresholds (auto-brief vs backlog vs registry vs urgent). Auto-brief generation: composite ≥0.7 AND evidence_strength ≥0.6 (human approves). Product Backlog: 0.5-0.7 OR evidence 0.4-0.6 (resurface next run). Discovery Registry: <0.5 AND evidence <0.4 (re-evaluate next run). Urgent Action: HIGH competitive urgency AND evidence_strength ≥0.7 → immediate human notification.

## Preconditions

1. After scoring

## Postconditions

1. Each idea routes to exactly one bucket per the threshold rules; auto-brief requires evidence ≥0.6 (not just composite).

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
| VP-001 | TBD — assertion derived from acceptance: "Each idea routes to exactly one bucket per the threshold rules; auto-brief requires evidence ≥0.6 (not just composite)." | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | TBD |
| L2 Domain Invariants | TBD |
| Architecture Module | plugins/vsdd-factory/skills/discovery-engine/SKILL.md |
| Stories | TBD |

## Related BCs (Recommended)

- TBD — link to sibling BCs in same skill (cross-reference once full SS-06 catalog written)

## Architecture Anchors (Recommended)

- `architecture/ss-06-skill-catalog.md#discovery-engine-routing-thresholds-(auto-brief-vs-backlog-vs-registry-vs-urgent)` — TBD anchor

## Story Anchor (Recommended)

TBD — assigned by story-writer

## VP Anchors (Recommended)

- VP-001 — TBD

---

### Brownfield-Specific Sections

#### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `plugins/vsdd-factory/skills/discovery-engine/SKILL.md` |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |
| **Source line(s) within skill** | 263-289 |

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
#### BC-AUDIT-330 — discovery-engine: routing thresholds (auto-brief vs backlog vs registry vs urgent)

**Skill:** `plugins/vsdd-factory/skills/discovery-engine/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 263-289
**Trigger:** After scoring
**Behavior:** Auto-brief generation: composite ≥0.7 AND evidence_strength ≥0.6 (human approves). Product Backlog: 0.5-0.7 OR evidence 0.4-0.6 (resurface next run). Discovery Registry: <0.5 AND evidence <0.4 (re-evaluate next run). Urgent Action: HIGH competitive urgency AND evidence_strength ≥0.7 → immediate human notification.
**Acceptance:** Each idea routes to exactly one bucket per the threshold rules; auto-brief requires evidence ≥0.6 (not just composite).
```
