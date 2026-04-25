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

# Behavioral Contract BC-5.32.021: discovery:product-scoring

## Description

Step `product-scoring` (line 242). Type: agent. Agent: product-owner. Depends: `[product-research, intelligence-synthesis]`. Condition: product_discovery enabled. Source 242-256.

## Preconditions

1. product-research and intelligence-synthesis completed.
2. product_discovery is enabled.

## Postconditions

1. Product ideas scored.

## Invariants

1. Step skipped when product_discovery disabled.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | No products to score | Empty output |
| EC-002 | Disabled | Skipped |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Products + synthesis | Scored | happy-path |
| Disabled | Skipped | edge-case |
| Empty | Empty output | edge-case |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| (TBD — to be assigned in Phase 1.6c) | Scorer reads both upstream inputs | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD |
| Architecture Module | SS-05 |
| Stories | TBD |

## Related BCs (Recommended)

- BC-5.32.020 — product-research
- BC-5.32.022 — deduplication (downstream)

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
| **Path** | `plugins/vsdd-factory/workflows/discovery.lobster` (lines 242-256) |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |

#### Evidence Types Used

- guard clause + documentation

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | reads (synthesis, research) |
| **Global state access** | reads filesystem |
| **Deterministic** | no |
| **Thread safety** | unknown |
| **Overall classification** | effectful shell |

#### Refactoring Notes

No refactoring needed.
