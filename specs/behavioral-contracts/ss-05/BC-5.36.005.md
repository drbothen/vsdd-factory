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
extracted_from: ".factory/stories/S-7.01-agent-prompt-discipline.md#AC-003"
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
bc_id: BC-5.36.005
section: "5.36"
---

# BC-5.36.005: adversary explicitly checks partial-fix-regression for every finding closed in a prior pass

## Description

The adversary agent prompt must include a mandatory Partial-Fix Regression review axis. For every finding marked closed in a prior pass (visible via the convergence report or fix commit), the adversary must verify that (a) the fix is correct and complete in the primary file and (b) the same pattern has been checked in all sibling files of the same type. Failing to perform this check is itself a finding.

## Preconditions

1. A convergence report or prior-pass finding list is available to the adversary.
2. At least one finding from a prior pass has been marked closed or fixed.
3. The adversary agent is performing a Spec Review or Implementation Review pass.

## Postconditions

1. For every closed prior-pass finding, the adversary's output includes an explicit "Partial-Fix Regression check" entry stating: (a) the fix was verified in the primary file, (b) sibling files were checked, (c) whether the same fix was needed and applied to siblings.
2. A finding where the fix was applied to the primary file but not propagated to required sibling files is reported as MEDIUM severity (or HIGH if blast radius is large).
3. The adversary prompt's Spec Review section (or a new "Regression Propagation" axis) contains the requirement to perform this check in every pass.

## Invariants

1. The Partial-Fix Regression axis is listed in the adversary prompt's review checklist — not as an optional note, but as a mandatory step.
2. "Sibling files" is explicitly defined in the prompt as: other BCs in the same subsystem, other agent prompts, other template files of the same type.
3. The adversary must provide file:line evidence for any "sibling not updated" finding.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Prior pass had zero findings (clean pass) | No regression check is needed. Adversary notes "no prior findings to check for regression." |
| EC-002 | Prior finding fix applied to primary but sibling files differ | The adversary cannot adjudicate intent (read-only profile). Adversary always reports the difference as a finding with severity LOW and tag '(pending intent verification)'. The orchestrator or human adjudicates based on a justification artifact (commit message, ADR, design note). The adversary's role is detection; orchestrator/human is adjudication. |
| EC-003 | Adversary finds a sibling that needs the same fix but was not in the original finding's scope | Reports as a new finding with MEDIUM severity. References the original closed finding for context. |
| EC-004 | Fix was applied to sibling but the application was incomplete (correct direction but wrong depth) | Reports as a new finding — partial propagation counts as incomplete propagation. |

## Canonical Test Vectors

| Input State | Expected Adversary Output | Category |
|-------------|--------------------------|----------|
| Prior pass: F-001 "BC-5.01.001 missing anchor" — closed. Sibling BC-5.01.002 has same anchor gap | Adversary reports new finding: propagation incomplete; BC-5.01.002 anchor gap missed | negative (regression found) |
| Prior pass: F-027 "capabilities.md count wrong" — closed. edge-cases.md also had count; was not updated | Adversary reports MEDIUM: fix applied to capabilities.md but edge-cases.md sibling not updated | negative (regression found) |
| Prior pass: F-011 "story-writer missing BC gate" — closed. product-owner and adversary prompts checked; both already had equivalent gate | Adversary reports: no regression; siblings correct | happy-path |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-061 | Partial-Fix Regression axis is present in adversary.md Spec Review section | static-check (peer review) |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-001 |
| Capability Anchor Justification | Anchored to CAP-001 ("Run a self-orchestrating LLM-driven SDLC pipeline") per capabilities.md §CAP-001 — the adversary agent is the review and convergence component of the self-orchestrating SDLC pipeline defined by CAP-001. |
| L2 Domain Invariants | none |
| Architecture Module | plugins/vsdd-factory/agents/adversary.md |
| Stories | S-7.01 |
| Source AC | S-7.01 §AC-003 |
| FR | FR-042 |

## Related BCs

- BC-5.36.006 — sibling (fix propagation scope; composes with this BC's regression check axis)
- BC-5.36.003 — sibling (adversary policy 5 also enforces capability anchor justification)

## Architecture Anchors

- `plugins/vsdd-factory/agents/adversary.md` — agent prompt file to be modified

## Story Anchor

S-7.01

## VP Anchors

- VP-061 — Partial-Fix Regression axis presence
