---
name: state-update
description: Update .factory/STATE.md with pipeline phase transitions and commit to factory-artifacts branch. Internal skill called by other skills — not invoked directly by users.
user-invocable: false

allowed-tools: Bash, Read, Edit
---

# State Update

Update `.factory/STATE.md` to reflect pipeline progress.

## Templates

Read and follow the output format in:
- `${CLAUDE_PLUGIN_ROOT}/templates/state-template.md` — STATE.md structure
- `${CLAUDE_PLUGIN_ROOT}/templates/factory-project-state-template.md` — project state tracking Called by other skills at phase transitions.

## Input

The calling skill provides:
- `phase`: New phase identifier (e.g., `phase-1`, `phase-2`, `phase-4`)
- `status`: Phase status (e.g., `in-progress`, `completed`, `blocked`)
- `notes`: Optional notes about the transition

## Procedure

### 1. Read current state

```bash
cat .factory/STATE.md
```

Parse the YAML frontmatter for current `phase` and `pipeline` status.

### 2. Update frontmatter

Update the YAML frontmatter fields:

```yaml
---
pipeline: <RUNNING|PAUSED|COMPLETED|BLOCKED>
phase: <new phase>
product: corverax
mode: <greenfield|brownfield|feature>
timestamp: <current ISO8601>
previous_phase: <old phase>
---
```

### 3. Append to phase history table

Add a new row to the Phase History table:

```markdown
| <phase> | <status> | <YYYY-MM-DD> | <notes> |
```

### 4. Commit to factory-artifacts

```bash
cd .factory
git add STATE.md
git commit -m "factory(<phase>): <status> — <brief description>"
```

## Pipeline Status Values

| Status | Meaning |
|--------|---------|
| `INITIALIZED` | Factory set up, no work started |
| `RUNNING` | Active phase in progress |
| `PAUSED` | Human-requested pause |
| `BLOCKED` | Waiting on dependency or decision |
| `COMPLETED` | All phases done, ready for release |

## Phase Identifiers

| Phase | Name |
|-------|------|
| `pre-1` | Initialization |
| `phase-1` | Spec Crystallization |
| `phase-2` | Story Decomposition |
| `phase-3` | Test-First Implementation (per wave) |
| `phase-4` | Holdout Evaluation |
| `phase-4` | Adversarial Refinement |
| `phase-5` | Formal Hardening |
| `phase-6` | Convergence |
| `release` | Release & Handoff |
