---
name: create-story
description: Create or refine a single story spec with full acceptance criteria, tasks, and implementation details. Takes a story ID and produces a sprint-ready story file.
argument-hint: "[STORY-NNN]"
disable-model-invocation: true
allowed-tools: Read, Write, Edit, Bash, AskUserQuestion
---

## Hard Gate

Do NOT skip to implementation or deliver the story before it is fully elaborated. Every mandatory section MUST be completed — no stub stories.

# Create Story

Flesh out a single story into a sprint-ready specification.

## Templates

Read and follow the output format in:
- `${CLAUDE_PLUGIN_ROOT}/templates/story-template.md` — STORY-NNN format

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

## Plan Failures

These patterns invalidate a story. If you catch any, fix before proceeding:

- "TBD", "TODO", or "implement later" in any section
- "Add appropriate error handling" without specifying which errors
- "Write tests for the above" without actual test descriptions
- "Similar to STORY-NNN" without repeating the relevant details
- Acceptance criteria without testable assertions
- File list that says "and other files as needed"
- Tasks that describe what to do without specifying how

## Output

Updated `.factory/stories/STORY-NNN.md` with full specification.

## Lessons Learned (apply to ALL projects)

### Centralized Version Pins

The story MUST include the external dependency table from `dependency-graph.md` verbatim in its "Library & Framework Requirements" section. Do not invent version numbers — reference the centralized pin. Version mismatches were the most persistent finding class in practice (multiple passes to resolve).

### Forbidden Dependencies Section

Every story MUST include a "Forbidden Dependencies" section listing crates/packages that must NOT appear in the implementing module's dependency graph. State these as compile-time enforcement rules. Example: "core-engine must NOT depend on query-runtime — export descriptors and register in the query module instead."

### Error Taxonomy Compliance

Stories MUST only reference error codes from the error taxonomy (`prd-supplements/error-taxonomy.md`). Do not invent codes outside the taxonomy or reuse codes with wrong semantics. If a new error code is needed, explicitly flag it: "NEW — add E-xxx-NNN to taxonomy with description: ..."

## Self-Review (before delivery)

Before marking this story as sprint-ready, check your own work:

1. **Completeness:** All mandatory template sections filled with real content?
2. **Consistency:** Do BC references exist in BC-INDEX? Do file paths match architecture?
3. **Testability:** Every acceptance criterion has a testable assertion?
4. **Context budget:** Token estimate present and under 60% of agent context?

Fix issues inline. This is a cheap filter — catch obvious gaps before delivery.

## After Writing

1. Commit to factory-artifacts.
2. Update STORY-INDEX.md status to `ready`.
3. Tell the user: "Story STORY-NNN is sprint-ready. Use `/deliver-story STORY-NNN` to start implementation."
