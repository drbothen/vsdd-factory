---
document_type: behavioral-contract
level: L3
version: "1.1"
status: draft
producer: "phase-1-4b-agent-8"
timestamp: 2026-04-25T00:00:00
phase: 0d
inputs: [.factory/phase-0-ingestion/pass-3-behavioral-contracts-deep-r1.md]
input-hash: "TBD"
traces_to: domain-spec/L2-INDEX.md
origin: brownfield
extracted_from: ".factory/phase-0-ingestion/pass-3-behavioral-contracts-deep-r1.md#L104"
subsystem: "SS-06"
capability: "CAP-TBD"
lifecycle_status: active
introduced: v1.0.0-beta.4
modified: []
deprecated: null
deprecated_by: null
replacement: null
retired: null
removed: null
removal_reason: null
---

# Behavioral Contract BC-6.02.001: SKILL.md frontmatter requires `name` and `description`; both are non-empty strings

## Description

SKILL.md frontmatter requires `name` and `description`; both are non-empty strings. Skill-class meta-contract — applies to every SKILL.md across the vsdd-factory skills catalog. Confidence: HIGH.

## Preconditions

1. A SKILL.md file exists under `plugins/vsdd-factory/skills/<dir>/SKILL.md`.

## Postconditions

1. YAML frontmatter (between `---` markers) declares non-empty `name:` and `description:`. Optional fields: `argument-hint:`, `disable-model-invocation:`, `allowed-tools:`, `model:`, `color:`, `tools:`. The frontmatter `name` matches the filesystem directory name (canonical skill identity).

## Invariants

1. The structural contract in Postconditions holds across every skill instance that satisfies the Preconditions; deviations indicate either skill-author error or a deliberate exception that must be documented in the skill body.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Skill missing the structural element | Skill fails template-compliance / class-contract validation |
| EC-002 | Skill declares a documented exception | Exception noted in skill body; class invariant is locally relaxed but recorded |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Preconditions met for a sample skill | Structural element present per Postconditions | happy-path |
| Preconditions met but element absent | Class-contract violation flagged | error |
| Preconditions not met | Contract is no-op (does not apply) | edge-case |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-001 | Postcondition holds for every sampled skill in evidence | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD |
| L2 Domain Invariants | TBD |
| Architecture Module | plugins/vsdd-factory/skills/ (class-level) |
| Stories | TBD |

## Related BCs (Recommended)

- TBD — sibling skill-class BCs (BC-6.02.001..BC-6.02.012)

## Architecture Anchors (Recommended)

- `plugins/vsdd-factory/skills/` — directory housing all SKILL.md files this class contract applies to

## Story Anchor (Recommended)

TBD

## VP Anchors (Recommended)

- [VP-001] — YAML frontmatter (between `---` markers) declares non-empty `name:` and `description:`. Optional fields: `argument-hint:`, `disable-model-invocation:`, `allowed-tools:`, `model:`, `color:`, `tools:`. The frontmatter `name` matches the filesystem directory name (canonical skill identity).

---

### Brownfield-Specific Sections

#### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `plugins/vsdd-factory/skills/` (class-level; representative samples in Evidence) |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |
| **Source Line** | 104 |
| **Audit ID** | BC-AUDIT-087 |
| **Evidence (verbatim)** | `plugins/vsdd-factory/skills/activate/SKILL.md:1-4` (`name: activate`, `description: Opt in to the VSDD factory persona…`); `plugins/vsdd-factory/skills/brownfield-ingest/SKILL.md:1-5`; `plugins/vsdd-factory/skills/wave-gate/SKILL.md:1-7`; `plugins/vsdd-factory/skills/release/SKILL.md:1-8`; `plugins/vsdd-factory/skills/create-prd/SKILL.md:1-6`; `plugins/vsdd-factory/skills/deliver-story/SKILL.md:1-7`. All 6 sampled skills have both fields. |
| **Confidence (verbatim)** | HIGH (uniform across all 9 sampled skills). |

#### Evidence Types Used

- **documentation**: stated in SKILL.md frontmatter / body across multiple sampled skills
- **inferred**: class-level pattern induced from a 6+ skill sample

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | not applicable (this is a structural contract on SKILL.md content, not an executable behavior) |
| **Global state access** | not applicable |
| **Deterministic** | yes — structural property of SKILL.md text |
| **Thread safety** | not applicable |
| **Overall classification** | pure (declarative class invariant) |

#### Refactoring Notes

Structural class contract — verifiable by static analysis of SKILL.md frontmatter / sections. Suitable for automated lint via validate-template-compliance or a custom script. No refactor required.
