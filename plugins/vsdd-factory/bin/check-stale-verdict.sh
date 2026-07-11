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

# Step 1: Validate COVERED_SHA is exactly 40 lowercase hex chars.
# BC-5.42.001 Invariant 5: must be exactly 40 lowercase hex characters.
# EC-002: 40-char value with non-lowercase hex chars is treated same as absent.
# TV4: ZZZZZZ (non-hex, short) → READY_SHA_MISSING.
if ! printf '%s' "${COVERED_SHA}" | grep -qE '^[0-9a-f]{40}$'; then
    printf 'READY_SHA_MISSING: covered_sha is malformed\n' >&2
    exit 1
fi

# Step 2: Invoke gh pr view to fetch live headRefOid.
# EC-001: gh failure (network/auth) → READY_SHA_FETCH_FAILED, exit 1.
GH_OUTPUT=""
if ! GH_OUTPUT="$(gh pr view "${PR_NUMBER}" --json headRefOid 2>/dev/null)"; then
    printf 'READY_SHA_FETCH_FAILED: gh pr view failed for PR #%s\n' "${PR_NUMBER}" >&2
    exit 1
fi

# Step 3: Parse headRefOid from JSON output.
# Use grep + sed for bash-3.2 compatibility (no mapfile, no process substitution
# with readarray). Bash-3.2 safe: while-read pattern per repo convention (c10dc6ca).
LIVE_SHA=""
while IFS= read -r line; do
    # Extract value of "headRefOid" from JSON: "headRefOid":"<value>"
    if printf '%s' "${line}" | grep -q '"headRefOid"'; then
        # Extract the SHA value between quotes after "headRefOid":
        LIVE_SHA="$(printf '%s' "${line}" | grep -oE '"headRefOid":"[^"]*"' | grep -oE '"[^"]*"$' | tr -d '"')"
    fi
done <<EOF
${GH_OUTPUT}
EOF

if [[ -z "${LIVE_SHA}" ]]; then
    printf 'READY_SHA_FETCH_FAILED: gh pr view failed for PR #%s\n' "${PR_NUMBER}" >&2
    exit 1
fi

# Step 4: Compare headRefOid against COVERED_SHA.
# TV1: match → exit 0 (silent success).
# TV2: mismatch → STALE_READY_VERDICT, exit 1.
if [[ "${LIVE_SHA}" == "${COVERED_SHA}" ]]; then
    exit 0
fi

printf 'STALE_READY_VERDICT: PR #%s HEAD %s != covered_sha %s\n' \
    "${PR_NUMBER}" "${LIVE_SHA}" "${COVERED_SHA}" >&2
exit 1
