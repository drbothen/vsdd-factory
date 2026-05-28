#!/usr/bin/env bats
# fail-open-index-unreadable.bats — AC-15: hook emits Continue (fail-open) when INDEX.md is unreadable
#
# Traces to:
#   BC-5.39.006 postcondition 10; invariant 9; EC-018
#
# Scenario: hook invoked with INDEX.md file_path but no INDEX.md exists in the WORK sandbox.
#           host::read_file returns a file-not-found error. Hook MUST fail-open (Continue, not Block).
# Note: fixture directory is empty by design — no INDEX.md present.
#
# RED GATE PHASE: test skips if validate-dispatch-advance.wasm is not yet compiled.

setup() {
  REPO_ROOT="$(cd "${BATS_TEST_DIRNAME}/../../../.." && pwd)"
  PLUGIN_ROOT="$REPO_ROOT/plugins/vsdd-factory"
  DISPATCHER="$REPO_ROOT/target/release/factory-dispatcher"
  WASM_PLUGIN="$PLUGIN_ROOT/hook-plugins/validate-dispatch-advance.wasm"
  # No fixture setup — unreadable scenario uses absent file
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
name = "validate-dispatch-advance"
event = "PostToolUse"
tool = "Edit|Write"
plugin = "hook-plugins/validate-dispatch-advance.wasm"
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
    skip "validate-dispatch-advance.wasm not built -- implement T-5 through T-7 of S-15.14"
  fi
}

_index_md_envelope_absent() {
  # Points to INDEX.md path that does not exist in WORK — triggers host::read_file error
  printf '{"event_name":"PostToolUse","tool_name":"Edit","session_id":"fail-open-index-unreadable","tool_input":{"file_path":".factory/cycles/v1.0-brownfield-backfill/INDEX.md","content":""},"tool_response":{"exit_code":0,"stdout":"","stderr":""}}'
}

# ---------------------------------------------------------------------------
# AC-15: INDEX.md absent => hook emits Continue (fail-open)
# Traces to BC-5.39.006 postcondition 10; invariant 9; EC-018
# ---------------------------------------------------------------------------

@test "AC-15 PASS (fail-open): hook emits Continue when INDEX.md is not readable via host::read_file" {
  _require_artifacts
  # Intentionally do NOT copy any INDEX.md fixture — file absent from WORK
  [ ! -f "$WORK/.factory/cycles/v1.0-brownfield-backfill/INDEX.md" ]
  _write_registry
  cp "$WASM_PLUGIN" "$WORK/hook-plugins/"

  local envelope
  envelope="$(_index_md_envelope_absent)"
  run bash -c "printf '%s' '$envelope' | CLAUDE_PLUGIN_ROOT='$WORK' CLAUDE_PROJECT_DIR='$WORK' '$DISPATCHER' 2>&1 >/dev/null"

  # Exit 0: fail-open means no block even though INDEX.md is unreadable
  [ "$status" -eq 0 ]

  # No blocking_plugins= for a fail-open pass
  [[ "$output" != *"blocking_plugins=validate-dispatch-advance"* ]]
}
