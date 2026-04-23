#!/usr/bin/env bats
# capture-pr-activity.bats — tests for the PR activity capture hook.
#
# The hook parses PostToolUse inputs for `gh pr create` / `gh pr merge`
# and emits pr.opened / pr.merged events. Tests cover:
#   - pr.opened emitted on successful `gh pr create` with URL in stdout
#   - pr.merged emitted on successful `gh pr merge`
#   - merge strategy extracted from flags
#   - positional PR number form (`gh pr merge 42`)
#   - non-matching commands are no-ops
#   - failed exit codes are no-ops
#   - missing CLAUDE_PLUGIN_ROOT / VSDD_TELEMETRY=off → graceful degradation
#
# Test approach: write JSON input to a temp file, feed via `< file` to
# the hook. Avoids bash -c double-quoting issues when the JSON body
# contains nested quotes.

setup() {
  HOOK="${BATS_TEST_DIRNAME}/../hooks/capture-pr-activity.sh"
  TMP_LOGDIR="$(mktemp -d)"
  INPUT_FILE="$(mktemp)"
  export VSDD_LOG_DIR="$TMP_LOGDIR"
  export CLAUDE_PLUGIN_ROOT="${BATS_TEST_DIRNAME}/.."
  unset VSDD_TELEMETRY VSDD_SESSION_ID CLAUDE_SESSION_ID || true
}

teardown() {
  chmod -R u+rwx "$TMP_LOGDIR" 2>/dev/null || true
  rm -rf "$TMP_LOGDIR" "$INPUT_FILE"
}

_logfile() {
  ls "$TMP_LOGDIR"/events-*.jsonl 2>/dev/null | head -1
}

_last_event() {
  tail -1 "$(_logfile)"
}

# Build a PostToolUse Bash input JSON and write it to $INPUT_FILE.
_set_input() {
  local command="$1"
  local stdout="$2"
  local exit_code="${3:-0}"
  jq -cn --arg cmd "$command" --arg out "$stdout" --argjson ec "$exit_code" '{
    tool_name: "Bash",
    tool_input: { command: $cmd },
    tool_response: { stdout: $out, stderr: "", exit_code: $ec }
  }' > "$INPUT_FILE"
}

# Run the hook with $INPUT_FILE as stdin.
_run_hook() {
  "$HOOK" < "$INPUT_FILE"
}

# ---------- Structural ----------

@test "hook: exists and executable" {
  [ -x "$HOOK" ]
}

@test "hook: passes bash syntax check" {
  bash -n "$HOOK"
}

# ---------- pr.opened ----------

@test "gh pr create with URL in stdout emits pr.opened" {
  _set_input 'gh pr create --title "feat: thing" --body "x"' 'https://github.com/owner/repo/pull/42'
  run _run_hook
  [ "$status" -eq 0 ]
  local f event
  f=$(_logfile)
  [ -n "$f" ]
  event=$(_last_event)
  [ "$(echo "$event" | jq -r '.type')" = "pr.opened" ]
  [ "$(echo "$event" | jq -r '.hook')" = "capture-pr-activity" ]
  [ "$(echo "$event" | jq -r '.pr_url')" = "https://github.com/owner/repo/pull/42" ]
  [ "$(echo "$event" | jq -r '.pr_number')" = "42" ]
  [ "$(echo "$event" | jq -r '.pr_repo')" = "owner/repo" ]
}

@test "gh pr create extracts --title if present" {
  _set_input 'gh pr create --title "feat(S-1.07): do the thing" --body b' 'https://github.com/o/r/pull/7'
  run _run_hook
  [ "$status" -eq 0 ]
  [ "$(_last_event | jq -r '.title')" = "feat(S-1.07): do the thing" ]
}

@test "gh pr create with NO URL in stdout is skipped (abnormal)" {
  _set_input 'gh pr create --title t --body b' 'something-unexpected' 0
  run _run_hook
  [ "$status" -eq 0 ]
  [ -z "$(_logfile)" ]
}

# ---------- pr.merged ----------

@test "gh pr merge with URL form emits pr.merged" {
  _set_input 'gh pr merge https://github.com/owner/repo/pull/99 --squash' 'Merged!'
  run _run_hook
  [ "$status" -eq 0 ]
  local event
  event=$(_last_event)
  [ "$(echo "$event" | jq -r '.type')" = "pr.merged" ]
  [ "$(echo "$event" | jq -r '.pr_url')" = "https://github.com/owner/repo/pull/99" ]
  [ "$(echo "$event" | jq -r '.pr_number')" = "99" ]
  [ "$(echo "$event" | jq -r '.pr_repo')" = "owner/repo" ]
  [ "$(echo "$event" | jq -r '.merge_strategy')" = "squash" ]
}

@test "gh pr merge with positional PR number emits pr.merged" {
  _set_input 'gh pr merge 42 --rebase' ''
  run _run_hook
  [ "$status" -eq 0 ]
  local event
  event=$(_last_event)
  [ "$(echo "$event" | jq -r '.type')" = "pr.merged" ]
  [ "$(echo "$event" | jq -r '.pr_number')" = "42" ]
  [ "$(echo "$event" | jq -r '.merge_strategy')" = "rebase" ]
}

@test "gh pr merge without PR number is skipped" {
  _set_input 'gh pr merge --help' 'USAGE...'
  run _run_hook
  [ "$status" -eq 0 ]
  [ -z "$(_logfile)" ]
}

# ---------- Filter predicate tightness ----------

@test "non-gh bash command is a no-op" {
  _set_input 'echo hello' 'hello'
  run _run_hook
  [ "$status" -eq 0 ]
  [ -z "$(_logfile)" ]
}

@test "echoed mention of gh pr create is NOT matched" {
  # The command contains the text but isn't really invoking gh. The
  # regex anchors to a whitespace/separator boundary, so this should not
  # match.
  _set_input 'echo "Use gh pr create to open a PR"' 'Use gh pr create to open a PR'
  run _run_hook
  [ "$status" -eq 0 ]
  [ -z "$(_logfile)" ]
}

@test "failed exit code is a no-op" {
  _set_input 'gh pr create --title t --body b' '' 1
  run _run_hook
  [ "$status" -eq 0 ]
  [ -z "$(_logfile)" ]
}

@test "non-Bash tool_name is a no-op" {
  echo '{"tool_name":"Edit","tool_input":{}}' > "$INPUT_FILE"
  run _run_hook
  [ "$status" -eq 0 ]
  [ -z "$(_logfile)" ]
}

# ---------- Graceful degradation ----------

@test "missing CLAUDE_PLUGIN_ROOT exits 0 without crashing" {
  unset CLAUDE_PLUGIN_ROOT
  _set_input 'gh pr create --title t --body b' 'https://github.com/o/r/pull/1'
  run _run_hook
  [ "$status" -eq 0 ]
  [ -z "$(_logfile)" ]
}

@test "VSDD_TELEMETRY=off does not affect the hook's own exit code" {
  _set_input 'gh pr create --title t --body b' 'https://github.com/o/r/pull/1'
  VSDD_TELEMETRY=off run _run_hook
  [ "$status" -eq 0 ]
  # emit-event silently no-ops due to the kill switch; hook is unaffected.
  [ -z "$(_logfile)" ]
}

@test "malformed input JSON does not crash the hook" {
  echo 'not-json' > "$INPUT_FILE"
  run _run_hook
  [ "$status" -eq 0 ]
}

# ---------- Open→merge duration pairing (v0.75+) ----------

@test "pr.merged computes open_to_merge_seconds from prior pr.opened with same pr_number" {
  # Seed a pr.opened event 60 seconds ago into the log file, then fire the
  # merge hook for the same PR number. Expect open_to_merge_seconds = ~60.
  local past_epoch
  past_epoch=$(($(date +%s) - 60))
  local today
  today=$(date +%Y-%m-%d)
  cat > "$TMP_LOGDIR/events-${today}.jsonl" <<JSONL
{"type":"pr.opened","hook":"capture-pr-activity","pr_number":"500","pr_url":"https://github.com/o/r/pull/500","ts_epoch":${past_epoch},"schema_version":1}
JSONL
  _set_input 'gh pr merge https://github.com/o/r/pull/500 --squash' 'Merged!'
  run _run_hook
  [ "$status" -eq 0 ]
  local event duration
  event=$(_last_event)
  [ "$(echo "$event" | jq -r '.type')" = "pr.merged" ]
  [ "$(echo "$event" | jq -r '.pr_number')" = "500" ]
  duration=$(echo "$event" | jq -r '.open_to_merge_seconds // "missing"')
  # Duration should be 59-61 seconds (allowing ±1s jitter).
  [ "$duration" != "missing" ]
  [ "$duration" -ge 58 ]
  [ "$duration" -le 62 ]
}

@test "pr.merged without a prior pr.opened omits open_to_merge_seconds" {
  # No seed — the merge should still emit, just without the duration field.
  _set_input 'gh pr merge 999 --squash' ''
  run _run_hook
  [ "$status" -eq 0 ]
  local event
  event=$(_last_event)
  [ "$(echo "$event" | jq -r '.type')" = "pr.merged" ]
  [ "$(echo "$event" | jq -r '.open_to_merge_seconds // "missing"')" = "missing" ]
}

@test "pr.merged rejects absurd durations (>30d) from stale pr.opened events" {
  # Seed a pr.opened event 40 days ago — stale. The hook should NOT use it.
  local way_past
  way_past=$(($(date +%s) - 3456000))  # 40 days
  local today
  today=$(date +%Y-%m-%d)
  cat > "$TMP_LOGDIR/events-${today}.jsonl" <<JSONL
{"type":"pr.opened","hook":"capture-pr-activity","pr_number":"700","pr_url":"https://github.com/o/r/pull/700","ts_epoch":${way_past},"schema_version":1}
JSONL
  _set_input 'gh pr merge 700 --squash' ''
  run _run_hook
  [ "$status" -eq 0 ]
  local event
  event=$(_last_event)
  [ "$(echo "$event" | jq -r '.type')" = "pr.merged" ]
  [ "$(echo "$event" | jq -r '.open_to_merge_seconds // "missing"')" = "missing" ]
}
