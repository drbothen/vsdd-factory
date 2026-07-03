#!/bin/bash
# check-harness-version.sh — PreCompact hook: verify Claude Code harness version
#
# Checks that the running Claude Code harness is >= v2.1.105, which is required
# for PreCompact block-intent propagation to be honoured (BC-1.15.001 PC1/PC4,
# BC-1.15.001 INV3).
#
# Exit behaviour (BC-1.15.001 INV3, AC-008):
#   - Exit 0: harness is >= v2.1.105 (version check passed).
#   - Exit 1: harness version cannot be determined or is below threshold (advisory).
#             The dispatcher continues (on_error=continue); compaction proceeds.
#
# Registered as PreCompact, on_error=continue (hooks-registry.toml S-18.00).
# Never exits 2 — block-intent is reserved for the `precompact-flush` PreCompact WASM plugin (S-18.04a).
#
# Version detection:
#   CLAUDE_CODE_VERSION env var (primary) or CLAUDE_VERSION env var (alias).
#   When neither is set, the harness version is considered undeterminable and the
#   script exits 1 (advisory). The dispatcher continues regardless (on_error=continue).
#
# BC-1.15.001 INV3: the dispatcher does NOT check harness version at runtime.
# This script provides an advisory operator signal; non-zero exit never blocks compaction.

set -euo pipefail

# ---------------------------------------------------------------------------
# Minimum required version (BC-1.15.001 §Preconditions)
# ---------------------------------------------------------------------------
MIN_MAJOR=2
MIN_MINOR=1
MIN_PATCH=105

# ---------------------------------------------------------------------------
# Version comparison: returns 0 (true) if v1 >= v2
# Arguments: major1 minor1 patch1 major2 minor2 patch2
# ---------------------------------------------------------------------------
_version_gte() {
    local ma1=$1 mi1=$2 pa1=$3 ma2=$4 mi2=$5 pa2=$6

    if [ "$ma1" -gt "$ma2" ]; then return 0; fi
    if [ "$ma1" -lt "$ma2" ]; then return 1; fi
    # major equal
    if [ "$mi1" -gt "$mi2" ]; then return 0; fi
    if [ "$mi1" -lt "$mi2" ]; then return 1; fi
    # minor equal
    if [ "$pa1" -ge "$pa2" ]; then return 0; fi
    return 1
}

# ---------------------------------------------------------------------------
# Detect harness version string from environment
# ---------------------------------------------------------------------------
VERSION_STRING=""

# Primary: CLAUDE_CODE_VERSION env var (set by harness >= v2.1.105)
if [ -n "${CLAUDE_CODE_VERSION:-}" ]; then
    VERSION_STRING="$CLAUDE_CODE_VERSION"
# Alias: CLAUDE_VERSION env var (alternate form)
elif [ -n "${CLAUDE_VERSION:-}" ]; then
    VERSION_STRING="$CLAUDE_VERSION"
fi

# ---------------------------------------------------------------------------
# Advisory exit when version is undeterminable
# ---------------------------------------------------------------------------
if [ -z "$VERSION_STRING" ]; then
    echo "check-harness-version: harness version undeterminable (CLAUDE_CODE_VERSION not set); PreCompact block-intent may not be honoured on harness < v${MIN_MAJOR}.${MIN_MINOR}.${MIN_PATCH} — set CLAUDE_CODE_VERSION in the harness environment" >&2
    exit 1
fi

# ---------------------------------------------------------------------------
# Guard: reject implausibly long version strings (SEC-001)
# ---------------------------------------------------------------------------
if [ "${#VERSION_STRING}" -gt 64 ]; then
    echo "check-harness-version: version string too long (${#VERSION_STRING} bytes); cannot parse version; advisory" >&2
    exit 1
fi

# ---------------------------------------------------------------------------
# Parse and compare
# ---------------------------------------------------------------------------
# Strip leading 'v' if present (e.g. "v2.1.177" → "2.1.177")
_clean_version="${VERSION_STRING#v}"

# Guard: reject pre-release versions (SEC-002)
# Pre-release versions (e.g. "2.1.105-beta.1") are semantically below the GA
# release per semver, so they do not satisfy the minimum version requirement.
case "$_clean_version" in
    *-*)
        echo "check-harness-version: harness v${_clean_version} is a pre-release version; treating as below threshold; advisory" >&2
        exit 1
        ;;
esac

_major=$(echo "$_clean_version" | cut -d. -f1)
_minor=$(echo "$_clean_version" | cut -d. -f2)
_patch=$(echo "$_clean_version" | cut -d. -f3 | grep -oE '^[0-9]+' || echo "0")

# Validate that we got numeric components
if ! printf '%s' "$_major" | grep -qE '^[0-9]+$' || \
   ! printf '%s' "$_minor" | grep -qE '^[0-9]+$' || \
   ! printf '%s' "$_patch" | grep -qE '^[0-9]+$'; then
    echo "check-harness-version: harness version '${VERSION_STRING}' is not a parseable semver; advisory — proceeding" >&2
    exit 1
fi

if _version_gte "$_major" "$_minor" "$_patch" "$MIN_MAJOR" "$MIN_MINOR" "$MIN_PATCH"; then
    echo "check-harness-version: harness v${_clean_version} >= v${MIN_MAJOR}.${MIN_MINOR}.${MIN_PATCH} — PreCompact block-intent supported" >&2
    exit 0
else
    echo "check-harness-version: harness v${_clean_version} < v${MIN_MAJOR}.${MIN_MINOR}.${MIN_PATCH} — PreCompact block-intent will not be honoured on this harness version; advisory" >&2
    exit 1
fi
