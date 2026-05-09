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
extracted_from: "plugins/vsdd-factory/workflows/phases/phase-1-spec-crystallization.lobster"
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

# Behavioral Contract BC-5.21.009: phase-1:backup-create-domain-spec

## Description

Workflow contract: phase-1:backup-create-domain-spec.

## Preconditions

1. Workflow loader has parsed the lobster YAML and resolved this contract.

## Postconditions

1. Step ``backup-create-domain-spec` (line 38; lobster carve-out: stable anchor is step name `backup-create-domain-spec`, not line number)` has run to completion and emitted any declared artifacts.
<!-- F-P23-001: lobster-line-cite annotated per pass-23 carve-out; stable anchor is step name `backup-create-domain-spec`; line number is unstable as lobster files evolve -->

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
| Upstream dependency completed | Step is invoked with its declared inputs | happy-path |
| Step exceeds its timeout | Workflow engine handles per `on_failure: escalate` (default) | edge-case |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| (TBD — to be assigned in Phase 1.6c) | Workflow YAML field shape matches declared schema for ``backup-create-domain-spec` (line 38; lobster carve-out: stable anchor is step name `backup-create-domain-spec`, not line number)` | manual |
| VP-002 | Topological sort of all `depends_on` references resolves without cycles | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-001 |
| L2 Domain Invariants | TBD |
| Architecture Module | SS-05 Pipeline Orchestration (plugins/vsdd-factory/workflows/phases/phase-1-spec-crystallization.lobster) |
| Stories | TBD |
| Source BC-AUDIT ID | BC-AUDIT-1328 |

## Related BCs (Recommended)

- TBD — sibling step contracts in section BC-5.21

## Architecture Anchors (Recommended)

- `architecture/ss-05-pipeline-orchestration.md#workflow-phase-1-spec-crystallization` — workflow declaration source

## Story Anchor (Recommended)

TBD

## VP Anchors (Recommended)

- [TBD — to be assigned in Phase 1.6c] — Workflow YAML field shape matches declared schema for ``backup-create-domain-spec` (line 38; lobster carve-out: stable anchor is step name `backup-create-domain-spec`, not line number)`
- [VP-002] — Topological sort of all `depends_on` references resolves without cycles

---

### Brownfield-Specific Sections

> Extracted from existing workflow YAML during Phase 0d brownfield ingest.

#### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `plugins/vsdd-factory/workflows/phases/phase-1-spec-crystallization.lobster` |
| **Source Document** | `.factory/phase-0-ingestion/pass-3-deep-workflows.md` (line 250; source-doc carve-out: line in phase-0 ingestion doc, not lobster step line) |
| **Source BC-AUDIT ID** | `BC-AUDIT-1328` |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |

**Extracted Fields:**

- **Step:** `backup-create-domain-spec` (line 38; lobster carve-out: stable anchor is step name `backup-create-domain-spec`, not line number)
- **Agent:** state-manager
- **Source line(s) in workflow YAML:** 38-44

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

## Amendment 2026-05-08 (v1.1 → v1.2 — F-P23-001: postcondition-form lobster-line-cite annotated with carve-out)

**Driver:** F-P23-001 pass-23 retroactive corpus-wide sweep (per L-P19-001 / L-P20-001 / L-P22-001 semantic-pattern-class discipline with FULL syntactic variant scope) — §Postconditions cited lobster step by line number using double-backtick form (`` ``backup-create-domain-spec` (line 38)` ``). This is a lobster-file reference and falls under the lobster-line-cite carve-out exception; the stable anchor is the step name `backup-create-domain-spec`, not the line number. Pass-23 adversary identified this as the sibling-class variant missed by F-P22-001's single-backtick-description-form sweep.

**Changes made:**
- §Postconditions: inline annotation added noting lobster carve-out (stable anchor = step name `backup-create-domain-spec`); HTML carve-out comment added citing F-P23-001 deferral.
- §Verification Properties: inline annotation added to VP table property cell.
- §VP Anchors: inline annotation added to VP anchor list item.
- §Source Evidence `**Step:**` field: inline annotation added noting lobster carve-out.
- §Source Evidence `**Source Document**` field: inline annotation added noting source-doc line carve-out (ingestion-doc line, not lobster step line).
- Frontmatter `version:` bumped `"1.1"` → `"1.2"`.
- Changelog entry added: F-P23-001 corpus-wide sweep, L-P19-001 + L-P20-001 + L-P22-001 applied with FULL syntactic variant and cross-subsystem scope.

## Changelog

| Version | Date | Author | Change |
|---------|------|--------|--------|
| v1.2 | 2026-05-08 | state-manager | F-P23-001 corpus-wide sweep: postcondition-form lobster-line-cite (double-backtick variant) annotated with carve-out per L-P19-001 + L-P20-001 + L-P22-001. Stable anchor is step name `backup-create-domain-spec`. |
