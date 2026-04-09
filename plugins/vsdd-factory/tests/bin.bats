#!/usr/bin/env bats
# bin.bats — TAP tests for vsdd-factory bin helpers

setup() {
  PLUGIN_ROOT="$(cd "$BATS_TEST_DIRNAME/.." && pwd)"
  BIN="$PLUGIN_ROOT/bin"
  WORK="$(mktemp -d)"
  cd "$WORK"
}

teardown() {
  rm -rf "$WORK"
}

# ---------- lobster-parse ----------

@test "lobster-parse: parses brownfield workflow name" {
  run "$BIN/lobster-parse" "$PLUGIN_ROOT/workflows/brownfield.lobster" '.workflow.name'
  [ "$status" -eq 0 ]
  [[ "$output" == '"brownfield-vsdd"' ]]
}

@test "lobster-parse: all 15 workflow files parse" {
  fail=0
  for f in "$PLUGIN_ROOT"/workflows/*.lobster "$PLUGIN_ROOT"/workflows/phases/*.lobster; do
    if ! "$BIN/lobster-parse" "$f" '.workflow.name' >/dev/null; then
      echo "FAIL: $f"
      fail=$((fail+1))
    fi
  done
  [ "$fail" -eq 0 ]
}

@test "lobster-parse: reports missing file" {
  run "$BIN/lobster-parse" /nonexistent/foo.lobster
  [ "$status" -ne 0 ]
  [[ "$output" == *"file not found"* ]]
}

# ---------- research-cache ----------

@test "research-cache: stats on empty cache" {
  VSDD_RESEARCH_CACHE_DIR="$WORK/cache" run "$BIN/research-cache" stats
  [ "$status" -eq 0 ]
  [[ "$output" == *"entries=0"* ]]
}

@test "research-cache: put then get round-trip" {
  export VSDD_RESEARCH_CACHE_DIR="$WORK/cache"
  key=$("$BIN/research-cache" key "test query")
  echo '{"result":"ok"}' | "$BIN/research-cache" put "$key"
  run "$BIN/research-cache" has "$key"
  [ "$status" -eq 0 ]
  run "$BIN/research-cache" get "$key"
  [ "$status" -eq 0 ]
  [[ "$output" == *'"result":"ok"'* ]]
}

@test "research-cache: key is deterministic" {
  k1=$("$BIN/research-cache" key "same query")
  k2=$("$BIN/research-cache" key "same query")
  [ "$k1" = "$k2" ]
}

@test "research-cache: key normalizes whitespace" {
  k1=$("$BIN/research-cache" key "hello  world")
  k2=$("$BIN/research-cache" key "hello world")
  [ "$k1" = "$k2" ]
}

# ---------- multi-repo-scan ----------

@test "multi-repo-scan: reports zero for empty project" {
  VSDD_PROJECT_ROOT="$WORK" run "$BIN/multi-repo-scan" --count
  [ "$status" -eq 0 ]
  [ "$output" = "0" ]
}

@test "multi-repo-scan: detects a worktree" {
  mkdir -p "$WORK/.worktrees/STORY-001/.git"
  touch "$WORK/.worktrees/STORY-001/Cargo.toml"
  VSDD_PROJECT_ROOT="$WORK" run "$BIN/multi-repo-scan" --count
  [ "$status" -eq 0 ]
  [ "$output" = "1" ]
}

@test "multi-repo-scan: emits JSON with manifest type" {
  mkdir -p "$WORK/.worktrees/STORY-001/.git"
  touch "$WORK/.worktrees/STORY-001/Cargo.toml"
  VSDD_PROJECT_ROOT="$WORK" run "$BIN/multi-repo-scan"
  [ "$status" -eq 0 ]
  [[ "$output" == *"Cargo.toml"* ]]
  [[ "$output" == *'"count": 1'* ]]
}

# ---------- wave-state ----------

@test "wave-state: fails cleanly when state file absent" {
  VSDD_SPRINT_STATE="$WORK/missing.yaml" run "$BIN/wave-state" current
  [ "$status" -ne 0 ]
  [[ "$output" == *"not found"* ]]
}

@test "wave-state: reports current wave" {
  cat > "$WORK/state.yaml" <<EOF
current_wave: 2
waves:
  - number: 1
    stories: [STORY-001, STORY-002]
  - number: 2
    stories: [STORY-003]
stories:
  - id: STORY-001
    status: ready
  - id: STORY-002
    status: ready
  - id: STORY-003
    status: in_progress
EOF
  VSDD_SPRINT_STATE="$WORK/state.yaml" run "$BIN/wave-state" current
  [ "$status" -eq 0 ]
  [ "$output" = "2" ]
}

@test "wave-state: summary prints path and wave" {
  cat > "$WORK/state.yaml" <<EOF
current_wave: 1
waves:
  - number: 1
    stories: []
EOF
  VSDD_SPRINT_STATE="$WORK/state.yaml" run "$BIN/wave-state" summary
  [ "$status" -eq 0 ]
  [[ "$output" == *"wave=1"* ]]
}
