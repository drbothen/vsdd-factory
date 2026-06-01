#!/usr/bin/env bats
# pass-per-pass-flag-unreadable-failopen-advisory.bats — AC-27 / Precondition 7 Step 4 +
#   inv-15: when the active cycle's per_pass_trajectory flag cannot be read (the cycle
#   INDEX.md is ABSENT / the secondary host::read_file fails), the resolved cycle-type
#   defaults to NOT per-pass (flag = FALSE) and PC3/PC4/PC5 route to ADVISORY
#   (host::log_warn + Continue), NEVER Block. This is fail-open-to-advisory, NEVER
#   fail-open-to-Block (a Block fail-open would re-introduce the v1.8 pipeline-brick).
#   PC1/PC2 are UNAFFECTED and always Block on missing tail (here they carry valid tails,
#   so they PASS).
#
# Fixture: STATE.md with current_cycle: v1.0-cycle-with-no-index-file (a cycle whose
#   INDEX.md is deliberately NOT created by _setup_fixture → secondary read fails);
#   PC1 (current_step) + PC2 (Last Updated) carry valid LENGTH=4 trajectory-tail markers;
#   PC3/PC4/PC5 tail-less.
# Expected (v1.9): hook exits 0 (Continue); PC3/PC4/PC5 fail-open-to-advisory (log_warn);
#   NO blocking_plugins.
#
# THIS IS A RED GATE: the current v1.8 impl ignores the flag and Blocks unconditionally on
# tail-less PC3/PC4/PC5, so it would exit 2 here. The expected exit 0 currently FAILS
# against v1.8 — proof the fail-open-to-advisory routing is not yet implemented.
#
# Traces to:
#   BC-5.39.009 v1.9 Precondition 7 Step 4 (fail-open-to-advisory default)
#   BC-5.39.009 v1.9 invariant 15 (fail-open-to-advisory; NEVER fail-open-to-Block)
#
# RED GATE PHASE: test skips when the WASM is not yet compiled.

setup() {
  REPO_ROOT="$(cd "${BATS_TEST_DIRNAME}/../../../.." && pwd)"
  PLUGIN_ROOT="$REPO_ROOT/plugins/vsdd-factory"
  DISPATCHER="$REPO_ROOT/target/release/factory-dispatcher"
  WASM_PLUGIN="$PLUGIN_ROOT/hook-plugins/validate-trajectory-tail-cell-completeness.wasm"
  FIXTURE_SRC="${BATS_TEST_DIRNAME}/../fixtures/validate-trajectory-tail-cell-completeness/pass-per-pass-flag-unreadable-failopen-advisory"
  WORK="$(mktemp -d)"
  mkdir -p "$WORK/hook-plugins"
  mkdir -p "$WORK/.factory/logs"
}

teardown() {
  [ -n "${WORK:-}" ] && [ -d "$WORK" ] && find "$WORK" -type f -delete && find "$WORK" -type d -mindepth 1 | sort -r | xargs rmdir 2>/dev/null && rmdir "$WORK" 2>/dev/null || true
}

_setup_fixture() {
  # Wire ONLY STATE.md. The cycle named in current_cycle:
  # (v1.0-cycle-with-no-index-file) deliberately has NO INDEX.md in the work tree, so the
  # hook's secondary read for the per_pass_trajectory flag FAILS → fail-open-to-advisory.
  mkdir -p "$WORK/.factory"
  cp "$FIXTURE_SRC/STATE.md" "$WORK/.factory/STATE.md"
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
  printf '{"event_name":"PostToolUse","tool_name":"Edit","session_id":"pass-per-pass-flag-unreadable-failopen-advisory","tool_input":{"file_path":".factory/STATE.md","content":""},"tool_response":{"exit_code":0,"stdout":"","stderr":""}}'
}

# ---------------------------------------------------------------------------
# AC-27 / Precondition 7 Step 4 + inv-15: flag unresolvable => fail-open-to-advisory (exit 0)
# ---------------------------------------------------------------------------

@test "test_BC_5_39_009_inv15_flag_unreadable_failopen_advisory_exits_0" {
  _require_artifacts
  _setup_fixture
  _write_registry
  cp "$WASM_PLUGIN" "$WORK/hook-plugins/"

  local envelope
  envelope="$(_state_md_envelope)"
  run bash -c "printf '%s' '$envelope' | CLAUDE_PLUGIN_ROOT='$WORK' CLAUDE_PROJECT_DIR='$WORK' '$DISPATCHER' 2>&1 >/dev/null"

  # Flag unresolvable (cycle INDEX.md absent) → fail-open-to-advisory → exit 0, NEVER Block.
  [ "$status" -eq 0 ]
}

@test "test_BC_5_39_009_inv15_flag_unreadable_never_blocks" {
  _require_artifacts
  _setup_fixture
  _write_registry
  cp "$WASM_PLUGIN" "$WORK/hook-plugins/"

  local envelope
  envelope="$(_state_md_envelope)"
  run bash -c "printf '%s' '$envelope' | CLAUDE_PLUGIN_ROOT='$WORK' CLAUDE_PROJECT_DIR='$WORK' '$DISPATCHER' 2>&1 >/dev/null"

  [ "$status" -eq 0 ]
  # inv-15: a Block fail-open is the v1.8 pipeline-brick ADR-023 cures — MUST NOT happen.
  [[ "$output" != *"blocking_plugins=validate-trajectory-tail-cell-completeness"* ]]
}
