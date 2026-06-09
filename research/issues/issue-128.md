# Issue #128 — pr-manager claims to delete the remote branch on merge but the branch often survives

**Date:** 2026-06-09
**Issue:** #128 (label: `bug`) — *"pr-manager claims to delete the remote branch on merge but the branch often survives"*
**Validator:** research-agent
**Branch/commit at validation:** `develop` @ `82163b7f`

---

## Restated Question

`pr-manager`'s 9-step playbook treats `gh pr merge --squash --delete-branch` exit 0 as proof the remote feature branch was deleted (Step 9 conceptually emits "remote branch deleted by gh merge"). In practice the branch often survives on origin: the gh exit code means "delete requested," not "branch gone." Stories ship "successfully" but leave dangling feature branches at the pre-squash SHA needing manual cleanup. Observed twice on `adamson34/otsniff` at vsdd-factory `1.0.0-rc.16` (PR #45, #46 of 4 in one session; the other two deleted cleanly — a transient/queued condition). The current workaround is a manual `git ls-remote origin refs/heads/feature/<branch>` + `git push origin --delete` when the leak appears. Proposed fix: follow the `gh pr merge` call with a `git ls-remote` verification, push-delete if non-empty, and only emit STEP_COMPLETE when `ls-remote` returns empty — applying the same "trust but verify / agent claim ≠ evidence" discipline `deliver-story` preaches elsewhere, to pr-manager's own Step 9.

---

## Codebase Grounding

### The merge step trusts `--delete-branch` with no verification

`plugins/vsdd-factory/agents/pr-manager.md`:
- **Step 8: Execute merge** (lines 228-249) dispatches `gh pr merge <PR_NUMBER> --squash --delete-branch` (line 236) and then "YOU must verify the merge succeeded" — but "verify" here means the *merge*, not the *branch deletion*. No `git ls-remote` follow-up.
- **Step 9: Post-merge** (lines 251-258) "Trigger worktree cleanup and state updates … emit STEP_COMPLETE: step=9 name=post-merge status=ok note=cleanup complete." There is **no** assertion that the remote branch is actually gone. The issue's description of Step 9 emitting "remote branch deleted" maps to this unconditional completion.
- "Dependency-Ordered Merge" (lines 302-312) and "Constraints" (lines 41-47) likewise have no remote-branch-deletion verification.

### The false assumption is stated explicitly in the sibling skill

`plugins/vsdd-factory/skills/code-delivery/SKILL.md`, Step 10 Post-Merge Cleanup (lines 186-194):

> *"1. Remote branch deleted (`--delete-branch` handles this)"*

This is the precise incorrect assumption the issue identifies — `--delete-branch` is treated as a guarantee. Sibling-sweep (TD-VSDD-060) flags this line as needing the same fix.

`plugins/vsdd-factory/skills/fix-pr-delivery/SKILL.md:132` and the workflow files (`workflows/code-delivery.lobster:412`, `workflows/greenfield.lobster:784`) all issue `gh pr merge --squash --delete-branch` with no `ls-remote` verification — same gap, multiple callsites.

### The engine already has the verifier primitive — just not applied here

- `plugins/vsdd-factory/skills/deliver-story/steps/step-f-pr-lifecycle.md:16` and `per-story-delivery.md:183` already use `git ls-remote origin feature/STORY-NNN-<desc>` as an **exit condition** for the push step.
- `plugins/vsdd-factory/skills/factory-worktree-health/SKILL.md:80` uses `git ls-remote --heads origin ${BRANCH_NAME}`.
- `_shared-context.md` "Verification Discipline" (lines 60-69): *"Agent says 'all tests pass' is a CLAIM, not EVIDENCE."*

So the exact verification primitive and the "trust but verify" doctrine already exist in-repo; they are simply not applied to pr-manager's branch-deletion step. The fix is consistent with established patterns.

### No prior fix

Grep across `plugins/`, `.factory/cycles/*/decision-log.md`, `STATE.md`, `CHANGELOG.md` found no post-merge `ls-remote` branch-deletion verification in pr-manager / code-delivery / fix-pr-delivery. `templates/merge-config-template.yaml:20` sets `delete_branch_on_merge: true` (a GitHub repo setting) — relevant but NOT a verification, and not universally enabled. Not addressed.

---

## External Research (technical soundness)

Primary-source confirmations (Perplexity deep-research, 2026-06-09):

- **`gh pr merge --delete-branch` exit 0 does NOT guarantee the remote branch is gone.** Documented in cli/cli#9073 ("A successful `gh pr merge --auto -d` does not delete the branch after a successful merge") — particularly with **merge queues**, where "the branch must exist while it is processed through the merge queue," so deletion is deferred. The CLI presents a synchronous interface over an ultimately asynchronous API operation. — https://github.com/cli/cli/issues/9073 , https://cli.github.com/manual/gh_pr_merge
- **A known race exists** where the branch is deleted (e.g., by GitHub's own auto-delete) before/around the CLI's delete attempt, causing the delete-ref API to 404 — fixed in gh v2.75.0, but confirming the operation is async and racy. — https://github.com/cli/cli/issues/11187
- **Recommended "trust but verify" patterns** (exactly the issue's proposal):
  - `git ls-remote origin refs/heads/<branch>` → empty output means the branch is gone (works against any host). — https://git-scm.com/book/ms/v2/Git-Branching-Remote-Branches
  - or `gh api repos/{owner}/{repo}/git/refs/heads/{branch}` → 404 means deleted. — https://docs.github.com/rest/git/refs
  - Treat branch deletion as **idempotent**: both "deleted" and "already-deleted (404)" are success; if `ls-remote` is non-empty, `git push origin --delete <branch>` and re-verify. A verification loop (optionally with short backoff) accommodates the async/queued case.
- **Repo-level `delete_branch_on_merge`** setting shifts deletion to GitHub's own async process — helpful but still async, so verification is still warranted. — https://github.com/cli/cli/issues/380

The issue's proposed fix (one extra `git ls-remote` after the merge, push-delete + re-verify if non-empty, gate STEP_COMPLETE on empty) is technically correct and aligns with the authoritative recommendation. The only refinement worth adding: a small bounded retry/backoff to absorb the queued-deletion latency before force-deleting, and idempotent 404 handling.

---

## Verdict

> **VALID-NEW** — Confidence: **High**
>
> The bug is real and root-caused: `pr-manager.md` Step 8/9 (lines 228-258) treats `--delete-branch` as a deletion guarantee with no `git ls-remote` verification, and `code-delivery/SKILL.md:190` states the false assumption explicitly. External primary sources (cli/cli#9073, #11187, gh manual, git-scm) confirm gh exit 0 does not guarantee remote deletion — it is asynchronous and racy, especially with merge queues. The verifier primitive (`git ls-remote origin <branch>`) and the "claim ≠ evidence" doctrine already exist elsewhere in the engine but are not applied here. No prior fix. Worth doing; low-cost (one extra `ls-remote` per PR).

---

## Recommended Approach (zero re-research)

**Route to:** orchestrator → engine-discipline codification. `pr-manager` delegates all `gh`/`git` to `github-ops` (pr-manager.md line 41), so the verification is added to the pr-manager playbook + executed via a github-ops dispatch. Agent/skill edits are `plugins/` source (develop-branch PR path); the decision/lesson record lands via `state-manager`.

**Key files to touch:**
1. `plugins/vsdd-factory/agents/pr-manager.md` — Step 8 (lines 228-249): after the `gh pr merge --squash --delete-branch` github-ops dispatch, add a **branch-deletion verification** github-ops dispatch:
   - `git ls-remote origin refs/heads/<branch>` (or `gh api repos/{owner}/{repo}/git/refs/heads/<branch>`).
   - If non-empty after a short bounded wait/retry (absorb queued deletion): `git push origin --delete <branch>` and re-verify; treat 404/already-gone as success (idempotent).
   - Only emit `STEP_COMPLETE: step=8` (and proceed to Step 9) once verification returns empty. Update Step 9 wording so "post-merge" includes *verified* remote-branch deletion, not assumed.
2. `plugins/vsdd-factory/skills/code-delivery/SKILL.md` — Step 10 line 190: replace "Remote branch deleted (`--delete-branch` handles this)" with a verify-then-delete step using `git ls-remote`. (TD-VSDD-060 sibling-sweep.)
3. `plugins/vsdd-factory/skills/fix-pr-delivery/SKILL.md:132` and `workflows/code-delivery.lobster:412`, `workflows/greenfield.lobster:784` — same sibling-sweep: add the post-merge `ls-remote` verification note so every merge callsite is consistent.

**Approach:**
- Codify the issue's proposed snippet, hardened with (a) idempotent 404 handling, (b) a small bounded retry (e.g., re-check up to ~3 times over a few seconds) before force-deleting, to avoid racing GitHub's own queued/auto deletion. Keep it one extra `ls-remote` in the common (already-deleted) case.
- Consider recommending repo-level `delete_branch_on_merge: true` (template already sets it) as a complement, but NOT a replacement for verification (still async).

**Risks:**
- Branch-protection rules can block deletion; the verify step should surface a clear BLOCKED reason rather than loop forever (cap retries, then escalate).
- Force-deleting a branch that GitHub is *about* to delete via merge queue could interact poorly — the bounded-wait-before-delete mitigates this; never force-delete a branch still in a merge queue.
- Multiple PRs in one session (the issue's 2-of-4 case): apply the verification per-PR; do not batch.

**Test strategy:**
- Prompt-contract bats test: assert `pr-manager.md` Step 8 contains a post-merge `git ls-remote`/`gh api` branch-deletion verification and gates STEP_COMPLETE on empty.
- Extend `plugins/vsdd-factory/tests/pr-lifecycle-hooks.bats` (already exercises `gh pr merge … --delete-branch`, line 179) with a follow-up `ls-remote` verification assertion (mock returns non-empty → expect a push-delete + re-verify; empty → STEP_COMPLETE).
- Integration: simulate the transient survive (ls-remote non-empty once, then empty) → assert pr-manager retries and converges.

**Dependencies:**
- Independent of #130/#169/#176 (different subsystem: PR merge lifecycle vs dispatcher path-resolution / adversary tree identity).
- Sibling-sweep dependency: code-delivery, fix-pr-delivery, and the two `.lobster` workflows must be updated in the same change (TD-VSDD-060) to avoid leaving the false assumption in sibling callsites.

---

## Research Methods

| Tool | Queries | Purpose |
|------|---------|---------|
| **Perplexity perplexity_research (PRIMARY)** | 1 (shared across #128/#130/#169/#176) | gh `--delete-branch` exit-code semantics, async/merge-queue deletion race, `git ls-remote`/`gh api` verify patterns, primary cli/cli issues + git-scm/gh docs |
| Read | 3 | pr-manager.md, code-delivery SKILL, deliver-story _shared-context.md |
| Grep | 2 | ls-remote / delete-branch / branch-deleted across plugins |
| Glob | 1 | agent + skill enumeration |
| Training data | 0 areas | All gh/git claims externally sourced; agent-prompt claims by direct read with line cites |

**Total MCP tool calls:** 1 (deep research, shared)
**Training data reliance:** Low — gh CLI async-deletion behavior verified against cli/cli#9073, #11187 and the gh manual; code claims verified by direct reading of `pr-manager.md` / `code-delivery/SKILL.md` with line cites.
