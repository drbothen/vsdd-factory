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
bc_id: BC-8.28.002
section: "8.28"
---

# BC-8.28.002: orchestrator cycle-closing checklist references lessons-codification.md rule

## Description

The orchestrator's cycle-closing checklist (wherever it is defined — as a skill prompt, a rule file, or a STATE.md protocol section) must include an explicit reference to `plugins/vsdd-factory/rules/lessons-codification.md`. Specifically, before a sub-cycle is declared CLOSED, the orchestrator must check whether any `[process-gap]` findings from the final convergence report have been addressed by a follow-up story or justified deferral.

## Preconditions

1. `plugins/vsdd-factory/rules/lessons-codification.md` exists (BC-8.28.001 satisfied).
2. The orchestrator is executing the cycle-closing checklist for a sub-cycle.
3. A final convergence report is available.

## Postconditions

1. The orchestrator's cycle-closing checklist includes a step: "Scan final convergence report for `[process-gap]` findings per `rules/lessons-codification.md`."
2. If any `[process-gap]` findings lack a follow-up story or justified deferral, the cycle status remains open until one is provided.
3. The rule reference is by file path, not by memory — ensuring the rule is actually consulted rather than recalled from model weights.

## Invariants

1. The checklist reference is present in whichever document defines the cycle-closing procedure.
2. The orchestrator must not bypass the check by declaring "no process gaps found" without actually reading the convergence report.
3. E-7 is cited in the rule file as a canonical example of this rule being applied.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Sub-cycle has zero adversarial findings (clean pass) | Checklist step runs; finding is "0 process-gap findings" — cycle may close. |
| EC-002 | The cycle-closing procedure is distributed across multiple documents/skills | Each such document must contain the reference. If it is in only one of three checklist locations, the other two are gaps. |
| EC-003 | Orchestrator is a human (not an LLM agent) for a particular cycle | The rule still applies. The human orchestrator is required to follow the same checklist. |

## Canonical Test Vectors

| Input State | Expected Behavior | Category |
|-------------|-------------------|----------|
| Cycle-closing checklist executed; convergence report has 0 `[process-gap]` findings | Cycle declared CLOSED | happy-path |
| Cycle-closing checklist executed; convergence report has 1 `[process-gap]` finding; follow-up story S-8.01 opened | Cycle declared CLOSED; S-8.01 reference logged | happy-path |
| Cycle-closing checklist executed; convergence report has 1 `[process-gap]` finding; no story, no deferral | Cycle remains OPEN; orchestrator must act | negative |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-062 | Cycle-closing checklist contains process-gap scan step referencing lessons-codification.md | static-check (review of checklist document) |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-001 |
| Capability Anchor Justification | Anchored to CAP-001 ("Run a self-orchestrating LLM-driven SDLC pipeline") per capabilities.md §CAP-001 — the cycle-closing checklist is a governance checkpoint in the self-orchestrating SDLC defined by CAP-001; this BC ensures the pipeline self-improves by acting on process gaps before declaring a cycle complete. |
| L2 Domain Invariants | none |
| Architecture Module | plugins/vsdd-factory/agents/orchestrator/orchestrator.md (primary — cycle-closing logic lives in the orchestrator sequence); TBD — implementer must identify exact skill/section during S-7.02 delivery and update this field. A task exists in S-7.02 Task 4 to locate the cycle-closing checklist. If the checklist spans multiple documents, all paths must be listed here. |
| Stories | S-7.02 |
| Source AC | S-7.02 §AC-004 |
| FR | FR-042 |

## Related BCs

- BC-8.28.001 — depends on (rule file must exist; this BC specifies its integration into the checklist)

## Architecture Anchors

- `plugins/vsdd-factory/rules/lessons-codification.md` — rule document referenced by checklist
- `plugins/vsdd-factory/agents/orchestrator/orchestrator.md` — primary orchestrator agent (cycle-closing checklist location; verify during implementation)
- TBD: exact skill/section within orchestrator to be identified by implementer during S-7.02 delivery

## Story Anchor

S-7.02

## VP Anchors

- VP-062 — cycle-closing checklist content
