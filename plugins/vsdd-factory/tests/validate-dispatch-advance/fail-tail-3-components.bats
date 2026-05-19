#!/usr/bin/env bats
# fail-tail-3-components.bats — AC-5: hook blocks when trajectory-tail has 3 arrow-N groups
#
# Traces to:
#   BC-5.39.006 postcondition 4; D-451(c); EC-006
#
# Fixture: STATE.md with current_step: trajectory tail →9→9→9 (3 components, not 4).
#          All other conditions valid: no forbidden patterns, all 4 index cites, valid D-chain.
# Expected: hook exits 2; block message names actual count (3) and required count (4), cites D-451(c).
#
# RED GATE PHASE: test skips if validate-dispatch-advance.wasm is not yet compiled.

setup() {
  REPO_ROOT="$(cd "${BATS_TEST_DIRNAME}/../../../.." && pwd)"
  PLUGIN_ROOT="$REPO_ROOT/plugins/vsdd-factory"
  DISPATCHER="$REPO_ROOT/target/release/factory-dispatcher"
  WASM_PLUGIN="$PLUGIN_ROOT/hook-plugins/validate-dispatch-advance.wasm"
  FIXTURE_SRC="${BATS_TEST_DIRNAME}/../fixtures/validate-dispatch-advance/fail-tail-3-components"
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
name = "validate-dispatch-advance"
event = "PostToolUse"
tool = "Edit|Write"
plugin = "hook-plugins/validate-dispatch-advance.wasm"
timeout_ms = 5000
on_error = "continue"

[hooks.capabilities.read_file]
path_allow = [
  ".factory",
]
TOML
}

_require_artifacts() {
  if [ ! -x "$DISPATCHER" ]; then
    skip "dispatcher binary not built -- run: cargo build --release -p factory-dispatcher"
  fi
  if [ ! -f "$WASM_PLUGIN" ]; then
    skip "validate-dispatch-advance.wasm not built -- implement T-5 through T-7 of S-15.14"
  fi
}

_state_md_envelope() {
  printf '{"event_name":"PostToolUse","tool_name":"Edit","session_id":"fail-tail-3-components","tool_input":{"file_path":".factory/STATE.md","content":""},"tool_response":{"exit_code":0,"stdout":"","stderr":""}}'
}

# ---------------------------------------------------------------------------
# AC-5: trajectory-tail →9→9→9 (3 components) => hook blocks (exit 2)
# Traces to BC-5.39.006 postcondition 4; D-451(c)
# ---------------------------------------------------------------------------

@test "AC-5 FAIL: hook blocks when trajectory-tail has 3 arrow-N groups (not 4)" {
  _require_artifacts
  _setup_fixture
  _write_registry
  cp "$WASM_PLUGIN" "$WORK/hook-plugins/"

  local envelope
  envelope="$(_state_md_envelope)"
  run bash -c "printf '%s' '$envelope' | CLAUDE_PLUGIN_ROOT='$WORK' CLAUDE_PROJECT_DIR='$WORK' '$DISPATCHER' 2>&1 >/dev/null"

  # Exit 2: block signal emitted for wrong tail length
  [ "$status" -eq 2 ]

  # blocking_plugins= names this hook
  [[ "$output" == *"blocking_plugins=validate-dispatch-advance"* ]]
}

@test "AC-5 FAIL: block message names 3 components and required LENGTH=4 citing D-451" {
  _require_artifacts
  _setup_fixture
  _write_registry
  cp "$WASM_PLUGIN" "$WORK/hook-plugins/"

  local envelope
  envelope="$(_state_md_envelope)"
  run bash -c "printf '%s' '$envelope' | CLAUDE_PLUGIN_ROOT='$WORK' CLAUDE_PROJECT_DIR='$WORK' '$DISPATCHER' 2>&1 >/dev/null"

  [ "$status" -eq 2 ]
  # Block message must name actual count 3 and required 4
  [[ "$output" == *"3"* ]]
  [[ "$output" == *"4"* ]]
  # Must cite D-451(c)
  [[ "$output" == *"D-451"* ]]
}
