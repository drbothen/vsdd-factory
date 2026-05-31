#!/usr/bin/env bats
# fail-state-f5-per-pass-block.bats — AC-4/AC-5/AC-6: F5-style per-pass cycle (active
#   cycle's INDEX.md has per_pass_trajectory: true) with PC3 (Phase Progress) + PC4
#   (Concurrent Cycles) + PC5 (Session Resume §1) TAIL-LESS => hook emits a single Block
#   cascade enumerating the missing per-pass sites. This is the GENUINE F5 degradation
#   case (the ADV-EDP1-P75-HIGH-002 finding class) the hook is designed to catch.
#
# Fixture: STATE.md with current_cycle: v1.0-feature-engine-discipline-pass-1; PC1
#   (current_step) + PC2 (Last Updated) carry valid LENGTH=4 trajectory-tail markers;
#   PC3/PC4/PC5 are tail-less per-pass rows + cycle INDEX.md WITH per_pass_trajectory: true.
# Expected (v1.9): hook exits 2 (Block); blocking_plugins present; cascade names the
#   missing per-pass sites (Phase Progress / Concurrent Cycles / Session Resume).
#
# Traces to:
#   BC-5.39.009 v1.9 PC3/PC4/PC5 (cycle-conditional Block; F5-per-pass → Block)
#   BC-5.39.009 v1.9 Precondition 7 (per_pass_trajectory: true → flag TRUE → Block)
#   BC-5.39.009 v1.9 invariant 14 (cycle-conditional severity); PC6 cascade (invariant 8)
#
# RED GATE PHASE: test skips when the WASM is not yet compiled. (Note: against the current
#   v1.8 unconditional-Block impl this fixture would also Block — but for the WRONG reason
#   [v1.8 ignores the flag]. The implementer makes this pass for the RIGHT reason: the flag
#   is read as TRUE and PC3/PC4/PC5 are added to the Block cascade. The sibling
#   pass-milestone-cycle-no-block.bats is the discriminating Red Gate that v1.8 fails.)

setup() {
  REPO_ROOT="$(cd "${BATS_TEST_DIRNAME}/../../../.." && pwd)"
  PLUGIN_ROOT="$REPO_ROOT/plugins/vsdd-factory"
  DISPATCHER="$REPO_ROOT/target/release/factory-dispatcher"
  WASM_PLUGIN="$PLUGIN_ROOT/hook-plugins/validate-trajectory-tail-cell-completeness.wasm"
  FIXTURE_SRC="${BATS_TEST_DIRNAME}/../fixtures/validate-trajectory-tail-cell-completeness/fail-state-f5-per-pass-block"
  WORK="$(mktemp -d)"
  mkdir -p "$WORK/hook-plugins"
  mkdir -p "$WORK/.factory/logs"
}

teardown() {
  [ -n "${WORK:-}" ] && [ -d "$WORK" ] && find "$WORK" -type f -delete && find "$WORK" -type d -mindepth 1 | sort -r | xargs rmdir 2>/dev/null && rmdir "$WORK" 2>/dev/null || true
}

_setup_fixture() {
  # F5-style per-pass cycle: wire STATE.md (write target) + the cycle INDEX.md carrying
  # per_pass_trajectory: true into the work tree.
  mkdir -p "$WORK/.factory/cycles/v1.0-feature-engine-discipline-pass-1"
  cp "$FIXTURE_SRC/STATE.md" "$WORK/.factory/STATE.md"
  cp "$FIXTURE_SRC/INDEX.md" "$WORK/.factory/cycles/v1.0-feature-engine-discipline-pass-1/INDEX.md"
}

_write_registry() {
  # POLICY 4: path_allow byte-identical to production priority-158 entry (path_allow=[".factory"]).
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
  printf '{"event_name":"PostToolUse","tool_name":"Edit","session_id":"fail-state-f5-per-pass-block","tool_input":{"file_path":".factory/STATE.md","content":""},"tool_response":{"exit_code":0,"stdout":"","stderr":""}}'
}

# ---------------------------------------------------------------------------
# AC-4/5/6 / PC3/PC4/PC5: F5-per-pass cycle, tail-less per-pass rows => Block exit 2
# ---------------------------------------------------------------------------

@test "test_BC_5_39_009_PC3_PC4_PC5_f5_per_pass_tailless_blocks_exits_2" {
  _require_artifacts
  _setup_fixture
  _write_registry
  cp "$WASM_PLUGIN" "$WORK/hook-plugins/"

  local envelope
  envelope="$(_state_md_envelope)"
  run bash -c "printf '%s' '$envelope' | CLAUDE_PLUGIN_ROOT='$WORK' CLAUDE_PROJECT_DIR='$WORK' '$DISPATCHER' 2>&1 >/dev/null"

  [ "$status" -eq 2 ]
}

@test "test_BC_5_39_009_PC6_f5_per_pass_block_cascade_names_per_pass_sites" {
  _require_artifacts
  _setup_fixture
  _write_registry
  cp "$WASM_PLUGIN" "$WORK/hook-plugins/"

  local envelope
  envelope="$(_state_md_envelope)"
  run bash -c "printf '%s' '$envelope' | CLAUDE_PLUGIN_ROOT='$WORK' CLAUDE_PROJECT_DIR='$WORK' '$DISPATCHER' 2>&1 >/dev/null"

  [ "$status" -eq 2 ]
  [[ "$output" == *"blocking_plugins=validate-trajectory-tail-cell-completeness"* ]]
  # In an F5-per-pass cycle the cascade Blocks on the per-pass sites.
  [[ "$output" == *"Phase Progress"* ]]
  [[ "$output" == *"Concurrent Cycles"* ]]
  [[ "$output" == *"Session Resume"* ]]
}
