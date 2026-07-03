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
  # .factory/HANDOFF.md is the production write path (per S-18.01 SKILL.md) and
  # the path_allow in the test registry.  Create the directory so per-scenario
  # setup can write the fixture file here without repeating the mkdir.
  mkdir -p "$WORK/.factory"
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
  # The read_file capability block is required: the WASM gate calls
  # host::read_file to read the full on-disk HANDOFF.md (F-001 fix).
  # Without it, the dispatcher denies the read and the gate fails-open on
  # every HANDOFF.md write — making Scenario B (incomplete HANDOFF.md → exit 2)
  # unreachable. path_allow matches the production registry entry.
  cat > "$WORK/hooks-registry.toml" << 'TOML'
schema_version = 2

[[hooks]]
name = "validate-wave-handoff-completeness"
event = "PostToolUse"
tool = "Edit|Write"
plugin = "hook-plugins/validate-wave-handoff-completeness.wasm"
timeout_ms = 5000
on_error = "continue"

[hooks.capabilities.read_file]
path_allow = [
  ".factory/HANDOFF.md",
]
TOML
}

_write_fixture_incomplete_handoff() {
  # Write a HANDOFF.md to disk with wave_id=2 but missing last_verified_develop_sha.
  # The gate reads the on-disk file via host::read_file (path_allow = .factory/HANDOFF.md).
  # With a missing required scalar field the gate must block (exit 2).
  cat > "$WORK/.factory/HANDOFF.md" << 'YAML'
wave_id: 2
active_bcs: []
next_wave_stories:
  - id: S-19.01
    status: pending
open_decisions: []
pending_fixes: []
process_gaps: []
YAML
  # last_verified_develop_sha, precompact_flush_sha, and factory_lock_holder are absent.
}

_write_envelope_incomplete_handoff() {
  # PostToolUse Write envelope whose file_path matches the path_allow entry
  # (.factory/HANDOFF.md) so the dispatcher grants the read_file capability and
  # the gate can read the on-disk fixture written by _write_fixture_incomplete_handoff.
  printf '{
    "event_name": "PostToolUse",
    "tool_name": "Write",
    "session_id": "ac-013-fail-open-test",
    "dispatcher_trace_id": "ac-013-trace",
    "tool_input": {
      "file_path": ".factory/HANDOFF.md",
      "content": "wave_id: 2\nactive_bcs: []\nnext_wave_stories:\n  - id: S-19.01\n    status: pending\nopen_decisions: []\npending_fixes: []\nprocess_gaps: []\n"
    },
    "tool_response": {"exit_code": 0}
  }'
}

_write_envelope_non_handoff() {
  # Write to a file whose filename is NOT exactly "HANDOFF.md" (path_is_handoff
  # uses path-component-strict matching after F-003 fix).  The gate must
  # return Continue (exit 0) immediately without reading any file.
  # Using .factory/xHANDOFF.md — same directory, different filename — so the
  # test cannot accidentally pass because the path_allow prefix is denied.
  printf '{
    "event_name": "PostToolUse",
    "tool_name": "Write",
    "session_id": "ac-013-non-handoff-test",
    "dispatcher_trace_id": "ac-013-non-handoff-trace",
    "tool_input": {
      "file_path": ".factory/xHANDOFF.md",
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

  # Write the fixture file to disk BEFORE running the dispatcher so the gate
  # can read it via host::read_file.  The fixture has wave_id=2 but is missing
  # last_verified_develop_sha, precompact_flush_sha, and factory_lock_holder —
  # all required by AC-004 / BC-4.14.001 PC7.
  _write_fixture_incomplete_handoff

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

# ---------------------------------------------------------------------------
# Scenario D: F-001 Edit-path — on-disk complete HANDOFF.md + Edit fragment → Continue
#
# Validates the F-001 fix: on_post_tool_use reads the full on-disk file via
# host::read_file rather than using tool_input["new_string"] (a fragment).
#
# Setup:
#   1. Write a complete, valid HANDOFF.md to disk (all 9 fields present).
#   2. Send an Edit PostToolUse envelope with "new_string" containing only a
#      fragment (a single incomplete field) — as a real Edit would deliver.
#   3. Because the gate now reads the full on-disk file, it sees the complete
#      content and returns Continue (no block).
#
# If the gate incorrectly validated tool_input["new_string"] instead of reading
# the disk file (the pre-F-001 bug), it would see the incomplete fragment and
# produce Block exit 2.
#
# AC-001 / BC-4.14.001 INV1: validates that Edit path is gated on the full file.
# ---------------------------------------------------------------------------

@test "F-001 LIVE: Edit HANDOFF.md with complete on-disk file and fragment new_string → Continue exit 0" {
  _require_dispatcher_and_wasm
  _write_test_registry
  cp "$WASM_PLUGIN" "$WORK/hook-plugins/"

  # Step 1: Write a complete, valid HANDOFF.md to $WORK/.factory/HANDOFF.md.
  # This path MUST agree with:
  #   (a) the registry path_allow = [".factory/HANDOFF.md"] (resolved under
  #       CLAUDE_PROJECT_DIR=$WORK → $WORK/.factory/HANDOFF.md), AND
  #   (b) the Edit envelope "path" field below.
  # Using $WORK/.factory/ (already created in setup()) — NOT factory-artifacts/.
  local handoff_path="$WORK/.factory/HANDOFF.md"
  cat > "$handoff_path" << 'YAML'
wave_id: 2
last_verified_develop_sha: abc123def456
precompact_flush_sha: null
factory_lock_holder: null
active_bcs:
  - BC-4.14.001
next_wave_stories:
  - id: S-19.01
    status: pending
open_decisions: []
pending_fixes: []
process_gaps: []
YAML

  # Step 2: Send an Edit envelope where new_string is only a fragment.
  # A real Edit call provides only the replacement text for old_string,
  # not the full file. Before F-001, the gate read new_string → block.
  # After F-001, the gate reads the full on-disk file via host::read_file
  # (using path_allow-granted capability) → sees complete content → Continue.
  # The "path" field must match the on-disk file and the path_allow entry.
  local envelope
  envelope=$(printf '{
    "event_name": "PostToolUse",
    "tool_name": "Edit",
    "session_id": "f-001-edit-path-test",
    "dispatcher_trace_id": "f-001-edit-trace",
    "tool_input": {
      "path": "%s",
      "old_string": "wave_id: 2",
      "new_string": "wave_id: 2"
    },
    "tool_response": {"exit_code": 0}
  }' "$handoff_path")

  run bash -c "printf '%s' '$envelope' | CLAUDE_PLUGIN_ROOT='$WORK' CLAUDE_PROJECT_DIR='$WORK' '$DISPATCHER' 2>&1 >/dev/null"

  # Exit 0: gate reads full on-disk file (complete HANDOFF.md) → Continue.
  # If exit 2: gate still reads new_string fragment instead of disk file
  # (F-001 not fixed), or disk write is not visible to host::read_file.
  [ "$status" -eq 0 ]

  # No blocking plugin: the gate must have returned Continue.
  [[ "$output" != *"blocking_plugins="* ]]
}
