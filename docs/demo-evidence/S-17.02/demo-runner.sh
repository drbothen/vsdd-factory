#!/usr/bin/env bash
# demo-runner.sh — shared setup + individual scenario runner for S-17.02 VHS demos.
# Called by each AC-NNN.tape script with the scenario name as $1.
#
# Usage: ./demo-runner.sh <scenario>
#   scenario: ac001 | ac002 | ac003 | ac005 | ac012 | ac013 | ac009 | ac010
set -uo pipefail

WORKTREE=/Users/zious/Documents/GITHUB/vsdd-factory/.worktrees/S-17.02
DISPATCHER="$WORKTREE/target/release/factory-dispatcher"
GUARD_WASM="$WORKTREE/plugins/vsdd-factory/hook-plugins/verify-factory-lock.wasm"
REGISTRY="$WORKTREE/plugins/vsdd-factory/hooks-registry.toml"

# ── Shared setup ──────────────────────────────────────────────────────────────
_setup_work() {
  WORK=$(mktemp -d)
  mkdir -p "$WORK/.factory/logs" "$WORK/hook-plugins"
  cp "$GUARD_WASM" "$WORK/hook-plugins/verify-factory-lock.wasm"
}

_write_full_registry() {
  cat > "$WORK/hooks-registry.toml" <<'EOF'
schema_version = 2
[[hooks]]
name = "verify-factory-lock"
plugin = "hook-plugins/verify-factory-lock.wasm"
event = "PreToolUse"
tool = "Edit|Write|Agent"
async = false
on_error = "continue"
timeout_ms = 5000
[hooks.capabilities.read_file]
path_allow = [".factory/STATE.md"]
[hooks.capabilities.exec_subprocess]
binary_allow = ["git"]
env_allow = ["HOME", "GIT_CONFIG_GLOBAL", "XDG_CONFIG_HOME"]
[[hooks]]
name = "verify-factory-lock-bash"
plugin = "hook-plugins/verify-factory-lock.wasm"
event = "PreToolUse"
tool = "Bash"
async = false
on_error = "continue"
timeout_ms = 5000
[hooks.capabilities.read_file]
path_allow = [".factory/STATE.md"]
[hooks.capabilities.exec_subprocess]
binary_allow = ["git"]
env_allow = ["HOME", "GIT_CONFIG_GLOBAL", "XDG_CONFIG_HOME"]
EOF
}

_write_capability_omitted_registry() {
  cat > "$WORK/hooks-registry.toml" <<'EOF'
schema_version = 2
# Intentionally missing [hooks.capabilities.read_file] — graceful degrade demo
[[hooks]]
name = "verify-factory-lock"
plugin = "hook-plugins/verify-factory-lock.wasm"
event = "PreToolUse"
tool = "Edit|Write|Agent"
async = false
on_error = "continue"
timeout_ms = 5000
[hooks.capabilities.exec_subprocess]
binary_allow = ["git"]
EOF
}

_write_foreign_unexpired_state() {
  cat > "$WORK/.factory/STATE.md" <<'EOF'
---
document_type: state
version: "0.0.1-demo"
phase: demo
current_step: "demo"
factory_lock:
  holder: "other@example.com"
  locked_at: "2026-06-11T10:00:00Z"
  expires_at: "2099-01-01T00:00:00Z"
---
# STATE — foreign unexpired lock fixture
EOF
}

_write_foreign_expired_state() {
  cat > "$WORK/.factory/STATE.md" <<'EOF'
---
document_type: state
version: "0.0.1-demo"
phase: demo
current_step: "demo"
factory_lock:
  holder: "other@example.com"
  locked_at: "2020-01-01T00:00:00Z"
  expires_at: "2020-01-01T00:45:00Z"
---
# STATE — foreign expired lock fixture
EOF
}

_write_self_held_state() {
  local self_email
  self_email="$(git -C "$WORKTREE" config user.email 2>/dev/null | tr -d '\n')"
  cat > "$WORK/.factory/STATE.md" <<EOF
---
document_type: state
version: "0.0.1-demo"
phase: demo
current_step: "demo"
factory_lock:
  holder: "${self_email}"
  locked_at: "2026-06-11T10:00:00Z"
  expires_at: "2099-01-01T00:00:00Z"
---
# STATE — self-held lock fixture (holder=${self_email})
EOF
}

_dispatch() {
  local envelope="$1"
  set +e
  printf '%s' "$envelope" | CLAUDE_PLUGIN_ROOT="$WORK" CLAUDE_PROJECT_DIR="$WORK" "$DISPATCHER" 2>&1
  local exit_code=$?
  set -e
  echo "--- exit code: $exit_code ---"
}

# ── Scenario dispatch ─────────────────────────────────────────────────────────
SCENARIO="${1:-}"

case "$SCENARIO" in

  ac001)
    echo "=== AC-001 (T-2): foreign unexpired lock → Edit → BLOCK ==="
    echo "BC-4.13.001 PC1: ForeignLockHeld with 5-field message"
    echo ""
    _setup_work
    _write_full_registry
    _write_foreign_unexpired_state
    E='{"event_name":"PreToolUse","tool_name":"Edit","session_id":"demo","dispatcher_trace_id":"demo-ac001","tool_input":{"file_path":".factory/STATE.md"}}'
    _dispatch "$E"
    rm -rf "$WORK"
    ;;

  ac002)
    echo "=== AC-002 (T-3): foreign EXPIRED lock → Edit → CONTINUE ==="
    echo "BC-4.13.001 PC2: LockExpired → fail-open (no block)"
    echo ""
    _setup_work
    _write_full_registry
    _write_foreign_expired_state
    E='{"event_name":"PreToolUse","tool_name":"Edit","session_id":"demo","dispatcher_trace_id":"demo-ac002","tool_input":{"file_path":".factory/STATE.md"}}'
    _dispatch "$E"
    rm -rf "$WORK"
    ;;

  ac003)
    echo "=== AC-003 (T-4): self-held lock → Edit → CONTINUE ==="
    echo "BC-4.13.001 PC3: SelfHeld → developer never blocked"
    echo ""
    _setup_work
    _write_full_registry
    _write_self_held_state
    E='{"event_name":"PreToolUse","tool_name":"Edit","session_id":"demo","dispatcher_trace_id":"demo-ac003","tool_input":{"file_path":".factory/STATE.md"}}'
    _dispatch "$E"
    rm -rf "$WORK"
    ;;

  ac005)
    echo "=== AC-005 (T-5): foreign lock → Read → NOT intercepted → CONTINUE ==="
    echo "BC-4.13.001 PC5: Read is not in registry tool regex — guard never invoked"
    echo ""
    _setup_work
    _write_full_registry
    _write_foreign_unexpired_state
    E='{"event_name":"PreToolUse","tool_name":"Read","session_id":"demo","dispatcher_trace_id":"demo-ac005","tool_input":{"file_path":".factory/STATE.md"}}'
    _dispatch "$E"
    rm -rf "$WORK"
    ;;

  ac012)
    echo "=== AC-012 (T-6): foreign lock → Bash 'git push origin factory-artifacts' → BLOCK ==="
    echo "BC-4.13.001 T-6: push-regex matches → ForeignLockHeld block"
    echo ""
    _setup_work
    _write_full_registry
    _write_foreign_unexpired_state
    E='{"event_name":"PreToolUse","tool_name":"Bash","session_id":"demo","dispatcher_trace_id":"demo-ac012","tool_input":{"command":"git push origin factory-artifacts"}}'
    _dispatch "$E"
    rm -rf "$WORK"
    ;;

  ac013)
    echo "=== AC-013 (T-7): foreign lock → Bash 'cat .factory/STATE.md' → CONTINUE ==="
    echo "BC-4.13.001 T-7: non-push Bash → push-regex no match → Continue (sub-ms)"
    echo ""
    _setup_work
    _write_full_registry
    _write_foreign_unexpired_state
    E='{"event_name":"PreToolUse","tool_name":"Bash","session_id":"demo","dispatcher_trace_id":"demo-ac013","tool_input":{"command":"cat .factory/STATE.md"}}'
    _dispatch "$E"
    rm -rf "$WORK"
    ;;

  ac009)
    echo "=== AC-009/AC-014/AC-016: Registry shape — two entries, async=false, env_allow ==="
    echo "BC-4.13.001 Invariant 5: both entries + both capability blocks + env_allow present"
    echo ""
    grep -A12 'verify-factory-lock' "$REGISTRY"
    echo ""
    echo "--- async_false_count (must be 2) ---"
    awk '
      /name = "verify-factory-lock/ { in_section=1; has_async_false=0 }
      /^\[\[hooks\]\]/ && in_section { if (has_async_false) count++; in_section = 0 }
      in_section && /^async = false/ { has_async_false = 1 }
      END { if (in_section && has_async_false) count++; print "async_false_count=" count+0 }
    ' "$REGISTRY"
    echo ""
    echo "--- env_allow_HOME_count (must be 2) ---"
    awk '
      /name = "verify-factory-lock/ { in_section=1 }
      /^\[\[hooks\]\]/ && in_section { in_section=0 }
      in_section && /env_allow/ && /HOME/ { count++ }
      END { print "env_allow_HOME_count=" count+0 }
    ' "$REGISTRY"
    ;;

  ac010)
    echo "=== AC-010 (T-8): capability-omitted registry → Edit → graceful-degrade → CONTINUE ==="
    echo "BC-4.13.001 Invariant 6: CapabilityDenied → fail-open (guard never crashes)"
    echo ""
    _setup_work
    _write_capability_omitted_registry
    _write_foreign_unexpired_state
    E='{"event_name":"PreToolUse","tool_name":"Edit","session_id":"demo","dispatcher_trace_id":"demo-ac010","tool_input":{"file_path":".factory/STATE.md"}}'
    _dispatch "$E"
    rm -rf "$WORK"
    ;;

  *)
    echo "Usage: $0 <scenario>"
    echo "Scenarios: ac001 ac002 ac003 ac005 ac012 ac013 ac009 ac010"
    exit 1
    ;;
esac
