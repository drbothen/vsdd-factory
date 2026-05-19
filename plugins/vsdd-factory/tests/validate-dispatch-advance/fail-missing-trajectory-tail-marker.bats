#!/usr/bin/env bats
# fail-missing-trajectory-tail-marker.bats — AC-22: hook blocks when trajectory-tail prefix absent
#
# Traces to:
#   BC-5.39.006 v1.2 postcondition 6; D-451(c)/F-P3-006/EC-023
#
# Fixture: STATE.md with current_step containing 4 arrow-N groups but NO
#          `trajectory-tail ` prefix. All other conditions valid: 4-index cites
#          present, valid D-chain, no forbidden meta-commentary.
# Expected: hook exits 2; block message names the missing canonical marker,
#           cites D-451(c)/F-P3-006/EC-023.
#
# Negative case: STATE.md with current_step containing `trajectory-tail →9→9→9→9`
#                — hook does NOT block on prefix (only LENGTH check would fire
#                separately if count != 4).
#
# RED GATE PHASE: test skips if validate-dispatch-advance.wasm is not yet compiled.

setup() {
  REPO_ROOT="$(cd "${BATS_TEST_DIRNAME}/../../../.." && pwd)"
  PLUGIN_ROOT="$REPO_ROOT/plugins/vsdd-factory"
  DISPATCHER="$REPO_ROOT/target/release/factory-dispatcher"
  WASM_PLUGIN="$PLUGIN_ROOT/hook-plugins/validate-dispatch-advance.wasm"
  FIXTURE_SRC="${BATS_TEST_DIRNAME}/../fixtures/validate-dispatch-advance/fail-missing-trajectory-tail-marker"
  PASS_FIXTURE_SRC="${BATS_TEST_DIRNAME}/../fixtures/validate-dispatch-advance/pass-all-valid-state"
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

_setup_pass_fixture() {
  cp -r "$PASS_FIXTURE_SRC/factory/." "$WORK/.factory/"
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

_state_md_envelope() {
  printf '{"event_name":"PostToolUse","tool_name":"Edit","session_id":"fail-missing-trajectory-tail-marker","tool_input":{"file_path":".factory/STATE.md","content":""},"tool_response":{"exit_code":0,"stdout":"","stderr":""}}'
}

# ---------------------------------------------------------------------------
# AC-22: missing `trajectory-tail ` prefix => hook blocks (exit 2)
# Traces to BC-5.39.006 v1.2 postcondition 6; D-451(c)/F-P3-006/EC-023
# ---------------------------------------------------------------------------

@test "AC-22 FAIL: hook blocks when trajectory-tail prefix absent from current_step" {
  _require_artifacts
  _setup_fixture
  _write_registry
  cp "$WASM_PLUGIN" "$WORK/hook-plugins/"

  local envelope
  envelope="$(_state_md_envelope)"
  run bash -c "printf '%s' '$envelope' | CLAUDE_PLUGIN_ROOT='$WORK' CLAUDE_PROJECT_DIR='$WORK' '$DISPATCHER' 2>&1 >/dev/null"

  # Exit 2: block signal emitted for missing canonical marker
  [ "$status" -eq 2 ]

  # blocking_plugins= names this hook
  [[ "$output" == *"blocking_plugins=validate-dispatch-advance"* ]]
}

@test "AC-22 FAIL: block message names missing trajectory-tail canonical marker and cites F-P3-006/EC-023" {
  _require_artifacts
  _setup_fixture
  _write_registry
  cp "$WASM_PLUGIN" "$WORK/hook-plugins/"

  local envelope
  envelope="$(_state_md_envelope)"
  run bash -c "printf '%s' '$envelope' | CLAUDE_PLUGIN_ROOT='$WORK' CLAUDE_PROJECT_DIR='$WORK' '$DISPATCHER' 2>&1 >/dev/null"

  [ "$status" -eq 2 ]
  # Block message must name the canonical marker
  [[ "$output" == *"trajectory-tail"* ]]
  # Must cite F-P3-006
  [[ "$output" == *"F-P3-006"* ]]
  # Must cite EC-023
  [[ "$output" == *"EC-023"* ]]
}

# ---------------------------------------------------------------------------
# Negative case: prefix present with LENGTH=4 => hook does NOT block on prefix
# (pass-all-valid-state fixture has trajectory-tail →9→9→9→9 — prefix present)
# ---------------------------------------------------------------------------

@test "AC-22 PASS: hook does not block when trajectory-tail prefix present with LENGTH=4" {
  _require_artifacts
  _setup_pass_fixture
  _write_registry
  cp "$WASM_PLUGIN" "$WORK/hook-plugins/"

  local envelope
  envelope="$(_state_md_envelope)"
  run bash -c "printf '%s' '$envelope' | CLAUDE_PLUGIN_ROOT='$WORK' CLAUDE_PROJECT_DIR='$WORK' '$DISPATCHER' 2>&1 >/dev/null"

  # Exit 0: no block when prefix present and LENGTH=4
  [ "$status" -eq 0 ]
}
