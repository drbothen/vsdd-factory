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
extracted_from: ".factory/stories/S-7.02-defensive-sweep-hook-meta-rule.md#AC-004"
subsystem: "SS-08"
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
bc_id: BC-8.28.001
section: "8.28"
---

# BC-8.28.001: rules/lessons-codification.md requires codification follow-up for every novel process catch

## Description

A new rule file `plugins/vsdd-factory/rules/lessons-codification.md` must be created defining the meta-rule: "every novel adversary process catch triggers a codification follow-up before cycle closure." The rule must define "novel process catch" using the `[process-gap]` tag, specify the three-step required follow-up protocol, and name the enforcement mechanism (orchestrator cycle-closing checklist).

## Preconditions

1. The file `plugins/vsdd-factory/rules/lessons-codification.md` does not yet exist.
2. All 9 existing rule files in `plugins/vsdd-factory/rules/` have been reviewed for conflicts.
3. No existing rule covers this same obligation.

## Postconditions

1. The file `plugins/vsdd-factory/rules/lessons-codification.md` exists with the following required content:
   - Rule statement: "Every novel adversary process catch → codification follow-up before cycle closure."
   - `[process-gap]` tag definition: a finding identifying a gap in an agent prompt, hook, rule file, or pipeline workflow (not a gap in a specific spec artifact).
   - Three-step Required Follow-up protocol: (1) scan final convergence report for `[process-gap]` findings, (2) for each finding, open a follow-up story or record a justified deferral, (3) cycle not CLOSED until either exists.
   - Enforcement: orchestrator references this rule during cycle-closure checklist; adversary uses `[process-gap]` tag when categorizing findings.
2. The ARCH-INDEX SS-08 rule file count is updated from 9 to 10 (this addition increments the count — tracked in EC-006 of story S-7.02).

## Invariants

1. The `[process-gap]` tag is defined precisely to distinguish process findings from artifact findings.
2. The cycle-closure obligation is absolute: no "CLOSED" status without either a story or a justified deferral.
3. The rule file must not embed BC-NNN IDs or VP-NNN IDs that reference external artifacts. It is a governance document.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | An existing rule file in rules/ partially overlaps (e.g., a "findings-handling" rule) | Cross-reference note added to both files. The new rule is more specific (process findings only); the overlap is noted explicitly. |
| EC-002 | The orchestrator closes a sub-cycle without reading the rule file | The adversary's updated prompt references this rule when tagging `[process-gap]` findings, creating a prompt-level enforcement backstop. |
| EC-003 | A `[process-gap]` finding is tagged by the adversary but the orchestrator deems it too minor to warrant a story | A "justified deferral" entry in the Drift Items table satisfies the rule. The rule does not require a story — it requires EITHER a story OR a documented deferral. |

## Canonical Test Vectors

| Input State | Expected Result | Category |
|-------------|----------------|----------|
| Adversary pass produces finding: `[process-gap] state-manager missing defensive sweep` | Orchestrator must open story or log deferral before closing cycle | rule fires |
| Adversary pass produces finding: `[artifact-gap] BC-5.01.001 missing edge case` | Rule does not apply; this is an artifact gap, not a process gap | rule does not fire |
| Cycle-closure checklist run; 1 `[process-gap]` finding has neither story nor deferral | Cycle remains OPEN | negative |
| Cycle-closure checklist run; 1 `[process-gap]` finding has justified deferral in Drift Items | Cycle may be CLOSED | happy-path |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-062 | Rule file exists at correct path with required content sections | static-check (file existence + content grep) |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-001 |
| Capability Anchor Justification | Anchored to CAP-001 ("Run a self-orchestrating LLM-driven SDLC pipeline") per capabilities.md §CAP-001 — this rule codifies the self-improvement loop of the pipeline, ensuring that process gaps discovered during adversarial review are systematically closed rather than re-discovered. This is a meta-property of the self-orchestrating SDLC. |
| L2 Domain Invariants | none |
| Architecture Module | plugins/vsdd-factory/rules/lessons-codification.md |
| Stories | S-7.02 |
| Source AC | S-7.02 §AC-004 |
| FR | FR-042 |

## Related BCs

- BC-8.28.002 — sibling (orchestrator cycle-closing checklist references this rule; composes with)
- BC-5.36.005 — sibling (adversary partial-fix-regression check that produces `[process-gap]` findings this rule consumes)

## Architecture Anchors

- `plugins/vsdd-factory/rules/lessons-codification.md` — rule file to be created
- `.factory/specs/architecture/ARCH-INDEX.md` — SS-08 rule file count must be updated to 10

## Story Anchor

S-7.02

## VP Anchors

- VP-062 — rule file existence and content
