#!/usr/bin/env bash
# stub-exit2.sh — test stub hook that always exits 2 (block intent).
# Used by bats integration tests for PreCompact/PostCompact routing.
# NOT registered in hooks-registry.toml; for test use only.
set -euo pipefail
exit 2
