---
document_type: behavioral-contract
level: L3
version: "1.2"
last_amended: 2026-05-08
status: draft
producer: phase-1-4b-agent-5
timestamp: 2026-04-25T00:00:00
phase: 1.4b
inputs: [.factory/phase-0-ingestion/pass-3-deep-workflows.md]
input-hash: "99bbe9c"
traces_to: domain-spec/L2-INDEX.md
origin: brownfield
extracted_from: "plugins/vsdd-factory/workflows/discovery.lobster"
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

# Behavioral Contract BC-5.32.001: autonomous-discovery: identity

## Description

`discovery.lobster` v2.0.0 — autonomous discovery engine continuously researching opportunities (features for existing products, new product concepts). DF-034 adds customer-feedback-ingestion + competitive-monitoring + analytics-integration + intelligence-synthesis as dedicated skills, with `evidence_strength` as a 7th scoring dimension. Has scheduled trigger cadences (market_research weekly, feedback_ingestion daily, competitive_monitoring weekly, analytics weekly, full_synthesis weekly) and 5 STATE.md fields tracked per run.

## Preconditions

1. Discovery configuration exists (config.yaml or equivalent) describing products, channels, competitors, analytics.
2. Scheduled trigger fires at the documented cadence.

## Postconditions

1. Workflow runs end-to-end ingestion → synthesis → scoring → review → routing.
2. State for the 5 STATE.md fields is updated for each run.

## Invariants

1. evidence_strength is one of the seven scoring dimensions (DF-034).
2. All scheduled cadences are honored unless explicitly overridden.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Cadence overlap | Run-queue serializes |
| EC-002 | Missing config | Workflow halts with explicit error |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Weekly trigger fires | Full pipeline runs | happy-path |
| Daily feedback trigger | Only feedback ingestion runs | edge-case |
| No config | Fail | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| (TBD — to be assigned in Phase 1.6c) | All 7 scoring dimensions are referenced | manual |
| VP-002 | All 5 STATE.md fields updated each run | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD |
| Architecture Module | SS-05 Pipeline Orchestration |
| Stories | TBD |

## Related BCs (Recommended)

- BC-5.32.002 — entry-point
- BC-5.32.003 — terminal-step
- BC-5.32.004 — DAG integrity

## Architecture Anchors (Recommended)

- `architecture/ss-05-orchestration.md#discovery-workflow`

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
| **Path** | `plugins/vsdd-factory/workflows/discovery.lobster` (lines 1-45; lobster path carve-out: line range unstable as lobster files evolve) |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |

#### Evidence Types Used

- documentation: workflow header

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | reads + writes (config + state) |
| **Global state access** | reads filesystem |
| **Deterministic** | yes |
| **Thread safety** | unknown |
| **Overall classification** | effectful shell |

#### Refactoring Notes

No refactoring needed.


---

## Amendment 2026-05-08 (v→ F-P23-001: lobster-line-cite corpus-wide sweep)

**Driver:** F-P23-001 pass-23 retroactive corpus-wide sweep (per L-P19-001 / L-P20-001 / L-P22-001) — additional lobster-file line cites annotated with carve-out.

**Changes made:** Inline lobster/source-doc carve-out annotations added. Frontmatter version incremented.

## Changelog

| Version | Date | Author | Change |
|---------|------|--------|--------|
| v1.2 | 2026-05-08 | state-manager | F-P23-001 additional sweep: remaining lobster-line-cite patterns annotated with carve-out. |
