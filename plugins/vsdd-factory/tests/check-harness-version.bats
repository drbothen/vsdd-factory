#!/usr/bin/env bats
# check-harness-version.bats — Integration tests for S-18.00 AC-008 / BC-1.15.001 INV3.
#
# Covers 3 tests (all GREEN):
#
#   test_harness_version_check_passes (AC-008 / INV3):
#     check-harness-version.sh must exit 0 when CLAUDE_CODE_VERSION >= v2.1.105.
#     Direct script invocation (unit-level). Delivered: real version detection.
#
#   test_harness_version_check_advisory_on_missing (AC-008 / INV3 / EC-005):
#     check-harness-version.sh must exit 1 (advisory) when harness version
#     cannot be determined. Direct script invocation. Delivered: exits 1 with advisory.
#
#   test_check_harness_version_registry_entry_has_correct_shape (AC-008 / INV3):
#     Production hooks-registry.toml entry has PreCompact + on_error=continue + priority=50.
#     Pure grep/awk structural test.
#
#   TC-ENV-001 (F-S1800-P7-001 regression guard):
#     THROUGH-DISPATCHER env-forwarding test. Exercises the REAL production path by
#     pointing CLAUDE_PLUGIN_ROOT at the real plugins/vsdd-factory directory so the
#     dispatcher reads the actual hooks-registry.toml env_allow. Asserts plugin.completed
#     with exit_code:0 through the dispatcher's exec_subprocess env-clear gate.
#     GREEN against the production fix (afbb0d4c); goes RED if env_allow regresses.
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
    echo "(block-intent is reserved for the precompact-flush PreCompact WASM plugin per S-18.04a)"
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
# TC-ENV-001 (F-S1800-P7-001 regression guard):
#   THROUGH-DISPATCHER env-forwarding test for check-harness-version.
#
# This test validates the ACTUAL production registry's env_allow so it couples
# directly to the production fix (commit afbb0d4c). It does NOT use a hardcoded
# fixture — it points CLAUDE_PLUGIN_ROOT at the real plugins/vsdd-factory
# directory so the dispatcher reads the production hooks-registry.toml.
#
# The production fix (afbb0d4c) added CLAUDE_CODE_VERSION and CLAUDE_VERSION to
# both [hooks.capabilities] env_allow and [hooks.capabilities.exec_subprocess]
# env_allow arrays in the real registry's check-harness-version entry. With that fix in
# place, the dispatcher forwards CLAUDE_CODE_VERSION through the exec_subprocess
# env-clear gate and the script exits 0 (supported version). This test is GREEN
# against the fixed production registry.
#
# Genuine regression guard: if anyone removes CLAUDE_CODE_VERSION from the real
# registry's env_allow, this test immediately goes RED — because CLAUDE_PLUGIN_ROOT
# points at the real production registry, not a frozen copy.
#
# Production path exercised:
#   harness-env (CLAUDE_CODE_VERSION=2.1.177)
#   → dispatcher process (reads real hooks-registry.toml via CLAUDE_PLUGIN_ROOT)
#   → exec_subprocess env-clear + env_allow filter
#   → bash check-harness-version.sh (from real hooks/check-harness-version.sh)
#   → exit 0 (version >= v2.1.105)
#   → observable in dispatcher internal log: plugin.completed exit_code:0
#
# Log isolation: VSDD_LOG_DIR is set to $WORK/.factory/logs so the dispatcher
# internal log is written to the temp workdir (not the production .factory/logs).
# CLAUDE_PROJECT_DIR is also $WORK so Level-C log resolution doesn't produce
# a .factory/logs inside the real repo.
#
# Finding: F-S1800-P7-001 (closed by production fix afbb0d4c)
# BC: BC-1.15.001 INV3 (harness-version check is non-blocking but must detect)
# AC: AC-008 (exits 0 if harness >= v2.1.105)
# VP: VP-086 (dispatcher binary-level harness; exec_subprocess env-forwarding)
# ---------------------------------------------------------------------------

# Helper: require the dispatcher binary, adapter WASM, and real hook script.
# Mirrors the _require_artifacts() pattern from precompact-routing.bats.
_require_dispatcher_artifacts() {
  if [ ! -x "$DISPATCHER" ]; then
    skip "dispatcher binary not built — run: cargo build --release -p factory-dispatcher"
  fi
  if [ ! -f "$ADAPTER_WASM" ]; then
    skip "legacy-bash-adapter.wasm not present — build hook-plugins"
  fi
  if [ ! -f "$SCRIPT" ]; then
    skip "check-harness-version.sh not found — S-18.00 implementation required"
  fi
}

@test "TC-ENV-001: check-harness-version sees CLAUDE_CODE_VERSION through real production registry env_allow" {
  _require_dispatcher_artifacts

  # Point CLAUDE_PLUGIN_ROOT at the REAL production plugin root so the dispatcher
  # reads the actual hooks-registry.toml (with the fixed env_allow per commit afbb0d4c).
  # The real hooks/ and hook-plugins/ subdirectories
  # are resolved relative to CLAUDE_PLUGIN_ROOT, so the production script and WASM
  # are used directly — no copying, no frozen fixture.
  #
  # The dispatcher is run with CWD set to the real plugin root so that the
  # resolvers-registry.toml's relative WASM path (hook-plugins/vsdd-context-resolvers.wasm)
  # resolves correctly — the resolver loader calls path.canonicalize() against CWD.
  #
  # VSDD_LOG_DIR and CLAUDE_PROJECT_DIR are set to $WORK so the internal log is
  # written to the isolated temp directory (not the real .factory/logs).
  local real_plugin_root="$REPO_ROOT/plugins/vsdd-factory"

  run env \
    CLAUDE_CODE_VERSION="2.1.177" \
    VSDD_LOG_DIR="$WORK/.factory/logs" \
    CLAUDE_PLUGIN_ROOT="$real_plugin_root" \
    CLAUDE_PROJECT_DIR="$WORK" \
    bash -c "cd '$real_plugin_root' && printf '%s' '{\"event_name\":\"PreCompact\",\"tool_name\":\"\",\"session_id\":\"tc-env-001\",\"tool_input\":{}}' | '$DISPATCHER'" 2>&1

  # The dispatcher itself exits 0 (on_error=continue suppresses block even when
  # the script exits 1). We cannot use $status to distinguish pass vs advisory.
  # Instead we inspect the internal log: plugin.completed with exit_code:0 proves
  # the script reached the "version supported" path through the real env-clear gate.
  local log
  log="$(ls "$WORK/.factory/logs/dispatcher-internal-"*.jsonl 2>/dev/null | head -1)"

  [ -n "$log" ] || {
    echo "FAIL: dispatcher did not write an internal log — was the dispatcher invoked correctly?"
    echo "DISPATCHER=$DISPATCHER"
    echo "CLAUDE_PLUGIN_ROOT=$real_plugin_root"
    echo "WORK=$WORK"
    echo "Output: $output"
    return 1
  }

  # Verify the plugin was actually invoked (not silently skipped due to routing failure).
  # This guards against the test becoming a no-op if the registry path is wrong.
  local invoked
  invoked="$(grep -c '"type":"plugin.invoked"' "$log" || true)"
  [ "$invoked" -ge 1 ] || {
    echo "FAIL: check-harness-version plugin was not invoked — registry routing failed."
    echo "Expected plugin.invoked event in internal log."
    echo "CLAUDE_PLUGIN_ROOT=$real_plugin_root"
    echo "Verify hooks-registry.toml is present there and the check-harness-version entry exists."
    echo "Log: $log"
    echo "Log contents:"
    cat "$log"
    echo "Dispatcher output: $output"
    return 1
  }

  # Core assertion: plugin.completed with exit_code:0 proves the script saw
  # CLAUDE_CODE_VERSION=2.1.177 (>= threshold v2.1.105) through the dispatcher's
  # exec_subprocess env-clear + env_allow gate, and exited with "version supported".
  #
  # This test is GREEN because the production registry (commit afbb0d4c) includes
  # CLAUDE_CODE_VERSION in both env_allow arrays.
  #
  # Regression: if CLAUDE_CODE_VERSION is removed from the real registry's env_allow,
  # this test goes RED — the script falls through to advisory exit-1, producing
  # exit_code:1 in the log, and the grep below finds zero matches.
  local plugin_exit_zero
  plugin_exit_zero="$(grep -c '"type":"plugin.completed".*"exit_code":0' "$log" || true)"

  [ "$plugin_exit_zero" -ge 1 ] || {
    echo "FAIL: check-harness-version exited non-zero through the dispatcher."
    echo ""
    echo "CLAUDE_CODE_VERSION=2.1.177 was exported into the dispatcher process but the"
    echo "script could not see it — the exec_subprocess env-clear gate stripped it."
    echo ""
    echo "This means CLAUDE_CODE_VERSION and/or CLAUDE_VERSION is MISSING from the"
    echo "production registry's env_allow (regression against commit afbb0d4c)."
    echo ""
    echo "Registry read: $real_plugin_root/hooks-registry.toml"
    echo "  Check the [hooks.capabilities] env_allow and [hooks.capabilities.exec_subprocess]"
    echo "  env_allow arrays for the 'check-harness-version' entry."
    echo "  Both arrays must include CLAUDE_CODE_VERSION and CLAUDE_VERSION."
    echo ""
    echo "Expected: plugin.completed with exit_code:0 (version supported path)"
    echo "Actual: no plugin.completed with exit_code:0 found in log"
    echo ""
    echo "Log entries:"
    grep '"type":"plugin.completed"' "$log" || echo "(no plugin.completed entries)"
    return 1
  }
}
