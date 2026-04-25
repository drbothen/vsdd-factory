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
extracted_from: .factory/phase-0-ingestion/pass-3-deep-agents.md:499
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

# Behavioral Contract BC-5.09.003: dtu-validator: drift >5% triggers stale flag and fix story

## Description

Maintenance-mode drift monitoring re-runs fidelity checks. If the score drops
more than 5 percentage points from the prior baseline, the clone is marked
stale and a fix story is created listing the changed endpoints.

## Preconditions

1. dtu-validator running drift check in maintenance mode.

## Postconditions

1. drift-report.md exists with `delta` field.
2. `delta > 0.05` produces a fix story entry in STORY-INDEX.md.
3. Clone marked stale.

## Invariants

1. The 5-point drift threshold is canonical.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Delta exactly 0.05 | TBD — likely no flag (use `>` not `>=`) |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Delta 0.04 | No flag | happy-path |
| Delta 0.07 | Stale flag + fix story | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-TBD | Drift > 0.05 produces fix-story entry | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD |
| L2 Domain Invariants | DI-TBD |
| Architecture Module | `plugins/vsdd-factory/agents/dtu-validator.md` |
| Stories | TBD |

## Related BCs (Recommended)

- BC-5.09.002 — composes with (fidelity thresholds)

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
| **Path** | `plugins/vsdd-factory/agents/dtu-validator.md:56, 96-101` |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |

#### Evidence Types Used

- documentation: explicit drift detection rule

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | writes (drift-report) |
| **Global state access** | reads prior baseline |
| **Deterministic** | yes |
| **Thread safety** | unknown |
| **Overall classification** | effectful shell |

#### Refactoring Notes

No refactoring needed.
