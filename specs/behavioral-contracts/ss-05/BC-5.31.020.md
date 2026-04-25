---
document_type: behavioral-contract
level: L3
version: "1.1"
status: draft
producer: phase-1-4b-agent-5
timestamp: 2026-04-25T00:00:00
phase: 1.4b
inputs: [.factory/phase-0-ingestion/pass-3-deep-workflows.md]
input-hash: "99bbe9c"
traces_to: domain-spec/L2-INDEX.md
origin: brownfield
extracted_from: "plugins/vsdd-factory/workflows/code-delivery.lobster"
subsystem: "SS-05"
capability: "CAP-TBD"
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

# Behavioral Contract BC-5.31.020: code-delivery:security-review

## Description

Step `security-review` (line 289). Type: agent. Agent: security-reviewer. Depends: `[create-pr]`. Condition: `module_criticality in ['CRITICAL', 'HIGH']`. Source 289-301. Information-asymmetry wall: excludes implementer notes. Max 3 security review cycles.

## Preconditions

1. module_criticality is CRITICAL or HIGH.
2. PR is open.

## Postconditions

1. Security verdict produced (or step skipped for low-criticality).
2. Implementer notes are not visible to security-reviewer.

## Invariants

1. Step is skipped for low-criticality modules.
2. Cycle count ≤ 3.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | module_criticality=LOW | Skipped |
| EC-002 | Reviewer attempts wall breach | Denied |
| EC-003 | 3 cycles without resolution | Escalation |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| CRITICAL module | Review run | happy-path |
| LOW module | Skipped | edge-case |
| Persistent finding | Escalate at cycle 3 | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-001 | Implementer notes never appear in reviewer context | manual |
| VP-002 | Max cycles ≤ 3 | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD |
| Architecture Module | SS-05 |
| Stories | TBD |

## Related BCs (Recommended)

- BC-5.31.018 — create-pr
- BC-5.31.021 — pr-review-convergence (parallel)

## Architecture Anchors (Recommended)

- `architecture/ss-05-orchestration.md#information-asymmetry-walls`

## Story Anchor (Recommended)

TBD

## VP Anchors (Recommended)

- VP-001
- VP-002

---

### Brownfield-Specific Sections

#### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `plugins/vsdd-factory/workflows/code-delivery.lobster` (lines 289-301) |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |

#### Evidence Types Used

- guard clause: criticality condition
- type constraint: max cycles

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | reads (filtered context) |
| **Global state access** | reads filtered filesystem |
| **Deterministic** | no |
| **Thread safety** | unknown |
| **Overall classification** | effectful shell |

#### Refactoring Notes

No refactoring needed.
