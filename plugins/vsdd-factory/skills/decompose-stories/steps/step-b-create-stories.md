---
name: step-b-create-stories
description: Break each epic into independently deliverable, testable stories with acceptance criteria traced to BCs.
---

# Step B: Create Stories

> **Shared context:** Read `./_shared-context.md` before executing this step — it contains templates, plan failure patterns, and reference repo tagging rules.

For each epic, break into stories. Each story should be:
- **Independently deliverable** — can be merged without other stories
- **Testable** — has clear acceptance criteria from BCs
- **Small enough** — 1-3 days of implementation work

## Procedure

1. Read `epics.md` and the relevant BCs for each epic
2. Decompose each epic into stories
3. Write each story to `.factory/stories/STORY-NNN.md` following `${CLAUDE_PLUGIN_ROOT}/templates/story-template.md`
4. Tag `implementation_strategy` per story:
   - `gene-transfusion` if implementing behavior from a reference repo (include Reference Source field)
   - `from-scratch` if diverging from reference behavior (note divergence in Dev Notes)

## Plan Failure Check

Before proceeding, scan every story for plan failures (see shared context). Fix any found.

## Artifacts

- `.factory/stories/STORY-NNN.md` — one file per story

## Success Criteria

- Every BC traces to at least one story's acceptance criteria
- No story has "TBD", "TODO", or placeholder acceptance criteria
- Each story is independently deliverable
- Story size is 1-3 days (no story exceeds 13 points)
