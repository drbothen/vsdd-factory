#!/usr/bin/env bats
# policy-enforcement.bats — tests for policy 6, 7, 8 PostToolUse hooks
#
# Tests validate-subsystem-names.sh (Policy 6),
# validate-bc-title.sh (Policy 7), and
# validate-story-bc-sync.sh (Policy 8) against shared fixtures.

setup() {
  HOOKS="${BATS_TEST_DIRNAME}/../hooks"
  FIX="${BATS_TEST_DIRNAME}/fixtures/policy-enforcement"
}

# ===== Policy 6: validate-subsystem-names.sh =====

# -- BC files --

@test "P6: BC with correct subsystem passes" {
  INPUT=$(jq -nc --arg fp "$FIX/specs/behavioral-contracts/BC-2.01.001.md" '{tool_input: {file_path: $fp}}')
  run bash -c "echo '$INPUT' | '$HOOKS/validate-subsystem-names.sh' 2>&1"
  [ "$status" -eq 0 ]
}

@test "P6: BC with incorrect subsystem fails" {
  INPUT=$(jq -nc --arg fp "$FIX/specs/behavioral-contracts/BC-2.01.002-bad.md" '{tool_input: {file_path: $fp}}')
  run bash -c "echo '$INPUT' | '$HOOKS/validate-subsystem-names.sh' 2>&1"
  [ "$status" -eq 2 ]
  [[ "$output" == *"POLICY 6 VIOLATION"* ]]
  [[ "$output" == *"SS-99"* ]]
}

@test "P6: BC violation lists available SS-IDs" {
  INPUT=$(jq -nc --arg fp "$FIX/specs/behavioral-contracts/BC-2.01.002-bad.md" '{tool_input: {file_path: $fp}}')
  run bash -c "echo '$INPUT' | '$HOOKS/validate-subsystem-names.sh' 2>&1"
  [[ "$output" == *"SS-01"* ]]
  [[ "$output" == *"SS-02"* ]]
}

# -- Story files --

@test "P6: story with correct subsystems passes" {
  INPUT=$(jq -nc --arg fp "$FIX/stories/STORY-001-good.md" '{tool_input: {file_path: $fp}}')
  run bash -c "echo '$INPUT' | '$HOOKS/validate-subsystem-names.sh' 2>&1"
  [ "$status" -eq 0 ]
}

@test "P6: story with incorrect subsystem fails" {
  INPUT=$(jq -nc --arg fp "$FIX/stories/STORY-002-bad-subsystem.md" '{tool_input: {file_path: $fp}}')
  run bash -c "echo '$INPUT' | '$HOOKS/validate-subsystem-names.sh' 2>&1"
  [ "$status" -eq 2 ]
  [[ "$output" == *"SS-99"* ]]
}

# -- Edge cases --

@test "P6: non-BC/story file passes through" {
  INPUT='{"tool_input":{"file_path":"src/main.rs"}}'
  run bash -c "echo '$INPUT' | '$HOOKS/validate-subsystem-names.sh'"
  [ "$status" -eq 0 ]
}

@test "P6: missing ARCH-INDEX passes (architecture not produced yet)" {
  # Use a path with no ARCH-INDEX.md
  WORK=$(mktemp -d)
  mkdir -p "$WORK/specs/behavioral-contracts"
  printf -- '---\nsubsystem: Whatever\n---\n# BC-1.01.001: Test\n' > "$WORK/specs/behavioral-contracts/BC-1.01.001.md"
  INPUT=$(jq -nc --arg fp "$WORK/specs/behavioral-contracts/BC-1.01.001.md" '{tool_input: {file_path: $fp}}')
  run bash -c "echo '$INPUT' | '$HOOKS/validate-subsystem-names.sh'"
  [ "$status" -eq 0 ]
  rm -rf "$WORK"
}

@test "P6: hook is executable" {
  [ -x "$HOOKS/validate-subsystem-names.sh" ]
}

@test "P6: hook passes syntax check" {
  bash -n "$HOOKS/validate-subsystem-names.sh"
}

# ===== Policy 7: validate-bc-title.sh =====

@test "P7: BC with matching H1 and BC-INDEX title passes" {
  INPUT=$(jq -nc --arg fp "$FIX/specs/behavioral-contracts/BC-2.01.001.md" '{tool_input: {file_path: $fp}}')
  run bash -c "echo '$INPUT' | '$HOOKS/validate-bc-title.sh' 2>&1"
  [ "$status" -eq 0 ]
}

@test "P7: BC with mismatched H1 and BC-INDEX title fails" {
  INPUT=$(jq -nc --arg fp "$FIX/specs/behavioral-contracts/BC-2.01.003-title-mismatch.md" '{tool_input: {file_path: $fp}}')
  run bash -c "echo '$INPUT' | '$HOOKS/validate-bc-title.sh' 2>&1"
  [ "$status" -eq 2 ]
  [[ "$output" == *"POLICY 7 VIOLATION"* ]]
}

@test "P7: title mismatch shows both titles" {
  INPUT=$(jq -nc --arg fp "$FIX/specs/behavioral-contracts/BC-2.01.003-title-mismatch.md" '{tool_input: {file_path: $fp}}')
  run bash -c "echo '$INPUT' | '$HOOKS/validate-bc-title.sh' 2>&1"
  [[ "$output" == *"Data Cleansing and Normalization"* ]]
  [[ "$output" == *"Data Sanitization"* ]]
}

@test "P7: non-BC file passes through" {
  INPUT='{"tool_input":{"file_path":"src/main.rs"}}'
  run bash -c "echo '$INPUT' | '$HOOKS/validate-bc-title.sh'"
  [ "$status" -eq 0 ]
}

@test "P7: BC-INDEX itself passes through" {
  INPUT=$(jq -nc --arg fp "$FIX/specs/behavioral-contracts/BC-INDEX.md" '{tool_input: {file_path: $fp}}')
  run bash -c "echo '$INPUT' | '$HOOKS/validate-bc-title.sh'"
  [ "$status" -eq 0 ]
}

@test "P7: missing BC-INDEX passes (not produced yet)" {
  WORK=$(mktemp -d)
  mkdir -p "$WORK/behavioral-contracts"
  printf -- '---\nsubsystem: Test\n---\n# BC-1.01.001: Test Title\n' > "$WORK/behavioral-contracts/BC-1.01.001.md"
  INPUT=$(jq -nc --arg fp "$WORK/behavioral-contracts/BC-1.01.001.md" '{tool_input: {file_path: $fp}}')
  run bash -c "echo '$INPUT' | '$HOOKS/validate-bc-title.sh'"
  [ "$status" -eq 0 ]
  rm -rf "$WORK"
}

@test "P7: hook is executable" {
  [ -x "$HOOKS/validate-bc-title.sh" ]
}

@test "P7: hook passes syntax check" {
  bash -n "$HOOKS/validate-bc-title.sh"
}

# ===== Policy 8: validate-story-bc-sync.sh =====

@test "P8: story with all BCs synced passes" {
  INPUT=$(jq -nc --arg fp "$FIX/stories/STORY-001-good.md" '{tool_input: {file_path: $fp}}')
  run bash -c "echo '$INPUT' | '$HOOKS/validate-story-bc-sync.sh' 2>&1"
  [ "$status" -eq 0 ]
}

@test "P8: story with BC in frontmatter but missing from body fails" {
  INPUT=$(jq -nc --arg fp "$FIX/stories/STORY-003-bad-sync.md" '{tool_input: {file_path: $fp}}')
  run bash -c "echo '$INPUT' | '$HOOKS/validate-story-bc-sync.sh' 2>&1"
  [ "$status" -eq 2 ]
  [[ "$output" == *"POLICY 8 VIOLATION"* ]]
}

@test "P8: missing BC identified by ID" {
  INPUT=$(jq -nc --arg fp "$FIX/stories/STORY-003-bad-sync.md" '{tool_input: {file_path: $fp}}')
  run bash -c "echo '$INPUT' | '$HOOKS/validate-story-bc-sync.sh' 2>&1"
  [[ "$output" == *"BC-2.01.005"* ]]
}

@test "P8: non-story file passes through" {
  INPUT='{"tool_input":{"file_path":"src/main.rs"}}'
  run bash -c "echo '$INPUT' | '$HOOKS/validate-story-bc-sync.sh'"
  [ "$status" -eq 0 ]
}

@test "P8: STORY-INDEX passes through" {
  INPUT='{"tool_input":{"file_path":".factory/stories/STORY-INDEX.md"}}'
  run bash -c "echo '$INPUT' | '$HOOKS/validate-story-bc-sync.sh'"
  [ "$status" -eq 0 ]
}

@test "P8: story with no bcs: field passes (early creation)" {
  WORK=$(mktemp -d)
  mkdir -p "$WORK/stories"
  printf -- '---\nstory_id: STORY-099\n---\n# STORY-099: Draft\n' > "$WORK/stories/STORY-099.md"
  INPUT=$(jq -nc --arg fp "$WORK/stories/STORY-099.md" '{tool_input: {file_path: $fp}}')
  run bash -c "echo '$INPUT' | '$HOOKS/validate-story-bc-sync.sh'"
  [ "$status" -eq 0 ]
  rm -rf "$WORK"
}

@test "P8: detects drift with canonical behavioral_contracts: field" {
  WORK=$(mktemp -d)
  mkdir -p "$WORK/stories"
  cat > "$WORK/stories/STORY-099.md" << 'FIXTURE'
---
story_id: STORY-099
behavioral_contracts: [BC-1.01.001, BC-1.01.002]
---

# STORY-099: Test

## Behavioral Contracts

| BC | Title |
|----|-------|
| BC-1.01.001 | Test |

## Acceptance Criteria

### AC-001 (traces to BC-1.01.001)
Test.
FIXTURE
  INPUT=$(jq -nc --arg fp "$WORK/stories/STORY-099.md" '{tool_input: {file_path: $fp}}')
  run bash -c "echo '$INPUT' | '$HOOKS/validate-story-bc-sync.sh' 2>&1"
  [ "$status" -eq 2 ]
  [[ "$output" == *"BC-1.01.002"* ]]
  rm -rf "$WORK"
}

@test "P8: reads legacy bcs: field (functional compatibility)" {
  WORK=$(mktemp -d)
  mkdir -p "$WORK/stories"
  cat > "$WORK/stories/STORY-098.md" << 'FIXTURE'
---
story_id: STORY-098
bcs: [BC-1.01.001]
---

# STORY-098: Test

## Behavioral Contracts

| BC | Title |
|----|-------|
| BC-1.01.001 | Test |

## Acceptance Criteria

### AC-001 (traces to BC-1.01.001)
Test.
FIXTURE
  INPUT=$(jq -nc --arg fp "$WORK/stories/STORY-098.md" '{tool_input: {file_path: $fp}}')
  run bash -c "echo '$INPUT' | '$HOOKS/validate-story-bc-sync.sh' 2>&1"
  [ "$status" -eq 0 ]
  rm -rf "$WORK"
}

@test "P8: hook is executable" {
  [ -x "$HOOKS/validate-story-bc-sync.sh" ]
}

@test "P8: hook passes syntax check" {
  bash -n "$HOOKS/validate-story-bc-sync.sh"
}

# ===== hooks.json wiring =====

@test "hooks.json wires validate-subsystem-names" {
  jq -e '.hooks.PostToolUse[0].hooks[] | select(.command | contains("validate-subsystem-names"))' "${BATS_TEST_DIRNAME}/../hooks/hooks.json" >/dev/null
}

@test "hooks.json wires validate-bc-title" {
  jq -e '.hooks.PostToolUse[0].hooks[] | select(.command | contains("validate-bc-title"))' "${BATS_TEST_DIRNAME}/../hooks/hooks.json" >/dev/null
}

@test "hooks.json wires validate-story-bc-sync" {
  jq -e '.hooks.PostToolUse[0].hooks[] | select(.command | contains("validate-story-bc-sync"))' "${BATS_TEST_DIRNAME}/../hooks/hooks.json" >/dev/null
}
