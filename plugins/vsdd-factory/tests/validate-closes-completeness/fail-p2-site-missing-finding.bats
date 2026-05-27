#!/usr/bin/env bats
# fail-p2-site-missing-finding.bats — AC-4: hook blocks when a citation site (STATE.md
#                                       Decisions Log row Closes) is missing a finding ID
#                                       from the canonical set; block message names the
#                                       site and missing ID, cites D-411(c)
#
# Traces to:
#   BC-5.39.007 Phase 2 P2-3 postcondition (site missing finding => BlockWithFix citing D-411(c))
#   D-411(c) (closure-set completeness lint: each site must enumerate all findings)
#   D-413(b) (full 8-site Closes cross-validation)
#
# Fixture: fail-p2-site-missing-finding/ — pointer (integer 15);
#          adv-cycle-pass-15.md with 3 findings (F-P15-001, F-P15-002, F-P15-003);
#          STATE.md Decisions Log row Closes cites only F-P15-001 and F-P15-002 (missing F-P15-003)
# Expected: hook exits 2 (block); block_reason names STATE.md and missing F-P15-003; cites D-411(c)
#
# RED GATE PHASE: @pending — Phase 2 logic not yet implemented. All tests in this file
# are marked @pending and will fail until the implementer adds Phase 2 code.

setup() {
  REPO_ROOT="$(cd "${BATS_TEST_DIRNAME}/../../../.." && pwd)"
  PLUGIN_ROOT="$REPO_ROOT/plugins/vsdd-factory"
  DISPATCHER="$REPO_ROOT/target/release/factory-dispatcher"
  WASM_PLUGIN="$PLUGIN_ROOT/hook-plugins/validate-closes-completeness.wasm"
  FIXTURE_SRC="${BATS_TEST_DIRNAME}/../fixtures/validate-closes-completeness/fail-p2-site-missing-finding"
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
  printf '{"event_name":"PostToolUse","tool_name":"Edit","session_id":"fail-p2-site-missing-finding","tool_input":{"file_path":".factory/STATE.md","content":""},"tool_response":{"exit_code":0,"stdout":"","stderr":""}}'
}

# ---------------------------------------------------------------------------
# AC-4: STATE.md missing F-P15-003 from Closes => exits 2 (block)
# Traces to BC-5.39.007 Phase 2 P2-3 postcondition + D-411(c)
# ---------------------------------------------------------------------------

@test "AC-4 FAIL: hook blocks when STATE.md Decisions Log row Closes is missing F-P15-003 from canonical finding set" {
  _require_artifacts
  _setup_fixture
  _write_registry
  cp "$WASM_PLUGIN" "$WORK/hook-plugins/"

  local envelope
  envelope="$(_state_md_envelope)"
  run bash -c "printf '%s' '$envelope' | CLAUDE_PLUGIN_ROOT='$WORK' CLAUDE_PROJECT_DIR='$WORK' '$DISPATCHER' 2>&1 >/dev/null"

  # Exit 2: block signal emitted for missing finding
  [ "$status" -eq 2 ]

  # blocking_plugins= names this hook
  [[ "$output" == *"blocking_plugins=validate-closes-completeness"* ]]
}

# ---------------------------------------------------------------------------
# AC-4: block message names missing finding F-P15-003
# Traces to BC-5.39.007 Phase 2 P2-3 (block message names site + missing IDs)
# ---------------------------------------------------------------------------

@test "AC-4 FAIL: block message names missing finding F-P15-003" {
  _require_artifacts
  _setup_fixture
  _write_registry
  cp "$WASM_PLUGIN" "$WORK/hook-plugins/"

  local envelope
  envelope="$(_state_md_envelope)"
  run bash -c "printf '%s' '$envelope' | CLAUDE_PLUGIN_ROOT='$WORK' CLAUDE_PROJECT_DIR='$WORK' '$DISPATCHER' 2>&1 >/dev/null"

  [ "$status" -eq 2 ]
  [[ "$output" == *"F-P15-003"* ]]
}

# ---------------------------------------------------------------------------
# AC-4: block message cites D-411(c) as the governing decision
# ---------------------------------------------------------------------------

@test "AC-4 FAIL: block message cites D-411 for missing finding at citation site" {
  _require_artifacts
  _setup_fixture
  _write_registry
  cp "$WASM_PLUGIN" "$WORK/hook-plugins/"

  local envelope
  envelope="$(_state_md_envelope)"
  run bash -c "printf '%s' '$envelope' | CLAUDE_PLUGIN_ROOT='$WORK' CLAUDE_PROJECT_DIR='$WORK' '$DISPATCHER' 2>&1 >/dev/null"

  [ "$status" -eq 2 ]
  [[ "$output" == *"D-411"* ]]
}
