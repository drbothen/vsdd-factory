#!/usr/bin/env bats
# hook-robustness.bats — tests for hook behavior on malformed/unexpected inputs
#
# Verifies all 7 enforcement hooks handle gracefully:
# - Empty JSON input
# - Missing tool_input field
# - Nonexistent file paths
# - Empty files (0 bytes)
# - Files with no/malformed frontmatter
# - Error output contract (POLICY N VIOLATION + BLOCKED)

setup() {
  HOOKS="${BATS_TEST_DIRNAME}/../hooks"
  WORK=$(mktemp -d)
}

teardown() {
  rm -rf "$WORK"
}

# ===== Helper: run a hook with given JSON input =====
_run_hook() {
  local hook="$1"
  local input="$2"
  run bash -c "echo '$input' | '$HOOKS/$hook' 2>&1"
}

# ===== Empty / missing JSON input =====
# All hooks should exit 0 (allow) when given empty or malformed JSON

@test "robustness: destructive-command-guard handles empty JSON" {
  _run_hook "destructive-command-guard.sh" '{}'
  [ "$status" -eq 0 ]
}

@test "robustness: verify-git-push handles empty JSON" {
  _run_hook "verify-git-push.sh" '{}'
  [ "$status" -eq 0 ]
}

@test "robustness: factory-branch-guard handles empty JSON" {
  _run_hook "factory-branch-guard.sh" '{}'
  [ "$status" -eq 0 ]
}

@test "robustness: validate-vp-consistency handles empty JSON" {
  _run_hook "validate-vp-consistency.sh" '{}'
  [ "$status" -eq 0 ]
}

@test "robustness: validate-subsystem-names handles empty JSON" {
  _run_hook "validate-subsystem-names.sh" '{}'
  [ "$status" -eq 0 ]
}

@test "robustness: validate-bc-title handles empty JSON" {
  _run_hook "validate-bc-title.sh" '{}'
  [ "$status" -eq 0 ]
}

@test "robustness: validate-story-bc-sync handles empty JSON" {
  _run_hook "validate-story-bc-sync.sh" '{}'
  [ "$status" -eq 0 ]
}

# ===== Missing tool_input fields =====

@test "robustness: destructive-command-guard handles missing command" {
  _run_hook "destructive-command-guard.sh" '{"tool_input":{}}'
  [ "$status" -eq 0 ]
}

@test "robustness: verify-git-push handles missing command" {
  _run_hook "verify-git-push.sh" '{"tool_input":{}}'
  [ "$status" -eq 0 ]
}

@test "robustness: factory-branch-guard handles missing file_path" {
  _run_hook "factory-branch-guard.sh" '{"tool_input":{}}'
  [ "$status" -eq 0 ]
}

@test "robustness: validate-subsystem-names handles missing file_path" {
  _run_hook "validate-subsystem-names.sh" '{"tool_input":{}}'
  [ "$status" -eq 0 ]
}

@test "robustness: validate-bc-title handles missing file_path" {
  _run_hook "validate-bc-title.sh" '{"tool_input":{}}'
  [ "$status" -eq 0 ]
}

@test "robustness: validate-story-bc-sync handles missing file_path" {
  _run_hook "validate-story-bc-sync.sh" '{"tool_input":{}}'
  [ "$status" -eq 0 ]
}

# ===== Nonexistent file paths =====
# File-based hooks should exit 0 when the file doesn't exist on disk

@test "robustness: validate-subsystem-names handles nonexistent file" {
  INPUT=$(jq -nc '{tool_input: {file_path: "/nonexistent/BC-1.01.001.md"}}')
  _run_hook "validate-subsystem-names.sh" "$INPUT"
  [ "$status" -eq 0 ]
}

@test "robustness: validate-bc-title handles nonexistent file" {
  INPUT=$(jq -nc '{tool_input: {file_path: "/nonexistent/BC-1.01.001.md"}}')
  _run_hook "validate-bc-title.sh" "$INPUT"
  [ "$status" -eq 0 ]
}

@test "robustness: validate-story-bc-sync handles nonexistent file" {
  INPUT=$(jq -nc '{tool_input: {file_path: "/nonexistent/STORY-001.md"}}')
  _run_hook "validate-story-bc-sync.sh" "$INPUT"
  [ "$status" -eq 0 ]
}

@test "robustness: factory-branch-guard handles nonexistent .factory path" {
  INPUT=$(jq -nc '{tool_input: {file_path: "/nonexistent/.factory/STATE.md"}}')
  _run_hook "factory-branch-guard.sh" "$INPUT"
  # Should block (no .git marker) or pass (path doesn't exist)
  # Either 0 or 2 is acceptable — must not crash (exit 1)
  [[ "$status" -eq 0 || "$status" -eq 2 ]]
}

# ===== Empty files (0 bytes) =====
# Hooks should not crash on empty files

@test "robustness: validate-subsystem-names handles empty BC file" {
  touch "$WORK/BC-1.01.001.md"
  mkdir -p "$WORK/../specs/behavioral-contracts" 2>/dev/null || true
  INPUT=$(jq -nc --arg fp "$WORK/BC-1.01.001.md" '{tool_input: {file_path: $fp}}')
  _run_hook "validate-subsystem-names.sh" "$INPUT"
  [ "$status" -eq 0 ]
}

@test "robustness: validate-bc-title handles empty BC file" {
  mkdir -p "$WORK/behavioral-contracts"
  touch "$WORK/behavioral-contracts/BC-1.01.001.md"
  INPUT=$(jq -nc --arg fp "$WORK/behavioral-contracts/BC-1.01.001.md" '{tool_input: {file_path: $fp}}')
  _run_hook "validate-bc-title.sh" "$INPUT"
  [ "$status" -eq 0 ]
}

@test "robustness: validate-story-bc-sync handles empty story file" {
  mkdir -p "$WORK/stories"
  touch "$WORK/stories/STORY-001.md"
  INPUT=$(jq -nc --arg fp "$WORK/stories/STORY-001.md" '{tool_input: {file_path: $fp}}')
  _run_hook "validate-story-bc-sync.sh" "$INPUT"
  [ "$status" -eq 0 ]
}

# ===== Files with no frontmatter =====

@test "robustness: validate-subsystem-names handles BC with no frontmatter" {
  mkdir -p "$WORK/specs/behavioral-contracts" "$WORK/specs/architecture"
  printf '# BC-1.01.001: Test\n\nNo frontmatter here.\n' > "$WORK/specs/behavioral-contracts/BC-1.01.001.md"
  printf '## Subsystem Registry\n| Subsystem Name |\n|---|\n| Test |\n' > "$WORK/specs/architecture/ARCH-INDEX.md"
  INPUT=$(jq -nc --arg fp "$WORK/specs/behavioral-contracts/BC-1.01.001.md" '{tool_input: {file_path: $fp}}')
  _run_hook "validate-subsystem-names.sh" "$INPUT"
  [ "$status" -eq 0 ]
}

@test "robustness: validate-bc-title handles BC with no H1" {
  mkdir -p "$WORK/behavioral-contracts"
  printf -- '---\nsubsystem: Test\n---\n\nNo H1 heading.\n' > "$WORK/behavioral-contracts/BC-1.01.001.md"
  printf '| BC | Title |\n|---|---|\n| BC-1.01.001 | Test |\n' > "$WORK/behavioral-contracts/BC-INDEX.md"
  INPUT=$(jq -nc --arg fp "$WORK/behavioral-contracts/BC-1.01.001.md" '{tool_input: {file_path: $fp}}')
  _run_hook "validate-bc-title.sh" "$INPUT"
  [ "$status" -eq 0 ]
}

@test "robustness: validate-story-bc-sync handles story with malformed frontmatter" {
  mkdir -p "$WORK/stories"
  printf -- '---\nstory_id: STORY-001\nbcs: this is not a list\n---\n# Test\n' > "$WORK/stories/STORY-001.md"
  INPUT=$(jq -nc --arg fp "$WORK/stories/STORY-001.md" '{tool_input: {file_path: $fp}}')
  _run_hook "validate-story-bc-sync.sh" "$INPUT"
  # Should not crash — exit 0 (no BCs parsed) or exit 2 (validation)
  [[ "$status" -eq 0 || "$status" -eq 2 ]]
}

# ===== Error output contract =====
# Violation messages must contain the policy number for agent parsing

@test "contract: destructive-command-guard says BLOCKED" {
  _run_hook "destructive-command-guard.sh" '{"tool_input":{"command":"rm -rf .factory/"}}'
  [ "$status" -eq 2 ]
  [[ "$output" == *"BLOCKED"* ]]
}

@test "contract: verify-git-push says BLOCKED" {
  _run_hook "verify-git-push.sh" '{"tool_input":{"command":"git push origin main"}}'
  [ "$status" -eq 2 ]
  [[ "$output" == *"BLOCKED"* ]]
}

@test "contract: factory-branch-guard says BLOCKED" {
  mkdir -p "$WORK/.factory"
  # No .git marker → block
  INPUT=$(jq -nc --arg fp "$WORK/.factory/STATE.md" '{tool_input: {file_path: $fp}}')
  _run_hook "factory-branch-guard.sh" "$INPUT"
  [ "$status" -eq 2 ]
  [[ "$output" == *"BLOCKED"* ]]
}

@test "contract: validate-vp-consistency says POLICY 9 VIOLATION" {
  # Use the canary fixture
  FIX="${BATS_TEST_DIRNAME}/fixtures/policy-9-canary/specs/verification-properties/VP-INDEX.md"
  INPUT=$(jq -nc --arg fp "$FIX" '{tool_input: {file_path: $fp}}')
  _run_hook "validate-vp-consistency.sh" "$INPUT"
  [ "$status" -eq 2 ]
  [[ "$output" == *"POLICY 9 VIOLATION"* ]]
}

@test "contract: validate-subsystem-names says POLICY 6 VIOLATION" {
  FIX="${BATS_TEST_DIRNAME}/fixtures/policy-enforcement/specs/behavioral-contracts/BC-2.01.002-bad.md"
  INPUT=$(jq -nc --arg fp "$FIX" '{tool_input: {file_path: $fp}}')
  _run_hook "validate-subsystem-names.sh" "$INPUT"
  [ "$status" -eq 2 ]
  [[ "$output" == *"POLICY 6 VIOLATION"* ]]
}

@test "contract: validate-bc-title says POLICY 7 VIOLATION" {
  FIX="${BATS_TEST_DIRNAME}/fixtures/policy-enforcement/specs/behavioral-contracts/BC-2.01.003-title-mismatch.md"
  INPUT=$(jq -nc --arg fp "$FIX" '{tool_input: {file_path: $fp}}')
  _run_hook "validate-bc-title.sh" "$INPUT"
  [ "$status" -eq 2 ]
  [[ "$output" == *"POLICY 7 VIOLATION"* ]]
}

@test "contract: validate-story-bc-sync says POLICY 8 VIOLATION" {
  FIX="${BATS_TEST_DIRNAME}/fixtures/policy-enforcement/stories/STORY-003-bad-sync.md"
  INPUT=$(jq -nc --arg fp "$FIX" '{tool_input: {file_path: $fp}}')
  _run_hook "validate-story-bc-sync.sh" "$INPUT"
  [ "$status" -eq 2 ]
  [[ "$output" == *"POLICY 8 VIOLATION"* ]]
}

# ===== All hooks have exit 0 fallback (never exit 1) =====
# A hook crashing (exit 1) looks like an error, not a policy violation.
# All hooks should exit 0 (allow) or 2 (block) — never 1.

@test "contract: validate-finding-format says ID FORMAT VIOLATION" {
  WORK=$(mktemp -d)
  mkdir -p "$WORK/.factory/cycles/v1/adversarial-reviews"
  echo "## ADV-001: Legacy finding" > "$WORK/.factory/cycles/v1/adversarial-reviews/pass-1.md"
  INPUT=$(jq -nc --arg fp "$WORK/.factory/cycles/v1/adversarial-reviews/pass-1.md" '{tool_input: {file_path: $fp}}')
  _run_hook "validate-finding-format.sh" "$INPUT"
  [ "$status" -eq 2 ]
  [[ "$output" == *"ID FORMAT VIOLATION"* ]]
  rm -rf "$WORK"
}

@test "robustness: validate-finding-format handles empty JSON" {
  _run_hook "validate-finding-format.sh" '{}'
  [ "$status" -eq 0 ]
}

@test "robustness: validate-finding-format handles missing file_path" {
  _run_hook "validate-finding-format.sh" '{"tool_input":{}}'
  [ "$status" -eq 0 ]
}

@test "contract: all hooks pass syntax check" {
  for hook in destructive-command-guard verify-git-push factory-branch-guard \
              validate-vp-consistency validate-subsystem-names validate-bc-title \
              validate-story-bc-sync validate-template-compliance validate-finding-format; do
    bash -n "$HOOKS/$hook.sh"
  done
}
