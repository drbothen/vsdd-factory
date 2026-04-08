---
name: phase-1-prd-revision
description: >
  Product Owner revises PRD v1 based on Architect's feasibility review.
  Incorporates architectural feedback, updates subsystem grouping if justified.
  Max 3 iterations before escalation to human.
agents:
  primary: product-owner
  supporting: [architect]
inputs:
  - .factory/specs/architecture-feasibility-report.md
  - .factory/specs/prd.md
  - .factory/specs/behavioral-contracts/
outputs:
  - .factory/specs/prd.md (updated)
  - .factory/specs/behavioral-contracts/ (updated)
gate: Architect approves or max 3 iterations reached
---

# PRD Revision (Given Architecture Feasibility Report)

## Purpose

When the architect flags issues in the Architecture Feasibility Report,
the product-owner revises the PRD and behavioral contracts to address them.

## Flow

1. Read `architecture-feasibility-report.md` for flagged issues
2. For each issue:
   - If subsystem restructuring proposed: evaluate and accept or contest
   - If NFR conflict: add architectural constraint annotations to affected BCs
   - If verifiability concern: adjust BC postconditions for testability
3. Update PRD v2 (or v3 if iteration 2)
4. Update affected BC files
5. Notify orchestrator: "PRD revision complete"

## Convergence

- Max 3 iterations between PO and Architect
- If disagreement persists after 3 rounds: escalate to human
- Decision reasoning recorded in Architecture Feasibility Report Decision Log

## Skip Condition

If architect's feasibility report says "validated — no issues": skip this step entirely.

## Quality Gate

- [ ] Every architect concern in the feasibility report is addressed or contested with rationale
- [ ] PRD updated with architectural constraint annotations where needed
- [ ] BC postconditions adjusted for testability where verifiability was flagged
- [ ] Feasibility review passes (architect approves) or max 3 iterations reached

## Failure Modes

- If architect and product-owner deadlock after 3 iterations: escalate to human with both positions
- If feasibility report references missing BCs: route to product-owner to create them first
- If NFR conflicts cannot be resolved without scope reduction: present trade-offs to human
