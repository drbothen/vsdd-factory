---
name: convergence-check
description: Run 7-dimension convergence validation — spec, tests, implementation, verification, visual, performance, documentation. Determines if the project is ready for release.
disable-model-invocation: true
allowed-tools: Read, Write, Bash, Glob, Grep
---

# Convergence Check

Phase 6 validation. Assess all 7 dimensions to determine if the project has converged.

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
