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
extracted_from: .factory/phase-0-ingestion/pass-3-deep-agents.md:483
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

# Behavioral Contract BC-5.09.002: dtu-validator: fidelity thresholds enforced per L-tier

## Description

A clone PASSES validation only when fidelity ≥ {L1=0.85, L2=0.90, L3=0.95,
L4=0.98}. Scores below the threshold trigger a fix story routed to implementer.
The validator MUST NOT mark a clone as validated below threshold.

## Preconditions

1. dtu-validator measured fidelity for a clone.

## Postconditions

1. Every fidelity-report.md states the L-tier and the measured score.
2. PASS only when score ≥ threshold for the declared tier.

## Invariants

1. Per-tier thresholds are mandatory and documented.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Score 0.94 for L3 | FAIL; fix story |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| L2 score 0.91 | PASS | happy-path |
| L4 score 0.97 | FAIL | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-TBD | PASS only when score ≥ tier threshold | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD |
| L2 Domain Invariants | DI-TBD |
| Architecture Module | `plugins/vsdd-factory/agents/dtu-validator.md` |
| Stories | TBD |

## Related BCs (Recommended)

- BC-5.09.001 — composes with (test API keys)
- BC-5.09.003 — composes with (drift detection)

## Architecture Anchors (Recommended)

- `architecture/SS-05-orchestration.md#dtu-validator`

## Story Anchor (Recommended)

TBD

## VP Anchors (Recommended)

- VP-TBD

---

### Brownfield-Specific Sections

#### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `plugins/vsdd-factory/agents/dtu-validator.md:39, 71-78` |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |

#### Evidence Types Used

- documentation: explicit fidelity thresholds table

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | writes (report) |
| **Global state access** | none |
| **Deterministic** | yes (given fixed measurements) |
| **Thread safety** | unknown |
| **Overall classification** | pure (threshold check) |

#### Refactoring Notes

No refactoring needed.
