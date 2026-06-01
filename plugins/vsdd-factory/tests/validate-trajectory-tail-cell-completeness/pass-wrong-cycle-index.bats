#!/usr/bin/env bats
# pass-wrong-cycle-index.bats — EC-008: INDEX.md from a non-active cycle path => Continue
#   (cycle-path guard dynamically resolves current_cycle from STATE.md; wrong-cycle INDEX.md
#   is rejected by the path-component-walk guard)
#
# Traces to:
#   BC-5.39.009 EC-008 (INDEX.md from different cycle path => Continue; Precondition 4 guard)
#   BC-5.39.009 Precondition 4 (INDEX.md arm fires ONLY for active-cycle path)
#
# Fixture: INDEX.md located at a v2.0-future-cycle path; STATE.md says current_cycle is
#   v1.0-brownfield-backfill. The path-component-walk check finds no match => Continue.
# Expected: hook exits 0 (Continue); no block signal
#
# RED GATE PHASE: test skips because validate-trajectory-tail-cell-completeness.wasm not yet compiled.

setup() {
  REPO_ROOT="$(cd "${BATS_TEST_DIRNAME}/../../../.." && pwd)"
  PLUGIN_ROOT="$REPO_ROOT/plugins/vsdd-factory"
  DISPATCHER="$REPO_ROOT/target/release/factory-dispatcher"
  WASM_PLUGIN="$PLUGIN_ROOT/hook-plugins/validate-trajectory-tail-cell-completeness.wasm"
  FIXTURE_SRC="${BATS_TEST_DIRNAME}/../fixtures/validate-trajectory-tail-cell-completeness/pass-wrong-cycle-index"
  WORK="$(mktemp -d)"
  mkdir -p "$WORK/hook-plugins"
  mkdir -p "$WORK/.factory/logs"
}

teardown() {
  [ -n "${WORK:-}" ] && [ -d "$WORK" ] && find "$WORK" -type f -delete && find "$WORK" -type d -mindepth 1 | sort -r | xargs rmdir 2>/dev/null && rmdir "$WORK" 2>/dev/null || true
}

_setup_fixture() {
  # Place INDEX.md under a v2.0-future-cycle path (NOT v1.0-brownfield-backfill)
  mkdir -p "$WORK/.factory/cycles/v2.0-future-cycle"
  cp "$FIXTURE_SRC/INDEX.md" "$WORK/.factory/cycles/v2.0-future-cycle/INDEX.md"
  # STATE.md says current_cycle: v1.0-brownfield-backfill
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

_wrong_cycle_index_envelope() {
  # file_path points to v2.0-future-cycle/INDEX.md — active cycle is v1.0-brownfield-backfill
  printf '{"event_name":"PostToolUse","tool_name":"Edit","session_id":"pass-wrong-cycle-index","tool_input":{"file_path":".factory/cycles/v2.0-future-cycle/INDEX.md","content":""},"tool_response":{"exit_code":0,"stdout":"","stderr":""}}'
}

# ---------------------------------------------------------------------------
# EC-008 / Precondition 4: wrong-cycle INDEX.md => dynamic guard => Continue exit 0
# ---------------------------------------------------------------------------

@test "test_BC_5_39_009_EC008_wrong_cycle_index_exits_0" {
  _require_artifacts
  _setup_fixture
  _write_registry
  cp "$WASM_PLUGIN" "$WORK/hook-plugins/"

  local envelope
  envelope="$(_wrong_cycle_index_envelope)"
  run bash -c "printf '%s' '$envelope' | CLAUDE_PLUGIN_ROOT='$WORK' CLAUDE_PROJECT_DIR='$WORK' '$DISPATCHER' 2>&1 >/dev/null"

  [ "$status" -eq 0 ]
}

@test "test_BC_5_39_009_EC008_wrong_cycle_index_no_block" {
  _require_artifacts
  _setup_fixture
  _write_registry
  cp "$WASM_PLUGIN" "$WORK/hook-plugins/"

  local envelope
  envelope="$(_wrong_cycle_index_envelope)"
  run bash -c "printf '%s' '$envelope' | CLAUDE_PLUGIN_ROOT='$WORK' CLAUDE_PROJECT_DIR='$WORK' '$DISPATCHER' 2>&1 >/dev/null"

  [ "$status" -eq 0 ]
  [[ "$output" != *"blocking_plugins=validate-trajectory-tail-cell-completeness"* ]]
}
