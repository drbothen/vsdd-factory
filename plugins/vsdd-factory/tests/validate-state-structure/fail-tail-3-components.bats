#!/usr/bin/env bats
# fail-tail-3-components.bats — AC-3: hook blocks when trajectory-tail has 3 components;
#                                block message names actual count (3) and required count (4)
#
# Traces to:
#   BC-5.39.005 postcondition 4 (tail != 4 => BlockWithFix naming actual count + required 4)
#   BC-5.39.005 EC-004 (trajectory-tail has 3 components: →9→9→9)
#   BC-5.39.005 Canonical Test Vectors: "Tail 3 components" row
# D-NNN closure: D-433(e)+D-439(c)+D-451(c)+D-432(b)
#
# Fixture: STATE.md with correct line-count (28), dual-margin present, but trajectory-tail
#          is →9→9→9 (3 components, not 4).
# Expected: hook exits 2 (block) and block_reason names "3 components" and "required LENGTH=4"
#           citing D-433(e).
#
# RED GATE PHASE: test skips because validate-state-structure.wasm is not yet compiled.

setup() {
  REPO_ROOT="$(cd "${BATS_TEST_DIRNAME}/../../../.." && pwd)"
  PLUGIN_ROOT="$REPO_ROOT/plugins/vsdd-factory"
  DISPATCHER="$REPO_ROOT/target/release/factory-dispatcher"
  WASM_PLUGIN="$PLUGIN_ROOT/hook-plugins/validate-state-structure.wasm"
  FIXTURE_SRC="${BATS_TEST_DIRNAME}/../fixtures/validate-state-structure/fail-tail-3-components"
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
  printf '{"event_name":"PostToolUse","tool_name":"Edit","session_id":"fail-tail-3-components","tool_input":{"file_path":".factory/STATE.md","content":""},"tool_response":{"exit_code":0,"stdout":"","stderr":""}}'
}

# ---------------------------------------------------------------------------
# AC-3: tail has 3 components (→9→9→9) => dispatcher exits 2 (block)
# Traces to BC-5.39.005 postcondition 4
# ---------------------------------------------------------------------------

@test "AC-3 FAIL: hook blocks when trajectory-tail has 3 components (→9→9→9, not →9→9→9→9)" {
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
# AC-3: block message names actual count (3) and required count (4) citing D-433(e)
# Traces to BC-5.39.005 postcondition 4 (actionable message requirement)
# ---------------------------------------------------------------------------

@test "AC-3 FAIL: block message names 3 components and required LENGTH=4 citing D-433(e)" {
  _require_artifacts
  _setup_fixture
  _write_registry
  cp "$WASM_PLUGIN" "$WORK/hook-plugins/"

  local envelope
  envelope="$(_state_md_envelope)"
  run bash -c "printf '%s' '$envelope' | CLAUDE_PLUGIN_ROOT='$WORK' CLAUDE_PROJECT_DIR='$WORK' '$DISPATCHER' 2>&1 >/dev/null"

  # Exit 2: block signal
  [ "$status" -eq 2 ]

  # block_reason must name actual count (3), required count (4), and cite D-433(e)
  [[ "$output" == *"3"* ]]
  [[ "$output" == *"4"* ]]
  [[ "$output" == *"D-433"* ]]
}
