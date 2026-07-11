#!/usr/bin/env bash
set -euo pipefail

# check-stale-verdict.sh <pr_number> <covered_sha>
#
# Stale-verdict detection script (BC-5.42.001 part b; ADR-030 §Decision 2).
# Invoked by the orchestrator synchronously before every gh pr merge call on
# a READY verdict. Compares the live PR headRefOid against the covered_sha
# recorded in the READY verdict.
#
# INVOCATION (positional; ADR-030 §Decision 2 canonical form):
#   check-stale-verdict.sh <pr_number> <covered_sha>
#
#   pr_number:   GitHub PR number (positive integer)
#   covered_sha: 40 lowercase hex characters from the pr-manager READY verdict
#
# EXIT CODES (all non-match paths are fail-closed per ADR-030 §Decision 2):
#   0   fresh verdict — covered_sha matches live headRefOid; safe to merge
#   1   fail-closed  — SHA mismatch, malformed arg, gh failure, PR not open
#
# STDERR DIAGNOSTICS (BC-5.42.001 §Canonical Test Vectors verbatim):
#   STALE_READY_VERDICT: PR #<n> HEAD <current_sha> != covered_sha <pinned_sha>
#   READY_SHA_FETCH_FAILED: gh pr view failed for PR #<pr_number>
#   READY_SHA_MISSING: covered_sha is malformed
#   CHECK_STALE_VERDICT_ERROR: PR #<n> is <state> (expected: open)
#   CHECK_STALE_VERDICT_ERROR: <description>

_usage() {
    printf 'Usage: check-stale-verdict.sh <pr_number> <covered_sha>\n' >&2
    printf '  pr_number:   GitHub PR number (positive integer)\n' >&2
    printf '  covered_sha: 40-lowercase-hex SHA from the READY verdict\n' >&2
    exit 1
}

if [[ $# -lt 2 ]]; then
    _usage
fi

PR_NUMBER="$1"
COVERED_SHA="$2"

# UNIMPLEMENTED — Red Gate stub (S-19.01 T-003 / T-004)
# Implementation will:
#   1. Validate COVERED_SHA is exactly 40 lowercase hex chars;
#      on failure: printf 'READY_SHA_MISSING: covered_sha is malformed\n' >&2; exit 1
#   2. Invoke: gh pr view "$PR_NUMBER" --json headRefOid
#      on gh failure: printf 'READY_SHA_FETCH_FAILED: gh pr view failed for PR #%s\n' "$PR_NUMBER" >&2; exit 1
#   3. Parse headRefOid from JSON output; check PR state (expect: open)
#   4. Compare headRefOid against COVERED_SHA:
#      match: exit 0
#      mismatch: printf 'STALE_READY_VERDICT: PR #%s HEAD %s != covered_sha %s\n' ... >&2; exit 1
printf 'UNIMPLEMENTED: check-stale-verdict.sh not yet implemented (S-19.01)\n' >&2
exit 99
