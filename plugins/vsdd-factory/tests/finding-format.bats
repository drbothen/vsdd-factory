#!/usr/bin/env bats
# finding-format.bats — tests for validate-finding-format PostToolUse hook
#
# Validates adversarial finding and fix file ID formats.

setup() {
  HOOK="${BATS_TEST_DIRNAME}/../hooks/validate-finding-format.sh"
  WORK=$(mktemp -d)
  mkdir -p "$WORK/.factory/cycles/v1/adversarial-reviews"
}

teardown() {
  rm -rf "$WORK"
}

_write_and_check() {
  local file="$1"
  local content="$2"
  echo "$content" > "$file"
  INPUT=$(jq -nc --arg fp "$file" '{tool_input: {file_path: $fp}}')
  run bash -c "echo '$INPUT' | '$HOOK' 2>&1"
}

# ---------- Current format: passes ----------

@test "finding-format: current ADV format passes" {
  _write_and_check "$WORK/.factory/cycles/v1/adversarial-reviews/pass-1.md" \
    "# Pass 1
## ADV-P1CONV-P01-CRIT-001: Finding title
Some description."
  [ "$status" -eq 0 ]
}

@test "finding-format: current FIX format passes" {
  _write_and_check "$WORK/.factory/FIX-P4-001.md" \
    "# FIX-P4-001: Fix BC timeout
## Source Finding
ADV-P1CONV-P03-HIGH-002"
  [ "$status" -eq 0 ]
}

# ---------- Legacy ADV-NNN: blocked ----------

@test "finding-format: legacy ADV-NNN blocked" {
  _write_and_check "$WORK/.factory/cycles/v1/adversarial-reviews/pass-1.md" \
    "# Pass 1
## ADV-001: Finding title
Some description."
  [ "$status" -eq 2 ]
  [[ "$output" == *"Legacy finding ID"* ]]
  [[ "$output" == *"ADV-001"* ]]
}

# ---------- Legacy ADV-P[N]-NNN: blocked ----------

@test "finding-format: legacy ADV-P[N]-NNN blocked" {
  _write_and_check "$WORK/.factory/cycles/v1/adversarial-reviews/pass-2.md" \
    "# Pass 2
## ADV-P2-001: Cross-doc drift
Some description."
  [ "$status" -eq 2 ]
  [[ "$output" == *"Legacy finding ID"* ]]
  [[ "$output" == *"ADV-P2-001"* ]]
}

# ---------- Legacy STORY-NNN-FIX: blocked ----------

@test "finding-format: legacy STORY-NNN-FIX blocked" {
  _write_and_check "$WORK/.factory/FIX-P4-001.md" \
    "# Fix
Source: STORY-005-FIX-001"
  [ "$status" -eq 2 ]
  [[ "$output" == *"Legacy fix ID"* ]]
  [[ "$output" == *"STORY-005-FIX-001"* ]]
}

# ---------- Non-finding files: skipped ----------

@test "finding-format: non-.factory file passes through" {
  INPUT='{"tool_input":{"file_path":"src/main.rs"}}'
  run bash -c "echo '$INPUT' | '$HOOK' 2>&1"
  [ "$status" -eq 0 ]
}

@test "finding-format: non-finding .factory file passes through" {
  echo "# Story" > "$WORK/.factory/stories-STORY-001.md"
  INPUT=$(jq -nc --arg fp "$WORK/.factory/stories-STORY-001.md" '{tool_input: {file_path: $fp}}')
  run bash -c "echo '$INPUT' | '$HOOK' 2>&1"
  [ "$status" -eq 0 ]
}

# ---------- Structural ----------

@test "finding-format: hook is executable" {
  [ -x "$HOOK" ]
}

@test "finding-format: hook passes syntax check" {
  bash -n "$HOOK"
}

@test "finding-format: registry wires validate-finding-format" {
  load "${BATS_TEST_DIRNAME}/helpers/registry.bash"
  registry_has_hook "validate-finding-format" "PostToolUse"
}

# ---------- Edge cases ----------

@test "finding-format: empty file passes" {
  touch "$WORK/.factory/cycles/v1/adversarial-reviews/pass-1.md"
  INPUT=$(jq -nc --arg fp "$WORK/.factory/cycles/v1/adversarial-reviews/pass-1.md" '{tool_input: {file_path: $fp}}')
  run bash -c "echo '$INPUT' | '$HOOK' 2>&1"
  [ "$status" -eq 0 ]
}

@test "finding-format: mixed current and legacy in same file flags legacy only" {
  _write_and_check "$WORK/.factory/cycles/v1/adversarial-reviews/pass-3.md" \
    "# Pass 3
## ADV-P1CONV-P03-CRIT-001: Good finding
## ADV-003: Bad finding"
  [ "$status" -eq 2 ]
  [[ "$output" == *"ADV-003"* ]]
}
