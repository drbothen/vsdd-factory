---
document_type: behavioral-contract
level: L3
version: "1.1"
status: draft
producer: "codebase-analyzer"
timestamp: 2026-04-25T00:00:00
phase: 1.4b
inputs:
  - .factory/phase-0-ingestion/pass-3-deep-skills-batch-2.md
  - .factory/specs/behavioral-contracts/bc-id-mapping.md
input-hash: TBD
traces_to: domain-spec/L2-INDEX.md
origin: brownfield
extracted_from: ".factory/phase-0-ingestion/pass-3-deep-skills-batch-2.md"
subsystem: "SS-06"
capability: "CAP-TBD"
lifecycle_status: active
introduced: v1.0.0
modified: []
deprecated: null
deprecated_by: null
replacement: null
retired: null
removed: null
removal_reason: null
audit_source_id: "BC-AUDIT-546"
ss_section: "Feature-mode phase skills (f1-f7)"
skill: "phase-f3-incremental-stories"
---

# Behavioral Contract BC-6.17.019: phase-f3-incremental-stories: Quality Gate — IDs continue, per-file, BC-S.SS.NNN, testable AC, VP-NNN, no cycles, append-only, wave schedule + holdouts, conflicts resolved, human approved

## Description

Story IDs continue (no collisions); per-file STORY-NNN.md (not monolithic); references BCs (BC-S.SS.NNN format); testable AC; VP-NNN references; topological sort succeeds; deps extended without modifying existing; wave schedule computed; wave holdout scenarios per wave; DTU clone stories in Wave 1 if flagged; gene transfusion stories flagged; conflicts identified and resolved; effort estimated with critical path; human approved. Acceptance: Fourteen Quality Gate items.

## Preconditions

1. Phase complete.

## Postconditions

1. Story IDs continue (no collisions); per-file STORY-NNN.md (not monolithic); references BCs (BC-S.SS.NNN format); testable AC; VP-NNN references; topological sort succeeds; deps extended without modifying existing; wave schedule computed; wave holdout scenarios per wave; DTU clone stories in Wave 1 if flagged; gene transfusion stories flagged; conflicts identified and resolved; effort estimated with critical path; human approved.

## Invariants

1. Fourteen Quality Gate items.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | TBD | TBD |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| TBD | TBD | happy-path |
| TBD | TBD | edge-case |
| TBD | TBD | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-001 | TBD | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD |
| L2 Domain Invariants | TBD |
| Architecture Module | phase-f3-incremental-stories |
| Stories | TBD |

## Related BCs (Recommended)

- TBD

## Architecture Anchors (Recommended)

- `architecture/ss-06-skill-catalog.md` — Feature-mode phase skills (f1-f7)

## Story Anchor (Recommended)

TBD

## VP Anchors (Recommended)

- TBD

---

### Brownfield-Specific Sections

#### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `plugins/vsdd-factory/skills/phase-f4-delta-implementation/SKILL.md` (176 LOC)` |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |
| **Audit Source ID** | BC-AUDIT-546 |
| **Source Line(s)** | 132-148 (Quality Gate Criteria) |
| **Source File** | `.factory/phase-0-ingestion/pass-3-deep-skills-batch-2.md` |

#### Evidence Types Used

- **documentation**: extracted from SKILL.md frontmatter and Quality Gate sections

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | reads + writes (skill orchestrates filesystem and agent dispatch) |
| **Global state access** | reads global (STATE.md, .factory/ tree) |
| **Deterministic** | no -- depends on agent execution and human approval |
| **Thread safety** | not thread-safe |
| **Overall classification** | effectful shell |

#### Refactoring Notes

This BC describes a skill-level workflow contract. The acceptance criteria
encode the Quality Gate checks performed by the skill; these can be lifted
into automated assertions where the skill's underlying procedure is
deterministic. Adversarial and human-gated steps are explicitly opaque.
