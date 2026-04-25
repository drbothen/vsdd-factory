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

# Behavioral Contract BC-5.32.016: discovery:feature-scoring-value

## Description

Step `feature-scoring-value` (line 167). Type: agent. Agent: product-owner. Depends: `[intelligence-synthesis]`. Condition: `config.feature_discovery.enabled == true`. Source 167-178. Delphi Step 1 (independent scoring, no other scores visible). Scores Value, Alignment, Time Criticality, evidence_strength.

## Preconditions

1. intelligence-synthesis completed.
2. Feature discovery is enabled.
3. No other Delphi scorer's outputs are visible to this scorer.

## Postconditions

1. Value, Alignment, Time Criticality, and evidence_strength scores produced for each feature opportunity.

## Invariants

1. Scorer cannot read feasibility or novelty scores at this stage.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Wall breach attempt | Denied |
| EC-002 | No opportunities to score | No-op output |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Synthesis with opportunities | Scores produced | happy-path |
| Empty synthesis | Empty scoring | edge-case |
| Wall breach | Denied | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| (TBD — to be assigned in Phase 1.6c) | Other Delphi scorers' outputs not in scorer context | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD |
| Architecture Module | SS-05 |
| Stories | TBD |

## Related BCs (Recommended)

- BC-5.32.017 — feature-scoring-feasibility
- BC-5.32.018 — feature-scoring-novelty
- BC-5.32.019 — feature-debate

## Architecture Anchors (Recommended)

- `architecture/ss-05-orchestration.md#delphi-scoring`

## Story Anchor (Recommended)

TBD

## VP Anchors (Recommended)

- (TBD — to be assigned in Phase 1.6c)

---

### Brownfield-Specific Sections

#### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `plugins/vsdd-factory/workflows/discovery.lobster` (lines 167-178) |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |

#### Evidence Types Used

- documentation: Delphi step semantics

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | reads (synthesis) |
| **Global state access** | reads filtered context |
| **Deterministic** | no |
| **Thread safety** | unknown |
| **Overall classification** | effectful shell |

#### Refactoring Notes

No refactoring needed.
