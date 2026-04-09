---
name: decompose-stories
description: Decompose PRD and architecture into epics, stories, dependency graph, and wave schedule. Creates sprint-ready story files in .factory/stories/. Requires completed Phase 1 specs.
disable-model-invocation: true
allowed-tools: Read, Write, Edit, Bash, AskUserQuestion
---

# Decompose Stories

Break the PRD and architecture into implementable stories organized by dependency waves.

## Templates

Read and follow the output format in:
- `${CLAUDE_PLUGIN_ROOT}/templates/story-template.md` — STORY-NNN format
- `${CLAUDE_PLUGIN_ROOT}/templates/epic-template.md` — epic structure
- `${CLAUDE_PLUGIN_ROOT}/templates/wave-schedule-template.md` — wave schedule
- `${CLAUDE_PLUGIN_ROOT}/templates/holdout-scenario-template.md` — hidden acceptance scenarios
- `${CLAUDE_PLUGIN_ROOT}/templates/traceability-matrix-template.md` — requirement traceability

## Prerequisites

- `.factory/specs/prd.md` with behavioral contracts
- `.factory/specs/architecture/` with at least ARCH-INDEX.md
- Phase 1 should be complete (adversarial review passed)

### Reference Repos (conditional)

If `.factory/reference-manifest.yaml` exists, reference implementations inform story decomposition:
- Stories that implement behavior extracted from a reference repo should be tagged `implementation_strategy: gene-transfusion` and include a `Reference Source` field pointing to the relevant `.factory/semport/<project>/` artifacts.
- Stories that diverge from reference behavior should be tagged `implementation_strategy: from-scratch` and note the divergence in Dev Notes.
- When estimating complexity, factor in whether the reference implementation can be adapted vs. needs to be reimagined.

## Process

### 1. Define Epics

Group behavioral contracts into epics — cohesive chunks of user value:

```markdown
## Epic: <Name>
- **Goal:** <what user value this delivers>
- **BCs:** BC-1.01.001, BC-1.01.002, ...
- **Subsystems touched:** <list>
- **Estimated stories:** <count>
```

Write to `.factory/stories/epics.md`.

### 2. Create Stories

For each epic, break into stories. Each story should be:
- **Independently deliverable** — can be merged without other stories
- **Testable** — has clear acceptance criteria from BCs
- **Small enough** — 1-3 days of implementation work

Write each story to `.factory/stories/STORY-NNN.md` following `spec-format.md` story format.

### 3. Build Dependency Graph

Analyze story dependencies:
- Which stories must complete before others can start?
- Which stories are independent (can run in parallel)?

Write to `.factory/stories/dependency-graph.md`:

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

### 4. Assign Waves

Group stories into waves based on dependencies:
- **Wave 1**: Stories with no dependencies (foundation)
- **Wave 2**: Stories that depend only on Wave 1
- **Wave N**: Stories that depend only on completed waves

Assign each story a `wave` field in its file.

Write wave schedule to `.factory/cycles/<current>/wave-schedule.md`.

### 5. Create Story Index

Write `.factory/stories/STORY-INDEX.md`:

```markdown
# Story Index

| ID | Title | Epic | Wave | Status | Dependencies |
|----|-------|------|------|--------|-------------|
| STORY-001 | ... | ... | 1 | draft | none |
```

### 6. Initialize Sprint State

Write `.factory/stories/sprint-state.yaml`:

```yaml
current_wave: 1
stories:
  STORY-001:
    status: pending
    wave: 1
    branch: null
    worktree: null
  STORY-002:
    status: blocked
    wave: 2
    blocked_by: [STORY-001]
```

### 7. Create Holdout Scenarios

For each wave, create hidden acceptance scenarios in `.factory/holdout-scenarios/wave-scenarios/`:
- Scenarios the holdout evaluator will use
- Derived from BCs but phrased differently (black-box perspective)
- Focus on critical-path behaviors and edge cases

Write `.factory/holdout-scenarios/HS-INDEX.md`.

## Output Summary

Tell the user:
```
Story decomposition complete:
  Epics: <N>
  Stories: <N> across <N> waves
  Wave 1: <N> stories (ready to start)
  Holdout scenarios: <N>

Next: `/create-story STORY-001` to flesh out the first story, or `/deliver-story STORY-001` to start implementation.
```

## After Writing

1. Commit all files to factory-artifacts.
2. Invoke `state-update` to set phase to `phase-2`, status `completed`.
