---
name: decompose-stories
description: Decompose PRD and architecture into epics, stories, dependency graph, and wave schedule. Creates sprint-ready story files in .factory/stories/. Requires completed Phase 1 specs.

allowed-tools: Read, Write, Edit, Bash, AskUserQuestion
---

## The Iron Law

> **NO STORY WITHOUT BC TRACEABILITY FIRST**

Every story must trace to at least one behavioral contract. Every AC must trace to a BC precondition or postcondition. A story that cannot answer "which BC does this implement?" is a story that cannot be verified. Do NOT skip to implementation or story delivery — ALL stories MUST be decomposed, dependency-ordered into waves, and approved before any code is written.

## Red Flags

| Thought | Reality |
|---|---|
| "This story is obvious, I don't need to trace it to a BC" | Untraced stories drift from the spec. Trace it. |
| "The dependency graph looks right, I don't need to verify acyclicity" | Circular dependencies deadlock wave scheduling. Verify programmatically. |
| "This story is too big but I'll split it during implementation" | Stories over 13 points must be split NOW. Implementation is too late. |
| "All BCs are covered, I can skip the coverage check" | BC-to-story coverage gaps are the #1 source of missing features. Verify every BC has a story. |
| "The wave schedule is obvious from the dependency graph" | Wave scheduling must respect dependency ordering AND resource constraints. Verify both. |
| "Holdout scenarios can wait until after decomposition" | Holdout scenarios inform story completeness. Write them alongside stories. |
| "This acceptance criterion is clear enough without a BC trace" | "Clear enough" becomes "ambiguous enough" during implementation. Add the trace. |
| "I'll add the missing stories in the next cycle" | Missing stories compound. The adversary will find them — fix now. |

# Decompose Stories

Break the PRD and architecture into implementable stories organized by dependency waves.

## Templates

Read and follow the output format in:
- `${CLAUDE_PLUGIN_ROOT}/templates/story-template.md` — S-N.MM format
- `${CLAUDE_PLUGIN_ROOT}/templates/epic-template.md` — epic structure (E-N format)
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

## Scope Check

Before decomposing, verify the PRD describes a single product. If it contains multiple independent products or platforms, stop and split the PRD first — each product gets its own decomposition cycle.

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

Write each story to `.factory/stories/S-N.MM-<short>.md` following `spec-format.md` story format.

Story IDs use `S-N.MM` (section.story zero-padded). The `N` is the section/epic
grouping (matches the parent epic `E-N`); `MM` is the zero-padded story number
within that section. Example: `S-1.01`, `S-3.15`. The `S` in BC IDs (BC-S.SS.NNN)
and the `N` in story IDs (S-N.MM) are intentionally different hierarchies — BC `S`
is the subsystem number, story `N` is the epic/wave grouping. Stories cross
subsystem boundaries via the `subsystems: [SS-NN, ...]` frontmatter array.

### 3. Build Dependency Graph

Analyze story dependencies:
- Which stories must complete before others can start?
- Which stories are independent (can run in parallel)?

Write to `.factory/stories/dependency-graph.md`:

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
| S-1.01 | ... | E-1 | 1 | draft | none |
```

### 6. Initialize Sprint State

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

### 7. Create Holdout Scenarios

For each wave, create hidden acceptance scenarios in `.factory/holdout-scenarios/wave-scenarios/`:
- Scenarios the holdout evaluator will use
- Derived from BCs but phrased differently (black-box perspective)
- Focus on critical-path behaviors and edge cases

Write `.factory/holdout-scenarios/HS-INDEX.md`.

## Plan Failures

These patterns invalidate a story. If you catch any, fix before proceeding:

- "TBD", "TODO", or "implement later" in any section
- "Add appropriate error handling" without specifying which errors
- "Write tests for the above" without actual test descriptions
- "Similar to S-N.MM" without repeating the relevant details
- Acceptance criteria without testable assertions
- File list that says "and other files as needed"
- Tasks that describe what to do without specifying how

## Self-Review (before adversarial review)

Before routing to adversarial review, check your own work:

1. **Spec coverage:** Does every BC in the PRD trace to at least one story? List any gaps.
2. **Placeholder scan:** Any "TBD", stub stories, or incomplete dependency mappings? Fix them.
3. **Consistency:** Do story IDs match the index? Do wave assignments respect dependencies?
4. **Sizing:** Any story over 13 points? Any story estimated over 60% of agent context window?

Fix issues inline. This is a cheap filter — catch obvious gaps before spending tokens on the adversary.

## Output Summary

Tell the user:
```
Story decomposition complete:
  Epics: <N>
  Stories: <N> across <N> waves
  Wave 1: <N> stories (ready to start)
  Holdout scenarios: <N>

Next: `/create-story S-1.01` to flesh out the first story, or `/deliver-story S-1.01` to start implementation.
```

## After Writing

1. Commit all files to factory-artifacts.
2. Invoke `state-update` to set phase to `phase-2`, status `completed`.
