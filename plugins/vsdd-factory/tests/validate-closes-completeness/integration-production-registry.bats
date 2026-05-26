#!/usr/bin/env bats
# integration-production-registry.bats — F-S15.12-preemptive: production registry capability
#                                         shape regression test (preemptive per S-15.11 F-P2-001)
#
# Traces to:
#   BC-5.39.007 invariant 9 (fail-open on host::read_file error — must NOT be triggered
#                             by a production registry misconfiguration)
#   S-15.11 F-P2-001 lesson (production registry path_allow used "**" glob which caused
#                            canonicalize() to fail silently, neutering the hook)
#   Dispatch package §Hard Constraint 1 (bare paths, no "**" glob in path_allow)
#
# This test exercises the PRODUCTION registry capability shape — NOT the inline
# _write_registry() form used by other bats files. It extracts the path_allow entry
# from the production hooks-registry.toml verbatim to catch future drift.
#
# Two scenarios:
#   A. Production-shape registry with a VALID lessons.md => exit 0 (Continue, not fail-open)
#      This proves host::read_file succeeds (capability grants access).
#   B. Production-shape registry with a STRUCTURALLY INVALID lessons.md => exit 2 (Block)
#      This proves the hook evaluates the content (not silently fail-open).
#
# If either scenario regresses to the ** bug, canonicalize() fails, path_allowed()
# returns false, host::read_file returns CapabilityDenied, the hook fail-opens to
# Continue, and scenario B exits 0 instead of 2 — the test fails.
#
# RED GATE PHASE: test skips because validate-closes-completeness.wasm is not yet compiled.

setup() {
  REPO_ROOT="$(cd "${BATS_TEST_DIRNAME}/../../../.." && pwd)"
  PLUGIN_ROOT="$REPO_ROOT/plugins/vsdd-factory"
  DISPATCHER="$REPO_ROOT/target/release/factory-dispatcher"
  WASM_PLUGIN="$PLUGIN_ROOT/hook-plugins/validate-closes-completeness.wasm"
  PRODUCTION_REGISTRY="$REPO_ROOT/plugins/vsdd-factory/hooks-registry.toml"
  FIXTURE_VALID="${BATS_TEST_DIRNAME}/../fixtures/validate-closes-completeness/pass-lessons-valid-closes"
  FIXTURE_INVALID="${BATS_TEST_DIRNAME}/../fixtures/validate-closes-completeness/fail-lessons-missing-closes"
  FIXTURE_STATE_INVALID="${BATS_TEST_DIRNAME}/../fixtures/validate-closes-completeness/fail-state-umbrella-no-flag"
  FIXTURE_DECISIONLOG_INVALID="${BATS_TEST_DIRNAME}/../fixtures/validate-closes-completeness/fail-decisionlog-umbrella-no-flag"
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
    skip "validate-closes-completeness.wasm not built -- implement T-4 through T-7 of S-15.12"
  fi
}

# Extract the production path_allow for validate-closes-completeness from hooks-registry.toml.
# Writes a minimal registry file containing ONLY the validate-closes-completeness hook entry
# with the PRODUCTION path_allow value (not the inline bats form).
#
# This prevents the production registry from drifting away from what bats tests
# validate — the test fails if the production path_allow is incorrect.
_write_production_registry() {
  # Extract the path_allow value from the production registry.
  # Scoped between the validate-closes-completeness hook entry and the next [[hooks]] entry.
  local prod_path_allow
  prod_path_allow=$(awk '
    /^name = "validate-closes-completeness"$/ { in_hook=1 }
    in_hook && /^path_allow = \[/ { in_pa=1; next }
    in_pa && /^\]/ { in_pa=0; in_hook=0 }
    in_pa { gsub(/^[[:space:]]+/, ""); gsub(/,$/, ""); print }
  ' "$PRODUCTION_REGISTRY")

  if [ -z "$prod_path_allow" ]; then
    echo "FAIL: could not extract path_allow from production registry at $PRODUCTION_REGISTRY" >&2
    echo "FAIL: validate-closes-completeness entry must be added to hooks-registry.toml by implementer" >&2
    return 1
  fi

  # Confirm production path_allow does NOT use ** glob (lesson #1 from S-15.11 F-P2-001)
  if echo "$prod_path_allow" | grep -q '\*\*'; then
    echo "FAIL: production path_allow contains '**' glob — canonicalize() will fail → fail-open" >&2
    echo "FAIL: use bare directory path (e.g., \".factory/cycles\") not glob" >&2
    return 1
  fi

  cat > "$WORK/hooks-registry.toml" << TOML
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
  ${prod_path_allow}
]
TOML
}

_lessons_md_envelope() {
  local session_id="${1:-prod-registry-test}"
  printf '{"event_name":"PostToolUse","tool_name":"Edit","session_id":"%s","tool_input":{"file_path":".factory/cycles/v1.0-brownfield-backfill/lessons.md","content":""},"tool_response":{"exit_code":0,"stdout":"","stderr":""}}' \
    "$session_id"
}

_state_md_envelope() {
  local session_id="${1:-prod-registry-state-test}"
  printf '{"event_name":"PostToolUse","tool_name":"Edit","session_id":"%s","tool_input":{"file_path":".factory/STATE.md","content":""},"tool_response":{"exit_code":0,"stdout":"","stderr":""}}' \
    "$session_id"
}

_decision_log_md_envelope() {
  local session_id="${1:-prod-registry-decisionlog-test}"
  printf '{"event_name":"PostToolUse","tool_name":"Edit","session_id":"%s","tool_input":{"file_path":".factory/cycles/v1.0-brownfield-backfill/decision-log.md","content":""},"tool_response":{"exit_code":0,"stdout":"","stderr":""}}' \
    "$session_id"
}

# ---------------------------------------------------------------------------
# Scenario A: production-shape registry + valid lessons.md => exit 0 (Continue)
#
# This proves host::read_file succeeds with the production path_allow. If the
# path_allow is misconfigured (e.g. "**" glob), canonicalize() fails, read_file
# returns CapabilityDenied, the hook fail-opens — but this scenario would STILL
# exit 0. The real distinguishing test is Scenario B below.
# ---------------------------------------------------------------------------

@test "PROD-REGISTRY: hook emits Continue for valid lessons.md using production path_allow entry" {
  _require_artifacts
  cp -r "$FIXTURE_VALID/factory/." "$WORK/.factory/"
  _write_production_registry || {
    echo "production registry extraction failed — test cannot proceed" >&2
    return 1
  }
  # Verify the synthesized registry is well-formed: path_allow must be present.
  grep -q 'path_allow = \[' "$WORK/hooks-registry.toml" || {
    echo "FAIL: synthesized registry missing path_allow block" >&2
    return 1
  }
  cp "$WASM_PLUGIN" "$WORK/hook-plugins/"

  local envelope
  envelope="$(_lessons_md_envelope "prod-valid")"
  run bash -c "printf '%s' '$envelope' | CLAUDE_PLUGIN_ROOT='$WORK' CLAUDE_PROJECT_DIR='$WORK' '$DISPATCHER' 2>&1 >/dev/null"

  # Exit 0: valid lessons.md => Continue
  [ "$status" -eq 0 ]

  # No blocking_plugins for a clean pass
  [[ "$output" != *"blocking_plugins="* ]]
}

# ---------------------------------------------------------------------------
# Scenario B: production-shape registry + invalid lessons.md => exit 2 (Block)
#
# This is the LOAD-BEARING test for S-15.11 F-P2-001 lesson (preemptive application):
#   - If path_allow is ".factory/cycles/**": canonicalize() fails → path_allowed()
#     returns false → host::read_file returns CapabilityDenied → hook fail-opens →
#     exit 0. TEST FAILS (we expect exit 2).
#   - If path_allow is ".factory/cycles": canonicalize() succeeds → path_allowed()
#     returns true → host::read_file returns content → hook evaluates → blocks →
#     exit 2. TEST PASSES.
#
# A regression to "**" causes this test to fail with: expected status 2, got 0.
# ---------------------------------------------------------------------------

@test "PROD-REGISTRY: hook blocks for invalid lessons.md using production path_allow entry (not fail-open)" {
  _require_artifacts
  cp -r "$FIXTURE_INVALID/factory/." "$WORK/.factory/"
  _write_production_registry || {
    echo "production registry extraction failed — test cannot proceed" >&2
    return 1
  }
  # Verify the synthesized registry is well-formed: path_allow must be present.
  grep -q 'path_allow = \[' "$WORK/hooks-registry.toml" || {
    echo "FAIL: synthesized registry missing path_allow block" >&2
    return 1
  }
  cp "$WASM_PLUGIN" "$WORK/hook-plugins/"

  local envelope
  envelope="$(_lessons_md_envelope "prod-invalid")"
  run bash -c "printf '%s' '$envelope' | CLAUDE_PLUGIN_ROOT='$WORK' CLAUDE_PROJECT_DIR='$WORK' '$DISPATCHER' 2>&1 >/dev/null"

  # Exit 2: invalid lessons.md => Block. If this exits 0, the hook silently
  # fail-opened due to CapabilityDenied from a misconfigured path_allow.
  [ "$status" -eq 2 ]

  # blocking_plugins= names this hook (not fail-open)
  [[ "$output" == *"blocking_plugins=validate-closes-completeness"* ]]
}

# ---------------------------------------------------------------------------
# Scenario C: production-shape registry + STATE.md with bare umbrella cite => exit 2 (Block)
#
# This is the LOAD-BEARING test for CRIT-001 (adversary pass-1):
#   - If path_allow is ".factory/cycles" only: STATE.md at ".factory/STATE.md" is
#     outside the allowed path → host::read_file returns CapabilityDenied → hook
#     fail-opens → exit 0. TEST FAILS (we expect exit 2).
#   - If path_allow is ".factory": STATE.md is within the allowed subtree →
#     host::read_file returns content → hook evaluates → blocks → exit 2. TEST PASSES.
#
# A regression to ".factory/cycles"-only causes this test to fail with:
# expected status 2, got 0.
# ---------------------------------------------------------------------------

@test "PROD-REGISTRY: hook blocks for invalid STATE.md using production path_allow entry (not fail-open)" {
  _require_artifacts
  mkdir -p "$WORK/.factory"
  cp "$FIXTURE_STATE_INVALID/factory/STATE.md" "$WORK/.factory/STATE.md"
  _write_production_registry || {
    echo "production registry extraction failed — test cannot proceed" >&2
    return 1
  }
  grep -q 'path_allow = \[' "$WORK/hooks-registry.toml" || {
    echo "FAIL: synthesized registry missing path_allow block" >&2
    return 1
  }
  cp "$WASM_PLUGIN" "$WORK/hook-plugins/"

  local envelope
  envelope="$(_state_md_envelope "prod-state-invalid")"
  run bash -c "printf '%s' '$envelope' | CLAUDE_PLUGIN_ROOT='$WORK' CLAUDE_PROJECT_DIR='$WORK' '$DISPATCHER' 2>&1 >/dev/null"

  # Exit 2: invalid STATE.md => Block. If this exits 0, path_allow does not cover
  # .factory/STATE.md — the STATE.md umbrella-flag arm is inert (CRIT-001).
  [ "$status" -eq 2 ]

  # blocking_plugins= names this hook (not fail-open)
  [[ "$output" == *"blocking_plugins=validate-closes-completeness"* ]]
}

# ---------------------------------------------------------------------------
# Scenario D: production-shape registry + decision-log.md with bare umbrella cite => exit 2 (Block)
#
# This mirrors Scenario C for decision-log.md (also under .factory/cycles, which is
# already covered, but the explicit test provides regression protection if path_allow
# is ever narrowed further — e.g., to a specific cycle directory).
# ---------------------------------------------------------------------------

@test "PROD-REGISTRY: hook blocks for invalid decision-log.md using production path_allow entry (not fail-open)" {
  _require_artifacts
  mkdir -p "$WORK/.factory/cycles/v1.0-brownfield-backfill"
  cp "$FIXTURE_DECISIONLOG_INVALID/factory/cycles/v1.0-brownfield-backfill/decision-log.md" \
     "$WORK/.factory/cycles/v1.0-brownfield-backfill/decision-log.md"
  _write_production_registry || {
    echo "production registry extraction failed — test cannot proceed" >&2
    return 1
  }
  grep -q 'path_allow = \[' "$WORK/hooks-registry.toml" || {
    echo "FAIL: synthesized registry missing path_allow block" >&2
    return 1
  }
  cp "$WASM_PLUGIN" "$WORK/hook-plugins/"

  local envelope
  envelope="$(_decision_log_md_envelope "prod-decisionlog-invalid")"
  run bash -c "printf '%s' '$envelope' | CLAUDE_PLUGIN_ROOT='$WORK' CLAUDE_PROJECT_DIR='$WORK' '$DISPATCHER' 2>&1 >/dev/null"

  # Exit 2: invalid decision-log.md => Block.
  [ "$status" -eq 2 ]

  # blocking_plugins= names this hook (not fail-open)
  [[ "$output" == *"blocking_plugins=validate-closes-completeness"* ]]
}
