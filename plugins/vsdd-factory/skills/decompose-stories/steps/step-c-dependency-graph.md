---
name: step-c-dependency-graph
description: Analyze story dependencies and identify independent groups for parallel execution.
---

# Step C: Build Dependency Graph

> **Shared context:** Read `./_shared-context.md` before executing this step.

Analyze story dependencies to determine execution order.

## Procedure

1. Read all `S-N.MM-<short>.md` story files
2. For each story, identify which other stories must complete first
3. Identify stories that are independent (can run in parallel)
4. Write the dependency graph:

```markdown
# Story Dependency Graph

## Dependencies
S-1.02 → S-1.01  (S-1.02 depends on S-1.01)
S-1.03 → S-1.01
S-1.04 → S-1.02, S-1.03

## Independent Groups
[S-1.01]              # Wave 1
[S-1.02, S-1.03]      # Wave 2 (both depend on S-1.01)
[S-1.04]              # Wave 3
```

## Artifacts

- `.factory/stories/dependency-graph.md`

## Success Criteria

- Every story appears in the dependency graph
- No circular dependencies
- Independent groups are correctly identified
- Dependencies respect subsystem boundaries from architecture
