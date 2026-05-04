#!/usr/bin/env bats
# template-compliance.bats — tests for validate-template-compliance PostToolUse hook
#
# Tests that the hook correctly identifies missing frontmatter fields and
# required sections when files are written to .factory/.

setup() {
  HOOK="${BATS_TEST_DIRNAME}/../hooks/validate-template-compliance.sh"
  FIX="${BATS_TEST_DIRNAME}/fixtures/template-compliance"
  # Set CLAUDE_PLUGIN_ROOT so the hook can find templates
  export CLAUDE_PLUGIN_ROOT="${BATS_TEST_DIRNAME}/.."
}

# Helper: simulate a Write PostToolUse with a .factory/ path
_run_hook() {
  local file="$1"
  local factory_path="$2"
  INPUT=$(jq -nc --arg fp "$factory_path" '{tool_input: {file_path: $fp}}')
  # The hook reads the actual file at the path, so we need to either
  # use a real path or symlink. For testing, we pass the fixture path
  # but disguised as a .factory/ path. The hook checks the actual file
  # content at FILE_PATH, so we need the real fixture path.
  # Workaround: set FILE_PATH to the fixture but pretend it's .factory/
  # Actually the hook reads FILE_PATH from JSON, so we need the JSON
  # path to match .factory/* AND the file to exist at that path.
  # Solution: create a temp .factory/ structure pointing to fixtures.
  run bash -c "echo '$INPUT' | CLAUDE_PLUGIN_ROOT='$CLAUDE_PLUGIN_ROOT' '$HOOK' 2>&1"
}

# ---------- Non-.factory files: hook is a no-op ----------

@test "template-compliance: non-.factory file passes through" {
  INPUT='{"tool_input":{"file_path":"src/main.rs"}}'
  run bash -c "echo '$INPUT' | CLAUDE_PLUGIN_ROOT='$CLAUDE_PLUGIN_ROOT' '$HOOK' 2>&1"
  [ "$status" -eq 0 ]
}

# ---------- INDEX files: skipped ----------

@test "template-compliance: INDEX files are skipped" {
  INPUT='{"tool_input":{"file_path":"/project/.factory/specs/behavioral-contracts/BC-INDEX.md"}}'
  run bash -c "echo '$INPUT' | CLAUDE_PLUGIN_ROOT='$CLAUDE_PLUGIN_ROOT' '$HOOK' 2>&1"
  [ "$status" -eq 0 ]
}

@test "template-compliance: STORY-INDEX is skipped" {
  INPUT='{"tool_input":{"file_path":"/project/.factory/stories/STORY-INDEX.md"}}'
  run bash -c "echo '$INPUT' | CLAUDE_PLUGIN_ROOT='$CLAUDE_PLUGIN_ROOT' '$HOOK' 2>&1"
  [ "$status" -eq 0 ]
}

# ---------- Good BC: should pass ----------

@test "template-compliance: compliant BC passes" {
  # Create a temp .factory structure with the good BC
  WORK=$(mktemp -d)
  mkdir -p "$WORK/.factory/specs/behavioral-contracts"
  cp "$FIX/good-bc.md" "$WORK/.factory/specs/behavioral-contracts/BC-2.01.001.md"
  INPUT=$(jq -nc --arg fp "$WORK/.factory/specs/behavioral-contracts/BC-2.01.001.md" '{tool_input: {file_path: $fp}}')
  run bash -c "echo '$INPUT' | CLAUDE_PLUGIN_ROOT='$CLAUDE_PLUGIN_ROOT' '$HOOK' 2>&1"
  [ "$status" -eq 0 ]
  rm -rf "$WORK"
}

# ---------- Bad BC: missing fields and sections ----------

@test "template-compliance: non-compliant BC warns" {
  WORK=$(mktemp -d)
  mkdir -p "$WORK/.factory/specs/behavioral-contracts"
  cp "$FIX/bad-bc.md" "$WORK/.factory/specs/behavioral-contracts/BC-2.01.002.md"
  INPUT=$(jq -nc --arg fp "$WORK/.factory/specs/behavioral-contracts/BC-2.01.002.md" '{tool_input: {file_path: $fp}}')
  run bash -c "echo '$INPUT' | CLAUDE_PLUGIN_ROOT='$CLAUDE_PLUGIN_ROOT' '$HOOK' 2>&1"
  [ "$status" -eq 2 ]
  [[ "$output" == *"TEMPLATE COMPLIANCE WARNING"* ]]
}

@test "template-compliance: bad BC reports missing frontmatter" {
  WORK=$(mktemp -d)
  mkdir -p "$WORK/.factory/specs/behavioral-contracts"
  cp "$FIX/bad-bc.md" "$WORK/.factory/specs/behavioral-contracts/BC-2.01.002.md"
  INPUT=$(jq -nc --arg fp "$WORK/.factory/specs/behavioral-contracts/BC-2.01.002.md" '{tool_input: {file_path: $fp}}')
  run bash -c "echo '$INPUT' | CLAUDE_PLUGIN_ROOT='$CLAUDE_PLUGIN_ROOT' '$HOOK' 2>&1"
  [[ "$output" == *"Frontmatter"* ]]
  [[ "$output" == *"Missing"* ]]
}

@test "template-compliance: bad BC reports missing sections" {
  WORK=$(mktemp -d)
  mkdir -p "$WORK/.factory/specs/behavioral-contracts"
  cp "$FIX/bad-bc.md" "$WORK/.factory/specs/behavioral-contracts/BC-2.01.002.md"
  INPUT=$(jq -nc --arg fp "$WORK/.factory/specs/behavioral-contracts/BC-2.01.002.md" '{tool_input: {file_path: $fp}}')
  run bash -c "echo '$INPUT' | CLAUDE_PLUGIN_ROOT='$CLAUDE_PLUGIN_ROOT' '$HOOK' 2>&1"
  [[ "$output" == *"Sections missing"* ]]
  # Should report missing Edge Cases, Canonical Test Vectors, Verification Properties, Traceability
  [[ "$output" == *"Canonical Test Vectors"* ]] || [[ "$output" == *"Traceability"* ]]
}

# ---------- Bad story: wrong schema ----------

@test "template-compliance: non-compliant story warns" {
  WORK=$(mktemp -d)
  mkdir -p "$WORK/.factory/stories"
  cp "$FIX/bad-story.md" "$WORK/.factory/stories/STORY-001.md"
  INPUT=$(jq -nc --arg fp "$WORK/.factory/stories/STORY-001.md" '{tool_input: {file_path: $fp}}')
  run bash -c "echo '$INPUT' | CLAUDE_PLUGIN_ROOT='$CLAUDE_PLUGIN_ROOT' '$HOOK' 2>&1"
  [ "$status" -eq 2 ]
  [[ "$output" == *"TEMPLATE COMPLIANCE WARNING"* ]]
}

@test "template-compliance: bad story reports missing document_type" {
  WORK=$(mktemp -d)
  mkdir -p "$WORK/.factory/stories"
  cp "$FIX/bad-story.md" "$WORK/.factory/stories/STORY-001.md"
  INPUT=$(jq -nc --arg fp "$WORK/.factory/stories/STORY-001.md" '{tool_input: {file_path: $fp}}')
  run bash -c "echo '$INPUT' | CLAUDE_PLUGIN_ROOT='$CLAUDE_PLUGIN_ROOT' '$HOOK' 2>&1"
  [[ "$output" == *"document_type"* ]]
}

# ---------- Holdout with no frontmatter ----------

@test "template-compliance: holdout with no frontmatter warns via path match" {
  WORK=$(mktemp -d)
  mkdir -p "$WORK/.factory/holdout-scenarios"
  cp "$FIX/no-frontmatter.md" "$WORK/.factory/holdout-scenarios/HS-001.md"
  INPUT=$(jq -nc --arg fp "$WORK/.factory/holdout-scenarios/HS-001.md" '{tool_input: {file_path: $fp}}')
  run bash -c "echo '$INPUT' | CLAUDE_PLUGIN_ROOT='$CLAUDE_PLUGIN_ROOT' '$HOOK' 2>&1"
  [ "$status" -eq 2 ]
  [[ "$output" == *"TEMPLATE COMPLIANCE WARNING"* ]]
}

# ---------- Suggests conform-to-template ----------

@test "template-compliance: warning suggests conform-to-template skill" {
  WORK=$(mktemp -d)
  mkdir -p "$WORK/.factory/specs/behavioral-contracts"
  cp "$FIX/bad-bc.md" "$WORK/.factory/specs/behavioral-contracts/BC-2.01.002.md"
  INPUT=$(jq -nc --arg fp "$WORK/.factory/specs/behavioral-contracts/BC-2.01.002.md" '{tool_input: {file_path: $fp}}')
  run bash -c "echo '$INPUT' | CLAUDE_PLUGIN_ROOT='$CLAUDE_PLUGIN_ROOT' '$HOOK' 2>&1"
  [[ "$output" == *"conform-to-template"* ]]
}

# ---------- Structural ----------

@test "template-compliance: hook is executable" {
  [ -x "$HOOK" ]
}

@test "template-compliance: hook passes syntax check" {
  bash -n "$HOOK"
}

@test "template-compliance: registry wires validate-template-compliance" {
  load "${BATS_TEST_DIRNAME}/helpers/registry.bash"
  registry_has_hook "validate-template-compliance" "PostToolUse"
}
