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
extracted_from: "plugins/vsdd-factory/workflows/maintenance.lobster"
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

# Behavioral Contract BC-5.33.005: maintenance: failure semantics

## Description

`maintenance.lobster` § workflow-defaults — lines `12-15`, `381` cited as point-in-time evidence pending lobster section-stability verification — declare workflow defaults: **`on_failure: skip`** (notable — the only mode workflow defaulting to skip, not escalate), retries 1, timeout 1h. `maintenance-gate.fail_action: warn` (advisory, not blocking). <!-- F-P21-001: lobster-line-class deferred per pass-21 retroactive sweep; line refs preserved as source evidence -->

## Preconditions

1. Workflow invoked under default failure config.

## Postconditions

1. Step failures retry once then skip (do not escalate).
2. maintenance-gate emits warnings rather than blocking.

## Invariants

1. Maintenance is the only mode workflow with `on_failure: skip`.
2. No `fail_action: block` in this workflow.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Single sweep crashes | Skip, downstream proceeds |
| EC-002 | Many sweep failures | Warnings accumulated; workflow still completes |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Healthy run | All sweeps complete | happy-path |
| One sweep crash | Skipped | edge-case |
| Gate failure | Warn, not block | edge-case |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| (TBD — to be assigned in Phase 1.6c) | on_failure default = skip | manual |
| VP-002 | maintenance-gate fail_action = warn | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD |
| Architecture Module | SS-05 |
| Stories | TBD |

## Related BCs (Recommended)

- BC-5.33.001 — identity
- BC-5.33.036 — maintenance-gate

## Architecture Anchors (Recommended)

- `architecture/ss-05-orchestration.md#maintenance-workflow`

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
| **Path** | `plugins/vsdd-factory/workflows/maintenance.lobster` (lines 12-15, 381) |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |

#### Evidence Types Used

- documentation: declarative defaults

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

## Amendment 2026-05-08 (v1.1 → v1.2 — F-P21-001: §Description lobster line-refs annotated with carve-out deferral)

**Driver:** F-P21-001 pass-21 retroactive sibling sweep (per L-P19-001 / L-P20-001 semantic-pattern-class discipline) — §Description cited `maintenance.lobster` workflow-defaults and maintenance-gate by line numbers (`lines 12-15`, `line 381`). These are lobster-file references and fall under the lobster-line-class carve-out exception pending lobster section-stability verification.

**Change made:**
- §Description: line refs preserved as point-in-time evidence; `maintenance.lobster` file name and logical sections (workflow-defaults, maintenance-gate) made explicit; HTML comment added citing F-P21-001 deferral.
- Frontmatter `version:` bumped `"1.1"` → `"1.2"`.
