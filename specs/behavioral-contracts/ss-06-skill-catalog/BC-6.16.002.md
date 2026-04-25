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
audit_source_id: "BC-AUDIT-458"
ss_section: "Phase orchestration and mode skills"
skill: "mode-decision-guide"
---

# Behavioral Contract BC-6.16.002: mode-decision-guide: Feature Mode threshold — <30% files changed AND <50% components AND ≤2 cascade levels

## Description

Feature Mode appropriate when files changed <30% AND components affected <50% AND fewer new modules than existing AND dependency depth ≤ 2. Acceptance: All four numeric thresholds.

## Preconditions

1. Mode is not obvious; quantitative heuristic applied.

## Postconditions

1. Feature Mode appropriate when files changed <30% AND components affected <50% AND fewer new modules than existing AND dependency depth ≤ 2.

## Invariants

1. All four numeric thresholds.

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
| Architecture Module | mode-decision-guide |
| Stories | TBD |

## Related BCs (Recommended)

- TBD

## Architecture Anchors (Recommended)

- `architecture/ss-06-skill-catalog.md` — Phase orchestration and mode skills

## Story Anchor (Recommended)

TBD

## VP Anchors (Recommended)

- TBD

---

### Brownfield-Specific Sections

#### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `plugins/vsdd-factory/skills/mode-decision-guide/SKILL.md` |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |
| **Audit Source ID** | BC-AUDIT-458 |
| **Source Line(s)** | 75-83 (Feature Mode Threshold) |
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
