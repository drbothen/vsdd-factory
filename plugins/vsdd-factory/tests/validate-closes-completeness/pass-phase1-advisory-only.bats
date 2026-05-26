#!/usr/bin/env bats
# pass-phase1-advisory-only.bats — AC-18: cross-site staleness (cite "D-999" with correct
#                                    format, nonexistent in decision-log) produces advisory
#                                    log only — NOT a block (Phase 1 boundary)
#
# Traces to:
#   BC-5.39.007 postcondition 10 (cross-site staleness => advisory log only, NOT block)
#   BC-5.39.007 EC-010 (Phase 1/2 boundary: staleness detection deferred to Phase 2)
#   BC-5.39.007 invariant 3 (path-component-strict guard)
#
# CRITICAL BOUNDARY: D-999 is a correctly-formatted cite ID matching the D-\d+ pattern.
# Phase 1 does NOT block on correctly-formatted IDs that reference nonexistent decisions.
# Cross-site staleness detection is Phase 2 scope per BC-5.39.007 Phase 1/2 boundary table.
#
# Fixture: lessons.md entry with "**Closes:** D-999" (correct format, nonexistent D-NNN).
# Expected: hook exits 0 (no block) — advisory log_warn only.
#
# RED GATE PHASE: test skips because validate-closes-completeness.wasm is not yet compiled.

setup() {
  REPO_ROOT="$(cd "${BATS_TEST_DIRNAME}/../../../.." && pwd)"
  PLUGIN_ROOT="$REPO_ROOT/plugins/vsdd-factory"
  DISPATCHER="$REPO_ROOT/target/release/factory-dispatcher"
  WASM_PLUGIN="$PLUGIN_ROOT/hook-plugins/validate-closes-completeness.wasm"
  FIXTURE_SRC="${BATS_TEST_DIRNAME}/../fixtures/validate-closes-completeness/pass-phase1-advisory-only"
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
  printf '{"event_name":"PostToolUse","tool_name":"Edit","session_id":"pass-phase1-advisory-only","tool_input":{"file_path":".factory/cycles/v1.0-brownfield-backfill/lessons.md","content":""},"tool_response":{"exit_code":0,"stdout":"","stderr":""}}'
}

# ---------------------------------------------------------------------------
# AC-18: correctly-formatted cite D-999 (nonexistent) => exits 0 (advisory only, NOT block)
# Traces to BC-5.39.007 postcondition 10 / EC-010
# CRITICAL: if this test fails (exits 2), the Phase 1/2 boundary is broken.
# ---------------------------------------------------------------------------

@test "AC-18 PASS: hook emits Continue for correctly-formatted D-999 cite (cross-site staleness is advisory only in Phase 1)" {
  _require_artifacts
  _setup_fixture
  _write_registry
  cp "$WASM_PLUGIN" "$WORK/hook-plugins/"

  local envelope
  envelope="$(_lessons_md_envelope)"
  run bash -c "printf '%s' '$envelope' | CLAUDE_PLUGIN_ROOT='$WORK' CLAUDE_PROJECT_DIR='$WORK' '$DISPATCHER' 2>&1 >/dev/null"

  # Exit 0: correctly-formatted ID must NOT block in Phase 1
  [ "$status" -eq 0 ]

  # No blocking_plugins= — advisory only, not a block
  [[ "$output" != *"blocking_plugins="* ]]
}
