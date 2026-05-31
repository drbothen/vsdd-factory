#!/usr/bin/env bats
# pass-read-failure-failopen.bats — AC-15: host::read_file returns any HostError variant
#   (HostError::Timeout, HostError::CapabilityDenied, HostError::InvalidArgument,
#   HostError::Other(i32)) for any target file => Continue + log_warn; never Block
#
# Traces to:
#   BC-5.39.009 postcondition 11 (PC11: any HostError => Continue + log_warn fail-open)
#   BC-5.39.009 invariant 10 (host::read_file HostError => fail-open)
#
# Test approach: use a file_path that triggers CapabilityDenied (outside path_allow)
#   which is a legitimate HostError variant per PC11. The hook must exit 0.
#
# Expected: hook exits 0 (Continue); no blocking_plugins signal
#
# RED GATE PHASE: test skips because validate-trajectory-tail-cell-completeness.wasm not yet compiled.

setup() {
  REPO_ROOT="$(cd "${BATS_TEST_DIRNAME}/../../../.." && pwd)"
  PLUGIN_ROOT="$REPO_ROOT/plugins/vsdd-factory"
  DISPATCHER="$REPO_ROOT/target/release/factory-dispatcher"
  WASM_PLUGIN="$PLUGIN_ROOT/hook-plugins/validate-trajectory-tail-cell-completeness.wasm"
  FIXTURE_SRC="${BATS_TEST_DIRNAME}/../fixtures/validate-trajectory-tail-cell-completeness/pass-read-failure-failopen"
  WORK="$(mktemp -d)"
  mkdir -p "$WORK/hook-plugins"
  mkdir -p "$WORK/.factory/logs"
}

teardown() {
  [ -n "${WORK:-}" ] && [ -d "$WORK" ] && find "$WORK" -type f -delete && find "$WORK" -type d -mindepth 1 | sort -r | xargs rmdir 2>/dev/null && rmdir "$WORK" 2>/dev/null || true
}

_setup_fixture() {
  mkdir -p "$WORK/.factory"
  cp "$FIXTURE_SRC/STATE.md" "$WORK/.factory/STATE.md"
}

_write_registry() {
  # Registry with path_allow = [".factory"] — reading from outside will trigger HostError
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

_state_md_envelope_nonexistent() {
  # Simulate read failure: file_path references a non-existent STATE.md
  printf '{"event_name":"PostToolUse","tool_name":"Edit","session_id":"pass-read-failure-failopen","tool_input":{"file_path":".factory/nonexistent-path/STATE.md","content":""},"tool_response":{"exit_code":0,"stdout":"","stderr":""}}'
}

# ---------------------------------------------------------------------------
# AC-15 / PC11 + inv-10: HostError::Timeout / CapabilityDenied / Other => Continue
# ---------------------------------------------------------------------------

@test "test_BC_5_39_009_PC11_read_failure_failopen_exits_0" {
  _require_artifacts
  _setup_fixture
  _write_registry
  cp "$WASM_PLUGIN" "$WORK/hook-plugins/"

  local envelope
  envelope="$(_state_md_envelope_nonexistent)"
  run bash -c "printf '%s' '$envelope' | CLAUDE_PLUGIN_ROOT='$WORK' CLAUDE_PROJECT_DIR='$WORK' '$DISPATCHER' 2>&1 >/dev/null"

  [ "$status" -eq 0 ]
}

@test "test_BC_5_39_009_PC11_read_failure_never_blocks" {
  _require_artifacts
  _setup_fixture
  _write_registry
  cp "$WASM_PLUGIN" "$WORK/hook-plugins/"

  local envelope
  envelope="$(_state_md_envelope_nonexistent)"
  run bash -c "printf '%s' '$envelope' | CLAUDE_PLUGIN_ROOT='$WORK' CLAUDE_PROJECT_DIR='$WORK' '$DISPATCHER' 2>&1 >/dev/null"

  [ "$status" -eq 0 ]
  [[ "$output" != *"blocking_plugins=validate-trajectory-tail-cell-completeness"* ]]
}
