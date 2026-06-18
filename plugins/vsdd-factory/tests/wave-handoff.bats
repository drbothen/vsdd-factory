#!/usr/bin/env bats
# wave-handoff.bats — Red Gate tests for the wave-handoff skill
#
# Story:   S-18.01 — HANDOFF.md Schema + wave-handoff Skill; wave-state.yaml Atomic Production
# BCs:     BC-5.41.001 (HANDOFF.md with 9 base required fields + anti-fabrication cross-checks)
#          BC-5.41.002 (wave-state.yaml curated manifest; BrokenSprintState; atomicity)
# VPs:     VP-081 (Wave cannot close without verified HANDOFF.md)
#          VP-087 (atomicity + real-substrate derivation + BrokenSprintState)
#
# RED GATE discipline: every test MUST FAIL against the stub (stubs exit 1, write nothing).
# Tests assert OBSERVABLE OUTPUT: file written, fields present with correct values,
# exact exit codes, exact stderr messages, exactly-one-commit. No tautologies.
#
# POLICY 11 compliance: no test merely invokes the skill and asserts "no error".
# Every test asserts a CONCRETE postcondition (field existence, value pattern,
# file presence/absence, commit count, exact message content).
#
# No jq dependency. No Python. POSIX bash + awk + grep + git.

# ---------------------------------------------------------------------------
# Setup / teardown
# ---------------------------------------------------------------------------

setup() {
  SKILL_DIR="$(cd "${BATS_TEST_DIRNAME}/../skills/wave-handoff" && pwd)"
  SKILL="$SKILL_DIR/wave-handoff.sh"

  # Create a hermetic git repo in a temp directory.
  # Architecture:
  #   WORK/               — the "main" working repo (non-bare)
  #   WORK/factory-wt/   — git worktree checked out on the factory-artifacts orphan branch
  #
  # Step 1: initialise repo on an explicit branch name ("feature-test")
  # so we never depend on the system default branch name (master vs main).
  WORK="$(mktemp -d)"
  ARTIFACTS_WT="$WORK/factory-wt"

  git -C "$WORK" init -q -b feature-test
  git -C "$WORK" config user.email "test@example.com"
  git -C "$WORK" config user.name "Test"

  # Step 2: create a root commit on feature-test so HEAD is valid
  echo "feature-test root" > "$WORK/root.txt"
  git -C "$WORK" add root.txt
  git -C "$WORK" commit -q -m "feature-test root"

  # Step 3: simulate origin/develop — skill calls `git rev-parse origin/develop`
  DEVELOP_SHA="$(git -C "$WORK" rev-parse HEAD)"
  git -C "$WORK" update-ref refs/remotes/origin/develop "$DEVELOP_SHA"

  # Step 4: create the factory-artifacts orphan branch inside the same repo.
  # We save the current branch name first so we can return to it by name
  # after the orphan checkout (git checkout - doesn't work after --orphan).
  local saved_branch
  saved_branch="$(git -C "$WORK" branch --show-current)"
  git -C "$WORK" checkout --orphan factory-artifacts -q
  git -C "$WORK" rm -rf . -q 2>/dev/null || true
  echo "factory-artifacts root" > "$WORK/.gitkeep"
  git -C "$WORK" add .gitkeep
  git -C "$WORK" commit -q -m "factory-artifacts init"
  # Return to the saved branch name
  git -C "$WORK" checkout -q "$saved_branch"

  # Step 5: add a linked worktree for factory-artifacts
  mkdir -p "$ARTIFACTS_WT"
  git -C "$WORK" worktree add -q "$ARTIFACTS_WT" factory-artifacts

  # Step 6: create fixture directories in the artifacts worktree
  mkdir -p "$ARTIFACTS_WT/.factory/hooks"
  mkdir -p "$ARTIFACTS_WT/.factory/specs/behavioral-contracts/ss-05"

  # Create a real BC file so active_bcs check can resolve at least one entry
  echo "# BC-5.41.001 stub" \
    > "$ARTIFACTS_WT/.factory/specs/behavioral-contracts/ss-05/BC-5.41.001.md"

  # Write a default sprint-state.yaml (happy-path: pending + draft story)
  _write_sprint_state_pending

  # Write a minimal STATE.md for wave_id derivation
  _write_state_md "2"
}

teardown() {
  # Remove worktrees before removing WORK
  git -C "$WORK" worktree remove --force "$ARTIFACTS_WT" 2>/dev/null || true
  rm -rf "$WORK"
}

# ---------------------------------------------------------------------------
# Fixture helpers
# ---------------------------------------------------------------------------

_write_sprint_state_pending() {
  cat > "$WORK/sprint-state.yaml" << 'EOF'
stories:
  - id: S-18.02
    status: pending
  - id: S-18.03
    status: draft
EOF
}

_write_sprint_state_all_terminal() {
  cat > "$WORK/sprint-state.yaml" << 'EOF'
stories:
  - id: S-18.02
    status: merged
  - id: S-18.03
    status: cancelled
EOF
}

_write_sprint_state_review_pending() {
  # EC-005 / AC-020 / VP-087 discriminating fixture: review-pending only
  cat > "$WORK/sprint-state.yaml" << 'EOF'
stories:
  - id: S-18.02
    status: review-pending
EOF
}

_write_sprint_state_in_progress() {
  # AC-018 fixture: in-progress with no pending/draft
  cat > "$WORK/sprint-state.yaml" << 'EOF'
stories:
  - id: S-18.02
    status: in-progress
EOF
}

_write_state_md() {
  local step="${1:-2}"
  cat > "$WORK/STATE.md" << EOF
---
current_step: "pass-${step}"
current_cycle: "v1.0-feature-context-durability-E18"
factory_lock: null
---
# STATE
EOF
}

# Run the skill with standard arguments pointing at our hermetic fixtures.
# The skill needs:
#   --artifacts-worktree: path to factory-artifacts worktree
#   --sprint-state: path to sprint-state.yaml
#   --state-md: path to STATE.md
#   --bc-dir: path to directory containing active BCs
# If the skill doesn't support named args yet (stubs don't), we set env vars
# as a fallback convention — the real skill will support both.
_run_skill() {
  run bash -c "
    export ARTIFACTS_WT='${ARTIFACTS_WT}'
    export SPRINT_STATE_YAML='${WORK}/sprint-state.yaml'
    export STATE_MD_PATH='${WORK}/STATE.md'
    export BC_DIR='${ARTIFACTS_WT}/.factory/specs/behavioral-contracts'
    export PRECOMPACT_FLUSH_LOG='${ARTIFACTS_WT}/.factory/hooks/precompact-flush-log'
    export GIT_DIR='${WORK}/.git'
    export FACTORY_REPO='${WORK}'
    '${SKILL}' \
      --artifacts-worktree '${ARTIFACTS_WT}' \
      --sprint-state '${WORK}/sprint-state.yaml' \
      --state-md '${WORK}/STATE.md' \
      --bc-dir '${ARTIFACTS_WT}/.factory/specs/behavioral-contracts' \
      2>&1
  "
}

# Count commits on factory-artifacts branch
_artifact_commit_count() {
  git -C "$WORK" rev-list --count factory-artifacts
}

# Get the log of files changed in the most recent factory-artifacts commit
_artifact_last_commit_files() {
  git -C "$WORK" diff-tree --no-commit-id -r --name-only factory-artifacts
}

# ---------------------------------------------------------------------------
# test_handoff_writes_all_9_base_fields
# AC-001 / BC-5.41.001 PC1
# Asserts HANDOFF.md is written with all 9 required base fields present.
# Red Gate: stub writes nothing → file absent → test fails.
# ---------------------------------------------------------------------------

@test "test_handoff_writes_all_9_base_fields" {
  _run_skill

  # File must exist in the factory-artifacts worktree
  [ -f "$ARTIFACTS_WT/HANDOFF.md" ] || {
    echo "FAIL: HANDOFF.md not written to $ARTIFACTS_WT" >&2
    false
  }

  local content
  content="$(cat "$ARTIFACTS_WT/HANDOFF.md")"

  # Assert all 9 base fields are present (key: value format in YAML)
  for field in wave_id last_verified_develop_sha active_bcs next_wave_stories \
               open_decisions pending_fixes process_gaps precompact_flush_sha \
               factory_lock_holder; do
    echo "$content" | grep -q "^${field}:" || {
      echo "FAIL: field '${field}' missing from HANDOFF.md" >&2
      false
    }
  done
}

# ---------------------------------------------------------------------------
# test_wave_id_derived_from_sprint_state_not_phantom
# AC-002 / BC-5.41.001 PC2
# Asserts wave_id is an integer derived from real substrate (sprint-state or
# STATE.md current_step). Asserts NO `current_wave:` field is read (the field
# does not exist; the skill must NOT produce it either).
# Red Gate: stub writes nothing → HANDOFF.md absent → test fails.
# ---------------------------------------------------------------------------

@test "test_wave_id_derived_from_sprint_state_not_phantom" {
  _run_skill

  [ -f "$ARTIFACTS_WT/HANDOFF.md" ] || {
    echo "FAIL: HANDOFF.md not written" >&2
    false
  }

  local content
  content="$(cat "$ARTIFACTS_WT/HANDOFF.md")"

  # wave_id must be present
  echo "$content" | grep -q "^wave_id:" || {
    echo "FAIL: wave_id field missing from HANDOFF.md" >&2
    false
  }

  # wave_id value must be a positive integer
  local wave_val
  wave_val="$(echo "$content" | grep "^wave_id:" | awk '{print $2}')"
  echo "$wave_val" | grep -qE '^[0-9]+$' || {
    echo "FAIL: wave_id is not a positive integer: '${wave_val}'" >&2
    false
  }

  # HANDOFF.md must NOT contain a `current_wave:` field (phantom field — does not exist)
  echo "$content" | grep -qv "^current_wave:" || {
    echo "FAIL: HANDOFF.md contains phantom 'current_wave:' field" >&2
    false
  }
}

# ---------------------------------------------------------------------------
# test_last_verified_sha_is_40char_hex
# AC-003 / BC-5.41.001 PC3
# Asserts last_verified_develop_sha is a 40-char lowercase hex SHA matching
# git rev-parse origin/develop in the fixture repo.
# Red Gate: stub writes nothing → field absent → pattern match fails.
# ---------------------------------------------------------------------------

@test "test_last_verified_sha_is_40char_hex" {
  _run_skill

  [ -f "$ARTIFACTS_WT/HANDOFF.md" ] || {
    echo "FAIL: HANDOFF.md not written" >&2
    false
  }

  local content
  content="$(cat "$ARTIFACTS_WT/HANDOFF.md")"

  # Extract the SHA value
  local sha_val
  sha_val="$(echo "$content" | grep "^last_verified_develop_sha:" | awk '{print $2}')"

  # Must be exactly 40 lowercase hex chars
  echo "$sha_val" | grep -qE '^[0-9a-f]{40}$' || {
    echo "FAIL: last_verified_develop_sha '${sha_val}' is not 40-char lowercase hex" >&2
    false
  }

  # Must equal the actual origin/develop SHA in the fixture repo
  local expected_sha
  expected_sha="$(git -C "$WORK" rev-parse origin/develop)"
  [ "$sha_val" = "$expected_sha" ] || {
    echo "FAIL: last_verified_develop_sha '${sha_val}' != git rev-parse origin/develop '${expected_sha}'" >&2
    false
  }
}

# ---------------------------------------------------------------------------
# test_active_bcs_nonempty_or_hard_error
# AC-004 / BC-5.41.001 PC4 / EC-007
# When BC dir is empty (no active BCs), skill must exit 1 (hard error).
# Red Gate: stub exits 1 regardless, but for the wrong reason (not yet implemented).
# For the GREEN path (non-empty BC dir), HANDOFF.md must have active_bcs with
# at least one entry. We test both: hard-error on empty, non-empty on happy path.
# ---------------------------------------------------------------------------

@test "test_active_bcs_nonempty_or_hard_error" {
  # First verify the happy path: BC dir has a file → skill must succeed and
  # write active_bcs with at least one entry.
  _run_skill

  [ -f "$ARTIFACTS_WT/HANDOFF.md" ] || {
    echo "FAIL: HANDOFF.md not written (happy path)" >&2
    false
  }

  local content
  content="$(cat "$ARTIFACTS_WT/HANDOFF.md")"

  # active_bcs must be present
  echo "$content" | grep -q "^active_bcs:" || {
    echo "FAIL: active_bcs field missing from HANDOFF.md" >&2
    false
  }

  # active_bcs must NOT be an empty list: must have at least one entry below it
  # (a YAML list entry starts with "  - " on the next line)
  echo "$content" | grep -A 5 "^active_bcs:" | grep -q "^  - " || {
    echo "FAIL: active_bcs is empty — must have at least one BC entry" >&2
    false
  }
}

@test "test_active_bcs_empty_dir_causes_hard_error" {
  # Remove all BC files from the fixture BC dir
  rm -rf "$ARTIFACTS_WT/.factory/specs/behavioral-contracts"
  mkdir -p "$ARTIFACTS_WT/.factory/specs/behavioral-contracts"

  # Skill must exit 1 (hard error per AC-004 / EC-007)
  run bash -c "
    export ARTIFACTS_WT='${ARTIFACTS_WT}'
    export SPRINT_STATE_YAML='${WORK}/sprint-state.yaml'
    export STATE_MD_PATH='${WORK}/STATE.md'
    export BC_DIR='${ARTIFACTS_WT}/.factory/specs/behavioral-contracts'
    export PRECOMPACT_FLUSH_LOG='${ARTIFACTS_WT}/.factory/hooks/precompact-flush-log'
    export GIT_DIR='${WORK}/.git'
    export FACTORY_REPO='${WORK}'
    '${SKILL}' \
      --artifacts-worktree '${ARTIFACTS_WT}' \
      --sprint-state '${WORK}/sprint-state.yaml' \
      --state-md '${WORK}/STATE.md' \
      --bc-dir '${ARTIFACTS_WT}/.factory/specs/behavioral-contracts' \
      2>&1
  "
  [ "$status" -eq 1 ] || {
    echo "FAIL: skill exited ${status}, expected exit 1 on empty active_bcs" >&2
    false
  }
  # HANDOFF.md must NOT be written (no partial write on hard error)
  [ ! -f "$ARTIFACTS_WT/HANDOFF.md" ] || {
    echo "FAIL: HANDOFF.md written despite empty active_bcs — must not write partial artifact" >&2
    false
  }
  # The error message must mention active_bcs or the empty-BC condition —
  # NOT the stub's generic "not yet implemented" placeholder.
  # This discriminates the real implementation's specific error from the stub.
  echo "$output" | grep -qiE "(active_bcs|no active BC|empty.*BC|BC.*empty|NoBCsFound|NoActiveBCs)" || {
    echo "FAIL: error output does not identify active_bcs as the cause." >&2
    echo "Expected mention of active_bcs or empty BC condition." >&2
    echo "Actual output: $output" >&2
    false
  }
}

# ---------------------------------------------------------------------------
# test_precompact_flush_sha_three_state_rule
# AC-009 / BC-5.41.001 PC5 / EC-001/EC-002/EC-003
# The three-state rule:
#   EC-001: log absent → null
#   EC-002: log present but FIELD-4 != "commit" → null
#   EC-003: log present + FIELD-4=commit + valid FIELD-2 → SHA value
# Red Gate: stub writes nothing → all three assertions fail.
# ---------------------------------------------------------------------------

@test "test_precompact_flush_sha_three_state_rule" {
  # --- EC-001: log absent → precompact_flush_sha: null ---
  rm -f "$ARTIFACTS_WT/.factory/hooks/precompact-flush-log"
  _run_skill

  [ -f "$ARTIFACTS_WT/HANDOFF.md" ] || {
    echo "FAIL (EC-001): HANDOFF.md not written when log absent" >&2
    false
  }
  local val_absent
  val_absent="$(grep "^precompact_flush_sha:" "$ARTIFACTS_WT/HANDOFF.md" | awk '{print $2}')"
  [ "$val_absent" = "null" ] || {
    echo "FAIL (EC-001): precompact_flush_sha should be null when log absent, got '${val_absent}'" >&2
    false
  }

  # Clean up for next case
  rm -f "$ARTIFACTS_WT/HANDOFF.md"

  # --- EC-002: log present but FIELD-4 != "commit" → null ---
  echo "2026-06-17T12:00:00Z aabbcc1122334455667788990011aabbccdd1122 cycle/pass-2 pushed" \
    > "$ARTIFACTS_WT/.factory/hooks/precompact-flush-log"
  _run_skill

  [ -f "$ARTIFACTS_WT/HANDOFF.md" ] || {
    echo "FAIL (EC-002): HANDOFF.md not written when log FIELD-4 != commit" >&2
    false
  }
  local val_corrupt
  val_corrupt="$(grep "^precompact_flush_sha:" "$ARTIFACTS_WT/HANDOFF.md" | awk '{print $2}')"
  [ "$val_corrupt" = "null" ] || {
    echo "FAIL (EC-002): precompact_flush_sha should be null when FIELD-4 != commit, got '${val_corrupt}'" >&2
    false
  }

  # Clean up for next case
  rm -f "$ARTIFACTS_WT/HANDOFF.md"

  # --- EC-003: log present + FIELD-4=commit + valid FIELD-2 → SHA value ---
  local expected_sha="aabbccddeeff00112233445566778899aabbccdd"
  echo "2026-06-17T12:00:00Z ${expected_sha} cycle/pass-2 commit" \
    > "$ARTIFACTS_WT/.factory/hooks/precompact-flush-log"
  _run_skill

  [ -f "$ARTIFACTS_WT/HANDOFF.md" ] || {
    echo "FAIL (EC-003): HANDOFF.md not written when log valid" >&2
    false
  }
  local val_valid
  val_valid="$(grep "^precompact_flush_sha:" "$ARTIFACTS_WT/HANDOFF.md" | awk '{print $2}')"
  [ "$val_valid" = "$expected_sha" ] || {
    echo "FAIL (EC-003): precompact_flush_sha should be '${expected_sha}', got '${val_valid}'" >&2
    false
  }
}

# ---------------------------------------------------------------------------
# test_epic_complete_no_wave_state_written
# AC-012 / BC-5.41.001 INV2 + BC-5.41.002 PC5
# When all sprint-state entries are terminal:
#   - HANDOFF.md written with epic_status: complete AND next_wave_stories: []
#   - wave-state.yaml NOT written
#   - stdout contains "EPIC-COMPLETE"
# Red Gate: stub exits 1, writes nothing → all assertions fail.
# ---------------------------------------------------------------------------

@test "test_epic_complete_no_wave_state_written" {
  _write_sprint_state_all_terminal

  run bash -c "
    export ARTIFACTS_WT='${ARTIFACTS_WT}'
    export SPRINT_STATE_YAML='${WORK}/sprint-state.yaml'
    export STATE_MD_PATH='${WORK}/STATE.md'
    export BC_DIR='${ARTIFACTS_WT}/.factory/specs/behavioral-contracts'
    export PRECOMPACT_FLUSH_LOG='${ARTIFACTS_WT}/.factory/hooks/precompact-flush-log'
    export GIT_DIR='${WORK}/.git'
    export FACTORY_REPO='${WORK}'
    '${SKILL}' \
      --artifacts-worktree '${ARTIFACTS_WT}' \
      --sprint-state '${WORK}/sprint-state.yaml' \
      --state-md '${WORK}/STATE.md' \
      --bc-dir '${ARTIFACTS_WT}/.factory/specs/behavioral-contracts' \
      2>&1
  "

  # Must exit 0 on EPIC-COMPLETE
  [ "$status" -eq 0 ] || {
    echo "FAIL: skill exited ${status} on EPIC-COMPLETE, expected 0. Output: $output" >&2
    false
  }

  # HANDOFF.md must be written
  [ -f "$ARTIFACTS_WT/HANDOFF.md" ] || {
    echo "FAIL: HANDOFF.md not written on EPIC-COMPLETE" >&2
    false
  }

  # HANDOFF.md must contain epic_status: complete
  grep -q "^epic_status:.*complete" "$ARTIFACTS_WT/HANDOFF.md" || {
    echo "FAIL: HANDOFF.md missing 'epic_status: complete' on EPIC-COMPLETE" >&2
    false
  }

  # HANDOFF.md must contain next_wave_stories: []
  grep -q "^next_wave_stories: \[\]" "$ARTIFACTS_WT/HANDOFF.md" || {
    echo "FAIL: HANDOFF.md missing 'next_wave_stories: []' on EPIC-COMPLETE" >&2
    false
  }

  # wave-state.yaml must NOT be written
  [ ! -f "$ARTIFACTS_WT/wave-state.yaml" ] || {
    echo "FAIL: wave-state.yaml written on EPIC-COMPLETE — must not be written" >&2
    false
  }

  # stdout must contain EPIC-COMPLETE announcement
  echo "$output" | grep -qi "EPIC-COMPLETE" || {
    echo "FAIL: stdout does not contain 'EPIC-COMPLETE' announcement. Output: $output" >&2
    false
  }
}

# ---------------------------------------------------------------------------
# test_wave_state_has_6_required_fields
# AC-013 / BC-5.41.002 PC1
# wave-state.yaml must contain all 6 required fields:
#   wave_id, generated_at, generated_from_handoff_sha, stories, arch_files, state_pointer
# Red Gate: stub writes nothing → file absent → test fails.
# ---------------------------------------------------------------------------

@test "test_wave_state_has_6_required_fields" {
  _run_skill

  [ -f "$ARTIFACTS_WT/wave-state.yaml" ] || {
    echo "FAIL: wave-state.yaml not written" >&2
    false
  }

  local content
  content="$(cat "$ARTIFACTS_WT/wave-state.yaml")"

  for field in wave_id generated_at generated_from_handoff_sha stories arch_files state_pointer; do
    echo "$content" | grep -q "^${field}:" || {
      echo "FAIL: field '${field}' missing from wave-state.yaml" >&2
      false
    }
  done
}

# ---------------------------------------------------------------------------
# test_wave_state_stories_from_sprint_state_only
# AC-015 / BC-5.41.002 PC3
# stories list in wave-state.yaml must contain only stories from sprint-state.yaml
# with status: pending or status: draft. No phantom wave frontmatter, no RAG.
# Red Gate: stub writes nothing → file absent → test fails.
# ---------------------------------------------------------------------------

@test "test_wave_state_stories_from_sprint_state_only" {
  # sprint-state has S-18.02 (pending) and S-18.03 (draft)
  _write_sprint_state_pending
  _run_skill

  [ -f "$ARTIFACTS_WT/wave-state.yaml" ] || {
    echo "FAIL: wave-state.yaml not written" >&2
    false
  }

  local content
  content="$(cat "$ARTIFACTS_WT/wave-state.yaml")"

  # S-18.02 (pending) must appear
  echo "$content" | grep -q "S-18.02" || {
    echo "FAIL: S-18.02 (status:pending) missing from wave-state.yaml stories" >&2
    false
  }

  # S-18.03 (draft) must appear
  echo "$content" | grep -q "S-18.03" || {
    echo "FAIL: S-18.03 (status:draft) missing from wave-state.yaml stories" >&2
    false
  }

  # state_pointer must be ".factory/STATE.md" (BC-5.41.002 PC2 field spec)
  local state_ptr
  state_ptr="$(echo "$content" | grep "^state_pointer:" | awk '{print $2}')"
  [ "$state_ptr" = ".factory/STATE.md" ] || {
    echo "FAIL: state_pointer should be '.factory/STATE.md', got '${state_ptr}'" >&2
    false
  }
}

# ---------------------------------------------------------------------------
# test_atomic_single_commit_both_files
# AC-017 / BC-5.41.002 PC6 / VP-087
# HANDOFF.md and wave-state.yaml must be committed in a SINGLE git commit to
# factory-artifacts. Exactly +1 new commit; the single commit touches BOTH files.
# Red Gate: stub exits 1, writes nothing → commit count unchanged → test fails.
# ---------------------------------------------------------------------------

@test "test_atomic_single_commit_both_files" {
  local before_count
  before_count="$(_artifact_commit_count)"

  _run_skill

  local after_count
  after_count="$(_artifact_commit_count)"

  # Must be exactly +1 commit
  local delta=$(( after_count - before_count ))
  [ "$delta" -eq 1 ] || {
    echo "FAIL: expected exactly 1 new commit on factory-artifacts, got ${delta} new commits" >&2
    false
  }

  # The single new commit must contain BOTH HANDOFF.md and wave-state.yaml
  local changed_files
  changed_files="$(_artifact_last_commit_files)"

  echo "$changed_files" | grep -q "HANDOFF.md" || {
    echo "FAIL: HANDOFF.md not in the atomic commit. Files changed: ${changed_files}" >&2
    false
  }

  echo "$changed_files" | grep -q "wave-state.yaml" || {
    echo "FAIL: wave-state.yaml not in the atomic commit. Files changed: ${changed_files}" >&2
    false
  }
}

# ---------------------------------------------------------------------------
# test_commit_message_format
# AC-011 / BC-5.41.001 INV1
# Commit message on factory-artifacts must match:
#   HANDOFF wave-<N> <ISO-timestamp>
# Red Gate: stub writes nothing → no new commit → message check fails.
# ---------------------------------------------------------------------------

@test "test_commit_message_format" {
  _run_skill

  # Get the most recent commit message on factory-artifacts
  local msg
  msg="$(git -C "$WORK" log -1 --format='%s' factory-artifacts)"

  # Must match: HANDOFF wave-<integer> <ISO-timestamp (YYYY-MM-DDTHH:MM:SSZ or similar)>
  echo "$msg" | grep -qE '^HANDOFF wave-[0-9]+ [0-9]{4}-[0-9]{2}-[0-9]{2}T[0-9]{2}:[0-9]{2}:[0-9]{2}' || {
    echo "FAIL: commit message '${msg}' does not match 'HANDOFF wave-<N> <ISO-timestamp>' format" >&2
    false
  }
}

# ---------------------------------------------------------------------------
# test_generated_from_handoff_sha_equals_handoff_commit
# AC-014 / BC-5.41.002 PC2
# generated_from_handoff_sha in wave-state.yaml must equal the SHA of the
# HANDOFF.md commit (same commit, since they are atomic).
# Red Gate: stub writes nothing → wave-state.yaml absent → test fails.
# ---------------------------------------------------------------------------

@test "test_generated_from_handoff_sha_equals_handoff_commit" {
  _run_skill

  [ -f "$ARTIFACTS_WT/wave-state.yaml" ] || {
    echo "FAIL: wave-state.yaml not written" >&2
    false
  }

  local handoff_commit_sha
  handoff_commit_sha="$(git -C "$WORK" rev-parse factory-artifacts)"

  local gen_sha
  gen_sha="$(grep "^generated_from_handoff_sha:" "$ARTIFACTS_WT/wave-state.yaml" | awk '{print $2}')"

  # Must be a 40-char hex SHA
  echo "$gen_sha" | grep -qE '^[0-9a-f]{40}$' || {
    echo "FAIL: generated_from_handoff_sha '${gen_sha}' is not a 40-char hex SHA" >&2
    false
  }

  # Must equal the factory-artifacts HEAD (which is the HANDOFF commit)
  [ "$gen_sha" = "$handoff_commit_sha" ] || {
    echo "FAIL: generated_from_handoff_sha '${gen_sha}' != factory-artifacts HEAD '${handoff_commit_sha}'" >&2
    false
  }
}

# ---------------------------------------------------------------------------
# test_broken_sprint_state_canonical_message
# AC-018 / BC-5.41.002 PC7 + INV1
# When sprint-state has non-terminal, non-pending/draft entries (e.g., in-progress),
# skill must exit 1 with the EXACT canonical BrokenSprintState message.
# Red Gate: stub exits 1 with "TODO S-18.01: wave-handoff not yet implemented",
# which does NOT match the canonical message → test fails.
# ---------------------------------------------------------------------------

@test "test_broken_sprint_state_canonical_message" {
  _write_sprint_state_in_progress

  run bash -c "
    export ARTIFACTS_WT='${ARTIFACTS_WT}'
    export SPRINT_STATE_YAML='${WORK}/sprint-state.yaml'
    export STATE_MD_PATH='${WORK}/STATE.md'
    export BC_DIR='${ARTIFACTS_WT}/.factory/specs/behavioral-contracts'
    export PRECOMPACT_FLUSH_LOG='${ARTIFACTS_WT}/.factory/hooks/precompact-flush-log'
    export GIT_DIR='${WORK}/.git'
    export FACTORY_REPO='${WORK}'
    '${SKILL}' \
      --artifacts-worktree '${ARTIFACTS_WT}' \
      --sprint-state '${WORK}/sprint-state.yaml' \
      --state-md '${WORK}/STATE.md' \
      --bc-dir '${ARTIFACTS_WT}/.factory/specs/behavioral-contracts' \
      2>&1
  "

  # Must exit 1
  [ "$status" -eq 1 ] || {
    echo "FAIL: skill exited ${status} on BrokenSprintState, expected exit 1. Output: $output" >&2
    false
  }

  # Must emit the EXACT canonical message (from ADR-026 §Terminal-Wave Discriminator
  # and BC-5.41.002 PC3 + v1.7 fix burst)
  local canonical="BrokenSprintState: stories in non-terminal, non-pending states exist but no next-wave stories are pending/draft. Update sprint-state.yaml to reflect actual story states."
  echo "$output" | grep -qF "$canonical" || {
    echo "FAIL: canonical BrokenSprintState message not found in output." >&2
    echo "Expected substring: ${canonical}" >&2
    echo "Actual output: $output" >&2
    false
  }

  # HANDOFF.md must NOT be written (no partial write on hard error)
  [ ! -f "$ARTIFACTS_WT/HANDOFF.md" ] || {
    echo "FAIL: HANDOFF.md written despite BrokenSprintState — must not write partial artifact" >&2
    false
  }

  # wave-state.yaml must NOT be written
  [ ! -f "$ARTIFACTS_WT/wave-state.yaml" ] || {
    echo "FAIL: wave-state.yaml written despite BrokenSprintState" >&2
    false
  }
}

# ---------------------------------------------------------------------------
# test_review_pending_triggers_broken_sprint_state
# AC-020 / BC-5.41.002 INV2 / VP-087 discriminating fixture
# status: review-pending is NOT terminal AND NOT pending/draft → BrokenSprintState.
# Red Gate: stub exits 1 with wrong message → canonical message check fails.
# ---------------------------------------------------------------------------

@test "test_review_pending_triggers_broken_sprint_state" {
  _write_sprint_state_review_pending

  run bash -c "
    export ARTIFACTS_WT='${ARTIFACTS_WT}'
    export SPRINT_STATE_YAML='${WORK}/sprint-state.yaml'
    export STATE_MD_PATH='${WORK}/STATE.md'
    export BC_DIR='${ARTIFACTS_WT}/.factory/specs/behavioral-contracts'
    export PRECOMPACT_FLUSH_LOG='${ARTIFACTS_WT}/.factory/hooks/precompact-flush-log'
    export GIT_DIR='${WORK}/.git'
    export FACTORY_REPO='${WORK}'
    '${SKILL}' \
      --artifacts-worktree '${ARTIFACTS_WT}' \
      --sprint-state '${WORK}/sprint-state.yaml' \
      --state-md '${WORK}/STATE.md' \
      --bc-dir '${ARTIFACTS_WT}/.factory/specs/behavioral-contracts' \
      2>&1
  "

  # Must exit 1 — review-pending is a non-terminal, non-pending/draft state
  [ "$status" -eq 1 ] || {
    echo "FAIL: skill exited ${status} on review-pending sprint state, expected exit 1. Output: $output" >&2
    false
  }

  # Must emit canonical BrokenSprintState message
  local canonical="BrokenSprintState: stories in non-terminal, non-pending states exist but no next-wave stories are pending/draft. Update sprint-state.yaml to reflect actual story states."
  echo "$output" | grep -qF "$canonical" || {
    echo "FAIL: BrokenSprintState message not emitted for review-pending status." >&2
    echo "Expected: ${canonical}" >&2
    echo "Actual: $output" >&2
    false
  }

  # No files must be written
  [ ! -f "$ARTIFACTS_WT/HANDOFF.md" ] || {
    echo "FAIL: HANDOFF.md written on review-pending BrokenSprintState" >&2
    false
  }
  [ ! -f "$ARTIFACTS_WT/wave-state.yaml" ] || {
    echo "FAIL: wave-state.yaml written on review-pending BrokenSprintState" >&2
    false
  }
}

# ---------------------------------------------------------------------------
# test_sprint_state_absent_treated_as_epic_complete
# EC-006 / BC-5.41.002 PC3 EPIC-COMPLETE exception
# When sprint-state.yaml is absent entirely, treat as empty → EPIC-COMPLETE path.
# Red Gate: stub exits 1, writes nothing → test fails on exit code.
# ---------------------------------------------------------------------------

@test "test_sprint_state_absent_treated_as_epic_complete" {
  rm -f "$WORK/sprint-state.yaml"

  run bash -c "
    export ARTIFACTS_WT='${ARTIFACTS_WT}'
    export SPRINT_STATE_YAML='${WORK}/sprint-state.yaml'
    export STATE_MD_PATH='${WORK}/STATE.md'
    export BC_DIR='${ARTIFACTS_WT}/.factory/specs/behavioral-contracts'
    export PRECOMPACT_FLUSH_LOG='${ARTIFACTS_WT}/.factory/hooks/precompact-flush-log'
    export GIT_DIR='${WORK}/.git'
    export FACTORY_REPO='${WORK}'
    '${SKILL}' \
      --artifacts-worktree '${ARTIFACTS_WT}' \
      --sprint-state '${WORK}/sprint-state.yaml' \
      --state-md '${WORK}/STATE.md' \
      --bc-dir '${ARTIFACTS_WT}/.factory/specs/behavioral-contracts' \
      2>&1
  "

  # Must exit 0 (absent sprint-state = empty = EPIC-COMPLETE)
  [ "$status" -eq 0 ] || {
    echo "FAIL: skill exited ${status} when sprint-state.yaml absent, expected 0. Output: $output" >&2
    false
  }

  # HANDOFF.md must be written
  [ -f "$ARTIFACTS_WT/HANDOFF.md" ] || {
    echo "FAIL: HANDOFF.md not written when sprint-state.yaml absent (EPIC-COMPLETE path)" >&2
    false
  }

  # Must include epic_status: complete
  grep -q "^epic_status:.*complete" "$ARTIFACTS_WT/HANDOFF.md" || {
    echo "FAIL: HANDOFF.md missing epic_status: complete when sprint-state.yaml absent" >&2
    false
  }

  # wave-state.yaml must NOT be written
  [ ! -f "$ARTIFACTS_WT/wave-state.yaml" ] || {
    echo "FAIL: wave-state.yaml written when sprint-state.yaml absent (EPIC-COMPLETE path)" >&2
    false
  }
}
