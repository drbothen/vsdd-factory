#!/usr/bin/env bats
# fail-6-blocks.bats — AC-1 + AC-8: hook blocks when burst entry has only 6 of 9 blocks;
#                       block message names the 3 missing block types
#
# Traces to:
#   BC-5.39.004 postcondition 3 (missing block types => HookResult::BlockWithFix naming each)
#   BC-5.39.004 Canonical Test Vectors: "Only 6 blocks" row
# D-NNN closure: D-444(c) + D-446(a)
#
# Fixture: burst-log.md with correct h2; 6 blocks present (Parent-commit, Adversary verdict,
#          Files touched (Dim-1), Codifications, Dim-7, Closes); Dim-2/Dim-5/Dim-6 absent.
# Expected: hook exits 2 (block) and block_reason names Dim-2, Dim-5, Dim-6.
#
# RED GATE PHASE: test skips because validate-burst-log.wasm is not yet compiled.

setup() {
  REPO_ROOT="$(cd "${BATS_TEST_DIRNAME}/../../../.." && pwd)"
  PLUGIN_ROOT="$REPO_ROOT/plugins/vsdd-factory"
  DISPATCHER="$REPO_ROOT/target/release/factory-dispatcher"
  WASM_PLUGIN="$PLUGIN_ROOT/hook-plugins/validate-burst-log.wasm"
  FIXTURE_SRC="${BATS_TEST_DIRNAME}/../fixtures/validate-burst-log/fail-6-blocks"
  WORK="$(mktemp -d)"
  mkdir -p "$WORK/hook-plugins"
  mkdir -p "$WORK/.factory/logs"
}

teardown() {
  [ -n "${WORK:-}" ] && [ -d "$WORK" ] && find "$WORK" -type f -delete && find "$WORK" -type d -mindepth 1 | sort -r | xargs rmdir 2>/dev/null && rmdir "$WORK" 2>/dev/null || true
}

_setup_fixture() {
  cp -r "$FIXTURE_SRC/factory/." "$WORK/.factory/"
}

_write_registry() {
  cat > "$WORK/hooks-registry.toml" << 'TOML'
schema_version = 2

[[hooks]]
name = "validate-burst-log"
event = "PostToolUse"
tool = "Edit|Write"
plugin = "hook-plugins/validate-burst-log.wasm"
timeout_ms = 5000
on_error = "continue"

[hooks.capabilities.read_file]
path_allow = [
  ".factory/cycles/**",
]
TOML
}

_require_artifacts() {
  if [ ! -x "$DISPATCHER" ]; then
    skip "dispatcher binary not built -- run: cargo build --release -p factory-dispatcher"
  fi
  if [ ! -f "$WASM_PLUGIN" ]; then
    skip "validate-burst-log.wasm not built -- implement T-4 through T-7 of S-15.11"
  fi
}

_burst_log_envelope() {
  printf '{"event_name":"PostToolUse","tool_name":"Edit","session_id":"fail-6-blocks","tool_input":{"file_path":".factory/cycles/v1.0-feature-engine-discipline-pass-1/burst-log.md","content":""},"tool_response":{"exit_code":0,"stdout":"","stderr":""}}'
}

# ---------------------------------------------------------------------------
# AC-1: 6-block entry => dispatcher exits 2 (block)
# Traces to BC-5.39.004 postcondition 3
# ---------------------------------------------------------------------------

@test "AC-1 FAIL: hook blocks when burst entry has only 6 of 9 required blocks" {
  _require_artifacts
  _setup_fixture
  _write_registry
  cp "$WASM_PLUGIN" "$WORK/hook-plugins/"

  local envelope
  envelope="$(_burst_log_envelope)"
  run bash -c "printf '%s' '$envelope' | CLAUDE_PLUGIN_ROOT='$WORK' CLAUDE_PROJECT_DIR='$WORK' '$DISPATCHER' 2>&1 >/dev/null"

  # Exit 2: block signal emitted
  [ "$status" -eq 2 ]

  # blocking_plugins= names this hook
  [[ "$output" == *"blocking_plugins=validate-burst-log"* ]]
}

# ---------------------------------------------------------------------------
# AC-8: block message names each missing block type individually
# Traces to BC-5.39.004 postcondition 3
# ---------------------------------------------------------------------------

@test "AC-8 FAIL: block message names Dim-2, Dim-5, and Dim-6 as the three missing block types" {
  _require_artifacts
  _setup_fixture
  _write_registry
  cp "$WASM_PLUGIN" "$WORK/hook-plugins/"

  local envelope
  envelope="$(_burst_log_envelope)"
  run bash -c "printf '%s' '$envelope' | CLAUDE_PLUGIN_ROOT='$WORK' CLAUDE_PROJECT_DIR='$WORK' '$DISPATCHER' 2>&1 >/dev/null"

  # Exit 2: block signal
  [ "$status" -eq 2 ]

  # block_reason must name each missing block type (D-444(c) enforcement)
  [[ "$output" == *"Dim-2"* ]]
  [[ "$output" == *"Dim-5"* ]]
  [[ "$output" == *"Dim-6"* ]]
}
