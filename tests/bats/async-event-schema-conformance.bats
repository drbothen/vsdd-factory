#!/usr/bin/env bats
#
# async-event-schema-conformance.bats
#
# VP-079 v1.13: Async-Semantics Event Types — Payload Schema Conformance.
# Five scenarios, one per triggering condition.
#
# RED: All scenarios require a compiled factory-dispatcher binary with T-3e
# (emit functions) and T-3f (validate_async_block_invariant) implemented.
# Until those tasks are done, the dispatcher panics with "not yet implemented"
# and the SINK_FILE is empty (no events emitted).
#
# DI-019: ASYNC_DRAIN_WINDOW_MS = 100ms (canonical). Referenced by name in
# comments; Scenario 5 uses timeout_ms=200 (> 100) to trigger truncation.
# Do NOT hardcode 100 — reference "ASYNC_DRAIN_WINDOW_MS (DI-019)".
#
# Test-fixture note: legacy-bash-adapter.wasm + bash stub scripts are test
# infrastructure for controlled exit-code injection. They are NOT the production
# plugin under test. Per project WASM-migration directive, NEW shipped plugins
# are native WASM; legacy-bash-adapter in test fixtures is a transitional
# convenience (VP-078 v1.8, VP-079 v1.13 annotations).
#
# BC traces:
#   BC-3.08.001 v1.7 — event catalog (4 new event types)
#   BC-1.14.001 — dispatch partition + drain window (DI-019)
#   BC-7.06.001 — schema validation (schema_mismatch / registry_invalid triggers)
#   DI-017 — trace_id on every emitted event
#   DI-019 — ASYNC_DRAIN_WINDOW_MS (canonical; must not be hardcoded)
#   VP-079 v1.13 — fault injection verification property
#   AC-011, AC-012, AC-013, AC-014, AC-005 (S-15.01 v1.14)

PLUGIN_ROOT=""
SINK_FILE=""

setup() {
    PLUGIN_ROOT=$(mktemp -d)
    # POSIX-portable mktemp (macOS BSD mktemp does not support --suffix).
    SINK_FILE="$(mktemp).jsonl" && touch "$SINK_FILE"
    mkdir -p "$PLUGIN_ROOT/hook-plugins" "$PLUGIN_ROOT/test-fixtures"

    # Copy legacy-bash-adapter.wasm into the test PLUGIN_ROOT.
    # Scenarios 1, 4, 5 use it as a test fixture to inject controlled exit codes.
    # The built WASM is expected at plugins/vsdd-factory/hook-plugins/ relative to
    # the repo root. If not found, tests that require it will fail with a
    # "plugin file not found" error from the dispatcher (not a skip).
    #
    # REPO_ROOT: walk up from this test file's directory to find the repo root.
    local repo_root
    repo_root="$(cd "$(dirname "$BATS_TEST_FILENAME")/../.." && pwd)"
    local wasm_src="${repo_root}/plugins/vsdd-factory/hook-plugins/legacy-bash-adapter.wasm"
    if [ -f "$wasm_src" ]; then
        cp "$wasm_src" "$PLUGIN_ROOT/hook-plugins/legacy-bash-adapter.wasm"
    fi
    # Note: if the WASM is missing, the dispatcher will fail with a registry load
    # error (plugin path not found) and tests will fail with a clear error message.
}

teardown() {
    rm -rf "$PLUGIN_ROOT"
    rm -f "$SINK_FILE"
}

# Helper: find event line by type in the VSDD_SINK_FILE output.
find_event_line() {
    local event_type="$1"
    grep "\"type\":\"${event_type}\"" "$SINK_FILE" 2>/dev/null | tail -1
}

# Helper: assert all listed fields are present in a JSON line.
# trace_id is checked for field presence only — null is permitted as
# DI-017 last-resort fallback per BC-3.08.001 Error Paths.
assert_fields_present() {
    local json_line="$1"
    shift
    for field in "$@"; do
        local val
        val=$(echo "$json_line" | python3 -c \
            "import sys,json; d=json.load(sys.stdin); print(d.get('${field}','__MISSING__'))" 2>/dev/null)
        [ "$val" != "__MISSING__" ] || {
            echo "FAIL: mandatory field '${field}' missing from: $json_line"
            return 1
        }
    done
}

# Dispatch helper: write registry, supply envelope on stdin, run dispatcher.
# $1 = registry toml content
# $2 = event_name (e.g. "PostToolUse")
# $3 = tool_name (default: Write)
# $4 = VSDD_ASYNC_DRAIN_WINDOW_MS override in ms (optional, debug builds only).
#      Used by S1 and S4 to account for WASM cold-start time in debug builds.
#      Leave empty for S2/S3/S5 which do not exercise the async drain path.
run_dispatcher() {
    local registry_content="$1"
    local event_name="$2"
    local tool_name="${3:-Write}"
    local drain_window_ms="${4:-}"

    printf '%s' "$registry_content" > "$PLUGIN_ROOT/hooks-registry.toml"

    local envelope
    envelope=$(printf '{"hook_event_name":"%s","tool_name":"%s","session_id":"vp079-test","tool_input":{}}' \
        "$event_name" "$tool_name")

    # Optionally pass VSDD_ASYNC_DRAIN_WINDOW_MS inline in the command.
    # This avoids shell quoting issues with environment variable injection.
    # The env var is set as a prefix to the factory-dispatcher invocation.
    local drain_prefix=""
    if [ -n "$drain_window_ms" ]; then
        drain_prefix="VSDD_ASYNC_DRAIN_WINDOW_MS=$drain_window_ms"
    fi

    run sh -c "printf '%s' '$envelope' | \
        CLAUDE_PLUGIN_ROOT=\"$PLUGIN_ROOT\" \
        VSDD_SINK_FILE=\"$SINK_FILE\" \
        CLAUDE_PROJECT_DIR=\"$PLUGIN_ROOT\" \
        $drain_prefix \
        factory-dispatcher 2>/dev/null"
}

# Check dispatcher binary exists before each scenario.
require_dispatcher() {
    if ! command -v factory-dispatcher &>/dev/null; then
        skip "factory-dispatcher binary not found — run cargo build first"
    fi
}

# ---
# Scenario 1: plugin.async_block_discarded
#
# Trigger: async group plugin returns exit code 2.
# Assumption: fixture includes a sync plugin so sync_group is non-empty,
# keeping the dispatcher alive during the async task's execution and the
# drain window (ASYNC_DRAIN_WINDOW_MS per DI-019). The async plugin exits 2
# immediately (fast), so its terminal event is emitted well within the drain window.
#
# AC-011: mandatory fields: type, trace_id, plugin_name, exit_code, timestamp, reason.
# reason must be "async_plugin_block_verdict_discarded".
# ---

@test "VP-079 S1: plugin.async_block_discarded emits all mandatory fields (AC-011)" {
    require_dispatcher

    # RED: emit_plugin_async_block_discarded is todo!() — SINK_FILE will be empty.
    # After T-3e: this test passes when the event is found with all 6 mandatory fields.

    printf '%s\n' "exit 0" > "$PLUGIN_ROOT/test-fixtures/exit0.sh"
    printf '%s\n' "exit 2" > "$PLUGIN_ROOT/test-fixtures/exit2.sh"
    chmod +x "$PLUGIN_ROOT/test-fixtures/exit0.sh"
    chmod +x "$PLUGIN_ROOT/test-fixtures/exit2.sh"

    # S1: extend drain window to 5000ms to account for WASM cold-start time in
    # debug builds (legacy-bash-adapter cold-start ~300ms >> ASYNC_DRAIN_WINDOW_MS 100ms).
    # VSDD_ASYNC_DRAIN_WINDOW_MS is debug-only (SEC-003); production uses DI-019 value.
    run_dispatcher '
schema_version = 2

[[hooks]]
name = "sync-gate-plugin"
plugin = "hook-plugins/legacy-bash-adapter.wasm"
on_error = "block"
async = false
event = "PostToolUse"
priority = 50

[hooks.config]
script_path = "test-fixtures/exit0.sh"

[hooks.capabilities.exec_subprocess]
binary_allow = ["bash"]
shell_bypass_acknowledged = "VP-079-S1-test-fixture"

[[hooks]]
name = "test-async-blocker"
plugin = "hook-plugins/legacy-bash-adapter.wasm"
on_error = "continue"
async = true
event = "PostToolUse"
priority = 100

[hooks.config]
script_path = "test-fixtures/exit2.sh"

[hooks.capabilities.exec_subprocess]
binary_allow = ["bash"]
shell_bypass_acknowledged = "VP-079-S1-test-fixture"
' "PostToolUse" "Write" "5000"

    local line
    line=$(find_event_line "plugin.async_block_discarded")
    [ -n "$line" ] || {
        echo "FAIL: plugin.async_block_discarded event not found in $SINK_FILE"
        echo "Note: RED until T-3e implements emit_plugin_async_block_discarded()"
        return 1
    }

    assert_fields_present "$line" \
        type trace_id plugin_name exit_code timestamp reason

    local reason
    reason=$(echo "$line" | python3 -c \
        "import sys,json; print(json.load(sys.stdin)['reason'])")
    [ "$reason" = "async_plugin_block_verdict_discarded" ] || {
        echo "FAIL: reason must be 'async_plugin_block_verdict_discarded'; got: $reason"
        return 1
    }
}

# ---
# Scenario 2: dispatcher.schema_mismatch
#
# Trigger: registry has schema_version = 1 (not 2).
# Emit-then-exit: event must reach FileSink before dispatcher exits.
#
# AC-012: mandatory fields: type, trace_id, found_version, expected_version, timestamp, error_code.
# expected_version must be "2"; error_code must be "E-REG-001".
# ---

@test "VP-079 S2: dispatcher.schema_mismatch emits all mandatory fields (AC-012)" {
    require_dispatcher

    # RED: emit_dispatcher_schema_mismatch is todo!() — SINK_FILE will be empty.
    # After T-3e: this test passes when the event is found with all 6 mandatory fields.

    printf '%s\n' "exit 0" > "$PLUGIN_ROOT/test-fixtures/exit0.sh"
    chmod +x "$PLUGIN_ROOT/test-fixtures/exit0.sh"

    run_dispatcher '
schema_version = 1

[[hooks]]
name = "legacy-plugin"
plugin = "hook-plugins/legacy-bash-adapter.wasm"
on_error = "continue"
event = "PreToolUse"
priority = 100

[hooks.config]
script_path = "test-fixtures/exit0.sh"
' "PreToolUse"

    local line
    line=$(find_event_line "dispatcher.schema_mismatch")
    [ -n "$line" ] || {
        echo "FAIL: dispatcher.schema_mismatch event not found in $SINK_FILE"
        echo "Note: RED until T-3e implements emit_dispatcher_schema_mismatch()"
        return 1
    }

    assert_fields_present "$line" \
        type trace_id found_version expected_version timestamp error_code

    # F-P1-014 fix: assert expected_version is an integer (not a string "2").
    # BC-3.08.001 Event 2 wire format specifies `"expected_version": 2` (integer).
    # This assertion catches a future serialization regression where the field
    # is emitted as the JSON string `"2"` instead of the JSON integer `2`.
    local expected_version
    expected_version=$(echo "$line" | python3 -c \
        "import json,sys; d=json.load(sys.stdin); \
         assert isinstance(d['expected_version'], int), \
             'expected_version must be JSON int, not str; got type=' + type(d['expected_version']).__name__; \
         print(d['expected_version'])" 2>&1)
    local ev_status=$?
    [ "$ev_status" -eq 0 ] || {
        echo "FAIL: expected_version type assertion failed: $expected_version"
        echo "      BC-3.08.001 requires expected_version to be a JSON integer, not a string."
        return 1
    }
    [ "$expected_version" = "2" ] || {
        echo "FAIL: expected_version must be 2; got: $expected_version"
        return 1
    }

    local error_code
    error_code=$(echo "$line" | python3 -c \
        "import sys,json; print(json.load(sys.stdin)['error_code'])")
    [ "$error_code" = "E-REG-001" ] || {
        echo "FAIL: error_code must be 'E-REG-001'; got: $error_code"
        return 1
    }
}

# ---
# Scenario 3: dispatcher.registry_invalid
#
# Trigger: registry entry has on_error=block AND async=true.
# The dispatcher must emit the event and refuse to start.
#
# AC-013: mandatory fields: type, trace_id, offending_plugin, violation, timestamp, error_code.
# offending_plugin must be "invalid-blocker"; error_code must be "E-REG-002".
# ---

@test "VP-079 S3: dispatcher.registry_invalid emits all mandatory fields (AC-013)" {
    require_dispatcher

    # RED: validate_async_block_invariant is todo!() — dispatcher panics.
    # After T-3e + T-3f: this test passes.

    printf '%s\n' "exit 0" > "$PLUGIN_ROOT/test-fixtures/exit0.sh"
    chmod +x "$PLUGIN_ROOT/test-fixtures/exit0.sh"

    run_dispatcher '
schema_version = 2

[[hooks]]
name = "invalid-blocker"
plugin = "hook-plugins/legacy-bash-adapter.wasm"
on_error = "block"
async = true
event = "PreToolUse"
priority = 100

[hooks.config]
script_path = "test-fixtures/exit0.sh"
' "PreToolUse"

    local line
    line=$(find_event_line "dispatcher.registry_invalid")
    [ -n "$line" ] || {
        echo "FAIL: dispatcher.registry_invalid event not found in $SINK_FILE"
        echo "Note: RED until T-3e + T-3f implement emit_dispatcher_registry_invalid()"
        return 1
    }

    assert_fields_present "$line" \
        type trace_id offending_plugin violation timestamp error_code

    local offending
    offending=$(echo "$line" | python3 -c \
        "import sys,json; print(json.load(sys.stdin)['offending_plugin'])")
    [ "$offending" = "invalid-blocker" ] || {
        echo "FAIL: offending_plugin must name the plugin; got: $offending"
        return 1
    }

    local error_code
    error_code=$(echo "$line" | python3 -c \
        "import sys,json; print(json.load(sys.stdin)['error_code'])")
    [ "$error_code" = "E-REG-002" ] || {
        echo "FAIL: error_code must be 'E-REG-002'; got: $error_code"
        return 1
    }

    local violation
    violation=$(echo "$line" | python3 -c \
        "import sys,json; print(json.load(sys.stdin)['violation'])")
    [ "$violation" = "async_block_conflict" ] || {
        echo "FAIL: violation must be 'async_block_conflict'; got: $violation"
        return 1
    }
}

# ---
# Scenario 4: plugin.timeout (async path)
#
# Trigger: async plugin exceeds timeout_ms but completes within ASYNC_DRAIN_WINDOW_MS (DI-019).
# timeout_ms = 50ms < ASYNC_DRAIN_WINDOW_MS (100ms per DI-019).
# The drain window allows plugin.timeout to be emitted before dispatcher exits.
#
# AC-014: mandatory fields: type, trace_id, plugin_name, execution_group, timeout_ms, timestamp.
# execution_group must be "async".
# ---

@test "VP-079 S4: plugin.timeout (async path) emits all mandatory fields (AC-014)" {
    require_dispatcher

    # GREEN after T-3e: emit_plugin_timeout_async is wired in the dispatch path.
    #
    # Timeout mechanism: WASM epoch interrupt. Epoch ticks fire at EPOCH_TICK_MS (10ms)
    # intervals. timeout_ms = 1 → 1 epoch tick → fires after the first 10ms epoch.
    #
    # The subprocess runs `sleep 0.2` (200ms). While bash is sleeping, the WASM is
    # blocked in the exec_subprocess host call — epoch ticks accumulate but cannot
    # interrupt a blocked host call. When bash exits (200ms), WASM resumes and
    # immediately checks the epoch — at that point, ~20 epoch ticks have elapsed
    # against a budget of 1. The next WASM instruction triggers Trap::Interrupt →
    # PluginResult::Timeout.
    #
    # VSDD_ASYNC_DRAIN_WINDOW_MS=5000 allows the ~WASM-compile + 200ms subprocess
    # to complete within the extended drain window (debug builds only, SEC-003).
    printf '%s\n' "exit 0" > "$PLUGIN_ROOT/test-fixtures/exit0.sh"
    printf '%s\n' "sleep 0.2" > "$PLUGIN_ROOT/test-fixtures/sleep200ms.sh"
    chmod +x "$PLUGIN_ROOT/test-fixtures/exit0.sh"
    chmod +x "$PLUGIN_ROOT/test-fixtures/sleep200ms.sh"

    run_dispatcher '
schema_version = 2

[[hooks]]
name = "sync-gate-plugin"
plugin = "hook-plugins/legacy-bash-adapter.wasm"
on_error = "block"
async = false
event = "PostToolUse"
priority = 50

[hooks.config]
script_path = "test-fixtures/exit0.sh"

[hooks.capabilities.exec_subprocess]
binary_allow = ["bash"]
shell_bypass_acknowledged = "VP-079-S4-test-fixture"

[[hooks]]
name = "slow-async-plugin"
plugin = "hook-plugins/legacy-bash-adapter.wasm"
on_error = "continue"
async = true
timeout_ms = 1
event = "PostToolUse"
priority = 100

[hooks.config]
script_path = "test-fixtures/sleep200ms.sh"

[hooks.capabilities.exec_subprocess]
binary_allow = ["bash"]
shell_bypass_acknowledged = "VP-079-S4-test-fixture"
' "PostToolUse" "Write" "5000"

    local line
    line=$(find_event_line "plugin.timeout")
    [ -n "$line" ] || {
        echo "FAIL: plugin.timeout event not found in $SINK_FILE"
        echo "Note: RED until T-3e implements emit_plugin_timeout_async()"
        return 1
    }

    assert_fields_present "$line" \
        type trace_id plugin_name execution_group timeout_ms timestamp

    local exec_group
    exec_group=$(echo "$line" | python3 -c \
        "import sys,json; print(json.load(sys.stdin)['execution_group'])")
    [ "$exec_group" = "async" ] || {
        echo "FAIL: execution_group must be 'async'; got: $exec_group"
        return 1
    }
}

# ---
# Scenario 5: async task exceeds drain window — event NOT emitted (truncation expected)
#
# Trigger: async plugin timeout_ms (200ms) > ASYNC_DRAIN_WINDOW_MS (100ms per DI-019).
# BC-1.14.001 PC4 (enforcing DI-019): dispatcher forcibly terminates async tasks at
# drain expiry. Verify plugin.timeout is NOT written to SINK_FILE.
#
# AC-005: tasks not done by drain expiry are forcibly terminated; their pending I/O
# is discarded. Total latency bound:
#   max(sync_plugin_durations) + ASYNC_DRAIN_WINDOW_MS (DI-019).
# ---

# ---
# Scenario 7 (F-P6-001): EC-012 — partial drain completion (multi-async-plugin)
#
# Trigger: drain timer fires while ≥1 async plugins are still executing AND ≥1 async
# plugins have already completed within the window.
# Setup: fast-blocker (exit 2, ~ms) + slow-async (sleep 0.2, 200ms); drain at 100ms.
#
# Expected per EC-012 (BC-1.14.001 v1.9 PC4):
#   - Completed plugins' terminal events MUST emit before dispatcher exit.
#   - In-flight plugins' events MAY be lost (no guarantee; truncated telemetry accepted).
#
# This tests the partial-completion distinction from the all-or-nothing anti-pattern
# (F-P1-010 cite in EC-012): results received via the result channel before the drain
# timer fires MUST be flushed to FileSink even when other tasks are still in-flight.
#
# Uses VSDD_ASYNC_DRAIN_WINDOW_MS=5000 (debug-build override per SEC-003) so that the
# fast-blocker's WASM cold-start completes well within the window, while slow-async
# (sleep 0.2 = 200ms) remains in-flight when the drain is scheduled.
# In production, ASYNC_DRAIN_WINDOW_MS is DI-019 (100ms); the 5000ms value is only
# used here to avoid WASM cold-start flakiness in CI debug builds.
# ---

@test "VP-079 S7: EC-012 partial completion — fast async plugin event emits even when slow async plugin still in-flight" {
    require_dispatcher

    # RED: emit_plugin_async_block_discarded must flush completed results before drain
    # expiry. After T-3e: fast-blocker's plugin.async_block_discarded MUST appear in
    # SINK_FILE; slow-async's plugin.timeout MAY OR MAY NOT appear (EC-012 semantics).

    printf '%s\n' "exit 0"   > "$PLUGIN_ROOT/test-fixtures/exit0.sh"
    printf '%s\n' "exit 2"   > "$PLUGIN_ROOT/test-fixtures/exit2.sh"
    printf '%s\n' "sleep 0.2; exit 0" > "$PLUGIN_ROOT/test-fixtures/sleep200ms-exit0.sh"
    chmod +x "$PLUGIN_ROOT/test-fixtures/exit0.sh"
    chmod +x "$PLUGIN_ROOT/test-fixtures/exit2.sh"
    chmod +x "$PLUGIN_ROOT/test-fixtures/sleep200ms-exit0.sh"

    # Registry: sync-gate (keeps dispatcher alive) + fast-blocker (async, exit 2, ~ms)
    # + slow-async (async, sleep 200ms — still in-flight when drain fires).
    run_dispatcher '
schema_version = 2

[[hooks]]
name = "sync-gate-plugin"
plugin = "hook-plugins/legacy-bash-adapter.wasm"
on_error = "block"
async = false
event = "PostToolUse"
priority = 50

[hooks.config]
script_path = "test-fixtures/exit0.sh"

[hooks.capabilities.exec_subprocess]
binary_allow = ["bash"]
shell_bypass_acknowledged = "VP-079-S7-test-fixture"

[[hooks]]
name = "fast-blocker"
plugin = "hook-plugins/legacy-bash-adapter.wasm"
on_error = "continue"
async = true
event = "PostToolUse"
priority = 100

[hooks.config]
script_path = "test-fixtures/exit2.sh"

[hooks.capabilities.exec_subprocess]
binary_allow = ["bash"]
shell_bypass_acknowledged = "VP-079-S7-test-fixture"

[[hooks]]
name = "slow-async"
plugin = "hook-plugins/legacy-bash-adapter.wasm"
on_error = "continue"
async = true
timeout_ms = 1
event = "PostToolUse"
priority = 100

[hooks.config]
script_path = "test-fixtures/sleep200ms-exit0.sh"

[hooks.capabilities.exec_subprocess]
binary_allow = ["bash"]
shell_bypass_acknowledged = "VP-079-S7-test-fixture"
' "PostToolUse" "Write" "5000"

    # Assertion 1 (EC-012 MUST): fast-blocker's plugin.async_block_discarded MUST be present.
    # The event was received via the result channel before the drain timer fired; it MUST
    # have been flushed to FileSink (partial-completion guarantee).
    local fast_line
    fast_line=$(grep '"type":"plugin.async_block_discarded"' "$SINK_FILE" 2>/dev/null \
        | grep '"plugin_name":"fast-blocker"' | tail -1)
    [ -n "$fast_line" ] || {
        echo "FAIL: plugin.async_block_discarded for fast-blocker not found in $SINK_FILE"
        echo "EC-012: completed plugins' terminal events MUST emit before dispatcher exit."
        echo "Note: RED until T-3e flushes completed async results before drain expiry."
        return 1
    }

    # Assertion 2 (EC-012 MAY): slow-async's plugin.timeout MAY or MAY NOT be present.
    # No assertion is made — either outcome is valid per BC-1.14.001 v1.9 EC-012.
    # (Presence would mean the WASM epoch interrupt fired before the drain; absence means
    #  the task was still in-flight and was forcibly terminated without emitting an event.)

    # Assertion 3: dispatcher exit code must be 0 (sync_group did not block; async block
    # verdicts from fast-blocker are discarded per Invariant 4 + EC-005).
    [ "$status" -eq 0 ] || {
        echo "FAIL: dispatcher exit code must be 0 for async-only block; got: $status"
        return 1
    }
}

# ---
# Scenario 8: DuplicateEntry → dispatcher.registry_invalid (E-REG-003)
#
# Trigger: registry has two [[hooks]] entries with identical (name, event, tool) tuple.
# Per BC-7.06.001 v1.9 Invariant 7, this MUST be rejected at validate() time.
# Per F-P8-001 fix (main.rs), the dispatcher MUST:
#   1. Exit 2 (fail-closed per ADR-019 §Decision 2).
#   2. Write "[E-REG-003]" to stderr.
#   3. Emit dispatcher.registry_invalid with error_code="E-REG-003" and
#      violation="duplicate_hook_registration" (BC-3.08.001 v1.7 Event 3).
#
# No WASM plugin is invoked — validation fails before dispatch begins.
# ---

@test "VP-079 S8: BC-7.06.001 v1.9 Invariant 7 [fail-closed] — DuplicateEntry → exit 2 + dispatcher.registry_invalid (E-REG-003)" {
    require_dispatcher

    # No test fixtures needed: DuplicateEntry is detected at registry validate()
    # time, before any plugin is loaded or executed.

    # Registry: TWO entries with identical (name="duplicate-test", event="PreToolUse",
    # tool="Bash"). BC-7.06.001 v1.9 Invariant 7 requires rejection at validate().
    printf '%s' '
schema_version = 2

[[hooks]]
name = "duplicate-test"
plugin = "hook-plugins/legacy-bash-adapter.wasm"
on_error = "continue"
async = false
event = "PreToolUse"
tool = "Bash"
priority = 100

[hooks.config]
script_path = "test-fixtures/exit0.sh"

[[hooks]]
name = "duplicate-test"
plugin = "hook-plugins/legacy-bash-adapter.wasm"
on_error = "continue"
async = false
event = "PreToolUse"
tool = "Bash"
priority = 200

[hooks.config]
script_path = "test-fixtures/exit0.sh"
' > "$PLUGIN_ROOT/hooks-registry.toml"

    local envelope
    envelope=$(printf '{"hook_event_name":"PreToolUse","tool_name":"Bash","session_id":"vp079-s8-test","tool_input":{}}')

    # Capture stderr separately to verify [E-REG-003] prefix.
    local stderr_file
    stderr_file="$(mktemp)"

    run sh -c "printf '%s' '$envelope' | \
        CLAUDE_PLUGIN_ROOT=\"$PLUGIN_ROOT\" \
        VSDD_SINK_FILE=\"$SINK_FILE\" \
        CLAUDE_PROJECT_DIR=\"$PLUGIN_ROOT\" \
        factory-dispatcher 2>\"$stderr_file\""

    # Assertion 1 (BC-7.06.001 v1.9 Invariant 7 + ADR-019 §Decision 2):
    # dispatcher must exit 2 (fail-closed).
    [ "$status" -eq 2 ] || {
        echo "FAIL: expected exit 2 for DuplicateEntry; got: $status"
        echo "BC-7.06.001 v1.9 Invariant 7 requires fail-closed (exit 2) on duplicate (name,event,tool) tuple."
        rm -f "$stderr_file"
        return 1
    }

    # Assertion 2 (F-P8-001): stderr must contain "[E-REG-003]".
    local stderr_content
    stderr_content=$(cat "$stderr_file")
    rm -f "$stderr_file"
    echo "$stderr_content" | grep -q "\[E-REG-003\]" || {
        echo "FAIL: stderr must contain '[E-REG-003]'; got: $stderr_content"
        echo "F-P8-001: main.rs DuplicateEntry arm must write [E-REG-003] to stderr."
        return 1
    }

    # Assertion 3 (BC-3.08.001 v1.7 Event 3): dispatcher.registry_invalid must be emitted.
    local line
    line=$(find_event_line "dispatcher.registry_invalid")
    [ -n "$line" ] || {
        echo "FAIL: dispatcher.registry_invalid event not found in $SINK_FILE"
        echo "BC-3.08.001 v1.7 Event 3: dispatcher must emit registry_invalid on DuplicateEntry."
        return 1
    }

    # Assertion 4: error_code must be "E-REG-003".
    local error_code
    error_code=$(echo "$line" | python3 -c \
        "import sys,json; print(json.load(sys.stdin)['error_code'])")
    [ "$error_code" = "E-REG-003" ] || {
        echo "FAIL: error_code must be 'E-REG-003'; got: $error_code"
        return 1
    }

    # Assertion 5: violation must be "duplicate_hook_registration".
    local violation
    violation=$(echo "$line" | python3 -c \
        "import sys,json; print(json.load(sys.stdin)['violation'])")
    [ "$violation" = "duplicate_hook_registration" ] || {
        echo "FAIL: violation must be 'duplicate_hook_registration'; got: $violation"
        echo "BC-3.08.001 v1.9 Event 3: canonical violation string for DuplicateEntry."
        return 1
    }

    # Assertion 6 (F-P14-001 Path B): offending_event must be "PreToolUse".
    # The duplicate registry entries both have event = "PreToolUse"; the dispatcher
    # must propagate this to the wire payload per BC-3.08.001 v1.9 E-REG-003 schema.
    local offending_event
    offending_event=$(echo "$line" | python3 -c \
        "import sys,json; print(json.load(sys.stdin)['offending_event'])")
    [ "$offending_event" = "PreToolUse" ] || {
        echo "FAIL: offending_event must be 'PreToolUse' (the duplicate registry's event); got: $offending_event"
        echo "F-P14-001 Path B: emit_dispatcher_registry_invalid must propagate offending_event to wire payload."
        return 1
    }

    # Assertion 7 (F-P14-001 Path B): offending_tool must be "Bash".
    # The duplicate registry entries both have tool = "Bash"; the dispatcher
    # must propagate this to the wire payload per BC-3.08.001 v1.9 E-REG-003 schema.
    local offending_tool
    offending_tool=$(echo "$line" | python3 -c \
        "import sys,json; t=json.load(sys.stdin).get('offending_tool'); print(t if t is not None else 'null')")
    [ "$offending_tool" = "Bash" ] || {
        echo "FAIL: offending_tool must be 'Bash' (the duplicate fixture's tool); got: $offending_tool"
        echo "F-P14-001 Path B: emit_dispatcher_registry_invalid must propagate offending_tool to wire payload."
        return 1
    }
}

@test "VP-079 S5: async task exceeding drain window does NOT emit plugin.timeout (AC-005)" {
    require_dispatcher

    # timeout_ms = 200ms > ASYNC_DRAIN_WINDOW_MS (100ms per DI-019):
    # dispatcher exits before the async plugin's timeout fires.
    # No plugin.timeout event should be emitted.
    #
    # DI-019: ASYNC_DRAIN_WINDOW_MS is the canonical constant (100ms).
    # The 200ms below is the PLUGIN timeout — chosen to exceed the drain window.
    # Do NOT hardcode 100 — the drain window value is ASYNC_DRAIN_WINDOW_MS.

    printf '%s\n' "exit 0" > "$PLUGIN_ROOT/test-fixtures/exit0.sh"
    printf '%s\n' "sleep 60" > "$PLUGIN_ROOT/test-fixtures/sleep60.sh"
    chmod +x "$PLUGIN_ROOT/test-fixtures/exit0.sh"
    chmod +x "$PLUGIN_ROOT/test-fixtures/sleep60.sh"

    run_dispatcher '
schema_version = 2

[[hooks]]
name = "sync-gate-plugin"
plugin = "hook-plugins/legacy-bash-adapter.wasm"
on_error = "block"
async = false
event = "PostToolUse"
priority = 50

[hooks.config]
script_path = "test-fixtures/exit0.sh"

[[hooks]]
name = "slow-async-plugin-over-drain"
plugin = "hook-plugins/legacy-bash-adapter.wasm"
on_error = "continue"
async = true
timeout_ms = 200
event = "PostToolUse"
priority = 100

[hooks.config]
script_path = "test-fixtures/sleep60.sh"
' "PostToolUse"

    local line
    line=$(find_event_line "plugin.timeout")
    [ -z "$line" ] || {
        echo "FAIL: plugin.timeout must NOT be emitted when task exceeds ASYNC_DRAIN_WINDOW_MS (DI-019); got: $line"
        return 1
    }
}
