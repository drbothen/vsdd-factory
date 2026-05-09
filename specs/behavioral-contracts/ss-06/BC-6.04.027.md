---
document_type: behavioral-contract
level: L3
version: "1.2"
last_amended: 2026-05-08
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
audit_source_id: "BC-AUDIT-428"
ss_section: "Adversarial and review skills"
skill: "holdout-eval"
---

# Behavioral Contract BC-6.04.027: holdout-eval: Gate is mean satisfaction ≥ 0.85 AND every critical scenario ≥ 0.60

## Description

PASS = mean satisfaction ≥ 0.85, every critical scenario ≥ 0.60. FAIL = below thresholds → stories need remediation before next wave. Acceptance: 0.85 / 0.60 thresholds; explicit "0.84 fails. No rounding." in red-flag table (§ "Red Flag Indicators"; lobster carve-out: table name is the stable anchor, not line number).
<!-- F-P23-002: cross-subsystem lobster-line-cite annotated per pass-23 carve-out; `red-flag table` is the stable anchor; line 27 is unstable as SKILL.md evolves -->

## Preconditions

1. Evaluation completes.

## Postconditions

1. PASS = mean satisfaction ≥ 0.85, every critical scenario ≥ 0.60. FAIL = below thresholds → stories need remediation before next wave.

## Invariants

1. 0.85 / 0.60 thresholds; explicit "0.84 fails. No rounding." in red-flag table (§ "Red Flag Indicators"; source-line carve-out: table name is stable anchor, not line 27).

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
| Architecture Module | holdout-eval |
| Stories | TBD |

## Related BCs (Recommended)

- TBD

## Architecture Anchors (Recommended)

- `architecture/ss-06-skill-catalog.md` — Adversarial and review skills

## Story Anchor (Recommended)

TBD

## VP Anchors (Recommended)

- TBD

---

### Brownfield-Specific Sections

#### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `plugins/vsdd-factory/skills/holdout-eval/SKILL.md` |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |
| **Audit Source ID** | BC-AUDIT-428 |
| **Source Line(s)** | 69-72 (Gate Criteria) |
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

---

## Amendment 2026-05-08 (v1.1 → v1.2 — F-P23-002: cross-subsystem source-line-cite migrated to stable section anchor)

**Driver:** F-P23-002 pass-23 cross-subsystem corpus sweep — §Description and §Invariants cited `red-flag table (line 27)`. This references a line in `plugins/vsdd-factory/skills/holdout-eval/SKILL.md` which drifts as the SKILL.md evolves. Per TD-VSDD-091, source-file line cites must migrate to stable symbol anchors. The `red-flag table` (§ "Red Flag Indicators") is the stable anchor.

**Changes made:**
- §Description: `red-flag table (line 27)` → `red-flag table (§ "Red Flag Indicators"; lobster carve-out: table name is the stable anchor, not line number)`. HTML carve-out comment added.
- §Invariants §1: same `(line 27)` replaced with stable section anchor.
- Frontmatter `version:` bumped `"1.1"` → `"1.2"`.

## Changelog

| Version | Date | Author | Change |
|---------|------|--------|--------|
| v1.2 | 2026-05-08 | state-manager | F-P23-002 cross-subsystem sweep: `red-flag table (line 27)` cite migrated to stable section anchor `§ "Red Flag Indicators"` per TD-VSDD-091. |
