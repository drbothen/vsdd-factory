#!/usr/bin/env bats
# pass-utf8-decode-failure-failopen.bats — AC-24: host::read_file returns Ok(bytes) for a
#   target file where bytes is not valid UTF-8 => String::from_utf8 decode fails =>
#   Continue + log_warn fail-open; never Block
#
# Traces to:
#   BC-5.39.009 PC11 (uniform HostError fail-open + invariant 10)
#   BC-5.39.009 invariant 10 (HostError / decode error => fail-open)
#   BC-5.39.009 EC-020 (story-local; UTF-8 decode failure => fail-open)
#   BC-5.39.009 invariant 13 (inv-13 encoding gate: UTF-8 failure before extractors)
#
# Fixture: STATE.md fixture with non-UTF-8 bytes injected (see fixture file — binary content)
# Expected: hook exits 0 (Continue); no blocking_plugins signal
#   (String::from_utf8 fails => EC-020 fail-open before any extractor runs)
#
# RED GATE PHASE: test skips because validate-trajectory-tail-cell-completeness.wasm not yet compiled.

setup() {
  REPO_ROOT="$(cd "${BATS_TEST_DIRNAME}/../../../.." && pwd)"
  PLUGIN_ROOT="$REPO_ROOT/plugins/vsdd-factory"
  DISPATCHER="$REPO_ROOT/target/release/factory-dispatcher"
  WASM_PLUGIN="$PLUGIN_ROOT/hook-plugins/validate-trajectory-tail-cell-completeness.wasm"
  FIXTURE_SRC="${BATS_TEST_DIRNAME}/../fixtures/validate-trajectory-tail-cell-completeness/pass-utf8-decode-failure-failopen"
  WORK="$(mktemp -d)"
  mkdir -p "$WORK/hook-plugins"
  mkdir -p "$WORK/.factory/logs"
  export VSDD_LOG_DIR="$WORK/.factory/logs"
}

teardown() {
  [ -n "${WORK:-}" ] && [ -d "$WORK" ] && find "$WORK" -type f -delete && find "$WORK" -type d -mindepth 1 | sort -r | xargs rmdir 2>/dev/null && rmdir "$WORK" 2>/dev/null || true
}

_setup_fixture() {
  mkdir -p "$WORK/.factory"
  # F-004: GENERATE genuinely non-UTF-8 bytes in setup rather than relying on a
  # committed file that an editor could silently re-encode to valid UTF-8. 0xFF / 0xFE
  # are never valid UTF-8 lead bytes, guaranteeing String::from_utf8 takes the Err branch.
  printf '%s' '---' > "$WORK/.factory/STATE.md"
  printf '\n' >> "$WORK/.factory/STATE.md"
  printf 'document_type: state\n' >> "$WORK/.factory/STATE.md"
  printf 'current_cycle: "v1.0-brownfield-backfill"\n' >> "$WORK/.factory/STATE.md"
  printf '%s\n' '---' >> "$WORK/.factory/STATE.md"
  printf '\xff\xfe\x80 invalid UTF-8 bytes here\n' >> "$WORK/.factory/STATE.md"
}

_write_registry() {
  cat > "$WORK/hooks-registry.toml" << 'TOML'
schema_version = 2

[[hooks]]
name = "validate-trajectory-tail-cell-completeness"
event = "PostToolUse"
tool = "Edit|Write"
plugin = "hook-plugins/validate-trajectory-tail-cell-completeness.wasm"
priority = 158
timeout_ms = 5000
on_error = "continue"

[hooks.capabilities.read_file]
path_allow = [".factory"]
TOML
}

_require_artifacts() {
  if [ ! -x "$DISPATCHER" ]; then
    skip "dispatcher binary not built -- run: cargo build --release -p factory-dispatcher"
  fi
  if [ ! -f "$WASM_PLUGIN" ]; then
    skip "validate-trajectory-tail-cell-completeness.wasm not built -- implement T-5 through T-8 of S-15.17"
  fi
}

_state_md_envelope() {
  printf '{"event_name":"PostToolUse","tool_name":"Edit","session_id":"pass-utf8-decode-failure-failopen","tool_input":{"file_path":".factory/STATE.md","content":""},"tool_response":{"exit_code":0,"stdout":"","stderr":""}}'
}

# ---------------------------------------------------------------------------
# AC-24 / EC-020 / inv-13: non-UTF-8 bytes => String::from_utf8 fails => Continue
# ---------------------------------------------------------------------------

@test "test_BC_5_39_009_EC020_utf8_decode_failure_exits_0" {
  _require_artifacts
  _setup_fixture
  _write_registry
  cp "$WASM_PLUGIN" "$WORK/hook-plugins/"

  local envelope
  envelope="$(_state_md_envelope)"
  run bash -c "printf '%s' '$envelope' | CLAUDE_PLUGIN_ROOT='$WORK' CLAUDE_PROJECT_DIR='$WORK' '$DISPATCHER' 2>&1 >/dev/null"

  # String::from_utf8 failure => EC-020 fail-open => exit 0 (Continue)
  [ "$status" -eq 0 ]
}

@test "test_BC_5_39_009_EC020_utf8_decode_failure_never_blocks" {
  _require_artifacts
  _setup_fixture
  _write_registry
  cp "$WASM_PLUGIN" "$WORK/hook-plugins/"

  local envelope
  envelope="$(_state_md_envelope)"
  run bash -c "printf '%s' '$envelope' | CLAUDE_PLUGIN_ROOT='$WORK' CLAUDE_PROJECT_DIR='$WORK' '$DISPATCHER' 2>&1 >/dev/null"

  [ "$status" -eq 0 ]
  [[ "$output" != *"blocking_plugins=validate-trajectory-tail-cell-completeness"* ]]
}

# F-004: make AC-24/EC-020 LOAD-BEARING — prove the String::from_utf8 Err branch actually
# ran by asserting the "invalid UTF-8" advisory is emitted (plugin.log warn). Without
# this, the hook could have failed open on a read error (a DIFFERENT branch) and the
# exit-0 assertions above would still pass.
@test "test_BC_5_39_009_EC020_utf8_decode_failure_emits_invalid_utf8_advisory" {
  _require_artifacts
  _setup_fixture
  _write_registry
  cp "$WASM_PLUGIN" "$WORK/hook-plugins/"

  local envelope
  envelope="$(_state_md_envelope)"
  run bash -c "printf '%s' '$envelope' | FACTORY_DISPATCHER_INTERNAL_LOG=1 CLAUDE_PLUGIN_ROOT='$WORK' CLAUDE_PROJECT_DIR='$WORK' '$DISPATCHER' >/dev/null 2>&1"

  [ "$status" -eq 0 ]
  local logf
  logf="$(ls "$WORK/.factory/logs/"dispatcher-internal-*.jsonl 2>/dev/null | head -1)"
  [ -n "$logf" ]
  # The UTF-8 decode Err branch emits an advisory containing "invalid UTF-8".
  grep -q "invalid UTF-8" "$logf"
  # And it must NOT be a Block.
  [[ "$output" != *"blocking_plugins=validate-trajectory-tail-cell-completeness"* ]]
}
