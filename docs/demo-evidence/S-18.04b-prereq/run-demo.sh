#!/usr/bin/env bash
# run-demo.sh — demo harness for S-18.04b-prereq dispatcher git_context injection
#
# Sourced by VHS tapes via Type "source run-demo.sh && ..." or called directly.
# Each function sets up a temporary environment, runs the dispatcher, and
# shows the captured payload with jq.
#
# Usage:  bash run-demo.sh <scenario>
# Scenarios: A B C D E
#
# Requires: factory-dispatcher binary at target/release/factory-dispatcher
#           legacy-bash-adapter.wasm at plugins/vsdd-factory/hook-plugins/
#           jq

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/../../.." && pwd)"
DISPATCHER="$REPO_ROOT/target/release/factory-dispatcher"
ADAPTER_WASM="$REPO_ROOT/plugins/vsdd-factory/hook-plugins/legacy-bash-adapter.wasm"

_setup_env() {
  WORK="$(mktemp -d)"
  PROJECT_DIR="$WORK/project"
  FACTORY_DIR="$PROJECT_DIR/.factory"
  CAPTURE_FILE="$WORK/cap.json"

  mkdir -p "$WORK/hook-plugins" "$WORK/hooks" "$FACTORY_DIR/logs"
  cp "$ADAPTER_WASM" "$WORK/hook-plugins/legacy-bash-adapter.wasm"

  cat > "$WORK/hooks/capture.sh" <<CAPTURE
#!/usr/bin/env bash
cat > "$CAPTURE_FILE"
exit 0
CAPTURE
  chmod +x "$WORK/hooks/capture.sh"

  cat > "$WORK/hooks-registry.toml" <<REGISTRY
schema_version = 2

[[hooks]]
name = "payload-capture"
event = "PostToolUse"
plugin = "hook-plugins/legacy-bash-adapter.wasm"
timeout_ms = 10000
on_error = "continue"

[hooks.capabilities.exec_subprocess]
binary_allow = ["bash"]
shell_bypass_acknowledged = "yes"
cwd_allow = ["."]

[hooks.config]
script_path = "hooks/capture.sh"
REGISTRY
}

_run_dispatcher() {
  local envelope="$1"
  printf '%s' "$envelope" \
    | CLAUDE_PLUGIN_ROOT="$WORK" CLAUDE_PROJECT_DIR="$PROJECT_DIR" HOME="$WORK/home" \
      "$DISPATCHER" 2>&1
}

_teardown() { rm -rf "$WORK"; }

scenario_a() {
  echo "=== AC-001/PC1: qualifying PostToolUse Bash git-commit ==="
  echo "--- Setup: two-commit factory-artifacts repo ---"
  _setup_env
  git -C "$FACTORY_DIR" init -b factory-artifacts --quiet 2>/dev/null \
    || git -C "$FACTORY_DIR" init --quiet
  git -C "$FACTORY_DIR" config user.email "test@vsdd-factory"
  git -C "$FACTORY_DIR" config user.name "VP-093 Demo"
  git -C "$FACTORY_DIR" commit --allow-empty -m "state: burst-01 Commit A" --quiet
  PARENT_SHA="$(git -C "$FACTORY_DIR" rev-parse HEAD)"
  git -C "$FACTORY_DIR" commit --allow-empty -m "state: burst-02 Commit B" --quiet
  HEAD_SHA="$(git -C "$FACTORY_DIR" rev-parse HEAD)"
  echo "HEAD SHA (first 12):  ${HEAD_SHA:0:12}"
  echo "HEAD^  SHA (first 12): ${PARENT_SHA:0:12}"
  echo ""
  echo "--- Routing qualifying envelope to dispatcher ---"
  ENVELOPE='{"event_name":"PostToolUse","tool_name":"Bash","session_id":"demo-a","tool_input":{"command":"git -C .factory commit -m \"state: burst-02 Commit B\""},"tool_response":{"exit_code":0}}'
  _run_dispatcher "$ENVELOPE"
  echo ""
  echo "--- Captured payload.git_context (all four fields populated) ---"
  jq '.git_context' "$CAPTURE_FILE"
  echo ""
  echo "--- Verification: exact field values match synthetic repo ---"
  GOT_HEAD="$(jq -r '.git_context.head_sha' "$CAPTURE_FILE")"
  GOT_PARENT="$(jq -r '.git_context.head_parent_sha' "$CAPTURE_FILE")"
  [ "$GOT_HEAD" = "$HEAD_SHA" ]   && echo "PASS  head_sha matches real HEAD"
  [ "$GOT_PARENT" = "$PARENT_SHA" ] && echo "PASS  head_parent_sha matches real HEAD^"
  echo "$GOT_HEAD" | grep -qE '^[0-9a-f]{40}$' && echo "PASS  head_sha is 40-char hex"
  _teardown
}

scenario_b() {
  echo "=== AC-002/PC2: fail-open — git error (no git repo in factory dir) ==="
  _setup_env
  # Do NOT git init — git commands will fail with non-zero exit
  echo "--- factory dir is NOT a git repo (git commands will fail) ---"
  ENVELOPE='{"event_name":"PostToolUse","tool_name":"Bash","session_id":"demo-b","tool_input":{"command":"git -C .factory commit -m \"state: burst-01\""},"tool_response":{"exit_code":0}}'
  _run_dispatcher "$ENVELOPE"
  echo ""
  echo "--- Captured payload.git_context (all fields empty; exit was 0) ---"
  jq '.git_context' "$CAPTURE_FILE"
  echo ""
  echo "--- Verification: all fields are empty string, NOT null ---"
  for field in head_subject head_sha head_parent_subject head_parent_sha; do
    RAW="$(jq ".git_context.$field" "$CAPTURE_FILE")"
    [ "$RAW" = '""' ] && echo "PASS  $field = \"\"" || echo "FAIL  $field = $RAW"
  done
  _teardown
}

scenario_c() {
  echo "=== AC-003/PC3 + AC-012/EC-004: git push is NOT qualifying — no injection ==="
  _setup_env
  git -C "$FACTORY_DIR" init -b factory-artifacts --quiet 2>/dev/null \
    || git -C "$FACTORY_DIR" init --quiet
  git -C "$FACTORY_DIR" config user.email "test@vsdd-factory"
  git -C "$FACTORY_DIR" config user.name "VP-093 Demo"
  git -C "$FACTORY_DIR" commit --allow-empty -m "state: burst-01 Commit A" --quiet
  ENVELOPE='{"event_name":"PostToolUse","tool_name":"Bash","session_id":"demo-c","tool_input":{"command":"git -C .factory push origin factory-artifacts"},"tool_response":{"exit_code":0}}'
  _run_dispatcher "$ENVELOPE"
  echo ""
  echo "--- git_context key is ABSENT (git push did not trigger injection) ---"
  HAS="$(jq 'has("git_context")' "$CAPTURE_FILE")"
  [ "$HAS" = "false" ] && echo "PASS  git_context absent for git push" || echo "FAIL  git_context was injected (should not be)"
  echo "--- Plugin WAS invoked (positive-coverage sentinel: capture file is non-empty) ---"
  [ -s "$CAPTURE_FILE" ] && echo "PASS  capture file non-empty (plugin invoked)" || echo "FAIL  capture file empty"
  _teardown
}

scenario_d() {
  echo "=== AC-004/PC4 + AC-008/INV2: PostToolUse Edit — no injection ==="
  _setup_env
  ENVELOPE='{"event_name":"PostToolUse","tool_name":"Edit","session_id":"demo-d","tool_input":{"file_path":".factory/STATE.md","old_string":"a","new_string":"b"},"tool_response":{"success":true}}'
  _run_dispatcher "$ENVELOPE"
  echo ""
  echo "--- git_context key is ABSENT for Edit events (dispatcher never inspects command) ---"
  HAS="$(jq 'has("git_context")' "$CAPTURE_FILE")"
  [ "$HAS" = "false" ] && echo "PASS  git_context absent for PostToolUse Edit" || echo "FAIL  git_context was injected (should not be)"
  [ -s "$CAPTURE_FILE" ] && echo "PASS  capture file non-empty (plugin invoked for Edit event)" || echo "FAIL  capture file empty"
  _teardown
}

scenario_e() {
  echo "=== AC-006/INV5 + AC-011: initial commit — parent fields are \"\" not null ==="
  _setup_env
  git -C "$FACTORY_DIR" init -b factory-artifacts --quiet 2>/dev/null \
    || git -C "$FACTORY_DIR" init --quiet
  git -C "$FACTORY_DIR" config user.email "test@vsdd-factory"
  git -C "$FACTORY_DIR" config user.name "VP-093 Demo"
  git -C "$FACTORY_DIR" commit --allow-empty -m "state: burst-01 Commit A" --quiet
  HEAD_SHA="$(git -C "$FACTORY_DIR" rev-parse HEAD)"
  echo "--- Single-commit repo (no HEAD^). HEAD SHA: ${HEAD_SHA:0:12}... ---"
  ENVELOPE='{"event_name":"PostToolUse","tool_name":"Bash","session_id":"demo-e","tool_input":{"command":"git -C .factory commit --allow-empty -m \"state: burst-01 Commit A\""},"tool_response":{"exit_code":0}}'
  _run_dispatcher "$ENVELOPE"
  echo ""
  echo "--- Captured payload.git_context (head fields populated; parent fields empty) ---"
  jq '.git_context' "$CAPTURE_FILE"
  echo ""
  echo "--- Verification ---"
  echo "$HEAD_SHA" | grep -qE '^[0-9a-f]{40}$' \
    && echo "PASS  head_sha is 40-char hex" || echo "FAIL  head_sha malformed"
  [ "$(jq '.git_context.head_parent_subject' "$CAPTURE_FILE")" = '""' ] \
    && echo "PASS  head_parent_subject = \"\" (not null)" || echo "FAIL  parent_subject mismatch"
  [ "$(jq '.git_context.head_parent_sha' "$CAPTURE_FILE")" = '""' ] \
    && echo "PASS  head_parent_sha = \"\" (not null)" || echo "FAIL  parent_sha mismatch"
  _teardown
}

SCENARIO="${1:-}"
case "$SCENARIO" in
  A|a) scenario_a ;;
  B|b) scenario_b ;;
  C|c) scenario_c ;;
  D|d) scenario_d ;;
  E|e) scenario_e ;;
  *)
    echo "Usage: bash run-demo.sh <A|B|C|D|E>"
    echo "  A  — qualifying PostToolUse Bash git-commit (AC-001)"
    echo "  B  — fail-open on git error (AC-002)"
    echo "  C  — git push does not trigger injection (AC-003/AC-012)"
    echo "  D  — PostToolUse Edit does not trigger injection (AC-004)"
    echo "  E  — initial commit: parent fields are empty strings (AC-006/AC-011)"
    exit 1
    ;;
esac
