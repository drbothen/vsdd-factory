#!/usr/bin/env bats
# emit-event.bats — tests for the failure-tolerant event emitter.
#
# Phase 1 goal: prove emit-event can never break a hook. Every test in the
# "Exit code guarantees" and "Graceful degradation" sections must pass before
# any hook is instrumented to call emit-event.

setup() {
  HELPER="${BATS_TEST_DIRNAME}/../bin/emit-event"
  TMP_LOGDIR="$(mktemp -d)"
  export VSDD_LOG_DIR="$TMP_LOGDIR"
  unset VSDD_TELEMETRY
}

teardown() {
  # Restore any chmod changes made during tests before cleanup.
  chmod -R u+rwx "$TMP_LOGDIR" 2>/dev/null || true
  rm -rf "$TMP_LOGDIR"
}

# Return path of today's log file (or empty if none).
_logfile() {
  ls "$TMP_LOGDIR"/events-*.jsonl 2>/dev/null | head -1
}

# ---------- Structural ----------

@test "emit-event: file exists and is executable" {
  [ -x "$HELPER" ]
}

@test "emit-event: passes bash syntax check" {
  bash -n "$HELPER"
}

@test "emit-event: shebang is present" {
  head -1 "$HELPER" | grep -qE '^#!'
}

# ---------- Exit code guarantees ----------
# emit-event MUST exit 0 on every path. These tests are the foundation —
# if any of them fail, emit-event is unsafe to call from hooks.

@test "emit-event: exits 0 with no args" {
  run "$HELPER"
  [ "$status" -eq 0 ]
}

@test "emit-event: exits 0 with garbage args (no =)" {
  run "$HELPER" xxx yyy zzz
  [ "$status" -eq 0 ]
}

@test "emit-event: exits 0 with empty-key arg" {
  run "$HELPER" "=value"
  [ "$status" -eq 0 ]
}

@test "emit-event: exits 0 with binary data in value" {
  run "$HELPER" type=test val=$'\x01\x02\x03'
  [ "$status" -eq 0 ]
}

@test "emit-event: exits 0 with newlines in value" {
  run "$HELPER" type=test val=$'line1\nline2\nline3'
  [ "$status" -eq 0 ]
}

@test "emit-event: exits 0 with very long value (10KB)" {
  local big
  big=$(head -c 10000 /dev/urandom | base64 | tr -d '\n')
  run "$HELPER" type=test val="$big"
  [ "$status" -eq 0 ]
}

@test "emit-event: exits 0 with many args (50 pairs)" {
  local args=()
  for i in $(seq 1 50); do
    args+=("key${i}=value${i}")
  done
  run "$HELPER" "${args[@]}"
  [ "$status" -eq 0 ]
}

# ---------- Kill switch ----------

@test "emit-event: VSDD_TELEMETRY=off suppresses writes" {
  VSDD_TELEMETRY=off run "$HELPER" type=test
  [ "$status" -eq 0 ]
  [ -z "$(_logfile)" ]
}

@test "emit-event: VSDD_TELEMETRY=off with many args still suppresses" {
  VSDD_TELEMETRY=off run "$HELPER" type=x a=1 b=2 c=3 d=4 e=5
  [ "$status" -eq 0 ]
  [ -z "$(_logfile)" ]
}

@test "emit-event: VSDD_TELEMETRY unset enables emission" {
  run "$HELPER" type=test
  [ "$status" -eq 0 ]
  [ -n "$(_logfile)" ]
}

@test "emit-event: VSDD_TELEMETRY=on (explicit) enables emission" {
  VSDD_TELEMETRY=on run "$HELPER" type=test
  [ "$status" -eq 0 ]
  [ -n "$(_logfile)" ]
}

# ---------- Graceful degradation ----------

@test "emit-event: missing jq exits 0 with no writes" {
  # PATH=/bin:/usr/bin keeps mkdir/date/etc. available so the helper can
  # start, but excludes /opt/homebrew/bin and /usr/local/bin where jq lives
  # on macOS and most Linux installs. On machines where jq is in /usr/bin,
  # this test would be a no-op — skip in that case.
  if [ -x /usr/bin/jq ]; then
    skip "jq installed in /usr/bin; cannot simulate missing jq on this system"
  fi
  PATH="/bin:/usr/bin" run "$HELPER" type=test
  [ "$status" -eq 0 ]
  [ -z "$(_logfile)" ]
}

@test "emit-event: readonly log dir exits 0, no crash" {
  chmod 555 "$TMP_LOGDIR"
  run "$HELPER" type=test
  local rc=$status
  chmod 755 "$TMP_LOGDIR"
  [ "$rc" -eq 0 ]
}

@test "emit-event: unwritable parent dir exits 0" {
  local parent
  parent=$(mktemp -d)
  chmod 555 "$parent"
  VSDD_LOG_DIR="$parent/subdir" run "$HELPER" type=test
  local rc=$status
  chmod 755 "$parent"
  rm -rf "$parent"
  [ "$rc" -eq 0 ]
}

@test "emit-event: nonexistent log dir is auto-created" {
  local deep="$TMP_LOGDIR/a/b/c"
  VSDD_LOG_DIR="$deep" run "$HELPER" type=test
  [ "$status" -eq 0 ]
  [ -d "$deep" ]
  [ -n "$(ls "$deep"/events-*.jsonl 2>/dev/null | head -1)" ]
}

# ---------- Successful emission: structure ----------

@test "emit-event: successful call writes one line" {
  run "$HELPER" type=test
  [ "$status" -eq 0 ]
  local f
  f=$(_logfile)
  [ -n "$f" ]
  [ "$(wc -l < "$f" | tr -d ' ')" = "1" ]
}

@test "emit-event: emitted line is valid JSON" {
  run "$HELPER" type=test hook=x reason=y
  local f
  f=$(_logfile)
  jq -e '.' < "$f" >/dev/null
}

@test "emit-event: emitted event has ts field" {
  run "$HELPER" type=test
  local f
  f=$(_logfile)
  local ts
  ts=$(jq -r '.ts' < "$f")
  [ -n "$ts" ]
  [ "$ts" != "null" ]
}

@test "emit-event: ts is ISO-8601 with timezone" {
  run "$HELPER" type=test
  local f
  f=$(_logfile)
  local ts
  ts=$(jq -r '.ts' < "$f")
  # Should look like 2026-04-22T14:30:00+0000 or similar
  [[ "$ts" =~ ^[0-9]{4}-[0-9]{2}-[0-9]{2}T[0-9]{2}:[0-9]{2}:[0-9]{2}[+-][0-9]{4}$ ]]
}

@test "emit-event: emitted event has schema_version field" {
  run "$HELPER" type=test
  local f
  f=$(_logfile)
  [ "$(jq -r '.schema_version' < "$f")" = "1" ]
}

# ---------- Successful emission: field handling ----------

@test "emit-event: caller fields appear in output" {
  run "$HELPER" type=hook.block hook=destructive-guard reason=rm_root
  local f
  f=$(_logfile)
  [ "$(jq -r '.type' < "$f")" = "hook.block" ]
  [ "$(jq -r '.hook' < "$f")" = "destructive-guard" ]
  [ "$(jq -r '.reason' < "$f")" = "rm_root" ]
}

@test "emit-event: values with spaces are preserved" {
  run "$HELPER" type=test command="rm -rf /"
  local f
  f=$(_logfile)
  [ "$(jq -r '.command' < "$f")" = "rm -rf /" ]
}

@test "emit-event: values with quotes escaped correctly" {
  run "$HELPER" type=test val='a"b'\''c'
  local f
  f=$(_logfile)
  [ "$(jq -r '.val' < "$f")" = 'a"b'"'"'c' ]
}

@test "emit-event: values with backslashes preserved" {
  run "$HELPER" type=test val='a\b\c'
  local f
  f=$(_logfile)
  [ "$(jq -r '.val' < "$f")" = 'a\b\c' ]
}

@test "emit-event: value containing = is preserved intact" {
  run "$HELPER" type=test val="a=b=c=d"
  local f
  f=$(_logfile)
  [ "$(jq -r '.val' < "$f")" = "a=b=c=d" ]
}

@test "emit-event: key with dot stays flat (not nested)" {
  run "$HELPER" "hook.name=destructive-guard" type=test
  local f
  f=$(_logfile)
  [ "$(jq -r '."hook.name"' < "$f")" = "destructive-guard" ]
  # Should NOT be nested
  [ "$(jq -r '.hook.name // empty' < "$f")" = "" ]
}

@test "emit-event: empty value allowed" {
  run "$HELPER" type=test val=
  [ "$status" -eq 0 ]
  local f
  f=$(_logfile)
  [ "$(jq -r '.val' < "$f")" = "" ]
}

# ---------- Append semantics ----------

@test "emit-event: multiple calls append to same daily file" {
  "$HELPER" type=one
  "$HELPER" type=two
  "$HELPER" type=three
  local f
  f=$(_logfile)
  [ "$(wc -l < "$f" | tr -d ' ')" = "3" ]
  # Verify all three events present in order
  [ "$(jq -r '.type' < "$f" | head -1)" = "one" ]
  [ "$(jq -r '.type' < "$f" | sed -n '2p')" = "two" ]
  [ "$(jq -r '.type' < "$f" | sed -n '3p')" = "three" ]
}

@test "emit-event: each line is independently valid JSON" {
  "$HELPER" type=first
  "$HELPER" type=second
  local f
  f=$(_logfile)
  # Validate each line parses
  while IFS= read -r line; do
    echo "$line" | jq -e '.' >/dev/null
  done < "$f"
}

# ---------- Filename conventions ----------

@test "emit-event: daily file name uses YYYY-MM-DD" {
  run "$HELPER" type=test
  local f
  f=$(basename "$(_logfile)")
  [[ "$f" =~ ^events-[0-9]{4}-[0-9]{2}-[0-9]{2}\.jsonl$ ]]
}

# ---------- Argument edge cases ----------

@test "emit-event: key with unsafe chars is sanitized" {
  # Keys with slashes, quotes, $ are stripped to [A-Za-z0-9_.].
  # "foo/bar$baz" becomes "foobarbaz".
  run "$HELPER" "foo/bar\$baz=v" type=test
  [ "$status" -eq 0 ]
  local f
  f=$(_logfile)
  [ "$(jq -r '.foobarbaz' < "$f")" = "v" ]
}

@test "emit-event: key becoming empty after sanitize is skipped" {
  run "$HELPER" "///=v" type=test
  [ "$status" -eq 0 ]
  local f
  f=$(_logfile)
  # Should still have type field, but no field from the sanitized-to-empty key
  [ "$(jq -r '.type' < "$f")" = "test" ]
}

# ---------- Session ID auto-injection (v0.67+) ----------

@test "emit-event: VSDD_SESSION_ID auto-injects session_id field" {
  VSDD_SESSION_ID="sess-test-xyz" run "$HELPER" type=test
  [ "$status" -eq 0 ]
  local f
  f=$(_logfile)
  [ "$(jq -r '.session_id' < "$f")" = "sess-test-xyz" ]
}

@test "emit-event: CLAUDE_SESSION_ID is also recognized" {
  unset VSDD_SESSION_ID || true
  CLAUDE_SESSION_ID="claude-sess-abc" run "$HELPER" type=test
  [ "$status" -eq 0 ]
  local f
  f=$(_logfile)
  [ "$(jq -r '.session_id' < "$f")" = "claude-sess-abc" ]
}

@test "emit-event: VSDD_SESSION_ID wins over CLAUDE_SESSION_ID" {
  VSDD_SESSION_ID=override CLAUDE_SESSION_ID=claude run "$HELPER" type=test
  [ "$status" -eq 0 ]
  local f
  f=$(_logfile)
  [ "$(jq -r '.session_id' < "$f")" = "override" ]
}

@test "emit-event: caller-provided session_id beats env vars" {
  VSDD_SESSION_ID=env-value run "$HELPER" type=test session_id=caller-value
  [ "$status" -eq 0 ]
  local f
  f=$(_logfile)
  [ "$(jq -r '.session_id' < "$f")" = "caller-value" ]
}

@test "emit-event: no session_id when neither env nor caller provides one" {
  unset VSDD_SESSION_ID CLAUDE_SESSION_ID || true
  run "$HELPER" type=test
  [ "$status" -eq 0 ]
  local f
  f=$(_logfile)
  # When neither set, session_id should be absent (null via jq)
  local sid
  sid=$(jq -r '.session_id // "ABSENT"' < "$f")
  [ "$sid" = "ABSENT" ]
}
