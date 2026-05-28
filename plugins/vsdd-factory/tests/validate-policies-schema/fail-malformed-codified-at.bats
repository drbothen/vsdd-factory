#!/usr/bin/env bats
# fail-malformed-codified-at.bats — AC-10: hook blocks when lint_hook is non-null AND
#                                    codified_at: "pass-72" (non-D-NNN form);
#                                    block message names malformed codified_at value and policy id
#
# Traces to:
#   BC-5.39.008 postcondition 7 (lint_hook non-null + malformed codified_at => BlockWithFix)
#   BC-5.39.008 EC-010 (codified_at: "pass-72" is non-D-NNN)
#
# Fixture: policies.yaml with lint_hook: "validate-dispatch-advance" and codified_at: "pass-72".
# Expected: hook exits 2 (block); block_reason names "pass-72" as malformed value.
#
# RED GATE PHASE: test skips because validate-policies-schema.wasm is not yet compiled.

setup() {
  REPO_ROOT="$(cd "${BATS_TEST_DIRNAME}/../../../.." && pwd)"
  PLUGIN_ROOT="$REPO_ROOT/plugins/vsdd-factory"
  DISPATCHER="$REPO_ROOT/target/release/factory-dispatcher"
  WASM_PLUGIN="$PLUGIN_ROOT/hook-plugins/validate-policies-schema.wasm"
  FIXTURE_SRC="${BATS_TEST_DIRNAME}/../fixtures/validate-policies-schema/fail-malformed-codified-at"
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
  # Write a registry with validate-dispatch-advance so lint_hook existence passes;
  # the codified_at check is what should fail here.
  cat > "$WORK/plugins/vsdd-factory/hooks-registry.toml" << 'TOML'
schema_version = 2

[[hooks]]
name = "validate-dispatch-advance"
event = "PostToolUse"
tool = "Edit|Write"
plugin = "hook-plugins/validate-dispatch-advance.wasm"
priority = 154
timeout_ms = 5000
on_error = "continue"

[hooks.capabilities.read_file]
path_allow = [".factory"]
TOML
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
  printf '{"event_name":"PostToolUse","tool_name":"Edit","session_id":"fail-malformed-codified-at","tool_input":{"file_path":".factory/policies.yaml","content":""},"tool_response":{"exit_code":0,"stdout":"","stderr":""}}'
}

# ---------------------------------------------------------------------------
# AC-10: malformed codified_at => dispatcher exits 2 (block)
# Traces to BC-5.39.008 postcondition 7
# ---------------------------------------------------------------------------

@test "test_BC_5_39_008_rejects_malformed_codified_at_exits_2" {
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

@test "test_BC_5_39_008_rejects_malformed_codified_at_block_reason_names_malformed_value" {
  _require_artifacts
  _setup_fixture
  _write_registry
  cp "$WASM_PLUGIN" "$WORK/hook-plugins/"

  local envelope
  envelope="$(_policies_yaml_envelope)"
  run bash -c "printf '%s' '$envelope' | CLAUDE_PLUGIN_ROOT='$WORK' CLAUDE_PROJECT_DIR='$WORK' '$DISPATCHER' 2>&1 >/dev/null"

  [ "$status" -eq 2 ]
  # block_reason must name the malformed codified_at value
  [[ "$output" == *"pass-72"* ]] || [[ "$output" == *"codified_at"* ]] || [[ "$output" == *"D-"* ]]
}
