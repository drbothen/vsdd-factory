---
name: convergence-check
description: Run 7-dimension convergence validation — spec, tests, implementation, verification, visual, performance, documentation. Determines if the project is ready for release.
disable-model-invocation: true
allowed-tools: Read, Write, Bash, Glob, Grep
---

# Convergence Check

Phase 7 validation. Assess all 7 dimensions to determine if the project has converged.

## The Iron Law

> **NO RELEASE WITHOUT ALL SEVEN DIMENSIONS CONVERGED**

Violating the letter of the rule is violating the spirit of the rule. "Six out of seven is good enough" is not convergence. A dimension that was skipped, mocked, or has stale data counts as NOT_CONVERGED. The human can override with documented rationale, but the orchestrator cannot.

## Red Flags

| Thought | Reality |
|---|---|
| "Six dimensions passed, one is borderline — ship it" | All seven must pass. Override requires human approval with documented rationale. |
| "The adversary converged so implementation must be fine" | Adversary convergence (Dim 1/3) does not imply test convergence (Dim 2) or verification convergence (Dim 4). Each is independent. |
| "Demo evidence exists for most stories" | Visual convergence (Dim 5) requires 100% coverage. Missing evidence = NOT_CONVERGED. |
| "The docs are mostly up to date" | Documentation convergence (Dim 7) requires zero stale references. "Mostly" = NOT_CONVERGED. |
| "Mutation kill rate is 89% for a HIGH module, close to 90%" | 90% is the threshold. 89% fails. No rounding. |
| "Holdout satisfaction is high so we can skip formal verification" | Behavioral validation (Dim 5) does not replace formal proofs (Dim 4). Both must converge. |
| "Performance budgets weren't defined, so Dim 6 passes by default" | Undefined budgets are a gap, not a pass. Define them or document the skip with rationale. |
| "We'll fix the remaining items post-release" | Post-release fixes are maintenance mode. Convergence means ready NOW, not ready eventually. |

## Templates

Read and follow the output format in:
- `${CLAUDE_PLUGIN_ROOT}/templates/convergence-report-template.md` — convergence report structure
- `${CLAUDE_PLUGIN_ROOT}/templates/release-notes-template.md` — release notes format

## The 7 Dimensions

### 1. Spec Convergence

- Read `.factory/cycles/<current>/adversarial-reviews/`
- Check latest adversary pass: is novelty LOW?
- All findings addressed or accepted?
- **Pass criteria**: Adversary critiques are nitpicks (wording, not missing behavior)

### 2. Test Convergence

- Run: `cargo test --release 2>&1`
- Check mutation testing results (from `/formal-verify`)
- **Pass criteria**: Mutation kill rate ≥ 90%, coverage ≥ 85%

### 3. Implementation Convergence

- All tests green
- No spec drift (from `/spec-drift`)
- All code review findings addressed
- No `todo!()`, `unimplemented!()`, or `FIXME` in production code
- **Pass criteria**: Clean build, clean lint, no known gaps

### 4. Verification Convergence

- All Kani proofs pass
- No fuzz crashes
- Purity boundaries intact (pure core has no side effects)
- **Pass criteria**: All formal verification green

### 5. Visual Convergence

- Demo recordings exist for all stories
- Design system compliance (if applicable)
- **Pass criteria**: Demo evidence covers all acceptance criteria

### 6. Performance Convergence

- Performance budgets met (from `/perf-check`)
- No benchmark regressions
- **Pass criteria**: All budgets within tolerance

### 7. Documentation Convergence

- CLAUDE.md updated with current architecture
- API docs generated (if applicable)
- README reflects current state
- **Pass criteria**: Docs match implementation

## Output

Write to `.factory/cycles/<current>/convergence-report.md`:

```markdown
# Convergence Report — <version>

## Summary

| Dimension | Status | Notes |
|-----------|--------|-------|
| 1. Spec | ✅/❌ | ... |
| 2. Tests | ✅/❌ | ... |
| 3. Implementation | ✅/❌ | ... |
| 4. Verification | ✅/❌ | ... |
| 5. Visual | ✅/❌ | ... |
| 6. Performance | ✅/❌ | ... |
| 7. Documentation | ✅/❌ | ... |

## Overall: CONVERGED | NOT CONVERGED

## Remaining Items
<List of what needs to happen before convergence>
```

## Gate

- **CONVERGED**: All 7 dimensions pass → ready for release
- **NOT CONVERGED**: List remaining items with severity and estimated effort
