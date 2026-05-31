#!/usr/bin/env bats
# pass-milestone-cycle-no-block.bats — AC-25 / EC-021: milestone cycle (active cycle's
#   INDEX.md has per_pass_trajectory ABSENT) with PC1 (current_step) + PC2 (Last Updated)
#   carrying valid LENGTH=4 trajectory-tail markers but PC3/PC4/PC5 (Phase Progress /
#   Concurrent Cycles / Session Resume §1) being TAIL-LESS milestone/status rows =>
#   hook emits NO Block (PC1/PC2 PASS; PC3/PC4/PC5 advisory log_warn + Continue).
#
# THIS IS THE RED GATE REGRESSION FIXTURE for the live-STATE.md pass-5 brick case:
# the current v1.8 implementation Blocks UNCONDITIONALLY on all 5 STATE.md sites, so it
# would Block this milestone fixture (exit 2). This test expecting exit 0 therefore
# currently FAILS against v1.8 — that failing test IS the proof the v1.9 cycle-conditional
# behavior is not yet implemented (ADR-023 Option (c)).
#
# Fixture: STATE.md mirrors the REAL .factory/STATE.md milestone shape (current_cycle:
#   v1.0-brownfield-backfill; current_step + Last Updated carry trajectory-tail markers;
#   Phase Progress / Concurrent Cycles / Session Resume §1 are tail-less milestone rows) +
#   cycle INDEX.md WITHOUT per_pass_trajectory (milestone convention → flag FALSE).
# Expected (v1.9): hook exits 0 (Continue); NO blocking_plugins; PC3/PC4/PC5 advisory
#   log_warn strings present (advisory fired, not Block).
#
# Traces to:
#   BC-5.39.009 v1.9 EC-021 (milestone-cycle tail-less PC3/PC4/PC5 → NO Block, advisory only)
#   BC-5.39.009 v1.9 Precondition 7 (per_pass_trajectory absent → flag FALSE → advisory)
#   BC-5.39.009 v1.9 invariant 14 (cycle-conditional severity)
#
# RED GATE PHASE: test skips when the WASM is not yet compiled; once compiled it FAILS
#   against v1.8 (Block) until the v1.9 cycle-conditional model is implemented.

setup() {
  REPO_ROOT="$(cd "${BATS_TEST_DIRNAME}/../../../.." && pwd)"
  PLUGIN_ROOT="$REPO_ROOT/plugins/vsdd-factory"
  DISPATCHER="$REPO_ROOT/target/release/factory-dispatcher"
  WASM_PLUGIN="$PLUGIN_ROOT/hook-plugins/validate-trajectory-tail-cell-completeness.wasm"
  FIXTURE_SRC="${BATS_TEST_DIRNAME}/../fixtures/validate-trajectory-tail-cell-completeness/pass-milestone-cycle-no-block"
  WORK="$(mktemp -d)"
  mkdir -p "$WORK/hook-plugins"
  mkdir -p "$WORK/.factory/logs"
}

teardown() {
  [ -n "${WORK:-}" ] && [ -d "$WORK" ] && find "$WORK" -type f -delete && find "$WORK" -type d -mindepth 1 | sort -r | xargs rmdir 2>/dev/null && rmdir "$WORK" 2>/dev/null || true
}

_setup_fixture() {
  # The hook resolves per_pass_trajectory by reading STATE.md current_cycle: then the
  # active cycle's INDEX.md frontmatter. Wire BOTH the STATE.md (write target) AND the
  # milestone cycle's INDEX.md (without the flag) into the work tree.
  mkdir -p "$WORK/.factory/cycles/v1.0-brownfield-backfill"
  cp "$FIXTURE_SRC/STATE.md" "$WORK/.factory/STATE.md"
  cp "$FIXTURE_SRC/INDEX.md" "$WORK/.factory/cycles/v1.0-brownfield-backfill/INDEX.md"
}

_write_registry() {
  # POLICY 4: path_allow MUST stay byte-identical to the production hooks-registry.toml
  # priority-158 entry (path_allow = [".factory"]). The cycle INDEX.md secondary read for
  # cycle-type detection is under .factory, so no path_allow change is needed.
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
  printf '{"event_name":"PostToolUse","tool_name":"Edit","session_id":"pass-milestone-cycle-no-block","tool_input":{"file_path":".factory/STATE.md","content":""},"tool_response":{"exit_code":0,"stdout":"","stderr":""}}'
}

# ---------------------------------------------------------------------------
# AC-25 / EC-021: milestone cycle, tail-less PC3/PC4/PC5 => NO Block (exit 0)
# ---------------------------------------------------------------------------

@test "test_BC_5_39_009_EC021_milestone_cycle_tailless_per_pass_no_block_exits_0" {
  _require_artifacts
  _setup_fixture
  _write_registry
  cp "$WASM_PLUGIN" "$WORK/hook-plugins/"

  local envelope
  envelope="$(_state_md_envelope)"
  run bash -c "printf '%s' '$envelope' | CLAUDE_PLUGIN_ROOT='$WORK' CLAUDE_PROJECT_DIR='$WORK' '$DISPATCHER' 2>&1 >/dev/null"

  # v1.9: milestone cycle (per_pass_trajectory absent) → PC1/PC2 PASS; PC3/PC4/PC5 advisory.
  # NO Block. (Under v1.8 this currently exits 2 — the Red Gate failure that proves the
  # cycle-conditional behavior is not yet implemented.)
  [ "$status" -eq 0 ]
}

@test "test_BC_5_39_009_EC021_milestone_cycle_no_blocking_plugins" {
  _require_artifacts
  _setup_fixture
  _write_registry
  cp "$WASM_PLUGIN" "$WORK/hook-plugins/"

  local envelope
  envelope="$(_state_md_envelope)"
  run bash -c "printf '%s' '$envelope' | CLAUDE_PLUGIN_ROOT='$WORK' CLAUDE_PROJECT_DIR='$WORK' '$DISPATCHER' 2>&1 >/dev/null"

  [ "$status" -eq 0 ]
  # The hook MUST NOT appear as a blocking plugin in a milestone cycle.
  [[ "$output" != *"blocking_plugins=validate-trajectory-tail-cell-completeness"* ]]
}

@test "test_BC_5_39_009_EC021_milestone_cycle_per_pass_sites_advisory_fired" {
  _require_artifacts
  _setup_fixture
  _write_registry
  cp "$WASM_PLUGIN" "$WORK/hook-plugins/"

  local envelope
  envelope="$(_state_md_envelope)"
  run bash -c "printf '%s' '$envelope' | CLAUDE_PLUGIN_ROOT='$WORK' CLAUDE_PROJECT_DIR='$WORK' '$DISPATCHER' 2>&1 >/dev/null"

  [ "$status" -eq 0 ]
  # LOAD-BEARING: the advisory (log_warn) for the tail-less per-pass sites MUST be observable
  # (cycle-conditional → advisory, NOT silent). This proves the hook saw the tail-less
  # PC3/PC4/PC5 rows and routed them to advisory rather than ignoring them entirely.
  [[ "$output" == *"advisory"* ]]
}
