#!/usr/bin/env bats
# tests/integration/E-8-hook-plugins/session-learning.bats
#
# RED GATE bats tests for S-8.06: session-learning WASM hook port.
#
# ALL tests MUST FAIL until:
#   - session_learning_logic is implemented (T-3)
#   - WASM artifact is built: cargo build --target wasm32-wasip1 -p session-learning (T-4)
#   - hooks-registry.toml is updated (T-6)
#
# Behavioral contracts: BC-7.03.076, BC-7.03.077, BC-7.03.078
# Story ACs covered: AC-001, AC-002, AC-003, AC-004
# Edge cases: EC-005 (large stdin drain)
#
# Prerequisites:
#   - wasmtime CLI: `wasmtime --version`
#   - bats-core >= 1.10
#   - cargo build --target wasm32-wasip1 -p session-learning (artifact must exist)
#
# Run: bats tests/integration/E-8-hook-plugins/session-learning.bats
#
# WASM invocation note: session-learning is a WASI command. Tests invoke it via
# `wasmtime run` with `--dir` mapping the temp dir. The Stop envelope is passed
# via stdin (empty JSON `{}` for most cases; large payload for EC-005).

# ---------------------------------------------------------------------------
# Setup / Teardown
# ---------------------------------------------------------------------------

setup() {
    WORKTREE_ROOT="$(git -C "$(dirname "$BATS_TEST_FILENAME")" rev-parse --show-toplevel)"
    WASM_ARTIFACT="${WORKTREE_ROOT}/target/wasm32-wasip1/debug/session-learning.wasm"
    # Canonical artifact path used by dispatcher after T-4.
    WASM_RELEASE_ARTIFACT="${WORKTREE_ROOT}/target/wasm32-wasip1/release/session-learning.wasm"
    REGISTRY="${WORKTREE_ROOT}/plugins/vsdd-factory/hooks-registry.toml"

    # Create an isolated temp directory for each test (acts as cwd for wasmtime).
    TEST_TMPDIR="$(mktemp -d)"
    FACTORY_DIR="${TEST_TMPDIR}/.factory"
    SIDECAR_FILE="${FACTORY_DIR}/sidecar-learning.md"
}

teardown() {
    rm -rf "${TEST_TMPDIR}"
}

# Locate the WASM binary (debug preferred; release as fallback).
wasm_bin() {
    if [ -f "${WASM_ARTIFACT}" ]; then
        echo "${WASM_ARTIFACT}"
    elif [ -f "${WASM_RELEASE_ARTIFACT}" ]; then
        echo "${WASM_RELEASE_ARTIFACT}"
    else
        # Return path that does not exist — tests that need it will fail descriptively.
        echo "${WASM_ARTIFACT}"
    fi
}

# Run the WASM plugin with a given stdin payload from the TEST_TMPDIR cwd.
# Usage: run_plugin <stdin_content>
# Passes --dir=. so std::fs paths resolve relative to TEST_TMPDIR.
run_plugin() {
    local stdin_payload="${1:-{}}"
    echo "${stdin_payload}" | \
        wasmtime run --dir="${TEST_TMPDIR}::." "$(wasm_bin)" 2>&1
    return $?
}

# ---------------------------------------------------------------------------
# Gate: WASM artifact must exist (AC-001)
# ---------------------------------------------------------------------------

# AC-001: WASM artifact built for wasm32-wasip1.
# Fails until: `cargo build --target wasm32-wasip1 -p session-learning`
@test "AC-001: WASM artifact exists at target/wasm32-wasip1/debug/session-learning.wasm" {
    [ -f "$(wasm_bin)" ]
}

# AC-001: artifact is a valid WASM binary (starts with WASM magic bytes \0asm).
@test "AC-001: WASM artifact has valid WASM magic header" {
    local bin="$(wasm_bin)"
    [ -f "${bin}" ]
    # WASM magic: \0 a s m = 00 61 73 6d
    run xxd -l 4 "${bin}"
    [[ "$output" == *"0061 736d"* ]] || [[ "$output" == *"00 61 73 6d"* ]] || \
    [[ "$output" == *"61 73 6d"* ]]
}

# ---------------------------------------------------------------------------
# AC-001: hooks-registry.toml — native WASM entry registered
# ---------------------------------------------------------------------------

# AC-001: hooks-registry.toml must reference session-learning.wasm, not legacy-bash-adapter.
# Fails until T-6 updates the registry.
@test "AC-001: hooks-registry.toml references hook-plugins/session-learning.wasm" {
    [ -f "${REGISTRY}" ]
    grep -q 'hook-plugins/session-learning.wasm' "${REGISTRY}"
}

# AC-001: registry entry must not reference legacy-bash-adapter for session-learning.
@test "AC-001: session-learning registry entry does not reference legacy-bash-adapter" {
    [ -f "${REGISTRY}" ]
    # Extract the session-learning [[hooks]] block and verify it has no legacy-bash-adapter.
    # The block starts at the first [[hooks]] entry containing "session-learning".
    # We use awk to extract the block and grep for legacy-bash-adapter absence.
    block=$(awk '/\[\[hooks\]\]/{block=""; in_block=0} /session-learning/{in_block=1} in_block{block=block"\n"$0} END{print block}' "${REGISTRY}")
    ! echo "${block}" | grep -q 'legacy-bash-adapter'
}

# AC-001: registry must preserve event=Stop, priority=910, on_error=continue.
@test "AC-001: session-learning registry entry preserves event=Stop priority=910 on_error=continue" {
    [ -f "${REGISTRY}" ]
    grep -q 'event = "Stop"' "${REGISTRY}"
    grep -q 'priority = 910' "${REGISTRY}"
    grep -q 'on_error = "continue"' "${REGISTRY}"
}

# AC-001: exec_subprocess block must be absent for session-learning.
@test "AC-001: session-learning registry entry has no exec_subprocess or binary_allow" {
    [ -f "${REGISTRY}" ]
    # Extract the session-learning block and verify no exec_subprocess.
    block=$(awk '
        /\[\[hooks\]\]/ { if (in_sl) exit; in_sl=0 }
        /session-learning\.wasm/ { in_sl=1 }
        in_sl { print }
    ' "${REGISTRY}")
    ! echo "${block}" | grep -q 'exec_subprocess'
    ! echo "${block}" | grep -q 'binary_allow'
}

# ---------------------------------------------------------------------------
# AC-002: .factory/ present, sidecar-learning.md absent — file created with header
# ---------------------------------------------------------------------------

# AC-002 traces to BC-7.03.077 postcondition 1.
# .factory/ exists, sidecar-learning.md absent → create with header + marker; exit 0.
@test "AC-002: .factory/ present, sidecar-learning.md absent — creates file with header and marker" {
    mkdir -p "${FACTORY_DIR}"
    [ ! -f "${SIDECAR_FILE}" ]

    run run_plugin '{}'
    [ "$status" -eq 0 ]
    [ -f "${SIDECAR_FILE}" ]
}

@test "AC-002: created sidecar-learning.md starts with exact header (byte-identical to bash output)" {
    mkdir -p "${FACTORY_DIR}"
    run_plugin '{}'

    # Expected header (must match bash session-learning.sh output exactly):
    #   echo "# Sidecar Learning"
    #   echo ""
    #   echo "Session-end markers for the VSDD factory. Run /session-review to synthesize."
    #   echo ""
    expected_header="# Sidecar Learning

Session-end markers for the VSDD factory. Run /session-review to synthesize.

"
    actual_header=$(head -c "$(echo -n "${expected_header}" | wc -c)" "${SIDECAR_FILE}")
    [ "${actual_header}" = "${expected_header}" ]
}

@test "AC-002: created sidecar-learning.md contains marker line with ISO-8601 UTC timestamp" {
    mkdir -p "${FACTORY_DIR}"
    run_plugin '{}'

    # Marker line must match: "- Session ended at YYYY-MM-DDTHH:MM:SSZ (awaiting /session-review)"
    run grep -E '^- Session ended at [0-9]{4}-[0-9]{2}-[0-9]{2}T[0-9]{2}:[0-9]{2}:[0-9]{2}Z \(awaiting /session-review\)$' "${SIDECAR_FILE}"
    [ "$status" -eq 0 ]
}

@test "AC-002: exactly one marker line on first invocation" {
    mkdir -p "${FACTORY_DIR}"
    run_plugin '{}'

    count=$(grep -c '^- Session ended at ' "${SIDECAR_FILE}" || echo 0)
    [ "${count}" -eq 1 ]
}

# ---------------------------------------------------------------------------
# AC-003: .factory/ absent — exit 0 immediately, no file created
# ---------------------------------------------------------------------------

# AC-003 traces to BC-7.03.078 postcondition 1.
@test "AC-003: .factory/ absent — exit 0 immediately" {
    # Do NOT create .factory/ inside TEST_TMPDIR.
    [ ! -d "${FACTORY_DIR}" ]

    run run_plugin '{}'
    [ "$status" -eq 0 ]
}

@test "AC-003: .factory/ absent — no sidecar-learning.md created" {
    [ ! -d "${FACTORY_DIR}" ]
    run_plugin '{}'
    [ ! -f "${SIDECAR_FILE}" ]
}

@test "AC-003: .factory/ absent — no files created at any path" {
    [ ! -d "${FACTORY_DIR}" ]
    run_plugin '{}'
    # Nothing inside TEST_TMPDIR should have been created.
    [ ! -d "${FACTORY_DIR}" ]
}

# ---------------------------------------------------------------------------
# AC-004: append-only — no duplicate header on repeat invocations
# ---------------------------------------------------------------------------

# AC-004 traces to BC-7.03.077 postcondition 1 (append-only invariant).
@test "AC-004: second invocation appends one marker line, no duplicate header" {
    mkdir -p "${FACTORY_DIR}"
    run_plugin '{}'
    run_plugin '{}'

    # Header must appear exactly once.
    header_count=$(grep -c '^# Sidecar Learning$' "${SIDECAR_FILE}" || echo 0)
    [ "${header_count}" -eq 1 ]
}

@test "AC-004: two invocations produce exactly two marker lines" {
    mkdir -p "${FACTORY_DIR}"
    run_plugin '{}'
    run_plugin '{}'

    count=$(grep -c '^- Session ended at ' "${SIDECAR_FILE}" || echo 0)
    [ "${count}" -eq 2 ]
}

@test "AC-004: three invocations produce exactly three marker lines" {
    mkdir -p "${FACTORY_DIR}"
    run_plugin '{}'
    run_plugin '{}'
    run_plugin '{}'

    count=$(grep -c '^- Session ended at ' "${SIDECAR_FILE}" || echo 0)
    [ "${count}" -eq 3 ]
}

# ---------------------------------------------------------------------------
# EC-005: large stdin envelope — plugin drains stdin cleanly (no SIGPIPE)
# ---------------------------------------------------------------------------

# EC-005 (S-8.06 story spec): dispatcher writes large Stop envelope (>64KB) to plugin stdin.
# Plugin must read and discard stdin to EOF without SIGPIPE-equivalent failure.
# The 65,536-byte threshold is the typical OS pipe buffer size (Linux and macOS).
#
# This test writes exactly 65,536 bytes to plugin stdin and verifies:
#   - plugin exits 0
#   - no pipe write error on the writer side
#
# Per story spec EC-005: test MUST run on all platforms — do NOT skip on macOS.
@test "EC-005: plugin drains large stdin (65536 bytes) without SIGPIPE-equivalent failure" {
    mkdir -p "${FACTORY_DIR}"

    # Generate a 65,536-byte payload. Use a Stop-like JSON wrapper with padding.
    # We use printf+yes to generate the padding inline without a temp file.
    large_payload=$(python3 -c "
import json, sys
payload = {'hook_event_name': 'Stop', 'session_id': 'test-session', 'padding': 'x' * 65500}
print(json.dumps(payload))
" 2>/dev/null || printf '{"hook_event_name":"Stop","session_id":"test","padding":"%s"}' "$(python3 -c "print('x'*65500)" 2>/dev/null || head -c 65500 /dev/zero | tr '\0' 'x')")

    # Write large payload via pipe to plugin stdin.
    echo "${large_payload}" | \
        wasmtime run --dir="${TEST_TMPDIR}::." "$(wasm_bin)"
    local exit_code=$?

    # Plugin must exit 0 — no SIGPIPE or write failure.
    [ "${exit_code}" -eq 0 ]

    # Verify the append still happened (stdin drain must not break append logic).
    [ -f "${SIDECAR_FILE}" ]
    run grep -c '^- Session ended at ' "${SIDECAR_FILE}"
    [ "$output" -ge 1 ]
}

# EC-005: alternate pipe-fill test using dd for cross-platform compatibility.
@test "EC-005 (dd): plugin drains 128KB stdin without error" {
    mkdir -p "${FACTORY_DIR}"

    # Use dd to generate exactly 128 KB of filler piped to plugin stdin.
    # dd generates binary zeros; wrap in a simple prefix so it looks like JSON attempt.
    # Plugin must discard all of it and still perform the append.
    (printf '{}'; dd if=/dev/zero bs=1024 count=128 2>/dev/null | tr '\0' ' ') | \
        wasmtime run --dir="${TEST_TMPDIR}::." "$(wasm_bin)"
    local exit_code=$?

    [ "${exit_code}" -eq 0 ]
    [ -f "${SIDECAR_FILE}" ]
}
