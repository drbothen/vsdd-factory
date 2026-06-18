#!/usr/bin/env bats
# check-harness-version.bats — Red Gate bats tests for S-18.00 AC-008 / BC-1.15.001 INV3.
#
# Covers the 2 bats integration cases from S-18.00 (both now GREEN):
#
#   test_harness_version_check_passes (AC-008 / INV3):
#     check-harness-version.sh must exit 0 when harness >= v2.1.105.
#     Delivered: real version detection; exits 0 when CLAUDE_CODE_VERSION >= threshold.
#
#   test_harness_version_check_advisory_on_missing (AC-008 / INV3 / EC-005):
#     check-harness-version.sh must exit 1 (advisory) when harness version
#     cannot be determined. Delivered: exits 1 with advisory message per BC-1.15.001 INV3.
#
# Story: S-18.00 — Dispatcher PreCompact/PostCompact Routing + check-harness-version.sh
# BC:    BC-1.15.001 INV3 — Harness-version precondition is non-blocking at dispatcher level
# AC:    AC-008 — check-harness-version.sh registered as PreCompact plugin with on_error=continue;
#        exits 0 if harness >= v2.1.105; exits 1 (advisory) if version undeterminable or below threshold.
#
# Edge Cases exercised:
#   EC-005: check-harness-version.sh cannot determine harness version → exits 1 (advisory)
#   EC-006: harness below v2.1.105 → exits 1 (advisory); non-blocking
#
# RED GATE strategy (historical — S-18.00 is now implemented and all tests pass GREEN):
#   At Red Gate the stub script exited 1 unconditionally. The tests were designed to:
#   1. test_harness_version_check_passes: assert exit 0 — failed because stub always exited 1.
#   2. test_harness_version_check_advisory_on_missing: assert the ADVISORY message format —
#      failed because the stub emitted "not yet implemented" instead of the real advisory.
#   The real implementation in hooks/check-harness-version.sh satisfies both assertions.
#
# Both tests are load-bearing: they exercise the real script, not a self-constructed value.
#
# Invocation pattern: direct script invocation (no dispatcher needed for these unit-level
# bats tests). The script is a bash hook; it reads environment for harness version detection.
#
# File location: plugins/vsdd-factory/tests/check-harness-version.bats
# (FLAT path — discovered by run-all.sh `tests/*.bats` glob)
#
# Run:
#   bats plugins/vsdd-factory/tests/check-harness-version.bats

# ---------------------------------------------------------------------------
# setup / teardown
# ---------------------------------------------------------------------------

setup() {
  REPO_ROOT="$(cd "${BATS_TEST_DIRNAME}/../../.." && pwd)"
  SCRIPT="$REPO_ROOT/plugins/vsdd-factory/hooks/check-harness-version.sh"
  DISPATCHER="$REPO_ROOT/target/release/factory-dispatcher"
  ADAPTER_WASM="$REPO_ROOT/plugins/vsdd-factory/hook-plugins/legacy-bash-adapter.wasm"

  WORK="$(mktemp -d)"
  mkdir -p "$WORK/.factory/logs" "$WORK/hook-plugins" "$WORK/hooks"

  # Copy the real check-harness-version.sh into the WORK/hooks directory so
  # the dispatcher can resolve it via the registry's relative script_path.
  if [ -f "$SCRIPT" ]; then
    cp "$SCRIPT" "$WORK/hooks/check-harness-version.sh"
    chmod +x "$WORK/hooks/check-harness-version.sh"
  fi

  # Copy the legacy-bash-adapter.wasm so registry plugin paths resolve correctly.
  if [ -f "$ADAPTER_WASM" ]; then
    cp "$ADAPTER_WASM" "$WORK/hook-plugins/legacy-bash-adapter.wasm"
  fi
}

teardown() {
  rm -rf "$WORK"
}

# ---------------------------------------------------------------------------
# Preflight helper
# ---------------------------------------------------------------------------

# Verify the check-harness-version.sh script exists and is executable.
# The script must be present and executable — absence is a hard failure here.
_require_script() {
  if [ ! -f "$SCRIPT" ]; then
    echo "FAIL: check-harness-version.sh not found at: $SCRIPT"
    echo "Implementer: create plugins/vsdd-factory/hooks/check-harness-version.sh (S-18.00 T-6)"
    return 1
  fi
  if [ ! -x "$SCRIPT" ]; then
    echo "FAIL: check-harness-version.sh exists but is not executable: $SCRIPT"
    echo "Implementer: run: chmod +x plugins/vsdd-factory/hooks/check-harness-version.sh"
    return 1
  fi
}

# ---------------------------------------------------------------------------
# T-1 (AC-008 / INV3): Script exits 0 when harness version is >= v2.1.105.
#
# BC-1.15.001 INV3: "check-harness-version.sh is registered as a PreCompact plugin
# with on_error=continue. The script exits 0 if the harness reports
# claude-code >= v2.1.105 (or equivalent)."
#
# AC-008: "exits 0 if the harness reports claude-code >= v2.1.105 (or equivalent)"
#
# This test simulates a harness-present environment by setting the environment
# variable CLAUDE_CODE_VERSION to a known-valid version (v2.1.177, which is the
# production version per BC-1.15.001 §Preconditions: "confirmed in production:
# v2.1.177 per F1 delta analysis").
#
# Delivered implementation exits 0 when CLAUDE_CODE_VERSION >= v2.1.105 (AC-008).
# ---------------------------------------------------------------------------

@test "test_harness_version_check_passes" {
  _require_script

  # Simulate a harness-present environment with a version above the v2.1.105 threshold.
  # BC-1.15.001 §Preconditions: "confirmed in production: v2.1.177 per F1 delta analysis"
  # The implementer's real script must detect this version and exit 0.
  #
  # If the script queries CLAUDE_CODE_VERSION env var, this provides the answer.
  # If it uses a different mechanism (claude --version, etc.), the detection method
  # is documented in hooks/check-harness-version.sh — delivered and GREEN.
  run env CLAUDE_CODE_VERSION="2.1.177" bash "$SCRIPT" 2>&1

  # Must exit 0 — harness >= v2.1.105 (advisory version check passed).
  [ "$status" -eq 0 ] || {
    echo "FAIL: expected exit 0 (harness version check passed) but got status=$status"
    echo "CLAUDE_CODE_VERSION was set to 2.1.177 (>= threshold v2.1.105)"
    echo "AC-008: script exits 0 when harness reports claude-code >= v2.1.105"
    echo "BC-1.15.001 INV3: harness-version precondition check"
    echo "Output: $output"
    return 1
  }

  # Verify the script does not emit a stub/placeholder message (regression guard).
  [[ "$output" != *"stub not yet implemented"* ]] || {
    echo "FAIL: script emitted stub placeholder message — regression detected."
    echo "Output: $output"
    return 1
  }
}

# ---------------------------------------------------------------------------
# T-2 (AC-008 / INV3 / EC-005): Script exits 1 (advisory) when harness version
#     cannot be determined.
#
# BC-1.15.001 INV3: "exits 1 (advisory) if harness version cannot be determined
# or is below threshold. The dispatcher continues even if check-harness-version.sh
# exits non-zero."
#
# AC-008: "exits 1 (advisory) if harness version cannot be determined or is below threshold"
#
# EC-005: "check-harness-version.sh cannot determine harness version → exits 1 (advisory);
# dispatcher continues"
#
# This test runs the script without any harness-version environment variable set,
# simulating a pre-v2.1.105 harness or an environment where the version is
# undeterminable.
#
# Delivered implementation exits 1 with the correct advisory message when version
# is undeterminable (AC-008 / EC-005). The message content assertion is load-bearing.
# ---------------------------------------------------------------------------

@test "test_harness_version_check_advisory_on_missing" {
  _require_script

  # Run the script with no harness-version environment variable.
  # This simulates an environment where the harness version cannot be determined.
  # Unset any potential version env vars to ensure the script gets a clean state.
  run env -u CLAUDE_CODE_VERSION -u CLAUDE_VERSION bash "$SCRIPT" 2>&1

  # Must exit 1 — harness version undeterminable; advisory (non-blocking).
  # The load-bearing assertion is the message check below.
  [ "$status" -eq 1 ] || {
    echo "FAIL: expected exit 1 (advisory — version undeterminable) but got status=$status"
    echo "AC-008: exits 1 (advisory) if harness version cannot be determined or is below threshold"
    echo "EC-005: harness version undeterminable → exits 1 advisory"
    echo "NOTE: exit code 2 would be incorrect — check-harness-version.sh never exits 2"
    echo "(block-intent is reserved for precompact-flush.sh per S-18.04a)"
    echo "Output: $output"
    return 1
  }

  # The advisory message must match the delivered implementation's format.
  # BC-1.15.001 INV3: the script must emit an informative advisory message, not a
  # developer-facing placeholder, when it cannot determine the harness version.
  # This guard also prevents regression back to stub behaviour.
  [[ "$output" != *"stub not yet implemented"* ]] || {
    echo "FAIL: script emitted stub placeholder message — regression detected."
    echo "The implementation must emit an advisory message explaining the version check"
    echo "outcome, not a developer placeholder."
    echo "AC-008 / BC-1.15.001 INV3 not satisfied."
    echo "Output: $output"
    return 1
  }

  # Verify the exit-1 is accompanied by an informative advisory message.
  # The message should mention version or harness to be meaningful for operators.
  # This guards against an empty exit-1 that provides no diagnostic value.
  [[ "$output" == *"check-harness-version"* ]] || {
    echo "FAIL: advisory output does not identify the script by name ('check-harness-version')."
    echo "AC-008: the advisory message must be operator-actionable and identify the check."
    echo "Output: $output"
    return 1
  }
}

# ---------------------------------------------------------------------------
# Registry assertion test — check-harness-version entry in hooks-registry.toml
#
# AC-008 / BC-1.15.001 INV3: "check-harness-version.sh is registered as a
# PreCompact plugin with on_error=continue."
#
# Story T-7 (S-18.00): "Add hooks-registry.toml entry for check-harness-version
# (PreCompact, legacy-bash-adapter.wasm, on_error=continue, priority=50)"
#
# This test inspects the production hooks-registry.toml and verifies the entry
# is present with the correct shape. It does NOT require the dispatcher binary or any
# WASM artifact — it is a pure grep/awk structural test.
#
# Required entry shape:
#   name = "check-harness-version"
#   event = "PreCompact"
#   on_error = "continue"   (BC-1.15.001 INV3: dispatcher continues even on non-zero exit)
#   priority = 50           (S-18.00 T-7)
#
# Entry added by stub-architect at commit 36cff71f; real script wired at S-18.00 TDD green.
# ---------------------------------------------------------------------------

@test "test_check_harness_version_registry_entry_has_correct_shape" {
  local registry="$REPO_ROOT/plugins/vsdd-factory/hooks-registry.toml"

  [ -f "$registry" ] || {
    echo "FAIL: production hooks-registry.toml not found at: $registry"
    return 1
  }

  # Check that check-harness-version is registered.
  grep -q 'name = "check-harness-version"' "$registry" || {
    echo "FAIL: check-harness-version entry not found in production hooks-registry.toml."
    echo "Implementer: add the [[hooks]] entry per S-18.00 T-7 (BC-1.15.001 INV3)."
    echo "Registry: $registry"
    return 1
  }

  # Verify required fields within the check-harness-version section.
  # Score 3 for full compliance: event=PreCompact + on_error=continue + priority=50.
  local score
  score=$(awk '
    /name = "check-harness-version"/ {
      in_section = 1
      has_precompact_event = 0
      has_on_error_continue = 0
      has_priority_50 = 0
    }
    /^\[\[hooks\]\]/ && in_section && !/name = "check-harness-version"/ {
      in_section = 0
    }
    in_section && /event = "PreCompact"/ { has_precompact_event = 1 }
    in_section && /on_error = "continue"/ { has_on_error_continue = 1 }
    in_section && /priority = 50/ { has_priority_50 = 1 }
    END {
      total = has_precompact_event + has_on_error_continue + has_priority_50
      print total
    }
  ' "$registry")

  [ "$score" -eq 3 ] || {
    echo "FAIL: check-harness-version registry entry is incomplete (score=$score/3)."
    echo "Required fields:"
    echo "  1. event = \"PreCompact\"       (BC-1.15.001 INV1 — first-class event type)"
    echo "  2. on_error = \"continue\"      (BC-1.15.001 INV3 — non-blocking; dispatcher continues)"
    echo "  3. priority = 50              (S-18.00 T-7 spec)"
    echo "See BC-1.15.001 INV3 / S-18.00 T-7 / AC-008."
    echo "Registry: $registry"
    return 1
  }
}

# ---------------------------------------------------------------------------
# TC-ENV-001 (F-S1800-P7-001 / F-S1800-P8-001 Red Gate):
#   THROUGH-DISPATCHER env-forwarding test for check-harness-version.
#
# DEFECT: hooks-registry.toml [hooks.capabilities.exec_subprocess] env_allow
# omits CLAUDE_CODE_VERSION and CLAUDE_VERSION. The dispatcher does env_clear()
# then re-adds only the listed env_allow names before spawning the subprocess.
# Therefore check-harness-version.sh NEVER sees the version vars and ALWAYS
# exits 1 ("undeterminable") in production, even when the harness exports a
# supported version. Existing tests miss this because they invoke the script
# DIRECTLY via `env CLAUDE_CODE_VERSION=... bash $SCRIPT`, bypassing the
# dispatcher env-clear gate.
#
# This test exercises the REAL production path:
#   harness-env → dispatcher process → exec_subprocess env-clear + env_allow
#   filter → bash check-harness-version.sh → exit code observable in dispatcher log
#
# RED GATE ASSERTION: asserts plugin.completed with exit_code:0 — this MUST FAIL
# against current wiring because env_allow strips CLAUDE_CODE_VERSION, so the
# script exits 1, producing plugin.completed with exit_code:1.
#
# After the implementer adds CLAUDE_CODE_VERSION and CLAUDE_VERSION to env_allow
# in hooks-registry.toml (both [hooks.capabilities] and
# [hooks.capabilities.exec_subprocess] sections), the script will see the version
# and exit 0, and this test will turn GREEN.
#
# Registry shape: mirrors the ACTUAL production registry entry at
# hooks-registry.toml:1241-1259 with the exec_subprocess env_allow defect
# preserved (not fixed) — this is the defect the test must catch.
#
# Finding: F-S1800-P7-001 / F-S1800-P8-001 (two independent LOCAL adversaries)
# BC: BC-1.15.001 INV3 (harness-version check is non-blocking but must detect)
# AC: AC-008 (exits 0 if harness >= v2.1.105)
# VP: VP-086 (dispatcher binary-level harness; exec_subprocess env-forwarding)
# ---------------------------------------------------------------------------

# Helper: require the dispatcher binary and adapter WASM are present.
# Mirrors the _require_artifacts() pattern from precompact-routing.bats.
_require_dispatcher_artifacts() {
  if [ ! -x "$DISPATCHER" ]; then
    skip "dispatcher binary not built — run: cargo build --release -p factory-dispatcher"
  fi
  if [ ! -f "$WORK/hook-plugins/legacy-bash-adapter.wasm" ]; then
    skip "legacy-bash-adapter.wasm not present — build hook-plugins or copy to hook-plugins/"
  fi
  if [ ! -f "$WORK/hooks/check-harness-version.sh" ]; then
    skip "check-harness-version.sh not found — S-18.00 implementation required"
  fi
}

# Helper: write a registry that mirrors the ACTUAL production entry shape for
# check-harness-version, preserving the defective env_allow (no CLAUDE_CODE_VERSION).
# This is the shape that MUST trigger the Red Gate failure.
_write_defective_env_allow_registry() {
  cat > "$WORK/hooks-registry.toml" <<'EOF'
schema_version = 2

[[hooks]]
name = "check-harness-version"
event = "PreCompact"
plugin = "hook-plugins/legacy-bash-adapter.wasm"
priority = 50
timeout_ms = 5000
on_error = "continue"
async = false

[hooks.config]
script_path = "hooks/check-harness-version.sh"

[hooks.capabilities]
env_allow = ["PATH", "HOME", "TMPDIR", "CLAUDE_PROJECT_DIR", "CLAUDE_PLUGIN_ROOT"]

[hooks.capabilities.exec_subprocess]
binary_allow = ["bash"]
shell_bypass_acknowledged = "legacy-bash-adapter runs unported hooks"
env_allow = ["PATH", "HOME", "TMPDIR", "CLAUDE_PROJECT_DIR", "CLAUDE_PLUGIN_ROOT"]
EOF
}

@test "TC-ENV-001 (Red Gate): check-harness-version sees CLAUDE_CODE_VERSION through dispatcher env-clear gate" {
  _require_dispatcher_artifacts

  # Write the registry with the DEFECTIVE env_allow (current production shape:
  # CLAUDE_CODE_VERSION and CLAUDE_VERSION absent from env_allow).
  # This is intentional — the test must catch the env-clear bug.
  _write_defective_env_allow_registry

  # Export CLAUDE_CODE_VERSION into the DISPATCHER PROCESS environment.
  # The dispatcher does env_clear() then re-adds only env_allow names before
  # spawning the subprocess. With the current defective env_allow, this var
  # is stripped — the script never sees it and exits 1 (advisory).
  #
  # The test asserts the script reaches exit-0 (version check passed).
  # RED GATE: this assertion MUST FAIL against current wiring.
  # After env_allow is fixed (implementer adds CLAUDE_CODE_VERSION + CLAUDE_VERSION),
  # the script will see 2.1.177 >= 2.1.105 and exit 0 → test turns GREEN.
  run env \
    CLAUDE_CODE_VERSION="2.1.177" \
    VSDD_LOG_DIR="$WORK/.factory/logs" \
    CLAUDE_PLUGIN_ROOT="$WORK" \
    CLAUDE_PROJECT_DIR="$WORK" \
    bash -c "printf '%s' '{\"event_name\":\"PreCompact\",\"tool_name\":\"\",\"session_id\":\"tc-env-001\",\"tool_input\":{}}' | '$DISPATCHER'" 2>&1

  # The dispatcher itself exits 0 (on_error=continue suppresses block even if
  # the script exits 1). We cannot use $status to distinguish. Instead we must
  # inspect the internal log: plugin.completed with exit_code:0 proves the
  # script reached the "version supported" path through the real env-clear gate.
  local log
  log="$(ls "$WORK/.factory/logs/dispatcher-internal-"*.jsonl 2>/dev/null | head -1)"

  [ -n "$log" ] || {
    echo "FAIL: dispatcher did not write an internal log — was the dispatcher invoked correctly?"
    echo "DISPATCHER=$DISPATCHER"
    echo "WORK=$WORK"
    echo "Output: $output"
    return 1
  }

  # Verify the plugin was actually invoked (not silently skipped).
  # This guards against the test becoming a no-op if routing fails.
  local invoked
  invoked="$(grep -c '"type":"plugin.invoked"' "$log" || true)"
  [ "$invoked" -ge 1 ] || {
    echo "FAIL: check-harness-version plugin was not invoked — registry routing failed."
    echo "Expected plugin.invoked event in internal log."
    echo "Log: $log"
    echo "Log contents:"
    cat "$log"
    echo "Dispatcher output: $output"
    return 1
  }

  # Core Red Gate assertion: plugin.completed with exit_code:0 proves the script
  # saw CLAUDE_CODE_VERSION and exited with "version supported" (exit 0).
  #
  # With DEFECTIVE env_allow (current wiring): script exits 1 → exit_code:1 →
  # this assertion FAILS (Red Gate).
  #
  # With FIXED env_allow (after implementer adds the vars): script sees 2.1.177
  # >= 2.1.105 → exits 0 → exit_code:0 → this assertion PASSES (Green).
  local plugin_exit_zero
  plugin_exit_zero="$(grep -c '"type":"plugin.completed".*"exit_code":0' "$log" || true)"

  [ "$plugin_exit_zero" -ge 1 ] || {
    echo "FAIL (Red Gate): check-harness-version exited non-zero through the dispatcher."
    echo ""
    echo "This means CLAUDE_CODE_VERSION was stripped by the exec_subprocess env-clear gate."
    echo "The script could not see the version and fell through to the advisory exit-1 path."
    echo ""
    echo "Root cause (F-S1800-P7-001 / F-S1800-P8-001):"
    echo "  [hooks.capabilities.exec_subprocess] env_allow does NOT include CLAUDE_CODE_VERSION"
    echo "  or CLAUDE_VERSION. The dispatcher strips all env vars not in env_allow before"
    echo "  spawning the bash subprocess. Result: script always exits 1 in production."
    echo ""
    echo "Fix (implementer): add CLAUDE_CODE_VERSION and CLAUDE_VERSION to both"
    echo "  [hooks.capabilities] env_allow AND [hooks.capabilities.exec_subprocess] env_allow"
    echo "  in plugins/vsdd-factory/hooks-registry.toml (around line 1254 and 1259)."
    echo ""
    echo "CLAUDE_CODE_VERSION exported into dispatcher: 2.1.177 (>= threshold v2.1.105)"
    echo "Expected: plugin.completed with exit_code:0 (script reached supported-version path)"
    echo "Actual: no plugin.completed with exit_code:0 found in log"
    echo ""
    echo "Log entries:"
    grep '"type":"plugin.completed"' "$log" || echo "(no plugin.completed entries)"
    return 1
  }
}
