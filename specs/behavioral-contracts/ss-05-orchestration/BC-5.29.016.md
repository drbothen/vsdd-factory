---
document_type: behavioral-contract
level: L3
version: "1.1"
status: draft
producer: "phase-1-4b-bcs-agent-4"
timestamp: 2026-04-25T00:00:00
phase: 1.4b
inputs:
  - .factory/specs/behavioral-contracts/bc-id-mapping.md
  - .factory/phase-0-ingestion/pass-3-deep-workflows.md
  - .factory/specs/architecture/ARCH-INDEX.md
input-hash: "b115391"
traces_to: .factory/specs/architecture/ARCH-INDEX.md#ss-05-pipeline-orchestration
origin: brownfield
extracted_from: "plugins/vsdd-factory/workflows/brownfield.lobster"
subsystem: "SS-05"
capability: "CAP-079"
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

# Behavioral Contract BC-5.29.016: brownfield:phase-0-human-approval

## Description

Workflow contract: brownfield:phase-0-human-approval.

## Preconditions

1. Upstream dependencies completed successfully: `[phase-0-gate]`.

## Postconditions

1. Step `phase-0-human-approval (line 169)` has run to completion and emitted any declared artifacts.

## Invariants

1. Step execution must complete within timeout: 24h.
2. Step does not modify upstream artifacts; only emits its declared outputs.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Step times out before completion | Workflow engine raises timeout escalation per `on_failure` policy. |
| EC-002 | Upstream dependency emits no artifact | Step receives empty input; behavior depends on step type (skill aborts; gate fails; agent escalates). |

## Canonical Test Vectors

> Golden-file test inputs and expected outputs. Used for regression testing
> and agent validation. Include at minimum: one happy-path, one edge-case,
> one error-case vector.

| Input | Expected Output | Category |
|-------|----------------|----------|
| Upstream dependency completed | Step is invoked with its declared inputs | happy-path |
| Step exceeds its timeout | Workflow engine handles per `on_failure: escalate` (default) | edge-case |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-001 | Workflow YAML field shape matches declared schema for `phase-0-human-approval (line 169)` | manual |
| VP-002 | Topological sort of all `depends_on` references resolves without cycles | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-079 |
| L2 Domain Invariants | TBD |
| Architecture Module | SS-05 Pipeline Orchestration (plugins/vsdd-factory/workflows/brownfield.lobster) |
| Stories | TBD |
| Source BC-AUDIT ID | BC-AUDIT-1555 |

## Related BCs (Recommended)

- TBD — sibling step contracts in section BC-5.29

## Architecture Anchors (Recommended)

- `architecture/ss-05-pipeline-orchestration.md#workflow-brownfield` — workflow declaration source

## Story Anchor (Recommended)

TBD

## VP Anchors (Recommended)

- [VP-001] — Workflow YAML field shape matches declared schema for `phase-0-human-approval (line 169)`
- [VP-002] — Topological sort of all `depends_on` references resolves without cycles

---

### Brownfield-Specific Sections

> Extracted from existing workflow YAML during Phase 0d brownfield ingest.

#### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `plugins/vsdd-factory/workflows/brownfield.lobster` |
| **Source Document** | `.factory/phase-0-ingestion/pass-3-deep-workflows.md` (line 974) |
| **Source BC-AUDIT ID** | `BC-AUDIT-1555` |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |

**Extracted Fields:**

- **Step:** phase-0-human-approval (line 169)
- **Type:** human-approval
- **Depends on:** `[phase-0-gate]`
- **Timeout:** 24h
- **Source line(s) in workflow YAML:** 169-188

#### Evidence Types Used

- **documentation**: stated in workflow YAML declarations (lobster file)
- **inferred**: workflow engine semantics applied per declared fields

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | reads + writes (orchestrates skill/agent invocations and STATE.md updates) |
| **Global state access** | reads + writes STATE.md, .factory/cycles/**, factory-artifacts branch |
| **Deterministic** | yes given fixed workflow YAML; downstream skills may introduce nondeterminism |
| **Thread safety** | not thread-safe (single workflow run per cycle) |
| **Overall classification** | effectful shell |

#### Refactoring Notes

Workflow YAML is a declarative DAG; the orchestrator is the effectful shell. Pure-core extraction would isolate the lobster parser and topological sort logic from the orchestration loop.
