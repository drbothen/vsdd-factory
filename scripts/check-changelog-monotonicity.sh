#!/usr/bin/env bash
# check-changelog-monotonicity.sh -- verify CHANGELOG.md date monotonicity.
#
# Parses ## [version] - YYYY-MM-DD or ## version -- ... (YYYY-MM-DD) headings
# and verifies dates appear in strictly descending order (newest first).
#
# Exit codes (AC-13 exit-code grammar):
#   0 -- CHANGELOG.md exists and all entries are chronologically monotonic
#        (dates are strictly descending: newest entry first)
#   1 -- non-monotonic: a later entry has an older or equal date than a prior entry
#   2 -- parse error or CHANGELOG.md missing
#
# Usage:
#   scripts/check-changelog-monotonicity.sh [CHANGELOG.md path]
#   Default path: CHANGELOG.md (in repo root or current directory)
#
# Date format recognised:
#   ## x.y.z -- Description (YYYY-MM-DD)
#   ## [x.y.z] - YYYY-MM-DD
#   Any ## heading line containing a YYYY-MM-DD date in parentheses or after a dash

set -euo pipefail

SCRIPT_NAME="$(basename "$0")"

usage() {
    cat >&2 <<EOF
Usage:
  $SCRIPT_NAME [CHANGELOG_PATH]

Arguments:
  CHANGELOG_PATH   Path to CHANGELOG.md (default: CHANGELOG.md in CWD)

Exit codes:
  0 -- monotonic (dates strictly descending, newest first)
  1 -- non-monotonic (a later entry has an older or equal date)
  2 -- parse error or CHANGELOG.md missing
EOF
}

# ---------------------------------------------------------------------------
# Argument handling
# ---------------------------------------------------------------------------

if [[ "${1:-}" == "--help" || "${1:-}" == "-h" ]]; then
    usage
    exit 0
fi

# Determine CHANGELOG path
if [[ -n "${1:-}" ]]; then
    CHANGELOG_PATH="$1"
else
    CHANGELOG_PATH="CHANGELOG.md"
fi

# ---------------------------------------------------------------------------
# Existence check
# ---------------------------------------------------------------------------

if [[ ! -f "$CHANGELOG_PATH" ]]; then
    echo "Error: CHANGELOG.md not found at '$CHANGELOG_PATH'" >&2
    exit 2
fi

# ---------------------------------------------------------------------------
# Date extraction
# ---------------------------------------------------------------------------

# Extract YYYY-MM-DD dates from ## heading lines.
# Handles formats:
#   ## 1.0.0-rc.1 -- Release Candidate 1 (2026-04-28)
#   ## [1.0.0-rc.1] - 2026-04-28
#   ## 1.0.0-beta.4 -- previous (2026-04-25)
extract_dates() {
    local file="$1"
    grep -E '^## ' "$file" | \
        grep -oE '[0-9]{4}-[0-9]{2}-[0-9]{2}' | \
        head -1000  # safety limit
}

dates_raw="$(extract_dates "$CHANGELOG_PATH")"

if [[ -z "$dates_raw" ]]; then
    echo "Error: no date headings found in '$CHANGELOG_PATH'" >&2
    echo "Expected format: ## version -- Description (YYYY-MM-DD) or ## [version] - YYYY-MM-DD" >&2
    exit 2
fi

# ---------------------------------------------------------------------------
# Monotonicity check (dates must be strictly descending: newest first)
# ---------------------------------------------------------------------------

prev_date=""
line_num=0
non_monotonic=0

while IFS= read -r date; do
    line_num=$(( line_num + 1 ))
    if [[ -z "$date" ]]; then
        continue
    fi

    # Validate date format YYYY-MM-DD
    if ! echo "$date" | grep -qE '^[0-9]{4}-[0-9]{2}-[0-9]{2}$'; then
        echo "Error: malformed date '$date' at entry #$line_num in '$CHANGELOG_PATH'" >&2
        exit 2
    fi

    if [[ -n "$prev_date" ]]; then
        # Dates must be strictly decreasing (prev > current, because list is newest-first)
        # String comparison works for ISO-8601 dates (YYYY-MM-DD)
        if [[ ! "$date" < "$prev_date" ]]; then
            echo "Non-monotonic: entry #$line_num date '$date' is not older than previous '$prev_date'" >&2
            non_monotonic=1
            break
        fi
    fi
    prev_date="$date"
done <<< "$dates_raw"

if [[ "$non_monotonic" -eq 1 ]]; then
    exit 1
fi

echo "CHANGELOG monotonicity check passed: $line_num date(s) in strictly descending order" >&2
exit 0
