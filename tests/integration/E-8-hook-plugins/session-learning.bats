#!/usr/bin/env bats
# tests/integration/E-8-hook-plugins/session-learning.bats
#
# Integration tests for S-8.06: session-learning WASM hook port.
#
# Invocation pattern: factory-dispatcher (canonical, per regression-v1.0.bats).
# The dispatcher routes the Stop event to session-learning.wasm via the
# hooks-registry.toml entry. Tests verify the end-to-end pipeline:
#   dispatcher -> session-learning.wasm -> .factory/sidecar-learning.md
#
# Behavioral contracts: BC-7.03.076, BC-7.03.077, BC-7.03.078
# Story ACs covered: AC-001, AC-002, AC-003, AC-004
# Edge cases: EC-005 (large stdin drain)
#
# Prerequisites:
#   - dispatcher built: cargo build --workspace --release
#   - WASM artifact in plugins/vsdd-factory/hook-plugins/session-learning.wasm
#
# Run: bats tests/integration/E-8-hook-plugins/session-learning.bats

# ---------------------------------------------------------------------------
# Setup / Teardown
# ---------------------------------------------------------------------------

setup() {
    # BATS_TEST_DIRNAME is tests/integration/E-8-hook-plugins — 3 levels up to repo root.
    REPO_ROOT="$(cd "${BATS_TEST_DIRNAME}/../../.." && pwd)"
    PLUGIN_ROOT="$REPO_ROOT/plugins/vsdd-factory"
    DISPATCHER="$REPO_ROOT/target/release/factory-dispatcher"
    REGISTRY="${PLUGIN_ROOT}/hooks-registry.toml"
    WASM_ARTIFACT="${PLUGIN_ROOT}/hook-plugins/session-learning.wasm"

    # Isolated project dir for each test.
    WORK="$BATS_TEST_TMPDIR/proj"
    mkdir -p "$WORK"
    FACTORY_DIR="${WORK}/.factory"
    SIDECAR_FILE="${FACTORY_DIR}/sidecar-learning.md"
}

# Run the hook pipeline via the dispatcher with a Stop envelope.
# Usage: run_plugin [<extra_stdin_prefix>]
# Sets CLAUDE_PROJECT_DIR=$WORK so session-learning writes to $WORK/.factory/
# Sets CLAUDE_PLUGIN_ROOT=$PLUGIN_ROOT so the registry is found.
run_plugin() {
    local stdin_payload='{"event_name":"Stop","session_id":"test-session"}'
    if [ ! -x "$DISPATCHER" ]; then
        skip "dispatcher not built — run 'cargo build --workspace --release' first"
    fi
    run env CLAUDE_PLUGIN_ROOT="$PLUGIN_ROOT" CLAUDE_PROJECT_DIR="$WORK" \
        bash -c "echo '${stdin_payload}' | '${DISPATCHER}'"
}

# ---------------------------------------------------------------------------
# Gate: WASM artifact must exist (AC-001)
# ---------------------------------------------------------------------------

# AC-001: WASM artifact placed in hook-plugins/.
@test "AC-001: WASM artifact exists at hook-plugins/session-learning.wasm" {
    [ -f "${WASM_ARTIFACT}" ]
}

# AC-001: artifact is a valid WASM binary (starts with WASM magic bytes \0asm).
@test "AC-001: WASM artifact has valid WASM magic header" {
    [ -f "${WASM_ARTIFACT}" ]
    # WASM magic: \0 a s m = 00 61 73 6d
    run xxd -l 4 "${WASM_ARTIFACT}"
    [[ "$output" == *"0061 736d"* ]] || [[ "$output" == *"00 61 73 6d"* ]] || \
    [[ "$output" == *"61 73 6d"* ]]
}

# ---------------------------------------------------------------------------
# AC-001: hooks-registry.toml — native WASM entry registered
# ---------------------------------------------------------------------------

# AC-001: hooks-registry.toml must reference session-learning.wasm, not legacy-bash-adapter.
@test "AC-001: hooks-registry.toml references hook-plugins/session-learning.wasm" {
    [ -f "${REGISTRY}" ]
    grep -q 'hook-plugins/session-learning.wasm' "${REGISTRY}"
}

# AC-001: registry entry must not reference legacy-bash-adapter for session-learning.
@test "AC-001: session-learning registry entry does not reference legacy-bash-adapter" {
    [ -f "${REGISTRY}" ]
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
    if [ ! -x "$DISPATCHER" ]; then skip "dispatcher not built"; fi
    mkdir -p "${FACTORY_DIR}"
    [ ! -f "${SIDECAR_FILE}" ]

    run_plugin
    [ "$status" -eq 0 ]
    [ -f "${SIDECAR_FILE}" ]
}

@test "AC-002: created sidecar-learning.md starts with exact header (byte-identical to bash output)" {
    if [ ! -x "$DISPATCHER" ]; then skip "dispatcher not built"; fi
    mkdir -p "${FACTORY_DIR}"
    run_plugin

    # Expected header (must match bash session-learning.sh output exactly):
    #   echo "# Sidecar Learning"
    #   echo ""
    #   echo "Session-end markers for the VSDD factory. Run /session-review to synthesize."
    #   echo ""
    # NOTE: use python3 for the entire comparison to avoid bash $() command substitution
    # stripping trailing newlines (shell strips trailing \n from $(...) output, so a
    # byte-identical comparison of a header ending with \n\n would always fail if done
    # via a bash variable assignment).
    run python3 -c "
import sys
expected = '# Sidecar Learning\n\nSession-end markers for the VSDD factory. Run /session-review to synthesize.\n\n'
with open(sys.argv[1], 'rb') as f:
    actual = f.read(len(expected.encode('utf-8'))).decode('utf-8')
if actual != expected:
    print('MISMATCH: expected=%r actual=%r' % (expected, actual), file=sys.stderr)
    sys.exit(1)
" "${SIDECAR_FILE}"
    [ "$status" -eq 0 ]
}

@test "AC-002: created sidecar-learning.md contains marker line with ISO-8601 UTC timestamp" {
    if [ ! -x "$DISPATCHER" ]; then skip "dispatcher not built"; fi
    mkdir -p "${FACTORY_DIR}"
    run_plugin

    # Marker line must match: "- Session ended at YYYY-MM-DDTHH:MM:SSZ (awaiting /session-review)"
    run grep -E '^- Session ended at [0-9]{4}-[0-9]{2}-[0-9]{2}T[0-9]{2}:[0-9]{2}:[0-9]{2}Z \(awaiting /session-review\)$' "${SIDECAR_FILE}"
    [ "$status" -eq 0 ]
}

@test "AC-002: exactly one marker line on first invocation" {
    if [ ! -x "$DISPATCHER" ]; then skip "dispatcher not built"; fi
    mkdir -p "${FACTORY_DIR}"
    run_plugin

    count=$(grep -c '^- Session ended at ' "${SIDECAR_FILE}" || echo 0)
    [ "${count}" -eq 1 ]
}

# ---------------------------------------------------------------------------
# AC-003: .factory/ absent — exit 0 immediately, no file created
# ---------------------------------------------------------------------------

# AC-003 traces to BC-7.03.078 postcondition 1.
#
# Note: when using the dispatcher, it creates .factory/logs/ for its own
# internal log before invoking plugins. This means the plugin always sees
# .factory/ when run through the dispatcher. AC-003 is therefore tested via
# direct wasmtime invocation to exercise the plugin logic in isolation.
@test "AC-003: .factory/ absent — plugin exits 0 (direct WASM invocation)" {
    [ -f "${WASM_ARTIFACT}" ] || skip "WASM artifact not built"
    command -v wasmtime >/dev/null 2>&1 || skip "wasmtime CLI not installed"
    # Do NOT create .factory/ inside WORK.
    [ ! -d "${FACTORY_DIR}" ]

    run bash -c "echo '{}' | wasmtime run --dir='${WORK}::.' '${WASM_ARTIFACT}'"
    [ "$status" -eq 0 ]
}

@test "AC-003: .factory/ absent — no sidecar-learning.md created (direct WASM invocation)" {
    [ -f "${WASM_ARTIFACT}" ] || skip "WASM artifact not built"
    command -v wasmtime >/dev/null 2>&1 || skip "wasmtime CLI not installed"
    [ ! -d "${FACTORY_DIR}" ]
    bash -c "echo '{}' | wasmtime run --dir='${WORK}::.' '${WASM_ARTIFACT}'"
    [ ! -f "${SIDECAR_FILE}" ]
}

@test "AC-003: .factory/ absent — no .factory directory created (direct WASM invocation)" {
    [ -f "${WASM_ARTIFACT}" ] || skip "WASM artifact not built"
    command -v wasmtime >/dev/null 2>&1 || skip "wasmtime CLI not installed"
    [ ! -d "${FACTORY_DIR}" ]
    bash -c "echo '{}' | wasmtime run --dir='${WORK}::.' '${WASM_ARTIFACT}'"
    [ ! -d "${FACTORY_DIR}" ]
}

# ---------------------------------------------------------------------------
# AC-004: append-only — no duplicate header on repeat invocations
# ---------------------------------------------------------------------------

# AC-004 traces to BC-7.03.077 postcondition 1 (append-only invariant).
@test "AC-004: second invocation appends one marker line, no duplicate header" {
    if [ ! -x "$DISPATCHER" ]; then skip "dispatcher not built"; fi
    mkdir -p "${FACTORY_DIR}"
    run_plugin
    run_plugin

    # Header must appear exactly once.
    header_count=$(grep -c '^# Sidecar Learning$' "${SIDECAR_FILE}" || echo 0)
    [ "${header_count}" -eq 1 ]
}

@test "AC-004: two invocations produce exactly two marker lines" {
    if [ ! -x "$DISPATCHER" ]; then skip "dispatcher not built"; fi
    mkdir -p "${FACTORY_DIR}"
    run_plugin
    run_plugin

    count=$(grep -c '^- Session ended at ' "${SIDECAR_FILE}" || echo 0)
    [ "${count}" -eq 2 ]
}

@test "AC-004: three invocations produce exactly three marker lines" {
    if [ ! -x "$DISPATCHER" ]; then skip "dispatcher not built"; fi
    mkdir -p "${FACTORY_DIR}"
    run_plugin
    run_plugin
    run_plugin

    count=$(grep -c '^- Session ended at ' "${SIDECAR_FILE}" || echo 0)
    [ "${count}" -eq 3 ]
}

# ---------------------------------------------------------------------------
# EC-005: large stdin envelope — plugin drains stdin cleanly (no SIGPIPE)
# ---------------------------------------------------------------------------

# EC-005 (S-8.06 story spec): dispatcher writes large Stop envelope (>64KB) to plugin stdin.
# Plugin must read and discard stdin to EOF without SIGPIPE-equivalent failure.
# The dispatcher pipes the full envelope into the plugin; this test exercises that path
# by sending a large envelope through the dispatcher pipeline.
@test "EC-005: plugin drains large Stop envelope (65536+ bytes) without SIGPIPE-equivalent failure" {
    if [ ! -x "$DISPATCHER" ]; then skip "dispatcher not built"; fi
    mkdir -p "${FACTORY_DIR}"

    # Generate a large Stop envelope with padding.
    large_payload=$(python3 -c "
import json
payload = {'event_name': 'Stop', 'session_id': 'ec005-test', 'padding': 'x' * 65500}
print(json.dumps(payload))
" 2>/dev/null || printf '{"event_name":"Stop","session_id":"ec005","stop_hook_active":true}')

    run env CLAUDE_PLUGIN_ROOT="$PLUGIN_ROOT" CLAUDE_PROJECT_DIR="$WORK" \
        bash -c "echo '${large_payload}' | '${DISPATCHER}'"

    # Dispatcher must exit 0 — no SIGPIPE or write failure.
    [ "$status" -eq 0 ]

    # Verify the append still happened.
    [ -f "${SIDECAR_FILE}" ]
    run grep -c '^- Session ended at ' "${SIDECAR_FILE}"
    [ "$output" -ge 1 ]
}

# EC-005: direct wasmtime test — plugin receives 128KB via stdin and exits 0.
# This exercises the WASM stdin drain path directly (no dispatcher involved).
@test "EC-005 (direct): plugin drains 128KB stdin without error when invoked directly" {
    local wasm_bin="${WASM_ARTIFACT}"
    [ -f "${wasm_bin}" ] || skip "WASM artifact not built"
    command -v wasmtime >/dev/null 2>&1 || skip "wasmtime CLI not installed"
    mkdir -p "${FACTORY_DIR}"

    # Use dd to generate exactly 128 KB of filler piped to plugin stdin.
    (printf '{}'; dd if=/dev/zero bs=1024 count=128 2>/dev/null | tr '\0' ' ') | \
        wasmtime run --dir="${WORK}::." "${wasm_bin}"
    local exit_code=$?

    [ "${exit_code}" -eq 0 ]
    [ -f "${SIDECAR_FILE}" ]
}
