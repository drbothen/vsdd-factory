---
name: phase-f5-scoped-adversarial
description: >
  Feature Mode Phase F5: Adversarial review scoped to changed/new code only.
  Fresh context, different model family.
---

> **Delegation Reference:** This skill describes work the orchestrator delegates
> to specialist agents via the Agent tool. Each step names the target agent.
> The orchestrator does NOT execute these steps directly — it spawns the named
> agent for each step and reviews the output.

# Phase F5: Scoped Adversarial Review

## Prerequisites

- Phase F4 Delta Implementation complete (all tests green, regression clean)
- `.factory/phase-f4-implementation/summary.md` exists
- `.factory/phase-f1-delta-analysis/affected-files.txt` exists

## Workflow

### Step 1: Prepare Review Package

Build the scoped review package for the Adversary. Include ONLY:

**Changed files** (from `.factory/phase-f1-delta-analysis/affected-files.txt`):
- New source files
- Modified source files (provide full file, not just diff -- the Adversary needs context)
- New test files

**Context files** (read-only, for reference):
- Relevant spec sections (PRD delta, architecture delta)
- Story specs for the implemented stories
- Existing conventions documentation (FACTORY.md, coding rules)

**Explicitly excluded** (do NOT provide to Adversary — information asymmetry walls, DF-025):
- Unchanged source files (unless they are direct dependents of changed code)
- Previous adversarial review reports (fresh perspective required)
- Implementation notes or rationale (the Adversary should judge the code, not the intent)
- Phase F4 TDD logs and implementer session notes
- .factory/semport/** (gene transfusion history, DF-028)
- .factory/cycles/**/implementation/red-gate-log*, implementer-notes*

### Step 2: Spawn Primary Adversary (Fresh Context)

Spawn `adversary` agent with:
- Model: adversary model (must be different model family from builder agents)
- Context: ONLY the review package from Step 1 (fresh context, no prior conversation)
- Instructions: Review the delta for the following categories

### Step 3: Adversary Review Categories

The Adversary evaluates the delta across these dimensions:

**3a. Spec Fidelity**
- Does every new requirement from the PRD delta have corresponding implementation?
- Do modified requirements have updated implementation?
- Are acceptance criteria from story specs met?

**3b. Regression Risk**
- Could any change to an existing file break existing behavior?
- Are there implicit dependencies that the tests might not cover?
- Are error paths in modified code still correct?

**3c. Convention Adherence**
- Does new code follow the same naming patterns as existing code?
- Are error types structured consistently with existing error types?
- Is the module structure consistent with the architecture?

**3d. Security Review**
- Are there new trust boundaries introduced?
- Is input validation present at new entry points?
- Are error messages sanitized (no internal details leaked)?

**3e. Test Quality**
- Are boundary cases tested?
- Are negative cases tested (invalid input, error paths)?
- Could any test pass vacuously (silent failure)?

### Step 4: Adversary Report

The Adversary writes findings to
`.factory/phase-f5-adversarial/adversarial-delta-review.md`:

Each finding must include:
- Severity: CRITICAL / HIGH / MEDIUM / LOW / COSMETIC
- Category: spec-fidelity / regression-risk / convention / security / test-quality
- File and line reference
- Description of the issue
- Suggested fix (optional)

### Step 5: Triage and Fix

Route findings to responsible agents:
- CRITICAL / HIGH: must be fixed before proceeding
- MEDIUM: should be fixed, Orchestrator decides
- LOW / COSMETIC: documented but not blocking

For each fix:
1. Agent makes the fix
2. Re-run relevant tests (new + regression)
3. Verify the fix addresses the finding

### Step 6: Re-Review (if needed)

If CRITICAL or HIGH findings were fixed:
- Spawn Adversary again with fresh context
- Include only the FIXED files (not the full delta again)
- Iterate until no CRITICAL or HIGH findings remain

Convergence criterion: Adversary's findings are all MEDIUM or below,
and novelty score < 0.15 (findings are cosmetic or repeated).

### Step 7: Secondary Adversarial Pass (review-tier model — Review Tier)

After adversary convergence, optionally spawn a secondary review using
`review/primary` (review-tier model) for cognitive diversity on the delta:

- Recommended for: security-critical delta, large delta spanning many files
  (Gemini's 1M context can review the entire delta + dependent files in one pass),
  or when maximum cognitive diversity is valued
- NOT recommended for: trivial bug fixes or cosmetic changes
- Write secondary findings to `.factory/phase-f5-adversarial/gemini-review.md`
- Any new CRITICAL/HIGH findings route through Step 5/6 fix cycle

### Step 8: Adversarial Convergence Report

Write convergence summary to
`.factory/phase-f5-adversarial/convergence-summary.md`:
- Number of review rounds (per model)
- Findings by severity (initial vs final)
- Novelty score per round
- Cross-model unique findings (findings from review-tier not found by adversary, and vice versa)
- Final verdict: CONVERGED / NOT-CONVERGED

Phase F5 is COMPLETE when primary adversary (adversary model) convergence is reached on the delta.
Secondary adversary (review-tier) findings are additive — they can only extend, not replace.
No human gate -- this is an automated quality gate.

### Fix PR Delivery (DF-025)

When F5 finds issues on merged develop, fixes go through the per-story delivery
flow via code-delivery.lobster:
- FIX-F5-NNN -> worktree -> fix -> demo (if behavior-changing) -> PR -> AI review
  -> security review (if applicable) -> merge
- Then re-verify only failing checks

### Holdout Regression (conditional)

If F5 fixes are behavior-changing, re-evaluate affected holdout scenarios
to verify no regressions were introduced by the fixes.

### Security Review Touchpoint (DF-025)

If the adversary identifies security findings, security-reviewer performs
a dedicated CWE/OWASP analysis (Security Review Touchpoint #3). The
security-reviewer cannot see the adversary's implementation reasoning
(information asymmetry wall).

### Multi-Repo: Contract Compliance Validation (DF-013)

For cross-repo deltas that modify API contracts:
- Run **contract testing** (Pact/Specmatic) to validate that all consumer repos still
  comply with the updated contract
- The adversary reviews both the contract change AND the consumer-side adaptations
- Any breaking contract change without corresponding consumer updates is a CRITICAL finding

## Output Artifacts

- `.factory/phase-f5-adversarial/adversarial-delta-review.md`
- `.factory/phase-f5-adversarial/gemini-review.md` (if secondary pass was run)
- `.factory/phase-f5-adversarial/convergence-summary.md`
- `.factory/phase-f5-adversarial/round-N-review.md` (one per review round)
- `.factory/phase-f5-adversarial/contract-compliance.md` (multi-repo projects only)

## Quality Gate Criteria

- [ ] Review scoped to delta files only (not full codebase)
- [ ] Primary adversary (adversary model) uses different model family from Builder
- [ ] Fresh context for each review round (no carryover)
- [ ] For multi-repo projects: contract compliance validated across all consumer repos
- [ ] All CRITICAL and HIGH findings resolved
- [ ] Primary adversary convergence reached (novelty score < 0.15)
- [ ] Secondary adversary (review-tier) pass completed for security-critical deltas
- [ ] Regression suite still passes after all fixes
