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
audit_source_id: "BC-AUDIT-532"
ss_section: "Feature-mode phase skills (f1-f7)"
skill: "phase-f1-delta-analysis"
---

# Behavioral Contract BC-6.17.005: phase-f1-delta-analysis: Severity (bug-fix only) — CRITICAL triggers expedited flow with skipped baseline/proofs

## Description

Severity table: CRITICAL (production down, data loss, security breach), HIGH (major broken, no workaround), MEDIUM (impaired, workaround exists), LOW (minor, cosmetic, edge). CRITICAL → skip demo baseline, F1 minimal, async human approval, F5 1 round max, F6 security scan only (skip proofs/fuzz/mutation), release immediately, logged in STATE.md as expedited:true with justification, skipped steps queued for follow-up. Acceptance: Four severity classes; expedited flow specifics.

## Preconditions

1. Intent is bug-fix.

## Postconditions

1. Severity table: CRITICAL (production down, data loss, security breach), HIGH (major broken, no workaround), MEDIUM (impaired, workaround exists), LOW (minor, cosmetic, edge). CRITICAL → skip demo baseline, F1 minimal, async human approval, F5 1 round max, F6 security scan only (skip proofs/fuzz/mutation), release immediately, logged in STATE.md as expedited:true with justification, skipped steps queued for follow-up.

## Invariants

1. Four severity classes; expedited flow specifics.

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
| Architecture Module | phase-f1-delta-analysis |
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
| **Path** | `plugins/vsdd-factory/skills/phase-f1-delta-analysis/SKILL.md` |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |
| **Audit Source ID** | BC-AUDIT-532 |
| **Source Line(s)** | 100-124 (Step 4d: Severity Classification) |
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
