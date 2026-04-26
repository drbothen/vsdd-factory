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
extracted_from: ".factory/stories/S-7.01-agent-prompt-discipline.md#AC-002"
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
bc_id: BC-5.36.004
section: "5.36"
---

# BC-5.36.004: product-owner cites capabilities.md verbatim in every capability anchor justification

## Description

The product-owner agent prompt must require that the Capability Anchor Justification row in every BC's Traceability table includes a verbatim quote of the capability title from `capabilities.md` along with the file section reference. "Verbatim" means the exact title string as it appears in capabilities.md — paraphrased or abbreviated versions are insufficient.

## Preconditions

1. The product-owner agent is authoring a BC's Traceability table.
2. A CAP-NNN has been chosen as the capability anchor.
3. `specs/domain-spec/capabilities.md` is readable and contains the CAP-NNN entry.

## Postconditions

1. The Capability Anchor Justification cell contains the exact capability title in quotes — e.g., `CAP-001 ("Run a self-orchestrating LLM-driven SDLC pipeline") per capabilities.md §CAP-001`.
2. The justification sentence states WHY this CAP is the correct anchor (one sentence).
3. The capability title in the justification cell is a character-for-character match to the title in capabilities.md for that CAP-NNN.

## Invariants

1. The product-owner prompt specifies that paraphrased capability titles fail the requirement.
2. The product-owner prompt cites an example of a valid verbatim justification.
3. Every BC authored under this requirement is auditable by diffing the justification text against capabilities.md.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Capability title in capabilities.md is updated after a BC was authored | The BC's justification becomes stale but is not retroactively invalid. On the next BC revision, the PO updates the verbatim quote to match the new title. |
| EC-002 | Two capabilities both partially describe the BC's scope | PO chooses the most specific matching CAP and explains the choice. The justification sentence must name the rejected CAP and explain why the chosen one is superior. |
| EC-003 | PO quotes the capability description rather than the title | Fails. The title (the text after `—` on the CAP-NNN line) must appear verbatim in the justification. Quoting only the description block without the title is insufficient. |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| `Anchored to CAP-001 ("Run a self-orchestrating LLM-driven SDLC pipeline") per capabilities.md §CAP-001` — exact title match | Passes | happy-path |
| `Anchored to CAP-001 ("Self-orchestrating pipeline") per capabilities.md` — abbreviated title | Fails; adversary flags MEDIUM | negative |
| `CAP-001 per capabilities.md` — no quoted title | Fails; adversary flags MEDIUM | negative |
| `Anchored to CAP-001 ("Run a self-orchestrating LLM-driven SDLC pipeline") per capabilities.md §CAP-001 — because this BC governs story-writer agent behavior` | Passes (includes justification sentence) | happy-path |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-061 | Verbatim citation requirement is present in product-owner.md BC authoring section | static-check (peer review) |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-001 |
| Capability Anchor Justification | Anchored to CAP-001 ("Run a self-orchestrating LLM-driven SDLC pipeline") per capabilities.md §CAP-001 — this BC specifies verbatim citation discipline for the product-owner step of the SDLC pipeline that CAP-001 describes. |
| L2 Domain Invariants | none |
| Architecture Module | plugins/vsdd-factory/agents/product-owner.md |
| Stories | S-7.01 |
| Source AC | S-7.01 §AC-002 |
| FR | FR-042 |

## Related BCs

- BC-5.36.003 — depends on (presence requirement is the precondition for verbatim requirement)
- BC-5.36.005 — sibling (adversary audit catches violations of this BC)

## Architecture Anchors

- `plugins/vsdd-factory/agents/product-owner.md` — agent prompt file
- `plugins/vsdd-factory/agents/adversary.md` — enforcement via adversary policy 5

## Story Anchor

S-7.01

## VP Anchors

- VP-061 — verbatim citation requirement presence
