---
name: step-a-define-epics
description: Group behavioral contracts into epics — cohesive chunks of user value.
---

# Step A: Define Epics

> **Shared context:** Read `./_shared-context.md` before executing this step — it contains the Hard Gate, templates, prerequisites, and plan failure patterns.

Group behavioral contracts into epics — cohesive chunks of user value.

## Procedure

1. Read `.factory/specs/prd.md` and `.factory/specs/behavioral-contracts/BC-INDEX.md`
2. Group BCs into epics by cohesive user value
3. Write each epic with:

```markdown
## Epic: <Name>
- **Goal:** <what user value this delivers>
- **BCs:** BC-1.01.001, BC-1.01.002, ...
- **Subsystems touched:** <list>
- **Estimated stories:** <count>
```

## Artifacts

- `.factory/stories/epics.md`

## Success Criteria

- Every BC in the PRD is assigned to exactly one epic
- Each epic has a clear goal statement grounded in user value
- Subsystems touched align with ARCH-INDEX subsystem registry
