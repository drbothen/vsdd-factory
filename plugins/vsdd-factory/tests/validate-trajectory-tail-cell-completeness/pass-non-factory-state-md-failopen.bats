#!/usr/bin/env bats
# pass-non-factory-state-md-failopen.bats — AC-23: STATE.md written to a non-factory path
#   (e.g., file_path without .factory path component) => Continue immediately; STATE.md arm
#   MUST NOT fire for STATE.md files outside .factory/
#
# Traces to:
#   BC-5.39.009 EC-019 (non-factory STATE.md => path-component-walk rejects; Continue)
#   BC-5.39.009 Precondition 4 (STATE.md arm requires .factory path component)
#   BC-5.39.009 invariant 3 (path-component-strict)
#
# Fixture: STATE.md content with no trajectory tail; envelope file_path is
#   "some-other/STATE.md" — lacks .factory component.
# Expected: hook exits 0 (Continue); no blocking_plugins signal
#   (Precondition 4 guard: components walk finds no .factory component)
#
# RED GATE PHASE: test skips because validate-trajectory-tail-cell-completeness.wasm not yet compiled.

setup() {
  REPO_ROOT="$(cd "${BATS_TEST_DIRNAME}/../../../.." && pwd)"
  PLUGIN_ROOT="$REPO_ROOT/plugins/vsdd-factory"
  DISPATCHER="$REPO_ROOT/target/release/factory-dispatcher"
  WASM_PLUGIN="$PLUGIN_ROOT/hook-plugins/validate-trajectory-tail-cell-completeness.wasm"
  FIXTURE_SRC="${BATS_TEST_DIRNAME}/../fixtures/validate-trajectory-tail-cell-completeness/pass-non-factory-state-md-failopen"
  WORK="$(mktemp -d)"
  mkdir -p "$WORK/hook-plugins"
  mkdir -p "$WORK/.factory/logs"
}

teardown() {
  [ -n "${WORK:-}" ] && [ -d "$WORK" ] && find "$WORK" -type f -delete && find "$WORK" -type d -mindepth 1 | sort -r | xargs rmdir 2>/dev/null && rmdir "$WORK" 2>/dev/null || true
}

_setup_fixture() {
  # Place STATE.md in a non-.factory directory
  mkdir -p "$WORK/some-other"
  cp "$FIXTURE_SRC/STATE.md" "$WORK/some-other/STATE.md"
}

_write_registry() {
  cat > "$WORK/hooks-registry.toml" << 'TOML'
schema_version = 2

[[hooks]]
name = "validate-trajectory-tail-cell-completeness"
event = "PostToolUse"
tool = "Edit|Write"
plugin = "hook-plugins/validate-trajectory-tail-cell-completeness.wasm"
priority = 158
timeout_ms = 5000
on_error = "continue"

[hooks.capabilities.read_file]
path_allow = [".factory"]
TOML
}

_require_artifacts() {
  if [ ! -x "$DISPATCHER" ]; then
    skip "dispatcher binary not built -- run: cargo build --release -p factory-dispatcher"
  fi
  if [ ! -f "$WASM_PLUGIN" ]; then
    skip "validate-trajectory-tail-cell-completeness.wasm not built -- implement T-5 through T-8 of S-15.17"
  fi
}

_non_factory_state_md_envelope() {
  # file_path has STATE.md basename but NO .factory component in path
  printf '{"event_name":"PostToolUse","tool_name":"Edit","session_id":"pass-non-factory-state-md","tool_input":{"file_path":"some-other/STATE.md","content":""},"tool_response":{"exit_code":0,"stdout":"","stderr":""}}'
}

# ---------------------------------------------------------------------------
# AC-23 / EC-019 + Precondition 4: non-factory STATE.md => Continue exit 0
# ---------------------------------------------------------------------------

@test "test_BC_5_39_009_EC019_non_factory_state_md_exits_0" {
  _require_artifacts
  _setup_fixture
  _write_registry
  cp "$WASM_PLUGIN" "$WORK/hook-plugins/"

  local envelope
  envelope="$(_non_factory_state_md_envelope)"
  run bash -c "printf '%s' '$envelope' | CLAUDE_PLUGIN_ROOT='$WORK' CLAUDE_PROJECT_DIR='$WORK' '$DISPATCHER' 2>&1 >/dev/null"

  [ "$status" -eq 0 ]
}

@test "test_BC_5_39_009_EC019_non_factory_state_md_no_block" {
  _require_artifacts
  _setup_fixture
  _write_registry
  cp "$WASM_PLUGIN" "$WORK/hook-plugins/"

  local envelope
  envelope="$(_non_factory_state_md_envelope)"
  run bash -c "printf '%s' '$envelope' | CLAUDE_PLUGIN_ROOT='$WORK' CLAUDE_PROJECT_DIR='$WORK' '$DISPATCHER' 2>&1 >/dev/null"

  [ "$status" -eq 0 ]
  [[ "$output" != *"blocking_plugins=validate-trajectory-tail-cell-completeness"* ]]
}
