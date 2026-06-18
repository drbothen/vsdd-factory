#!/usr/bin/env bats
# wave-handoff.bats — Red Gate tests for the wave-handoff skill
#
# Story:   S-18.01 — HANDOFF.md Schema + wave-handoff Skill; wave-state.yaml Atomic Production
# BCs:     BC-5.41.001 v1.18 (HANDOFF.md with 9 base required fields + anti-fabrication cross-checks)
#          BC-5.41.002 v1.13 (wave-state.yaml curated manifest; BrokenSprintState; atomicity;
#                              AC-014 clarified: generated_from_handoff_sha = PRIOR HANDOFF commit SHA)
# VPs:     VP-081 (Wave cannot close without verified HANDOFF.md)
#          VP-087 (atomicity + real-substrate derivation + BrokenSprintState)
#
# RED GATE discipline: every test MUST FAIL against the implementation at 91a6d6a4 for the
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
# ADR-027 FIXTURE DISCIPLINE (F-S1801-P3-001): the factory-artifacts worktree temp dir
# (ARTIFACTS_WT = $WORK/factory-wt) mirrors the production layout where the factory-
# artifacts orphan branch IS mounted as .factory/ — i.e., specs live directly at
# $ARTIFACTS_WT/specs/..., hooks at $ARTIFACTS_WT/hooks/..., stories at
# $ARTIFACTS_WT/stories/..., with NO nested .factory/ subdirectory inside ARTIFACTS_WT.
# The skill receives --bc-dir $ARTIFACTS_WT/specs/behavioral-contracts (explicit arg;
# the skill does NOT derive BC_DIR from $ARTIFACTS_WT with an additional .factory/ prefix).
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

  # Step 6: create fixture directories in the artifacts worktree.
  # ADR-027 production layout: no nested .factory/ inside ARTIFACTS_WT.
  # Specs live directly at $ARTIFACTS_WT/specs/..., hooks at $ARTIFACTS_WT/hooks/...
  # This mirrors production where ARTIFACTS_WT = .factory (the worktree root itself).
  mkdir -p "$ARTIFACTS_WT/hooks"
  mkdir -p "$ARTIFACTS_WT/specs/behavioral-contracts/ss-05"
  mkdir -p "$ARTIFACTS_WT/specs/architecture/decisions"
  mkdir -p "$ARTIFACTS_WT/stories"

  # Create a real BC file so active_bcs check can resolve at least one entry
  echo "# BC-5.41.001 stub" \
    > "$ARTIFACTS_WT/specs/behavioral-contracts/ss-05/BC-5.41.001.md"

  # Create real architecture files for arch_files path resolution (F-S1801-P1-004)
  echo "# ARCH-INDEX" > "$ARTIFACTS_WT/specs/architecture/ARCH-INDEX.md"
  echo "# ADR-026" > "$ARTIFACTS_WT/specs/architecture/decisions/ADR-026-wave-boundary-checkpoint-reset-and-lossless-intra-wave-compaction.md"
  echo "# ADR-025" > "$ARTIFACTS_WT/specs/architecture/decisions/ADR-025-single-writer-factory-locklease-prevent-concurrent-session-races-on-factory-artifacts-orphan-branch.md"

  # Create STORY-INDEX.md with S-18.02 and S-18.03 entries (F-S1801-P1-005)
  # Default: S-18.02 depends_on nothing; S-18.03 depends_on S-18.02.
  # Used by topo-sort tests — STORY-INDEX.md is the depends_on source per BC-5.41.002 PC3.
  cat > "$ARTIFACTS_WT/stories/STORY-INDEX.md" << 'EOF'
# STORY-INDEX
| ID | Title | depends_on | Status |
|----|-------|------------|--------|
| S-18.02 | Stub story 02 | | pending |
| S-18.03 | Stub story 03 | S-18.02 | draft |
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
# ADR-027 two-arg invocation model: --bc-dir is passed explicitly as
# $ARTIFACTS_WT/specs/behavioral-contracts (no extra .factory/ prefix added by the skill).
_run_skill() {
  run bash -c "
    export ARTIFACTS_WT='${ARTIFACTS_WT}'
    export SPRINT_STATE_YAML='${WORK}/sprint-state.yaml'
    export STATE_MD_PATH='${WORK}/STATE.md'
    export BC_DIR='${ARTIFACTS_WT}/specs/behavioral-contracts'
    export PRECOMPACT_FLUSH_LOG='${ARTIFACTS_WT}/hooks/precompact-flush-log'
    export GIT_DIR='${WORK}/.git'
    export FACTORY_REPO='${WORK}'
    '${SKILL}' \
      --artifacts-worktree '${ARTIFACTS_WT}' \
      --sprint-state '${WORK}/sprint-state.yaml' \
      --state-md '${WORK}/STATE.md' \
      --bc-dir '${ARTIFACTS_WT}/specs/behavioral-contracts' \
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
  rm -rf "$ARTIFACTS_WT/specs/behavioral-contracts"
  mkdir -p "$ARTIFACTS_WT/specs/behavioral-contracts"

  # Skill must exit 1 (hard error per AC-004 / EC-007)
  run bash -c "
    export ARTIFACTS_WT='${ARTIFACTS_WT}'
    export SPRINT_STATE_YAML='${WORK}/sprint-state.yaml'
    export STATE_MD_PATH='${WORK}/STATE.md'
    export BC_DIR='${ARTIFACTS_WT}/specs/behavioral-contracts'
    export PRECOMPACT_FLUSH_LOG='${ARTIFACTS_WT}/hooks/precompact-flush-log'
    export GIT_DIR='${WORK}/.git'
    export FACTORY_REPO='${WORK}'
    '${SKILL}' \
      --artifacts-worktree '${ARTIFACTS_WT}' \
      --sprint-state '${WORK}/sprint-state.yaml' \
      --state-md '${WORK}/STATE.md' \
      --bc-dir '${ARTIFACTS_WT}/specs/behavioral-contracts' \
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
  rm -f "$ARTIFACTS_WT/hooks/precompact-flush-log"
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
    > "$ARTIFACTS_WT/hooks/precompact-flush-log"
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
    > "$ARTIFACTS_WT/hooks/precompact-flush-log"
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
# F-P2-001 / BC-5.41.001 PC5 / EC-011
# Drives the REAL production guard — NO FORCE_PRECOMPACT_SHA env hatch.
#
# The real EC-011 trigger: precompact-flush-log is present with FIELD-4=commit BUT
# FIELD-2 is MALFORMED (not 40-char hex, e.g. "deadbeef"). The log claims a commit
# happened but the SHA cannot be trusted → the skill MUST exit 1 with
# PrecompactShaMismatch. The current impl returns FIELD-2 blindly from
# get_precompact_flush_sha without validating it is 40-char hex, then writes
# the malformed value to HANDOFF.md without hard-blocking → RED gate.
#
# Positive path (A): log present + FIELD-4=commit + valid 40-hex FIELD-2 →
#   skill succeeds; the COMMITTED HANDOFF.md precompact_flush_sha equals the log SHA.
#
# Negative path (B): log present + FIELD-4=commit + MALFORMED FIELD-2 (not 40-hex) →
#   skill MUST exit 1 with "PrecompactShaMismatch" in output.
#   HANDOFF.md MUST NOT be committed.
#
# No FORCE_PRECOMPACT_SHA set in either path — tests the real invocation contract.
# ---------------------------------------------------------------------------

@test "test_precompact_sha_mismatch_hard_blocks" {
  # --- Part A (positive): valid 40-hex SHA in log → skill succeeds; committed
  # HANDOFF.md precompact_flush_sha == log SHA. ---
  local log_sha="aabbccddeeff00112233445566778899aabbccdd"
  echo "2026-06-18T00:00:00Z ${log_sha} cycle/pass-2 commit" \
    > "$ARTIFACTS_WT/hooks/precompact-flush-log"

  _run_skill

  [ "$status" -eq 0 ] || {
    echo "FAIL (Part A): skill exited ${status} with valid 40-hex log SHA, expected 0." >&2
    echo "Actual output: $output" >&2
    false
  }

  # Assert via COMMITTED blob — not working-tree file (VP-087 proof harness)
  git -C "$WORK" show factory-artifacts:HANDOFF.md >/dev/null 2>&1 || {
    echo "FAIL (Part A): HANDOFF.md not committed to factory-artifacts" >&2
    false
  }
  local committed_sha
  committed_sha="$(git -C "$WORK" show factory-artifacts:HANDOFF.md \
    | grep "^precompact_flush_sha:" | awk '{print $2}')"
  [ "$committed_sha" = "$log_sha" ] || {
    echo "FAIL (Part A): committed HANDOFF.md precompact_flush_sha '${committed_sha}'" >&2
    echo "  expected log SHA '${log_sha}'" >&2
    false
  }

  # Clean the committed HANDOFF.md so Part B starts fresh
  rm -f "$ARTIFACTS_WT/HANDOFF.md"

  # --- Part B (negative): malformed FIELD-2 (not 40-hex) → skill MUST exit 1 with
  # PrecompactShaMismatch. This drives the REAL production guard — no env hatch. ---
  local malformed_sha="deadbeef"
  echo "2026-06-18T00:00:01Z ${malformed_sha} cycle/pass-2 commit" \
    > "$ARTIFACTS_WT/hooks/precompact-flush-log"

  # Run without FORCE_PRECOMPACT_SHA — real invocation contract
  run bash -c "
    export ARTIFACTS_WT='${ARTIFACTS_WT}'
    export SPRINT_STATE_YAML='${WORK}/sprint-state.yaml'
    export STATE_MD_PATH='${WORK}/STATE.md'
    export BC_DIR='${ARTIFACTS_WT}/specs/behavioral-contracts'
    export PRECOMPACT_FLUSH_LOG='${ARTIFACTS_WT}/hooks/precompact-flush-log'
    export FACTORY_REPO='${WORK}'
    '${SKILL}' \
      --artifacts-worktree '${ARTIFACTS_WT}' \
      --sprint-state '${WORK}/sprint-state.yaml' \
      --state-md '${WORK}/STATE.md' \
      --bc-dir '${ARTIFACTS_WT}/specs/behavioral-contracts' \
      2>&1
  "

  # Must exit 1 — PrecompactShaMismatch hard block (malformed FIELD-2 cannot be trusted)
  [ "$status" -eq 1 ] || {
    echo "FAIL (Part B): skill exited ${status}, expected exit 1 on PrecompactShaMismatch." >&2
    echo "When precompact-flush-log has FIELD-4=commit but FIELD-2='${malformed_sha}' is not" >&2
    echo "40-char hex, the skill must hard-block (BC-5.41.001 PC5/EC-011)." >&2
    echo "The current impl returns FIELD-2 blindly — no validation → this test REDs that path." >&2
    echo "Actual output: $output" >&2
    false
  }

  # Must emit PrecompactShaMismatch in output
  echo "$output" | grep -qi "PrecompactShaMismatch" || {
    echo "FAIL (Part B): PrecompactShaMismatch not in output. Got: $output" >&2
    false
  }

  # HANDOFF.md must NOT be committed (no partial artifact on hard error)
  # Assert via git show on the committed tree, not the working tree
  local before_count after_count
  before_count="$(git -C "$WORK" rev-list --count factory-artifacts)"
  # (The skill already ran and failed — commit count should not have grown from Part A)
  # Working-tree HANDOFF.md must also be absent (no partial write)
  [ ! -f "$ARTIFACTS_WT/HANDOFF.md" ] || {
    echo "FAIL (Part B): HANDOFF.md written to working tree despite PrecompactShaMismatch" >&2
    false
  }
}

# ---------------------------------------------------------------------------
# test_epic_complete_canonical_stdout_message
# F-S1801-P1-007 / F-S1801-P3-003 / BC-5.41.002 PC7 / BC-5.41.001 PC8 v1.18
# When EPIC-COMPLETE, stdout must contain ALL THREE canonical lines VERBATIM per
# BC-5.41.001 PC8 v1.18 ≡ BC-5.41.002 PC7 (reconciled at v1.18):
#   Line 1: "EPIC-COMPLETE: All stories in sprint-state.yaml have reached terminal status."
#   Line 2: "Epic <epic-id> is complete. No wave-state.yaml written for next wave."
#   Line 3: "HANDOFF.md committed to factory-artifacts with epic_status: complete."
# Where <epic-id> is derived from STATE.md current_cycle.
# The current impl (91a6d6a4) emits line 2 as "Epic <epic-id> is now complete." which
# is missing "No wave-state.yaml written for next wave." and has "is now" not "is".
# This test asserts the full verbatim line 2 — REDs against the current implementation.
# ---------------------------------------------------------------------------

@test "test_epic_complete_canonical_stdout_message" {
  _write_sprint_state_all_terminal
  _write_state_md "3"

  run bash -c "
    export ARTIFACTS_WT='${ARTIFACTS_WT}'
    export SPRINT_STATE_YAML='${WORK}/sprint-state.yaml'
    export STATE_MD_PATH='${WORK}/STATE.md'
    export BC_DIR='${ARTIFACTS_WT}/specs/behavioral-contracts'
    export PRECOMPACT_FLUSH_LOG='${ARTIFACTS_WT}/hooks/precompact-flush-log'
    export GIT_DIR='${WORK}/.git'
    export FACTORY_REPO='${WORK}'
    '${SKILL}' \
      --artifacts-worktree '${ARTIFACTS_WT}' \
      --sprint-state '${WORK}/sprint-state.yaml' \
      --state-md '${WORK}/STATE.md' \
      --bc-dir '${ARTIFACTS_WT}/specs/behavioral-contracts' \
      2>&1
  "

  # Must exit 0
  [ "$status" -eq 0 ] || {
    echo "FAIL: skill exited ${status} on EPIC-COMPLETE, expected 0. Output: $output" >&2
    false
  }

  # Must contain line 1 VERBATIM (BC-5.41.001 PC8 / BC-5.41.002 PC7)
  echo "$output" | grep -qF "EPIC-COMPLETE: All stories in sprint-state.yaml have reached terminal status." || {
    echo "FAIL: canonical EPIC-COMPLETE line 1 missing." >&2
    echo "Expected: 'EPIC-COMPLETE: All stories in sprint-state.yaml have reached terminal status.'" >&2
    echo "Actual output: $output" >&2
    false
  }

  # Must contain line 2 VERBATIM per BC-5.41.001 PC8 v1.18 / BC-5.41.002 PC7.
  # STATE.md has current_cycle: "v1.0-feature-context-durability-E18"
  # canonical: "Epic v1.0-feature-context-durability-E18 is complete. No wave-state.yaml written for next wave."
  # The current impl (91a6d6a4) emits: "Epic v1.0-feature-context-durability-E18 is now complete."
  # which is WRONG — "is now" vs "is", and "No wave-state.yaml written for next wave." is absent.
  # This assertion REDs the current implementation.
  echo "$output" | grep -qF "Epic v1.0-feature-context-durability-E18 is complete. No wave-state.yaml written for next wave." || {
    echo "FAIL (F-S1801-P3-003): canonical EPIC-COMPLETE line 2 wrong or missing." >&2
    echo "Expected verbatim: 'Epic v1.0-feature-context-durability-E18 is complete. No wave-state.yaml written for next wave.'" >&2
    echo "BC-5.41.001 PC8 v1.18 / BC-5.41.002 PC7 canonical text requires:" >&2
    echo "  'Epic <epic-id> is complete. No wave-state.yaml written for next wave.'" >&2
    echo "Current impl (91a6d6a4) emits 'Epic <epic-id> is now complete.' — 'is now' is wrong," >&2
    echo "  and 'No wave-state.yaml written for next wave.' is absent." >&2
    echo "Actual output: $output" >&2
    false
  }

  # Must contain line 3 VERBATIM (BC-5.41.001 PC8 / BC-5.41.002 PC7)
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
    export BC_DIR='${ARTIFACTS_WT}/specs/behavioral-contracts'
    export PRECOMPACT_FLUSH_LOG='${ARTIFACTS_WT}/hooks/precompact-flush-log'
    export GIT_DIR='${WORK}/.git'
    export FACTORY_REPO='${WORK}'
    '${SKILL}' \
      --artifacts-worktree '${ARTIFACTS_WT}' \
      --sprint-state '${WORK}/sprint-state.yaml' \
      --state-md '${WORK}/STATE.md' \
      --bc-dir '${ARTIFACTS_WT}/specs/behavioral-contracts' \
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
    export BC_DIR='${ARTIFACTS_WT}/specs/behavioral-contracts'
    export PRECOMPACT_FLUSH_LOG='${ARTIFACTS_WT}/hooks/precompact-flush-log'
    export GIT_DIR='${WORK}/.git'
    export FACTORY_REPO='${WORK}'
    '${SKILL}' \
      --artifacts-worktree '${ARTIFACTS_WT}' \
      --sprint-state '${WORK}/sprint-state.yaml' \
      --state-md '${WORK}/STATE.md' \
      --bc-dir '${ARTIFACTS_WT}/specs/behavioral-contracts' \
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

  # Assert no MODIFIED or STAGED tracked changes in the factory-artifacts worktree.
  # Untracked files (e.g., fixture spec directories: specs/, stories/) are intentionally
  # not staged by the narrow-staging commit (ADR-027 discipline). We check only for
  # modified or staged tracked file changes: lines NOT starting with '??' in porcelain output.
  # The original bug: impl awk-patched working-tree files AFTER committing → those tracked
  # files (HANDOFF.md, wave-state.yaml) show as modified ('M') after the commit.
  # That path is what we assert against here.
  local modified_output
  modified_output="$(git -C "$ARTIFACTS_WT" status --porcelain | grep -v '^??' || true)"
  [ -z "$modified_output" ] || {
    echo "FAIL: factory-artifacts worktree has modified or staged tracked files after skill run." >&2
    echo "git status --porcelain (tracked changes only):" >&2
    echo "$modified_output" >&2
    echo "HANDOFF.md and wave-state.yaml must match the committed tree after the atomic commit." >&2
    echo "Post-commit awk-patching of working-tree files leaves them in a modified state." >&2
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

  mkdir -p "$ARTIFACTS_WT2/hooks"
  mkdir -p "$ARTIFACTS_WT2/specs/behavioral-contracts/ss-05"
  mkdir -p "$ARTIFACTS_WT2/specs/architecture/decisions"
  mkdir -p "$ARTIFACTS_WT2/stories"
  echo "# BC-5.41.001 stub" > "$ARTIFACTS_WT2/specs/behavioral-contracts/ss-05/BC-5.41.001.md"
  echo "# ARCH-INDEX" > "$ARTIFACTS_WT2/specs/architecture/ARCH-INDEX.md"
  echo "# ADR-026" > "$ARTIFACTS_WT2/specs/architecture/decisions/ADR-026-wave-boundary-checkpoint-reset-and-lossless-intra-wave-compaction.md"
  echo "# ADR-025" > "$ARTIFACTS_WT2/specs/architecture/decisions/ADR-025-single-writer-factory-locklease-prevent-concurrent-session-races-on-factory-artifacts-orphan-branch.md"
  cat > "$ARTIFACTS_WT2/stories/STORY-INDEX.md" << 'EOF'
# STORY-INDEX
| ID | Title | depends_on | Status |
|----|-------|------------|--------|
| S-18.02 | Stub story 02 | | pending |
| S-18.03 | Stub story 03 | S-18.02 | draft |
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
    export BC_DIR="${ARTIFACTS_WT2}/specs/behavioral-contracts"
    export PRECOMPACT_FLUSH_LOG="${ARTIFACTS_WT2}/hooks/precompact-flush-log"
    export GIT_DIR="${WORK2}/.git"
    export FACTORY_REPO="${WORK2}"
    "${SKILL}" \
      --artifacts-worktree "${ARTIFACTS_WT2}" \
      --sprint-state "${sprint2}" \
      --state-md "${statemd2}" \
      --bc-dir "${ARTIFACTS_WT2}/specs/behavioral-contracts" \
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
  story_index_content="$(cat "$ARTIFACTS_WT/stories/STORY-INDEX.md")"
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
    export BC_DIR='${ARTIFACTS_WT}/specs/behavioral-contracts'
    export PRECOMPACT_FLUSH_LOG='${ARTIFACTS_WT}/hooks/precompact-flush-log'
    export GIT_DIR='${WORK}/.git'
    export FACTORY_REPO='${WORK}'
    '${SKILL}' \
      --artifacts-worktree '${ARTIFACTS_WT}' \
      --sprint-state '${WORK}/sprint-state.yaml' \
      --state-md '${WORK}/STATE.md' \
      --bc-dir '${ARTIFACTS_WT}/specs/behavioral-contracts' \
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
    export BC_DIR='${ARTIFACTS_WT}/specs/behavioral-contracts'
    export PRECOMPACT_FLUSH_LOG='${ARTIFACTS_WT}/hooks/precompact-flush-log'
    export GIT_DIR='${WORK}/.git'
    export FACTORY_REPO='${WORK}'
    '${SKILL}' \
      --artifacts-worktree '${ARTIFACTS_WT}' \
      --sprint-state '${WORK}/sprint-state.yaml' \
      --state-md '${WORK}/STATE.md' \
      --bc-dir '${ARTIFACTS_WT}/specs/behavioral-contracts' \
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
# test_commit_contains_only_handoff_and_wave_state
# F-P2-002 / BC-5.41.002 PC6 / VP-087 atomic commit scope
# The atomic commit to factory-artifacts MUST contain only HANDOFF.md and
# wave-state.yaml — no unrelated working-tree files.
#
# The current impl calls `git add -A` in commit-to-artifacts.sh which stages
# ALL untracked files, including any unrelated dirty file in the artifacts worktree.
# This test REDs that path.
#
# Setup: plant an unrelated dirty file in the artifacts worktree (unrelated.txt),
# run the has-next-wave path, then assert via `git show --stat factory-artifacts`
# that `unrelated.txt` does NOT appear in the commit tree.
# ---------------------------------------------------------------------------

@test "test_commit_contains_only_handoff_and_wave_state" {
  # Plant an unrelated file in the artifacts worktree (not staged, not tracked)
  echo "this file must not appear in the wave-handoff commit" \
    > "$ARTIFACTS_WT/unrelated.txt"

  _run_skill

  [ "$status" -eq 0 ] || {
    echo "FAIL: skill exited ${status}, expected 0. Output: $output" >&2
    false
  }

  # Assert the committed tree does NOT contain unrelated.txt
  # `git show <commit>:<file>` returns exit 128 if the path is absent from the tree
  local latest_commit
  latest_commit="$(git -C "$WORK" rev-parse factory-artifacts)"

  if git -C "$WORK" show "${latest_commit}:unrelated.txt" >/dev/null 2>&1; then
    echo "FAIL (F-P2-002): unrelated.txt is in the committed factory-artifacts tree." >&2
    echo "The atomic commit must contain ONLY HANDOFF.md and wave-state.yaml." >&2
    echo "Current impl uses 'git add -A' which stages all untracked files." >&2
    echo "Commit: ${latest_commit}" >&2
    echo "Files in commit:" >&2
    git -C "$WORK" show --stat "${latest_commit}" >&2
    false
  fi

  # Positive assertion: HANDOFF.md and wave-state.yaml ARE in the committed tree
  git -C "$WORK" show "${latest_commit}:HANDOFF.md" >/dev/null 2>&1 || {
    echo "FAIL (F-P2-002): HANDOFF.md missing from committed tree." >&2
    false
  }
  git -C "$WORK" show "${latest_commit}:wave-state.yaml" >/dev/null 2>&1 || {
    echo "FAIL (F-P2-002): wave-state.yaml missing from committed tree." >&2
    false
  }
}

# ---------------------------------------------------------------------------
# test_wave_state_wave_id_is_next_wave
# F-P2-003 / BC-5.41.002 PC2
# The wave_id in the COMMITTED wave-state.yaml must be (HANDOFF.md wave_id + 1).
# wave-state.yaml describes the NEXT wave; HANDOFF.md describes the wave just closed.
#
# The current impl passes the SAME wave_id to both write_handoff and write_wave_state,
# so both files get the same wave_id. This test REDs that path.
#
# Assert via committed blobs (git show factory-artifacts:FILE) — not working-tree files.
# ---------------------------------------------------------------------------

@test "test_wave_state_wave_id_is_next_wave" {
  # STATE.md has current_step: "pass-2" → skill derives wave_id=2 for HANDOFF.md
  # wave-state.yaml must have wave_id=3 (next wave = current + 1)
  _write_state_md "2"
  _run_skill

  [ "$status" -eq 0 ] || {
    echo "FAIL: skill exited ${status}, expected 0. Output: $output" >&2
    false
  }

  # Read COMMITTED blobs (VP-087 proof harness)
  git -C "$WORK" show factory-artifacts:HANDOFF.md >/dev/null 2>&1 || {
    echo "FAIL: HANDOFF.md not in committed factory-artifacts tree" >&2
    false
  }
  git -C "$WORK" show factory-artifacts:wave-state.yaml >/dev/null 2>&1 || {
    echo "FAIL: wave-state.yaml not in committed factory-artifacts tree" >&2
    false
  }

  local handoff_wave_id wave_state_wave_id
  handoff_wave_id="$(git -C "$WORK" show factory-artifacts:HANDOFF.md \
    | grep "^wave_id:" | awk '{print $2}')"
  wave_state_wave_id="$(git -C "$WORK" show factory-artifacts:wave-state.yaml \
    | grep "^wave_id:" | awk '{print $2}')"

  # Both must be integers
  echo "$handoff_wave_id" | grep -qE '^[0-9]+$' || {
    echo "FAIL: HANDOFF.md wave_id '${handoff_wave_id}' is not an integer" >&2
    false
  }
  echo "$wave_state_wave_id" | grep -qE '^[0-9]+$' || {
    echo "FAIL: wave-state.yaml wave_id '${wave_state_wave_id}' is not an integer" >&2
    false
  }

  # wave-state.yaml wave_id must be exactly handoff wave_id + 1
  local expected_next_wave=$(( handoff_wave_id + 1 ))
  [ "$wave_state_wave_id" -eq "$expected_next_wave" ] || {
    echo "FAIL (F-P2-003): wave-state.yaml wave_id='${wave_state_wave_id}'" >&2
    echo "  expected: ${expected_next_wave} (HANDOFF wave_id ${handoff_wave_id} + 1)" >&2
    echo "  BC-5.41.002 PC2: wave-state.yaml describes the NEXT wave; its wave_id must be" >&2
    echo "  the committed HANDOFF.md wave_id + 1." >&2
    echo "  Current impl passes the SAME wave_id to both files → this test REDs that path." >&2
    false
  }
}

# ---------------------------------------------------------------------------
# test_wave_id_non_silent_when_state_md_lacks_pass_step
# F-P2-004 / BC-5.41.001 PC2 (anti-fabrication: wave_id must be real-substrate-derived)
# When STATE.md has no valid "pass-N" current_step (e.g., current_step: "something-else"),
# derive_wave_id currently silently returns 1 — a phantom value, not a real derivation.
# BC-5.41.001 PC2 anti-fabrication requires wave_id come from real substrate.
# A silent fallback to 1 violates this — it fabricates a wave_id out of thin air.
# The skill MUST either:
#   (a) derive wave_id from an explicit real substrate (sprint-state topo-sort ordinal), OR
#   (b) exit 1 with an explicit hard error (AntiFabricationFailed / NoWaveIdSubstrate)
# rather than silently outputting 1.
#
# This test asserts the NON-SILENT outcome: skill either exits 1 with an error mentioning
# wave_id derivation, OR exits 0 with a wave_id derived from a documented real substrate
# (NOT the silent-fallback-to-1 path).
# REDs the current `echo "1"` silent fallback in derive_wave_id.
# ---------------------------------------------------------------------------

@test "test_wave_id_non_silent_when_state_md_lacks_pass_step" {
  # Write STATE.md with a non-pass-N current_step (engine-only scope assumed)
  cat > "$WORK/STATE.md" << 'EOF'
---
current_step: "something-else"
current_cycle: "v1.0-feature-context-durability-E18"
factory_lock: null
---
# STATE
EOF

  run bash -c "
    export ARTIFACTS_WT='${ARTIFACTS_WT}'
    export SPRINT_STATE_YAML='${WORK}/sprint-state.yaml'
    export STATE_MD_PATH='${WORK}/STATE.md'
    export BC_DIR='${ARTIFACTS_WT}/specs/behavioral-contracts'
    export PRECOMPACT_FLUSH_LOG='${ARTIFACTS_WT}/hooks/precompact-flush-log'
    export FACTORY_REPO='${WORK}'
    '${SKILL}' \
      --artifacts-worktree '${ARTIFACTS_WT}' \
      --sprint-state '${WORK}/sprint-state.yaml' \
      --state-md '${WORK}/STATE.md' \
      --bc-dir '${ARTIFACTS_WT}/specs/behavioral-contracts' \
      2>&1
  "

  # The skill must NOT silently use wave_id=1 as a fabricated fallback.
  # Two acceptable outcomes (either satisfies BC-5.41.001 PC2):
  #   (a) exit 1 with an explicit error about wave_id derivation
  #   (b) exit 0 with a wave_id that is provably derived from a real documented substrate
  #       (NOT the numeric literal 1 emitted by the current silent fallback)
  #
  # The current impl exits 0 and silently writes wave_id: 1 — the committed HANDOFF.md
  # would show wave_id: 1 with no substrate basis. This test REDs that path.

  if [ "$status" -eq 1 ]; then
    # Acceptable outcome (a): explicit error
    echo "$output" | grep -qiE "(wave_id|NoWaveIdSubstrate|AntiFabricationFailed|cannot derive)" || {
      echo "FAIL: skill exited 1 but output does not mention wave_id derivation failure." >&2
      echo "Expected: error message mentioning wave_id, NoWaveIdSubstrate, AntiFabricationFailed," >&2
      echo "  or derivation failure." >&2
      echo "Actual output: $output" >&2
      false
    }
    # Outcome (a) accepted — explicit hard error
    return 0
  fi

  # If exit 0: must NOT have used the silent-fallback-to-1
  # Assert via committed blob
  git -C "$WORK" show factory-artifacts:HANDOFF.md >/dev/null 2>&1 || {
    echo "FAIL (F-P2-004): skill exited 0 but HANDOFF.md not committed." >&2
    false
  }
  local committed_wave_id
  committed_wave_id="$(git -C "$WORK" show factory-artifacts:HANDOFF.md \
    | grep "^wave_id:" | awk '{print $2}')"

  # wave_id must NOT be 1 from a silent fallback when no pass-N step exists.
  # If it IS 1 and came from the silent fallback, that is fabrication.
  # We detect the silent fallback by checking: if no real substrate exists AND wave_id=1,
  # the only way to get 1 is the silent `echo "1"` in derive_wave_id → fabrication.
  if [ "$committed_wave_id" = "1" ]; then
    echo "FAIL (F-P2-004): wave_id=1 committed despite no valid 'pass-N' current_step in STATE.md." >&2
    echo "  current_step was 'something-else'; derive_wave_id silently returns 1 as a fallback." >&2
    echo "  BC-5.41.001 PC2 requires real-substrate derivation, not a silent numeric literal." >&2
    echo "  The skill must exit 1 with an explicit error OR derive wave_id from a documented" >&2
    echo "  real substrate other than the 'echo 1' fallback." >&2
    false
  fi
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
    export BC_DIR='${ARTIFACTS_WT}/specs/behavioral-contracts'
    export PRECOMPACT_FLUSH_LOG='${ARTIFACTS_WT}/hooks/precompact-flush-log'
    export GIT_DIR='${WORK}/.git'
    export FACTORY_REPO='${WORK}'
    '${SKILL}' \
      --artifacts-worktree '${ARTIFACTS_WT}' \
      --sprint-state '${WORK}/sprint-state.yaml' \
      --state-md '${WORK}/STATE.md' \
      --bc-dir '${ARTIFACTS_WT}/specs/behavioral-contracts' \
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

# ---------------------------------------------------------------------------
# test_BC_5_41_002_F_P3_001_story_index_read_from_adr027_path
# F-S1801-P3-001 / ADR-027 Decision 3 / BC-5.41.001 PC3 anti-fabrication / VP-087
# ADR-027 canonical wiring: ARTIFACTS_WT is the factory-artifacts worktree root.
# In production ARTIFACTS_WT = .factory. The fixture mirrors this: spec files live
# directly at $ARTIFACTS_WT/specs/..., stories at $ARTIFACTS_WT/stories/..., with
# NO nested .factory/ subdirectory inside ARTIFACTS_WT.
#
# The skill's write-wave-state.sh reads STORY-INDEX.md from
#   ${artifacts_wt}/.factory/stories/STORY-INDEX.md  (hardcoded .factory/ prefix)
# In the ADR-027 fixture, that path does NOT exist. The actual STORY-INDEX.md is at
#   ${artifacts_wt}/stories/STORY-INDEX.md
#
# BC-5.41.001 PC3 anti-fabrication: story IDs in wave-state.yaml MUST be cross-checked
# against STORY-INDEX.md. When the skill cannot find STORY-INDEX.md (wrong path), it
# silently skips the cross-check — a SOUL.md §4 silent failure.
#
# RED gate: plant a fabricated story ID (S-99.99) in sprint-state.yaml that does NOT
# exist in STORY-INDEX.md ($ARTIFACTS_WT/stories/STORY-INDEX.md). Assert the skill
# exits 1 with AntiFabricationFailed — if the skill reads STORY-INDEX.md from the
# correct ADR-027 path, it detects the phantom ID and hard-errors. If it reads from the
# wrong hardcoded .factory/ path (91a6d6a4), the file is absent → anti-fabrication silently
# skips → skill exits 0 → this test fails (WRONG exit code).
# ---------------------------------------------------------------------------

@test "test_BC_5_41_002_F_P3_001_story_index_read_from_adr027_path" {
  # STORY-INDEX.md (at $ARTIFACTS_WT/stories/STORY-INDEX.md) contains S-18.02 and S-18.03.
  # S-99.99 is NOT in STORY-INDEX.md — it is a fabricated phantom story ID.
  cat > "$WORK/sprint-state.yaml" << 'EOF'
stories:
  - id: S-99.99
    status: pending
EOF

  run bash -c "
    export ARTIFACTS_WT='${ARTIFACTS_WT}'
    export SPRINT_STATE_YAML='${WORK}/sprint-state.yaml'
    export STATE_MD_PATH='${WORK}/STATE.md'
    export BC_DIR='${ARTIFACTS_WT}/specs/behavioral-contracts'
    export PRECOMPACT_FLUSH_LOG='${ARTIFACTS_WT}/hooks/precompact-flush-log'
    export GIT_DIR='${WORK}/.git'
    export FACTORY_REPO='${WORK}'
    '${SKILL}' \
      --artifacts-worktree '${ARTIFACTS_WT}' \
      --sprint-state '${WORK}/sprint-state.yaml' \
      --state-md '${WORK}/STATE.md' \
      --bc-dir '${ARTIFACTS_WT}/specs/behavioral-contracts' \
      2>&1
  "

  # Must exit 1 — S-99.99 is not in STORY-INDEX.md → AntiFabricationFailed
  # If the skill reads STORY-INDEX.md from the CORRECT ADR-027 path
  #   ($ARTIFACTS_WT/stories/STORY-INDEX.md), it finds the file, checks S-99.99,
  #   doesn't find it, and hard-errors.
  # If the skill reads from the WRONG hardcoded path
  #   ($ARTIFACTS_WT/.factory/stories/STORY-INDEX.md — which doesn't exist in the
  #   ADR-027 fixture), the `if [ -f ... ]` guard silently skips → skill exits 0
  #   without detecting the phantom ID.
  # Current impl (91a6d6a4) uses the wrong hardcoded .factory/ path → exits 0 here.
  [ "$status" -eq 1 ] || {
    echo "FAIL (F-S1801-P3-001): skill exited ${status}, expected 1 (AntiFabricationFailed)." >&2
    echo "" >&2
    echo "Fabricated story ID S-99.99 is NOT in STORY-INDEX.md." >&2
    echo "BC-5.41.001 PC3 anti-fabrication requires the skill to hard-error on phantom story IDs." >&2
    echo "" >&2
    echo "ADR-027 path discipline: STORY-INDEX.md is at:" >&2
    echo "  ${ARTIFACTS_WT}/stories/STORY-INDEX.md  (ADR-027 correct path)" >&2
    echo "Current impl (91a6d6a4) reads from:" >&2
    echo "  ${ARTIFACTS_WT}/.factory/stories/STORY-INDEX.md  (hardcoded wrong path, does not exist)" >&2
    echo "The absent file causes anti-fabrication to silently skip → phantom ID S-99.99 passes." >&2
    echo "" >&2
    echo "Actual output: $output" >&2
    false
  }

  # Error output must mention AntiFabricationFailed or S-99.99
  echo "$output" | grep -qiE "(AntiFabricationFailed|S-99\.99|not found in STORY-INDEX)" || {
    echo "FAIL (F-S1801-P3-001): skill exited 1 but output doesn't mention AntiFabricationFailed." >&2
    echo "Expected error mentioning AntiFabricationFailed or S-99.99 or 'not found in STORY-INDEX'" >&2
    echo "Actual output: $output" >&2
    false
  }

  # Positive side-check: STORY-INDEX.md must exist at ADR-027 path (fixture sanity)
  [ -f "$ARTIFACTS_WT/stories/STORY-INDEX.md" ] || {
    echo "FAIL (F-S1801-P3-001): fixture STORY-INDEX.md missing at ${ARTIFACTS_WT}/stories/STORY-INDEX.md" >&2
    echo "This is a fixture setup error, not an impl error." >&2
    false
  }
}

# ---------------------------------------------------------------------------
# test_BC_5_41_002_F_P3_002_stories_topological_order_in_committed_blob
# F-S1801-P3-002 / BC-5.41.002 PC3 / VP-087
# The stories list in the COMMITTED wave-state.yaml must be ordered by the
# dependency graph from STORY-INDEX.md depends_on: arrays — not by the order
# they appear in sprint-state.yaml.
#
# Fixture: sprint-state.yaml lists stories in REVERSE dependency order:
#   - id: S-18.03   status: draft    (depends_on S-18.02 per STORY-INDEX.md)
#   - id: S-18.02   status: pending  (no dependency — must come first)
# STORY-INDEX.md declares: S-18.03 depends_on S-18.02
# Correct topo order: S-18.02 first, then S-18.03
#
# REDs the current implementation which uses file-order (sprint-state.yaml order)
# rather than dependency-graph topological order: S-18.03 appears first in
# sprint-state.yaml → current impl emits S-18.03 first in wave-state.yaml
# → this test fails the order assertion.
#
# Assert via COMMITTED blob (git show factory-artifacts:wave-state.yaml).
# ---------------------------------------------------------------------------

@test "test_BC_5_41_002_F_P3_002_stories_topological_order_in_committed_blob" {
  # Write sprint-state.yaml in REVERSE dependency order (S-18.03 first, S-18.02 second).
  # STORY-INDEX.md (set up in setup()) declares S-18.03 depends_on S-18.02.
  cat > "$WORK/sprint-state.yaml" << 'EOF'
stories:
  - id: S-18.03
    status: draft
  - id: S-18.02
    status: pending
EOF

  _run_skill

  [ "$status" -eq 0 ] || {
    echo "FAIL: skill exited ${status}, expected 0. Output: $output" >&2
    false
  }

  # Read the COMMITTED blob (VP-087 proof harness — not working-tree file)
  git -C "$WORK" show factory-artifacts:wave-state.yaml >/dev/null 2>&1 || {
    echo "FAIL: wave-state.yaml not in committed factory-artifacts tree" >&2
    false
  }

  local committed_content
  committed_content="$(git -C "$WORK" show factory-artifacts:wave-state.yaml)"

  # Extract the story IDs in the order they appear in the committed blob
  # Lines matching "  - id: S-" under the stories: block
  local ordered_ids
  ordered_ids="$(echo "$committed_content" | grep -E '^\s+-\s+id:\s+S-' | awk '{print $NF}')"

  # Correct topo order: S-18.02 (no deps) must appear BEFORE S-18.03 (depends_on S-18.02)
  local first_id second_id
  first_id="$(echo "$ordered_ids" | sed -n '1p')"
  second_id="$(echo "$ordered_ids" | sed -n '2p')"

  [ "$first_id" = "S-18.02" ] || {
    echo "FAIL (F-S1801-P3-002): topological sort failed." >&2
    echo "  Expected first story: S-18.02 (no dependencies)" >&2
    echo "  Got first story:      '${first_id}'" >&2
    echo "" >&2
    echo "  STORY-INDEX.md declares: S-18.03 depends_on S-18.02" >&2
    echo "  sprint-state.yaml order: S-18.03 first, S-18.02 second (REVERSE of correct order)" >&2
    echo "  BC-5.41.002 PC3 requires stories be ordered by dependency graph, not file order." >&2
    echo "" >&2
    echo "  Current impl (91a6d6a4) uses sprint-state.yaml file order directly → S-18.03 first." >&2
    echo "  This REDs the topological sort requirement." >&2
    echo "" >&2
    echo "  Committed wave-state.yaml stories section:" >&2
    echo "$committed_content" | grep -A 10 "^stories:" >&2
    false
  }

  [ "$second_id" = "S-18.03" ] || {
    echo "FAIL (F-S1801-P3-002): expected second story to be S-18.03, got '${second_id}'" >&2
    echo "  Committed wave-state.yaml stories section:" >&2
    echo "$committed_content" | grep -A 10 "^stories:" >&2
    false
  }
}

# ---------------------------------------------------------------------------
# test_BC_5_41_001_F_P3_003_epic_complete_all_three_lines_verbatim
# F-S1801-P3-003 / BC-5.41.001 PC8 v1.18 ≡ BC-5.41.002 PC7 (verbatim-identical after v1.18)
# EPIC-COMPLETE stdout must contain ALL THREE canonical lines VERBATIM.
# Each line is asserted separately so failure points to the exact deviation.
#
# Canonical three-line format (BC-5.41.001 PC8 v1.18 / BC-5.41.002 PC7):
#   Line 1: "EPIC-COMPLETE: All stories in sprint-state.yaml have reached terminal status."
#   Line 2: "Epic <epic-id> is complete. No wave-state.yaml written for next wave."
#   Line 3: "HANDOFF.md committed to factory-artifacts with epic_status: complete."
#
# Current impl (91a6d6a4) emits line 2 as:
#   "Epic <epic-id> is now complete."
# which deviates on two counts:
#   (a) "is now complete." vs "is complete." — extra "now"
#   (b) "No wave-state.yaml written for next wave." absent entirely
#
# This test asserts line 2 verbatim — REDs against 91a6d6a4.
# ---------------------------------------------------------------------------

@test "test_BC_5_41_001_F_P3_003_epic_complete_all_three_lines_verbatim" {
  _write_sprint_state_all_terminal
  _write_state_md "4"

  run bash -c "
    export ARTIFACTS_WT='${ARTIFACTS_WT}'
    export SPRINT_STATE_YAML='${WORK}/sprint-state.yaml'
    export STATE_MD_PATH='${WORK}/STATE.md'
    export BC_DIR='${ARTIFACTS_WT}/specs/behavioral-contracts'
    export PRECOMPACT_FLUSH_LOG='${ARTIFACTS_WT}/hooks/precompact-flush-log'
    export GIT_DIR='${WORK}/.git'
    export FACTORY_REPO='${WORK}'
    '${SKILL}' \
      --artifacts-worktree '${ARTIFACTS_WT}' \
      --sprint-state '${WORK}/sprint-state.yaml' \
      --state-md '${WORK}/STATE.md' \
      --bc-dir '${ARTIFACTS_WT}/specs/behavioral-contracts' \
      2>&1
  "

  # Must exit 0 on EPIC-COMPLETE
  [ "$status" -eq 0 ] || {
    echo "FAIL (F-S1801-P3-003): skill exited ${status} on EPIC-COMPLETE, expected 0." >&2
    echo "Output: $output" >&2
    false
  }

  # --- Line 1 VERBATIM ---
  echo "$output" | grep -qF \
    "EPIC-COMPLETE: All stories in sprint-state.yaml have reached terminal status." || {
    echo "FAIL (F-S1801-P3-003 line 1): canonical EPIC-COMPLETE line 1 missing or wrong." >&2
    echo "Expected verbatim: 'EPIC-COMPLETE: All stories in sprint-state.yaml have reached terminal status.'" >&2
    echo "Actual output: $output" >&2
    false
  }

  # --- Line 2 VERBATIM (BC-5.41.001 PC8 v1.18 / BC-5.41.002 PC7 canonical text) ---
  # STATE.md current_cycle = "v1.0-feature-context-durability-E18"
  # Canonical: "Epic v1.0-feature-context-durability-E18 is complete. No wave-state.yaml written for next wave."
  # Current impl (91a6d6a4) emits: "Epic v1.0-feature-context-durability-E18 is now complete."
  #   Deviation 1: "is now complete" vs "is complete"
  #   Deviation 2: "No wave-state.yaml written for next wave." is absent
  echo "$output" | grep -qF \
    "Epic v1.0-feature-context-durability-E18 is complete. No wave-state.yaml written for next wave." || {
    echo "FAIL (F-S1801-P3-003 line 2): canonical EPIC-COMPLETE line 2 wrong or missing." >&2
    echo "Expected verbatim: 'Epic v1.0-feature-context-durability-E18 is complete. No wave-state.yaml written for next wave.'" >&2
    echo "BC-5.41.001 PC8 v1.18 / BC-5.41.002 PC7 canonical line 2 template:" >&2
    echo "  'Epic <epic-id> is complete. No wave-state.yaml written for next wave.'" >&2
    echo "Current impl (91a6d6a4) emits: 'Epic <epic-id> is now complete.'" >&2
    echo "  Deviation 1: 'is now complete' (extra 'now')" >&2
    echo "  Deviation 2: 'No wave-state.yaml written for next wave.' is absent" >&2
    echo "Actual output: $output" >&2
    false
  }

  # --- Line 3 VERBATIM ---
  echo "$output" | grep -qF \
    "HANDOFF.md committed to factory-artifacts with epic_status: complete." || {
    echo "FAIL (F-S1801-P3-003 line 3): canonical EPIC-COMPLETE line 3 missing or wrong." >&2
    echo "Expected verbatim: 'HANDOFF.md committed to factory-artifacts with epic_status: complete.'" >&2
    echo "Actual output: $output" >&2
    false
  }
}

# ---------------------------------------------------------------------------
# test_BC_5_41_001_F_P3_004_last_verified_sha_cwd_independent
# F-S1801-P3-004 / BC-5.41.001 PC3 / ADR-027 Decision 1
# last_verified_develop_sha in HANDOFF.md must resolve correctly even when the
# skill is invoked from an UNRELATED cwd (not the artifacts worktree, not the
# main repo — e.g., /tmp or any directory with no git repo).
#
# The skill must use `git -C "$ARTIFACTS_WT"` (or an explicit repo path via
# FACTORY_REPO / git -C) to resolve origin/develop — NOT a bare
# `git rev-parse origin/develop` which is cwd-dependent and would fail or
# return wrong results when cwd has no git repo.
#
# Current impl (91a6d6a4) in write-handoff.sh get_last_verified_develop_sha():
#   if FACTORY_REPO is set: git -C "$factory_repo" rev-parse origin/develop  ✓ (cwd-independent)
#   else: git rev-parse origin/develop                                         ✗ (cwd-dependent)
# This test invokes the skill WITHOUT FACTORY_REPO and with cwd=/tmp.
# The bare `git rev-parse origin/develop` in /tmp will fail (no git repo) → exit non-zero.
#
# REDs the fallback path (bare git rev-parse) in get_last_verified_develop_sha when
# FACTORY_REPO is not set and cwd is not a git repo.
#
# Assert: skill exits 0 AND the committed HANDOFF.md contains a valid 40-char hex SHA
# matching the fixture's origin/develop ref.
# ---------------------------------------------------------------------------

@test "test_BC_5_41_001_F_P3_004_last_verified_sha_cwd_independent" {
  local fixture_develop_sha
  fixture_develop_sha="$(git -C "$WORK" rev-parse origin/develop)"

  # Invoke skill from /tmp (unrelated cwd, definitely no git repo there).
  # FACTORY_REPO is NOT set — forces the bare `git rev-parse origin/develop` fallback.
  # Note: ARTIFACTS_WT and all explicit paths are absolute, so the skill can resolve them
  # correctly IF it uses -C flags. Only the develop SHA lookup depends on cwd.
  run bash -c "
    cd /tmp
    export ARTIFACTS_WT='${ARTIFACTS_WT}'
    export SPRINT_STATE_YAML='${WORK}/sprint-state.yaml'
    export STATE_MD_PATH='${WORK}/STATE.md'
    export BC_DIR='${ARTIFACTS_WT}/specs/behavioral-contracts'
    export PRECOMPACT_FLUSH_LOG='${ARTIFACTS_WT}/hooks/precompact-flush-log'
    '${SKILL}' \
      --artifacts-worktree '${ARTIFACTS_WT}' \
      --sprint-state '${WORK}/sprint-state.yaml' \
      --state-md '${WORK}/STATE.md' \
      --bc-dir '${ARTIFACTS_WT}/specs/behavioral-contracts' \
      2>&1
  "
  # Note: FACTORY_REPO and GIT_DIR intentionally NOT exported here.
  # The skill must resolve origin/develop via git -C ARTIFACTS_WT or equivalent.

  # Must exit 0 — a cwd-independent implementation succeeds from /tmp
  [ "$status" -eq 0 ] || {
    echo "FAIL (F-S1801-P3-004): skill exited ${status} when invoked from /tmp without FACTORY_REPO." >&2
    echo "Expected exit 0: skill must resolve origin/develop via git -C ARTIFACTS_WT (cwd-independent)." >&2
    echo "Current impl (91a6d6a4) has a bare 'git rev-parse origin/develop' fallback in" >&2
    echo "  get_last_verified_develop_sha() when FACTORY_REPO is unset — this fails in /tmp" >&2
    echo "  because /tmp has no git repository." >&2
    echo "Actual output: $output" >&2
    false
  }

  # Read committed HANDOFF.md blob (VP-087 proof harness — not working-tree file)
  git -C "$WORK" show factory-artifacts:HANDOFF.md >/dev/null 2>&1 || {
    echo "FAIL (F-S1801-P3-004): HANDOFF.md not committed to factory-artifacts." >&2
    false
  }

  local committed_sha
  committed_sha="$(git -C "$WORK" show factory-artifacts:HANDOFF.md \
    | grep "^last_verified_develop_sha:" | awk '{print $2}')"

  # Must be a valid 40-char hex SHA
  echo "$committed_sha" | grep -qE '^[0-9a-f]{40}$' || {
    echo "FAIL (F-S1801-P3-004): last_verified_develop_sha in committed HANDOFF.md" >&2
    echo "  '${committed_sha}' is not a 40-char lowercase hex SHA." >&2
    echo "  When skill is invoked from /tmp without FACTORY_REPO, get_last_verified_develop_sha" >&2
    echo "  must still resolve origin/develop correctly via git -C ARTIFACTS_WT." >&2
    false
  }

  # Must match the fixture's origin/develop SHA exactly
  [ "$committed_sha" = "$fixture_develop_sha" ] || {
    echo "FAIL (F-S1801-P3-004): last_verified_develop_sha in committed HANDOFF.md" >&2
    echo "  '${committed_sha}' != fixture origin/develop '${fixture_develop_sha}'" >&2
    echo "  The skill must resolve origin/develop from the fixture repo (via git -C ARTIFACTS_WT" >&2
    echo "  or git -C WORK), not from cwd=/tmp (which has no git repo)." >&2
    false
  }
}
