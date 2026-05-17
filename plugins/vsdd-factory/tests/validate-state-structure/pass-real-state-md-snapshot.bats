#!/usr/bin/env bats
# pass-real-state-md-snapshot.bats — F-P2-002: full-surface integration test against
#                                     a snapshot of the real .factory/STATE.md.
#
# Traces to:
#   BC-5.39.005 postcondition 1 (all properties hold => HookResult::Continue)
#   F-P2-001: tighter trajectory predicate (canonical-tail discriminator)
#   F-P2-002: real STATE.md exercises the FULL validator surface
#
# Fixture: an exact copy of .factory/STATE.md as of pass-2 fix-burst
#          (428 lines, wc-l claim matches, dual-margin present, trajectory tail
#          found via full-document fallback scan on table rows with adjacent →N→N).
# Expected: hook exits 0 (Continue — no block).
#
# This is the LOAD-BEARING bats evidence that F-P2-001 + F-P2-002 are closed:
# the real STATE.md banner contains `(363→310 lines)` (1 component, non-adjacent)
# and tracker lines with spread arrows — neither is a false-positive trajectory tail.

setup() {
  REPO_ROOT="$(cd "${BATS_TEST_DIRNAME}/../../../.." && pwd)"
  PLUGIN_ROOT="$REPO_ROOT/plugins/vsdd-factory"
  DISPATCHER="$REPO_ROOT/target/release/factory-dispatcher"
  WASM_PLUGIN="$PLUGIN_ROOT/hook-plugins/validate-state-structure.wasm"
  FIXTURE_SRC="${BATS_TEST_DIRNAME}/../fixtures/validate-state-structure/pass-real-state-md-snapshot"
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
    skip "validate-state-structure.wasm not built"
  fi
}

_state_md_envelope() {
  printf '{"event_name":"PostToolUse","tool_name":"Edit","session_id":"pass-real-state-md-snapshot","tool_input":{"file_path":".factory/STATE.md","content":""},"tool_response":{"exit_code":0,"stdout":"","stderr":""}}'
}

# ---------------------------------------------------------------------------
# F-P2-002: real STATE.md snapshot exercises full validator surface => Continue
# ---------------------------------------------------------------------------

@test "F-P2-002 PASS: real STATE.md snapshot passes full validator surface (no false-positive block)" {
  _require_artifacts
  _setup_fixture
  _write_registry
  cp "$WASM_PLUGIN" "$WORK/hook-plugins/"

  local envelope
  envelope="$(_state_md_envelope)"
  run bash -c "printf '%s' '$envelope' | CLAUDE_PLUGIN_ROOT='$WORK' CLAUDE_PROJECT_DIR='$WORK' '$DISPATCHER' 2>&1 >/dev/null"

  # Exit 0: no block signal emitted (Continue)
  [ "$status" -eq 0 ]

  # No blocking_plugins= for a clean pass
  [[ "$output" != *"blocking_plugins="* ]]
}

@test "F-P2-002 PASS: banner (363→310 lines) narrative arrow does NOT trigger trajectory-tail false positive" {
  _require_artifacts
  _setup_fixture
  _write_registry
  cp "$WASM_PLUGIN" "$WORK/hook-plugins/"

  local envelope
  envelope="$(_state_md_envelope)"
  run bash -c "printf '%s' '$envelope' | CLAUDE_PLUGIN_ROOT='$WORK' CLAUDE_PROJECT_DIR='$WORK' '$DISPATCHER' 2>&1 >/dev/null"

  # Must exit 0 — if the (363→310) line were picked up as trajectory tail (1 component),
  # the hook would exit 2 with "1 components; required LENGTH=4". Exit 0 proves it is not.
  [ "$status" -eq 0 ]

  # Specifically: no trajectory-tail violation in block_reason
  [[ "$output" != *"trajectory-tail has"* ]]
  [[ "$output" != *"trajectory-tail"*"components"* ]]
}
