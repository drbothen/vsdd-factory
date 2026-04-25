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
audit_source_id: "BC-AUDIT-587"
ss_section: "Factory operations and dashboards skills"
skill: "policy-registry"
---

# Behavioral Contract BC-6.11.045: policy-registry: Validate checks ID/name uniqueness, snake_case, severity ∈ {HIGH,MEDIUM}, lint_hook exists+executable, scope ∈ allowed types

## Description

Parse YAML; if invalid syntax → report and stop. Verify per policy: ID uniqueness, name uniqueness, name format snake_case, required fields non-empty (id, name, description, severity, enforced_by), severity HIGH or MEDIUM, lint_hook exists at `${CLAUDE_PLUGIN_ROOT}/<lint_hook>` and is executable, scope from allowed list (bc, vp, di, cap, story, hs, architecture, prd, nfr). Report PASS/FAIL per policy with remediation. Acceptance: Six validation checks.

## Preconditions

1. `policy-registry validate`.

## Postconditions

1. Parse YAML; if invalid syntax → report and stop. Verify per policy: ID uniqueness, name uniqueness, name format snake_case, required fields non-empty (id, name, description, severity, enforced_by), severity HIGH or MEDIUM, lint_hook exists at `${CLAUDE_PLUGIN_ROOT}/<lint_hook>` and is executable, scope from allowed list (bc, vp, di, cap, story, hs, architecture, prd, nfr). Report PASS/FAIL per policy with remediation.

## Invariants

1. Six validation checks.

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
| **Path** | `plugins/vsdd-factory/skills/policy-registry/SKILL.md` |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |
| **Audit Source ID** | BC-AUDIT-587 |
| **Source Line(s)** | 53-67 (Validate) |
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
