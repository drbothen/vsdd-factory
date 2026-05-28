#!/usr/bin/env bats
# pass-null-lint-hook.bats — AC-9: hook emits Continue when lint_hook: null (null is valid;
#                             no existence check or codified_at check needed)
#
# Traces to:
#   BC-5.39.008 postcondition 9 (all checks pass => HookResult::Continue)
#   BC-5.39.008 invariant 5 (lint_hook: null is valid; no lint-hook-existence check triggered)
#   BC-5.39.008 EC-008 (lint_hook: null)
#
# Fixture: policies.yaml with lint_hook: null and codified_at: null.
# Expected: hook exits 0 (Continue).
#
# RED GATE PHASE: test skips because validate-policies-schema.wasm is not yet compiled.

setup() {
  REPO_ROOT="$(cd "${BATS_TEST_DIRNAME}/../../../.." && pwd)"
  PLUGIN_ROOT="$REPO_ROOT/plugins/vsdd-factory"
  DISPATCHER="$REPO_ROOT/target/release/factory-dispatcher"
  WASM_PLUGIN="$PLUGIN_ROOT/hook-plugins/validate-policies-schema.wasm"
  FIXTURE_SRC="${BATS_TEST_DIRNAME}/../fixtures/validate-policies-schema/pass-null-lint-hook"
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
  printf '{"event_name":"PostToolUse","tool_name":"Edit","session_id":"pass-null-lint-hook","tool_input":{"file_path":".factory/policies.yaml","content":""},"tool_response":{"exit_code":0,"stdout":"","stderr":""}}'
}

# ---------------------------------------------------------------------------
# AC-9: lint_hook: null => dispatcher exits 0 (Continue)
# Traces to BC-5.39.008 postcondition 9 + invariant 5
# ---------------------------------------------------------------------------

@test "test_BC_5_39_008_accepts_null_lint_hook_exits_0" {
  _require_artifacts
  _setup_fixture
  _write_registry
  cp "$WASM_PLUGIN" "$WORK/hook-plugins/"

  local envelope
  envelope="$(_policies_yaml_envelope)"
  run bash -c "printf '%s' '$envelope' | CLAUDE_PLUGIN_ROOT='$WORK' CLAUDE_PROJECT_DIR='$WORK' '$DISPATCHER' 2>&1 >/dev/null"

  [ "$status" -eq 0 ]
}

@test "test_BC_5_39_008_accepts_null_lint_hook_no_blocking_plugins" {
  _require_artifacts
  _setup_fixture
  _write_registry
  cp "$WASM_PLUGIN" "$WORK/hook-plugins/"

  local envelope
  envelope="$(_policies_yaml_envelope)"
  run bash -c "printf '%s' '$envelope' | CLAUDE_PLUGIN_ROOT='$WORK' CLAUDE_PROJECT_DIR='$WORK' '$DISPATCHER' 2>&1 >/dev/null"

  [ "$status" -eq 0 ]
  [[ "$output" != *"blocking_plugins=validate-policies-schema"* ]]
}
