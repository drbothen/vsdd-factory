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

set -euo pipefail

# Step 1: Non-Darwin platforms exit 0 (graceful skip per EC-003).
# The bats-darwin-leg-macos CI job only runs on macOS; Linux runners
# do not have Apple's patched Bash 3.2 and the suite is intentionally absent.
if [[ "$(uname)" != "Darwin" ]]; then
    exit 0
fi

# Step 2: Run /bin/bash --version and capture first line.
BASH_VERSION_LINE="$(/bin/bash --version 2>/dev/null | head -1)"

# Step 3: Check if the version line contains "version 3.2".
# Apple's system bash on macOS is 3.2.x (Apple patched variant).
# The preflight is the drift sentinel: if this fails, investigate before running
# darwin-leg script validation.
if printf '%s' "${BASH_VERSION_LINE}" | grep -q 'version 3\.2'; then
    exit 0
fi

# Step 4: Wrong interpreter — emit diagnostic and exit 1.
printf 'DARWIN_LEG_WRONG_INTERPRETER: expected /bin/bash 3.2.x, got %s\n' \
    "${BASH_VERSION_LINE}" >&2
exit 1
