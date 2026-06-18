#!/usr/bin/env bash
# stub-exit0.sh — test stub hook that always exits 0 (advisory pass).
# Used by bats integration tests for PreCompact/PostCompact routing.
# NOT registered in hooks-registry.toml; for test use only.
set -euo pipefail
exit 0
