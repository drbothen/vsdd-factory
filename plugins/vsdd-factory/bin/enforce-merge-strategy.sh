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

# Step 1: Fetch headRefName to determine the PR branch.
# ADR-030 §Decision 3: fetch branch via --json headRefName.
# On failure: treat as non-release (fail-open for branch resolution per rationale).
BRANCH_NAME=""
GH_OUTPUT=""
if GH_OUTPUT="$(gh pr view "${PR_NUMBER}" --json headRefName 2>/dev/null)"; then
    # Parse headRefName from JSON — bash-3.2 compatible (no mapfile; while-read pattern).
    while IFS= read -r line; do
        if printf '%s' "${line}" | grep -q '"headRefName"'; then
            BRANCH_NAME="$(printf '%s' "${line}" | grep -oE '"headRefName":"[^"]*"' | grep -oE '"[^"]*"$' | tr -d '"' || true)"
        fi
    done <<EOF
${GH_OUTPUT}
EOF
fi

# Step 2: Check if branch matches ^release/v pattern.
IS_RELEASE="false"
if printf '%s' "${BRANCH_NAME}" | grep -qE '^release/v'; then
    IS_RELEASE="true"
fi

# Step 3: Enforce merge strategy for release branches.
if [[ "${IS_RELEASE}" == "true" ]]; then
    # BC-5.42.001 Invariant 3: release-branch squash/rebase is mechanically impossible.
    # Reject --squash or --rebase before any GitHub API call.
    if [[ "${MERGE_FLAG}" == "--squash" || "${MERGE_FLAG}" == "--rebase" ]]; then
        printf 'RELEASE_PR_SQUASH_FORBIDDEN: branch %s requires --merge per RELEASING.md\n' \
            "${BRANCH_NAME}" >&2
        exit 1
    fi
    # EC-005: no explicit flag on release branch → inject --merge.
    # EC-004: explicit --merge on release branch → pass through.
    MERGE_FLAG="--merge"
fi

# Step 4: Delegate to gh pr merge, propagating its exit code.
# Invariant 4: non-release branches pass caller-supplied flag unchanged.
# Quote MERGE_FLAG when non-empty; omit entirely when empty (no flag supplied
# for a non-release branch) so gh does not receive a spurious empty argument.
if [[ -n "${MERGE_FLAG}" ]]; then
    gh pr merge "${PR_NUMBER}" "${MERGE_FLAG}"
else
    gh pr merge "${PR_NUMBER}"
fi
