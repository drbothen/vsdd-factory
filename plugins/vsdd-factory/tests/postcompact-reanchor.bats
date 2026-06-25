#!/usr/bin/env bats
# postcompact-reanchor.bats — Red Gate bats test suite for S-18.05
#
# All 8 tests MUST FAIL before the hook is implemented (Red Gate discipline).
# Hook invocation convention (from precompact-routing.bats precedent):
#   The hook is a bash script that receives a JSON PostCompact event envelope on stdin.
#   Invocation: printf '%s' '<json-envelope>' | bash <hook-script>
#   JSON envelope: {"event_name":"PostCompact","tool_name":"","session_id":"<id>","tool_input":{}}
#   The hook reads from factory-artifacts via: git show factory-artifacts:.factory/STATE.md
#   Hook writes JSONL log to: .factory/logs/postcompact-reanchor-YYYY-MM-DD.jsonl
#   (relative to CWD, which the tests set to WORK via cd inside the run call)
#
# Red Gate: stub at plugins/vsdd-factory/hooks/postcompact-reanchor.sh prints
#   "not implemented" and exits 1. Every assertion below fails against the stub
#   for an assertion reason, not a bats syntax error.
#
# Tests:
#   1. test_postcompact_reanchor_emits_anchor_block_with_git_sourced_values   (AC-001)
#   2. test_postcompact_reanchor_appends_jsonl_log_with_correct_fields_no_wave_id (AC-002)
#   3. test_postcompact_reanchor_does_not_commit_to_factory_artifacts          (AC-004)
#   4. test_postcompact_reanchor_exits_0_and_warns_on_factory_artifacts_unreachable (AC-005)
#   5. test_postcompact_reanchor_emits_context_unknown_when_fields_absent      (AC-006)
#   6. test_postcompact_reanchor_exits_0_on_all_error_paths                   (AC-008)
#   7. test_postcompact_reanchor_values_are_git_sourced_not_in_context        (AC-009)
#   8. test_postcompact_reanchor_registry_entry_has_correct_shape             (AC-010)
#
# BC: BC-7.07.002 v1.12
# VP: VP-089 v1.0
# Story: S-18.05
# Subsystem: SS-07 (Hook Bash Layer)

# ---------------------------------------------------------------------------
# setup / teardown
# ---------------------------------------------------------------------------

setup() {
  REPO_ROOT="$(cd "${BATS_TEST_DIRNAME}/../../.." && pwd)"
  HOOK_SCRIPT="$REPO_ROOT/plugins/vsdd-factory/hooks/postcompact-reanchor.sh"
  REGISTRY="$REPO_ROOT/plugins/vsdd-factory/hooks-registry.toml"

  WORK="$(mktemp -d)"

  # Configure git identity so git operations in the fixture work without system config.
  FAKE_HOME="$WORK/home"
  mkdir -p "$FAKE_HOME"
  cat > "$FAKE_HOME/.gitconfig" <<'GITCFG'
[user]
  name = Test Agent
  email = test@factory.local
[init]
  defaultBranch = develop
GITCFG

  # Create .factory/logs directory so the hook can write the daily log.
  # The hook is invoked with CWD=$WORK so paths relative to CWD land in WORK.
  mkdir -p "$WORK/.factory/logs"

  export WORK FAKE_HOME HOOK_SCRIPT REGISTRY
}

teardown() {
  rm -rf "$WORK"
}

# ---------------------------------------------------------------------------
# Git fixture helper: _init_factory_artifacts_fixture
#
# Creates a git repository at $WORK/repo with:
#   - develop branch (main repo HEAD)
#   - factory-artifacts branch containing .factory/STATE.md with caller-supplied content
#
# The hook reads STATE.md via: git show factory-artifacts:.factory/STATE.md
# This requires that from the CWD the git repo is accessible (or GIT_DIR is set).
# We set GIT_DIR=$WORK/repo/.git and run the hook from $WORK so that:
#   git show factory-artifacts:.factory/STATE.md
# resolves the branch correctly.
#
# Args:
#   $1 - state_md_content: full text of .factory/STATE.md to place on factory-artifacts
#
# Sets REPO_DIR (path to the git repo).
# ---------------------------------------------------------------------------
_init_factory_artifacts_fixture() {
  local state_md_content="$1"

  REPO_DIR="$WORK/repo"
  mkdir -p "$REPO_DIR"

  # Init repo on develop branch
  HOME="$FAKE_HOME" git init "$REPO_DIR" >/dev/null 2>&1
  git -c user.name="Test Agent" -c user.email="test@factory.local" \
    -C "$REPO_DIR" commit --allow-empty -m "init develop" >/dev/null 2>&1

  # Create factory-artifacts branch with .factory/STATE.md content
  git -C "$REPO_DIR" checkout -b factory-artifacts >/dev/null 2>&1

  # On factory-artifacts the branch stores paths relative to root.
  # The hook uses: git show factory-artifacts:.factory/STATE.md
  # So the file on factory-artifacts must be at .factory/STATE.md.
  mkdir -p "$REPO_DIR/.factory"
  printf '%s' "$state_md_content" > "$REPO_DIR/.factory/STATE.md"
  git -c user.name="Test Agent" -c user.email="test@factory.local" \
    -C "$REPO_DIR" add ".factory/STATE.md" >/dev/null 2>&1
  git -c user.name="Test Agent" -c user.email="test@factory.local" \
    -C "$REPO_DIR" commit -m "init factory-artifacts with STATE.md" >/dev/null 2>&1

  # Switch back to develop
  git -C "$REPO_DIR" checkout develop >/dev/null 2>&1

  export REPO_DIR
}

# Run the hook with GIT_DIR pointing at the fixture repo, CWD=$WORK.
# The hook is expected to write logs relative to CWD (.factory/logs/).
# Args: [extra env vars as VAR=VALUE ...]
_run_hook() {
  local extra_env=("$@")
  # Build the env prefix
  local env_prefix="GIT_DIR='$REPO_DIR/.git' HOME='$FAKE_HOME'"
  for ev in "${extra_env[@]}"; do
    env_prefix="$env_prefix $ev"
  done
  # Run from $WORK so relative .factory/logs/ writes land in $WORK/.factory/logs/
  run bash -c "cd '$WORK' && printf '%s' '{\"event_name\":\"PostCompact\",\"tool_name\":\"\",\"session_id\":\"bats-test\",\"tool_input\":{}}' | $env_prefix bash '$HOOK_SCRIPT' 2>&1"
}

# Standard STATE.md fixture content with known values (AC-001 canonical test vector)
_STATE_MD_HAPPY="---
document_type: state
version: \"0.0.1-test\"
current_cycle: v1.0-feature-context-durability-E18
current_step: S-18.04
last_verified_develop_sha: abc123def456
---

# STATE (bats test fixture)
"

# ---------------------------------------------------------------------------
# Test 1 — AC-001
# test_postcompact_reanchor_emits_anchor_block_with_git_sourced_values
#
# BC-7.07.002 postcondition 1: re-anchor block to stdout; git-sourced; correct format.
# VP-089 §1 Stdout Re-Anchor Block.
#
# Red Gate: stub exits 1; stdout is "not implemented" — neither the
# [PostCompact Re-anchor] line nor the Source line appear; test fails.
# ---------------------------------------------------------------------------

@test "test_postcompact_reanchor_emits_anchor_block_with_git_sourced_values" {
  # Setup: factory-artifacts STATE.md has known current_cycle, current_step, sha
  _init_factory_artifacts_fixture "$_STATE_MD_HAPPY"

  _run_hook

  # Assert: exit 0 (hook must not fail)
  [ "$status" -eq 0 ]

  # Assert: stdout contains the canonical re-anchor line (BC-7.07.002 PC1 format)
  [[ "$output" == *"[PostCompact Re-anchor] context=v1.0-feature-context-durability-E18/S-18.04 sha=abc123def456"* ]]

  # Assert: stdout contains the Source line
  [[ "$output" == *"Source: factory-artifacts STATE.md"* ]]

  # Assert: stdout does NOT contain "current_wave" (phantom field prohibition)
  [[ "$output" != *"current_wave"* ]]
}

# ---------------------------------------------------------------------------
# Test 2 — AC-002
# test_postcompact_reanchor_appends_jsonl_log_with_correct_fields_no_wave_id
#
# BC-7.07.002 postcondition 2: log appended; exactly 6 fields; no wave_id.
# VP-089 §2 Log Entry Appended.
#
# Red Gate: stub exits 1; no log file written; all jq assertions fail.
# ---------------------------------------------------------------------------

@test "test_postcompact_reanchor_appends_jsonl_log_with_correct_fields_no_wave_id" {
  # Setup: factory-artifacts with valid STATE.md
  _init_factory_artifacts_fixture "$_STATE_MD_HAPPY"

  # Verify no log file exists before hook runs
  local log_pattern="$WORK/.factory/logs/postcompact-reanchor-*.jsonl"
  # shellcheck disable=SC2086
  [ "$(ls $log_pattern 2>/dev/null | wc -l)" -eq 0 ]

  _run_hook

  # Assert: exit 0
  [ "$status" -eq 0 ]

  # Assert: exactly one daily JSONL log file now exists
  local log_files
  # shellcheck disable=SC2086
  log_files=($(ls $log_pattern 2>/dev/null))
  [ "${#log_files[@]}" -ge 1 ]

  # Read the last line of the log (most recent JSONL entry)
  local log_line
  log_line=$(tail -1 "${log_files[0]}")

  # Assert: valid JSON (jq can parse it)
  echo "$log_line" | jq -e '.' >/dev/null 2>&1

  # Assert: all 6 required fields are present
  echo "$log_line" | jq -e '.event'                     >/dev/null 2>&1
  echo "$log_line" | jq -e '.current_cycle'             >/dev/null 2>&1
  echo "$log_line" | jq -e '.current_step'              >/dev/null 2>&1
  echo "$log_line" | jq -e '.last_verified_develop_sha' >/dev/null 2>&1
  echo "$log_line" | jq -e '.timestamp'                 >/dev/null 2>&1
  echo "$log_line" | jq -e '.status'                    >/dev/null 2>&1

  # Assert: status is "ok" on happy path
  local status_val
  status_val=$(echo "$log_line" | jq -r '.status')
  [ "$status_val" = "ok" ]

  # Assert: wave_id is ABSENT (BC-7.07.002 PC2 no wave_id)
  # jq -e '.wave_id' exits non-zero when the key is null or absent.
  if echo "$log_line" | jq -e '.wave_id' >/dev/null 2>&1; then
    # If wave_id exists and is not null, that is a specification violation
    local wave_id_val
    wave_id_val=$(echo "$log_line" | jq -r '.wave_id')
    [ "$wave_id_val" = "null" ]
  fi
  # Explicit key-absence check: the field must not appear in the JSON object keys
  local has_wave_id
  has_wave_id=$(echo "$log_line" | jq 'has("wave_id")')
  [ "$has_wave_id" = "false" ]
}

# ---------------------------------------------------------------------------
# Test 3 — AC-004
# test_postcompact_reanchor_does_not_commit_to_factory_artifacts
#
# BC-7.07.002 postcondition 4 + invariant 1: read-only on factory-artifacts.
# VP-089 §3 No factory-artifacts Commits.
#
# Red Gate part A — HEAD check: stub exits 1 before any git operations;
#   HEAD_BEFORE == HEAD_AFTER is satisfied (trivially), so this assertion alone
#   doesn't fail the test. However:
# Red Gate part B — source grep: stub says "not implemented"; no `git commit`
#   in hook source is actually correct for the stub (stub has none). So part B
#   passes vacuously.
# BUT part A: stub exits 1 → [ "$status" -eq 0 ] fails. This is the Red Gate
#   assertion that makes the test fail.
#
# After implementation: both A (HEAD unchanged) and B (no git commit in source)
#   must pass with exit 0.
# ---------------------------------------------------------------------------

@test "test_postcompact_reanchor_does_not_commit_to_factory_artifacts" {
  # Setup: factory-artifacts with valid STATE.md
  _init_factory_artifacts_fixture "$_STATE_MD_HAPPY"

  # Capture factory-artifacts HEAD before hook invocation
  local head_before
  head_before=$(GIT_DIR="$REPO_DIR/.git" git rev-parse factory-artifacts)

  _run_hook

  # Assert: exit 0 (hook must not fail — this is the Red Gate assertion)
  [ "$status" -eq 0 ]

  # Assert: factory-artifacts HEAD is unchanged after hook ran
  local head_after
  head_after=$(GIT_DIR="$REPO_DIR/.git" git rev-parse factory-artifacts)
  [ "$head_before" = "$head_after" ]

  # Assert: hook source does NOT contain any git write commands targeting factory-artifacts
  # (BC-7.07.002 invariant 1 absolute prohibition; load-bearing source check)
  # Check for the forbidden patterns in the hook source file
  [ -f "$HOOK_SCRIPT" ]
  ! grep -qE '^[^#]*git (commit|push|add)[^|&;]*factory.artifacts' "$HOOK_SCRIPT"
  ! grep -qE '^[^#]*git -C[^|&;]*(commit|push)[^|&;]+factory.artifacts' "$HOOK_SCRIPT"
}

# ---------------------------------------------------------------------------
# Test 4 — AC-005
# test_postcompact_reanchor_exits_0_and_warns_on_factory_artifacts_unreachable
#
# BC-7.07.002 postcondition 5 + EC-002: fail-open on factory-artifacts unreachable.
# VP-089 §4 Exit 0 on All Error Paths.
#
# Red Gate: stub exits 1 → [ "$status" -eq 0 ] fails.
# ---------------------------------------------------------------------------

@test "test_postcompact_reanchor_exits_0_and_warns_on_factory_artifacts_unreachable" {
  # Setup: use a non-existent GIT_DIR so git show factory-artifacts:... will fail
  REPO_DIR="$WORK/nonexistent-repo"
  # Do NOT call _init_factory_artifacts_fixture — repo does not exist

  # Invoke hook with unreachable git repo
  run bash -c "cd '$WORK' && printf '%s' '{\"event_name\":\"PostCompact\",\"session_id\":\"bats-unreachable\",\"tool_input\":{}}' | GIT_DIR='$REPO_DIR/.git' HOME='$FAKE_HOME' bash '$HOOK_SCRIPT' 2>&1"

  # Assert: exit 0 (fail-open — BC-7.07.002 PC5)
  [ "$status" -eq 0 ]

  # Assert: stdout contains the WARN advisory (EC-002 canonical message)
  [[ "$output" == *"[PostCompact Re-anchor] WARN: factory-artifacts unreachable"* ]]

  # Assert: daily log entry has status="warn"
  local log_pattern="$WORK/.factory/logs/postcompact-reanchor-*.jsonl"
  # shellcheck disable=SC2086
  local log_files=($(ls $log_pattern 2>/dev/null))
  if [ "${#log_files[@]}" -ge 1 ]; then
    local log_line
    log_line=$(tail -1 "${log_files[0]}")
    local status_val
    status_val=$(echo "$log_line" | jq -r '.status' 2>/dev/null || echo "missing")
    [ "$status_val" = "warn" ]
  fi
  # Note: if the hook cannot write the log (e.g., log dir creation failed under
  # this error condition), we only require exit 0 + WARN stdout (AC-005 minimum).
  # The test does not require the log file to exist in the unreachable-git case.
}

# ---------------------------------------------------------------------------
# Test 5 — AC-006
# test_postcompact_reanchor_emits_context_unknown_when_fields_absent
#
# BC-7.07.002 EC-003: STATE.md present but current_cycle/current_step absent →
#   emit context=UNKNOWN; log status=warn; exit 0.
# VP-089 §4 Exit 0 on All Error Paths.
#
# Red Gate: stub exits 1 → [ "$status" -eq 0 ] fails.
# ---------------------------------------------------------------------------

@test "test_postcompact_reanchor_emits_context_unknown_when_fields_absent" {
  # Setup: STATE.md is present but has NO current_cycle: or current_step: fields
  local state_md_no_fields="---
document_type: state
version: \"0.0.1-test\"
---

# STATE (bats test fixture — fields deliberately absent for EC-003)
"
  _init_factory_artifacts_fixture "$state_md_no_fields"

  _run_hook

  # Assert: exit 0 (fail-open)
  [ "$status" -eq 0 ]

  # Assert: stdout contains context=UNKNOWN (EC-003 canonical)
  [[ "$output" == *"context=UNKNOWN"* ]]

  # Assert: daily log entry has status="warn"
  local log_pattern="$WORK/.factory/logs/postcompact-reanchor-*.jsonl"
  # shellcheck disable=SC2086
  local log_files=($(ls $log_pattern 2>/dev/null))
  if [ "${#log_files[@]}" -ge 1 ]; then
    local log_line
    log_line=$(tail -1 "${log_files[0]}")
    local status_val
    status_val=$(echo "$log_line" | jq -r '.status' 2>/dev/null || echo "missing")
    [ "$status_val" = "warn" ]
  fi
}

# ---------------------------------------------------------------------------
# Test 6 — AC-008
# test_postcompact_reanchor_exits_0_on_all_error_paths
#
# BC-7.07.002 postconditions 5+6: exit 0 under ALL error conditions.
# VP-089 §4 Exit 0 on All Error Paths.
#
# Tests three error scenarios in sequence; each must exit 0.
# Red Gate: stub exits 1 on every path → first scenario fails immediately.
# ---------------------------------------------------------------------------

@test "test_postcompact_reanchor_exits_0_on_all_error_paths" {
  # --- Scenario A: factory-artifacts unreachable ---
  REPO_DIR="$WORK/nonexistent-for-ac008"
  run bash -c "cd '$WORK' && printf '%s' '{\"event_name\":\"PostCompact\",\"session_id\":\"bats-ac008-a\",\"tool_input\":{}}' | GIT_DIR='$REPO_DIR/.git' HOME='$FAKE_HOME' bash '$HOOK_SCRIPT' 2>&1"
  [ "$status" -eq 0 ] # MUST exit 0 (fail-open)

  # --- Scenario B: STATE.md fields absent ---
  local state_md_empty="---
document_type: state
---
"
  _init_factory_artifacts_fixture "$state_md_empty"
  _run_hook
  [ "$status" -eq 0 ] # MUST exit 0

  # --- Scenario C: .factory/logs/ directory not writable ---
  # Setup valid STATE.md but make logs/ directory unwritable
  _init_factory_artifacts_fixture "$_STATE_MD_HAPPY"
  chmod 000 "$WORK/.factory/logs" 2>/dev/null || true
  _run_hook
  local exit_status_c="$status"
  # Restore permissions for teardown
  chmod 755 "$WORK/.factory/logs" 2>/dev/null || true
  [ "$exit_status_c" -eq 0 ] # MUST exit 0 even if log write fails
}

# ---------------------------------------------------------------------------
# Test 7 — AC-009
# test_postcompact_reanchor_values_are_git_sourced_not_in_context
#
# BC-7.07.002 invariant 2: values must come from git, not env vars or in-context.
# VP-089 §1 git-sourced, not in-context.
#
# Setup: factory-artifacts STATE.md has current_cycle: A
#        env var CURRENT_CYCLE=B is set (stale in-memory value)
# Assert: output contains context=A/... (git value wins)
#         output does NOT contain context=B/... (env var ignored)
#         output does NOT contain "current_wave" (phantom field)
#
# Red Gate: stub exits 1 → [ "$status" -eq 0 ] fails.
# This test is LOAD-BEARING: it verifies the core git-sourced invariant.
# ---------------------------------------------------------------------------

@test "test_postcompact_reanchor_values_are_git_sourced_not_in_context" {
  # Setup: factory-artifacts STATE.md has current_cycle: A (git value)
  local state_md_cycle_a="---
document_type: state
version: \"0.0.1-test\"
current_cycle: A
current_step: step-from-git
last_verified_develop_sha: sha-from-git-000
---

# STATE — current_cycle is A (git value; env var says B — hook must use A)
"
  _init_factory_artifacts_fixture "$state_md_cycle_a"

  # Run hook with stale env var CURRENT_CYCLE=B
  run bash -c "cd '$WORK' && printf '%s' '{\"event_name\":\"PostCompact\",\"session_id\":\"bats-ac009\",\"tool_input\":{}}' | GIT_DIR='$REPO_DIR/.git' HOME='$FAKE_HOME' CURRENT_CYCLE='B' bash '$HOOK_SCRIPT' 2>&1"

  # Assert: exit 0
  [ "$status" -eq 0 ]

  # Assert: output uses git value A, NOT env var B
  [[ "$output" == *"context=A/"* ]]
  [[ "$output" != *"context=B/"* ]]

  # Assert: no "current_wave" in output (phantom field prohibition — BC-7.07.002 Inv2)
  [[ "$output" != *"current_wave"* ]]
  [[ "$output" != *"wave="* ]]
}

# ---------------------------------------------------------------------------
# Test 8 — AC-010
# test_postcompact_reanchor_registry_entry_has_correct_shape
#
# BC-7.07.002 precondition 1: hooks-registry.toml PostCompact entry must exist
#   with the canonical shape.
#
# Red Gate: registry entry does not exist yet → grep fails → test fails.
# This test exercises the registry shape, not the hook runtime.
# ---------------------------------------------------------------------------

@test "test_postcompact_reanchor_registry_entry_has_correct_shape" {
  # Assert: registry file exists (sanity check)
  [ -f "$REGISTRY" ]

  # Assert: postcompact-reanchor entry exists in registry
  # (This will fail at Red Gate because T-3 has not added the entry yet)
  grep -q 'name = "postcompact-reanchor"' "$REGISTRY"

  # Extract the [[hooks]] block for postcompact-reanchor (up to 20 lines)
  local block
  block=$(grep -A 20 'name = "postcompact-reanchor"' "$REGISTRY")

  # Assert: event = "PostCompact" (correct event)
  echo "$block" | grep -q 'event = "PostCompact"'

  # Assert: on_error = "continue" (mandatory per BC-7.07.002 PC6)
  echo "$block" | grep -q 'on_error = "continue"'

  # Assert: plugin = "hook-plugins/legacy-bash-adapter.wasm" (correct adapter)
  echo "$block" | grep -q 'plugin = "hook-plugins/legacy-bash-adapter.wasm"'

  # Assert: script_path = "hooks/postcompact-reanchor.sh" in [hooks.config]
  # (may be on a different line than the hook block header, so search after the name line)
  local config_block
  config_block=$(grep -A 30 'name = "postcompact-reanchor"' "$REGISTRY")
  echo "$config_block" | grep -q 'script_path = "hooks/postcompact-reanchor.sh"'
}
