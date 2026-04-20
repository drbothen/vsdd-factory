#!/usr/bin/env bats
# state-health.bats — tests for STATE.md size enforcement hook,
# check-state-health skill, compact-state skill, and state-manager
# content routing rules.

setup() {
  PLUGIN_ROOT="$(cd "$BATS_TEST_DIRNAME/.." && pwd)"
  HOOK="$PLUGIN_ROOT/hooks/validate-state-size.sh"
  WORK=$(mktemp -d)
  mkdir -p "$WORK/.factory"
  # Initialize a git repo so the hook can compare against HEAD
  git -C "$WORK/.factory" init -q
  git -C "$WORK/.factory" checkout -q --orphan factory-artifacts
  echo "# Initial STATE" > "$WORK/.factory/STATE.md"
  git -C "$WORK/.factory" add STATE.md
  git -C "$WORK/.factory" commit -q -m "init"
}

teardown() {
  rm -rf "$WORK"
}

_run_hook() {
  local file="$1"
  INPUT=$(jq -nc --arg fp "$file" '{tool_input: {file_path: $fp}}')
  echo "$INPUT" | bash "$HOOK" 2>&1
}

_run_hook_status() {
  local file="$1"
  INPUT=$(jq -nc --arg fp "$file" '{tool_input: {file_path: $fp}}')
  run bash -c "echo '$INPUT' | '$HOOK' 2>&1"
}

_generate_lines() {
  local count=$1
  local file=$2
  : > "$file"
  for ((i=1; i<=count; i++)); do
    echo "line $i" >> "$file"
  done
}

# ---------- Hook: validate-state-size.sh ----------

@test "state-size: hook passes syntax check" {
  run bash -n "$HOOK"
  [ "$status" -eq 0 ]
}

@test "state-size: ignores non-STATE.md files" {
  _run_hook_status "$WORK/.factory/specs/prd.md"
  [ "$status" -eq 0 ]
  [[ "$output" != *"STATE.md"* ]]
}

@test "state-size: passes for small STATE.md (<200 lines)" {
  _generate_lines 50 "$WORK/.factory/STATE.md"
  git -C "$WORK/.factory" add STATE.md && git -C "$WORK/.factory" commit -q -m "small"
  _generate_lines 100 "$WORK/.factory/STATE.md"
  _run_hook_status "$WORK/.factory/STATE.md"
  [ "$status" -eq 0 ]
  [[ "$output" != *"WARNING"* ]]
}

@test "state-size: warns at 201+ lines" {
  _generate_lines 100 "$WORK/.factory/STATE.md"
  git -C "$WORK/.factory" add STATE.md && git -C "$WORK/.factory" commit -q -m "medium"
  _generate_lines 250 "$WORK/.factory/STATE.md"
  _run_hook_status "$WORK/.factory/STATE.md"
  [ "$status" -eq 0 ]
  [[ "$output" == *"SIZE WARNING"* ]]
  [[ "$output" == *"250 lines"* ]]
}

@test "state-size: blocks at 501+ lines when file grew" {
  _generate_lines 100 "$WORK/.factory/STATE.md"
  git -C "$WORK/.factory" add STATE.md && git -C "$WORK/.factory" commit -q -m "small"
  _generate_lines 550 "$WORK/.factory/STATE.md"
  _run_hook_status "$WORK/.factory/STATE.md"
  [ "$status" -eq 2 ]
  [[ "$output" == *"BLOAT"* ]]
  [[ "$output" == *"compact-state"* ]]
}

@test "state-size: allows compaction even at 501+ lines" {
  # Commit a large file
  _generate_lines 800 "$WORK/.factory/STATE.md"
  git -C "$WORK/.factory" add STATE.md && git -C "$WORK/.factory" commit -q -m "bloated"
  # Now write a smaller version (compaction)
  _generate_lines 600 "$WORK/.factory/STATE.md"
  _run_hook_status "$WORK/.factory/STATE.md"
  # Should pass because line count decreased (600 < 800)
  [ "$status" -eq 0 ]
}

@test "state-size: allows compaction to under 200" {
  _generate_lines 500 "$WORK/.factory/STATE.md"
  git -C "$WORK/.factory" add STATE.md && git -C "$WORK/.factory" commit -q -m "large"
  _generate_lines 150 "$WORK/.factory/STATE.md"
  _run_hook_status "$WORK/.factory/STATE.md"
  [ "$status" -eq 0 ]
  [[ "$output" != *"WARNING"* ]]
}

@test "state-size: blocks growth from 400 to 550" {
  _generate_lines 400 "$WORK/.factory/STATE.md"
  git -C "$WORK/.factory" add STATE.md && git -C "$WORK/.factory" commit -q -m "400"
  _generate_lines 550 "$WORK/.factory/STATE.md"
  _run_hook_status "$WORK/.factory/STATE.md"
  [ "$status" -eq 2 ]
}

@test "state-size: passes for non-existent file" {
  _run_hook_status "$WORK/.factory/DOES-NOT-EXIST.md"
  [ "$status" -eq 0 ]
}

@test "state-size: passes with empty input" {
  run bash -c 'echo "{}" | "'"$HOOK"'" 2>&1'
  [ "$status" -eq 0 ]
}

# ---------- Hook: wired in hooks.json ----------

@test "state-size: hooks.json wires validate-state-size" {
  run grep -c "validate-state-size.sh" "$PLUGIN_ROOT/hooks/hooks.json"
  [ "$status" -eq 0 ]
  [ "$output" -ge 1 ]
}

# ---------- Skills: existence ----------

@test "state-health: check-state-health skill exists" {
  [ -f "$PLUGIN_ROOT/skills/check-state-health/SKILL.md" ]
}

@test "state-health: check-state-health command exists" {
  [ -f "$PLUGIN_ROOT/commands/check-state-health.md" ]
}

@test "state-health: compact-state skill exists" {
  [ -f "$PLUGIN_ROOT/skills/compact-state/SKILL.md" ]
}

@test "state-health: compact-state command exists" {
  [ -f "$PLUGIN_ROOT/commands/compact-state.md" ]
}

# ---------- Skills: content checks ----------

@test "state-health: check-state-health has 7 checks" {
  run grep -c "^### [0-9]\." "$PLUGIN_ROOT/skills/check-state-health/SKILL.md"
  [ "$output" -eq 7 ]
}

@test "state-health: compact-state has 6 steps" {
  run grep -c "^### Step [0-9]" "$PLUGIN_ROOT/skills/compact-state/SKILL.md"
  [ "$output" -eq 6 ]
}

@test "state-health: compact-state references burst-log.md" {
  grep -q "burst-log.md" "$PLUGIN_ROOT/skills/compact-state/SKILL.md"
}

@test "state-health: compact-state references convergence-trajectory.md" {
  grep -q "convergence-trajectory.md" "$PLUGIN_ROOT/skills/compact-state/SKILL.md"
}

@test "state-health: compact-state references session-checkpoints.md" {
  grep -q "session-checkpoints.md" "$PLUGIN_ROOT/skills/compact-state/SKILL.md"
}

@test "state-health: compact-state references lessons.md" {
  grep -q "lessons.md" "$PLUGIN_ROOT/skills/compact-state/SKILL.md"
}

# ---------- State-manager: content routing rules ----------

@test "state-manager: has content routing rules section" {
  grep -q "## Content Routing Rules" "$PLUGIN_ROOT/agents/state-manager.md"
}

@test "state-manager: routes burst narratives to cycle files" {
  grep -q "burst-log.md" "$PLUGIN_ROOT/agents/state-manager.md"
}

@test "state-manager: routes adversary passes to convergence-trajectory" {
  grep -q "convergence-trajectory.md" "$PLUGIN_ROOT/agents/state-manager.md"
}

@test "state-manager: has 200-line limit documented" {
  grep -q "200 lines" "$PLUGIN_ROOT/agents/state-manager.md"
}

@test "state-manager: has anti-patterns section" {
  grep -q "Anti-Patterns" "$PLUGIN_ROOT/agents/state-manager.md"
}

@test "state-manager: forbids burst narratives in STATE.md" {
  grep -q "NEVER.*burst narratives" "$PLUGIN_ROOT/agents/state-manager.md"
}

# ---------- Template: state-template.md ----------

@test "state-template: includes Phase 0" {
  grep -q "0: Codebase Ingestion" "$PLUGIN_ROOT/templates/state-template.md"
}

@test "state-template: includes size budget comment" {
  grep -q "SIZE BUDGET" "$PLUGIN_ROOT/templates/state-template.md"
}

@test "state-template: includes Historical Content section" {
  grep -q "## Historical Content" "$PLUGIN_ROOT/templates/state-template.md"
}

@test "state-template: references compact-state skill" {
  grep -q "compact-state" "$PLUGIN_ROOT/templates/state-template.md"
}

@test "state-template: has Session Resume Checkpoint section" {
  grep -q "## Session Resume Checkpoint" "$PLUGIN_ROOT/templates/state-template.md"
}

@test "state-template: limits current phase steps to 5" {
  grep -q "last 5" "$PLUGIN_ROOT/templates/state-template.md"
}
