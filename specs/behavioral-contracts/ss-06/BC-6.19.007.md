---
document_type: behavioral-contract
level: L3
version: "1.1"
status: draft
producer: "phase-1-4b-agent-8"
timestamp: 2026-04-25T00:00:00
phase: 0d
inputs: [.factory/phase-0-ingestion/pass-3-deep-skills-batch-3.md]
input-hash: "TBD"
traces_to: domain-spec/L2-INDEX.md
origin: brownfield
extracted_from: ".factory/phase-0-ingestion/pass-3-deep-skills-batch-3.md#L454"
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

# Behavioral Contract BC-6.19.007: repo-initialization: dx-engineer environment setup (DF-027)

## Description

repo-initialization: dx-engineer environment setup (DF-027). Confidence: HIGH. Extracted from `plugins/vsdd-factory/skills/repo-initialization/SKILL.md` (lines 275-347).

## Preconditions

1. Step 5 after repo creation.

## Postconditions

1. Installs direnv; creates committed `.envrc` (`dotenv .env`) and `.env.example` (empty at init, populated during DTU); updates `.gitignore` for `.env`/`.env.local`; runs `direnv allow .`; installs `mcporter`; configures perplexity/context7/playwright MCP servers; LLM health check (3 model families); MCP preflight.

## Invariants

1. TBD — invariants not separately enumerated in source; see Acceptance for verification surface.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | TBD | TBD |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Trigger condition met | Behavior executed; acceptance criteria satisfied | happy-path |
| Trigger condition absent | Skill is no-op (or alternative path per skill body) | edge-case |
| Acceptance criterion violated | TBD — failure path per skill failure-modes section | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-001 | Acceptance: All eight numbered substeps completed; LLM health blocks if any model unavailable. | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD |
| L2 Domain Invariants | TBD |
| Architecture Module | plugins/vsdd-factory/skills/repo-initialization/SKILL.md |
| Stories | TBD |

## Related BCs (Recommended)

- TBD — sibling BCs in same skill (see other BCs for `plugins/vsdd-factory/skills/repo-initialization/SKILL.md` in this directory)

## Architecture Anchors (Recommended)

- `plugins/vsdd-factory/skills/repo-initialization/SKILL.md` — defining skill body for this contract

## Story Anchor (Recommended)

TBD

## VP Anchors (Recommended)

- [VP-001] — All eight numbered substeps completed; LLM health blocks if any model unavailable.

---

### Brownfield-Specific Sections

#### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `plugins/vsdd-factory/skills/repo-initialization/SKILL.md` |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |
| **Source Lines** | 275-347 |
| **Audit ID** | BC-AUDIT-644 |

#### Evidence Types Used

- **documentation**: stated in SKILL.md body and frontmatter (declarative skill-spec evidence)
- **inferred**: triggers and acceptance conditions derived from skill prose

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | TBD — depends on skill (most skills perform reads + writes via allowed-tools) |
| **Global state access** | TBD |
| **Deterministic** | TBD — most skills are deterministic given identical inputs and allowed-tools surface |
| **Thread safety** | not applicable (skills run sequentially within a Claude Code session) |
| **Overall classification** | mixed |

#### Refactoring Notes

This contract codifies a SKILL.md-driven behavior. The skill body is the canonical specification; this BC extracts the procedural contract for downstream verification. No code refactor implied — refinement of the SKILL.md or its templates would be the corrective surface.
