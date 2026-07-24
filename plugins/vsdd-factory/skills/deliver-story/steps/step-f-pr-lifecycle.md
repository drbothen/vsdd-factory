---
name: step-f-pr-lifecycle
description: Dispatch implementer to push the feature branch, then dispatch pr-manager to run the full 9-step PR process.
---

# Step F: Push + PR Lifecycle

> **Shared context:** Read `./_shared-context.md` before executing this step — it contains dispatch discipline, story split recovery, and verification rules.

## Sub-step F.1: Push Feature Branch

**Agent:** `implementer` (model tier: Fast)

**Task:** "Push `feature/STORY-NNN-<desc>` to remote origin."

**Exit condition:** `git ls-remote origin feature/STORY-NNN-<desc>` returns the expected SHA.

### Rebase Before Push: Post-Rebase Diff-Integrity Gate Required

**Role ownership:** The implementer (Sub-step F.1 agent) executing the rebase-then-force-push
MUST follow `plugins/vsdd-factory/agents/devops-engineer.md §Inter-Wave Rebase` post-rebase
diff-integrity gate before any `git push --force-with-lease`.

If the feature branch must be rebased onto `origin/develop` before pushing (e.g., after
sibling stories merged), the **post-rebase diff-integrity gate** (BC-5.44.001, ADR-031
§Decision 6) is a **REQUIRED** step between `git rebase origin/develop` and
`git push --force-with-lease`. The gate MUST run before force-push; it cannot run after
(BC-5.44.001 Invariant 1).

The gate uses `git range-diff <pre-rebase-tip>...<post-rebase-tip>` as the primary
detector (git ≥ 2.19) and falls back to `git diff origin/develop --stat` on older git.
Any file showing an unverified net-negative delta in a file also modified by a
recently-merged sibling story halts the push with `UnverifiedNetNegativeDelta`.

See `plugins/vsdd-factory/agents/devops-engineer.md §Inter-Wave Rebase` for the full gate
procedure (PC1/PC2/PC3/PC4 postconditions).

## Sub-step F.2: PR Lifecycle

**Agent:** `pr-manager` (model tier: Standard)

**Task:** "Run the full PR process for STORY-NNN. Feature branch: `feature/STORY-NNN-<desc>`. Target: `develop`. Follow your 9-step process: populate PR description from `${CLAUDE_PLUGIN_ROOT}/templates/pr-description-template.md`, verify demo evidence, create PR via github-ops, security review, pr-reviewer convergence loop, wait for CI, dependency check, merge. Do NOT skip any step."

**Context to pass:** Story ID, feature branch name, PR template path.

**Do not compose the PR body yourself.** pr-manager owns the full PR lifecycle and uses its own templates. Your job here is delegation, not authorship.

## Exit Condition

pr-manager reports the PR merged (or reports a blocker that requires human intervention).

If pr-manager returns "diff too large, recommend split" — follow the Story Split Recovery procedure in shared context.

## Artifacts

- Feature branch pushed to remote
- Pull request created, reviewed, and merged
