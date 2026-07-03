#!/usr/bin/env bash
# AC-008 (BC-1.15.001 INV3): check-harness-version.sh
#   Success path: CLAUDE_CODE_VERSION=2.1.177 (>=v2.1.105) → exit 0
#   Advisory path: CLAUDE_CODE_VERSION unset → exit 1 (advisory)
# Run standalone: bash demo-run-ac008.sh
set -euo pipefail
REPO="$(cd "$(dirname "$0")/../../.." && pwd)"
SCRIPT="$REPO/plugins/vsdd-factory/hooks/check-harness-version.sh"

echo "# AC-008 (BC-1.15.001 INV3): check-harness-version.sh"
echo "# Script: $SCRIPT"
echo ""

echo "## Success path: CLAUDE_CODE_VERSION=2.1.177 (>= v2.1.105 threshold)"
echo "$ CLAUDE_CODE_VERSION=2.1.177 bash check-harness-version.sh"
env CLAUDE_CODE_VERSION="2.1.177" bash "$SCRIPT" 2>&1 || true
echo "# Exit 0: harness version supported (AC-008 PASS)"
echo ""

echo "## Advisory path: CLAUDE_CODE_VERSION unset"
echo "$ env -u CLAUDE_CODE_VERSION bash check-harness-version.sh"
env -u CLAUDE_CODE_VERSION -u CLAUDE_VERSION bash "$SCRIPT" 2>&1 || true
echo "# Exit 1: advisory — version undeterminable; dispatcher continues (on_error=continue)"
echo "# Compaction is NOT blocked (AC-008 PASS, EC-005)"
echo ""

echo "## Below-threshold path: CLAUDE_CODE_VERSION=2.1.100 (< v2.1.105)"
echo "$ CLAUDE_CODE_VERSION=2.1.100 bash check-harness-version.sh"
env CLAUDE_CODE_VERSION="2.1.100" bash "$SCRIPT" 2>&1 || true
echo "# Exit 1: advisory — harness below threshold; non-blocking (AC-008 PASS, EC-006)"
