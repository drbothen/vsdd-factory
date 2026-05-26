#!/usr/bin/env bats
# fail-lessons-missing-closes.bats — AC-1: hook blocks when lessons.md entry is missing **Closes:** line;
#                                     block message names the entry heading and cites D-448(b)
#
# Traces to:
#   BC-5.39.007 postcondition 2 (lessons.md entry with no **Closes:** line => BlockWithFix citing D-448(b))
#   BC-5.39.007 invariant 3 (path-component-strict guard: file_name() == "lessons.md")
# D-NNN closure: D-448(b)
#
# Fixture: lessons.md with ## L-EDP1-007 entry but no **Closes:** line and no exemption declaration.
# Expected: hook exits 2 (block) and block_reason cites D-448.
#
# RED GATE PHASE: test skips because validate-closes-completeness.wasm is not yet compiled.

setup() {
  REPO_ROOT="$(cd "${BATS_TEST_DIRNAME}/../../../.." && pwd)"
  PLUGIN_ROOT="$REPO_ROOT/plugins/vsdd-factory"
  DISPATCHER="$REPO_ROOT/target/release/factory-dispatcher"
  WASM_PLUGIN="$PLUGIN_ROOT/hook-plugins/validate-closes-completeness.wasm"
  FIXTURE_SRC="${BATS_TEST_DIRNAME}/../fixtures/validate-closes-completeness/fail-lessons-missing-closes"
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

_lessons_md_envelope() {
  printf '{"event_name":"PostToolUse","tool_name":"Edit","session_id":"fail-lessons-missing-closes","tool_input":{"file_path":".factory/cycles/v1.0-brownfield-backfill/lessons.md","content":""},"tool_response":{"exit_code":0,"stdout":"","stderr":""}}'
}

# ---------------------------------------------------------------------------
# AC-1: lessons.md entry missing **Closes:** => dispatcher exits 2 (block)
# Traces to BC-5.39.007 postcondition 2
# ---------------------------------------------------------------------------

@test "AC-1 FAIL: hook blocks when lessons.md entry is missing **Closes:** line" {
  _require_artifacts
  _setup_fixture
  _write_registry
  cp "$WASM_PLUGIN" "$WORK/hook-plugins/"

  local envelope
  envelope="$(_lessons_md_envelope)"
  run bash -c "printf '%s' '$envelope' | CLAUDE_PLUGIN_ROOT='$WORK' CLAUDE_PROJECT_DIR='$WORK' '$DISPATCHER' 2>&1 >/dev/null"

  # Exit 2: block signal emitted
  [ "$status" -eq 2 ]

  # blocking_plugins= names this hook
  [[ "$output" == *"blocking_plugins=validate-closes-completeness"* ]]
}

# ---------------------------------------------------------------------------
# AC-1: block message cites D-448(b)
# Traces to BC-5.39.007 postcondition 2 (actionable block message requirement)
# ---------------------------------------------------------------------------

@test "AC-1 FAIL: block message cites D-448 for missing **Closes:** line" {
  _require_artifacts
  _setup_fixture
  _write_registry
  cp "$WASM_PLUGIN" "$WORK/hook-plugins/"

  local envelope
  envelope="$(_lessons_md_envelope)"
  run bash -c "printf '%s' '$envelope' | CLAUDE_PLUGIN_ROOT='$WORK' CLAUDE_PROJECT_DIR='$WORK' '$DISPATCHER' 2>&1 >/dev/null"

  # Exit 2: block signal
  [ "$status" -eq 2 ]

  # block_reason must cite D-448
  [[ "$output" == *"D-448"* ]]
}
