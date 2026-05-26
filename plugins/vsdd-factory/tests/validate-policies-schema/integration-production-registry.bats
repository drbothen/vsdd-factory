#!/usr/bin/env bats
# integration-production-registry.bats — AC-25: production registry capability shape regression test
#
# Traces to:
#   BC-5.39.008 invariant 9 (fail-open on host::read_file error must NOT be triggered by a
#                             production registry misconfiguration)
#   S-15.11 F-P2-001 lesson (preemptive application — included from day 1, not added in fix-burst)
#   Dispatch package §Hard Constraint 1 (bare paths, no "**" glob in path_allow)
#
# Two scenarios:
#   A. Production-shape registry + valid policies.yaml => exit 0 (Continue, not fail-open)
#      Proves host::read_file succeeds (capability grants access).
#   B. Production-shape registry + invalid policies.yaml (missing version header) => exit 2 (Block)
#      Proves hook actually evaluates content (distinguishes real Continue from fail-open Continue).
#
# If path_allow uses "**" glob (regression to S-15.07/S-15.11 bug): canonicalize() fails =>
# path_allowed() returns false => host::read_file returns CapabilityDenied => hook fail-opens =>
# Scenario B exits 0 instead of 2. Test fails.
#
# Also validates AC-25 structural checks:
#   - validate-policies-schema entry present in production registry
#   - priority = 157 (NOT 155 or 156)
#   - tool = "Edit|Write" (canonical Q5 form, NOT "Write|Edit")
#   - no "**" glob in path_allow
#
# RED GATE PHASE: test skips if validate-policies-schema.wasm is not yet compiled.

setup() {
  REPO_ROOT="$(cd "${BATS_TEST_DIRNAME}/../../../.." && pwd)"
  PLUGIN_ROOT="$REPO_ROOT/plugins/vsdd-factory"
  DISPATCHER="$REPO_ROOT/target/release/factory-dispatcher"
  WASM_PLUGIN="$PLUGIN_ROOT/hook-plugins/validate-policies-schema.wasm"
  PRODUCTION_REGISTRY="$REPO_ROOT/plugins/vsdd-factory/hooks-registry.toml"
  FIXTURE_VALID="${BATS_TEST_DIRNAME}/../fixtures/validate-policies-schema/integration-production-registry"
  FIXTURE_INVALID="${BATS_TEST_DIRNAME}/../fixtures/validate-policies-schema/fail-missing-header-field"
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
    skip "validate-policies-schema.wasm not built -- implement T-6 through T-11 of S-15.15"
  fi
}

# Extract the production path_allow for validate-policies-schema from hooks-registry.toml.
# Also validates AC-25 structural assertions:
#   - entry exists
#   - priority = 157
#   - tool = "Edit|Write" (canonical Q5 form)
#   - no "**" glob in path_allow
_write_production_registry() {
  # AC-25 check 1: hook entry must exist
  if ! grep -q 'name = "validate-policies-schema"' "$PRODUCTION_REGISTRY"; then
    echo "FAIL: validate-policies-schema entry not found in production registry at $PRODUCTION_REGISTRY" >&2
    echo "FAIL: entry must be added to hooks-registry.toml by implementer (T-10 of S-15.15)" >&2
    return 1
  fi

  # AC-25 check 2: priority must be 157 (NOT 155 or 156)
  local priority_line
  priority_line=$(awk '/^name = "validate-policies-schema"$/{found=1} found && /^priority =/{print; exit}' "$PRODUCTION_REGISTRY")
  if ! echo "$priority_line" | grep -q 'priority = 157'; then
    echo "FAIL: production registry validate-policies-schema priority is wrong: $priority_line" >&2
    echo "FAIL: must be priority = 157 (155 = validate-stable-anchors PreToolUse; 156 = validate-closes-completeness PostToolUse)" >&2
    return 1
  fi

  # AC-25 check 3: tool = "Edit|Write" (canonical Q5 form — NOT "Write|Edit")
  local tool_line
  tool_line=$(awk '/^name = "validate-policies-schema"$/{found=1} found && /^tool =/{print; exit}' "$PRODUCTION_REGISTRY")
  if ! echo "$tool_line" | grep -q 'tool = "Edit|Write"'; then
    echo "FAIL: production registry uses wrong tool form: $tool_line" >&2
    echo "FAIL: must be tool = \"Edit|Write\" (canonical Q5 form)" >&2
    return 1
  fi

  # Extract path_allow from production registry for validate-policies-schema entry
  local prod_path_allow
  prod_path_allow=$(awk '
    /^name = "validate-policies-schema"$/ { in_hook=1 }
    in_hook && /^path_allow = \[/ { in_pa=1; next }
    in_pa && /^\]/ { in_pa=0; in_hook=0 }
    in_pa { gsub(/^[[:space:]]+/, ""); gsub(/,$/, ""); print }
  ' "$PRODUCTION_REGISTRY")

  if [ -z "$prod_path_allow" ]; then
    echo "FAIL: could not extract path_allow from production registry" >&2
    return 1
  fi

  # Confirm production path_allow does NOT use ** glob (S-15.11 F-P2-001 lesson preemptive application)
  if echo "$prod_path_allow" | grep -q '\*\*'; then
    echo "FAIL: production path_allow contains '**' glob — canonicalize() will fail => fail-open" >&2
    echo "FAIL: use bare directory path (e.g., \".factory\") not glob" >&2
    return 1
  fi

  cat > "$WORK/hooks-registry.toml" << TOML
schema_version = 2

[[hooks]]
name = "validate-policies-schema"
event = "PostToolUse"
tool = "Edit|Write"
plugin = "hook-plugins/validate-policies-schema.wasm"
priority = 157
timeout_ms = 5000
on_error = "continue"

[hooks.capabilities.read_file]
path_allow = [
  ${prod_path_allow}
]
TOML
}

_policies_yaml_envelope() {
  local session_id="${1:-prod-registry-test}"
  printf '{"event_name":"PostToolUse","tool_name":"Edit","session_id":"%s","tool_input":{"file_path":".factory/policies.yaml","content":""},"tool_response":{"exit_code":0,"stdout":"","stderr":""}}' \
    "$session_id"
}

# ---------------------------------------------------------------------------
# AC-25 structural check: production registry has validate-policies-schema at priority 157
# with tool = "Edit|Write"
# ---------------------------------------------------------------------------

@test "test_BC_5_39_008_integration_production_registry_has_correct_entry" {
  _require_artifacts
  _write_production_registry || {
    echo "production registry structural checks failed — test cannot proceed" >&2
    return 1
  }
  # If _write_production_registry succeeded, all structural assertions passed
  grep -q 'path_allow = \[' "$WORK/hooks-registry.toml"
}

# ---------------------------------------------------------------------------
# AC-25 / Scenario A: production-shape registry + valid policies.yaml => exit 0 (Continue)
# Proves host::read_file succeeds with the production path_allow.
# ---------------------------------------------------------------------------

@test "PROD-REGISTRY: hook emits Continue for valid policies.yaml using production path_allow" {
  _require_artifacts
  cp -r "$FIXTURE_VALID/factory/." "$WORK/.factory/"
  _write_production_registry || {
    echo "production registry validation failed — test cannot proceed" >&2
    return 1
  }
  grep -q 'path_allow = \[' "$WORK/hooks-registry.toml" || {
    echo "FAIL: synthesized registry missing path_allow block" >&2
    return 1
  }
  cp "$WASM_PLUGIN" "$WORK/hook-plugins/"

  local envelope
  envelope="$(_policies_yaml_envelope "prod-valid")"
  run bash -c "printf '%s' '$envelope' | CLAUDE_PLUGIN_ROOT='$WORK' CLAUDE_PROJECT_DIR='$WORK' '$DISPATCHER' 2>&1 >/dev/null"

  # Exit 0: valid policies.yaml => Continue
  [ "$status" -eq 0 ]
  [[ "$output" != *"blocking_plugins=validate-policies-schema"* ]]
}

# ---------------------------------------------------------------------------
# AC-25 / Scenario B: production-shape registry + invalid policies.yaml => exit 2 (Block)
#
# LOAD-BEARING for S-15.11 F-P2-001 preemptive application:
#   - If path_allow = ".factory/**": canonicalize() fails => path_allowed() false =>
#     host::read_file returns CapabilityDenied => hook fail-opens => exit 0.
#     TEST FAILS (expected 2, got 0).
#   - If path_allow = ".factory": canonicalize() succeeds => read_file returns content =>
#     hook evaluates => blocks => exit 2. TEST PASSES.
# ---------------------------------------------------------------------------

@test "PROD-REGISTRY: hook blocks for invalid policies.yaml using production path_allow (not fail-open)" {
  _require_artifacts
  cp -r "$FIXTURE_INVALID/factory/." "$WORK/.factory/"
  _write_production_registry || {
    echo "production registry validation failed — test cannot proceed" >&2
    return 1
  }
  grep -q 'path_allow = \[' "$WORK/hooks-registry.toml" || {
    echo "FAIL: synthesized registry missing path_allow block" >&2
    return 1
  }
  cp "$WASM_PLUGIN" "$WORK/hook-plugins/"

  local envelope
  envelope="$(_policies_yaml_envelope "prod-invalid")"
  run bash -c "printf '%s' '$envelope' | CLAUDE_PLUGIN_ROOT='$WORK' CLAUDE_PROJECT_DIR='$WORK' '$DISPATCHER' 2>&1 >/dev/null"

  # Exit 2: invalid policies.yaml => Block. Exit 0 here means hook silently fail-opened
  # due to CapabilityDenied from misconfigured path_allow — the preemptive S-15.11 regression.
  [ "$status" -eq 2 ]
  [[ "$output" == *"blocking_plugins=validate-policies-schema"* ]]
}
