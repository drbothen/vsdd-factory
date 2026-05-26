#!/usr/bin/env bats
# fail-exemption-not-declared.bats — AC-14: hook blocks when lessons.md entry silently
#                                     omits **Closes:** with no exemption declaration
#
# Traces to:
#   BC-5.39.007 EC-013 (silent omission without exemption declaration => BlockWithFix)
#   BC-5.39.007 invariant 7 (silent omission is NOT valid — exemption must be explicit)
#   BC-5.39.007 invariant 3 (path-component-strict guard)
# D-NNN closure: D-443(b)+D-448(b)
#
# Fixture: lessons.md entry with no **Closes:** line and no exemption declaration.
# Expected: hook exits 2 (block) and block_reason cites D-448.
#
# RED GATE PHASE: test skips because validate-closes-completeness.wasm is not yet compiled.

setup() {
  REPO_ROOT="$(cd "${BATS_TEST_DIRNAME}/../../../.." && pwd)"
  PLUGIN_ROOT="$REPO_ROOT/plugins/vsdd-factory"
  DISPATCHER="$REPO_ROOT/target/release/factory-dispatcher"
  WASM_PLUGIN="$PLUGIN_ROOT/hook-plugins/validate-closes-completeness.wasm"
  FIXTURE_SRC="${BATS_TEST_DIRNAME}/../fixtures/validate-closes-completeness/fail-exemption-not-declared"
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
  printf '{"event_name":"PostToolUse","tool_name":"Edit","session_id":"fail-exemption-not-declared","tool_input":{"file_path":".factory/cycles/v1.0-brownfield-backfill/lessons.md","content":""},"tool_response":{"exit_code":0,"stdout":"","stderr":""}}'
}

# ---------------------------------------------------------------------------
# AC-14: silent omission of **Closes:** with no exemption => exits 2 (block)
# Traces to BC-5.39.007 EC-013 / invariant 7
# ---------------------------------------------------------------------------

@test "AC-14 FAIL: hook blocks when lessons.md entry silently omits **Closes:** with no exemption declaration" {
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
# AC-14: block message cites D-448 (missing Closes) or D-443 (undeclared exemption)
# ---------------------------------------------------------------------------

@test "AC-14 FAIL: block message cites D-448 for silent omission without exemption" {
  _require_artifacts
  _setup_fixture
  _write_registry
  cp "$WASM_PLUGIN" "$WORK/hook-plugins/"

  local envelope
  envelope="$(_lessons_md_envelope)"
  run bash -c "printf '%s' '$envelope' | CLAUDE_PLUGIN_ROOT='$WORK' CLAUDE_PROJECT_DIR='$WORK' '$DISPATCHER' 2>&1 >/dev/null"

  [ "$status" -eq 2 ]
  [[ "$output" == *"D-448"* ]]
}
