#!/usr/bin/env bash
# propagation-completeness.sh — D-452(a) post-derivation propagation-completeness check
#
# Closes: D-452(a) — Post-derivation propagation-completeness — derived value must appear
# at ALL prescribed sites, not just the primary site.
#
# Usage: propagation-completeness.sh <derived-value> <prescribed-sites-file>
# For each line in prescribed-sites-file (format "<file-path>:<grep-pattern>"),
# greps the file for the derived-value.
# Exits 0 only if ALL sites pass; exits 1 with per-site report on FAIL.
#
# Note: file paths in prescribed-sites-file are relative to CWD at invocation time.
# EC-006: exits 1 with specific message if a referenced file does not exist.

set -euo pipefail

# ---- help ----
if [[ "${1:-}" == "--help" ]] || [[ "${1:-}" == "-h" ]]; then
  cat <<'USAGE'
propagation-completeness.sh — D-452(a) post-derivation propagation-completeness gate

USAGE:
  propagation-completeness.sh <derived-value> <prescribed-sites-file>

ARGUMENTS:
  derived-value           The value that must appear at all prescribed sites
  prescribed-sites-file   File listing "<file-path>:<grep-pattern>" pairs (one per line)
                          File paths are resolved relative to CWD at invocation time.

EXIT CODES:
  0 — derived-value found at ALL prescribed sites (PASS)
  1 — derived-value missing from one or more prescribed sites (FAIL)

SITES FILE FORMAT:
  .factory/STATE.md:D-453
  .factory/cycles/.../INDEX.md:D-453

EXAMPLES:
  # Run from factory root so relative paths in sites.txt resolve correctly:
  cd /path/to/factory
  propagation-completeness.sh "D-453" sites.txt

  propagation-completeness.sh "v1.0.0-rc.17" prescribed-sites.txt

NOTES:
  Call from the directory that anchors the relative paths in prescribed-sites-file
  (typically the factory root). Per D-452(a): checks ALL prescribed sites, not just
  the primary. Invoked during Dim-2 fix-burst attestation per D-449(a).
USAGE
  exit 0
fi

# ---- argument validation ----
if [[ $# -lt 2 ]]; then
  echo "ERROR: propagation-completeness.sh requires 2 arguments: <derived-value> <prescribed-sites-file>" >&2
  echo "Run with --help for usage." >&2
  exit 1
fi

DERIVED_VALUE="$1"
SITES_FILE="$2"

if [[ ! -f "$SITES_FILE" ]]; then
  echo "FAIL: prescribed-sites-file not found: ${SITES_FILE}" >&2
  exit 1
fi

echo "$ grep -F '${DERIVED_VALUE}' <prescribed-sites>"
echo "---"

FAIL=0
PASS_COUNT=0
FAIL_COUNT=0

while IFS= read -r SITE_LINE; do
  # Skip blank lines
  [[ -z "${SITE_LINE// }" ]] && continue

  # Parse "<file-path>:<grep-pattern>"
  FILE_PATH="${SITE_LINE%%:*}"
  GREP_PATTERN="${SITE_LINE#*:}"

  if [[ ! -f "$FILE_PATH" ]]; then
    echo "FAIL: prescribed file not found: ${FILE_PATH} (EC-006)"
    FAIL=1
    FAIL_COUNT=$(( FAIL_COUNT + 1 ))
    continue
  fi

  # Check for derived-value in the file
  COUNT=$(grep -cF "${DERIVED_VALUE}" "$FILE_PATH" || true)
  if [[ "$COUNT" -ge 1 ]]; then
    echo "PASS: '${DERIVED_VALUE}' found (${COUNT} occurrence(s)) in ${FILE_PATH}"
    PASS_COUNT=$(( PASS_COUNT + 1 ))
  else
    echo "FAIL: '${DERIVED_VALUE}' NOT found in ${FILE_PATH} (pattern: ${GREP_PATTERN})"
    FAIL=1
    FAIL_COUNT=$(( FAIL_COUNT + 1 ))
  fi
done < "$SITES_FILE"

echo "---"
echo "Sites checked: $(( PASS_COUNT + FAIL_COUNT )) (${PASS_COUNT} PASS, ${FAIL_COUNT} FAIL)"

if [[ "$FAIL" -eq 0 ]]; then
  echo "PASS: derived value '${DERIVED_VALUE}' present at all $(( PASS_COUNT )) prescribed site(s)"
  exit 0
else
  echo "FAIL: derived value '${DERIVED_VALUE}' missing from ${FAIL_COUNT} prescribed site(s)"
  exit 1
fi
