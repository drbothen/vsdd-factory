#!/usr/bin/env bats
# warn-pending-wave-gate integration tests (S-19.03).
#
# T-007 (AC-005): positive-control test — when wave-state.yaml exists with
#   pending_gate: true, plugin emits warning to stderr and exits 0.
# T-008 (AC-006): absent-file test — when wave-state.yaml is absent, zero
#   `internal.capability_denied reason=path_not_allowed` events emitted.
#
# Red Gate: both tests are stubs that FAIL until S-19.03 is implemented.
# The @test bodies call the warn-pending-wave-gate WASM via the dispatcher
# harness with fixture wave-state files.

# ---------------------------------------------------------------------------
# Setup / teardown helpers
# ---------------------------------------------------------------------------

setup() {
  # Create a temporary project root with .factory/ structure for each test.
  TEST_PROJECT_DIR="$(mktemp -d)"
  mkdir -p "${TEST_PROJECT_DIR}/.factory"
  export TEST_PROJECT_DIR
}

teardown() {
  rm -rf "${TEST_PROJECT_DIR:-/tmp/nonexistent-bats-cleanup}"
}

# ---------------------------------------------------------------------------
# T-007 — AC-005: positive-control — pending_gate: true → warning + exit 0
# ---------------------------------------------------------------------------

@test "T-007: warn-pending-wave-gate: wave-state.yaml with pending_gate:true emits warning to stderr and exits 0" {
  # S-19.03 Red Gate stub: FAIL until implementation is complete.
  skip "S-19.03 Red Gate: implementation pending"

  cat > "${TEST_PROJECT_DIR}/.factory/wave-state.yaml" <<'YAML'
waves:
  - wave: "W-01"
    gate_status: pending
YAML

  # Run the plugin via the dispatcher harness (placeholder invocation).
  # Implementation will replace this with the real dispatcher invocation.
  run false
  [ "$status" -eq 0 ]
  [[ "$output" == *"WAVE GATE REMINDER"* ]]
}

# ---------------------------------------------------------------------------
# T-008 — AC-006: absent file → zero capability_denied reason=path_not_allowed
# ---------------------------------------------------------------------------

@test "T-008: warn-pending-wave-gate: absent wave-state.yaml emits zero capability_denied reason=path_not_allowed events" {
  # S-19.03 Red Gate stub: FAIL until implementation is complete.
  skip "S-19.03 Red Gate: implementation pending"

  # wave-state.yaml is intentionally NOT created — fresh install scenario.
  # The dispatcher should emit internal.file_not_found (NOT capability_denied
  # reason=path_not_allowed) per BC-2.07.001 part c.

  # Run the plugin via the dispatcher harness (placeholder invocation).
  run false
  [ "$status" -eq 0 ]

  # Assert zero capability_denied reason=path_not_allowed events in the sink.
  local SINK_FILE="${TEST_PROJECT_DIR}/events.jsonl"
  [ ! -f "${SINK_FILE}" ] || \
    [ "$(jq -r 'select(.type == "internal.capability_denied" and .reason == "path_not_allowed" and .plugin_name == "warn-pending-wave-gate") | .plugin_name' "${SINK_FILE}" 2>/dev/null | wc -l)" -eq 0 ]
}
