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
extracted_from: ".factory/stories/S-7.01-agent-prompt-discipline.md#AC-004"
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
bc_id: BC-5.36.007
section: "5.36"
---

# BC-5.36.007: all three agent prompts updated atomically in single delivery; no partial update

## Description

The delivery of S-7.01 changes to story-writer.md, product-owner.md, and adversary.md must be atomic: all three files must appear in the same PR diff. A delivery that modifies two of the three files but omits the third fails this story's definition of done. Additionally, the changes to each agent prompt must be purely additive — no existing rules, policies, or workflow steps may be removed or semantically weakened.

## Preconditions

1. The implementer is delivering S-7.01.
2. All three agent prompt files exist at their canonical paths under `plugins/vsdd-factory/agents/`.
3. The delivery is in progress (PR open or commit being composed).

## Postconditions

1. The delivery diff shows modifications to all three files: `story-writer.md`, `product-owner.md`, and `adversary.md`.
2. No existing policy section, constraint, or rule in any of the three files has been deleted or weakened.
3. All additions are in the correct sections: story-writer Constraints/Rules, product-owner BC authoring section, adversary Spec Review section.

## Invariants

1. A partial delivery (1 or 2 of 3 files modified) must not be merged. The story remains `draft` until all three are delivered.
2. Additions are purely additive. The implementer verifies by diffing each file: zero deletions in policy sections.
3. The agent prompt files remain pure Markdown instruction documents — no executable code, no BC-NNN ID embeds that don't exist.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Implementer ships story-writer and adversary updates but forgets product-owner | AC-004 fails. PR reviewer blocks merge. Implementer adds product-owner changes in the same PR before merge. |
| EC-002 | validate-template-compliance.sh is not configured for agent prompt .md files | Peer review is the enforcement mechanism. Implementer notes in PR description that compliance script does not cover agents/. |
| EC-003 | One agent file requires a structural refactor to insert the new section cleanly | Acceptable, but the refactor must be purely additive. If an existing section must be renamed to accommodate the addition, both the old and new section names are preserved (old with a deprecation note, new with the content). |

## Canonical Test Vectors

| Input State | Expected Output | Category |
|-------------|----------------|----------|
| Diff shows story-writer.md, product-owner.md, adversary.md all modified | AC-004 passes | happy-path |
| Diff shows story-writer.md and adversary.md modified; product-owner.md absent | AC-004 fails; story remains `draft` | negative |
| Diff shows story-writer.md modified with a deletion in the Rules section | AC-005 fails; additive-only violated | negative |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-061 | All three agent prompt files updated in same delivery | static-check (diff review) |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-001 |
| Capability Anchor Justification | Anchored to CAP-001 ("Run a self-orchestrating LLM-driven SDLC pipeline") per capabilities.md §CAP-001 — the atomic update of all three pipeline agents (story-writer, product-owner, adversary) ensures consistent pipeline behavior, which is a property of the self-orchestrating SDLC described by CAP-001. |
| L2 Domain Invariants | none |
| Architecture Module | plugins/vsdd-factory/agents/ |
| Stories | S-7.01 |
| Source AC | S-7.01 §AC-004, §AC-005 |
| FR | FR-042 |

## Related BCs

- BC-5.36.001 — sibling (spec-first gate; same delivery)
- BC-5.36.003 — sibling (product-owner anchor requirement; same delivery)
- BC-5.36.005 — sibling (adversary regression axis; same delivery)

## Architecture Anchors

- `plugins/vsdd-factory/agents/story-writer.md`
- `plugins/vsdd-factory/agents/product-owner.md`
- `plugins/vsdd-factory/agents/adversary.md`

## Story Anchor

S-7.01

## VP Anchors

- VP-061 — atomic delivery check
