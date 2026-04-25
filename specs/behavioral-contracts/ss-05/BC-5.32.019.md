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

# Behavioral Contract BC-5.32.019: discovery:feature-debate

## Description

Step `feature-debate` (line 206). Type: agent. Agent: adversary. Depends: `[feature-scoring-value, feature-scoring-feasibility, feature-scoring-novelty]`. Condition: feature_discovery enabled. Source 206-223. Delphi Step 3: adversarial challenge with fresh context. Identifies disagreements >0.3, challenges scores, classifies into 6 idea profiles, computes confidence-weighted composite scores.

## Preconditions

1. All three Delphi Step 1 scorers have completed.
2. Adversary has fresh context.

## Postconditions

1. Disagreements >0.3 are identified across the three scorer dimensions.
2. Each opportunity is classified into one of 6 idea profiles.
3. Confidence-weighted composite scores computed.

## Invariants

1. Disagreement threshold is 0.3.
2. Exactly 6 idea profiles defined (per discovery.lobster spec).

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | All scorers agree | No disagreements flagged |
| EC-002 | Extreme disagreement | Score challenge produced |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Aligned scores | No challenges | happy-path |
| Diverging scores | Challenges + classification | edge-case |
| Wall breach | Denied | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| (TBD — to be assigned in Phase 1.6c) | Disagreement threshold = 0.3 | manual |
| VP-002 | All ideas classified into one of 6 profiles | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD |
| Architecture Module | SS-05 |
| Stories | TBD |

## Related BCs (Recommended)

- BC-5.32.016 — feature-scoring-value
- BC-5.32.017 — feature-scoring-feasibility
- BC-5.32.018 — feature-scoring-novelty

## Architecture Anchors (Recommended)

- `architecture/ss-05-orchestration.md#delphi-scoring`

## Story Anchor (Recommended)

TBD

## VP Anchors (Recommended)

- (TBD — to be assigned in Phase 1.6c)
- VP-002

---

### Brownfield-Specific Sections

#### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `plugins/vsdd-factory/workflows/discovery.lobster` (lines 206-223) |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |

#### Evidence Types Used

- documentation

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | reads (filtered context) |
| **Global state access** | reads filtered |
| **Deterministic** | no |
| **Thread safety** | unknown |
| **Overall classification** | effectful shell |

#### Refactoring Notes

No refactoring needed.
