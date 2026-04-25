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
audit_source_id: "BC-AUDIT-569"
ss_section: "Feature-mode phase skills (f1-f7)"
skill: "phase-f7-delta-convergence"
---

# Behavioral Contract BC-6.17.042: phase-f7-delta-convergence: Cost-benefit — flag MAXIMUM_VIABLE_REFINEMENT_REACHED if cost > P(finding) * Value_avg / 1.5

## Description

Include cost-benefit data from cost-tracker (DF-027): total cost, cost per dimension, projected cost of additional cycles. Compare P(finding in next iteration) * Value_avg vs Cost_iteration * 1.5. Flag MAXIMUM_VIABLE_REFINEMENT_REACHED if cost exceeds expected value. Acceptance: Specific formula; flag name.

## Preconditions

1. Step 2b runs.

## Postconditions

1. Include cost-benefit data from cost-tracker (DF-027): total cost, cost per dimension, projected cost of additional cycles. Compare P(finding in next iteration) * Value_avg vs Cost_iteration * 1.5. Flag MAXIMUM_VIABLE_REFINEMENT_REACHED if cost exceeds expected value.

## Invariants

1. Specific formula; flag name.

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
| Architecture Module | phase-f7-delta-convergence |
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
| **Path** | `plugins/vsdd-factory/skills/phase-f7-delta-convergence/SKILL.md` |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |
| **Audit Source ID** | BC-AUDIT-569 |
| **Source Line(s)** | 68-75 (Step 2b: Cost-Benefit Analysis) |
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
