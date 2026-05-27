#!/usr/bin/env bats
# pass-p2-adversary-unreadable.bats — AC-2: when pointer file is present (integer 15) but
#                                       the derived adversary review file path does not exist
#                                       in the sandbox, hook is fail-open (Continue + advisory)
#
# Traces to:
#   BC-5.39.007 Phase 2 P2-2 postcondition (adversary file unreadable => Continue + advisory)
#   BC-5.39.007 Phase 2 invariant 1 (fail-open at every step)
#   BC-5.39.007 invariant 9 (all host::read_file errors are fail-open)
#
# Fixture: pass-p2-adversary-unreadable/ — STATE.md + pointer file (integer 15);
#          NO .factory/cycles/v1.0-brownfield-backfill/adv-cycle-pass-15.md
# Expected: hook exits 0 (Continue); no blocking_plugins= in output
#
# RED GATE PHASE: @pending — Phase 2 logic not yet implemented. All tests in this file
# are marked @pending and will fail until the implementer adds Phase 2 code.

setup() {
  REPO_ROOT="$(cd "${BATS_TEST_DIRNAME}/../../../.." && pwd)"
  PLUGIN_ROOT="$REPO_ROOT/plugins/vsdd-factory"
  DISPATCHER="$REPO_ROOT/target/release/factory-dispatcher"
  WASM_PLUGIN="$PLUGIN_ROOT/hook-plugins/validate-closes-completeness.wasm"
  FIXTURE_SRC="${BATS_TEST_DIRNAME}/../fixtures/validate-closes-completeness/pass-p2-adversary-unreadable"
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
  printf '{"event_name":"PostToolUse","tool_name":"Edit","session_id":"pass-p2-adversary-unreadable","tool_input":{"file_path":".factory/STATE.md","content":""},"tool_response":{"exit_code":0,"stdout":"","stderr":""}}'
}

# ---------------------------------------------------------------------------
# AC-2: pointer present, adversary file absent => exits 0 (fail-open)
# Traces to BC-5.39.007 Phase 2 P2-2 + invariant 9
# ---------------------------------------------------------------------------

@test "AC-2 PASS: hook emits Continue when pointer file present but adversary review file is unreadable (fail-open)" {
  @pending
  _require_artifacts
  _setup_fixture
  _write_registry
  cp "$WASM_PLUGIN" "$WORK/hook-plugins/"

  # Pointer file present (integer 15)
  [ -f "$WORK/.factory/current-adversary-pass.txt" ]
  # Adversary review file must NOT exist in the sandbox
  [ ! -f "$WORK/.factory/cycles/v1.0-brownfield-backfill/adv-cycle-pass-15.md" ]

  local envelope
  envelope="$(_state_md_envelope)"
  run bash -c "printf '%s' '$envelope' | CLAUDE_PLUGIN_ROOT='$WORK' CLAUDE_PROJECT_DIR='$WORK' '$DISPATCHER' 2>&1 >/dev/null"

  # Exit 0: fail-open when adversary file is unreadable
  [ "$status" -eq 0 ]

  # No blocking_plugins= — advisory only, not a block
  [[ "$output" != *"blocking_plugins="* ]]
}

# ---------------------------------------------------------------------------
# AC-2 supplemental: no block message when adversary file absent
# (each host::read_file error is independently fail-open per invariant 9)
# ---------------------------------------------------------------------------

@test "AC-2 PASS: no block message when adversary review file is absent" {
  @pending
  _require_artifacts
  _setup_fixture
  _write_registry
  cp "$WASM_PLUGIN" "$WORK/hook-plugins/"

  local envelope
  envelope="$(_state_md_envelope)"
  run bash -c "printf '%s' '$envelope' | CLAUDE_PLUGIN_ROOT='$WORK' CLAUDE_PROJECT_DIR='$WORK' '$DISPATCHER' 2>&1 >/dev/null"

  [ "$status" -eq 0 ]
  [[ "$output" != *"block_reason="* ]]
}
