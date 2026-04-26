---
document_type: behavioral-contract
level: L3
version: "1.0"
status: draft
producer: product-owner
timestamp: 2026-04-25T00:00:00
phase: 1a
inputs: [.factory/stories/S-7.01-agent-prompt-discipline.md]
input-hash: ""
traces_to: .factory/stories/S-7.01-agent-prompt-discipline.md
origin: greenfield
extracted_from: ".factory/stories/S-7.01-agent-prompt-discipline.md#AC-001"
subsystem: "SS-05"
capability: "CAP-001"
lifecycle_status: active
introduced: v1.0-brownfield-backfill
modified: []
deprecated: null
deprecated_by: null
replacement: null
retired: null
removed: null
removal_reason: null
bc_id: BC-5.36.002
section: "5.36"
---

# BC-5.36.002: story-writer requires AC↔BC bidirectional traces before marking a story ready

## Description

Before a story can be transitioned to `status: ready`, the story-writer must verify that every Acceptance Criterion (AC-NNN) in the story body is referenced by at least one entry in `behavioral_contracts:`, and every BC listed in `behavioral_contracts:` traces back to at least one AC in the story's Behavioral Contracts table. This bidirectional check is part of the Spec-First Gate.

## Preconditions

1. The story has at least one AC defined in its Acceptance Criteria section.
2. The story-writer is attempting to set `status: ready`.
3. The story's `behavioral_contracts:` array is non-empty (gate in BC-5.36.001 has already been passed).

## Postconditions

1. Every AC-NNN in the story body has at least one BC-NNN in the Behavioral Contracts table row for that AC.
2. Every BC-NNN in `behavioral_contracts:` frontmatter appears in the story body's Behavioral Contracts table with a "Covering AC" column entry.
3. If any AC has no BC or any BC has no AC, `status` remains `draft`.

## Invariants

1. The Behavioral Contracts table in the story body is the join table for AC↔BC bidirectional coverage.
2. A BC listed in `behavioral_contracts:` with no corresponding table row is a traceability gap — it fails the gate.
3. An AC with `(traces to BC-TBD)` in its description does not satisfy the gate.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Story has 5 ACs but Behavioral Contracts table only has 4 rows (one AC missed) | Gate fires; story remains `draft`; story-writer notes the missing AC in a comment |
| EC-002 | BC-NNN is in frontmatter array but the Behavioral Contracts table row lists it as BC-TBD | Does not satisfy gate. Placeholder BC-TBD is treated as an empty reference. |
| EC-003 | Multiple BCs cover one AC (acceptable fan-out) | Valid; gate passes for that AC. Multiple rows for one AC are permitted. |
| EC-004 | One BC covers multiple ACs (acceptable fan-in) | Valid; the BC must appear in each AC's row. Gate passes if all ACs have at least one BC. |

## Canonical Test Vectors

| Input State | Expected Output | Category |
|-------------|----------------|----------|
| 3 ACs, all with BC-NNN in table, all BCs in frontmatter array | `status: ready` allowed | happy-path |
| 3 ACs, AC-002 row missing from table, 2 BCs in frontmatter | `status` remains `draft`; AC-002 gap noted | negative |
| 3 ACs, frontmatter has BC-A, BC-B, BC-C; table only references BC-A, BC-B | `status` remains `draft`; BC-C orphan noted | negative |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-061 | Bidirectional AC↔BC trace rule is present in story-writer.md Spec-First Gate | static-check (peer review) |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-001 |
| Capability Anchor Justification | Anchored to CAP-001 ("Run a self-orchestrating LLM-driven SDLC pipeline") per capabilities.md §CAP-001 — this BC governs the story-writer agent's spec-first discipline, which is part of the pipeline self-governance loop in CAP-001. |
| L2 Domain Invariants | none |
| Architecture Module | plugins/vsdd-factory/agents/story-writer.md |
| Stories | S-7.01 |
| Source AC | S-7.01 §AC-001 |
| FR | FR-042 |

## Related BCs

- BC-5.36.001 — depends on (this gate fires after BC-5.36.001's empty-array check passes)
- BC-5.36.007 — sibling (agent prompt update atomicity)

## Architecture Anchors

- `plugins/vsdd-factory/agents/story-writer.md` — agent prompt file

## Story Anchor

S-7.01

## VP Anchors

- VP-061 — Spec-First Gate presence check
