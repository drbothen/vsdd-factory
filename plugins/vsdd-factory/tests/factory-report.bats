#!/usr/bin/env bats
# factory-report.bats — tests for the observability summary CLI.

setup() {
  TOOL="${BATS_TEST_DIRNAME}/../bin/factory-report"
  SCRATCH=$(mktemp -d)
  export VSDD_LOG_DIR="$SCRATCH"
  # Seed events dated today so the default 'daily' subcommand has something
  # to report on.
  TODAY=$(date +%Y-%m-%d)
  cat > "$SCRATCH/events-${TODAY}.jsonl" <<EOF
{"type":"hook.block","reason":"catastrophic_root","hook":"destructive-command-guard","matcher":"Bash","ts":"${TODAY}T10:00:00-0500","schema_version":1}
{"type":"hook.block","reason":"catastrophic_root","hook":"destructive-command-guard","matcher":"Bash","ts":"${TODAY}T10:05:00-0500","schema_version":1}
{"type":"hook.block","reason":"env_file_read_shell","hook":"protect-secrets","matcher":"Bash","ts":"${TODAY}T11:00:00-0500","schema_version":1}
{"type":"hook.block","reason":"subagent_empty_result","hook":"handoff-validator","matcher":"SubagentStop","severity":"warn","ts":"${TODAY}T13:00:00-0500","schema_version":1}
{"type":"hook.action","reason":"wave_merge_recorded","hook":"update-wave-state-on-merge","matcher":"SubagentStop","story_id":"S-1.01","wave":"wave-1","gate_transitioned":"True","ts":"${TODAY}T14:00:00-0500","schema_version":1}
{"type":"hook.block","reason":"pending_wave_gate_at_session_end","hook":"warn-pending-wave-gate","matcher":"Stop","severity":"warn","pending_waves":"wave-1","ts":"${TODAY}T15:00:00-0500","schema_version":1}
EOF
}

teardown() {
  rm -rf "$SCRATCH"
  unset VSDD_LOG_DIR
}

# ---------- Structural ----------

@test "factory-report: exists and executable" {
  [ -x "$TOOL" ]
}

@test "factory-report: passes syntax check" {
  bash -n "$TOOL"
}

@test "factory-report: help shows usage" {
  run "$TOOL" help
  [ "$status" -eq 0 ]
  [[ "$output" == *"Usage:"* ]]
}

# ---------- daily ----------

@test "daily: contains markdown H1 header" {
  run "$TOOL" daily
  [ "$status" -eq 0 ]
  [[ "$output" == *"# Factory event report"* ]]
}

@test "daily: reports correct totals" {
  run "$TOOL" daily
  [[ "$output" == *"| Total events | 6 |"* ]]
  [[ "$output" == *"| Blocks (hard) | 3 |"* ]]
  [[ "$output" == *"| Blocks (warn) | 2 |"* ]]
  [[ "$output" == *"| Actions | 1 |"* ]]
}

@test "daily: includes top block reasons table" {
  run "$TOOL" daily
  [[ "$output" == *"## Top block reasons"* ]]
  [[ "$output" == *"catastrophic_root"* ]]
}

@test "daily: includes hook activity table" {
  run "$TOOL" daily
  [[ "$output" == *"## Hook activity"* ]]
  [[ "$output" == *"destructive-command-guard"* ]]
}

@test "daily: includes wave merges section when present" {
  run "$TOOL" daily
  [[ "$output" == *"## Wave merges"* ]]
  [[ "$output" == *"S-1.01"* ]]
  [[ "$output" == *"wave-1"* ]]
}

@test "daily: includes session-end warnings section when present" {
  run "$TOOL" daily
  [[ "$output" == *"## Session-end gate warnings"* ]]
}

@test "daily --date to a day with no events produces empty report" {
  run "$TOOL" daily --date "2020-01-01"
  [ "$status" -eq 0 ]
  [[ "$output" == *"_No events in this range._"* ]]
}

# ---------- weekly ----------

@test "weekly: covers 7 days" {
  run "$TOOL" weekly
  [ "$status" -eq 0 ]
  [[ "$output" == *"7 day(s)"* ]]
}

@test "weekly: shows range in header" {
  run "$TOOL" weekly
  [[ "$output" == *"**Range:**"* ]]
}

# ---------- range ----------

@test "range: explicit from/to" {
  TODAY=$(date +%Y-%m-%d)
  run "$TOOL" range --from "$TODAY" --to "$TODAY"
  [ "$status" -eq 0 ]
  [[ "$output" == *"| Total events | 6 |"* ]]
}

@test "range: missing --from errors" {
  run "$TOOL" range --to "2026-04-22"
  [ "$status" -ne 0 ]
}

@test "range: missing --to errors" {
  run "$TOOL" range --from "2026-04-22"
  [ "$status" -ne 0 ]
}

@test "range: no events in window produces empty section" {
  run "$TOOL" range --from "2020-01-01" --to "2020-01-07"
  [ "$status" -eq 0 ]
  [[ "$output" == *"_No events in this range._"* ]]
}

# ---------- Empty state ----------

@test "daily: empty log dir produces clean empty report" {
  rm -f "$SCRATCH"/events-*.jsonl
  run "$TOOL" daily
  [ "$status" -eq 0 ]
  [[ "$output" == *"# Factory event report"* ]]
  [[ "$output" == *"| Total events | 0 |"* ]]
}
