---
name: step-c-dependency-graph
description: Analyze story dependencies and identify independent groups for parallel execution.
---

# Step C: Build Dependency Graph

> **Shared context:** Read `./_shared-context.md` before executing this step.

Analyze story dependencies to determine execution order.

## Procedure

1. Read all `STORY-NNN.md` files
2. For each story, identify which other stories must complete first
3. Identify stories that are independent (can run in parallel)
4. Write the dependency graph:

```markdown
# Story Dependency Graph

## Dependencies
STORY-002 → STORY-001  (002 depends on 001)
STORY-003 → STORY-001
STORY-004 → STORY-002, STORY-003

## Independent Groups
[STORY-001]           # Wave 1
[STORY-002, STORY-003] # Wave 2 (both depend on 001)
[STORY-004]           # Wave 3
```

## Artifacts

- `.factory/stories/dependency-graph.md`

## Success Criteria

- Every story appears in the dependency graph
- No circular dependencies
- Independent groups are correctly identified
- Dependencies respect subsystem boundaries from architecture
