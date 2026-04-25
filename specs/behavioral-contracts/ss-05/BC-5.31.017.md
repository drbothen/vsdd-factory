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

# Behavioral Contract BC-5.31.017: code-delivery:squash-and-push

## Description

Step `squash-and-push` (line 241). Type: agent. Agent: implementer. Depends: `[demo-recording, e2e-tests]`. Source 241-249. Squashes the per-story commits and pushes to the remote branch in preparation for PR creation.

## Preconditions

1. demo-recording and e2e-tests upstream gates have passed (per DAG; e2e may be skipped).
2. Remote branch is writable.

## Postconditions

1. Worktree branch contains a single squash commit representing the story.
2. Remote is updated with the squashed branch.

## Invariants

1. Original micro-commits are not preserved on the remote feature branch.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Push rejected (non-fast-forward) | Step fails or rebases per agent contract |
| EC-002 | Network outage | Step fails / escalates |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Clean branch | Squashed and pushed | happy-path |
| Diverged remote | Step fails | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| (TBD — to be assigned in Phase 1.6c) | Remote branch contains exactly one new commit after step | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD |
| Architecture Module | SS-05 |
| Stories | TBD |

## Related BCs (Recommended)

- BC-5.31.018 — create-pr (downstream)

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
| **Path** | `plugins/vsdd-factory/workflows/code-delivery.lobster` (lines 241-249) |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |

#### Evidence Types Used

- documentation

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | reads + writes + network |
| **Global state access** | git remote |
| **Deterministic** | yes |
| **Thread safety** | unknown |
| **Overall classification** | effectful shell |

#### Refactoring Notes

No refactoring needed.
