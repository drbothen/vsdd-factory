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

# Behavioral Contract BC-5.32.022: discovery:deduplication

## Description

Step `deduplication` (line 263). Type: agent. Agent: consistency-validator. Depends: `[feature-debate, product-scoring]`. Source 263-281. Embedding-based three-tier deduplication: >0.92 auto-merge, 0.85-0.92 human review, 0.70-0.85 related, <0.70 distinct. HDBSCAN cluster analysis.

## Preconditions

1. feature-debate and product-scoring completed.
2. Embedding model available.

## Postconditions

1. Each idea is classified into one of four similarity buckets per the threshold table.
2. HDBSCAN clusters produced.

## Invariants

1. Threshold table is exactly the specified values: 0.92, 0.85, 0.70.
2. Dedup is deterministic given embeddings + thresholds.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | All distinct | No merges |
| EC-002 | Borderline (0.85-0.92) | Routed to human review |
| EC-003 | Embedding model failure | Step fails |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Diverse ideas | Clustered | happy-path |
| Near-duplicates | Auto-merged | edge-case |
| Model failure | Escalate | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-001 | Bucketing matches threshold table exactly | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD |
| Architecture Module | SS-05 |
| Stories | TBD |

## Related BCs (Recommended)

- BC-5.32.019 — feature-debate
- BC-5.32.021 — product-scoring

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
| **Path** | `plugins/vsdd-factory/workflows/discovery.lobster` (lines 263-281) |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |

#### Evidence Types Used

- documentation: explicit threshold values

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | reads (embeddings) |
| **Global state access** | reads embedding store |
| **Deterministic** | yes (given embeddings) |
| **Thread safety** | unknown |
| **Overall classification** | pure (decision) |

#### Refactoring Notes

No refactoring needed.
