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
extracted_from: .factory/phase-0-ingestion/pass-3-deep-agents.md:953
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

# Behavioral Contract BC-5.08.018: pr-manager: max 3 CI fix cycles before human escalation

## Description

When CI fails, the pr-manager spawns implementer to fix and re-pushes — at most
3 times. After 3 consecutive CI failures, escalate.

## Preconditions

1. pr-manager in Step 6 (wait for CI), CI fails.

## Postconditions

1. Step 6 retry counter ≤3.
2. Cycle 4 only as BLOCKED escalation.

## Invariants

1. The 3-cycle CI fix bound is hard.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Cycle 3 fails | Escalate to human |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Fix on cycle 2 | CI green; proceed | happy-path |
| 3 cycles of CI failure | Human escalation | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-TBD | CI fix counter ≤3 in pr-manager log | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD |
| L2 Domain Invariants | DI-TBD |
| Architecture Module | `plugins/vsdd-factory/agents/pr-manager.md` |
| Stories | TBD |

## Related BCs (Recommended)

- BC-5.08.014 — composes with (9-step coordinator)

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
| **Path** | `plugins/vsdd-factory/agents/pr-manager.md:204` |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |

#### Evidence Types Used

- documentation: explicit max 3 CI fix cycles rule

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | reads/writes (CI state) |
| **Global state access** | none |
| **Deterministic** | yes |
| **Thread safety** | unknown |
| **Overall classification** | pure (counter) |

#### Refactoring Notes

No refactoring needed.
