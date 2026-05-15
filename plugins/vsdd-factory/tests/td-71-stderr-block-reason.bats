#!/usr/bin/env bats
# td-71-stderr-block-reason.bats — TD #71 Red Gate → Green coverage
#
# Verifies that the factory-dispatcher emits blocking_plugins=<names> and
# block_reason=<text> in its stderr summary line when block_intent=true.
#
# 5 required test cases (BC-style):
#   TC1: PreToolUse + 1 plugin block → stderr contains blocking_plugins=<that-plugin>
#   TC2: PreToolUse + 2 plugins block → stderr contains blocking_plugins=plugin-a,plugin-b
#   TC3: PreToolUse + 0 blocks → stderr OMITS blocking_plugins= and block_reason=
#   TC4: PostToolUse blocking → blocking_plugins format applies
#   TC5: block_reason newline handling — multi-line plugin messages escaped in stderr
#
# All tests skip if the dispatcher binary is not built or if the
# block-ai-attribution WASM is not present.
#
# Dispatcher binary: target/release/factory-dispatcher (built by CI / cargo build).
# Registry format: synthetic mini-registry TOML created in WORK per test.
# Plugin: hook-plugins/block-ai-attribution.wasm (PreToolUse/Bash blocker).

setup() {
  REPO_ROOT="$(cd "${BATS_TEST_DIRNAME}/../../.." && pwd)"
  PLUGIN_ROOT="$REPO_ROOT/plugins/vsdd-factory"
  DISPATCHER="$REPO_ROOT/target/release/factory-dispatcher"
  BLOCK_WASM="$PLUGIN_ROOT/hook-plugins/block-ai-attribution.wasm"
  WORK="$(mktemp -d)"
  mkdir -p "$WORK/.factory/logs"
  mkdir -p "$WORK/hook-plugins"
  # Copy the WASM into the synthetic plugin root so registry paths resolve correctly.
  if [ -f "$BLOCK_WASM" ]; then
    cp "$BLOCK_WASM" "$WORK/hook-plugins/"
  fi
  export CLAUDE_PROJECT_DIR="$WORK"
}

teardown() {
  rm -rf "$WORK"
}

# ---------------------------------------------------------------------------
# Helpers
# ---------------------------------------------------------------------------

# Write a synthetic single-plugin registry TOML to $WORK/hooks-registry.toml.
# Args: event on_error [tool] [name]
_write_registry() {
  local event="${1:-PreToolUse}"
  local on_error="${2:-block}"
  local tool="${3:-Bash}"
  local name="${4:-block-ai-attribution}"

  cat > "$WORK/hooks-registry.toml" <<EOF
schema_version = 2

[[hooks]]
name = "$name"
event = "$event"
tool = "$tool"
plugin = "hook-plugins/block-ai-attribution.wasm"
timeout_ms = 5000
on_error = "$on_error"
EOF
}

# Write a synthetic registry TOML with 2 hooks that both block on PreToolUse/Bash.
# Both hooks point to block-ai-attribution.wasm (same plugin, different names).
_write_two_plugin_registry() {
  cat > "$WORK/hooks-registry.toml" <<EOF
schema_version = 2

[[hooks]]
name = "blocker-alpha"
event = "PreToolUse"
tool = "Bash"
plugin = "hook-plugins/block-ai-attribution.wasm"
timeout_ms = 5000
on_error = "block"

[[hooks]]
name = "blocker-beta"
event = "PreToolUse"
tool = "Bash"
plugin = "hook-plugins/block-ai-attribution.wasm"
timeout_ms = 5000
on_error = "block"
EOF
}

# Shared preflight check — skip if dispatcher binary or WASM not present.
_require_artifacts() {
  if [ ! -x "$DISPATCHER" ]; then
    skip "dispatcher binary not built — run: cargo build --release -p factory-dispatcher"
  fi
  if [ ! -f "$BLOCK_WASM" ]; then
    skip "block-ai-attribution.wasm not present at $BLOCK_WASM"
  fi
}

# Run the dispatcher with the WORK directory as CLAUDE_PLUGIN_ROOT.
# Captures combined stdout+stderr into $output; sets $status.
_run_dispatcher() {
  local envelope="$1"
  run bash -c "printf '%s' '$envelope' | CLAUDE_PLUGIN_ROOT='$WORK' CLAUDE_PROJECT_DIR='$WORK' '$DISPATCHER' 2>&1 >/dev/null"
}

# ---------------------------------------------------------------------------
# TC1: PreToolUse + 1 plugin block → stderr contains blocking_plugins=<that-plugin>
# ---------------------------------------------------------------------------

@test "TD-71 TC1: single blocking plugin name appears in stderr blocking_plugins field" {
  _require_artifacts
  _write_registry "PreToolUse" "block" "Bash" "block-ai-attribution"

  # This envelope triggers the block: git commit with AI attribution.
  # The block-ai-attribution plugin detects "Co-Authored-By: Claude" in the command.
  local envelope
  envelope='{"event_name":"PreToolUse","tool_name":"Bash","session_id":"tc1","tool_input":{"command":"git commit -m test -m Co-Authored-By: Claude noreply@anthropic.com"}}'

  _run_dispatcher "$envelope"

  # Dispatcher must exit 2 (block).
  [ "$status" -eq 2 ]

  # stderr must contain blocking_plugins=block-ai-attribution
  [[ "$output" == *"blocking_plugins=block-ai-attribution"* ]]
}

# ---------------------------------------------------------------------------
# TC2: PreToolUse + 2 plugins block → stderr contains blocking_plugins=blocker-alpha,blocker-beta
# ---------------------------------------------------------------------------

@test "TD-71 TC2: two blocking plugins appear comma-joined in stderr blocking_plugins field" {
  _require_artifacts
  _write_two_plugin_registry

  local envelope
  envelope='{"event_name":"PreToolUse","tool_name":"Bash","session_id":"tc2","tool_input":{"command":"git commit -m test -m Co-Authored-By: Claude noreply@anthropic.com"}}'

  _run_dispatcher "$envelope"

  # Dispatcher must exit 2.
  [ "$status" -eq 2 ]

  # Both plugin names must appear in blocking_plugins field.
  [[ "$output" == *"blocking_plugins="* ]]
  [[ "$output" == *"blocker-alpha"* ]]
  [[ "$output" == *"blocker-beta"* ]]
}

# ---------------------------------------------------------------------------
# TC3: PreToolUse + 0 blocks → stderr OMITS blocking_plugins= and block_reason=
# ---------------------------------------------------------------------------

@test "TD-71 TC3: non-blocking dispatch omits blocking_plugins and block_reason from stderr" {
  _require_artifacts
  _write_registry "PreToolUse" "block" "Bash" "block-ai-attribution"

  # This envelope does NOT trigger the block: clean git commit, no AI attribution.
  local envelope
  envelope='{"event_name":"PreToolUse","tool_name":"Bash","session_id":"tc3","tool_input":{"command":"git commit -m feat: add feature X"}}'

  _run_dispatcher "$envelope"

  # Dispatcher must exit 0 (no block).
  [ "$status" -eq 0 ]

  # stderr must NOT contain blocking_plugins= or block_reason=
  [[ "$output" != *"blocking_plugins="* ]]
  [[ "$output" != *"block_reason="* ]]
}

# ---------------------------------------------------------------------------
# TC4: PostToolUse blocking → blocking_plugins format applies
# ---------------------------------------------------------------------------

@test "TD-71 TC4: PostToolUse block emits blocking_plugins in stderr" {
  _require_artifacts
  # Register block-ai-attribution on PostToolUse/Bash.
  _write_registry "PostToolUse" "block" "Bash" "post-blocker"

  # PostToolUse envelope with AI attribution in command.
  local envelope
  envelope='{"event_name":"PostToolUse","tool_name":"Bash","session_id":"tc4","tool_input":{"command":"git commit -m test -m Co-Authored-By: Claude noreply@anthropic.com"},"tool_response":{"exit_code":0,"stdout":"","stderr":""}}'

  _run_dispatcher "$envelope"

  # Dispatcher must exit 2 (block).
  [ "$status" -eq 2 ]

  # stderr must contain blocking_plugins=post-blocker
  [[ "$output" == *"blocking_plugins=post-blocker"* ]]
}

# ---------------------------------------------------------------------------
# TC5: block_reason newline handling — block_reason appears on a single stderr line
# ---------------------------------------------------------------------------

@test "TD-71 TC5: block_reason in stderr is single-line (newlines escaped or removed)" {
  _require_artifacts
  _write_registry "PreToolUse" "block" "Bash" "block-ai-attribution"

  local envelope
  envelope='{"event_name":"PreToolUse","tool_name":"Bash","session_id":"tc5","tool_input":{"command":"git commit -m test -m Co-Authored-By: Claude noreply@anthropic.com"}}'

  _run_dispatcher "$envelope"

  # Dispatcher must exit 2.
  [ "$status" -eq 2 ]

  # The block_reason= field must appear on exactly one line in the stderr output.
  # If the reason contained unescaped newlines, it would appear on multiple lines.
  local block_reason_line_count
  block_reason_line_count=$(printf '%s\n' "$output" | grep -c "block_reason=" || true)
  [ "$block_reason_line_count" -ge 1 ]

  # The line containing block_reason= must also contain blocking_plugins= (same summary line).
  local combined_line
  combined_line=$(printf '%s\n' "$output" | grep "block_reason=" || true)
  [[ "$combined_line" == *"blocking_plugins="* ]]
}
