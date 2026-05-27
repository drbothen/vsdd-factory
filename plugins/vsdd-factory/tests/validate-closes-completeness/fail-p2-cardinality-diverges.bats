#!/usr/bin/env bats
# fail-p2-cardinality-diverges.bats — AC-5: hook blocks when cardinality of cited findings
#                                       diverges across citation sites; one STATE.md row
#                                       cites 3 findings, another cites only 2;
#                                       block message cites D-420(a)
#
# Traces to:
#   BC-5.39.007 Phase 2 P2-4 postcondition (cardinality divergence => BlockWithFix citing D-420(a))
#   D-420(a) (multi-site closure-set cardinality agreement: all sites must cite same count)
#
# Fixture: fail-p2-cardinality-diverges/ — pointer (integer 15);
#          adv-cycle-pass-15.md with 3 findings;
#          STATE.md row D-411 Closes cites all 3 (F-P15-001, F-P15-002, F-P15-003);
#          STATE.md row D-413 Closes cites only 2 (F-P15-001, F-P15-002)
# Expected: hook exits 2 (block); block_reason cites D-420(a)
#
# RED GATE PHASE: @pending — Phase 2 logic not yet implemented. All tests in this file
# are marked @pending and will fail until the implementer adds Phase 2 code.

setup() {
  REPO_ROOT="$(cd "${BATS_TEST_DIRNAME}/../../../.." && pwd)"
  PLUGIN_ROOT="$REPO_ROOT/plugins/vsdd-factory"
  DISPATCHER="$REPO_ROOT/target/release/factory-dispatcher"
  WASM_PLUGIN="$PLUGIN_ROOT/hook-plugins/validate-closes-completeness.wasm"
  FIXTURE_SRC="${BATS_TEST_DIRNAME}/../fixtures/validate-closes-completeness/fail-p2-cardinality-diverges"
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
name = "validate-closes-completeness"
event = "PostToolUse"
tool = "Edit|Write"
plugin = "hook-plugins/validate-closes-completeness.wasm"
priority = 156
timeout_ms = 5000
on_error = "continue"

[hooks.capabilities.read_file]
path_allow = [
  ".factory/cycles",
  ".factory/STATE.md",
  ".factory/current-adversary-pass.txt",
  ".factory/cycles/v1.0-brownfield-backfill/INDEX.md",
  ".factory/cycles/v1.0-brownfield-backfill/lessons.md",
]
TOML
}

_require_artifacts() {
  if [ ! -x "$DISPATCHER" ]; then
    skip "dispatcher binary not built -- run: cargo build --release -p factory-dispatcher"
  fi
  if [ ! -f "$WASM_PLUGIN" ]; then
    skip "validate-closes-completeness.wasm not built -- implement T-4 through T-7 of S-15.13"
  fi
}

_state_md_envelope() {
  printf '{"event_name":"PostToolUse","tool_name":"Edit","session_id":"fail-p2-cardinality-diverges","tool_input":{"file_path":".factory/STATE.md","content":""},"tool_response":{"exit_code":0,"stdout":"","stderr":""}}'
}

# ---------------------------------------------------------------------------
# AC-5: cardinality divergence (3 vs 2 findings across STATE.md rows) => exits 2 (block)
# Traces to BC-5.39.007 Phase 2 P2-4 postcondition + D-420(a)
# ---------------------------------------------------------------------------

@test "AC-5 FAIL: hook blocks when citation sites have divergent finding counts (3 vs 2)" {
  _require_artifacts
  _setup_fixture
  _write_registry
  cp "$WASM_PLUGIN" "$WORK/hook-plugins/"

  local envelope
  envelope="$(_state_md_envelope)"
  run bash -c "printf '%s' '$envelope' | CLAUDE_PLUGIN_ROOT='$WORK' CLAUDE_PROJECT_DIR='$WORK' '$DISPATCHER' 2>&1 >/dev/null"

  # Exit 2: block on cardinality mismatch
  [ "$status" -eq 2 ]

  # blocking_plugins= names this hook
  [[ "$output" == *"blocking_plugins=validate-closes-completeness"* ]]
}

# ---------------------------------------------------------------------------
# AC-5: block message cites D-420(a) for cardinality violation
# Traces to BC-5.39.007 Phase 2 P2-4 — block message must cite D-420(a)
# ---------------------------------------------------------------------------

@test "AC-5 FAIL: block message cites D-420(a) for cardinality divergence across citation sites" {
  _require_artifacts
  _setup_fixture
  _write_registry
  cp "$WASM_PLUGIN" "$WORK/hook-plugins/"

  local envelope
  envelope="$(_state_md_envelope)"
  run bash -c "printf '%s' '$envelope' | CLAUDE_PLUGIN_ROOT='$WORK' CLAUDE_PROJECT_DIR='$WORK' '$DISPATCHER' 2>&1 >/dev/null"

  [ "$status" -eq 2 ]
  [[ "$output" == *"D-420"* ]]
}
