#!/usr/bin/env bats
# integration-production-registry.bats — F-S15.09-preemptive: production registry capability
#                                         shape regression test
#
# Traces to:
#   BC-5.39.005 invariant 7 (fail-open on host::read_file error — must NOT be triggered
#                             by a production registry misconfiguration)
#   S-15.11 F-P2-001 lesson (production registry path_allow used "**" glob which caused
#                             canonicalize() to fail silently, neutering the hook)
#   Dispatch package §Hard Constraint 1 (bare paths, no "**" glob in path_allow)
#
# This test exercises the PRODUCTION registry capability shape — NOT the inline
# _write_registry() form used by other bats files. It extracts the path_allow entry
# from the production hooks-registry.toml verbatim to catch future drift.
#
# Two scenarios:
#   A. Production-shape registry with a VALID STATE.md => exit 0 (Continue, not fail-open)
#      This proves host::read_file succeeds (capability grants access).
#   B. Production-shape registry with a STRUCTURALLY INVALID STATE.md => exit 2 (Block)
#      This proves the hook evaluates the content (not silently fail-open).
#
# If either scenario regresses to the ** bug, canonicalize() fails, path_allowed()
# returns false, host::read_file returns CapabilityDenied, the hook fail-opens to
# Continue, and scenario B exits 0 instead of 2 — the test fails.
#
# RED GATE PHASE: test skips because validate-state-structure.wasm is not yet compiled.

setup() {
  REPO_ROOT="$(cd "${BATS_TEST_DIRNAME}/../../../.." && pwd)"
  PLUGIN_ROOT="$REPO_ROOT/plugins/vsdd-factory"
  DISPATCHER="$REPO_ROOT/target/release/factory-dispatcher"
  WASM_PLUGIN="$PLUGIN_ROOT/hook-plugins/validate-state-structure.wasm"
  PRODUCTION_REGISTRY="$REPO_ROOT/plugins/vsdd-factory/hooks-registry.toml"
  FIXTURE_VALID="${BATS_TEST_DIRNAME}/../fixtures/validate-state-structure/pass-all-valid"
  FIXTURE_INVALID="${BATS_TEST_DIRNAME}/../fixtures/validate-state-structure/fail-banner-wc-off-by-one"
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
    skip "validate-state-structure.wasm not built -- implement T-5 through T-7 of S-15.09"
  fi
}

# Extract the production path_allow for validate-state-structure from hooks-registry.toml.
# Writes a minimal registry file containing ONLY the validate-state-structure hook entry
# with the PRODUCTION path_allow value (not the inline bats form).
#
# This prevents the production registry from drifting away from what bats tests
# validate — the test fails if the production path_allow is incorrect.
_write_production_registry() {
  # Extract the path_allow value from the production registry.
  # Scoped between the validate-state-structure hook entry and the next [[hooks]] entry.
  local prod_path_allow
  prod_path_allow=$(awk '
    /^name = "validate-state-structure"$/ { in_hook=1 }
    in_hook && /^path_allow = \[/ { in_pa=1; next }
    in_pa && /^\]/ { in_pa=0; in_hook=0 }
    in_pa { gsub(/^[[:space:]]+/, ""); gsub(/,$/, ""); print }
  ' "$PRODUCTION_REGISTRY")

  if [ -z "$prod_path_allow" ]; then
    echo "FAIL: could not extract path_allow from production registry at $PRODUCTION_REGISTRY" >&2
    echo "FAIL: validate-state-structure entry must be added to hooks-registry.toml by implementer" >&2
    return 1
  fi

  # Confirm production path_allow does NOT use ** glob (lesson #1 from S-15.11 F-P2-001)
  if echo "$prod_path_allow" | grep -q '\*\*'; then
    echo "FAIL: production path_allow contains '**' glob — canonicalize() will fail → fail-open" >&2
    echo "FAIL: use bare directory path (e.g., \".factory\") not glob" >&2
    return 1
  fi

  cat > "$WORK/hooks-registry.toml" << TOML
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
  ${prod_path_allow}
]
TOML
}

_state_md_envelope() {
  local session_id="${1:-prod-registry-test}"
  printf '{"event_name":"PostToolUse","tool_name":"Edit","session_id":"%s","tool_input":{"file_path":".factory/STATE.md","content":""},"tool_response":{"exit_code":0,"stdout":"","stderr":""}}' \
    "$session_id"
}

# ---------------------------------------------------------------------------
# Scenario A: production-shape registry + valid STATE.md => exit 0 (Continue)
#
# This proves host::read_file succeeds with the production path_allow. If the
# path_allow is misconfigured (e.g. "**" glob), canonicalize() fails, read_file
# returns CapabilityDenied, the hook fail-opens — but this scenario would STILL
# exit 0. The real distinguishing test is Scenario B below.
# ---------------------------------------------------------------------------

@test "PROD-REGISTRY: hook emits Continue for valid STATE.md using production path_allow entry" {
  _require_artifacts
  cp -r "$FIXTURE_VALID/factory/." "$WORK/.factory/"
  _write_production_registry
  cp "$WASM_PLUGIN" "$WORK/hook-plugins/"

  local envelope
  envelope="$(_state_md_envelope "prod-valid")"
  run bash -c "printf '%s' '$envelope' | CLAUDE_PLUGIN_ROOT='$WORK' CLAUDE_PROJECT_DIR='$WORK' '$DISPATCHER' 2>&1 >/dev/null"

  # Exit 0: valid STATE.md => Continue
  [ "$status" -eq 0 ]

  # No blocking_plugins for a clean pass
  [[ "$output" != *"blocking_plugins="* ]]
}

# ---------------------------------------------------------------------------
# Scenario B: production-shape registry + invalid STATE.md => exit 2 (Block)
#
# This is the LOAD-BEARING test for S-15.11 F-P2-001 lesson (preemptive application):
#   - If path_allow is ".factory/**": canonicalize() fails → path_allowed()
#     returns false → host::read_file returns CapabilityDenied → hook fail-opens →
#     exit 0. TEST FAILS (we expect exit 2).
#   - If path_allow is ".factory": canonicalize() succeeds → path_allowed()
#     returns true → host::read_file returns content → hook evaluates → blocks →
#     exit 2. TEST PASSES.
#
# A regression to "**" causes this test to fail with: expected status 2, got 0.
# ---------------------------------------------------------------------------

@test "PROD-REGISTRY: hook blocks for invalid STATE.md using production path_allow entry (not fail-open)" {
  _require_artifacts
  cp -r "$FIXTURE_INVALID/factory/." "$WORK/.factory/"
  _write_production_registry
  cp "$WASM_PLUGIN" "$WORK/hook-plugins/"

  local envelope
  envelope="$(_state_md_envelope "prod-invalid")"
  run bash -c "printf '%s' '$envelope' | CLAUDE_PLUGIN_ROOT='$WORK' CLAUDE_PROJECT_DIR='$WORK' '$DISPATCHER' 2>&1 >/dev/null"

  # Exit 2: invalid STATE.md => Block. If this exits 0, the hook silently
  # fail-opened due to CapabilityDenied from a misconfigured path_allow.
  [ "$status" -eq 2 ]

  # blocking_plugins= names this hook (not fail-open)
  [[ "$output" == *"blocking_plugins=validate-state-structure"* ]]
}
