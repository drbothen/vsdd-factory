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

# Behavioral Contract BC-5.31.025: code-delivery:dependency-merge-check

## Description

Step `dependency-merge-check` (line 394). Type: agent. Agent: pr-manager. Depends: `[wait-for-ci]`. Source 394-402. Verifies upstream/downstream dependency PRs are mergeable in the right order.

## Preconditions

1. wait-for-ci passed.
2. Repo's dependency graph between pending PRs is computable.

## Postconditions

1. Decision recorded: this PR is safe to merge with respect to in-flight dependent PRs.

## Invariants

1. Step never bypasses an unmerged required upstream.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Required upstream open | Block merge |
| EC-002 | No deps | Pass |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Standalone PR | Pass | happy-path |
| Pending upstream | Block | edge-case |
| Cycle | Fail | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| (TBD — to be assigned in Phase 1.6c) | Merge always serializes after upstream PRs | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD |
| Architecture Module | SS-05 |
| Stories | TBD |

## Related BCs (Recommended)

- BC-5.31.024 — wait-for-ci
- BC-5.31.026 — merge-pr (downstream)

## Architecture Anchors (Recommended)

- `architecture/ss-05-orchestration.md#code-delivery-workflow`

## Story Anchor (Recommended)

TBD

## VP Anchors (Recommended)

- (TBD — to be assigned in Phase 1.6c)

---

### Brownfield-Specific Sections

#### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `plugins/vsdd-factory/workflows/code-delivery.lobster` (lines 394-402) |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |

#### Evidence Types Used

- documentation

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | network calls (PR API) |
| **Global state access** | reads PR host |
| **Deterministic** | yes |
| **Thread safety** | unknown |
| **Overall classification** | effectful shell |

#### Refactoring Notes

No refactoring needed.
