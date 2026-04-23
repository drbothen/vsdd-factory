#!/usr/bin/env bats
# capture-commit-activity.bats — tests for the commit activity capture hook.
#
# The hook parses PostToolUse inputs for `git commit` and emits a
# commit.made event. Tests cover:
#   - commit.made emitted on successful `git commit`
#   - sha + branch extracted from `[<branch> <sha>]` preamble
#   - message_subject extracted from stdout first line
#   - --amend invocations flag amended=true
#   - non-matching commands (echo, amend-only text, etc) are no-ops
#   - failed exit codes are no-ops
#   - malformed stdout is a no-op (no garbage events)
#   - hooks.json registration
#   - graceful degradation (missing CLAUDE_PLUGIN_ROOT, kill-switch)
#
# Same input-via-tempfile-to-stdin approach as capture-pr-activity.bats —
# avoids shell-quoting pain when the JSON body contains nested quotes.

setup() {
  HOOK="${BATS_TEST_DIRNAME}/../hooks/capture-commit-activity.sh"
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

@test "hooks.json: capture-commit-activity wired under PostToolUse Bash" {
  # The hook must be registered alongside capture-pr-activity so it
  # actually fires during Claude sessions.
  local registered
  registered=$(jq -r '
    .hooks.PostToolUse[]
    | select(.matcher == "Bash")
    | .hooks[].command
  ' "${CLAUDE_PLUGIN_ROOT}/hooks/hooks.json" | grep -c capture-commit-activity.sh)
  [ "$registered" -eq 1 ]
}

# ---------- commit.made (positive cases) ----------

@test "successful git commit emits commit.made with sha + branch" {
  _set_input 'git commit -m "feat: test"' \
$'[main abc1234] feat: test\n 1 file changed, 1 insertion(+)' 0
  run _run_hook
  [ "$status" -eq 0 ]
  local event
  event=$(_last_event)
  [ "$(echo "$event" | jq -r '.type')" = "commit.made" ]
  [ "$(echo "$event" | jq -r '.hook')" = "capture-commit-activity" ]
  [ "$(echo "$event" | jq -r '.matcher')" = "Bash" ]
  [ "$(echo "$event" | jq -r '.commit_sha')" = "abc1234" ]
  [ "$(echo "$event" | jq -r '.branch')" = "main" ]
  [ "$(echo "$event" | jq -r '.message_subject')" = "feat: test" ]
  # Amended should be absent on a normal commit.
  [ "$(echo "$event" | jq -r '.amended // "missing"')" = "missing" ]
}

@test "full 40-char sha is accepted" {
  _set_input 'git commit -m x' \
$'[feature abcdef0123456789abcdef0123456789abcdef01] x' 0
  run _run_hook
  [ "$status" -eq 0 ]
  [ "$(_last_event | jq -r '.commit_sha')" = "abcdef0123456789abcdef0123456789abcdef01" ]
}

@test "git commit -a -m combined short flags emits" {
  _set_input 'git commit -am "chore: tidy"' \
$'[main def5678] chore: tidy\n 2 files changed' 0
  run _run_hook
  [ "$status" -eq 0 ]
  [ "$(_last_event | jq -r '.type')" = "commit.made" ]
  [ "$(_last_event | jq -r '.commit_sha')" = "def5678" ]
}

@test "git commit --amend flags amended=true" {
  _set_input 'git commit --amend --no-edit' \
$'[main feed001] previous subject unchanged\n 0 files changed' 0
  run _run_hook
  [ "$status" -eq 0 ]
  local event
  event=$(_last_event)
  [ "$(echo "$event" | jq -r '.type')" = "commit.made" ]
  [ "$(echo "$event" | jq -r '.amended')" = "true" ]
}

@test "branches with slashes are preserved verbatim" {
  # e.g., feature/x-y-z. The bracket-contents tokenization splits on
  # whitespace, not /, so slashes ride through fine.
  _set_input 'git commit -m x' \
$'[feature/add-thing cafe000] x' 0
  run _run_hook
  [ "$status" -eq 0 ]
  [ "$(_last_event | jq -r '.branch')" = "feature/add-thing" ]
}

@test "root-commit preamble is handled (sha is last token)" {
  # Git prints "(root-commit)" as an extra bracket token for the first
  # commit. We take the LAST token as the sha, so this still works.
  _set_input 'git init && git commit -m first' \
$'[main (root-commit) 9001abc] first' 0
  run _run_hook
  [ "$status" -eq 0 ]
  [ "$(_last_event | jq -r '.commit_sha')" = "9001abc" ]
  [ "$(_last_event | jq -r '.branch')" = "main" ]
}

# ---------- Filter predicate tightness ----------

@test "echo of 'git commit' text is NOT matched" {
  _set_input 'echo "run git commit before pushing"' 'run git commit before pushing' 0
  run _run_hook
  [ "$status" -eq 0 ]
  [ -z "$(_logfile)" ]
}

@test "git commit-tree is NOT matched (word boundary)" {
  # commit-tree is a plumbing subcommand, not commit. The \b-equivalent
  # anchoring in the hook rejects it.
  _set_input 'git commit-tree -m x HEAD' '' 0
  run _run_hook
  [ "$status" -eq 0 ]
  [ -z "$(_logfile)" ]
}

@test "non-git bash command is a no-op" {
  _set_input 'ls -la' 'a b c' 0
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

# ---------- Failure + malformed-stdout handling ----------

@test "failed exit code is a no-op (pre-commit hook rejected)" {
  _set_input 'git commit -m x' '' 1
  run _run_hook
  [ "$status" -eq 0 ]
  [ -z "$(_logfile)" ]
}

@test "git commit --dry-run has no [<branch> <sha>] preamble — no-op" {
  _set_input 'git commit --dry-run -m x' \
$'On branch main\nChanges to be committed:\n\tmodified:   foo.txt' 0
  run _run_hook
  [ "$status" -eq 0 ]
  [ -z "$(_logfile)" ]
}

@test "stdout that starts with [ but no sha is rejected" {
  # Someone's custom alias prints '[INFO] git commit ran' — not a commit.
  _set_input 'git commit -m x' '[INFO] something non-sha' 0
  run _run_hook
  [ "$status" -eq 0 ]
  [ -z "$(_logfile)" ]
}

@test "garbage 6-char token in brackets is rejected (min 7)" {
  _set_input 'git commit -m x' '[main abc123] oops only 6 chars' 0
  run _run_hook
  [ "$status" -eq 0 ]
  [ -z "$(_logfile)" ]
}

@test "non-hex token in brackets is rejected" {
  _set_input 'git commit -m x' '[main NOTSHAs] fake' 0
  run _run_hook
  [ "$status" -eq 0 ]
  [ -z "$(_logfile)" ]
}

# ---------- Graceful degradation ----------

@test "missing CLAUDE_PLUGIN_ROOT exits 0 without crashing" {
  unset CLAUDE_PLUGIN_ROOT
  _set_input 'git commit -m x' $'[main abc1234] x' 0
  run _run_hook
  [ "$status" -eq 0 ]
  [ -z "$(_logfile)" ]
}

@test "VSDD_TELEMETRY=off does not affect the hook's own exit code" {
  _set_input 'git commit -m x' $'[main abc1234] x' 0
  VSDD_TELEMETRY=off run _run_hook
  [ "$status" -eq 0 ]
  # emit-event silently no-ops due to the kill switch; hook unaffected.
  [ -z "$(_logfile)" ]
}

@test "malformed input JSON does not crash the hook" {
  echo 'not-json' > "$INPUT_FILE"
  run _run_hook
  [ "$status" -eq 0 ]
}
