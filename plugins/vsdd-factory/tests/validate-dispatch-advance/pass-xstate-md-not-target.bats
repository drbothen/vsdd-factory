#!/usr/bin/env bats
# pass-xstate-md-not-target.bats — AC-16: path xSTATE.md does NOT trigger STATE.md validation
#
# Traces to:
#   BC-5.39.006 invariant 3; EC-019
#
# Scenario: file_path component is "xSTATE.md" not "STATE.md".
#           is_state_md_target() returns false. Hook must NOT validate the file.
#           Even though xSTATE.md content has all forbidden patterns, no block should occur.
#
# RED GATE PHASE: test skips if validate-dispatch-advance.wasm is not yet compiled.

setup() {
  REPO_ROOT="$(cd "${BATS_TEST_DIRNAME}/../../../.." && pwd)"
  PLUGIN_ROOT="$REPO_ROOT/plugins/vsdd-factory"
  DISPATCHER="$REPO_ROOT/target/release/factory-dispatcher"
  WASM_PLUGIN="$PLUGIN_ROOT/hook-plugins/validate-dispatch-advance.wasm"
  FIXTURE_SRC="${BATS_TEST_DIRNAME}/../fixtures/validate-dispatch-advance/pass-xstate-md-not-target"
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

_xstate_md_envelope() {
  # file_path ends with xSTATE.md — is_state_md_target must return false
  printf '{"event_name":"PostToolUse","tool_name":"Edit","session_id":"pass-xstate-md-not-target","tool_input":{"file_path":".factory/xSTATE.md","content":""},"tool_response":{"exit_code":0,"stdout":"","stderr":""}}'
}

# ---------------------------------------------------------------------------
# AC-16: path xSTATE.md => is_state_md_target returns false => no validation => Continue
# Traces to BC-5.39.006 invariant 3; EC-019
# ---------------------------------------------------------------------------

@test "AC-16 PASS: path-component-strict guard passes when file is xSTATE.md (not STATE.md)" {
  _require_artifacts
  _setup_fixture
  _write_registry
  cp "$WASM_PLUGIN" "$WORK/hook-plugins/"

  local envelope
  envelope="$(_xstate_md_envelope)"
  run bash -c "printf '%s' '$envelope' | CLAUDE_PLUGIN_ROOT='$WORK' CLAUDE_PROJECT_DIR='$WORK' '$DISPATCHER' 2>&1 >/dev/null"

  # Exit 0: xSTATE.md is not a target, so hook must not block even with forbidden content
  [ "$status" -eq 0 ]

  # No blocking_plugins= — hook skipped this file
  [[ "$output" != *"blocking_plugins=validate-dispatch-advance"* ]]
}
