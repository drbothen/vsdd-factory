#!/usr/bin/env bats
# fail-banner-wc-off-by-one.bats — AC-1 + AC-8: hook blocks when banner claims N-1 lines
#                                   but file has N newlines; block message names both counts
#
# Traces to:
#   BC-5.39.005 postcondition 2 (banner wc-l diverges => HookResult::BlockWithFix naming counts)
#   BC-5.39.005 EC-001 (banner claims 27 lines, file has 28 newlines)
#   BC-5.39.005 Canonical Test Vectors: "Banner wc-l off by one" row
# D-NNN closure: D-421(c)+D-422(c)+D-424(b)+D-428(d)+D-438(a)+D-440(d)+D-442(d)
#
# Fixture: STATE.md where banner says "27 lines (wc-l)" but file has 28 newlines.
#          Dual-margin and trajectory-tail are correct in this fixture.
# Expected: hook exits 2 (block) and block_reason names claimed count (27) and actual count (28).
#
# RED GATE PHASE: test skips because validate-state-structure.wasm is not yet compiled.
# Implementer (T-5 through T-7 of S-15.09) must compile the WASM to activate this test.

setup() {
  REPO_ROOT="$(cd "${BATS_TEST_DIRNAME}/../../../.." && pwd)"
  PLUGIN_ROOT="$REPO_ROOT/plugins/vsdd-factory"
  DISPATCHER="$REPO_ROOT/target/release/factory-dispatcher"
  WASM_PLUGIN="$PLUGIN_ROOT/hook-plugins/validate-state-structure.wasm"
  FIXTURE_SRC="${BATS_TEST_DIRNAME}/../fixtures/validate-state-structure/fail-banner-wc-off-by-one"
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
  printf '{"event_name":"PostToolUse","tool_name":"Edit","session_id":"fail-banner-wc-off-by-one","tool_input":{"file_path":".factory/STATE.md","content":""},"tool_response":{"exit_code":0,"stdout":"","stderr":""}}'
}

# ---------------------------------------------------------------------------
# AC-1: banner claims 27 lines but file has 28 => dispatcher exits 2 (block)
# Traces to BC-5.39.005 postcondition 2
# ---------------------------------------------------------------------------

@test "AC-1 FAIL: hook blocks when banner claims 27 lines but file has 28 newlines" {
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
# AC-8: block message names claimed count (27) and actual count (28)
# Traces to BC-5.39.005 postcondition 2 (actionable message requirement)
# ---------------------------------------------------------------------------

@test "AC-8 FAIL: block message names claimed count (27) and actual count (28) for banner wc-l mismatch" {
  _require_artifacts
  _setup_fixture
  _write_registry
  cp "$WASM_PLUGIN" "$WORK/hook-plugins/"

  local envelope
  envelope="$(_state_md_envelope)"
  run bash -c "printf '%s' '$envelope' | CLAUDE_PLUGIN_ROOT='$WORK' CLAUDE_PROJECT_DIR='$WORK' '$DISPATCHER' 2>&1 >/dev/null"

  # Exit 2: block signal
  [ "$status" -eq 2 ]

  # block_reason must name both counts (D-421(c) anchor)
  [[ "$output" == *"27"* ]]
  [[ "$output" == *"28"* ]]
}
