---
document_type: behavioral-contract
level: L3
version: "1.1"
status: draft
producer: codebase-analyzer
timestamp: 2026-04-25T00:00:00
phase: 1.4b
inputs: [.factory/phase-0-ingestion/pass-3-behavioral-contracts-deep-r1.md]
input-hash: "a022087"
traces_to: .factory/specs/architecture/ARCH-INDEX.md
origin: brownfield
extracted_from: .factory/phase-0-ingestion/pass-3-behavioral-contracts-deep-r1.md:320
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

# Behavioral Contract BC-5.01.008: `human-approval` steps declare `approval: { prompt, artifacts, timeout }`

## Description

Steps with `type: human-approval` declare an `approval:` block containing a
folded-scalar `prompt:` shown to the operator, an `artifacts:` list of file globs
to review, and a `timeout:` (typically `"24h"`, `"48h"`, or `"72h"`). The
orchestrator pauses the workflow and resumes only after explicit operator approval
within the timeout.

## Preconditions

1. Step has `type: human-approval`.

## Postconditions

1. `approval.prompt` is a folded-scalar string shown to the operator.
2. `approval.artifacts` is a list of file globs the operator should review.
3. `approval.timeout` is set (typical values: `"24h"`, `"48h"`, `"72h"`).
4. Orchestrator pauses the workflow until explicit operator approval.
5. Workflow resumes only if approval is received within the timeout.

## Invariants

1. All three approval keys (prompt, artifacts, timeout) are required.
2. Timeout is bounded — workflow does not pause indefinitely.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Operator approves within timeout | Workflow resumes |
| EC-002 | Operator does not approve before timeout | TBD — workflow fails or escalates |
| EC-003 | Operator rejects | TBD — workflow halts or branches |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| `greenfield.lobster:181-191` (design-system-approval) | Pause, await operator | happy-path |
| `greenfield.lobster:336-355` (phase-1-human-approval, 24h, 12 artifacts) | Pause with 12-file review list | happy-path |
| `greenfield.lobster:1311-1322` (phase-6-human-approval, 48h timeout) | Pause for 48h | happy-path |
| `brownfield.lobster:170-188` (phase-0-human-approval, 24h, 9 artifacts) | Pause for ingestion approval | happy-path |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-TBD | Every `human-approval` step has all three approval keys | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD |
| L2 Domain Invariants | DI-TBD |
| Architecture Module | orchestrator human-approval pause/resume logic |
| Stories | TBD |

## Related BCs (Recommended)

- BC-5.01.003 — composes with (step taxonomy: human-approval)
- BC-5.02.007 — composes with (orchestrator input-hash drift check before human approval)

## Architecture Anchors (Recommended)

- `architecture/SS-05-orchestration.md#human-approval`

## Story Anchor (Recommended)

TBD

## VP Anchors (Recommended)

- VP-TBD

---

### Brownfield-Specific Sections

#### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `greenfield.lobster:181-191, 201-211, 336-355, 1311-1322`; `brownfield.lobster:170-188` |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |

#### Evidence Types Used

- documentation: explicit human-approval blocks in workflow YAML

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | reads + writes (operator notification + approval state) |
| **Global state access** | reads/writes pause/resume state |
| **Deterministic** | no (depends on human + clock) |
| **Thread safety** | unknown |
| **Overall classification** | effectful shell |

#### Refactoring Notes

The schema is pure data; the pause/resume mechanism is effectful and lives in the orchestrator shell.
