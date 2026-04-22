---
name: phase-f4-delta-implementation
description: >
  Feature Mode Phase F4: TDD implementation scoped to new stories only,
  with full regression suite as safety net.
---

> **Delegation Reference:** This skill describes work the orchestrator delegates
> to specialist agents via the Agent tool. Each step names the target agent.
> The orchestrator does NOT execute these steps directly — it spawns the named
> agent for each step and reviews the output.

# Phase F4: Delta Implementation (TDD)

## Prerequisites

- Phase F3 Incremental Stories complete and human-approved
- `.factory/phase-f3-stories/` contains new story specs
- Existing test suite passes (regression baseline established)

## Workflow

### Step 1: Establish Regression Baseline

Before writing any new code, run the FULL existing test suite:

```bash
# Language-specific -- the orchestrator selects the correct command
# Rust:
cargo test --workspace
# TypeScript:
npm test
# Python:
pytest
```

Record the result in `.factory/phase-f4-implementation/regression-baseline.md`:
- Total tests: N
- Passing: N
- Failing: 0 (if any fail, STOP -- fix regressions before proceeding)
- Timestamp

This baseline is the contract: after implementation, every one of these tests
must still pass.

### Step 2: Per-Story Delivery via Wave Loop

Implementation uses the per-story delivery flow (DF-024) organized in waves:

For each wave in the wave schedule (from F3):

**2a. Create Worktrees**
devops-engineer creates git worktrees for all stories in this wave:
`git worktree add .worktrees/STORY-NNN -b feature/STORY-NNN develop`

**2b. Per-Story (parallel within wave):**

Each story is delivered via the code-delivery.lobster sub-workflow which
encapsulates:

1. **Stubs (Two-Step Red Gate):** test-writer creates compilable stubs
2. **Tests:** test-writer writes failing tests (Red Gate: compile + fail)
3. **Implementation:** implementer via TDD in isolated worktree
   - Or Semport translation if story has `implementation_strategy: gene-transfusion`
   - Micro-commits per test
   - Follow existing code conventions
4. **E2E Tests:** e2e-tester (if UI/full-stack feature type)
5. **Demo:** demo-recorder records per-AC demos
6. **Push + PR:** squash, push --force-with-lease, create PR with Mermaid diagrams
7. **AI Review:** pr-reviewer (Gemini, information asymmetry wall: cannot see .factory/)
8. **Security Review:** security-reviewer (if CRIT/HIGH module from module-criticality.md)
9. **Converge:** max 10 review cycles
10. **Merge:** dependency check + merge to develop
11. **Cleanup:** devops-engineer removes worktree

**2c. Wave Integration Gate:**

After all stories in wave merge to develop:
- Full test suite (ALL tests, including existing + all prior waves)
- Adversary review of combined wave diff (information asymmetry wall)
- Security review of wave (if wave has CRIT/HIGH stories)
- Holdout regression on wave scenarios
- Accessibility audit (if UI product)
- Demo of wave-level integration
- Fix loop (max 10 cycles) if issues found

**2d. DTU Validation:**
If new DTU clones were built in this wave, validate fidelity.

### Step 3: Regression Baseline

Before writing any new code (before Step 2), run the FULL existing test suite
and record the baseline. This is the contract: after implementation, every one
of these tests must still pass.

### Step 5: Run New Tests (Green Gate)

Run the new test suite. All new tests must pass.
If any fail, iterate on the implementation until they pass.

### Step 6: Run Regression Suite

Run the FULL existing test suite (same command as Step 1):
- Compare against the baseline from Step 1
- Every previously-passing test must still pass
- If ANY regression is detected:
  1. Log the regression to `.factory/phase-f4-implementation/regression-log.md`
  2. Identify which implementation change caused the regression
  3. Fix the implementation (not the test)
  4. Re-run until regression suite is clean

The `plugins/src/regression-gate.ts` hook automates regression detection.

### Step 7: Refactor (if needed)

With all tests green (new + regression), refactor:
- Extract common patterns
- Improve naming
- Reduce duplication between new and existing code
- Re-run ALL tests after each refactor step

### Step 8: Implementation Summary

Write implementation summary to `.factory/phase-f4-implementation/summary.md`:
- Stories implemented (with links)
- New files created
- Existing files modified
- Test count: new tests added, total tests, regression result
- Any deviations from the story spec (must be justified)

Phase F4 is COMPLETE when all new tests pass AND the full regression suite passes.
No human gate -- this is an automated quality gate.

### Multi-Repo: Contract Change Propagation (DF-013)

When Phase F4 implementation changes an API contract (OpenAPI spec, protobuf, GraphQL schema):
1. The contract change is detected by the contract-steward (spec-steward role, see DF-002)
2. SDK regeneration is triggered in all consumer repos (see DF-013 SDK generation skill)
3. Consumer repos' regression suites are re-run against the new contract
4. If any consumer's tests fail, the contract change must be reviewed for backward compatibility
5. Contract testing (Pact/Specmatic) validates consumer-provider agreement

## Output Artifacts

- `.factory/phase-f4-implementation/regression-baseline.md`
- `.factory/cycles/<cycle-id>/<story-id>/implementation/red-gate-log.md`
- `.factory/cycles/<cycle-id>/<story-id>/implementation/regression-log.md`
- `.factory/phase-f4-implementation/summary.md`
- New test files (in project test directory)
- New implementation files (in project source directory)

## Quality Gate Criteria

- [ ] Regression baseline recorded before any new code (commit SHA preserved)
- [ ] Per-story delivery: worktree -> tests -> implement -> demo -> PR -> review -> merge
- [ ] Two-step Red Gate: stubs first, then tests
- [ ] All new tests pass (Green Gate)
- [ ] Full regression suite passes (zero regressions)
- [ ] pr-reviewer (Gemini) reviewed each story PR
- [ ] security-reviewer engaged for CRIT/HIGH modules
- [ ] PR review converges in max 10 rounds per story
- [ ] Wave integration gate runs after each wave merges
- [ ] Wave gate includes adversary, security, holdout, a11y (if UI)
- [ ] E2E tests run for UI stories (if UI/full-stack feature type)
- [ ] Implementation follows existing code conventions
- [ ] No modifications to files outside the Delta Analysis scope
- [ ] Implementation summary written with deviation log
- [ ] All story PRs merged to develop

## Failure Modes

- If story worktree cannot be created (branch conflict, disk space): report to orchestrator with the specific error before proceeding
- If tests do not compile after stub creation: spawn test-writer to fix compilation errors before implementation begins
- If regression suite fails after implementation: log regression in regression-log.md, identify the causal change, and fix the implementation (not the test)
- If a wave integration gate fails repeatedly (>3 cycles): escalate to human with the failing dimension
