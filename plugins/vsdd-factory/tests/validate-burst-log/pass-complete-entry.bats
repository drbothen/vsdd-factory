#!/usr/bin/env bats
# pass-complete-entry.bats — AC-3: hook emits Continue when burst entry has all 9 blocks and Dim-1 matches
#
# Traces to: BC-5.39.004 postcondition 1 (all properties hold => HookResult::Continue)
# Canonical test vector: "Complete valid entry" row in BC-5.39.004 Canonical Test Vectors
# D-NNN closure: D-444(c) + D-446(a) + D-432(e)
#
# RED GATE PHASE: test skips because validate-burst-log.wasm is not yet compiled.
# Implementer (T-7 of S-15.11) must compile the WASM to activate this test.

setup() {
  REPO_ROOT="$(cd "${BATS_TEST_DIRNAME}/../../../.." && pwd)"
  PLUGIN_ROOT="$REPO_ROOT/plugins/vsdd-factory"
  DISPATCHER="$REPO_ROOT/target/release/factory-dispatcher"
  WASM_PLUGIN="$PLUGIN_ROOT/hook-plugins/validate-burst-log.wasm"
  FIXTURE_SRC="${BATS_TEST_DIRNAME}/../fixtures/validate-burst-log/pass-complete-entry"
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
name = "validate-burst-log"
event = "PostToolUse"
tool = "Edit|Write"
plugin = "hook-plugins/validate-burst-log.wasm"
timeout_ms = 5000
on_error = "continue"

[hooks.capabilities.read_file]
path_allow = [
  ".factory/cycles/",
]
TOML
}

_require_artifacts() {
  if [ ! -x "$DISPATCHER" ]; then
    skip "dispatcher binary not built -- run: cargo build --release -p factory-dispatcher"
  fi
  if [ ! -f "$WASM_PLUGIN" ]; then
    skip "validate-burst-log.wasm not built -- implement T-4 through T-7 of S-15.11"
  fi
}

_burst_log_envelope() {
  printf '{"event_name":"PostToolUse","tool_name":"Edit","session_id":"pass-complete","tool_input":{"file_path":".factory/cycles/v1.0-feature-engine-discipline-pass-1/burst-log.md","content":""},"tool_response":{"exit_code":0,"stdout":"","stderr":""}}'
}

# ---------------------------------------------------------------------------
# AC-3: all 9 blocks + Dim-1 correct => dispatcher exits 0 (no block)
# ---------------------------------------------------------------------------

@test "AC-3 PASS: hook emits Continue when burst entry has all 9 blocks and Dim-1 count matches" {
  _require_artifacts
  _setup_fixture
  _write_registry
  cp "$WASM_PLUGIN" "$WORK/hook-plugins/"

  local envelope
  envelope="$(_burst_log_envelope)"
  run bash -c "printf '%s' '$envelope' | CLAUDE_PLUGIN_ROOT='$WORK' CLAUDE_PROJECT_DIR='$WORK' '$DISPATCHER' 2>&1 >/dev/null"

  # Exit 0: no block signal emitted
  [ "$status" -eq 0 ]

  # No blocking_plugins= in stderr for a clean pass
  [[ "$output" != *"blocking_plugins="* ]]
}
