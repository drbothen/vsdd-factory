#!/usr/bin/env bats
# pass-registry-read-failure.bats — AC-15: hooks-registry.toml read failure skips lint_hook-existence
#                                    check; log_warn emitted; other checks continue; no block
#
# Traces to:
#   BC-5.39.008 invariant 9(b) (registry read failure => skip lint_hook-existence only; other checks run)
#   BC-5.39.008 EC-016 (hooks-registry.toml read fails)
#
# Test mechanism: policies.yaml has a valid structure with a non-null lint_hook value, but
# the registry is NOT in the path_allow scope, so the hook gets CapabilityDenied when
# reading hooks-registry.toml. The lint_hook-existence check must be skipped; other checks
# (id format, required fields, header) continue and pass.
#
# The fixture has a valid policies.yaml with lint_hook: "validate-dispatch-advance" and
# a correct D-NNN codified_at. The path_allow only covers ".factory" (not "plugins/vsdd-factory"),
# so the registry read fails. The hook should still pass because other checks are clean.
#
# RED GATE PHASE: test skips because validate-policies-schema.wasm is not yet compiled.

setup() {
  REPO_ROOT="$(cd "${BATS_TEST_DIRNAME}/../../../.." && pwd)"
  PLUGIN_ROOT="$REPO_ROOT/plugins/vsdd-factory"
  DISPATCHER="$REPO_ROOT/target/release/factory-dispatcher"
  WASM_PLUGIN="$PLUGIN_ROOT/hook-plugins/validate-policies-schema.wasm"
  FIXTURE_SRC="${BATS_TEST_DIRNAME}/../fixtures/validate-policies-schema/pass-registry-read-failure"
  WORK="$(mktemp -d)"
  mkdir -p "$WORK/hook-plugins"
  mkdir -p "$WORK/.factory/logs"
}

teardown() {
  [ -n "${WORK:-}" ] && [ -d "$WORK" ] && find "$WORK" -type f -delete && find "$WORK" -type d -mindepth 1 | sort -r | xargs rmdir 2>/dev/null && rmdir "$WORK" 2>/dev/null || true
}

_setup_fixture() {
  cp -r "$FIXTURE_SRC/factory/." "$WORK/.factory/"
  # Deliberately do NOT copy any hooks-registry.toml to plugins/vsdd-factory/
  # This simulates the registry being inaccessible to the hook
}

_write_registry_no_plugins_path() {
  # path_allow covers .factory but NOT plugins/vsdd-factory —
  # so hooks-registry.toml read at "plugins/vsdd-factory/hooks-registry.toml" returns CapabilityDenied
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
path_allow = [".factory"]
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
  printf '{"event_name":"PostToolUse","tool_name":"Edit","session_id":"pass-registry-read-failure","tool_input":{"file_path":".factory/policies.yaml","content":""},"tool_response":{"exit_code":0,"stdout":"","stderr":""}}'
}

# ---------------------------------------------------------------------------
# AC-15: registry read failure => skip lint_hook-existence check; other checks pass => Continue
# Traces to BC-5.39.008 invariant 9(b)
# ---------------------------------------------------------------------------

@test "test_BC_5_39_008_invariant_registry_read_failure_skips_lint_hook_check_exits_0" {
  _require_artifacts
  _setup_fixture
  _write_registry_no_plugins_path
  cp "$WASM_PLUGIN" "$WORK/hook-plugins/"

  local envelope
  envelope="$(_policies_yaml_envelope)"
  run bash -c "printf '%s' '$envelope' | CLAUDE_PLUGIN_ROOT='$WORK' CLAUDE_PROJECT_DIR='$WORK' '$DISPATCHER' 2>&1 >/dev/null"

  [ "$status" -eq 0 ]
}

@test "test_BC_5_39_008_invariant_registry_read_failure_no_block_from_missing_registry" {
  _require_artifacts
  _setup_fixture
  _write_registry_no_plugins_path
  cp "$WASM_PLUGIN" "$WORK/hook-plugins/"

  local envelope
  envelope="$(_policies_yaml_envelope)"
  run bash -c "printf '%s' '$envelope' | CLAUDE_PLUGIN_ROOT='$WORK' CLAUDE_PROJECT_DIR='$WORK' '$DISPATCHER' 2>&1 >/dev/null"

  [ "$status" -eq 0 ]
  [[ "$output" != *"blocking_plugins=validate-policies-schema"* ]]
}
