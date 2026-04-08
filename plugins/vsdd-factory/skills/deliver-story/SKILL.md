---
name: deliver-story
description: Execute per-story TDD delivery — create worktree, generate stubs, write failing tests (red), implement to green, create PR. Full test-first implementation workflow for a single story.
argument-hint: "[STORY-NNN]"
disable-model-invocation: true
allowed-tools: Read, Write, Edit, Bash, Glob, Grep, AskUserQuestion
---

# Deliver Story

Full TDD delivery for a single story. This is the core implementation workflow.

## Templates

Read and follow the output format in:
- `.claude/templates/red-gate-log-template.md` — red gate verification log
- `.claude/templates/cycle-manifest-template.md` — cycle tracking

## Input

`$ARGUMENTS` — story ID (e.g., `STORY-001`)

## Prerequisites

- Story must be `ready` in STORY-INDEX.md
- All dependency stories must be `completed`
- Sprint state must show this story as `pending` or `in-progress`

## Delivery Steps

### 1. Create Worktree

Follow the `worktree-protocol` rule (`.claude/rules/worktree-protocol.md`). Use the `/worktree-manage` skill:

```
/worktree-manage create STORY-NNN <description>
```

This creates `.worktrees/STORY-NNN/` on branch `feature/STORY-NNN-<desc>` from `develop`. If `develop` doesn't exist, worktree-manage creates it from `main`.

Verify the worktree is ready before proceeding:
```bash
cd .worktrees/STORY-NNN && git branch --show-current
```

### 2. Generate Stubs

In the worktree, create the module structure from the story's file list:
- Create directories
- Create stub files with types and function signatures
- Stubs must **compile** but not implement logic (return `todo!()` or `unimplemented!()`)

Commit: `feat(STORY-NNN): add module stubs`

### 3. Red Phase — Write Failing Tests

For each acceptance criterion / behavioral contract:
- Write a test that exercises the behavior
- The test must **fail** (red) because stubs don't implement logic
- Tests must be specific — test the contract, not the implementation

Commit: `test(STORY-NNN): add failing tests for <BC-ref>`

**Red Gate**: Run tests and verify they fail for the right reasons (not compilation errors).

```bash
cd .worktrees/STORY-NNN && cargo test 2>&1
```

All tests should fail with assertion errors, not build errors.

### 4. Green Phase — Implement

For each failing test:
1. Write the minimum code to make that test pass
2. Run the test suite
3. Micro-commit on each test pass: `feat(STORY-NNN): implement <behavior>`

Do NOT write code that isn't covered by a test. The test defines the contract.

### 5. Refactor Phase

With all tests green:
- Clean up implementation (extract functions, improve names)
- Ensure code follows project conventions (`.claude/rules/rust.md`)
- Run full lint: `cargo clippy --workspace --all-targets --all-features -- -D warnings`

Commit: `refactor(STORY-NNN): clean up implementation`

### 6. Gene Transfusion (if applicable)

If story has `implementation_strategy: gene-transfusion`:
- Read `.factory/semport/<module>-target-design.md`
- Read the reference source files listed in the story's tasks (paths into `.reference/<project>/`)
- Use the translation strategy instead of from-scratch coding
- Still follow Red→Green→Refactor — tests validate the translation
- Mark `// SEMPORT-REVIEW` on uncertain translations
- When the semport summary is insufficient, go read the actual code in `.reference/<project>/` — the summary is a map, not a substitute

### 7. Verify

- All tests pass
- Clippy clean
- Format clean: `cargo +nightly fmt --all --check`
- No `todo!()` or `unimplemented!()` remaining in production code

### 8. Create PR

Use `/pr-create` or create manually:
- Title: `feat(STORY-NNN): <story title>`
- Body: story context, mermaid dependency diagram, BC traceability
- Target: `develop`

### 9. Update State

- Update sprint-state.yaml: story status → `in-review`
- Update STORY-INDEX.md
- Commit state changes to factory-artifacts

## After Delivery

Tell the user:
```
Story STORY-NNN delivered:
  Tests: <N> passing
  Commits: <N> (TDD progression)
  PR: <link or instructions>

Next: PR review, then `/worktree-manage cleanup STORY-NNN` after merge.
```
