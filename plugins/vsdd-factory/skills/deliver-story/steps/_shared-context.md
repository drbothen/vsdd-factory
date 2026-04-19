---
name: deliver-story-shared-context
description: Shared context loaded by all deliver-story step files. Contains the Iron Law, Red Flags, dispatch discipline, verification discipline, model selection, and context discipline.
---

# Deliver Story — Shared Context

This file is loaded by every step in the deliver-story skill. It contains cross-cutting constraints that apply to all steps.

## Dispatcher Identity

<EXTREMELY-IMPORTANT>
This skill is a **dispatcher**, not an implementer. It does not write code, write tests, create worktrees, or open PRs directly. It reads the canonical workflow from `agents/orchestrator/per-story-delivery.md` and delegates each step to a fresh specialist subagent.

**Single-context delivery is a correctness bug**, not a shortcut. A single agent running all 9 steps suffers context exhaustion, loses the Red Gate discipline, and mixes test-writing judgment with implementation judgment — which is exactly what the specialist split is designed to prevent.
</EXTREMELY-IMPORTANT>

## The Iron Law

> **NO IMPLEMENTATION WITHOUT RED GATE VERIFICATION FIRST**

Violating the letter of the rule is violating the spirit of the rule. "I already know what the tests will say" is not a Red Gate.

## Red Flags

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

## Verification Discipline

Never trust agent reports at face value. After every specialist dispatch:

1. **Run the verification command yourself** — test suite, build, lint
2. **Read the FULL output** — not just the summary
3. **Compare against the expected exit condition** for that step
4. **Only then proceed** to the next dispatch

Agent says "all tests pass" is a CLAIM, not EVIDENCE. `cargo test` output showing 34/34 pass is evidence.

If verification reveals the agent's report was inaccurate, dispatch a new agent to fix — do not trust subsequent claims from the same session.

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

## Lessons Learned

### Verification After Every Fix

After dispatching an agent to fix an adversarial finding, independently verify the fix:
1. Read the modified file to confirm the change landed
2. Grep for the original finding pattern to confirm it's gone
3. Grep for unintended side effects (truncated content, wrong context replacements)

In practice, S-1.13 was emptied by a bad sed replacement, requiring a full rewrite from scratch. Multiple "fixed" findings recurred because the fix was a phantom — the text changed but the semantic meaning didn't.

## Prerequisites (check before dispatching anything)

- Story status is `ready` in `.factory/stories/STORY-INDEX.md`
- All dependency stories are `completed` (read `depends_on` from the story file)
- Sprint state shows this story as `pending` or `in-progress`
- Story has a Token Budget Estimate section (dispatch story-writer to add one if missing)
- Story acceptance criteria trace to behavioral contracts (BC-S.SS.NNN)

If any prerequisite fails, STOP and report to the user. Do not dispatch anything.

## Canonical Source

The step-by-step workflow lives in **`agents/orchestrator/per-story-delivery.md`** and is the authoritative source. If this skill and the orchestrator file ever disagree, the orchestrator file wins.

## Templates

- `${CLAUDE_PLUGIN_ROOT}/templates/red-gate-log-template.md` — Red Gate verification log
- `${CLAUDE_PLUGIN_ROOT}/templates/cycle-manifest-template.md` — cycle tracking
- `${CLAUDE_PLUGIN_ROOT}/templates/pr-description-template.md` — PR body (owned by pr-manager)
