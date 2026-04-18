---
name: orchestrator-per-story-delivery
description: Orchestrator workflow reference for the per-story TDD delivery cycle (red-green-refactor, PR, review, merge). Loaded by the orchestrator agent during the corresponding phase. Not directly invokable.
disable-model-invocation: true
---

> **Global Operating Rules:** Read `../../docs/FACTORY.md` and `../../docs/VSDD.md` for factory-wide constraints.


# Per-Story Code Delivery Cycle

**Canonical source** for per-story TDD delivery. The `deliver-story` skill (`skills/deliver-story/SKILL.md`) is the entry point that dispatches through this workflow; this file is the playbook. If the two disagree, this file wins — update the skill to match.

Reference file for the orchestrator. Load during Phase 3 implementation.

## Per-Story Delivery (within each wave)

1. Spawn devops-engineer: "Create worktrees for Wave N stories"
2. For each story (parallelized within wave groups):
   a. Spawn test-writer: "Create stubs in .worktrees/STORY-NNN/"
   b. Spawn test-writer: "Write failing tests in .worktrees/STORY-NNN/"
   c. Spawn implementer: "Implement via TDD in .worktrees/STORY-NNN/"
   d. Spawn demo-recorder: "Record per-AC demos in .worktrees/STORY-NNN/.
      Output to docs/demo-evidence/ (committed to feature branch).
      Use VHS for CLI or Playwright for web. Both success and error paths.
      Generate docs/demo-evidence/evidence-report.md."
   e. Spawn implementer: "Push feature/STORY-NNN to remote"
   f. Spawn pr-manager: "Run the full PR process for STORY-NNN.
      Feature branch: feature/STORY-NNN. Target: develop.
      Project path: <resolved-project-path>.
      Follow your 9-step process: populate PR description from template,
      verify demo evidence, create PR via github-ops, security review,
      pr-reviewer convergence loop, wait for CI, dependency check, merge.
      Do NOT skip any step. Read ../../templates/pr-description-template.md
      for the PR body format."
      NOTE: Do NOT compose the PR body or gh commands in this task —
      pr-manager owns the full PR lifecycle and uses its own templates.
   g. Spawn devops-engineer: "Remove worktree .worktrees/STORY-NNN"
3. After all stories in wave merged: run wave integration gate
4. Wave gate passes -> start next wave

## Wave Integration Gate

After all stories in a wave have merged to develop, run this loop (max 10 cycles):

a. Spawn implementer: "Run FULL test suite on merged develop"
b. Spawn adversary: "Review combined wave diff with fresh context"
c. Spawn code-reviewer: "Constructive code review of wave diff (post-adversary)"
d. Spawn security-reviewer: "Wave security review"
e. Spawn consistency-validator: "Cross-story consistency check — spec traceability,
   BC coverage, story dependency integrity across wave"
f. Spawn holdout-evaluator: "Re-evaluate affected holdout scenarios"
g. Spawn demo-recorder: "Record wave-level integration demos" (if multi-story wave)
h. If ALL pass → wave gate passes, proceed to next wave
i. If issues found:
   - Route findings per VSDD.md Feedback Integration Loop
   - Spawn pr-manager to triage findings into fix stories (STORY-NNN-FIX-001)
   - Run each fix story through per-story delivery (worktree → fix → demo → PR → review → merge)
   - Go back to (a) to re-validate
After 10 cycles: escalate to human.

## Dependency-Ordered Merge Coordination

Before allowing pr-manager to merge a story PR:
1. Read the story's `depends_on` from STORY-INDEX.md
2. For each dependency: verify its PR is already merged
3. If any dependency PR is NOT merged: hold this merge until it is
4. After all deps merged: proceed with merge

## Task Sizing Rules

When spawning implementers or test-writers:
- Check story points: S/M (1-5) -> max 2 stories per agent
- L/XL (8-13) -> exactly 1 story per agent
- NEVER combine "write code" and "run full test suite" in one agent task
- If an agent times out, spawn a NEW agent with narrower scope
- Before each implementation wave, verify the workspace compiles:
  spawn a test-writer with task "run cargo check and report status"

## Story Split Recovery

When pr-manager returns with "diff too large, recommend split":

1. **Close the PR** — Spawn github-ops: "Close PR without merging, add label `split-needed`"
2. **Keep the worktree** — Do NOT remove `.worktrees/STORY-NNN/` — the work is preserved
3. **Human approval** — Present the split recommendation to the human:
   "STORY-NNN PR is too large (>500 lines). Recommend splitting. Approve split?"
4. **Split the story** — Spawn story-writer: "Split STORY-NNN into smaller stories.
   Preserve the existing implementation in the worktree. Update STORY-INDEX.md,
   dependency-graph.md, and wave schedule with the new stories."
5. **Human reviews split** — Human approves the new story breakdown
6. **Create sub-worktrees** — Spawn devops-engineer: "Create worktrees for split stories,
   cherry-pick relevant commits from the original worktree"
7. **Resume per-story delivery** — Each split story goes through the normal cycle
   (tests → implementation → demo → PR → review → merge)
8. **Clean up original** — After all split stories merge, remove the original worktree

If human rejects the split, the orchestrator can override the size finding
(add a note to review-findings.md) and tell pr-manager to continue the review loop.

## Token Budget Check

Before spawning test-writer or implementer for a story, read the story's
**Token Budget Estimate** section:
- If estimated budget exceeds 60% of the target agent's context window,
  flag the story for splitting before proceeding
- If the Token Budget Estimate section is missing, spawn the story-writer
  to add it before proceeding

## Red Gate Delegation

For each story in the current wave:
1. Spawn test-writer: "Create compilable stubs for STORY-NNN" (Stub Architect)
2. Wait for completion. Verify cargo check passes.
3. If cargo check fails: spawn new test-writer to fix stubs
4. Spawn test-writer: "Write failing tests for STORY-NNN" (Test Writer)
5. Wait for completion. Verify cargo test compiles + all tests fail.
6. If Red Gate fails: spawn new test-writer to fix tests
7. Record both steps in red-gate-log

Only after Red Gate verified -> spawn implementer for STORY-NNN

## Verification Discipline

After every specialist dispatch, independently verify the exit condition before proceeding. Agent reports are claims — verification output is evidence. Run the verification command yourself and read the full output.

## Model Selection

Use the least powerful model that can handle each specialist dispatch. Mechanical tasks (worktree ops, stubs, demo recording) use fast/cheap models. Tasks requiring BC understanding or multi-file coordination use standard models. Review triage and architectural judgment use the most capable model.

If an agent reports BLOCKED or DONE_WITH_CONCERNS, consider re-dispatching with a stronger model before splitting the task.

## Adversarial Pass Context Injection

When spawning the adversary for Pass 2+:
1. Include FINDINGS.md (cross-pass tracker) in the context
2. Include the phase-specific prompt template
3. Explicitly state: "These findings were resolved since your last pass:
   [list]. Verify the fixes in Part A, then find NEW issues in Part B."

## State-Manager Ordering (MANDATORY)

State-manager runs LAST in every dispatch burst. Never dispatch state-manager concurrently with story-writer or product-owner — version-race pattern causes the same regression repeatedly. Wait for all artifact-producing agents to complete, then dispatch state-manager to update indexes and STATE.md.

## Context Discipline for Sub-Agent Spawning (DF-021)

When spawning sub-agents, pass only the specific detail files relevant to
the agent's task, not entire directories:
- **Implementer:** module-decomposition.md + dependency-graph.md + api-surface.md +
  relevant BC files
- **Formal-verifier:** verification-architecture.md + purity-boundary-map.md +
  tooling-selection.md + relevant VP files
- **Story-writer:** module-decomposition.md + dependency-graph.md
- **Test-writer:** api-surface.md + test-vectors.md + relevant BC files
- **Consistency-validator:** ARCH-INDEX.md + verification-coverage-matrix.md +
  BC-INDEX.md + VP-INDEX.md
