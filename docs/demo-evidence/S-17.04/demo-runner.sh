#!/usr/bin/env bash
# demo-runner.sh — reusable dispatcher driver for S-17.04 VHS tapes.
# Accepts scenario name as $1 and prints labelled output.
# Called by *.tape files. Must be run from worktree root.
set -euo pipefail

WORKTREE_ROOT="$(cd "$(dirname "$0")/../../.." && pwd)"
DISPATCHER="$WORKTREE_ROOT/target/release/factory-dispatcher"
WASM="$WORKTREE_ROOT/plugins/vsdd-factory/hook-plugins/verify-state-timestamp-refresh.wasm"

SCENARIO="${1:-}"

WORK="$(mktemp -d)"
cleanup() { rm -rf "$WORK"; }
trap cleanup EXIT

mkdir -p "$WORK/.factory/logs" "$WORK/hook-plugins"
cp "$WASM" "$WORK/hook-plugins/verify-state-timestamp-refresh.wasm"

cat > "$WORK/hooks-registry.toml" <<'TOML'
schema_version = 2
[[hooks]]
name = "verify-state-timestamp-refresh"
event = "PreToolUse"
tool = "Edit|Write|MultiEdit"
plugin = "hook-plugins/verify-state-timestamp-refresh.wasm"
priority = 143
timeout_ms = 5000
on_error = "continue"
async = false
[hooks.capabilities.read_file]
path_allow = [".factory/STATE.md"]
TOML

TS_OLD="2026-06-11T10:00:00Z"
TS_NEW="2026-06-11T11:00:00Z"

write_state_no_lock() {
  local ts="${1:-$TS_OLD}"
  cat > "$WORK/.factory/STATE.md" <<EOF
---
document_type: state
version: "0.0.1-demo"
timestamp: "${ts}"
phase: test
current_step: "demo"
---

# STATE (no lock)
EOF
}

write_state_with_lock() {
  local ts="${1:-$TS_OLD}"
  local exp="${2:-2026-06-11T10:45:00Z}"
  cat > "$WORK/.factory/STATE.md" <<EOF
---
document_type: state
version: "0.0.1-demo"
timestamp: "${ts}"
phase: test
current_step: "demo"
factory_lock:
  holder: "dev@example.com"
  locked_at: "2026-06-11T09:00:00Z"
  expires_at: "${exp}"
---

# STATE (lock held)
EOF
}

run_dispatcher() {
  local envelope="$1"
  printf '%s' "$envelope" \
    | CLAUDE_PLUGIN_ROOT="$WORK" CLAUDE_PROJECT_DIR="$WORK" "$DISPATCHER" 2>&1
  echo "exit_code=$?"
}

case "$SCENARIO" in

  ac005-stale)
    echo "# SCENARIO: AC-005 — stale timestamp Write → Block (TimestampStale)"
    echo "# On-disk STATE.md: timestamp=${TS_OLD}"
    echo "# Proposed content: SAME timestamp (not advanced) → guard must block"
    echo ""
    write_state_no_lock "$TS_OLD"

    PROPOSED="---\\ndocument_type: state\\nversion: 0.0.1-demo\\ntimestamp: ${TS_OLD}\\nphase: test\\ncurrent_step: demo\\n---\\n\\n# STATE\\n"
    ENVELOPE="{\"event_name\":\"PreToolUse\",\"tool_name\":\"Write\",\"session_id\":\"ac005\",\"dispatcher_trace_id\":\"d-ac005\",\"tool_input\":{\"file_path\":\".factory/STATE.md\",\"content\":\"${PROPOSED}\"}}"

    echo "$ printf '%s' \$ENVELOPE | CLAUDE_PLUGIN_ROOT=\$WORK CLAUDE_PROJECT_DIR=\$WORK factory-dispatcher 2>&1"
    run_dispatcher "$ENVELOPE"
    ;;

  ac003-allow-fresh)
    echo "# SCENARIO: AC-003 — fresh timestamp Write → Continue (exit 0 + guard_ran sentinel)"
    echo "# On-disk STATE.md: timestamp=${TS_OLD}"
    echo "# Proposed content: timestamp=${TS_NEW} (advanced) → guard must allow"
    echo ""
    write_state_no_lock "$TS_OLD"

    PROPOSED="---\\ndocument_type: state\\nversion: 0.0.1-demo\\ntimestamp: ${TS_NEW}\\nphase: test\\ncurrent_step: demo\\n---\\n\\n# STATE\\n"
    ENVELOPE="{\"event_name\":\"PreToolUse\",\"tool_name\":\"Write\",\"session_id\":\"ac003\",\"dispatcher_trace_id\":\"d-ac003\",\"tool_input\":{\"file_path\":\".factory/STATE.md\",\"content\":\"${PROPOSED}\"}}"

    echo "$ printf '%s' \$ENVELOPE | CLAUDE_PLUGIN_ROOT=\$WORK CLAUDE_PROJECT_DIR=\$WORK factory-dispatcher 2>&1"
    run_dispatcher "$ENVELOPE"
    ;;

  ac018-absolute-path)
    echo "# SCENARIO: AC-018 — absolute file_path + stale → Block (P0 ends_with fix)"
    echo "# Claude Code emits absolute paths (e.g. /var/.../proj/.factory/STATE.md)."
    echo "# Prior EC-006 used env-var strip (dead in WASM sandbox) — guard was inert."
    echo "# Fix: trigger when path ends_with '/.factory/STATE.md'."
    echo ""
    write_state_no_lock "$TS_OLD"

    ABS_PATH="$WORK/.factory/STATE.md"
    PROPOSED="---\\ndocument_type: state\\nversion: 0.0.1-demo\\ntimestamp: ${TS_OLD}\\nphase: test\\ncurrent_step: demo\\n---\\n\\n# STATE\\n"
    ENVELOPE="{\"event_name\":\"PreToolUse\",\"tool_name\":\"Write\",\"session_id\":\"ac018\",\"dispatcher_trace_id\":\"d-ac018\",\"tool_input\":{\"file_path\":\"${ABS_PATH}\",\"content\":\"${PROPOSED}\"}}"

    echo "# file_path: ${ABS_PATH}"
    echo "$ printf '%s' \$ENVELOPE | CLAUDE_PLUGIN_ROOT=\$WORK CLAUDE_PROJECT_DIR=\$WORK factory-dispatcher 2>&1"
    run_dispatcher "$ENVELOPE"
    ;;

  ac006-lock-expiry-stale)
    echo "# SCENARIO: AC-006 — lock held, expires_at absent → Block (LockExpiryStale)"
    echo "# On-disk STATE.md: lock held with expires_at=${TS_OLD/T10/T10:45}Z"
    echo "# Proposed content: timestamp advanced but NO expires_at in lock block"
    echo ""
    write_state_with_lock "$TS_OLD" "2026-06-11T10:45:00Z"

    PROPOSED="---\\ndocument_type: state\\nversion: 0.0.1-demo\\ntimestamp: ${TS_NEW}\\nphase: test\\ncurrent_step: demo\\nfactory_lock:\\n  holder: dev@example.com\\n  locked_at: 2026-06-11T09:00:00Z\\n---\\n\\n# STATE\\n"
    ENVELOPE="{\"event_name\":\"PreToolUse\",\"tool_name\":\"Write\",\"session_id\":\"ac006\",\"dispatcher_trace_id\":\"d-ac006\",\"tool_input\":{\"file_path\":\".factory/STATE.md\",\"content\":\"${PROPOSED}\"}}"

    echo "$ printf '%s' \$ENVELOPE | CLAUDE_PLUGIN_ROOT=\$WORK CLAUDE_PROJECT_DIR=\$WORK factory-dispatcher 2>&1"
    run_dispatcher "$ENVELOPE"
    ;;

  *)
    echo "Usage: demo-runner.sh <scenario>"
    echo "Scenarios: ac005-stale  ac003-allow-fresh  ac018-absolute-path  ac006-lock-expiry-stale"
    exit 1
    ;;
esac
