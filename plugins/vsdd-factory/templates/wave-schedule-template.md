---
document_type: wave-schedule
level: ops
version: "1.0"
status: draft
producer: orchestrator
timestamp: YYYY-MM-DDTHH:MM:SS
phase: 3
inputs: [STORY-INDEX.md, dependency-graph.md]
traces_to: STORY-INDEX.md
---

# Wave Schedule: [Project Name]

## Summary

| Metric | Value |
|--------|-------|
| Total stories | |
| Total waves | |
| Max parallelism (groups per wave) | |
| Estimated agent spawns | |

## Wave Plan

### Wave 1 (no dependencies)

| Group | Stories | Points | Complexity | Agent Scope |
|-------|---------|--------|-----------|-------------|
| A | S-1.01, S-1.02 | 3, 5 | S, M | 2 stories/agent |
| B | S-1.03 | 13 | XL | 1 story/agent |

### Wave 2 (depends on Wave 1)

| Group | Stories | Points | Complexity | Agent Scope |
|-------|---------|--------|-----------|-------------|

[Continue for all waves]

## Pipeline Overlap Plan

| Parallel Activity | When |
|------------------|------|
| Wave 2 stubs | Start when Wave 1 stubs compile |
| Wave 2 tests | Start when Wave 1 types available |
| Wave 2 implementation | Start after Wave 2 Red Gate verified |

## Critical Path

[Longest chain of dependent stories with total point estimate]
