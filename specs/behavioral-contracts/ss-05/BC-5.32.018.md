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

# Behavioral Contract BC-5.32.018: discovery:feature-scoring-novelty

## Description

Step `feature-scoring-novelty` (line 193). Type: agent. Agent: adversary. Depends: `[intelligence-synthesis]`. Condition: `config.feature_discovery.enabled == true`. Source 193-204. Delphi Step 1 with fresh context. Scores Novelty; flags "smart plagiarism".

## Preconditions

1. intelligence-synthesis completed.
2. Feature discovery enabled.
3. Adversary is given fresh context (no prior conversation history).

## Postconditions

1. Novelty score produced per opportunity.
2. Smart-plagiarism flags raised where applicable.

## Invariants

1. Scorer runs with fresh context (no history).
2. Other Delphi scorer outputs not visible.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Plagiarism detected | Flag raised |
| EC-002 | Wall breach | Denied |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Original idea | High novelty | happy-path |
| Smart plagiarism | Flag raised | edge-case |
| Wall breach | Denied | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-001 | Adversary invoked with fresh context | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD |
| Architecture Module | SS-05 |
| Stories | TBD |

## Related BCs (Recommended)

- BC-5.32.016 — feature-scoring-value
- BC-5.32.017 — feature-scoring-feasibility
- BC-5.32.019 — feature-debate

## Architecture Anchors (Recommended)

- `architecture/ss-05-orchestration.md#delphi-scoring`

## Story Anchor (Recommended)

TBD

## VP Anchors (Recommended)

- VP-001

---

### Brownfield-Specific Sections

#### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `plugins/vsdd-factory/workflows/discovery.lobster` (lines 193-204) |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |

#### Evidence Types Used

- documentation

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | reads (filtered) |
| **Global state access** | reads filtered |
| **Deterministic** | no |
| **Thread safety** | unknown |
| **Overall classification** | effectful shell |

#### Refactoring Notes

No refactoring needed.
