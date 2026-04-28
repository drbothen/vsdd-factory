#!/bin/bash
# check-shakedown-window.sh — mechanically enforce the beta-shakedown
# window gate per BC-9.01.006.
#
# Stub implementation — NOT YET IMPLEMENTED.
# This script is a required artifact for S-4.08 (AC-9, AC-10).
# The implementer must replace this stub with the full BC-9.01.006 logic.
#
# Exit codes (per BC-9.01.006 canonical test vector):
#   0 — window satisfied (>=days elapsed, no open P0 issues)
#   1 — window not satisfied (clock not started, P0 open, or insufficient days)
#   2 — tag not found / git error
#   3 — gh CLI error / network failure
#
# Usage:
#   scripts/check-shakedown-window.sh --tag <prerelease-tag> --days <N> \
#       --p0-query '<gh issue list query>'
#   scripts/check-shakedown-window.sh --stories S-3.01,S-3.02,S-3.03 --days 7
#
# This stub ALWAYS exits 1 (window not satisfied) so that AC-9 and AC-10
# gate checks fail before the real implementation is in place.
# Replace with real implementation per BC-9.01.006 PC1..PC5.

set -euo pipefail

echo "check-shakedown-window.sh: NOT IMPLEMENTED — stub only" >&2
echo "Replace this stub with BC-9.01.006 implementation before rc.1 cut." >&2
exit 1
