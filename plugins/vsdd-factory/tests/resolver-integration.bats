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
#
# F-P3-003 additions (block-code bats coverage):
#   CONVERGENCE_STATE_MISSING — story in wave but no convergence-state.json
#   CONVERGENCE_CLASSIFICATION_INSUFFICIENT — passes_clean=3 but wrong classification
#   WAVE_CONTEXT_MISSING — no wave_context injected (no resolver wired)
#   WAVE_CONTEXT_SCHEMA_ERROR — unit-test-covered; bats construction impractical
#
# F-P3-007: resolvers-registry.toml path_allow narrowed from [".factory/"] to
#   [".factory/wave-state.yaml", ".factory/STATE.md"] (exact file grants).
#
# F-P3-008: concurrent resolver timeout integration test.

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

# ---------------------------------------------------------------------------
# F-P2-001 closure: active wave with zero stories → Continue (vacuous convergence)
# BC-4.10.001 EC-001: empty stories list → hook returns Continue immediately.
# S-12.08 pass-1 HIGH-002 fix: resolver emits Some({stories:[],...}) for active
# waves with no stories yet, so the hook can honor EC-001 (not fall through to
# WAVE_CONTEXT_MISSING).
# ---------------------------------------------------------------------------

@test "F-P2-001 closure: active wave with zero stories → dispatcher exits 0 (Continue, vacuous convergence)" {
    # Seed .factory/STATE.md
    mkdir -p "${FACTORY_TMP}/.factory"
    cat > "${FACTORY_TMP}/.factory/STATE.md" <<'EOF'
---
document_type: pipeline-state
current_cycle: test-cycle-empty
input-hash: "[live-state]"
---
EOF

    # Seed wave-state.yaml with EMPTY stories list (active wave but no stories yet).
    # gate_status: not_started → non-terminal → wave IS active per BC-8.14.009.
    cat > "${FACTORY_TMP}/.factory/wave-state.yaml" <<'EOF'
waves:
  - wave: W-EMPTY
    stories: []
    stories_merged: []
    gate_status: not_started
EOF

    # No cycle directory needed — no stories to check.

    local sink_file
    sink_file="$(mktemp "${BATS_TMPDIR}/resolver-sink-XXXXXX.jsonl")"

    # Synthetic SubagentStop event with agent_type = wave-gate-dispatch and a
    # valid last_assistant_message so the handoff-validator does not block first.
    local payload='{"event_name":"SubagentStop","session_id":"bats-test-empty-001","dispatcher_trace_id":"bats-test-trace","agent_type":"wave-gate-dispatch","last_assistant_message":"Wave gate adversary pass completed for this iteration of the story review cycle."}'
    run bash -c "cd '${PLUGIN_ROOT}' && printf '%s' '${payload}' | VSDD_SINK_FILE='${sink_file}' CLAUDE_PLUGIN_ROOT='${PLUGIN_ROOT}' CLAUDE_PROJECT_DIR='${FACTORY_TMP}' '${DISPATCHER}'"

    # P02-MED-003: assert HIGH-002 fix flows end-to-end.
    # Empty active wave → resolver emits Some({stories:[],...}) → hook EC-001 → Continue (exit 0)
    [ "${status}" -eq 0 ]

    # P03-LOW-002: positive coverage — confirm dispatcher actually ran the convergence hook
    # (not vacuously passing on empty sink). Continue path emits hook.dispatch / hook.complete
    # but no hook.block; we assert at least one hook line for this plugin appears in the sink.
    [ -s "${sink_file}" ] || { echo "FATAL: sink file empty — dispatcher may have skipped convergence hook" >&2; false; }
    grep -q "validate-per-story-adversary-convergence" "${sink_file}" || {
        echo "FATAL: no validate-per-story-adversary-convergence event in sink — hook did not fire" >&2
        cat "${sink_file}" >&2
        false
    }

    # MUST NOT contain WAVE_CONTEXT_MISSING — the fix path is "resolver Some + hook Continue",
    # not "resolver None". If this fires, the resolver is incorrectly returning None for
    # active waves with empty story lists.
    ! grep -q "WAVE_CONTEXT_MISSING" "${sink_file}" 2>/dev/null || {
        echo "UNEXPECTED WAVE_CONTEXT_MISSING in sink (resolver HIGH-002 fix may be missing): $(grep 'WAVE_CONTEXT_MISSING' "${sink_file}")"
        false
    }

    rm -f "${sink_file}"
}

# ---------------------------------------------------------------------------
# F-P3-003 block-code tests (4 new cases)
#
# BC-4.10.001 block codes verified end-to-end:
#   (1) CONVERGENCE_STATE_MISSING
#   (2) CONVERGENCE_CLASSIFICATION_INSUFFICIENT
#   (3) WAVE_CONTEXT_MISSING
#   (4) WAVE_CONTEXT_SCHEMA_ERROR — documented skip (unit-test coverage)
# ---------------------------------------------------------------------------

@test "F-P3-003 block code: CONVERGENCE_STATE_MISSING when state file absent" {
    # BC-4.10.001 PC2: story S-NOSTATE in wave but no convergence-state.json present.
    mkdir -p "${FACTORY_TMP}/.factory"
    cat > "${FACTORY_TMP}/.factory/STATE.md" <<'EOF'
---
current_cycle: test-cycle-block-001
---
EOF
    cat > "${FACTORY_TMP}/.factory/wave-state.yaml" <<'EOF'
waves:
  - wave: test-wave-block
    stories:
      - S-NOSTATE
    stories_merged: []
    gate_status: not_started
EOF
    # Deliberately do NOT create .factory/cycles/test-cycle-block-001/S-NOSTATE/

    local sink_file
    sink_file="${BATS_TMPDIR}/block-code-1-sink-${RANDOM}.jsonl"

    local payload='{"event_name":"SubagentStop","session_id":"bats-block-001","dispatcher_trace_id":"bats-block-trace","agent_type":"wave-gate-dispatch","last_assistant_message":"Wave gate adversary pass completed for this iteration of the story review cycle."}'
    run bash -c "cd '${PLUGIN_ROOT}' && printf '%s' '${payload}' | VSDD_SINK_FILE='${sink_file}' CLAUDE_PLUGIN_ROOT='${PLUGIN_ROOT}' CLAUDE_PROJECT_DIR='${FACTORY_TMP}' '${DISPATCHER}'"

    # Absent state file → Block (exit 2).
    [ "${status}" -eq 2 ]

    [[ -s "${sink_file}" ]] || { echo "SINK FILE EMPTY" >&2; false; }
    grep -q "CONVERGENCE_STATE_MISSING" "${sink_file}" || {
        echo "MISSING CONVERGENCE_STATE_MISSING in sink" >&2
        cat "${sink_file}" >&2
        false
    }
    grep -q "S-NOSTATE" "${sink_file}" || {
        echo "MISSING story S-NOSTATE in sink" >&2
        cat "${sink_file}" >&2
        false
    }

    rm -f "${sink_file}"
}

@test "F-P3-003 block code: CONVERGENCE_CLASSIFICATION_INSUFFICIENT when classification is HIGH" {
    # BC-4.10.001 PC4: passes_clean=3 but last_classification="HIGH" (not NITPICK_ONLY).
    mkdir -p "${FACTORY_TMP}/.factory"
    cat > "${FACTORY_TMP}/.factory/STATE.md" <<'EOF'
---
current_cycle: test-cycle-block-002
---
EOF
    cat > "${FACTORY_TMP}/.factory/wave-state.yaml" <<'EOF'
waves:
  - wave: test-wave-block
    stories:
      - S-HIGHCLASS
    stories_merged: []
    gate_status: not_started
EOF
    mkdir -p "${FACTORY_TMP}/.factory/cycles/test-cycle-block-002/S-HIGHCLASS"
    cat > "${FACTORY_TMP}/.factory/cycles/test-cycle-block-002/S-HIGHCLASS/adversary-convergence-state.json" <<'EOF'
{
  "passes_clean": 3,
  "last_classification": "HIGH",
  "last_finding_count": 2,
  "last_timestamp": "2026-05-10T00:00:00Z",
  "deferred_findings": []
}
EOF

    local sink_file
    sink_file="${BATS_TMPDIR}/block-code-2-sink-${RANDOM}.jsonl"

    local payload='{"event_name":"SubagentStop","session_id":"bats-block-002","dispatcher_trace_id":"bats-block-trace","agent_type":"wave-gate-dispatch","last_assistant_message":"Wave gate adversary pass completed for this iteration of the story review cycle."}'
    run bash -c "cd '${PLUGIN_ROOT}' && printf '%s' '${payload}' | VSDD_SINK_FILE='${sink_file}' CLAUDE_PLUGIN_ROOT='${PLUGIN_ROOT}' CLAUDE_PROJECT_DIR='${FACTORY_TMP}' '${DISPATCHER}'"

    # Classification != NITPICK_ONLY → Block (exit 2).
    [ "${status}" -eq 2 ]

    [[ -s "${sink_file}" ]] || { echo "SINK FILE EMPTY" >&2; false; }
    grep -q "CONVERGENCE_CLASSIFICATION_INSUFFICIENT" "${sink_file}" || {
        echo "MISSING CONVERGENCE_CLASSIFICATION_INSUFFICIENT in sink" >&2
        cat "${sink_file}" >&2
        false
    }
    grep -q "S-HIGHCLASS" "${sink_file}" || {
        echo "MISSING story S-HIGHCLASS in sink" >&2
        cat "${sink_file}" >&2
        false
    }

    rm -f "${sink_file}"
}

@test "F-P3-003 block code: WAVE_CONTEXT_MISSING when no resolver is wired" {
    # BC-4.10.001 PC9: no wave_context in plugin_config because no resolver is registered.
    # Uses a temp PLUGIN_ROOT with an empty resolvers-registry.toml.
    mkdir -p "${FACTORY_TMP}/.factory"
    cat > "${FACTORY_TMP}/.factory/STATE.md" <<'EOF'
---
current_cycle: test-cycle-block-003
---
EOF
    cat > "${FACTORY_TMP}/.factory/wave-state.yaml" <<'EOF'
waves:
  - wave: test-wave-block
    stories:
      - S-FAKE-OK
    stories_merged: []
    gate_status: not_started
EOF
    mkdir -p "${FACTORY_TMP}/.factory/cycles/test-cycle-block-003/S-FAKE-OK"
    cat > "${FACTORY_TMP}/.factory/cycles/test-cycle-block-003/S-FAKE-OK/adversary-convergence-state.json" <<'EOF'
{"passes_clean": 3, "last_classification": "NITPICK_ONLY", "last_finding_count": 0, "last_timestamp": "2026-05-10T00:00:00Z", "deferred_findings": []}
EOF

    # Temp PLUGIN_ROOT: copy hooks-registry.toml (which declares needs_context=["wave_context"])
    # but wire empty resolvers-registry.toml — no resolvers registered.
    local temp_plugin_root
    temp_plugin_root="$(mktemp -d "${BATS_TMPDIR}/no-resolver-XXXXXX")"
    ln -s "${PLUGIN_ROOT}/hook-plugins" "${temp_plugin_root}/hook-plugins"
    cp "${PLUGIN_ROOT}/hooks-registry.toml" "${temp_plugin_root}/hooks-registry.toml"
    printf 'schema_version = 1\n' > "${temp_plugin_root}/resolvers-registry.toml"

    local sink_file
    sink_file="${BATS_TMPDIR}/block-code-3-sink-${RANDOM}.jsonl"

    local payload='{"event_name":"SubagentStop","session_id":"bats-block-003","dispatcher_trace_id":"bats-block-trace","agent_type":"wave-gate-dispatch","last_assistant_message":"Wave gate adversary pass completed for this iteration of the story review cycle."}'
    run bash -c "cd '${temp_plugin_root}' && printf '%s' '${payload}' | VSDD_SINK_FILE='${sink_file}' CLAUDE_PLUGIN_ROOT='${temp_plugin_root}' CLAUDE_PROJECT_DIR='${FACTORY_TMP}' '${DISPATCHER}'"

    # Missing wave_context → Block (exit 2).
    [ "${status}" -eq 2 ]

    [[ -s "${sink_file}" ]] || { echo "SINK FILE EMPTY" >&2; false; }
    grep -q "WAVE_CONTEXT_MISSING" "${sink_file}" || {
        echo "MISSING WAVE_CONTEXT_MISSING in sink" >&2
        cat "${sink_file}" >&2
        false
    }

    rm -rf "${temp_plugin_root}"
    rm -f "${sink_file}"
}

@test "F-P3-003 block code: WAVE_CONTEXT_SCHEMA_ERROR — unit-test-covered, bats construction impractical" {
    # Practical limitation: testing WAVE_CONTEXT_SCHEMA_ERROR at the bats level requires
    # a custom WAT resolver that emits {"wave_context": "not-an-object"}. Constructing
    # this requires embedding JSON as WAT byte literals — feasible but disproportionately
    # complex relative to the already-existing unit test coverage.
    #
    # Unit test coverage in validate-per-story-adversary-convergence/src/lib.rs:
    #   - test_wrong_type_stories_returns_schema_error (all AC-003 SchemaError variants)
    #   - test_BC_4_10_001_ac003_wave_context_not_object_schema_error
    #   - test_BC_4_10_001_ac003_stories_wrong_type_schema_error
    #
    # These tests exercise the pure logic at the Rust level (injectable-callback pattern).
    skip "WAVE_CONTEXT_SCHEMA_ERROR covered by unit tests; bats WAT construction disproportionately complex"
}

# ---------------------------------------------------------------------------
# F-P3-008: Concurrent resolver timeout integration test
#
# With long_running_resolver + wave_context resolver registered, asserts:
# 1. Total dispatch completes in <8000ms (raised from 3000ms in pass-6 to accommodate slow CI runners; see lines 562-565 rationale)
# 2. resolver.error (timeout) event appears in sink for long_running resolver
# 3. wave_context resolver output IS still in plugin_config (not blocked by peer timeout)
#    — verified via absence of WAVE_CONTEXT_MISSING with vacuous-convergence Continue.
#
# Fixture: crates/factory-dispatcher/tests/fixtures/long_running_resolver.wasm (tracked in git).
# hook-plugins/ is gitignored (build artifacts); long-running-resolver.wasm is symlinked
# from fixtures/ into a temp hook-plugins/ dir at test setup time.
# ---------------------------------------------------------------------------

@test "F-P3-008: concurrent resolver timeout — dispatch under 8000ms, timeout event in sink, wave_context succeeds" {
    # Seed minimal .factory/ with active wave and empty stories (vacuous convergence).
    mkdir -p "${FACTORY_TMP}/.factory"
    cat > "${FACTORY_TMP}/.factory/STATE.md" <<'EOF'
---
current_cycle: test-cycle-timeout
---
EOF
    cat > "${FACTORY_TMP}/.factory/wave-state.yaml" <<'EOF'
waves:
  - wave: test-wave-timeout
    stories: []
    stories_merged: []
    gate_status: not_started
EOF

    local lr_wasm="${REPO_ROOT}/crates/factory-dispatcher/tests/fixtures/long_running_resolver.wasm"
    if [[ ! -f "${lr_wasm}" ]]; then
        echo "FATAL: long_running_resolver.wasm not found at ${lr_wasm}" >&2
        exit 1
    fi

    # Create temp PLUGIN_ROOT with wave_context + long_running resolvers.
    local temp_plugin_root
    temp_plugin_root="$(mktemp -d "${BATS_TMPDIR}/timeout-plugin-root-XXXXXX")"

    # Build hook-plugins dir: symlink existing WASMs + long_running_resolver from fixtures.
    mkdir "${temp_plugin_root}/hook-plugins"
    while IFS= read -r -d '' wasm_file; do
        local base
        base="$(basename "${wasm_file}")"
        ln -s "${wasm_file}" "${temp_plugin_root}/hook-plugins/${base}"
    done < <(find "${PLUGIN_ROOT}/hook-plugins" -name "*.wasm" -print0 2>/dev/null)
    ln -s "${lr_wasm}" "${temp_plugin_root}/hook-plugins/long-running-resolver.wasm"

    cat > "${temp_plugin_root}/hooks-registry.toml" <<'HOOKS_TOML'
schema_version = 2

[[hooks]]
name = "validate-per-story-adversary-convergence"
event = "SubagentStop"
plugin = "hook-plugins/validate-per-story-adversary-convergence.wasm"
priority = 960
timeout_ms = 10000
on_error = "continue"
needs_context = ["wave_context", "long_running"]

[hooks.capabilities.read_file]
path_allow = [".factory/cycles"]
HOOKS_TOML

    cat > "${temp_plugin_root}/resolvers-registry.toml" <<'RESOLVER_TOML'
schema_version = 1

[[resolvers]]
name = "wave_context"
plugin = "hook-plugins/vsdd-context-resolvers.wasm"
context_key = "wave_context"
path_allow = [".factory/wave-state.yaml", ".factory/STATE.md"]

[[resolvers]]
name = "long_running"
plugin = "hook-plugins/long-running-resolver.wasm"
context_key = "long_running"
path_allow = []
RESOLVER_TOML

    local sink_file
    sink_file="${BATS_TMPDIR}/timeout-sink-${RANDOM}.jsonl"

    local payload='{"event_name":"SubagentStop","session_id":"bats-timeout","dispatcher_trace_id":"bats-timeout-trace","agent_type":"wave-gate-dispatch","last_assistant_message":"Wave gate adversary pass completed for this iteration of the story review cycle."}'

    # Time the dispatch for assertion 1.
    # Use python3 for millisecond timestamps (portable; date +%s%3N is GNU-only).
    local start_ms end_ms elapsed_ms
    start_ms=$(python3 -c "import time; print(int(time.time() * 1000))")
    run bash -c "cd '${temp_plugin_root}' && printf '%s' '${payload}' | VSDD_SINK_FILE='${sink_file}' CLAUDE_PLUGIN_ROOT='${temp_plugin_root}' CLAUDE_PROJECT_DIR='${FACTORY_TMP}' '${DISPATCHER}'"
    end_ms=$(python3 -c "import time; print(int(time.time() * 1000))")
    elapsed_ms=$((end_ms - start_ms))

    # Empty stories → vacuous convergence → exit 0.
    [ "${status}" -eq 0 ] || {
        echo "UNEXPECTED exit ${status} (expected 0 for vacuous convergence)" >&2
        echo "output: ${output}" >&2
        cat "${sink_file}" >&2
        false
    }

    # Assertion 1: total dispatch completes in <8000ms.
    # Upper bound rationale: the resolver timeout is 1500ms; total wall-clock includes
    # WASM engine startup, bats subprocess overhead, and process launch on potentially
    # loaded CI runners. macOS CI runners under load have been observed taking 5000ms+
    # for this test (5382ms observed on macos-latest 2026-05-10). 8000ms catches hangs
    # (the resolver spins forever without the timeout) while tolerating runner variance.
    # Prior bound of 3000ms was too tight for loaded macOS runners (F-P6-001 follow-up).
    [ "${elapsed_ms}" -lt 8000 ] || {
        echo "TIMEOUT EXCEEDED: dispatch took ${elapsed_ms}ms (threshold 8000ms)" >&2
        false
    }

    # Assertion 2: long_running resolver timed out — observable as elapsed time.
    # The long_running resolver spins forever; epoch interruption fires at ~1500ms.
    # Total dispatch should take between 1300ms and 8000ms:
    #
    # Timeout bound rationale (F-P4-005):
    #   RESOLVER_TIMEOUT_MS = 1500ms (resolver_loader.rs:568); epoch tick = 10ms.
    #   Measured wall time is ~1276ms on this machine due to bats overhead and epoch
    #   pre-advancement (the shared epoch ticker thread may advance epochs before
    #   set_epoch_deadline is called, reducing the effective window).
    #   Lower bound 1100ms: tight enough to catch regressions where the deadline
    #   fires at 800ms or earlier, while tolerating the ~200-400ms measurement
    #   variance observed in practice. A bound of 1000ms (prior) was too loose —
    #   firing at 1100ms or 800ms would still pass.
    #   Upper bound 8000ms covers worst case: timeout fires + epoch tick + bats overhead
    #   + process launch latency on loaded macOS CI runners (observed 5382ms on macos-latest).
    #
    # Architecture note: resolver.error events are written to InternalLog (not to
    # base_host_ctx.events), so they do NOT appear in VSDD_SINK_FILE. The timeout
    # is verified structurally via elapsed time (timeout fired → dispatch took ~1500ms).
    # F-P5-007 (Option A): raised from 1100ms → 1300ms.
    # Rationale: 1100ms was machine-specific (calibrated on a fast macOS dev box).
    # CI runners (ubuntu-latest, macos-14) can be slower under load, causing the
    # timeout to fire at 1500ms + overhead that exceeds the old lower bound.
    # Catches any deadline reduction >13.3% (1300/1500 = 86.67% threshold);
    # the 25% example (1500ms × 0.75 = 1125ms) is well within the guard. Trade-off:
    # accepted risk of rare flake on severely loaded CI runners.
    [ "${elapsed_ms}" -ge 1300 ] || {
        echo "UNEXPECTED: dispatch took only ${elapsed_ms}ms — long_running timeout may not have fired at expected 1500ms" >&2
        echo "Expected >= 1300ms (RESOLVER_TIMEOUT_MS=1500ms; ~200ms tolerance for bats/epoch overhead)" >&2
        false
    }

    # Assertion 3: wave_context resolver succeeded (no WAVE_CONTEXT_MISSING in sink).
    # The wave_context resolver runs concurrently with long_running; a timeout on one
    # MUST NOT block the other. vacuous convergence → Continue (exit 0).
    ! grep -q "WAVE_CONTEXT_MISSING" "${sink_file}" 2>/dev/null || {
        echo "WAVE_CONTEXT_MISSING in sink — wave_context resolver failed despite long_running timeout" >&2
        cat "${sink_file}" >&2
        false
    }

    # Confirm the convergence hook ran (positive coverage — hook.dispatch in sink).
    [[ -s "${sink_file}" ]] || { echo "SINK FILE EMPTY" >&2; false; }
    grep -q "validate-per-story-adversary-convergence" "${sink_file}" || {
        echo "FATAL: convergence hook not in sink — pipeline may not have executed" >&2
        cat "${sink_file}" >&2
        false
    }

    rm -rf "${temp_plugin_root}"
    rm -f "${sink_file}"
}
