#!/usr/bin/env bash
# demo-AC017-durability-degraded.sh
# Demonstrates: AC-017 — DURABILITY DEGRADED advisory + exit 0 when the
# factory-artifacts worktree is not found (PROJECT_DIR is not a git repo).
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "$0")/../../.." && pwd)"
DISPATCHER="$REPO_ROOT/target/release/factory-dispatcher"
PRECOMPACT_WASM="$REPO_ROOT/plugins/vsdd-factory/hook-plugins/precompact-flush.wasm"

WORK="$(mktemp -d)"
PROJECT_DIR="$WORK/project"  # plain directory — NOT a git repo
mkdir -p "$WORK/.factory/logs" "$WORK/hook-plugins" "$PROJECT_DIR"
cp "$PRECOMPACT_WASM" "$WORK/hook-plugins/precompact-flush.wasm"

cat > "$WORK/hooks-registry.toml" <<EOF
schema_version = 2

[[hooks]]
name = "precompact-flush"
event = "PreCompact"
plugin = "hook-plugins/precompact-flush.wasm"
priority = 100
timeout_ms = 30000
on_error = "continue"
async = false

[hooks.capabilities.read_file]
path_allow = ["${PROJECT_DIR}/.factory/"]

[hooks.capabilities.write_file]
path_allow = ["${PROJECT_DIR}/.factory/"]

[hooks.capabilities.exec_subprocess]
binary_allow = ["git"]
env_allow = ["HOME", "GIT_CONFIG_GLOBAL", "XDG_CONFIG_HOME", "PATH", "SSH_AUTH_SOCK"]
EOF

echo "=== Setup: PROJECT_DIR is NOT a git repo (no worktree mounted) ==="
echo "  PROJECT_DIR: $PROJECT_DIR"
ls -la "$PROJECT_DIR"
echo ""

echo "=== Sending PreCompact to dispatcher (error path) ==="
ENVELOPE='{"event_name":"PreCompact","tool_name":"","session_id":"demo-err-001","dispatcher_trace_id":"demo-trace-err-001"}'
OUTPUT=$(printf '%s' "$ENVELOPE" | \
    CLAUDE_PLUGIN_ROOT="$WORK" \
    CLAUDE_PROJECT_DIR="$PROJECT_DIR" \
    HOME="$WORK/home" \
    "$DISPATCHER" 2>&1)
EXIT_CODE=$?

echo "$OUTPUT"
echo ""
echo "=== Checking exit code and advisory message ==="
echo "exit code: $EXIT_CODE"

if [ "$EXIT_CODE" -eq 0 ]; then
    echo "PASS: exit 0 (advisory, not blocking)"
else
    echo "FAIL: expected exit 0, got $EXIT_CODE"
    exit 1
fi

if echo "$OUTPUT" | grep -q "DURABILITY DEGRADED"; then
    echo "PASS: DURABILITY DEGRADED advisory emitted"
else
    echo "FAIL: DURABILITY DEGRADED message not found"
    exit 1
fi
