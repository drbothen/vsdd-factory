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
extracted_from: ".factory/phase-0-ingestion/pass-3-behavioral-contracts.md#L472"
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

# Behavioral Contract BC-6.01.002: brownfield-ingest "Iron Law" — no round completion without honest convergence check

> Source: `pass-3-behavioral-contracts.md` line 472 (was `BC-AUDIT-071`)
> Subsystem: SS-06 — Skill Catalog
> Section: BC-6.01 — Skill quality-gate contracts (broad-sweep)

## Description

brownfield-ingest's "Iron Law": padding or fabricating round content is strictly worse than declaring NITPICK without emitting a file. No round may complete without an honest convergence check.

## Preconditions

1. Round in progress.

## Postconditions

1. Padding/fabrication strictly worse than declaring NITPICK without emitting a file.

## Invariants

1. TBD — derive from skill SKILL.md frontmatter and acceptance criteria.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Agent has nothing substantive to add | Declare NITPICK without emitting a deepening file |
| EC-002 | Agent fabricates filler to justify a SUBSTANTIVE label | Worse outcome than EC-001; violates Iron Law |
| EC-003 | Agent emits a deepening file with genuine new findings | SUBSTANTIVE label is honest and acceptable |

## Canonical Test Vectors

> Golden-file test inputs and expected outputs.

| Input | Expected Output | Category |
|-------|----------------|----------|
| No new substantive findings; agent declares NITPICK without file | Iron Law upheld | happy-path |
| No new findings; agent pads file to claim SUBSTANTIVE | Iron Law violated — convergence integrity compromised | error |
| Genuine new findings; agent emits deepening file | Iron Law upheld | edge-case |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-001 | Padding/fabrication is strictly worse than honest NITPICK without file emission | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | TBD |
| L2 Domain Invariants | TBD |
| Architecture Module | plugins/vsdd-factory/skills/brownfield-ingest/SKILL.md |
| Stories | TBD |

## Related BCs (Recommended)

- BC-6.01.001 — brownfield-ingest strict-binary novelty (companion gating contract)

## Architecture Anchors (Recommended)

- `architecture/ss-06-skill-catalog.md#brownfield-ingest-iron-law` — TBD anchor

## Story Anchor (Recommended)

TBD — assigned by story-writer

## VP Anchors (Recommended)

- VP-001 — Honest convergence check required

---

### Brownfield-Specific Sections

#### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `plugins/vsdd-factory/skills/brownfield-ingest/SKILL.md` |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |
| **Source Line** | 472 |
| **Audit ID** | BC-AUDIT-071 |
| **Evidence (verbatim)** | `skills/brownfield-ingest/SKILL.md`. |
| **Confidence (verbatim)** | HIGH. |

#### Evidence Types Used

- documentation: stated in SKILL.md prose

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | not applicable (declarative rule of agent conduct) |
| **Global state access** | not applicable |
| **Deterministic** | yes — rule is invariant |
| **Thread safety** | not applicable |
| **Overall classification** | pure (declarative behavioral rule) |

#### Refactoring Notes

Conduct rule for agent self-discipline. Hard to mechanically enforce; partially observable via downstream validators that detect padded outputs.

#### Source Excerpt (verbatim)

```text
### BC-AUDIT-071: brownfield-ingest "Iron Law" — no round completion without honest convergence check
- **Preconditions:** Round in progress.
- **Postconditions:** Padding/fabrication strictly worse than declaring NITPICK without emitting a file.
- **Evidence:** `skills/brownfield-ingest/SKILL.md`.
- **Confidence:** HIGH.
```
