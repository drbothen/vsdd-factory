---
name: pr-review-triage
description: >
  Finding classification and dispatch for PR review findings. Used by
  pr-manager to triage pr-reviewer comments into fix routes.
---

# PR Review Triage: Finding Classification & Dispatch

## When This Skill Runs

After the pr-reviewer posts REQUEST_CHANGES on a PR, the pr-manager
invokes this skill to classify each finding and route it to the
appropriate agent for resolution.

## Input

- PR review comments from pr-reviewer (via GitHub API or review-findings.md)
- Story spec (STORY-NNN.md) for context
- Current review cycle number

## Classification Process

### Step 1: Extract Findings

Read all pr-reviewer comments on the PR. Each comment should contain:
- Severity: blocking / suggestion / nit
- Category: coherence / coverage / description / size / missing / dependency
- Finding: description of the issue
- Suggestion: what to fix

### Step 2: Classify and Route

For each finding, determine the fix route:

| Category | Severity | Route To | Action |
|----------|----------|---------|--------|
| coherence | blocking | implementer | Remove unrelated changes from diff |
| coherence | suggestion | implementer | Clean up tangential changes |
| coverage | blocking | test-writer | Add missing tests for changed lines |
| coverage | suggestion | test-writer | Add edge case tests |
| description | any | pr-manager | Update PR body directly |
| size | blocking | pr-manager STOPS | Return to orchestrator with story split recommendation |
| size | suggestion | implementer | Extract helper commits |
| missing | blocking | implementer | Implement missing AC |
| missing | suggestion | implementer | Add missing handling |
| dependency | blocking | pr-manager | WAIT for upstream PR merge |
| dependency | suggestion | pr-manager | Note dependency status |

### Step 3: Dispatch Fixes

For each routed finding:
1. Create a task description for the target agent
2. Include the finding details and suggested fix
3. Specify the worktree path: `.worktrees/STORY-NNN/`
4. After fix: agent commits and pushes to the feature branch

### Step 4: Update Convergence Tracking

Write results to `.factory/code-delivery/STORY-NNN/review-findings.md`:
- Add new cycle row to convergence table
- Record each finding with ID, severity, category, resolution
- Update triage routing table

## Escalation Rules

- After 10 review cycles with blocking findings: escalate to human
- If a "size | blocking" finding: pr-manager STOPS the review loop and returns to orchestrator with story split recommendation
- If dependency is blocked: do NOT retry, just wait
- Nits are collected but do NOT block merge

## Output

- Updated `.factory/code-delivery/STORY-NNN/review-findings.md`
- Task descriptions dispatched to appropriate agents
- Status: `in-review` (fixes pending) or `converged` (all clear)

## Quality Gate

- [ ] Every pr-reviewer finding classified with severity and category
- [ ] Every finding routed to a specific agent with task description
- [ ] No findings left unclassified in review-findings.md
- [ ] Convergence table updated with current cycle row

## Failure Modes

- If pr-reviewer comments lack severity/category tags: infer from content and flag for pr-reviewer prompt improvement
- If a finding maps to no known route in the classification table: escalate to orchestrator
- If dependency-blocked findings exceed 3 per PR: flag potential story splitting need to orchestrator
