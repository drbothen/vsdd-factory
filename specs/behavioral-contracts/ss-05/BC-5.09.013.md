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
extracted_from: .factory/phase-0-ingestion/pass-3-deep-agents.md:1207
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

# Behavioral Contract BC-5.09.013: session-reviewer: tracks own cost; flags >5% of pipeline run cost

## Description

The session-reviewer measures its own cost and flags optimization in its own
improvement proposals if it exceeds 5% of the pipeline run cost.

## Preconditions

1. session-reviewer concluding a session review.

## Postconditions

1. Session review cost-summary shows separate `session_review_cost` and `pipeline_run_cost`.
2. Ratio > 0.05 triggers a self-optimization improvement proposal entry.

## Invariants

1. Self-cost-awareness is mandatory.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Pipeline run cost unavailable | Document gap; skip ratio check |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| review_cost / run_cost = 0.03 | No flag | happy-path |
| ratio = 0.07 | Self-optimization proposal | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-TBD | cost-summary has both fields; ratio > 0.05 triggers proposal | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD |
| L2 Domain Invariants | DI-TBD |
| Architecture Module | `plugins/vsdd-factory/agents/session-reviewer.md` |
| Stories | TBD |

## Related BCs (Recommended)

- BC-5.09.011 — composes with (actionable proposals)

## Architecture Anchors (Recommended)

- `architecture/SS-05-orchestration.md#session-reviewer`

## Story Anchor (Recommended)

TBD

## VP Anchors (Recommended)

- VP-TBD

---

### Brownfield-Specific Sections

#### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `plugins/vsdd-factory/agents/session-reviewer.md:188-191` |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |

#### Evidence Types Used

- documentation: explicit Self-Cost Awareness section

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | reads (cost data) + writes (proposal) |
| **Global state access** | none |
| **Deterministic** | yes |
| **Thread safety** | unknown |
| **Overall classification** | pure (numerical check) |

#### Refactoring Notes

No refactoring needed.
