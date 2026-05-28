#!/usr/bin/env bats
# fail-nonexistent-plugin.bats — AC-8: hook blocks when lint_hook: "nonexistent-plugin" is not
#                                 present in hooks-registry.toml; block message names missing plugin
#
# Traces to:
#   BC-5.39.008 postcondition 6 (lint_hook plugin not in registry => BlockWithFix naming missing plugin)
#   BC-5.39.008 EC-007 (nonexistent-plugin not in registry)
#
# Fixture: policies.yaml with lint_hook: "nonexistent-plugin" + hooks-registry.toml WITHOUT
#          a nonexistent-plugin entry.
# Expected: hook exits 2 (block); block_reason names the missing plugin.
#
# RED GATE PHASE: test skips because validate-policies-schema.wasm is not yet compiled.

setup() {
  REPO_ROOT="$(cd "${BATS_TEST_DIRNAME}/../../../.." && pwd)"
  PLUGIN_ROOT="$REPO_ROOT/plugins/vsdd-factory"
  DISPATCHER="$REPO_ROOT/target/release/factory-dispatcher"
  WASM_PLUGIN="$PLUGIN_ROOT/hook-plugins/validate-policies-schema.wasm"
  FIXTURE_SRC="${BATS_TEST_DIRNAME}/../fixtures/validate-policies-schema/fail-nonexistent-plugin"
  WORK="$(mktemp -d)"
  mkdir -p "$WORK/hook-plugins"
  mkdir -p "$WORK/.factory/logs"
  mkdir -p "$WORK/plugins/vsdd-factory"
}

teardown() {
  [ -n "${WORK:-}" ] && [ -d "$WORK" ] && find "$WORK" -type f -delete && find "$WORK" -type d -mindepth 1 | sort -r | xargs rmdir 2>/dev/null && rmdir "$WORK" 2>/dev/null || true
}

_setup_fixture() {
  cp -r "$FIXTURE_SRC/factory/." "$WORK/.factory/"
  cp "$FIXTURE_SRC/hooks-registry.toml" "$WORK/plugins/vsdd-factory/hooks-registry.toml"
}

_write_registry() {
  cat > "$WORK/hooks-registry.toml" << 'TOML'
schema_version = 2

[[hooks]]
name = "validate-policies-schema"
event = "PostToolUse"
tool = "Edit|Write"
plugin = "hook-plugins/validate-policies-schema.wasm"
priority = 157
timeout_ms = 5000
on_error = "continue"

[hooks.capabilities.read_file]
path_allow = [".factory", "plugins/vsdd-factory"]
TOML
}

_require_artifacts() {
  if [ ! -x "$DISPATCHER" ]; then
    skip "dispatcher binary not built -- run: cargo build --release -p factory-dispatcher"
  fi
  if [ ! -f "$WASM_PLUGIN" ]; then
    skip "validate-policies-schema.wasm not built -- implement T-6 through T-11 of S-15.15"
  fi
}

_policies_yaml_envelope() {
  printf '{"event_name":"PostToolUse","tool_name":"Edit","session_id":"fail-nonexistent-plugin","tool_input":{"file_path":".factory/policies.yaml","content":""},"tool_response":{"exit_code":0,"stdout":"","stderr":""}}'
}

# ---------------------------------------------------------------------------
# AC-8: lint_hook plugin not in registry => dispatcher exits 2 (block)
# Traces to BC-5.39.008 postcondition 6
# ---------------------------------------------------------------------------

@test "test_BC_5_39_008_rejects_nonexistent_lint_hook_plugin_exits_2" {
  _require_artifacts
  _setup_fixture
  _write_registry
  cp "$WASM_PLUGIN" "$WORK/hook-plugins/"

  local envelope
  envelope="$(_policies_yaml_envelope)"
  run bash -c "printf '%s' '$envelope' | CLAUDE_PLUGIN_ROOT='$WORK' CLAUDE_PROJECT_DIR='$WORK' '$DISPATCHER' 2>&1 >/dev/null"

  [ "$status" -eq 2 ]
  [[ "$output" == *"blocking_plugins=validate-policies-schema"* ]]
}

@test "test_BC_5_39_008_rejects_nonexistent_lint_hook_plugin_block_reason_names_missing_plugin" {
  _require_artifacts
  _setup_fixture
  _write_registry
  cp "$WASM_PLUGIN" "$WORK/hook-plugins/"

  local envelope
  envelope="$(_policies_yaml_envelope)"
  run bash -c "printf '%s' '$envelope' | CLAUDE_PLUGIN_ROOT='$WORK' CLAUDE_PROJECT_DIR='$WORK' '$DISPATCHER' 2>&1 >/dev/null"

  [ "$status" -eq 2 ]
  [[ "$output" == *"nonexistent-plugin"* ]]
}
