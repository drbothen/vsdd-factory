#!/usr/bin/env bats
# fail-p2-pointer-zero.bats — AC-1b: when .factory/current-adversary-pass.txt
#                              is present but its content is "0" (not a valid
#                              positive integer), hook emits a hard block with
#                              parse-error message directing developer to run
#                              state-manager update-pass-pointer
#
# Traces to:
#   BC-5.39.007 Phase 2 P2-1 postcondition (parse-error path is a hard block, NOT fail-open)
#   ADR-022 parse-error handling ("If the file exists but content is not a valid positive
#           integer, the hook emits a hard block with a clear error message")
#
# Fixture: fail-p2-pointer-zero/ — STATE.md present;
#          .factory/current-adversary-pass.txt contains "0" (zero is not a positive integer)
# Expected: hook exits 2 (block); block_reason contains "not a valid positive integer"
#
# RED GATE PHASE: exercising the zero-rejection branch of read_current_adversary_pass_number.

setup() {
  REPO_ROOT="$(cd "${BATS_TEST_DIRNAME}/../../../.." && pwd)"
  PLUGIN_ROOT="$REPO_ROOT/plugins/vsdd-factory"
  DISPATCHER="$REPO_ROOT/target/release/factory-dispatcher"
  WASM_PLUGIN="$PLUGIN_ROOT/hook-plugins/validate-closes-completeness.wasm"
  FIXTURE_SRC="${BATS_TEST_DIRNAME}/../fixtures/validate-closes-completeness/fail-p2-pointer-zero"
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
name = "validate-closes-completeness"
event = "PostToolUse"
tool = "Edit|Write"
plugin = "hook-plugins/validate-closes-completeness.wasm"
priority = 156
timeout_ms = 5000
on_error = "continue"

[hooks.capabilities.read_file]
path_allow = [
  ".factory/cycles",
  ".factory/STATE.md",
  ".factory/current-adversary-pass.txt",
  ".factory/cycles/v1.0-brownfield-backfill/INDEX.md",
  ".factory/cycles/v1.0-brownfield-backfill/lessons.md",
]
TOML
}

_require_artifacts() {
  if [ ! -x "$DISPATCHER" ]; then
    skip "dispatcher binary not built -- run: cargo build --release -p factory-dispatcher"
  fi
  if [ ! -f "$WASM_PLUGIN" ]; then
    skip "validate-closes-completeness.wasm not built -- implement T-4 through T-7 of S-15.13"
  fi
}

_state_md_envelope() {
  printf '{"event_name":"PostToolUse","tool_name":"Edit","session_id":"fail-p2-pointer-zero","tool_input":{"file_path":".factory/STATE.md","content":""},"tool_response":{"exit_code":0,"stdout":"","stderr":""}}'
}

# ---------------------------------------------------------------------------
# AC-1b: pointer file containing "0" => exits 2 (hard block, NOT fail-open)
# Traces to BC-5.39.007 Phase 2 P2-1 parse-error path + ADR-022 parse-error handling
# CRITICAL: zero is not a valid positive integer; this must be a hard block
# ---------------------------------------------------------------------------

@test "AC-1b FAIL: hook emits block (exits 2) when current-adversary-pass.txt contains zero" {
  _require_artifacts
  _setup_fixture
  _write_registry
  cp "$WASM_PLUGIN" "$WORK/hook-plugins/"

  # Verify the fixture has the pointer file containing "0".
  [ -f "$WORK/.factory/current-adversary-pass.txt" ]
  [[ "$(cat "$WORK/.factory/current-adversary-pass.txt")" == "0" ]]

  local envelope
  envelope="$(_state_md_envelope)"
  run bash -c "printf '%s' '$envelope' | CLAUDE_PLUGIN_ROOT='$WORK' CLAUDE_PROJECT_DIR='$WORK' '$DISPATCHER' 2>&1 >/dev/null"

  # Exit 2: hard block on zero pointer value
  [ "$status" -eq 2 ]

  # blocking_plugins= names this hook
  [[ "$output" == *"blocking_plugins=validate-closes-completeness"* ]]
}

# ---------------------------------------------------------------------------
# AC-1b: block message contains "not a valid positive integer"
# Traces to ADR-022 parse-error handling — message must direct to update-pass-pointer
# ---------------------------------------------------------------------------

@test "AC-1b FAIL: block message contains 'not a valid positive integer' for zero pointer" {
  _require_artifacts
  _setup_fixture
  _write_registry
  cp "$WASM_PLUGIN" "$WORK/hook-plugins/"

  local envelope
  envelope="$(_state_md_envelope)"
  run bash -c "printf '%s' '$envelope' | CLAUDE_PLUGIN_ROOT='$WORK' CLAUDE_PROJECT_DIR='$WORK' '$DISPATCHER' 2>&1 >/dev/null"

  [ "$status" -eq 2 ]
  # Block message must mention the parse-error constraint
  [[ "$output" == *"not a valid positive integer"* ]]
}
