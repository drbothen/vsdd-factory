#!/usr/bin/env bats
# pass-read-failure-failopen.bats — AC-14: when host::read_file returns HostError for policies.yaml
#                                    (file not found in path_allow scope), hook emits Continue;
#                                    never blocks (fail-open invariant)
#
# Traces to:
#   BC-5.39.008 postcondition 11 (host::read_file HostError => HookResult::Continue + log_warn)
#   BC-5.39.008 invariant 9(a) (fail-open: unreadable file => Continue, never block)
#   BC-5.39.008 EC-015 (HostError::Timeout or similar for policies.yaml)
#
# Test mechanism: restrict path_allow to a non-existent path so that host::read_file returns
# CapabilityDenied (a HostError variant) when attempting to read policies.yaml. The hook
# must fail-open to Continue rather than blocking.
#
# RED GATE PHASE: test skips because validate-policies-schema.wasm is not yet compiled.

setup() {
  REPO_ROOT="$(cd "${BATS_TEST_DIRNAME}/../../../.." && pwd)"
  PLUGIN_ROOT="$REPO_ROOT/plugins/vsdd-factory"
  DISPATCHER="$REPO_ROOT/target/release/factory-dispatcher"
  WASM_PLUGIN="$PLUGIN_ROOT/hook-plugins/validate-policies-schema.wasm"
  WORK="$(mktemp -d)"
  mkdir -p "$WORK/hook-plugins"
  mkdir -p "$WORK/.factory/logs"
}

teardown() {
  [ -n "${WORK:-}" ] && [ -d "$WORK" ] && find "$WORK" -type f -delete && find "$WORK" -type d -mindepth 1 | sort -r | xargs rmdir 2>/dev/null && rmdir "$WORK" 2>/dev/null || true
}

_write_registry_restricted_path_allow() {
  # path_allow does NOT include ".factory" — so read_file for ".factory/policies.yaml" returns CapabilityDenied
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
path_allow = ["some-nonexistent-path-that-does-not-exist"]
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
  printf '{"event_name":"PostToolUse","tool_name":"Edit","session_id":"pass-read-failure-failopen","tool_input":{"file_path":".factory/policies.yaml","content":""},"tool_response":{"exit_code":0,"stdout":"","stderr":""}}'
}

# ---------------------------------------------------------------------------
# AC-14: host::read_file HostError for policies.yaml => Continue (fail-open)
# Traces to BC-5.39.008 postcondition 11 + invariant 9(a)
# ---------------------------------------------------------------------------

@test "test_BC_5_39_008_invariant_failopen_read_error_policies_yaml_exits_0" {
  _require_artifacts
  _write_registry_restricted_path_allow
  cp "$WASM_PLUGIN" "$WORK/hook-plugins/"

  local envelope
  envelope="$(_policies_yaml_envelope)"
  run bash -c "printf '%s' '$envelope' | CLAUDE_PLUGIN_ROOT='$WORK' CLAUDE_PROJECT_DIR='$WORK' '$DISPATCHER' 2>&1 >/dev/null"

  [ "$status" -eq 0 ]
}

@test "test_BC_5_39_008_invariant_failopen_read_error_no_block" {
  _require_artifacts
  _write_registry_restricted_path_allow
  cp "$WASM_PLUGIN" "$WORK/hook-plugins/"

  local envelope
  envelope="$(_policies_yaml_envelope)"
  run bash -c "printf '%s' '$envelope' | CLAUDE_PLUGIN_ROOT='$WORK' CLAUDE_PROJECT_DIR='$WORK' '$DISPATCHER' 2>&1 >/dev/null"

  [ "$status" -eq 0 ]
  [[ "$output" != *"blocking_plugins=validate-policies-schema"* ]]
}
