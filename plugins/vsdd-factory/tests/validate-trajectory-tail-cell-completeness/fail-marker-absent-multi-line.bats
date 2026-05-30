#!/usr/bin/env bats
# fail-marker-absent-multi-line.bats — AC-21 (F-SP8-005): STATE.md with multi-line YAML
#   block-scalar current_step: where trajectory-tail marker IS ABSENT => Block
#   Even though →N→N→N→N appears in the value, WITHOUT the marker prefix, Step 1 fails.
#
# Traces to:
#   BC-5.39.009 EC-017 + invariant 4 (two-step marker-prefix check: Step 1 fails => Block)
#   BC-5.39.009 PC1 (current_step missing trajectory_tail marker => Block)
#
# Fixture: STATE.md with multi-line current_step: containing prose with arrow-sequences
#   BUT the literal "trajectory-tail " marker is ABSENT from the entire value.
#   →9→9→9→9 appears in the value (from raw prose trend), but without the marker.
#   Step 1 of inv-4 two-step check: marker absent => site missing => Block.
#
# Expected: hook exits 2 (Block)
#
# RED GATE PHASE: test skips because validate-trajectory-tail-cell-completeness.wasm not yet compiled.

setup() {
  REPO_ROOT="$(cd "${BATS_TEST_DIRNAME}/../../../.." && pwd)"
  PLUGIN_ROOT="$REPO_ROOT/plugins/vsdd-factory"
  DISPATCHER="$REPO_ROOT/target/release/factory-dispatcher"
  WASM_PLUGIN="$PLUGIN_ROOT/hook-plugins/validate-trajectory-tail-cell-completeness.wasm"
  FIXTURE_SRC="${BATS_TEST_DIRNAME}/../fixtures/validate-trajectory-tail-cell-completeness/fail-marker-absent-multi-line"
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

_state_md_envelope() {
  printf '{"event_name":"PostToolUse","tool_name":"Edit","session_id":"fail-marker-absent-multi-line","tool_input":{"file_path":".factory/STATE.md","content":""},"tool_response":{"exit_code":0,"stdout":"","stderr":""}}'
}

# ---------------------------------------------------------------------------
# AC-21 (F-SP8-005) / EC-017 + inv-4: marker ABSENT in multi-line value => Block
# ---------------------------------------------------------------------------

@test "test_BC_5_39_009_EC017_marker_absent_multi_line_exits_2" {
  _require_artifacts
  _setup_fixture
  _write_registry
  cp "$WASM_PLUGIN" "$WORK/hook-plugins/"

  local envelope
  envelope="$(_state_md_envelope)"
  run bash -c "printf '%s' '$envelope' | CLAUDE_PLUGIN_ROOT='$WORK' CLAUDE_PROJECT_DIR='$WORK' '$DISPATCHER' 2>&1 >/dev/null"

  [ "$status" -eq 2 ]
  [[ "$output" == *"blocking_plugins=validate-trajectory-tail-cell-completeness"* ]]
}
