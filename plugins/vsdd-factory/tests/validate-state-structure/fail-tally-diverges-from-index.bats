#!/usr/bin/env bats
# fail-tally-diverges-from-index.bats — AC-4: hook blocks when STATE.md tally values diverge
#                                         from INDEX.md Convergence Status; block message names
#                                         STATE.md value vs INDEX.md value and cites D-432(a)
#
# Traces to:
#   BC-5.39.005 postcondition 5 (cascade: P2-PC-1 tally divergence)
#   D-432(a) — tally-sync: STATE.md tally cells MUST match INDEX.md Convergence Status values
#   D-434(b) — codifying-burst tally agreement
#
# Fixture pair:
#   STATE.md — Convergence Status "pass count: 72"
#   INDEX.md — Convergence Status "pass count: 73" (diverges from STATE.md)
# Expected: hook exits 2 (block) and block_reason names both values (72 vs 73) and cites D-432(a).
#
# RED GATE PHASE: Phase 2 implementation does not exist yet.
# All tests in this file skip pending Phase 2 implementation (T-5 through T-8 of S-15.10).

setup() {
  REPO_ROOT="$(cd "${BATS_TEST_DIRNAME}/../../../.." && pwd)"
  PLUGIN_ROOT="$REPO_ROOT/plugins/vsdd-factory"
  DISPATCHER="$REPO_ROOT/target/release/factory-dispatcher"
  WASM_PLUGIN="$PLUGIN_ROOT/hook-plugins/validate-state-structure.wasm"
  FIXTURE_SRC="${BATS_TEST_DIRNAME}/../fixtures/validate-state-structure/fail-tally-diverges-from-index"
  WORK="$(mktemp -d)"
  mkdir -p "$WORK/hook-plugins"
  mkdir -p "$WORK/.factory/logs"
}

teardown() {
  [ -n "${WORK:-}" ] && [ -d "$WORK" ] && find "$WORK" -type f -delete && find "$WORK" -type d -mindepth 1 | sort -r | xargs rmdir 2>/dev/null && rmdir "$WORK" 2>/dev/null || true
}

_setup_fixture() {
  # Copies both STATE.md and cycles/v1.0-brownfield-backfill/INDEX.md
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
  printf '{"event_name":"PostToolUse","tool_name":"Edit","session_id":"fail-tally-diverges-from-index","tool_input":{"file_path":".factory/STATE.md","content":""},"tool_response":{"exit_code":0,"stdout":"","stderr":""}}'
}

# ---------------------------------------------------------------------------
# AC-4: STATE.md pass count 72, INDEX.md pass count 73 => dispatcher exits 2 (block)
# Traces to BC-5.39.005 P2-PC-1 / D-432(a)+D-434(b)
# ---------------------------------------------------------------------------

@test "AC-4 FAIL: hook blocks when STATE.md pass count (72) diverges from INDEX.md pass count (73)" {
  skip "pending Phase 2 implementation (S-15.10 T-5 through T-8)"
  _require_artifacts
  _setup_fixture
  _write_registry
  cp "$WASM_PLUGIN" "$WORK/hook-plugins/"

  local envelope
  envelope="$(_state_md_envelope)"
  run bash -c "printf '%s' '$envelope' | CLAUDE_PLUGIN_ROOT='$WORK' CLAUDE_PROJECT_DIR='$WORK' '$DISPATCHER' 2>&1 >/dev/null"

  # Exit 2: block signal emitted for tally mismatch
  [ "$status" -eq 2 ]

  # blocking_plugins= names this hook
  [[ "$output" == *"blocking_plugins=validate-state-structure"* ]]
}

@test "AC-4 FAIL: block message names STATE.md value (72) and INDEX.md value (73) and cites D-432(a)" {
  skip "pending Phase 2 implementation (S-15.10 T-5 through T-8)"
  _require_artifacts
  _setup_fixture
  _write_registry
  cp "$WASM_PLUGIN" "$WORK/hook-plugins/"

  local envelope
  envelope="$(_state_md_envelope)"
  run bash -c "printf '%s' '$envelope' | CLAUDE_PLUGIN_ROOT='$WORK' CLAUDE_PROJECT_DIR='$WORK' '$DISPATCHER' 2>&1 >/dev/null"

  [ "$status" -eq 2 ]

  # Block message must name both tally values
  [[ "$output" == *"72"* ]]
  [[ "$output" == *"73"* ]]
  # Must cite D-432(a)
  [[ "$output" == *"D-432"* ]]
}
