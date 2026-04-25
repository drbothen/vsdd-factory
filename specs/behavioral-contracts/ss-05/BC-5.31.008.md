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

# Behavioral Contract BC-5.31.008: code-delivery:write-tests

## Description

Step `write-tests` (line 64). Type: agent. Agent: test-writer. Depends: `[generate-stubs]`. Source 64-72. Behavior: writes failing tests that compile but all fail (Red Gate).

## Preconditions

1. Stubs exist and build is green.
2. Story acceptance criteria are documented.

## Postconditions

1. New test cases compile.
2. All newly-written tests fail when executed (Red state).

## Invariants

1. Tests are authored before implementation logic is added.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Test compilation error | Step fails |
| EC-002 | Some tests accidentally pass | Treated as Red Gate violation downstream |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Complete spec | Failing tests authored | happy-path |
| Spec gap | test-writer flags missing acceptance criteria | edge-case |
| Compile error | Step fails | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-001 | All new tests fail at completion of write-tests | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD |
| Architecture Module | SS-05 |
| Stories | TBD |

## Related BCs (Recommended)

- BC-5.31.007 — generate-stubs (depends on)
- BC-5.31.009 — red-gate (downstream)

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
| **Path** | `plugins/vsdd-factory/workflows/code-delivery.lobster` (lines 64-72) |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |

#### Evidence Types Used

- documentation: declarative step + behavior comment

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | reads + writes (test files) |
| **Global state access** | reads filesystem |
| **Deterministic** | no |
| **Thread safety** | unknown |
| **Overall classification** | effectful shell |

#### Refactoring Notes

No refactoring needed.
