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
extracted_from: ".factory/phase-0-ingestion/pass-3-deep-skills-batch-3.md#L1238"
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

# Behavioral Contract BC-6.19.031: toolchain-provisioning: Storybook + Excalidraw MCP for UI products

## Description

toolchain-provisioning: Storybook + Excalidraw MCP for UI products. Confidence: HIGH. Extracted from `plugins/vsdd-factory/skills/toolchain-provisioning/SKILL.md` (lines 314-405).

## Preconditions

1. UI/full-stack feature_type.

## Postconditions

1. Install Storybook + addon-mcp; configure .storybook/main.ts; register MCP at http://localhost:6006/mcp; write to toolchain-state.yaml ui_tooling.storybook. Also Excalidraw via mcporter (`mcporter config add excalidraw --url https://mcp.excalidraw.com`); install `@tommywalkie/excalidraw-cli`. Quality gate: Excalidraw responds to list-tools before UX spec.

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
| VP-001 | Acceptance: UI products have both Storybook MCP and Excalidraw MCP entries in toolchain-state. | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD |
| L2 Domain Invariants | TBD |
| Architecture Module | plugins/vsdd-factory/skills/toolchain-provisioning/SKILL.md |
| Stories | TBD |

## Related BCs (Recommended)

- TBD — sibling BCs in same skill (see other BCs for `plugins/vsdd-factory/skills/toolchain-provisioning/SKILL.md` in this directory)

## Architecture Anchors (Recommended)

- `plugins/vsdd-factory/skills/toolchain-provisioning/SKILL.md` — defining skill body for this contract

## Story Anchor (Recommended)

TBD

## VP Anchors (Recommended)

- [VP-001] — UI products have both Storybook MCP and Excalidraw MCP entries in toolchain-state.

---

### Brownfield-Specific Sections

#### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `plugins/vsdd-factory/skills/toolchain-provisioning/SKILL.md` |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |
| **Source Lines** | 314-405 |
| **Audit ID** | BC-AUDIT-724 |

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
