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
extracted_from: ".factory/phase-0-ingestion/pass-3-behavioral-contracts-deep-r1.md#L181"
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

# Behavioral Contract BC-6.02.012: Skill output paths follow `${CLAUDE_PLUGIN_ROOT}` / `.factory/` placement convention

## Description

Skill output paths follow `${CLAUDE_PLUGIN_ROOT}` / `.factory/` placement convention. Skill-class meta-contract — applies to every SKILL.md across the vsdd-factory skills catalog. Confidence: HIGH.

## Preconditions

1. Skill produces artifacts.

## Postconditions

1. Outputs go to `.factory/<subtree>/` (project-local persistence); skill steps NEVER write outside `.factory/` or product working directories. Sandbox patterns: `.factory/specs/`, `.factory/stories/`, `.factory/cycles/`, `.factory/phase-0-ingestion/`, `.factory/semport/`. Plugin-distributed reference content lives at `${CLAUDE_PLUGIN_ROOT}/templates/`.

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

- [VP-001] — Outputs go to `.factory/<subtree>/` (project-local persistence); skill steps NEVER write outside `.factory/` or product working directories. Sandbox patterns: `.factory/specs/`, `.factory/stories/`, `.factory/cycles/`, `.factory/phase-0-ingestion/`, `.factory/semport/`. Plugin-distributed reference content lives at `${CLAUDE_PLUGIN_ROOT}/templates/`.

---

### Brownfield-Specific Sections

#### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `plugins/vsdd-factory/skills/` (class-level; representative samples in Evidence) |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |
| **Source Line** | 181 |
| **Audit ID** | BC-AUDIT-098 |
| **Evidence (verbatim)** | `create-prd/SKILL.md:3` ("writes to .factory/specs/prd.md and supplements"); `brownfield-ingest/SKILL.md:74-78` (analysis writes under `.factory/semport/<project>/`); `disposition-pass/SKILL.md:46` (reads `.factory/semport/<repo>/`). |
| **Confidence (verbatim)** | HIGH (universal across all sampled skills). |

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
