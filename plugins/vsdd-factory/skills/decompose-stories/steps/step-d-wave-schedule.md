---
name: step-d-wave-schedule
description: Assign stories to waves based on dependencies, create story index and sprint state.
---

# Step D: Wave Schedule + Story Index + Sprint State

> **Shared context:** Read `./_shared-context.md` before executing this step — it contains the wave schedule template.

Group stories into waves based on dependencies and create the tracking artifacts.

## Procedure

### 1. Assign Waves

- **Wave 1**: Stories with no dependencies (foundation)
- **Wave 2**: Stories that depend only on Wave 1
- **Wave N**: Stories that depend only on completed waves

Assign each story a `wave` field in its file. Write wave schedule to `.factory/cycles/<current>/wave-schedule.md` following `${CLAUDE_PLUGIN_ROOT}/templates/wave-schedule-template.md`.

### 2. Create Story Index

Write `.factory/stories/STORY-INDEX.md`:

```markdown
# Story Index

| ID | Title | Epic | Wave | Status | Dependencies |
|----|-------|------|------|--------|-------------|
| S-1.01 | ... | E-1 | 1 | draft | none |
```

### 3. Initialize Sprint State

Write `.factory/stories/sprint-state.yaml`:

```yaml
current_wave: 1
stories:
  S-1.01:
    status: pending
    wave: 1
    branch: null
    worktree: null
  S-1.02:
    status: blocked
    wave: 2
    blocked_by: [S-1.01]
```

## Artifacts

- `.factory/cycles/<current>/wave-schedule.md`
- `.factory/stories/STORY-INDEX.md`
- `.factory/stories/sprint-state.yaml`

## Success Criteria

- Every story has a wave assignment
- Wave assignments respect dependency graph (no story in wave N depends on a story in wave N or later)
- STORY-INDEX matches the individual story files
- sprint-state.yaml has entries for all stories
