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
extracted_from: ".factory/phase-0-ingestion/pass-3-behavioral-contracts-deep-r1.md#L118"
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

# Behavioral Contract BC-6.02.003: Skill invocation surface is `/vsdd-factory:<skill-name>` slash command

## Description

Skill invocation surface is `/vsdd-factory:<skill-name>` slash command. Skill-class meta-contract — applies to every SKILL.md across the vsdd-factory skills catalog. Confidence: MEDIUM.

## Preconditions

1. Operator running Claude Code with vsdd-factory plugin activated.

## Postconditions

1. Typing `/vsdd-factory:<skill-name>` (where `<skill-name>` matches the frontmatter `name`) invokes the procedure body. The slash-command surface is enumerated separately in `plugins/vsdd-factory/commands/*.md` (110 files in pass-0 inventory).

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
| (TBD — to be assigned in Phase 1.6c) | Postcondition holds for every sampled skill in evidence | manual |

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

- [TBD — to be assigned in Phase 1.6c] — Typing `/vsdd-factory:<skill-name>` (where `<skill-name>` matches the frontmatter `name`) invokes the procedure body. The slash-command surface is enumerated separately in `plugins/vsdd-factory/commands/*.md` (110 files in pass-0 inventory).

---

### Brownfield-Specific Sections

#### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `plugins/vsdd-factory/skills/` (class-level; representative samples in Evidence) |
| **Confidence** | medium |
| **Extraction Date** | 2026-04-25 |
| **Source Line** | 118 |
| **Audit ID** | BC-AUDIT-089 |
| **Evidence (verbatim)** | `pass-0-inventory.md` Section 5; `skills/activate/SKILL.md` "See also" section invokes `/vsdd-factory:deactivate`. Each skill has a 1:1 slash command in `commands/`. |
| **Confidence (verbatim)** | MEDIUM (110 commands vs 119 skills — there's a 9-skill gap; not every skill has a slash command. Verified by file count discrepancy, not yet fully enumerated). |

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
