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
# Step 2 (test-writer): test bodies are documented with full seeding logic and
# skip directives. Step 3 (implementer) removes the skip lines and fills in
# the implementation to make these tests GREEN.
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
    export FACTORY_ROOT="${FACTORY_TMP}"

    # Determine the dispatcher binary path.
    # In CI and local cargo builds the binary lives in target/debug/ or target/release/.
    # CARGO_MANIFEST_DIR is not set in bats; resolve from __file__.
    REPO_ROOT="$(cd "$(dirname "${BATS_TEST_FILENAME}")/../../../.." && pwd)"
    DISPATCHER="${REPO_ROOT}/target/debug/factory-dispatcher"
    if [[ ! -x "${DISPATCHER}" ]]; then
        DISPATCHER="${REPO_ROOT}/target/release/factory-dispatcher"
    fi
    export DISPATCHER
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

    # .factory/STATE.md — current_cycle pointer
    mkdir -p "${FACTORY_TMP}/.factory"
    cat > "${FACTORY_TMP}/.factory/STATE.md" <<'EOF'
---
current_cycle: test-cycle-001
---
EOF

    # .factory/wave-state.yaml — active wave with two stories
    cat > "${FACTORY_TMP}/.factory/wave-state.yaml" <<'EOF'
wave_id: test-wave-001
status: active
stories:
  - S-FAKE-001
  - S-FAKE-002
cycle_id: test-cycle-001
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

# Build a synthetic SubagentStop event payload for the dispatcher.
# The agent_type "wave-gate-dispatch" is the canonical wave-gate identity
# (BC-4.10.002 invariant 4 / graceful_degrade_outside_wave_gate).
make_subagentstop_payload() {
    cat <<'EOF'
{
  "hook_event_name": "SubagentStop",
  "event_name": "SubagentStop",
  "session_id": "bats-test-session",
  "dispatcher_trace_id": "bats-test-trace",
  "agent_type": "wave-gate-dispatch"
}
EOF
}

# ---------------------------------------------------------------------------
# AC-008: F-P2-001 closure — unconverged story → dispatcher exits 2 (Block)
# ---------------------------------------------------------------------------

@test "F-P2-001 closure: unconverged story → dispatcher exits 2 (Block)" {
    skip "TODO Step 3 implementer: remove this skip line and fill in the
implementation after WaveContextResolver (S-12.07) is wired and
extract_stories_from_wave_context is implemented.

Step 3 implementation checklist:
1. Remove the 'skip' directive above.
2. Ensure 'cargo build -p factory-dispatcher' produces \${DISPATCHER}.
3. Verify that WaveContextResolver reads wave-state.yaml from FACTORY_ROOT
   and injects wave_context.stories into plugin_config before dispatching
   the convergence hook.
4. Run: bats plugins/vsdd-factory/tests/resolver-integration.bats
5. Assert: exit code 2 (Block) and stderr contains CONVERGENCE_PASSES_INSUFFICIENT
   and mentions S-FAKE-001.

The seed_factory_root helper (above) and the assertion block (below) are
complete — Step 3 only needs to remove this skip and ensure the binary exists."

    # Seed: S-FAKE-001 unconverged (passes_clean=1), S-FAKE-002 converged (passes_clean=3)
    seed_factory_root 1 3

    # Assert dispatcher binary exists (build it if missing)
    if [[ ! -x "${DISPATCHER}" ]]; then
        REPO_ROOT="$(cd "$(dirname "${BATS_TEST_FILENAME}")/../../../.." && pwd)"
        cargo build -p factory-dispatcher --manifest-path "${REPO_ROOT}/Cargo.toml" 2>&1
        DISPATCHER="${REPO_ROOT}/target/debug/factory-dispatcher"
    fi
    [[ -x "${DISPATCHER}" ]] || skip "factory-dispatcher binary not found at ${DISPATCHER}"

    # Run dispatcher with synthetic SubagentStop event via stdin
    run bash -c "make_subagentstop_payload | FACTORY_ROOT='${FACTORY_ROOT}' '${DISPATCHER}'"

    # AC-008: exit code 2 = Block (dispatcher convention: 0=Continue, 2=Block)
    [ "${status}" -eq 2 ]

    # Block reason must reference the CONVERGENCE_PASSES_INSUFFICIENT code
    # (S-FAKE-001 has passes_clean=1 < 3).
    [[ "${output}" == *"CONVERGENCE_PASSES_INSUFFICIENT"* ]]

    # Block reason must identify the failing story
    [[ "${output}" == *"S-FAKE-001"* ]]
}

# ---------------------------------------------------------------------------
# AC-009: F-P2-001 closure — all converged → dispatcher exits 0 (Continue)
# ---------------------------------------------------------------------------

@test "F-P2-001 closure: all converged → dispatcher exits 0 (Continue)" {
    skip "TODO Step 3 implementer: remove this skip line and fill in the
implementation after WaveContextResolver (S-12.07) is wired and
extract_stories_from_wave_context is implemented.

Step 3 implementation checklist:
1. Remove the 'skip' directive above.
2. Ensure 'cargo build -p factory-dispatcher' produces \${DISPATCHER}.
3. Verify that WaveContextResolver reads wave-state.yaml from FACTORY_ROOT
   and injects wave_context.stories into plugin_config before dispatching
   the convergence hook.
4. Run: bats plugins/vsdd-factory/tests/resolver-integration.bats
5. Assert: exit code 0 (Continue) and no block output.

The seed_factory_root helper and assertion block are complete."

    # Seed: ALL stories converged (passes_clean=3)
    seed_factory_root 3 3

    # Assert dispatcher binary exists
    if [[ ! -x "${DISPATCHER}" ]]; then
        REPO_ROOT="$(cd "$(dirname "${BATS_TEST_FILENAME}")/../../../.." && pwd)"
        cargo build -p factory-dispatcher --manifest-path "${REPO_ROOT}/Cargo.toml" 2>&1
        DISPATCHER="${REPO_ROOT}/target/debug/factory-dispatcher"
    fi
    [[ -x "${DISPATCHER}" ]] || skip "factory-dispatcher binary not found at ${DISPATCHER}"

    # Run dispatcher with synthetic SubagentStop event via stdin
    run bash -c "make_subagentstop_payload | FACTORY_ROOT='${FACTORY_ROOT}' '${DISPATCHER}'"

    # AC-009: exit code 0 = Continue (all stories converged)
    [ "${status}" -eq 0 ]

    # No block output
    [[ "${output}" != *"BLOCK"* ]]
    [[ "${output}" != *"WAVE_CONTEXT"* ]]
}
