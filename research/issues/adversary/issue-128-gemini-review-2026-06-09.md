# Issue #128 — Gemini Adversary Review (consolidated)

**Date:** 2026-06-09
**Reviewer:** Gemini 3.5 Flash (High) via `agy` (antigravity-cli), cross-model-family fresh-context adversary
**Method:** 6 per-file slices fed via stdin (`agy --print --model "Gemini 3.5 Flash (High)" < slice-prompt`)
**Target:** branch `feature/issue-128-verify-branch-deletion` @ f9c30863 vs develop
**Outcome:** ALL 6 slices returned VERDICT: CHANGES-REQUESTED — findings highly consistent across slices.

## Consolidated findings (deduplicated across 6 slices)

1. **HIGH — Bounded retry lacks sleep/backoff.** "re-check up to 3 times" with no wait → re-checks fire in milliseconds, fail to absorb GitHub async deletion latency, trigger premature force-delete. Fix: instruct wait 5–10s between re-checks. (slices 1,2,3,4,5,6)
2. **BLOCKER/HIGH — Forked-PR branch mismatch.** `git ls-remote origin` / `git push origin --delete` assume branch is on origin; for fork PRs the branch is on the contributor's fork → false "deleted" and/or failed delete (no push access). Fix: detect cross-repo PR (`gh pr view --json isCrossRepository,headRepositoryOwner,headRefName`); if fork, skip origin verification/deletion. (all slices)
3. **BLOCKER/HIGH/MED — Merge-queue hazard.** Force-deleting while PR is in a GitHub Merge Queue deletes the branch before the queue completes the merge → breaks the run. Fix: confirm PR is actually merged (`gh pr view --json state`), and do not force-delete while queued. (slices 1,2,3,4,5,6)
4. **MED/LOW — Non-idempotent force-delete.** `git push origin --delete` returns non-zero if branch already gone (race) → agent treats as failure and halts. Fix: treat "remote ref does not exist"/already-gone as success. (all slices)
5. **MED — ls-remote prefix matching.** `git ls-remote origin refs/heads/<b>` can match `refs/heads/<b>-suffix` → false "still exists". Fix: exact-ref match or `git ls-remote --exit-code origin refs/heads/<branch>` (exact when fully qualified). (slices 1,2,3,4)
6. **MED — Branch-protection dead-end.** Protected branch can't be deleted; current prose emits BLOCKED. Reviewers want warn-and-proceed (don't fail the whole delivery). Fix: on protection rejection, log warning and proceed. (slices 4,5,6)
7. **HIGH/MED — Test coverage of new behaviors (slice 4).** Prompt-contract bats must add assertions for: sleep/backoff, fork guard, merge-queue guard, idempotent delete, exact match, branch-protection-proceed. Also flagged exact-string-coupling fragility (accepted tradeoff of prompt-contract tests in this repo).
8. **NIT — Prose vs concrete commands (slice 3).** Verification block is prose; could give a more concrete command sequence.

## Disposition
Findings 1,4,5 = correctness fixes. Findings 2,3,6 = defensive hardening (factory's own PRs are same-repo, non-queue, but pr-manager is a general agent → guards are correct). Finding 7 = TDD test extension. Finding 8 = optional concreteness. All fixed in-scope per production-grade default.

## Raw per-slice reviews

### Slice 1
```
HIGH — Bounded-retry lacks sleep/backoff — The agent is instructed to re-check up to 3 times immediately when `git ls-remote` returns non-empty. Without a sleep or backoff interval between retries, these checks will execute in rapid succession (within milliseconds), failing to absorb GitHub's asynchronous deletion latency. — Instruct the agent to wait/sleep (e.g., 5–10 seconds) between retry attempts.

HIGH — Forked PR branch verification failure — The check queries `origin` (`git ls-remote origin refs/heads/<branch-name>`). For PRs submitted from external forks, the branch does not reside on the target `origin` repository. The command will return empty immediately, causing the agent to falsely conclude the branch is deleted. — Detect if the PR is from a fork; if so, verify the branch on the fork's remote/repository, or skip verification if permissions to the fork are unavailable.

HIGH — Merge-queue hazard — Force-deleting the branch (`git push origin --delete`) when the PR is in a merge queue (async merge) will delete the branch before the merge queue completes the merge, causing the merge queue build to fail. — Check if the PR is queued in a merge queue, and do not force-delete the branch if it is still queued.

HIGH — Non-idempotent force-delete failure — `git push origin --delete <branch-name>` returns a non-zero exit code if the branch was already deleted by GitHub's async process during the retry window. An agent will treat this non-zero exit code as a command failure and halt. — Instruct the agent to ignore errors/non-zero exit codes from the deletion command if the error indicates the branch does not exist (e.g., `remote ref does not exist`).

MEDIUM — Loose prefix match in `git ls-remote` — `git ls-remote` matches patterns as globs or prefixes (e.g., querying `refs/heads/feat` can return `refs/heads/feat-2`). Checking for any "non-empty output" can result in false positives. — Instruct the agent to parse the output and verify if the exact ref `refs/heads/<branch-name>` is present, rather than checking for any non-empty output.

MEDIUM — Fragile test coupling in STEP_COMPLETE note — Changing the output contract note to `note=PR #<N> merged; remote branch confirmed deleted via ls-remote` will break existing prompt-contract bats tests that assert on the exact legacy note format `note=PR #<N> merged`. — Maintain the original note prefix format or update the bats tests to accommodate the modified string.

VERDICT: CHANGES-REQUESTED
```

### Slice 2
```
HIGH — Unbounded retry loop without backoff — The instruction "If non-empty, dispatch github-ops to `git push...` and re-verify" does not define a maximum retry limit or wait/sleep interval, leading to a tight, token-expensive loop.
Fix: Add a bounded retry limit (e.g., "up to 3 times") and a brief delay (e.g., 5-10 seconds) between retries.

HIGH — Forked PR remote mismatch — Checking `origin` assumes the branch resides on the upstream repository. For PRs submitted from forks, the branch is on the contributor's fork, so `git ls-remote origin` will falsely report the branch as deleted.
Fix: Instruct the agent to target the correct fork remote/repository URL (e.g., retrieved via `gh pr view`) instead of hardcoded `origin`.

MEDIUM — Merge queue disruption — Force-deleting the branch via `git push origin --delete` while the PR is queued in a GitHub Merge Queue can disrupt or abort the merge queue run before the branch is safely merged.
Fix: Instruct the agent to check if the PR is in a merge queue state, and if so, skip force-deletion.

MEDIUM — Git ls-remote prefix matching — `git ls-remote` performs pattern/prefix matching. If another branch exists with a matching prefix (e.g., `refs/heads/<branch>-suffix`), the command will return a non-empty result even if the target branch was successfully deleted.
Fix: Instruct the agent to perform an exact string match against the returned output lines.

LOW — Hard exit on race-condition delete failures — If the branch is deleted asynchronously by GitHub between the check and the push, `git push origin --delete` will fail with a non-zero exit code, which might cause the agent to abort the workflow.
Fix: Explicitly instruct the agent to ignore git delete exit code failures if the error output indicates the branch was already deleted/not found.

VERDICT: CHANGES-REQUESTED
```

### Slice 3
```
### Findings

* **SEVERITY: BLOCKER** — **Merge-Queue Hazard** — If a PR is queued in a GitHub Merge Queue, the branch must remain alive until the merge is completed. Force-deleting the branch (`git push origin --delete <branch>`) while it is still in the queue will break the merge queue run. — **Fix**: Instruct the agent to verify that the PR is fully merged (e.g., using `gh pr view --json state`) before attempting branch deletion/verification, or skip force-deletion if the PR is in a queued state.

* **SEVERITY: HIGH** — **Forked-PR Remote Failure** — For PRs originating from forks, the branch does not exist on `origin`. Running `git ls-remote origin` will falsely report empty (deleted) or fail, and `git push origin --delete` will fail because the user does not have push access to delete it on `origin` (or it will target the wrong repository). — **Fix**: Query the head repository and branch name via `gh pr view` to determine the correct remote and owner, and skip force-deletion on forks if permissions are lacking.

* **SEVERITY: HIGH** — **No-Sleep Polling Loop** — The instructions command "bounded retry (up to 3 re-checks)" but do not specify any sleep/backoff delay. An agent following this will execute the checks within milliseconds, failing to wait for the asynchronous remote deletion. — **Fix**: Add an explicit instruction to wait/sleep (e.g., 5–10 seconds) between each check.

* **SEVERITY: MEDIUM** — **Non-Idempotent Force-Delete** — If the branch is deleted by the remote host concurrently or just after the check, `git push origin --delete <branch>` will return a non-zero exit code (failure). An agent expecting clean command execution will fail the run. — **Fix**: Instruct the agent to ignore "remote ref does not exist" or 404 errors during force-deletion.

* **SEVERITY: MEDIUM** — **Prefix Matching Matcher** — `git ls-remote origin refs/heads/<branch>` performs a prefix/pattern match. If another branch exists named `refs/heads/<branch>-suffix`, it will match and return non-empty output even if the target branch is deleted. — **Fix**: Instruct the agent to verify an exact match on the returned ref string, or use `git ls-remote --exit-code origin refs/heads/<branch>` (which matches exactly when fully qualified).

* **SEVERITY: NIT** — **Prose instructions instead of commands** — The instructions inside the comment block tell the agent *what* to do but lack concrete code/command structures for the loop, leaving the exact agent action unpredictable. — **Fix**: Provide a explicit bash loop or clear steps in the prompt contract.

VERDICT: CHANGES-REQUESTED
```

### Slice 4
```
HIGH — Bounded-retry with no sleep/backoff — The tests assert a "bounded retry" is documented in `pr-manager.md` but do not verify that a sleep/backoff interval (e.g., "sleep", "wait 10 seconds") is instructed. Without a pause, the agent will loop instantly, exhausting retries in milliseconds and triggering premature force-deletions. Add a test asserting that a backoff/sleep instruction exists.

HIGH — Forked-PR branch failure — The tests enforce checking `origin` via `git ls-remote origin` and deleting via `git push origin --delete`. For PRs from forks, the branch is not on `origin`, and the agent has no write access to delete it on the fork. Add tests verifying that the playbook instructs the agent to check if the PR is a fork and bypass deletion if so.

HIGH — Merge-queue deletion hazard — If a PR is merged via GitHub Merge Queue, the branch is still needed by the queue. Instantly verifying deletion or force-deleting the branch will disrupt the merge queue. Add a test verifying that the playbook checks for merge queue status before deletion.

HIGH — Fragile exact-wording test coupling — The tests use tight string matching (`grep -F 'push origin --delete'`, `grep -F 'refs/heads/'`) which couples correctness directly to specific markdown prose. Playbook wording changes will break these tests. Relax the grep patterns or target behavioral/conceptual validation.

MEDIUM — Prefix matching on `git ls-remote` — `git ls-remote origin refs/heads/branch` matches any branch starting with that prefix (e.g., matching `branch-2` when validating `branch`). The tests do not verify that the playbook instructs the agent to perform an exact match. Add a test checking that exact matching is mandated.

MEDIUM — Idempotency / 404 handling — If the branch is deleted concurrently or already deleted, `git push origin --delete` will return a non-zero exit status. The playbook needs to instruct the agent to treat "remote ref does not exist" as a success (idempotent). Add a test verifying this error-handling instruction.

MEDIUM — Branch-protection dead-end — If the branch has protection rules blocking deletion, both automatic and force-deletion will fail. The agent needs an explicit fallback path (e.g., log warning and proceed). Add a test verifying the playbook contains a fallback instructions for blocked deletions.

VERDICT: CHANGES-REQUESTED
```

### Slice 5
```
* **BLOCKER** — Forked-PR branch failure — Checking and force-deleting on `origin` fails if the PR originates from a fork, as the branch resides on a different remote/repository.
  * **Fix**: Instruct the agent to verify the PR is not from a fork before checking or deleting the branch on `origin`.
* **HIGH** — Bounded-retry without delay — No sleep or backoff interval is specified between the 3 re-checks, causing the agent to execute them instantly and exhaust the retries before the async deletion finishes.
  * **Fix**: Instruct the agent to wait (e.g., 5-10 seconds) between each re-check.
* **HIGH** — Merge-queue deletion hazard — If the PR is in a Merge Queue, `gh pr merge` returns success while the branch is still queued. Force-deleting it via `git push` will delete the branch before it actually merges, breaking the queue.
  * **Fix**: Instruct the agent to check the PR status (e.g., via `gh pr view`) to ensure it is fully merged before attempting any branch deletion checks.
* **MEDIUM** — Idempotency / already-deleted failure — If the branch is deleted by GitHub asynchronously between the check and the push, `git push origin --delete` returns a non-zero exit code ("remote ref does not exist"), which will fail the workflow.
  * **Fix**: Instruct the agent to ignore errors indicating the remote ref does not exist.
* **MEDIUM** — Branch protection rules — Force-deleting branches via `git push` may fail due to branch protection / repository rules.
  * **Fix**: Instruct the agent to handle permission/protection errors gracefully without failing the entire workflow.

VERDICT: CHANGES-REQUESTED
```

### Slice 6
```
HIGH — Bounded retry lacks sleep/backoff — Without instructions to sleep/pause between the 3 re-checks, the agent will execute them instantly in milliseconds, failing to absorb GitHub's async deletion delay and causing premature force-deletes. — Add a pause instruction (e.g., "wait 5-10 seconds between re-checks").

HIGH — Forked PR branch target mismatch — The instructions hardcode checking and deleting on `origin`. For PRs from forks, the branch is on the fork remote, not `origin`, leading to false positives or failed push/delete commands. — Dynamically determine the branch's correct remote rather than hardcoding `origin`.

HIGH — Merge queue force-delete hazard — Force-deleting the branch while the PR is in a GitHub Merge Queue will abort/corrupt the merge process. — Instruct the agent to check if the PR is in a merge queue and skip force-deletion if so.

MEDIUM — Non-idempotent delete/404 handling — `git push origin --delete` returns a non-zero exit code if the branch was already deleted by a race condition. Lacking instructions to tolerate this, the agent will treat it as a workflow failure. — Instruct the agent to ignore "remote ref does not exist" errors.

MEDIUM — Branch protection dead-end — If branch protection rules prevent deletion, `git push origin --delete` will fail and block the workflow. — Instruct the agent to log a warning and proceed if deletion is rejected due to branch protection.

VERDICT: CHANGES-REQUESTED
```

---

## Convergence trail (adversary loop)

| Pass | Target commit | Findings | Verdict | Action |
|------|---------------|----------|---------|--------|
| 1 | f9c30863 | 6 (4 HIGH, 2 MED) — sleep/backoff, fork, merge-queue, idempotent-delete, prefix-match, fragile-test | CHANGES-REQUESTED | hardened → 180add1e |
| 2 | 180add1e | 4 (1 HIGH, 1 MED, 2 LOW) — 6 prior CONFIRMED-FIXED; new: branch-protection completion DEADLOCK, unbounded merge-queue wait, static note, ls-remote-128 | CHANGES-REQUESTED | fixed → fbc25c3a |
| 3 | fbc25c3a | 4 (2 HIGH, 1 MED, 1 LOW) — post-delete replication-lag deadlock, --exit-code prefix (likely-moot), force-delete error taxonomy, abort→step9 fall-through | CHANGES-REQUESTED | fixed → abde4c68 |

**Convergence declared at pass 3 → abde4c68 (asymptotic).** Trend 6→4→4 with severity shifting from core-correctness to fine-grained prose-robustness edges (each fix narrows the surface; the loop now surfaces ever-finer edge states). Consistent with this project's D-386 Option C asymptotic-acceptance model (BC-5.39.001 3-CLEAN is documented as structurally unreachable on prose-only codification, L-EDP1-007/051/061). All findings through pass 3 were FIXED in-scope (none deferred). Final: 45/45 pr-lifecycle-hooks.bats green; Step 8a-8d now handles merge-queue (bounded poll + CLOSED abort), fork/cross-repo skip, exact-ref ls-remote (--exit-code + stdout parse), idempotent + bounded post-delete verify, branch-protection warn-and-proceed, abort-halts-not-proceed, and a force-delete error taxonomy.

### Pass 2 raw review
```
### Review of the Six Concerns
* **Sleep/backoff between re-checks**: Adequately addressed in Step 8c (5–10s wait, up to 3 retries).
* **Forked/cross-repo PRs**: Adequately addressed in Step 8b (skips origin verification if `isCrossRepository == true`).
* **Merge-queue state**: Adequately addressed in Step 8a (confirms `state == MERGED` before proceeding).
* **Idempotent already-gone delete**: Adequately addressed in Step 8d (interpreting "remote ref does not exist" push error as success).
* **Exact-ref match**: Adequately addressed in Step 8c (uses `refs/heads/<branch-name>` and `--exit-code`).
* **Branch-protection warn-and-proceed**: Conceptually addressed in Step 8d, but blocked by a logical contradiction in the completion condition.

---

### Remaining Gaps & New Problems

* **HIGH** — Contradiction between branch-protection warn-and-proceed and step completion criteria — Step 8d states that branch-protection rejection is not fatal and the agent should log a warning and proceed. However, the final paragraph forbids emitting `STEP_COMPLETE: step=8` unless the ls-remote check returns exit code 2 (which is impossible if the deletion was rejected). This creates a deadlock. — Update the final sentence to: "Do NOT emit `STEP_COMPLETE: step=8` until the ls-remote exact-ref check returns exit code 2 (branch confirmed deleted), deletion was rejected due to branch protection, or the fork guard determined deletion verification is not applicable."
* **MEDIUM** — Undefined loop and terminal conditions for merge-queue waiting — Step 8a tells the agent to "Wait for the state to reach MERGED before continuing" if `state != MERGED` but does not define a polling interval, max retries, or timeout, which could cause a hot loop or infinite hang. It also fails to handle terminal states like `CLOSED` where the PR is aborted and will never merge. — Define a bounded retry mechanism (e.g., poll every 30 seconds, up to 10 times) and explicitly fail the step if the PR state becomes `CLOSED`.
* **LOW** — Hardcoded STEP_COMPLETE note template is inaccurate for forks or branch-protection outcomes — The static note template `PR #<N> merged; remote branch confirmed deleted via ls-remote` is inaccurate if the check was skipped (forks) or if branch protection prevented deletion. — Instruct the agent to adjust the note dynamically (e.g., note fork skip or branch protection rejection).
* **LOW** — Undefined handling for other `git ls-remote` exit codes — Step 8c only handles exit codes 0 and 2. A network/auth failure will return exit code 128, which is left unhandled. — Explicitly instruct the agent to fail or retry if `git ls-remote` returns an unexpected non-zero exit code (other than 2).

VERDICT: CHANGES-REQUESTED
```
### Pass 3 raw review
```
### Findings

* **HIGH** — **Prefix Matching Contradiction in `git ls-remote --exit-code`** — The playbook warns that `git ls-remote` performs prefix matching (e.g. matching `refs/heads/<branch-name>-suffix`), but then instructs the agent to rely on `--exit-code`. However, `git ls-remote --exit-code` also returns status `0` (present) if *any* matching ref is found, including prefix/suffix matches. If the target branch is deleted but a suffix branch exists, `--exit-code` will return `0`, causing a false positive. — **Fix**: Instruct the agent to run `git ls-remote origin refs/heads/<branch-name>` and parse the stdout to verify if any line ends with exactly `\trefs/heads/<branch-name>`. Do not rely solely on the exit code.

* **HIGH** — **Deadlock due to Replication Lag / Verification Gate Rigidity** — Step 8d requires a post-force-delete `ls-remote` check to return exit code 2 (absent) to satisfy completion condition (a). Due to GitHub replication lag, a successful `git push origin --delete` might be followed by an `ls-remote` check that still returns 0 (present). Since no retry loop is defined for the post-delete check and the gate is rigid, the agent will get stuck. — **Fix**: (1) Add a bounded retry (polling with 5-10s wait) for the post-force-delete verification check, and (2) add a completion gate condition that permits success if the force-delete command itself returned exit `0` (success) or indicating "remote ref does not exist", even if replication lag temporarily shows the branch as present.

* **MEDIUM** — **Unhandled force-delete command failures** — Step 8d does not specify how the agent should handle unexpected non-zero exit codes from the force-delete command (e.g., network timeout, auth failures). The agent may mistake these for branch-protection rejections or get stuck. — **Fix**: Instruct the agent to inspect stderr: retry on transient/network errors, warn-and-proceed on branch-protection/permissions, and abort on persistent unexpected errors.

* **LOW** — **Ambiguous abort flow vs "Proceed immediately to Step 9"** — If Step 8a aborts (due to a timeout or a CLOSED PR), the playbook says "abort Step 8 and surface a clear BLOCKED note" but then ends the section with a bolded "**Proceed immediately to step 9.**". An agent might proceed to Step 9 (which deletes local worktrees and compiles final deliverables) even though the PR was never merged. — **Fix**: Explicitly state that if Step 8 is aborted, the agent must halt execution entirely and must NOT proceed to Step 9.

***

**VERDICT: CHANGES-REQUESTED**
```
