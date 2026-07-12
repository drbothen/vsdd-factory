#!/usr/bin/env bash
set -euo pipefail

# enforce-merge-strategy.sh <pr_number> [--merge|--squash|--rebase] [residual-args...]
#
# Merge-strategy enforcement wrapper / governed pass-through (BC-5.42.001 part c;
# ADR-030 §Decision 3 v1.4). The SOLE script used by the orchestrator to invoke
# gh pr merge. Enforces --merge for release-branch PRs (branches matching ^release/v);
# forwards residual args ("${@:3}") verbatim after the resolved strategy flag.
#
# INVOCATION (positional; ADR-030 §Decision 3 canonical form):
#   enforce-merge-strategy.sh <pr_number> [--merge|--squash|--rebase] [residual-args...]
#
#   pr_number:     GitHub PR number (positive integer)
#   strategy flag: --merge | --squash | --rebase (positional $2; optional for non-release
#                  branches; defaults to --merge on release/v* branches per EC-005)
#   residual-args: additional flags forwarded verbatim to gh pr merge after the resolved
#                  strategy flag (e.g. --delete-branch); subject to the deny-list below
#
# EXIT CODES:
#   0   merge delegated to gh pr merge and succeeded (exit code propagated)
#   1   RELEASE_PR_SQUASH_FORBIDDEN — --squash/--rebase on release/v* branch
#       (no gh API call is made; exits before any GitHub API invocation)
#   2   STRATEGY_SMUGGLING_FORBIDDEN — a residual arg ($3+) is a strategy or admin flag
#       (deny-list rejects before any gh API call)
#
# STDERR DIAGNOSTICS (BC-5.42.001 §Canonical Test Vectors verbatim):
#   RELEASE_PR_SQUASH_FORBIDDEN: branch <branch_name> requires --merge per RELEASING.md
#   STRATEGY_SMUGGLING_FORBIDDEN: residual arg <arg> is a strategy or admin flag
#
# RESIDUAL-ARG DENY-LIST (strategy-smuggling prevention; ADR-030 §Decision 3 v1.4):
#   Rejected: --squash/--merge/--rebase/--admin (long), =-fused (e.g. --squash=auto),
#             bare short (-s/-m/-r/-A) and combined short clusters starting with or
#             containing s/m/r/A (e.g. -sd, -ds).
#   Allowed:  --delete-branch, -d, and all other non-strategy non-admin flags.
#
# NOTE: Direct gh pr merge calls outside this wrapper are a protocol violation
# per BC-5.42.001 Precondition 5 + ADR-030 §Decision 3.

_usage() {
    printf 'Usage: enforce-merge-strategy.sh <pr_number> [--merge|--squash|--rebase] [residual-args...]\n' >&2
    printf '  pr_number:  GitHub PR number (positive integer)\n' >&2
    printf '  strategy:   --merge (required for release/v* branches), --squash, or --rebase\n' >&2
    printf '  residual:   additional flags forwarded to gh pr merge (--delete-branch etc.)\n' >&2
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

# Step 2: Residual-arg deny-list scan (ADR-030 §Decision 3 v1.4).
# Reject any residual arg ($3+) that is a strategy override or privilege-escalation flag
# before any gh invocation. Prevents strategy-smuggling via residual args on release PRs.
# Bash-3.2 compatible: case glob patterns (no associative arrays; no mapfile).
# Empty "${@:3}" slice does not trip set -u (bash positional slice semantics).
for _residual_arg in "${@:3}"; do
    case "${_residual_arg}" in
        --squash|--merge|--rebase|--admin)
            # Long exact forms.
            printf 'STRATEGY_SMUGGLING_FORBIDDEN: residual arg %s is a strategy or admin flag\n' "${_residual_arg}" >&2
            exit 2
            ;;
        --squash=*|--merge=*|--rebase=*|--admin=*)
            # =-fused long forms (e.g. --squash=auto, --merge=commit).
            printf 'STRATEGY_SMUGGLING_FORBIDDEN: residual arg %s is a strategy or admin flag\n' "${_residual_arg}" >&2
            exit 2
            ;;
        -[smrA]*)
            # Bare short forms (-s, -m, -r, -A) and clusters starting with a strategy char
            # (-sd, -sm, etc.). -[smrA]* matches -s, -sd, -sX for any X.
            printf 'STRATEGY_SMUGGLING_FORBIDDEN: residual arg %s is a strategy or admin flag\n' "${_residual_arg}" >&2
            exit 2
            ;;
        -[!-]*[smrA]*)
            # Combined short-flag clusters not starting with a strategy char but containing
            # one (e.g. -ds, -dA, -dmr). -[!-] = non-dash first char; *[smrA]* = contains
            # at least one strategy/admin char.
            printf 'STRATEGY_SMUGGLING_FORBIDDEN: residual arg %s is a strategy or admin flag\n' "${_residual_arg}" >&2
            exit 2
            ;;
    esac
done

# Step 3: Check if branch matches ^release/v pattern.
IS_RELEASE="false"
if printf '%s' "${BRANCH_NAME}" | grep -qE '^release/v'; then
    IS_RELEASE="true"
fi

# Step 4: Enforce merge strategy for release branches.
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

# Step 5: Delegate to gh pr merge, forwarding residual args ("${@:3}") verbatim.
# ADR-030 §Decision 3: wrapper is a faithful governed pass-through; "${@:3}" is forwarded
# after the resolved strategy flag. Bash-3.2: positional slice; empty slice safe under set -u.
# Quote MERGE_FLAG when non-empty; omit entirely when empty (no flag supplied for a
# non-release branch) so gh does not receive a spurious empty argument.
if [[ -n "${MERGE_FLAG}" ]]; then
    gh pr merge "${PR_NUMBER}" "${MERGE_FLAG}" "${@:3}"
else
    gh pr merge "${PR_NUMBER}" "${@:3}"
fi
