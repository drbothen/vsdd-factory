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
