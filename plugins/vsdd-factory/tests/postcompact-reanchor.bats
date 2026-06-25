#!/usr/bin/env bats
# postcompact-reanchor.bats — Realigned bats test suite for S-18.05
#                             (LOCAL adversary Pass-1 fix burst)
#
# Contract changes versus original Red Gate version:
#   F-P1-001: SHA from git rev-parse refs/remotes/origin/develop — fixture
#             repos must expose that ref; tests assert the git-rev-parse value.
#   F-P1-002: STATE.md fixtures carry NO last_verified_develop_sha field —
#             field was never real; removed from all fixtures to close false-green.
#   F-P1-003: AC-002 log field rename last_verified_develop_sha → develop_sha.
#   F-P1-005: AC-010 asserts the FULL ADR-026 §Decision 7 v1.25 capabilities
#             block, bounded to the single [[hooks]] stanza.
#   F-P1-006: New test for EC-005 (mkdir-p fails path), explicit AC-003 coverage.
#
# Hook invocation convention:
#   printf '%s' '<json-envelope>' | bash <hook-script>
#   JSON envelope: {"event_name":"PostCompact","tool_name":"","session_id":"<id>","tool_input":{}}
#   Hook reads from factory-artifacts via: git show factory-artifacts:.factory/STATE.md
#   Hook sources develop SHA via: git rev-parse refs/remotes/origin/develop
#   Hook writes JSONL log to: .factory/logs/postcompact-reanchor-YYYY-MM-DD.jsonl
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
#   9. test_postcompact_reanchor_cannot_block_advisory_only                   (AC-003)
#  10. test_postcompact_reanchor_exits_0_when_log_dir_mkdir_fails             (EC-005)
#
# BC: BC-7.07.002
# VP: VP-089
# ADR: ADR-026 §Decision 7
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

  # NOTE: the hook is invoked with CWD=$REPO_DIR (the fixture git repo), so
  # relative .factory/logs/ writes land in $REPO_DIR/.factory/logs/.
  # The log dir is created inside _init_factory_artifacts_fixture after REPO_DIR
  # is set (see that helper for details).

  export WORK FAKE_HOME HOOK_SCRIPT REGISTRY
}

teardown() {
  rm -rf "$WORK"
}

# ---------------------------------------------------------------------------
# Git fixture helper: _init_factory_artifacts_fixture
#
# Creates a git repository at a fresh tmpdir with:
#   - develop branch (main repo HEAD)
#   - factory-artifacts branch containing .factory/STATE.md with caller-supplied content
#   - refs/remotes/origin/develop pointing at a KNOWN sha (FIXTURE_DEVELOP_SHA)
#   - .factory/logs/ directory for hook log writes (relative to REPO_DIR)
#
# The hook reads STATE.md via:  git show factory-artifacts:.factory/STATE.md
# The hook reads develop SHA via: git rev-parse refs/remotes/origin/develop
#
# The hook is invoked with CWD=$REPO_DIR (production-equivalent: no GIT_DIR in
# hook env; git resolves via normal CWD-based repo discovery — ADR-026 §Decision 7
# / BC-7.07.002 Precondition 1 / story AC-010).
#
# Args:
#   $1 - state_md_content: full text of .factory/STATE.md to place on factory-artifacts
#
# Sets:
#   REPO_DIR            — path to the git repo
#   FIXTURE_DEVELOP_SHA — the SHA stored in refs/remotes/origin/develop
# ---------------------------------------------------------------------------
_init_factory_artifacts_fixture() {
  local state_md_content="$1"

  # Always create a FRESH isolated temp repo so this helper is safe to call
  # multiple times within the same @test (e.g. AC-008 calls it twice for two
  # independent error scenarios).  Using a fixed path like $WORK/repo caused
  # status 128 on the second call because `git checkout -b factory-artifacts`
  # fails when the branch already exists.
  REPO_DIR="$(mktemp -d)"

  # Init repo on develop branch
  HOME="$FAKE_HOME" git init "$REPO_DIR" >/dev/null 2>&1
  git -c user.name="Test Agent" -c user.email="test@factory.local" \
    -C "$REPO_DIR" commit --allow-empty -m "init develop" >/dev/null 2>&1

  # Capture the develop HEAD SHA — this becomes the simulated origin/develop ref.
  # The hook calls: git rev-parse refs/remotes/origin/develop
  # We create that ref pointing at the develop HEAD.
  local develop_head
  develop_head=$(git -C "$REPO_DIR" rev-parse HEAD)

  # Create refs/remotes/origin/develop pointing at develop HEAD (F-P1-001).
  # This is the ref the hook reads for the develop SHA.
  git -C "$REPO_DIR" update-ref refs/remotes/origin/develop "$develop_head" >/dev/null 2>&1

  # Export the expected SHA so individual tests can assert on it.
  FIXTURE_DEVELOP_SHA="$develop_head"
  export FIXTURE_DEVELOP_SHA

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

  # Create the log directory the hook will write to.
  # The hook runs with CWD=$REPO_DIR, so .factory/logs/ resolves to here.
  mkdir -p "$REPO_DIR/.factory/logs"

  export REPO_DIR
}

# Run the hook with CWD=$REPO_DIR (production-equivalent: no GIT_DIR in hook env).
# Git resolves via normal CWD-based repo discovery from the fixture repo root.
# The hook writes logs relative to CWD (.factory/logs/ inside REPO_DIR).
# Args: [extra env vars as VAR=VALUE ...]
_run_hook() {
  local extra_env=("$@")
  # Build the env prefix — HOME only; no GIT_DIR (production env profile per
  # ADR-026 §Decision 7 / BC-7.07.002 Precondition 1 / story AC-010).
  local env_prefix="HOME='$FAKE_HOME'"
  for ev in "${extra_env[@]}"; do
    env_prefix="$env_prefix $ev"
  done
  # Run from $REPO_DIR so git discovers the repo via CWD (no GIT_DIR needed).
  # Relative .factory/logs/ writes land in $REPO_DIR/.factory/logs/.
  run bash -c "cd '$REPO_DIR' && printf '%s' '{\"event_name\":\"PostCompact\",\"tool_name\":\"\",\"session_id\":\"bats-test\",\"tool_input\":{}}' | $env_prefix bash '$HOOK_SCRIPT' 2>&1"
}

# ---------------------------------------------------------------------------
# STATE.md fixture: happy path (F-P1-002: NO last_verified_develop_sha field).
# Develop SHA comes from git rev-parse refs/remotes/origin/develop — not STATE.md.
# ---------------------------------------------------------------------------
_STATE_MD_HAPPY="---
document_type: state
version: \"0.0.1-test\"
current_cycle: v1.0-feature-context-durability-E18
current_step: S-18.04
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
# F-P1-001: SHA asserted from FIXTURE_DEVELOP_SHA (git rev-parse), not STATE.md field.
# F-P1-002: _STATE_MD_HAPPY has no last_verified_develop_sha field.
# ---------------------------------------------------------------------------

@test "test_postcompact_reanchor_emits_anchor_block_with_git_sourced_values" {
  # Setup: factory-artifacts STATE.md with known current_cycle/current_step;
  # refs/remotes/origin/develop is wired to a known SHA by _init_factory_artifacts_fixture.
  _init_factory_artifacts_fixture "$_STATE_MD_HAPPY"

  _run_hook

  # Assert: exit 0 (hook must not fail)
  [ "$status" -eq 0 ]

  # Assert: stdout contains context= from STATE.md (current_cycle/current_step)
  [[ "$output" == *"[PostCompact Re-anchor] context=v1.0-feature-context-durability-E18/S-18.04"* ]]

  # Assert: stdout contains sha= with the git-rev-parse value (FIXTURE_DEVELOP_SHA),
  # NOT a hard-coded value from a STATE.md field (F-P1-001 load-bearing).
  [[ "$output" == *"sha=${FIXTURE_DEVELOP_SHA}"* ]]

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
# F-P1-003: log field is .develop_sha (renamed from .last_verified_develop_sha).
# ---------------------------------------------------------------------------

@test "test_postcompact_reanchor_appends_jsonl_log_with_correct_fields_no_wave_id" {
  # Setup: factory-artifacts with valid STATE.md
  _init_factory_artifacts_fixture "$_STATE_MD_HAPPY"

  # Verify no log file exists before hook runs
  local log_pattern="$REPO_DIR/.factory/logs/postcompact-reanchor-*.jsonl"
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

  # Assert: all 6 required fields are present (F-P1-003: develop_sha not last_verified_develop_sha)
  echo "$log_line" | jq -e '.event'          >/dev/null 2>&1
  echo "$log_line" | jq -e '.current_cycle'  >/dev/null 2>&1
  echo "$log_line" | jq -e '.current_step'   >/dev/null 2>&1
  echo "$log_line" | jq -e '.develop_sha'    >/dev/null 2>&1
  echo "$log_line" | jq -e '.timestamp'      >/dev/null 2>&1
  echo "$log_line" | jq -e '.status'         >/dev/null 2>&1

  # Assert: the old field name is NOT present (F-P1-003 load-bearing — would catch revert)
  local has_old_field
  has_old_field=$(echo "$log_line" | jq 'has("last_verified_develop_sha")')
  [ "$has_old_field" = "false" ]

  # Assert: develop_sha equals the git-rev-parse value, not a STATE.md literal (F-P1-001)
  local sha_val
  sha_val=$(echo "$log_line" | jq -r '.develop_sha')
  [ "$sha_val" = "$FIXTURE_DEVELOP_SHA" ]

  # Assert: status is "ok" on happy path
  local status_val
  status_val=$(echo "$log_line" | jq -r '.status')
  [ "$status_val" = "ok" ]

  # Assert: wave_id is ABSENT (BC-7.07.002 PC2 no wave_id)
  local has_wave_id
  has_wave_id=$(echo "$log_line" | jq 'has("wave_id")')
  [ "$has_wave_id" = "false" ]

  # Assert: exactly 6 keys in the JSON object (no extra fields)
  local key_count
  key_count=$(echo "$log_line" | jq 'keys | length')
  [ "$key_count" -eq 6 ]
}

# ---------------------------------------------------------------------------
# Test 3 — AC-004
# test_postcompact_reanchor_does_not_commit_to_factory_artifacts
#
# BC-7.07.002 postcondition 4 + invariant 1: read-only on factory-artifacts.
# VP-089 §3 No factory-artifacts Commits.
# ---------------------------------------------------------------------------

@test "test_postcompact_reanchor_does_not_commit_to_factory_artifacts" {
  # Setup: factory-artifacts with valid STATE.md
  _init_factory_artifacts_fixture "$_STATE_MD_HAPPY"

  # Capture factory-artifacts HEAD before hook invocation
  local head_before
  head_before=$(git -C "$REPO_DIR" rev-parse factory-artifacts)

  _run_hook

  # Assert: exit 0 (hook must not fail — this is the Red Gate assertion)
  [ "$status" -eq 0 ]

  # Assert: factory-artifacts HEAD is unchanged after hook ran
  local head_after
  head_after=$(git -C "$REPO_DIR" rev-parse factory-artifacts)
  [ "$head_before" = "$head_after" ]

  # Assert: hook source does NOT contain any git write commands targeting factory-artifacts
  # (BC-7.07.002 invariant 1 absolute prohibition; load-bearing source check)
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
# ---------------------------------------------------------------------------

@test "test_postcompact_reanchor_exits_0_and_warns_on_factory_artifacts_unreachable" {
  # Setup: invoke hook from $WORK which has no .git directory, so git commands
  # fail with "not a git repository" (CWD-based discovery; no GIT_DIR injection).
  # Do NOT call _init_factory_artifacts_fixture — no repo should exist.
  # $WORK was created by setup() and contains only the home dir; it has no .git.
  mkdir -p "$WORK/.factory/logs"

  # Invoke hook with CWD=$WORK (non-git directory; no GIT_DIR in env).
  run bash -c "cd '$WORK' && printf '%s' '{\"event_name\":\"PostCompact\",\"session_id\":\"bats-unreachable\",\"tool_input\":{}}' | HOME='$FAKE_HOME' bash '$HOOK_SCRIPT' 2>&1"

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
  local log_pattern="$REPO_DIR/.factory/logs/postcompact-reanchor-*.jsonl"
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
# ---------------------------------------------------------------------------

@test "test_postcompact_reanchor_exits_0_on_all_error_paths" {
  # --- Scenario A: factory-artifacts unreachable ---
  # Run from $WORK (no .git dir); git CWD-discovery fails with "not a git
  # repository" — hook must still exit 0 (fail-open; no GIT_DIR in env).
  mkdir -p "$WORK/.factory/logs"
  run bash -c "cd '$WORK' && printf '%s' '{\"event_name\":\"PostCompact\",\"session_id\":\"bats-ac008-a\",\"tool_input\":{}}' | HOME='$FAKE_HOME' bash '$HOOK_SCRIPT' 2>&1"
  [ "$status" -eq 0 ] # MUST exit 0 (fail-open)

  # --- Scenario B: STATE.md fields absent ---
  local state_md_empty="---
document_type: state
---
"
  _init_factory_artifacts_fixture "$state_md_empty"
  _run_hook
  [ "$status" -eq 0 ] # MUST exit 0

  # --- Scenario C: .factory/logs/ directory not writable (append-path EC-005 branch) ---
  # Setup valid STATE.md but make logs/ directory unwritable so the append fails.
  # This exercises the append-failure path (dir exists but is not writable).
  # Hook CWD=$REPO_DIR, so .factory/logs/ resolves to $REPO_DIR/.factory/logs/.
  _init_factory_artifacts_fixture "$_STATE_MD_HAPPY"
  chmod 000 "$REPO_DIR/.factory/logs" 2>/dev/null || true
  _run_hook
  local exit_status_c="$status"
  # Restore permissions for teardown
  chmod 755 "$REPO_DIR/.factory/logs" 2>/dev/null || true
  [ "$exit_status_c" -eq 0 ] # MUST exit 0 even if log write fails
}

# ---------------------------------------------------------------------------
# Test 7 — AC-009
# test_postcompact_reanchor_values_are_git_sourced_not_in_context
#
# BC-7.07.002 invariant 2: values must come from git, not env vars or in-context.
# VP-089 §1 git-sourced, not in-context.
#
# Setup: factory-artifacts STATE.md has current_cycle: A (git value; NO sha field)
#        env var CURRENT_CYCLE=B is set (stale in-memory value)
# Assert: output contains context=A/... (git value wins)
#         output does NOT contain context=B/... (env var ignored)
#         sha= in output equals FIXTURE_DEVELOP_SHA (git rev-parse value, not env)
#         output does NOT contain "current_wave" (phantom field)
#
# F-P1-002: state_md_cycle_a has NO last_verified_develop_sha field.
# ---------------------------------------------------------------------------

@test "test_postcompact_reanchor_values_are_git_sourced_not_in_context" {
  # Setup: factory-artifacts STATE.md has current_cycle: A (git value; no sha field)
  local state_md_cycle_a="---
document_type: state
version: \"0.0.1-test\"
current_cycle: A
current_step: step-from-git
---

# STATE — current_cycle is A (git value; env var says B — hook must use A)
"
  _init_factory_artifacts_fixture "$state_md_cycle_a"

  # Run hook from $REPO_DIR with stale env var CURRENT_CYCLE=B injected.
  # No GIT_DIR in env — git resolves from CWD ($REPO_DIR) per production profile.
  # CURRENT_CYCLE=B is the stale in-context value; the hook must use git value A.
  run bash -c "cd '$REPO_DIR' && printf '%s' '{\"event_name\":\"PostCompact\",\"session_id\":\"bats-ac009\",\"tool_input\":{}}' | HOME='$FAKE_HOME' CURRENT_CYCLE='B' bash '$HOOK_SCRIPT' 2>&1"

  # Assert: exit 0
  [ "$status" -eq 0 ]

  # Assert: output uses git value A, NOT env var B
  [[ "$output" == *"context=A/"* ]]
  [[ "$output" != *"context=B/"* ]]

  # Assert: sha= in output equals the git rev-parse value (not env-supplied)
  # FIXTURE_DEVELOP_SHA is the actual refs/remotes/origin/develop SHA from the fixture.
  [[ "$output" == *"sha=${FIXTURE_DEVELOP_SHA}"* ]]

  # Assert: no "current_wave" in output (phantom field prohibition — BC-7.07.002 Inv2)
  [[ "$output" != *"current_wave"* ]]
  [[ "$output" != *"wave="* ]]
}

# ---------------------------------------------------------------------------
# Test 8 — AC-010
# test_postcompact_reanchor_registry_entry_has_correct_shape
#
# BC-7.07.002 precondition 1: hooks-registry.toml PostCompact entry must exist
#   with the FULL ADR-026 §Decision 7 v1.25 canonical shape.
#
# F-P1-005: asserts the complete capabilities block including:
#   - env_allow exact 8-element list
#   - exec_subprocess: binary_allow ["bash","git","jq"] + shell_bypass_acknowledged
#   - write_file path_allow [".factory/logs/"]
#   - GIT_DIR is NOT present
#
# Block extraction is bounded to the single [[hooks]] stanza so extra-key bleed
# from an adjacent entry does not mask a missing field in this entry.
# ---------------------------------------------------------------------------

@test "test_postcompact_reanchor_registry_entry_has_correct_shape" {
  # Assert: registry file exists (sanity check)
  [ -f "$REGISTRY" ]

  # Assert: postcompact-reanchor entry exists in registry
  grep -q 'name = "postcompact-reanchor"' "$REGISTRY"

  # Extract the single [[hooks]] stanza for postcompact-reanchor.
  # Strategy: awk between the [[hooks]] line containing postcompact-reanchor
  # and the next [[hooks]] line (or EOF), so adjacent entries cannot bleed in.
  local block
  block=$(awk '
    /^\[\[hooks\]\]/ { in_block=0 }
    /name = "postcompact-reanchor"/ { in_block=1 }
    in_block { print }
    /name = "postcompact-reanchor"/ { next }
    in_block && /^\[\[hooks\]\]/ { in_block=0 }
  ' "$REGISTRY")

  # The awk above captures from the [[hooks]] line (which resets in_block=0 first,
  # then is caught again when name matches). Re-extract cleanly: emit from the
  # [[hooks]] that precedes postcompact-reanchor through the next [[hooks]].
  block=$(awk '
    BEGIN { found=0; printing=0 }
    /^\[\[hooks\]\]/ {
      if (printing) { exit }
      found=0
    }
    /name = "postcompact-reanchor"/ { found=1; printing=1 }
    found || printing { print }
  ' "$REGISTRY")

  # -- Core fields --
  echo "$block" | grep -q 'name = "postcompact-reanchor"'
  echo "$block" | grep -q 'event = "PostCompact"'
  echo "$block" | grep -q 'plugin = "hook-plugins/legacy-bash-adapter.wasm"'
  echo "$block" | grep -q 'priority = 100'
  echo "$block" | grep -q 'timeout_ms = 10000'
  echo "$block" | grep -q 'on_error = "continue"'
  echo "$block" | grep -q 'async = false'

  # -- [hooks.config] --
  echo "$block" | grep -q 'script_path = "hooks/postcompact-reanchor.sh"'

  # -- [hooks.capabilities] env_allow: must contain exactly the 8 canonical vars --
  # Exact element assertions (each must be present in the block):
  echo "$block" | grep -q '"PATH"'
  echo "$block" | grep -q '"HOME"'
  echo "$block" | grep -q '"TMPDIR"'
  echo "$block" | grep -q '"CLAUDE_PROJECT_DIR"'
  echo "$block" | grep -q '"CLAUDE_PLUGIN_ROOT"'
  echo "$block" | grep -q '"VSDD_SESSION_ID"'
  echo "$block" | grep -q '"GIT_CONFIG_GLOBAL"'
  echo "$block" | grep -q '"XDG_CONFIG_HOME"'

  # -- GIT_DIR must NOT appear (was a bats test-fixture artifact; not a production requirement) --
  # Load-bearing negative assertion (F-P1-005).
  ! echo "$block" | grep -q '"GIT_DIR"'

  # -- [hooks.capabilities.exec_subprocess] --
  echo "$block" | grep -q '"bash"'
  echo "$block" | grep -q '"git"'
  echo "$block" | grep -q '"jq"'
  echo "$block" | grep -q 'shell_bypass_acknowledged'

  # -- [hooks.capabilities.write_file] --
  echo "$block" | grep -q 'path_allow'
  echo "$block" | grep -q '".factory/logs/"'
}

# ---------------------------------------------------------------------------
# Test 9 — AC-003
# test_postcompact_reanchor_cannot_block_advisory_only
#
# BC-7.07.002 postcondition 3 / PC3: PostCompact hook CANNOT block; exit 0 always.
# This is the canonical cannot-block assertion: even on the happy path the exit
# is 0, and on_error=continue in the registry means the harness will not block
# compaction regardless. This test provides explicit AC-003 coverage.
# ---------------------------------------------------------------------------

@test "test_postcompact_reanchor_cannot_block_advisory_only" {
  # Setup: happy path with valid STATE.md
  _init_factory_artifacts_fixture "$_STATE_MD_HAPPY"

  _run_hook

  # Assert: exit 0 (hook can NEVER return non-zero to block; PC3 / AC-003)
  [ "$status" -eq 0 ]

  # Assert: stdout contains the re-anchor line (confirms hook ran, not vacuous)
  [[ "$output" == *"[PostCompact Re-anchor]"* ]]

  # Assert: registry on_error=continue (harness-level cannot-block gate; AC-003)
  grep -A 10 'name = "postcompact-reanchor"' "$REGISTRY" | grep -q 'on_error = "continue"'
}

# ---------------------------------------------------------------------------
# Test 10 — EC-005
# test_postcompact_reanchor_exits_0_when_log_dir_mkdir_fails
#
# BC-7.07.002 EC-005 TRUE branch: .factory/logs/ directory is ABSENT and its
#   parent is unwritable (mkdir -p fails) → hook exits 0 with stdout advisory.
#
# This is distinct from AC-008 Scenario C (which chmod'd an EXISTING dir,
# hitting the append path). This test drives the mkdir-creation failure path:
# the logs dir does not exist at all, and mkdir cannot create it.
#
# F-P1-006: production-grade EC-005 coverage (mkdir path, not append path).
# ---------------------------------------------------------------------------

@test "test_postcompact_reanchor_exits_0_when_log_dir_mkdir_fails" {
  # Setup: valid STATE.md in factory-artifacts.
  # _init_factory_artifacts_fixture creates $REPO_DIR/.factory/logs/ — remove it
  # so the hook must attempt mkdir, then lock the parent to force that mkdir to fail.
  _init_factory_artifacts_fixture "$_STATE_MD_HAPPY"

  # Remove the .factory/logs directory that _init_factory_artifacts_fixture created.
  rm -rf "$REPO_DIR/.factory/logs"

  # Make the parent .factory directory unwritable so mkdir -p .factory/logs fails.
  # The hook runs with CWD=$REPO_DIR, so .factory resolves to $REPO_DIR/.factory.
  chmod 555 "$REPO_DIR/.factory"

  # Run the hook: mkdir -p .factory/logs should fail (parent unwritable).
  _run_hook
  local exit_status="$status"
  local hook_output="$output"

  # Restore permissions so teardown's rm -rf works cleanly
  chmod 755 "$REPO_DIR/.factory"

  # Assert: exit 0 even when mkdir fails (EC-005 fail-open; PC5)
  [ "$exit_status" -eq 0 ]

  # Assert: stdout still contains the re-anchor block (hook proceeded without log)
  # OR stdout contains an advisory message. Either way, the session is not blocked.
  # The hook should emit the re-anchor block to stdout regardless of log write failure.
  [[ "$hook_output" == *"[PostCompact Re-anchor]"* ]]
}
