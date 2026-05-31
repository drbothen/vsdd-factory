#!/usr/bin/env bats
# pass-index-all-sites-present.bats — AC-13: INDEX.md with all prescribed sites populated
#   => hook emits Continue cleanly
#
# Traces to:
#   BC-5.39.009 postcondition 12 (PC12: all applicable sites present => Continue no warnings)
#
# Fixture: INDEX.md with both Convergence Status row and latest adv-review row populated
# Expected: hook exits 0 (Continue); no blocking_plugins signal
#
# RED GATE PHASE: test skips because validate-trajectory-tail-cell-completeness.wasm not yet compiled.

setup() {
  REPO_ROOT="$(cd "${BATS_TEST_DIRNAME}/../../../.." && pwd)"
  PLUGIN_ROOT="$REPO_ROOT/plugins/vsdd-factory"
  DISPATCHER="$REPO_ROOT/target/release/factory-dispatcher"
  WASM_PLUGIN="$PLUGIN_ROOT/hook-plugins/validate-trajectory-tail-cell-completeness.wasm"
  FIXTURE_SRC="${BATS_TEST_DIRNAME}/../fixtures/validate-trajectory-tail-cell-completeness/pass-index-all-sites-present"
  WORK="$(mktemp -d)"
  mkdir -p "$WORK/hook-plugins"
  mkdir -p "$WORK/.factory/logs"
}

teardown() {
  [ -n "${WORK:-}" ] && [ -d "$WORK" ] && find "$WORK" -type f -delete && find "$WORK" -type d -mindepth 1 | sort -r | xargs rmdir 2>/dev/null && rmdir "$WORK" 2>/dev/null || true
}

_setup_fixture() {
  mkdir -p "$WORK/.factory/cycles/v1.0-brownfield-backfill"
  cp "$FIXTURE_SRC/INDEX.md" "$WORK/.factory/cycles/v1.0-brownfield-backfill/INDEX.md"
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
  printf '{"event_name":"PostToolUse","tool_name":"Edit","session_id":"pass-index-all-sites-present","tool_input":{"file_path":".factory/cycles/v1.0-brownfield-backfill/INDEX.md","content":""},"tool_response":{"exit_code":0,"stdout":"","stderr":""}}'
}

# ---------------------------------------------------------------------------
# AC-13 / PC12: INDEX.md all sites present => Continue exit 0
# ---------------------------------------------------------------------------

@test "test_BC_5_39_009_PC12_index_all_sites_present_exits_0" {
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

# F-002: prove the all-sites-present path emits NO advisories. Without this, a wholly
# inert INDEX.md arm would also pass the exit-0 assertion above; this test pins that BOTH
# INDEX.md advisory strings are ABSENT when all sites carry a valid tail.
@test "test_BC_5_39_009_PC12_index_all_sites_present_no_advisory" {
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
  # Internal logging is enabled; the log file must exist and neither INDEX.md advisory
  # string may appear (all sites carry a valid tail).
  [ -n "$logf" ]
  ! grep -q "INDEX.md adv-table latest row missing" "$logf"
  ! grep -q "INDEX.md Convergence Status row missing" "$logf"
}
