#!/usr/bin/env bats
# pass-length4-marker-with-length5-prose.bats — AC-26 / EC-022: a STATE.md cell carrying
#   BOTH a 4-segment `trajectory-tail →9→9→9→11` marker AND a 5-segment
#   `Full-cycle trajectory: →9→9→9→9→11` prose string => hook emits PASS (Continue).
#   The inv-4 two-step marker-prefix check scopes the count to the segment from the
#   `trajectory-tail ` marker (exclusive) to the first `;` — the 5-segment prose is NOT a
#   `trajectory-tail ` marker, so the scoped count is exactly 4 → PASS. This LOCKS inv-4's
#   marker-prefix first-semicolon-segment scoping against the LENGTH=5 prose form (the fix
#   MUST NOT relax inv-4 to LENGTH≥4 to "accommodate" the prose).
#
# Fixture: milestone cycle (no per_pass_trajectory flag); PC1 (current_step) + PC2
#   (Last Updated) each carry BOTH a 4-segment trajectory-tail marker AND a 5-segment
#   Full-cycle-trajectory prose string; PC3/PC4/PC5 tail-less (advisory in milestone cycle).
# Expected: hook exits 0 (Continue) — PC1/PC2 PASS via marker-prefix scoping.
#
# Traces to:
#   BC-5.39.009 v1.9 EC-022 (LENGTH=4 marker coexisting with LENGTH=5 prose → PASS)
#   BC-5.39.009 v1.9 invariant 4 (LENGTH=4 STRICT marker-prefix scoping)
#
# RED GATE PHASE: test skips when the WASM is not yet compiled. (The inv-4 marker-prefix
#   scoping is already implemented at v1.8, so this is primarily a REGRESSION-LOCK ensuring
#   the v1.9 cycle-conditional re-spec does not relax inv-4 to LENGTH≥4.)

setup() {
  REPO_ROOT="$(cd "${BATS_TEST_DIRNAME}/../../../.." && pwd)"
  PLUGIN_ROOT="$REPO_ROOT/plugins/vsdd-factory"
  DISPATCHER="$REPO_ROOT/target/release/factory-dispatcher"
  WASM_PLUGIN="$PLUGIN_ROOT/hook-plugins/validate-trajectory-tail-cell-completeness.wasm"
  FIXTURE_SRC="${BATS_TEST_DIRNAME}/../fixtures/validate-trajectory-tail-cell-completeness/pass-length4-marker-with-length5-prose"
  WORK="$(mktemp -d)"
  mkdir -p "$WORK/hook-plugins"
  mkdir -p "$WORK/.factory/logs"
}

teardown() {
  [ -n "${WORK:-}" ] && [ -d "$WORK" ] && find "$WORK" -type f -delete && find "$WORK" -type d -mindepth 1 | sort -r | xargs rmdir 2>/dev/null && rmdir "$WORK" 2>/dev/null || true
}

_setup_fixture() {
  mkdir -p "$WORK/.factory/cycles/v1.0-brownfield-backfill"
  cp "$FIXTURE_SRC/STATE.md" "$WORK/.factory/STATE.md"
  cp "$FIXTURE_SRC/INDEX.md" "$WORK/.factory/cycles/v1.0-brownfield-backfill/INDEX.md"
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
  printf '{"event_name":"PostToolUse","tool_name":"Edit","session_id":"pass-length4-marker-with-length5-prose","tool_input":{"file_path":".factory/STATE.md","content":""},"tool_response":{"exit_code":0,"stdout":"","stderr":""}}'
}

# ---------------------------------------------------------------------------
# AC-26 / EC-022: 4-segment marker + 5-segment prose in same cell => PASS (exit 0)
# ---------------------------------------------------------------------------

@test "test_BC_5_39_009_EC022_length4_marker_with_length5_prose_passes_exits_0" {
  _require_artifacts
  _setup_fixture
  _write_registry
  cp "$WASM_PLUGIN" "$WORK/hook-plugins/"

  local envelope
  envelope="$(_state_md_envelope)"
  run bash -c "printf '%s' '$envelope' | CLAUDE_PLUGIN_ROOT='$WORK' CLAUDE_PROJECT_DIR='$WORK' '$DISPATCHER' 2>&1 >/dev/null"

  # PC1/PC2 marker-prefix scoping counts only the 4-segment trajectory-tail marker → PASS.
  [ "$status" -eq 0 ]
}

@test "test_BC_5_39_009_EC022_length4_marker_with_length5_prose_no_block" {
  _require_artifacts
  _setup_fixture
  _write_registry
  cp "$WASM_PLUGIN" "$WORK/hook-plugins/"

  local envelope
  envelope="$(_state_md_envelope)"
  run bash -c "printf '%s' '$envelope' | CLAUDE_PLUGIN_ROOT='$WORK' CLAUDE_PROJECT_DIR='$WORK' '$DISPATCHER' 2>&1 >/dev/null"

  [ "$status" -eq 0 ]
  [[ "$output" != *"blocking_plugins=validate-trajectory-tail-cell-completeness"* ]]
}
