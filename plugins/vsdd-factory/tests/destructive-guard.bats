#!/usr/bin/env bats
# destructive-guard.bats — tests for the destructive-command-guard hook
#
# Tests both blocked (exit 2) and allowed (exit 0) commands to verify
# the hook correctly distinguishes dangerous from safe operations.

setup() {
  HOOK="${BATS_TEST_DIRNAME}/../hooks/destructive-command-guard.sh"
}

# Helper: simulate a Bash PreToolUse input
_run_hook() {
  local cmd="$1"
  local input
  input=$(jq -nc --arg c "$cmd" '{tool_input: {command: $c}}')
  run bash -c "echo '$input' | '$HOOK' 2>&1"
}

# ---------- BLOCKED: git reset --hard ----------

@test "blocks git reset --hard" {
  _run_hook "git reset --hard HEAD~1"
  [ "$status" -eq 2 ]
  [[ "$output" == *"BLOCKED"* ]]
  [[ "$output" == *"git reset --hard"* ]]
}

@test "blocks git reset --hard with no ref" {
  _run_hook "git reset --hard"
  [ "$status" -eq 2 ]
}

# ---------- ALLOWED: git reset --soft ----------

@test "allows git reset --soft" {
  _run_hook "git reset --soft HEAD~1"
  [ "$status" -eq 0 ]
}

@test "allows git stash" {
  _run_hook "git stash"
  [ "$status" -eq 0 ]
}

# ---------- BLOCKED: git clean -f ----------

@test "blocks git clean -f" {
  _run_hook "git clean -f"
  [ "$status" -eq 2 ]
  [[ "$output" == *"BLOCKED"* ]]
}

@test "blocks git clean -fd" {
  _run_hook "git clean -fd"
  [ "$status" -eq 2 ]
}

@test "blocks git clean --force" {
  _run_hook "git clean --force"
  [ "$status" -eq 2 ]
}

# ---------- ALLOWED: git clean -n (dry-run) ----------

@test "allows git clean -n (dry-run)" {
  _run_hook "git clean -n"
  [ "$status" -eq 0 ]
}

@test "allows git clean --dry-run" {
  _run_hook "git clean --dry-run"
  [ "$status" -eq 0 ]
}

# ---------- BLOCKED: git checkout -- . ----------

@test "blocks git checkout -- ." {
  _run_hook "git checkout -- ."
  [ "$status" -eq 2 ]
  [[ "$output" == *"BLOCKED"* ]]
}

# ---------- ALLOWED: git checkout -- specific file ----------

@test "allows git checkout -- specific file" {
  _run_hook "git checkout -- src/main.rs"
  [ "$status" -eq 0 ]
}

# ---------- BLOCKED: git restore . ----------

@test "blocks git restore ." {
  _run_hook "git restore ."
  [ "$status" -eq 2 ]
}

# ---------- BLOCKED: rm -rf on protected paths ----------

@test "blocks rm -rf .factory/" {
  _run_hook "rm -rf .factory/"
  [ "$status" -eq 2 ]
  [[ "$output" == *"BLOCKED"* ]]
  [[ "$output" == *"protected path"* ]]
}

@test "blocks rm -rf .factory/specs/" {
  _run_hook "rm -rf .factory/specs/"
  [ "$status" -eq 2 ]
}

@test "blocks rm -rf .factory/stories/" {
  _run_hook "rm -rf .factory/stories/"
  [ "$status" -eq 2 ]
}

@test "blocks rm -rf src/" {
  _run_hook "rm -rf src/"
  [ "$status" -eq 2 ]
}

@test "blocks rm -rf tests/" {
  _run_hook "rm -rf tests/"
  [ "$status" -eq 2 ]
}

@test "blocks rm -r .factory/specs/" {
  _run_hook "rm -r .factory/specs/"
  [ "$status" -eq 2 ]
}

@test "blocks rm -Rf .factory/" {
  _run_hook "rm -Rf .factory/"
  [ "$status" -eq 2 ]
}

@test "blocks rm -fr .factory/" {
  _run_hook "rm -fr .factory/"
  [ "$status" -eq 2 ]
}

# ---------- ALLOWED: rm -rf on build directories ----------

@test "allows rm -rf target/" {
  _run_hook "rm -rf target/"
  [ "$status" -eq 0 ]
}

@test "allows rm -rf node_modules/" {
  _run_hook "rm -rf node_modules/"
  [ "$status" -eq 0 ]
}

@test "allows rm -rf dist/" {
  _run_hook "rm -rf dist/"
  [ "$status" -eq 0 ]
}

# ---------- ALLOWED: worktree cleanup ----------

@test "allows rm -rf .worktrees/STORY-001/" {
  _run_hook "rm -rf .worktrees/STORY-001/"
  [ "$status" -eq 0 ]
}

# ---------- BLOCKED: rm on source-of-truth files ----------

@test "blocks rm STATE.md" {
  _run_hook "rm .factory/STATE.md"
  [ "$status" -eq 2 ]
  [[ "$output" == *"source-of-truth"* ]]
}

@test "blocks rm BC-INDEX.md" {
  _run_hook "rm .factory/specs/behavioral-contracts/BC-INDEX.md"
  [ "$status" -eq 2 ]
}

@test "blocks rm VP-INDEX.md" {
  _run_hook "rm .factory/specs/verification-properties/VP-INDEX.md"
  [ "$status" -eq 2 ]
}

@test "blocks rm STORY-INDEX.md" {
  _run_hook "rm .factory/stories/STORY-INDEX.md"
  [ "$status" -eq 2 ]
}

@test "blocks rm prd.md" {
  _run_hook "rm .factory/specs/prd.md"
  [ "$status" -eq 2 ]
}

# ---------- ALLOWED: rm on non-critical files ----------

@test "allows rm of temp file" {
  _run_hook "rm /tmp/scratch.txt"
  [ "$status" -eq 0 ]
}

@test "allows rm of build artifact" {
  _run_hook "rm target/debug/myapp"
  [ "$status" -eq 0 ]
}

# ---------- BLOCKED: git rm on protected paths ----------

@test "blocks git rm -r .factory/specs/" {
  _run_hook "git rm -r .factory/specs/"
  [ "$status" -eq 2 ]
  [[ "$output" == *"BLOCKED"* ]]
}

@test "blocks git rm .factory/STATE.md" {
  _run_hook "git rm .factory/STATE.md"
  [ "$status" -eq 2 ]
}

@test "blocks git rm -r .factory/stories/" {
  _run_hook "git rm -r .factory/stories/"
  [ "$status" -eq 2 ]
}

# ---------- ALLOWED: non-destructive commands ----------

@test "allows cargo test" {
  _run_hook "cargo test"
  [ "$status" -eq 0 ]
}

@test "allows git status" {
  _run_hook "git status"
  [ "$status" -eq 0 ]
}

@test "allows git add" {
  _run_hook "git add src/main.rs"
  [ "$status" -eq 0 ]
}

@test "allows git commit" {
  _run_hook "git commit -m 'test'"
  [ "$status" -eq 0 ]
}

@test "allows ls" {
  _run_hook "ls -la .factory/"
  [ "$status" -eq 0 ]
}

@test "allows cat" {
  _run_hook "cat .factory/STATE.md"
  [ "$status" -eq 0 ]
}

# ---------- Structural ----------

@test "hook file exists and is executable" {
  [ -x "$HOOK" ]
}

@test "hook passes syntax check" {
  bash -n "$HOOK"
}

@test "hooks.json wires destructive-command-guard" {
  jq -e '.hooks.PreToolUse[1].hooks[] | select(.command | contains("destructive-command-guard"))' "${BATS_TEST_DIRNAME}/../hooks/hooks.json" >/dev/null
}

@test "hook provides fix suggestions in block messages" {
  _run_hook "git reset --hard HEAD"
  [[ "$output" == *"Suggestion:"* ]]
}

# ---------- Edge cases ----------

@test "allows empty command" {
  local input='{"tool_input":{"command":""}}'
  run bash -c "echo '$input' | '$HOOK' 2>&1"
  [ "$status" -eq 0 ]
}

@test "allows missing command field" {
  local input='{"tool_input":{}}'
  run bash -c "echo '$input' | '$HOOK' 2>&1"
  [ "$status" -eq 0 ]
}
