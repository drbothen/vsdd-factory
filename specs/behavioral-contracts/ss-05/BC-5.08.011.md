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
extracted_from: .factory/phase-0-ingestion/pass-3-deep-agents.md:651
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

# Behavioral Contract BC-5.08.011: github-ops: returns full stdout + stderr unmodified

## Description

Every github-ops response MUST contain the complete stdout + stderr from the
executed `gh` command. Truncation, error suppression, or summarization is forbidden.

## Preconditions

1. github-ops executed a gh command.

## Postconditions

1. Response payload byte length ≥ length of `gh` command output (modulo agent message wrapping).
2. stderr present when command failed.

## Invariants

1. Output fidelity is mandatory.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Output very large | Return full output (no truncation) |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Command success | Full stdout returned | happy-path |
| Command failure | Full stderr returned | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-TBD | Response contains complete stdout + stderr | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD |
| L2 Domain Invariants | DI-TBD |
| Architecture Module | `plugins/vsdd-factory/agents/github-ops.md` |
| Stories | TBD |

## Related BCs (Recommended)

- BC-5.08.010 — composes with (executes only)

## Architecture Anchors (Recommended)

- `architecture/SS-05-orchestration.md#github-ops`

## Story Anchor (Recommended)

TBD

## VP Anchors (Recommended)

- VP-TBD

---

### Brownfield-Specific Sections

#### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `plugins/vsdd-factory/agents/github-ops.md:38, 88, 110` |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |

#### Evidence Types Used

- documentation: explicit full-output rule

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | shell + return-as-text |
| **Global state access** | none |
| **Deterministic** | yes |
| **Thread safety** | unknown |
| **Overall classification** | effectful shell |

#### Refactoring Notes

No refactoring needed.
