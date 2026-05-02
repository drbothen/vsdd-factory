#!/usr/bin/env bats
# tests/integration/hooks/session-learning.bats
#
# Parity bats tests for S-8.06 AC-004 (story-canonical location).
# These are the 4 test cases specified in T-5 of the story spec:
#   Case 1 (AC-002): first invocation creates file + header + one marker
#   Case 2 (AC-004): second invocation appends one more marker, no duplicate header
#   Case 3 (AC-003): invocation without .factory/ exits 0, no file created
#   Case 4 (EC-005): large stdin envelope — plugin completes cleanly
#
# RED GATE: ALL tests MUST FAIL until WASM artifact is built (T-4) and
# session_learning_logic is implemented (T-3).
#
# This file is the AC-004-canonical location. The extended test suite is at:
#   tests/integration/E-8-hook-plugins/session-learning.bats
#
# Run: bats tests/integration/hooks/session-learning.bats

setup() {
    WORKTREE_ROOT="$(git -C "$(dirname "$BATS_TEST_FILENAME")" rev-parse --show-toplevel)"
    WASM_ARTIFACT="${WORKTREE_ROOT}/target/wasm32-wasip1/debug/session-learning.wasm"
    WASM_RELEASE_ARTIFACT="${WORKTREE_ROOT}/target/wasm32-wasip1/release/session-learning.wasm"

    TEST_TMPDIR="$(mktemp -d)"
    FACTORY_DIR="${TEST_TMPDIR}/.factory"
    SIDECAR_FILE="${FACTORY_DIR}/sidecar-learning.md"
}

teardown() {
    rm -rf "${TEST_TMPDIR}"
}

wasm_bin() {
    if [ -f "${WASM_ARTIFACT}" ]; then
        echo "${WASM_ARTIFACT}"
    elif [ -f "${WASM_RELEASE_ARTIFACT}" ]; then
        echo "${WASM_RELEASE_ARTIFACT}"
    else
        echo "${WASM_ARTIFACT}"
    fi
}

run_plugin() {
    local stdin_payload="${1:-{}}"
    echo "${stdin_payload}" | \
        wasmtime run --dir="${TEST_TMPDIR}::." "$(wasm_bin)" 2>&1
    return $?
}

# ---------------------------------------------------------------------------
# T-5 Case 1 (AC-002): .factory/ present, sidecar-learning.md absent
# ---------------------------------------------------------------------------

@test "T5-case1 (AC-002): first invocation creates file with header and one marker; exit 0" {
    mkdir -p "${FACTORY_DIR}"
    [ ! -f "${SIDECAR_FILE}" ]

    run run_plugin '{}'
    [ "$status" -eq 0 ]
    [ -f "${SIDECAR_FILE}" ]

    # Must start with correct header (trailing blank line required).
    run grep -c '^# Sidecar Learning$' "${SIDECAR_FILE}"
    [ "$output" -eq 1 ]

    # Must have exactly one marker line.
    run grep -c '^- Session ended at ' "${SIDECAR_FILE}"
    [ "$output" -eq 1 ]

    # Marker line must match ISO-8601 UTC format.
    run grep -E '^- Session ended at [0-9]{4}-[0-9]{2}-[0-9]{2}T[0-9]{2}:[0-9]{2}:[0-9]{2}Z \(awaiting /session-review\)$' "${SIDECAR_FILE}"
    [ "$status" -eq 0 ]
}

# ---------------------------------------------------------------------------
# T-5 Case 2 (AC-004): .factory/ present, sidecar-learning.md pre-exists
# ---------------------------------------------------------------------------

@test "T5-case2 (AC-004): second invocation appends one more marker, no duplicate header; exit 0" {
    mkdir -p "${FACTORY_DIR}"
    run_plugin '{}'  # first invocation — creates file
    run_plugin '{}'  # second invocation — must append

    # Header must appear exactly once.
    header_count=$(grep -c '^# Sidecar Learning$' "${SIDECAR_FILE}" || echo 0)
    [ "${header_count}" -eq 1 ]

    # Must have exactly two marker lines.
    marker_count=$(grep -c '^- Session ended at ' "${SIDECAR_FILE}" || echo 0)
    [ "${marker_count}" -eq 2 ]
}

# ---------------------------------------------------------------------------
# T-5 Case 3 (AC-003): .factory/ absent — exit 0, no file created
# ---------------------------------------------------------------------------

@test "T5-case3 (AC-003): invocation without .factory/ exits 0, no file created" {
    [ ! -d "${FACTORY_DIR}" ]

    run run_plugin '{}'
    [ "$status" -eq 0 ]
    [ ! -f "${SIDECAR_FILE}" ]
    [ ! -d "${FACTORY_DIR}" ]
}

# ---------------------------------------------------------------------------
# T-5 Case 4 (EC-005): large stdin envelope — plugin completes cleanly
# ---------------------------------------------------------------------------

@test "T5-case4 (EC-005): large Stop envelope (>64KB) drained without write error; exit 0" {
    mkdir -p "${FACTORY_DIR}"

    # Generate a Stop envelope padded to exceed the 64KB pipe buffer threshold.
    # python3 is available in CI; fall back to dd+prefix if absent.
    if command -v python3 &>/dev/null; then
        large_payload=$(python3 -c "
import json
payload = {'hook_event_name': 'Stop', 'session_id': 'test-ec005', 'padding': 'x' * 65500}
print(json.dumps(payload))
")
    else
        large_payload=$(printf '{"hook_event_name":"Stop","padding":"%s"}' \
            "$(dd if=/dev/zero bs=1 count=65500 2>/dev/null | tr '\0' 'x')")
    fi

    echo "${large_payload}" | \
        wasmtime run --dir="${TEST_TMPDIR}::." "$(wasm_bin)"
    local exit_code=$?

    # Must exit 0 (no SIGPIPE-equivalent failure).
    [ "${exit_code}" -eq 0 ]

    # Append must have completed despite the large stdin.
    [ -f "${SIDECAR_FILE}" ]
    marker_count=$(grep -c '^- Session ended at ' "${SIDECAR_FILE}" || echo 0)
    [ "${marker_count}" -ge 1 ]
}
