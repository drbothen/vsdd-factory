#!/usr/bin/env bats
# integration-production-registry.bats — F-S15.11-LOCAL-P2-001 fix verification
#
# Traces to:
#   BC-5.39.004 invariant 5 (fail-open on host::read_file error — must NOT be
#                             triggered by a production registry misconfiguration)
#   F-S15.11-LOCAL-P2-001 (production registry path_allow used "**" glob which
#                          caused canonicalize() to fail silently, neutering the hook)
#
# This test exercises the PRODUCTION registry capability shape — NOT the inline
# _write_registry() form used by other bats files. It extracts the path_allow entry
# from the production hooks-registry.toml verbatim to catch future drift.
#
# Two scenarios:
#   A. Production-shape registry with a VALID burst-log => exit 0 (Continue, not fail-open)
#      This proves host::read_file succeeds (capability grants access).
#   B. Production-shape registry with a STRUCTURALLY INCOMPLETE burst-log => exit 2 (Block)
#      This proves the hook evaluates the content (not silently fail-open).
#
# If either scenario regresses to the ** bug, canonicalize() fails, path_allowed()
# returns false, host::read_file returns CapabilityDenied, the hook fail-opens to
# Continue, and scenario B exits 0 instead of 2 — the test fails.
#
# RED GATE PHASE: test skips because validate-burst-log.wasm is not yet compiled.

setup() {
  REPO_ROOT="$(cd "${BATS_TEST_DIRNAME}/../../../.." && pwd)"
  PLUGIN_ROOT="$REPO_ROOT/plugins/vsdd-factory"
  DISPATCHER="$REPO_ROOT/target/release/factory-dispatcher"
  WASM_PLUGIN="$PLUGIN_ROOT/hook-plugins/validate-burst-log.wasm"
  PRODUCTION_REGISTRY="$REPO_ROOT/plugins/vsdd-factory/hooks-registry.toml"
  FIXTURE_VALID="${BATS_TEST_DIRNAME}/../fixtures/validate-burst-log/pass-complete-entry"
  FIXTURE_INCOMPLETE="${BATS_TEST_DIRNAME}/../fixtures/validate-burst-log/fail-6-blocks"
  WORK="$(mktemp -d)"
  mkdir -p "$WORK/hook-plugins"
  mkdir -p "$WORK/.factory/logs"
}

teardown() {
  [ -n "${WORK:-}" ] && [ -d "$WORK" ] && find "$WORK" -type f -delete && find "$WORK" -type d -mindepth 1 | sort -r | xargs rmdir 2>/dev/null && rmdir "$WORK" 2>/dev/null || true
}

_require_artifacts() {
  if [ ! -x "$DISPATCHER" ]; then
    skip "dispatcher binary not built -- run: cargo build --release -p factory-dispatcher"
  fi
  if [ ! -f "$WASM_PLUGIN" ]; then
    skip "validate-burst-log.wasm not built -- implement T-4 through T-7 of S-15.11"
  fi
}

# Extract the production path_allow for validate-burst-log from hooks-registry.toml.
# Writes a minimal registry file containing ONLY the validate-burst-log hook entry
# with the PRODUCTION path_allow value (not the inline bats form).
#
# This prevents the production registry from drifting away from what bats tests
# validate — the test fails if the production path_allow is incorrect.
_write_production_registry() {
  # Extract the path_allow value from the production registry.
  # The production entry is the only path_allow under [hooks.capabilities.read_file]
  # for the validate-burst-log hook. We extract it with awk scoped between the hook
  # entry and the next hook entry.
  local prod_path_allow
  prod_path_allow=$(awk '
    /^name = "validate-burst-log"$/ { in_hook=1 }
    in_hook && /^path_allow = \[/ { in_pa=1; next }
    in_pa && /^\]/ { in_pa=0; in_hook=0 }
    in_pa { gsub(/^[[:space:]]+/, ""); gsub(/,$/, ""); print }
  ' "$PRODUCTION_REGISTRY")

  if [ -z "$prod_path_allow" ]; then
    echo "FAIL: could not extract path_allow from production registry at $PRODUCTION_REGISTRY" >&2
    return 1
  fi

  cat > "$WORK/hooks-registry.toml" << TOML
schema_version = 2

[[hooks]]
name = "validate-burst-log"
event = "PostToolUse"
tool = "Edit|Write"
plugin = "hook-plugins/validate-burst-log.wasm"
timeout_ms = 5000
on_error = "continue"

[hooks.capabilities.read_file]
path_allow = [
  ${prod_path_allow}
]
TOML
}

_burst_log_envelope() {
  local session_id="${1:-prod-registry-test}"
  printf '{"event_name":"PostToolUse","tool_name":"Edit","session_id":"%s","tool_input":{"file_path":".factory/cycles/v1.0-feature-engine-discipline-pass-1/burst-log.md","content":""},"tool_response":{"exit_code":0,"stdout":"","stderr":""}}' \
    "$session_id"
}

# ---------------------------------------------------------------------------
# Scenario A: production-shape registry + valid burst-log => exit 0 (Continue)
#
# This proves host::read_file succeeds with the production path_allow. If the
# path_allow is misconfigured (e.g. "**" glob), canonicalize() fails, read_file
# returns CapabilityDenied, the hook fail-opens — but this scenario would STILL
# exit 0. The real distinguishing test is Scenario B below.
# ---------------------------------------------------------------------------

@test "PROD-REGISTRY: hook emits Continue for valid burst-log using production path_allow entry" {
  _require_artifacts
  cp -r "$FIXTURE_VALID/factory/." "$WORK/.factory/"
  _write_production_registry
  cp "$WASM_PLUGIN" "$WORK/hook-plugins/"

  local envelope
  envelope="$(_burst_log_envelope "prod-valid")"
  run bash -c "printf '%s' '$envelope' | CLAUDE_PLUGIN_ROOT='$WORK' CLAUDE_PROJECT_DIR='$WORK' '$DISPATCHER' 2>&1 >/dev/null"

  # Exit 0: valid burst-log => Continue
  [ "$status" -eq 0 ]

  # No blocking_plugins for a clean pass
  [[ "$output" != *"blocking_plugins="* ]]
}

# ---------------------------------------------------------------------------
# Scenario B: production-shape registry + structurally incomplete burst-log => exit 2 (Block)
#
# This is the LOAD-BEARING test for F-S15.11-LOCAL-P2-001:
#   - If path_allow is ".factory/cycles/**": canonicalize() fails → path_allowed()
#     returns false → host::read_file returns CapabilityDenied → hook fail-opens →
#     exit 0. TEST FAILS (we expect exit 2).
#   - If path_allow is ".factory/cycles": canonicalize() succeeds → path_allowed()
#     returns true → host::read_file returns content → hook evaluates → blocks →
#     exit 2. TEST PASSES.
#
# A regression to "**" causes this test to fail with: expected status 2, got 0.
# ---------------------------------------------------------------------------

@test "PROD-REGISTRY: hook blocks for incomplete burst-log using production path_allow entry (not fail-open)" {
  _require_artifacts
  cp -r "$FIXTURE_INCOMPLETE/factory/." "$WORK/.factory/"
  _write_production_registry
  cp "$WASM_PLUGIN" "$WORK/hook-plugins/"

  local envelope
  envelope="$(_burst_log_envelope "prod-incomplete")"
  run bash -c "printf '%s' '$envelope' | CLAUDE_PLUGIN_ROOT='$WORK' CLAUDE_PROJECT_DIR='$WORK' '$DISPATCHER' 2>&1 >/dev/null"

  # Exit 2: incomplete burst-log => Block. If this exits 0, the hook silently
  # fail-opened due to CapabilityDenied from a misconfigured path_allow.
  [ "$status" -eq 2 ]

  # blocking_plugins= names this hook (not fail-open)
  [[ "$output" == *"blocking_plugins=validate-burst-log"* ]]
}
