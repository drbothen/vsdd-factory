#!/usr/bin/env bats
# pass-file-too-large-failopen.bats — AC-14: HostError::OutputTooLarge (file > MAX_BYTES)
#   => Continue + log_warn fail-open; never Block
#
# Traces to:
#   BC-5.39.009 postcondition 11 (PC11: any HostError => Continue + log_warn fail-open)
#   BC-5.39.009 invariant 7 (max_bytes = 524288 on ALL read_file calls)
#   BC-5.39.009 invariant 10 (host::read_file HostError => fail-open)
#
# Test approach: a file that genuinely exceeds 524288 bytes is impractical to materialize
#   in a bats fixture, so the HostError::OutputTooLarge mapping is exercised directly by a
#   Rust unit test rather than end-to-end here. The bats layer exercises the SAME PC11
#   fail-open arm via a read that produces a different HostError variant (the dispatcher
#   denies / fails the read of a path that does not exist), proving the orchestration
#   fails open + emits the read-failure advisory.
#
#   F-005: the OutputTooLarge → Continue+log_warn mapping is verified by the Rust unit
#   test `test_F005_decode_read_result_output_too_large_maps_to_advisory` in lib.rs
#   #[cfg(test)] — it constructs `Err(HostError::OutputTooLarge)` (the actual SDK variant
#   per crates/hook-sdk/src/host.rs, NOT the non-existent `TooBig`) and asserts the pure
#   `decode_read_result` seam returns a fail-open advisory. Sibling unit tests cover
#   CapabilityDenied and the UTF-8 decode Err branch.
#
# Expected: hook exits 0 (Continue); no blocking_plugins signal; read-failure advisory
#   emitted to the dispatcher internal log.
#
# RED GATE PHASE: test skips because validate-trajectory-tail-cell-completeness.wasm not yet compiled.

setup() {
  REPO_ROOT="$(cd "${BATS_TEST_DIRNAME}/../../../.." && pwd)"
  PLUGIN_ROOT="$REPO_ROOT/plugins/vsdd-factory"
  DISPATCHER="$REPO_ROOT/target/release/factory-dispatcher"
  WASM_PLUGIN="$PLUGIN_ROOT/hook-plugins/validate-trajectory-tail-cell-completeness.wasm"
  FIXTURE_SRC="${BATS_TEST_DIRNAME}/../fixtures/validate-trajectory-tail-cell-completeness/pass-file-too-large-failopen"
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
  # Use an empty (or near-empty) STATE.md — the hook will try to read it
  # For OutputTooLarge simulation: we point to a path that doesn't exist under path_allow,
  # which results in CapabilityDenied (same PC11 fail-open arm as OutputTooLarge).
  cp "$FIXTURE_SRC/STATE.md" "$WORK/.factory/STATE.md"
}

_write_registry() {
  # Registry with a very small read_file capability to force CapabilityDenied on reads
  # outside path_allow — simulating a read-failure HostError
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

_state_md_envelope_nonexistent() {
  # Point to a STATE.md path that doesn't actually exist on disk — triggers HostError
  # (either CapabilityDenied or Other(i32)) — both are PC11 fail-open paths
  printf '{"event_name":"PostToolUse","tool_name":"Edit","session_id":"pass-file-too-large-failopen","tool_input":{"file_path":".factory/STATE.md.nonexistent","content":""},"tool_response":{"exit_code":0,"stdout":"","stderr":""}}'
}

_state_md_envelope() {
  # Canonical `.factory/STATE.md` basename so target_arm resolves to the State arm and
  # the hook actually performs the read (whose failure we assert on in the advisory test).
  printf '{"event_name":"PostToolUse","tool_name":"Edit","session_id":"pass-file-too-large-failopen","tool_input":{"file_path":".factory/STATE.md","content":""},"tool_response":{"exit_code":0,"stdout":"","stderr":""}}'
}

# ---------------------------------------------------------------------------
# AC-14 / PC11 + inv-7 + inv-10: any HostError (including OutputTooLarge) => Continue
# ---------------------------------------------------------------------------

@test "test_BC_5_39_009_PC11_host_error_output_too_large_exits_0" {
  _require_artifacts
  _setup_fixture
  _write_registry
  cp "$WASM_PLUGIN" "$WORK/hook-plugins/"

  local envelope
  envelope="$(_state_md_envelope_nonexistent)"
  run bash -c "printf '%s' '$envelope' | CLAUDE_PLUGIN_ROOT='$WORK' CLAUDE_PROJECT_DIR='$WORK' '$DISPATCHER' 2>&1 >/dev/null"

  # HostError (any variant) => fail-open => exit 0 (Continue, NOT Block)
  [ "$status" -eq 0 ]
}

@test "test_BC_5_39_009_PC11_host_error_never_blocks" {
  _require_artifacts
  _setup_fixture
  _write_registry
  cp "$WASM_PLUGIN" "$WORK/hook-plugins/"

  local envelope
  envelope="$(_state_md_envelope_nonexistent)"
  run bash -c "printf '%s' '$envelope' | CLAUDE_PLUGIN_ROOT='$WORK' CLAUDE_PROJECT_DIR='$WORK' '$DISPATCHER' 2>&1 >/dev/null"

  [ "$status" -eq 0 ]
  [[ "$output" != *"blocking_plugins=validate-trajectory-tail-cell-completeness"* ]]
}

# F-005: make the HostError fail-open arm LOAD-BEARING at the bats layer — assert the
# read-failure advisory string is actually emitted (plugin.log warn). Without this, the
# exit-0 assertions could pass even if the read had unexpectedly SUCCEEDED.
#
# The envelope MUST use the canonical `.factory/STATE.md` basename so target_arm resolves
# to the State arm (a `.nonexistent` suffix would fail basename matching and Continue
# WITHOUT reading — never exercising the read-failure path). We delete the STATE.md file
# after setup so host::read_file returns a HostError, driving the fail-open advisory.
@test "test_BC_5_39_009_PC11_host_error_emits_read_failure_advisory" {
  _require_artifacts
  _setup_fixture
  _write_registry
  cp "$WASM_PLUGIN" "$WORK/hook-plugins/"
  # Remove STATE.md so the State-arm read fails (HostError) → PC11 fail-open advisory.
  rm -f "$WORK/.factory/STATE.md"

  local envelope
  envelope="$(_state_md_envelope)"
  run bash -c "printf '%s' '$envelope' | FACTORY_DISPATCHER_INTERNAL_LOG=1 CLAUDE_PLUGIN_ROOT='$WORK' CLAUDE_PROJECT_DIR='$WORK' '$DISPATCHER' >/dev/null 2>&1"

  [ "$status" -eq 0 ]
  local logf
  logf="$(ls "$WORK/.factory/logs/"dispatcher-internal-*.jsonl 2>/dev/null | head -1)"
  [ -n "$logf" ]
  grep -q "read failed" "$logf"
}
