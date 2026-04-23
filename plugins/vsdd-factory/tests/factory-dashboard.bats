#!/usr/bin/env bats
# factory-dashboard.bats — tests for the pipeline dashboard script.

setup() {
  TOOL="${BATS_TEST_DIRNAME}/../bin/factory-dashboard"
  SCRATCH=$(mktemp -d)
  mkdir -p "$SCRATCH/.factory/logs"
  TODAY=$(date +%Y-%m-%d)
  # Cd into scratch so .factory/ resolves correctly
  cd "$SCRATCH" || exit 1
}

teardown() {
  cd / || true
  rm -rf "$SCRATCH"
}

_seed_state() {
  cat > "$SCRATCH/.factory/STATE.md" <<'EOF'
---
document_type: pipeline-state
project: demo-app
mode: greenfield
phase: 3
status: in_progress
current_step: Delivering wave-2 stories (pass 1)
current_cycle: v1.0.0-greenfield
---
# Pipeline State
body
EOF
}

_seed_wave_state() {
  cat > "$SCRATCH/.factory/wave-state.yaml" <<'EOF'
next_gate_required: wave-2
waves:
  wave-1:
    stories: [S-1.01, S-1.02]
    stories_merged: [S-1.01, S-1.02]
    gate_status: passed
  wave-2:
    stories: [S-2.01, S-2.02]
    stories_merged: [S-2.01, S-2.02]
    gate_status: pending
EOF
}

_seed_events() {
  cat > "$SCRATCH/.factory/logs/events-${TODAY}.jsonl" <<EOF
{"type":"hook.block","reason":"catastrophic_root","hook":"destructive-command-guard","matcher":"Bash","ts":"${TODAY}T10:00:00-0500","schema_version":1}
{"type":"hook.block","reason":"catastrophic_root","hook":"destructive-command-guard","matcher":"Bash","ts":"${TODAY}T10:05:00-0500","schema_version":1}
{"type":"hook.block","reason":"env_file_read_shell","hook":"protect-secrets","matcher":"Bash","ts":"${TODAY}T11:00:00-0500","schema_version":1}
{"type":"hook.block","reason":"pending_wave_gate_at_session_end","hook":"warn-pending-wave-gate","matcher":"Stop","severity":"warn","pending_waves":"wave-2","ts":"${TODAY}T15:00:00-0500","schema_version":1}
EOF
}

# ---------- Structural ----------

@test "factory-dashboard: exists and executable" {
  [ -x "$TOOL" ]
}

@test "factory-dashboard: passes syntax check" {
  bash -n "$TOOL"
}

@test "factory-dashboard: --help shows usage" {
  run "$TOOL" --help
  [ "$status" -eq 0 ]
  [[ "$output" == *"Usage:"* ]]
}

@test "factory-dashboard: unknown flag fails" {
  run "$TOOL" --bogus
  [ "$status" -ne 0 ]
}

# ---------- Empty state ----------

@test "empty state: produces dashboard with not-initialized notices" {
  run "$TOOL"
  [ "$status" -eq 0 ]
  [[ "$output" == *"# Factory dashboard"* ]]
  [[ "$output" == *"factory not initialized"* ]]
  [[ "$output" == *"no wave-based workflow"* ]] || [[ "$output" == *"no wave-based"* ]]
}

@test "empty state: Health checks section shows missing items" {
  run "$TOOL"
  [[ "$output" == *"## Health checks"* ]]
  [[ "$output" == *"✗ STATE.md"* ]]
  [[ "$output" == *"✗ wave-state.yaml"* ]]
}

# ---------- With STATE.md only ----------

@test "STATE only: project table populated" {
  _seed_state
  run "$TOOL"
  [ "$status" -eq 0 ]
  [[ "$output" == *"| project | \`demo-app\` |"* ]]
  [[ "$output" == *"| phase | 3 |"* ]]
  [[ "$output" == *"in_progress"* ]]
}

@test "STATE only: size warning threshold" {
  _seed_state
  # Grow STATE.md past 500 lines
  for i in $(seq 1 520); do echo "line $i"; done >> "$SCRATCH/.factory/STATE.md"
  run "$TOOL"
  [[ "$output" == *"exceeds 500 lines"* ]]
}

# ---------- With wave-state ----------

@test "wave-state: renders waves table" {
  _seed_state
  _seed_wave_state
  run "$TOOL"
  [[ "$output" == *"## Waves"* ]]
  [[ "$output" == *"| \`wave-1\` |"* ]]
  [[ "$output" == *"| \`wave-2\` |"* ]]
  # wave-2 is next gate — marked "yes"
  [[ "$output" == *"yes"* ]]
}

@test "wave-state: malformed YAML produces error notice, not crash" {
  _seed_state
  echo "!!! not valid yaml :" > "$SCRATCH/.factory/wave-state.yaml"
  run "$TOOL"
  [ "$status" -eq 0 ]
  # Either emits a parse error notice or reports empty waves; both are acceptable.
  [[ "$output" == *"## Waves"* ]]
}

# ---------- With events ----------

@test "events: recent activity section populated" {
  _seed_state
  _seed_events
  run "$TOOL"
  [[ "$output" == *"## Recent activity"* ]]
  [[ "$output" == *"Total events:"* ]]
  [[ "$output" == *"Blocks (hard):"* ]]
}

@test "events: top block reasons table appears" {
  _seed_state
  _seed_events
  run "$TOOL"
  [[ "$output" == *"### Top block reasons"* ]]
  [[ "$output" == *"catastrophic_root"* ]]
}

@test "events: pending wave gate reminder surfaces" {
  _seed_state
  _seed_events
  run "$TOOL"
  [[ "$output" == *"## Pending wave gates"* ]]
  [[ "$output" == *"wave-2"* ]]
}

@test "events: health checks all ✓ when everything present" {
  _seed_state
  _seed_wave_state
  _seed_events
  run "$TOOL"
  local miss
  miss=$(echo "$output" | grep -c '✗ ' || true)
  [ "$miss" -eq 0 ]
}

# ---------- Flags ----------

@test "--days N passes through to recent-activity lookback" {
  _seed_state
  _seed_events
  run "$TOOL" --days 30
  [ "$status" -eq 0 ]
  [[ "$output" == *"Recent activity (last 30 days)"* ]]
}

@test "--factory PATH redirects to alternate factory dir" {
  # Create a second factory location
  local ALT="$SCRATCH/alt-factory"
  mkdir -p "$ALT"
  cat > "$ALT/STATE.md" <<'EOF'
---
project: alt-project
phase: 1
status: complete
---
EOF
  run "$TOOL" --factory "$ALT"
  [[ "$output" == *"alt-project"* ]]
  [[ "$output" == *"phase | 1"* ]]
}
