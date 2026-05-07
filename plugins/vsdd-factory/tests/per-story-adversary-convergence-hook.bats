#!/usr/bin/env bats
# per-story-adversary-convergence-hook.bats — integration tests for the
# validate-per-story-adversary-convergence WASM hook.
#
# AC-001 through AC-014 traced to BC-4.10.001 and BC-4.10.002.
#
# These tests exercise the built WASM artifact through the hook runner.
# They FAIL until Step 4 (implementation) builds the .wasm artifact.
# The first test checks artifact existence to distinguish "not built yet"
# from "built but wrong behavior."
#
# Pattern:
#   - setup/teardown create a synthetic .factory/ tree with per-story state files
#   - Tests invoke the hook runner with a crafted SubagentStop JSON envelope
#   - Assertions check exit code and output for canonical block_with_fix format

WASM_ARTIFACT="${BATS_TEST_DIRNAME}/../hook-plugins/validate-per-story-adversary-convergence.wasm"
HOOK_RUNNER="${BATS_TEST_DIRNAME}/../hooks/dispatcher/bin/$(uname -s | tr '[:upper:]' '[:lower:]')-$(uname -m | sed 's/x86_64/x64/' | sed 's/aarch64/arm64/')/factory-dispatcher"

setup() {
    WORK=$(mktemp -d)
    # Standard .factory cycle directory layout
    mkdir -p "$WORK/.factory/cycles/v1.0-test-cycle/S-A"
    mkdir -p "$WORK/.factory/cycles/v1.0-test-cycle/S-B"
    CYCLE_ID="v1.0-test-cycle"
}

teardown() {
    rm -rf "$WORK"
}

# ---------------------------------------------------------------------------
# Helper: write a cleared convergence state file for a story
# ---------------------------------------------------------------------------
_write_cleared_state() {
    local story_dir="$1"
    cat > "$story_dir/adversary-convergence-state.json" <<'JSON'
{
  "passes_clean": 3,
  "last_classification": "NITPICK_ONLY",
  "last_finding_count": 0,
  "last_timestamp": "2026-05-06T00:00:00Z",
  "deferred_findings": []
}
JSON
}

# Helper: write a cleared state with deferred findings
_write_cleared_state_with_deferrals() {
    local story_dir="$1"
    cat > "$story_dir/adversary-convergence-state.json" <<'JSON'
{
  "passes_clean": 3,
  "last_classification": "NITPICK_ONLY",
  "last_finding_count": 0,
  "last_timestamp": "2026-05-06T00:00:00Z",
  "deferred_findings": [
    {"id": "D-001", "summary": "cross-story interface concern", "severity": "MEDIUM"}
  ]
}
JSON
}

# Helper: write an insufficient-passes state (passes_clean < 3)
_write_insufficient_passes_state() {
    local story_dir="$1"
    local passes="${2:-1}"
    cat > "$story_dir/adversary-convergence-state.json" <<JSON
{
  "passes_clean": $passes,
  "last_classification": "NITPICK_ONLY",
  "last_finding_count": 1,
  "last_timestamp": "2026-05-06T00:00:00Z",
  "deferred_findings": []
}
JSON
}

# Helper: write a non-NITPICK classification state
_write_high_classification_state() {
    local story_dir="$1"
    cat > "$story_dir/adversary-convergence-state.json" <<'JSON'
{
  "passes_clean": 3,
  "last_classification": "HIGH",
  "last_finding_count": 2,
  "last_timestamp": "2026-05-06T00:00:00Z",
  "deferred_findings": []
}
JSON
}

# Helper: produce a wave-gate SubagentStop envelope
_wave_gate_payload() {
    local stories="${1:-S-A}"
    cat <<JSON
{
  "event_name": "SubagentStop",
  "session_id": "bats-test-session",
  "dispatcher_trace_id": "bats-trace-001",
  "agent_type": "wave-gate-dispatch",
  "plugin_config": {
    "cycle_id": "$CYCLE_ID",
    "wave_stories": $(printf '["%s"]' "${stories}")
  }
}
JSON
}

# ---------------------------------------------------------------------------
# Structural: WASM artifact existence gate
# ---------------------------------------------------------------------------

# AC-013 traces to BC-4.10.001 precondition 1: hooks-registry.toml registration.
# Before the artifact exists, all behavioral tests will fail at the runner level.
@test "structural: WASM artifact exists at canonical path" {
    # AC-013 traces to BC-4.10.001 precondition 1: the artifact
    # plugins/vsdd-factory/hook-plugins/validate-per-story-adversary-convergence.wasm
    # must exist. Fails until Step 4 builds it.
    if [[ ! -f "$WASM_ARTIFACT" ]]; then
        fail "BC-4.10.001 precondition 1: WASM artifact not yet built — \
expected at: $WASM_ARTIFACT — run Step 4 (implementation + cargo build --target wasm32-wasip1)"
    fi
}

# ---------------------------------------------------------------------------
# AC-001 traces to BC-4.10.001 PC1+PC5: cleared wave → no block signal
# ---------------------------------------------------------------------------

# AC-001 traces to BC-4.10.001 PC1+PC5: hook fires on wave-gate event, reads
# state files, returns Continue when all stories are cleared.
@test "integration: cleared wave produces no block signal (AC-001, BC-4.10.001 PC5)" {
    # AC-001: S-A fully cleared → HookResult::Continue
    _write_cleared_state "$WORK/.factory/cycles/$CYCLE_ID/S-A"
    local payload
    payload=$(_wave_gate_payload "S-A")

    run env CLAUDE_PROJECT_DIR="$WORK" \
        sh -c "echo '${payload}' | '${WASM_ARTIFACT}' 2>/dev/null" 2>/dev/null

    if [[ ! -f "$WASM_ARTIFACT" ]]; then
        skip "WASM artifact not yet built (Step 4 pending)"
    fi

    # Exit code 0 = Continue; exit code 2 = Block
    assert_equal "$status" 0 \
        "BC-4.10.001 PC5: cleared wave must produce exit 0 (HookResult::Continue)"

    # Must not contain block_with_fix output in stdout
    refute_output --partial "BLOCKED by validate-per-story-adversary-convergence"
}

# ---------------------------------------------------------------------------
# AC-002 traces to BC-4.10.001 PC2: missing state file → block
# CONVERGENCE_STATE_MISSING
# ---------------------------------------------------------------------------

# AC-002 traces to BC-4.10.001 PC2: missing state file must block.
@test "integration: missing state file produces CONVERGENCE_STATE_MISSING block (AC-002, BC-4.10.001 PC2)" {
    # Do NOT write any state file for S-A — absent file → BLOCK
    local payload
    payload=$(_wave_gate_payload "S-A")

    if [[ ! -f "$WASM_ARTIFACT" ]]; then
        skip "WASM artifact not yet built (Step 4 pending)"
    fi

    run env CLAUDE_PROJECT_DIR="$WORK" \
        sh -c "echo '${payload}' | '${WASM_ARTIFACT}'" 2>/dev/null

    # Exit code 2 = Block
    assert_equal "$status" 2 \
        "BC-4.10.001 PC2: missing state file must produce exit 2 (HookResult::Block)"

    assert_output --partial "CONVERGENCE_STATE_MISSING" \
        "BC-4.10.001 PC2: block output must contain code CONVERGENCE_STATE_MISSING"

    assert_output --partial "BLOCKED by validate-per-story-adversary-convergence" \
        "BC-4.10.001 PC2: block output must use canonical block_with_fix format"
}

# ---------------------------------------------------------------------------
# AC-002 traces to BC-4.10.001 PC3: passes_clean < 3 → block
# CONVERGENCE_PASSES_INSUFFICIENT
# ---------------------------------------------------------------------------

# AC-002 traces to BC-4.10.001 PC3: passes_clean < 3 must block.
@test "integration: passes_clean < 3 produces CONVERGENCE_PASSES_INSUFFICIENT block (AC-002, BC-4.10.001 PC3)" {
    # passes_clean=1 is below threshold of 3
    _write_insufficient_passes_state "$WORK/.factory/cycles/$CYCLE_ID/S-A" 1
    local payload
    payload=$(_wave_gate_payload "S-A")

    if [[ ! -f "$WASM_ARTIFACT" ]]; then
        skip "WASM artifact not yet built (Step 4 pending)"
    fi

    run env CLAUDE_PROJECT_DIR="$WORK" \
        sh -c "echo '${payload}' | '${WASM_ARTIFACT}'" 2>/dev/null

    assert_equal "$status" 2 \
        "BC-4.10.001 PC3: passes_clean < 3 must produce exit 2 (HookResult::Block)"

    assert_output --partial "CONVERGENCE_PASSES_INSUFFICIENT" \
        "BC-4.10.001 PC3: block output must contain code CONVERGENCE_PASSES_INSUFFICIENT"
}

# ---------------------------------------------------------------------------
# AC-002 traces to BC-4.10.001 PC4: non-NITPICK_ONLY → block
# CONVERGENCE_CLASSIFICATION_INSUFFICIENT
# ---------------------------------------------------------------------------

# AC-002 traces to BC-4.10.001 PC4: non-NITPICK_ONLY classification must block.
@test "integration: HIGH classification produces CONVERGENCE_CLASSIFICATION_INSUFFICIENT block (AC-002, BC-4.10.001 PC4)" {
    _write_high_classification_state "$WORK/.factory/cycles/$CYCLE_ID/S-A"
    local payload
    payload=$(_wave_gate_payload "S-A")

    if [[ ! -f "$WASM_ARTIFACT" ]]; then
        skip "WASM artifact not yet built (Step 4 pending)"
    fi

    run env CLAUDE_PROJECT_DIR="$WORK" \
        sh -c "echo '${payload}' | '${WASM_ARTIFACT}'" 2>/dev/null

    assert_equal "$status" 2 \
        "BC-4.10.001 PC4: non-NITPICK_ONLY classification must produce exit 2 (HookResult::Block)"

    assert_output --partial "CONVERGENCE_CLASSIFICATION_INSUFFICIENT" \
        "BC-4.10.001 PC4: block output must contain code CONVERGENCE_CLASSIFICATION_INSUFFICIENT"
}

# ---------------------------------------------------------------------------
# AC-004 / BC-4.10.002 inv-3: absent cycle directory → graceful degrade
# ---------------------------------------------------------------------------

# AC-004 traces to BC-4.10.002 invariant 3: absent .factory/cycles/ directory →
# graceful degrade (exit 0, no block).
@test "integration: absent cycle directory produces graceful degrade Continue (AC-004, BC-4.10.002 inv-3)" {
    # Fresh repo: remove the cycles directory entirely
    rm -rf "$WORK/.factory/cycles"
    local payload
    payload=$(_wave_gate_payload "S-A")

    if [[ ! -f "$WASM_ARTIFACT" ]]; then
        skip "WASM artifact not yet built (Step 4 pending)"
    fi

    run env CLAUDE_PROJECT_DIR="$WORK" \
        sh -c "echo '${payload}' | '${WASM_ARTIFACT}'" 2>/dev/null

    assert_equal "$status" 0 \
        "BC-4.10.002 inv-3: absent cycle directory must produce exit 0 (HookResult::Continue)"

    refute_output --partial "BLOCKED by validate-per-story-adversary-convergence" \
        "BC-4.10.002 inv-3: absent cycle directory must not produce a block signal"
}

# ---------------------------------------------------------------------------
# AC-007 traces to BC-4.10.001 EC-002: malformed JSON → CONVERGENCE_STATE_MALFORMED
# ---------------------------------------------------------------------------

# AC-007 traces to BC-4.10.001 EC-002: malformed JSON in state file must block
# with code CONVERGENCE_STATE_MALFORMED. Must not panic.
@test "integration: malformed JSON state file produces CONVERGENCE_STATE_MALFORMED block (AC-007, BC-4.10.001 EC-002)" {
    # Write malformed JSON to state file
    echo "{ this is not valid json !!!" > \
        "$WORK/.factory/cycles/$CYCLE_ID/S-A/adversary-convergence-state.json"
    local payload
    payload=$(_wave_gate_payload "S-A")

    if [[ ! -f "$WASM_ARTIFACT" ]]; then
        skip "WASM artifact not yet built (Step 4 pending)"
    fi

    run env CLAUDE_PROJECT_DIR="$WORK" \
        sh -c "echo '${payload}' | '${WASM_ARTIFACT}'" 2>/dev/null

    # Must block (exit 2) — not crash (exit 1 would indicate Error)
    assert_equal "$status" 2 \
        "BC-4.10.001 EC-002: malformed JSON must produce exit 2 (HookResult::Block), \
not exit 1 (Error/crash) or exit 0 (Continue)"

    assert_output --partial "CONVERGENCE_STATE_MALFORMED" \
        "BC-4.10.001 EC-002: block output must contain code CONVERGENCE_STATE_MALFORMED"
}

# ---------------------------------------------------------------------------
# AC-002: block output contains canonical block_with_fix fields
# (BLOCKED by ...; Fix: ...; Code: ...)
# ---------------------------------------------------------------------------

# AC-002 traces to BC-4.10.001 PC2+PC8: block output must use canonical
# block_with_fix form (Why/Fix/Code pattern per HOST_ABI.md §WASM hooks).
@test "integration: block output uses canonical block_with_fix format (AC-002, BC-4.10.001 PC8)" {
    # Missing state file → guaranteed block path
    local payload
    payload=$(_wave_gate_payload "S-A")

    if [[ ! -f "$WASM_ARTIFACT" ]]; then
        skip "WASM artifact not yet built (Step 4 pending)"
    fi

    run env CLAUDE_PROJECT_DIR="$WORK" \
        sh -c "echo '${payload}' | '${WASM_ARTIFACT}'" 2>/dev/null

    # Canonical format: "BLOCKED by validate-per-story-adversary-convergence: <reason>. Fix: <fix>. Code: <code>."
    assert_output --partial "BLOCKED by validate-per-story-adversary-convergence:" \
        "BC-4.10.001 PC8: block must start with 'BLOCKED by <hook>:'"

    assert_output --partial "Fix:" \
        "BC-4.10.001 PC8: canonical block_with_fix must include 'Fix:' segment"

    assert_output --partial "Code:" \
        "BC-4.10.001 PC8: canonical block_with_fix must include 'Code:' segment"
}

# ---------------------------------------------------------------------------
# AC-003 / BC-4.10.002 EC-001: per-story SubagentStop (wrong agent) → Continue
# ---------------------------------------------------------------------------

# AC-003 traces to BC-4.10.002 EC-001: hook fired on a non-wave-gate
# SubagentStop must gracefully degrade.
@test "integration: per-story SubagentStop (wrong agent_type) produces graceful degrade (AC-003, BC-4.10.002 EC-001)" {
    _write_cleared_state "$WORK/.factory/cycles/$CYCLE_ID/S-A"

    # Per-story implementer SubagentStop — not a wave-gate dispatch
    local payload
    payload=$(cat <<'JSON'
{
  "event_name": "SubagentStop",
  "session_id": "bats-test-session",
  "dispatcher_trace_id": "bats-trace-002",
  "agent_type": "implementer",
  "plugin_config": {}
}
JSON
)

    if [[ ! -f "$WASM_ARTIFACT" ]]; then
        skip "WASM artifact not yet built (Step 4 pending)"
    fi

    run env CLAUDE_PROJECT_DIR="$WORK" \
        sh -c "echo '${payload}' | '${WASM_ARTIFACT}'" 2>/dev/null

    assert_equal "$status" 0 \
        "BC-4.10.002 EC-001: per-story SubagentStop must produce exit 0 (HookResult::Continue)"

    refute_output --partial "BLOCKED by validate-per-story-adversary-convergence" \
        "BC-4.10.002 EC-001: per-story SubagentStop must not produce a block signal"
}

# ---------------------------------------------------------------------------
# AC-005: deferred_findings present but cleared → Continue (no block)
# ---------------------------------------------------------------------------

# AC-005 traces to BC-4.10.001 EC-004: cleared wave with non-empty
# deferred_findings must return Continue (deferred findings do not block).
@test "integration: cleared wave with deferred_findings returns Continue (AC-005, BC-4.10.001 EC-004)" {
    _write_cleared_state_with_deferrals "$WORK/.factory/cycles/$CYCLE_ID/S-A"
    local payload
    payload=$(_wave_gate_payload "S-A")

    if [[ ! -f "$WASM_ARTIFACT" ]]; then
        skip "WASM artifact not yet built (Step 4 pending)"
    fi

    run env CLAUDE_PROJECT_DIR="$WORK" \
        sh -c "echo '${payload}' | '${WASM_ARTIFACT}'" 2>/dev/null

    assert_equal "$status" 0 \
        "BC-4.10.001 EC-004: cleared story with non-empty deferred_findings must \
produce exit 0 (HookResult::Continue)"

    refute_output --partial "BLOCKED by validate-per-story-adversary-convergence" \
        "BC-4.10.001 EC-004: deferred_findings must not cause a block"
}

# ---------------------------------------------------------------------------
# AC-006 traces to BC-4.10.001 EC-005: multiple failing stories → block on FIRST
# ---------------------------------------------------------------------------

# AC-006 traces to BC-4.10.001 EC-005: when S-A and S-B both fail, hook blocks
# on S-A (first failure), not S-B. Single block message.
@test "integration: multiple failing stories blocks on first failure only (AC-006, BC-4.10.001 EC-005)" {
    # Both S-A and S-B fail (missing state files)
    local payload
    payload=$(_wave_gate_payload "S-A S-B")

    if [[ ! -f "$WASM_ARTIFACT" ]]; then
        skip "WASM artifact not yet built (Step 4 pending)"
    fi

    run env CLAUDE_PROJECT_DIR="$WORK" \
        sh -c "echo '${payload}' | '${WASM_ARTIFACT}'" 2>/dev/null

    assert_equal "$status" 2 \
        "BC-4.10.001 EC-005: multiple failing stories must produce exit 2 (block on first)"

    # The output should mention S-A (first story), not enumerate both failures
    assert_output --partial "S-A" \
        "BC-4.10.001 EC-005: block message must identify the first failing story (S-A)"
}

# ---------------------------------------------------------------------------
# AC-013 traces to BC-4.10.001 precondition 1: hook registered in hooks-registry.toml
# ---------------------------------------------------------------------------

# AC-013 traces to BC-4.10.001 precondition 1: hooks-registry.toml must contain
# an entry for validate-per-story-adversary-convergence.wasm.
@test "structural: hooks-registry.toml contains validate-per-story-adversary-convergence entry (AC-013, BC-4.10.001 PC1)" {
    local registry="${BATS_TEST_DIRNAME}/../hooks-registry.toml"

    if [[ ! -f "$registry" ]]; then
        fail "hooks-registry.toml not found at: $registry"
    fi

    grep -q "validate-per-story-adversary-convergence" "$registry" ||
        fail "BC-4.10.001 precondition 1: hooks-registry.toml must contain an entry for \
validate-per-story-adversary-convergence.wasm (AC-013)"
}
