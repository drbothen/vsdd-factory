#!/usr/bin/env bats
# pr-lifecycle-hooks.bats — tests for PR lifecycle enforcement hooks:
#   validate-pr-description-completeness.sh (PostToolUse on Write)
#   validate-pr-merge-prerequisites.sh (PreToolUse on Agent)
# NOTE: validate-pr-review-posted.sh was ported to native WASM (W-15);
# its bats tests were removed.
# NOTE: block-ai-attribution.sh was ported to native WASM (W-15);
# its bats tests were removed.

setup() {
  PLUGIN_ROOT="$(cd "$BATS_TEST_DIRNAME/.." && pwd)"
  HOOKS="$PLUGIN_ROOT/hooks"
  WORK=$(mktemp -d)
  mkdir -p "$WORK/.factory/code-delivery/STORY-001"
}

teardown() {
  rm -rf "$WORK"
}

_run_posttool_write() {
  local hook="$1"
  local file="$2"
  INPUT=$(jq -nc --arg fp "$file" '{tool_input: {file_path: $fp}}')
  run bash -c "echo '$INPUT' | '$HOOKS/$hook' 2>&1"
}

_run_pretool_agent() {
  local subagent="$1"
  local prompt="$2"
  INPUT=$(jq -nc --arg s "$subagent" --arg p "$prompt" '{tool_name: "Agent", tool_input: {subagent_type: $s, prompt: $p}}')
  run bash -c "cd '$WORK' && echo '$INPUT' | '$HOOKS/validate-pr-merge-prerequisites.sh' 2>&1"
}

# ========================================================================
# Syntax and wiring
# ========================================================================

@test "pr-description-completeness: passes syntax check" {
  run bash -n "$HOOKS/validate-pr-description-completeness.sh"
  [ "$status" -eq 0 ]
}

@test "pr-merge-prerequisites: passes syntax check" {
  run bash -n "$HOOKS/validate-pr-merge-prerequisites.sh"
  [ "$status" -eq 0 ]
}

@test "hooks.json wires pr-description-completeness under PostToolUse" {
  jq -e '.hooks.PostToolUse[] | .hooks[] | select(.command | contains("validate-pr-description-completeness"))' "$PLUGIN_ROOT/hooks/hooks.json" >/dev/null
}

@test "hooks.json wires pr-merge-prerequisites under PreToolUse Agent" {
  jq -e '.hooks.PreToolUse[] | select(.matcher == "Agent") | .hooks[] | select(.command | contains("validate-pr-merge-prerequisites"))' "$PLUGIN_ROOT/hooks/hooks.json" >/dev/null
}

# ========================================================================
# validate-pr-description-completeness
# ========================================================================

@test "pr-description: passes complete description" {
  cat > "$WORK/.factory/code-delivery/STORY-001/pr-description.md" << 'EOF'
# [S-0.01] Test Story

## Architecture Changes
Component diagram here.

## Story Dependencies
No deps.

## Spec Traceability
BC-2.01.001 -> AC-1 -> test_auth

## Test Evidence
5/5 passing, 90% coverage

## Demo Evidence
![demo](docs/demo-evidence/S-0.01/AC-001.gif)

## Pre-Merge Checklist
- [x] Tests pass
- [x] Demo recorded
EOF
  _run_posttool_write validate-pr-description-completeness.sh "$WORK/.factory/code-delivery/STORY-001/pr-description.md"
  [ "$status" -eq 0 ]
}

@test "pr-description: blocks missing Architecture Changes section" {
  cat > "$WORK/.factory/code-delivery/STORY-001/pr-description.md" << 'EOF'
# [S-0.01] Test Story

## Story Dependencies
No deps.

## Spec Traceability
chain here

## Test Evidence
passing

## Demo Evidence
recorded

## Pre-Merge Checklist
done
EOF
  _run_posttool_write validate-pr-description-completeness.sh "$WORK/.factory/code-delivery/STORY-001/pr-description.md"
  [ "$status" -eq 2 ]
  [[ "$output" == *"Architecture Changes"* ]]
}

@test "pr-description: blocks missing multiple sections" {
  cat > "$WORK/.factory/code-delivery/STORY-001/pr-description.md" << 'EOF'
# [S-0.01] Test Story

## Architecture Changes
done

## Pre-Merge Checklist
done
EOF
  _run_posttool_write validate-pr-description-completeness.sh "$WORK/.factory/code-delivery/STORY-001/pr-description.md"
  [ "$status" -eq 2 ]
  [[ "$output" == *"Story Dependencies"* ]]
  [[ "$output" == *"Test Evidence"* ]]
}

@test "pr-description: blocks unresolved template placeholders" {
  cat > "$WORK/.factory/code-delivery/STORY-001/pr-description.md" << 'EOF'
# [{story_id}] {story_title}

## Architecture Changes
{component_A} calls {component_B}

## Story Dependencies
none

## Spec Traceability
chain

## Test Evidence
{pass_count}/{total_count}

## Demo Evidence
recorded

## Pre-Merge Checklist
done
EOF
  _run_posttool_write validate-pr-description-completeness.sh "$WORK/.factory/code-delivery/STORY-001/pr-description.md"
  [ "$status" -eq 2 ]
  [[ "$output" == *"placeholder"* ]]
  [[ "$output" == *"{story_id}"* ]]
}

@test "pr-description: ignores non-pr-description files" {
  mkdir -p "$WORK/.factory/specs"
  echo "# Not a PR description" > "$WORK/.factory/specs/test.md"
  _run_posttool_write validate-pr-description-completeness.sh "$WORK/.factory/specs/test.md"
  [ "$status" -eq 0 ]
}

@test "pr-description: ignores pr-review.md" {
  echo "# Review findings" > "$WORK/.factory/code-delivery/STORY-001/pr-review.md"
  _run_posttool_write validate-pr-description-completeness.sh "$WORK/.factory/code-delivery/STORY-001/pr-review.md"
  [ "$status" -eq 0 ]
}

# ========================================================================
# validate-pr-merge-prerequisites
# ========================================================================

@test "pr-merge-prerequisites: passes when all evidence files exist" {
  echo "# Description" > "$WORK/.factory/code-delivery/STORY-001/pr-description.md"
  echo "# Review" > "$WORK/.factory/code-delivery/STORY-001/pr-review.md"
  echo "# Security" > "$WORK/.factory/code-delivery/STORY-001/security-review.md"
  _run_pretool_agent "vsdd-factory:github-ops" "cd $WORK && gh pr merge 42 --squash --delete-branch for STORY-001"
  [ "$status" -eq 0 ]
}

@test "pr-merge-prerequisites: blocks when pr-description.md missing" {
  echo "# Review" > "$WORK/.factory/code-delivery/STORY-001/pr-review.md"
  echo "# Security" > "$WORK/.factory/code-delivery/STORY-001/security-review.md"
  _run_pretool_agent "vsdd-factory:github-ops" "cd $WORK && gh pr merge 42 --squash for STORY-001"
  [ "$status" -eq 2 ]
  [[ "$output" == *"pr-description.md"* ]]
}

@test "pr-merge-prerequisites: blocks when pr-review.md missing" {
  echo "# Description" > "$WORK/.factory/code-delivery/STORY-001/pr-description.md"
  echo "# Security" > "$WORK/.factory/code-delivery/STORY-001/security-review.md"
  _run_pretool_agent "vsdd-factory:github-ops" "cd $WORK && gh pr merge 42 --squash for STORY-001"
  [ "$status" -eq 2 ]
  [[ "$output" == *"pr-review.md"* ]]
}

@test "pr-merge-prerequisites: blocks when security-review.md missing" {
  echo "# Description" > "$WORK/.factory/code-delivery/STORY-001/pr-description.md"
  echo "# Review" > "$WORK/.factory/code-delivery/STORY-001/pr-review.md"
  _run_pretool_agent "vsdd-factory:github-ops" "cd $WORK && gh pr merge 42 --squash for STORY-001"
  [ "$status" -eq 2 ]
  [[ "$output" == *"security-review.md"* ]]
}

@test "pr-merge-prerequisites: passes security check when description says no findings" {
  cat > "$WORK/.factory/code-delivery/STORY-001/pr-description.md" << 'EOF'
# Description
## Security Review
Security review: no findings. All clear.
EOF
  echo "# Review" > "$WORK/.factory/code-delivery/STORY-001/pr-review.md"
  _run_pretool_agent "vsdd-factory:github-ops" "cd $WORK && gh pr merge 42 --squash for STORY-001"
  [ "$status" -eq 0 ]
}

@test "pr-merge-prerequisites: ignores non-merge dispatches" {
  _run_pretool_agent "vsdd-factory:github-ops" "cd $WORK && gh pr create --title test for STORY-001"
  [ "$status" -eq 0 ]
}

@test "pr-merge-prerequisites: ignores non-github-ops agents" {
  _run_pretool_agent "vsdd-factory:implementer" "cd $WORK && Implement STORY-001"
  [ "$status" -eq 0 ]
}

@test "pr-merge-prerequisites: reports all missing files in one message" {
  # No evidence files at all
  _run_pretool_agent "vsdd-factory:github-ops" "cd $WORK && gh pr merge 42 --squash for STORY-001"
  [ "$status" -eq 2 ]
  [[ "$output" == *"pr-description.md"* ]]
  [[ "$output" == *"pr-review.md"* ]]
  [[ "$output" == *"security-review.md"* ]]
}

@test "pr-merge-prerequisites: warns when delivery dir not found" {
  _run_pretool_agent "vsdd-factory:github-ops" "cd $WORK && gh pr merge 42 --squash for STORY-999"
  [ "$status" -eq 0 ]
  [[ "$output" == *"WARNING"* ]]
}

