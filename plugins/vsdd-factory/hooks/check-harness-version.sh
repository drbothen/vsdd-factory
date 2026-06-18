#!/bin/bash
# check-harness-version.sh — PreCompact hook: verify Claude Code harness version
#
# Checks that the running Claude Code harness is >= v2.1.105, which is required
# for PreCompact block-intent propagation to be honoured (BC-1.15.001 PC1/PC4,
# BC-1.15.001 INV3).
#
# Exit behaviour (BC-1.15.001 INV3, AC-008):
#   - Exit 0: harness is >= v2.1.105 (all good).
#   - Exit 1: harness version cannot be determined or is below threshold (advisory).
#             The dispatcher continues (on_error=continue); compaction proceeds.
#
# Registered as PreCompact, on_error=continue (hooks-registry.toml S-18.00 stub).
# Never exits 2 — block-intent is reserved for precompact-flush.sh (S-18.04a).
#
# TODO S-18.00 — Red Gate stub. The real implementation (S-18.00 TDD green step)
# queries the harness version from the Claude Code process or environment, parses
# the semver, and compares against the v2.1.105 threshold. This stub exits 1 so
# bats tests (check-harness-version.bats) properly fail the Red Gate.

set -euo pipefail

# S-18.00 stub: the real implementation queries the harness version.
# Exiting 1 here ensures Red Gate bats tests fail as required.
echo "check-harness-version: stub not yet implemented (S-18.00 Red Gate)" >&2
exit 1
