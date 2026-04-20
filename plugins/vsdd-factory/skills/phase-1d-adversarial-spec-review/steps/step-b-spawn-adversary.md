---
name: step-b-spawn-adversary
description: Spawn the adversary agent with fresh context and read-only access to the spec package.
---

# Step B: Spawn Adversary

> **Shared context:** Read `./_shared-context.md` before executing this step.

Spawn the `adversary` agent with fresh context to review the spec package. The adversary is a different model family from the builder agents.

## Procedure

1. **Load policy rubric** (see shared context — Policy Rubric Auto-Loading).

2. **Spawn `adversary` agent** with:
   - **Model tier:** `adversary` (must be different model family from builder)
   - **Context:** FRESH — no prior conversation history, no prior review passes
   - **Tools:** Read-only (`Read`, `Grep`, `Glob`)
   - **Context include:**
     - `.factory/specs/domain-spec/L2-INDEX.md`
     - `.factory/specs/architecture/ARCH-INDEX.md`
     - `.factory/specs/architecture/*.md`
     - `.factory/specs/behavioral-contracts/**`
     - `.factory/specs/verification-properties/**`
     - `.factory/specs/architecture-feasibility-report.md`
     - `.factory/specs/prd.md`
     - `.factory/specs/prd-supplements/**`
     - `.factory/specs/module-criticality.md`
     - `.factory/specs/ux-spec/**` (if applicable)
   - **Context exclude:**
     - `.factory/holdout-scenarios/**` (train/test separation)
     - `.factory/specs/adversarial-reviews/**` (fresh perspective — no prior pass history)
     - `.factory/semport/**` (no brownfield ingestion history)

3. **Adversary review instructions** — attack the specs looking for:
   - Ambiguous language (could a requirement be interpreted two ways?)
   - Missing edge cases (what inputs/states aren't covered?)
   - Implicit unstated assumptions (what does the spec assume but not say?)
   - Contradictions between sections (do any specs conflict?)
   - Properties marked "testable only" that should be provable
   - Purity boundary violations
   - Verification tool mismatches
   - Security gaps (attack vectors not addressed)
   - Performance blind spots
   - Integration gaps (system boundary issues)

4. **Include the prompt template** from `${CLAUDE_PLUGIN_ROOT}/templates/adversary-prompt-templates/phase-1d-spec-review.md`.

5. **Include the policy rubric** under `## Project Policy Rubric` in the task prompt.

## Artifacts

- Adversary agent spawned and running

## Success Criteria

- Adversary is a different model family from the builder
- Fresh context — no prior conversation history provided
- Read-only tool access enforced
- Holdout scenarios excluded from context
- Prior adversarial reviews excluded from context
- Policy rubric injected into task prompt
