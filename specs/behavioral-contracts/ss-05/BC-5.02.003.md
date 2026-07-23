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
extracted_from: .factory/phase-0-ingestion/pass-3-deep-agents.md:781
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

# Behavioral Contract BC-5.02.003: orchestrator: never skips per-story delivery sub-steps

## Description

Every story MUST traverse all 7 per-story-delivery sub-steps in order:
(a) test-writer: stubs → (b) test-writer: failing tests → (c) implementer: TDD →
(d) demo-recorder: per-AC demos → (e) push → (f) pr-manager: full 9-step PR
process → (g) worktree cleanup. Shortcuts are forbidden.

## Preconditions

1. A story enters the per-story delivery flow.

## Postconditions

1. Orchestrator dispatch log per story shows all 7 sub-steps in order.
2. Story is not marked complete until all 7 sub-steps have completed.
3. Skipping demo recording or going directly to github-ops is rejected.

## Invariants

1. The 7-step sequence is canonical and append-only.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Test-writer reports BLOCKED at step (b) | Halt sequence; do not advance |
| EC-002 | demo-recorder produces blank demo | Reported BLOCKED per BC-5.03.017 |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Per-story dispatch log | 7 sub-steps in canonical order | happy-path |
| Dispatch jumping from (c) to (e) | Rejected | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-TBD | Every completed story has 7 sub-step entries in the orchestrator log | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD |
| L2 Domain Invariants | DI-TBD |
| Architecture Module | `plugins/vsdd-factory/agents/orchestrator/orchestrator.md`, `per-story-delivery.md` |
| Stories | TBD |

## Related BCs (Recommended)

- BC-5.07.028 — composes with (implementer Red Gate)
- BC-5.03.006 — composes with (demo-recorder output destination)
- BC-5.08.014 — composes with (pr-manager 9-step coordinator)

## Architecture Anchors (Recommended)

- `architecture/SS-05-orchestration.md#per-story-delivery`

## Story Anchor (Recommended)

TBD

## VP Anchors (Recommended)

- VP-TBD

---

### Brownfield-Specific Sections

#### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `plugins/vsdd-factory/agents/orchestrator/orchestrator.md:104-107` |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |

#### Evidence Types Used

- documentation: explicit per-story-delivery checklist in orchestrator agent body

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | reads (delivery checklist) |
| **Global state access** | reads pipeline state |
| **Deterministic** | yes |
| **Thread safety** | unknown |
| **Overall classification** | effectful shell |

#### Refactoring Notes

No refactoring needed.
