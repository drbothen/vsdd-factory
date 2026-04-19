---
name: convergence-check-shared-context
description: Shared context for convergence check steps. Contains templates, output format, and gate criteria.
---

# Convergence Check — Shared Context

This file is loaded by every step in the convergence-check skill.

## Templates

- `${CLAUDE_PLUGIN_ROOT}/templates/convergence-report-template.md` — convergence report structure
- `${CLAUDE_PLUGIN_ROOT}/templates/release-notes-template.md` — release notes format

## Output Location

All dimension results are written to `.factory/cycles/<current>/convergence-report.md`.

The first step creates the file with the Summary table. Each subsequent step fills in its row and detail section. The final step writes the overall verdict.

## Report Format

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

## Gate Criteria

- **CONVERGED**: All 7 dimensions pass → ready for release
- **NOT CONVERGED**: List remaining items with severity and estimated effort

## Allowed Tools

- Read, Write, Bash, Glob, Grep
- Model invocation is disabled (`disable-model-invocation: true`)
