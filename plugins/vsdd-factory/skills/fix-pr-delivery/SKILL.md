---
name: fix-pr-delivery
description: >
  Streamlined delivery flow for fix PRs created during adversarial refinement,
  formal hardening, and convergence. Same rigor as story PRs (worktree, AI
  review, security review) but skips stubs, Red Gate, and wave integration gates.
---

# Fix PR Delivery Flow

## When This Skill Runs

When adversarial refinement, formal hardening, or convergence produce findings
that require code fixes on the merged develop branch. Fix tasks are named
`FIX-P[phase]-NNN` (e.g., FIX-P4-001, FIX-P5-003).

## Differences from Story PR Delivery

| Aspect | Story PR | Fix PR |
|--------|---------|--------|
| Branch prefix | `feature/STORY-NNN` | `fix/FIX-P4-NNN` |
| PR title | `feat(STORY-NNN): ...` | `fix(FIX-P4-NNN): ...` |
| Demo required | Always (per AC) | Only if behavior-changing |
| PR description | Full template with diagrams | Streamlined -- links to finding |
| Wave gate after | Yes (wave integration gate) | No -- fix PRs merge individually |
| Stubs + Red Gate | Yes (two-step) | No -- fix goes directly to implementation |

## Workflow

### Step 1: Orchestrator Creates Fix Task

When a phase finding requires a code fix, the orchestrator creates a fix task:

```
Finding: [finding ID from adversarial/hardening/convergence]
Fix Task: FIX-P[phase]-NNN
Description: [one-line description of what needs fixing]
Source: [adversarial | hardening | convergence]
Severity: [CRITICAL | HIGH | MEDIUM | LOW]
```

### Step 2: DevOps-Engineer Creates Worktree

```bash
git worktree add .worktrees/FIX-P4-001 -b fix/FIX-P4-001 develop
```

### Step 3: Implementer Fixes in Worktree

The implementer works in `.worktrees/FIX-P[phase]-NNN/`:

1. Read the finding report for context
2. Apply the fix
3. Run tests -- all must pass
4. Commit with message: `fix(FIX-P[phase]-NNN): [description]`

### Step 4: Demo Recording (Conditional)

Demo-recorder records a demo ONLY if the fix is behavior-changing:

- **Behavior-changing fixes:** Changes to output, error messages, CLI flags,
  API responses, security restrictions, or any user-observable behavior.
  Record a demo showing the corrected behavior.
- **Transparent fixes:** Refactoring, performance improvements, internal error
  handling, code style. No demo needed.

### Step 5: Push Branch

```bash
cd .worktrees/FIX-P[phase]-NNN/
git push --force-with-lease origin fix/FIX-P[phase]-NNN
```

### Step 6: PR-Manager Creates PR

Create a streamlined PR description:

```markdown
## Fix: FIX-P[phase]-NNN

**Finding:** [link to finding report and finding ID]
**Phase:** [4 | 5 | 6]
**Severity:** [CRITICAL | HIGH | MEDIUM | LOW]

### What Changed
[Brief description of the fix]

### Why
[Link to the finding that triggered this fix]

### Testing
- [ ] All existing tests pass
- [ ] New test added for the specific finding (if applicable)
- [ ] Demo recorded (if behavior-changing)
```

pr-manager spawns github-ops to create the PR, and spawns pr-reviewer to review:

### Step 7: PR-Reviewer Reviews Fix Diff

The pr-reviewer (4th model family) reviews the fix PR diff:
- Same 8-item checklist as story PRs
- Smaller scope -- only the fix changes
- Focus on: does the fix actually address the finding?

### Step 8: Security Review (Conditional)

If the fix is security-related (CWE/OWASP finding, dependency vulnerability,
auth/authz change):

```
security-reviewer (T2, NO IMPL REASONING):
  Input: Fix PR diff only
  Action: Verify the fix addresses the security finding without
          introducing new vulnerabilities
  Output: APPROVE / REQUEST_CHANGES via github-ops
```

### Step 9: Review Convergence

Max 10 review cycles:
1. PR-reviewer posts findings
2. PR-manager triages findings, routes to implementer
3. Implementer fixes in worktree, pushes
4. PR-reviewer re-reviews
5. Repeat until APPROVE or 10 cycles exhausted

### Step 10: Merge and Cleanup

pr-manager spawns github-ops to merge, then orchestrator spawns devops-engineer and state-manager for cleanup:
```
github-ops: "cd <project-path> && gh pr merge --squash --delete-branch"
devops-engineer: "cd <project-path> && git worktree remove .worktrees/FIX-P[phase]-NNN"
state-manager: "Update STATE.md with FIX-P[phase]-NNN completion — merge status, PR number, timestamp"
```

## Source-Specific Re-Verification After Fix PRs Merge

After fix PRs merge to develop, re-verification depends on the source phase:

### Adversarial Fixes

Re-run adversary with fresh context -- full adversarial pass on develop.

### Hardening Fixes

1. **Adversarial "lite" review first:** Adversary (fresh context) reviews ONLY
   the fix diffs, not a full codebase re-review.
2. **Re-run ONLY failing checks:**
   - If Kani VP-003 failed: re-run only VP-003 (locked VPs are not re-run)
   - If fuzz target crashed: re-run only that fuzz target
   - Do NOT re-run already-passing checks (mutation, security, purity)
3. If still fails: another fix cycle (max 10 per finding)
4. If passes: hardening gate clears

### Convergence Fixes

Route to the specific failing dimension:
- Implementation not converged -> another adversarial pass
- Holdout not converged -> re-evaluate holdout scenarios, fix, re-evaluate
- Tests not converged -> additional testing
- Verification not converged -> additional formal verification

Then re-run convergence assessment.

## Quality Gate Criteria

- [ ] Fix PR uses `fix/FIX-P[phase]-NNN` branch naming
- [ ] PR title uses `fix(FIX-P[phase]-NNN): ...` format
- [ ] Fix was developed in isolated worktree
- [ ] PR-reviewer (4th model) reviewed the fix diff
- [ ] Security-reviewer reviewed if security-related fix
- [ ] All tests pass after fix
- [ ] Demo recorded if behavior-changing fix
- [ ] Worktree cleaned up after merge
