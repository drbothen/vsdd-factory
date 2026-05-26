#!/usr/bin/env bats
# fail-phase-progress-no-fix-burst-row.bats — AC-8: hook blocks when Phase Progress section
#                                              has no fix-burst row; block message cites D-447(d)
#
# Traces to:
#   BC-5.39.005 postcondition 5 (cascade: P2-PC-3 Phase Progress fix-burst row absent)
#   D-447(d) — Phase Progress pass-N fix burst row mandatory at Commit E
#   D-435(b) — Phase Progress section MUST include both adversary and fix-burst rows
#
# Fixture: STATE.md with Phase Progress section containing only adversary rows
#          (no "fix burst" row); all Phase 1 properties valid.
# Expected: hook exits 2 (block) and block_reason cites D-447(d).
#
# RED GATE PHASE: Phase 2 implementation does not exist yet.
# All tests in this file skip pending Phase 2 implementation (T-5 through T-8 of S-15.10).

setup() {
  REPO_ROOT="$(cd "${BATS_TEST_DIRNAME}/../../../.." && pwd)"
  PLUGIN_ROOT="$REPO_ROOT/plugins/vsdd-factory"
  DISPATCHER="$REPO_ROOT/target/release/factory-dispatcher"
  WASM_PLUGIN="$PLUGIN_ROOT/hook-plugins/validate-state-structure.wasm"
  FIXTURE_SRC="${BATS_TEST_DIRNAME}/../fixtures/validate-state-structure/fail-phase-progress-no-fix-burst-row"
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
name = "validate-state-structure"
event = "PostToolUse"
tool = "Edit|Write"
plugin = "hook-plugins/validate-state-structure.wasm"
timeout_ms = 5000
on_error = "continue"

[hooks.capabilities.read_file]
path_allow = [
  ".factory",
]
TOML
}

_require_artifacts() {
  if [ ! -x "$DISPATCHER" ]; then
    skip "dispatcher binary not built -- run: cargo build --release -p factory-dispatcher"
  fi
  if [ ! -f "$WASM_PLUGIN" ]; then
    skip "validate-state-structure.wasm not built -- implement T-5 through T-8 of S-15.10"
  fi
}

_state_md_envelope() {
  printf '{"event_name":"PostToolUse","tool_name":"Edit","session_id":"fail-phase-progress-no-fix-burst-row","tool_input":{"file_path":".factory/STATE.md","content":""},"tool_response":{"exit_code":0,"stdout":"","stderr":""}}'
}

# ---------------------------------------------------------------------------
# AC-8: Phase Progress has only adversary rows, no fix-burst row => exits 2 (block)
# Traces to BC-5.39.005 P2-PC-3 / D-447(d)
# ---------------------------------------------------------------------------

@test "AC-8 FAIL: hook blocks when Phase Progress section has no fix-burst row (D-447(d))" {
  skip "pending Phase 2 implementation (S-15.10 T-5 through T-8)"
  _require_artifacts
  _setup_fixture
  _write_registry
  cp "$WASM_PLUGIN" "$WORK/hook-plugins/"

  local envelope
  envelope="$(_state_md_envelope)"
  run bash -c "printf '%s' '$envelope' | CLAUDE_PLUGIN_ROOT='$WORK' CLAUDE_PROJECT_DIR='$WORK' '$DISPATCHER' 2>&1 >/dev/null"

  # Exit 2: block signal emitted for missing fix-burst row
  [ "$status" -eq 2 ]

  # blocking_plugins= names this hook
  [[ "$output" == *"blocking_plugins=validate-state-structure"* ]]
}

@test "AC-8 FAIL: block message cites D-447(d) for missing fix-burst row in Phase Progress" {
  skip "pending Phase 2 implementation (S-15.10 T-5 through T-8)"
  _require_artifacts
  _setup_fixture
  _write_registry
  cp "$WASM_PLUGIN" "$WORK/hook-plugins/"

  local envelope
  envelope="$(_state_md_envelope)"
  run bash -c "printf '%s' '$envelope' | CLAUDE_PLUGIN_ROOT='$WORK' CLAUDE_PROJECT_DIR='$WORK' '$DISPATCHER' 2>&1 >/dev/null"

  [ "$status" -eq 2 ]

  # Block message must cite D-447(d)
  [[ "$output" == *"D-447"* ]]
}
