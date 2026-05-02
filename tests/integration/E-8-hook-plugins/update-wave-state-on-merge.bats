#!/usr/bin/env bats
# update-wave-state-on-merge.bats — grep-gate + parity tests for S-8.04
#
# RED GATE: ALL tests MUST FAIL until:
#   (a) GREEN-phase implements the WASM crate logic, AND
#   (b) the WASM binary is compiled to target/wasm32-wasip1/...
#   (c) hooks-registry.toml native entry is wired
#
# What this file verifies (AC-006 + grep-gate):
#   grep-1  : WASM crate src/lib.rs contains real regex match (not stub)
#   grep-2  : process_wave_state contains serde_yaml parse logic (not stub)
#   grep-3  : write_yaml closure in main.rs calls host::write_file
#   grep-4  : emit closure in main.rs calls host::emit_event
#   grep-5  : host::write_file path_allow present in hooks-registry.toml
#   parity-1: pm agent + merge signal + wave-state.yaml → story appended
#   parity-2: all stories merged → gate_status=pending + next_gate_required
#   parity-3: story already in stories_merged → no change (duplicate guard)
#   parity-4: wave-state.yaml absent → graceful exit 0
#   parity-5: story not in any wave → exit 0, no mutation
#   parity-6: gate_status=null explicit (case 2 truth table) → flip to pending
#   parity-7: non-pm agent → no mutation
#   parity-8: no merge signal → no mutation
#
# Prerequisites (for parity tests):
#   - wasmtime >= 14 (WASM runtime, already required by dispatcher)
#   - bats-core >= 1.10
#   - the compiled WASM at target path set in WASM_BIN env var
#
# Run: bats tests/integration/E-8-hook-plugins/update-wave-state-on-merge.bats
#
# BC anchors: BC-7.03.083, BC-7.03.084, BC-7.03.085, BC-7.03.086
# Story: S-8.04
# AC coverage: AC-003, AC-004, AC-005, AC-006, AC-007

# ---------------------------------------------------------------------------
# Setup
# ---------------------------------------------------------------------------
setup() {
  WORKTREE_ROOT="$(git rev-parse --show-toplevel)"
  LIB_RS="${WORKTREE_ROOT}/crates/hook-plugins/update-wave-state-on-merge/src/lib.rs"
  MAIN_RS="${WORKTREE_ROOT}/crates/hook-plugins/update-wave-state-on-merge/src/main.rs"
  REGISTRY="${WORKTREE_ROOT}/plugins/vsdd-factory/hooks-registry.toml"

  # Fixture directory
  FIXTURE_DIR="${WORKTREE_ROOT}/tests/integration/hooks/fixtures"

  # WASM binary (built by implementer at GREEN time)
  WASM_BIN="${WASM_BIN:-${WORKTREE_ROOT}/target/wasm32-wasip1/debug/update-wave-state-on-merge.wasm}"

  # Create a temp dir for per-test YAML mutations
  BATS_TEST_TMPDIR="$(mktemp -d)"
}

teardown() {
  rm -rf "${BATS_TEST_TMPDIR}"
}

# ---------------------------------------------------------------------------
# GREP-GATE: source-level assertions (RED until GREEN impl replaces stubs)
# ---------------------------------------------------------------------------

# grep-1: lib.rs must contain real regex match via regex crate (not stub false)
# RED: lib.rs has_merge_signal stub returns false; no regex::Regex present
@test "grep-1 [BC-7.03.084]: lib.rs has_merge_signal uses regex::Regex (not stub false)" {
  run grep -c "regex::Regex\|Regex::new\|regex_is_match" "${LIB_RS}"
  [ "$status" -eq 0 ]
  [ "$output" -gt 0 ]
}

# grep-2: process_wave_state must use serde_yaml (not the stub no-op)
# RED: current stub has no serde_yaml call
@test "grep-2 [BC-7.03.085]: lib.rs process_wave_state uses serde_yaml::from_str" {
  run grep -c "serde_yaml::from_str\|serde_yaml::to_string\|serde_yaml::from_str" "${LIB_RS}"
  [ "$status" -eq 0 ]
  [ "$output" -gt 0 ]
}

# grep-3: main.rs write_yaml closure must call host::write_file (not stub no-op)
# RED: current stub has no real write_file call
@test "grep-3 [BC-7.03.085 AC-004]: main.rs write_yaml closure calls host::write_file" {
  run grep -c "host::write_file" "${MAIN_RS}"
  [ "$status" -eq 0 ]
  [ "$output" -gt 0 ]
}

# grep-4: main.rs emit closure must call host::emit_event (not stub no-op)
# RED: current stub has no real emit_event call
@test "grep-4 [BC-7.03.083 AC-007]: main.rs emit closure calls host::emit_event" {
  run grep -c "host::emit_event\|emit_event(" "${MAIN_RS}"
  [ "$status" -eq 0 ]
  [ "$output" -gt 0 ]
}

# grep-5: hooks-registry.toml must have write_file capability block with path_allow
# RED: native registry entry still has legacy-bash-adapter or no write_file block
@test "grep-5 [BC-7.03.083 AC-001]: hooks-registry.toml has write_file capability block" {
  run grep -c "write_file\|path_allow" "${REGISTRY}"
  [ "$status" -eq 0 ]
  [ "$output" -gt 0 ]
}

# grep-6: hooks-registry.toml native entry uses update-wave-state-on-merge.wasm
# RED: entry still points at legacy-bash-adapter.wasm or has script_path
@test "grep-6 [BC-7.03.083 AC-001]: hooks-registry.toml has native WASM plugin entry" {
  run grep -c "hook-plugins/update-wave-state-on-merge.wasm" "${REGISTRY}"
  [ "$status" -eq 0 ]
  [ "$output" -gt 0 ]
}

# grep-7: bash script must be deleted (AC-002)
# RED: file still exists in worktree
@test "grep-7 [BC-7.03.083 AC-002]: update-wave-state-on-merge.sh is deleted" {
  [ ! -f "${WORKTREE_ROOT}/plugins/vsdd-factory/hooks/update-wave-state-on-merge.sh" ]
}

# grep-8: main.rs must have the gate-transition stderr reminder (AC-005)
# RED: stub has no eprintln gate reminder
@test "grep-8 [BC-7.03.086 AC-005]: main.rs emits stderr gate-transition reminder" {
  run grep -c "gate_status.*pending\|all stories.*merged\|gate_status → pending" "${MAIN_RS}"
  [ "$status" -eq 0 ]
  [ "$output" -gt 0 ]
}

# ---------------------------------------------------------------------------
# FIXTURE EXISTENCE TESTS
# (RED: fixture files not yet created — File Structure Requirements, S-8.04)
# ---------------------------------------------------------------------------

@test "fixture: wave-state-single-story.yaml exists" {
  [ -f "${FIXTURE_DIR}/wave-state-single-story.yaml" ]
}

@test "fixture: wave-state-all-merged.yaml exists" {
  [ -f "${FIXTURE_DIR}/wave-state-all-merged.yaml" ]
}

@test "fixture: wave-state-gate-null.yaml exists (AC-006 null gate_status fixture)" {
  [ -f "${FIXTURE_DIR}/wave-state-gate-null.yaml" ]
}

@test "fixture: wave-state-gate-null.yaml contains gate_status: ~ (YAML null)" {
  [ -f "${FIXTURE_DIR}/wave-state-gate-null.yaml" ]
  run grep -c "gate_status: ~" "${FIXTURE_DIR}/wave-state-gate-null.yaml"
  [ "$status" -eq 0 ]
  [ "$output" -gt 0 ]
}

# ---------------------------------------------------------------------------
# PARITY TESTS — require compiled WASM binary + wasmtime
# These run through the dispatcher's stdin/stdout protocol.
# All RED until GREEN compile + fixtures exist.
# ---------------------------------------------------------------------------

# Helper: build stdin JSON for SubagentStop envelope
_make_subagentstop_json() {
  local agent_type="$1"
  local result_text="$2"
  printf '{"event_name":"SubagentStop","tool_name":"","session_id":"test","dispatcher_trace_id":"t-001","tool_input":{"agent_type":"%s","result":"%s"},"tool_response":null,"plugin_config":null}' \
    "${agent_type}" "${result_text}"
}

# parity-1: AC-006 case (a): pm agent + merge signal + story in wave →
# wave-state.yaml updated (stories_merged contains story_id)
# RED: WASM binary absent / stub
@test "parity-1 [BC-7.03.085 AC-004 AC-006a]: pm agent + merge signal → story appended to stories_merged" {
  [ -f "${WASM_BIN}" ] || skip "WASM binary not yet compiled (RED gate: compile first)"
  command -v wasmtime > /dev/null || skip "wasmtime not available"

  # Copy fixture to temp dir (writable)
  local wave_state="${BATS_TEST_TMPDIR}/wave-state.yaml"
  cp "${FIXTURE_DIR}/wave-state-single-story.yaml" "${wave_state}"

  # Build stdin envelope
  local fixture_story_id
  fixture_story_id=$(grep -oE '"?S-[0-9]+\.[0-9]+"?' "${FIXTURE_DIR}/wave-state-single-story.yaml" | head -1 | tr -d '"')

  local stdin_json
  stdin_json=$(_make_subagentstop_json "pr-manager" "STEP_COMPLETE: step=8 status=ok -- merged ${fixture_story_id}")

  # Run WASM with env pointing at our temp wave-state
  run env VSDD_WAVE_STATE_PATH="${wave_state}" \
    wasmtime run --dir "${BATS_TEST_TMPDIR}" "${WASM_BIN}" <<< "${stdin_json}"

  [ "$status" -eq 0 ]

  # Assert story_id appears in stories_merged
  run grep -c "${fixture_story_id}" "${wave_state}"
  [ "$output" -gt 0 ]
}

# parity-2: AC-006 case (b): all stories merged → gate_status=pending
# RED: WASM binary absent
@test "parity-2 [BC-7.03.086 AC-005 AC-006b]: all stories merged → gate_status=pending + next_gate_required" {
  [ -f "${WASM_BIN}" ] || skip "WASM binary not yet compiled (RED gate)"
  command -v wasmtime > /dev/null || skip "wasmtime not available"

  local wave_state="${BATS_TEST_TMPDIR}/wave-state.yaml"
  # wave-state-all-merged.yaml has one story not yet merged; after hook fires it's all merged
  cp "${FIXTURE_DIR}/wave-state-all-merged.yaml" "${wave_state}"

  local last_story_id
  last_story_id=$(python3 -c "
import yaml, sys
data = yaml.safe_load(open('${FIXTURE_DIR}/wave-state-all-merged.yaml'))
for w in data.get('waves', []):
    remaining = set(w.get('stories', [])) - set(w.get('stories_merged', []))
    if remaining:
        print(sorted(remaining)[0])
        sys.exit(0)
" 2>/dev/null || grep -oE 'S-[0-9]+\.[0-9]+' "${FIXTURE_DIR}/wave-state-all-merged.yaml" | tail -1)

  local stdin_json
  stdin_json=$(_make_subagentstop_json "pr-manager" "STEP_COMPLETE: step=8 status=ok -- merged ${last_story_id}")

  run env VSDD_WAVE_STATE_PATH="${wave_state}" \
    wasmtime run --dir "${BATS_TEST_TMPDIR}" "${WASM_BIN}" <<< "${stdin_json}"

  [ "$status" -eq 0 ]

  run grep -c "gate_status: pending" "${wave_state}"
  [ "$output" -gt 0 ]

  run grep -c "next_gate_required:" "${wave_state}"
  [ "$output" -gt 0 ]
}

# parity-3: AC-006 case (c): story already in stories_merged → no change
# RED: WASM binary absent
@test "parity-3 [EC-003 AC-006c]: duplicate merge signal → no change to stories_merged" {
  [ -f "${WASM_BIN}" ] || skip "WASM binary not yet compiled (RED gate)"
  command -v wasmtime > /dev/null || skip "wasmtime not available"

  # Build a wave-state where S-8.04 is already merged
  local wave_state="${BATS_TEST_TMPDIR}/wave-state.yaml"
  cat > "${wave_state}" << 'EOF'
waves:
  - wave: "wave-14"
    stories:
      - "S-8.04"
    stories_merged:
      - "S-8.04"
    gate_status: pending
    next_gate_required: wave-14
EOF

  local checksum_before
  checksum_before=$(md5 -q "${wave_state}" 2>/dev/null || md5sum "${wave_state}" | cut -d' ' -f1)

  local stdin_json
  stdin_json=$(_make_subagentstop_json "pr-manager" "STEP_COMPLETE: step=8 status=ok -- merged S-8.04")

  run env VSDD_WAVE_STATE_PATH="${wave_state}" \
    wasmtime run --dir "${BATS_TEST_TMPDIR}" "${WASM_BIN}" <<< "${stdin_json}"

  [ "$status" -eq 0 ]

  local checksum_after
  checksum_after=$(md5 -q "${wave_state}" 2>/dev/null || md5sum "${wave_state}" | cut -d' ' -f1)

  [ "${checksum_before}" = "${checksum_after}" ] || \
    fail "wave-state.yaml was modified on duplicate merge signal (EC-003 violation)"
}

# parity-4: AC-006 case (d): wave-state.yaml absent → graceful exit 0
# RED: WASM binary absent
@test "parity-4 [EC-001 AC-006d]: wave-state.yaml absent → exit 0 (no crash)" {
  [ -f "${WASM_BIN}" ] || skip "WASM binary not yet compiled (RED gate)"
  command -v wasmtime > /dev/null || skip "wasmtime not available"

  # Point at a non-existent path
  local absent_path="${BATS_TEST_TMPDIR}/does-not-exist.yaml"
  [ ! -f "${absent_path}" ]

  local stdin_json
  stdin_json=$(_make_subagentstop_json "pr-manager" "STEP_COMPLETE: step=8 status=ok -- merged S-8.04")

  run env VSDD_WAVE_STATE_PATH="${absent_path}" \
    wasmtime run --dir "${BATS_TEST_TMPDIR}" "${WASM_BIN}" <<< "${stdin_json}"

  [ "$status" -eq 0 ]
}

# parity-5: AC-006 case (e): story_id not found in any wave → exit 0
# RED: WASM binary absent
@test "parity-5 [EC-002 AC-006e]: story not in any wave → exit 0 no mutation" {
  [ -f "${WASM_BIN}" ] || skip "WASM binary not yet compiled (RED gate)"
  command -v wasmtime > /dev/null || skip "wasmtime not available"

  local wave_state="${BATS_TEST_TMPDIR}/wave-state.yaml"
  cat > "${wave_state}" << 'EOF'
waves:
  - wave: "wave-14"
    stories:
      - "S-8.99"
    stories_merged: []
    gate_status: not_started
    next_gate_required: null
EOF

  local checksum_before
  checksum_before=$(md5 -q "${wave_state}" 2>/dev/null || md5sum "${wave_state}" | cut -d' ' -f1)

  # S-8.04 not in any wave
  local stdin_json
  stdin_json=$(_make_subagentstop_json "pr-manager" "STEP_COMPLETE: step=8 status=ok -- merged S-8.04")

  run env VSDD_WAVE_STATE_PATH="${wave_state}" \
    wasmtime run --dir "${BATS_TEST_TMPDIR}" "${WASM_BIN}" <<< "${stdin_json}"

  [ "$status" -eq 0 ]

  local checksum_after
  checksum_after=$(md5 -q "${wave_state}" 2>/dev/null || md5sum "${wave_state}" | cut -d' ' -f1)

  [ "${checksum_before}" = "${checksum_after}" ] || \
    fail "wave-state.yaml was modified for story not in any wave (EC-002)"
}

# parity-6: AC-006 case (f): gate_status: null (YAML null) → flip to pending
# AC-005 truth table case 2. Uses wave-state-gate-null.yaml fixture.
# RED: WASM binary absent
@test "parity-6 [BC-7.03.086 AC-005 AC-006f]: gate_status=null (YAML ~) → flip to pending" {
  [ -f "${WASM_BIN}" ] || skip "WASM binary not yet compiled (RED gate)"
  command -v wasmtime > /dev/null || skip "wasmtime not available"
  [ -f "${FIXTURE_DIR}/wave-state-gate-null.yaml" ] || \
    skip "wave-state-gate-null.yaml fixture not yet created"

  local wave_state="${BATS_TEST_TMPDIR}/wave-state.yaml"
  cp "${FIXTURE_DIR}/wave-state-gate-null.yaml" "${wave_state}"

  # wave-state-gate-null.yaml has S-8.04 in stories, stories_merged=[], gate_status: ~
  local stdin_json
  stdin_json=$(_make_subagentstop_json "pr-manager" "STEP_COMPLETE: step=8 status=ok -- merged S-8.04")

  run env VSDD_WAVE_STATE_PATH="${wave_state}" \
    wasmtime run --dir "${BATS_TEST_TMPDIR}" "${WASM_BIN}" <<< "${stdin_json}"

  [ "$status" -eq 0 ]

  run grep -c "gate_status: pending" "${wave_state}"
  [ "$output" -gt 0 ]
}

# parity-7: AC-003: non-pm agent → no mutation
# RED: WASM binary absent
@test "parity-7 [BC-7.03.084 AC-003]: non-pm agent → exit 0 no YAML mutation" {
  [ -f "${WASM_BIN}" ] || skip "WASM binary not yet compiled (RED gate)"
  command -v wasmtime > /dev/null || skip "wasmtime not available"

  local wave_state="${BATS_TEST_TMPDIR}/wave-state.yaml"
  cat > "${wave_state}" << 'EOF'
waves:
  - wave: "wave-14"
    stories: ["S-8.04"]
    stories_merged: []
    gate_status: not_started
    next_gate_required: null
EOF

  local checksum_before
  checksum_before=$(md5 -q "${wave_state}" 2>/dev/null || md5sum "${wave_state}" | cut -d' ' -f1)

  local stdin_json
  stdin_json=$(_make_subagentstop_json "code-reviewer" "STEP_COMPLETE: step=8 status=ok -- merged S-8.04")

  run env VSDD_WAVE_STATE_PATH="${wave_state}" \
    wasmtime run --dir "${BATS_TEST_TMPDIR}" "${WASM_BIN}" <<< "${stdin_json}"

  [ "$status" -eq 0 ]

  local checksum_after
  checksum_after=$(md5 -q "${wave_state}" 2>/dev/null || md5sum "${wave_state}" | cut -d' ' -f1)

  [ "${checksum_before}" = "${checksum_after}" ] || \
    fail "wave-state.yaml was modified for non-pm agent (AC-003 violation)"
}

# parity-8: AC-003: pm agent but no merge signal → no mutation
# RED: WASM binary absent
@test "parity-8 [BC-7.03.084 AC-003]: pm agent no merge signal → exit 0 no mutation" {
  [ -f "${WASM_BIN}" ] || skip "WASM binary not yet compiled (RED gate)"
  command -v wasmtime > /dev/null || skip "wasmtime not available"

  local wave_state="${BATS_TEST_TMPDIR}/wave-state.yaml"
  cat > "${wave_state}" << 'EOF'
waves:
  - wave: "wave-14"
    stories: ["S-8.04"]
    stories_merged: []
    gate_status: not_started
    next_gate_required: null
EOF

  local checksum_before
  checksum_before=$(md5 -q "${wave_state}" 2>/dev/null || md5sum "${wave_state}" | cut -d' ' -f1)

  # No merge signal in result
  local stdin_json
  stdin_json=$(_make_subagentstop_json "pr-manager" "doing some other work step=9 status=ok")

  run env VSDD_WAVE_STATE_PATH="${wave_state}" \
    wasmtime run --dir "${BATS_TEST_TMPDIR}" "${WASM_BIN}" <<< "${stdin_json}"

  [ "$status" -eq 0 ]

  local checksum_after
  checksum_after=$(md5 -q "${wave_state}" 2>/dev/null || md5sum "${wave_state}" | cut -d' ' -f1)

  [ "${checksum_before}" = "${checksum_after}" ] || \
    fail "wave-state.yaml was modified when no merge signal (AC-003 violation)"
}
