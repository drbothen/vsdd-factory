#!/usr/bin/env bats
# resolver-integration.bats — End-to-end integration test for the
# WaveContextResolver → validate-per-story-adversary-convergence hook pipeline.
#
# Closes F-P2-001: proves the convergence hook is operationally effective in
# production by exercising the complete producer (WaveContextResolver) →
# consumer (convergence hook) pipeline.
#
# AC-008: unconverged story → dispatcher exits 2 (Block)
# AC-009: all converged → dispatcher exits 0 (Continue)
#
# S-12.08 implementation status:
# - Unit tests: 48/48 passing (cargo test -p validate-per-story-adversary-convergence)
# - hook_logic: rewired to use extract_stories_from_wave_context (AC-001 through AC-010)
# - WASM artifact: rebuilt with correct wave_context path
# - Integration test status: GREEN (S-12.08 Step 3b — resolver linker WASI fix)
#
# Fix (S-12.08 Step 3b): WaveContextResolver WASM uses wasm32-wasip1 target (Rust
# standard library) which imports wasi_snapshot_preview1::environ_get and related
# syscalls. resolver_loader.rs::CompiledWasmResolver::resolve now uses ResolverStoreData
# (host: HostContext + wasi: WasiP1Ctx) and calls p1::add_to_linker_sync to wire
# WASI p1 syscalls. The WasiCtx is restricted (no preopens, no stdio) to maintain
# the resolver sandbox (BC-4.12.003 INV2). Mirrors the pattern in invoke.rs::StoreData.
#
# BC traces:
#   BC-1.13.001 postcondition 4 — needs_context = ["wave_context"] triggers
#     WaveContextResolver injection before each hook dispatch.
#   BC-4.10.001 postcondition 1 — hook reads story list from wave_context.stories.
#   BC-4.10.001 postcondition 2 — unconverged story → Block.
#   F-P2-001, F-P2-008 — root cause: hook never received story list; fixed by
#     consuming wave_context.stories produced by WaveContextResolver (S-12.07).

# ---------------------------------------------------------------------------
# Setup / teardown helpers
# ---------------------------------------------------------------------------

setup() {
    # Create a temporary working directory for each test.
    # BATS_TMPDIR is provided by the bats runtime.
    FACTORY_TMP="$(mktemp -d "${BATS_TMPDIR}/resolver-integration-XXXXXX")"

    # Determine the repo root and dispatcher binary path.
    # CARGO_MANIFEST_DIR is not set in bats; resolve from __file__.
    # Path: plugins/vsdd-factory/tests/<file> → 3 levels up = worktree root.
    REPO_ROOT="$(cd "$(dirname "${BATS_TEST_FILENAME}")/../../.." && pwd)"
    PLUGIN_ROOT="${REPO_ROOT}/plugins/vsdd-factory"

    # MED-004: binary-missing is FATAL (not a skip). Pre-build the dispatcher if needed.
    # We prefer the debug build for fast iteration; fall back to release if available.
    DISPATCHER="${REPO_ROOT}/target/debug/factory-dispatcher"
    if [[ ! -x "${DISPATCHER}" ]]; then
        DISPATCHER="${REPO_ROOT}/target/release/factory-dispatcher"
    fi
    if [[ ! -x "${DISPATCHER}" ]]; then
        # Neither build exists — build now (debug). Fail hard if cargo fails.
        cargo build -p factory-dispatcher >&2 || {
            echo "FATAL: factory-dispatcher build failed" >&2
            exit 1
        }
        DISPATCHER="${REPO_ROOT}/target/debug/factory-dispatcher"
        if [[ ! -x "${DISPATCHER}" ]]; then
            echo "FATAL: dispatcher binary not found at ${DISPATCHER} after build" >&2
            exit 1
        fi
    fi

    export REPO_ROOT PLUGIN_ROOT DISPATCHER FACTORY_TMP
}

teardown() {
    # Remove the temporary factory root after each test.
    if [[ -n "${FACTORY_TMP}" && -d "${FACTORY_TMP}" ]]; then
        rm -rf "${FACTORY_TMP}"
    fi
}

# ---------------------------------------------------------------------------
# Shared helpers (used by both test cases)
# ---------------------------------------------------------------------------

# Seed the shared filesystem fixtures for both test cases.
# Args:
#   $1 - passes_clean for S-FAKE-001 (unconverged = 1, converged = 3)
#   $2 - passes_clean for S-FAKE-002 (always converged = 3 in both cases)
seed_factory_root() {
    local s001_passes="${1}"
    local s002_passes="${2}"

    # .factory/STATE.md — current_cycle pointer (YAML frontmatter format)
    mkdir -p "${FACTORY_TMP}/.factory"
    cat > "${FACTORY_TMP}/.factory/STATE.md" <<'EOF'
---
current_cycle: test-cycle-001
---
EOF

    # .factory/wave-state.yaml — active wave with two stories.
    # Uses the canonical WaveState schema (waves: list with WaveEntry structs).
    # MED-005: gate_status: not_started explicitly seeded (non-terminal per BC-8.14.009)
    # to avoid relying on serde default and make the active-wave state unambiguous.
    cat > "${FACTORY_TMP}/.factory/wave-state.yaml" <<'EOF'
waves:
  - wave: test-wave-001
    stories:
      - S-FAKE-001
      - S-FAKE-002
    stories_merged: []
    gate_status: not_started
EOF

    # .factory/cycles/test-cycle-001/S-FAKE-001/adversary-convergence-state.json
    mkdir -p "${FACTORY_TMP}/.factory/cycles/test-cycle-001/S-FAKE-001"
    cat > "${FACTORY_TMP}/.factory/cycles/test-cycle-001/S-FAKE-001/adversary-convergence-state.json" \
        <<EOF
{
  "passes_clean": ${s001_passes},
  "last_classification": "NITPICK_ONLY",
  "last_finding_count": 0,
  "last_timestamp": "2026-05-10T00:00:00Z",
  "deferred_findings": []
}
EOF

    # .factory/cycles/test-cycle-001/S-FAKE-002/adversary-convergence-state.json
    mkdir -p "${FACTORY_TMP}/.factory/cycles/test-cycle-001/S-FAKE-002"
    cat > "${FACTORY_TMP}/.factory/cycles/test-cycle-001/S-FAKE-002/adversary-convergence-state.json" \
        <<EOF
{
  "passes_clean": ${s002_passes},
  "last_classification": "NITPICK_ONLY",
  "last_finding_count": 0,
  "last_timestamp": "2026-05-10T00:00:00Z",
  "deferred_findings": []
}
EOF
}

# ---------------------------------------------------------------------------
# AC-008: F-P2-001 closure — unconverged story → dispatcher exits 2 (Block)
# ---------------------------------------------------------------------------

@test "F-P2-001 closure: unconverged story → dispatcher exits 2 (Block)" {
    # Seed: S-FAKE-001 unconverged (passes_clean=1), S-FAKE-002 converged (passes_clean=3)
    seed_factory_root 1 3

    # VSDD_SINK_FILE: capture internal events to verify block code.
    # The dispatcher emits hook.block events (with the CONVERGENCE_PASSES_INSUFFICIENT code)
    # to the internal log. VSDD_SINK_FILE (debug-only) flushes them to a JSONL file.
    local sink_file
    sink_file="$(mktemp "${BATS_TMPDIR}/resolver-sink-XXXXXX.jsonl")"

    # Run dispatcher with synthetic SubagentStop event via stdin.
    # CWD = PLUGIN_ROOT so resolver WASM path resolves correctly.
    #
    # last_assistant_message (>= 40 non-whitespace chars) satisfies the
    # handoff-validator so it does NOT block — the convergence hook is the
    # intended blocking path for this test (priority 960 > handoff-validator 910).
    local payload='{"event_name":"SubagentStop","session_id":"bats-test-session","dispatcher_trace_id":"bats-test-trace","agent_type":"wave-gate-dispatch","last_assistant_message":"Wave gate adversary pass completed for this iteration of the story review cycle."}'
    run bash -c "cd '${PLUGIN_ROOT}' && printf '%s' '${payload}' | VSDD_SINK_FILE='${sink_file}' CLAUDE_PLUGIN_ROOT='${PLUGIN_ROOT}' CLAUDE_PROJECT_DIR='${FACTORY_TMP}' '${DISPATCHER}'"

    # AC-008: exit code 2 = Block (dispatcher convention: 0=Continue, 2=Block)
    [ "${status}" -eq 2 ]

    # Verify CONVERGENCE_PASSES_INSUFFICIENT in internal events (VSDD_SINK_FILE).
    # The hook.block event carries code="per_story_adversary_unconverged_CONVERGENCE_PASSES_INSUFFICIENT"
    # and story="S-FAKE-001". Both must appear in the sink JSONL.
    # (S-FAKE-001 has passes_clean=1 < 3).
    [[ -s "${sink_file}" ]] || {
        echo "SINK FILE EMPTY: ${sink_file}"
        echo "dispatcher output: ${output}"
        false
    }
    grep -q "CONVERGENCE_PASSES_INSUFFICIENT" "${sink_file}"
    grep -q "S-FAKE-001" "${sink_file}"

    rm -f "${sink_file}"
}

# ---------------------------------------------------------------------------
# AC-009: F-P2-001 closure — all converged → dispatcher exits 0 (Continue)
# ---------------------------------------------------------------------------

@test "F-P2-001 closure: all converged → dispatcher exits 0 (Continue)" {
    # Seed: ALL stories converged (passes_clean=3)
    seed_factory_root 3 3

    # Run dispatcher with synthetic SubagentStop event via stdin.
    # last_assistant_message satisfies the handoff-validator (>= 40 non-whitespace chars).
    local sink_file
    sink_file="$(mktemp "${BATS_TMPDIR}/resolver-sink-XXXXXX.jsonl")"

    local payload='{"event_name":"SubagentStop","session_id":"bats-test-session","dispatcher_trace_id":"bats-test-trace","agent_type":"wave-gate-dispatch","last_assistant_message":"Wave gate adversary pass completed for this iteration of the story review cycle."}'
    run bash -c "cd '${PLUGIN_ROOT}' && printf '%s' '${payload}' | VSDD_SINK_FILE='${sink_file}' CLAUDE_PLUGIN_ROOT='${PLUGIN_ROOT}' CLAUDE_PROJECT_DIR='${FACTORY_TMP}' '${DISPATCHER}'"

    # AC-009: exit code 0 = Continue (all stories converged)
    [ "${status}" -eq 0 ]

    # No convergence block code in sink events.
    # When all stories are converged, hook.block must NOT appear for the convergence hook.
    ! grep -q "CONVERGENCE_PASSES_INSUFFICIENT" "${sink_file}" 2>/dev/null || {
        echo "UNEXPECTED CONVERGENCE BLOCK in sink: $(grep 'CONVERGENCE_PASSES_INSUFFICIENT' "${sink_file}")"
        false
    }

    # No WAVE_CONTEXT_MISSING block — confirms resolver injected context successfully.
    ! grep -q "WAVE_CONTEXT_MISSING" "${sink_file}" 2>/dev/null || {
        echo "UNEXPECTED WAVE_CONTEXT_MISSING in sink: $(grep 'WAVE_CONTEXT_MISSING' "${sink_file}")"
        false
    }

    rm -f "${sink_file}"
}
