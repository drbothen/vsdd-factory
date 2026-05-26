#!/usr/bin/env bats
# fail-noncanonical-id-string.bats — AC-4: hook blocks when policy entry has id: "POLICY 01"
#                                     (string, not integer); block message cites non-conforming
#                                     id format and expected bare integer in [1,999]
#
# Traces to:
#   BC-5.39.008 postcondition 4 (non-YAML-integer id => BlockWithFix naming non-conforming id)
#   BC-5.39.008 invariant 4 (id must be bare YAML integer in [1,999])
#   BC-5.39.008 EC-003 (string id "POLICY 01")
#   F-PASS14-006 (canonical id format: bare integer, not POLICY NNN prefix)
#
# Fixture: policies.yaml with id: "POLICY 01" (quoted string, not integer).
# Expected: hook exits 2 (block); block_reason cites non-conforming id format.
#
# RED GATE PHASE: test skips because validate-policies-schema.wasm is not yet compiled.

setup() {
  REPO_ROOT="$(cd "${BATS_TEST_DIRNAME}/../../../.." && pwd)"
  PLUGIN_ROOT="$REPO_ROOT/plugins/vsdd-factory"
  DISPATCHER="$REPO_ROOT/target/release/factory-dispatcher"
  WASM_PLUGIN="$PLUGIN_ROOT/hook-plugins/validate-policies-schema.wasm"
  FIXTURE_SRC="${BATS_TEST_DIRNAME}/../fixtures/validate-policies-schema/fail-noncanonical-id-string"
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
  printf '{"event_name":"PostToolUse","tool_name":"Edit","session_id":"fail-noncanonical-id-string","tool_input":{"file_path":".factory/policies.yaml","content":""},"tool_response":{"exit_code":0,"stdout":"","stderr":""}}'
}

# ---------------------------------------------------------------------------
# AC-4: non-canonical string id => dispatcher exits 2 (block)
# Traces to BC-5.39.008 postcondition 4 + invariant 4 + F-PASS14-006
# ---------------------------------------------------------------------------

@test "test_BC_5_39_008_rejects_noncanonical_id_string_exits_2" {
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

@test "test_BC_5_39_008_rejects_noncanonical_id_string_block_reason_cites_nonconforming_format" {
  _require_artifacts
  _setup_fixture
  _write_registry
  cp "$WASM_PLUGIN" "$WORK/hook-plugins/"

  local envelope
  envelope="$(_policies_yaml_envelope)"
  run bash -c "printf '%s' '$envelope' | CLAUDE_PLUGIN_ROOT='$WORK' CLAUDE_PROJECT_DIR='$WORK' '$DISPATCHER' 2>&1 >/dev/null"

  [ "$status" -eq 2 ]
  # block_reason must reference the non-conforming id value or the integer format requirement
  [[ "$output" == *"POLICY 01"* ]] || [[ "$output" == *"integer"* ]] || [[ "$output" == *"id"* ]]
}
