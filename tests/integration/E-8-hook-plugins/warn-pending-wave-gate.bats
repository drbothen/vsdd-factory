#!/usr/bin/env bats
# tests/integration/E-8-hook-plugins/warn-pending-wave-gate.bats
#
# Bats parity tests for warn-pending-wave-gate WASM hook plugin (S-8.07).
#
# Tests invoke the hook via the vsdd-factory dispatcher (production path) per
# AC-005. Direct .wasm invocation via wasmtime is NOT used for these tests.
#
# BC traces:
#   BC-7.03.091: identity & registry binding (AC-001, AC-004)
#   BC-7.03.092: stderr warning when any wave has gate_status: pending (AC-003, AC-005)
#
# Stop hook stdin envelope (per T-5 / AC-005):
#   {"hook_event_name": "Stop", "session_id": "test-session-001",
#    "transcript_path": "/tmp/test-transcript.jsonl"}
#
# RED GATE: all 5 tests fail until:
#   (a) the WASM plugin is built and deployed to the plugins directory, AND
#   (b) hooks-registry.toml is updated (T-6), AND
#   (c) the dispatcher binary is available on PATH.
#
# bats-assert and bats-support are required for assert_output/refute_output.
# Load them if available, else fall back to manual assertions.

# ---------------------------------------------------------------------------
# Setup / teardown
# ---------------------------------------------------------------------------

setup() {
    # Locate workspace root — bats sets BATS_TEST_FILENAME to the test file path.
    # Walk up from the test file to find the workspace root (contains Cargo.lock).
    local dir
    dir="$(dirname "$BATS_TEST_FILENAME")"
    while [[ "$dir" != "/" ]]; do
        if [[ -f "$dir/Cargo.lock" ]]; then
            WORKSPACE_ROOT="$dir"
            break
        fi
        dir="$(dirname "$dir")"
    done

    if [[ -z "${WORKSPACE_ROOT:-}" ]]; then
        skip "workspace root not found (Cargo.lock absent); cannot run integration tests"
    fi

    # Create a temporary directory for fixture files
    FIXTURE_DIR="$(mktemp -d)"

    # Set CLAUDE_PROJECT_DIR to fixture dir so dispatcher looks there for
    # .factory/wave-state.yaml
    CLAUDE_PROJECT_DIR="$FIXTURE_DIR"
    export CLAUDE_PROJECT_DIR

    # The Stop hook stdin envelope (per T-5 / AC-005)
    STOP_STDIN='{"hook_event_name": "Stop", "session_id": "test-session-001", "transcript_path": "/tmp/test-transcript.jsonl"}'

    # Dispatcher binary path — used to invoke the hook via production path.
    # Falls back to cargo-installed path in CARGO_TARGET_DIR if present.
    DISPATCHER_BIN="${WORKSPACE_ROOT}/target/debug/factory-dispatcher"
    if [[ ! -x "$DISPATCHER_BIN" ]]; then
        DISPATCHER_BIN="${WORKSPACE_ROOT}/target/release/factory-dispatcher"
    fi
}

teardown() {
    # Clean up fixture directory
    if [[ -n "${FIXTURE_DIR:-}" && -d "$FIXTURE_DIR" ]]; then
        rm -rf "$FIXTURE_DIR"
    fi
}

# ---------------------------------------------------------------------------
# Helper: write wave-state.yaml fixture to FIXTURE_DIR
# ---------------------------------------------------------------------------

write_wave_state() {
    local content="$1"
    mkdir -p "$FIXTURE_DIR/.factory"
    printf '%s' "$content" > "$FIXTURE_DIR/.factory/wave-state.yaml"
}

# ---------------------------------------------------------------------------
# Helper: invoke dispatcher with Stop envelope, capture exit code and stderr
# ---------------------------------------------------------------------------

run_hook() {
    # Invoke dispatcher with the Stop event via stdin.
    # The dispatcher routes to warn-pending-wave-gate per hooks-registry.toml.
    #
    # RED GATE: fails if dispatcher binary is absent or registry not updated.
    if [[ ! -x "$DISPATCHER_BIN" ]]; then
        fail "AC-005: dispatcher binary not found at $DISPATCHER_BIN — build with 'cargo build -p factory-dispatcher' before running bats tests"
    fi

    run bash -c "cd '$FIXTURE_DIR' && echo '$STOP_STDIN' | '$DISPATCHER_BIN' 2>&1"
}

# ---------------------------------------------------------------------------
# Test 1 (AC-005 case a): one pending wave → exit 0 + REMINDER stderr + comma-joined field
# ---------------------------------------------------------------------------

@test "AC-005(a): one pending wave → exit 0 + WAVE GATE REMINDER in stderr" {
    # Fixture: wave-state.yaml with one pending wave (BC-7.03.092 canonical test vector)
    write_wave_state "$(cat <<'YAML'
waves:
  W-15:
    gate_status: pending
    started: 2026-04-01
YAML
)"

    run_hook

    # AC-005 / BC-7.03.091 PC-2: always exits 0
    [ "$status" -eq 0 ]

    # AC-005 / BC-7.03.092 PC-1: stderr must contain WAVE GATE REMINDER header
    [[ "$output" == *"WAVE GATE REMINDER:"* ]] || \
        fail "AC-005(a): output must contain 'WAVE GATE REMINDER:'; got: $output"

    # BC-7.03.092 PC-1: wave line with exact format
    [[ "$output" == *"  - W-15 gate is pending. Run the gate before starting the next wave."* ]] || \
        fail "AC-005(a): output must contain W-15 reminder line; got: $output"

    # BC-7.03.092 PC-1: invocation hint
    [[ "$output" == *"Invoke /vsdd-factory:wave-gate"* ]] || \
        fail "AC-005(a): output must contain invocation hint; got: $output"
}

# ---------------------------------------------------------------------------
# Test 2 (AC-005 case b): two pending waves → exit 0 + both names in REMINDER
# ---------------------------------------------------------------------------

@test "AC-005(b): two pending waves → exit 0 + both wave names in REMINDER" {
    # Fixture: wave-state.yaml with two pending waves (EC-004)
    write_wave_state "$(cat <<'YAML'
waves:
  W-15:
    gate_status: pending
  W-16:
    gate_status: pending
YAML
)"

    run_hook

    # AC-005 / BC-7.03.091 PC-2: always exits 0
    [ "$status" -eq 0 ]

    # BC-7.03.092 PC-1 + EC-004: both wave names must appear in output
    [[ "$output" == *"W-15"* ]] || \
        fail "AC-005(b): output must contain 'W-15'; got: $output"

    [[ "$output" == *"W-16"* ]] || \
        fail "AC-005(b): output must contain 'W-16'; got: $output"

    # WAVE GATE REMINDER header must be present
    [[ "$output" == *"WAVE GATE REMINDER:"* ]] || \
        fail "AC-005(b): output must contain 'WAVE GATE REMINDER:'; got: $output"
}

# ---------------------------------------------------------------------------
# Test 3 (AC-005 case c): all waves passed → exit 0 + no stderr output
# ---------------------------------------------------------------------------

@test "AC-005(c): all waves passed → exit 0 + no REMINDER in output" {
    # Fixture: wave-state.yaml with all waves passed
    write_wave_state "$(cat <<'YAML'
waves:
  W-14:
    gate_status: passed
  W-15:
    gate_status: passed
YAML
)"

    run_hook

    # BC-7.03.091 PC-2: always exits 0
    [ "$status" -eq 0 ]

    # BC-7.03.091 PC-2(c): no pending waves → no REMINDER output
    [[ "$output" != *"WAVE GATE REMINDER:"* ]] || \
        fail "AC-005(c): output must NOT contain 'WAVE GATE REMINDER:' when all waves passed; got: $output"

    # Output should be empty (silent exit)
    [ -z "$output" ] || \
        fail "AC-005(c): output must be empty when no pending waves; got: $output"
}

# ---------------------------------------------------------------------------
# Test 4 (AC-005 case d): wave-state.yaml absent → exit 0 + no output
# ---------------------------------------------------------------------------

@test "AC-005(d): wave-state.yaml absent → exit 0 + no output" {
    # Do NOT create .factory/wave-state.yaml (fixture dir is empty)
    # BC-7.03.091 PC-2(a): absent wave-state.yaml → silent exit 0

    run_hook

    # Always exits 0
    [ "$status" -eq 0 ]

    # No output at all
    [ -z "$output" ] || \
        fail "AC-005(d): output must be empty when wave-state.yaml absent; got: $output"
}

# ---------------------------------------------------------------------------
# Test 5 (AC-005 case e): malformed YAML → exit 0 + no output
# ---------------------------------------------------------------------------

@test "AC-005(e): malformed YAML → exit 0 + no output (graceful parse error)" {
    # Fixture: malformed YAML that causes parse failure
    write_wave_state "$(cat <<'YAML'
waves:
  W-15:
    gate_status: pending
  bad_entry:
 - broken yaml content
    [unclosed bracket: value
YAML
)"

    run_hook

    # BC-7.03.091 PC-2(b): YAML parse fails → silent exit 0
    [ "$status" -eq 0 ]

    # No output (graceful degradation)
    [ -z "$output" ] || \
        fail "AC-005(e): output must be empty on malformed YAML; got: $output"
}
