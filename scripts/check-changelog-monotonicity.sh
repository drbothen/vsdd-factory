#!/bin/bash
# check-changelog-monotonicity.sh — verify CHANGELOG.md date monotonicity.
#
# Stub implementation — NOT YET IMPLEMENTED.
# This script is a required artifact for S-4.08 (AC-13).
# The implementer must replace this stub with the real logic.
#
# Exit codes (per AC-13 exit-code grammar):
#   0 — CHANGELOG.md exists and all entries are chronologically monotonic
#       (later entries have strictly newer dates than earlier entries)
#   1 — non-monotonic: a later entry has an older date than a prior entry
#   2 — parse error or CHANGELOG.md missing
#
# Usage:
#   scripts/check-changelog-monotonicity.sh [CHANGELOG.md path]
#   Default path: CHANGELOG.md (relative to CWD or repo root)
#
# This stub ALWAYS exits 2 (parse error / unimplemented) so that AC-13
# gate checks fail before the real implementation is in place.
# Replace with real implementation that parses CHANGELOG.md date headings.

set -euo pipefail

echo "check-changelog-monotonicity.sh: NOT IMPLEMENTED — stub only" >&2
echo "Replace this stub with real CHANGELOG monotonicity check before rc.1 cut." >&2
exit 2
