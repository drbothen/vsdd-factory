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
extracted_from: ".factory/phase-0-ingestion/pass-3-behavioral-contracts-deep-r1.md#L174"
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

# Behavioral Contract BC-6.02.011: Skills with `argument-hint:` declare inline `$ARGUMENTS[N]` / `$ARGUMENTS` semantics

## Description

Skills with `argument-hint:` declare inline `$ARGUMENTS[N]` / `$ARGUMENTS` semantics. Skill-class meta-contract — applies to every SKILL.md across the vsdd-factory skills catalog. Confidence: HIGH..

## Preconditions

1. Skill takes user-supplied positional arguments.

## Postconditions

1. Frontmatter `argument-hint:` shows the expected shape (e.g., `[STORY-NNN]`, `[<repo>|--all] [--rollup] [--update-vision]`). Body references `$ARGUMENTS[0]`, `$ARGUMENTS[1]`, etc., with explicit type/semantics.

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

- [VP-001] — Frontmatter `argument-hint:` shows the expected shape (e.g., `[STORY-NNN]`, `[<repo>|--all] [--rollup] [--update-vision]`). Body references `$ARGUMENTS[0]`, `$ARGUMENTS[1]`, etc., with explicit type/semantics.

---

### Brownfield-Specific Sections

#### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `plugins/vsdd-factory/skills/` (class-level; representative samples in Evidence) |
| **Confidence** | medium |
| **Extraction Date** | 2026-04-25 |
| **Source Line** | 174 |
| **Audit ID** | BC-AUDIT-097 |
| **Evidence (verbatim)** | `release/SKILL.md:7` (`argument-hint: "[init | <version> | --dry-run]"`); `deliver-story/SKILL.md:4` (`argument-hint: "[STORY-NNN]"`); `disposition-pass/SKILL.md:4` (`argument-hint: "[<repo>|--all] [--rollup] [--update-vision]"`). |
| **Confidence (verbatim)** | HIGH. |

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
