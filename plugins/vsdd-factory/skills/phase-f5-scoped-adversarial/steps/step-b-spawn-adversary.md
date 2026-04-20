---
name: step-b-spawn-adversary
description: Spawn the primary adversary agent with fresh context and the scoped review package.
---

# Step B: Spawn Primary Adversary

> **Shared context:** Read `./_shared-context.md` before executing this step.

Spawn the `adversary` agent with fresh context and ONLY the review package from Step A.

## Procedure

1. **Spawn `adversary` agent** with:
   - **Model tier:** `adversary` (must be different model family from builder agents)
   - **Context:** ONLY the review package from Step A (fresh context, no prior conversation)
   - **Tools:** Read-only (`Read`, `Grep`, `Glob`)
   - **Context include:**
     - Changed/new source files from the review package
     - Changed/new test files
     - Relevant spec sections (PRD delta, architecture delta)
     - Story specs for implemented stories
   - **Context exclude:**
     ```
     # ▓ WALL: no implementation reasoning
     - ".factory/cycles/**/implementation/red-gate-log*"
     - ".factory/cycles/**/implementation/implementer-notes*"
     # ▓ WALL: no prior adversarial history
     - ".factory/cycles/**/adversarial-reviews/**"
     - ".factory/phase-f5-adversarial/**"
     # ▓ WALL: no semport history (DF-028)
     - ".factory/semport/**"
     # ▓ WALL: no per-story PR review findings
     - ".factory/code-delivery/*/review-findings.md"
     ```

2. **Instructions:** Review the delta for the categories defined in Step C.

## Artifacts

- Adversary agent spawned and running

## Success Criteria

- Adversary is a different model family from the builder
- Fresh context — no prior conversation history
- Only review package provided, not full codebase
- All information asymmetry walls enforced
