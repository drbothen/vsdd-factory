#!/usr/bin/env bats
# wave-handoff.bats — Red Gate tests for the wave-handoff skill
#
# Story:   S-18.01 — HANDOFF.md Schema + wave-handoff Skill; wave-state.yaml Atomic Production
# BCs:     BC-5.41.001 v1.17 (HANDOFF.md with 9 base required fields + anti-fabrication cross-checks)
#          BC-5.41.002 v1.13 (wave-state.yaml curated manifest; BrokenSprintState; atomicity;
#                              AC-014 clarified: generated_from_handoff_sha = PRIOR HANDOFF commit SHA)
# VPs:     VP-081 (Wave cannot close without verified HANDOFF.md)
#          VP-087 (atomicity + real-substrate derivation + BrokenSprintState)
#
# RED GATE discipline: every test MUST FAIL against the implementation at d59ffa97 for the
# RIGHT REASON — not build errors, but assertion failures on the COMMITTED artifact.
#
# POLICY 11 compliance: no test merely invokes the skill and asserts "no error".
# Every test asserts a CONCRETE postcondition (field existence, value pattern,
# file presence/absence, commit count, exact message content).
#
# CRITICAL INVARIANT (F-S1801-P1-002 / VP-087 proof harness): for artifacts whose
# consumer reads the committed branch, assertions MUST use:
#   git -C <repo> show <branch>:<file>
# NOT the working-tree file. The working tree may differ from the committed blob
# (the current impl awk-patches the working tree AFTER committing via update-ref).
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
  mkdir -p "$ARTIFACTS_WT/.factory/specs/architecture/decisions"
  mkdir -p "$ARTIFACTS_WT/.factory/stories"

  # Create a real BC file so active_bcs check can resolve at least one entry
  echo "# BC-5.41.001 stub" \
    > "$ARTIFACTS_WT/.factory/specs/behavioral-contracts/ss-05/BC-5.41.001.md"

  # Create real architecture files for arch_files path resolution (F-S1801-P1-004)
  echo "# ARCH-INDEX" > "$ARTIFACTS_WT/.factory/specs/architecture/ARCH-INDEX.md"
  echo "# ADR-026" > "$ARTIFACTS_WT/.factory/specs/architecture/decisions/ADR-026-wave-boundary-checkpoint-reset-and-lossless-intra-wave-compaction.md"
  echo "# ADR-025" > "$ARTIFACTS_WT/.factory/specs/architecture/decisions/ADR-025-single-writer-factory-locklease-prevent-concurrent-session-races-on-factory-artifacts-orphan-branch.md"

  # Create STORY-INDEX.md with S-18.02 and S-18.03 entries (F-S1801-P1-005)
  cat > "$ARTIFACTS_WT/.factory/stories/STORY-INDEX.md" << 'EOF'
# STORY-INDEX
| ID | Title | Status |
|----|-------|--------|
| S-18.02 | Stub story 02 | pending |
| S-18.03 | Stub story 03 | draft |
EOF

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
# test_precompact_sha_mismatch_hard_blocks
# F-S1801-P1-006 / BC-5.41.001 PC5 / EC-011
# When the precompact-flush-log is present + valid (FIELD-4=commit, valid SHA in FIELD-2)
# but the HANDOFF.md written would have a null or mismatched precompact_flush_sha,
# the skill MUST exit 1 with PrecompactShaMismatch.
# Covers: the current impl silently auto-populates from the log (no hard-block for a null
# that contradicts the log); this test REDs that path.
# ---------------------------------------------------------------------------

@test "test_precompact_sha_mismatch_hard_blocks" {
  # Setup: valid precompact-flush-log exists
  local log_sha="aabbccddeeff00112233445566778899aabbccdd"
  echo "2026-06-17T12:00:00Z ${log_sha} cycle/pass-2 commit" \
    > "$ARTIFACTS_WT/.factory/hooks/precompact-flush-log"

  # Inject a conflicting (null) precompact_flush_sha into the environment so the skill
  # would write null — simulating a context that has "forgotten" the precompact SHA while
  # the log still shows it. We force this by passing an env override that the skill must
  # cross-check against the log.
  # If the skill correctly implements PC5: log present + FIELD-4=commit + null SHA → HARD BLOCK.
  run bash -c "
    export ARTIFACTS_WT='${ARTIFACTS_WT}'
    export SPRINT_STATE_YAML='${WORK}/sprint-state.yaml'
    export STATE_MD_PATH='${WORK}/STATE.md'
    export BC_DIR='${ARTIFACTS_WT}/.factory/specs/behavioral-contracts'
    export PRECOMPACT_FLUSH_LOG='${ARTIFACTS_WT}/.factory/hooks/precompact-flush-log'
    export GIT_DIR='${WORK}/.git'
    export FACTORY_REPO='${WORK}'
    export FORCE_PRECOMPACT_SHA=null
    '${SKILL}' \
      --artifacts-worktree '${ARTIFACTS_WT}' \
      --sprint-state '${WORK}/sprint-state.yaml' \
      --state-md '${WORK}/STATE.md' \
      --bc-dir '${ARTIFACTS_WT}/.factory/specs/behavioral-contracts' \
      2>&1
  "

  # Must exit 1 — PrecompactShaMismatch hard block
  [ "$status" -eq 1 ] || {
    echo "FAIL: skill exited ${status}, expected exit 1 on PrecompactShaMismatch." >&2
    echo "When precompact-flush-log contains a valid commit SHA and FORCE_PRECOMPACT_SHA=null" >&2
    echo "is injected, the skill must hard-block (PC5 EC-011), not silently accept null." >&2
    echo "Actual output: $output" >&2
    false
  }

  # Must emit PrecompactShaMismatch in output
  echo "$output" | grep -qi "PrecompactShaMismatch" || {
    echo "FAIL: PrecompactShaMismatch not in output. Got: $output" >&2
    false
  }

  # HANDOFF.md must NOT be written
  [ ! -f "$ARTIFACTS_WT/HANDOFF.md" ] || {
    echo "FAIL: HANDOFF.md written despite PrecompactShaMismatch — must not write partial artifact" >&2
    false
  }
}

# ---------------------------------------------------------------------------
# test_epic_complete_canonical_stdout_message
# F-S1801-P1-007 / BC-5.41.002 PC7 / BC-5.41.001 PC8
# When EPIC-COMPLETE, stdout must contain the CANONICAL multi-line message from
# BC-5.41.002 PC7, including the <epic-id> derived from STATE.md current_cycle.
# The current impl emits a one-liner "EPIC-COMPLETE: all stories in terminal state"
# which does NOT match the canonical 3-line form.
# ---------------------------------------------------------------------------

@test "test_epic_complete_canonical_stdout_message" {
  _write_sprint_state_all_terminal
  _write_state_md "3"

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

  # Must exit 0
  [ "$status" -eq 0 ] || {
    echo "FAIL: skill exited ${status} on EPIC-COMPLETE, expected 0. Output: $output" >&2
    false
  }

  # Must contain line 1: "EPIC-COMPLETE: All stories in sprint-state.yaml have reached terminal status."
  echo "$output" | grep -qF "EPIC-COMPLETE: All stories in sprint-state.yaml have reached terminal status." || {
    echo "FAIL: canonical EPIC-COMPLETE line 1 missing." >&2
    echo "Expected: 'EPIC-COMPLETE: All stories in sprint-state.yaml have reached terminal status.'" >&2
    echo "Actual output: $output" >&2
    false
  }

  # Must contain line 2 with the epic-id derived from STATE.md current_cycle.
  # STATE.md has current_cycle: "v1.0-feature-context-durability-E18" → epic-id = v1.0-feature-context-durability-E18
  echo "$output" | grep -qF "v1.0-feature-context-durability-E18" || {
    echo "FAIL: canonical EPIC-COMPLETE line 2 missing epic-id from STATE.md current_cycle." >&2
    echo "Expected output to contain 'v1.0-feature-context-durability-E18'" >&2
    echo "Actual output: $output" >&2
    false
  }

  # Must contain line 3: "HANDOFF.md committed to factory-artifacts with epic_status: complete."
  echo "$output" | grep -qF "HANDOFF.md committed to factory-artifacts with epic_status: complete." || {
    echo "FAIL: canonical EPIC-COMPLETE line 3 missing." >&2
    echo "Expected: 'HANDOFF.md committed to factory-artifacts with epic_status: complete.'" >&2
    echo "Actual output: $output" >&2
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

  # wave-state.yaml must NOT be written to working tree
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
# test_epic_complete_no_stale_wave_state_in_commit
# F-S1801-P1-008 / BC-5.41.002 PC3 EPIC-COMPLETE exception
# When wave-state.yaml PRE-EXISTS on factory-artifacts (from a prior wave),
# and the EPIC-COMPLETE path runs, the resulting commit tree MUST NOT contain
# wave-state.yaml. Assert via `git show <commit>:wave-state.yaml` failing.
# REDs: current impl doesn't delete a pre-existing wave-state.yaml from the tree.
# ---------------------------------------------------------------------------

@test "test_epic_complete_no_stale_wave_state_in_commit" {
  # Plant a pre-existing wave-state.yaml on factory-artifacts (simulates prior wave)
  echo "wave_id: 1" > "$ARTIFACTS_WT/wave-state.yaml"
  git -C "$ARTIFACTS_WT" add wave-state.yaml
  git -C "$ARTIFACTS_WT" -c user.email="test@example.com" -c user.name="Test" \
    commit -q -m "HANDOFF wave-1 2026-06-01T00:00:00Z"

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

  # Must exit 0
  [ "$status" -eq 0 ] || {
    echo "FAIL: skill exited ${status} on EPIC-COMPLETE with pre-existing wave-state.yaml. Output: $output" >&2
    false
  }

  # The EPIC-COMPLETE commit must NOT contain wave-state.yaml in the committed tree.
  # Use `git show <commit>:<file>` — failure means the file is absent from the tree.
  # Success (exit 0) means the stale wave-state.yaml is still in the tree — a FAIL.
  local latest_commit
  latest_commit="$(git -C "$WORK" rev-parse factory-artifacts)"
  if git -C "$WORK" show "${latest_commit}:wave-state.yaml" >/dev/null 2>&1; then
    echo "FAIL: wave-state.yaml still exists in committed tree after EPIC-COMPLETE." >&2
    echo "EPIC-COMPLETE must remove wave-state.yaml from the commit tree." >&2
    echo "Latest factory-artifacts commit: ${latest_commit}" >&2
    false
  fi
}

# ---------------------------------------------------------------------------
# test_wave_state_has_6_required_fields_in_committed_blob
# F-S1801-P1-002 + AC-013 / BC-5.41.002 PC1
# wave-state.yaml must contain all 6 required fields in the COMMITTED blob
# on factory-artifacts (read via `git show factory-artifacts:wave-state.yaml`),
# NOT in the working-tree file.
# REDs: current impl awk-patches the working-tree AFTER committing via update-ref,
# so the committed blob still has the parent SHA placeholder — this test reads the
# committed blob directly.
# ---------------------------------------------------------------------------

@test "test_wave_state_has_6_required_fields_in_committed_blob" {
  _run_skill

  # Assert the committed blob exists (not just the working-tree file)
  git -C "$WORK" show factory-artifacts:wave-state.yaml >/dev/null 2>&1 || {
    echo "FAIL: wave-state.yaml not in committed factory-artifacts tree" >&2
    false
  }

  # Read the COMMITTED blob — the consumer reads this, not the working tree
  local committed_content
  committed_content="$(git -C "$WORK" show factory-artifacts:wave-state.yaml)"

  for field in wave_id generated_at generated_from_handoff_sha stories arch_files state_pointer; do
    echo "$committed_content" | grep -q "^${field}:" || {
      echo "FAIL: field '${field}' missing from COMMITTED wave-state.yaml blob" >&2
      echo "Committed content: ${committed_content}" >&2
      false
    }
  done
}

# ---------------------------------------------------------------------------
# test_wave_state_stories_from_sprint_state_only_in_committed_blob
# F-S1801-P1-002 + AC-015 / BC-5.41.002 PC3
# stories list in the COMMITTED wave-state.yaml blob must contain only stories
# from sprint-state.yaml with status: pending or status: draft.
# Reads the committed blob via `git show`, not the working-tree file.
# ---------------------------------------------------------------------------

@test "test_wave_state_stories_from_sprint_state_only_in_committed_blob" {
  # sprint-state has S-18.02 (pending) and S-18.03 (draft)
  _write_sprint_state_pending
  _run_skill

  # Read the COMMITTED blob
  git -C "$WORK" show factory-artifacts:wave-state.yaml >/dev/null 2>&1 || {
    echo "FAIL: wave-state.yaml not in committed factory-artifacts tree" >&2
    false
  }

  local committed_content
  committed_content="$(git -C "$WORK" show factory-artifacts:wave-state.yaml)"

  # S-18.02 (pending) must appear in committed blob
  echo "$committed_content" | grep -q "S-18.02" || {
    echo "FAIL: S-18.02 (status:pending) missing from COMMITTED wave-state.yaml stories" >&2
    false
  }

  # S-18.03 (draft) must appear in committed blob
  echo "$committed_content" | grep -q "S-18.03" || {
    echo "FAIL: S-18.03 (status:draft) missing from COMMITTED wave-state.yaml stories" >&2
    false
  }

  # state_pointer must be ".factory/STATE.md" in committed blob (BC-5.41.002 PC2 field spec)
  local state_ptr
  state_ptr="$(echo "$committed_content" | grep "^state_pointer:" | awk '{print $2}')"
  [ "$state_ptr" = ".factory/STATE.md" ] || {
    echo "FAIL: state_pointer in COMMITTED blob should be '.factory/STATE.md', got '${state_ptr}'" >&2
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
# test_worktree_clean_after_commit
# F-S1801-P1-003 / VP-087 atomicity
# After a successful has-next-wave run, the factory-artifacts worktree MUST be
# clean — no uncommitted / dangling wave-state.yaml or HANDOFF.md changes.
# REDs: current impl awk-patches the working-tree AFTER committing, leaving a
# dirty worktree (the patched wave-state.yaml is never staged or committed).
# ---------------------------------------------------------------------------

@test "test_worktree_clean_after_commit" {
  _run_skill

  [ "$status" -eq 0 ] || {
    echo "FAIL: skill exited ${status}, expected 0. Output: $output" >&2
    false
  }

  # Assert the factory-artifacts worktree is clean (no uncommitted changes)
  local porcelain_output
  porcelain_output="$(git -C "$ARTIFACTS_WT" status --porcelain)"
  [ -z "$porcelain_output" ] || {
    echo "FAIL: factory-artifacts worktree is dirty after skill run." >&2
    echo "git status --porcelain output:" >&2
    echo "$porcelain_output" >&2
    echo "The working-tree must match the committed tree after the atomic commit." >&2
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
# test_generated_from_handoff_sha_is_prior_handoff_commit_in_committed_blob
# F-S1801-P1-002 + AC-014 v1.4 / BC-5.41.002 PC2
# generated_from_handoff_sha in the COMMITTED wave-state.yaml blob must equal
# the factory-artifacts HEAD SHA captured BEFORE the atomic commit (the prior
# verified HANDOFF.md commit SHA), NOT the SHA of the commit that contains this
# wave-state.yaml (cryptographic fixed-point — infeasible).
#
# Two assertions:
#   (A) has-next-wave case: assert committed blob's generated_from_handoff_sha
#       equals prior_handoff_sha (factory-artifacts HEAD before skill ran), not
#       the self-commit SHA.
#   (B) wave-1 null case: when no prior HANDOFF commit exists on factory-artifacts,
#       generated_from_handoff_sha must be null in the committed blob.
#
# REDs current impl on:
#   - Reading working-tree instead of committed blob (the working-tree is awk-patched
#     to commit_sha AFTER the commit — the committed blob has a different value)
#   - Old test asserted gen_sha == factory-artifacts HEAD (new commit SHA), which was
#     wrong per the clarified spec; now asserts gen_sha == prior_handoff_sha
# ---------------------------------------------------------------------------

@test "test_generated_from_handoff_sha_is_prior_handoff_commit_in_committed_blob" {
  # --- Part A: has-next-wave with existing prior HANDOFF commit ---
  # Create a prior HANDOFF commit on factory-artifacts so there IS a prior SHA
  local prior_handoff_sha
  echo "prior: true" > "$ARTIFACTS_WT/HANDOFF.md"
  git -C "$ARTIFACTS_WT" add HANDOFF.md
  git -C "$ARTIFACTS_WT" -c user.email="test@example.com" -c user.name="Test" \
    commit -q -m "HANDOFF wave-1 2026-06-01T00:00:00Z"
  prior_handoff_sha="$(git -C "$WORK" rev-parse factory-artifacts)"
  # Remove the HANDOFF.md from worktree so the skill writes a fresh one
  rm -f "$ARTIFACTS_WT/HANDOFF.md"

  _run_skill

  [ "$status" -eq 0 ] || {
    echo "FAIL (Part A): skill exited ${status}. Output: $output" >&2
    false
  }

  # Read the COMMITTED blob (VP-087 proof harness: use git show)
  git -C "$WORK" show factory-artifacts:wave-state.yaml >/dev/null 2>&1 || {
    echo "FAIL (Part A): wave-state.yaml not in committed factory-artifacts tree" >&2
    false
  }
  local committed_content
  committed_content="$(git -C "$WORK" show factory-artifacts:wave-state.yaml)"

  # Extract generated_from_handoff_sha from COMMITTED blob
  local gen_sha
  gen_sha="$(echo "$committed_content" | grep "^generated_from_handoff_sha:" | awk '{print $2}')"

  # Must be a 40-char hex SHA (not null, not placeholder)
  echo "$gen_sha" | grep -qE '^[0-9a-f]{40}$' || {
    echo "FAIL (Part A): generated_from_handoff_sha in COMMITTED blob '${gen_sha}' is not a 40-char hex SHA" >&2
    false
  }

  # Must equal prior_handoff_sha (the factory-artifacts HEAD BEFORE the atomic commit),
  # NOT the new commit SHA (cryptographic fixed-point contradiction per AC-014 v1.4).
  local new_commit_sha
  new_commit_sha="$(git -C "$WORK" rev-parse factory-artifacts)"
  [ "$gen_sha" = "$prior_handoff_sha" ] || {
    echo "FAIL (Part A): generated_from_handoff_sha in COMMITTED blob '${gen_sha}'" >&2
    echo "  expected: prior_handoff_sha='${prior_handoff_sha}' (factory-artifacts HEAD BEFORE commit)" >&2
    echo "  self-commit SHA='${new_commit_sha}' (MUST NOT be used — cryptographic fixed-point)" >&2
    false
  }

  # --- Part B: wave-1 null case — no prior HANDOFF commit on factory-artifacts ---
  # Use a completely fresh hermetic repo so we don't touch the current WORK fixture.
  local WORK2
  WORK2="$(mktemp -d)"
  local ARTIFACTS_WT2="${WORK2}/factory-wt2"

  git -C "$WORK2" init -q -b feature-test2
  git -C "$WORK2" config user.email "test@example.com"
  git -C "$WORK2" config user.name "Test"

  echo "root" > "$WORK2/root.txt"
  git -C "$WORK2" add root.txt
  git -C "$WORK2" commit -q -m "root"
  git -C "$WORK2" update-ref refs/remotes/origin/develop "$(git -C "$WORK2" rev-parse HEAD)"

  local saved_branch2
  saved_branch2="$(git -C "$WORK2" branch --show-current)"
  git -C "$WORK2" checkout --orphan factory-artifacts -q
  git -C "$WORK2" rm -rf . -q 2>/dev/null || true
  echo "factory-artifacts root" > "$WORK2/.gitkeep"
  git -C "$WORK2" add .gitkeep
  git -C "$WORK2" commit -q -m "factory-artifacts init"
  git -C "$WORK2" checkout -q "$saved_branch2"

  mkdir -p "$ARTIFACTS_WT2"
  git -C "$WORK2" worktree add -q "$ARTIFACTS_WT2" factory-artifacts

  mkdir -p "$ARTIFACTS_WT2/.factory/hooks"
  mkdir -p "$ARTIFACTS_WT2/.factory/specs/behavioral-contracts/ss-05"
  mkdir -p "$ARTIFACTS_WT2/.factory/specs/architecture/decisions"
  mkdir -p "$ARTIFACTS_WT2/.factory/stories"
  echo "# BC-5.41.001 stub" > "$ARTIFACTS_WT2/.factory/specs/behavioral-contracts/ss-05/BC-5.41.001.md"
  echo "# ARCH-INDEX" > "$ARTIFACTS_WT2/.factory/specs/architecture/ARCH-INDEX.md"
  echo "# ADR-026" > "$ARTIFACTS_WT2/.factory/specs/architecture/decisions/ADR-026-wave-boundary-checkpoint-reset-and-lossless-intra-wave-compaction.md"
  echo "# ADR-025" > "$ARTIFACTS_WT2/.factory/specs/architecture/decisions/ADR-025-single-writer-factory-locklease-prevent-concurrent-session-races-on-factory-artifacts-orphan-branch.md"
  cat > "$ARTIFACTS_WT2/.factory/stories/STORY-INDEX.md" << 'EOF'
# STORY-INDEX
| ID | Title | Status |
|----|-------|--------|
| S-18.02 | Stub story 02 | pending |
| S-18.03 | Stub story 03 | draft |
EOF

  local sprint2="$WORK2/sprint-state.yaml"
  cat > "$sprint2" << 'EOF'
stories:
  - id: S-18.02
    status: pending
  - id: S-18.03
    status: draft
EOF

  local statemd2="$WORK2/STATE.md"
  cat > "$statemd2" << 'EOF'
---
current_step: "pass-1"
current_cycle: "v1.0-feature-context-durability-E18"
factory_lock: null
---
# STATE
EOF

  local wave1_exit_code=0
  local wave1_output
  wave1_output="$(
    export ARTIFACTS_WT="${ARTIFACTS_WT2}"
    export SPRINT_STATE_YAML="${sprint2}"
    export STATE_MD_PATH="${statemd2}"
    export BC_DIR="${ARTIFACTS_WT2}/.factory/specs/behavioral-contracts"
    export PRECOMPACT_FLUSH_LOG="${ARTIFACTS_WT2}/.factory/hooks/precompact-flush-log"
    export GIT_DIR="${WORK2}/.git"
    export FACTORY_REPO="${WORK2}"
    "${SKILL}" \
      --artifacts-worktree "${ARTIFACTS_WT2}" \
      --sprint-state "${sprint2}" \
      --state-md "${statemd2}" \
      --bc-dir "${ARTIFACTS_WT2}/.factory/specs/behavioral-contracts" \
      2>&1
  )" || wave1_exit_code=$?

  # Capture committed blob BEFORE cleanup
  local committed_content_wave1=""
  local blob_exit=0
  committed_content_wave1="$(git -C "$WORK2" show factory-artifacts:wave-state.yaml 2>&1)" || blob_exit=$?

  # Cleanup WORK2
  git -C "$WORK2" worktree remove --force "$ARTIFACTS_WT2" 2>/dev/null || true
  rm -rf "$WORK2"

  [ "$wave1_exit_code" -eq 0 ] || {
    echo "FAIL (Part B wave-1 null): skill exited ${wave1_exit_code}. Output: ${wave1_output}" >&2
    false
  }

  [ "$blob_exit" -eq 0 ] || {
    echo "FAIL (Part B wave-1 null): wave-state.yaml not in committed factory-artifacts tree" >&2
    false
  }

  # generated_from_handoff_sha must be null (no prior HANDOFF commit exists — wave 1)
  local gen_sha_wave1
  gen_sha_wave1="$(echo "$committed_content_wave1" | grep "^generated_from_handoff_sha:" | awk '{print $2}')"
  [ "$gen_sha_wave1" = "null" ] || {
    echo "FAIL (Part B wave-1 null): generated_from_handoff_sha in committed blob should be 'null'" >&2
    echo "  got: '${gen_sha_wave1}'" >&2
    echo "  For wave 1 with no prior HANDOFF commit on factory-artifacts, null is correct (AC-014 v1.4 / BC-5.41.002 EC-004)." >&2
    false
  }
}

# ---------------------------------------------------------------------------
# test_arch_files_paths_resolve_on_disk
# F-S1801-P1-004 / BC-5.41.002 PC5 arch_files minimum set
# Every path in the COMMITTED wave-state.yaml arch_files list must resolve
# to an existing file on disk (relative to the ARTIFACTS_WT directory).
# REDs: current impl emits non-existent paths like
# .factory/specs/architecture/ADRs/ADR-026.md (wrong path — the fixture has
# ADR-026-wave-boundary-*.md, not the short-form ADRs/ADR-026.md alias).
# ---------------------------------------------------------------------------

@test "test_arch_files_paths_resolve_on_disk" {
  _run_skill

  [ "$status" -eq 0 ] || {
    echo "FAIL: skill exited ${status}. Output: $output" >&2
    false
  }

  # Read the COMMITTED blob
  git -C "$WORK" show factory-artifacts:wave-state.yaml >/dev/null 2>&1 || {
    echo "FAIL: wave-state.yaml not in committed factory-artifacts tree" >&2
    false
  }

  local committed_content
  committed_content="$(git -C "$WORK" show factory-artifacts:wave-state.yaml)"

  # Extract all arch_files entries (lines matching "  - <path>" after arch_files:)
  # We parse lines between arch_files: and the next top-level key
  local in_arch_files=0
  local failed_paths=""
  while IFS= read -r line; do
    if echo "$line" | grep -q "^arch_files:"; then
      in_arch_files=1
      continue
    fi
    # Stop at the next top-level key (no leading spaces)
    if [ "$in_arch_files" -eq 1 ] && echo "$line" | grep -qE '^[a-z_]'; then
      in_arch_files=0
    fi
    if [ "$in_arch_files" -eq 1 ] && echo "$line" | grep -qE '^\s+-\s+'; then
      local path_entry
      path_entry="$(echo "$line" | sed 's/^[[:space:]]*-[[:space:]]*//')"
      # Resolve path relative to ARTIFACTS_WT
      local abs_path="${ARTIFACTS_WT}/${path_entry}"
      if [ ! -f "$abs_path" ]; then
        failed_paths="${failed_paths}\n  MISSING: ${path_entry} (resolved: ${abs_path})"
      fi
    fi
  done <<< "$committed_content"

  [ -z "$failed_paths" ] || {
    echo "FAIL: arch_files in COMMITTED wave-state.yaml contains non-existent paths:" >&2
    printf "%b" "$failed_paths" >&2
    echo "" >&2
    echo "All paths in arch_files must resolve to existing files on disk (BC-5.41.002 PC5)." >&2
    false
  }
}

# ---------------------------------------------------------------------------
# test_stories_have_spec_files_and_topo_order_and_resolve_story_index
# F-S1801-P1-005 / BC-5.41.002 PC2 + PC3 + INV3
# Each story entry in the COMMITTED wave-state.yaml must have a spec_files: key.
# Story IDs in next_wave_stories must exist in STORY-INDEX.md (anti-fabrication).
# Stories must appear in dependency-topological order.
# ---------------------------------------------------------------------------

@test "test_stories_have_spec_files_and_resolve_story_index" {
  _run_skill

  [ "$status" -eq 0 ] || {
    echo "FAIL: skill exited ${status}. Output: $output" >&2
    false
  }

  # Read the COMMITTED blob
  git -C "$WORK" show factory-artifacts:wave-state.yaml >/dev/null 2>&1 || {
    echo "FAIL: wave-state.yaml not in committed factory-artifacts tree" >&2
    false
  }

  local committed_content
  committed_content="$(git -C "$WORK" show factory-artifacts:wave-state.yaml)"

  # Extract story IDs from the committed blob (lines matching "    id: S-NN.NN")
  local story_ids
  story_ids="$(echo "$committed_content" | grep -E '^\s+id:\s+S-' | awk '{print $2}')"

  # Each story ID must appear in STORY-INDEX.md (anti-fabrication per BC-5.41.001 PC3)
  local story_index_content
  story_index_content="$(cat "$ARTIFACTS_WT/.factory/stories/STORY-INDEX.md")"
  local bad_ids=""
  while IFS= read -r sid; do
    [ -z "$sid" ] && continue
    echo "$story_index_content" | grep -q "$sid" || {
      bad_ids="${bad_ids} ${sid}"
    }
  done <<< "$story_ids"

  [ -z "$bad_ids" ] || {
    echo "FAIL: story IDs in COMMITTED wave-state.yaml not found in STORY-INDEX.md:${bad_ids}" >&2
    echo "BC-5.41.001 PC3 anti-fabrication: every next_wave story ID must exist in STORY-INDEX.md." >&2
    false
  }

  # Each story entry in the committed blob must have a spec_files: key (BC-5.41.002 PC2)
  # We check that any "id: S-" entry is followed by a spec_files: key before the next "id:"
  # Parse story blocks: find all "  - id:" entries and check for spec_files within block
  local missing_spec_files=""
  local current_id=""
  local has_spec_files=0
  while IFS= read -r line; do
    if echo "$line" | grep -qE '^\s+-\s+id:\s+S-'; then
      # New story block — check previous one
      if [ -n "$current_id" ] && [ "$has_spec_files" -eq 0 ]; then
        missing_spec_files="${missing_spec_files} ${current_id}"
      fi
      current_id="$(echo "$line" | awk '{print $NF}')"
      has_spec_files=0
    elif echo "$line" | grep -qE '^\s+spec_files:'; then
      has_spec_files=1
    fi
  done <<< "$committed_content"
  # Check last story block
  if [ -n "$current_id" ] && [ "$has_spec_files" -eq 0 ]; then
    missing_spec_files="${missing_spec_files} ${current_id}"
  fi

  [ -z "$missing_spec_files" ] || {
    echo "FAIL: stories in COMMITTED wave-state.yaml missing spec_files: key:${missing_spec_files}" >&2
    echo "BC-5.41.002 PC2: each story entry must have a spec_files: list." >&2
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
