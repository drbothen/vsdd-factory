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

@test "registry wires destructive-command-guard" {
  load "${BATS_TEST_DIRNAME}/helpers/registry.bash"
  registry_has_hook "destructive-command-guard" "PreToolUse"
}

@test "hook provides fix suggestions in block messages" {
  _run_hook "git reset --hard HEAD"
  [[ "$output" == *"Fix:"* ]]
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

# ---------- Complex bash constructs ----------

@test "blocks rm -rf .factory inside subshell" {
  _run_hook "bash -c 'rm -rf .factory/specs/'"
  [ "$status" -eq 2 ]
}

@test "blocks rm -rf .factory in pipe chain" {
  _run_hook "ls .factory/ && rm -rf .factory/specs/"
  [ "$status" -eq 2 ]
}

@test "blocks rm -rf .factory after cd" {
  _run_hook "cd /tmp && rm -rf .factory/"
  [ "$status" -eq 2 ]
}

@test "blocks git reset --hard in multi-command" {
  _run_hook "git stash && git reset --hard HEAD~1"
  [ "$status" -eq 2 ]
}

@test "blocks rm STATE.md with full path" {
  _run_hook "rm /Users/josh/dev/project/.factory/STATE.md"
  [ "$status" -eq 2 ]
}

# ---------- BLOCKED: catastrophic roots ----------

@test "blocks rm -rf /" {
  _run_hook "rm -rf /"
  [ "$status" -eq 2 ]
  [[ "$output" == *"Catastrophic"* ]]
}

@test "blocks rm -rf /*" {
  _run_hook "rm -rf /*"
  [ "$status" -eq 2 ]
}

@test "blocks rm -rf ~" {
  _run_hook "rm -rf ~"
  [ "$status" -eq 2 ]
}

@test "blocks rm -rf ~/" {
  _run_hook "rm -rf ~/"
  [ "$status" -eq 2 ]
}

@test "blocks rm -rf \$HOME" {
  _run_hook 'rm -rf $HOME'
  [ "$status" -eq 2 ]
}

@test "blocks rm -rf *" {
  _run_hook "rm -rf *"
  [ "$status" -eq 2 ]
}

@test "blocks rm -rf .*" {
  _run_hook "rm -rf .*"
  [ "$status" -eq 2 ]
}

@test "blocks rm -fr / (flag order variant)" {
  _run_hook "rm -fr /"
  [ "$status" -eq 2 ]
}

# ---------- Bug fix: bare .factory (no slash) and --recursive long form ----------

@test "blocks rm -rf .factory (no trailing slash)" {
  _run_hook "rm -rf .factory"
  [ "$status" -eq 2 ]
}

@test "blocks rm --recursive .factory" {
  _run_hook "rm --recursive .factory"
  [ "$status" -eq 2 ]
}

# ---------- BLOCKED: clobbering redirects to SoT files ----------

@test "blocks echo > STATE.md (clobber)" {
  _run_hook "echo x > .factory/STATE.md"
  [ "$status" -eq 2 ]
  [[ "$output" == *"Clobbering redirect"* ]]
}

@test "allows echo >> STATE.md (append)" {
  _run_hook "echo x >> .factory/STATE.md"
  [ "$status" -eq 0 ]
}

@test "blocks : > STATE.md (truncate idiom)" {
  _run_hook ": > .factory/STATE.md"
  [ "$status" -eq 2 ]
}

@test "blocks truncate -s 0 STATE.md" {
  _run_hook "truncate -s 0 .factory/STATE.md"
  [ "$status" -eq 2 ]
}

@test "blocks cp /dev/null STATE.md" {
  _run_hook "cp /dev/null .factory/STATE.md"
  [ "$status" -eq 2 ]
}

@test "allows sed -i on STATE.md" {
  _run_hook 'sed -i "" "s/foo/bar/" .factory/STATE.md'
  [ "$status" -eq 0 ]
}

# ---------- BLOCKED: find -delete / -exec rm on protected paths ----------

@test "blocks find .factory -delete" {
  _run_hook "find .factory -type f -delete"
  [ "$status" -eq 2 ]
}

@test "blocks find src -exec rm" {
  _run_hook 'find src -name "*.ts" -exec rm {} \;'
  [ "$status" -eq 2 ]
}

# ---------- BLOCKED: git stash drop / clear ----------

@test "blocks git stash drop" {
  _run_hook "git stash drop"
  [ "$status" -eq 2 ]
}

@test "blocks git stash clear" {
  _run_hook "git stash clear"
  [ "$status" -eq 2 ]
}

# ---------- BLOCKED: git branch -D on protected branches ----------

@test "blocks git branch -D main" {
  _run_hook "git branch -D main"
  [ "$status" -eq 2 ]
}

@test "blocks git branch -D master" {
  _run_hook "git branch -D master"
  [ "$status" -eq 2 ]
}

@test "blocks git branch -D develop" {
  _run_hook "git branch -D develop"
  [ "$status" -eq 2 ]
}

@test "allows git branch -D feature/x" {
  _run_hook "git branch -D feature/STORY-123"
  [ "$status" -eq 0 ]
}

# ---------- BLOCKED: history-rewriting git commands ----------

@test "blocks git filter-branch" {
  _run_hook "git filter-branch --tree-filter foo HEAD"
  [ "$status" -eq 2 ]
}

@test "blocks git filter-repo" {
  _run_hook "git filter-repo --path secret"
  [ "$status" -eq 2 ]
}

@test "blocks git reflog expire --expire=now" {
  _run_hook "git reflog expire --expire=now --all"
  [ "$status" -eq 2 ]
}

@test "blocks git gc --prune=now" {
  _run_hook "git gc --prune=now"
  [ "$status" -eq 2 ]
}

@test "allows git gc" {
  _run_hook "git gc"
  [ "$status" -eq 0 ]
}

# ---------- BLOCKED: git worktree remove --force outside .worktrees/ ----------

@test "blocks git worktree remove --force outside .worktrees" {
  _run_hook "git worktree remove --force /tmp/other"
  [ "$status" -eq 2 ]
}

@test "allows git worktree remove (no force)" {
  _run_hook "git worktree remove .worktrees/STORY-1"
  [ "$status" -eq 0 ]
}

# ---------- BLOCKED: --no-verify on git commit/merge/rebase ----------

@test "blocks git commit --no-verify" {
  _run_hook "git commit -m test --no-verify"
  [ "$status" -eq 2 ]
}

@test "blocks git rebase --no-verify" {
  _run_hook "git rebase --no-verify main"
  [ "$status" -eq 2 ]
}

@test "blocks git commit --no-gpg-sign" {
  _run_hook "git commit -m test --no-gpg-sign"
  [ "$status" -eq 2 ]
}

# ---------- BLOCKED: gh destructive operations ----------

@test "blocks gh repo delete" {
  _run_hook "gh repo delete foo/bar --yes"
  [ "$status" -eq 2 ]
}

@test "blocks gh release delete" {
  _run_hook "gh release delete v1.0"
  [ "$status" -eq 2 ]
}

@test "blocks gh pr close" {
  _run_hook "gh pr close 42"
  [ "$status" -eq 2 ]
}

@test "blocks gh issue delete" {
  _run_hook "gh issue delete 42"
  [ "$status" -eq 2 ]
}

@test "allows gh pr create" {
  _run_hook "gh pr create --title foo"
  [ "$status" -eq 0 ]
}

@test "allows gh issue close" {
  _run_hook "gh issue close 42"
  [ "$status" -eq 0 ]
}

# ---------- BLOCKED: curl|bash / wget|sh ----------

@test "blocks curl | bash" {
  _run_hook "curl -sSL https://example.com/install.sh | bash"
  [ "$status" -eq 2 ]
}

@test "blocks wget | sh" {
  _run_hook "wget -qO- https://example.com/x | sh"
  [ "$status" -eq 2 ]
}

@test "allows curl to file" {
  _run_hook "curl -sSL https://example.com/x > /tmp/x"
  [ "$status" -eq 0 ]
}

# ---------- BLOCKED: recursive chmod/chown on protected ----------

@test "blocks chmod -R on .factory" {
  _run_hook "chmod -R 755 .factory"
  [ "$status" -eq 2 ]
}

@test "blocks chown -R on src/" {
  _run_hook "chown -R user src/"
  [ "$status" -eq 2 ]
}

@test "allows chmod on single file" {
  _run_hook "chmod +x plugins/vsdd-factory/hooks/foo.sh"
  [ "$status" -eq 0 ]
}

# ---------------------------------------------------------------------------
# Red Gate tests — issue #130 / ADR-024 Decision 4
#
# Three tests:
#   1. guard_allows_shadow_delete    — NEW RED   (currently blocked by guard)
#   2. guard_blocks_real_factory     — REGRESSION PASS (already blocked)
#   3. guard_blocks_factory_specs    — REGRESSION PASS (already blocked)
#
# ADR-024 Decision 4: the guard must allow deletion of the exact shadow
# path `.factory/.factory/` while keeping all other `.factory/` paths
# blocked.  The fix adds a shadow-exception predicate BEFORE the existing
# `.worktrees/` exception inside the `for protected_re in ...` loop and
# inside the `find ... -delete` block.
# ---------------------------------------------------------------------------

# NEW RED: .factory/.factory/ is the recursive shadow created by the bug.
# Currently the guard blocks it because it matches the \.factory/ protected
# regex.  After ADR-024 Decision 4 is implemented, this must exit 0.
@test "guard_allows_shadow_delete: rm -rf .factory/.factory/ exits 0" {
  _run_hook "rm -rf .factory/.factory/"
  [ "$status" -eq 0 ]
}

# REGRESSION PASS: real .factory/ must remain blocked after the shadow
# exception is added.
@test "guard_blocks_real_factory_delete: rm -rf .factory/ exits 2" {
  _run_hook "rm -rf .factory/"
  [ "$status" -eq 2 ]
  [[ "$output" == *"BLOCKED"* ]]
}

# REGRESSION PASS: subdirectory of real .factory/ must remain blocked.
@test "guard_blocks_factory_specs_delete: rm -rf .factory/specs/ exits 2" {
  _run_hook "rm -rf .factory/specs/"
  [ "$status" -eq 2 ]
  [[ "$output" == *"BLOCKED"* ]]
}

# NEW RED: find variant of shadow delete must also be allowed.
# Currently blocked by the find-delete guard's \.factory\b pattern.
# After the find-delete shadow exception (ADR-024 Decision 4, second block)
# is implemented, this must exit 0.
@test "guard_allows_shadow_find_delete: find .factory/.factory -delete exits 0" {
  _run_hook "find .factory/.factory -type f -delete"
  [ "$status" -eq 0 ]
}

# REGRESSION PASS: find on real .factory/ (no shadow) stays blocked.
@test "guard_blocks_real_factory_find_delete: find .factory -delete exits 2" {
  _run_hook "find .factory -type f -delete"
  [ "$status" -eq 2 ]
}

# ---------- Emit-event integration ----------
# These tests verify that (a) events are emitted when the hook blocks, and
# (b) the hook still blocks correctly even when emit-event is missing or
# broken. (b) is the critical assertion — failure of the emission path
# must never change block/pass behavior.

_run_hook_with_emit() {
  # Like _run_hook, but exports CLAUDE_PLUGIN_ROOT and VSDD_LOG_DIR so events
  # land in a scratch dir we can inspect. Must export (not `VAR=x cmd`
  # prefix) because otherwise the var doesn't propagate across the pipe to
  # the hook binary on the right side.
  local cmd="$1"
  local input
  input=$(jq -nc --arg c "$cmd" '{tool_input: {command: $c}}')
  export CLAUDE_PLUGIN_ROOT="${BATS_TEST_DIRNAME}/.."
  export VSDD_LOG_DIR="$EMIT_TMPDIR"
  run bash -c "echo '$input' | '$HOOK' 2>&1"
  unset CLAUDE_PLUGIN_ROOT
  unset VSDD_LOG_DIR
}

@test "emit: block event written on rm -rf /" {
  EMIT_TMPDIR=$(mktemp -d)
  _run_hook_with_emit "rm -rf /"
  local rc=$status
  local logfile
  logfile=$(ls "$EMIT_TMPDIR"/events-*.jsonl 2>/dev/null | head -1)
  [ "$rc" -eq 2 ]
  [ -n "$logfile" ]
  local evt
  evt=$(cat "$logfile")
  [ "$(echo "$evt" | jq -r '.type')" = "hook.block" ]
  [ "$(echo "$evt" | jq -r '.hook')" = "destructive-command-guard" ]
  [ "$(echo "$evt" | jq -r '.reason')" = "catastrophic_root" ]
  rm -rf "$EMIT_TMPDIR"
}

@test "emit: reason=git_reset_hard on git reset --hard" {
  EMIT_TMPDIR=$(mktemp -d)
  _run_hook_with_emit "git reset --hard HEAD"
  [ "$status" -eq 2 ]
  local logfile=$(ls "$EMIT_TMPDIR"/events-*.jsonl 2>/dev/null | head -1)
  [ "$(jq -r '.reason' < "$logfile")" = "git_reset_hard" ]
  rm -rf "$EMIT_TMPDIR"
}

@test "emit: reason=sot_delete on rm STATE.md" {
  EMIT_TMPDIR=$(mktemp -d)
  _run_hook_with_emit "rm .factory/STATE.md"
  [ "$status" -eq 2 ]
  local logfile=$(ls "$EMIT_TMPDIR"/events-*.jsonl 2>/dev/null | head -1)
  [ "$(jq -r '.reason' < "$logfile")" = "sot_delete" ]
  rm -rf "$EMIT_TMPDIR"
}

@test "emit: reason=protected_path_delete on rm -rf .factory/" {
  EMIT_TMPDIR=$(mktemp -d)
  _run_hook_with_emit "rm -rf .factory/"
  [ "$status" -eq 2 ]
  local logfile=$(ls "$EMIT_TMPDIR"/events-*.jsonl 2>/dev/null | head -1)
  [ -n "$logfile" ]
  [ "$(jq -r '.reason' < "$logfile")" = "protected_path_delete" ]
  rm -rf "$EMIT_TMPDIR"
}

@test "emit: allowed commands produce NO event" {
  EMIT_TMPDIR=$(mktemp -d)
  _run_hook_with_emit "ls -la"
  [ "$status" -eq 0 ]
  [ -z "$(ls "$EMIT_TMPDIR"/events-*.jsonl 2>/dev/null)" ]
  rm -rf "$EMIT_TMPDIR"
}

# --- CRITICAL: hook must still block when emit-event is missing ------------

@test "emit: hook still blocks when CLAUDE_PLUGIN_ROOT is unset" {
  local input
  input=$(jq -nc --arg c "rm -rf /" '{tool_input: {command: $c}}')
  # Explicitly unset CLAUDE_PLUGIN_ROOT
  run bash -c "unset CLAUDE_PLUGIN_ROOT; echo '$input' | '$HOOK' 2>&1"
  [ "$status" -eq 2 ]
  [[ "$output" == *"BLOCKED"* ]]
}

@test "emit: hook still blocks when emit-event path is broken" {
  local input
  input=$(jq -nc --arg c "rm -rf /" '{tool_input: {command: $c}}')
  run bash -c "CLAUDE_PLUGIN_ROOT='/nonexistent/path' echo '$input' | '$HOOK' 2>&1"
  [ "$status" -eq 2 ]
  [[ "$output" == *"BLOCKED"* ]]
}

@test "emit: VSDD_TELEMETRY=off — hook still blocks, no event written" {
  EMIT_TMPDIR=$(mktemp -d)
  local input
  input=$(jq -nc --arg c "rm -rf /" '{tool_input: {command: $c}}')
  export VSDD_TELEMETRY=off
  export CLAUDE_PLUGIN_ROOT="${BATS_TEST_DIRNAME}/.."
  export VSDD_LOG_DIR="$EMIT_TMPDIR"
  run bash -c "echo '$input' | '$HOOK' 2>&1"
  unset VSDD_TELEMETRY CLAUDE_PLUGIN_ROOT VSDD_LOG_DIR
  [ "$status" -eq 2 ]
  [ -z "$(ls "$EMIT_TMPDIR"/events-*.jsonl 2>/dev/null)" ]
  rm -rf "$EMIT_TMPDIR"
}

# ---------------------------------------------------------------------------
# RED GATE: adversary pass-1 findings — C-1/C-2/H-1
# multi-target bypass: shadow exception must be TARGET-SCOPED, never
# command-string-scoped.
#
# The bug: the current shadow exception fires when the command STRING
# contains ".factory/.factory" — so a command like
#   rm -rf .factory/ .factory/.factory
# contains the shadow substring, causing the exception to `continue` and
# skip checking the real .factory/ target on the SAME command line.
#
# Security requirement: if ANY target on the command line is a real
# .factory/ path, the command must be BLOCKED (exit 2) regardless of
# whether the shadow path also appears.
#
# These tests will CURRENTLY PASS-the-wrong-way (exit 0) — that is the RED.
# The implementer must narrow the exception to apply only when the ENTIRE
# rm target matches .factory/.factory (and not also a real .factory path).
# ---------------------------------------------------------------------------

# C-1 / H-1: multi-target — real .factory/ PLUS shadow on same rm command.
# MUST exit 2 (real .factory/ target present).
# CURRENTLY exits 0 — BUG (shadow substring causes exception to fire for
# the whole command, bypassing the block).
@test "C-1: rm -rf .factory/ .factory/.factory MUST exit 2 (real target present)" {
  _run_hook "rm -rf .factory/ .factory/.factory"
  [ "$status" -eq 2 ]
}

# C-2: real subtree (.factory/specs) plus shadow on same rm command.
# MUST exit 2.
# CURRENTLY exits 0 — BUG.
@test "C-2: rm -rf .factory/specs .factory/.factory MUST exit 2 (real subtree present)" {
  _run_hook "rm -rf .factory/specs .factory/.factory"
  [ "$status" -eq 2 ]
}

# H-1 (compound): find on real .factory combined with shadow echo in same command.
# The compound command contains ".factory/.factory" as a literal string in the
# echo argument, but the find target is the real .factory tree.
# MUST exit 2 (find on real .factory present).
# CURRENTLY exits 0 — BUG (command string contains ".factory/.factory").
@test "H-1a: find .factory -delete ; echo .factory/.factory MUST exit 2 (real find target)" {
  _run_hook "find .factory -delete ; echo .factory/.factory"
  [ "$status" -eq 2 ]
}

# H-1 (compound): shadow find THEN real find.  Both subcommands are present;
# real one must still be blocked.
# MUST exit 2.
# CURRENTLY exits 0 — BUG.
@test "H-1b: find .factory/.factory -delete ; find .factory -delete MUST exit 2 (real find present)" {
  _run_hook "find .factory/.factory -delete ; find .factory -delete"
  [ "$status" -eq 2 ]
}

# Positive regression: shadow-only rm must remain allowed (exit 0).
# This test ALREADY passes and must not regress after the fix.
@test "positive: rm -rf .factory/.factory/ alone exits 0 (shadow only)" {
  _run_hook "rm -rf .factory/.factory/"
  [ "$status" -eq 0 ]
}

# Positive regression: shadow-only find must remain allowed (exit 0).
@test "positive: find .factory/.factory -delete alone exits 0 (shadow only)" {
  _run_hook "find .factory/.factory -delete"
  [ "$status" -eq 0 ]
}

# ---------------------------------------------------------------------------
# RED GATE: adversary pass-2 findings — C2-CRIT-1 / C2-HIGH-2
#
# C2-CRIT-1 (guard traversal under-protect):
#   Paths containing ".factory/.factory/.." resolve via ".." to the REAL
#   .factory/ tree.  The current strip-and-check logic replaces the literal
#   substring ".factory/.factory" with "__SHADOW__" and then checks whether
#   ".factory" remains.  A path like ".factory/.factory/../specs" becomes
#   "__SHADOW__/../specs" — which does NOT contain ".factory", so the guard
#   allows the command.  But __SHADOW__/../specs resolves to the real
#   .factory/specs path on disk.
#
#   These commands MUST exit 2 (blocked).  They currently exit 0 — that is
#   the RED.  The implementer must add ".." (and "..") detection: any
#   command whose shadow-stripped form still contains ".." escaping out of
#   the shadow subtree must be blocked.
#
# C2-HIGH-2 (nested-shadow over-block):
#   The shadow may itself contain a sub-directory also named ".factory",
#   e.g. if a mis-configured child workspace created .factory/.factory/.factory.
#   The current guard blocks ".factory/.factory/.factory" because after
#   shadow-stripping ".factory/.factory" becomes "__SHADOW__" and then
#   ".factory" still appears (from the third component).  But
#   ".factory/.factory/.factory" is entirely inside the shadow subtree —
#   it is NOT the real .factory/ worktree.
#
#   These commands MUST exit 0 (allowed).  They currently exit 2 — that is
#   the RED.  The implementer must extend the shadow exception so that any
#   path that begins with the literal prefix ".factory/.factory" (regardless
#   of what follows) is considered shadow-only.
#
# ALLOW/BLOCK matrix the implementer must satisfy:
#
#   BLOCK (exit 2) — path resolves to real .factory tree:
#     rm -rf .factory/.factory/../specs            (..  escapes to .factory/specs)
#     rm -rf .factory/.factory/..                  (.. escapes to .factory itself)
#     rm -rf .factory/.factory/../../              (../.. escapes above repo root)
#     find .factory/.factory/.. -delete            (.. escapes to real .factory)
#     find .factory/.factory -delete ; find .factory -delete   (already tested above)
#
#   ALLOW (exit 0) — path stays inside shadow subtree:
#     rm -rf .factory/.factory/             (already pass-1 green; keep regression)
#     rm -rf .factory/.factory/logs         (sub-dir of shadow; already green; keep)
#     rm -rf .factory/.factory/             (already pass-1 green; keep)
#     find .factory/.factory -delete        (already pass-1 green; keep)
#     rm -rf .factory/.factory/.factory     (nested shadow — C2-HIGH-2; currently RED)
# ---------------------------------------------------------------------------

# ---- C2-CRIT-1: traversal under-protect (currently exit 0 — BUG) --------

# ".." after ".factory/.factory/" resolves to the real .factory/specs path.
# MUST exit 2.  CURRENTLY exits 0.
@test "C2-CRIT-1a: rm -rf .factory/.factory/../specs MUST exit 2 (..traversal to real specs)" {
  _run_hook "rm -rf .factory/.factory/../specs"
  [ "$status" -eq 2 ]
}

# ".." after ".factory/.factory" resolves to the real .factory/ directory.
# MUST exit 2.  CURRENTLY exits 0.
@test "C2-CRIT-1b: rm -rf .factory/.factory/.. MUST exit 2 (..traversal to real .factory)" {
  _run_hook "rm -rf .factory/.factory/.."
  [ "$status" -eq 2 ]
}

# "../.." escapes entirely above the .factory/ worktree — catastrophic.
# MUST exit 2.  CURRENTLY exits 0.
@test "C2-CRIT-1c: rm -rf .factory/.factory/../../ MUST exit 2 (double ..traversal)" {
  _run_hook "rm -rf .factory/.factory/../../"
  [ "$status" -eq 2 ]
}

# find variant with ".." traversal: find .factory/.factory/.. -delete resolves
# to find .factory -delete — the real .factory tree.
# MUST exit 2.  CURRENTLY exits 0.
@test "C2-CRIT-1d: find .factory/.factory/.. -delete MUST exit 2 (..traversal in find)" {
  _run_hook "find .factory/.factory/.. -delete"
  [ "$status" -eq 2 ]
}

# ---- C2-HIGH-2: nested-shadow over-block (currently exit 2 — BUG) --------

# ".factory/.factory/.factory" is entirely inside the shadow subtree — no
# ".." escape, no real .factory/ worktree path.  MUST exit 0.  CURRENTLY exits 2.
@test "C2-HIGH-2a: rm -rf .factory/.factory/.factory MUST exit 0 (nested shadow, no escape)" {
  _run_hook "rm -rf .factory/.factory/.factory"
  [ "$status" -eq 0 ]
}

# ---- Regression guards: pass-1 greens that must remain green ------------

# These were introduced in pass-1 and must not regress after the pass-2 fix.
@test "regression: rm -rf .factory/.factory/ still exits 0 after pass-2 fix" {
  _run_hook "rm -rf .factory/.factory/"
  [ "$status" -eq 0 ]
}

@test "regression: rm -rf .factory/.factory/logs still exits 0 after pass-2 fix" {
  _run_hook "rm -rf .factory/.factory/logs"
  [ "$status" -eq 0 ]
}

@test "regression: find .factory/.factory -delete still exits 0 after pass-2 fix" {
  _run_hook "find .factory/.factory -delete"
  [ "$status" -eq 0 ]
}
