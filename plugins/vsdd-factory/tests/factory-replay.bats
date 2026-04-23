#!/usr/bin/env bats
# factory-replay.bats — tests for the session replay CLI.

setup() {
  TOOL="${BATS_TEST_DIRNAME}/../bin/factory-replay"
  SCRATCH=$(mktemp -d)
  export VSDD_LOG_DIR="$SCRATCH"
  # Seed events from two distinct sessions plus one orphan.
  # Timestamps deliberately ordered sess-A-1 < sess-A-2 < sess-B-1.
  cat > "$SCRATCH/events-2026-04-22.jsonl" <<'EOF'
{"type":"hook.block","reason":"catastrophic_root","hook":"destructive-command-guard","matcher":"Bash","command":"rm -rf /","session_id":"sess-A","ts":"2026-04-22T10:00:00-0500","schema_version":1}
{"type":"hook.block","reason":"env_file_read_shell","hook":"protect-secrets","matcher":"Bash","command":"cat .env","session_id":"sess-A","ts":"2026-04-22T10:05:00-0500","schema_version":1}
{"type":"hook.block","reason":"git_reset_hard","hook":"destructive-command-guard","matcher":"Bash","command":"git reset --hard","session_id":"sess-B","ts":"2026-04-22T11:00:00-0500","schema_version":1}
{"type":"hook.block","reason":"orphan_event","hook":"test","matcher":"Bash","ts":"2026-04-22T09:00:00-0500","schema_version":1}
EOF
}

teardown() {
  rm -rf "$SCRATCH"
  unset VSDD_LOG_DIR
}

# ---------- Structural ----------

@test "factory-replay: exists and executable" {
  [ -x "$TOOL" ]
}

@test "factory-replay: passes syntax check" {
  bash -n "$TOOL"
}

@test "factory-replay: help shows usage" {
  run "$TOOL" help
  [ "$status" -eq 0 ]
  [[ "$output" == *"Usage:"* ]]
}

@test "factory-replay: unknown subcommand fails" {
  run "$TOOL" fake
  [ "$status" -ne 0 ]
}

# ---------- sessions ----------

@test "sessions: lists all sessions including (no-session)" {
  run "$TOOL" sessions
  [ "$status" -eq 0 ]
  [[ "$output" == *"sess-A"* ]]
  [[ "$output" == *"sess-B"* ]]
  [[ "$output" == *"(no-session)"* ]]
}

@test "sessions: reports correct event counts" {
  run "$TOOL" sessions --tsv
  [ "$status" -eq 0 ]
  # sess-A has 2 events
  [[ "$output" == *$'sess-A\t2\t'* ]]
  # sess-B has 1
  [[ "$output" == *$'sess-B\t1\t'* ]]
}

@test "sessions: sorted by last_ts descending (most recent first)" {
  run "$TOOL" sessions --tsv
  # Expected order: sess-B (11:00) before sess-A (10:05) before (no-session) (09:00)
  local first_line
  first_line=$(echo "$output" | head -1)
  [[ "$first_line" == sess-B* ]]
}

@test "sessions: empty log dir produces graceful message" {
  rm -f "$SCRATCH"/events-*.jsonl
  run "$TOOL" sessions
  [ "$status" -eq 0 ]
  [[ "$output" == *"No events found"* ]]
}

# ---------- show ----------

@test "show: renders only matching session events" {
  run "$TOOL" show sess-A
  [ "$status" -eq 0 ]
  [[ "$output" == *"2 event(s)"* ]]
  [[ "$output" == *"catastrophic_root"* ]]
  [[ "$output" == *"env_file_read_shell"* ]]
  [[ "$output" != *"git_reset_hard"* ]]
}

@test "show: chronological order" {
  run "$TOOL" show sess-A
  # catastrophic_root (10:00) should appear before env_file_read_shell (10:05)
  local first_line second_line
  first_line=$(echo "$output" | grep -E "catastrophic_root|env_file" | head -1)
  second_line=$(echo "$output" | grep -E "catastrophic_root|env_file" | sed -n 2p)
  [[ "$first_line" == *"catastrophic_root"* ]]
  [[ "$second_line" == *"env_file"* ]]
}

@test "show: unknown session returns graceful message" {
  run "$TOOL" show sess-nonexistent
  [ "$status" -eq 0 ]
  [[ "$output" == *"no events"* ]]
}

@test "show: missing sid arg errors" {
  run "$TOOL" show
  [ "$status" -ne 0 ]
}

@test "show: (no-session) matches orphan events" {
  run "$TOOL" show "(no-session)"
  [ "$status" -eq 0 ]
  [[ "$output" == *"1 event(s)"* ]]
  [[ "$output" == *"orphan_event"* ]]
}

# ---------- latest ----------

@test "latest: shows the most recent session" {
  run "$TOOL" latest
  [ "$status" -eq 0 ]
  # sess-B is most recent (11:00)
  [[ "$output" == *"session sess-B"* ]]
  [[ "$output" == *"git_reset_hard"* ]]
}

@test "latest: no sessions produces graceful message" {
  rm "$SCRATCH"/events-*.jsonl
  run "$TOOL" latest
  [ "$status" -eq 0 ]
  [[ "$output" == *"No events found"* ]]
}

@test "latest: only orphan events produces graceful message" {
  cat > "$SCRATCH/events-2026-04-22.jsonl" <<'EOF'
{"type":"hook.block","reason":"x","hook":"y","ts":"2026-04-22T10:00:00-0500","schema_version":1}
EOF
  run "$TOOL" latest
  [ "$status" -eq 0 ]
  [[ "$output" == *"no sessions recorded"* ]]
}
