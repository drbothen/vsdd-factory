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
bc_id: BC-5.36.001
section: "5.36"
---

# BC-5.36.001: story-writer agent rejects status=ready when behavioral_contracts is empty

## Description

The story-writer agent prompt must contain a Spec-First Gate rule that prevents a story's `status:` field from being set to `ready` while `behavioral_contracts: []` is empty or absent. The gate applies both at initial story authoring and at any status-transition edit. The story file must include a frontmatter comment acknowledging the pending state.

## Preconditions

1. A story file is being authored or its `status:` field is being updated.
2. The story's `behavioral_contracts:` field is either absent, `null`, or an empty array (`[]`).
3. The story-writer agent is the active agent performing the operation.

## Postconditions

1. The story's `status:` field is set to `draft`, not `ready`. The `behavioral_contracts:` field is non-empty AND every entry matches the canonical BC pattern `BC-\d+\.\d{2}\.\d{3}` (i.e., not a `BC-TBD` placeholder, not malformed). Empty array, missing field, or any non-canonical entry blocks the `draft → ready` status transition.
2. The story file contains an inline note such as `# BC status: pending PO authorship` (or equivalent) adjacent to the `behavioral_contracts: []` field.
3. No PR or delivery is marked ready for a story with an unpopulated `behavioral_contracts:` array.

## Invariants

1. The `draft → ready` transition is impossible while `behavioral_contracts: []` OR while any entry fails to match `BC-\d+\.\d{2}\.\d{3}` (e.g., `BC-TBD`, malformed IDs, or bare placeholder text are all blocking).
2. The gate applies to both new story authorship and retrospective status edits.
3. The Spec-First Gate rule text is present in the story-writer agent prompt's Constraints or Rules section.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Story-writer authors a story during decomposition before PO has been dispatched | Story is created with `status: draft` and `behavioral_contracts: []` — this is expected and valid. Gate only blocks transition to `ready`. |
| EC-002 | Story-writer attempts to set `status: ready` on a partially-filled `behavioral_contracts: [BC-TBD]` where all values are placeholders | Placeholder BC-TBD values do not satisfy the gate. The array must contain at least one real BC-NNN ID. |
| EC-003 | Agent prompt is loaded from a cached version predating this story's merge | Prompt changes take effect on next agent dispatch only. In-flight sessions complete with old rules. No rollback needed. |

## Canonical Test Vectors

| Input State | Expected Output | Category |
|-------------|----------------|----------|
| Story with `behavioral_contracts: []`, story-writer sets `status: ready` | `status` remains `draft`; inline BC-status note added | negative (gate fires) |
| Story with `behavioral_contracts: [BC-5.36.001]`, story-writer sets `status: ready` | `status: ready` allowed; note not required | happy-path |
| Story with `behavioral_contracts: [BC-TBD]`, story-writer sets `status: ready` | `status` remains `draft`; placeholder BCs do not satisfy gate | negative (gate fires) |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-061 | Spec-First Gate rule text is present in story-writer.md Constraints section | static-check (peer review) |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-001 |
| Capability Anchor Justification | Anchored to CAP-001 ("Run a self-orchestrating LLM-driven SDLC pipeline") per capabilities.md §CAP-001 — this BC governs the story-writer agent, which is a core pipeline agent executing the SDLC workflow defined by CAP-001. |
| L2 Domain Invariants | none |
| Architecture Module | plugins/vsdd-factory/agents/story-writer.md |
| Stories | S-7.01 |
| Source AC | S-7.01 §AC-001 |
| FR | FR-042 |

## Related BCs

- BC-5.36.002 — sibling (story-writer AC↔BC bidirectional trace gate, composes with this BC's spec-first gate)
- BC-5.36.007 — sibling (agent prompt versioning requirement; same delivery atomicity)

## Architecture Anchors

- `plugins/vsdd-factory/agents/story-writer.md` — agent prompt file to be modified

## Story Anchor

S-7.01

## VP Anchors

- VP-061 — Spec-First Gate presence check
