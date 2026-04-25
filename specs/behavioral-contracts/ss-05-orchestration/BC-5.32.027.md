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
extracted_from: "plugins/vsdd-factory/workflows/discovery.lobster"
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

# Behavioral Contract BC-5.32.027: discovery:discovery-review

## Description

Step `discovery-review` (line 337). Type: human-approval. Timeout: 72h. Depends: `[generate-report]`. Source 337-349.

## Preconditions

1. generate-report completed and report is human-readable.

## Postconditions

1. Human review decision recorded (approve / defer / reject) within 72h.
2. On timeout, escalation per workflow defaults.

## Invariants

1. Human approval is bounded by 72h.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Timeout | Escalate |
| EC-002 | Reject all | Routing-step handles rejection |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Approve some | Decision captured | happy-path |
| Defer | Decision captured | edge-case |
| Timeout | Escalate | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-001 | Bounded by 72h | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD |
| Architecture Module | SS-05 |
| Stories | TBD |

## Related BCs (Recommended)

- BC-5.32.024 — generate-report
- BC-5.32.028 — route-approved-ideas

## Architecture Anchors (Recommended)

- `architecture/ss-05-orchestration.md#discovery-workflow`

## Story Anchor (Recommended)

TBD

## VP Anchors (Recommended)

- VP-001

---

### Brownfield-Specific Sections

#### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `plugins/vsdd-factory/workflows/discovery.lobster` (lines 337-349) |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |

#### Evidence Types Used

- type constraint: human-approval + timeout

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
