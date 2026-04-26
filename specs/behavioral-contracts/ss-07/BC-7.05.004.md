---
document_type: behavioral-contract
level: L3
version: "1.0"
status: draft
producer: product-owner
timestamp: 2026-04-25T00:00:00
phase: 1a
inputs: [.factory/stories/S-7.02-defensive-sweep-hook-meta-rule.md]
input-hash: ""
traces_to: .factory/stories/S-7.02-defensive-sweep-hook-meta-rule.md
origin: greenfield
extracted_from: ".factory/stories/S-7.02-defensive-sweep-hook-meta-rule.md#AC-003"
subsystem: "SS-07"
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
bc_id: BC-7.05.004
section: "7.05"
---

# BC-7.05.004: hooks-registry.toml registers validate-count-propagation.sh as PostToolUse on index file writes

## Description

`plugins/vsdd-factory/hooks-registry.toml` must contain a new entry that registers `validate-count-propagation.sh` as a PostToolUse hook triggered on Write and Edit operations to the key index files: `STATE.md`, `ARCH-INDEX.md`, `BC-INDEX.md`, `VP-INDEX.md`, `STORY-INDEX.md`, and `prd.md`. The entry must follow the existing hooks-registry.toml format and use the same tier as other lint/validation hooks.

## Preconditions

1. `validate-count-propagation.sh` exists at `plugins/vsdd-factory/hooks/validate-count-propagation.sh` (BC-7.05.001 satisfied).
2. `hooks-registry.toml` exists at `plugins/vsdd-factory/hooks-registry.toml`.
3. The existing hooks-registry.toml entry format has been read to determine the correct field names, trigger syntax, and tier value.

## Postconditions

1. `plugins/vsdd-factory/hooks-registry.toml` contains a new entry with:
   - Hook name: `validate-count-propagation` (or similar per existing naming convention)
   - Script path: `plugins/vsdd-factory/hooks/validate-count-propagation.sh`
   - Trigger: PostToolUse on Write/Edit operations
   - File scope: matches `STATE.md`, `ARCH-INDEX.md`, `BC-INDEX.md`, `VP-INDEX.md`, `STORY-INDEX.md`, `prd.md`, and `SS-NN-*.md` sharded architecture files
   - Tier: same as existing lint/validation hooks (inspect current entries; do not guess)
2. Running `cat plugins/vsdd-factory/hooks-registry.toml | grep validate-count-propagation` returns a non-empty result.
3. No existing hooks-registry.toml entry is modified or deleted; the new entry is purely additive.

## Invariants

1. The hooks-registry.toml entry format is identical to existing hook entries — no novel field names.
2. The trigger must be PostToolUse (not PreToolUse) — the hook checks state after writes, not before.
3. If a duplicate entry would result (same trigger path already registered), the implementer merges trigger conditions into the existing entry rather than creating a second entry.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | An existing entry already matches PostToolUse Write/Edit on a similar file pattern | Implementer merges the file scope into the existing entry. No duplicate entries. |
| EC-002 | hooks-registry.toml's format requires a `regex:` field for file matching rather than a list | The file scope is expressed as a regex (e.g., `(STATE\|ARCH-INDEX\|BC-INDEX\|VP-INDEX\|STORY-INDEX\|prd)\.md$\|SS-\d{2}-.*\.md$`) per the existing format convention. |
| EC-003 | The dispatcher does not load the new entry until the next activation | Normal. Hook takes effect on next hook dispatch cycle. No restart required unless the activation process explicitly requires it. |

## Canonical Test Vectors

| Input State | Expected Outcome | Category |
|-------------|-----------------|----------|
| `grep validate-count-propagation plugins/vsdd-factory/hooks-registry.toml` | Non-empty output with the new entry | happy-path |
| New entry's `trigger` field | `PostToolUse` with Write/Edit event type | happy-path |
| No pre-existing entry for validate-count-propagation | New entry created; existing entries unchanged | happy-path |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-062 | hooks-registry.toml contains the validate-count-propagation entry | static-check (grep assertion) |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-001 |
| Capability Anchor Justification | Anchored to CAP-001 ("Run a self-orchestrating LLM-driven SDLC pipeline") per capabilities.md §CAP-001 — hook registration in hooks-registry.toml is the configuration activation step that wires the count-propagation hook into the self-orchestrating pipeline's tool-call interception layer. |
| L2 Domain Invariants | none |
| Architecture Module | plugins/vsdd-factory/hooks-registry.toml |
| Stories | S-7.02 |
| Source AC | S-7.02 §AC-003 |
| FR | FR-042 |

## Related BCs

- BC-7.05.001 — depends on (hook must exist before it can be registered)

## Architecture Anchors

- `plugins/vsdd-factory/hooks-registry.toml` — hook routing table
- `plugins/vsdd-factory/hooks/validate-count-propagation.sh` — registered hook script

## Story Anchor

S-7.02

## VP Anchors

- VP-062 — hook registration coverage
