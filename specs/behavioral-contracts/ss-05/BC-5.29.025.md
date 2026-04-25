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
capability: "CAP-001"
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

# Behavioral Contract BC-5.29.025: brownfield:brownfield-to-greenfield-transition

## Description

Aggregates Phase 0 outputs + market intel + design system into Phase 1 context. Notable: uses `wait_for_optional` field (synchronization barrier for conditional optional predecessors).

## Preconditions

1. Upstream dependencies completed successfully: `[brownfield-market-review]`.
2. Conditional gate evaluates true: `routing.choice == 'feature'`.

## Postconditions

1. Aggregates Phase 0 outputs + market intel + design system into Phase 1 context.

## Invariants

1. Step does not modify upstream artifacts; only emits its declared outputs.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Step times out before completion | Workflow engine raises timeout escalation per `on_failure` policy. |
| EC-002 | Upstream dependency emits no artifact | Step receives empty input; behavior depends on step type (skill aborts; gate fails; agent escalates). |
| EC-003 | Condition `routing.choice == 'feature'` is false | Step is skipped; downstream `depends_on` consumers proceed without this step's outputs. |

## Canonical Test Vectors

> Golden-file test inputs and expected outputs. Used for regression testing
> and agent validation. Include at minimum: one happy-path, one edge-case,
> one error-case vector.

| Input | Expected Output | Category |
|-------|----------------|----------|
| Workflow YAML loads with this step defined | Loader emits step record matching declared fields | happy-path |
| Upstream dependency completed | Step is invoked with its declared inputs | happy-path |
| Step exceeds its timeout | Workflow engine handles per `on_failure: escalate` (default) | edge-case |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| (TBD — to be assigned in Phase 1.6c) | Workflow YAML field shape matches declared schema for `brownfield-to-greenfield-transition (line 301)` | manual |
| VP-002 | Topological sort of all `depends_on` references resolves without cycles | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-001 |
| L2 Domain Invariants | TBD |
| Architecture Module | SS-05 Pipeline Orchestration (plugins/vsdd-factory/workflows/brownfield.lobster) |
| Stories | TBD |
| Source BC-AUDIT ID | BC-AUDIT-1564 |

## Related BCs (Recommended)

- TBD — sibling step contracts in section BC-5.29

## Architecture Anchors (Recommended)

- `architecture/ss-05-pipeline-orchestration.md#workflow-brownfield` — workflow declaration source

## Story Anchor (Recommended)

TBD

## VP Anchors (Recommended)

- [TBD — to be assigned in Phase 1.6c] — Workflow YAML field shape matches declared schema for `brownfield-to-greenfield-transition (line 301)`
- [VP-002] — Topological sort of all `depends_on` references resolves without cycles

---

### Brownfield-Specific Sections

> Extracted from existing workflow YAML during Phase 0d brownfield ingest.

#### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `plugins/vsdd-factory/workflows/brownfield.lobster` |
| **Source Document** | `.factory/phase-0-ingestion/pass-3-deep-workflows.md` (line 1002) |
| **Source BC-AUDIT ID** | `BC-AUDIT-1564` |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |

**Extracted Fields:**

- **Step:** brownfield-to-greenfield-transition (line 301)
- **Type:** agent
- **Agent:** state-manager
- **Depends on:** `[brownfield-market-review]`
- **Condition:** `routing.choice == 'feature'`
- **Source line(s) in workflow YAML:** 301-313
- **Behavior:** Aggregates Phase 0 outputs + market intel + design system into Phase 1 context. Notable: uses `wait_for_optional` field (synchronization barrier for conditional optional predecessors).

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
