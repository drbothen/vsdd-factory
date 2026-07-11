#!/usr/bin/env bash
# darwin-leg-preflight.sh — darwin-leg interpreter preflight check.
#
# Validates that /bin/bash reports a "version 3.2" substring on macOS.
# If not, exits non-zero with DARWIN_LEG_WRONG_INTERPRETER on stderr.
# On non-Darwin (Linux) platforms, exits 0 (graceful skip).
#
# This script is invoked in bats setup_file for the bats-darwin-leg-macos
# suite (AC-004; S-19.01 T-008/T-009).
#
# Exit codes:
#   0  — /bin/bash version matches "version 3.2" (macOS system bash), or non-Darwin
#   1  — wrong interpreter detected on Darwin
#
# Stderr diagnostics (AC-004 verbatim):
#   DARWIN_LEG_WRONG_INTERPRETER: expected /bin/bash 3.2.x, got <actual>
#
# S-19.01: Red Gate stub — UNIMPLEMENTED.
# Implementation will:
#   1. Check uname; if not Darwin, exit 0 (graceful skip per EC-003)
#   2. Run /bin/bash --version; capture first line
#   3. If version line contains "version 3.2": exit 0
#   4. Otherwise: emit DARWIN_LEG_WRONG_INTERPRETER diagnostic to stderr; exit 1

set -euo pipefail

printf 'UNIMPLEMENTED: darwin-leg-preflight.sh not yet implemented (S-19.01)\n' >&2
exit 99
