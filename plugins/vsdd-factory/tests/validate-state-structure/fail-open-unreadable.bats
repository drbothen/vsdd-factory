#!/usr/bin/env bats
# fail-open-unreadable.bats — AC-7: hook emits Continue (fail-open) when STATE.md is unreadable
#
# Traces to:
#   BC-5.39.005 postcondition 6 (host::read_file error => Continue + log_warn, NOT Block)
#   BC-5.39.005 invariant 7 (all host::read_file calls are fail-open)
#   BC-5.39.005 EC-007 (host::read_file returns HostError::CapabilityDenied => Continue + log_warn)
#   BC-5.39.005 Canonical Test Vectors: "Read failure" row
# D-NNN closure: BC-5.39.005 invariant 7 (fail-open guarantee)
#
# Scenario: the hook is invoked with a STATE.md file path that does not exist in the
#           sandbox (WORK directory has no STATE.md). The hook attempts host::read_file
#           and receives a file-not-found error, which must be treated as fail-open (Continue).
#
# Note: this test does NOT arrange an unreadable fixture file on disk — it relies on the
#       dispatcher envelope naming a file that is not present in WORK, triggering a read error.
#       The fixture directory is empty by design (no STATE.md).
#
# RED GATE PHASE: test skips because validate-state-structure.wasm is not yet compiled.

setup() {
  REPO_ROOT="$(cd "${BATS_TEST_DIRNAME}/../../../.." && pwd)"
  PLUGIN_ROOT="$REPO_ROOT/plugins/vsdd-factory"
  DISPATCHER="$REPO_ROOT/target/release/factory-dispatcher"
  WASM_PLUGIN="$PLUGIN_ROOT/hook-plugins/validate-state-structure.wasm"
  # No fixture setup needed — unreadable scenario uses absent file
  WORK="$(mktemp -d)"
  mkdir -p "$WORK/hook-plugins"
  mkdir -p "$WORK/.factory/logs"
}

teardown() {
  [ -n "${WORK:-}" ] && [ -d "$WORK" ] && find "$WORK" -type f -delete && find "$WORK" -type d -mindepth 1 | sort -r | xargs rmdir 2>/dev/null && rmdir "$WORK" 2>/dev/null || true
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

_state_md_envelope_absent_file() {
  # Points to a STATE.md path that does not exist in $WORK — triggers host::read_file error
  printf '{"event_name":"PostToolUse","tool_name":"Edit","session_id":"fail-open-unreadable","tool_input":{"file_path":".factory/STATE.md","content":""},"tool_response":{"exit_code":0,"stdout":"","stderr":""}}'
}

# ---------------------------------------------------------------------------
# AC-7: STATE.md absent from sandbox => hook emits Continue (fail-open, not Block)
# Traces to BC-5.39.005 postcondition 6 + invariant 7
# ---------------------------------------------------------------------------

@test "AC-7 PASS (fail-open): hook emits Continue when STATE.md is not readable via host::read_file" {
  _require_artifacts
  # Intentionally do NOT copy any fixture — STATE.md absent from $WORK
  [ ! -f "$WORK/.factory/STATE.md" ]
  _write_registry
  cp "$WASM_PLUGIN" "$WORK/hook-plugins/"

  local envelope
  envelope="$(_state_md_envelope_absent_file)"
  run bash -c "printf '%s' '$envelope' | CLAUDE_PLUGIN_ROOT='$WORK' CLAUDE_PROJECT_DIR='$WORK' '$DISPATCHER' 2>&1 >/dev/null"

  # Exit 0: fail-open means no block even though STATE.md is unreadable
  [ "$status" -eq 0 ]

  # No blocking_plugins= for a fail-open pass
  [[ "$output" != *"blocking_plugins="* ]]
}
