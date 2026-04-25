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
extracted_from: "plugins/vsdd-factory/workflows/multi-repo.lobster"
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

# Behavioral Contract BC-5.34.009: multi-repo:compute-repo-waves

## Description

Step `compute-repo-waves` (line 79). Type: agent. Agent: orchestrator. Depends: `[read-project-manifest]`. Source 79-89. Behavior: Kahn's algorithm at repo level. Detects circular dependencies. Distinguishes contract dependencies from generation dependencies.

## Preconditions

1. read-project-manifest completed; per-repo dep graph available.

## Postconditions

1. Repos partitioned into waves via Kahn's algorithm.
2. Cycles detected and reported as workflow halt.
3. Contract deps and generation deps are tracked distinctly.

## Invariants

1. Repos in wave N depend only on repos in waves <N.
2. Cycles always halt the workflow.
3. The two dep types are not conflated.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Cyclic dep graph | Halt |
| EC-002 | Single-repo project | Single-wave output |
| EC-003 | Generation-only dep | Tracked separately from contract dep |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Linear chain | N waves of size 1 | happy-path |
| Independent repos | 1 wave with all repos | edge-case |
| Cycle | Halt | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| (TBD — to be assigned in Phase 1.6c) | Output is a valid topological partitioning | manual |
| VP-002 | Contract vs generation dep distinction preserved | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD |
| Architecture Module | SS-05 |
| Stories | TBD |

## Related BCs (Recommended)

- BC-5.34.008 — read-project-manifest

## Architecture Anchors (Recommended)

- `architecture/ss-05-orchestration.md#multi-repo-workflow`

## Story Anchor (Recommended)

TBD

## VP Anchors (Recommended)

- (TBD — to be assigned in Phase 1.6c)
- VP-002

---

### Brownfield-Specific Sections

#### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `plugins/vsdd-factory/workflows/multi-repo.lobster` (lines 79-89) |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |

#### Evidence Types Used

- documentation: explicit Kahn's algorithm + dep distinction

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | reads (manifests) |
| **Global state access** | reads filesystem |
| **Deterministic** | yes |
| **Thread safety** | N/A |
| **Overall classification** | pure (decision) |

#### Refactoring Notes

No refactoring needed.
