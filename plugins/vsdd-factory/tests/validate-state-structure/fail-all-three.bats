#!/usr/bin/env bats
# fail-all-three.bats — AC-6: multiple violations produce a single block message enumerating all
#
# Traces to:
#   BC-5.39.005 postcondition 5 (multiple violations => single BlockWithFix enumerating all)
#   BC-5.39.005 EC-006 (all three validations fail simultaneously)
#   BC-5.39.005 Canonical Test Vectors: "All three violations" row
# D-NNN closure: BC-5.39.005 postcondition 5
#
# Fixture: STATE.md that violates all three properties simultaneously:
#   - Banner claims 25 lines but file has 28 newlines (banner wc-l off by 3)
#   - Banner has only one margin (soft-target only; "margin from actual" absent)
#   - Trajectory-tail is →9→9→9 (3 components, not 4)
# Expected: hook exits 2 (block) with a single block message naming all 3 violation types.
#
# RED GATE PHASE: test skips because validate-state-structure.wasm is not yet compiled.

setup() {
  REPO_ROOT="$(cd "${BATS_TEST_DIRNAME}/../../../.." && pwd)"
  PLUGIN_ROOT="$REPO_ROOT/plugins/vsdd-factory"
  DISPATCHER="$REPO_ROOT/target/release/factory-dispatcher"
  WASM_PLUGIN="$PLUGIN_ROOT/hook-plugins/validate-state-structure.wasm"
  FIXTURE_SRC="${BATS_TEST_DIRNAME}/../fixtures/validate-state-structure/fail-all-three"
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
    skip "validate-state-structure.wasm not built -- implement T-5 through T-7 of S-15.09"
  fi
}

_state_md_envelope() {
  printf '{"event_name":"PostToolUse","tool_name":"Edit","session_id":"fail-all-three","tool_input":{"file_path":".factory/STATE.md","content":""},"tool_response":{"exit_code":0,"stdout":"","stderr":""}}'
}

# ---------------------------------------------------------------------------
# AC-6: all three violations => dispatcher exits 2 (block)
# Traces to BC-5.39.005 postcondition 5
# ---------------------------------------------------------------------------

@test "AC-6 FAIL: hook blocks when all three violations are present (wc-l + dual-margin + tail)" {
  _require_artifacts
  _setup_fixture
  _write_registry
  cp "$WASM_PLUGIN" "$WORK/hook-plugins/"

  local envelope
  envelope="$(_state_md_envelope)"
  run bash -c "printf '%s' '$envelope' | CLAUDE_PLUGIN_ROOT='$WORK' CLAUDE_PROJECT_DIR='$WORK' '$DISPATCHER' 2>&1 >/dev/null"

  # Exit 2: block signal emitted
  [ "$status" -eq 2 ]

  # blocking_plugins= names this hook
  [[ "$output" == *"blocking_plugins=validate-state-structure"* ]]
}

# ---------------------------------------------------------------------------
# AC-6: single block message enumerates all three violation types
# Traces to BC-5.39.005 postcondition 5 (single combined message requirement)
# ---------------------------------------------------------------------------

@test "AC-6 FAIL: single block message names banner wc-l, dual-margin, and trajectory-tail violations" {
  _require_artifacts
  _setup_fixture
  _write_registry
  cp "$WASM_PLUGIN" "$WORK/hook-plugins/"

  local envelope
  envelope="$(_state_md_envelope)"
  run bash -c "printf '%s' '$envelope' | CLAUDE_PLUGIN_ROOT='$WORK' CLAUDE_PROJECT_DIR='$WORK' '$DISPATCHER' 2>&1 >/dev/null"

  # Exit 2: block signal
  [ "$status" -eq 2 ]

  # block_reason must name all three violation classes (banner wc-l, dual-margin, trajectory-tail)
  # Each of these must appear somewhere in the combined block message
  [[ "$output" == *"D-446"* ]] || [[ "$output" == *"dual-margin"* ]] || [[ "$output" == *"margin"* ]]
  [[ "$output" == *"D-433"* ]] || [[ "$output" == *"trajectory"* ]] || [[ "$output" == *"tail"* ]]
  # Banner wc-l referenced by line count divergence
  [[ "$output" == *"25"* ]] || [[ "$output" == *"28"* ]] || [[ "$output" == *"banner"* ]]
}
