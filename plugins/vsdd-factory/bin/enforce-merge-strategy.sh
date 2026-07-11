#!/usr/bin/env bash
set -euo pipefail

# enforce-merge-strategy.sh <pr_number> [--merge|--squash|--rebase]
#
# Merge-strategy enforcement wrapper (BC-5.42.001 part c; ADR-030 §Decision 3).
# The SOLE script used by the orchestrator to invoke gh pr merge. Enforces
# --merge for release-branch PRs (branches matching ^release/v); delegates
# the caller-supplied flag unchanged for all other branches.
#
# INVOCATION (positional; ADR-030 §Decision 3 canonical form):
#   enforce-merge-strategy.sh <pr_number> [--merge|--squash|--rebase]
#
#   pr_number:  GitHub PR number (positive integer)
#   merge flag: --merge | --squash | --rebase (optional for non-release branches;
#               defaults to --merge on release/v* branches per ADR-030 §Decision 3 step 2)
#
# EXIT CODES (fail-closed for release branch violations per ADR-030 §Decision 3):
#   0   merge delegated to gh pr merge and succeeded (or exit code propagated)
#   1   RELEASE_PR_SQUASH_FORBIDDEN — --squash/--rebase on release/v* branch
#       (no gh API call is made; exits before any GitHub API invocation)
#
# STDERR DIAGNOSTICS (BC-5.42.001 §Canonical Test Vectors verbatim):
#   RELEASE_PR_SQUASH_FORBIDDEN: branch <branch_name> requires --merge per RELEASING.md
#
# NOTE: Direct gh pr merge calls outside this wrapper are a protocol violation
# per BC-5.42.001 Precondition 5 + ADR-030 §Decision 3.

_usage() {
    printf 'Usage: enforce-merge-strategy.sh <pr_number> [--merge|--squash|--rebase]\n' >&2
    printf '  pr_number:  GitHub PR number (positive integer)\n' >&2
    printf '  merge flag: --merge (required for release/v* branches), --squash, or --rebase\n' >&2
    exit 1
}

if [[ $# -lt 1 ]]; then
    _usage
fi

PR_NUMBER="$1"
MERGE_FLAG="${2:-}"

# UNIMPLEMENTED — Red Gate stub (S-19.01 T-005 / T-006 / T-007)
# Implementation will:
#   1. Invoke: gh pr view "$PR_NUMBER" --json headRefName
#      on failure: treat as non-release (fail-open per ADR-030 §Decision 3 rationale)
#   2. Parse headRefName from JSON
#   3. If branch matches ^release/v:
#      a. If MERGE_FLAG is --squash or --rebase:
#         printf 'RELEASE_PR_SQUASH_FORBIDDEN: branch %s requires --merge per RELEASING.md\n' "$branch" >&2
#         exit 1
#      b. Force MERGE_FLAG=--merge regardless of input
#   4. Invoke: gh pr merge "$PR_NUMBER" $MERGE_FLAG; propagate exit code
printf 'UNIMPLEMENTED: enforce-merge-strategy.sh not yet implemented (S-19.01)\n' >&2
exit 99
