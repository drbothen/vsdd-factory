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

### Spec-Path Discipline (canonical repo-root paths only)

**All spec, BC, and ADR files passed to specialists MUST be canonical repo-root absolute paths** — paths rooted at the main checkout where the `factory-artifacts` branch is mounted at `.factory/`. For example: `/path/to/repo/.factory/specs/behavioral-contracts/BC-5.39.001.md`.

**No `.factory/` directory is created in a story worktree at `git worktree add` time.** `.factory/` is gitignored on the product branch, so no shadow directory is created at checkout time. Any `.factory/` content found in a story worktree is therefore live shadow-write evidence (issue #523 class) — an agent wrote to a CWD-relative `.factory/` path while operating from inside the worktree; it is NOT a stale snapshot of the canonical tree. Such shadow content is neither tracked on `factory-artifacts` nor ever updated. It is off-limits for spec ground-truth and MUST be reported as a defect signal (not dismissed as a pathing artifact). Passing any worktree-local `.factory/` path to the adversary or any spec-reading specialist causes phantom "absent BC", "missing story spec", or "outdated spec" findings. The spec ground-truth — including STORY specs in `.factory/stories/` — comes ONLY from `<canonical-repo-root>/.factory/`.

**Enforcement:** Before building the context package for any specialist dispatch involving spec files, the orchestrator MUST resolve the canonical repo-root path for each spec file and pass that path — not `<worktree>/.factory/<anything>`. If the canonical path cannot be resolved (e.g., factory-artifacts worktree is not mounted), STOP and report to the human before dispatching.

#### Write Discipline — `.factory/**` artifact writes from story worktrees (BC-6.26.001 PC1, Invariants 1, 3, 4)

All `.factory/**` artifact writes performed during story delivery MUST use canonical absolute paths
anchored to the main-checkout root. CWD-relative paths (e.g., `.factory/stories/S-NNN-DELIVERY.md`
resolved from the story worktree CWD) are FORBIDDEN — such writes land silently in the story
worktree's shadow `.factory/` subtree and are permanently destroyed at teardown (issue #523
gitignored-shadow mechanism; BC-6.26.001 Invariant 5).

**Load-bearing cases (BC-6.26.001 Invariant 4):** The DELIVERY ledger (`*-DELIVERY.md`),
story-frontmatter files, and `pr-review.md` records are the primary artifacts at risk. All writes
to any `.factory/**` path are covered by this rule — not only DELIVERY ledgers.

**Canonical root determination (BC-6.26.001 Invariant 3):** The canonical `.factory/` root MUST
be determined via one of two methods:

- `CANONICAL_FACTORY_ROOT` — orchestrator-provided variable. **This variable holds the absolute
  path to the MAIN-checkout repository root** (e.g., `/abs/path/to/repo`) — NOT the `.factory/`
  mount directory itself. The canonical artifact path is `$CANONICAL_FACTORY_ROOT/.factory/<artifact>`;
  if the mount path were passed instead, you would get `$CANONICAL_FACTORY_ROOT/.factory/.factory/<artifact>` nesting.
  Orchestrator dispatch preambles SHOULD provide this variable. When absent, agents MUST resolve it
  via the git method below.
  **Assert non-empty before expanding:** never expand `$CANONICAL_FACTORY_ROOT` without first
  verifying it is set and non-empty — an unset expansion yields `/.factory/...`, outside the repo.

- `git -C <main-worktree-path> rev-parse --show-toplevel` — where `<main-worktree-path>` is the
  path of the MAIN checkout, NOT the story worktree path. To obtain `<main-worktree-path>` from
  inside a story worktree, run `git worktree list --porcelain` from ANY worktree — the FIRST
  `worktree` entry in the output is always the main checkout. Extract its path and use that as
  `<main-worktree-path>`:
  ```bash
  main_worktree_path="$(git worktree list --porcelain | while IFS= read -r line; do
    case "$line" in
      "worktree "*) printf '%s\n' "${line#worktree }"; break ;;
    esac
  done)"
  [ -n "$main_worktree_path" ] || {
    echo "HALT: canonical factory root could not be resolved — git worktree list returned no main worktree path; cannot proceed"
    exit 1
  }
  CANONICAL_FACTORY_ROOT="$(git -C "$main_worktree_path" rev-parse --show-toplevel)"
  ```

**WARNING (EC-006):** Running `git -C <story-worktree-path> rev-parse --show-toplevel` — i.e.,
using the story worktree path instead of the main worktree path — returns the story worktree root,
NOT the main checkout root. The two roots differ, and using the story worktree root defeats the
purpose of canonical path resolution. Always resolve from the MAIN worktree path or use the
pre-provided `CANONICAL_FACTORY_ROOT`.

- **Correct:** `Write(file_path="$CANONICAL_FACTORY_ROOT/.factory/stories/S-NNN-DELIVERY.md", ...)`
- **Forbidden:** `Write(file_path=".factory/stories/S-NNN-DELIVERY.md", ...)` (relative path — silently writes to shadow tree)

### Story-Size Gate

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

For the full post-approval procedure — including split execution, sub-worktree creation, and
original worktree cleanup — follow `agents/orchestrator/per-story-delivery.md §Story Split
Recovery` (the canonical 8-step version). In particular, step 8 of that version requires running
the §G.1 preflight (step-g-cleanup.md §G.1, BC-6.26.001 PC2) before removing the original
worktree. Cross-referencing rather than duplicating prevents drift when the canonical procedure
evolves.

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
