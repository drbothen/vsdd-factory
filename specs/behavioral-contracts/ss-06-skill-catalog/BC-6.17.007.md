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
audit_source_id: "BC-AUDIT-534"
ss_section: "Feature-mode phase skills (f1-f7)"
skill: "phase-f1-delta-analysis"
---

# Behavioral Contract BC-6.17.007: phase-f1-delta-analysis: Quality Gate — feature_type, intent, scope, severity (if bug-fix), BC-S.SS.NNN refs, multi-repo, human-approved

## Description

All affected components classified; regression risk assessed; existing tests in risk zone enumerated; files NOT changed listed as baseline; feature type classified (ui|backend|full-stack|infrastructure); intent classified; trivial scope assessed; severity classified if bug-fix; uses BC-S.SS.NNN identifiers (no FR-NNN); multi-repo affected repos + contract changes identified; human explicitly approved. Acceptance: Eleven Quality Gate items.

## Preconditions

1. Phase complete.

## Postconditions

1. All affected components classified; regression risk assessed; existing tests in risk zone enumerated; files NOT changed listed as baseline; feature type classified (ui|backend|full-stack|infrastructure); intent classified; trivial scope assessed; severity classified if bug-fix; uses BC-S.SS.NNN identifiers (no FR-NNN); multi-repo affected repos + contract changes identified; human explicitly approved.

## Invariants

1. Eleven Quality Gate items.

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
| **Path** | `plugins/vsdd-factory/skills/phase-f2-spec-evolution/SKILL.md` (185 LOC)` |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |
| **Audit Source ID** | BC-AUDIT-534 |
| **Source Line(s)** | 174-186 (Quality Gate Criteria) |
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
