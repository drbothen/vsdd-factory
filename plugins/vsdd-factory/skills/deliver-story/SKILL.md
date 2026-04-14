---
name: deliver-story
description: Use when delivering a single story through the VSDD TDD pipeline. Dispatches fresh specialist subagents (test-writer → implementer → demo-recorder → pr-manager → devops-engineer) via the per-story-delivery orchestrator workflow. Each step runs in isolated context to preserve reasoning quality and prevent context exhaustion.
argument-hint: "[STORY-NNN]"
disable-model-invocation: true
allowed-tools: Read, Bash, Glob, Grep, AskUserQuestion, Task
---

# Deliver Story

<EXTREMELY-IMPORTANT>
This skill is a **dispatcher**, not an implementer. It does not write code, write tests, create worktrees, or open PRs directly. It reads the canonical workflow from `agents/orchestrator/per-story-delivery.md` and delegates each step to a fresh specialist subagent.

**Single-context delivery is a correctness bug**, not a shortcut. A single agent running all 9 steps suffers context exhaustion, loses the Red Gate discipline, and mixes test-writing judgment with implementation judgment — which is exactly what the specialist split is designed to prevent.
</EXTREMELY-IMPORTANT>

## The Iron Law

> **NO IMPLEMENTATION WITHOUT RED GATE VERIFICATION FIRST**

Violating the letter of the rule is violating the spirit of the rule. "I already know what the tests will say" is not a Red Gate.

## Announce at Start

Before any other action, say verbatim:

> I'm using the deliver-story skill to dispatch STORY-NNN through the per-story-delivery orchestrator workflow.

Then create a TodoWrite entry per step in the workflow. Mark each in-progress before dispatching and completed after the subagent returns.

## Input

`$ARGUMENTS` — story ID (e.g., `STORY-001`)

## Prerequisites (check before dispatching anything)

- Story status is `ready` in `.factory/stories/STORY-INDEX.md`
- All dependency stories are `completed` (read `depends_on` from the story file)
- Sprint state shows this story as `pending` or `in-progress`
- Story has a Token Budget Estimate section (dispatch story-writer to add one if missing)
- Story acceptance criteria trace to behavioral contracts (BC-S.SS.NNN)

If any prerequisite fails, STOP and report to the user. Do not dispatch anything.

## Canonical Source

The step-by-step workflow lives in **`agents/orchestrator/per-story-delivery.md`** and is the authoritative source. This skill is the entry point; the orchestrator file is the playbook. If the two ever disagree, the orchestrator file wins.

Read the orchestrator file at the start of every dispatch. Do not cache it between runs — it may have been updated.

## Dispatch Sequence

For each step below, launch a **fresh subagent** via the Task tool. Pass only the minimum context needed (see Context Discipline below). Wait for completion and verify the step's exit condition before dispatching the next one.

### Step 1 — Create worktree (devops-engineer)

Dispatch `devops-engineer` with task: "Create worktree `.worktrees/STORY-NNN/` on branch `feature/STORY-NNN-<desc>` from `develop`."

**Exit condition:** `git worktree list` shows the new worktree on the correct branch. Verify before proceeding.

### Step 2 — Generate stubs (test-writer as Stub Architect)

Dispatch `test-writer` with task: "Create compilable stubs in `.worktrees/STORY-NNN/` matching the story's file list. Use `todo!()` or `unimplemented!()` bodies. Commit: `feat(STORY-NNN): add module stubs`."

**Exit condition:** `cargo check` passes inside the worktree. If it fails, dispatch a new test-writer to fix stubs — do not proceed until clean.

### Step 3 — Write failing tests (test-writer as Test Writer)

Dispatch `test-writer` with task: "Write failing tests in `.worktrees/STORY-NNN/` for each acceptance criterion / BC. Commit: `test(STORY-NNN): add failing tests for <BC-ref>`."

**Red Gate (mandatory).** After dispatch returns, independently run `cd .worktrees/STORY-NNN && cargo test` and verify:

- Tests compile
- All new tests fail
- Tests fail with **assertion errors**, not build errors
- The failure messages reference the behavior under test (not "not yet implemented")

If Red Gate fails, dispatch a new test-writer to fix the tests. Do not proceed to implementation until Red Gate is green (i.e., tests are correctly red).

Record the Red Gate outcome in `.factory/stories/red-gate-log.md` following `${CLAUDE_PLUGIN_ROOT}/templates/red-gate-log-template.md`.

### Step 4 — Implement (implementer)

Dispatch `implementer` with task: "Implement in `.worktrees/STORY-NNN/` via TDD. For each failing test, write the minimum code to make it pass. Micro-commit per test: `feat(STORY-NNN): implement <behavior>`. Do not write code not covered by a test."

If the story has `implementation_strategy: gene-transfusion`, include in the task: "Read `.factory/semport/<module>/<module>-target-design.md` and the reference source files listed in the story. Use the translation strategy. Mark uncertain translations `// SEMPORT-REVIEW`."

**Exit condition:** all tests green, clippy clean, `cargo +nightly fmt --all --check` clean, zero `todo!()` / `unimplemented!()` in production code.

### Step 5 — Record demos (demo-recorder)

Dispatch `demo-recorder` with task: "Record per-AC demos in `.worktrees/STORY-NNN/docs/demo-evidence/`. Use VHS for CLI or Playwright for web. Capture both success and error paths. Generate `docs/demo-evidence/evidence-report.md`."

**Exit condition:** every acceptance criterion has at least one demo artifact referenced in the evidence report.

### Step 6 — Push feature branch (implementer)

Dispatch `implementer` with task: "Push `feature/STORY-NNN-<desc>` to remote origin."

**Exit condition:** `git ls-remote origin feature/STORY-NNN-<desc>` returns the expected SHA.

### Step 7 — PR lifecycle (pr-manager)

Dispatch `pr-manager` with task: "Run the full PR process for STORY-NNN. Feature branch: `feature/STORY-NNN-<desc>`. Target: `develop`. Follow your 9-step process: populate PR description from `${CLAUDE_PLUGIN_ROOT}/templates/pr-description-template.md`, verify demo evidence, create PR via github-ops, security review, pr-reviewer convergence loop, wait for CI, dependency check, merge. Do NOT skip any step."

**Do not compose the PR body yourself.** pr-manager owns the full PR lifecycle and uses its own templates. Your job here is delegation, not authorship.

**Exit condition:** pr-manager reports the PR merged (or reports a blocker that requires human intervention).

### Step 8 — Cleanup (devops-engineer)

Dispatch `devops-engineer` with task: "Remove worktree `.worktrees/STORY-NNN/` and delete local branch `feature/STORY-NNN-<desc>`."

**Exit condition:** `git worktree list` no longer shows the worktree; `git branch --list 'feature/STORY-NNN-*'` returns empty for this story.

### Step 9 — State update

Update `.factory/stories/sprint-state.yaml`: story status → `completed`.
Update `.factory/stories/STORY-INDEX.md`: status column for this story.
Commit to `factory-artifacts` branch: `factory(phase-3): STORY-NNN delivered`.

## Context Discipline for Dispatches

Pass only the specific files each specialist needs. Never pass the whole story file to every agent — that causes context exhaustion and topic drift.

| Specialist | Pass these files |
|---|---|
| devops-engineer | worktree protocol rules |
| test-writer (stubs) | story file, dependency-graph.md, api-surface.md, relevant BC files |
| test-writer (tests) | story file, api-surface.md, test-vectors.md, relevant BC files |
| implementer | story file, module-decomposition.md, dependency-graph.md, api-surface.md, relevant BC files |
| demo-recorder | story file, acceptance criteria extract only |
| pr-manager | story ID, feature branch name, PR template path |

If a story is too large to fit any specialist's budget (≥60% of target model's context window), STOP and dispatch story-writer to split it before proceeding.

## Model Selection

Use the least powerful model that can handle each dispatch:

| Task | Complexity signal | Model tier |
|------|------------------|------------|
| Worktree creation/cleanup | Always mechanical | Fast (cheapest) |
| Test stubs | Mostly mechanical, clear spec | Fast |
| Failing tests | Requires BC understanding | Standard |
| TDD implementation (S/M story) | Clear spec, 1-2 files | Standard |
| TDD implementation (L/XL story) | Multi-file, integration | Capable |
| Demo recording | Mechanical, follows patterns | Fast |
| PR lifecycle | Coordination, judgment calls | Standard |
| Review triage | Requires codebase understanding | Capable |

If an agent reports BLOCKED or produces low-quality output, re-dispatch with the next tier up — not the same tier.

## Task Sizing Rules

From `agents/orchestrator/per-story-delivery.md`:

- S/M stories (1-5 points) → max 2 stories per agent
- L/XL stories (8-13 points) → exactly 1 story per agent
- NEVER combine "write code" and "run full test suite" in one dispatch
- If an agent times out, dispatch a new agent with narrower scope — do not retry the same prompt

## Story Split Recovery

If pr-manager returns "diff too large, recommend split":

1. Dispatch `github-ops` to close the PR with label `split-needed`
2. KEEP the worktree — the work is preserved
3. Ask the human to approve the split
4. If approved: dispatch `story-writer` to split, then resume per-story delivery on each split story
5. If rejected: the human can override — add a note to `review-findings.md` and tell pr-manager to continue the review loop

## Red Flags

Stop and check yourself if you find yourself thinking any of these:

| Thought | Reality |
|---|---|
| "I'll just run the tests myself before dispatching test-writer" | That skips the Red Gate specialization. Dispatch. |
| "This story is small, one agent can do the whole thing" | Story size is orthogonal to specialist split. Dispatch each step. |
| "I already know what the implementation will look like, let me write it inline" | Your knowledge is not a Red Gate. Dispatch the test-writer first. |
| "The test-writer wrote bad tests, I'll fix them myself" | Dispatch a new test-writer with narrower scope. Do not hand-edit specialist output. |
| "pr-manager is taking too long, let me create the PR" | pr-manager owns the 9-step process. Wait or escalate to the human. |
| "The orchestrator file says X but I think Y is better" | Update the orchestrator file in a separate PR first, then re-run. |
| "Red Gate failed because the tests are too strict, let me relax them" | Red Gate failure means the test OR the understanding of the contract is wrong. Investigate. |
| "I'll skip demo-recording and do it after the merge" | Demos are part of the merge gate. Dispatch demo-recorder before pr-manager. |
| "The worktree cleanup can wait until later" | Stale worktrees accumulate. Dispatch devops-engineer now. |
| "This feels like a lot of context switching" | That's the feature. Fresh context per specialist is what prevents single-agent drift. |
| "I'll dispatch the implementer and reviewer at the same time" | Sequential, not parallel. Spec compliance review MUST complete before code quality review. |
| "Both stories can share an implementer agent" | Fresh agent per story. Shared context causes cross-contamination. |
| "The spec reviewer said it's fine, skip the code quality review" | Two-stage review is mandatory. Spec compliance and code quality check different things. |
| "I'll retry with the same model, maybe it'll work this time" | If an agent failed, something needs to change — more context, stronger model, or narrower scope. |

## After Delivery

Tell the user:

```
Story STORY-NNN delivered:
  Red Gate:       PASSED (see .factory/stories/red-gate-log.md)
  Implementation: <N> micro-commits
  Demos:          <N> artifacts in docs/demo-evidence/
  PR:             #<N> merged to develop
  Worktree:       cleaned up
  State:          sprint-state.yaml updated

Next: /wave-gate wave-N when all wave stories are complete.
```

## Templates

- `${CLAUDE_PLUGIN_ROOT}/templates/red-gate-log-template.md` — Red Gate verification log
- `${CLAUDE_PLUGIN_ROOT}/templates/cycle-manifest-template.md` — cycle tracking
- `${CLAUDE_PLUGIN_ROOT}/templates/pr-description-template.md` — PR body (owned by pr-manager, referenced here)

## Source of Truth

`agents/orchestrator/per-story-delivery.md` is the canonical workflow. This skill is the entry point that enforces dispatch discipline; the orchestrator file is the playbook specialists follow. Keep them in sync.
