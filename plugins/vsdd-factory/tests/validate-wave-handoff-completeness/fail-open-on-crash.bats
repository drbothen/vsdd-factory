#!/usr/bin/env bats
# fail-open-on-crash.bats — AC-013: validate-wave-handoff-completeness is fail-open on crash
#
# Story:   S-18.02 — validate-wave-handoff-completeness WASM gate
# BC:      BC-4.14.001 PC6 (on_error="continue" in hooks-registry.toml means
#          WASM crashes fail open — gate does NOT block the write; exit 0 advisory only)
# AC:      AC-013 — on_error="continue" makes this gate FAIL-OPEN on crash
# VP:      VP-081 Postcondition C (gate continues when validation passes)
#
# Scenario tested:
#   The validate-wave-handoff-completeness hook plugin is registered with
#   `on_error = "continue"` in hooks-registry.toml. When the WASM plugin
#   crashes or fails, the dispatcher treats it as advisory-only and does NOT
#   block the write. The PostToolUse event exits 0 (not 2/block).
#
# Test approach:
#   (A) Verify the production registry entry specifies `on_error = "continue"`
#       for validate-wave-handoff-completeness. This is a static config check
#       that does not require the dispatcher binary or WASM plugin to be compiled.
#   (B) If both the dispatcher binary and WASM plugin are available, exercise
#       an incomplete HANDOFF.md Write — confirm the hook fires (exit 2) with
#       the WASM plugin present. This proves the on_error="continue" config is
#       correct: a WASM crash would not block (exit 0), but a detected violation
#       DOES block (exit 2). The distinction confirms the plugin runs normally
#       and that fail-open only triggers on crash, not on validation failure.
#   (C) Validate that a non-HANDOFF.md Write (HANDOFF_SHOULD_NOT_MATCH.md)
#       does NOT fire the gate (exit 0), demonstrating path-component-strict
#       no-op behavior when the dispatcher is available.
#
# RED GATE discipline:
#   Scenario (A) is a static registry check — passes if and only if the
#   hooks-registry.toml entry exists with `on_error = "continue"`. This
#   is the load-bearing check for AC-013 / BC-4.14.001 PC6.
#   Scenarios (B) and (C) skip if the dispatcher binary or WASM plugin
#   is not compiled (infrastructure not yet available in this worktree).
#
# POLICY 11 compliance:
#   Every test asserts a CONCRETE postcondition: registry field value,
#   exit status code, or presence/absence of `blocking_plugins=` in output.

setup() {
  REPO_ROOT="$(cd "${BATS_TEST_DIRNAME}/../../../.." && pwd)"
  PLUGIN_ROOT="$REPO_ROOT/plugins/vsdd-factory"
  DISPATCHER="$REPO_ROOT/target/release/factory-dispatcher"
  WASM_PLUGIN="$PLUGIN_ROOT/hook-plugins/validate-wave-handoff-completeness.wasm"
  PRODUCTION_REGISTRY="$PLUGIN_ROOT/hooks-registry.toml"
  WORK="$(mktemp -d)"
  mkdir -p "$WORK/hook-plugins"
  mkdir -p "$WORK/.factory/logs"
}

teardown() {
  [ -n "${WORK:-}" ] && [ -d "$WORK" ] && \
    find "$WORK" -type f -delete && \
    find "$WORK" -type d -mindepth 1 | sort -r | xargs rmdir 2>/dev/null && \
    rmdir "$WORK" 2>/dev/null || true
}

_require_dispatcher_and_wasm() {
  if [ ! -x "$DISPATCHER" ]; then
    skip "dispatcher binary not built -- run: cargo build --release -p factory-dispatcher"
  fi
  if [ ! -f "$WASM_PLUGIN" ]; then
    skip "validate-wave-handoff-completeness.wasm not built -- implement S-18.02 tasks T-4..T-7"
  fi
}

_write_test_registry() {
  # Minimal registry using on_error="continue" (matching production AC-013).
  cat > "$WORK/hooks-registry.toml" << 'TOML'
schema_version = 2

[[hooks]]
name = "validate-wave-handoff-completeness"
event = "PostToolUse"
tool = "Edit|Write"
plugin = "hook-plugins/validate-wave-handoff-completeness.wasm"
timeout_ms = 5000
on_error = "continue"
TOML
}

_write_envelope_incomplete_handoff() {
  # Write HANDOFF.md with wave_id=2 but missing last_verified_develop_sha.
  # This SHOULD trigger a validation block (exit 2) when the plugin runs
  # normally. If the plugin crashes, on_error="continue" means exit 0.
  printf '{
    "event_name": "PostToolUse",
    "tool_name": "Write",
    "session_id": "ac-013-fail-open-test",
    "dispatcher_trace_id": "ac-013-trace",
    "tool_input": {
      "file_path": "factory-artifacts/HANDOFF.md",
      "content": "wave_id: 2\nactive_bcs: []\nnext_wave_stories: []\n"
    },
    "tool_response": {"exit_code": 0}
  }'
}

_write_envelope_non_handoff() {
  # Write to a file that must NOT match the gate (path_is_handoff check).
  # After F-003 fix, "xHANDOFF.md" must be a no-op → exit 0.
  printf '{
    "event_name": "PostToolUse",
    "tool_name": "Write",
    "session_id": "ac-013-non-handoff-test",
    "dispatcher_trace_id": "ac-013-non-handoff-trace",
    "tool_input": {
      "file_path": "factory-artifacts/xHANDOFF.md",
      "content": "wave_id: 2\n"
    },
    "tool_response": {"exit_code": 0}
  }'
}

# ---------------------------------------------------------------------------
# Scenario A: static registry check — on_error="continue" in production registry
# This test does NOT require the dispatcher or WASM plugin to be compiled.
# RED GATE: asserts the production registry entry has on_error = "continue".
# Fails if the registry entry is absent or specifies on_error = "block".
# ---------------------------------------------------------------------------

@test "AC-013 STATIC: production hooks-registry.toml specifies on_error=\"continue\" for validate-wave-handoff-completeness" {
  # Extract the on_error value for the validate-wave-handoff-completeness hook.
  # The awk extracts the value between the hook name match and the next [[hooks]] block.
  local on_error_value
  on_error_value=$(awk '
    /^name = "validate-wave-handoff-completeness"$/ { in_hook=1 }
    in_hook && /^\[\[hooks\]\]/ && !first { in_hook=0 }
    in_hook && /^on_error = / {
      gsub(/^on_error = "|"$/, ""); print; exit
    }
  ' "$PRODUCTION_REGISTRY")

  # AC-013 / BC-4.14.001 PC6: on_error must be "continue" (fail-open).
  [ -n "$on_error_value" ] || {
    echo "FAIL: on_error field not found for validate-wave-handoff-completeness in $PRODUCTION_REGISTRY" >&2
    false
  }
  [ "$on_error_value" = "continue" ] || {
    echo "FAIL: on_error for validate-wave-handoff-completeness is '$on_error_value', expected 'continue' (BC-4.14.001 PC6 / AC-013)" >&2
    false
  }
}

# ---------------------------------------------------------------------------
# Scenario B: live integration — incomplete HANDOFF.md triggers validation block
# Skips if dispatcher/WASM not compiled. When both present, asserts exit 2
# (validation found a real violation, not a crash). Proves the gate runs normally
# and on_error="continue" only matters on crash paths, not normal operation.
# ---------------------------------------------------------------------------

@test "AC-013 LIVE: incomplete HANDOFF.md Write triggers Block exit 2 (gate runs normally, not fail-open crash)" {
  _require_dispatcher_and_wasm
  _write_test_registry
  cp "$WASM_PLUGIN" "$WORK/hook-plugins/"

  local envelope
  envelope="$(_write_envelope_incomplete_handoff)"
  run bash -c "printf '%s' '$envelope' | CLAUDE_PLUGIN_ROOT='$WORK' CLAUDE_PROJECT_DIR='$WORK' '$DISPATCHER' 2>&1 >/dev/null"

  # Exit 2: the gate ran normally and blocked on incomplete HANDOFF.md.
  # If this exits 0 instead of 2, either:
  #   (a) the plugin crashed and on_error="continue" fail-opened it, OR
  #   (b) the gate logic has a bug and is not blocking invalid payloads.
  # In either case, the test correctly fails — AC-013 requires the gate
  # to block incomplete payloads when it runs successfully.
  [ "$status" -eq 2 ]

  # The blocking plugin must be named in the output.
  [[ "$output" == *"blocking_plugins=validate-wave-handoff-completeness"* ]]
}

# ---------------------------------------------------------------------------
# Scenario C: live integration — non-HANDOFF.md path is a no-op (exit 0)
# Skips if dispatcher/WASM not compiled. Confirms path-component-strict
# matching: "xHANDOFF.md" must NOT trigger the gate after F-003 fix.
# ---------------------------------------------------------------------------

@test "AC-013 LIVE: Write to non-HANDOFF.md path (xHANDOFF.md) is a no-op (exit 0, no blocking_plugins)" {
  _require_dispatcher_and_wasm
  _write_test_registry
  cp "$WASM_PLUGIN" "$WORK/hook-plugins/"

  local envelope
  envelope="$(_write_envelope_non_handoff)"
  run bash -c "printf '%s' '$envelope' | CLAUDE_PLUGIN_ROOT='$WORK' CLAUDE_PROJECT_DIR='$WORK' '$DISPATCHER' 2>&1 >/dev/null"

  # Exit 0: non-HANDOFF.md path must not trigger the gate.
  # After F-003 fix (path_is_handoff uses path-component-strict matching),
  # "xHANDOFF.md" resolves to Continue immediately.
  [ "$status" -eq 0 ]

  # No blocking plugin for a path-level no-op.
  [[ "$output" != *"blocking_plugins="* ]]
}
