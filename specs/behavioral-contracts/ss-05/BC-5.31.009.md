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
extracted_from: "plugins/vsdd-factory/workflows/code-delivery.lobster"
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

# Behavioral Contract BC-5.31.009: code-delivery:red-gate

## Description

Step `red-gate` (line 73). Type: gate. Depends: `[write-tests]`. fail_action: block. Source 73-80. Verifies tests compile AND all tests fail before implementation may proceed.

## Preconditions

1. write-tests has completed.
2. Build/test runner is available.

## Postconditions

1. If pass condition met (compile-OK + all-fail), gate passes and downstream proceeds.
2. Otherwise, gate blocks downstream (fail_action: block).

## Invariants

1. Gate semantics are pure: same input → same gate decision.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Tests fail to compile | Gate blocks |
| EC-002 | Some tests pass | Gate blocks (Red required) |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| All tests fail, compile OK | Gate passes | happy-path |
| Compile error | Gate blocks | error |
| Mixed pass/fail | Gate blocks | edge-case |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-001 | Gate blocks unless tests compile and all fail | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD |
| Architecture Module | SS-05 |
| Stories | TBD |

## Related BCs (Recommended)

- BC-5.31.008 — write-tests (depends on)
- BC-5.31.010 — implement (downstream)

## Architecture Anchors (Recommended)

- `architecture/ss-05-orchestration.md#code-delivery-workflow`

## Story Anchor (Recommended)

TBD

## VP Anchors (Recommended)

- VP-001

---

### Brownfield-Specific Sections

#### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `plugins/vsdd-factory/workflows/code-delivery.lobster` (lines 73-80) |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |

#### Evidence Types Used

- type constraint: gate type with fail_action: block
- documentation

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | reads (test runner output) |
| **Global state access** | reads test results |
| **Deterministic** | yes (given test outcomes) |
| **Thread safety** | N/A |
| **Overall classification** | pure (decision) |

#### Refactoring Notes

No refactoring needed.
