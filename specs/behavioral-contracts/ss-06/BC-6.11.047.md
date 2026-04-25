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
audit_source_id: "BC-AUDIT-589"
ss_section: "Factory operations and dashboards skills"
skill: "policy-registry"
---

# Behavioral Contract BC-6.11.047: policy-registry: Adversarial review auto-loads policies.yaml as rubric

## Description

Orchestrator reads `.factory/policies.yaml` at dispatch — if missing, warn but continue (policies also in agent prompts). For each policy, format as rubric item with id, name, description, severity, scope. Append to adversary's task prompt under "## Project Policy Rubric" heading. Adversary verifies each policy as a review axis and reports compliance per-policy. "Why both? Agent prompts carry the enforcement logic (HOW); the registry carries the catalog (WHAT).". Acceptance: Auto-load procedure; rubric format; warn-not-fail on missing.

## Preconditions

1. Adversary dispatch.

## Postconditions

1. Orchestrator reads `.factory/policies.yaml` at dispatch — if missing, warn but continue (policies also in agent prompts). For each policy, format as rubric item with id, name, description, severity, scope. Append to adversary's task prompt under "## Project Policy Rubric" heading. Adversary verifies each policy as a review axis and reports compliance per-policy. "Why both? Agent prompts carry the enforcement logic (HOW); the registry carries the catalog (WHAT)."

## Invariants

1. Auto-load procedure; rubric format; warn-not-fail on missing.

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
| (TBD — to be assigned in Phase 1.6c) | TBD | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD |
| L2 Domain Invariants | TBD |
| Architecture Module | policy-registry |
| Stories | TBD |

## Related BCs (Recommended)

- TBD

## Architecture Anchors (Recommended)

- `architecture/ss-06-skill-catalog.md` — Factory operations and dashboards skills

## Story Anchor (Recommended)

TBD

## VP Anchors (Recommended)

- TBD

---

### Brownfield-Specific Sections

#### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `plugins/vsdd-factory/skills/post-feature-validation/SKILL.md` (213 LOC)` |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |
| **Audit Source ID** | BC-AUDIT-589 |
| **Source Line(s)** | 92-105 (Integration with Adversarial Review) |
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
