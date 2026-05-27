#!/usr/bin/env bats
# fail-p2-pointer-invalid-integer.bats — AC-1b: when .factory/current-adversary-pass.txt
#                                          is present but its content is not a valid positive
#                                          integer (e.g., "abc"), hook emits a hard block
#                                          with parse-error message directing developer to
#                                          run state-manager update-pass-pointer
#
# Traces to:
#   BC-5.39.007 Phase 2 P2-1 postcondition (parse-error path is a hard block, NOT fail-open)
#   ADR-022 parse-error handling ("If the file exists but content is not a valid positive
#           integer, the hook emits a hard block with a clear error message")
#
# Fixture: fail-p2-pointer-invalid-integer/ — STATE.md present;
#          .factory/current-adversary-pass.txt contains "abc" (non-integer)
# Expected: hook exits 2 (block); block_reason references the invalid content
#
# RED GATE PHASE: @pending — Phase 2 logic not yet implemented. All tests in this file
# are marked @pending and will fail until the implementer adds Phase 2 code.

setup() {
  REPO_ROOT="$(cd "${BATS_TEST_DIRNAME}/../../../.." && pwd)"
  PLUGIN_ROOT="$REPO_ROOT/plugins/vsdd-factory"
  DISPATCHER="$REPO_ROOT/target/release/factory-dispatcher"
  WASM_PLUGIN="$PLUGIN_ROOT/hook-plugins/validate-closes-completeness.wasm"
  FIXTURE_SRC="${BATS_TEST_DIRNAME}/../fixtures/validate-closes-completeness/fail-p2-pointer-invalid-integer"
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
  ".factory/current-adversary-pass.txt",
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
    skip "validate-closes-completeness.wasm not built -- implement T-4 through T-7 of S-15.13"
  fi
}

_state_md_envelope() {
  printf '{"event_name":"PostToolUse","tool_name":"Edit","session_id":"fail-p2-pointer-invalid-integer","tool_input":{"file_path":".factory/STATE.md","content":""},"tool_response":{"exit_code":0,"stdout":"","stderr":""}}'
}

# ---------------------------------------------------------------------------
# AC-1b: non-integer pointer file => exits 2 (hard block, NOT fail-open)
# Traces to BC-5.39.007 Phase 2 P2-1 parse-error path + ADR-022 parse-error handling
# CRITICAL: a corrupt pointer file must NOT be fail-open; it must be a hard block
# ---------------------------------------------------------------------------

@test "AC-1b FAIL: hook emits block (exits 2) when current-adversary-pass.txt contains non-integer content" {
  @pending
  _require_artifacts
  _setup_fixture
  _write_registry
  cp "$WASM_PLUGIN" "$WORK/hook-plugins/"

  # Verify the fixture has the invalid pointer file.
  [ -f "$WORK/.factory/current-adversary-pass.txt" ]

  local envelope
  envelope="$(_state_md_envelope)"
  run bash -c "printf '%s' '$envelope' | CLAUDE_PLUGIN_ROOT='$WORK' CLAUDE_PROJECT_DIR='$WORK' '$DISPATCHER' 2>&1 >/dev/null"

  # Exit 2: hard block on corrupt pointer file
  [ "$status" -eq 2 ]

  # blocking_plugins= names this hook
  [[ "$output" == *"blocking_plugins=validate-closes-completeness"* ]]
}

# ---------------------------------------------------------------------------
# AC-1b: block message references the invalid content and directs to fix
# Traces to ADR-022 parse-error handling — message must direct to update-pass-pointer
# ---------------------------------------------------------------------------

@test "AC-1b FAIL: block message references invalid pointer content and parse-error fix" {
  @pending
  _require_artifacts
  _setup_fixture
  _write_registry
  cp "$WASM_PLUGIN" "$WORK/hook-plugins/"

  local envelope
  envelope="$(_state_md_envelope)"
  run bash -c "printf '%s' '$envelope' | CLAUDE_PLUGIN_ROOT='$WORK' CLAUDE_PROJECT_DIR='$WORK' '$DISPATCHER' 2>&1 >/dev/null"

  [ "$status" -eq 2 ]
  # Block message must mention the invalid content or parse-error context
  [[ "$output" == *"update-pass-pointer"* ]] || [[ "$output" == *"not a valid positive integer"* ]]
}
