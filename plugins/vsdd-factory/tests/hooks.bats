#!/usr/bin/env bats
# hooks.bats — TAP tests for vsdd-factory hooks
#
# Asserts allow and block paths for every hook. Uses bats-core. Install:
#   brew install bats-core
#
# Run from plugin root:
#   bats tests/hooks.bats

setup() {
  PLUGIN_ROOT="$(cd "$BATS_TEST_DIRNAME/.." && pwd)"
  HOOKS="$PLUGIN_ROOT/hooks"
  WORK="$(mktemp -d)"
  cd "$WORK"
  mkdir -p .factory/specs/verification-properties
  mkdir -p .factory/specs/behavioral-contracts
  mkdir -p .reference/someproject
}

teardown() {
  rm -rf "$WORK"
}

# ---------- brownfield-discipline ----------

@test "brownfield-discipline: allows edit outside .reference/" {
  run bash -c 'echo "{\"tool_input\":{\"file_path\":\"src/lib.rs\"}}" | "'"$HOOKS"'/brownfield-discipline.sh"'
  [ "$status" -eq 0 ]
}

@test "brownfield-discipline: blocks edit to .reference/" {
  run bash -c 'echo "{\"tool_input\":{\"file_path\":\".reference/foo/bar.md\"}}" | "'"$HOOKS"'/brownfield-discipline.sh"'
  [ "$status" -eq 2 ]
  [[ "$output" == *"read-only"* ]]
}

@test "brownfield-discipline: blocks nested .reference path" {
  run bash -c 'echo "{\"tool_input\":{\"file_path\":\"/abs/path/.reference/x.rs\"}}" | "'"$HOOKS"'/brownfield-discipline.sh"'
  [ "$status" -eq 2 ]
}

@test "brownfield-discipline: allows empty input" {
  run bash -c 'echo "{}" | "'"$HOOKS"'/brownfield-discipline.sh"'
  [ "$status" -eq 0 ]
}

# ---------- protect-vp ----------

@test "protect-vp: allows edit to non-VP file" {
  run bash -c 'echo "{\"tool_input\":{\"file_path\":\"src/lib.rs\"}}" | "'"$HOOKS"'/protect-vp.sh"'
  [ "$status" -eq 0 ]
}

@test "protect-vp: allows edit to new VP file (not yet on disk)" {
  run bash -c 'echo "{\"tool_input\":{\"file_path\":\".factory/specs/verification-properties/VP-999.md\"}}" | "'"$HOOKS"'/protect-vp.sh"'
  [ "$status" -eq 0 ]
}

@test "protect-vp: allows edit to draft VP (no green status)" {
  echo "# VP-001" > .factory/specs/verification-properties/VP-001.md
  echo "Status: draft" >> .factory/specs/verification-properties/VP-001.md
  run bash -c 'echo "{\"tool_input\":{\"file_path\":\".factory/specs/verification-properties/VP-001.md\"}}" | "'"$HOOKS"'/protect-vp.sh"'
  [ "$status" -eq 0 ]
}

@test "protect-vp: blocks edit to green VP" {
  echo "# VP-001" > .factory/specs/verification-properties/VP-001.md
  echo "Status: green" >> .factory/specs/verification-properties/VP-001.md
  run bash -c 'echo "{\"tool_input\":{\"file_path\":\".factory/specs/verification-properties/VP-001.md\"}}" | "'"$HOOKS"'/protect-vp.sh"'
  [ "$status" -eq 0 ]
  [[ "$output" == *'"permissionDecision":"deny"'* ]]
  [[ "$output" == *"immutable"* ]]
  [[ "$output" == *"supersede"* ]]
}

# ---------- protect-bc ----------

@test "protect-bc: allows edit to non-BC file" {
  run bash -c 'echo "{\"tool_input\":{\"file_path\":\"src/lib.rs\"}}" | "'"$HOOKS"'/protect-bc.sh"'
  [ "$status" -eq 0 ]
}

@test "protect-bc: allows edit to draft BC" {
  echo "# BC-1.01.001" > .factory/specs/behavioral-contracts/BC-1.01.001.md
  echo "Status: draft" >> .factory/specs/behavioral-contracts/BC-1.01.001.md
  run bash -c 'echo "{\"tool_input\":{\"file_path\":\".factory/specs/behavioral-contracts/BC-1.01.001.md\"}}" | "'"$HOOKS"'/protect-bc.sh"'
  [ "$status" -eq 0 ]
}

@test "protect-bc: blocks edit to green BC" {
  echo "# BC-1.01.001" > .factory/specs/behavioral-contracts/BC-1.01.001.md
  echo "Status: green" >> .factory/specs/behavioral-contracts/BC-1.01.001.md
  run bash -c 'echo "{\"tool_input\":{\"file_path\":\".factory/specs/behavioral-contracts/BC-1.01.001.md\"}}" | "'"$HOOKS"'/protect-bc.sh"'
  [ "$status" -eq 0 ]
  [[ "$output" == *'"permissionDecision":"deny"'* ]]
  [[ "$output" == *"immutable"* ]]
  [[ "$output" == *"supersede"* ]]
}

# ---------- red-gate ----------

@test "red-gate: allows when state file absent" {
  run bash -c 'echo "{\"tool_input\":{\"file_path\":\"src/lib.rs\"}}" | "'"$HOOKS"'/red-gate.sh"'
  [ "$status" -eq 0 ]
}

@test "red-gate: allows test files unconditionally" {
  echo '{"mode":"strict","red":[]}' > .factory/red-gate-state.json
  run bash -c 'echo "{\"tool_input\":{\"file_path\":\"tests/foo_test.rs\"}}" | "'"$HOOKS"'/red-gate.sh"'
  [ "$status" -eq 0 ]
}

@test "red-gate: allows when mode is off" {
  echo '{"mode":"off","red":[]}' > .factory/red-gate-state.json
  run bash -c 'echo "{\"tool_input\":{\"file_path\":\"src/lib.rs\"}}" | "'"$HOOKS"'/red-gate.sh"'
  [ "$status" -eq 0 ]
}

@test "red-gate: blocks in strict mode when file not in red list" {
  echo '{"mode":"strict","red":[]}' > .factory/red-gate-state.json
  run bash -c 'echo "{\"tool_input\":{\"file_path\":\"src/lib.rs\"}}" | "'"$HOOKS"'/red-gate.sh"'
  [ "$status" -eq 2 ]
  [[ "$output" == *"red list"* ]]
}

@test "red-gate: allows in strict mode when file is in red list" {
  echo '{"mode":"strict","red":["src/lib.rs"]}' > .factory/red-gate-state.json
  run bash -c 'echo "{\"tool_input\":{\"file_path\":\"src/lib.rs\"}}" | "'"$HOOKS"'/red-gate.sh"'
  [ "$status" -eq 0 ]
}

@test "red-gate: ignores unknown extensions" {
  echo '{"mode":"strict","red":[]}' > .factory/red-gate-state.json
  run bash -c 'echo "{\"tool_input\":{\"file_path\":\"README.md\"}}" | "'"$HOOKS"'/red-gate.sh"'
  [ "$status" -eq 0 ]
}

# ---------- red-gate: absolute paths ----------

@test "red-gate: allows absolute path when in red list (PWD-stripped)" {
  echo '{"mode":"strict","red":["src/lib.rs"]}' > .factory/red-gate-state.json
  run bash -c 'echo "{\"tool_input\":{\"file_path\":\"'"$WORK"'/src/lib.rs\"}}" | "'"$HOOKS"'/red-gate.sh"'
  [ "$status" -eq 0 ]
}

@test "red-gate: blocks absolute path when not in red list" {
  echo '{"mode":"strict","red":[]}' > .factory/red-gate-state.json
  run bash -c 'echo "{\"tool_input\":{\"file_path\":\"'"$WORK"'/src/lib.rs\"}}" | "'"$HOOKS"'/red-gate.sh"'
  [ "$status" -eq 2 ]
}

@test "red-gate: allows absolute test path unconditionally" {
  echo '{"mode":"strict","red":[]}' > .factory/red-gate-state.json
  run bash -c 'echo "{\"tool_input\":{\"file_path\":\"'"$WORK"'/tests/foo_test.rs\"}}" | "'"$HOOKS"'/red-gate.sh"'
  [ "$status" -eq 0 ]
}

# ---------- purity-check ----------

@test "purity-check: no-op on non-pure path" {
  mkdir -p src
  echo 'fn main() { println!("hi"); }' > src/main.rs
  run bash -c 'echo "{\"tool_input\":{\"file_path\":\"src/main.rs\"}}" | "'"$HOOKS"'/purity-check.sh"'
  [ "$status" -eq 0 ]
  [ -z "$output" ]
}

@test "purity-check: warns on side effects in pure path" {
  mkdir -p crates/core/src/pure
  echo 'use std::fs;' > crates/core/src/pure/thing.rs
  run bash -c 'echo "{\"tool_input\":{\"file_path\":\"crates/core/src/pure/thing.rs\"}}" | "'"$HOOKS"'/purity-check.sh"'
  [ "$status" -eq 0 ]
  [[ "$output" == *"side-effect"* ]]
}

@test "purity-check: clean pure file emits nothing" {
  mkdir -p src/pure
  echo 'pub fn add(a: i32, b: i32) -> i32 { a + b }' > src/pure/math.rs
  run bash -c 'echo "{\"tool_input\":{\"file_path\":\"src/pure/math.rs\"}}" | "'"$HOOKS"'/purity-check.sh"'
  [ "$status" -eq 0 ]
  [ -z "$output" ]
}

# ---------- handoff-validator ----------

@test "handoff-validator: warns on empty result" {
  run bash -c 'echo "{\"subagent_name\":\"adversary\",\"result\":\"\"}" | "'"$HOOKS"'/handoff-validator.sh"'
  [ "$status" -eq 0 ]
  [[ "$output" == *"empty"* ]]
}

@test "handoff-validator: warns on tiny result" {
  run bash -c 'echo "{\"subagent_name\":\"adversary\",\"result\":\"ok\"}" | "'"$HOOKS"'/handoff-validator.sh"'
  [ "$status" -eq 0 ]
  [[ "$output" == *"Suspiciously short"* ]]
}

@test "handoff-validator: silent on normal result" {
  long="This is a legitimate-looking subagent response with more than forty characters."
  run bash -c 'echo "{\"subagent_name\":\"adversary\",\"result\":\"'"$long"'\"}" | "'"$HOOKS"'/handoff-validator.sh"'
  [ "$status" -eq 0 ]
  [ -z "$output" ]
}

# ---------- regression-gate ----------

@test "regression-gate: ignores non-test commands" {
  run bash -c 'echo "{\"tool_name\":\"Bash\",\"tool_input\":{\"command\":\"ls\"},\"tool_response\":{\"exit_code\":0}}" | "'"$HOOKS"'/regression-gate.sh"'
  [ "$status" -eq 0 ]
}

@test "regression-gate: records passing test" {
  run bash -c 'echo "{\"tool_name\":\"Bash\",\"tool_input\":{\"command\":\"cargo test --workspace\"},\"tool_response\":{\"exit_code\":0}}" | "'"$HOOKS"'/regression-gate.sh"'
  [ "$status" -eq 0 ]
  [ -f .factory/regression-state.json ]
  status_val=$(jq -r .status .factory/regression-state.json)
  [ "$status_val" = "pass" ]
}

@test "regression-gate: warns on pass -> fail transition" {
  echo '{"status":"pass","timestamp":"now","command":"cargo test"}' > .factory/regression-state.json
  run bash -c 'echo "{\"tool_name\":\"Bash\",\"tool_input\":{\"command\":\"cargo test\"},\"tool_response\":{\"exit_code\":1}}" | "'"$HOOKS"'/regression-gate.sh"'
  [ "$status" -eq 0 ]
  [[ "$output" == *"pass → fail"* ]]
}

# ---------- session-learning ----------

@test "session-learning: appends marker when .factory exists" {
  run bash -c 'echo "{}" | "'"$HOOKS"'/session-learning.sh"'
  [ "$status" -eq 0 ]
  [ -f .factory/sidecar-learning.md ]
  grep -q "Session ended" .factory/sidecar-learning.md
}

@test "session-learning: no-op when .factory missing" {
  rm -rf .factory
  run bash -c 'echo "{}" | "'"$HOOKS"'/session-learning.sh"'
  [ "$status" -eq 0 ]
  [ ! -f sidecar-learning.md ]
}

# ---------- verify-git-push ----------

@test "verify-git-push: allows push to feature branch" {
  run bash -c 'echo "{\"tool_input\":{\"command\":\"git push origin feature/STORY-001\"}}" | "'"$HOOKS"'/verify-git-push.sh"'
  [ "$status" -eq 0 ]
}

@test "verify-git-push: allows push to factory-artifacts" {
  run bash -c 'echo "{\"tool_input\":{\"command\":\"git push origin factory-artifacts\"}}" | "'"$HOOKS"'/verify-git-push.sh"'
  [ "$status" -eq 0 ]
}

@test "verify-git-push: allows push with -u (set upstream)" {
  run bash -c 'echo "{\"tool_input\":{\"command\":\"git push -u origin my-branch\"}}" | "'"$HOOKS"'/verify-git-push.sh"'
  [ "$status" -eq 0 ]
}

@test "verify-git-push: blocks push to main" {
  run bash -c 'echo "{\"tool_input\":{\"command\":\"git push origin main\"}}" | "'"$HOOKS"'/verify-git-push.sh" 2>&1'
  [ "$status" -eq 2 ]
  [[ "$output" == *"BLOCKED"* ]]
  [[ "$output" == *"protected branch"* ]]
}

@test "verify-git-push: blocks push to master" {
  run bash -c 'echo "{\"tool_input\":{\"command\":\"git push origin master\"}}" | "'"$HOOKS"'/verify-git-push.sh" 2>&1'
  [ "$status" -eq 2 ]
  [[ "$output" == *"protected branch"* ]]
}

@test "verify-git-push: blocks push to develop" {
  run bash -c 'echo "{\"tool_input\":{\"command\":\"git push origin develop\"}}" | "'"$HOOKS"'/verify-git-push.sh" 2>&1'
  [ "$status" -eq 2 ]
  [[ "$output" == *"protected branch"* ]]
}

@test "verify-git-push: blocks force push" {
  run bash -c 'echo "{\"tool_input\":{\"command\":\"git push --force origin feature/x\"}}" | "'"$HOOKS"'/verify-git-push.sh" 2>&1'
  [ "$status" -eq 2 ]
  [[ "$output" == *"BLOCKED"* ]]
  [[ "$output" == *"Force push"* ]]
}

@test "verify-git-push: blocks force push with -f flag" {
  run bash -c 'echo "{\"tool_input\":{\"command\":\"git push origin feature/x -f\"}}" | "'"$HOOKS"'/verify-git-push.sh" 2>&1'
  [ "$status" -eq 2 ]
}

@test "verify-git-push: allows non-push git commands" {
  run bash -c 'echo "{\"tool_input\":{\"command\":\"git status\"}}" | "'"$HOOKS"'/verify-git-push.sh"'
  [ "$status" -eq 0 ]
}

@test "verify-git-push: block message suggests PR workflow" {
  run bash -c 'echo "{\"tool_input\":{\"command\":\"git push origin main\"}}" | "'"$HOOKS"'/verify-git-push.sh" 2>&1'
  [[ "$output" == *"gh pr create"* ]]
}

@test "verify-git-push: allows --force-with-lease (safe force push)" {
  run bash -c 'echo "{\"tool_input\":{\"command\":\"git push --force-with-lease origin feature/x\"}}" | "'"$HOOKS"'/verify-git-push.sh"'
  [ "$status" -eq 0 ]
}

@test "verify-git-push: blocks -f at end of command" {
  run bash -c 'echo "{\"tool_input\":{\"command\":\"git push origin feature/x -f\"}}" | "'"$HOOKS"'/verify-git-push.sh" 2>&1'
  [ "$status" -eq 2 ]
}

# ---------- check-factory-commit ----------

@test "check-factory-commit: allows non-commit commands" {
  run bash -c 'echo "{\"tool_input\":{\"command\":\"git status\"}}" | "'"$HOOKS"'/check-factory-commit.sh"'
  [ "$status" -eq 0 ]
}

@test "check-factory-commit: allows commit outside .factory" {
  run bash -c 'echo "{\"tool_input\":{\"command\":\"git commit -m fix\"}}" | "'"$HOOKS"'/check-factory-commit.sh"'
  [ "$status" -eq 0 ]
}

@test "check-factory-commit: warns on .factory commit without STATE.md" {
  run bash -c 'echo "{\"tool_input\":{\"command\":\"cd .factory && git commit -m artifacts\"}}" | "'"$HOOKS"'/check-factory-commit.sh"'
  [ "$status" -eq 0 ]
  [[ "$output" == *"STATE.md"* ]]
}

@test "check-factory-commit: silent when STATE.md is in commit" {
  run bash -c 'echo "{\"tool_input\":{\"command\":\"cd .factory && git add STATE.md && git commit -m update\"}}" | "'"$HOOKS"'/check-factory-commit.sh"'
  [ "$status" -eq 0 ]
  [[ "$output" != *"STATE.md was not updated"* ]]
}

# ---------- factory-branch-guard ----------

@test "factory-branch-guard: allows non-.factory paths" {
  run bash -c 'echo "{\"tool_input\":{\"file_path\":\"src/main.rs\"}}" | "'"$HOOKS"'/factory-branch-guard.sh"'
  [ "$status" -eq 0 ]
}

@test "factory-branch-guard: blocks .factory/ write when no worktree" {
  # .factory/ exists as plain dir (created in setup) but no .git marker
  run bash -c 'echo "{\"tool_input\":{\"file_path\":\"'$WORK'/.factory/specs/prd.md\"}}" | "'"$HOOKS"'/factory-branch-guard.sh" 2>&1'
  [ "$status" -eq 2 ]
  [[ "$output" == *"BLOCKED"* ]]
  [[ "$output" == *"not mounted as a git worktree"* ]]
}

@test "factory-branch-guard: block message includes recovery command" {
  run bash -c 'echo "{\"tool_input\":{\"file_path\":\"'$WORK'/.factory/STATE.md\"}}" | "'"$HOOKS"'/factory-branch-guard.sh" 2>&1'
  [[ "$output" == *"git worktree add"* ]]
}

@test "factory-branch-guard: allows .factory/ write when worktree exists" {
  # Simulate worktree by creating .git marker file
  echo "gitdir: /fake/path" > .factory/.git
  run bash -c 'echo "{\"tool_input\":{\"file_path\":\"'$WORK'/.factory/specs/prd.md\"}}" | "'"$HOOKS"'/factory-branch-guard.sh"'
  [ "$status" -eq 0 ]
}

@test "factory-branch-guard: allows empty file path" {
  run bash -c 'echo "{\"tool_input\":{\"file_path\":\"\"}}" | "'"$HOOKS"'/factory-branch-guard.sh"'
  [ "$status" -eq 0 ]
}

@test "factory-branch-guard: allows missing file path" {
  run bash -c 'echo "{\"tool_input\":{}}" | "'"$HOOKS"'/factory-branch-guard.sh"'
  [ "$status" -eq 0 ]
}
