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
extracted_from: ".factory/phase-0-ingestion/pass-3-behavioral-contracts-deep-r1.md#L153"
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

# Behavioral Contract BC-6.02.008: Skill quality gates expressed as a "Hard Gate" or "Iron Law" prose section

## Description

Skill quality gates expressed as a "Hard Gate" or "Iron Law" prose section. Skill-class meta-contract — applies to every SKILL.md across the vsdd-factory skills catalog. Confidence: HIGH.

## Preconditions

1. Skill enforces an invariant the operator/agent must not violate.

## Postconditions

1. Skill body declares the invariant under "## Hard Gate" or "## The Iron Law" with imperative wording ("DO NOT skip…", "NO ROUND COMPLETION…"). These are the testable acceptance gates that downstream BC extraction can crystallize.

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

- [TBD — to be assigned in Phase 1.6c] — Skill body declares the invariant under "## Hard Gate" or "## The Iron Law" with imperative wording ("DO NOT skip…", "NO ROUND COMPLETION…"). These are the testable acceptance gates that downstream BC extraction can crystallize.

---

### Brownfield-Specific Sections

#### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `plugins/vsdd-factory/skills/` (class-level; representative samples in Evidence) |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |
| **Source Line** | 153 |
| **Audit ID** | BC-AUDIT-094 |
| **Evidence (verbatim)** | `create-prd/SKILL.md:9-11` ("Hard Gate: Do NOT skip to architecture design…"); `brownfield-ingest/SKILL.md:11-15` ("The Iron Law: NO ROUND COMPLETION WITHOUT HONEST CONVERGENCE CHECK FIRST"); `wave-gate/SKILL.md:13-17` ("NO WAVE ADVANCE WITHOUT ALL SIX GATES PASSING FIRST"); `deliver-story/SKILL.md:17-21` ("NO IMPLEMENTATION WITHOUT RED GATE VERIFICATION FIRST"). |
| **Confidence (verbatim)** | HIGH (consistent across 4 sampled "high-stakes" skills). |

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
