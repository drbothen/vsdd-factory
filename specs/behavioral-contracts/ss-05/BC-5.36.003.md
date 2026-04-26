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
bc_id: BC-5.36.003
section: "5.36"
---

# BC-5.36.003: product-owner agent requires Capability Anchor Justification cell on every BC

## Description

The product-owner agent prompt must contain a Capability Anchor Justification requirement mandating that every behavioral contract includes a "Capability Anchor Justification" row in its Traceability section. A bare `CAP-NNN` with no verbatim citation from `capabilities.md` is explicitly declared insufficient by the prompt. This requirement is enforced at authorship time.

## Preconditions

1. The product-owner agent is authoring or reviewing a behavioral contract file.
2. The BC has a Traceability section.
3. The product-owner agent prompt contains the Capability Anchor Justification requirement in its BC authoring section.

## Postconditions

1. Every BC authored by the product-owner has a "Capability Anchor Justification" row in its Traceability table.
2. The justification row cites the specific capability name and file location verbatim — e.g., `CAP-017 ("Create and manage formal ADR records") per capabilities.md §CAP-017`.
3. No BC is filed with only `CAP-NNN` as its capability anchor (bare ID without title and file reference).

## Invariants

1. The Capability Anchor Justification requirement text is present in the product-owner.md BC authoring section.
2. The adversary policy for treating missing justification as MEDIUM-severity is referenced in the product-owner prompt (cross-reference to adversary policy 5).
3. A BC with `Capability Anchor Justification: none` or an empty value fails the requirement.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | No existing capability in capabilities.md fits the BC semantically | PO must propose a new CAP with justification rather than force-fit a poor anchor. The justification cell says "Proposing CAP-NNN: <title> — see discussion" with rationale. |
| EC-002 | BC is authored before capabilities.md is available (very early pipeline phase) | Use `CAP-TBD` with a note explaining why it is deferred; adversary treats this as a LOW finding until resolved. |
| EC-003 | Model ignores the justification requirement in a particular invocation | Adversary updated-policy-5 axis catches the omission in the next review pass (defense-in-depth). |

## Canonical Test Vectors

| Input State | Expected Output | Category |
|-------------|----------------|----------|
| BC with `Capability Anchor Justification: Anchored to CAP-001 ("Run...") per capabilities.md §CAP-001` | Passes requirement | happy-path |
| BC with `Capability Anchor Justification: CAP-001` (bare ID) | Fails requirement — adversary flags as MEDIUM | negative |
| BC with Traceability table missing the row entirely | Fails requirement — adversary flags as MEDIUM | negative |
| BC with `Capability Anchor Justification: CAP-TBD — no matching capability yet` | Passes (deferred) — adversary notes as LOW | edge-case |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-061 | Capability Anchor Justification requirement is present in product-owner.md BC authoring section | static-check (peer review) |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-001 |
| Capability Anchor Justification | Anchored to CAP-001 ("Run a self-orchestrating LLM-driven SDLC pipeline") per capabilities.md §CAP-001 — this BC governs the product-owner agent's BC authoring behavior, which is a step in the self-orchestrating SDLC pipeline described by CAP-001. |
| L2 Domain Invariants | none |
| Architecture Module | plugins/vsdd-factory/agents/product-owner.md |
| Stories | S-7.01 |
| Source AC | S-7.01 §AC-002 |
| FR | FR-042 |

## Related BCs

- BC-5.36.004 — sibling (verbatim citation requirement; composes with this BC's presence requirement)
- BC-5.36.005 — sibling (adversary partial-fix check that enforces this BC's audit)

## Architecture Anchors

- `plugins/vsdd-factory/agents/product-owner.md` — agent prompt file to be modified

## Story Anchor

S-7.01

## VP Anchors

- VP-061 — presence of Capability Anchor Justification requirement
