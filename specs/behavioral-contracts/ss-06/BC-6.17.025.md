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
audit_source_id: "BC-AUDIT-552"
ss_section: "Feature-mode phase skills (f1-f7)"
skill: "phase-f4-delta-implementation"
---

# Behavioral Contract BC-6.17.025: phase-f4-delta-implementation: Quality Gate — regression baseline + Two-Step Red Gate + full regression pass + reviewer + security if CRIT/HIGH + max 10 + wave gate + E2E for UI + no out-of-scope edits + summary

## Description

Regression baseline recorded with commit SHA; per-story flow worktree→tests→implement→demo→PR→review→merge; Two-Step Red Gate (stubs first, then tests); all new tests pass (Green Gate); full regression passes (zero regressions); pr-reviewer (Gemini) reviewed each story PR; security-reviewer engaged for CRIT/HIGH; max 10 review rounds per story; wave integration gate; gate includes adversary+security+holdout+a11y if UI; E2E tests for UI/full-stack; existing conventions followed; no edits outside delta scope; implementation summary written with deviation log; all PRs merged. Acceptance: Fifteen Quality Gate items.

## Preconditions

1. Phase complete.

## Postconditions

1. Regression baseline recorded with commit SHA; per-story flow worktree→tests→implement→demo→PR→review→merge; Two-Step Red Gate (stubs first, then tests); all new tests pass (Green Gate); full regression passes (zero regressions); pr-reviewer (Gemini) reviewed each story PR; security-reviewer engaged for CRIT/HIGH; max 10 review rounds per story; wave integration gate; gate includes adversary+security+holdout+a11y if UI; E2E tests for UI/full-stack; existing conventions followed; no edits outside delta scope; implementation summary written with deviation log; all PRs merged.

## Invariants

1. Fifteen Quality Gate items.

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
| Architecture Module | phase-f4-delta-implementation |
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
| **Path** | `plugins/vsdd-factory/skills/phase-f4-delta-implementation/SKILL.md` |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |
| **Audit Source ID** | BC-AUDIT-552 |
| **Source Line(s)** | 152-169 (Quality Gate Criteria) |
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
