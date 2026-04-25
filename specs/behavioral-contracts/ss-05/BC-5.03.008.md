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
extracted_from: .factory/phase-0-ingestion/pass-3-deep-agents.md:399
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

# Behavioral Contract BC-5.03.008: demo-recorder: both success AND error paths recorded per AC

## Description

Each AC must have at least two recordings — one for the success path, one for
the error path. Recording only the happy path is insufficient.

## Preconditions

1. demo-recorder is dispatched against a story with one or more ACs.

## Postconditions

1. evidence-report.md table shows ≥2 entries per AC.
2. Each AC entry distinguishes success from error path.

## Invariants

1. Single-path AC coverage is rejected.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | AC has no error path conceptually | TBD — explicit "no error path" justification required |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| AC-001 with both success and error demo | Accepted | happy-path |
| AC-001 with only success demo | Rejected | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-TBD | Every AC has ≥2 demo entries (success + error) | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD |
| L2 Domain Invariants | DI-TBD |
| Architecture Module | `plugins/vsdd-factory/agents/demo-recorder.md` |
| Stories | TBD |

## Related BCs (Recommended)

- BC-5.03.006 — composes with (output destination)
- BC-5.03.009 — composes with (AC-NNN naming)

## Architecture Anchors (Recommended)

- `architecture/SS-05-orchestration.md#demo-recorder`

## Story Anchor (Recommended)

TBD

## VP Anchors (Recommended)

- VP-TBD

---

### Brownfield-Specific Sections

#### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `plugins/vsdd-factory/agents/demo-recorder.md:36, 40, 74` |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |

#### Evidence Types Used

- documentation: explicit success+error rule

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | writes (recordings + report) |
| **Global state access** | none |
| **Deterministic** | yes |
| **Thread safety** | unknown |
| **Overall classification** | effectful shell |

#### Refactoring Notes

No refactoring needed.
