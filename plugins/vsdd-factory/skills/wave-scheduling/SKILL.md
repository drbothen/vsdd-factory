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

**Authority:** BC-5.41.004 v1.3 PC3 (sprint-state.yaml producer contract) —
producer MUST write `stories:` as a YAML sequence of `{id, status}` objects,
not as a count-summary mapping. Ordering governed by ADR-026 §Decision 3a
two-partition algorithm (PC3: terminal partition precedes non-terminal). EC-010
narrows the TopoViolation guard: tolerate supersession edges (terminal→superseded
non-terminal); abort genuine anomalies (terminal→active non-terminal).

**Consumer dependency:** BC-5.41.002 PC3 — consumer derives per-story status
from `stories[*].status: draft` entries; `pending` is a reserved no-op token.

**Two-partition ordering algorithm (BC-5.41.004 PC3 + ADR-026 §Decision 3a):**

The `stories:` sequence MUST be emitted as two contiguous, non-interleaved
partitions — Partition A (terminal) then Partition B (non-terminal):

1. **Classify** each STORY-INDEX story as terminal
   (`merged` / `withdrawn` / `cancelled`) or non-terminal (all other statuses).

2. **Partition A — terminal stories:**
   Apply Kahn/DFS topological sort over `depends_on:` edges **restricted to
   edges between terminal stories only** (cross-partition edges from terminal to
   non-terminal are excluded from the sort). Tie-break within the same wave
   ordinal: story ID string ascending (lexicographic, EC-003).

3. **Partition B — non-terminal stories:**
   Apply Kahn/DFS topological sort over `depends_on:` edges **restricted to
   edges between non-terminal stories only**. Same lex tie-break (EC-003).

4. **Emit** Partition A (all terminal entries) then Partition B (all
   non-terminal entries). No terminal entry may appear after any non-terminal
   entry (BC-5.41.001 P-SPRINT-STATE-WAVE-ORDER precondition).

5. **TopoViolation guard (narrowed — ADR-026 §Decision 3a v1.36 / BC-5.41.004 v1.3 EC-010):**
   Before sorting, for each terminal story T in the classified set:
     for each dep_id in T.depends_on:
       if dep_id is non-terminal in STORY-INDEX.md:
         read dep_id's story-file frontmatter for a `superseded_by:` field
           (plain working-tree read: `grep -m1 '^superseded_by:' .factory/stories/<dep_id>-*.md`
            — NOT git exec; INV-4 compliant)
         if superseded_by: PRESENT  → TOLERATE: exclude this edge from the Partition-A
           intra-partition topo-sort; continue (no abort)
         if superseded_by: ABSENT   → HARD-ABORT: "TopoViolation: terminal story <T.id>
           depends_on non-terminal story <dep_id>"; no sprint-state.yaml write

**Wave ordinal definition (INV-3-compatible):**
- Restricted Kahn: wave 1 = stories with no intra-partition deps; wave N+1 =
  stories whose all intra-partition dependencies are in waves 1..N.
- No `wave:` field is written to `stories:` entries. Wave ordering is expressed
  by list position only (BC-5.41.004 INV-3).

**Status values (BC-5.41.004 INV-1):**
Exactly 8 valid values: `draft`, `ready`, `in-progress`, `partial`, `blocked`,
`merged`, `withdrawn`, `cancelled`. Hard-abort on any other token (EC-007).

**Completeness (BC-5.41.004 PC4):**
Every non-retired story from STORY-INDEX.md MUST appear in exactly one
partition. Retired stories are omitted. No phantom entries for stories not in
STORY-INDEX.md.

**Preserve existing sections (PC5):** `epics:`, `frontier:`, `next_refinement:`,
and `story_updates:` are independent keys in sprint-state.yaml and MUST be
preserved byte-identical when updating the `stories:` list.

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
