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

# Behavioral Contract BC-5.31.027: code-delivery:delivery-human-approval

## Description

Step `delivery-human-approval` (line 418). Type: human-approval. Timeout: 24h. Depends: `[merge-pr]`. Condition: `merge_decision.requires_human == true`. Source 418-424.

## Preconditions

1. merge-pr completed and merge decision flags `requires_human=true`.

## Postconditions

1. Human approval recorded (approved or rejected) within 24h.
2. On timeout, escalation per workflow defaults.

## Invariants

1. Step is skipped when human approval is not required.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | requires_human=false | Skipped |
| EC-002 | Timeout reached | Escalate |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Manual approval needed | Wait, then proceed | happy-path |
| Auto-merge eligible | Skipped | edge-case |
| 24h timeout | Escalate | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| (TBD — to be assigned in Phase 1.6c) | Step bounded by 24h timeout | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD |
| Architecture Module | SS-05 |
| Stories | TBD |

## Related BCs (Recommended)

- BC-5.31.026 — merge-pr
- BC-5.31.028 — cleanup-worktree

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
| **Path** | `plugins/vsdd-factory/workflows/code-delivery.lobster` (lines 418-424) |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |

#### Evidence Types Used

- type constraint: human-approval + timeout
- guard clause: condition

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | reads (approval store) |
| **Global state access** | external (human input) |
| **Deterministic** | no |
| **Thread safety** | unknown |
| **Overall classification** | opaque |

#### Refactoring Notes

No refactoring needed.
