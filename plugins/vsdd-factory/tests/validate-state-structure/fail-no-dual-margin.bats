#!/usr/bin/env bats
# fail-no-dual-margin.bats — AC-2: hook blocks when dual-margin form is absent from banner;
#                             block message cites D-446(c)
#
# Traces to:
#   BC-5.39.005 postcondition 3 (dual-margin absent => HookResult::BlockWithFix citing D-446(c))
#   BC-5.39.005 EC-003 (banner has only one margin: soft-target present, hard-target absent)
#   BC-5.39.005 Canonical Test Vectors: "Missing dual-margin" row
# D-NNN closure: D-446(c)
#
# Fixture: STATE.md with correct line-count (28) and trajectory-tail (→9→9→9→9) but banner
#          has only one margin expression (soft-target only; "margin from actual" absent).
# Expected: hook exits 2 (block) and block_reason cites D-446(c).
#
# RED GATE PHASE: test skips because validate-state-structure.wasm is not yet compiled.

setup() {
  REPO_ROOT="$(cd "${BATS_TEST_DIRNAME}/../../../.." && pwd)"
  PLUGIN_ROOT="$REPO_ROOT/plugins/vsdd-factory"
  DISPATCHER="$REPO_ROOT/target/release/factory-dispatcher"
  WASM_PLUGIN="$PLUGIN_ROOT/hook-plugins/validate-state-structure.wasm"
  FIXTURE_SRC="${BATS_TEST_DIRNAME}/../fixtures/validate-state-structure/fail-no-dual-margin"
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
  printf '{"event_name":"PostToolUse","tool_name":"Edit","session_id":"fail-no-dual-margin","tool_input":{"file_path":".factory/STATE.md","content":""},"tool_response":{"exit_code":0,"stdout":"","stderr":""}}'
}

# ---------------------------------------------------------------------------
# AC-2: dual-margin absent => dispatcher exits 2 (block)
# Traces to BC-5.39.005 postcondition 3
# ---------------------------------------------------------------------------

@test "AC-2 FAIL: hook blocks when banner has only one margin (soft-target only, actual margin absent)" {
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
# AC-2: block message cites D-446(c) for the missing dual-margin form
# Traces to BC-5.39.005 postcondition 3 (actionable message with D-446(c) citation)
# ---------------------------------------------------------------------------

@test "AC-2 FAIL: block message cites D-446(c) when dual-margin form is absent" {
  _require_artifacts
  _setup_fixture
  _write_registry
  cp "$WASM_PLUGIN" "$WORK/hook-plugins/"

  local envelope
  envelope="$(_state_md_envelope)"
  run bash -c "printf '%s' '$envelope' | CLAUDE_PLUGIN_ROOT='$WORK' CLAUDE_PROJECT_DIR='$WORK' '$DISPATCHER' 2>&1 >/dev/null"

  # Exit 2: block signal
  [ "$status" -eq 2 ]

  # block_reason must cite D-446(c) (canonical anchor for dual-margin requirement)
  [[ "$output" == *"D-446"* ]]
}
