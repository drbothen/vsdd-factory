---
name: compact-state
description: Extract historical content from STATE.md into cycle files (burst logs, adversary passes, session checkpoints, lessons). Slims STATE.md to <200 lines.
disable-model-invocation: false
allowed-tools: Read, Write, Edit, Bash, Glob, Grep
---

# Compact State

Extract historical content from a bloated STATE.md into proper cycle-scoped files, leaving a lean status document.

## Announce at Start

Before any other action, say verbatim:

> I'm using the compact-state skill to extract historical content from STATE.md into cycle files.

## Prerequisites

- `.factory/STATE.md` exists
- `.factory/` is a git worktree on `factory-artifacts`
- `current_cycle` is set in STATE.md frontmatter (or `.factory/current-cycle` exists)

## Procedure

### Step 1: Read and Analyze

1. Read STATE.md fully
2. Count total lines
3. Identify the current cycle name from frontmatter `current_cycle` or `.factory/current-cycle`
4. Ensure `cycles/<cycle>/` directory exists (create if needed)

### Step 2: Identify Extractable Content

Scan for these section patterns that should be moved to cycle files:

| Pattern | Target File |
|---------|------------|
| `## Burst N` sections (burst narratives) | `cycles/<cycle>/burst-log.md` |
| `## Pass N` sections (adversary pass summaries) | `cycles/<cycle>/convergence-trajectory.md` |
| `adversary_pass_*` frontmatter fields | `cycles/<cycle>/convergence-trajectory.md` |
| `## Session Resume Checkpoint` sections (all except the LAST one) | `cycles/<cycle>/session-checkpoints.md` |
| `## Lessons` section content | `cycles/<cycle>/lessons.md` |
| Resolved rows from `## Blocking Issues` table | `cycles/<cycle>/blocking-issues-resolved.md` |
| `## Session Chain Summary` sections | `cycles/<cycle>/session-checkpoints.md` |

### Step 3: Extract to Cycle Files

For each extractable section:

1. **Read the section content** from STATE.md
2. **Append to the target cycle file** (create if it doesn't exist)
   - Each cycle file starts with a header: `# <Title> — <Cycle Name>`
   - Appended content is timestamped: `## Extracted from STATE.md on <date>`
3. **Preserve chronological order** in the target file

#### Burst Log Format

```markdown
# Burst Log — <cycle-name>

## Burst 1 (YYYY-MM-DD)
<burst narrative from STATE.md>

## Burst 2 (YYYY-MM-DD)
<burst narrative from STATE.md>
...
```

#### Convergence Trajectory Format

```markdown
# Convergence Trajectory — <cycle-name>

## Finding Progression

| Pass | Date | Findings | CRIT | HIGH | MED | LOW | Novelty | Counter |
|------|------|----------|------|------|-----|-----|---------|---------|
| 1 | YYYY-MM-DD | 29 | 6 | 9 | 9 | 5 | HIGH | 0/3 |
| 2 | YYYY-MM-DD | 24 | 6 | 7 | 6 | 5 | HIGH | 0/3 |
...

## Per-Pass Details

### Pass 1 (YYYY-MM-DD)
<pass details from STATE.md>
...
```

### Step 4: Slim STATE.md

After extracting, rewrite STATE.md keeping ONLY:

1. **Frontmatter** — remove `adversary_pass_*` fields. Keep: project, mode, phase, status, current_step, awaiting, current_cycle, dtu_required, plugin_version_adopted, historical_cycles
2. **Project Metadata** table
3. **Phase Progress** table (one row per phase, with trajectory shorthand in the Finding Progression column)
4. **Current Phase Steps** — last 5 rows only
5. **Decisions Log** — all (small, stable)
6. **Skip Log** — all (small, stable)
7. **Blocking Issues** — open only
8. **Phase Numbering Reconciliation** — keep if it exists (explains history)
9. **Latest Session Resume Checkpoint** — the most recent one only

Replace extracted sections with pointers:

```markdown
## Historical Content

Burst logs, adversary pass details, session checkpoints, and lessons
have been extracted to cycle files:

- Burst history: `cycles/<cycle>/burst-log.md`
- Convergence trajectory: `cycles/<cycle>/convergence-trajectory.md`
- Session checkpoints: `cycles/<cycle>/session-checkpoints.md`
- Lessons learned: `cycles/<cycle>/lessons.md`
- Resolved blockers: `cycles/<cycle>/blocking-issues-resolved.md`
```

### Step 5: Verify

1. Count lines in the new STATE.md — should be <200
2. Verify all cycle files were created and contain the extracted content
3. Verify no content was lost (extracted sections + remaining STATE.md = original)

### Step 6: Commit

```bash
cd .factory
git add -A
git commit -m "factory(state): compact STATE.md — extract historical content to cycle files"
```

## Output

Report:

```
STATE.md compaction complete:
  Before: <N> lines
  After:  <N> lines
  Extracted:
    - <N> burst narratives → cycles/<cycle>/burst-log.md
    - <N> adversary passes → cycles/<cycle>/convergence-trajectory.md
    - <N> session checkpoints → cycles/<cycle>/session-checkpoints.md
    - <N> lessons → cycles/<cycle>/lessons.md
    - <N> resolved blockers → cycles/<cycle>/blocking-issues-resolved.md
```

## Safety

- This skill ONLY moves content between files — it never deletes content
- All extracted content is written to cycle files BEFORE being removed from STATE.md
- If any write fails, abort without modifying STATE.md
- The git commit captures both the slim STATE.md and the new cycle files atomically
