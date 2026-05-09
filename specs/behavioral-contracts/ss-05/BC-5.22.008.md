---
document_type: behavioral-contract
level: L3
version: "1.2"
last_amended: 2026-05-08
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
extracted_from: "plugins/vsdd-factory/workflows/phases/phase-2-story-decomposition.lobster"
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

# Behavioral Contract BC-5.22.008: phase-2:create-stories

## Description

Workflow contract: phase-2:create-stories.

## Preconditions

1. Upstream dependencies completed successfully: `[backup-define-epics]`.

## Postconditions

1. Step `create-stories (line 33; lobster carve-out: stable anchor is step name `create-stories`, not line number)` has run to completion and emitted any declared artifacts.
<!-- F-P23-001: lobster-line-cite annotated per pass-23 carve-out; stable anchor is step name `create-stories`; line number is unstable as lobster files evolve -->

## Invariants

1. Step execution must complete within timeout: 1h.
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
| (TBD — to be assigned in Phase 1.6c) | Workflow YAML field shape matches declared schema for `create-stories (line 33; lobster carve-out: stable anchor is step name `create-stories`, not line number)` | manual |
| VP-002 | Topological sort of all `depends_on` references resolves without cycles | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-001 |
| L2 Domain Invariants | TBD |
| Architecture Module | SS-05 Pipeline Orchestration (plugins/vsdd-factory/workflows/phases/phase-2-story-decomposition.lobster) |
| Stories | TBD |
| Source BC-AUDIT ID | BC-AUDIT-1347 |

## Related BCs (Recommended)

- TBD — sibling step contracts in section BC-5.22

## Architecture Anchors (Recommended)

- `architecture/ss-05-pipeline-orchestration.md#workflow-phase-2-story-decomposition` — workflow declaration source

## Story Anchor (Recommended)

TBD

## VP Anchors (Recommended)

- [TBD — to be assigned in Phase 1.6c] — Workflow YAML field shape matches declared schema for `create-stories (line 33; lobster carve-out: stable anchor is step name `create-stories`, not line number)`
- [VP-002] — Topological sort of all `depends_on` references resolves without cycles

---

### Brownfield-Specific Sections

> Extracted from existing workflow YAML during Phase 0d brownfield ingest.

#### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `plugins/vsdd-factory/workflows/phases/phase-2-story-decomposition.lobster` |
| **Source Document** | `.factory/phase-0-ingestion/pass-3-deep-workflows.md` (line 353; source-doc carve-out: line in phase-0 ingestion doc, not lobster step line) |
| **Source BC-AUDIT ID** | `BC-AUDIT-1347` |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |

**Extracted Fields:**

- **Step:** create-stories (line 33; lobster carve-out: stable anchor is step name `create-stories`, not line number)
- **Type:** skill
- **Skill:** `decompose-stories/steps/step-b-create-stories.md`
- **Depends on:** `[backup-define-epics]`
- **Timeout:** 1h
- **Source line(s) in workflow YAML:** 33-37

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


---

## Amendment 2026-05-08 (v→ F-P23-001: lobster-line-cite annotated with carve-out)

**Driver:** F-P23-001 pass-23 retroactive corpus-wide sweep (per L-P19-001 / L-P20-001 / L-P22-001) — lobster step cited by line number (`create-stories (line 33)`). Stable anchor is step name `create-stories`.

**Changes made:**
- §Postconditions/§VP/§Step: inline lobster carve-out annotation added.
- §Source Document: source-doc line carve-out annotation added.
- Frontmatter `version:` incremented. Changelog entry added.

## Changelog

| Version | Date | Author | Change |
|---------|------|--------|--------|
| v1.2 | 2026-05-08 | state-manager | F-P23-001 corpus-wide sweep: lobster-line-cite annotated with carve-out. Stable anchor is step name `create-stories`. |
