#!/usr/bin/env bats
# validate-unvalidated-mutation-marker.bats — VP-105 integration tests.
#
# Tests the two-arm next-advance gate (BC-1.18.002) end-to-end via the real
# factory-dispatcher binary + the compiled WASM plugin.
#
# VP-105-A through VP-105-G cover:
#   A: marker present → Agent dispatch blocked (BC-1.18.002 PC1; AC-007)
#   B: marker absent  → Agent dispatch allowed (BC-1.18.002 PC3; AC-010)
#   C: operator rm of marker unblocks both arms (BC-1.18.003 PC3; AC-014)
#   D: Edit tool dispatch not gated (BC-1.18.002 PC3; AC-009)
#   E: marker present + git commit Bash dispatch → blocked (BC-1.18.002 PC2; AC-008)
#   F: marker absent  + git push Bash dispatch  → allowed (BC-1.18.002 PC4; AC-010)
#   G: git status Bash dispatch not gated even when marker exists (BC-1.18.002 PC3; AC-009)
#
# Red Gate state: these tests FAIL until T-4 (build WASM binary) and T-5
# (register in hooks-registry.toml) are complete. The assertion
# `[ -f "$GATE_WASM" ]` (in _require_gate_wasm) fails explicitly —
# this is the correct Red Gate state per BC-5.38.001 (S-25.01 tdd_mode=strict).
#
# Story:    S-25.01 — Dispatcher INDETERMINATE Outcome Layer 1
# BCs:      BC-1.18.002 §PC1 §PC2 §PC3 §PC4 (gate block/allow rules)
#           BC-1.18.003 §PC3 (operator escape hatch — manual rm)
# VPs:      VP-105 (all 7 scenarios: A–G)
# ADR:      ADR-047 §Decision 4 (gate entries), §Decision 5 (marker clear)
#
# Run:
#   bats plugins/vsdd-factory/tests/validate-unvalidated-mutation-marker.bats

# ---------------------------------------------------------------------------
# setup / teardown
# ---------------------------------------------------------------------------

setup() {
    REPO_ROOT="$(cd "${BATS_TEST_DIRNAME}/../../.." && pwd)"
    PLUGIN_ROOT="${BATS_TEST_DIRNAME}/.."
    DISPATCHER="$REPO_ROOT/target/release/factory-dispatcher"
    # The gate plugin WASM — built by implementer in T-4 and placed per T-4/T-5.
    GATE_WASM="$PLUGIN_ROOT/hook-plugins/validate-unvalidated-mutation-marker.wasm"
    # Isolated scratch dir — each test gets a clean .factory/ directory.
    WORK="$(mktemp -d)"
    mkdir -p "$WORK/.factory"
    export VSDD_LOG_DIR="$WORK/.factory/logs"
    mkdir -p "$VSDD_LOG_DIR"
}

teardown() {
    rm -rf "$WORK"
    unset VSDD_LOG_DIR CLAUDE_PLUGIN_ROOT CLAUDE_PROJECT_DIR
}

# ---------------------------------------------------------------------------
# Preflight helpers
# ---------------------------------------------------------------------------

# Require the dispatcher binary. Skips the test if not built.
# Pattern: same as check-harness-version.bats TC-ENV-001.
_require_dispatcher() {
    if [ ! -x "$DISPATCHER" ]; then
        skip "factory-dispatcher binary not built — run: cargo build --release -p factory-dispatcher"
    fi
}

# Require the gate WASM binary. FAILS (does NOT skip) if missing.
# Red Gate: the WASM binary does not exist until T-4 is implemented.
# A failing assertion here is the correct Red Gate state for bats tests.
_require_gate_wasm() {
    if [ ! -f "$GATE_WASM" ]; then
        echo "FAIL: validate-unvalidated-mutation-marker.wasm not found at: $GATE_WASM" >&2
        echo "Implementer: build it via:" >&2
        echo "  cargo build --target wasm32-wasip1 --release -p validate-unvalidated-mutation-marker" >&2
        echo "  cp target/wasm32-wasip1/release/validate-unvalidated-mutation-marker.wasm $GATE_WASM" >&2
        echo "(Red Gate: this failure is EXPECTED before T-4 is complete — BC-5.38.001 tdd_mode=strict)" >&2
        return 1  # explicit failure, not skip
    fi
}

# Write a valid 5-field TOML marker to .factory/unvalidated-mutation.marker.
# $1: WORK dir whose .factory/ receives the marker.
_write_marker() {
    local work_dir="$1"
    cat > "$work_dir/.factory/unvalidated-mutation.marker" << 'TOML'
timestamp = "2026-08-30T00:00:00Z"
plugin_name = "validate-factory-path-staging"
artifact_path = "/tmp/.factory/STATE.md"
cause = "fuel"
trace_id = "deadbeef-0001-0001-0001-000000000001"
TOML
}

# Invoke the real dispatcher with a PreToolUse Agent payload.
# Uses CLAUDE_PLUGIN_ROOT pointing to the real plugin root so the dispatcher
# reads the production hooks-registry.toml (which includes the gate plugin).
# Exports CLAUDE_PROJECT_DIR=$WORK so the dispatcher resolves .factory/ there.
# Returns the dispatcher exit code via `run`.
_run_dispatcher_pretooluse_agent() {
    local work_dir="$1"
    local payload
    payload=$(jq -nc '{
        hook_event_name: "PreToolUse",
        event_name: "PreToolUse",
        tool_name: "Agent",
        session_id: "vp105-test-session",
        tool_input: {
            subagent_type: "vsdd-factory:test-writer",
            prompt: "Dispatch an Agent for VP-105 test"
        }
    }')
    run env \
        CLAUDE_PLUGIN_ROOT="$PLUGIN_ROOT" \
        CLAUDE_PROJECT_DIR="$work_dir" \
        VSDD_LOG_DIR="$work_dir/.factory/logs" \
        bash -c "cd '$PLUGIN_ROOT' && printf '%s' '$payload' | '$DISPATCHER'" 2>&1
}

# Invoke the real dispatcher with a PreToolUse Bash payload.
# $1: WORK dir; $2: the command string for the Bash tool_input.
_run_dispatcher_pretooluse_bash() {
    local work_dir="$1"
    local command_str="$2"
    local payload
    payload=$(jq -nc --arg cmd "$command_str" '{
        hook_event_name: "PreToolUse",
        event_name: "PreToolUse",
        tool_name: "Bash",
        session_id: "vp105-test-session",
        tool_input: {
            command: $cmd
        }
    }')
    run env \
        CLAUDE_PLUGIN_ROOT="$PLUGIN_ROOT" \
        CLAUDE_PROJECT_DIR="$work_dir" \
        VSDD_LOG_DIR="$work_dir/.factory/logs" \
        bash -c "cd '$PLUGIN_ROOT' && printf '%s' '$payload' | '$DISPATCHER'" 2>&1
}

# Invoke the real dispatcher with a PreToolUse Edit payload (non-Agent, non-Bash).
# Arm 1 and Arm 2 must NOT gate Edit dispatches (AC-009 / BC-1.18.002 PC3).
_run_dispatcher_pretooluse_edit() {
    local work_dir="$1"
    local payload
    payload=$(jq -nc '{
        hook_event_name: "PreToolUse",
        event_name: "PreToolUse",
        tool_name: "Edit",
        session_id: "vp105-test-session",
        tool_input: {
            file_path: "/tmp/foo.rs",
            old_string: "old",
            new_string: "new"
        }
    }')
    run env \
        CLAUDE_PLUGIN_ROOT="$PLUGIN_ROOT" \
        CLAUDE_PROJECT_DIR="$work_dir" \
        VSDD_LOG_DIR="$work_dir/.factory/logs" \
        bash -c "cd '$PLUGIN_ROOT' && printf '%s' '$payload' | '$DISPATCHER'" 2>&1
}

# ---------------------------------------------------------------------------
# VP-105-A: marker present → Agent dispatch blocked (AC-007; BC-1.18.002 PC1)
# ---------------------------------------------------------------------------

@test "VP-105-A: marker present → Agent dispatch blocked (exit 2; AC-007; BC-1.18.002 PC1)" {
    _require_dispatcher
    _require_gate_wasm

    # Write a valid marker to simulate an INDETERMINATE state
    _write_marker "$WORK"

    _run_dispatcher_pretooluse_agent "$WORK"

    # Dispatcher MUST exit 2 when the gate plugin returns exit_code=2 (block)
    [ "$status" -eq 2 ] || {
        echo "FAIL: expected dispatcher exit 2 (block) when marker exists; got exit $status"
        echo "Output: $output"
        return 1
    }
}

# ---------------------------------------------------------------------------
# VP-105-B: marker absent → Agent dispatch allowed (AC-010; BC-1.18.002 PC3)
# ---------------------------------------------------------------------------

@test "VP-105-B: marker absent → Agent dispatch allowed (exit 0; AC-010; BC-1.18.002 PC3)" {
    _require_dispatcher
    _require_gate_wasm

    # No marker written — .factory/ directory is empty
    [ ! -f "$WORK/.factory/unvalidated-mutation.marker" ] || \
        rm "$WORK/.factory/unvalidated-mutation.marker"

    _run_dispatcher_pretooluse_agent "$WORK"

    # Dispatcher MUST exit 0 when the gate plugin returns exit_code=0 (allow)
    [ "$status" -eq 0 ] || {
        echo "FAIL: expected dispatcher exit 0 (allow) when marker absent; got exit $status"
        echo "Output: $output"
        return 1
    }
}

# ---------------------------------------------------------------------------
# VP-105-C: operator rm → both arms unblocked (AC-014; BC-1.18.003 PC3)
# ---------------------------------------------------------------------------

@test "VP-105-C: operator rm of marker unblocks both arms (AC-014; BC-1.18.003 PC3)" {
    _require_dispatcher
    _require_gate_wasm

    # Step 1: write marker → verify it blocks
    _write_marker "$WORK"

    _run_dispatcher_pretooluse_agent "$WORK"
    [ "$status" -eq 2 ] || {
        echo "FAIL: expected block (exit 2) before rm; got exit $status"
        echo "Output: $output"
        return 1
    }

    # Step 2: operator manual escape hatch — rm the marker
    rm "$WORK/.factory/unvalidated-mutation.marker"

    # Step 3: verify both arms now allow
    _run_dispatcher_pretooluse_agent "$WORK"
    [ "$status" -eq 0 ] || {
        echo "FAIL: expected allow (exit 0) after rm (Arm 1 Agent); got exit $status"
        echo "Output: $output"
        return 1
    }

    # Arm 2 (git commit) must also be unblocked after rm
    _run_dispatcher_pretooluse_bash "$WORK" "git commit -m 'post-rm commit'"
    [ "$status" -eq 0 ] || {
        echo "FAIL: expected allow (exit 0) after rm (Arm 2 Bash git commit); got exit $status"
        echo "Output: $output"
        return 1
    }
}

# ---------------------------------------------------------------------------
# VP-105-D: Edit tool not gated (AC-009; BC-1.18.002 PC3)
# ---------------------------------------------------------------------------

@test "VP-105-D: Edit tool dispatch not gated even when marker exists (AC-009; BC-1.18.002 PC3)" {
    _require_dispatcher
    _require_gate_wasm

    # Marker present — gate plugin is active
    _write_marker "$WORK"

    # Edit dispatch must NOT be blocked — neither Arm 1 (^Agent$) nor Arm 2 (^Bash$) fires
    _run_dispatcher_pretooluse_edit "$WORK"

    # Edit is not Agent, so Arm 1 must not fire; Edit is not Bash, so Arm 2 must not fire
    [ "$status" -eq 0 ] || {
        echo "FAIL: Edit dispatch MUST NOT be gated by validate-unvalidated-mutation-marker"
        echo "Expected exit 0 (allow); got exit $status"
        echo "Output: $output"
        return 1
    }
}

# ---------------------------------------------------------------------------
# VP-105-E: marker present + git commit Bash → blocked (AC-008; BC-1.18.002 PC2)
# ---------------------------------------------------------------------------

@test "VP-105-E: marker present + git commit Bash dispatch → blocked (AC-008; BC-1.18.002 PC2)" {
    _require_dispatcher
    _require_gate_wasm

    # Write marker
    _write_marker "$WORK"

    # git commit -m 'test' matches \bgit\b.*\b(commit|push)\b — MUST be blocked
    _run_dispatcher_pretooluse_bash "$WORK" "git commit -m 'test message'"

    [ "$status" -eq 2 ] || {
        echo "FAIL: expected block (exit 2) for 'git commit' when marker exists; got exit $status"
        echo "Output: $output"
        return 1
    }
}

# ---------------------------------------------------------------------------
# VP-105-F: marker absent + git push Bash → allowed (AC-010; BC-1.18.002 PC4)
# ---------------------------------------------------------------------------

@test "VP-105-F: marker absent + git push Bash dispatch → allowed (AC-010; BC-1.18.002 PC4)" {
    _require_dispatcher
    _require_gate_wasm

    # No marker present
    [ ! -f "$WORK/.factory/unvalidated-mutation.marker" ] || \
        rm "$WORK/.factory/unvalidated-mutation.marker"

    # git push with no marker must allow
    _run_dispatcher_pretooluse_bash "$WORK" "git push origin factory-artifacts"

    [ "$status" -eq 0 ] || {
        echo "FAIL: expected allow (exit 0) for 'git push' when marker absent; got exit $status"
        echo "Output: $output"
        return 1
    }
}

# ---------------------------------------------------------------------------
# VP-105-G: git status not gated even when marker exists (AC-009; BC-1.18.002 PC3)
# ---------------------------------------------------------------------------

@test "VP-105-G: git status Bash dispatch not gated even when marker exists (AC-009; BC-1.18.002 PC3)" {
    _require_dispatcher
    _require_gate_wasm

    # Marker present — Arm 2 fires for git commit/push BUT must NOT fire for git status
    _write_marker "$WORK"

    # git status does NOT match \bgit\b.*\b(commit|push)\b — Arm 2 must NOT gate it
    _run_dispatcher_pretooluse_bash "$WORK" "git status --porcelain"

    [ "$status" -eq 0 ] || {
        echo "FAIL: git status MUST NOT be gated (only commit/push are gated by Arm 2)"
        echo "Expected exit 0 (allow); got exit $status"
        echo "Output: $output"
        return 1
    }
}
