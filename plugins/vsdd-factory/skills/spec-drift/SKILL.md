---
name: spec-drift
description: Compare implementation against spec documents to detect drift — behavioral contracts not implemented, architecture decisions violated, naming mismatches, missing error handling. Fresh context for objectivity.
disable-model-invocation: true
context: fork
agent: Explore
---

# Spec Drift Detection

Compare what was specified against what was built. Find divergences.

## Process

### 1. Load Specs

Read all spec documents:
- `.factory/specs/prd.md`
- `.factory/specs/behavioral-contracts/*`
- `.factory/specs/verification-properties/*`
- `.factory/specs/architecture/*`
- `.factory/specs/prd-supplements/*`

### 2. Scan Implementation

For each behavioral contract:
- Find the implementing code (grep for BC references, trace from architecture)
- Verify the behavior matches the contract
- Check that error cases are handled as specified
- Verify naming matches spec terminology

For each architecture decision:
- Verify the chosen option is what was implemented
- Check dependency direction matches the spec
- Verify component boundaries are respected

### 3. Check Naming Consistency

- Entity names in code match domain spec ubiquitous language
- Error types match error taxonomy
- API endpoints match interface definitions

### 4. Find Orphans

- Code that exists but has no spec coverage (unspecified behavior)
- Specs that exist but have no implementation (missing behavior)

## Templates

Use `${CLAUDE_PLUGIN_ROOT}/templates/spec-drift-report-template.md` for the spec drift report format.

## Output

Write to `.factory/cycles/<current>/spec-drift-report.md`:

```markdown
# Spec Drift Report

## Summary
- BCs checked: <N>
- BCs with drift: <N>
- Architecture violations: <N>
- Naming mismatches: <N>
- Orphaned code: <N> functions
- Unimplemented specs: <N> BCs

## Drift Details

### BC-S.SS.NNN: <title>
- **Spec says:** <expected behavior>
- **Code does:** <actual behavior>
- **Severity:** critical | important | cosmetic

### Architecture: <decision>
- **Spec says:** <chosen option>
- **Code does:** <what was actually implemented>

## Recommendations
<What to fix, in priority order>
```
