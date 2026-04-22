#!/usr/bin/env bats
# wave-gate-hooks.bats — tests for the wave gate enforcement hooks:
#   validate-wave-gate-prerequisite.sh (PreToolUse on Agent)
#   update-wave-state-on-merge.sh (SubagentStop on pr-manager)
#   warn-pending-wave-gate.sh (Stop hook)

setup() {
  PLUGIN_ROOT="$(cd "$BATS_TEST_DIRNAME/.." && pwd)"
  HOOKS="$PLUGIN_ROOT/hooks"
  WORK=$(mktemp -d)

  # Default wave-state.yaml with 3 waves
  mkdir -p "$WORK/.factory"
}

teardown() {
  rm -rf "$WORK"
}

_write_wave_state() {
  cat > "$WORK/.factory/wave-state.yaml" << EOF
$1
EOF
}

_run_pretool_agent() {
  local subagent="$1"
  local prompt="$2"
  INPUT=$(jq -nc --arg s "$subagent" --arg p "$prompt" '{tool_name: "Agent", tool_input: {subagent_type: $s, prompt: $p}}')
  run bash -c "cd '$WORK' && echo '$INPUT' | '$HOOKS/validate-wave-gate-prerequisite.sh' 2>&1"
}

_run_subagent_stop() {
  local agent="$1"
  local result="$2"
  INPUT=$(jq -nc --arg a "$agent" --arg r "$result" '{agent_type: $a, last_assistant_message: $r}')
  run bash -c "cd '$WORK' && echo '$INPUT' | '$HOOKS/update-wave-state-on-merge.sh' 2>&1"
}

_run_stop_hook() {
  run bash -c "cd '$WORK' && echo '{}' | '$HOOKS/warn-pending-wave-gate.sh' 2>&1"
}

# ========================================================================
# Syntax and wiring checks
# ========================================================================

@test "wave-gate-prerequisite: passes syntax check" {
  run bash -n "$HOOKS/validate-wave-gate-prerequisite.sh"
  [ "$status" -eq 0 ]
}

@test "wave-gate-prerequisite: hook is executable" {
  [ -x "$HOOKS/validate-wave-gate-prerequisite.sh" ]
}

@test "update-wave-state-on-merge: passes syntax check" {
  run bash -n "$HOOKS/update-wave-state-on-merge.sh"
  [ "$status" -eq 0 ]
}

@test "update-wave-state-on-merge: hook is executable" {
  [ -x "$HOOKS/update-wave-state-on-merge.sh" ]
}

@test "warn-pending-wave-gate: passes syntax check" {
  run bash -n "$HOOKS/warn-pending-wave-gate.sh"
  [ "$status" -eq 0 ]
}

@test "warn-pending-wave-gate: hook is executable" {
  [ -x "$HOOKS/warn-pending-wave-gate.sh" ]
}

@test "wave-gate hooks: hooks.json wires prerequisite under PreToolUse Agent" {
  run jq '.hooks.PreToolUse[] | select(.matcher == "Agent") | .hooks[] | select(.command | contains("validate-wave-gate-prerequisite"))' "$PLUGIN_ROOT/hooks/hooks.json"
  [ "$status" -eq 0 ]
  [[ "$output" == *"validate-wave-gate-prerequisite"* ]]
}

@test "wave-gate hooks: hooks.json wires update-on-merge under SubagentStop" {
  run jq '.hooks.SubagentStop[0].hooks[] | select(.command | contains("update-wave-state-on-merge"))' "$PLUGIN_ROOT/hooks/hooks.json"
  [ "$status" -eq 0 ]
  [[ "$output" == *"update-wave-state-on-merge"* ]]
}

@test "wave-gate hooks: hooks.json wires warn-pending under Stop" {
  run jq '.hooks.Stop[0].hooks[] | select(.command | contains("warn-pending-wave-gate"))' "$PLUGIN_ROOT/hooks/hooks.json"
  [ "$status" -eq 0 ]
  [[ "$output" == *"warn-pending-wave-gate"* ]]
}

# ========================================================================
# validate-wave-gate-prerequisite: scope filtering
# ========================================================================

@test "prerequisite: ignores non-Agent tools" {
  INPUT=$(jq -nc '{tool_name: "Edit", tool_input: {file_path: "/tmp/test.md"}}')
  run bash -c "cd '$WORK' && echo '$INPUT' | '$HOOKS/validate-wave-gate-prerequisite.sh' 2>&1"
  [ "$status" -eq 0 ]
}

@test "prerequisite: ignores non-worker subagents" {
  _write_wave_state "current_wave: wave_1
next_gate_required: wave_0a
waves:
  wave_0a:
    stories: [S-0.01]
    stories_merged: [S-0.01]
    gate_status: pending"
  _run_pretool_agent "vsdd-factory:adversary" "cd $WORK && Review specs"
  [ "$status" -eq 0 ]
}

@test "prerequisite: ignores prompts without story ID" {
  _write_wave_state "current_wave: wave_1
next_gate_required: wave_0a
waves:
  wave_0a:
    stories: [S-0.01]
    stories_merged: [S-0.01]
    gate_status: pending"
  _run_pretool_agent "vsdd-factory:implementer" "cd $WORK && Run cargo check"
  [ "$status" -eq 0 ]
}

@test "prerequisite: passes when no wave-state.yaml exists" {
  rm -f "$WORK/.factory/wave-state.yaml"
  _run_pretool_agent "vsdd-factory:implementer" "cd $WORK && Implement S-6.07"
  [ "$status" -eq 0 ]
}

@test "prerequisite: passes when story not in any wave" {
  _write_wave_state "current_wave: wave_1
next_gate_required: null
waves:
  wave_0a:
    stories: [S-0.01]
    stories_merged: [S-0.01]
    gate_status: passed"
  _run_pretool_agent "vsdd-factory:implementer" "cd $WORK && Implement S-99.99"
  [ "$status" -eq 0 ]
}

# ========================================================================
# validate-wave-gate-prerequisite: pass cases
# ========================================================================

@test "prerequisite: passes when all prior gates passed" {
  _write_wave_state "current_wave: wave_1
next_gate_required: null
waves:
  wave_0a:
    stories: [S-0.01]
    stories_merged: [S-0.01]
    gate_status: passed
  wave_0b:
    stories: [S-6.06]
    stories_merged: [S-6.06]
    gate_status: passed
  wave_1:
    stories: [S-6.07]
    stories_merged: []
    gate_status: not_started"
  _run_pretool_agent "vsdd-factory:implementer" "cd $WORK && Implement S-6.07"
  [ "$status" -eq 0 ]
}

@test "prerequisite: passes when prior gate deferred" {
  _write_wave_state "current_wave: wave_1
next_gate_required: null
waves:
  wave_0a:
    stories: [S-0.01]
    stories_merged: [S-0.01]
    gate_status: deferred
    rationale: pure docs wave
  wave_1:
    stories: [S-6.07]
    stories_merged: []
    gate_status: not_started"
  _run_pretool_agent "vsdd-factory:implementer" "cd $WORK && Implement S-6.07"
  [ "$status" -eq 0 ]
}

@test "prerequisite: passes dispatching work for wave_0a (no predecessors)" {
  _write_wave_state "current_wave: wave_0a
next_gate_required: null
waves:
  wave_0a:
    stories: [S-0.01]
    stories_merged: []
    gate_status: not_started"
  _run_pretool_agent "vsdd-factory:test-writer" "cd $WORK && Write tests for S-0.01"
  [ "$status" -eq 0 ]
}

# ========================================================================
# validate-wave-gate-prerequisite: block cases
# ========================================================================

@test "prerequisite: blocks wave_1 dispatch when wave_0a gate pending" {
  _write_wave_state "current_wave: wave_1
next_gate_required: wave_0a
waves:
  wave_0a:
    stories: [S-0.01]
    stories_merged: [S-0.01]
    gate_status: pending
  wave_1:
    stories: [S-6.07]
    stories_merged: []
    gate_status: not_started"
  _run_pretool_agent "vsdd-factory:implementer" "cd $WORK && Implement S-6.07"
  [ "$status" -eq 2 ]
  [[ "$output" == *"BLOCKED"* ]]
  [[ "$output" == *"wave_0a"* ]]
  [[ "$output" == *"pending"* ]]
}

@test "prerequisite: blocks wave_1 dispatch when wave_0a gate not_started" {
  _write_wave_state "current_wave: wave_1
next_gate_required: null
waves:
  wave_0a:
    stories: [S-0.01]
    stories_merged: [S-0.01]
    gate_status: not_started
  wave_1:
    stories: [S-6.07]
    stories_merged: []
    gate_status: not_started"
  _run_pretool_agent "vsdd-factory:test-writer" "cd $WORK && Write tests for S-6.07"
  [ "$status" -eq 2 ]
  [[ "$output" == *"BLOCKED"* ]]
  [[ "$output" == *"wave_0a"* ]]
}

@test "prerequisite: blocks wave_1 dispatch when wave_0b failed" {
  _write_wave_state "current_wave: wave_1
next_gate_required: null
waves:
  wave_0a:
    stories: [S-0.01]
    stories_merged: [S-0.01]
    gate_status: passed
  wave_0b:
    stories: [S-6.06]
    stories_merged: [S-6.06]
    gate_status: failed
  wave_1:
    stories: [S-6.07]
    stories_merged: []
    gate_status: not_started"
  _run_pretool_agent "vsdd-factory:demo-recorder" "cd $WORK && Record demos for S-6.07"
  [ "$status" -eq 2 ]
  [[ "$output" == *"wave_0b"* ]]
  [[ "$output" == *"failed"* ]]
}

@test "prerequisite: error message includes fix instructions" {
  _write_wave_state "current_wave: wave_1
next_gate_required: wave_0a
waves:
  wave_0a:
    stories: [S-0.01]
    stories_merged: [S-0.01]
    gate_status: pending
  wave_1:
    stories: [S-6.07]
    stories_merged: []
    gate_status: not_started"
  _run_pretool_agent "vsdd-factory:pr-manager" "cd $WORK && Run PR process for S-6.07"
  [ "$status" -eq 2 ]
  [[ "$output" == *"wave-gate"* ]]
  [[ "$output" == *"deferred"* ]]
}

@test "prerequisite: blocks pr-manager dispatch" {
  _write_wave_state "current_wave: wave_1
next_gate_required: wave_0a
waves:
  wave_0a:
    stories: [S-0.01]
    stories_merged: [S-0.01]
    gate_status: pending
  wave_1:
    stories: [S-6.07]
    stories_merged: []
    gate_status: not_started"
  _run_pretool_agent "vsdd-factory:pr-manager" "cd $WORK && Full PR process for S-6.07"
  [ "$status" -eq 2 ]
}

@test "prerequisite: blocks devops-engineer dispatch" {
  _write_wave_state "current_wave: wave_1
next_gate_required: wave_0a
waves:
  wave_0a:
    stories: [S-0.01]
    stories_merged: [S-0.01]
    gate_status: pending
  wave_1:
    stories: [S-6.07]
    stories_merged: []
    gate_status: not_started"
  _run_pretool_agent "vsdd-factory:devops-engineer" "cd $WORK && Create worktree for S-6.07"
  [ "$status" -eq 2 ]
}

# ========================================================================
# update-wave-state-on-merge: scope filtering
# ========================================================================

@test "merge-update: ignores non-pr-manager agents" {
  _write_wave_state "current_wave: wave_0a
next_gate_required: null
waves:
  wave_0a:
    stories: [S-0.01]
    stories_merged: []
    gate_status: not_started"
  _run_subagent_stop "implementer" "STEP_COMPLETE: step=8 name=execute-merge status=ok note=merged S-0.01"
  [ "$status" -eq 0 ]
  # wave-state should be unchanged
  run python3 -c "
import yaml
with open('$WORK/.factory/wave-state.yaml') as f: state = yaml.safe_load(f)
print(len(state['waves']['wave_0a']['stories_merged']))
"
  [[ "$output" == "0" ]]
}

@test "merge-update: ignores pr-manager without merge confirmation" {
  _write_wave_state "current_wave: wave_0a
next_gate_required: null
waves:
  wave_0a:
    stories: [S-0.01]
    stories_merged: []
    gate_status: not_started"
  _run_subagent_stop "pr-manager" "STEP_COMPLETE: step=3 name=create-pr status=ok note=PR #42 created for S-0.01"
  [ "$status" -eq 0 ]
  run python3 -c "
import yaml
with open('$WORK/.factory/wave-state.yaml') as f: state = yaml.safe_load(f)
print(len(state['waves']['wave_0a']['stories_merged']))
"
  [[ "$output" == "0" ]]
}

# ========================================================================
# update-wave-state-on-merge: successful updates
# ========================================================================

@test "merge-update: appends merged story to stories_merged" {
  _write_wave_state "current_wave: wave_0a
next_gate_required: null
waves:
  wave_0a:
    stories: [S-0.01, S-0.02]
    stories_merged: []
    gate_status: not_started"
  _run_subagent_stop "pr-manager" "STEP_COMPLETE: step=8 name=execute-merge status=ok note=PR #42 merged
S-0.01 merged to develop."
  [ "$status" -eq 0 ]
  run python3 -c "
import yaml
with open('$WORK/.factory/wave-state.yaml') as f: state = yaml.safe_load(f)
merged = state['waves']['wave_0a']['stories_merged']
print('S-0.01' in merged)
"
  [[ "$output" == "True" ]]
}

@test "merge-update: flips gate_status to pending when all stories merged" {
  _write_wave_state "current_wave: wave_0a
next_gate_required: null
waves:
  wave_0a:
    stories: [S-0.01, S-0.02]
    stories_merged: [S-0.01]
    gate_status: not_started"
  _run_subagent_stop "pr-manager" "STEP_COMPLETE: step=8 name=execute-merge status=ok note=merged
S-0.02 PR squash-merged to develop."
  [ "$status" -eq 0 ]
  run python3 -c "
import yaml
with open('$WORK/.factory/wave-state.yaml') as f: state = yaml.safe_load(f)
print(state['waves']['wave_0a']['gate_status'])
print(state.get('next_gate_required', 'null'))
"
  [[ "${lines[0]}" == "pending" ]]
  [[ "${lines[1]}" == "wave_0a" ]]
}

@test "merge-update: does not duplicate already-merged story" {
  _write_wave_state "current_wave: wave_0a
next_gate_required: null
waves:
  wave_0a:
    stories: [S-0.01]
    stories_merged: [S-0.01]
    gate_status: pending"
  _run_subagent_stop "pr-manager" "STEP_COMPLETE: step=8 name=execute-merge status=ok note=merged S-0.01 again"
  [ "$status" -eq 0 ]
  run python3 -c "
import yaml
with open('$WORK/.factory/wave-state.yaml') as f: state = yaml.safe_load(f)
print(len(state['waves']['wave_0a']['stories_merged']))
"
  [[ "$output" == "1" ]]
}

@test "merge-update: handles no wave-state file" {
  rm -f "$WORK/.factory/wave-state.yaml"
  _run_subagent_stop "pr-manager" "STEP_COMPLETE: step=8 name=execute-merge status=ok note=merged S-0.01"
  [ "$status" -eq 0 ]
}

# ========================================================================
# warn-pending-wave-gate: Stop hook
# ========================================================================

@test "warn-pending: no warning when no wave-state file" {
  rm -f "$WORK/.factory/wave-state.yaml"
  _run_stop_hook
  [ "$status" -eq 0 ]
  [[ "$output" != *"REMINDER"* ]]
}

@test "warn-pending: no warning when all gates passed" {
  _write_wave_state "current_wave: wave_1
next_gate_required: null
waves:
  wave_0a:
    stories: [S-0.01]
    stories_merged: [S-0.01]
    gate_status: passed
  wave_1:
    stories: [S-6.07]
    stories_merged: []
    gate_status: not_started"
  _run_stop_hook
  [ "$status" -eq 0 ]
  [[ "$output" != *"REMINDER"* ]]
}

@test "warn-pending: warns when gate is pending" {
  _write_wave_state "current_wave: wave_1
next_gate_required: wave_0a
waves:
  wave_0a:
    stories: [S-0.01]
    stories_merged: [S-0.01]
    gate_status: pending
  wave_1:
    stories: [S-6.07]
    stories_merged: []
    gate_status: not_started"
  _run_stop_hook
  [ "$status" -eq 0 ]
  [[ "$output" == *"REMINDER"* ]]
  [[ "$output" == *"wave_0a"* ]]
}

@test "warn-pending: warns about multiple pending gates" {
  _write_wave_state "current_wave: wave_1
next_gate_required: wave_0a
waves:
  wave_0a:
    stories: [S-0.01]
    stories_merged: [S-0.01]
    gate_status: pending
  wave_0b:
    stories: [S-6.06]
    stories_merged: [S-6.06]
    gate_status: pending
  wave_1:
    stories: [S-6.07]
    stories_merged: []
    gate_status: not_started"
  _run_stop_hook
  [ "$status" -eq 0 ]
  [[ "$output" == *"wave_0a"* ]]
  [[ "$output" == *"wave_0b"* ]]
}

@test "warn-pending: does not warn for deferred gates" {
  _write_wave_state "current_wave: wave_1
next_gate_required: null
waves:
  wave_0a:
    stories: [S-0.01]
    stories_merged: [S-0.01]
    gate_status: deferred
    rationale: pure docs wave"
  _run_stop_hook
  [ "$status" -eq 0 ]
  [[ "$output" != *"REMINDER"* ]]
}

@test "warn-pending: never blocks (always exit 0)" {
  _write_wave_state "current_wave: wave_1
next_gate_required: wave_0a
waves:
  wave_0a:
    stories: [S-0.01]
    stories_merged: [S-0.01]
    gate_status: pending"
  _run_stop_hook
  [ "$status" -eq 0 ]
}

# ========================================================================
# validate-wave-gate-completeness: PostToolUse on wave-state.yaml
# ========================================================================

@test "gate-completeness: passes syntax check" {
  run bash -n "$HOOKS/validate-wave-gate-completeness.sh"
  [ "$status" -eq 0 ]
}

@test "gate-completeness: hook is executable" {
  [ -x "$HOOKS/validate-wave-gate-completeness.sh" ]
}

@test "gate-completeness: hooks.json wires under PostToolUse" {
  jq -e '.hooks.PostToolUse[] | .hooks[] | select(.command | contains("validate-wave-gate-completeness"))' "$PLUGIN_ROOT/hooks/hooks.json" >/dev/null
}

@test "gate-completeness: passes when gate report has all 6 gates" {
  mkdir -p "$WORK/.factory/cycles/wave-gates"
  cat > "$WORK/.factory/cycles/wave-gates/wave-0a.md" << 'EOF'
# Wave Gate: wave-0a

Gate 1 — Test Suite: PASS (42 tests)
Gate 2 — DTU Validation: SKIP (no critical modules)
Gate 3 — Adversarial Review: PASS (0 critical)
Gate 4 — Demo Evidence: PASS (2 stories)
Gate 5 — Holdout Eval: PASS (mean 0.92)
Gate 6 — State Update: PASS

GATE_CHECK: gate=1 name=test-suite status=pass note=42 tests
GATE_CHECK: gate=2 name=dtu-validation status=skip note=no critical modules
GATE_CHECK: gate=3 name=adversarial-review status=pass note=0 critical
GATE_CHECK: gate=4 name=demo-evidence status=pass note=2 stories
GATE_CHECK: gate=5 name=holdout-eval status=pass note=mean 0.92
GATE_CHECK: gate=6 name=state-update status=pass note=done
EOF
  _write_wave_state "current_wave: wave_1
next_gate_required: null
waves:
  wave_0a:
    stories: [S-0.01]
    stories_merged: [S-0.01]
    gate_status: passed
    gate_report: cycles/wave-gates/wave-0a.md"

  INPUT=$(jq -nc --arg fp "$WORK/.factory/wave-state.yaml" '{tool_input: {file_path: $fp}}')
  run bash -c "cd '$WORK/.factory' && echo '$INPUT' | '$HOOKS/validate-wave-gate-completeness.sh' 2>&1"
  [ "$status" -eq 0 ]
}

@test "gate-completeness: blocks when gate report missing Gate 3" {
  mkdir -p "$WORK/.factory/cycles/wave-gates"
  cat > "$WORK/.factory/cycles/wave-gates/wave-0a.md" << 'EOF'
Gate 1 — Test Suite: PASS
Gate 2 — DTU Validation: SKIP
Gate 4 — Demo Evidence: PASS
Gate 5 — Holdout Eval: PASS
Gate 6 — State Update: PASS
EOF
  _write_wave_state "current_wave: wave_1
next_gate_required: null
waves:
  wave_0a:
    stories: [S-0.01]
    stories_merged: [S-0.01]
    gate_status: passed
    gate_report: cycles/wave-gates/wave-0a.md"

  INPUT=$(jq -nc --arg fp "$WORK/.factory/wave-state.yaml" '{tool_input: {file_path: $fp}}')
  run bash -c "cd '$WORK/.factory' && echo '$INPUT' | '$HOOKS/validate-wave-gate-completeness.sh' 2>&1"
  [ "$status" -eq 2 ]
  [[ "$output" == *"Gate 3"* ]]
  [[ "$output" == *"Adversarial"* ]]
}

@test "gate-completeness: blocks when gate_report path missing" {
  _write_wave_state "current_wave: wave_1
next_gate_required: null
waves:
  wave_0a:
    stories: [S-0.01]
    stories_merged: [S-0.01]
    gate_status: passed"

  INPUT=$(jq -nc --arg fp "$WORK/.factory/wave-state.yaml" '{tool_input: {file_path: $fp}}')
  run bash -c "cd '$WORK/.factory' && echo '$INPUT' | '$HOOKS/validate-wave-gate-completeness.sh' 2>&1"
  [ "$status" -eq 2 ]
  [[ "$output" == *"no gate_report"* ]]
}

@test "gate-completeness: blocks when gate report file not found" {
  _write_wave_state "current_wave: wave_1
next_gate_required: null
waves:
  wave_0a:
    stories: [S-0.01]
    stories_merged: [S-0.01]
    gate_status: passed
    gate_report: cycles/wave-gates/nonexistent.md"

  INPUT=$(jq -nc --arg fp "$WORK/.factory/wave-state.yaml" '{tool_input: {file_path: $fp}}')
  run bash -c "cd '$WORK/.factory' && echo '$INPUT' | '$HOOKS/validate-wave-gate-completeness.sh' 2>&1"
  [ "$status" -eq 2 ]
  [[ "$output" == *"not found"* ]]
}

@test "gate-completeness: ignores non-wave-state files" {
  INPUT=$(jq -nc --arg fp "$WORK/.factory/STATE.md" '{tool_input: {file_path: $fp}}')
  run bash -c "echo '$INPUT' | '$HOOKS/validate-wave-gate-completeness.sh' 2>&1"
  [ "$status" -eq 0 ]
}

@test "gate-completeness: passes with GATE_CHECK telemetry lines only" {
  mkdir -p "$WORK/.factory/cycles/wave-gates"
  cat > "$WORK/.factory/cycles/wave-gates/wave-0a.md" << 'EOF'
GATE_CHECK: gate=1 name=test-suite status=pass note=ok
GATE_CHECK: gate=2 name=dtu status=skip note=na
GATE_CHECK: gate=3 name=adversarial status=pass note=ok
GATE_CHECK: gate=4 name=demo status=pass note=ok
GATE_CHECK: gate=5 name=holdout status=pass note=ok
GATE_CHECK: gate=6 name=state status=pass note=ok
EOF
  _write_wave_state "current_wave: wave_1
next_gate_required: null
waves:
  wave_0a:
    stories: [S-0.01]
    stories_merged: [S-0.01]
    gate_status: passed
    gate_report: cycles/wave-gates/wave-0a.md"

  INPUT=$(jq -nc --arg fp "$WORK/.factory/wave-state.yaml" '{tool_input: {file_path: $fp}}')
  run bash -c "cd '$WORK/.factory' && echo '$INPUT' | '$HOOKS/validate-wave-gate-completeness.sh' 2>&1"
  [ "$status" -eq 0 ]
}

@test "gate-completeness: blocks when only 3 of 6 gates present" {
  mkdir -p "$WORK/.factory/cycles/wave-gates"
  cat > "$WORK/.factory/cycles/wave-gates/wave-0a.md" << 'EOF'
Gate 1 — Test Suite: PASS
Gate 4 — Demo Evidence: PASS
Gate 6 — State Update: PASS
EOF
  _write_wave_state "current_wave: wave_1
next_gate_required: null
waves:
  wave_0a:
    stories: [S-0.01]
    stories_merged: [S-0.01]
    gate_status: passed
    gate_report: cycles/wave-gates/wave-0a.md"

  INPUT=$(jq -nc --arg fp "$WORK/.factory/wave-state.yaml" '{tool_input: {file_path: $fp}}')
  run bash -c "cd '$WORK/.factory' && echo '$INPUT' | '$HOOKS/validate-wave-gate-completeness.sh' 2>&1"
  [ "$status" -eq 2 ]
  [[ "$output" == *"Gate 2"* ]]
  [[ "$output" == *"Gate 3"* ]]
  [[ "$output" == *"Gate 5"* ]]
}

# ========================================================================
# Template existence
# ========================================================================

@test "wave-state template exists" {
  [ -f "$PLUGIN_ROOT/templates/wave-state-template.yaml" ]
}

@test "wave-state template is valid YAML" {
  python3 -c "
import yaml
with open('$PLUGIN_ROOT/templates/wave-state-template.yaml') as f:
    state = yaml.safe_load(f)
assert 'current_wave' in state
assert 'waves' in state
"
}
