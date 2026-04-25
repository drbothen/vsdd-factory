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
extracted_from: .factory/phase-0-ingestion/pass-3-deep-agents.md:659
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

# Behavioral Contract BC-5.08.012: github-ops: retry once on transient errors, then report

## Description

Retry policy is exactly one retry on transient network errors. Persistent
failures (auth, rate limit, command-level errors) are reported with full error
output, not retried.

## Preconditions

1. github-ops experiences a command failure.

## Postconditions

1. Agent log shows ≤1 retry per command.
2. Auth/rate-limit failures yield immediate error reports including reset timestamp.

## Invariants

1. Retry budget is exactly 1; non-transient failures are reported, not retried.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Transient network blip | One retry |
| EC-002 | Auth failure | Immediate report (no retry) |
| EC-003 | Rate-limit | Immediate report with reset timestamp |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Transient error → retry succeeds | Single retry executed | happy-path |
| Auth error | No retry; immediate error report | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-TBD | ≤1 retry per command in github-ops logs | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD |
| L2 Domain Invariants | DI-TBD |
| Architecture Module | `plugins/vsdd-factory/agents/github-ops.md` |
| Stories | TBD |

## Related BCs (Recommended)

- BC-5.08.011 — composes with (full output)

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
| **Path** | `plugins/vsdd-factory/agents/github-ops.md:93-95, 109` |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |

#### Evidence Types Used

- documentation: explicit Failure & Escalation section

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | shell (gh with retry) |
| **Global state access** | none |
| **Deterministic** | depends on remote |
| **Thread safety** | unknown |
| **Overall classification** | effectful shell |

#### Refactoring Notes

No refactoring needed.
