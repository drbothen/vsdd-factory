---
document_type: behavioral-contract
level: L3
version: "1.1"
status: draft
producer: codebase-analyzer
timestamp: 2026-04-25T00:00:00
phase: 1.4b
inputs: [.factory/phase-0-ingestion/pass-3-deep-agents.md]
input-hash: "595f07d"
traces_to: .factory/specs/architecture/ARCH-INDEX.md
origin: brownfield
extracted_from: .factory/phase-0-ingestion/pass-3-deep-agents.md:921
subsystem: SS-05
capability: CAP-TBD
lifecycle_status: active
introduced: v1.0.0-beta.4
modified: []
deprecated: null
deprecated_by: null
replacement: null
retired: null
removed: null
removal_reason: null
---

# Behavioral Contract BC-5.08.014: pr-manager: 9-step coordinator, never exits mid-flow

## Description

The pr-manager MUST execute all 9 steps in order: (1) populate PR description,
(2) verify demo evidence, (3) create PR, (4) security review, (5) review
convergence loop, (6) wait for CI, (7) dependency check, (8) execute merge,
(9) post-merge cleanup. Each step ends with `STEP_COMPLETE: step=N name=... status=ok`
followed by immediate progression to the next step. Treating a sub-agent's
"APPROVE" return as completion is forbidden.

## Preconditions

1. pr-manager dispatched.

## Postconditions

1. pr-manager session log shows 9 STEP_COMPLETE markers in order.
2. Final exit only after step 9.

## Invariants

1. The 9-step sequence is canonical and complete.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Sub-agent returns APPROVE early | Continue to next step; do not exit |
| EC-002 | Step blocks | Use BLOCKED escalation; do not skip |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Full 9-step execution | Accepted | happy-path |
| Exit after step 4 (security APPROVE) | Rejected | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-TBD | Session log has 9 STEP_COMPLETE markers in order | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD |
| L2 Domain Invariants | DI-TBD |
| Architecture Module | `plugins/vsdd-factory/agents/pr-manager.md` |
| Stories | TBD |

## Related BCs (Recommended)

- BC-5.08.015 — composes with (delegates gh/git to github-ops)
- BC-5.08.016 — composes with (max 10 review cycles)

## Architecture Anchors (Recommended)

- `architecture/SS-05-orchestration.md#pr-manager`

## Story Anchor (Recommended)

TBD

## VP Anchors (Recommended)

- VP-TBD

---

### Brownfield-Specific Sections

#### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `plugins/vsdd-factory/agents/pr-manager.md:26-29, 89-258, 354` |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |

#### Evidence Types Used

- documentation: explicit COORDINATOR RULE + 9-step definitions

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | reads/writes (PR + dispatches) |
| **Global state access** | reads/writes pipeline state |
| **Deterministic** | yes |
| **Thread safety** | unknown |
| **Overall classification** | effectful shell |

#### Refactoring Notes

No refactoring needed.
