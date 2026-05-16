#!/usr/bin/env bash
# active-branches-sha-currency.sh — D-450(d) SHA currency for Active Branches table rows
#
# Closes: D-450(d) (SHA sub-clause) — "git rev-parse origin/<branch>" currency check
# for all Active Branches table rows in STATE.md.
#
# Usage: active-branches-sha-currency.sh <factory-root> <state-md-path>
# For each branch row in the Active Branches table (main, develop, factory-artifacts),
# runs "git rev-parse origin/<branch>" from <factory-root> and compares to the SHA
# cell in STATE.md. Exits 0 if all match; exits 1 with per-branch report on FAIL.
#
# EC-001: exits 1 with human-readable message if not inside a git repository.
#
# Testing: supports GIT_TEST_SHA_OVERRIDE_<branch>=<sha> env vars to avoid
# requiring a real git remote (branch hyphens become underscores in the var name).

set -euo pipefail

# ---- help ----
if [[ "${1:-}" == "--help" ]] || [[ "${1:-}" == "-h" ]]; then
  cat <<'USAGE'
active-branches-sha-currency.sh — D-450(d) Active Branches SHA currency gate

USAGE:
  active-branches-sha-currency.sh <factory-root> <state-md-path>

ARGUMENTS:
  factory-root    Root of the factory project (used for git rev-parse calls)
  state-md-path   Path to STATE.md containing the Active Branches table

EXIT CODES:
  0 — all branch SHAs in STATE.md match git rev-parse origin/<branch> (PASS)
  1 — one or more branch SHAs are stale (FAIL), or not a git repository

TESTING (env-var override for hermetic tests):
  GIT_TEST_SHA_OVERRIDE_<branch>=<sha>
  Branch hyphens become underscores in the env var name:
    factory-artifacts -> GIT_TEST_SHA_OVERRIDE_factory_artifacts

EXAMPLES:
  active-branches-sha-currency.sh . .factory/STATE.md
  active-branches-sha-currency.sh /path/to/factory /path/to/STATE.md

NOTES:
  Invoked during Dim-2 fix-burst attestation per D-449(a). Output includes the
  literal git rev-parse stdout per D-449(a) compliance.
USAGE
  exit 0
fi

# ---- argument validation ----
if [[ $# -lt 2 ]]; then
  echo "ERROR: active-branches-sha-currency.sh requires 2 arguments: <factory-root> <state-md-path>" >&2
  echo "Run with --help for usage." >&2
  exit 1
fi

FACTORY_ROOT="$1"
STATE_MD="$2"

if [[ ! -d "$FACTORY_ROOT" ]]; then
  echo "FAIL: factory-root directory not found: ${FACTORY_ROOT}" >&2
  exit 1
fi

if [[ ! -f "$STATE_MD" ]]; then
  echo "FAIL: STATE.md file not found: ${STATE_MD}" >&2
  exit 1
fi

# ---- SHA lookup with env-var override (for testing) ----
_get_sha() {
  local BRANCH="$1"
  # Transform branch name to env var name: replace hyphens with underscores
  local ENV_VAR_NAME="GIT_TEST_SHA_OVERRIDE_$(echo "$BRANCH" | tr '-' '_')"

  # Check if env-var override is set
  local OVERRIDE_VAL="${!ENV_VAR_NAME:-}"
  if [[ -n "$OVERRIDE_VAL" ]]; then
    echo "$OVERRIDE_VAL"
    return 0
  fi

  # Fall back to real git rev-parse
  if ! git -C "$FACTORY_ROOT" rev-parse "origin/${BRANCH}" 2>/dev/null; then
    echo "ERROR: git rev-parse origin/${BRANCH} failed — not a git repository or no such remote branch" >&2
    return 1
  fi
}

# ---- parse Active Branches table from STATE.md ----
# Extract rows matching "| branch-name | SHA | ..."
# Table format: | Branch | HEAD SHA | Status |
#               | main   | abc123   | stable |

echo "$ git rev-parse origin/<branch> (for each Active Branches row)"
echo "---"

FAIL=0
BRANCHES_CHECKED=0

while IFS= read -r TABLE_LINE; do
  # Match table rows with pipe-separated columns
  # Pattern: | <branch-name> | <sha> | <status> |
  if echo "$TABLE_LINE" | grep -qE '^\| [a-z]'; then
    # Extract branch and SHA from table row
    BRANCH=$(echo "$TABLE_LINE" | awk -F'|' '{gsub(/^[ \t]+|[ \t]+$/, "", $2); print $2}')
    SHA_IN_STATE=$(echo "$TABLE_LINE" | awk -F'|' '{gsub(/^[ \t]+|[ \t]+$/, "", $3); print $3}')

    # Skip header row
    if [[ "$BRANCH" == "Branch" ]] || [[ -z "$BRANCH" ]] || [[ -z "$SHA_IN_STATE" ]]; then
      continue
    fi

    # Skip rows where SHA looks like a header or non-SHA text
    if ! echo "$SHA_IN_STATE" | grep -qE '^[0-9a-f]{7,40}$'; then
      continue
    fi

    BRANCHES_CHECKED=$(( BRANCHES_CHECKED + 1 ))

    # Get the current SHA via git (or env-var override)
    CURRENT_SHA=""
    if ! CURRENT_SHA=$(_get_sha "$BRANCH" 2>&1); then
      echo "FAIL: ${BRANCH} — could not get SHA: ${CURRENT_SHA}"
      FAIL=1
      continue
    fi

    echo "$ git rev-parse origin/${BRANCH}"
    echo "  ${CURRENT_SHA}"

    if [[ "$CURRENT_SHA" == "$SHA_IN_STATE" ]]; then
      echo "  STATE.md: ${SHA_IN_STATE} — MATCH"
    else
      echo "  STATE.md: ${SHA_IN_STATE} — MISMATCH (stale)"
      echo "  FAIL: ${BRANCH} SHA is stale — STATE.md has ${SHA_IN_STATE}, current is ${CURRENT_SHA}"
      FAIL=1
    fi
    echo ""
  fi
done < "$STATE_MD"

echo "---"
echo "Branches checked: ${BRANCHES_CHECKED}"

if [[ "$BRANCHES_CHECKED" -eq 0 ]]; then
  echo "PASS: no Active Branches rows found in STATE.md — vacuously current"
  exit 0
fi

if [[ "$FAIL" -eq 0 ]]; then
  echo "PASS: all ${BRANCHES_CHECKED} branch SHA(s) are current"
  exit 0
else
  echo "FAIL: one or more branch SHAs are stale in ${STATE_MD}"
  exit 1
fi
