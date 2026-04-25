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
extracted_from: .factory/phase-0-ingestion/pass-3-deep-agents.md:937
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

# Behavioral Contract BC-5.08.016: pr-manager: max 10 review convergence cycles before human escalation

## Description

The review convergence loop runs at most 10 cycles. If blocking findings remain
after cycle 10, the agent escalates to human with BLOCKED status — it does not
loop indefinitely.

## Preconditions

1. pr-manager in Step 5 (review convergence loop).

## Postconditions

1. review-findings.md cycle table has ≤10 rows.
2. Cycle 11 only appears as a BLOCKED escalation.

## Invariants

1. The 10-cycle bound is hard.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Cycle 10 still has BLOCKING | Escalate to human |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Convergence at cycle 4 | Accepted | happy-path |
| Cycle 11 attempt | Rejected; human escalation | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-TBD | review-findings.md has ≤10 cycle rows | manual |

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
| **Path** | `plugins/vsdd-factory/agents/pr-manager.md:165, 188, 296-298` |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |

#### Evidence Types Used

- documentation: explicit max 10 cycles rule

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | reads/writes (review log) |
| **Global state access** | none |
| **Deterministic** | yes |
| **Thread safety** | unknown |
| **Overall classification** | pure (counter) |

#### Refactoring Notes

No refactoring needed.
