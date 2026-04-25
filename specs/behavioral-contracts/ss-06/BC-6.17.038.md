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
audit_source_id: "BC-AUDIT-565"
ss_section: "Feature-mode phase skills (f1-f7)"
skill: "phase-f6-targeted-hardening"
---

# Behavioral Contract BC-6.17.038: phase-f6-targeted-hardening: Quality Gate — proofs pass, fuzz clean, mutation 90% (95% critical), no CRIT/HIGH, regression passes, DTU adversarial if external svc, a11y if UI, FIX-F6-NNN via code-delivery, partial re-verification

## Description

Kani proofs pass for new VPs (or justified skip); fuzz clean after 5 min/target (or skip); mutation kill ≥90% (≥95% critical); zero CRITICAL/HIGH security findings (CRIT/HIGH → BLOCK for human); full regression suite passes; DTU adversarial testing if external service interaction changed; a11y re-check if UI feature; fix PRs via code-delivery.lobster (FIX-F6-NNN); F6 re-verifies only failing checks after fix (partial re-verification); hardening summary with all metrics. Acceptance: Ten Quality Gate items.

## Preconditions

1. Phase complete.

## Postconditions

1. Kani proofs pass for new VPs (or justified skip); fuzz clean after 5 min/target (or skip); mutation kill ≥90% (≥95% critical); zero CRITICAL/HIGH security findings (CRIT/HIGH → BLOCK for human); full regression suite passes; DTU adversarial testing if external service interaction changed; a11y re-check if UI feature; fix PRs via code-delivery.lobster (FIX-F6-NNN); F6 re-verifies only failing checks after fix (partial re-verification); hardening summary with all metrics.

## Invariants

1. Ten Quality Gate items.

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
| Architecture Module | phase-f6-targeted-hardening |
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
| **Path** | `plugins/vsdd-factory/skills/phase-f7-delta-convergence/SKILL.md` (180 LOC)` |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |
| **Audit Source ID** | BC-AUDIT-565 |
| **Source Line(s)** | 161-172 (Quality Gate Criteria) |
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
