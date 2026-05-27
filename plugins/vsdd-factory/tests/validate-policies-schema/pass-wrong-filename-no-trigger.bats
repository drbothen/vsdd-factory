#!/usr/bin/env bats
# pass-wrong-filename-no-trigger.bats — AC-13: path-component-strict guard: file_name "xpolicies.yaml"
#                                        is NOT "policies.yaml" so hook emits Continue without validation
#
# Traces to:
#   BC-5.39.008 invariant 3 (path-component-strict: file_name() == "policies.yaml" NOT ends_with)
#   BC-5.39.008 EC-020 (/some/dir/xpolicies.yaml file_name is "xpolicies.yaml" not a target)
#
# This test verifies the hook does NOT use ends_with("policies.yaml") which would match
# "xpolicies.yaml". The hook must use Path::file_name() == "policies.yaml" strictly.
#
# Fixture: xpolicies.yaml (not policies.yaml) with deliberately invalid content.
#          The envelope file_path is ".factory/xpolicies.yaml".
# Expected: hook exits 0 (Continue) — path-component guard rejects the trigger.
#
# RED GATE PHASE: test skips because validate-policies-schema.wasm is not yet compiled.

setup() {
  REPO_ROOT="$(cd "${BATS_TEST_DIRNAME}/../../../.." && pwd)"
  PLUGIN_ROOT="$REPO_ROOT/plugins/vsdd-factory"
  DISPATCHER="$REPO_ROOT/target/release/factory-dispatcher"
  WASM_PLUGIN="$PLUGIN_ROOT/hook-plugins/validate-policies-schema.wasm"
  FIXTURE_SRC="${BATS_TEST_DIRNAME}/../fixtures/validate-policies-schema/pass-wrong-filename-no-trigger"
  WORK="$(mktemp -d)"
  mkdir -p "$WORK/hook-plugins"
  mkdir -p "$WORK/.factory/logs"
}

teardown() {
  [ -n "${WORK:-}" ] && [ -d "$WORK" ] && find "$WORK" -type f -delete && find "$WORK" -type d -mindepth 1 | sort -r | xargs rmdir 2>/dev/null && rmdir "$WORK" 2>/dev/null || true
}

_setup_fixture() {
  # Copy xpolicies.yaml (not policies.yaml) to .factory/ — invalid content but wrong filename
  mkdir -p "$WORK/.factory"
  cp "$FIXTURE_SRC/xpolicies.yaml" "$WORK/.factory/xpolicies.yaml"
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

_xpolicies_yaml_envelope() {
  # file_path is xpolicies.yaml — ends_with "policies.yaml" would match this, but file_name() would not
  printf '{"event_name":"PostToolUse","tool_name":"Edit","session_id":"pass-wrong-filename-no-trigger","tool_input":{"file_path":".factory/xpolicies.yaml","content":""},"tool_response":{"exit_code":0,"stdout":"","stderr":""}}'
}

# ---------------------------------------------------------------------------
# AC-13: xpolicies.yaml file_name != "policies.yaml" => path-component-strict guard => Continue
# Traces to BC-5.39.008 invariant 3
# ---------------------------------------------------------------------------

@test "test_BC_5_39_008_invariant_path_component_strict_wrong_filename_exits_0" {
  _require_artifacts
  _setup_fixture
  _write_registry
  cp "$WASM_PLUGIN" "$WORK/hook-plugins/"

  local envelope
  envelope="$(_xpolicies_yaml_envelope)"
  run bash -c "printf '%s' '$envelope' | CLAUDE_PLUGIN_ROOT='$WORK' CLAUDE_PROJECT_DIR='$WORK' '$DISPATCHER' 2>&1 >/dev/null"

  [ "$status" -eq 0 ]
}

@test "test_BC_5_39_008_invariant_path_component_strict_wrong_filename_no_block" {
  _require_artifacts
  _setup_fixture
  _write_registry
  cp "$WASM_PLUGIN" "$WORK/hook-plugins/"

  local envelope
  envelope="$(_xpolicies_yaml_envelope)"
  run bash -c "printf '%s' '$envelope' | CLAUDE_PLUGIN_ROOT='$WORK' CLAUDE_PROJECT_DIR='$WORK' '$DISPATCHER' 2>&1 >/dev/null"

  [ "$status" -eq 0 ]
  [[ "$output" != *"blocking_plugins=validate-policies-schema"* ]]
}
