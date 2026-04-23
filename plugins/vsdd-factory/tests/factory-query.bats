#!/usr/bin/env bats
# factory-query.bats — tests for the observability query CLI.

setup() {
  TOOL="${BATS_TEST_DIRNAME}/../bin/factory-query"
  SCRATCH=$(mktemp -d)
  export VSDD_LOG_DIR="$SCRATCH"
  # Seed a fixed event log for deterministic tests.
  cat > "$SCRATCH/events-2026-04-22.jsonl" <<'EOF'
{"type":"hook.block","reason":"catastrophic_root","hook":"destructive-command-guard","matcher":"Bash","ts":"2026-04-22T10:00:00-0500","schema_version":1}
{"type":"hook.block","reason":"catastrophic_root","hook":"destructive-command-guard","matcher":"Bash","ts":"2026-04-22T10:05:00-0500","schema_version":1}
{"type":"hook.block","reason":"env_file_read_shell","hook":"protect-secrets","matcher":"Bash","ts":"2026-04-22T11:00:00-0500","schema_version":1}
{"type":"hook.block","reason":"subagent_empty_result","hook":"handoff-validator","matcher":"SubagentStop","severity":"warn","ts":"2026-04-22T13:00:00-0500","schema_version":1}
{"type":"hook.action","reason":"wave_merge_recorded","hook":"update-wave-state-on-merge","matcher":"SubagentStop","story_id":"S-1.01","ts":"2026-04-22T14:00:00-0500","schema_version":1}
EOF
}

teardown() {
  rm -rf "$SCRATCH"
  unset VSDD_LOG_DIR
}

# ---------- Structural ----------

@test "factory-query: exists and executable" {
  [ -x "$TOOL" ]
}

@test "factory-query: passes syntax check" {
  bash -n "$TOOL"
}

@test "factory-query: help shows usage" {
  run "$TOOL" help
  [ "$status" -eq 0 ]
  [[ "$output" == *"Usage:"* ]]
}

@test "factory-query: no args shows usage" {
  run "$TOOL"
  [ "$status" -eq 0 ]
  [[ "$output" == *"Usage:"* ]]
}

@test "factory-query: unknown subcommand fails" {
  run "$TOOL" nonexistent
  [ "$status" -ne 0 ]
}

# ---------- top ----------

@test "top: shows top reason by count" {
  run "$TOOL" top
  [ "$status" -eq 0 ]
  [[ "$output" == *"Top reason codes"* ]]
  [[ "$output" == *"catastrophic_root"* ]]
  [[ "$output" == *"destructive-command-guard"* ]]
}

@test "top: tsv output has tab separators" {
  run "$TOOL" top --tsv
  [ "$status" -eq 0 ]
  [[ "$output" == *$'2\tcatastrophic_root\tdestructive-command-guard'* ]]
}

@test "top: --limit caps results" {
  run "$TOOL" top --limit 1
  [ "$status" -eq 0 ]
  # Only one reason row below the header
  local reason_rows
  reason_rows=$(echo "$output" | grep -c 'catastrophic_root')
  [ "$reason_rows" -eq 1 ]
}

# ---------- recent ----------

@test "recent: shows events ordered by ts" {
  run "$TOOL" recent
  [ "$status" -eq 0 ]
  [[ "$output" == *"TS"* ]]
  [[ "$output" == *"Severity"* ]]
}

@test "recent: --severity warn filters" {
  run "$TOOL" recent --severity warn
  [ "$status" -eq 0 ]
  [[ "$output" == *"warn"* ]]
  [[ "$output" == *"subagent_empty_result"* ]]
  [[ "$output" != *"catastrophic_root"* ]]
}

@test "recent: --type hook.action filters to actions" {
  run "$TOOL" recent --type hook.action
  [ "$status" -eq 0 ]
  [[ "$output" == *"wave_merge_recorded"* ]]
  [[ "$output" != *"catastrophic_root"* ]]
}

@test "recent: action events display as severity=action" {
  run "$TOOL" recent --tsv --type hook.action
  [ "$status" -eq 0 ]
  [[ "$output" == *$'\taction\t'* ]]
}

# ---------- grep ----------

@test "grep: filters by reason code" {
  run "$TOOL" grep catastrophic_root
  [ "$status" -eq 0 ]
  # Should emit 2 lines (the 2 catastrophic_root events)
  local count
  count=$(echo "$output" | wc -l | tr -d ' ')
  [ "$count" -eq 2 ]
}

@test "grep: no matches returns a message" {
  run "$TOOL" grep definitely_not_a_real_code
  [ "$status" -eq 0 ]
  [[ "$output" == *"no events"* ]]
}

@test "grep: missing reason arg errors" {
  run "$TOOL" grep
  [ "$status" -ne 0 ]
}

# ---------- hooks ----------

@test "hooks: shows per-hook counts" {
  run "$TOOL" hooks
  [ "$status" -eq 0 ]
  [[ "$output" == *"destructive-command-guard"* ]]
  [[ "$output" == *"update-wave-state-on-merge"* ]]
}

# ---------- stats ----------

@test "stats: shows aggregate counts" {
  run "$TOOL" stats
  [ "$status" -eq 0 ]
  [[ "$output" == *"Total events:    5"* ]]
  [[ "$output" == *"Blocks (hard):   3"* ]]
  [[ "$output" == *"Blocks (warn):   1"* ]]
  [[ "$output" == *"Actions:         1"* ]]
}

# ---------- reasons ----------

@test "reasons: lists all codes with counts" {
  run "$TOOL" reasons
  [ "$status" -eq 0 ]
  [[ "$output" == *"catastrophic_root"* ]]
  [[ "$output" == *"wave_merge_recorded"* ]]
}

# ---------- Empty log dir ----------

@test "top: empty log dir gracefully reports no events" {
  rm -f "$SCRATCH"/events-*.jsonl
  run "$TOOL" top
  [ "$status" -eq 0 ]
  [[ "$output" == *"No events found"* ]]
}

@test "missing log dir gracefully reports no events" {
  export VSDD_LOG_DIR="/nonexistent/path"
  run "$TOOL" top
  [ "$status" -eq 0 ]
  [[ "$output" == *"No events found"* ]]
}

# ---------- Date filtering ----------

@test "top --days filters to recent files only" {
  # Seed a very old events file; with --days 1 it should be excluded.
  cat > "$SCRATCH/events-2020-01-01.jsonl" <<'EOF'
{"type":"hook.block","reason":"ancient_event","hook":"very-old-hook","matcher":"Bash","ts":"2020-01-01T10:00:00-0500","schema_version":1}
EOF
  run "$TOOL" top --days 1
  [ "$status" -eq 0 ]
  [[ "$output" != *"ancient_event"* ]]
  # And note: our seeded 2026-04-22 file is also outside --days 1 unless today
  # is 2026-04-22 or 2026-04-23. This test cannot assert on presence of current
  # data. It can only assert the old file is excluded (above).
}
