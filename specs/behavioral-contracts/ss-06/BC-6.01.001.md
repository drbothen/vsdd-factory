---
document_type: behavioral-contract
level: L3
version: "1.1"
status: draft
producer: phase-1-4b-bc-extractor
timestamp: 2026-04-25T00:00:00
phase: 1.4b
inputs:
  - .factory/phase-0-ingestion/pass-3-behavioral-contracts.md
  - .factory/specs/behavioral-contracts/bc-id-mapping.md
input-hash: "1e73fa7"
traces_to: .factory/specs/architecture/ARCH-INDEX.md
origin: brownfield
extracted_from: ".factory/phase-0-ingestion/pass-3-behavioral-contracts.md#L466"
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

# Behavioral Contract BC-6.01.001: brownfield-ingest enforces strict-binary novelty

> Source: `pass-3-behavioral-contracts.md` line 466 (was `BC-AUDIT-070`)
> Subsystem: SS-06 — Skill Catalog
> Section: BC-6.01 — Skill quality-gate contracts (broad-sweep)

## Description

brownfield-ingest enforces strict-binary novelty. Round results must be exactly SUBSTANTIVE or NITPICK; soft phrases (`borderline`, `effectively`, `recommend halting`) are treated as SUBSTANTIVE; only the literal `NITPICK` token counts as convergence.

## Preconditions

1. Deepening round in progress.

## Postconditions

1. Round result is exactly SUBSTANTIVE or NITPICK; soft phrases (`borderline`, `effectively`, `recommend halting`) treated as SUBSTANTIVE; only literal `NITPICK` token counts as convergence.

## Invariants

1. TBD — derive from skill SKILL.md frontmatter and acceptance criteria.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Round emits soft phrase ("borderline NITPICK") | Treated as SUBSTANTIVE — another round required |
| EC-002 | Round emits literal `NITPICK` token | Convergence accepted |
| EC-003 | Round emits `SUBSTANTIVE` | Another round required |

## Canonical Test Vectors

> Golden-file test inputs and expected outputs.

| Input | Expected Output | Category |
|-------|----------------|----------|
| Round output contains literal `NITPICK` | Convergence declared | happy-path |
| Round output contains "borderline NITPICK" | Treated as SUBSTANTIVE | edge-case |
| Round output contains "effectively converged" | Treated as SUBSTANTIVE | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-001 | Only the literal token `NITPICK` (case-sensitive) counts as convergence; all soft / hedged phrases route to SUBSTANTIVE | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | TBD |
| L2 Domain Invariants | TBD |
| Architecture Module | plugins/vsdd-factory/skills/brownfield-ingest/SKILL.md |
| Stories | TBD |

## Related BCs (Recommended)

- BC-6.01.002 — brownfield-ingest "Iron Law" (companion convergence-honesty contract)

## Architecture Anchors (Recommended)

- `architecture/ss-06-skill-catalog.md#brownfield-ingest-strict-binary-novelty` — TBD anchor

## Story Anchor (Recommended)

TBD — assigned by story-writer

## VP Anchors (Recommended)

- VP-001 — Only the literal token `NITPICK` (case-sensitive) counts as convergence

---

### Brownfield-Specific Sections

#### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `plugins/vsdd-factory/skills/brownfield-ingest/SKILL.md` |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |
| **Source Line** | 466 |
| **Audit ID** | BC-AUDIT-070 |
| **Evidence (verbatim)** | `skills/brownfield-ingest/SKILL.md` Strict-binary enforcement section. |
| **Confidence (verbatim)** | HIGH. |

#### Evidence Types Used

- documentation: stated in SKILL.md prose
- guard clause: explicit token-match enforcement described in skill body

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | reads only (token-match against round output text) |
| **Global state access** | none |
| **Deterministic** | yes — token equality check |
| **Thread safety** | not applicable (advisory contract on convergence reporting) |
| **Overall classification** | pure (declarative gating rule) |

#### Refactoring Notes

Token-match gating rule — verifiable by static lint of round-output files. Suitable for automated enforcement via a validate-novelty-assessment-style hook.

#### Source Excerpt (verbatim)

```text
### BC-AUDIT-070: brownfield-ingest enforces strict-binary novelty
- **Preconditions:** Deepening round in progress.
- **Postconditions:** Round result is exactly SUBSTANTIVE or NITPICK; soft phrases (`borderline`, `effectively`, `recommend halting`) treated as SUBSTANTIVE; only literal `NITPICK` token counts as convergence.
- **Evidence:** `skills/brownfield-ingest/SKILL.md` Strict-binary enforcement section.
- **Confidence:** HIGH.
```
