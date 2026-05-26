#!/usr/bin/env bats
# fail-multiple-violations.bats — AC-15: multiple violations in one write produce a single
#                                   BlockWithFix enumerating all violations
#
# Traces to:
#   BC-5.39.007 postcondition 9 (multiple violations => single BlockWithFix enumerating all)
#   BC-5.39.007 invariant 3 (path-component-strict guard)
# D-NNN closure: D-448(b)+D-419(c)+D-441(c)
#
# Fixture: lessons.md with three violations in the same file:
#   - L-EDP1-020: empty **Closes:** line (D-448(b) violation)
#   - L-EDP1-021: "**Closes:** resolved several issues" (D-419(c) malformed cite)
#   - L-EDP1-021: bare umbrella cite D-389..D-480 without sample-vs-exhaustive flag (D-441(c))
# Expected: hook exits 2 (block); single block message names all 3 violations.
#
# RED GATE PHASE: test skips because validate-closes-completeness.wasm is not yet compiled.

setup() {
  REPO_ROOT="$(cd "${BATS_TEST_DIRNAME}/../../../.." && pwd)"
  PLUGIN_ROOT="$REPO_ROOT/plugins/vsdd-factory"
  DISPATCHER="$REPO_ROOT/target/release/factory-dispatcher"
  WASM_PLUGIN="$PLUGIN_ROOT/hook-plugins/validate-closes-completeness.wasm"
  FIXTURE_SRC="${BATS_TEST_DIRNAME}/../fixtures/validate-closes-completeness/fail-multiple-violations"
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
  printf '{"event_name":"PostToolUse","tool_name":"Edit","session_id":"fail-multiple-violations","tool_input":{"file_path":".factory/cycles/v1.0-brownfield-backfill/lessons.md","content":""},"tool_response":{"exit_code":0,"stdout":"","stderr":""}}'
}

# ---------------------------------------------------------------------------
# AC-15: multiple violations => dispatcher exits 2 (single block, not multiple)
# Traces to BC-5.39.007 postcondition 9
# ---------------------------------------------------------------------------

@test "AC-15 FAIL: hook blocks with single BlockWithFix when lessons.md has multiple violations" {
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
# AC-15: block message names all three violation types
#   (D-448 empty Closes, D-419 malformed cite, D-441 bare umbrella cite)
# Traces to BC-5.39.007 postcondition 9 (enumeration requirement)
# ---------------------------------------------------------------------------

@test "AC-15 FAIL: single block message enumerates all three violations (D-448, D-419, D-441)" {
  _require_artifacts
  _setup_fixture
  _write_registry
  cp "$WASM_PLUGIN" "$WORK/hook-plugins/"

  local envelope
  envelope="$(_lessons_md_envelope)"
  run bash -c "printf '%s' '$envelope' | CLAUDE_PLUGIN_ROOT='$WORK' CLAUDE_PROJECT_DIR='$WORK' '$DISPATCHER' 2>&1 >/dev/null"

  [ "$status" -eq 2 ]

  # Block reason must reference all three violation types
  [[ "$output" == *"D-448"* ]]
  [[ "$output" == *"D-419"* ]]
  [[ "$output" == *"D-441"* ]]
}
