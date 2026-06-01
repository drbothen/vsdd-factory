#!/usr/bin/env bats
# integration-production-registry.bats — AC-1: production registry capability shape test
#
# Traces to:
#   BC-5.39.009 §Architecture Anchors (Hook registry entry priority 158)
#   BC-5.39.009 invariant 1 (read-only validator; no write capability)
#   BC-5.39.009 invariant 12 (on_error = "continue")
#   S-15.11 F-P2-001 lesson (preemptive application — included from day 1)
#   Dispatch package §Hard Constraint 1 (bare paths, no "**" glob in path_allow)
#
# CRITICAL per PG-S-15.11 (Drift Item): bats inline _write_registry() path_allow arrays
#   MUST be byte-identical to the eventual production hooks-registry.toml entry for this hook:
#   priority = 158, PostToolUse, tool = "Edit|Write", path_allow = [".factory"]
#
# Three scenarios:
#   A. Production-shape registry + valid STATE.md (all sites present) => exit 0 (Continue)
#      Proves host::read_file succeeds (capability grants access to .factory/).
#   B. Production-shape registry + invalid STATE.md (missing frontmatter tail) => exit 2 (Block)
#      Proves hook actually evaluates content (distinguishes real Continue from fail-open).
#
# Also validates AC-1 structural checks:
#   - validate-trajectory-tail-cell-completeness entry present in production registry
#   - priority = 158 (NOT 157 or lower)
#   - tool = "Edit|Write" (canonical form; NOT "Write|Edit")
#   - on_error = "continue" (invariant 12)
#   - path_allow = [".factory"] (NOT [".factory/**"] — bare path, no glob)
#   - no "**" glob in path_allow (S-15.11 F-P2-001 preemptive lesson)
#
# RED GATE PHASE: test skips if validate-trajectory-tail-cell-completeness.wasm not yet compiled.

setup() {
  REPO_ROOT="$(cd "${BATS_TEST_DIRNAME}/../../../.." && pwd)"
  PLUGIN_ROOT="$REPO_ROOT/plugins/vsdd-factory"
  DISPATCHER="$REPO_ROOT/target/release/factory-dispatcher"
  WASM_PLUGIN="$PLUGIN_ROOT/hook-plugins/validate-trajectory-tail-cell-completeness.wasm"
  PRODUCTION_REGISTRY="$REPO_ROOT/plugins/vsdd-factory/hooks-registry.toml"
  FIXTURE_VALID="${BATS_TEST_DIRNAME}/../fixtures/validate-trajectory-tail-cell-completeness/integration-production-registry"
  FIXTURE_INVALID="${BATS_TEST_DIRNAME}/../fixtures/validate-trajectory-tail-cell-completeness/fail-state-frontmatter-missing-tail"
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
    skip "validate-trajectory-tail-cell-completeness.wasm not built -- implement T-5 through T-8 of S-15.17"
  fi
}

# Extract production entry for validate-trajectory-tail-cell-completeness and validate
# structural invariants per AC-1. Returns 0 on success; writes validated registry to
# $WORK/hooks-registry.toml for use in scenario tests.
_write_production_registry() {
  # AC-1 check 1: hook entry must exist
  if ! grep -q 'name = "validate-trajectory-tail-cell-completeness"' "$PRODUCTION_REGISTRY"; then
    echo "FAIL: validate-trajectory-tail-cell-completeness entry not found in production registry at $PRODUCTION_REGISTRY" >&2
    echo "FAIL: entry must be added to hooks-registry.toml by implementer (T-7 of S-15.17)" >&2
    return 1
  fi

  # AC-1 check 2: priority must be 158
  local priority_line
  priority_line=$(awk '/^name = "validate-trajectory-tail-cell-completeness"$/{found=1} found && /^priority =/{print; exit}' "$PRODUCTION_REGISTRY")
  if ! echo "$priority_line" | grep -q 'priority = 158'; then
    echo "FAIL: production registry validate-trajectory-tail-cell-completeness priority is wrong: $priority_line" >&2
    echo "FAIL: must be priority = 158 (next after validate-policies-schema at 157)" >&2
    return 1
  fi

  # AC-1 check 3: tool = "Edit|Write" (canonical form)
  local tool_line
  tool_line=$(awk '/^name = "validate-trajectory-tail-cell-completeness"$/{found=1} found && /^tool =/{print; exit}' "$PRODUCTION_REGISTRY")
  if ! echo "$tool_line" | grep -q 'tool = "Edit|Write"'; then
    echo "FAIL: production registry uses wrong tool form: $tool_line" >&2
    echo "FAIL: must be tool = \"Edit|Write\" (canonical form)" >&2
    return 1
  fi

  # AC-1 check 4: on_error = "continue" (invariant 12)
  local on_error_line
  on_error_line=$(awk '/^name = "validate-trajectory-tail-cell-completeness"$/{found=1} found && /^on_error =/{print; exit}' "$PRODUCTION_REGISTRY")
  if ! echo "$on_error_line" | grep -q 'on_error = "continue"'; then
    echo "FAIL: production registry on_error is wrong: $on_error_line" >&2
    echo "FAIL: must be on_error = \"continue\" per BC-5.39.009 invariant 12" >&2
    return 1
  fi

  # Extract path_allow from production registry for this hook
  local prod_path_allow
  prod_path_allow=$(awk '
    /^name = "validate-trajectory-tail-cell-completeness"$/ { in_hook=1 }
    in_hook && /^path_allow = \[/ { in_pa=1; next }
    in_pa && /^\]/ { in_pa=0; in_hook=0 }
    in_pa { gsub(/^[[:space:]]+/, ""); gsub(/,$/, ""); print }
  ' "$PRODUCTION_REGISTRY")

  if [ -z "$prod_path_allow" ]; then
    echo "FAIL: could not extract path_allow from production registry" >&2
    return 1
  fi

  # Confirm path_allow does NOT use ** glob (S-15.11 F-P2-001 preemptive application)
  if echo "$prod_path_allow" | grep -q '\*\*'; then
    echo "FAIL: production path_allow contains '**' glob — canonicalize() will fail => fail-open" >&2
    echo "FAIL: use bare directory path (\".factory\") not glob" >&2
    return 1
  fi

  # Convert multi-line path_allow entries to inline TOML array
  local path_allow_toml
  path_allow_toml=$(echo "$prod_path_allow" | awk 'BEGIN{ORS=""} NR>1{printf ", "} {print}')

  cat > "$WORK/hooks-registry.toml" << TOML
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
path_allow = [${path_allow_toml}]
TOML
}

_state_md_envelope() {
  local session_id="${1:-prod-registry-test}"
  printf '{"event_name":"PostToolUse","tool_name":"Edit","session_id":"%s","tool_input":{"file_path":".factory/STATE.md","content":""},"tool_response":{"exit_code":0,"stdout":"","stderr":""}}' \
    "$session_id"
}

# ---------------------------------------------------------------------------
# AC-1 structural check: production registry has correct entry at priority 158
# ---------------------------------------------------------------------------

@test "test_BC_5_39_009_integration_production_registry_has_correct_entry" {
  _require_artifacts
  _write_production_registry || {
    echo "production registry structural checks failed — test cannot proceed" >&2
    return 1
  }
  grep -q 'path_allow = \[' "$WORK/hooks-registry.toml"
}

# ---------------------------------------------------------------------------
# AC-1 / Scenario A: production-shape registry + valid STATE.md => exit 0
# Proves host::read_file succeeds with production path_allow = [".factory"]
# ---------------------------------------------------------------------------

@test "PROD-REGISTRY: hook emits Continue for valid STATE.md using production path_allow" {
  _require_artifacts
  cp "$FIXTURE_VALID/STATE.md" "$WORK/.factory/STATE.md" || { mkdir -p "$WORK/.factory"; cp "$FIXTURE_VALID/STATE.md" "$WORK/.factory/STATE.md"; }
  _write_production_registry || {
    echo "production registry validation failed — test cannot proceed" >&2
    return 1
  }
  cp "$WASM_PLUGIN" "$WORK/hook-plugins/"

  local envelope
  envelope="$(_state_md_envelope "prod-valid")"
  run bash -c "printf '%s' '$envelope' | CLAUDE_PLUGIN_ROOT='$WORK' CLAUDE_PROJECT_DIR='$WORK' '$DISPATCHER' 2>&1 >/dev/null"

  [ "$status" -eq 0 ]
  [[ "$output" != *"blocking_plugins=validate-trajectory-tail-cell-completeness"* ]]
}

# ---------------------------------------------------------------------------
# AC-1 / Scenario B: production-shape registry + invalid STATE.md => exit 2 (Block)
# LOAD-BEARING: if path_allow uses "**" glob, canonicalize() fails => CapabilityDenied =>
# fail-open => exit 0 instead of 2 => TEST FAILS (S-15.11 preemptive regression guard)
# ---------------------------------------------------------------------------

@test "PROD-REGISTRY: hook blocks for invalid STATE.md using production path_allow (not fail-open)" {
  _require_artifacts
  mkdir -p "$WORK/.factory"
  cp "$FIXTURE_INVALID/STATE.md" "$WORK/.factory/STATE.md"
  _write_production_registry || {
    echo "production registry validation failed — test cannot proceed" >&2
    return 1
  }
  cp "$WASM_PLUGIN" "$WORK/hook-plugins/"

  local envelope
  envelope="$(_state_md_envelope "prod-invalid")"
  run bash -c "printf '%s' '$envelope' | CLAUDE_PLUGIN_ROOT='$WORK' CLAUDE_PROJECT_DIR='$WORK' '$DISPATCHER' 2>&1 >/dev/null"

  # Exit 2: invalid STATE.md => Block. Exit 0 means hook silently fail-opened
  # due to CapabilityDenied from misconfigured path_allow — S-15.11 regression.
  [ "$status" -eq 2 ]
  [[ "$output" == *"blocking_plugins=validate-trajectory-tail-cell-completeness"* ]]
}
