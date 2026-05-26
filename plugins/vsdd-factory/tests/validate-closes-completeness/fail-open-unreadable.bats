#!/usr/bin/env bats
# fail-open-unreadable.bats — AC-16: hook is fail-open when file is unreadable;
#                               unreadable file emits Continue + log_warn (NOT Block)
#
# Traces to:
#   BC-5.39.007 postcondition 10 (host::read_file error => Continue + log_warn, NOT Block)
#   BC-5.39.007 invariant 9 (all read failures are fail-open)
#   BC-5.39.007 invariant 3 (path-component-strict guard)
#
# Scenario: the hook is invoked with a lessons.md file path that does not exist in the
#           sandbox (WORK directory has no lessons.md). The hook attempts host::read_file
#           and receives a file-not-found error, which must be treated as fail-open (Continue).
#
# Note: this test does NOT arrange a fixture on disk — it relies on the dispatcher envelope
#       naming a file that is not present in WORK, triggering a read error.
#       The fixture directory is empty by design.
#
# RED GATE PHASE: test skips because validate-closes-completeness.wasm is not yet compiled.

setup() {
  REPO_ROOT="$(cd "${BATS_TEST_DIRNAME}/../../../.." && pwd)"
  PLUGIN_ROOT="$REPO_ROOT/plugins/vsdd-factory"
  DISPATCHER="$REPO_ROOT/target/release/factory-dispatcher"
  WASM_PLUGIN="$PLUGIN_ROOT/hook-plugins/validate-closes-completeness.wasm"
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
    skip "validate-closes-completeness.wasm not built -- implement T-4 through T-7 of S-15.12"
  fi
}

_lessons_md_envelope_absent_file() {
  # Points to a lessons.md path that does not exist in $WORK — triggers host::read_file error
  printf '{"event_name":"PostToolUse","tool_name":"Edit","session_id":"fail-open-unreadable","tool_input":{"file_path":".factory/cycles/v1.0-brownfield-backfill/lessons.md","content":""},"tool_response":{"exit_code":0,"stdout":"","stderr":""}}'
}

# ---------------------------------------------------------------------------
# AC-16: lessons.md absent from sandbox => hook emits Continue (fail-open, not Block)
# Traces to BC-5.39.007 postcondition 10 + invariant 9
# ---------------------------------------------------------------------------

@test "AC-16 PASS (fail-open): hook emits Continue when lessons.md is not readable via host::read_file" {
  _require_artifacts
  # Intentionally do NOT copy any fixture — lessons.md absent from $WORK
  [ ! -f "$WORK/.factory/cycles/v1.0-brownfield-backfill/lessons.md" ]
  _write_registry
  cp "$WASM_PLUGIN" "$WORK/hook-plugins/"

  local envelope
  envelope="$(_lessons_md_envelope_absent_file)"
  run bash -c "printf '%s' '$envelope' | CLAUDE_PLUGIN_ROOT='$WORK' CLAUDE_PROJECT_DIR='$WORK' '$DISPATCHER' 2>&1 >/dev/null"

  # Exit 0: fail-open means no block even though lessons.md is unreadable
  [ "$status" -eq 0 ]

  # No blocking_plugins= for a fail-open pass
  [[ "$output" != *"blocking_plugins="* ]]
}
