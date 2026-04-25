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
audit_source_id: "BC-AUDIT-567"
ss_section: "Feature-mode phase skills (f1-f7)"
skill: "phase-f7-delta-convergence"
---

# Behavioral Contract BC-6.17.040: phase-f7-delta-convergence: 5 dimensions — Spec novelty<0.15, Test mutation≥90%, Impl verification rate<60%, Verification all-pass, Holdout≥0.85

## Description

Dim 1 Spec — adversary novelty score < 0.15 on spec delta. Dim 2 Test — mutation kill rate ≥ 90% on changed files; no vacuously true tests. Dim 3 Implementation — adversary verification rate < 60% (hallucinating flaws); no CRITICAL/HIGH open. Dim 4 Verification — all Kani proofs pass; fuzz clean (5 min/target); no security vulns; purity intact. Dim 5 Holdout — mean ≥ 0.85; no must-pass < 0.6; regression holdout still passes. Acceptance: Five dimensions with specific metric and threshold.

## Preconditions

1. Step 1 runs.

## Postconditions

1. Dim 1 Spec — adversary novelty score < 0.15 on spec delta. Dim 2 Test — mutation kill rate ≥ 90% on changed files; no vacuously true tests. Dim 3 Implementation — adversary verification rate < 60% (hallucinating flaws); no CRITICAL/HIGH open. Dim 4 Verification — all Kani proofs pass; fuzz clean (5 min/target); no security vulns; purity intact. Dim 5 Holdout — mean ≥ 0.85; no must-pass < 0.6; regression holdout still passes.

## Invariants

1. Five dimensions with specific metric and threshold.

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
| **Audit Source ID** | BC-AUDIT-567 |
| **Source Line(s)** | 22-56 (Step 1) |
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
