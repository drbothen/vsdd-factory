#!/usr/bin/env bats
# fail-state-session-resume-missing-tail.bats — AC-6: STATE.md with Session Resume Section 1
#   missing arrow-sequence => hook emits Block naming "STATE.md Session Resume Section 1"
#
# v1.9 ALIGNMENT (NOT weakening): under BC-5.39.009 v1.9 (ADR-023 Option (c)) PC5 is
#   CYCLE-CONDITIONAL — it Blocks ONLY in an F5-style per-pass cycle (per_pass_trajectory:
#   true). The fixture's current_cycle: was changed to v1.0-feature-engine-discipline-pass-1
#   and a cycle INDEX.md carrying per_pass_trajectory: true was added, so this test still
#   correctly asserts Block — now via the v1.9 F5-per-pass arm. (The milestone arm is
#   covered by pass-milestone-cycle-no-block.bats.)
#
# Traces to:
#   BC-5.39.009 v1.9 postcondition 5 (PC5: cycle-conditional; F5-per-pass → Block)
#   BC-5.39.009 v1.9 Precondition 7 (per_pass_trajectory: true → flag TRUE → Block)
#   BC-5.39.009 v1.9 invariant 14 (cycle-conditional severity)
#
# Fixture: STATE.md (current_cycle: v1.0-feature-engine-discipline-pass-1) where Session
#   Resume §1 body has no trajectory-tail marker + cycle INDEX.md per_pass_trajectory: true.
# Expected: hook exits 2 (Block); block_reason names "STATE.md Session Resume Section 1".
#
# RED GATE PHASE: test skips because validate-trajectory-tail-cell-completeness.wasm not yet compiled.

setup() {
  REPO_ROOT="$(cd "${BATS_TEST_DIRNAME}/../../../.." && pwd)"
  PLUGIN_ROOT="$REPO_ROOT/plugins/vsdd-factory"
  DISPATCHER="$REPO_ROOT/target/release/factory-dispatcher"
  WASM_PLUGIN="$PLUGIN_ROOT/hook-plugins/validate-trajectory-tail-cell-completeness.wasm"
  FIXTURE_SRC="${BATS_TEST_DIRNAME}/../fixtures/validate-trajectory-tail-cell-completeness/fail-state-session-resume-missing-tail"
  WORK="$(mktemp -d)"
  mkdir -p "$WORK/hook-plugins"
  mkdir -p "$WORK/.factory/logs"
}

teardown() {
  [ -n "${WORK:-}" ] && [ -d "$WORK" ] && find "$WORK" -type f -delete && find "$WORK" -type d -mindepth 1 | sort -r | xargs rmdir 2>/dev/null && rmdir "$WORK" 2>/dev/null || true
}

_setup_fixture() {
  # v1.9: wire STATE.md (write target) + the F5 cycle INDEX.md carrying
  # per_pass_trajectory: true so PC5 routes to the Block arm (cycle-conditional).
  mkdir -p "$WORK/.factory/cycles/v1.0-feature-engine-discipline-pass-1"
  cp "$FIXTURE_SRC/STATE.md" "$WORK/.factory/STATE.md"
  cp "$FIXTURE_SRC/INDEX.md" "$WORK/.factory/cycles/v1.0-feature-engine-discipline-pass-1/INDEX.md"
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
  printf '{"event_name":"PostToolUse","tool_name":"Edit","session_id":"fail-state-session-resume-missing-tail","tool_input":{"file_path":".factory/STATE.md","content":""},"tool_response":{"exit_code":0,"stdout":"","stderr":""}}'
}

# ---------------------------------------------------------------------------
# AC-6 / PC5: Session Resume Section 1 missing trajectory_tail => Block exit 2
# ---------------------------------------------------------------------------

@test "test_BC_5_39_009_PC5_session_resume_missing_tail_exits_2" {
  _require_artifacts
  _setup_fixture
  _write_registry
  cp "$WASM_PLUGIN" "$WORK/hook-plugins/"

  local envelope
  envelope="$(_state_md_envelope)"
  run bash -c "printf '%s' '$envelope' | CLAUDE_PLUGIN_ROOT='$WORK' CLAUDE_PROJECT_DIR='$WORK' '$DISPATCHER' 2>&1 >/dev/null"

  [ "$status" -eq 2 ]
}

@test "test_BC_5_39_009_PC5_session_resume_missing_tail_block_names_site" {
  _require_artifacts
  _setup_fixture
  _write_registry
  cp "$WASM_PLUGIN" "$WORK/hook-plugins/"

  local envelope
  envelope="$(_state_md_envelope)"
  run bash -c "printf '%s' '$envelope' | CLAUDE_PLUGIN_ROOT='$WORK' CLAUDE_PROJECT_DIR='$WORK' '$DISPATCHER' 2>&1 >/dev/null"

  [ "$status" -eq 2 ]
  [[ "$output" == *"blocking_plugins=validate-trajectory-tail-cell-completeness"* ]]
  [[ "$output" == *"Session Resume"* ]]
}
