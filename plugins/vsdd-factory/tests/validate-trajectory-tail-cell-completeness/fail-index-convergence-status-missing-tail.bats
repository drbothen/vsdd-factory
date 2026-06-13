#!/usr/bin/env bats
# fail-index-convergence-status-missing-tail.bats — AC-9: INDEX.md with Convergence Status row
#   missing arrow-sequence => hook emits log_warn advisory (NOT Block) + Continue
#
# Traces to:
#   BC-5.39.009 postcondition 7 (PC7: INDEX.md Convergence Status row missing tail => advisory)
#   BC-5.39.009 invariant 6 (non-STATE.md sites: advisory severity only)
#
# Fixture: INDEX.md with Convergence Status row missing trajectory-tail marker.
#   Also requires STATE.md with current_cycle: v1.0-brownfield-backfill so dynamic
#   cycle-path guard resolves this INDEX.md as the active cycle.
# Expected: hook exits 0 (Continue, NOT Block); stderr contains advisory log_warn message.
#
# RED GATE PHASE: test skips because validate-trajectory-tail-cell-completeness.wasm not yet compiled.

setup() {
  REPO_ROOT="$(cd "${BATS_TEST_DIRNAME}/../../../.." && pwd)"
  PLUGIN_ROOT="$REPO_ROOT/plugins/vsdd-factory"
  DISPATCHER="$REPO_ROOT/target/release/factory-dispatcher"
  WASM_PLUGIN="$PLUGIN_ROOT/hook-plugins/validate-trajectory-tail-cell-completeness.wasm"
  FIXTURE_SRC="${BATS_TEST_DIRNAME}/../fixtures/validate-trajectory-tail-cell-completeness/fail-index-convergence-status-missing-tail"
  WORK="$(mktemp -d)"
  mkdir -p "$WORK/hook-plugins"
  mkdir -p "$WORK/.factory/logs"
  export VSDD_LOG_DIR="$WORK/.factory/logs"
}

teardown() {
  [ -n "${WORK:-}" ] && [ -d "$WORK" ] && find "$WORK" -type f -delete && find "$WORK" -type d -mindepth 1 | sort -r | xargs rmdir 2>/dev/null && rmdir "$WORK" 2>/dev/null || true
}

_setup_fixture() {
  mkdir -p "$WORK/.factory/cycles/v1.0-brownfield-backfill"
  cp "$FIXTURE_SRC/INDEX.md" "$WORK/.factory/cycles/v1.0-brownfield-backfill/INDEX.md"
  # Also place STATE.md so cycle-guard secondary read succeeds
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

_index_md_envelope() {
  printf '{"event_name":"PostToolUse","tool_name":"Edit","session_id":"fail-index-convergence-status-missing-tail","tool_input":{"file_path":".factory/cycles/v1.0-brownfield-backfill/INDEX.md","content":""},"tool_response":{"exit_code":0,"stdout":"","stderr":""}}'
}

# ---------------------------------------------------------------------------
# AC-9 / PC7 + inv-6: Convergence Status row missing tail => advisory Continue
# ---------------------------------------------------------------------------

@test "test_BC_5_39_009_PC7_convergence_status_missing_tail_exits_0" {
  _require_artifacts
  _setup_fixture
  _write_registry
  cp "$WASM_PLUGIN" "$WORK/hook-plugins/"

  local envelope
  envelope="$(_index_md_envelope)"
  run bash -c "printf '%s' '$envelope' | CLAUDE_PLUGIN_ROOT='$WORK' CLAUDE_PROJECT_DIR='$WORK' '$DISPATCHER' 2>&1 >/dev/null"

  # Must exit 0 (Continue, NOT Block) — advisory severity per inv-6
  [ "$status" -eq 0 ]
}

@test "test_BC_5_39_009_PC7_convergence_status_missing_tail_no_block" {
  _require_artifacts
  _setup_fixture
  _write_registry
  cp "$WASM_PLUGIN" "$WORK/hook-plugins/"

  local envelope
  envelope="$(_index_md_envelope)"
  run bash -c "printf '%s' '$envelope' | CLAUDE_PLUGIN_ROOT='$WORK' CLAUDE_PROJECT_DIR='$WORK' '$DISPATCHER' 2>&1 >/dev/null"

  [ "$status" -eq 0 ]
  [[ "$output" != *"blocking_plugins=validate-trajectory-tail-cell-completeness"* ]]
}

# F-002: make the PC7 advisory arm LOAD-BEARING — assert the specific Convergence Status
# advisory string is emitted (plugin.log warn in the internal log), and that the
# adv-table advisory (which HAS a tail in this fixture) is ABSENT.
@test "test_BC_5_39_009_PC7_convergence_status_missing_tail_emits_advisory" {
  _require_artifacts
  _setup_fixture
  _write_registry
  cp "$WASM_PLUGIN" "$WORK/hook-plugins/"

  local envelope
  envelope="$(_index_md_envelope)"
  run bash -c "printf '%s' '$envelope' | FACTORY_DISPATCHER_INTERNAL_LOG=1 CLAUDE_PLUGIN_ROOT='$WORK' CLAUDE_PROJECT_DIR='$WORK' '$DISPATCHER' >/dev/null 2>&1"

  [ "$status" -eq 0 ]
  local logf
  logf="$(ls "$WORK/.factory/logs/"dispatcher-internal-*.jsonl 2>/dev/null | head -1)"
  [ -n "$logf" ]
  grep -q "INDEX.md Convergence Status row missing" "$logf"
  # The adv-table latest row HAS a tail in this fixture → its advisory must be ABSENT.
  ! grep -q "INDEX.md adv-table latest row missing" "$logf"
}
