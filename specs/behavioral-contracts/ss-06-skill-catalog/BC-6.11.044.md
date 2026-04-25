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
audit_source_id: "BC-AUDIT-586"
ss_section: "Factory operations and dashboards skills"
skill: "policy-registry"
---

# Behavioral Contract BC-6.11.044: policy-registry: Init copies template + populates 9 baseline governance policies

## Description

If `.factory/policies.yaml` already exists → ask user overwrite or merge. If overwrite: replace with baseline. If merge: add baseline policies not already present (match by name). If no: copy `${CLAUDE_PLUGIN_ROOT}/templates/policies-template.yaml` and populate with 9 baseline policies. Reports init status. Acceptance: 9 baseline policies (append_only_numbering, lift_invariants_to_bcs, state_manager_runs_last, semantic_anchoring_integrity, creators_justify_anchors, architecture_is_subsystem_name_source_of_truth, bc_h1_is_title_source_of_truth, bc_array_changes_propagate_to_body_and_acs, vp_index_is_vp_catalog_source_of_truth).

## Preconditions

1. `policy-registry init`.

## Postconditions

1. If `.factory/policies.yaml` already exists → ask user overwrite or merge. If overwrite: replace with baseline. If merge: add baseline policies not already present (match by name). If no: copy `${CLAUDE_PLUGIN_ROOT}/templates/policies-template.yaml` and populate with 9 baseline policies. Reports init status.

## Invariants

1. 9 baseline policies (append_only_numbering, lift_invariants_to_bcs, state_manager_runs_last, semantic_anchoring_integrity, creators_justify_anchors, architecture_is_subsystem_name_source_of_truth, bc_h1_is_title_source_of_truth, bc_array_changes_propagate_to_body_and_acs, vp_index_is_vp_catalog_source_of_truth).

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
| **Path** | `plugins/vsdd-factory/skills/policy-registry/SKILL.md` |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |
| **Audit Source ID** | BC-AUDIT-586 |
| **Source Line(s)** | 31-39 (Init), 75-89 (Baseline Policies) |
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
