#!/usr/bin/env bats
# factory-sla.bats — tests for the agent duration tracking CLI.

setup() {
  TOOL="${BATS_TEST_DIRNAME}/../bin/factory-sla"
  SCRATCH=$(mktemp -d)
  export VSDD_LOG_DIR="$SCRATCH"
  # Seed a log with two complete pairs + one open start.
  cat > "$SCRATCH/events-2026-04-22.jsonl" <<'EOF'
{"type":"agent.start","subagent":"pr-manager","session_id":"sess-A","story_id":"S-1.01","ts":"2026-04-22T10:00:00-0500","ts_epoch":1745326800,"schema_version":1}
{"type":"agent.stop","subagent":"pr-manager","session_id":"sess-A","exit_class":"ok","ts":"2026-04-22T10:15:00-0500","ts_epoch":1745327700,"schema_version":1}
{"type":"agent.start","subagent":"test-writer","session_id":"sess-A","story_id":"S-1.01","ts":"2026-04-22T10:16:00-0500","ts_epoch":1745327760,"schema_version":1}
{"type":"agent.stop","subagent":"test-writer","session_id":"sess-A","exit_class":"ok","ts":"2026-04-22T10:20:00-0500","ts_epoch":1745328000,"schema_version":1}
{"type":"agent.start","subagent":"pr-manager","session_id":"sess-B","story_id":"S-2.02","ts":"2026-04-22T11:00:00-0500","ts_epoch":1745330400,"schema_version":1}
{"type":"agent.stop","subagent":"pr-manager","session_id":"sess-B","exit_class":"blocked","ts":"2026-04-22T11:30:00-0500","ts_epoch":1745332200,"schema_version":1}
{"type":"agent.start","subagent":"pr-manager","session_id":"sess-C","ts":"2026-04-22T12:00:00-0500","ts_epoch":1745334000,"schema_version":1}
EOF
}

teardown() {
  rm -rf "$SCRATCH"
  unset VSDD_LOG_DIR
}

# ---------- Structural ----------

@test "factory-sla: exists and executable" {
  [ -x "$TOOL" ]
}

@test "factory-sla: passes syntax check" {
  bash -n "$TOOL"
}

@test "factory-sla: help shows usage" {
  run "$TOOL" help
  [ "$status" -eq 0 ]
  [[ "$output" == *"Usage:"* ]]
}

@test "factory-sla: no args shows usage" {
  run "$TOOL"
  [ "$status" -eq 0 ]
  [[ "$output" == *"Usage:"* ]]
}

@test "factory-sla: unknown subcommand errors" {
  run "$TOOL" fake
  [ "$status" -ne 0 ]
}

# ---------- durations ----------

@test "durations: lists matched pairs with durations" {
  run "$TOOL" durations
  [ "$status" -eq 0 ]
  [[ "$output" == *"pr-manager"* ]]
  [[ "$output" == *"test-writer"* ]]
  [[ "$output" == *"900s"* ]]    # 10:00 → 10:15 = 900 seconds
  [[ "$output" == *"240s"* ]]    # 10:16 → 10:20 = 240 seconds
  [[ "$output" == *"1800s"* ]]   # 11:00 → 11:30 = 1800 seconds
}

@test "durations: --subagent filters to one agent type" {
  run "$TOOL" durations --subagent pr-manager
  [ "$status" -eq 0 ]
  [[ "$output" == *"pr-manager"* ]]
  [[ "$output" != *"test-writer"* ]]
}

@test "durations: --session filters to one session" {
  run "$TOOL" durations --session sess-B
  [ "$status" -eq 0 ]
  [[ "$output" == *"sess-B"* ]]
  [[ "$output" != *"sess-A"* ]]
}

@test "durations: --tsv produces tab-separated output" {
  run "$TOOL" durations --tsv
  [ "$status" -eq 0 ]
  [[ "$output" == *$'sess-A\tpr-manager\t'* ]]
}

@test "durations: shows exit class" {
  run "$TOOL" durations
  [[ "$output" == *"blocked"* ]]  # sess-B pr-manager
}

@test "durations: shows story_id when present in start event" {
  run "$TOOL" durations --subagent test-writer
  [[ "$output" == *"S-1.01"* ]]
}

# ---------- summary ----------

@test "summary: one row per subagent" {
  run "$TOOL" summary --tsv
  [ "$status" -eq 0 ]
  local rows
  rows=$(echo "$output" | grep -c '^[a-z]')
  [ "$rows" -eq 2 ]  # pr-manager + test-writer
}

@test "summary: correct count per subagent" {
  run "$TOOL" summary --tsv
  # pr-manager: 2 matched pairs (sess-A, sess-B)
  [[ "$output" == *$'pr-manager\t2\t'* ]]
  # test-writer: 1 matched pair
  [[ "$output" == *$'test-writer\t1\t'* ]]
}

@test "summary: min/max/mean for pr-manager" {
  run "$TOOL" summary --tsv
  # pr-manager has 900 + 1800 → min=900 max=1800 mean=1350
  [[ "$output" == *$'pr-manager\t2\t900\t'* ]]
  [[ "$output" == *"1800"* ]]
  [[ "$output" == *"1350"* ]]
}

@test "summary: default output is aligned columns" {
  run "$TOOL" summary
  [ "$status" -eq 0 ]
  [[ "$output" == *"Subagent"* ]]
  [[ "$output" == *"P50(s)"* ]]
  [[ "$output" == *"Mean(s)"* ]]
}

# ---------- open ----------

@test "open: shows unmatched starts" {
  run "$TOOL" open
  [ "$status" -eq 0 ]
  [[ "$output" == *"sess-C"* ]]
  [[ "$output" == *"pr-manager"* ]]
}

@test "open: sess-A and sess-B should NOT appear (both matched)" {
  run "$TOOL" open
  [[ "$output" != *"sess-A"* ]]
  [[ "$output" != *"sess-B"* ]]
}

@test "open: no open pairs produces graceful message" {
  # Remove the orphan start so every start has a matching stop.
  sed -i.bak '$d' "$SCRATCH/events-2026-04-22.jsonl" && rm "$SCRATCH/events-2026-04-22.jsonl.bak"
  run "$TOOL" open
  [ "$status" -eq 0 ]
  [[ "$output" == *"no open"* ]]
}

# ---------- Edge cases ----------

@test "durations: empty log dir graceful" {
  rm "$SCRATCH"/events-*.jsonl
  run "$TOOL" durations
  [ "$status" -eq 0 ]
  [[ "$output" == *"No events found"* ]]
}

@test "summary: empty log dir graceful" {
  rm "$SCRATCH"/events-*.jsonl
  run "$TOOL" summary
  [ "$status" -eq 0 ]
  [[ "$output" == *"No events found"* ]]
}

@test "durations: events without agent.start/stop types are ignored" {
  cat > "$SCRATCH/events-2026-04-23.jsonl" <<'EOF'
{"type":"hook.block","reason":"x","hook":"y","ts":"2026-04-23T10:00:00-0500","ts_epoch":1745413200,"schema_version":1}
EOF
  run "$TOOL" durations
  [ "$status" -eq 0 ]
  # Should still show the seed data's pairs (not counted the hook.block event)
  [[ "$output" == *"pr-manager"* ]]
}
