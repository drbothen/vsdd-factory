#!/usr/bin/env bats
# warn-pending-wave-gate integration tests (S-19.03).
#
# T-007 (AC-005): positive-control test — when wave-state.yaml exists with
#   gate_status: pending, the plugin emits a warning to stderr and exits 0.
#   Red Gate: T-007 MAY PASS with the pre-built WASM (existing logic handles
#   this path correctly); it is a positive-control that validates preserved
#   behavior and must still pass after S-19.03 implementation.
#
# T-008 (AC-006): absent-file test — when wave-state.yaml is absent, zero
#   `internal.capability_denied reason=path_not_allowed` events are emitted.
#   Red Gate: FAILS with the pre-built WASM because the old path_allowed()
#   uses canonicalize() which fails for absent files → emits capability_denied
#   reason=path_not_allowed (the rc.22 smoke FINDING-2 defect, trace bc687a0f).
#
# Dispatcher invocation pattern: feed a Stop event to stdin of the dispatcher
# binary with CLAUDE_PLUGIN_ROOT pointing at the real plugin root. The
# dispatcher runs the warn-pending-wave-gate WASM plugin from the pre-built
# hook-plugins/ directory.
#
# Prerequisites at Red Gate:
#   - target/debug/factory-dispatcher (debug build, available from prior cargo build)
#   - plugins/vsdd-factory/hook-plugins/warn-pending-wave-gate.wasm (pre-built)
#
# Prerequisites post-implementation:
#   - rebuild WASM: cargo build --target wasm32-wasip1 -p warn-pending-wave-gate
#   - rebuild dispatcher: cargo build -p factory-dispatcher

# ---------------------------------------------------------------------------
# Setup / teardown helpers
# ---------------------------------------------------------------------------

setup() {
  REPO_ROOT="$(cd "${BATS_TEST_DIRNAME}/../../.." && pwd)"
  PLUGIN_ROOT="${REPO_ROOT}/plugins/vsdd-factory"
  WASM="${PLUGIN_ROOT}/hook-plugins/warn-pending-wave-gate.wasm"

  # Prefer debug build (built by cargo test --workspace); fall back to
  # platform-specific pre-built binary.
  local platform
  platform="$(uname -s)-$(uname -m)"
  case "$platform" in
    Darwin-arm64)  DISPATCHER="${REPO_ROOT}/plugins/vsdd-factory/hooks/dispatcher/bin/darwin-arm64/factory-dispatcher" ;;
    Darwin-x86_64) DISPATCHER="${REPO_ROOT}/plugins/vsdd-factory/hooks/dispatcher/bin/darwin-x64/factory-dispatcher" ;;
    Linux-x86_64)  DISPATCHER="${REPO_ROOT}/plugins/vsdd-factory/hooks/dispatcher/bin/linux-x64/factory-dispatcher" ;;
    Linux-aarch64) DISPATCHER="${REPO_ROOT}/plugins/vsdd-factory/hooks/dispatcher/bin/linux-arm64/factory-dispatcher" ;;
    *)             DISPATCHER="${REPO_ROOT}/target/debug/factory-dispatcher" ;;
  esac
  # Prefer debug build if it exists (most up-to-date)
  if [ -x "${REPO_ROOT}/target/debug/factory-dispatcher" ]; then
    DISPATCHER="${REPO_ROOT}/target/debug/factory-dispatcher"
  fi

  TEST_PROJECT_DIR="$(mktemp -d)"
  mkdir -p "${TEST_PROJECT_DIR}/.factory"
  LOG_DIR="${TEST_PROJECT_DIR}/.factory/logs"
  mkdir -p "${LOG_DIR}"
  export TEST_PROJECT_DIR LOG_DIR REPO_ROOT PLUGIN_ROOT DISPATCHER WASM
}

teardown() {
  rm -rf "${TEST_PROJECT_DIR:-/tmp/nonexistent-bats-cleanup}"
}

# Fail with a clear message if either required artifact is missing.
_require_dispatcher_and_wasm() {
  if [ ! -x "$DISPATCHER" ]; then
    echo "FAIL: factory-dispatcher binary not found or not executable at: $DISPATCHER"
    echo "Implementer: run 'cargo build -p factory-dispatcher' (or 'cargo build --workspace')"
    echo "to build the debug binary before running bats T-007/T-008."
    return 1
  fi
  if [ ! -f "$WASM" ]; then
    echo "FAIL: warn-pending-wave-gate.wasm not found at: $WASM"
    echo "Implementer: run 'cargo build --target wasm32-wasip1 -p warn-pending-wave-gate'"
    echo "to build the WASM before running bats T-007/T-008."
    return 1
  fi
}

# Invoke the dispatcher with a Stop event and capture all output.
# Sets $output (combined stdout+stderr) and $status per bats `run` convention.
_run_dispatcher_stop_event() {
  local stop_event='{"event_name":"Stop","tool_name":"","session_id":"bats-s19-03","dispatcher_trace_id":"bats-trace-s1903","tool_input":{}}'
  run env \
    VSDD_LOG_DIR="${LOG_DIR}" \
    CLAUDE_PLUGIN_ROOT="${PLUGIN_ROOT}" \
    CLAUDE_PROJECT_DIR="${TEST_PROJECT_DIR}" \
    bash -c "cd '${PLUGIN_ROOT}' && printf '%s' '${stop_event}' | '${DISPATCHER}'" 2>&1
}

# ---------------------------------------------------------------------------
# T-007 — AC-005: positive-control — pending gate → warning + exit 0
# ---------------------------------------------------------------------------

@test "T-007: warn-pending-wave-gate: wave-state.yaml with gate_status:pending emits WAVE GATE REMINDER to stderr and exits 0" {
  _require_dispatcher_and_wasm

  # Create wave-state.yaml with a pending gate.
  cat > "${TEST_PROJECT_DIR}/.factory/wave-state.yaml" <<'YAML'
waves:
  - wave: "W-01"
    gate_status: pending
YAML

  _run_dispatcher_stop_event

  # AC-005: dispatcher exits 0 (on_error=continue; plugin advisory).
  [ "$status" -eq 0 ] || {
    echo "FAIL T-007: dispatcher exited $status (expected 0)"
    echo "Output: $output"
    return 1
  }

  # AC-005: stderr must contain the WAVE GATE REMINDER.
  # With the old WASM: this PASSES (the plugin reads the file and finds pending gate).
  # With the new WASM (post-S-19.03): this also PASSES (same path, now via error dispatch).
  [[ "$output" == *"WAVE GATE REMINDER"* ]] || {
    echo "FAIL T-007 AC-005: output does not contain 'WAVE GATE REMINDER'"
    echo "Full output: $output"
    return 1
  }
}

# ---------------------------------------------------------------------------
# T-008 — AC-006: absent file → zero capability_denied reason=path_not_allowed
# ---------------------------------------------------------------------------

@test "T-008: warn-pending-wave-gate: absent wave-state.yaml emits zero internal.capability_denied reason=path_not_allowed events" {
  _require_dispatcher_and_wasm

  # wave-state.yaml is intentionally NOT created — fresh install / no wave handed off.
  # After S-19.03 fix: path_allowed uses rejoin algorithm → returns true for absent allowlisted
  # file → read_bounded returns ReadErr::NotFound → internal.file_not_found emitted (NOT
  # capability_denied). Plugin receives NOT_FOUND → exits silently.
  #
  # Red Gate (pre-fix): old path_allowed calls canonicalize() which fails for absent files
  # → returns false → prepare() emits internal.capability_denied reason=path_not_allowed.
  # This test FAILS at Red Gate because the sink will contain a capability_denied event.

  _run_dispatcher_stop_event

  # Dispatcher exits 0 regardless (on_error=continue).
  [ "$status" -eq 0 ] || {
    echo "FAIL T-008: dispatcher exited $status (expected 0 — on_error=continue)"
    echo "Output: $output"
    return 1
  }

  # AC-006: check internal log for capability_denied reason=path_not_allowed events
  # for this plugin. The internal log is in JSONL format at LOG_DIR.
  local SINK_FILE
  SINK_FILE="$(ls "${LOG_DIR}"/dispatcher-internal-*.jsonl 2>/dev/null | head -1)"

  if [ -z "$SINK_FILE" ]; then
    # No log file means no events — zero capability_denied events is satisfied.
    return 0
  fi

  # AC-006 gate (F-P15-007 form): count path_not_allowed events for this plugin.
  # Uses jq-e + wc-l to avoid grep -c exit-1-on-zero-matches defect.
  local cap_denied_count
  if ! jq -e 'true' < "$SINK_FILE" > /dev/null 2>&1; then
    # Non-JSON or empty log; no capability_denied events.
    cap_denied_count=0
  else
    cap_denied_count=$(
      jq -r 'select(
        .type == "internal.capability_denied" and
        .reason == "path_not_allowed" and
        .plugin_name == "warn-pending-wave-gate"
      ) | .plugin_name' "$SINK_FILE" 2>/dev/null | wc -l
    )
  fi

  [ "$cap_denied_count" -eq 0 ] || {
    echo "FAIL T-008 AC-006: found ${cap_denied_count} 'internal.capability_denied reason=path_not_allowed'"
    echo "event(s) for plugin_name=warn-pending-wave-gate — this is the rc.22 smoke FINDING-2"
    echo "false-positive defect (dispatcher trace bc687a0f). Fix: S-19.03 implementation."
    echo "Log file: $SINK_FILE"
    echo "Relevant log entries:"
    jq -c 'select(.type == "internal.capability_denied" and .plugin_name == "warn-pending-wave-gate")' \
      "$SINK_FILE" 2>/dev/null || true
    return 1
  }
}
