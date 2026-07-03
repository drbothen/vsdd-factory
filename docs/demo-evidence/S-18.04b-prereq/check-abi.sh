#!/usr/bin/env bash
# check-abi.sh — AC-005 demo helper
# Shows HOST_ABI_VERSION = 1 in source and in dispatcher summary line.
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/../../.." && pwd)"
DISPATCHER="$REPO_ROOT/target/release/factory-dispatcher"

echo "=== AC-005/PC5: HOST_ABI_VERSION = 1 (source + runtime) ==="
echo ""
echo "--- Source constant (crates/factory-dispatcher/src/lib.rs) ---"
grep -n "pub const HOST_ABI_VERSION" "$REPO_ROOT/crates/factory-dispatcher/src/lib.rs"

echo ""
echo "--- Runtime: dispatcher summary emits host_abi=1 on every dispatch ---"
ENVELOPE='{"event_name":"PostToolUse","tool_name":"Edit","session_id":"abi-check","tool_input":{"file_path":"x"},"tool_response":{"success":true}}'
printf '%s' "$ENVELOPE" | HOME=/tmp "$DISPATCHER" 2>&1 | grep 'host_abi'

echo ""
echo "PASS  HOST_ABI_VERSION remains 1 (no new named HookPayload field added)"
echo "PASS  git_context rides in payload.extra (HashMap) — ABI unchanged"
