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
extracted_from: "plugins/vsdd-factory/workflows/phases/phase-1-spec-crystallization.lobster"
subsystem: "SS-05"
capability: "CAP-071"
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

# Behavioral Contract BC-5.21.017: phase-1:adversarial-spec-review

## Description

Each iteration spawns adversary agent (model_tier: adversary) on `.factory/specs/**` and `module-criticality.md`, excluding `holdout-scenarios/**` (information-asymmetry wall). Adversary writes findings to `adversarial-spec-review.md`; `phase-1d-adversarial-spec-review` skill fixes findings. Exits on convergence.

## Preconditions

1. Upstream dependencies completed successfully: `[spec-gate]`.

## Postconditions

1. Each iteration spawns adversary agent (model_tier: adversary) on `.factory/specs/**` and `module-criticality.md`, excluding `holdout-scenarios/**` (information-asymmetry wall).

## Invariants

1. Step does not modify upstream artifacts; only emits its declared outputs.

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
| Workflow YAML loads with this step defined | Loader emits step record matching declared fields | happy-path |
| Upstream dependency completed | Step is invoked with its declared inputs | happy-path |
| Step exceeds its timeout | Workflow engine handles per `on_failure: escalate` (default) | edge-case |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-001 | Workflow YAML field shape matches declared schema for ``adversarial-spec-review` (line 108)` | manual |
| VP-002 | Topological sort of all `depends_on` references resolves without cycles | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-071 |
| L2 Domain Invariants | TBD |
| Architecture Module | SS-05 Pipeline Orchestration (plugins/vsdd-factory/workflows/phases/phase-1-spec-crystallization.lobster) |
| Stories | TBD |
| Source BC-AUDIT ID | BC-AUDIT-1336 |

## Related BCs (Recommended)

- TBD — sibling step contracts in section BC-5.21

## Architecture Anchors (Recommended)

- `architecture/ss-05-pipeline-orchestration.md#workflow-phase-1-spec-crystallization` — workflow declaration source

## Story Anchor (Recommended)

TBD

## VP Anchors (Recommended)

- [VP-001] — Workflow YAML field shape matches declared schema for ``adversarial-spec-review` (line 108)`
- [VP-002] — Topological sort of all `depends_on` references resolves without cycles

---

### Brownfield-Specific Sections

> Extracted from existing workflow YAML during Phase 0d brownfield ingest.

#### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `plugins/vsdd-factory/workflows/phases/phase-1-spec-crystallization.lobster` |
| **Source Document** | `.factory/phase-0-ingestion/pass-3-deep-workflows.md` (line 298) |
| **Source BC-AUDIT ID** | `BC-AUDIT-1336` |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |

**Extracted Fields:**

- **Step:** `adversarial-spec-review` (line 108)
- **Type:** loop
- **Depends on:** `[spec-gate]`
- **Source line(s) in workflow YAML:** 108-135
- **Behavior:** Each iteration spawns adversary agent (model_tier: adversary) on `.factory/specs/**` and `module-criticality.md`, excluding `holdout-scenarios/**` (information-asymmetry wall). Adversary writes findings to `adversarial-spec-review.md`; `phase-1d-adversarial-spec-review` skill fixes findings. Exits on convergence.

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
