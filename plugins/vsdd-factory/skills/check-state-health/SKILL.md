---
name: check-state-health
description: Diagnostic check of STATE.md — validates structure, size, phase numbering, frontmatter, and content routing compliance. Reports HEALTHY, WARNINGS, or NEEDS-COMPACT.
disable-model-invocation: true
allowed-tools: Read, Glob, Grep, Bash
---

# Check State Health

Validate STATE.md against VSDD standards. This is a diagnostic — it reads and reports but does not modify any files.

## Announce at Start

Before any other action, say verbatim:

> I'm using the check-state-health skill to validate .factory/STATE.md.

## Checks

### 1. Existence

- `.factory/STATE.md` exists
- `.factory/` is a git worktree on `factory-artifacts`

If STATE.md does not exist, report `NO STATE` and stop.

### 2. Frontmatter Validation

Read STATE.md frontmatter and verify:

| Field | Required | Valid Values |
|-------|----------|-------------|
| `document_type` | yes | `pipeline-state` |
| `project` | yes | non-empty string |
| `mode` | yes | `greenfield`, `brownfield`, `feature`, `maintenance`, `discovery`, `multi-repo` |
| `phase` | yes | `0`, `1`, `2`, `3`, `4`, `5`, `6`, `7` (integer, not compound like `2-story-decomposition-patch-cycle`) |
| `status` | yes | `in_progress`, `complete`, `blocked` |
| `current_step` | yes | non-empty string |
| `current_cycle` | no | string matching a directory in `cycles/` |

Flag any non-standard `phase` values (e.g., `2-story-decomposition-patch-cycle` should be `2`).

### 3. Size Check

Count lines in STATE.md:

| Lines | Verdict |
|-------|---------|
| 0-200 | HEALTHY |
| 201-500 | WARNING — recommend `/vsdd-factory:compact-state` |
| 501+ | NEEDS-COMPACT — historical content must be extracted |

### 4. Phase Numbering

Grep STATE.md for stale phase references:

- `Phase 3.5` or `phase: 3.5` → should be Phase 4
- `Phase 4.*adversar` → should be Phase 5
- `Phase 5.*formal` or `Phase 5.*harden` → should be Phase 6
- `Phase 6.*converg` → should be Phase 7

Report each stale reference with line number.

### 5. Structure Compliance

Check that these sections exist in STATE.md:

- `## Project Metadata` (with table)
- `## Phase Progress` (with table)
- `## Current Phase Steps` (with table)
- `## Decisions Log`
- `## Skip Log`
- `## Blocking Issues`

Flag missing sections.

### 6. Content Routing Compliance

Check for content that should NOT be in STATE.md:

| Pattern | Problem | Should Be In |
|---------|---------|-------------|
| More than 10 `## Burst` or `## Pass` sections | Burst narratives accumulated | `cycles/<cycle>/burst-log.md` |
| More than 1 `## Session Resume Checkpoint` | Old checkpoints not archived | `cycles/<cycle>/session-checkpoints.md` |
| More than 20 `adversary_pass_*` frontmatter fields | Per-pass tracking in frontmatter | `cycles/<cycle>/convergence-trajectory.md` |
| `## Lessons` section with >5 entries | Lessons accumulated | `cycles/<cycle>/lessons.md` |
| Resolved blocking issues still listed | Closed blockers not archived | `cycles/<cycle>/blocking-issues-resolved.md` |

### 7. Convergence Counter

If a convergence counter exists, verify format: `N of 3` where N is 0-3.

## Output

Report as a table:

```markdown
# STATE.md Health Check

| Check | Status | Details |
|-------|--------|---------|
| Existence | PASS/FAIL | |
| Frontmatter | PASS/WARN/FAIL | [issues] |
| Size | HEALTHY/WARNING/NEEDS-COMPACT | [N lines] |
| Phase numbering | PASS/FAIL | [N stale references] |
| Structure | PASS/WARN | [missing sections] |
| Content routing | PASS/WARN/FAIL | [N violations] |
| Convergence counter | PASS/WARN/N/A | |

**Overall: HEALTHY / WARNINGS / NEEDS-COMPACT**
```

## When to Run

- At the start of every session (alongside `/vsdd-factory:factory-health`)
- After any phase transition
- Before declaring convergence
- When the state-size hook fires a warning
