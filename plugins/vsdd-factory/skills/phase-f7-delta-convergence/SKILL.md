---
name: phase-f7-delta-convergence
description: >
  Feature Mode Phase F7: Five-dimensional convergence check on the delta
  plus regression validation on the full codebase. Final human gate.
---

> **Delegation Reference:** This skill describes work the orchestrator delegates
> to specialist agents via the Agent tool. Each step names the target agent.
> The orchestrator does NOT execute these steps directly — it spawns the named
> agent for each step and reviews the output.

# Phase F7: Delta Convergence

## Prerequisites

- Phase F6 Targeted Hardening complete (all checks pass)
- `.factory/phase-f6-hardening/summary.md` exists

## Workflow

### Step 1: Five-Dimensional Convergence on Delta

Evaluate convergence across all five VSDD dimensions, scoped to the delta:

**Dimension 1: Spec Convergence**
- Every new/modified PRD requirement has corresponding implementation
- Adversarial spec review findings are cosmetic only
- Spec version is current and changelog is complete
- Metric: adversary novelty score < 0.15 on spec delta

**Dimension 2: Test Convergence**
- Every new acceptance criterion has at least one test
- Mutation kill rate >= 90% on changed files
- No vacuously true tests in the new test suite
- Metric: mutation kill rate on delta files

**Dimension 3: Implementation Convergence**
- Adversarial code review findings are cosmetic only
- No CRITICAL or HIGH findings remain open
- Adversary verification rate < 60% (hallucinating flaws)
- Metric: adversary finding verification rate on delta

**Dimension 4: Verification Convergence**
- All Kani proofs pass for new verification properties
- Fuzz testing clean after 5 min/target
- No security vulnerabilities in changed or new code
- Purity boundaries intact (no effectful code in pure modules)
- Metric: all proofs pass, fuzz clean, audit clean

**Dimension 5: Holdout Convergence**
- Run holdout scenarios (Phase 4 pattern) against the updated system
- Mean satisfaction score >= 0.85 across delta-relevant scenarios
- No must-pass scenario below 0.6 satisfaction
- Regression holdout scenarios (brownfield baseline) still pass
- Metric: holdout satisfaction score on delta + regression scenarios

### Step 2: Regression Validation

Separate from delta convergence, validate the FULL existing codebase:
- Run the complete test suite (new + existing)
- Compare against the Phase F4 regression baseline
- Verify zero regressions
- This is not "convergence" -- it is a binary pass/fail

### Step 2b: Cost-Benefit Analysis

Include cost-benefit data from the cost-tracker (DF-027):
- Total cost of this feature cycle (agent calls, LLM tokens)
- Cost per convergence dimension
- Projected cost of additional cycles (if not converged)
- Compare: P(finding in next iteration) * Value_avg vs Cost_iteration * 1.5
- Flag MAXIMUM_VIABLE_REFINEMENT_REACHED if cost exceeds expected value

### Step 3: Traceability Chain Extension

Update the traceability chain to include the new feature using 4-level hierarchy:

For each new requirement:
```
BC-S.SS.NNN -> VP-NNN -> test_xxx -> src/xxx.rs -> ADV-PASS-N -> KANI-xxx-PASS
```

For cross-references (new feature depends on existing feature):
```
BC-S.SS.NNN depends_on BC-S.SS.MMM (existing)
STORY-XXX extends STORY-YYY (existing)
```

Write the extended traceability chain to
`.factory/phase-f7-convergence/traceability-chain-delta.md`

Also update the main traceability chain at
`.factory/cycles/**/convergence/traceability-chain.md` by APPENDING (not replacing)
the new links.

### Step 4: Delta Convergence Report

Write the convergence report to
`.factory/phase-f7-convergence/delta-convergence-report.md`:

```markdown
# Delta Convergence Report: [Feature Name]

## Feature Summary
- Feature request: [link]
- Spec version: v1.X.0 -> v1.Y.0
- Stories implemented: STORY-XXX through STORY-XXX
- Files changed: N new, M modified

## Five-Dimensional Convergence (Delta)

| Dimension | Metric | Target | Actual | Status |
|-----------|--------|--------|--------|--------|
| Spec | Adversary novelty score | < 0.15 | X.XX | PASS/FAIL |
| Test | Mutation kill rate | >= 90% | XX% | PASS/FAIL |
| Implementation | Adversary verification rate | < 60% | XX% | PASS/FAIL |
| Verification | Proofs + fuzz + audit | All pass | X/Y pass | PASS/FAIL |
| Holdout | Satisfaction score | >= 0.85 | X.XX | PASS/FAIL |

## Regression Validation

| Metric | Baseline | Current | Status |
|--------|----------|---------|--------|
| Total tests | N | N+M | -- |
| Existing tests passing | N | N | PASS/FAIL |
| New tests passing | -- | M | PASS/FAIL |

## Traceability Chain

[New chain links listed]

## Recommendation

[READY FOR MERGE / NEEDS WORK: specific issues]
```

### Step 5: Human Authorization Gate

Present the Delta Convergence Report to the human.

The human must explicitly authorize the merge. This is the final gate --
the feature is either approved for integration or sent back for rework.

If sent back: identify which phase needs re-execution and restart from there.

Phase F7 is COMPLETE only when the human explicitly authorizes the merge.

After human approval, the release step follows:
- MINOR bump (new feature) or PATCH (enhancement)
- semver -> CHANGELOG -> tag -> gh release -> publish
- Notify human: feature shipped

### Convergence Failure Routing

If NOT CONVERGED after max 10 cycles, route to failing dimensions:
- Spec/Impl not converged -> F5 (adversarial) with fix PRs
- Tests not converged -> F6 (mutation + tests) with fix PRs
- Holdout not converged -> Holdout re-evaluation with fix PRs
- Fix PRs through code-delivery.lobster sub-workflow

## Output Artifacts

- `.factory/phase-f7-convergence/delta-convergence-report.md`
- `.factory/phase-f7-convergence/traceability-chain-delta.md`
- Updated: `.factory/cycles/**/convergence/traceability-chain.md`

## Quality Gate Criteria

- [ ] All five convergence dimensions pass on the delta
- [ ] Full regression suite passes (zero regressions)
- [ ] Traceability chain extended with new links (not replaced)
- [ ] Cross-references link new features to existing features they depend on
- [ ] Delta convergence report written with all metrics
- [ ] Cost-benefit analysis included (DF-027)
- [ ] Max 5 convergence cycles with cost-benefit escalation
- [ ] Fix PRs via code-delivery.lobster (FIX-F7-NNN)
- [ ] Human has explicitly authorized the merge
