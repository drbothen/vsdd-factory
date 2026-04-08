---
name: create-story
description: Create or refine a single story spec with full acceptance criteria, tasks, and implementation details. Takes a story ID and produces a sprint-ready story file.
argument-hint: "[STORY-NNN]"
disable-model-invocation: true
allowed-tools: Read, Write, Edit, Bash, AskUserQuestion
---

# Create Story

Flesh out a single story into a sprint-ready specification.

## Templates

Read and follow the output format in:
- `.claude/templates/story-template.md` — STORY-NNN format

## Input

`$ARGUMENTS` — story ID (e.g., `STORY-001`)

## Prerequisites

- Story file should exist in `.factory/stories/STORY-NNN.md` (at least a stub from decomposition)
- PRD and architecture docs available for reference

## Process

### 1. Read Context

- Read the story stub
- Read the related behavioral contracts
- Read relevant architecture sections
- Read dependency stories (if any) to understand the foundation
- **If story has `implementation_strategy: gene-transfusion`**: Read the referenced `.factory/semport/<project>/` artifacts. Include specific `.reference/<project>/<file>` paths in task descriptions so the implementer knows exactly which source files to study.

### 2. Validate Self-Containment

Run the story completeness checklist (from `.claude/rules/story-completeness.md`):
- All deliverable files specified
- Technical gotchas documented
- Prerequisites listed
- Internal consistency verified

### 3. Elaborate the Story

Ensure the story has:

**Acceptance Criteria** — one per behavioral contract, testable:
```markdown
- [ ] Given <precondition>, when <action>, then <expected outcome>
```

**Tasks** — ordered implementation steps:
```markdown
1. Create module structure
2. Write failing tests for BC-1.01.001
3. Implement until tests pass
4. ...
```

**Implementation Strategy**:
- `from-scratch` — write new code following architecture
- `gene-transfusion` — use semport to translate reference implementation

**Dev Notes** — gotchas, quirks, non-obvious decisions the implementer needs to know.

**File List** — every file this story creates or modifies.

### 4. Estimate Complexity

Rate the story:
- **S** (small): 1-2 files, straightforward
- **M** (medium): 3-5 files, some design decisions
- **L** (large): 6+ files, complex interactions
- **XL** (extra large): should be split into multiple stories

If XL, recommend splitting and ask the user.

## Output

Updated `.factory/stories/STORY-NNN.md` with full specification.

## After Writing

1. Commit to factory-artifacts.
2. Update STORY-INDEX.md status to `ready`.
3. Tell the user: "Story STORY-NNN is sprint-ready. Use `/deliver-story STORY-NNN` to start implementation."
