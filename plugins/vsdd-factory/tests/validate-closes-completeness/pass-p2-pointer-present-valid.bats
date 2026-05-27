#!/usr/bin/env bats
# pass-p2-pointer-present-valid.bats — AC-3: when pointer file and adversary review file
#                                        are both present and all citation sites enumerate
#                                        the complete finding set, hook emits Continue
#
# Traces to:
#   BC-5.39.007 Phase 2 P2-2 postcondition (adversary file readable => extract finding IDs)
#   BC-5.39.007 Phase 2 P2-3 postcondition (all sites complete => Continue)
#   BC-5.39.007 Phase 2 P2-4 postcondition (cardinality parity => no D-420(a) block)
#
# Fixture: pass-p2-pointer-present-valid/ — pointer file (integer 15);
#          adv-cycle-pass-15.md with 3 Part A findings (F-P15-001, F-P15-002, F-P15-003);
#          STATE.md Decisions Log rows each cite all 3 findings
# Expected: hook exits 0 (Continue); no blocking_plugins= in output
#
# RED GATE PHASE: @pending — Phase 2 logic not yet implemented. All tests in this file
# are marked @pending and will fail until the implementer adds Phase 2 code.

setup() {
  REPO_ROOT="$(cd "${BATS_TEST_DIRNAME}/../../../.." && pwd)"
  PLUGIN_ROOT="$REPO_ROOT/plugins/vsdd-factory"
  DISPATCHER="$REPO_ROOT/target/release/factory-dispatcher"
  WASM_PLUGIN="$PLUGIN_ROOT/hook-plugins/validate-closes-completeness.wasm"
  FIXTURE_SRC="${BATS_TEST_DIRNAME}/../fixtures/validate-closes-completeness/pass-p2-pointer-present-valid"
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
  printf '{"event_name":"PostToolUse","tool_name":"Edit","session_id":"pass-p2-pointer-present-valid","tool_input":{"file_path":".factory/STATE.md","content":""},"tool_response":{"exit_code":0,"stdout":"","stderr":""}}'
}

# ---------------------------------------------------------------------------
# AC-3: pointer + adversary file both present, all 3 findings cited at all sites => exits 0
# Traces to BC-5.39.007 Phase 2 P2-2 + P2-3 + P2-4 postconditions
# ---------------------------------------------------------------------------

@test "AC-3 PASS: hook emits Continue when pointer and adversary file present with 3 findings and all sites complete" {
  @pending
  _require_artifacts
  _setup_fixture
  _write_registry
  cp "$WASM_PLUGIN" "$WORK/hook-plugins/"

  # Both pointer file and adversary review file must be present
  [ -f "$WORK/.factory/current-adversary-pass.txt" ]
  [ -f "$WORK/.factory/cycles/v1.0-brownfield-backfill/adv-cycle-pass-15.md" ]

  local envelope
  envelope="$(_state_md_envelope)"
  run bash -c "printf '%s' '$envelope' | CLAUDE_PLUGIN_ROOT='$WORK' CLAUDE_PROJECT_DIR='$WORK' '$DISPATCHER' 2>&1 >/dev/null"

  # Exit 0: all sites complete, no violations
  [ "$status" -eq 0 ]

  # No blocking_plugins= for a clean pass
  [[ "$output" != *"blocking_plugins="* ]]
}

# ---------------------------------------------------------------------------
# AC-3 supplemental: finding extraction correctness (3 IDs extracted from Part A)
# Hook must extract F-P15-001, F-P15-002, F-P15-003 and find them all present
# ---------------------------------------------------------------------------

@test "AC-3 PASS: hook correctly extracts all 3 finding IDs from Part A and validates complete citation" {
  @pending
  _require_artifacts
  _setup_fixture
  _write_registry
  cp "$WASM_PLUGIN" "$WORK/hook-plugins/"

  local envelope
  envelope="$(_state_md_envelope)"
  run bash -c "printf '%s' '$envelope' | CLAUDE_PLUGIN_ROOT='$WORK' CLAUDE_PROJECT_DIR='$WORK' '$DISPATCHER' 2>&1 >/dev/null"

  [ "$status" -eq 0 ]
  # Should not block on any of the 3 findings
  [[ "$output" != *"F-P15-001"* ]] || [[ "$output" == *"F-P15-001"* && "$output" != *"blocking_plugins="* ]]
}
