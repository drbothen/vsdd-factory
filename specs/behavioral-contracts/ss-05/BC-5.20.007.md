---
document_type: behavioral-contract
level: L3
version: "1.3"
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
extracted_from: "plugins/vsdd-factory/workflows/phases/phase-0-codebase-ingestion.lobster"
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

# Behavioral Contract BC-5.20.007: phase-0:backup-source-acquisition

## Description

Commits source-acquisition artifacts to factory-artifacts; updates `STATE.md` with `phase: 0, step: source-acquisition, status: complete`. STATE.md reflects step completion; factory-artifacts branch contains the artefacts.

## Preconditions

1. Upstream dependencies completed successfully: `[source-acquisition]` | timeouts/retries: defaults.

## Postconditions

1. Commits source-acquisition artifacts to factory-artifacts; updates `STATE.md` with `phase: 0, step: source-acquisition, status: complete`.
2. Acceptance: STATE.md reflects step completion; factory-artifacts branch contains the artefacts.

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
| (TBD — to be assigned in Phase 1.6c) | Workflow YAML field shape matches declared schema for ``backup-source-acquisition` (line 23; lobster carve-out: stable anchor is step name `backup-source-acquisition`, not line number)` | manual |
| VP-002 | Topological sort of all `depends_on` references resolves without cycles | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-001 |
| L2 Domain Invariants | TBD |
| Architecture Module | SS-05 Pipeline Orchestration (plugins/vsdd-factory/workflows/phases/phase-0-codebase-ingestion.lobster) |
| Stories | TBD |
| Source BC-AUDIT ID | BC-AUDIT-1306 |

## Related BCs (Recommended)

- TBD — sibling step contracts in section BC-5.20

## Architecture Anchors (Recommended)

- `architecture/ss-05-pipeline-orchestration.md#workflow-phase-0-codebase-ingestion` — workflow declaration source

## Story Anchor (Recommended)

TBD

## VP Anchors (Recommended)

- [TBD — to be assigned in Phase 1.6c] — Workflow YAML field shape matches declared schema for ``backup-source-acquisition` (line 23; lobster carve-out: stable anchor is step name `backup-source-acquisition`, not line number)`
- [VP-002] — Topological sort of all `depends_on` references resolves without cycles

---

### Brownfield-Specific Sections

> Extracted from existing workflow YAML during Phase 0d brownfield ingest.

#### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `plugins/vsdd-factory/workflows/phases/phase-0-codebase-ingestion.lobster` |
| **Source Document** | `.factory/phase-0-ingestion/pass-3-deep-workflows.md` (line 99; source-doc carve-out: line in phase-0 ingestion doc, not lobster step line) |
| **Source BC-AUDIT ID** | `BC-AUDIT-1306` |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |

**Extracted Fields:**

- **Step:** `backup-source-acquisition` (line 23; lobster carve-out: stable anchor is step name `backup-source-acquisition`, not line number)
- **Type:** agent
- **Agent:** state-manager
- **Depends on:** `[source-acquisition]` | timeouts/retries: defaults
- **Source line(s) in workflow YAML:** 23-29
- **Behavior:** Commits source-acquisition artifacts to factory-artifacts; updates `STATE.md` with `phase: 0, step: source-acquisition, status: complete`.
- **Acceptance:** STATE.md reflects step completion; factory-artifacts branch contains the artefacts.

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

**Driver:** F-P23-001 pass-23 retroactive corpus-wide sweep (per L-P19-001 / L-P20-001 / L-P22-001) — lobster step cited by line number (`backup-source-acquisition (line 23)`). Stable anchor is step name `backup-source-acquisition`.

**Changes made:**
- §Postconditions/§VP/§Step: inline lobster carve-out annotation added.
- §Source Document: source-doc line carve-out annotation added.
- Frontmatter `version:` incremented. Changelog entry added.


---

## Amendment 2026-05-08 (v→ F-P23-001: lobster-line-cite annotated with carve-out)

**Driver:** F-P23-001 pass-23 retroactive corpus-wide sweep (per L-P19-001 / L-P20-001 / L-P22-001) — double-backtick VP/anchor: step `backup-source-acquisition` (line 23); double-backtick VP/anchor: step `backup-source-acquisition` (line 23); **Step:** bt form: step `backup-source-acquisition` (line 23)

**Changes made:**
- Inline lobster carve-out annotation added to all active-body line cites.
- Frontmatter `version:` incremented. Changelog entry added.

## Changelog

| Version | Date | Author | Change |
|---------|------|--------|--------|
| v1.3 | 2026-05-08 | state-manager | F-P23-001 corpus-wide sweep: lobster-line-cite annotated with carve-out per L-P19-001 + L-P20-001 + L-P22-001. |
| v1.2 | 2026-05-08 | state-manager | F-P23-001 corpus-wide sweep: lobster-line-cite annotated with carve-out. Stable anchor is step name `backup-source-acquisition`. |
