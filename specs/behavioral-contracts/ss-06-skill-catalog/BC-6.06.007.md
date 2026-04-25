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
extracted_from: ".factory/phase-0-ingestion/pass-3-deep-skills-batch-1.md#L502"
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

# Behavioral Contract BC-6.06.007: check-state-health: stale-phase detection patterns

> Source: `pass-3-deep-skills-batch-1.md` line 502 (was `BC-AUDIT-245`)
> Subsystem: SS-06 — Skill Catalog
> Section: State and convergence skills

## Description

check-state-health: stale-phase detection patterns. Greps STATE.md for stale phase references and reports each with line number: `Phase 3.5` or `phase: 3.5` → 4; `Phase 4.*adversar` → 5; `Phase 5.*formal` or `Phase 5.*harden` → 6; `Phase 6.*converg` → 7.

## Preconditions

1. Check 4

## Postconditions

1. Stale-phase findings include exact line numbers; non-standard compound phases (e.g., `2-story-decomposition-patch-cycle`) flagged.

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
| VP-001 | TBD — assertion derived from acceptance: "Stale-phase findings include exact line numbers; non-standard compound phases (e.g., `2-story-decomposition-patch-cycle`) flagged." | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | TBD |
| L2 Domain Invariants | TBD |
| Architecture Module | plugins/vsdd-factory/skills/check-state-health/SKILL.md |
| Stories | TBD |

## Related BCs (Recommended)

- TBD — link to sibling BCs in same skill (cross-reference once full SS-06 catalog written)

## Architecture Anchors (Recommended)

- `architecture/ss-06-skill-catalog.md#check-state-health-stale-phase-detection-patterns` — TBD anchor

## Story Anchor (Recommended)

TBD — assigned by story-writer

## VP Anchors (Recommended)

- VP-001 — TBD

---

### Brownfield-Specific Sections

#### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `plugins/vsdd-factory/skills/check-state-health/SKILL.md` |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |
| **Source line(s) within skill** | 54-62 |

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
#### BC-AUDIT-245 — check-state-health: stale-phase detection patterns

**Skill:** `plugins/vsdd-factory/skills/check-state-health/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 54-62
**Trigger:** Check 4
**Behavior:** Greps STATE.md for stale phase references and reports each with line number: `Phase 3.5` or `phase: 3.5` → 4; `Phase 4.*adversar` → 5; `Phase 5.*formal` or `Phase 5.*harden` → 6; `Phase 6.*converg` → 7.
**Acceptance:** Stale-phase findings include exact line numbers; non-standard compound phases (e.g., `2-story-decomposition-patch-cycle`) flagged.
```
