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

  # Create STORY-INDEX.md in MULTI-TABLE production shape (F-P5-001/F-P5-002 regression fixture).
  #
  # The REAL production STORY-INDEX.md (23+ epic tables) has heterogeneous headers:
  #   - Early epic tables (E-0, E-1, E-2 …) use SPACED "Depends On" (7-column format, no Blocks/BCs)
  #   - E-18+ tables use HYPHENATED "Depends-On" (9-column format, with Blocks and BCs)
  #
  # The production bug (F-P5-001): the parser's `grep -m1 '| Story ID'` picks the FIRST
  # matching table header — the E-0 table at the top, which uses "Depends On" (space).
  # The parser then scans that header for "Depends-On" (hyphen) and finds nothing →
  # depends_on_col stays 0 → the dep-loading guard `[ "$depends_on_col" -gt 0 ]` is false
  # → all story deps are empty → Kahn's algorithm degrades to sprint-state file-order.
  #
  # This fixture mirrors that production shape:
  #   LEADING TABLE: E-0 style — "Depends On" (SPACE, 7 columns), unrelated S-0.* stories
  #   LATER TABLE:   E-18       — "Depends-On" (HYPHEN, 9 columns), the S-18.* in-wave stories
  #
  # The correct implementation must find the table that CONTAINS the in-wave story IDs
  # (S-18.*), not the first "| Story ID" match.  It must also tolerate both header variants.
  #
  # Fixture stories in E-18 table:
  #   S-18.02 — no dependencies (root of DAG)
  #   S-18.03 — depends on [S-18.02] (single dep)
  #   S-18.04a — depends on [S-18.02, S-18.03] (multi-dep diamond; exercises column-6 multi-dep parsing)
  cat > "$ARTIFACTS_WT/stories/STORY-INDEX.md" << 'EOF'
---
document_type: story-index
level: ops
version: "1.0"
status: current
---

# STORY-INDEX

## Epic E-0 — Infrastructure Prep (leading table; spaced Depends On; 7 columns)

| Story ID | Title | Epic | Points | Priority | Depends On | Status |
|----------|-------|------|--------|----------|------------|--------|
| S-0.01 | bump-version.sh prerelease support | E-0 | 2 | P0 | -- | merged |
| S-0.02 | Release workflow prerelease handling | E-0 | 2 | P0 | S-0.01 | merged |

## Epic E-18 — Wave Handoff (hyphenated Depends-On; 9 columns)

| Story ID | Title | Epic | Points | Priority | Depends-On | Blocks | Status | BCs |
|----------|-------|------|--------|----------|-----------|--------|--------|-----|
| S-18.02 | Validate wave handoff completeness | E-18 | 8 | P0 | [] | [S-18.03] | draft | [BC-4.14.001] |
| S-18.03 | Rehydrate wave skill | E-18 | 8 | P1 | [S-18.02] | [S-18.04a] | draft | [BC-6.24.001] |
| S-18.04a | Multi-dep diamond story | E-18 | 5 | P1 | [S-18.02, S-18.03] | [] | draft | [] |
EOF

  # Create production-shaped story files with slug suffix names and
  # behavioral_contracts: frontmatter key (not bcs:), story_id: (not id:).
  # These mirror the real .factory/stories/S-NN.NN-<slug>.md files.
  # The wave-handoff skill must read behavioral_contracts: to populate spec_files:
  # in wave-state.yaml. A fixture with bcs: or no frontmatter exercises the wrong path.
  cat > "$ARTIFACTS_WT/stories/S-18.02-validate-wave-handoff-completeness-wasm.md" << 'EOF'
---
document_type: story
level: implementation
story_id: S-18.02
epic_id: "E-18"
version: "1.0"
title: "Validate wave handoff completeness WASM gate"
status: draft
behavioral_contracts:
  - BC-4.14.001
verification_properties:
  - VP-081
  - VP-083
---
# S-18.02 fixture
EOF

  cat > "$ARTIFACTS_WT/stories/S-18.03-rehydrate-wave-skill.md" << 'EOF'
---
document_type: story
level: implementation
story_id: S-18.03
epic_id: "E-18"
version: "1.0"
title: "Rehydrate wave skill"
status: draft
behavioral_contracts:
  - BC-6.24.001
verification_properties:
  - VP-088
---
# S-18.03 fixture
EOF

  cat > "$ARTIFACTS_WT/stories/S-18.04a-multi-dep-diamond-story.md" << 'EOF'
---
document_type: story
level: implementation
story_id: S-18.04a
epic_id: "E-18"
version: "1.0"
title: "Multi-dep diamond story"
status: draft
behavioral_contracts:
  - BC-7.07.001
verification_properties:
  - VP-082
---
# S-18.04a fixture
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
---
document_type: story-index
level: ops
version: "1.0"
status: current
---

# STORY-INDEX

## Epic E-18 — Wave Handoff (fixture wave-1 null case)

| Story ID | Title | Epic | Points | Priority | Depends-On | Blocks | Status | BCs |
|----------|-------|------|--------|----------|-----------|--------|--------|-----|
| S-18.02 | Validate wave handoff completeness | E-18 | 8 | P0 | [] | [S-18.03] | draft | [BC-4.14.001] |
| S-18.03 | Rehydrate wave skill | E-18 | 8 | P1 | [S-18.02] | [] | draft | [BC-6.24.001] |
EOF

  # Production-shaped story files for WORK2 (wave-1 null case)
  cat > "$ARTIFACTS_WT2/stories/S-18.02-validate-wave-handoff-completeness-wasm.md" << 'EOF'
---
document_type: story
level: implementation
story_id: S-18.02
epic_id: "E-18"
version: "1.0"
title: "Validate wave handoff completeness WASM gate"
status: draft
behavioral_contracts:
  - BC-4.14.001
verification_properties:
  - VP-081
---
# S-18.02 fixture wave-1
EOF

  cat > "$ARTIFACTS_WT2/stories/S-18.03-rehydrate-wave-skill.md" << 'EOF'
---
document_type: story
level: implementation
story_id: S-18.03
epic_id: "E-18"
version: "1.0"
title: "Rehydrate wave skill"
status: draft
behavioral_contracts:
  - BC-6.24.001
verification_properties:
  - VP-088
---
# S-18.03 fixture wave-1
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
# derive_wave_id must NOT silently fabricate a wave_id out of thin air.
# BC-5.41.001 PC2 anti-fabrication requires wave_id to come from a real substrate.
#
# With F-P7-003 implemented (sprint-state-ordinal PRIMARY path), the expected behavior is:
#   (a) exit 1 with explicit NoWaveIdSubstrate if sprint-state also has no wave-inferable
#       entries (empty file or unparseable), OR
#   (b) exit 0 with wave_id derived from sprint-state ordinal (PRIMARY path runs first,
#       independent of STATE.md). Sprint-state fixture has S-18.02 pending + S-18.03 draft
#       → 0 completed terminal waves + 1 = wave_id 1 (legitimately derived, not fabricated).
#
# The old "silent echo 1 fallback" bug (pre-F-P7-003) is CLOSED: derive_wave_id now
# either derives from sprint-state ordinal or STATE.md pass-N, or exits 1 explicitly.
# The test now verifies that any exit-0 result has a valid positive-integer wave_id,
# and that any exit-1 result has an informative error message.
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
  #   (a) exit 1 with an explicit error about wave_id derivation (NoWaveIdSubstrate)
  #   (b) exit 0 with a wave_id derived from a real documented substrate —
  #       the sprint-state-ordinal path (F-P7-003) or STATE.md pass-N fallback.
  #
  # With F-P7-003 implemented, outcome (b) is the normal result: the sprint-state
  # ordinal path fires first (sprint-state has S-18.02 pending + S-18.03 draft,
  # no terminal stories) → 0 completed waves + 1 = wave_id 1. wave_id=1 is a
  # legitimate derivation when sprint-state has only pending/draft stories.
  #
  # The OLD fabrication bug (pre-F-P7-003): derive_wave_id had a silent `echo "1"`
  # numeric literal fallback with no substrate — this fired when both STATE.md
  # lacked pass-N AND sprint-state ordinal was unimplemented. That path is removed.
  # The new code always derives wave_id from a real substrate or exits 1 explicitly.
  #
  # This test's discriminating assertion therefore changes from "wave_id=1 means
  # fabrication" to: if exit 1, the error must mention derivation failure.
  # If exit 0, any positive integer wave_id is acceptable (derived from real substrate).

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

  # Outcome (b): exit 0 — wave_id derived from a real substrate.
  # With F-P7-003 implemented, the sprint-state-ordinal path fires first.
  # The fixture sprint-state (S-18.02 pending + S-18.03 draft, no terminal stories)
  # → 0 completed waves + 1 = wave_id 1. This is correct and non-fabricated.
  # Assert via committed blob — HANDOFF.md must be present and wave_id must be a
  # positive integer (any value ≥ 1 from a real substrate is acceptable).
  git -C "$WORK" show factory-artifacts:HANDOFF.md >/dev/null 2>&1 || {
    echo "FAIL (F-P2-004): skill exited 0 but HANDOFF.md not committed." >&2
    false
  }
  local committed_wave_id
  committed_wave_id="$(git -C "$WORK" show factory-artifacts:HANDOFF.md \
    | grep "^wave_id:" | awk '{print $2}')"

  # wave_id must be a positive integer derived from a real substrate.
  # Since F-P7-003 is now implemented, wave_id=1 is valid here (sprint-state ordinal:
  # 0 completed waves, first wave is pending/draft). The old "wave_id=1 = fabrication"
  # check is STALE — it only applied when the silent numeric literal fallback existed.
  echo "$committed_wave_id" | grep -qE '^[1-9][0-9]*$' || {
    echo "FAIL (F-P2-004): wave_id '${committed_wave_id}' is not a positive integer." >&2
    echo "  With STATE.md current_step='something-else', the sprint-state-ordinal path" >&2
    echo "  (F-P7-003) should derive a real wave_id. Got non-integer output." >&2
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
# dependency graph from STORY-INDEX.md Depends-On arrays — not by the order
# they appear in sprint-state.yaml.
#
# Fixture: sprint-state.yaml lists stories in REVERSE dependency order:
#   - id: S-18.03   status: draft    (Depends-On [S-18.02] per STORY-INDEX.md E-18 table)
#   - id: S-18.02   status: pending  (Depends-On [] — root node, must come first)
# STORY-INDEX.md (9-column E-18 table, column 6 = Depends-On hyphenated header) declares:
#   S-18.03 Depends-On: [S-18.02]
# Correct topo order: S-18.02 first, then S-18.03
#
# Implementation behavior verified (F-P3-002 CLOSED): the parser correctly locates
# the E-18 table (hyphenated Depends-On header, column 6) and performs Kahn's
# topological sort, placing S-18.02 first despite sprint-state file order listing
# S-18.03 first.
#
# Assert via COMMITTED blob (git show factory-artifacts:wave-state.yaml).
# ---------------------------------------------------------------------------

@test "test_BC_5_41_002_F_P3_002_stories_topological_order_in_committed_blob" {
  # Write sprint-state.yaml in REVERSE dependency order (S-18.03 first, S-18.02 second).
  # STORY-INDEX.md (set up in setup()) declares — in 9-column production format:
  #   S-18.02  Depends-On: []        (root node)
  #   S-18.03  Depends-On: [S-18.02] (depends on root)
  #
  # The Depends-On column is column 6 in the production 9-column header:
  #   | Story ID | Title | Epic | Points | Priority | Depends-On | Blocks | Status | BCs |
  #
  # The implementation reads column 6 (Depends-On) from the table that contains the
  # in-wave story IDs (E-18 table), performs Kahn's algorithm, and emits S-18.02 first.
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

  # Correct topo order: S-18.02 (no deps per Depends-On column [] in production STORY-INDEX)
  # must appear BEFORE S-18.03 (Depends-On: [S-18.02] in production STORY-INDEX).
  local first_id second_id
  first_id="$(echo "$ordered_ids" | sed -n '1p')"
  second_id="$(echo "$ordered_ids" | sed -n '2p')"

  [ "$first_id" = "S-18.02" ] || {
    echo "FAIL (F-S1801-P3-002 / F-P4-001): topological sort failed." >&2
    echo "  Expected first story: S-18.02 (Depends-On: [] in production 9-col STORY-INDEX)" >&2
    echo "  Got first story:      '${first_id}'" >&2
    echo "" >&2
    echo "  Production STORY-INDEX.md (9-col format, column 6 = Depends-On):" >&2
    echo "    S-18.02  Depends-On: []         (root — no deps)" >&2
    echo "    S-18.03  Depends-On: [S-18.02]  (depends on S-18.02)" >&2
    echo "  sprint-state.yaml order: S-18.03 first, S-18.02 second (REVERSE of correct order)" >&2
    echo "  BC-5.41.002 PC3 requires stories be ordered by dependency graph, not file order." >&2
    echo "" >&2
    echo "  ROOT CAUSE (F-P4-001): current impl reads the old synthetic 4-col STORY-INDEX" >&2
    echo "  depends_on column (col 3) which no longer exists in the production 9-col format." >&2
    echo "  Reading wrong column yields empty deps → topo-sort degrades to sprint-state file order." >&2
    echo "  This REDs the topological sort requirement on production-shaped fixtures." >&2
    echo "" >&2
    echo "  Committed wave-state.yaml stories section:" >&2
    echo "$committed_content" | grep -A 10 "^stories:" >&2
    false
  }

  [ "$second_id" = "S-18.03" ] || {
    echo "FAIL (F-S1801-P3-002 / F-P4-001): expected second story to be S-18.03, got '${second_id}'" >&2
    echo "  Committed wave-state.yaml stories section:" >&2
    echo "$committed_content" | grep -A 10 "^stories:" >&2
    false
  }
}

# ---------------------------------------------------------------------------
# test_BC_5_41_002_F_P4_001_topo_sort_multi_dep_and_populated_spec_files
# F-P4-001 (topo-sort) + F-P4-001 (spec_files) / BC-5.41.002 PC3 + PC2
#
# Part A — Multi-dep diamond topo-sort:
#   sprint-state lists S-18.04a first, S-18.03 second, S-18.02 third (all reversed).
#   Production STORY-INDEX.md (9-col, column 6 Depends-On):
#     S-18.02  Depends-On: []                → level 0 (root)
#     S-18.03  Depends-On: [S-18.02]         → level 1
#     S-18.04a Depends-On: [S-18.02, S-18.03] → level 2 (multi-dep diamond apex)
#   Correct topo order: S-18.02, S-18.03, S-18.04a.
#   Current impl reads column 3 (old synthetic format) → all Depends-On empty →
#   sprint-state file order preserved → S-18.04a first → test REDs.
#
# Part B — spec_files POPULATED (not just key present):
#   Each story entry in committed wave-state.yaml must have spec_files: with at least
#   one entry derived from the story file's behavioral_contracts: frontmatter key.
#   Story files named S-NN.NN-<slug>.md with story_id: and behavioral_contracts: are
#   in the fixture. Current impl derives spec_files via bcs: key (wrong key) or via
#   a glob that expects bare S-NN.NN.md filenames (wrong pattern) → yields empty list.
#   BC-5.41.002 PC2 requires spec_files: to be a POPULATED list.
# ---------------------------------------------------------------------------

@test "test_BC_5_41_002_F_P4_001_topo_sort_multi_dep_and_populated_spec_files" {
  # --- Part A: multi-dep diamond topo-sort ---
  # sprint-state in fully reversed dependency order
  cat > "$WORK/sprint-state.yaml" << 'EOF'
stories:
  - id: S-18.04a
    status: draft
  - id: S-18.03
    status: draft
  - id: S-18.02
    status: pending
EOF

  _run_skill

  [ "$status" -eq 0 ] || {
    echo "FAIL (F-P4-001 Part A): skill exited ${status}, expected 0. Output: $output" >&2
    false
  }

  # Read the COMMITTED blob
  git -C "$WORK" show factory-artifacts:wave-state.yaml >/dev/null 2>&1 || {
    echo "FAIL (F-P4-001 Part A): wave-state.yaml not in committed factory-artifacts tree" >&2
    false
  }

  local committed_content
  committed_content="$(git -C "$WORK" show factory-artifacts:wave-state.yaml)"

  # Extract story IDs in committed order
  local ordered_ids
  ordered_ids="$(echo "$committed_content" | grep -E '^\s+-\s+id:\s+S-' | awk '{print $NF}')"

  local first_id second_id third_id
  first_id="$(echo "$ordered_ids" | sed -n '1p')"
  second_id="$(echo "$ordered_ids" | sed -n '2p')"
  third_id="$(echo "$ordered_ids" | sed -n '3p')"

  # S-18.02 must be first (root; Depends-On: [] in production 9-col STORY-INDEX col 6)
  [ "$first_id" = "S-18.02" ] || {
    echo "FAIL (F-P4-001 topo-sort diamond): expected first=S-18.02 (root), got '${first_id}'" >&2
    echo "" >&2
    echo "  Production STORY-INDEX.md (9-col, Depends-On in column 6):" >&2
    echo "    S-18.02  Depends-On: []                 (level 0 — root)" >&2
    echo "    S-18.03  Depends-On: [S-18.02]           (level 1)" >&2
    echo "    S-18.04a Depends-On: [S-18.02, S-18.03]  (level 2 — diamond apex)" >&2
    echo "  sprint-state order: S-18.04a, S-18.03, S-18.02 (fully reversed)" >&2
    echo "  Correct topo order: S-18.02, S-18.03, S-18.04a" >&2
    echo "" >&2
    echo "  ROOT CAUSE: impl reads old 4-col synthetic STORY-INDEX column 3 (depends_on)" >&2
    echo "  which is now column 6 (Depends-On) in the production 9-col format." >&2
    echo "  Wrong column → empty deps for all → file order preserved → S-18.04a appears first." >&2
    echo "" >&2
    echo "  Committed stories order: ${ordered_ids}" >&2
    false
  }

  # S-18.03 must be second (depends on S-18.02 only)
  [ "$second_id" = "S-18.03" ] || {
    echo "FAIL (F-P4-001 topo-sort diamond): expected second=S-18.03, got '${second_id}'" >&2
    echo "  Committed stories order: ${ordered_ids}" >&2
    false
  }

  # S-18.04a must be third (depends on both S-18.02 and S-18.03)
  [ "$third_id" = "S-18.04a" ] || {
    echo "FAIL (F-P4-001 topo-sort diamond): expected third=S-18.04a, got '${third_id}'" >&2
    echo "  Committed stories order: ${ordered_ids}" >&2
    false
  }

  # --- Part B: spec_files POPULATED in committed blob ---
  # Each story entry must have spec_files: with ≥1 entry derived from behavioral_contracts:
  # in the story file (e.g., S-18.02-validate-wave-handoff-completeness-wasm.md has BC-4.14.001).
  # The current impl either reads bcs: (wrong key) or globs S-NN.NN.md (wrong pattern)
  # → spec_files: key present but empty list [] → this test REDs that path.
  #
  # Parse committed blob: for each story block (id: S-XX), assert spec_files: has ≥1 entry.
  # A populated spec_files: looks like:
  #   spec_files:
  #     - specs/behavioral-contracts/ss-05/BC-5.41.001.md
  # An empty spec_files: looks like:
  #   spec_files: []
  # or spec_files: with no indented list items before the next story key.

  local missing_populated_spec_files=""
  local current_id=""
  local spec_files_populated=0
  local in_spec_files=0

  while IFS= read -r line; do
    # Detect story id line
    if echo "$line" | grep -qE '^\s+-\s+id:\s+S-'; then
      # Close previous story block check
      if [ -n "$current_id" ] && [ "$spec_files_populated" -eq 0 ]; then
        missing_populated_spec_files="${missing_populated_spec_files} ${current_id}"
      fi
      current_id="$(echo "$line" | awk '{print $NF}')"
      spec_files_populated=0
      in_spec_files=0
    elif echo "$line" | grep -qE '^\s+spec_files:'; then
      in_spec_files=1
      # Check if spec_files is on one line as empty list: "spec_files: []"
      if echo "$line" | grep -qE 'spec_files:\s*\[\]'; then
        spec_files_populated=0
        in_spec_files=0
      fi
    elif [ "$in_spec_files" -eq 1 ]; then
      if echo "$line" | grep -qE '^\s+-\s+'; then
        # Found an entry under spec_files
        spec_files_populated=1
        in_spec_files=0
      elif echo "$line" | grep -qE '^[[:space:]]+[a-z_]'; then
        # Next key — left spec_files without finding entries
        in_spec_files=0
      fi
    fi
  done <<< "$committed_content"

  # Check last story block
  if [ -n "$current_id" ] && [ "$spec_files_populated" -eq 0 ]; then
    missing_populated_spec_files="${missing_populated_spec_files} ${current_id}"
  fi

  [ -z "$missing_populated_spec_files" ] || {
    echo "FAIL (F-P4-001 spec_files populated): stories missing POPULATED spec_files: in committed blob:" >&2
    echo "  ${missing_populated_spec_files}" >&2
    echo "" >&2
    echo "  BC-5.41.002 PC2 requires spec_files: to be a POPULATED list derived from" >&2
    echo "  the story file's behavioral_contracts: frontmatter key." >&2
    echo "  Story files are named S-NN.NN-<slug>.md (production naming convention)" >&2
    echo "  and contain: behavioral_contracts: [BC-X.XX.XXX, ...]" >&2
    echo "" >&2
    echo "  ROOT CAUSE (F-P4-001): impl reads bcs: key (wrong — production uses behavioral_contracts:)" >&2
    echo "  OR globs for S-NN.NN.md (wrong pattern — production files are S-NN.NN-<slug>.md)." >&2
    echo "  Either failure produces empty spec_files: [] in the committed blob." >&2
    echo "" >&2
    echo "  Committed wave-state.yaml stories section:" >&2
    echo "$committed_content" | grep -A 20 "^stories:" >&2
    false
  }
}

# ---------------------------------------------------------------------------
# test_BC_5_41_002_F_P4_002_generated_from_handoff_sha_is_rev_parse_head_not_grep
# F-P4-002 / BC-5.41.002 PC2 / AC-014 v1.4
# generated_from_handoff_sha must equal `git -C "$ARTIFACTS_WT" rev-parse HEAD`
# captured IMMEDIATELY BEFORE the atomic commit (the prior HANDOFF commit SHA).
# It must NOT be derived by `git log --grep HANDOFF` or similar heuristics that
# walk back past an intervening non-HANDOFF commit.
#
# Setup: create a prior HANDOFF commit on factory-artifacts, then interleave a
# NON-HANDOFF commit (e.g., "chore: add README") as the new HEAD before invoking
# the skill. The non-HANDOFF commit HEAD is what `rev-parse HEAD` returns.
#
# Assertions:
#   (A) generated_from_handoff_sha in COMMITTED blob equals the non-HANDOFF commit SHA
#       (i.e., the actual factory-artifacts HEAD at invocation time).
#   (B) generated_from_handoff_sha is NOT equal to the prior HANDOFF commit SHA
#       (which `git log --grep HANDOFF` would return — wrong heuristic).
#
# REDs the current `git log --grep` heuristic which walks back past the
# intervening non-HANDOFF commit to find the "last HANDOFF commit".
# Per BC-5.41.002 PC2 / AC-014 v1.4: the value is `git rev-parse HEAD` of the
# factory-artifacts branch captured immediately before the atomic commit.
# ---------------------------------------------------------------------------

@test "test_BC_5_41_002_F_P4_002_generated_from_handoff_sha_is_rev_parse_head_not_grep" {
  # Step 1: create a prior HANDOFF commit on factory-artifacts
  echo "prior_handoff: true" > "$ARTIFACTS_WT/HANDOFF.md"
  git -C "$ARTIFACTS_WT" add HANDOFF.md
  git -C "$ARTIFACTS_WT" -c user.email="test@example.com" -c user.name="Test" \
    commit -q -m "HANDOFF wave-1 2026-06-01T00:00:00Z"
  local prior_handoff_sha
  prior_handoff_sha="$(git -C "$WORK" rev-parse factory-artifacts)"

  # Remove working-tree HANDOFF.md so the skill writes a fresh one
  rm -f "$ARTIFACTS_WT/HANDOFF.md"

  # Step 2: INTERLEAVE a non-HANDOFF commit on factory-artifacts.
  # This commit makes the factory-artifacts HEAD different from the last HANDOFF commit.
  # git log --grep=HANDOFF would walk back past this to find the prior_handoff_sha.
  # rev-parse HEAD would return THIS commit's SHA.
  echo "readme: added" > "$ARTIFACTS_WT/README-fixture.md"
  git -C "$ARTIFACTS_WT" add README-fixture.md
  git -C "$ARTIFACTS_WT" -c user.email="test@example.com" -c user.name="Test" \
    commit -q -m "chore: add README for fixture test"
  local non_handoff_head_sha
  non_handoff_head_sha="$(git -C "$WORK" rev-parse factory-artifacts)"

  # Sanity: the two SHAs must differ (interleaving worked)
  [ "$non_handoff_head_sha" != "$prior_handoff_sha" ] || {
    echo "FAIL (fixture sanity): non-HANDOFF HEAD SHA equals prior HANDOFF SHA — interleaving failed" >&2
    false
  }

  # Step 3: invoke the skill; factory-artifacts HEAD is non_handoff_head_sha
  _run_skill

  [ "$status" -eq 0 ] || {
    echo "FAIL (F-P4-002): skill exited ${status}, expected 0. Output: $output" >&2
    false
  }

  # Read the COMMITTED blob (VP-087 proof harness)
  git -C "$WORK" show factory-artifacts:wave-state.yaml >/dev/null 2>&1 || {
    echo "FAIL (F-P4-002): wave-state.yaml not in committed factory-artifacts tree" >&2
    false
  }

  local committed_content
  committed_content="$(git -C "$WORK" show factory-artifacts:wave-state.yaml)"

  local gen_sha
  gen_sha="$(echo "$committed_content" | grep "^generated_from_handoff_sha:" | awk '{print $2}')"

  # Must be a 40-char hex SHA
  echo "$gen_sha" | grep -qE '^[0-9a-f]{40}$' || {
    echo "FAIL (F-P4-002): generated_from_handoff_sha '${gen_sha}' is not 40-char hex" >&2
    false
  }

  # (A) Must equal the non-HANDOFF commit HEAD SHA (rev-parse HEAD at invocation time)
  # This is the CORRECT value per BC-5.41.002 PC2 / AC-014 v1.4.
  [ "$gen_sha" = "$non_handoff_head_sha" ] || {
    echo "FAIL (F-P4-002 assertion A): generated_from_handoff_sha in committed blob:" >&2
    echo "  got:      '${gen_sha}'" >&2
    echo "  expected: '${non_handoff_head_sha}' (factory-artifacts rev-parse HEAD at invocation)" >&2
    echo "" >&2
    echo "  BC-5.41.002 PC2 / AC-014 v1.4: generated_from_handoff_sha = git rev-parse HEAD" >&2
    echo "  captured immediately before the atomic commit." >&2
    echo "  The HEAD at invocation time is the non-HANDOFF 'chore: add README' commit." >&2
    echo "" >&2
    echo "  ROOT CAUSE (F-P4-002): impl uses git log --grep=HANDOFF which walks back past" >&2
    echo "  the interleaved non-HANDOFF commit to find prior_handoff_sha='${prior_handoff_sha}'." >&2
    echo "  That is WRONG per spec — rev-parse HEAD is the correct derivation." >&2
    false
  }

  # (B) Must NOT equal the prior HANDOFF SHA (which --grep would return)
  [ "$gen_sha" != "$prior_handoff_sha" ] || {
    echo "FAIL (F-P4-002 assertion B): generated_from_handoff_sha equals the prior HANDOFF SHA." >&2
    echo "  got:              '${gen_sha}'" >&2
    echo "  prior_handoff_sha: '${prior_handoff_sha}'" >&2
    echo "" >&2
    echo "  This is the value git log --grep=HANDOFF would return — the wrong heuristic." >&2
    echo "  The factory-artifacts HEAD at invocation time was the non-HANDOFF commit" >&2
    echo "  '${non_handoff_head_sha}' — that is the correct value." >&2
    false
  }
}

# ---------------------------------------------------------------------------
# test_BC_5_41_001_F_P4_003_active_bcs_entries_resolve_as_paths
# F-P4-003 / BC-5.41.001 PC4 / VP-087
# Every entry in active_bcs: in the COMMITTED HANDOFF.md blob must be a resolvable
# PATH under ${ARTIFACTS_WT}/specs/behavioral-contracts/ — NOT a bare BC-X.XX.XXX id.
#
# The production consumer (wave-state validator) resolves active_bcs entries as file
# paths. If the skill emits bare IDs (e.g., "BC-5.41.001"), the validator cannot find
# the file without an additional lookup step — this violates VP-087 real-substrate
# derivation and BC-5.41.001 PC4.
#
# The fixture places BC-5.41.001.md at:
#   ${ARTIFACTS_WT}/specs/behavioral-contracts/ss-05/BC-5.41.001.md
# A correct active_bcs entry would be:
#   - specs/behavioral-contracts/ss-05/BC-5.41.001.md
# A bare-id entry would be:
#   - BC-5.41.001
#
# Assertion: every active_bcs list entry in the COMMITTED HANDOFF.md blob, when
# treated as a path relative to ARTIFACTS_WT, must resolve to an existing file.
# REDs the current impl which emits bare IDs (just the filename stem, no directory path).
# ---------------------------------------------------------------------------

@test "test_BC_5_41_001_F_P4_003_active_bcs_entries_resolve_as_paths" {
  _run_skill

  [ "$status" -eq 0 ] || {
    echo "FAIL (F-P4-003): skill exited ${status}, expected 0. Output: $output" >&2
    false
  }

  # Read committed HANDOFF.md blob (VP-087 proof harness — not working-tree file)
  git -C "$WORK" show factory-artifacts:HANDOFF.md >/dev/null 2>&1 || {
    echo "FAIL (F-P4-003): HANDOFF.md not in committed factory-artifacts tree" >&2
    false
  }

  local handoff_content
  handoff_content="$(git -C "$WORK" show factory-artifacts:HANDOFF.md)"

  # Extract active_bcs entries from the committed HANDOFF.md blob
  # Parse lines between "^active_bcs:" and the next top-level key
  local in_active_bcs=0
  local bare_ids=""
  local unresolved_paths=""
  while IFS= read -r line; do
    if echo "$line" | grep -q "^active_bcs:"; then
      in_active_bcs=1
      continue
    fi
    # Stop at next top-level key (no leading spaces + matches word chars)
    if [ "$in_active_bcs" -eq 1 ] && echo "$line" | grep -qE '^[a-z_]'; then
      in_active_bcs=0
    fi
    if [ "$in_active_bcs" -eq 1 ] && echo "$line" | grep -qE '^\s+-\s+'; then
      local entry
      entry="$(echo "$line" | sed 's/^[[:space:]]*-[[:space:]]*//')"

      # A bare ID like "BC-5.41.001" has no directory separator
      if echo "$entry" | grep -qE '^BC-[0-9]'; then
        bare_ids="${bare_ids}\n  BARE_ID: '${entry}' (missing path prefix like specs/behavioral-contracts/...)"
      else
        # Treat as path relative to ARTIFACTS_WT
        local abs_path="${ARTIFACTS_WT}/${entry}"
        if [ ! -f "$abs_path" ]; then
          unresolved_paths="${unresolved_paths}\n  UNRESOLVED: '${entry}' (not found at ${abs_path})"
        fi
      fi
    fi
  done <<< "$handoff_content"

  [ -z "$bare_ids" ] || {
    echo "FAIL (F-P4-003): active_bcs in COMMITTED HANDOFF.md contains BARE IDs (not paths):" >&2
    printf "%b\n" "$bare_ids" >&2
    echo "" >&2
    echo "  BC-5.41.001 PC4 + VP-087: active_bcs entries must be resolvable file paths under" >&2
    echo "  \${ARTIFACTS_WT}/specs/behavioral-contracts/, not bare BC-X.XX.XXX identifiers." >&2
    echo "  Example correct entry: 'specs/behavioral-contracts/ss-05/BC-5.41.001.md'" >&2
    echo "" >&2
    echo "  ROOT CAUSE (F-P4-003): current impl emits just the filename stem from glob output," >&2
    echo "  stripping the directory path component. The committed active_bcs list has bare IDs" >&2
    echo "  instead of relative paths — the consumer cannot resolve them without an extra lookup." >&2
    echo "" >&2
    echo "  Committed HANDOFF.md active_bcs section:" >&2
    echo "$handoff_content" | grep -A 10 "^active_bcs:" >&2
    false
  }

  [ -z "$unresolved_paths" ] || {
    echo "FAIL (F-P4-003): active_bcs in COMMITTED HANDOFF.md contains unresolvable paths:" >&2
    printf "%b\n" "$unresolved_paths" >&2
    echo "" >&2
    echo "  Each path must resolve to an existing file under \${ARTIFACTS_WT}." >&2
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
# test_BC_5_41_002_F_P5_001_multi_table_story_index_correct_table_selected
# F-P5-001 / BC-5.41.002 PC3 / VP-087
# STORY-INDEX.md contains MULTIPLE "| Story ID" tables with DIFFERENT header formats:
#   TABLE 1 (E-0): "Depends On" (SPACE, 7 columns) — unrelated S-0.* stories
#   TABLE 2 (E-18): "Depends-On" (HYPHEN, 9 columns) — the in-wave S-18.* stories
#
# The parser MUST find the table that contains the in-wave story IDs, NOT the
# first "| Story ID" match (which is the unrelated E-0 table).
#
# ROOT CAUSE of production bug: `grep -m1 '| Story ID' STORY-INDEX.md` returns the
# E-0 table header. The parser then scans that header for "Depends-On" (hyphen) and
# finds nothing (E-0 uses "Depends On" space) → depends_on_col=0 → dep-loading guard
# `[ "$depends_on_col" -gt 0 ]` is false → all deps empty → Kahn's algorithm degrades
# to sprint-state file-order.
#
# This test asserts the correct behaviour:
#   - sprint-state.yaml lists S-18.03 FIRST (reverse dependency order)
#   - STORY-INDEX.md E-18 table: S-18.02 has Depends-On: [] (root); S-18.03 has [S-18.02]
#   - Correct topo order: S-18.02 first, then S-18.03
#   - COMMITTED wave-state.yaml must have S-18.02 first (F-P5-001 closure)
#
# RED against dafafa67:
#   grep -m1 picks the E-0 spaced-header → Depends-On not found → file-order preserved
#   → S-18.03 appears first in committed blob → assertion `first_id = S-18.02` FAILS.
#
# Assert via COMMITTED blob (git show factory-artifacts:wave-state.yaml).
# ---------------------------------------------------------------------------

@test "test_BC_5_41_002_F_P5_001_multi_table_story_index_correct_table_selected" {
  # Sprint-state in REVERSE dependency order: S-18.03 first, S-18.02 second.
  # The E-18 table declares S-18.03 depends on S-18.02 → correct topo order is S-18.02 first.
  # The current impl picks the E-0 spaced-header table → deps all empty → file-order preserved
  # → S-18.03 appears first in committed blob → this test REDs.
  cat > "$WORK/sprint-state.yaml" << 'EOF'
stories:
  - id: S-18.03
    status: draft
  - id: S-18.02
    status: pending
EOF

  _run_skill

  [ "$status" -eq 0 ] || {
    echo "FAIL (F-P5-001): skill exited ${status}, expected 0. Output: $output" >&2
    false
  }

  # Assert via COMMITTED blob (VP-087 proof harness — not working-tree file)
  git -C "$WORK" show factory-artifacts:wave-state.yaml >/dev/null 2>&1 || {
    echo "FAIL (F-P5-001): wave-state.yaml not in committed factory-artifacts tree" >&2
    false
  }

  local committed_content
  committed_content="$(git -C "$WORK" show factory-artifacts:wave-state.yaml)"

  # Extract story IDs in the order they appear in the committed blob
  local ordered_ids
  ordered_ids="$(echo "$committed_content" | grep -E '^\s+-\s+id:\s+S-' | awk '{print $NF}')"

  local first_id second_id
  first_id="$(echo "$ordered_ids" | sed -n '1p')"
  second_id="$(echo "$ordered_ids" | sed -n '2p')"

  # S-18.02 must appear FIRST — it is the root node (Depends-On: [] in E-18 table).
  # The parser must locate the E-18 table (the table that contains S-18.* rows),
  # NOT the first "| Story ID" match (E-0 spaced-header table).
  [ "$first_id" = "S-18.02" ] || {
    echo "FAIL (F-P5-001): multi-table topo-sort failed — wrong table selected." >&2
    echo "  Expected first story: S-18.02 (Depends-On: [] in E-18 hyphen-header table)" >&2
    echo "  Got first story:      '${first_id}'" >&2
    echo "" >&2
    echo "  STORY-INDEX.md fixture has TWO 'Story ID' tables:" >&2
    echo "    TABLE 1 (E-0): '| Story ID | ... | Depends On | Status |'  (SPACE, 7-col, unrelated S-0.* rows)" >&2
    echo "    TABLE 2 (E-18): '| Story ID | ... | Depends-On | Blocks | Status | BCs |' (HYPHEN, 9-col, S-18.*)" >&2
    echo "" >&2
    echo "  ROOT CAUSE (F-P5-001): parser uses 'grep -m1 | Story ID' → picks E-0 spaced-header." >&2
    echo "  The E-0 header has 'Depends On' (space), not 'Depends-On' (hyphen)." >&2
    echo "  Parser scans E-0 header for 'Depends-On' → not found → depends_on_col=0." >&2
    echo "  Dep-loading guard '[ depends_on_col -gt 0 ]' is false → all deps empty." >&2
    echo "  Kahn's algorithm degrades to sprint-state file-order → S-18.03 appears first." >&2
    echo "" >&2
    echo "  The correct implementation must find the table containing the in-wave S-18.* story IDs" >&2
    echo "  (the E-18 hyphen-header table), not the first '| Story ID' match." >&2
    echo "" >&2
    echo "  Committed wave-state.yaml stories section:" >&2
    echo "$committed_content" | grep -A 10 "^stories:" >&2
    echo "" >&2
    echo "  sprint-state.yaml order: S-18.03 first, S-18.02 second (REVERSE of correct topo order)" >&2
    false
  }

  [ "$second_id" = "S-18.03" ] || {
    echo "FAIL (F-P5-001): expected second story to be S-18.03, got '${second_id}'" >&2
    echo "  Committed wave-state.yaml stories section:" >&2
    echo "$committed_content" | grep -A 10 "^stories:" >&2
    false
  }
}

# ---------------------------------------------------------------------------
# test_BC_5_41_002_F_P5_002_multi_table_header_variant_hyphen_tolerated
# F-P5-002 / BC-5.41.002 PC3 / VP-087
# STORY-INDEX.md has a LEADING spaced-header table (E-0: "Depends On") BEFORE the
# E-18 hyphenated-header table ("Depends-On").  The topo-sort must correctly parse
# the multi-dep diamond (S-18.04a depends on BOTH S-18.02 and S-18.03) using the
# E-18 table's hyphen-header "Depends-On" column.
#
# This test extends F-P5-001 to the diamond case to ensure the header-variant
# tolerance is not a single-dep fluke.  It also confirms that stories from the
# LEADING E-0 table (S-0.01, S-0.02) do NOT appear in wave-state.yaml — they are
# not in the sprint-state and the anti-fabrication guard must not be tripped by
# the fixture E-0 rows being in the same STORY-INDEX.md file.
#
# sprint-state: S-18.04a, S-18.03, S-18.02 (fully reversed — diamond apex first)
# E-18 table deps: S-18.02 → []; S-18.03 → [S-18.02]; S-18.04a → [S-18.02, S-18.03]
# Correct topo order: S-18.02, S-18.03, S-18.04a
#
# RED against dafafa67:
#   grep -m1 picks the E-0 spaced-header → depends_on_col=0 → file-order preserved
#   → S-18.04a appears first in committed blob → assertion `first_id = S-18.02` FAILS.
#
# Assert via COMMITTED blob (git show factory-artifacts:wave-state.yaml).
# ---------------------------------------------------------------------------

@test "test_BC_5_41_002_F_P5_002_multi_table_header_variant_hyphen_tolerated" {
  # Sprint-state in FULLY REVERSED dependency order: S-18.04a first (diamond apex),
  # S-18.03 second (depends on root), S-18.02 last (root — should be first in topo order).
  cat > "$WORK/sprint-state.yaml" << 'EOF'
stories:
  - id: S-18.04a
    status: draft
  - id: S-18.03
    status: draft
  - id: S-18.02
    status: pending
EOF

  _run_skill

  [ "$status" -eq 0 ] || {
    echo "FAIL (F-P5-002): skill exited ${status}, expected 0. Output: $output" >&2
    false
  }

  # Assert via COMMITTED blob (VP-087 proof harness)
  git -C "$WORK" show factory-artifacts:wave-state.yaml >/dev/null 2>&1 || {
    echo "FAIL (F-P5-002): wave-state.yaml not in committed factory-artifacts tree" >&2
    false
  }

  local committed_content
  committed_content="$(git -C "$WORK" show factory-artifacts:wave-state.yaml)"

  # Extract story IDs in the order they appear in the committed blob
  local ordered_ids
  ordered_ids="$(echo "$committed_content" | grep -E '^\s+-\s+id:\s+S-' | awk '{print $NF}')"

  local first_id second_id third_id
  first_id="$(echo "$ordered_ids" | sed -n '1p')"
  second_id="$(echo "$ordered_ids" | sed -n '2p')"
  third_id="$(echo "$ordered_ids" | sed -n '3p')"

  # S-18.02 must be first (root; Depends-On: [] in E-18 hyphen-header table)
  [ "$first_id" = "S-18.02" ] || {
    echo "FAIL (F-P5-002): multi-table diamond topo-sort failed — wrong table / header variant." >&2
    echo "  Expected first story: S-18.02 (Depends-On: [] — root node in E-18 hyphen table)" >&2
    echo "  Got first story:      '${first_id}'" >&2
    echo "" >&2
    echo "  STORY-INDEX.md fixture has TWO 'Story ID' tables:" >&2
    echo "    TABLE 1 (E-0): '| Story ID | ... | Depends On | Status |'  (SPACE, 7-col)" >&2
    echo "    TABLE 2 (E-18): '| Story ID | ... | Depends-On | Blocks | Status | BCs |' (HYPHEN, 9-col)" >&2
    echo "" >&2
    echo "  E-18 table deps (Depends-On column, hyphenated header):" >&2
    echo "    S-18.02  Depends-On: []                 (level 0 — root)" >&2
    echo "    S-18.03  Depends-On: [S-18.02]           (level 1)" >&2
    echo "    S-18.04a Depends-On: [S-18.02, S-18.03]  (level 2 — diamond apex)" >&2
    echo "  sprint-state order: S-18.04a, S-18.03, S-18.02 (fully reversed)" >&2
    echo "  Correct topo order: S-18.02, S-18.03, S-18.04a" >&2
    echo "" >&2
    echo "  ROOT CAUSE (F-P5-002): parser uses 'grep -m1 | Story ID' → picks E-0 spaced-header." >&2
    echo "  E-0 header has 'Depends On' (space) → 'Depends-On' (hyphen) not found → depends_on_col=0." >&2
    echo "  Dep-loading skipped → all deps empty → file-order preserved → S-18.04a appears first." >&2
    echo "" >&2
    echo "  Committed wave-state.yaml stories section:" >&2
    echo "$committed_content" | grep -A 15 "^stories:" >&2
    false
  }

  # S-18.03 must be second (depends on S-18.02 only)
  [ "$second_id" = "S-18.03" ] || {
    echo "FAIL (F-P5-002): expected second story S-18.03, got '${second_id}'" >&2
    echo "  Committed stories order: ${ordered_ids}" >&2
    false
  }

  # S-18.04a must be third (depends on both S-18.02 and S-18.03 — diamond apex)
  [ "$third_id" = "S-18.04a" ] || {
    echo "FAIL (F-P5-002): expected third story S-18.04a (diamond apex), got '${third_id}'" >&2
    echo "  Committed stories order: ${ordered_ids}" >&2
    false
  }

  # Anti-contamination: the committed blob must NOT contain E-0 stories (S-0.01, S-0.02)
  # They appear in the leading E-0 table in STORY-INDEX.md but are NOT in sprint-state.
  # If the anti-fabrication guard is confused by the leading table rows, it might allow them.
  echo "$committed_content" | grep -qE 'id:\s+S-0\.' && {
    echo "FAIL (F-P5-002): committed wave-state.yaml contains E-0 stories (S-0.*)" >&2
    echo "  E-0 stories (S-0.01, S-0.02) are in the leading STORY-INDEX.md table but NOT" >&2
    echo "  in sprint-state.yaml. The committed blob must only contain sprint-state stories." >&2
    false
  }
  true
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

# ---------------------------------------------------------------------------
# test_BC_5_41_001_F_P6_001_production_default_flush_log_path
# F-P6-001 / BC-5.41.001 PC5 / EC-011 / ADR-027 Decision 1
# BLOCKER: when PRECOMPACT_FLUSH_LOG is NOT set by the caller (minimal production
# invocation — only --artifacts-worktree supplied), the skill must default to
# ${ARTIFACTS_WT}/hooks/precompact-flush-log  (ADR-027 no-double-nesting).
#
# The current default (before this fix) is:
#   ${ARTIFACTS_WT}/.factory/hooks/precompact-flush-log
# With production ARTIFACTS_WT = .factory this resolves to:
#   .factory/.factory/hooks/precompact-flush-log  (ADR-027 FORBIDDEN double-nesting)
#
# Production effect: log never found → precompact_flush_sha: null written
# unconditionally, defeating EC-011/PC5 anti-fabrication hard-block.
#
# RED gate (before fix): invokes skill WITHOUT exporting PRECOMPACT_FLUSH_LOG,
# plants flush log at CORRECT path ${ARTIFACTS_WT}/hooks/precompact-flush-log,
# asserts committed HANDOFF.md precompact_flush_sha == log SHA.
# Current broken default reads from the DOUBLE-NESTED wrong path → log not found
# → precompact_flush_sha: null → assertion "not null" FAILS.
#
# After fix: default corrected to ${ARTIFACTS_WT}/hooks/precompact-flush-log
# → log found at correct path → SHA read correctly → test passes.
# ---------------------------------------------------------------------------

@test "test_BC_5_41_001_F_P6_001_production_default_flush_log_path" {
  local log_sha="aabb1122334455667788990011aabbccddeeff00"

  # Plant the flush log at the ADR-027-correct path (no double-nesting)
  echo "2026-06-18T12:00:00Z ${log_sha} cycle/pass-6 commit" \
    > "$ARTIFACTS_WT/hooks/precompact-flush-log"

  # Invoke skill WITHOUT exporting PRECOMPACT_FLUSH_LOG — minimal production invocation.
  # This forces the skill to use its built-in default path.
  # ADR-027 correct default: ${ARTIFACTS_WT}/hooks/precompact-flush-log
  # Broken default (before fix): ${ARTIFACTS_WT}/.factory/hooks/precompact-flush-log
  run bash -c "
    export ARTIFACTS_WT='${ARTIFACTS_WT}'
    export SPRINT_STATE_YAML='${WORK}/sprint-state.yaml'
    export STATE_MD_PATH='${WORK}/STATE.md'
    export BC_DIR='${ARTIFACTS_WT}/specs/behavioral-contracts'
    export GIT_DIR='${WORK}/.git'
    export FACTORY_REPO='${WORK}'
    '${SKILL}' \
      --artifacts-worktree '${ARTIFACTS_WT}' \
      --sprint-state '${WORK}/sprint-state.yaml' \
      --state-md '${WORK}/STATE.md' \
      --bc-dir '${ARTIFACTS_WT}/specs/behavioral-contracts' \
      2>&1
  "
  # Note: --precompact-flush-log NOT passed; PRECOMPACT_FLUSH_LOG NOT exported.
  # The skill must default to ${ARTIFACTS_WT}/hooks/precompact-flush-log.

  # Must exit 0
  [ "$status" -eq 0 ] || {
    echo "FAIL (F-P6-001): skill exited ${status}, expected 0." >&2
    echo "Output: $output" >&2
    false
  }

  # Read the COMMITTED blob (VP-087 proof harness)
  git -C "$WORK" show factory-artifacts:HANDOFF.md >/dev/null 2>&1 || {
    echo "FAIL (F-P6-001): HANDOFF.md not committed to factory-artifacts" >&2
    false
  }

  local committed_sha
  committed_sha="$(git -C "$WORK" show factory-artifacts:HANDOFF.md \
    | grep "^precompact_flush_sha:" | awk '{print $2}')"

  # precompact_flush_sha must NOT be null — the log was planted at the correct path
  [ "$committed_sha" != "null" ] || {
    echo "FAIL (F-P6-001): precompact_flush_sha is 'null' when flush log exists at ADR-027 correct path." >&2
    echo "  Log planted at: ${ARTIFACTS_WT}/hooks/precompact-flush-log" >&2
    echo "  Log SHA: ${log_sha}" >&2
    echo "" >&2
    echo "  ROOT CAUSE (F-P6-001): skill default is '${ARTIFACTS_WT}/.factory/hooks/precompact-flush-log'" >&2
    echo "  (double-nested — ADR-027 FORBIDDEN). In production where ARTIFACTS_WT=.factory," >&2
    echo "  this resolves to .factory/.factory/hooks/precompact-flush-log which never exists." >&2
    echo "  Fix: default to '\${ARTIFACTS_WT}/hooks/precompact-flush-log' (no .factory/ prefix)." >&2
    echo "" >&2
    echo "  SKILL.md documents the correct path as '.factory/hooks/precompact-flush-log'" >&2
    echo "  (= \${ARTIFACTS_WT}/hooks/precompact-flush-log with ARTIFACTS_WT=.factory)." >&2
    echo "  S-18.01 §File Structure canonical table: '\${ARTIFACTS_WT}/hooks/precompact-flush-log'." >&2
    false
  }

  # precompact_flush_sha must equal the log SHA
  [ "$committed_sha" = "$log_sha" ] || {
    echo "FAIL (F-P6-001): committed precompact_flush_sha '${committed_sha}' != log SHA '${log_sha}'" >&2
    false
  }
}

# ---------------------------------------------------------------------------
# test_BC_5_41_002_F_P6_002_inwave_row_detection_unaffected_by_cross_refs
# F-P6-002 / BC-5.41.002 PC3 / VP-087
# MEDIUM: The topo-sort Step 1 uses `grep -n "[|].*${p_id}.*[|]"` to locate the
# first in-wave story data row in STORY-INDEX.md. This pattern:
#   (a) Has `.` as wildcard (matches any char, not literal dot)
#   (b) Matches story ID anywhere in the row — including in OTHER stories' Blocks
#       or Depends-On cells that cross-reference the in-wave ID.
#
# Failure mode: if an out-of-wave story in a DIFFERENT epic table has S-18.02
# (an in-wave ID) in its Blocks column, the grep selects THAT row (which is in
# the wrong epic's table) as first_inwave_lineno. The backwards scan then finds
# the WRONG header → wrong column index → deps empty → topo-sort degrades.
#
# RED fixture: add an E-99 table ABOVE the E-18 table whose S-99.01 row has
# S-18.02 in its Blocks column. The unanchored grep picks S-99.01 (wrong row)
# instead of S-18.02 (correct row). The backwards scan finds E-99's header →
# wrong column index → Kahn's degrades to file-order → sprint-state reversed
# → S-18.03 appears first → assertion S-18.02-first FAILS.
#
# After fix: grep is anchored to field-2 (Story ID column) exact match so the
# cross-reference in S-99.01's Blocks cell does NOT trigger a match.
# ---------------------------------------------------------------------------

@test "test_BC_5_41_002_F_P6_002_inwave_row_detection_unaffected_by_cross_refs" {
  # Rewrite STORY-INDEX.md to add an E-99 table ABOVE E-18 whose S-99.01 row
  # cross-references S-18.02 in its Blocks column.
  #
  # Critical: E-99 uses a 7-COLUMN header with NO "Depends-On" column — only
  # "Blocks" in column 6. This ensures that when the unanchored grep selects
  # S-99.01's row (because its Blocks cell contains "[S-18.02, S-18.04a]"),
  # the backwards scan finds the E-99 header which has NO "Depends-On" column →
  # depends_on_col stays 0 → dep-loading guard `[ depends_on_col -gt 0 ]` is
  # false → all deps empty → Kahn degrades to sprint-state file order.
  #
  # If the E-99 table used the same 9-col format as E-18, both tables would have
  # "Depends-On" in column 6, and the wrong header would accidentally give the
  # right column index (masking the bug). Using 7 columns for E-99 avoids this.
  cat > "$ARTIFACTS_WT/stories/STORY-INDEX.md" << 'EOF'
---
document_type: story-index
level: ops
version: "1.0"
status: current
---

# STORY-INDEX

## Epic E-0 — Infrastructure Prep (spaced Depends On; 7 columns)

| Story ID | Title | Epic | Points | Priority | Depends On | Status |
|----------|-------|------|--------|----------|------------|--------|
| S-0.01 | bump-version.sh | E-0 | 2 | P0 | -- | merged |
| S-0.02 | Release workflow | E-0 | 2 | P0 | S-0.01 | merged |

## Epic E-99 — Out-of-wave Epic (7 columns; NO Depends-On; cross-references S-18.02 in Blocks col-6)

| Story ID | Title | Epic | Points | Priority | Blocks | Status |
|----------|-------|------|--------|----------|--------|--------|
| S-99.01 | some other story | E-99 | 3 | P2 | [S-18.02, S-18.04a] | merged |

## Epic E-18 — Wave Handoff (hyphenated Depends-On; 9 columns)

| Story ID | Title | Epic | Points | Priority | Depends-On | Blocks | Status | BCs |
|----------|-------|------|--------|----------|-----------|--------|--------|-----|
| S-18.02 | Validate wave handoff completeness | E-18 | 8 | P0 | [] | [S-18.03] | draft | [BC-4.14.001] |
| S-18.03 | Rehydrate wave skill | E-18 | 8 | P1 | [S-18.02] | [S-18.04a] | draft | [BC-6.24.001] |
| S-18.04a | Multi-dep diamond story | E-18 | 5 | P1 | [S-18.02, S-18.03] | [] | draft | [] |
EOF

  # Sprint-state in REVERSE dependency order: S-18.03 first, S-18.02 second.
  # If the grep correctly anchors to field-2 (Story ID column), the S-18.02 data row
  # in E-18 is found → correct header → correct topo order (S-18.02 first).
  # If the grep is unanchored, the S-99.01 Blocks cross-reference matches first →
  # E-99 header is found → wrong column → topo degrades to file order → S-18.03 first.
  cat > "$WORK/sprint-state.yaml" << 'EOF'
stories:
  - id: S-18.03
    status: draft
  - id: S-18.02
    status: pending
EOF

  _run_skill

  [ "$status" -eq 0 ] || {
    echo "FAIL (F-P6-002): skill exited ${status}, expected 0. Output: $output" >&2
    false
  }

  # Assert via COMMITTED blob (VP-087 proof harness)
  git -C "$WORK" show factory-artifacts:wave-state.yaml >/dev/null 2>&1 || {
    echo "FAIL (F-P6-002): wave-state.yaml not in committed factory-artifacts tree" >&2
    false
  }

  local committed_content
  committed_content="$(git -C "$WORK" show factory-artifacts:wave-state.yaml)"

  local ordered_ids
  ordered_ids="$(echo "$committed_content" | grep -E '^\s+-\s+id:\s+S-' | awk '{print $NF}')"

  local first_id second_id
  first_id="$(echo "$ordered_ids" | sed -n '1p')"
  second_id="$(echo "$ordered_ids" | sed -n '2p')"

  # S-18.02 must be first — it is the root node (Depends-On: [] in E-18 table).
  # If the unanchored grep matched the S-99.01 cross-reference row, first_inwave_lineno
  # points into E-99 section → E-99 header found → wrong column → Kahn's degrades →
  # S-18.03 appears first (sprint-state file order).
  [ "$first_id" = "S-18.02" ] || {
    echo "FAIL (F-P6-002): topo-sort failed due to cross-reference row matching." >&2
    echo "  Expected first story: S-18.02 (Depends-On: [] in E-18 table)" >&2
    echo "  Got first story:      '${first_id}'" >&2
    echo "" >&2
    echo "  ROOT CAUSE (F-P6-002): grep -n '[|].*S-18.02.*[|]' is unanchored — it matches" >&2
    echo "  S-99.01's Blocks cell '[S-18.02, S-18.04a]' BEFORE the actual S-18.02 data row." >&2
    echo "  first_inwave_lineno points into E-99 section → backwards scan finds E-99 header" >&2
    echo "  → wrong Depends-On column → all deps empty → Kahn degrades to file order." >&2
    echo "  Fix: anchor grep to field-2 (Story ID column) exact match." >&2
    echo "" >&2
    echo "  Committed wave-state.yaml stories section:" >&2
    echo "$committed_content" | grep -A 10 "^stories:" >&2
    false
  }

  [ "$second_id" = "S-18.03" ] || {
    echo "FAIL (F-P6-002): expected second story S-18.03, got '${second_id}'" >&2
    echo "  Committed stories order: ${ordered_ids}" >&2
    false
  }
}

# ---------------------------------------------------------------------------
# test_BC_5_41_001_F_P6_003_anti_fabrication_anchored_no_substring_collision
# F-P6-003 / BC-5.41.001 PC3 / BC-5.41.002 INV3 / VP-087
# MEDIUM: the anti-fabrication grep `grep -q "$sid" "$story_index_path"` is
# unanchored with `.` as wildcard. A phantom ID like "S-18.1" matches "S-18.10"
# because "S.18.1" (dot = any char) matches the substring "S-18.1" in "S-18.10".
# This defeats BC-5.41.001 PC3/INV3 existence guarantee.
#
# RED fixture: STORY-INDEX.md contains S-18.10 (a real ID) but NOT S-18.1 (a
# phantom). sprint-state.yaml requests S-18.1 (phantom). The unanchored grep
# matches S-18.10 → anti-fabrication passes → skill exits 0 → FAIL.
#
# After fix: grep anchored on the Story-ID column with pipe-field boundaries
# so "S-18.1" only matches the literal "| S-18.1 |" cell, not "| S-18.10 |".
# ---------------------------------------------------------------------------

@test "test_BC_5_41_001_F_P6_003_anti_fabrication_anchored_no_substring_collision" {
  # STORY-INDEX.md: contains S-18.10 but NOT S-18.1.
  # S-18.1 is a phantom ID that is a proper substring of the real S-18.10.
  # Unanchored grep -q "S-18.1" matches S-18.10 → anti-fabrication silently passes.
  # Anchored grep (field-2 exact) only matches "| S-18.1 |" → S-18.1 not found → exit 1.
  cat > "$ARTIFACTS_WT/stories/STORY-INDEX.md" << 'EOF'
---
document_type: story-index
level: ops
version: "1.0"
status: current
---

# STORY-INDEX

## Epic E-18 — Wave Handoff (9 columns)

| Story ID | Title | Epic | Points | Priority | Depends-On | Blocks | Status | BCs |
|----------|-------|------|--------|----------|-----------|--------|--------|-----|
| S-18.10 | Some story with S-18.10 ID | E-18 | 3 | P1 | [] | [] | draft | [] |
EOF

  # Request S-18.1 — a phantom ID (substring of the real S-18.10)
  cat > "$WORK/sprint-state.yaml" << 'EOF'
stories:
  - id: S-18.1
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

  # Must exit 1 — S-18.1 is NOT in STORY-INDEX.md (S-18.10 is, but S-18.1 is not).
  # Unanchored grep -q "S-18.1" would match S-18.10 → exit 0 (WRONG — silently passes phantom).
  # Anchored grep (field-2 "| S-18.1 |") correctly finds no match → exit 1 (CORRECT).
  [ "$status" -eq 1 ] || {
    echo "FAIL (F-P6-003): skill exited ${status}, expected 1 (AntiFabricationFailed)." >&2
    echo "  Phantom ID 'S-18.1' is NOT in STORY-INDEX.md; S-18.10 IS." >&2
    echo "" >&2
    echo "  ROOT CAUSE (F-P6-003): anti-fabrication grep is unanchored with '.' wildcard:" >&2
    echo "    grep -q \"\$sid\" \"\$story_index_path\"" >&2
    echo "  'S-18.1' unanchored matches the substring 'S-18.1' in 'S-18.10'." >&2
    echo "  The anti-fabrication guard passes silently → phantom S-18.1 proceeds to wave-state." >&2
    echo "" >&2
    echo "  Fix: anchor on the Story-ID column with pipe-field boundaries:" >&2
    echo "    grep -qE \"\| *\${sid//./\\.} *\|\" (escape dots + pipe anchors)" >&2
    echo "" >&2
    echo "  Actual output: $output" >&2
    false
  }

  # Error output must mention AntiFabricationFailed or S-18.1
  echo "$output" | grep -qiE "(AntiFabricationFailed|S-18\.1|not found in STORY-INDEX)" || {
    echo "FAIL (F-P6-003): skill exited 1 but output doesn't identify anti-fabrication." >&2
    echo "Expected mention of AntiFabricationFailed, S-18.1, or 'not found in STORY-INDEX'" >&2
    echo "Actual output: $output" >&2
    false
  }
}

# ---------------------------------------------------------------------------
# test_BC_5_41_002_F_P6_004_arch_files_augmented_from_story_anchored_adrs
# F-P6-004 / BC-5.41.002 PC4 + PC5 / AC-016 / S-18.01 §Acceptance Criteria
# LOW: arch_files MUST include ADRs declared in each next-wave story's
# anchored_adrs: frontmatter array (BC-5.41.002 PC5 bullet 4: "Any ADR directly
# referenced by a story in stories[].spec_files"; AC-016: "derived from the
# stories' anchored_adrs and subsystem membership, not hardcoded").
#
# Current code: write-wave-state.sh arch_files block only emits the fixed
# minimum 3 files (ARCH-INDEX + ADR-026 + ADR-025). It does NOT read
# anchored_adrs: from story files. The doc comment claims derivation from
# anchored_adrs + subsystem membership (AC-016) but the code never reads
# the anchored_adrs: frontmatter array from story files.
#
# RED fixture: create a story file S-18.02 with anchored_adrs: [ADR-027] and
# plant ADR-027 at its canonical path. Assert the COMMITTED wave-state.yaml
# arch_files includes the ADR-027 path. The current impl never reads
# anchored_adrs: → ADR-027 absent from arch_files → test FAILS.
#
# After fix: skill reads anchored_adrs: from each wave story file and adds any
# ADR paths that exist on disk to arch_files.
# ---------------------------------------------------------------------------

@test "test_BC_5_41_002_F_P6_004_arch_files_augmented_from_story_anchored_adrs" {
  # Plant ADR-027 at its canonical path under ARTIFACTS_WT/specs/architecture/decisions/
  local adr027_filename="ADR-027-factory-artifacts-path-discipline-no-double-nested-factory-worktree.md"
  local adr027_rel="specs/architecture/decisions/${adr027_filename}"
  echo "# ADR-027 path discipline" > "${ARTIFACTS_WT}/${adr027_rel}"

  # Rewrite S-18.02 story file to declare anchored_adrs: [ADR-027]
  # (the ADR slug, which the skill must resolve to the full filename)
  # In production story files, anchored_adrs: lists ADR slugs like "ADR-027"
  # which the skill must resolve to the full file path under specs/architecture/decisions/.
  cat > "$ARTIFACTS_WT/stories/S-18.02-validate-wave-handoff-completeness-wasm.md" << EOF
---
document_type: story
level: implementation
story_id: S-18.02
epic_id: "E-18"
version: "1.0"
title: "Validate wave handoff completeness WASM gate"
status: draft
behavioral_contracts:
  - BC-4.14.001
verification_properties:
  - VP-081
anchored_adrs:
  - ${adr027_filename}
---
# S-18.02 fixture with anchored_adrs
EOF

  _run_skill

  [ "$status" -eq 0 ] || {
    echo "FAIL (F-P6-004): skill exited ${status}, expected 0. Output: $output" >&2
    false
  }

  # Read the COMMITTED blob (VP-087 proof harness)
  git -C "$WORK" show factory-artifacts:wave-state.yaml >/dev/null 2>&1 || {
    echo "FAIL (F-P6-004): wave-state.yaml not in committed factory-artifacts tree" >&2
    false
  }

  local committed_content
  committed_content="$(git -C "$WORK" show factory-artifacts:wave-state.yaml)"

  # The committed arch_files must contain the ADR-027 path (derived from S-18.02's anchored_adrs)
  echo "$committed_content" | grep -q "${adr027_rel}" || {
    echo "FAIL (F-P6-004): arch_files in COMMITTED wave-state.yaml does not contain ADR-027 path." >&2
    echo "  Expected path in arch_files: ${adr027_rel}" >&2
    echo "" >&2
    echo "  ROOT CAUSE (F-P6-004): write-wave-state.sh arch_files block only emits the 3-file" >&2
    echo "  minimum set (ARCH-INDEX + ADR-026 + ADR-025). It does NOT read anchored_adrs: from" >&2
    echo "  story files. The doc comment claims derivation from anchored_adrs + subsystem" >&2
    echo "  membership (AC-016) but the code never reads the anchored_adrs: frontmatter array." >&2
    echo "" >&2
    echo "  BC-5.41.002 PC5 bullet 4: 'Any ADR directly referenced by a story in" >&2
    echo "  stories[].spec_files'. AC-016: 'derived from the stories anchored_adrs and" >&2
    echo "  subsystem membership, not hardcoded'." >&2
    echo "" >&2
    echo "  Story S-18.02 declares: anchored_adrs: [${adr027_filename}]" >&2
    echo "  ADR-027 planted at: ${ARTIFACTS_WT}/${adr027_rel}" >&2
    echo "" >&2
    echo "  Committed arch_files section:" >&2
    echo "$committed_content" | grep -A 10 "^arch_files:" >&2
    false
  }

  # The 3-file minimum set must still be present (regression guard)
  echo "$committed_content" | grep -q "specs/architecture/ARCH-INDEX.md" || {
    echo "FAIL (F-P6-004): arch_files minimum set missing ARCH-INDEX.md" >&2
    false
  }
}

# ---------------------------------------------------------------------------
# test_BC_5_41_001_F_P7_002_epic_complete_idempotent_rerun_exits_0_with_announcement
# F-P7-002 / BC-5.41.001 PC8 / EC-015 (idempotent re-invocation)
# MEDIUM: When the EPIC-COMPLETE path is invoked a SECOND time with an identical
# HANDOFF.md already committed (byte-identical re-stage), `git commit` exits
# non-zero ("nothing to commit"). Under `set -euo pipefail` the script aborts
# BEFORE the mandated AC-012 3-line EPIC-COMPLETE announcement.
#
# EC-015 (new): after staging, detect empty staged diff and treat as idempotent
# success — skip the commit but STILL emit the canonical AC-012 EPIC-COMPLETE
# 3-line announcement and exit 0.
#
# Bug reproduced by:
#   1. Run the skill once in EPIC-COMPLETE state — succeeds, commits.
#   2. Remove working-tree HANDOFF.md so the skill rewrites it (same content).
#   3. Run again — `git add` stages a byte-identical file, `git commit` finds
#      nothing to commit, exits non-zero, script aborts under pipefail.
#
# Red test: assert the SECOND invocation exits 0 AND emits all 3 canonical
# EPIC-COMPLETE lines. Must fail before the EC-015 guard (abort/non-zero exit
# with no announcement) and pass after.
# ---------------------------------------------------------------------------

@test "test_BC_5_41_001_F_P7_002_epic_complete_idempotent_rerun_exits_0_with_announcement" {
  # Set up EPIC-COMPLETE: all stories in terminal status
  _write_sprint_state_all_terminal
  _write_state_md "5"

  # First invocation — succeeds, commits HANDOFF.md with epic_status: complete
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
  [ "$status" -eq 0 ] || {
    echo "FAIL (F-P7-002 first run): skill exited ${status}, expected 0." >&2
    echo "Output: $output" >&2
    false
  }

  # Capture commit count after first run
  local after_first_count
  after_first_count="$(git -C "$WORK" rev-list --count factory-artifacts)"

  # Remove working-tree HANDOFF.md so the skill rewrites it on the second run.
  # The content produced will be byte-identical (same sprint-state, same STATE.md)
  # — so `git add HANDOFF.md` stages a no-change, and `git commit` finds
  # "nothing to commit" → exits non-zero → script aborts under pipefail
  # BEFORE emitting the EPIC-COMPLETE announcement (the bug).
  rm -f "$ARTIFACTS_WT/HANDOFF.md"

  # Second invocation — this is the idempotent re-run
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

  # Must exit 0 — idempotent re-invocation must succeed (EC-015)
  [ "$status" -eq 0 ] || {
    echo "FAIL (F-P7-002 second run): skill exited ${status} on idempotent re-invocation, expected 0." >&2
    echo "" >&2
    echo "BUG (F-P7-002): EPIC-COMPLETE path stages a byte-identical HANDOFF.md (no change)." >&2
    echo "  'git commit' exits non-zero (nothing to commit) — under set -euo pipefail the" >&2
    echo "  script aborts BEFORE the mandatory AC-012 EPIC-COMPLETE announcement is emitted." >&2
    echo "  EC-015 fix: detect empty staged diff and treat as idempotent success — skip the" >&2
    echo "  commit but still emit the canonical 3-line announcement and exit 0." >&2
    echo "" >&2
    echo "Actual output: $output" >&2
    false
  }

  # Must emit ALL THREE canonical EPIC-COMPLETE lines (AC-012 / BC-5.41.001 PC8)
  # Line 1:
  echo "$output" | grep -qF \
    "EPIC-COMPLETE: All stories in sprint-state.yaml have reached terminal status." || {
    echo "FAIL (F-P7-002): canonical EPIC-COMPLETE line 1 missing on idempotent re-run." >&2
    echo "Expected: 'EPIC-COMPLETE: All stories in sprint-state.yaml have reached terminal status.'" >&2
    echo "Actual output: $output" >&2
    false
  }

  # Line 2 (STATE.md current_cycle = "v1.0-feature-context-durability-E18"):
  echo "$output" | grep -qF \
    "Epic v1.0-feature-context-durability-E18 is complete. No wave-state.yaml written for next wave." || {
    echo "FAIL (F-P7-002): canonical EPIC-COMPLETE line 2 missing on idempotent re-run." >&2
    echo "Expected: 'Epic v1.0-feature-context-durability-E18 is complete. No wave-state.yaml written for next wave.'" >&2
    echo "Actual output: $output" >&2
    false
  }

  # Line 3:
  echo "$output" | grep -qF \
    "HANDOFF.md committed to factory-artifacts with epic_status: complete." || {
    echo "FAIL (F-P7-002): canonical EPIC-COMPLETE line 3 missing on idempotent re-run." >&2
    echo "Expected: 'HANDOFF.md committed to factory-artifacts with epic_status: complete.'" >&2
    echo "Actual output: $output" >&2
    false
  }

  # No new commit must be created (idempotent: nothing changed)
  local after_second_count
  after_second_count="$(git -C "$WORK" rev-list --count factory-artifacts)"
  [ "$after_second_count" -eq "$after_first_count" ] || {
    echo "FAIL (F-P7-002): idempotent re-run created a new commit (delta=$((after_second_count - after_first_count)))." >&2
    echo "EC-015: when staged diff is empty, skip the commit (no new commit must be created)." >&2
    false
  }
}

# ---------------------------------------------------------------------------
# test_BC_5_41_001_F_P7_003_derive_wave_id_from_sprint_state_ordinal
# F-P7-003 / BC-5.41.001 PC2 / AC-002
# MEDIUM: derive_wave_id($1=sprint_state_yaml, $2=state_md) ignores its $1 arg.
# The sprint-state-ordinal path — the PRIMARY path for product pipelines — is
# unimplemented. Only the STATE.md current_step fallback is coded.
#
# Behavior to implement (PRIMARY path):
#   When sprint-state.yaml is present and parseable with entries, compute current
#   wave number = 1 + (number of fully-terminal waves). A wave is fully-terminal
#   when ALL its stories are in terminal status (merged/withdrawn/cancelled).
#   The current wave number is the number of the first wave with any pending/draft
#   stories.
#
# Red test: sprint-state.yaml with wave-1 stories all terminal (merged) and wave-2
# stories pending/draft. STATE.md is absent (no pass-N). Assert derive_wave_id
# returns 2 from sprint-state ordinal, INDEPENDENT of STATE.md.
#
# With the current implementation (only STATE.md path implemented), absent STATE.md
# → NoWaveIdSubstrate exit 1 — never reaches the sprint-state ordinal path.
# After the fix: derive_wave_id reads the sprint-state FIRST and returns 2.
#
# Wave assignment in this fixture:
#   Wave 1: S-18.02 (merged), S-18.03 (cancelled) — both terminal → wave 1 complete
#   Wave 2: S-18.04 (pending), S-18.05 (draft)    — pending/draft → current wave = 2
# ---------------------------------------------------------------------------

@test "test_BC_5_41_001_F_P7_003_derive_wave_id_from_sprint_state_ordinal" {
  # Sprint-state: wave-1 stories terminal, wave-2 stories pending/draft.
  # The derive_wave_id sprint-state-ordinal path MUST return 2.
  # STATE.md is intentionally absent — forces the implementation to use the
  # sprint-state-ordinal path (PRIMARY) rather than the STATE.md fallback.
  #
  # Wave assignment:
  #   Wave 1: S-18.02 (merged), S-18.03 (cancelled) — both terminal → wave 1 complete
  #   Wave 2: S-18.04 (pending), S-18.05 (draft)    — pending/draft → current wave = 2
  cat > "$WORK/sprint-state.yaml" << 'EOF'
stories:
  - id: S-18.02
    status: merged
  - id: S-18.03
    status: cancelled
  - id: S-18.04
    status: pending
  - id: S-18.05
    status: draft
EOF

  # Add S-18.04 and S-18.05 to the fixture STORY-INDEX and story files so
  # the anti-fabrication check passes (these IDs must exist in STORY-INDEX).
  # Append E-18 rows for the wave-2 stories to the existing STORY-INDEX fixture.
  cat >> "$ARTIFACTS_WT/stories/STORY-INDEX.md" << 'EOF'
| S-18.04 | Wave-2 story A | E-18 | 5 | P1 | [S-18.02] | [] | pending | [] |
| S-18.05 | Wave-2 story B | E-18 | 5 | P1 | [S-18.03] | [] | draft | [] |
EOF

  # Create matching story files for S-18.04 and S-18.05
  cat > "$ARTIFACTS_WT/stories/S-18.04-wave-2-story-a.md" << 'EOF'
---
document_type: story
level: implementation
story_id: S-18.04
epic_id: "E-18"
version: "1.0"
title: "Wave-2 story A"
status: pending
behavioral_contracts:
  - BC-4.14.001
verification_properties:
  - VP-081
---
# S-18.04 fixture (F-P7-003)
EOF

  cat > "$ARTIFACTS_WT/stories/S-18.05-wave-2-story-b.md" << 'EOF'
---
document_type: story
level: implementation
story_id: S-18.05
epic_id: "E-18"
version: "1.0"
title: "Wave-2 story B"
status: draft
behavioral_contracts:
  - BC-4.14.001
verification_properties:
  - VP-081
---
# S-18.05 fixture (F-P7-003)
EOF

  # Remove STATE.md entirely — if the sprint-state-ordinal path is not implemented,
  # derive_wave_id falls through to the STATE.md branch which finds no file and
  # exits 1 with NoWaveIdSubstrate. After the fix, the sprint-state ordinal path
  # runs first and returns 2 before ever checking STATE.md.
  rm -f "$WORK/STATE.md"

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

  # Must exit 0 — wave-2 stories exist so this is a has-next-wave scenario
  [ "$status" -eq 0 ] || {
    echo "FAIL (F-P7-003): skill exited ${status}, expected 0." >&2
    echo "" >&2
    echo "BUG (F-P7-003): derive_wave_id ignores its sprint_state_yaml argument." >&2
    echo "  The sprint-state-ordinal path (PRIMARY for product pipelines) is unimplemented." >&2
    echo "  Current code only reads STATE.md current_step: 'pass-N'." >&2
    echo "  With STATE.md absent, it exits 1 with NoWaveIdSubstrate — never reaching the" >&2
    echo "  sprint-state-ordinal path that should return 2." >&2
    echo "" >&2
    echo "  Sprint-state fixture:" >&2
    echo "    S-18.02 merged, S-18.03 cancelled  (wave-1: all terminal)" >&2
    echo "    S-18.04 pending, S-18.05 draft      (wave-2: current)" >&2
    echo "  Expected wave_id: 2 (derived from sprint-state ordinal, no STATE.md needed)" >&2
    echo "" >&2
    echo "  Actual output: $output" >&2
    false
  }

  # Read the COMMITTED blob and assert wave_id = 2
  git -C "$WORK" show factory-artifacts:HANDOFF.md >/dev/null 2>&1 || {
    echo "FAIL (F-P7-003): HANDOFF.md not committed to factory-artifacts." >&2
    false
  }

  local committed_wave_id
  committed_wave_id="$(git -C "$WORK" show factory-artifacts:HANDOFF.md \
    | grep "^wave_id:" | awk '{print $2}')"

  # wave_id must be 2 — derived from sprint-state ordinal (1 terminal wave + 1 = 2)
  [ "$committed_wave_id" = "2" ] || {
    echo "FAIL (F-P7-003): committed HANDOFF.md wave_id='${committed_wave_id}', expected 2." >&2
    echo "" >&2
    echo "  The sprint-state-ordinal derivation: 1 fully-terminal wave (wave-1: S-18.02+S-18.03)" >&2
    echo "  + 1 = current wave 2. This must be derived from sprint-state.yaml WITHOUT STATE.md." >&2
    echo "" >&2
    echo "  If wave_id is some other value, the implementation used a different (incorrect)" >&2
    echo "  substrate. If the skill exited non-zero (handled above), the PRIMARY sprint-state" >&2
    echo "  ordinal path is simply not implemented." >&2
    false
  }
}

# ---------------------------------------------------------------------------
# test_BC_5_41_001_F_P8_001_no_partial_handoff_on_anti_fabrication_failure
# F-P8-001 / BC-5.41.001 PC4 / BLOCKER
# When next_wave_stories contains a phantom ID (not in STORY-INDEX.md), the
# anti-fabrication check in write_wave_state exits 1 with AntiFabricationFailed.
# BC-5.41.001 PC4 requires: "If any required field is absent or any anti-fabrication
# check fails, wave-gate blocks wave close … and does NOT write a partial HANDOFF.md."
#
# BUG: In the has-next-wave path, write_handoff writes HANDOFF.md to the working
# tree BEFORE write_wave_state runs the STORY-INDEX cross-check. A phantom ID
# makes write_wave_state exit 1 — but HANDOFF.md is already written, leaving a
# partial artifact and a dirty worktree.
#
# Red assertions (must fail before fix, pass after):
#   (a) exit status 1
#   (b) HANDOFF.md must NOT be present in working tree
#   (c) worktree must be clean (git status --porcelain shows no HANDOFF.md entry)
#
# Red proof: current code writes HANDOFF.md BEFORE write_wave_state runs.
# ---------------------------------------------------------------------------

@test "test_BC_5_41_001_F_P8_001_no_partial_handoff_on_anti_fabrication_failure" {
  # sprint-state.yaml with a phantom next-wave story ID (not in STORY-INDEX.md)
  cat > "$WORK/sprint-state.yaml" << 'EOF'
stories:
  - id: S-99.99
    status: pending
EOF

  # STORY-INDEX.md (set up in setup()) does NOT contain S-99.99
  # Confirm fixture sanity: STORY-INDEX.md must exist at the ADR-027 path
  [ -f "$ARTIFACTS_WT/stories/STORY-INDEX.md" ] || {
    echo "FIXTURE ERROR: STORY-INDEX.md missing at ${ARTIFACTS_WT}/stories/STORY-INDEX.md" >&2
    false
  }

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

  # (a) Must exit 1 — anti-fabrication fails on phantom S-99.99
  [ "$status" -eq 1 ] || {
    echo "FAIL (F-P8-001 a): skill exited ${status}, expected 1 (AntiFabricationFailed)." >&2
    echo "  Phantom ID S-99.99 is NOT in STORY-INDEX.md; skill must exit 1." >&2
    echo "  Actual output: $output" >&2
    false
  }

  # (b) HANDOFF.md must NOT be present — no partial artifact written before validation
  # BC-5.41.001 PC4: "does NOT write a partial HANDOFF.md"
  [ ! -f "$ARTIFACTS_WT/HANDOFF.md" ] || {
    echo "FAIL (F-P8-001 b): HANDOFF.md was written to working tree before anti-fabrication check." >&2
    echo "  BC-5.41.001 PC4 requires: no partial HANDOFF.md on anti-fabrication failure." >&2
    echo "" >&2
    echo "  ROOT CAUSE (F-P8-001): In the has-next-wave path, write_handoff runs BEFORE" >&2
    echo "  write_wave_state. write_wave_state exits 1 on AntiFabricationFailed — but" >&2
    echo "  HANDOFF.md is already written to the working tree." >&2
    echo "" >&2
    echo "  FIX: Perform next_wave_stories anti-fabrication validation in a pre-flight step" >&2
    echo "  in main() BEFORE calling write_handoff. All story IDs must resolve in STORY-INDEX.md" >&2
    echo "  before any file is written." >&2
    false
  }

  # (c) Worktree must be clean — no HANDOFF.md in git status output
  local wt_status
  wt_status="$(git -C "$ARTIFACTS_WT" status --porcelain 2>/dev/null | grep 'HANDOFF.md' || true)"
  [ -z "$wt_status" ] || {
    echo "FAIL (F-P8-001 c): worktree is dirty — HANDOFF.md appears in git status --porcelain:" >&2
    echo "  ${wt_status}" >&2
    echo "  The partial HANDOFF.md write must not occur before validation." >&2
    false
  }
}

# ---------------------------------------------------------------------------
# test_BC_5_41_001_F_P8_002_story_index_absent_hard_errors_on_nonempty_stories
# F-P8-002 / BC-5.41.001 PC3 / BC-5.41.002 PC2 precond 2 / MEDIUM
# When STORY-INDEX.md is absent and next_wave_stories is non-empty (has-next-wave),
# the per-story anti-fabrication cross-check is silently skipped because the
# guard is `if [ -f "$story_index_path" ]; then ... fi` with no `else`.
#
# BC-5.41.002 PC2 precondition 2 declares STORY-INDEX.md "current and accessible";
# its absence at wave-close with non-empty story IDs is a block condition.
# SOUL.md §4 silent-failure: every story ID passes when STORY-INDEX is absent.
#
# Red assertion: has-next-wave invocation with STORY-INDEX.md removed → must exit
# non-zero with a named error (StoryIndexMissing or AntiFabricationFailed).
#
# Red proof: current code silently skips the cross-check when the file is absent,
# so the skill exits 0 with fabrication unchecked.
# ---------------------------------------------------------------------------

@test "test_BC_5_41_001_F_P8_002_story_index_absent_hard_errors_on_nonempty_stories" {
  # Default sprint-state (pending + draft stories) set up by setup()
  # Remove STORY-INDEX.md to trigger the absent-file path
  rm -f "$ARTIFACTS_WT/stories/STORY-INDEX.md"

  # Confirm STORY-INDEX.md is truly absent (fixture sanity)
  [ ! -f "$ARTIFACTS_WT/stories/STORY-INDEX.md" ] || {
    echo "FIXTURE ERROR: STORY-INDEX.md still exists after removal" >&2
    false
  }

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

  # Must exit non-zero — absent STORY-INDEX.md with non-empty story IDs is a block condition
  # BC-5.41.002 PC2 precondition 2: STORY-INDEX.md must be "current and accessible"
  [ "$status" -ne 0 ] || {
    echo "FAIL (F-P8-002): skill exited 0 when STORY-INDEX.md is absent." >&2
    echo "" >&2
    echo "  BUG (F-P8-002): The anti-fabrication guard is guarded by 'if [ -f story_index_path ]'" >&2
    echo "  with NO else clause. When STORY-INDEX.md is absent, the entire anti-fabrication" >&2
    echo "  cross-check is silently skipped — every story ID passes unchecked." >&2
    echo "" >&2
    echo "  BC-5.41.002 PC2 precondition 2 declares STORY-INDEX.md 'current and accessible'." >&2
    echo "  Its absence at wave-close with non-empty next_wave_stories is a hard block condition." >&2
    echo "  SOUL.md §4: silent failure is forbidden." >&2
    echo "" >&2
    echo "  FIX: When story_pairs is non-empty AND story_index_path does NOT exist," >&2
    echo "  hard-error with StoryIndexMissing (or AntiFabricationFailed) and exit 1." >&2
    echo "  Guard on story_pairs non-empty: empty next-wave (EPIC-COMPLETE) needs no index." >&2
    echo "" >&2
    echo "  Actual output: $output" >&2
    false
  }

  # Error output must mention the named error or the cause
  echo "$output" | grep -qiE "(StoryIndexMissing|AntiFabricationFailed|STORY-INDEX|story.index)" || {
    echo "FAIL (F-P8-002): skill exited non-zero but output does not identify the cause." >&2
    echo "  Expected: mention of StoryIndexMissing, AntiFabricationFailed, or STORY-INDEX" >&2
    echo "  Actual output: $output" >&2
    false
  }
}

# ---------------------------------------------------------------------------
# test_BC_5_41_001_O_P8_001_derive_wave_id_file_order_fragility_fail_loud
# O-P8-001 / BC-5.41.001 PC2 (anti-fabrication / SOUL.md §4 fail-loud)
#
# derive_wave_id uses a leading-contiguous-terminal-run model that relies on the
# precondition: sprint-state.yaml entries are ordered by wave (wave-1 stories
# before wave-2 stories, etc.). When this precondition is violated — a terminal
# entry appears AFTER a pending/draft entry — the algorithm silently returns a
# WRONG ordinal instead of failing loud.
#
# Concrete fragility:
#   Correct order:  [S-1.01 merged, S-2.01 pending]  → ordinal 2  (correct)
#   Wrong order:    [S-2.01 pending, S-1.01 merged]  → ordinal 1  (WRONG — silent)
#
# The detectable violation signature (with only statuses + file order):
#   A terminal entry appears AFTER _found_next_wave=1, i.e., after we have already
#   seen at least one pending/draft entry. This is the interleaved pattern
#   [pending, terminal] which cannot be safely interpreted under the
#   file-order==wave-order precondition. It indicates the precondition is violated.
#
# Note: mixed terminal/non-terminal WITHIN the current wave is legal — the detectable
# pathology is a terminal entry appearing after a pending/draft entry, not merely
# having both terminal and pending entries coexist (which is normal same-wave mixing).
# This test uses the minimal distinguishing input: [pending, terminal] with nothing
# before the pending entry — so the terminal after pending is unambiguously post-boundary.
#
# Production-grade requirement (SOUL.md §4): MUST NOT silently return a wrong ordinal.
# Must exit non-zero with a NAMED error (WaveOrderUnverifiable or NonContiguousWaveState).
#
# Red proof: current derive_wave_id in parse-sprint-state.sh (lines 145-227) has no
# post-boundary terminal detection guard. For [S-2.01 pending, S-1.01 merged], it sets
# _found_next_wave=1 when it sees the pending entry, then when it sees the terminal entry
# _in_terminal_run stays 0 (because _found_next_wave=1), producing _completed_waves=0,
# _found_next_wave=1 → ordinal = 1. This is returned silently — no error emitted.
# ---------------------------------------------------------------------------

@test "test_BC_5_41_001_O_P8_001_derive_wave_id_file_order_fragility_fail_loud" {
  # Out-of-order sprint-state: pending entry appears BEFORE a terminal entry.
  # This is the minimal distinguishing fixture: one pending then one terminal,
  # with no terminal entries before the pending entry.
  # Under correct file-order semantics: terminal entries should precede pending entries
  # (completed wave comes before next wave). This arrangement is unambiguously wrong.
  #
  # Use fixture story IDs that ARE in STORY-INDEX.md (set up by setup()) so the
  # pre-flight anti-fabrication check passes. The test specifically targets the
  # wave_id derivation fragility (Step 1), not the anti-fabrication check (Step 2b).
  # S-18.02 (pending) then S-18.03 (merged/terminal) — both are in STORY-INDEX.md.
  cat > "$WORK/sprint-state.yaml" << 'EOF'
stories:
  - id: S-18.02
    status: pending
  - id: S-18.03
    status: merged
EOF

  # Remove STATE.md to force derive_wave_id to use the sprint-state PRIMARY path.
  # (If STATE.md fallback fires, the test would mask the fragility.)
  rm -f "$WORK/STATE.md"

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

  # Must exit non-zero (exit 1) — the post-boundary terminal entry is detectable
  # and must trigger a named hard error rather than a silent wrong ordinal.
  # BC-5.41.001 PC2 anti-fabrication: wave_id MUST be derived from a real substrate.
  # SOUL.md §4: silent failure that returns a wrong ordinal is forbidden.
  [ "$status" -ne 0 ] || {
    echo "FAIL (O-P8-001): skill exited 0 with out-of-order sprint-state [pending, terminal]." >&2
    echo "" >&2
    echo "  BUG (O-P8-001): derive_wave_id in parse-sprint-state.sh has no guard for the" >&2
    echo "  post-boundary terminal pattern. When a terminal entry appears after a pending/draft" >&2
    echo "  entry (i.e., _found_next_wave=1 when we see the terminal), the algorithm silently" >&2
    echo "  sets _in_terminal_run=0 and ignores that terminal — producing a WRONG ordinal." >&2
    echo "" >&2
    echo "  Correct order  [terminal, pending]: ordinal=2 (1 completed wave + 1)" >&2
    echo "  Wrong order    [pending, terminal]: ordinal=1 (0 completed waves + 1) — WRONG" >&2
    echo "" >&2
    echo "  The file-order==wave-order precondition is violated but no error is emitted." >&2
    echo "  Fix: detect terminal-after-pending and exit 1 with WaveOrderUnverifiable." >&2
    echo "" >&2
    echo "  Actual output: $output" >&2
    false
  }

  # Must emit a NAMED error identifying the ordering violation
  # Accepts: WaveOrderUnverifiable OR NonContiguousWaveState (either naming is acceptable)
  echo "$output" | grep -qiE "(WaveOrderUnverifiable|NonContiguousWaveState|wave.order|order.*unverif|unverif.*order|post.boundary|terminal.*after.*pending|pending.*before.*terminal)" || {
    echo "FAIL (O-P8-001): skill exited non-zero but error output does not identify the cause." >&2
    echo "  Expected: mention of WaveOrderUnverifiable, NonContiguousWaveState, or wave ordering" >&2
    echo "  A named error is required so the caller knows HOW to remediate (reorder sprint-state)." >&2
    echo "  Actual output: $output" >&2
    false
  }
}

# ---------------------------------------------------------------------------
# test_BC_5_41_001_O_P10_001_factory_lock_inline_scalar_in_committed_blob
# O-P10-001 / BC-5.41.001 PC3 (factory_lock_holder field)
#
# When STATE.md factory_lock is a non-null INLINE SCALAR (e.g., factory_lock: "holder-name"),
# the committed HANDOFF.md blob must carry factory_lock_holder: holder-name (not null).
#
# All current fixtures use factory_lock: null. This test exercises the inline scalar
# non-null code path in write-handoff.sh get_factory_lock_holder().
#
# Production-faithful STATE.md fixture: `factory_lock: "session-abc123"` (quoted scalar).
# Assert on the committed blob (POLICY 11 — not working-tree file).
# ---------------------------------------------------------------------------

@test "test_BC_5_41_001_O_P10_001_factory_lock_inline_scalar_in_committed_blob" {
  # Write STATE.md with a non-null inline-scalar factory_lock value
  cat > "$WORK/STATE.md" << 'EOF'
---
current_step: "pass-2"
current_cycle: "v1.0-feature-context-durability-E18"
factory_lock: "session-abc123"
---
# STATE
EOF

  _run_skill

  [ "$status" -eq 0 ] || {
    echo "FAIL (O-P10-001 inline): skill exited ${status}, expected 0. Output: $output" >&2
    false
  }

  # Assert via COMMITTED blob (POLICY 11 — not working-tree file)
  git -C "$WORK" show factory-artifacts:HANDOFF.md >/dev/null 2>&1 || {
    echo "FAIL (O-P10-001 inline): HANDOFF.md not committed to factory-artifacts" >&2
    false
  }

  local committed_content
  committed_content="$(git -C "$WORK" show factory-artifacts:HANDOFF.md)"

  # factory_lock_holder must be present
  echo "$committed_content" | grep -q "^factory_lock_holder:" || {
    echo "FAIL (O-P10-001 inline): factory_lock_holder field missing from committed HANDOFF.md" >&2
    false
  }

  # factory_lock_holder must equal the inline-scalar value (without quotes)
  local holder_val
  holder_val="$(echo "$committed_content" | grep "^factory_lock_holder:" | awk '{print $2}')"
  [ "$holder_val" = "session-abc123" ] || {
    echo "FAIL (O-P10-001 inline): factory_lock_holder in committed HANDOFF.md" >&2
    echo "  expected: 'session-abc123' (from STATE.md factory_lock: \"session-abc123\")" >&2
    echo "  got:      '${holder_val}'" >&2
    echo "" >&2
    echo "  BC-5.41.001 PC3: factory_lock_holder must reflect the real lock state from STATE.md." >&2
    echo "  Inline scalar form: factory_lock: \"holder-name\" → factory_lock_holder: holder-name" >&2
    false
  }
}

# ---------------------------------------------------------------------------
# test_BC_5_41_001_O_P10_001_factory_lock_block_form_in_committed_blob
# O-P10-001 / BC-5.41.001 PC3 (factory_lock_holder field)
#
# When STATE.md factory_lock is a non-null BLOCK FORM (factory_lock:\n  holder: x),
# the committed HANDOFF.md blob must carry factory_lock_holder: x (not null).
#
# This exercises the block-form awk parse path in write-handoff.sh.
# The block form uses two lines:
#   factory_lock:
#     holder: <holder-name>
#
# Bug found during O-P10-001 investigation: the original awk pattern used `\s`
# (non-POSIX in awk) which silently fails on macOS/BSD awk. The fixed pattern
# uses `[[:space:]]` (POSIX). This test REDs the old \s pattern and passes the fix.
#
# Production-faithful fixture: inline factory_lock: null on its own is the common
# case; the block form appears when the factory is locked with metadata.
# Assert on the committed blob (POLICY 11 — not working-tree file).
# ---------------------------------------------------------------------------

@test "test_BC_5_41_001_O_P10_001_factory_lock_block_form_in_committed_blob" {
  # Write STATE.md with a non-null BLOCK FORM factory_lock
  cat > "$WORK/STATE.md" << 'EOF'
---
current_step: "pass-2"
current_cycle: "v1.0-feature-context-durability-E18"
factory_lock:
  holder: block-holder-xyz
  acquired_at: "2026-06-18T00:00:00Z"
  reason: "active session"
---
# STATE
EOF

  _run_skill

  [ "$status" -eq 0 ] || {
    echo "FAIL (O-P10-001 block): skill exited ${status}, expected 0. Output: $output" >&2
    false
  }

  # Assert via COMMITTED blob (POLICY 11 — not working-tree file)
  git -C "$WORK" show factory-artifacts:HANDOFF.md >/dev/null 2>&1 || {
    echo "FAIL (O-P10-001 block): HANDOFF.md not committed to factory-artifacts" >&2
    false
  }

  local committed_content
  committed_content="$(git -C "$WORK" show factory-artifacts:HANDOFF.md)"

  # factory_lock_holder must be present
  echo "$committed_content" | grep -q "^factory_lock_holder:" || {
    echo "FAIL (O-P10-001 block): factory_lock_holder field missing from committed HANDOFF.md" >&2
    false
  }

  # factory_lock_holder must equal the block-form holder value (not null)
  local holder_val
  holder_val="$(echo "$committed_content" | grep "^factory_lock_holder:" | awk '{print $2}')"
  [ "$holder_val" = "block-holder-xyz" ] || {
    echo "FAIL (O-P10-001 block): factory_lock_holder in committed HANDOFF.md" >&2
    echo "  expected: 'block-holder-xyz' (from STATE.md block-form factory_lock.holder)" >&2
    echo "  got:      '${holder_val}'" >&2
    echo "" >&2
    echo "  BC-5.41.001 PC3: factory_lock_holder must reflect the real lock state from STATE.md." >&2
    echo "  Block form:" >&2
    echo "    factory_lock:"  >&2
    echo "      holder: block-holder-xyz" >&2
    echo "  → factory_lock_holder: block-holder-xyz  (not null)" >&2
    echo "" >&2
    echo "  ROOT CAUSE (O-P10-001): write-handoff.sh awk pattern used non-POSIX \\s which" >&2
    echo "  silently fails on macOS/BSD awk — the holder line is never matched, producing" >&2
    echo "  factory_lock_holder: null despite the lock being held." >&2
    echo "  Fix: awk pattern changed from /^\\s+holder:/ to /^[[:space:]]+holder:/ (POSIX)." >&2
    false
  }
}
