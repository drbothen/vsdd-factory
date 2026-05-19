#!/usr/bin/env bats
# fail-open-state-unreadable.bats — AC-14: hook emits Continue (fail-open) when STATE.md is unreadable
#
# Traces to:
#   BC-5.39.006 postcondition 7; invariant 9; EC-017
#
# Scenario: hook invoked with STATE.md file_path but no STATE.md exists in the WORK sandbox.
#           host::read_file returns a file-not-found error. Hook MUST fail-open (Continue, not Block).
# Note: fixture directory is empty by design — no STATE.md present.
#
# RED GATE PHASE: test skips if validate-dispatch-advance.wasm is not yet compiled.

setup() {
  REPO_ROOT="$(cd "${BATS_TEST_DIRNAME}/../../../.." && pwd)"
  PLUGIN_ROOT="$REPO_ROOT/plugins/vsdd-factory"
  DISPATCHER="$REPO_ROOT/target/release/factory-dispatcher"
  WASM_PLUGIN="$PLUGIN_ROOT/hook-plugins/validate-dispatch-advance.wasm"
  # No fixture setup — unreadable scenario uses absent file
  WORK="$(mktemp -d)"
  mkdir -p "$WORK/hook-plugins"
  mkdir -p "$WORK/.factory/logs"
}

teardown() {
  [ -n "${WORK:-}" ] && [ -d "$WORK" ] && find "$WORK" -type f -delete && find "$WORK" -type d -mindepth 1 | sort -r | xargs rmdir 2>/dev/null && rmdir "$WORK" 2>/dev/null || true
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

_state_md_envelope_absent() {
  # Points to STATE.md path that does not exist in WORK — triggers host::read_file error
  printf '{"event_name":"PostToolUse","tool_name":"Edit","session_id":"fail-open-state-unreadable","tool_input":{"file_path":".factory/STATE.md","content":""},"tool_response":{"exit_code":0,"stdout":"","stderr":""}}'
}

# ---------------------------------------------------------------------------
# AC-14: STATE.md absent => hook emits Continue (fail-open)
# Traces to BC-5.39.006 postcondition 7; invariant 9; EC-017
# ---------------------------------------------------------------------------

@test "AC-14 PASS (fail-open): hook emits Continue when STATE.md is not readable via host::read_file" {
  _require_artifacts
  # Intentionally do NOT copy any fixture — STATE.md absent from WORK
  [ ! -f "$WORK/.factory/STATE.md" ]
  _write_registry
  cp "$WASM_PLUGIN" "$WORK/hook-plugins/"

  local envelope
  envelope="$(_state_md_envelope_absent)"
  run bash -c "printf '%s' '$envelope' | CLAUDE_PLUGIN_ROOT='$WORK' CLAUDE_PROJECT_DIR='$WORK' '$DISPATCHER' 2>&1 >/dev/null"

  # Exit 0: fail-open means no block even though STATE.md is unreadable
  [ "$status" -eq 0 ]

  # No blocking_plugins= for a fail-open pass
  [[ "$output" != *"blocking_plugins=validate-dispatch-advance"* ]]
}
