---
name: wave-scheduling
description: >
  Computes wave-based implementation order from story dependencies.
  Groups stories into waves for parallel execution within each wave.
---

# Wave Scheduling

## Purpose

Automatically partition stories into implementation waves using the
dependency graph, then sub-partition waves into parallel groups based
on the one-story-per-agent rule.

## Algorithm

### Step 1: Topological Sort

Read the story dependency graph from STORY-INDEX.md.
Compute topological order. Stories with no dependencies → Wave 1.
Stories whose dependencies are all in prior waves → next wave.

### Step 2: Wave Assignment

| Wave | Contains |
|------|----------|
| Wave 1 | All stories with `depends_on: []` |
| Wave 2 | Stories whose dependencies are all in Wave 1 |
| Wave N | Stories whose dependencies are all in Waves 1..(N-1) |

### Step 3: Parallel Group Sub-Partitioning

Within each wave, partition stories into parallel groups:
- Max 2 S/M stories per group
- Max 1 L/XL story per group
- Each group gets its own test-writer → implementer sequence

### Step 4: Pipeline Overlap

Start Wave N+1 stubs while Wave N implementation is still running:
- Wave N+1 stubs don't depend on Wave N implementation (only types)
- Wave N+1 tests DO depend on Wave N types being available
- Run `cargo check` between stub creation and test writing

### Output

Produce `wave-schedule.md` under `.factory/cycles/**/implementation/`:

| Wave | Group | Stories | Test-Writer Scope | Implementer Scope |
|------|-------|---------|------------------|------------------|
| 1 | A | STORY-001, STORY-002 | 2 stories | 2 stories |
| 1 | B | STORY-003 | 1 story (XL) | 1 story (XL) |
| 2 | A | STORY-004, STORY-005 | 2 stories | 2 stories |
| ... | | | | |

### Step 5: Per-Story sprint-state.yaml Emission (BC-5.41.004)

After computing wave assignments, emit the `stories:` sequence in
`.factory/stories/sprint-state.yaml` as a producer obligation.

**Authority:** BC-5.41.004 (sprint-state.yaml producer contract) — producer MUST
write `stories:` as a YAML sequence of `{id, status}` objects, not as a
count-summary mapping.

**Consumer dependency:** BC-5.41.002 PC3 — consumer derives per-story status
from `stories[*].status: draft` entries; `pending` is a reserved no-op token.

**Ordering rule (BC-5.41.004 PC3 + EC-003):**
- Sort entries wave-ascending via topological sort of `depends_on:` edges from
  STORY-INDEX.md (same topo-sort computed in Steps 1-2 above)
- Wave level = `max(wave of direct deps) + 1`; stories with no deps = wave 0
- Tie-break within the same wave: story ID string ascending (lexicographic)

**Status values (BC-5.41.004 INV-1):**
Exactly 8 valid values: `draft`, `ready`, `in-progress`, `partial`, `blocked`,
`merged`, `withdrawn`, `cancelled`. Hard-abort on any other token (EC-007).

**Completeness (BC-5.41.004 PC4):**
Every non-retired story from STORY-INDEX.md MUST appear. Retired stories are
omitted. No phantom entries for stories not in STORY-INDEX.md.

**No `wave:` field (BC-5.41.004 INV-3):**
Each entry contains ONLY `id:` and `status:`. Do NOT add a `wave:` field or
any other key. Wave ordering is expressed by list position, not by an inline
field.

**Preserve existing sections:** `epics:`, `frontier:`, `next_refinement:`, and
`story_updates:` are independent keys in sprint-state.yaml and MUST be
preserved unchanged when updating the `stories:` list.

## Templates

Use `${CLAUDE_PLUGIN_ROOT}/templates/wave-schedule-template.md` for the wave schedule output format.

## Quality Gate

- [ ] All stories assigned to a wave (none orphaned)
- [ ] Dependencies respected -- no story scheduled before its dependencies
- [ ] No circular dependencies in the dependency graph
- [ ] Parallel groups respect size limits (max 2 S/M or 1 L/XL per group)

## Failure Modes

- If circular dependency found: report the exact cycle (e.g., STORY-004 -> STORY-007 -> STORY-004) and stop -- do not attempt to schedule
- If a story references a dependency that does not exist in STORY-INDEX.md: flag the missing dependency and exclude the story from scheduling
- If all stories have dependencies (no Wave 1 candidates): report "no root stories found" and stop
