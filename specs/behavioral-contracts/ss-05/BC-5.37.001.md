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
extracted_from: ".factory/stories/S-7.02-defensive-sweep-hook-meta-rule.md#AC-001"
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
bc_id: BC-5.37.001
section: "5.37"
---

# BC-5.37.001: state-manager runs corpus-wide grep before declaring count change complete

## Description

The state-manager agent prompt must include a Defensive Propagation Sweep protocol: before declaring any count-changing update complete (e.g., "BC count is now 1,875"), the state-manager must run a corpus-wide grep over a defined file list to identify all files still containing the old count as a literal string. Only after all occurrences are updated may the state-manager declare the count change complete.

## Preconditions

1. The state-manager is performing an update that changes a count (BC count, VP count, story count, rule file count, etc.).
2. The state-manager agent prompt contains the Defensive Propagation Sweep protocol in its STATE.md Update Protocol section.
3. The new count differs from the previous count by any positive integer.

## Postconditions

1. Before declaring the count change complete, the state-manager has run a corpus-wide grep for the old count value in the contexts: "NNN BCs", "NNN VPs", "NNN stories", "Total.*NNN".
2. The sweep covers at minimum: `STATE.md`, `ARCH-INDEX.md`, `BC-INDEX.md`, `VP-INDEX.md`, `STORY-INDEX.md`, all `SS-NN-*.md` sharded architecture files, and `prd.md`.
3. Any file still containing the old count after the primary update is identified and updated before the state-manager reports completion.

## Invariants

1. The Defensive Propagation Sweep is mandatory, not optional — the protocol uses "MUST", not "SHOULD".
2. The grep uses anchored context patterns (not bare number grep) to avoid false matches on dates, IDs, or changelog entries.
3. The state-manager prompt's file list is the minimum scope; additional files found by grep are also updated.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Old count appears in a changelog entry as a historical note | Anchored grep pattern (e.g., "NNN BCs") avoids matching bare numbers. Changelog entries like "previously had 1,863 BCs" are expected to match and should be updated to reflect the new count, or the state-manager notes them as intentionally historical. |
| EC-002 | Corpus grep produces no matches for the old count in sibling files | Normal outcome for first count change after a previous sweep. State-manager declares completion after confirming zero matches. |
| EC-003 | State-manager updates counts across multiple files in a wave-gate burst; intermediate states have inconsistent counts | The sweep is run at the END of the burst, not after each individual file update. The state-manager must account for all in-flight changes before running the final sweep. |

## Canonical Test Vectors

| Input State | Expected Behavior | Category |
|-------------|-------------------|----------|
| STATE.md says "1,875 BCs"; ARCH-INDEX.md says "1,863 BCs"; state-manager updates STATE.md to 1,876 | Sweep finds ARCH-INDEX.md still has old count; state-manager updates it before completing | happy-path (sweep catches gap) |
| STATE.md says "1,875 BCs"; all sibling files already say "1,875 BCs"; count updates to 1,876 | Sweep finds all sibling files need update; state-manager updates all | happy-path |
| Count in a rule comment "# 42 rules as of 2026-01" in a Bash script | Context-anchored grep does not match; state-manager skips this false-positive | edge-case |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-062 | Count propagation is consistent across all index files after any state-manager count change | integration (validate-count-propagation.sh run post-burst) |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-001 |
| Capability Anchor Justification | Anchored to CAP-001 ("Run a self-orchestrating LLM-driven SDLC pipeline") per capabilities.md §CAP-001 — the state-manager is the pipeline's state-keeping agent; its count propagation discipline is a correctness property of the self-orchestrating SDLC pipeline. |
| L2 Domain Invariants | none |
| Architecture Module | plugins/vsdd-factory/agents/state-manager.md |
| Stories | S-7.02 |
| Source AC | S-7.02 §AC-001 |
| FR | FR-042 |

## Related BCs

- BC-5.37.002 — sibling (sweep result logging; composes with this BC)
- BC-7.05.001 — sibling (validate-count-propagation.sh hook automates what this BC requires of state-manager)

## Architecture Anchors

- `plugins/vsdd-factory/agents/state-manager.md` — agent prompt file to be modified

## Story Anchor

S-7.02

## VP Anchors

- VP-062 — count propagation consistency invariant
