#!/usr/bin/env bats
# edit-guards-emission.bats — emission tests for the 5 PreToolUse Edit|Write guards
# instrumented in observability phase 2b (v0.58.0).
#
# For each hook, we verify:
#   1. The block path writes a `hook.block` event with the correct reason code.
#   2. The hook still blocks correctly when emit-event is unreachable
#      (CLAUDE_PLUGIN_ROOT unset OR pointing at a nonexistent path OR
#      VSDD_TELEMETRY=off).
#
# These tests complement the pre-existing behavior tests in hooks.bats and
# hook-robustness.bats, which already cover the block/pass logic.

setup() {
  HOOKS_DIR="${BATS_TEST_DIRNAME}/../hooks"
  PLUGIN_ROOT="${BATS_TEST_DIRNAME}/.."
  EMIT_TMPDIR="$(mktemp -d)"
  # Point emission at scratch log dir; enable telemetry.
  export VSDD_LOG_DIR="$EMIT_TMPDIR"
  unset VSDD_TELEMETRY
}

teardown() {
  rm -rf "$EMIT_TMPDIR"
  unset VSDD_LOG_DIR CLAUDE_PLUGIN_ROOT VSDD_TELEMETRY
}

_logfile() {
  ls "$EMIT_TMPDIR"/events-*.jsonl 2>/dev/null | head -1
}

_run_edit() {
  local hook="$1" file="$2" tool="${3:-Edit}"
  local input
  input=$(jq -nc --arg f "$file" --arg t "$tool" '{tool_name: $t, tool_input: {file_path: $f}}')
  export CLAUDE_PLUGIN_ROOT="$PLUGIN_ROOT"
  run bash -c "echo '$input' | '$HOOKS_DIR/$hook' 2>&1"
}

# ---------- brownfield-discipline.sh ----------

@test "brownfield-discipline: emits reference_readonly on .reference write" {
  _run_edit "brownfield-discipline.sh" ".reference/some-repo/src/foo.ts"
  [ "$status" -eq 2 ]
  local f
  f=$(_logfile)
  [ -n "$f" ]
  [ "$(jq -r '.type' < "$f")" = "hook.block" ]
  [ "$(jq -r '.hook' < "$f")" = "brownfield-discipline" ]
  [ "$(jq -r '.reason' < "$f")" = "reference_readonly" ]
  [ "$(jq -r '.matcher' < "$f")" = "Edit" ]
  [ "$(jq -r '.file_path' < "$f")" = ".reference/some-repo/src/foo.ts" ]
}

@test "brownfield-discipline: still blocks when CLAUDE_PLUGIN_ROOT unset" {
  local input
  input=$(jq -nc --arg f ".reference/x.ts" '{tool_name: "Edit", tool_input: {file_path: $f}}')
  run bash -c "unset CLAUDE_PLUGIN_ROOT; echo '$input' | '$HOOKS_DIR/brownfield-discipline.sh' 2>&1"
  [ "$status" -eq 2 ]
}

@test "brownfield-discipline: still blocks when emit-event path broken" {
  local input
  input=$(jq -nc --arg f ".reference/x.ts" '{tool_name: "Edit", tool_input: {file_path: $f}}')
  run bash -c "CLAUDE_PLUGIN_ROOT='/nonexistent' echo '$input' | '$HOOKS_DIR/brownfield-discipline.sh' 2>&1"
  [ "$status" -eq 2 ]
}

@test "brownfield-discipline: allowed path produces no event" {
  _run_edit "brownfield-discipline.sh" "src/main.ts"
  [ "$status" -eq 0 ]
  [ -z "$(_logfile)" ]
}

# ---------- protect-vp.sh ----------

@test "protect-vp: emits vp_green_immutable on green VP edit" {
  # Create a temp VP file with Status: green in a scratch dir, then point at it.
  local scratch
  scratch=$(mktemp -d)
  mkdir -p "$scratch/.factory/specs/verification-properties"
  local vp="$scratch/.factory/specs/verification-properties/VP-001.md"
  printf "# VP-001\n\nStatus: green\n" > "$vp"
  _run_edit "protect-vp.sh" "$vp"
  [ "$status" -eq 0 ]  # protect-vp exits 0 with JSON envelope, not exit 2
  # Response should contain permissionDecision: deny
  [[ "$output" == *"deny"* ]]
  local f
  f=$(_logfile)
  [ -n "$f" ]
  [ "$(jq -r '.reason' < "$f")" = "vp_green_immutable" ]
  [ "$(jq -r '.hook' < "$f")" = "protect-vp" ]
  rm -rf "$scratch"
}

@test "protect-vp: still denies when CLAUDE_PLUGIN_ROOT unset" {
  local scratch
  scratch=$(mktemp -d)
  mkdir -p "$scratch/.factory/specs/verification-properties"
  local vp="$scratch/.factory/specs/verification-properties/VP-001.md"
  printf "Status: green\n" > "$vp"
  local input
  input=$(jq -nc --arg f "$vp" '{tool_name: "Edit", tool_input: {file_path: $f}}')
  run bash -c "unset CLAUDE_PLUGIN_ROOT; echo '$input' | '$HOOKS_DIR/protect-vp.sh' 2>&1"
  [ "$status" -eq 0 ]
  [[ "$output" == *"deny"* ]]
  rm -rf "$scratch"
}

@test "protect-vp: non-green VP emits no event" {
  local scratch
  scratch=$(mktemp -d)
  mkdir -p "$scratch/.factory/specs/verification-properties"
  local vp="$scratch/.factory/specs/verification-properties/VP-001.md"
  printf "Status: draft\n" > "$vp"
  _run_edit "protect-vp.sh" "$vp"
  [ "$status" -eq 0 ]
  [[ "$output" == *"allow"* ]]
  [ -z "$(_logfile)" ]
  rm -rf "$scratch"
}

# ---------- protect-bc.sh ----------

@test "protect-bc: emits bc_green_immutable on green BC edit" {
  local scratch
  scratch=$(mktemp -d)
  mkdir -p "$scratch/.factory/specs/behavioral-contracts"
  local bc="$scratch/.factory/specs/behavioral-contracts/BC-1.01.001.md"
  printf "Status: green\n" > "$bc"
  _run_edit "protect-bc.sh" "$bc"
  [ "$status" -eq 0 ]
  [[ "$output" == *"deny"* ]]
  local f
  f=$(_logfile)
  [ "$(jq -r '.reason' < "$f")" = "bc_green_immutable" ]
  [ "$(jq -r '.hook' < "$f")" = "protect-bc" ]
  rm -rf "$scratch"
}

@test "protect-bc: still denies when emit-event path broken" {
  local scratch
  scratch=$(mktemp -d)
  mkdir -p "$scratch/.factory/specs/behavioral-contracts"
  local bc="$scratch/.factory/specs/behavioral-contracts/BC-1.01.001.md"
  printf "Status: green\n" > "$bc"
  local input
  input=$(jq -nc --arg f "$bc" '{tool_name: "Edit", tool_input: {file_path: $f}}')
  run bash -c "CLAUDE_PLUGIN_ROOT='/nonexistent' echo '$input' | '$HOOKS_DIR/protect-bc.sh' 2>&1"
  [ "$status" -eq 0 ]
  [[ "$output" == *"deny"* ]]
  rm -rf "$scratch"
}

# ---------- red-gate.sh ----------

@test "red-gate: emits red_gate_strict_violation in strict mode" {
  # red-gate looks for .factory/red-gate-state.json relative to cwd. Run the
  # hook from a scratch cwd so we can set that up.
  local scratch
  scratch=$(mktemp -d)
  mkdir -p "$scratch/.factory"
  printf '{"mode":"strict","red":["other.rs"]}' > "$scratch/.factory/red-gate-state.json"
  local input
  input=$(jq -nc '{tool_name: "Edit", tool_input: {file_path: "src/forbidden.rs"}}')
  export CLAUDE_PLUGIN_ROOT="$PLUGIN_ROOT"
  run bash -c "cd '$scratch' && echo '$input' | '$HOOKS_DIR/red-gate.sh' 2>&1"
  [ "$status" -eq 2 ]
  local f
  f=$(_logfile)
  [ -n "$f" ]
  [ "$(jq -r '.reason' < "$f")" = "red_gate_strict_violation" ]
  [ "$(jq -r '.hook' < "$f")" = "red-gate" ]
  rm -rf "$scratch"
}

@test "red-gate: still blocks in strict mode when CLAUDE_PLUGIN_ROOT unset" {
  local scratch
  scratch=$(mktemp -d)
  mkdir -p "$scratch/.factory"
  printf '{"mode":"strict","red":["other.rs"]}' > "$scratch/.factory/red-gate-state.json"
  local input
  input=$(jq -nc '{tool_name: "Edit", tool_input: {file_path: "src/forbidden.rs"}}')
  run bash -c "unset CLAUDE_PLUGIN_ROOT; cd '$scratch' && echo '$input' | '$HOOKS_DIR/red-gate.sh' 2>&1"
  [ "$status" -eq 2 ]
  rm -rf "$scratch"
}

@test "red-gate: off mode emits no event" {
  local scratch
  scratch=$(mktemp -d)
  mkdir -p "$scratch/.factory"
  printf '{"mode":"off"}' > "$scratch/.factory/red-gate-state.json"
  local input
  input=$(jq -nc '{tool_name: "Edit", tool_input: {file_path: "src/forbidden.rs"}}')
  export CLAUDE_PLUGIN_ROOT="$PLUGIN_ROOT"
  run bash -c "cd '$scratch' && echo '$input' | '$HOOKS_DIR/red-gate.sh' 2>&1"
  [ "$status" -eq 0 ]
  [ -z "$(_logfile)" ]
  rm -rf "$scratch"
}

# ---------- factory-branch-guard.sh ----------

@test "factory-branch-guard: emits factory_not_worktree when .factory/ is plain dir" {
  local scratch
  scratch=$(mktemp -d)
  # Create .factory/ WITHOUT the .git marker — it's a plain dir, not a worktree.
  mkdir -p "$scratch/.factory"
  local input
  input=$(jq -nc --arg f "$scratch/.factory/STATE.md" '{tool_name: "Write", tool_input: {file_path: $f}}')
  export CLAUDE_PLUGIN_ROOT="$PLUGIN_ROOT"
  run bash -c "echo '$input' | '$HOOKS_DIR/factory-branch-guard.sh' 2>&1"
  [ "$status" -eq 2 ]
  local f
  f=$(_logfile)
  [ -n "$f" ]
  [ "$(jq -r '.reason' < "$f")" = "factory_not_worktree" ]
  [ "$(jq -r '.hook' < "$f")" = "factory-branch-guard" ]
  [ "$(jq -r '.matcher' < "$f")" = "Write" ]
  rm -rf "$scratch"
}

@test "factory-branch-guard: still blocks when emit-event path broken" {
  local scratch
  scratch=$(mktemp -d)
  mkdir -p "$scratch/.factory"
  local input
  input=$(jq -nc --arg f "$scratch/.factory/STATE.md" '{tool_name: "Write", tool_input: {file_path: $f}}')
  run bash -c "CLAUDE_PLUGIN_ROOT='/nonexistent' echo '$input' | '$HOOKS_DIR/factory-branch-guard.sh' 2>&1"
  [ "$status" -eq 2 ]
  rm -rf "$scratch"
}

@test "factory-branch-guard: non-.factory path emits no event" {
  _run_edit "factory-branch-guard.sh" "src/main.ts" "Write"
  [ "$status" -eq 0 ]
  [ -z "$(_logfile)" ]
}

# ---------- VSDD_TELEMETRY=off across all five hooks ----------

@test "VSDD_TELEMETRY=off: brownfield-discipline still blocks, no event" {
  local input
  input=$(jq -nc '{tool_name: "Edit", tool_input: {file_path: ".reference/x.ts"}}')
  export CLAUDE_PLUGIN_ROOT="$PLUGIN_ROOT"
  export VSDD_TELEMETRY=off
  run bash -c "echo '$input' | '$HOOKS_DIR/brownfield-discipline.sh' 2>&1"
  [ "$status" -eq 2 ]
  [ -z "$(_logfile)" ]
}

@test "VSDD_TELEMETRY=off: factory-branch-guard still blocks, no event" {
  local scratch
  scratch=$(mktemp -d)
  mkdir -p "$scratch/.factory"
  local input
  input=$(jq -nc --arg f "$scratch/.factory/STATE.md" '{tool_name: "Write", tool_input: {file_path: $f}}')
  export CLAUDE_PLUGIN_ROOT="$PLUGIN_ROOT"
  export VSDD_TELEMETRY=off
  run bash -c "echo '$input' | '$HOOKS_DIR/factory-branch-guard.sh' 2>&1"
  [ "$status" -eq 2 ]
  [ -z "$(_logfile)" ]
  rm -rf "$scratch"
}
