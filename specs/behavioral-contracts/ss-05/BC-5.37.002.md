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
bc_id: BC-5.37.002
section: "5.37"
---

# BC-5.37.002: state-manager logs sweep results before declaring count-change complete

## Description

After running the Defensive Propagation Sweep (BC-5.37.001), the state-manager must log the sweep results as part of its completion message. The log must name the files checked, the old count pattern searched, and either "all files updated" or list any remaining occurrences. This provides an audit trail for the sweep and allows post-hoc verification that the sweep was actually run.

## Preconditions

1. BC-5.37.001 preconditions are satisfied (state-manager is completing a count-changing update).
2. The corpus-wide grep has been run.
3. All identified files have been updated (or a justified exception noted).

## Postconditions

1. The state-manager's completion message includes a "Propagation Sweep Result" section or inline notation.
2. The notation includes: files checked, count pattern searched, and outcome (all consistent / N files updated / N exceptions noted).
3. The notation is visible in the conversation context so the adversary and orchestrator can audit it.

## Invariants

1. A count-change completion declaration without a sweep result log is invalid. The log is mandatory.
2. The log format need not be formal (no specific schema required) but must be human-auditable.
3. Exception cases (files intentionally not updated) must include a justification, not a silent skip.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | State-manager updates 15 files in one burst; sweep log would be very long | Summary format acceptable: "Sweep checked 7 index files; 5 required count update; all updated." Individual file list not required if count is high. |
| EC-002 | Sweep finds a file in a directory outside the standard list (e.g., a holdout scenario file) | State-manager logs it as an "extra match" and either updates it or explains why the holdout file intentionally preserves the old count for historical test purposes. |

## Canonical Test Vectors

| Input State | Expected Output | Category |
|-------------|----------------|----------|
| Count updated in STATE.md; sweep ran; 3 sibling files updated | Completion message: "Propagation sweep complete: STATE.md updated; ARCH-INDEX.md, BC-INDEX.md, prd.md also updated. All consistent." | happy-path |
| Count updated; sweep ran; no sibling files needed update | Completion message: "Propagation sweep complete: no sibling files required update. All consistent." | happy-path |
| Count updated; sweep ran; one exception (holdout file intentionally old) | Completion message: "Propagation sweep: N files updated. Exception: holdout-scenario-2024.md intentionally preserves old count for test reproducibility." | edge-case |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-062 | Sweep result logging requirement present in state-manager.md protocol | static-check (peer review) |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-001 |
| Capability Anchor Justification | Anchored to CAP-001 ("Run a self-orchestrating LLM-driven SDLC pipeline") per capabilities.md §CAP-001 — sweep result logging is an audit property of the state-manager step in the self-orchestrating SDLC pipeline. |
| L2 Domain Invariants | none |
| Architecture Module | plugins/vsdd-factory/agents/state-manager.md |
| Stories | S-7.02 |
| Source AC | S-7.02 §AC-001 |
| FR | FR-042 |

## Related BCs

- BC-5.37.001 — depends on (logging presupposes the sweep has run)

## Architecture Anchors

- `plugins/vsdd-factory/agents/state-manager.md` — agent prompt file

## Story Anchor

S-7.02

## VP Anchors

- VP-062 — count propagation consistency (sweep logging is part of the evidence)
