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

# Behavioral Contract BC-5.32.028: discovery:route-approved-ideas

## Description

Step `route-approved-ideas` (line 355). Type: agent. Agent: orchestrator. Depends: `[discovery-review]`. Source 355-370. Routes feature ideas to product `.factory/`, product ideas to `planning.lobster`, rejected ideas to cooldown YAML, deferred to re-evaluation YAML.

## Preconditions

1. discovery-review provided decisions.

## Postconditions

1. Approved feature ideas filed under product `.factory/`.
2. Approved product ideas queued for planning.lobster invocation.
3. Rejected ideas added to cooldown YAML.
4. Deferred ideas added to re-evaluation YAML.

## Invariants

1. Every reviewed idea ends up in exactly one downstream destination (no drops).

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | All rejected | Cooldown YAML grows; downstream sub-workflows skip |
| EC-002 | All approved features | Feature directory updated |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Mixed decisions | Routed to all 4 destinations | happy-path |
| All approved | Feeds product/planning | edge-case |
| All rejected | Cooldown only | edge-case |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| (TBD — to be assigned in Phase 1.6c) | Sum of routed ideas equals reviewed count | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD |
| Architecture Module | SS-05 |
| Stories | TBD |

## Related BCs (Recommended)

- BC-5.32.027 — discovery-review
- BC-5.32.030 — execute-product-ideas
- BC-5.32.031 — execute-feature-ideas

## Architecture Anchors (Recommended)

- `architecture/ss-05-orchestration.md#discovery-workflow`

## Story Anchor (Recommended)

TBD

## VP Anchors (Recommended)

- (TBD — to be assigned in Phase 1.6c)

---

### Brownfield-Specific Sections

#### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `plugins/vsdd-factory/workflows/discovery.lobster` (lines 355-370) |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |

#### Evidence Types Used

- documentation: routing table behavior

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | reads + writes (filesystem) |
| **Global state access** | filesystem |
| **Deterministic** | yes |
| **Thread safety** | unknown |
| **Overall classification** | effectful shell |

#### Refactoring Notes

No refactoring needed.
