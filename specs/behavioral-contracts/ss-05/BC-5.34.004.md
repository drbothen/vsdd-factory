---
document_type: behavioral-contract
level: L3
version: "1.2"
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

# Behavioral Contract BC-5.34.004: multi-repo: DAG integrity (primary track)

## Description

`multi-repo.lobster` defines 41 acyclic top-level steps on the primary track. Three additional sub-mode trees defined (feature_mode, bugfix_mode, maintenance_mode) in `multi-repo.lobster` § sub-mode workflow trees — line range `575-731` cited intentionally as point-in-time evidence pending lobster section-stability verification — alternative entry trees for orchestrator dispatch when `mode=feature/bug-fix/maintenance` and `project_type=multi-repo`. <!-- F-P21-001: lobster-line-class deferred per pass-21 carve-out; sibling of BC-5.34.003 v1.2 fix; line range preserved as source evidence --> Primary sequence: environment-setup → read-project-manifest → compute-repo-waves + per-repo-mode-detection → per-repo-setup → state-init → configure-workspaces → conditional per-repo-phase-0 (parallel-foreach) → project-level-synthesis → project-phase-0-gate → post-phase-0-routing → market-intelligence → market-intel-review → wave-0-spec (parallel-foreach) → wave-0-spec-approval → wave-0-impl (parallel-foreach) → wave-0-state-commit → contract-change-detection → wave-1-consumers + wave-1-sdk-gen + sdk-regeneration + sdk-validation → wave-1-state-commit → cross-repo-docker-env → 6 parallel cross-repo gates (e2e, holdout, adversary, security, a11y, pr-review) → integration-gate → integration-gate-state-commit → cross-repo-convergence → convergence-human-approval → coordinated-release → state-final → session-review → session-review-approval → process-review-decisions.

## Preconditions

1. Workflow file is parsed by lobster loader.

## Postconditions

1. Topological sort succeeds on primary track.
2. Six parallel cross-repo gates fan-in to integration-gate.

## Invariants

1. Primary track has 41 acyclic steps.
2. Three alternative entry trees (feature/bugfix/maintenance modes) coexist without overlapping primary track edges.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Mode=feature/bug-fix/maintenance | Alternative entry tree selected |
| EC-002 | Cycle hypothetical | Loader rejects |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Primary path | 41 steps | happy-path |
| Sub-mode | Alternative tree taken | edge-case |
| Cycle | Rejected | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| (TBD — to be assigned in Phase 1.6c) | DAG acyclic | manual / lobster-parse |
| VP-002 | 6 cross-repo gates fan-in to integration-gate | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD |
| Architecture Module | SS-05 |
| Stories | TBD |

## Related BCs (Recommended)

- BC-5.34.001 — identity

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
| **Path** | `plugins/vsdd-factory/workflows/multi-repo.lobster` |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |

#### Evidence Types Used

- documentation: declarative workflow

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | none (structural) |
| **Global state access** | none |
| **Deterministic** | yes |
| **Thread safety** | N/A |
| **Overall classification** | pure |

#### Refactoring Notes

No refactoring needed.

## Amendment 2026-05-08 (v1.1 → v1.2 — F-P21-001: §Description lobster line-range annotated with carve-out deferral)

**Driver:** F-P21-001 pass-21 retroactive sibling sweep — §Description cited `at lines 575-731` referencing `multi-repo.lobster` sub-mode workflow trees. This is a lobster-file reference and falls under the lobster-line-class carve-out exception (separate migration class pending lobster section-stability verification). BC-5.34.003 v1.2 fixed the identical reference in its §Invariants; BC-5.34.004 is the unfixed sibling.

**Change made:**
- §Description: line range `575-731` preserved as point-in-time evidence; file name `multi-repo.lobster` and logical section description (sub-mode workflow trees: `feature_mode`, `bugfix_mode`, `maintenance_mode`) made explicit for reader navigability; HTML comment added citing F-P21-001 deferral.
- Frontmatter `version:` bumped `"1.1"` → `"1.2"`.

**Source-of-truth verification (POLICY 4/5):** `grep -n "feature_mode\|bugfix_mode\|maintenance_mode" plugins/vsdd-factory/workflows/multi-repo.lobster` → lines 575, 654, 696 confirmed (same as BC-5.34.003 v1.2 audit). Line range 575-731 spans all three sub-mode tree definitions.
