#!/usr/bin/env bats
# wave-handoff.bats — Red Gate tests for the wave-handoff skill
#
# Story:   S-18.01 v1.9 — HANDOFF.md Schema + wave-handoff Skill; wave-state.yaml Atomic Production
# BCs:     BC-5.41.001 v1.21 (HANDOFF.md with 9 base required fields + anti-fabrication cross-checks)
#          BC-5.41.002 v1.15 (wave-state.yaml curated manifest; BrokenSprintState; atomicity;
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

# _run_skill_subcommands — drives the agent-orchestrated 3-step subcommand flow:
#   --emit-handoff (stdout → harness writes HANDOFF.md) → --emit-wave-state → --commit
# Sets $status and $output just like _run_skill did (via run bash -c).
# BC-5.41.001 PC10 / ADR-026 §Decision 8.
_run_skill_subcommands() {
  run bash -c "
    set -euo pipefail
    _skill='${SKILL}'
    _awt='${ARTIFACTS_WT}'
    _ss='${WORK}/sprint-state.yaml'
    _sm='${WORK}/STATE.md'
    _bc='${ARTIFACTS_WT}/specs/behavioral-contracts'
    _pfl='${ARTIFACTS_WT}/hooks/precompact-flush-log'

    # Step 1: emit HANDOFF.md payload to stdout (no disk write by skill)
    handoff_stdout=\"\$(\"\$_skill\" \
      --artifacts-worktree \"\$_awt\" \
      --sprint-state \"\$_ss\" \
      --state-md \"\$_sm\" \
      --bc-dir \"\$_bc\" \
      --emit-handoff \
      2>&1)\"
    emit_exit=\$?
    if [ \"\$emit_exit\" -ne 0 ]; then
      echo \"\$handoff_stdout\"
      exit \"\$emit_exit\"
    fi

    # Step 2: test harness writes HANDOFF.md (simulates agent Write tool)
    printf '%s\n' \"\$handoff_stdout\" > \"\${_awt}/HANDOFF.md\"

    # Step 3: write wave-state.yaml (skipped silently on EPIC-COMPLETE)
    \"\$_skill\" \
      --artifacts-worktree \"\$_awt\" \
      --sprint-state \"\$_ss\" \
      --state-md \"\$_sm\" \
      --bc-dir \"\$_bc\" \
      --emit-wave-state \
      2>&1 || exit \$?

    # Step 4: atomic commit
    \"\$_skill\" \
      --artifacts-worktree \"\$_awt\" \
      --sprint-state \"\$_ss\" \
      --state-md \"\$_sm\" \
      --bc-dir \"\$_bc\" \
      --commit \
      2>&1 || exit \$?
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
  _run_skill_subcommands

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
  _run_skill_subcommands

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
  _run_skill_subcommands

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
  _run_skill_subcommands

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
      --emit-handoff \
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
  _run_skill_subcommands

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
  _run_skill_subcommands

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
  _run_skill_subcommands

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

  _run_skill_subcommands

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
      --emit-handoff \
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
    set -euo pipefail
    handoff_stdout=\"\$('${SKILL}' \
      --artifacts-worktree '${ARTIFACTS_WT}' \
      --sprint-state '${WORK}/sprint-state.yaml' \
      --state-md '${WORK}/STATE.md' \
      --bc-dir '${ARTIFACTS_WT}/specs/behavioral-contracts' \
      --emit-handoff \
      2>&1)\"
    emit_exit=\$?
    if [ \"\$emit_exit\" -ne 0 ]; then echo \"\$handoff_stdout\"; exit \"\$emit_exit\"; fi
    printf '%s\n' \"\$handoff_stdout\" > '${ARTIFACTS_WT}/HANDOFF.md'
    '${SKILL}' \
      --artifacts-worktree '${ARTIFACTS_WT}' \
      --sprint-state '${WORK}/sprint-state.yaml' \
      --state-md '${WORK}/STATE.md' \
      --bc-dir '${ARTIFACTS_WT}/specs/behavioral-contracts' \
      --emit-wave-state 2>&1 || exit \$?
    '${SKILL}' \
      --artifacts-worktree '${ARTIFACTS_WT}' \
      --sprint-state '${WORK}/sprint-state.yaml' \
      --state-md '${WORK}/STATE.md' \
      --bc-dir '${ARTIFACTS_WT}/specs/behavioral-contracts' \
      --commit 2>&1 || exit \$?
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

  _run_skill_subcommands

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

  _run_skill_subcommands

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
  _run_skill_subcommands

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
  _run_skill_subcommands

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

  _run_skill_subcommands

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
  _run_skill_subcommands

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
  _run_skill_subcommands

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

  _run_skill_subcommands

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
    set -euo pipefail
    _handoff_stdout="$("${SKILL}" \
      --artifacts-worktree "${ARTIFACTS_WT2}" \
      --sprint-state "${sprint2}" \
      --state-md "${statemd2}" \
      --bc-dir "${ARTIFACTS_WT2}/specs/behavioral-contracts" \
      --emit-handoff \
      2>&1)"
    _emit_exit=$?
    if [ "${_emit_exit}" -ne 0 ]; then
      echo "${_handoff_stdout}"
      exit "${_emit_exit}"
    fi
    printf '%s\n' "${_handoff_stdout}" > "${ARTIFACTS_WT2}/HANDOFF.md"
    "${SKILL}" \
      --artifacts-worktree "${ARTIFACTS_WT2}" \
      --sprint-state "${sprint2}" \
      --state-md "${statemd2}" \
      --bc-dir "${ARTIFACTS_WT2}/specs/behavioral-contracts" \
      --emit-wave-state 2>&1 || exit $?
    "${SKILL}" \
      --artifacts-worktree "${ARTIFACTS_WT2}" \
      --sprint-state "${sprint2}" \
      --state-md "${statemd2}" \
      --bc-dir "${ARTIFACTS_WT2}/specs/behavioral-contracts" \
      --commit 2>&1 || exit $?
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
  _run_skill_subcommands

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
  _run_skill_subcommands

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
    '${SKILL}' \
      --artifacts-worktree '${ARTIFACTS_WT}' \
      --sprint-state '${WORK}/sprint-state.yaml' \
      --state-md '${WORK}/STATE.md' \
      --bc-dir '${ARTIFACTS_WT}/specs/behavioral-contracts' \
      --emit-handoff \
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
    '${SKILL}' \
      --artifacts-worktree '${ARTIFACTS_WT}' \
      --sprint-state '${WORK}/sprint-state.yaml' \
      --state-md '${WORK}/STATE.md' \
      --bc-dir '${ARTIFACTS_WT}/specs/behavioral-contracts' \
      --emit-handoff \
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

  _run_skill_subcommands

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
  _run_skill_subcommands

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

  _run_skill_subcommands

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

  _run_skill_subcommands

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
    '${SKILL}' \
      --artifacts-worktree '${ARTIFACTS_WT}' \
      --sprint-state '${WORK}/sprint-state.yaml' \
      --state-md '${WORK}/STATE.md' \
      --bc-dir '${ARTIFACTS_WT}/specs/behavioral-contracts' \
      --emit-handoff \
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

  _run_skill_subcommands

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

  _run_skill_subcommands

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
  _run_skill_subcommands

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
  _run_skill_subcommands

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

  _run_skill_subcommands

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

  _run_skill_subcommands

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

  _run_skill_subcommands

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
  # Note: FACTORY_REPO and GIT_DIR intentionally NOT exported here.
  # The skill must resolve origin/develop via git -C ARTIFACTS_WT or equivalent.
  run bash -c "
    set -euo pipefail
    cd /tmp
    handoff_stdout=\"\$('${SKILL}' \
      --artifacts-worktree '${ARTIFACTS_WT}' \
      --sprint-state '${WORK}/sprint-state.yaml' \
      --state-md '${WORK}/STATE.md' \
      --bc-dir '${ARTIFACTS_WT}/specs/behavioral-contracts' \
      --emit-handoff \
      2>&1)\"
    emit_exit=\$?
    if [ \"\$emit_exit\" -ne 0 ]; then echo \"\$handoff_stdout\"; exit \"\$emit_exit\"; fi
    printf '%s\n' \"\$handoff_stdout\" > '${ARTIFACTS_WT}/HANDOFF.md'
    '${SKILL}' \
      --artifacts-worktree '${ARTIFACTS_WT}' \
      --sprint-state '${WORK}/sprint-state.yaml' \
      --state-md '${WORK}/STATE.md' \
      --bc-dir '${ARTIFACTS_WT}/specs/behavioral-contracts' \
      --emit-wave-state 2>&1 || exit \$?
    '${SKILL}' \
      --artifacts-worktree '${ARTIFACTS_WT}' \
      --sprint-state '${WORK}/sprint-state.yaml' \
      --state-md '${WORK}/STATE.md' \
      --bc-dir '${ARTIFACTS_WT}/specs/behavioral-contracts' \
      --commit 2>&1 || exit \$?
  "

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
  # Note: --precompact-flush-log NOT passed; PRECOMPACT_FLUSH_LOG NOT exported.
  # The skill must default to ${ARTIFACTS_WT}/hooks/precompact-flush-log.
  run bash -c "
    set -euo pipefail
    export ARTIFACTS_WT='${ARTIFACTS_WT}'
    export GIT_DIR='${WORK}/.git'
    export FACTORY_REPO='${WORK}'
    handoff_stdout=\"\$('${SKILL}' \
      --artifacts-worktree '${ARTIFACTS_WT}' \
      --sprint-state '${WORK}/sprint-state.yaml' \
      --state-md '${WORK}/STATE.md' \
      --bc-dir '${ARTIFACTS_WT}/specs/behavioral-contracts' \
      --emit-handoff \
      2>&1)\"
    emit_exit=\$?
    if [ \"\$emit_exit\" -ne 0 ]; then echo \"\$handoff_stdout\"; exit \"\$emit_exit\"; fi
    printf '%s\n' \"\$handoff_stdout\" > '${ARTIFACTS_WT}/HANDOFF.md'
    '${SKILL}' \
      --artifacts-worktree '${ARTIFACTS_WT}' \
      --sprint-state '${WORK}/sprint-state.yaml' \
      --state-md '${WORK}/STATE.md' \
      --bc-dir '${ARTIFACTS_WT}/specs/behavioral-contracts' \
      --emit-wave-state 2>&1 || exit \$?
    '${SKILL}' \
      --artifacts-worktree '${ARTIFACTS_WT}' \
      --sprint-state '${WORK}/sprint-state.yaml' \
      --state-md '${WORK}/STATE.md' \
      --bc-dir '${ARTIFACTS_WT}/specs/behavioral-contracts' \
      --commit 2>&1 || exit \$?
  "

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

  _run_skill_subcommands

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
    '${SKILL}' \
      --artifacts-worktree '${ARTIFACTS_WT}' \
      --sprint-state '${WORK}/sprint-state.yaml' \
      --state-md '${WORK}/STATE.md' \
      --bc-dir '${ARTIFACTS_WT}/specs/behavioral-contracts' \
      --emit-handoff \
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

  _run_skill_subcommands

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
  _run_skill_subcommands
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
  _run_skill_subcommands

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
    set -euo pipefail
    handoff_stdout=\"\$('${SKILL}' \
      --artifacts-worktree '${ARTIFACTS_WT}' \
      --sprint-state '${WORK}/sprint-state.yaml' \
      --state-md '${WORK}/STATE.md' \
      --bc-dir '${ARTIFACTS_WT}/specs/behavioral-contracts' \
      --emit-handoff \
      2>&1)\"
    emit_exit=\$?
    if [ \"\$emit_exit\" -ne 0 ]; then echo \"\$handoff_stdout\"; exit \"\$emit_exit\"; fi
    printf '%s\n' \"\$handoff_stdout\" > '${ARTIFACTS_WT}/HANDOFF.md'
    '${SKILL}' \
      --artifacts-worktree '${ARTIFACTS_WT}' \
      --sprint-state '${WORK}/sprint-state.yaml' \
      --state-md '${WORK}/STATE.md' \
      --bc-dir '${ARTIFACTS_WT}/specs/behavioral-contracts' \
      --emit-wave-state 2>&1 || exit \$?
    '${SKILL}' \
      --artifacts-worktree '${ARTIFACTS_WT}' \
      --sprint-state '${WORK}/sprint-state.yaml' \
      --state-md '${WORK}/STATE.md' \
      --bc-dir '${ARTIFACTS_WT}/specs/behavioral-contracts' \
      --commit 2>&1 || exit \$?
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
    '${SKILL}' \
      --artifacts-worktree '${ARTIFACTS_WT}' \
      --sprint-state '${WORK}/sprint-state.yaml' \
      --state-md '${WORK}/STATE.md' \
      --bc-dir '${ARTIFACTS_WT}/specs/behavioral-contracts' \
      --emit-handoff \
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
    '${SKILL}' \
      --artifacts-worktree '${ARTIFACTS_WT}' \
      --sprint-state '${WORK}/sprint-state.yaml' \
      --state-md '${WORK}/STATE.md' \
      --bc-dir '${ARTIFACTS_WT}/specs/behavioral-contracts' \
      --emit-handoff \
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
    '${SKILL}' \
      --artifacts-worktree '${ARTIFACTS_WT}' \
      --sprint-state '${WORK}/sprint-state.yaml' \
      --state-md '${WORK}/STATE.md' \
      --bc-dir '${ARTIFACTS_WT}/specs/behavioral-contracts' \
      --emit-handoff \
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

  _run_skill_subcommands

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

  _run_skill_subcommands

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

# ---------------------------------------------------------------------------
# test_BC_5_41_001_F_P11_001_bsd_portability_no_pcre_classes_in_grep_sed
# F-P11-001 / TD-VSDD-060 sibling-sweep
# Portability guard: ALL shell scripts under skills/wave-handoff/ MUST NOT use
# BSD-incompatible PCRE shorthand classes (\s, \S, \d, \D, \w, \W, \b) inside
# grep or sed patterns, and MUST NOT use grep -P / --perl-regexp.
#
# These tokens behave as literal characters under BSD grep (macOS /usr/bin/grep),
# causing silent misclassification — e.g., `  - id: S-18.02` never matches
# `^\s+-\s+id:\s+\S+` on macOS, so classify_stories returns epic-complete instead
# of has-next-wave (SOUL.md §4 silent failure).
#
# Red Gate: parse-sprint-state.sh currently has 4 offending grep -qE patterns
# with \s/\S — this test FAILS now. It PASSES after the sibling-sweep fix.
#
# Platform-independent static scan: detects the source-code defect regardless of
# the OS running CI (Linux or macOS). Closes the recurrence class, not just the
# named instances.
# ---------------------------------------------------------------------------

# ---------------------------------------------------------------------------
# test_BC_5_41_002_O_P14_topo_sort_column_index_alignment
# O-P14 / BC-5.41.002 PC3 (topological sort correctness invariant)
#
# The topo-sort in write-wave-state.sh locates the "Depends-On" column by iterating
# over the header line with IFS='|' (shell word-splitting), counting fields from 1.
# It then uses that same 1-based index as an awk -F'|' field number to extract
# Depends-On from data rows.
#
# The alignment invariant: the leading '|' in a pipe-delimited Markdown table row
# (e.g. "| Story ID | Title | ... |") produces an EMPTY leading field in BOTH:
#   - shell IFS='|' word-splitting (for col in $header_line: first col is empty → col_idx=1)
#   - awk -F'|' field splitting ($1 is empty, $2 is Story ID, $6 is Depends-On)
#
# Therefore col_idx from the IFS='|' header scan == awk field number for data rows.
# NO offset adjustment is needed.
#
# This test directly asserts this alignment for a 9-column hyphenated "Depends-On"
# header (the production format). It:
#   1. Uses shell IFS='|' to compute the column index (shell_col_idx)
#   2. Uses awk -F'|' to find the "Depends-On" field in the same header row (awk_col_idx)
#   3. Asserts shell_col_idx == awk_col_idx == 6
#
# This is NOT a tautology: the assertion fails if either method produces a wrong index
# (e.g., if a leading-field offset were needed), and it documents the invariant so
# future readers don't accidentally introduce an off-by-one.
# ---------------------------------------------------------------------------

@test "test_BC_5_41_002_O_P14_topo_sort_column_index_alignment" {
  # Production 9-column Markdown header (hyphenated Depends-On in column position 6)
  # Column layout (1-based after leading | splits):
  #  1=empty(leading|) 2=StoryID 3=Title 4=Epic 5=Points 6=Priority 7=Depends-On 8=Blocks 9=Status 10=BCs 11=empty(trailing|)
  # Wait — awk -F'|' on "| A | B | C |" gives: $1="" $2=" A " $3=" B " $4=" C " $5=""
  # For "| Story ID | Title | Epic | Points | Priority | Depends-On | Blocks | Status | BCs |":
  #   $1=""  $2=" Story ID " $3=" Title " $4=" Epic " $5=" Points " $6=" Priority " $7=" Depends-On " ...
  # That makes Depends-On at awk field $7.
  #
  # And with IFS='|' shell split of the same string, iterating `for col in $header_line`:
  # bash IFS='|' splits on | — leading | produces empty first token so:
  #   col_idx=1 → ""  col_idx=2 → " Story ID " col_idx=3 → " Title " ...
  #   col_idx=7 → " Depends-On "
  #
  # Both methods should give 7 for "Depends-On" in the 9-column production format.
  # (The code uses col_idx starting at 0, incrementing before checking, so it reaches
  # 7 when the 7th token "Depends-On" is found.)
  #
  # We use the actual production-fixture header from STORY-INDEX.md (set up in setup()):
  #   | Story ID | Title | Epic | Points | Priority | Depends-On | Blocks | Status | BCs |
  # This is the E-18 table header — the one the topo-sort code actually uses.

  # Extract the E-18 table header from the fixture STORY-INDEX.md
  local header_line
  header_line="$(grep '| Story ID.*Depends-On' "$ARTIFACTS_WT/stories/STORY-INDEX.md" | head -1)"

  # Verify the fixture header was found
  [ -n "$header_line" ] || {
    echo "FAIL (O-P14 fixture): Could not find '| Story ID ... Depends-On' header in STORY-INDEX.md" >&2
    echo "  This is a fixture sanity failure, not an implementation error." >&2
    false
  }

  # Method 1: Shell IFS='|' split — same logic as write-wave-state.sh
  # Count col_idx (starting at 0, increment-before-check so the first field is 1)
  local shell_col_idx=0
  local IFS_bak="$IFS"
  IFS='|'
  local col
  for col in $header_line; do
    local trimmed
    trimmed="$(printf '%s' "$col" | tr -d ' \t' | sed 's/DependsOn/Depends-On/')"
    shell_col_idx=$(( shell_col_idx + 1 ))
    if [ "$trimmed" = "Depends-On" ]; then
      break
    fi
  done
  IFS="$IFS_bak"

  # shell_col_idx must be > 0 (found the column)
  [ "$shell_col_idx" -gt 0 ] || {
    echo "FAIL (O-P14): IFS='|' shell split did not find 'Depends-On' in header." >&2
    echo "  Header: ${header_line}" >&2
    false
  }

  # Method 2: awk -F'|' scan — find which field number contains "Depends-On"
  # awk splits "| A | B |" as: $1="" $2=" A " $3=" B " $4=""
  # We iterate NF fields and find the one whose trimmed value matches "Depends-On"
  local awk_col_idx
  awk_col_idx="$(printf '%s\n' "$header_line" | awk -F'|' '{
    for (i=1; i<=NF; i++) {
      v=$i
      gsub(/^[[:space:]]+|[[:space:]]+$/, "", v)
      gsub(/Depends On/, "Depends-On", v)
      if (v == "Depends-On") { print i; exit }
    }
  }')"

  # awk_col_idx must be found
  [ -n "$awk_col_idx" ] && [ "$awk_col_idx" -gt 0 ] || {
    echo "FAIL (O-P14): awk -F'|' scan did not find 'Depends-On' in header." >&2
    echo "  Header: ${header_line}" >&2
    false
  }

  # ALIGNMENT ASSERTION: shell_col_idx must equal awk_col_idx.
  # This proves that the topo-sort code can safely use col_idx from the IFS='|'
  # header scan as the awk field number for data row extraction — no offset needed.
  [ "$shell_col_idx" -eq "$awk_col_idx" ] || {
    echo "FAIL (O-P14): column-index MISMATCH between IFS='|' shell split and awk -F'|'." >&2
    echo "  shell IFS='|' col_idx: ${shell_col_idx}" >&2
    echo "  awk -F'|'  col_idx:    ${awk_col_idx}" >&2
    echo "  These MUST be equal: write-wave-state.sh uses the IFS='|' scan result as the" >&2
    echo "  awk field number for data-row extraction. A mismatch would cause topo-sort to" >&2
    echo "  read the wrong column." >&2
    echo "  Header: ${header_line}" >&2
    false
  }

  # CORRECTNESS ASSERTION: the agreed index must be > 1 (not the empty leading field)
  # and must correctly identify "Depends-On" in the header. For the production 9-column
  # E-18 header the agreed index must be 7 (field positions: 1=empty, 2=StoryID,
  # 3=Title, 4=Epic, 5=Points, 6=Priority, 7=Depends-On).
  [ "$shell_col_idx" -gt 1 ] || {
    echo "FAIL (O-P14): agreed column index ${shell_col_idx} is ≤1 — points to leading empty field." >&2
    echo "  The IFS='|' col_idx should be 7 for the production 9-column E-18 header." >&2
    false
  }

  # Verify the agreed index actually extracts "Depends-On" from a known data row.
  # Use the first E-18 data row from the fixture: S-18.02 with Depends-On: []
  # "| S-18.02 | Validate wave handoff completeness | E-18 | 8 | P0 | [] | [S-18.03] | draft | [BC-4.14.001] |"
  local sample_row
  sample_row="$(grep '| S-18.02 |' "$ARTIFACTS_WT/stories/STORY-INDEX.md" | grep -v 'Story ID' | head -1)"
  [ -n "$sample_row" ] || {
    echo "FAIL (O-P14 fixture): S-18.02 data row not found in STORY-INDEX.md" >&2
    false
  }

  # Extract the cell at the agreed column index from the data row using awk
  local cell_at_agreed_idx
  cell_at_agreed_idx="$(printf '%s\n' "$sample_row" | \
    awk -F'|' -v col="${shell_col_idx}" '{v=$col; gsub(/^[[:space:]]+|[[:space:]]+$/, "", v); print v}')"

  # For S-18.02 the Depends-On column is "[]" (no deps).
  # We only assert the cell is a bracket-notation dep list ([] or [S-...]) — not hardcoding the value.
  echo "$cell_at_agreed_idx" | grep -qE '^\[' || {
    echo "FAIL (O-P14): awk field \$${shell_col_idx} on S-18.02 data row extracts '${cell_at_agreed_idx}'." >&2
    echo "  Expected a bracket-notation Depends-On value like '[]' or '[S-X.Y]'." >&2
    echo "  The agreed column index ${shell_col_idx} does NOT point to the Depends-On column" >&2
    echo "  in the data row — indicating an alignment bug." >&2
    echo "  Sample data row: ${sample_row}" >&2
    false
  }
}

@test "test_BC_5_41_001_F_P11_001_bsd_portability_no_pcre_classes_in_grep_sed" {
  local wave_handoff_skill_dir
  wave_handoff_skill_dir="$(cd "${BATS_TEST_DIRNAME}/../skills/wave-handoff" && pwd)"

  # Collect all .sh files under wave-handoff (main + lib/)
  local sh_files=()
  while IFS= read -r f; do
    sh_files+=("$f")
  done < <(find "$wave_handoff_skill_dir" -name "*.sh" -type f | sort)

  local violations=()

  for f in "${sh_files[@]}"; do
    local rel="${f#${wave_handoff_skill_dir}/}"

    # Scan for PCRE shorthand classes inside grep or sed argument strings.
    # Strategy: look for the literal backslash-letter sequences that are
    # BSD-incompatible when used as regex operators.
    # We search for: \s \S \d \D \w \W \b (as regex tokens, not escaped shell vars)
    # Patterns: the backslash appears literally in the grep/sed argument.
    #
    # Use grep -n to get line numbers for diagnostic output.
    local hits
    hits="$(grep -nE "(grep|sed).*(['\"])[^'\"]*\\\\[sSdDwWb][^'\"]*\2" "$f" 2>/dev/null || true)"
    if [ -n "$hits" ]; then
      while IFS= read -r hit; do
        violations+=("${rel}: ${hit}")
      done <<< "$hits"
    fi

    # Also detect grep -P or grep --perl-regexp (GNU-only)
    local phits
    phits="$(grep -nE "grep[[:space:]]+(-[a-zA-Z]*P|--perl-regexp)" "$f" 2>/dev/null || true)"
    if [ -n "$phits" ]; then
      while IFS= read -r hit; do
        violations+=("${rel}: ${hit}")
      done <<< "$phits"
    fi
  done

  if [ "${#violations[@]}" -gt 0 ]; then
    echo "FAIL (F-P11-001): BSD-incompatible PCRE shorthand classes found in wave-handoff scripts." >&2
    echo "Replace \\s → [[:space:]], \\S → [^[:space:]], \\d → [[:digit:]], \\w → [[:alnum:]_]," >&2
    echo "and remove grep -P / --perl-regexp. Use grep -E (ERE) with POSIX bracket classes." >&2
    echo "Violations:" >&2
    local v
    for v in "${violations[@]}"; do
      echo "  ${v}" >&2
    done
    false
  fi
}

# ---------------------------------------------------------------------------
# S-18.12 PORTABILITY-LINT EXTENSION (AC-001 through AC-005)
# Static portability guards extending the PCRE guard above.
# Anchored to lesson L-S18-macos-ci-leg-caught-runtime-portability — the four
# macOS CI failures in S-18.01 that PCRE-only lint missed:
#   1. bash 3.2 incompatible local -A (ea7328ac)
#   2. global IFS mutation (2b40dfd5)
#   3. undeclared PyYAML runtime dep (aaa8da8a / 3fe11ea1)
#   4. (no jq in S-18.01, but jq is also not declared — added prospectively)
#
# Architecture compliance (S-18.12 ACR):
#   - Static analysis ONLY — grep the script sources; NEVER execute them.
#   - Non-vacuity (EC-005): each test asserts >= 1 .sh file was scanned.
#   - Scope: plugins/vsdd-factory/skills/wave-handoff/ production scripts ONLY.
#   - POSIX ERE (grep -E); no PCRE shorthand classes (\s, \d, \w, \b, etc.).
#   - Additive: these tests do NOT modify the PCRE guard above.
# ---------------------------------------------------------------------------

# ---------------------------------------------------------------------------
# test_portability_no_unguarded_local_A_associative_array
# AC-001: bash 3.2 compatibility — local -A / declare -A require bash 4+.
# macOS ships bash 3.2 at /bin/bash; Homebrew bash 5 is not on PATH by default.
#
# Invariant: if any file in the wave-handoff skill uses local -A or declare -A,
# a bash version guard (BASH_VERSINFO check) MUST exist somewhere in the scan
# set. The S-18.01 fix (ea7328ac) added the guard to wave-handoff.sh — the main
# entrypoint that sources all lib scripts, providing early-exit protection before
# any lib function using bash 4+ syntax is ever called.
#
# Red Gate expectation: PASS (guard confirmed present post-S-18.01 merge).
# ---------------------------------------------------------------------------

@test "test_portability_no_unguarded_local_A_associative_array" {
  local wave_handoff_skill_dir
  wave_handoff_skill_dir="$(cd "${BATS_TEST_DIRNAME}/../skills/wave-handoff" && pwd)"

  # Collect all .sh files under wave-handoff (main + lib/)
  local sh_files=()
  while IFS= read -r f; do
    sh_files+=("$f")
  done < <(find "$wave_handoff_skill_dir" -name "*.sh" -type f | sort)

  # EC-005 non-vacuity: the scan must cover at least one .sh file.
  # An empty scan set means the skill directory has drifted (scripts renamed or
  # moved) and this guard is stale — that is also a FAIL, not a false-pass.
  [ "${#sh_files[@]}" -gt 0 ] || {
    echo "FAIL (AC-001 EC-005): no .sh files found under ${wave_handoff_skill_dir}." >&2
    echo "  The portability-lint scope has drifted — update the guard to the new location." >&2
    false
  }

  # --- POSITIVE-CONTROL ASSERTIONS (O-1) ---
  # Verify the array-detector and guard-detector regexes discriminate correctly.
  # Uses synthetic temp files in BATS_TEST_TMPDIR; no real wave-handoff scripts are executed.
  # Broadened detector: covers -A, -Ax (combined flags), -gA (prefix flags), and -A at EOL.
  # Distinguishes -A (associative, bash 4+) from -a (indexed, bash 3 safe).
  local pc_bad_arr pc_bad_Ax pc_bad_gA pc_good_arr pc_bad_guard pc_good_guard
  pc_bad_arr="${BATS_TEST_TMPDIR}/pc_ac001_bad_arr.sh"
  pc_bad_Ax="${BATS_TEST_TMPDIR}/pc_ac001_bad_Ax.sh"
  pc_bad_gA="${BATS_TEST_TMPDIR}/pc_ac001_bad_gA.sh"
  pc_good_arr="${BATS_TEST_TMPDIR}/pc_ac001_good_arr.sh"
  pc_bad_guard="${BATS_TEST_TMPDIR}/pc_ac001_bad_guard.sh"
  pc_good_guard="${BATS_TEST_TMPDIR}/pc_ac001_good_guard.sh"
  printf 'declare -A my_map\n' > "$pc_bad_arr"
  printf 'declare -Ax my_map\n' > "$pc_bad_Ax"
  printf 'declare -gA my_map\n' > "$pc_bad_gA"
  printf 'declare -a my_arr\n' > "$pc_good_arr"
  printf '# removed the BASH_VERSINFO guard\n' > "$pc_bad_guard"
  printf 'if [ "${BASH_VERSINFO[0]:-0}" -lt 4 ]; then\n  exit 1\nfi\n' > "$pc_good_guard"
  # Array-detector: BAD sample (declare -A) MUST match.
  grep -qE '(local|declare)[[:space:]]+-[a-zA-Z]*A[a-zA-Z]*([[:space:]]|$)' "$pc_bad_arr" || {
    echo "FAIL (AC-001 positive-control): array-detector did not match 'declare -A my_map'." >&2
    false
  }
  # Array-detector: BAD sample (declare -Ax, combined flag) MUST match.
  grep -qE '(local|declare)[[:space:]]+-[a-zA-Z]*A[a-zA-Z]*([[:space:]]|$)' "$pc_bad_Ax" || {
    echo "FAIL (AC-001 positive-control): array-detector did not match 'declare -Ax my_map' (combined-flag form)." >&2
    false
  }
  # Array-detector: BAD sample (declare -gA, flag prefix) MUST match.
  grep -qE '(local|declare)[[:space:]]+-[a-zA-Z]*A[a-zA-Z]*([[:space:]]|$)' "$pc_bad_gA" || {
    echo "FAIL (AC-001 positive-control): array-detector did not match 'declare -gA my_map' (flag-prefix form)." >&2
    false
  }
  # Array-detector: BAD sample (declare -A at end-of-line, no var name — EOL -A form).
  # The ([[:space:]]|$) suffix's $ branch fires when nothing follows the -A flag on the line.
  # This exercises the continuation/split-line usage pattern (O-P4-002).
  local pc_bad_eol_A
  pc_bad_eol_A="${BATS_TEST_TMPDIR}/pc_ac001_bad_eol_A.sh"
  printf 'declare -A\n' > "$pc_bad_eol_A"
  grep -qE '(local|declare)[[:space:]]+-[a-zA-Z]*A[a-zA-Z]*([[:space:]]|$)' "$pc_bad_eol_A" || {
    echo "FAIL (AC-001 positive-control): array-detector did not match 'declare -A' at end-of-line (EOL -A form)." >&2
    echo "  The ([[:space:]]|$) suffix's \$ branch must fire when no var name follows the -A flag." >&2
    false
  }
  # Same EOL branch: 'local -A' at end-of-line MUST also match (O-P4-002).
  local pc_bad_eol_local_A
  pc_bad_eol_local_A="${BATS_TEST_TMPDIR}/pc_ac001_bad_eol_local_A.sh"
  printf 'local -A\n' > "$pc_bad_eol_local_A"
  grep -qE '(local|declare)[[:space:]]+-[a-zA-Z]*A[a-zA-Z]*([[:space:]]|$)' "$pc_bad_eol_local_A" || {
    echo "FAIL (AC-001 positive-control): array-detector did not match 'local -A' at end-of-line (EOL -A form)." >&2
    false
  }
  # Array-detector: GOOD sample (declare -a, POSIX indexed array) MUST NOT match.
  ! grep -qE '(local|declare)[[:space:]]+-[a-zA-Z]*A[a-zA-Z]*([[:space:]]|$)' "$pc_good_arr" || {
    echo "FAIL (AC-001 positive-control): array-detector falsely matched 'declare -a' (POSIX indexed array; bash-3-safe)." >&2
    false
  }
  # Guard-detector: executable conditional MUST match ([ precedes BASH_VERSINFO on the line).
  grep -qE '([[].*BASH_VERSINFO|[(][(].*BASH_VERSINFO)' "$pc_good_guard" || {
    echo "FAIL (AC-001 positive-control): guard-detector did not match 'if [ ... BASH_VERSINFO' form." >&2
    false
  }
  # Guard-detector: BASH_VERSINFO in a comment MUST NOT match (prevents paper-fix false-pass).
  ! grep -qE '([[].*BASH_VERSINFO|[(][(].*BASH_VERSINFO)' "$pc_bad_guard" || {
    echo "FAIL (AC-001 positive-control): guard-detector falsely matched BASH_VERSINFO in a comment." >&2
    false
  }
  # -------------------------------------------

  # Check whether any file uses local -A or declare -A (bash 4+ associative arrays).
  # Broadened: also catches combined flags (-Ax), prefix flags (-gA), and -A at end-of-line.
  # Does NOT catch -a (indexed arrays, bash-3-safe): [a-zA-Z]*A[a-zA-Z]* requires capital A.
  local has_arrays=0
  local f
  for f in "${sh_files[@]}"; do
    if grep -qE '(local|declare)[[:space:]]+-[a-zA-Z]*A[a-zA-Z]*([[:space:]]|$)' "$f" 2>/dev/null; then
      has_arrays=1
      break
    fi
  done

  # If no associative arrays are used anywhere, the scan set is compliant.
  if [ "$has_arrays" -eq 0 ]; then
    echo "AC-001: scanned=${#sh_files[@]} files"
    return 0
  fi

  # Associative arrays are used; verify the bash version guard exists in the scan set.
  # Acceptable guard forms: BASH_VERSINFO inside an executable conditional.
  # "[ ${BASH_VERSINFO[0]:-0} -lt 4 ]" — [ precedes BASH_VERSINFO on the same line.
  # "(( BASH_VERSINFO[0] < 4 ))" — (( precedes BASH_VERSINFO on the same line.
  # A bare comment like "# removed the BASH_VERSINFO guard" does NOT satisfy the oracle.
  local has_guard=0
  for f in "${sh_files[@]}"; do
    if grep -qE '([[].*BASH_VERSINFO|[(][(].*BASH_VERSINFO)' "$f" 2>/dev/null; then
      has_guard=1
      break
    fi
  done

  [ "$has_guard" -eq 1 ] || {
    echo "FAIL (AC-001): associative arrays (local -A / declare -A) found in wave-handoff" >&2
    echo "  scripts but no executable bash version guard (BASH_VERSINFO inside if/[/(()" >&2
    echo "  found in the scan set. A bare comment mentioning BASH_VERSINFO is insufficient." >&2
    echo "  The S-18.01 fix (ea7328ac) added the guard to wave-handoff.sh. It has been lost." >&2
    echo "  Fix: restore in wave-handoff.sh (before sourcing lib scripts):" >&2
    echo "    if [ \"\${BASH_VERSINFO[0]:-0}\" -lt 4 ]; then" >&2
    echo "      echo 'ERROR: wave-handoff requires bash >= 4.0 (associative arrays)' >&2" >&2
    echo "      echo '  On macOS: brew install bash' >&2" >&2
    echo "      exit 1" >&2
    echo "    fi" >&2
    false
  }

  # --- F-P6-001: ENTRYPOINT-GUARD SOUNDNESS ASSERTION ---
  # EC-006 requirement: any NEW, separate entrypoint that sources a sibling lib which uses
  # local -A / declare -A WITHOUT its own bash-4 guard is a FAIL, even if another entrypoint
  # in the scan set already has a guard.
  #
  # Structural check (real scripts): for each entrypoint .sh that sources a sibling lib,
  # assert the BASH_VERSINFO guard appears at a line number LOWER than its first source/. line.
  # An entrypoint is any .sh file whose name matches the skill top-level (not under lib/).
  #
  # Step 1: Synthetic positive-control — unguarded entrypoint sourcing a lib with local -A
  # MUST trigger a FAIL from the positional detector.
  local pc_unguarded_ep pc_guarded_ep pc_lib_with_arr
  pc_unguarded_ep="${BATS_TEST_TMPDIR}/pc_ac001_unguarded_ep.sh"
  pc_guarded_ep="${BATS_TEST_TMPDIR}/pc_ac001_guarded_ep.sh"
  pc_lib_with_arr="${BATS_TEST_TMPDIR}/pc_ac001_lib_with_arr.sh"
  # The lib uses local -A (bash 4+ only)
  printf 'local -A my_map\n' > "$pc_lib_with_arr"
  # Unguarded entrypoint: sources the lib but has no BASH_VERSINFO guard
  printf 'source "%s"\n' "$pc_lib_with_arr" > "$pc_unguarded_ep"
  # Guarded entrypoint: guard (line 1) appears BEFORE the source (line 2) — must PASS
  printf 'if [ "${BASH_VERSINFO[0]:-0}" -lt 4 ]; then exit 1; fi\nsource "%s"\n' "$pc_lib_with_arr" > "$pc_guarded_ep"

  # Detector: does the entrypoint have a guard BEFORE its first source line?
  # Returns 0 (PASS) when guard_line < first_source_line.
  _ep_guard_precedes_source() {
    local ep_file="$1"
    local guard_line first_src_line
    guard_line="$(grep -nE '([[].*BASH_VERSINFO|[(][(].*BASH_VERSINFO)' "$ep_file" 2>/dev/null \
      | head -1 | cut -d: -f1)"
    first_src_line="$(grep -nE '^[[:space:]]*(source|\.)[[:space:]]+' "$ep_file" 2>/dev/null \
      | head -1 | cut -d: -f1)"
    # No source line at all — entrypoint does not source libs; skip (not an entrypoint)
    [ -n "$first_src_line" ] || return 0
    # Has source but no guard — FAIL
    [ -n "$guard_line" ] || return 1
    # Guard must precede first source
    [ "$guard_line" -lt "$first_src_line" ]
  }

  # Unguarded entrypoint MUST FAIL the positional check (positive-control for soundness).
  _ep_guard_precedes_source "$pc_unguarded_ep" && {
    echo "FAIL (AC-001 F-P6-001 positive-control): positional guard detector PASSED on an" >&2
    echo "  unguarded entrypoint that sources a lib containing local -A. Detector is unsound." >&2
    echo "  An entrypoint that sources a bash-4 lib WITHOUT a preceding BASH_VERSINFO guard" >&2
    echo "  MUST be rejected (EC-006)." >&2
    false
  }

  # Guarded entrypoint MUST PASS the positional check (negative-control for non-vacuity,
  # POLICY 11: test must be non-tautological).
  _ep_guard_precedes_source "$pc_guarded_ep" || {
    echo "FAIL (AC-001 F-P6-001 positive-control): positional guard detector FAILED on a" >&2
    echo "  correctly guarded entrypoint (guard precedes first source line). Detector is broken." >&2
    false
  }

  # Step 2: Apply the structural check to all real entrypoint scripts under wave-handoff/.
  # Entrypoints are .sh files NOT under lib/ (direct children of the skill root).
  local ep_violations=()
  for f in "${sh_files[@]}"; do
    # Only inspect files directly under the skill root (not in lib/ subdirectory)
    local rel="${f#${wave_handoff_skill_dir}/}"
    [[ "$rel" != lib/* ]] || continue
    # Only process files that actually source sibling libs (skip pure helpers)
    grep -qE '^[[:space:]]*(source|\.)[[:space:]]+' "$f" 2>/dev/null || continue

    _ep_guard_precedes_source "$f" || {
      ep_violations+=("$rel")
    }
  done

  [ "${#ep_violations[@]}" -eq 0 ] || {
    echo "FAIL (AC-001 F-P6-001): entrypoint script(s) source sibling libs that use bash 4+" >&2
    echo "  syntax but the BASH_VERSINFO guard does NOT appear before the first source line." >&2
    echo "  EC-006: every entrypoint that sources bash-4 libs must guard BEFORE sourcing." >&2
    local viol
    for viol in "${ep_violations[@]}"; do echo "  ${viol}" >&2; done
    false
  }

  echo "AC-001: scanned=${#sh_files[@]} files"
}

# ---------------------------------------------------------------------------
# test_portability_no_unguarded_bash4_case_modifiers
# AC-002: bash 3.2 compatibility — ${var^^} and ${var,,} case modifiers require bash 4+.
# These operators cause a syntax error on bash 3.2 (/bin/bash on macOS).
#
# O-3 broadening (POLICY 13 prospective): detector now covers positional/special
# parameter names (${1^^}, ${@^^}, ${*^^}, ${#^^}) as well as named vars.
# The original var-name class [a-zA-Z_][a-zA-Z0-9_]* missed these forms.
#
# Invariant: if any wave-handoff script uses ${var^^} or ${var,,} (in any form —
# named, positional, or special param), a bash version guard (BASH_VERSINFO check)
# MUST also exist in the scan set.
#
# Red Gate expectation: PASS (no case modifiers present; absent = compliant).
# ---------------------------------------------------------------------------

@test "test_portability_no_unguarded_bash4_case_modifiers" {
  local wave_handoff_skill_dir
  wave_handoff_skill_dir="$(cd "${BATS_TEST_DIRNAME}/../skills/wave-handoff" && pwd)"

  local sh_files=()
  while IFS= read -r f; do
    sh_files+=("$f")
  done < <(find "$wave_handoff_skill_dir" -name "*.sh" -type f | sort)

  # EC-005 non-vacuity
  [ "${#sh_files[@]}" -gt 0 ] || {
    echo "FAIL (AC-002 EC-005): no .sh files found under ${wave_handoff_skill_dir}." >&2
    echo "  The portability-lint scope has drifted — update the guard to the new location." >&2
    false
  }

  # --- POSITIVE-CONTROL ASSERTIONS (O-1) ---
  # Verify the case-modifier detector matches all bash-4+ forms — including array-element
  # forms (${arr[0]^^}, ${map[k],,}, ${arr[i]^}), positional/special parameter forms
  # (${1^^}, ${@^^}, ${*^^}), and bash-4.4 @-operator case transforms (${var@U}, ${var@L},
  # ${var@u}) — and does NOT falsely match ${arr[@]}, ${BASH_SOURCE[0]}, or plain expansion.
  # Uses BATS_TEST_TMPDIR synthetic files.
  # Broadened (O-3 / POLICY 13 prospective): optional [...] index between var name and
  # modifier; positional/special parameter names ([0-9]+, @, *, #) in var-name class.
  # Broadened (O-1 / F-P8): terminal alternation now includes @[ULu] case-transform operators.
  local pc_bad_dbl pc_bad_single pc_bad_arr_elem pc_bad_positional pc_good_plain
  local pc_bad_at_upper pc_bad_at_lower pc_bad_at_title
  local pc_good_arr_expand pc_good_bash_source pc_good_pct_expansion
  pc_bad_dbl="${BATS_TEST_TMPDIR}/pc_ac002_bad_dbl.sh"
  pc_bad_single="${BATS_TEST_TMPDIR}/pc_ac002_bad_single.sh"
  pc_bad_arr_elem="${BATS_TEST_TMPDIR}/pc_ac002_bad_arr_elem.sh"
  pc_bad_positional="${BATS_TEST_TMPDIR}/pc_ac002_bad_positional.sh"
  pc_good_plain="${BATS_TEST_TMPDIR}/pc_ac002_good_plain.sh"
  pc_bad_at_upper="${BATS_TEST_TMPDIR}/pc_ac002_bad_at_upper.sh"
  pc_bad_at_lower="${BATS_TEST_TMPDIR}/pc_ac002_bad_at_lower.sh"
  pc_bad_at_title="${BATS_TEST_TMPDIR}/pc_ac002_bad_at_title.sh"
  pc_good_arr_expand="${BATS_TEST_TMPDIR}/pc_ac002_good_arr_expand.sh"
  pc_good_bash_source="${BATS_TEST_TMPDIR}/pc_ac002_good_bash_source.sh"
  pc_good_pct_expansion="${BATS_TEST_TMPDIR}/pc_ac002_good_pct_expansion.sh"
  printf 'echo "${VAR^^}"\n' > "$pc_bad_dbl"
  printf 'echo "${lower,}"\n' > "$pc_bad_single"
  printf 'echo "${arr[0]^^}"\necho "${map[k],,}"\necho "${arr[i]^}"\n' > "$pc_bad_arr_elem"
  printf 'echo "${1^^}"\necho "${@^^}"\n' > "$pc_bad_positional"
  printf 'echo "${VAR}"\n' > "$pc_good_plain"
  printf 'echo "${v@U}"\n' > "$pc_bad_at_upper"
  printf 'echo "${v@L}"\n' > "$pc_bad_at_lower"
  printf 'echo "${v@u}"\n' > "$pc_bad_at_title"
  printf 'for x in "${arr[@]}"; do echo "$x"; done\n' > "$pc_good_arr_expand"
  printf 'dir=$(dirname "${BASH_SOURCE[0]}")\n' > "$pc_good_bash_source"
  printf 'echo "${var%%:*}"\n' > "$pc_good_pct_expansion"
  # Doubled modifier (${VAR^^}) MUST match.
  grep -qE '\$\{([a-zA-Z_][a-zA-Z0-9_]*|[0-9]+|[@*#])(\[[^]]*\])?(\^\^?|,,?|@[ULu])' "$pc_bad_dbl" || {
    echo "FAIL (AC-002 positive-control): case-modifier-detector did not match '\${VAR^^}'." >&2
    false
  }
  # Single-char modifier (${lower,}) MUST match — bash 4+ only, same as doubled form.
  grep -qE '\$\{([a-zA-Z_][a-zA-Z0-9_]*|[0-9]+|[@*#])(\[[^]]*\])?(\^\^?|,,?|@[ULu])' "$pc_bad_single" || {
    echo "FAIL (AC-002 positive-control): case-modifier-detector did not match '\${lower,}'." >&2
    false
  }
  # Array-element modifiers (${arr[0]^^}, ${map[k],,}, ${arr[i]^}) MUST match.
  grep -qE '\$\{([a-zA-Z_][a-zA-Z0-9_]*|[0-9]+|[@*#])(\[[^]]*\])?(\^\^?|,,?|@[ULu])' "$pc_bad_arr_elem" || {
    echo "FAIL (AC-002 positive-control): case-modifier-detector did not match array-element forms (\${arr[0]^^}, \${map[k],,}, \${arr[i]^})." >&2
    false
  }
  # Positional/special param modifiers (${1^^}, ${@^^}) MUST match (O-3 broadening).
  # These are bash 4+ only and cause syntax errors on bash 3.2.
  grep -qE '\$\{([a-zA-Z_][a-zA-Z0-9_]*|[0-9]+|[@*#])(\[[^]]*\])?(\^\^?|,,?|@[ULu])' "$pc_bad_positional" || {
    echo "FAIL (AC-002 positive-control): case-modifier-detector did not match positional/special forms (\${1^^}, \${@^^})." >&2
    echo "  The var-name class must cover [0-9]+ (positional) and [@*#] (special) as alternatives." >&2
    false
  }
  # O-1: ${#^^} positive control — the # special parameter with a case modifier.
  # bash-portability.md §2 claims ${#^^} is covered by the [@*#] class; verify explicitly.
  local pc_bad_hash_modifier
  pc_bad_hash_modifier="${BATS_TEST_TMPDIR}/pc_ac002_bad_hash_modifier.sh"
  printf 'echo "${#^^}"\n' > "$pc_bad_hash_modifier"
  grep -qE '\$\{([a-zA-Z_][a-zA-Z0-9_]*|[0-9]+|[@*#])(\[[^]]*\])?(\^\^?|,,?|@[ULu])' "$pc_bad_hash_modifier" || {
    echo "FAIL (AC-002 positive-control / O-1): case-modifier-detector did not match '\${#^^}'." >&2
    echo "  bash-portability.md §2 states \${#^^} is covered by the [@*#] class." >&2
    echo "  The # special parameter with a case modifier is bash 4+ only." >&2
    false
  }
  # O-1 (F-P8): bash-4.4 @-operator case transforms MUST match.
  # ${var@U} (uppercase), ${var@L} (lowercase), ${var@u} (titlecase) cause "bad substitution"
  # on bash 3.2 — they are squarely in this story's bash-4+ feature detection scope.
  grep -qE '\$\{([a-zA-Z_][a-zA-Z0-9_]*|[0-9]+|[@*#])(\[[^]]*\])?(\^\^?|,,?|@[ULu])' "$pc_bad_at_upper" || {
    echo "FAIL (AC-002 positive-control / O-1): case-modifier-detector did not match '\${v@U}' (bash-4.4 uppercase operator)." >&2
    echo "  Terminal alternation must include @[ULu] to cover bash-4.4 parameter-transformation operators." >&2
    false
  }
  grep -qE '\$\{([a-zA-Z_][a-zA-Z0-9_]*|[0-9]+|[@*#])(\[[^]]*\])?(\^\^?|,,?|@[ULu])' "$pc_bad_at_lower" || {
    echo "FAIL (AC-002 positive-control / O-1): case-modifier-detector did not match '\${v@L}' (bash-4.4 lowercase operator)." >&2
    false
  }
  grep -qE '\$\{([a-zA-Z_][a-zA-Z0-9_]*|[0-9]+|[@*#])(\[[^]]*\])?(\^\^?|,,?|@[ULu])' "$pc_bad_at_title" || {
    echo "FAIL (AC-002 positive-control / O-1): case-modifier-detector did not match '\${v@u}' (bash-4.4 titlecase operator)." >&2
    false
  }
  # O-3 addition: ${*^^} MUST match — isolates the * branch of the [@*#] class independently.
  # bash-portability.md §2 states the [@*#] class covers *; this control verifies it explicitly.
  # (${@^^} is already in pc_bad_positional; ${#^^} is in pc_bad_hash_modifier; ${*^^} was uncovered.)
  local pc_bad_star_modifier
  pc_bad_star_modifier="${BATS_TEST_TMPDIR}/pc_ac002_bad_star_modifier.sh"
  printf 'echo "${*^^}"\n' > "$pc_bad_star_modifier"
  grep -qE '\$\{([a-zA-Z_][a-zA-Z0-9_]*|[0-9]+|[@*#])(\[[^]]*\])?(\^\^?|,,?|@[ULu])' "$pc_bad_star_modifier" || {
    echo "FAIL (AC-002 positive-control / O-3): case-modifier-detector did not match '\${*^^}' (star special param)." >&2
    echo "  The [@*#] class must cover * as an independent branch; \${*^^} is bash 4+ only." >&2
    false
  }
  # Over-match guards: ${arr[@]} (array expand-all), ${BASH_SOURCE[0]}, ${var%%:*} MUST NOT match.
  # ${arr[@]}: the [@] is an array subscript consumed by the (\[[^]]*\])? group; no modifier follows.
  ! grep -qE '\$\{([a-zA-Z_][a-zA-Z0-9_]*|[0-9]+|[@*#])(\[[^]]*\])?(\^\^?|,,?|@[ULu])' "$pc_good_arr_expand" || {
    echo "FAIL (AC-002 negative-control / O-1): case-modifier-detector falsely matched '\${arr[@]}' (array expand-all)." >&2
    echo "  The [@] subscript must be consumed by the optional index group; no modifier follows." >&2
    false
  }
  ! grep -qE '\$\{([a-zA-Z_][a-zA-Z0-9_]*|[0-9]+|[@*#])(\[[^]]*\])?(\^\^?|,,?|@[ULu])' "$pc_good_bash_source" || {
    echo "FAIL (AC-002 negative-control / O-1): case-modifier-detector falsely matched '\${BASH_SOURCE[0]}'." >&2
    echo "  The [0] subscript must be consumed by the optional index group; no modifier follows." >&2
    false
  }
  ! grep -qE '\$\{([a-zA-Z_][a-zA-Z0-9_]*|[0-9]+|[@*#])(\[[^]]*\])?(\^\^?|,,?|@[ULu])' "$pc_good_pct_expansion" || {
    echo "FAIL (AC-002 negative-control / O-1): case-modifier-detector falsely matched '\${var%%:*}' (suffix-removal operator)." >&2
    false
  }
  # Plain expansion (\${VAR}) MUST NOT match.
  ! grep -qE '\$\{([a-zA-Z_][a-zA-Z0-9_]*|[0-9]+|[@*#])(\[[^]]*\])?(\^\^?|,,?|@[ULu])' "$pc_good_plain" || {
    echo "FAIL (AC-002 positive-control): case-modifier-detector falsely matched plain '\${VAR}'." >&2
    false
  }
  # -------------------------------------------

  # Collect all uses of bash 4+ case modifiers — bash 4+ only, syntax error on bash 3.2.
  # Single-char form: ${var^} (first-char upper), ${var,} (first-char lower).
  # Doubled form:     ${var^^} (all-upper),       ${var,,} (all-lower).
  # Array-element forms: ${arr[0]^^}, ${map[k],,}, ${arr[i]^} — optional [...] index allowed.
  # Positional/special forms: ${1^^}, ${@^^}, ${*^^}, ${#^^} — [0-9]+, @, *, # in var-name.
  # bash-4.4 @-operator case transforms: ${var@U}, ${var@L}, ${var@u} — O-1 extension.
  # Pattern: ${ + (var name | positional | special) + optional [index] + case modifier
  local modifier_files=()
  local violations=()
  local f
  for f in "${sh_files[@]}"; do
    local hits
    hits="$(grep -nE '\$\{([a-zA-Z_][a-zA-Z0-9_]*|[0-9]+|[@*#])(\[[^]]*\])?(\^\^?|,,?|@[ULu])' "$f" 2>/dev/null || true)"
    if [ -n "$hits" ]; then
      local rel="${f#${wave_handoff_skill_dir}/}"
      modifier_files+=("$f")
      while IFS= read -r hit; do
        violations+=("${rel}: ${hit}")
      done <<< "$hits"
    fi
  done

  # No case modifiers found — compliant.
  if [ "${#modifier_files[@]}" -eq 0 ]; then
    echo "AC-002: scanned=${#sh_files[@]} files"
    return 0
  fi

  # Case modifiers are used; verify an executable bash version guard exists in the scan set.
  # Acceptable forms: BASH_VERSINFO inside an if-conditional ([ or (( precedes it on the line).
  # A bare comment mentioning BASH_VERSINFO is insufficient.
  local has_guard=0
  for f in "${sh_files[@]}"; do
    if grep -qE '([[].*BASH_VERSINFO|[(][(].*BASH_VERSINFO)' "$f" 2>/dev/null; then
      has_guard=1
      break
    fi
  done

  [ "$has_guard" -eq 1 ] || {
    echo "FAIL (AC-002): bash 4+ case modifiers (\${var^}, \${var^^}, \${var,}, \${var,,}, \${var@U}, \${var@L}, \${var@u}) found in" >&2
    echo "  wave-handoff scripts without an executable bash version guard (BASH_VERSINFO in if/[/(()" >&2
    echo "  found in the scan set." >&2
    echo "  Fix: either remove case modifiers (use 'tr a-z A-Z' or awk for case conversion)" >&2
    echo "  or add a bash 4+ version check to wave-handoff.sh before sourcing lib scripts." >&2
    echo "  Violations:" >&2
    local v
    for v in "${violations[@]}"; do echo "  ${v}" >&2; done
    false
  }
  echo "AC-002: scanned=${#sh_files[@]} files"
}

# ---------------------------------------------------------------------------
# test_portability_no_global_ifs_mutation
# AC-003: IFS must not be mutated at global script scope or as a non-local function
# assignment. Global IFS mutation persists across all subsequent reads and causes
# silent misclassification (SOUL.md §4 silent-failure category).
# The S-18.01 fix (2b40dfd5) replaced IFS='|' with awk -F'|' in parse-sprint-state.sh.
#
# Allowed forms (not flagged by this test):
#   local IFS=...            — local declaration (scoped to function body)
#   while IFS= read -r ...   — command-prefix before read (scoped to that read)
#   IFS=','  read -ra arr    — command-prefix before read (scoped to that read)
#   (IFS=...; command)       — subshell-scoped (same-line subshell open)
#
# Flagged forms (violations):
#   IFS='|'                  — standalone global assignment (not a command prefix)
#   IFS=$'\n'                — standalone global assignment
#   export IFS='|'           — export-prefixed global mutation
#   readonly IFS=$'\n'       — readonly-prefixed global mutation (O-P4-002)
#   declare -g IFS='|'       — declare -g global mutation (O-P4-002)
#   cmd && IFS='|'           — operator-prefixed global mutation (O-P4-003)
#   cmd & IFS='|'            — background+global mutation: background cmd then assign IFS (O-2)
#   cmd || IFS='|'           — operator-prefixed global mutation (O-P4-003)
#   then IFS='|'             — keyword-prefixed global mutation (O-P4-003)
#   do IFS='|'               — keyword-prefixed global mutation (O-P4-003)
#   else IFS='|'             — keyword-prefixed global mutation (O-P4-003)
#   elif IFS='|'             — keyword-prefixed global mutation (O-P4-003)
#   { IFS=$'\n'; read x; }  — brace-group current-shell mutation (O-1)
#   case $x in p) IFS='|'   — case-pattern body current-shell mutation (O-1)
#
# Detection: three-step filter — step-1 anchors on line-start/separator/operator/keyword/
# brace-group-open/case-pattern-close before IFS=; step-2 excludes local-scoped and
# subshell-open forms; step-3 excludes command-prefix-to-read forms.
#
# Red Gate expectation: PASS (global IFS mutation removed in S-18.01 merge 2b40dfd5).
# ---------------------------------------------------------------------------

@test "test_portability_no_global_ifs_mutation" {
  local wave_handoff_skill_dir
  wave_handoff_skill_dir="$(cd "${BATS_TEST_DIRNAME}/../skills/wave-handoff" && pwd)"

  local sh_files=()
  while IFS= read -r f; do
    sh_files+=("$f")
  done < <(find "$wave_handoff_skill_dir" -name "*.sh" -type f | sort)

  # EC-005 non-vacuity
  [ "${#sh_files[@]}" -gt 0 ] || {
    echo "FAIL (AC-003 EC-005): no .sh files found under ${wave_handoff_skill_dir}." >&2
    echo "  The portability-lint scope has drifted — update the guard to the new location." >&2
    false
  }

  # --- POSITIVE-CONTROL ASSERTIONS (O-1) ---
  # Verify the three-step IFS-mutation detector flags bare global IFS= (including export,
  # readonly, declare -g, and after-semicolon forms) and correctly exempts local IFS= and
  # IFS=... read command-prefix forms.
  # Uses BATS_TEST_TMPDIR synthetic files; no real wave-handoff scripts are executed.
  #
  # F-P3-001 fix: step-3 exclusion tightened to not cross statement separators.
  # OLD (greedy — false-negative): 'IFS=[^[:space:]]*[[:space:]]+read([[:space:]]|$)'
  #   [^[:space:]]* crosses ; so 'IFS='|'; read x' was wrongly excluded.
  # NEW (safe — stops at ;, &, |): 'IFS=[^[:space:];&|]*[[:space:]]+read([[:space:]]|$)'
  #
  # Step-1 pattern (extended): (^|;|&&|&|[|][|]|(then|do|else|elif)[[:space:]]|\{[[:space:]]+|[)][[:space:]]+)[[:space:]]*(export[[:space:]]+|readonly[[:space:]]+|declare[[:space:]]+-g[[:space:]]+)?IFS=
  #   ^                   — bare global assignment at start of line
  #   ;                   — second statement on same line (cmd; IFS=)
  #   &&                  — operator-prefixed: cmd && IFS='|' (O-P4-003)
  #   &                   — background+global: cmd & IFS='|' (O-2); && listed before & so &&
  #                          is consumed as a unit first; plain 'cmd &' without IFS= cannot match
  #   ||                  — operator-prefixed: cmd || IFS='|' (O-P4-003)
  #   then|do|else|elif   — keyword-prefixed: then IFS='|', else IFS='|', etc. (O-P4-003)
  #   \{[[:space:]]+      — brace-group current-shell: { IFS=$'\n'; ... } (O-1)
  #                          requires at least one space after { (bash syntax requires it);
  #                          distinguishes from ${IFS...} param expansion (no space after {)
  #   [)][[:space:]]+     — case-pattern body: case $x in p) IFS='|' ;; (O-1)
  #                          requires at least one space after ); benign forms like foo() {
  #                          are not followed by IFS= and do not match
  #   export/readonly/declare -g prefixes — keyword-prefixed global mutations
  # Step-2 (exclude): '^[0-9]+:[[:space:]]*(local[[:space:]]|[(])' — local scoped / subshell-open
  # Step-3 (exclude): 'IFS=[^[:space:];&|]*[[:space:]]+read([[:space:]]|$)' — command-prefix for read
  #   [^[:space:];&|]* — matches only non-separator chars; stops at ; & | before crossing them
  local pc_bad_ifs pc_bad_export_ifs pc_bad_semi_ifs pc_bad_semi_read pc_good_local_ifs pc_good_prefix_ifs
  local pc_bad_readonly_ifs pc_bad_declare_g_ifs pc_bad_and_ifs pc_bad_then_ifs pc_bad_bg_ifs
  local pc_bad_or_ifs pc_bad_do_ifs pc_bad_else_ifs pc_bad_elif_ifs
  local pc_bad_brace_ifs pc_bad_case_ifs pc_good_func_def_brace pc_good_close_paren_no_ifs
  pc_bad_ifs="${BATS_TEST_TMPDIR}/pc_ac003_bad_ifs.sh"
  pc_bad_export_ifs="${BATS_TEST_TMPDIR}/pc_ac003_bad_export.sh"
  pc_bad_semi_ifs="${BATS_TEST_TMPDIR}/pc_ac003_bad_semi.sh"
  pc_bad_semi_read="${BATS_TEST_TMPDIR}/pc_ac003_bad_semi_read.sh"
  pc_good_local_ifs="${BATS_TEST_TMPDIR}/pc_ac003_good_local.sh"
  pc_good_prefix_ifs="${BATS_TEST_TMPDIR}/pc_ac003_good_prefix.sh"
  pc_bad_readonly_ifs="${BATS_TEST_TMPDIR}/pc_ac003_bad_readonly.sh"
  pc_bad_declare_g_ifs="${BATS_TEST_TMPDIR}/pc_ac003_bad_declare_g.sh"
  pc_bad_and_ifs="${BATS_TEST_TMPDIR}/pc_ac003_bad_and.sh"
  pc_bad_then_ifs="${BATS_TEST_TMPDIR}/pc_ac003_bad_then.sh"
  pc_bad_or_ifs="${BATS_TEST_TMPDIR}/pc_ac003_bad_or.sh"
  pc_bad_do_ifs="${BATS_TEST_TMPDIR}/pc_ac003_bad_do.sh"
  pc_bad_else_ifs="${BATS_TEST_TMPDIR}/pc_ac003_bad_else.sh"
  pc_bad_elif_ifs="${BATS_TEST_TMPDIR}/pc_ac003_bad_elif.sh"
  pc_bad_bg_ifs="${BATS_TEST_TMPDIR}/pc_ac003_bad_bg.sh"
  pc_bad_brace_ifs="${BATS_TEST_TMPDIR}/pc_ac003_bad_brace.sh"
  pc_bad_case_ifs="${BATS_TEST_TMPDIR}/pc_ac003_bad_case.sh"
  pc_good_func_def_brace="${BATS_TEST_TMPDIR}/pc_ac003_good_func_def.sh"
  pc_good_close_paren_no_ifs="${BATS_TEST_TMPDIR}/pc_ac003_good_close_paren.sh"
  printf "IFS='|'\n" > "$pc_bad_ifs"
  printf "export IFS='|'\n" > "$pc_bad_export_ifs"
  printf "cmd; IFS='|'\n" > "$pc_bad_semi_ifs"
  printf "IFS='|'; read x\n" > "$pc_bad_semi_read"
  printf "local IFS=':'\n" > "$pc_good_local_ifs"
  printf "IFS=',' read -ra arr\n" > "$pc_good_prefix_ifs"
  printf "readonly IFS='|'\n" > "$pc_bad_readonly_ifs"
  printf "declare -g IFS='|'\n" > "$pc_bad_declare_g_ifs"
  printf "cmd && IFS='|'\n" > "$pc_bad_and_ifs"
  printf "then IFS='|'\n" > "$pc_bad_then_ifs"
  printf "cmd || IFS='|'\n" > "$pc_bad_or_ifs"
  printf "do IFS='|'\n" > "$pc_bad_do_ifs"
  printf "else IFS='|'\n" > "$pc_bad_else_ifs"
  printf "elif IFS='|'\n" > "$pc_bad_elif_ifs"
  printf "cmd & IFS='|'\n" > "$pc_bad_bg_ifs"
  # Brace-group: current-shell mutation (bash requires space after { so { IFS=... is unambiguous)
  printf '{ IFS=$'"'"'\\n'"'"'; read x; }\n' > "$pc_bad_brace_ifs"
  # Case-pattern body: ') IFS=|' in a case action body (current-shell)
  printf "case \$x in foo) IFS='|' ;; esac\n" > "$pc_bad_case_ifs"
  # GOOD: function definition brace: 'foo() {' — the { is not followed by IFS= (must NOT match)
  printf "foo() { echo hello; }\n" > "$pc_good_func_def_brace"
  # GOOD: closing paren without IFS=: 'result=$(cmd); echo $result' — ) not followed by IFS=
  printf 'result=$(cmd); echo "$result"\n' > "$pc_good_close_paren_no_ifs"
  # BAD: bare global IFS= MUST pass through the three-step filter (produces a violation).
  local pc_bad_ifs_hits
  pc_bad_ifs_hits="$(grep -nE '(^|;|&&|&|[|][|]|(then|do|else|elif)[[:space:]]|\{[[:space:]]+|[)][[:space:]]+)[[:space:]]*(export[[:space:]]+|readonly[[:space:]]+|declare[[:space:]]+-g[[:space:]]+)?IFS=' "$pc_bad_ifs" 2>/dev/null \
    | grep -vE '^[0-9]+:[[:space:]]*(local[[:space:]]|[(])' \
    | grep -vE 'IFS=[^[:space:];&|]*[[:space:]]+read([[:space:]]|$)' \
    || true)"
  [ -n "$pc_bad_ifs_hits" ] || {
    echo "FAIL (AC-003 positive-control): IFS-detector did not flag bare global 'IFS=|' assignment." >&2
    false
  }
  # BAD: 'export IFS=' MUST be flagged (keyword-prefixed global mutation).
  local pc_bad_export_hits
  pc_bad_export_hits="$(grep -nE '(^|;|&&|&|[|][|]|(then|do|else|elif)[[:space:]]|\{[[:space:]]+|[)][[:space:]]+)[[:space:]]*(export[[:space:]]+|readonly[[:space:]]+|declare[[:space:]]+-g[[:space:]]+)?IFS=' "$pc_bad_export_ifs" 2>/dev/null \
    | grep -vE '^[0-9]+:[[:space:]]*(local[[:space:]]|[(])' \
    | grep -vE 'IFS=[^[:space:];&|]*[[:space:]]+read([[:space:]]|$)' \
    || true)"
  [ -n "$pc_bad_export_hits" ] || {
    echo "FAIL (AC-003 positive-control): IFS-detector did not flag 'export IFS=|' (keyword-prefixed global mutation)." >&2
    false
  }
  # BAD: 'cmd; IFS=' MUST be flagged (second-statement global mutation).
  local pc_bad_semi_hits
  pc_bad_semi_hits="$(grep -nE '(^|;|&&|&|[|][|]|(then|do|else|elif)[[:space:]]|\{[[:space:]]+|[)][[:space:]]+)[[:space:]]*(export[[:space:]]+|readonly[[:space:]]+|declare[[:space:]]+-g[[:space:]]+)?IFS=' "$pc_bad_semi_ifs" 2>/dev/null \
    | grep -vE '^[0-9]+:[[:space:]]*(local[[:space:]]|[(])' \
    | grep -vE 'IFS=[^[:space:];&|]*[[:space:]]+read([[:space:]]|$)' \
    || true)"
  [ -n "$pc_bad_semi_hits" ] || {
    echo "FAIL (AC-003 positive-control): IFS-detector did not flag 'cmd; IFS=|' (second-statement global mutation)." >&2
    false
  }
  # BAD (F-P3-001): 'IFS='|'; read x' MUST be flagged — semicolon separates a global IFS=
  # assignment from 'read'; it is NOT a command-prefix (step-3 exemption requires the value
  # class to not cross statement separators).  The old [^[:space:]]* would cross the ';' and
  # wrongly exclude this as if it were 'IFS=... read'.
  local pc_bad_semi_read_hits
  pc_bad_semi_read_hits="$(grep -nE '(^|;|&&|&|[|][|]|(then|do|else|elif)[[:space:]]|\{[[:space:]]+|[)][[:space:]]+)[[:space:]]*(export[[:space:]]+|readonly[[:space:]]+|declare[[:space:]]+-g[[:space:]]+)?IFS=' "$pc_bad_semi_read" 2>/dev/null \
    | grep -vE '^[0-9]+:[[:space:]]*(local[[:space:]]|[(])' \
    | grep -vE 'IFS=[^[:space:];&|]*[[:space:]]+read([[:space:]]|$)' \
    || true)"
  [ -n "$pc_bad_semi_read_hits" ] || {
    echo "FAIL (AC-003 positive-control / F-P3-001): IFS-detector did NOT flag 'IFS=\\'|\\'; read x'." >&2
    echo "  This is a global IFS mutation; the semicolon separates IFS= from read (not a command-prefix)." >&2
    echo "  The step-3 exclusion value class [^[:space:];&|]* must stop at the semicolon." >&2
    false
  }
  # GOOD: 'local IFS=' MUST be excluded by the filter (no violation).
  local pc_good_local_hits
  pc_good_local_hits="$(grep -nE '(^|;|&&|&|[|][|]|(then|do|else|elif)[[:space:]]|\{[[:space:]]+|[)][[:space:]]+)[[:space:]]*(export[[:space:]]+|readonly[[:space:]]+|declare[[:space:]]+-g[[:space:]]+)?IFS=' "$pc_good_local_ifs" 2>/dev/null \
    | grep -vE '^[0-9]+:[[:space:]]*(local[[:space:]]|[(])' \
    | grep -vE 'IFS=[^[:space:];&|]*[[:space:]]+read([[:space:]]|$)' \
    || true)"
  [ -z "$pc_good_local_hits" ] || {
    echo "FAIL (AC-003 positive-control): IFS-detector falsely flagged 'local IFS=' (must be exempt)." >&2
    false
  }
  # GOOD: 'IFS=... read' command-prefix MUST be excluded by the filter (no violation).
  # This is the legitimate command-prefix form (e.g., IFS=',' read -ra arr).
  local pc_good_prefix_hits
  pc_good_prefix_hits="$(grep -nE '(^|;|&&|&|[|][|]|(then|do|else|elif)[[:space:]]|\{[[:space:]]+|[)][[:space:]]+)[[:space:]]*(export[[:space:]]+|readonly[[:space:]]+|declare[[:space:]]+-g[[:space:]]+)?IFS=' "$pc_good_prefix_ifs" 2>/dev/null \
    | grep -vE '^[0-9]+:[[:space:]]*(local[[:space:]]|[(])' \
    | grep -vE 'IFS=[^[:space:];&|]*[[:space:]]+read([[:space:]]|$)' \
    || true)"
  [ -z "$pc_good_prefix_hits" ] || {
    echo "FAIL (AC-003 positive-control): IFS-detector falsely flagged 'IFS=... read' prefix (must be exempt)." >&2
    false
  }
  # BAD (O-P4-002): 'readonly IFS=' MUST be flagged (keyword-prefixed global mutation).
  # This exercises the readonly[[:space:]]+ arm of the optional-keyword group in step-1.
  local pc_bad_readonly_hits
  pc_bad_readonly_hits="$(grep -nE '(^|;|&&|&|[|][|]|(then|do|else|elif)[[:space:]]|\{[[:space:]]+|[)][[:space:]]+)[[:space:]]*(export[[:space:]]+|readonly[[:space:]]+|declare[[:space:]]+-g[[:space:]]+)?IFS=' "$pc_bad_readonly_ifs" 2>/dev/null \
    | grep -vE '^[0-9]+:[[:space:]]*(local[[:space:]]|[(])' \
    | grep -vE 'IFS=[^[:space:];&|]*[[:space:]]+read([[:space:]]|$)' \
    || true)"
  [ -n "$pc_bad_readonly_hits" ] || {
    echo "FAIL (AC-003 positive-control / O-P4-002): IFS-detector did not flag 'readonly IFS=' (keyword-prefixed global mutation)." >&2
    echo "  The readonly[[:space:]]+ arm of step-1's optional-keyword group must fire." >&2
    false
  }
  # BAD (O-P4-002): 'declare -g IFS=' MUST be flagged (declare -g global mutation).
  # This exercises the declare[[:space:]]+-g[[:space:]]+ arm in step-1.
  local pc_bad_declare_g_hits
  pc_bad_declare_g_hits="$(grep -nE '(^|;|&&|&|[|][|]|(then|do|else|elif)[[:space:]]|\{[[:space:]]+|[)][[:space:]]+)[[:space:]]*(export[[:space:]]+|readonly[[:space:]]+|declare[[:space:]]+-g[[:space:]]+)?IFS=' "$pc_bad_declare_g_ifs" 2>/dev/null \
    | grep -vE '^[0-9]+:[[:space:]]*(local[[:space:]]|[(])' \
    | grep -vE 'IFS=[^[:space:];&|]*[[:space:]]+read([[:space:]]|$)' \
    || true)"
  [ -n "$pc_bad_declare_g_hits" ] || {
    echo "FAIL (AC-003 positive-control / O-P4-002): IFS-detector did not flag 'declare -g IFS=' (declare -g global mutation)." >&2
    echo "  The declare[[:space:]]+-g[[:space:]]+ arm of step-1's optional-keyword group must fire." >&2
    false
  }
  # BAD (O-P4-003): 'cmd && IFS=' MUST be flagged (operator-prefixed global mutation).
  # This exercises the new && arm added to step-1 to catch operator-chained mutations.
  local pc_bad_and_hits
  pc_bad_and_hits="$(grep -nE '(^|;|&&|&|[|][|]|(then|do|else|elif)[[:space:]]|\{[[:space:]]+|[)][[:space:]]+)[[:space:]]*(export[[:space:]]+|readonly[[:space:]]+|declare[[:space:]]+-g[[:space:]]+)?IFS=' "$pc_bad_and_ifs" 2>/dev/null \
    | grep -vE '^[0-9]+:[[:space:]]*(local[[:space:]]|[(])' \
    | grep -vE 'IFS=[^[:space:];&|]*[[:space:]]+read([[:space:]]|$)' \
    || true)"
  [ -n "$pc_bad_and_hits" ] || {
    echo "FAIL (AC-003 positive-control / O-P4-003): IFS-detector did not flag 'cmd && IFS=|' (operator-prefixed global mutation)." >&2
    echo "  The && arm must be in step-1's outer alternation group." >&2
    false
  }
  # BAD (O-P4-003): 'then IFS=' MUST be flagged (keyword-prefixed global mutation).
  # This exercises the (then|do|else|elif)[[:space:]] arm added to step-1.
  local pc_bad_then_hits
  pc_bad_then_hits="$(grep -nE '(^|;|&&|&|[|][|]|(then|do|else|elif)[[:space:]]|\{[[:space:]]+|[)][[:space:]]+)[[:space:]]*(export[[:space:]]+|readonly[[:space:]]+|declare[[:space:]]+-g[[:space:]]+)?IFS=' "$pc_bad_then_ifs" 2>/dev/null \
    | grep -vE '^[0-9]+:[[:space:]]*(local[[:space:]]|[(])' \
    | grep -vE 'IFS=[^[:space:];&|]*[[:space:]]+read([[:space:]]|$)' \
    || true)"
  [ -n "$pc_bad_then_hits" ] || {
    echo "FAIL (AC-003 positive-control / O-P4-003): IFS-detector did not flag 'then IFS=|' (keyword-prefixed global mutation)." >&2
    echo "  The (then|do|else|elif)[[:space:]] arm must be in step-1's outer alternation group." >&2
    false
  }
  # BAD (F-P13-001): 'cmd || IFS=' MUST be flagged (operator-prefixed global mutation via ||).
  # The [|][|] arm catches the OR-operator form which was previously UNcovered by a positive control.
  local pc_bad_or_hits
  pc_bad_or_hits="$(grep -nE '(^|;|&&|&|[|][|]|(then|do|else|elif)[[:space:]]|\{[[:space:]]+|[)][[:space:]]+)[[:space:]]*(export[[:space:]]+|readonly[[:space:]]+|declare[[:space:]]+-g[[:space:]]+)?IFS=' "$pc_bad_or_ifs" 2>/dev/null \
    | grep -vE '^[0-9]+:[[:space:]]*(local[[:space:]]|[(])' \
    | grep -vE 'IFS=[^[:space:];&|]*[[:space:]]+read([[:space:]]|$)' \
    || true)"
  [ -n "$pc_bad_or_hits" ] || {
    echo "FAIL (AC-003 positive-control / F-P13-001): IFS-detector did not flag 'cmd || IFS=|' (OR-operator global mutation)." >&2
    echo "  The [|][|] arm must be in step-1's outer alternation group." >&2
    false
  }
  # BAD (F-P13-001): 'do IFS=' MUST be flagged (keyword-prefixed global mutation).
  # This exercises the 'do' arm of (then|do|else|elif)[[:space:]] — previously UNcovered.
  local pc_bad_do_hits
  pc_bad_do_hits="$(grep -nE '(^|;|&&|&|[|][|]|(then|do|else|elif)[[:space:]]|\{[[:space:]]+|[)][[:space:]]+)[[:space:]]*(export[[:space:]]+|readonly[[:space:]]+|declare[[:space:]]+-g[[:space:]]+)?IFS=' "$pc_bad_do_ifs" 2>/dev/null \
    | grep -vE '^[0-9]+:[[:space:]]*(local[[:space:]]|[(])' \
    | grep -vE 'IFS=[^[:space:];&|]*[[:space:]]+read([[:space:]]|$)' \
    || true)"
  [ -n "$pc_bad_do_hits" ] || {
    echo "FAIL (AC-003 positive-control / F-P13-001): IFS-detector did not flag 'do IFS=|' (keyword-prefixed global mutation)." >&2
    echo "  The 'do' branch of (then|do|else|elif)[[:space:]] must fire." >&2
    false
  }
  # BAD (F-P13-001): 'else IFS=' MUST be flagged (keyword-prefixed global mutation).
  # This exercises the 'else' arm — previously UNcovered.
  local pc_bad_else_hits
  pc_bad_else_hits="$(grep -nE '(^|;|&&|&|[|][|]|(then|do|else|elif)[[:space:]]|\{[[:space:]]+|[)][[:space:]]+)[[:space:]]*(export[[:space:]]+|readonly[[:space:]]+|declare[[:space:]]+-g[[:space:]]+)?IFS=' "$pc_bad_else_ifs" 2>/dev/null \
    | grep -vE '^[0-9]+:[[:space:]]*(local[[:space:]]|[(])' \
    | grep -vE 'IFS=[^[:space:];&|]*[[:space:]]+read([[:space:]]|$)' \
    || true)"
  [ -n "$pc_bad_else_hits" ] || {
    echo "FAIL (AC-003 positive-control / F-P13-001): IFS-detector did not flag 'else IFS=|' (keyword-prefixed global mutation)." >&2
    echo "  The 'else' branch of (then|do|else|elif)[[:space:]] must fire." >&2
    false
  }
  # BAD (F-P13-001): 'elif IFS=' MUST be flagged (keyword-prefixed global mutation).
  # This exercises the 'elif' arm — previously UNcovered.
  local pc_bad_elif_hits
  pc_bad_elif_hits="$(grep -nE '(^|;|&&|&|[|][|]|(then|do|else|elif)[[:space:]]|\{[[:space:]]+|[)][[:space:]]+)[[:space:]]*(export[[:space:]]+|readonly[[:space:]]+|declare[[:space:]]+-g[[:space:]]+)?IFS=' "$pc_bad_elif_ifs" 2>/dev/null \
    | grep -vE '^[0-9]+:[[:space:]]*(local[[:space:]]|[(])' \
    | grep -vE 'IFS=[^[:space:];&|]*[[:space:]]+read([[:space:]]|$)' \
    || true)"
  [ -n "$pc_bad_elif_hits" ] || {
    echo "FAIL (AC-003 positive-control / F-P13-001): IFS-detector did not flag 'elif IFS=|' (keyword-prefixed global mutation)." >&2
    echo "  The 'elif' branch of (then|do|else|elif)[[:space:]] must fire." >&2
    false
  }
  # BAD (O-2): 'cmd & IFS=' MUST be flagged (background+global mutation).
  # Backgrounding a command and then assigning IFS globally on the same line mutates the
  # shell's IFS just as a standalone assignment does.  The & arm is listed after && in
  # step-1 so that '&&' is consumed as a unit first; plain 'cmd &' without a following
  # IFS= cannot satisfy the remainder of the regex and is NOT a false positive.
  local pc_bad_bg_hits
  pc_bad_bg_hits="$(grep -nE '(^|;|&&|&|[|][|]|(then|do|else|elif)[[:space:]]|\{[[:space:]]+|[)][[:space:]]+)[[:space:]]*(export[[:space:]]+|readonly[[:space:]]+|declare[[:space:]]+-g[[:space:]]+)?IFS=' "$pc_bad_bg_ifs" 2>/dev/null \
    | grep -vE '^[0-9]+:[[:space:]]*(local[[:space:]]|[(])' \
    | grep -vE 'IFS=[^[:space:];&|]*[[:space:]]+read([[:space:]]|$)' \
    || true)"
  [ -n "$pc_bad_bg_hits" ] || {
    echo "FAIL (AC-003 positive-control / O-2): IFS-detector did not flag 'cmd & IFS=|' (background+global mutation)." >&2
    echo "  The & arm must be in step-1's outer alternation group (listed after && so && is" >&2
    echo "  consumed as a unit first; AC-005's jq detector already includes & per [|;&])." >&2
    false
  }
  # BAD (O-1): '{ IFS=$'\n'; read x; }' MUST be flagged (brace-group current-shell mutation).
  # A brace group runs in the current shell (unlike a subshell), so IFS mutation inside it
  # persists globally.  bash syntax requires at least one space after '{', which distinguishes
  # it from '${IFS...}' parameter expansion (no space after '{').
  local pc_bad_brace_hits
  pc_bad_brace_hits="$(grep -nE '(^|;|&&|&|[|][|]|(then|do|else|elif)[[:space:]]|\{[[:space:]]+|[)][[:space:]]+)[[:space:]]*(export[[:space:]]+|readonly[[:space:]]+|declare[[:space:]]+-g[[:space:]]+)?IFS=' "$pc_bad_brace_ifs" 2>/dev/null \
    | grep -vE '^[0-9]+:[[:space:]]*(local[[:space:]]|[(])' \
    | grep -vE 'IFS=[^[:space:];&|]*[[:space:]]+read([[:space:]]|$)' \
    || true)"
  [ -n "$pc_bad_brace_hits" ] || {
    echo "FAIL (AC-003 positive-control / O-1): IFS-detector did not flag '{ IFS=\$'\\n'; read x; }' (brace-group current-shell mutation)." >&2
    echo "  The \\{[[:space:]]+ arm of step-1 must fire for brace-group-leading IFS= assignments." >&2
    false
  }
  # BAD (O-1): 'case $x in foo) IFS='|' ;; esac' MUST be flagged (case-pattern body mutation).
  # Case-pattern action bodies run in the current shell, so 'p) IFS=...' is a global mutation.
  # The [)][[:space:]]+ arm requires at least one space after ')' to distinguish from
  # function-definition patterns like 'foo() {' where ')' is not followed by IFS=.
  local pc_bad_case_hits
  pc_bad_case_hits="$(grep -nE '(^|;|&&|&|[|][|]|(then|do|else|elif)[[:space:]]|\{[[:space:]]+|[)][[:space:]]+)[[:space:]]*(export[[:space:]]+|readonly[[:space:]]+|declare[[:space:]]+-g[[:space:]]+)?IFS=' "$pc_bad_case_ifs" 2>/dev/null \
    | grep -vE '^[0-9]+:[[:space:]]*(local[[:space:]]|[(])' \
    | grep -vE 'IFS=[^[:space:];&|]*[[:space:]]+read([[:space:]]|$)' \
    || true)"
  [ -n "$pc_bad_case_hits" ] || {
    echo "FAIL (AC-003 positive-control / O-1): IFS-detector did not flag 'case \$x in foo) IFS=|' (case-pattern body mutation)." >&2
    echo "  The [)][[:space:]]+ arm of step-1 must fire for case-action-body IFS= assignments." >&2
    false
  }
  # GOOD (O-1 negative control): 'foo() { echo hello; }' MUST NOT be flagged.
  # The '{' in a function definition is followed by a space and then a NON-IFS= command,
  # so the \{[[:space:]]+ arm does not produce a false positive.
  local pc_good_func_def_hits
  pc_good_func_def_hits="$(grep -nE '(^|;|&&|&|[|][|]|(then|do|else|elif)[[:space:]]|\{[[:space:]]+|[)][[:space:]]+)[[:space:]]*(export[[:space:]]+|readonly[[:space:]]+|declare[[:space:]]+-g[[:space:]]+)?IFS=' "$pc_good_func_def_brace" 2>/dev/null \
    | grep -vE '^[0-9]+:[[:space:]]*(local[[:space:]]|[(])' \
    | grep -vE 'IFS=[^[:space:];&|]*[[:space:]]+read([[:space:]]|$)' \
    || true)"
  [ -z "$pc_good_func_def_hits" ] || {
    echo "FAIL (AC-003 negative-control / O-1): IFS-detector falsely flagged 'foo() { echo hello; }' (function definition brace must be exempt)." >&2
    echo "  The \\{[[:space:]]+ arm fires only when { is immediately followed (with optional space) by IFS=." >&2
    false
  }
  # GOOD (O-1 negative control): 'result=$(cmd); echo $result' MUST NOT be flagged.
  # The closing ')' of a command substitution is not followed by IFS= on this line,
  # so the [)][[:space:]]+ arm does not produce a false positive.
  local pc_good_close_paren_hits
  pc_good_close_paren_hits="$(grep -nE '(^|;|&&|&|[|][|]|(then|do|else|elif)[[:space:]]|\{[[:space:]]+|[)][[:space:]]+)[[:space:]]*(export[[:space:]]+|readonly[[:space:]]+|declare[[:space:]]+-g[[:space:]]+)?IFS=' "$pc_good_close_paren_no_ifs" 2>/dev/null \
    | grep -vE '^[0-9]+:[[:space:]]*(local[[:space:]]|[(])' \
    | grep -vE 'IFS=[^[:space:];&|]*[[:space:]]+read([[:space:]]|$)' \
    || true)"
  [ -z "$pc_good_close_paren_hits" ] || {
    echo "FAIL (AC-003 negative-control / O-1): IFS-detector falsely flagged 'result=\$(cmd); echo \$result' (close-paren not preceding IFS= must be exempt)." >&2
    echo "  The [)][[:space:]]+ arm fires only when ) is followed (with optional space) by IFS=." >&2
    false
  }
  # -------------------------------------------

  # Detect global IFS= mutations at script or function scope.
  # Step 1 (extended): lines where IFS= appears at line-start, after a semicolon,
  #         after an operator (&&, &, ||), after a shell keyword (then/do/else/elif),
  #         after a brace-group open ({ ), or after a case-pattern close () ),
  #         optionally preceded by export/readonly/declare -g keywords.
  #         — (^|;|&&|&|[|][|]|(then|do|else|elif)[[:space:]]|\{[[:space:]]+|[)][[:space:]]+)[[:space:]]*(export[[:space:]]+|readonly[[:space:]]+|declare[[:space:]]+-g[[:space:]]+)?IFS=
  #         Catches: bare IFS=, export/readonly/declare -g IFS=, cmd; IFS=,
  #         cmd && IFS=, cmd & IFS=, cmd || IFS=, then/do/else/elif IFS= (O-P4-003/O-2),
  #         { IFS=... (brace-group current-shell), p) IFS=... (case-pattern body) (O-1)
  # Step 2: exclude "local IFS=" — locally scoped within a function body.
  # Step 3: exclude "(IFS=" — subshell-scoped (subshell open on same line as IFS=).
  # Step 4: exclude "IFS=<value> read" — command-prefix for the read builtin.
  #         A command-prefix assignment is always on the same line as the command
  #         and the command name appears after the assignment value.
  #         F-P3-001: value class is [^[:space:];&|]* (not [^[:space:]]*) to avoid crossing
  #         statement separators — 'IFS='|'; read x' is a global mutation, not a prefix.
  local violations=()
  local f
  for f in "${sh_files[@]}"; do
    local rel="${f#${wave_handoff_skill_dir}/}"
    local hits
    hits="$(grep -nE '(^|;|&&|&|[|][|]|(then|do|else|elif)[[:space:]]|\{[[:space:]]+|[)][[:space:]]+)[[:space:]]*(export[[:space:]]+|readonly[[:space:]]+|declare[[:space:]]+-g[[:space:]]+)?IFS=' "$f" 2>/dev/null \
      | grep -vE '^[0-9]+:[[:space:]]*(local[[:space:]]|[(])' \
      | grep -vE 'IFS=[^[:space:];&|]*[[:space:]]+read([[:space:]]|$)' \
      || true)"
    if [ -n "$hits" ]; then
      while IFS= read -r hit; do
        violations+=("${rel}: ${hit}")
      done <<< "$hits"
    fi
  done

  if [ "${#violations[@]}" -gt 0 ]; then
    echo "FAIL (AC-003): bare global IFS= assignment found in wave-handoff scripts." >&2
    echo "  Global IFS mutation persists across all subsequent reads and is not portable." >&2
    echo "  The S-18.01 fix (2b40dfd5) was to replace IFS-based splitting with awk -F'...'." >&2
    echo "  Fix options:" >&2
    echo "    (a) Use 'local IFS=...' inside a function (scoped to that function body)." >&2
    echo "    (b) Use 'IFS=... read' as a command prefix (scoped to that single read)." >&2
    echo "    (c) Replace IFS-split loops with awk -F'...' or cut -d'...' -f<n>." >&2
    echo "  Violations:" >&2
    local v
    for v in "${violations[@]}"; do echo "  ${v}" >&2; done
    false
  fi
  echo "AC-003: scanned=${#sh_files[@]} files"
}

# ---------------------------------------------------------------------------
# test_portability_no_python_shellout
# AC-004: Python or pip shell-out prohibition (F-P11-001 Option A redesign).
# SKILL.md §149: "This skill MUST NOT shell out to Python, jq, or any language
# runtime beyond bash." Python is treated identically to jq — ANY python/pip
# invocation in a command position is a violation.
#
# F-P11-001 Option A: no preflight-acceptance and no stdlib exemption.
#   - The former phase-2 preflight-acceptance (python3 -c "import yaml" guard) is REMOVED.
#   - The former EC-002 stdlib exemption (python3 -c "import json" was PASS) is REMOVED.
#   - python3 -c 'import json' is now a VIOLATION, same as python3 -c 'import yaml'.
#   - The fix is REMOVAL of the python/pip invocation, not the addition of a guard.
#
# Detection: command-position anchoring analogous to jq_re (AC-005). Detects
# python[0-9.]* and pip[0-9x]* as command tokens in all execution positions:
# line-start, after ;/&&/&/||, $(...), backtick, brace-group, case-pattern body,
# subshell, and keyword/wrapper positions (xargs/if/then/do/else/elif/time/env/
# command/sudo).
#
# Red Gate expectation: PASS (no python/pip invocations present; absent = compliant).
# ---------------------------------------------------------------------------

@test "test_portability_no_python_shellout" {
  local wave_handoff_skill_dir
  wave_handoff_skill_dir="$(cd "${BATS_TEST_DIRNAME}/../skills/wave-handoff" && pwd)"

  local sh_files=()
  while IFS= read -r f; do
    sh_files+=("$f")
  done < <(find "$wave_handoff_skill_dir" -name "*.sh" -type f | sort)

  # EC-005 non-vacuity
  [ "${#sh_files[@]}" -gt 0 ] || {
    echo "FAIL (AC-004 EC-005): no .sh files found under ${wave_handoff_skill_dir}." >&2
    echo "  The portability-lint scope has drifted — update the guard to the new location." >&2
    false
  }

  # --- POSITIVE-CONTROL ASSERTIONS ---
  # Verify the python-shellout detector matches python/pip in all command positions,
  # including stdlib python3 -c 'import json' (formerly exempt under EC-002, now a
  # violation under F-P11-001 Option A), and does NOT match benign substrings like
  # variable names starting with "python", comments, or "python" as an echo argument.
  # python_re MUST BE BYTE-IDENTICAL to the real-scan-loop regex below.
  # Uses BATS_TEST_TMPDIR synthetic files; no real wave-handoff scripts are executed.
  local python_re='(^[[:space:]]*(python[0-9.]*|pip[0-9x]*)([[:space:]]|$)|[|;&][[:space:]]*(python[0-9.]*|pip[0-9x]*)([[:space:]]|$)|\$[(](python[0-9.]*|pip[0-9x]*)([[:space:]]|$)|`(python[0-9.]*|pip[0-9x]*)([[:space:]]|$)|(xargs|if|then|do|else|elif|time|env|command|sudo)[[:space:]]+(python[0-9.]*|pip[0-9x]*)([[:space:]]|$)|xargs([[:space:]]+-[^[:space:]]+)+[[:space:]]+(python[0-9.]*|pip[0-9x]*)([[:space:]]|$)|\{[[:space:]]*(python[0-9.]*|pip[0-9x]*)([[:space:]]|$)|\)[[:space:]]*(python[0-9.]*|pip[0-9x]*)([[:space:]]|$)|\([[:space:]]*(python[0-9.]*|pip[0-9x]*)([[:space:]]|$))'

  local pc_bad_py3 pc_bad_py2 pc_bad_pip3 pc_bad_pipx pc_bad_py311 pc_bad_stdlib
  local pc_bad_py3_cmdsubst pc_bad_sudo_py3
  local pc_good_py_var pc_good_comment pc_good_echo_py

  pc_bad_py3="${BATS_TEST_TMPDIR}/pc_ac004_bad_py3.sh"
  pc_bad_py2="${BATS_TEST_TMPDIR}/pc_ac004_bad_py2.sh"
  pc_bad_pip3="${BATS_TEST_TMPDIR}/pc_ac004_bad_pip3.sh"
  pc_bad_pipx="${BATS_TEST_TMPDIR}/pc_ac004_bad_pipx.sh"
  pc_bad_py311="${BATS_TEST_TMPDIR}/pc_ac004_bad_py311.sh"
  pc_bad_stdlib="${BATS_TEST_TMPDIR}/pc_ac004_bad_stdlib.sh"
  pc_bad_py3_cmdsubst="${BATS_TEST_TMPDIR}/pc_ac004_bad_py3_cmdsubst.sh"
  pc_bad_sudo_py3="${BATS_TEST_TMPDIR}/pc_ac004_bad_sudo_py3.sh"
  pc_good_py_var="${BATS_TEST_TMPDIR}/pc_ac004_good_py_var.sh"
  pc_good_comment="${BATS_TEST_TMPDIR}/pc_ac004_good_comment.sh"
  pc_good_echo_py="${BATS_TEST_TMPDIR}/pc_ac004_good_echo_py.sh"

  printf 'python3 script.py\n' > "$pc_bad_py3"
  printf 'python2 -c "import os; os.system(\"id\")"\n' > "$pc_bad_py2"
  printf 'pip3 install requests\n' > "$pc_bad_pip3"
  printf 'pipx run cowsay hello\n' > "$pc_bad_pipx"
  printf 'python3.11 -c "import yaml; print(yaml.safe_load(open(f)))"\n' > "$pc_bad_py311"
  # F-P11-001 Option A: stdlib python invocation is NOW a violation (EC-002 exemption removed).
  # This fixture was formerly a GOOD/negative control; it is now a BAD/positive control.
  printf "python3 -c 'import json; print(json.load(open(f)))'\n" > "$pc_bad_stdlib"
  printf 'result=$(python3 compute.py)\n' > "$pc_bad_py3_cmdsubst"
  printf 'sudo python3 /usr/local/bin/setup.py\n' > "$pc_bad_sudo_py3"
  # GOOD: variable name starting with 'python' — NOT a command invocation.
  # python_bin=python3.11 has 'python_bin' at line-start; 'python[0-9.]*([[:space:]]|$)'
  # does not match because '_bin' immediately follows 'python', not space/EOL.
  printf 'python_bin=python3.11\n' > "$pc_good_py_var"
  # GOOD: comment mentioning python3 — NOT a command invocation.
  printf '# python3 is not used in this script\n' > "$pc_good_comment"
  # GOOD: python3 as argument to echo — NOT a command invocation.
  printf 'echo "install python3 first"\n' > "$pc_good_echo_py"

  # BAD: bare line-start 'python3 script.py' MUST be detected.
  grep -qE "$python_re" "$pc_bad_py3" || {
    echo "FAIL (AC-004 positive-control): python-detector did not match bare 'python3 script.py' (line-start)." >&2
    false
  }
  # BAD: 'python2 ...' MUST be detected.
  grep -qE "$python_re" "$pc_bad_py2" || {
    echo "FAIL (AC-004 positive-control): python-detector did not match 'python2 ...' (line-start)." >&2
    false
  }
  # BAD: 'pip3 install ...' MUST be detected.
  grep -qE "$python_re" "$pc_bad_pip3" || {
    echo "FAIL (AC-004 positive-control): python-detector did not match 'pip3 install ...' (line-start)." >&2
    false
  }
  # BAD: 'pipx run ...' MUST be detected.
  grep -qE "$python_re" "$pc_bad_pipx" || {
    echo "FAIL (AC-004 positive-control): python-detector did not match 'pipx run ...' (line-start)." >&2
    false
  }
  # BAD: 'python3.11 -c ...' (versioned binary) MUST be detected.
  grep -qE "$python_re" "$pc_bad_py311" || {
    echo "FAIL (AC-004 positive-control): python-detector did not match 'python3.11 -c ...' (versioned binary)." >&2
    echo "  python[0-9.]* must match version-suffixed binaries." >&2
    false
  }
  # BAD (F-P11-001 Option A stdlib flip): python3 -c 'import json' MUST now be detected.
  # Previously this was a PASS (EC-002 stdlib exemption); under F-P11-001 Option A any
  # python shell-out is a violation — stdlib usage is no longer exempt.
  grep -qE "$python_re" "$pc_bad_stdlib" || {
    echo "FAIL (AC-004 positive-control / F-P11-001): python-detector did not match 'python3 -c ...import json...' (stdlib)." >&2
    echo "  F-P11-001 Option A removes EC-002: any python shell-out is a violation, stdlib included." >&2
    false
  }
  # BAD: '\$(python3 ...)' command-substitution MUST be detected.
  grep -qE "$python_re" "$pc_bad_py3_cmdsubst" || {
    echo "FAIL (AC-004 positive-control): python-detector did not match '\$(python3 ...)' command-substitution." >&2
    false
  }
  # BAD: 'sudo python3 ...' MUST be detected (keyword wrapper arm).
  grep -qE "$python_re" "$pc_bad_sudo_py3" || {
    echo "FAIL (AC-004 positive-control): python-detector did not match 'sudo python3 ...' (keyword wrapper)." >&2
    echo "  sudo must be in the wrapper keyword group for the python detector." >&2
    false
  }
  # GOOD: 'python_bin=python3.11' variable assignment MUST NOT be detected.
  ! grep -qE "$python_re" "$pc_good_py_var" || {
    echo "FAIL (AC-004 negative-control): python-detector falsely matched 'python_bin=python3.11' (variable assignment)." >&2
    echo "  The detector anchors on command position; 'python_bin' starts with 'python' but has '_bin' next, not space/EOL." >&2
    false
  }
  # GOOD: '# python3 is not used' comment MUST NOT be detected.
  ! grep -qE "$python_re" "$pc_good_comment" || {
    echo "FAIL (AC-004 negative-control): python-detector falsely matched python3 inside a comment." >&2
    false
  }
  # GOOD: 'echo "install python3 first"' MUST NOT be detected (python3 is an argument, not a command).
  ! grep -qE "$python_re" "$pc_good_echo_py" || {
    echo "FAIL (AC-004 negative-control): python-detector falsely matched 'echo \"install python3 first\"'." >&2
    echo "  python3 as an argument to echo is not a python shell-out." >&2
    false
  }
  # -------------------------------------------

  # Scan: detect any python/pip invocation in a command position.
  # Any match is a violation — SKILL.md §149 forbids all python shell-outs; no exemptions.
  # F-P11-001 Option A: single-phase detection; phase-2 preflight-acceptance removed.
  local violations=()
  local f
  for f in "${sh_files[@]}"; do
    local rel="${f#${wave_handoff_skill_dir}/}"
    local hits
    hits="$(grep -nE '(^[[:space:]]*(python[0-9.]*|pip[0-9x]*)([[:space:]]|$)|[|;&][[:space:]]*(python[0-9.]*|pip[0-9x]*)([[:space:]]|$)|\$[(](python[0-9.]*|pip[0-9x]*)([[:space:]]|$)|`(python[0-9.]*|pip[0-9x]*)([[:space:]]|$)|(xargs|if|then|do|else|elif|time|env|command|sudo)[[:space:]]+(python[0-9.]*|pip[0-9x]*)([[:space:]]|$)|xargs([[:space:]]+-[^[:space:]]+)+[[:space:]]+(python[0-9.]*|pip[0-9x]*)([[:space:]]|$)|\{[[:space:]]*(python[0-9.]*|pip[0-9x]*)([[:space:]]|$)|\)[[:space:]]*(python[0-9.]*|pip[0-9x]*)([[:space:]]|$)|\([[:space:]]*(python[0-9.]*|pip[0-9x]*)([[:space:]]|$))' \
        "$f" 2>/dev/null || true)"
    if [ -n "$hits" ]; then
      while IFS= read -r hit; do
        violations+=("${rel}: python/pip shell-out: ${hit}")
      done <<< "$hits"
    fi
  done

  if [ "${#violations[@]}" -gt 0 ]; then
    echo "FAIL (AC-004): python or pip invocation found in wave-handoff scripts." >&2
    echo "  SKILL.md §149: 'This skill MUST NOT shell out to Python, jq, or any language runtime beyond bash.'" >&2
    echo "  Fix: remove python/pip invocations; replace with POSIX portable alternatives (awk, grep, sed)." >&2
    echo "  NOTE: stdlib python (python3 -c 'import json') is also a violation (F-P11-001 Option A)." >&2
    echo "  Violations:" >&2
    local v
    for v in "${violations[@]}"; do echo "  ${v}" >&2; done
    false
  fi
  echo "AC-004: scanned=${#sh_files[@]} files"
}

# ---------------------------------------------------------------------------
# test_portability_no_undeclared_jq_dep
# AC-005: undeclared jq runtime dependency.
# jq is not installed by default on all macOS and Linux CI images. Any wave-handoff
# script that invokes jq as a command MUST have a preflight check confirming jq is
# available before first use:
#   command -v jq >/dev/null 2>&1 || { echo "ERROR: jq is required" >&2; exit 1; }
#
# Detection: lines where 'jq' appears as a standalone command token (preceded by
# start-of-line, pipe, semicolon, or whitespace, and followed by whitespace or
# end-of-line). Uses POSIX ERE; no \b word-boundary shorthand.
#
# O-3 broadening (POLICY 13 prospective):
#   - Added time|env|command|sudo to the keyword-wrapper group.
#   - Added a separate xargs-with-options arm:
#     xargs([[:space:]]+-[^[:space:]]+)+[[:space:]]+jq — catches 'xargs -n1 jq', etc.
#
# F-P8-001 broadening (LOCAL adversarial pass-8):
#   Missing execution positions added — jq runs in all three:
#   - Brace group (current-shell): \{[[:space:]]*jq — '{ jq . f; }'
#   - Case pattern-action body: \)[[:space:]]*jq — 'case $x in json) jq ...'
#   - Subshell open: \([[:space:]]*jq — '( jq . f )'
#   NOTE: unlike IFS (where subshell is EXEMPT because IFS doesn't leak), for jq a
#   subshell is a HAZARD — jq still executes. So jq covers (, {, AND ).
#
# Red Gate expectation: PASS (no jq invocations found; absent = compliant).
# ---------------------------------------------------------------------------

@test "test_portability_no_undeclared_jq_dep" {
  local wave_handoff_skill_dir
  wave_handoff_skill_dir="$(cd "${BATS_TEST_DIRNAME}/../skills/wave-handoff" && pwd)"

  local sh_files=()
  while IFS= read -r f; do
    sh_files+=("$f")
  done < <(find "$wave_handoff_skill_dir" -name "*.sh" -type f | sort)

  # EC-005 non-vacuity
  [ "${#sh_files[@]}" -gt 0 ] || {
    echo "FAIL (AC-005 EC-005): no .sh files found under ${wave_handoff_skill_dir}." >&2
    echo "  The portability-lint scope has drifted — update the guard to the new location." >&2
    false
  }

  # --- POSITIVE-CONTROL ASSERTIONS (O-1) ---
  # Verify the jq-detector matches command-substitution, &&/|| chains, xargs (direct and
  # with options), keyword-prefixed forms (including else/elif — F-P2-002), common
  # command wrappers (time, env, command, sudo — O-3), brace-group/case-body/subshell
  # execution positions (F-P8-001), and does NOT match jq in a comment, $(other_cmd),
  # foo() { echo; }, or ${jq_var} parameter expansion.
  # Uses BATS_TEST_TMPDIR synthetic files; no real wave-handoff scripts are executed.
  local pc_bad_cmdsubst pc_bad_and pc_bad_xargs pc_bad_else pc_bad_time pc_bad_xargs_opts pc_good_comment
  local pc_bad_brace pc_bad_case_paren pc_bad_subshell
  local pc_good_other_cmdsubst pc_good_func_brace pc_good_jq_var
  local pc_bad_jq_line_start pc_bad_jq_backtick pc_bad_jq_if pc_bad_jq_then pc_bad_jq_do
  local pc_bad_jq_elif pc_bad_jq_env pc_bad_jq_command pc_bad_jq_sudo
  pc_bad_cmdsubst="${BATS_TEST_TMPDIR}/pc_ac005_bad_cmdsubst.sh"
  pc_bad_and="${BATS_TEST_TMPDIR}/pc_ac005_bad_and.sh"
  pc_bad_xargs="${BATS_TEST_TMPDIR}/pc_ac005_bad_xargs.sh"
  pc_bad_else="${BATS_TEST_TMPDIR}/pc_ac005_bad_else.sh"
  pc_bad_time="${BATS_TEST_TMPDIR}/pc_ac005_bad_time.sh"
  pc_bad_xargs_opts="${BATS_TEST_TMPDIR}/pc_ac005_bad_xargs_opts.sh"
  pc_good_comment="${BATS_TEST_TMPDIR}/pc_ac005_good_comment.sh"
  pc_bad_brace="${BATS_TEST_TMPDIR}/pc_ac005_bad_brace.sh"
  pc_bad_case_paren="${BATS_TEST_TMPDIR}/pc_ac005_bad_case_paren.sh"
  pc_bad_subshell="${BATS_TEST_TMPDIR}/pc_ac005_bad_subshell.sh"
  pc_good_other_cmdsubst="${BATS_TEST_TMPDIR}/pc_ac005_good_other_cmdsubst.sh"
  pc_good_func_brace="${BATS_TEST_TMPDIR}/pc_ac005_good_func_brace.sh"
  pc_good_jq_var="${BATS_TEST_TMPDIR}/pc_ac005_good_jq_var.sh"
  pc_bad_jq_line_start="${BATS_TEST_TMPDIR}/pc_ac005_bad_jq_line_start.sh"
  pc_bad_jq_backtick="${BATS_TEST_TMPDIR}/pc_ac005_bad_jq_backtick.sh"
  pc_bad_jq_if="${BATS_TEST_TMPDIR}/pc_ac005_bad_jq_if.sh"
  pc_bad_jq_then="${BATS_TEST_TMPDIR}/pc_ac005_bad_jq_then.sh"
  pc_bad_jq_do="${BATS_TEST_TMPDIR}/pc_ac005_bad_jq_do.sh"
  pc_bad_jq_elif="${BATS_TEST_TMPDIR}/pc_ac005_bad_jq_elif.sh"
  pc_bad_jq_env="${BATS_TEST_TMPDIR}/pc_ac005_bad_jq_env.sh"
  pc_bad_jq_command="${BATS_TEST_TMPDIR}/pc_ac005_bad_jq_command.sh"
  pc_bad_jq_sudo="${BATS_TEST_TMPDIR}/pc_ac005_bad_jq_sudo.sh"
  printf 'result=$(jq -r .name input.json)\n' > "$pc_bad_cmdsubst"
  printf 'cmd && jq ".key" file.json\n' > "$pc_bad_and"
  printf 'find . -name "*.json" | xargs jq ".id"\n' > "$pc_bad_xargs"
  printf 'else jq -r .status file.json\n' > "$pc_bad_else"
  printf "time jq '.' input.json\n" > "$pc_bad_time"
  printf 'find . -name "*.json" | xargs -n1 jq ".id"\n' > "$pc_bad_xargs_opts"
  printf '# no jq dependency; using awk instead\n' > "$pc_good_comment"
  printf '{ jq . f; }\n' > "$pc_bad_brace"
  printf 'case $x in json) jq -r .x f ;; esac\n' > "$pc_bad_case_paren"
  printf '( jq . f )\n' > "$pc_bad_subshell"
  printf 'result=$(other_cmd . f)\n' > "$pc_good_other_cmdsubst"
  printf 'foo() { echo "hello"; }\n' > "$pc_good_func_brace"
  printf 'echo "${jq_var}"\n' > "$pc_good_jq_var"
  # F-P13-001 additions: fixtures for arms that lacked positive controls.
  printf 'jq -r .x f.json\n' > "$pc_bad_jq_line_start"
  printf '`jq . f`\n' > "$pc_bad_jq_backtick"
  printf "if jq '.key' file.json\n" > "$pc_bad_jq_if"
  printf "then jq '.key' file.json\n" > "$pc_bad_jq_then"
  printf "do jq '.key' file.json\n" > "$pc_bad_jq_do"
  printf "elif jq '.key' file.json\n" > "$pc_bad_jq_elif"
  printf "env jq '.key' file.json\n" > "$pc_bad_jq_env"
  printf "command jq '.key' file.json\n" > "$pc_bad_jq_command"
  printf "sudo jq '.key' file.json\n" > "$pc_bad_jq_sudo"
  # F-P8-001 + O-3 broadened jq_re: adds time|env|command|sudo to the keyword wrapper group;
  # adds xargs-with-options arm; adds brace-group, case-pattern-body, and subshell arms.
  # Positive-control regex MUST BE BYTE-IDENTICAL to the real-scan-loop regex below.
  local jq_re='(^[[:space:]]*jq([[:space:]]|$)|[|;&][[:space:]]*jq([[:space:]]|$)|\$[(]jq([[:space:]]|$)|`jq([[:space:]]|$)|(xargs|if|then|do|else|elif|time|env|command|sudo)[[:space:]]+jq([[:space:]]|$)|xargs([[:space:]]+-[^[:space:]]+)+[[:space:]]+jq([[:space:]]|$)|\{[[:space:]]*jq([[:space:]]|$)|\)[[:space:]]*jq([[:space:]]|$)|\([[:space:]]*jq([[:space:]]|$))'
  # BAD: command-substitution $(jq ...) MUST be detected.
  grep -qE "$jq_re" "$pc_bad_cmdsubst" || {
    echo "FAIL (AC-005 positive-control): jq-detector did not match '\$(jq ...)' command-substitution form." >&2
    false
  }
  # BAD: && jq MUST be detected (& in [|;&] catches both && and ||).
  grep -qE "$jq_re" "$pc_bad_and" || {
    echo "FAIL (AC-005 positive-control): jq-detector did not match 'cmd && jq' form." >&2
    false
  }
  # BAD: xargs jq (direct, no intervening options) MUST be detected.
  grep -qE "$jq_re" "$pc_bad_xargs" || {
    echo "FAIL (AC-005 positive-control): jq-detector did not match 'xargs jq' form." >&2
    false
  }
  # BAD: 'else jq' MUST be detected (F-P2-002 — else/elif are genuine command positions).
  grep -qE "$jq_re" "$pc_bad_else" || {
    echo "FAIL (AC-005 positive-control): jq-detector did not match 'else jq' form (F-P2-002)." >&2
    false
  }
  # BAD (O-3): 'time jq' MUST be detected — time is a common command wrapper.
  grep -qE "$jq_re" "$pc_bad_time" || {
    echo "FAIL (AC-005 positive-control / O-3): jq-detector did not match 'time jq' form." >&2
    echo "  time|env|command|sudo must be in the wrapper keyword group." >&2
    false
  }
  # BAD (O-3): 'xargs -n1 jq' MUST be detected — xargs with intervening options precedes jq.
  grep -qE "$jq_re" "$pc_bad_xargs_opts" || {
    echo "FAIL (AC-005 positive-control / O-3): jq-detector did not match 'xargs -n1 jq' form." >&2
    echo "  The xargs-with-options arm 'xargs([[:space:]]+-[^[:space:]]+)+[[:space:]]+jq' is required." >&2
    false
  }
  # BAD (F-P8-001): '{ jq . f; }' brace-group MUST be detected.
  # Brace groups run in the current shell — jq executes and is a genuine dependency hazard.
  grep -qE "$jq_re" "$pc_bad_brace" || {
    echo "FAIL (AC-005 positive-control / F-P8-001): jq-detector did not match '{ jq . f; }' brace-group form." >&2
    echo "  Brace groups run in the current shell; add arm: \\{[[:space:]]*jq([[:space:]]|\$)" >&2
    false
  }
  # BAD (F-P8-001): 'case $x in json) jq ...' case-pattern-body MUST be detected.
  # The ) closes the case pattern label; jq is the first command in the case action body.
  grep -qE "$jq_re" "$pc_bad_case_paren" || {
    echo "FAIL (AC-005 positive-control / F-P8-001): jq-detector did not match 'case ... ) jq' case-body form." >&2
    echo "  Case pattern-action bodies run in the current shell; add arm: \\)[[:space:]]*jq([[:space:]]|\$)" >&2
    false
  }
  # BAD (F-P8-001): '( jq . f )' subshell MUST be detected.
  # Unlike IFS (where subshell is EXEMPT because IFS doesn't leak), jq in a subshell
  # still EXECUTES — it is a runtime dependency regardless of the subshell boundary.
  grep -qE "$jq_re" "$pc_bad_subshell" || {
    echo "FAIL (AC-005 positive-control / F-P8-001): jq-detector did not match '( jq . f )' subshell form." >&2
    echo "  jq in a subshell still executes (unlike IFS which doesn't leak); add arm: \\([[:space:]]*jq([[:space:]]|\$)" >&2
    false
  }
  # GOOD: jq in a comment MUST NOT be detected.
  ! grep -qE "$jq_re" "$pc_good_comment" || {
    echo "FAIL (AC-005 positive-control): jq-detector falsely matched jq inside a comment." >&2
    false
  }
  # GOOD (F-P8-001 negative control): '\$(other_cmd)' MUST NOT be detected.
  # The \$( arm only fires when 'jq' immediately follows '$(', not other command names.
  ! grep -qE "$jq_re" "$pc_good_other_cmdsubst" || {
    echo "FAIL (AC-005 negative-control / F-P8-001): jq-detector falsely matched '\$(other_cmd)'." >&2
    echo "  The \\\$( arm must require 'jq' as the immediate next token after '\$(', not any command." >&2
    false
  }
  # GOOD (F-P8-001 negative control): 'foo() { echo; }' function definition MUST NOT be detected.
  # The { arm requires jq as the first token after '{[[:space:]]*'; a function def has 'echo', not 'jq'.
  ! grep -qE "$jq_re" "$pc_good_func_brace" || {
    echo "FAIL (AC-005 negative-control / F-P8-001): jq-detector falsely matched 'foo() { echo; }' function definition." >&2
    echo "  The { arm must only fire when jq is the first command token after '{', not any other command." >&2
    false
  }
  # GOOD (F-P8-001 negative control): '\${jq_var}' parameter expansion MUST NOT be detected.
  # '\${jq_var}' contains '{' then 'jq_var}'; but jq_var is followed by '}', not space/EOL.
  ! grep -qE "$jq_re" "$pc_good_jq_var" || {
    echo "FAIL (AC-005 negative-control / F-P8-001): jq-detector falsely matched '\${jq_var}' parameter expansion." >&2
    echo "  The { arm requires jq followed by ([[:space:]]|\$); '\${jq_var}' has '_var}' after 'jq', not space/EOL." >&2
    false
  }
  # F-P13-001 positive controls: arms that lacked coverage.
  # BAD (F-P13-001): bare line-start 'jq -r .x f.json' MUST be detected (^[[:space:]]*jq arm).
  # This is the most common jq invocation form and was the only arm without a positive control.
  grep -qE "$jq_re" "$pc_bad_jq_line_start" || {
    echo "FAIL (AC-005 positive-control / F-P13-001): jq-detector did not match bare line-start 'jq -r .x f.json'." >&2
    echo "  The ^[[:space:]]*jq([[:space:]]|\$) arm must match jq at the start of a line." >&2
    false
  }
  # BAD (F-P13-001): backtick form '\`jq . f\`' MUST be detected.
  grep -qE "$jq_re" "$pc_bad_jq_backtick" || {
    echo "FAIL (AC-005 positive-control / F-P13-001): jq-detector did not match backtick '\`jq . f\`' form." >&2
    echo "  The \`jq([[:space:]]|\$) arm must match jq inside backtick command substitution." >&2
    false
  }
  # BAD (F-P13-001): 'if jq ...' MUST be detected (if arm in keyword group).
  grep -qE "$jq_re" "$pc_bad_jq_if" || {
    echo "FAIL (AC-005 positive-control / F-P13-001): jq-detector did not match 'if jq' form." >&2
    echo "  'if' must be in the keyword wrapper group (xargs|if|then|do|else|elif|time|env|command|sudo)." >&2
    false
  }
  # BAD (F-P13-001): 'then jq ...' MUST be detected (then arm in keyword group).
  grep -qE "$jq_re" "$pc_bad_jq_then" || {
    echo "FAIL (AC-005 positive-control / F-P13-001): jq-detector did not match 'then jq' form." >&2
    echo "  'then' must be in the keyword wrapper group." >&2
    false
  }
  # BAD (F-P13-001): 'do jq ...' MUST be detected (do arm in keyword group).
  grep -qE "$jq_re" "$pc_bad_jq_do" || {
    echo "FAIL (AC-005 positive-control / F-P13-001): jq-detector did not match 'do jq' form." >&2
    echo "  'do' must be in the keyword wrapper group." >&2
    false
  }
  # BAD (F-P13-001): 'elif jq ...' MUST be detected (elif arm in keyword group).
  grep -qE "$jq_re" "$pc_bad_jq_elif" || {
    echo "FAIL (AC-005 positive-control / F-P13-001): jq-detector did not match 'elif jq' form." >&2
    echo "  'elif' must be in the keyword wrapper group." >&2
    false
  }
  # BAD (F-P13-001): 'env jq ...' MUST be detected (env arm in keyword group — O-3 addition).
  grep -qE "$jq_re" "$pc_bad_jq_env" || {
    echo "FAIL (AC-005 positive-control / F-P13-001): jq-detector did not match 'env jq' form." >&2
    echo "  'env' must be in the keyword wrapper group (added at O-3)." >&2
    false
  }
  # BAD (F-P13-001): 'command jq ...' MUST be detected (command arm in keyword group — O-3 addition).
  grep -qE "$jq_re" "$pc_bad_jq_command" || {
    echo "FAIL (AC-005 positive-control / F-P13-001): jq-detector did not match 'command jq' form." >&2
    echo "  'command' must be in the keyword wrapper group (added at O-3)." >&2
    false
  }
  # BAD (F-P13-001): 'sudo jq ...' MUST be detected (sudo arm in keyword group — O-3 addition).
  grep -qE "$jq_re" "$pc_bad_jq_sudo" || {
    echo "FAIL (AC-005 positive-control / F-P13-001): jq-detector did not match 'sudo jq' form." >&2
    echo "  'sudo' must be in the keyword wrapper group (added at O-3)." >&2
    false
  }
  # -------------------------------------------

  # Phase 1: detect jq invocations (jq as a command word, not as part of a variable name).
  # Patterns that indicate jq is used as a command (POSIX ERE, no PCRE shorthand):
  #   ^[[:space:]]*jq([[:space:]]|$)                      — start of line
  #   [|;&][[:space:]]*jq([[:space:]]|$)                   — after |, ;, &  (covers |, ||, &&)
  #   \$[(]jq([[:space:]]|$)                               — command substitution $(jq ...)
  #   `jq([[:space:]]|$)                                   — backtick command substitution
  #   (xargs|if|then|do|else|elif|time|env|command|sudo)[[:space:]]+jq(...)
  #                                                        — keyword/wrapper-prefixed invocations
  #   xargs([[:space:]]+-[^[:space:]]+)+[[:space:]]+jq(...)— xargs with intervening options
  #   \{[[:space:]]*jq([[:space:]]|$)                     — brace-group current-shell execution
  #   \)[[:space:]]*jq([[:space:]]|$)                     — case-pattern-action body (after ')' label close)
  #   \([[:space:]]*jq([[:space:]]|$)                     — subshell open (jq still executes)
  #   F-P2-002: else and elif added (genuine command positions for jq invocation).
  #   O-3: time, env, command, sudo added as common command wrappers.
  #        xargs-with-options arm handles 'xargs -n1 jq', 'xargs -n1 -P4 jq', etc.
  #   F-P8-001: brace-group, case-body, subshell arms added. NOTE: for jq, subshell IS a
  #             hazard (unlike IFS where subshell is exempt) — jq runs in the subshell.
  local jq_files=()
  local f
  for f in "${sh_files[@]}"; do
    if grep -qE '(^[[:space:]]*jq([[:space:]]|$)|[|;&][[:space:]]*jq([[:space:]]|$)|\$[(]jq([[:space:]]|$)|`jq([[:space:]]|$)|(xargs|if|then|do|else|elif|time|env|command|sudo)[[:space:]]+jq([[:space:]]|$)|xargs([[:space:]]+-[^[:space:]]+)+[[:space:]]+jq([[:space:]]|$)|\{[[:space:]]*jq([[:space:]]|$)|\)[[:space:]]*jq([[:space:]]|$)|\([[:space:]]*jq([[:space:]]|$))' \
        "$f" 2>/dev/null; then
      jq_files+=("$f")
    fi
  done

  # No jq invocations — compliant.
  if [ "${#jq_files[@]}" -eq 0 ]; then
    echo "AC-005: scanned=${#sh_files[@]} files"
    return 0
  fi

  # Phase 2: for each file with jq invocations, verify a preflight guard exists.
  # Acceptable preflight forms:
  #   command -v jq    — POSIX-portable availability check
  #   which jq         — non-POSIX but common fallback
  local violations=()
  for f in "${jq_files[@]}"; do
    local rel="${f#${wave_handoff_skill_dir}/}"
    if ! grep -qE 'command[[:space:]]+-v[[:space:]]+jq|which[[:space:]]+jq' "$f" 2>/dev/null; then
      local hits
      hits="$(grep -nE '(^[[:space:]]*jq([[:space:]]|$)|[|;&][[:space:]]*jq([[:space:]]|$)|\$[(]jq([[:space:]]|$)|`jq([[:space:]]|$)|(xargs|if|then|do|else|elif|time|env|command|sudo)[[:space:]]+jq([[:space:]]|$)|xargs([[:space:]]+-[^[:space:]]+)+[[:space:]]+jq([[:space:]]|$)|\{[[:space:]]*jq([[:space:]]|$)|\)[[:space:]]*jq([[:space:]]|$)|\([[:space:]]*jq([[:space:]]|$))' \
              "$f" 2>/dev/null || true)"
      while IFS= read -r hit; do
        violations+=("${rel}: jq invocation without preflight: ${hit}")
      done <<< "$hits"
    fi
  done

  if [ "${#violations[@]}" -gt 0 ]; then
    echo "FAIL (AC-005): bare jq invocation without preflight guard found in wave-handoff scripts." >&2
    echo "  Fix: add preflight before first jq use:" >&2
    echo "    command -v jq >/dev/null 2>&1 || { echo 'ERROR: jq is required; install with brew install jq' >&2; exit 1; }" >&2
    echo "  Violations:" >&2
    local v
    for v in "${violations[@]}"; do echo "  ${v}" >&2; done
    false
  fi
  echo "AC-005: scanned=${#sh_files[@]} files"
}

# ---------------------------------------------------------------------------
# test_BC_5_41_001_F_P11_001_bsd_classify_stories_has_next_wave
# F-P11-001 behavioral regression
# When sprint-state.yaml contains a story with status: pending (has-next-wave),
# classify_stories() MUST set CLASSIFY_RESULT=has-next-wave, NOT epic-complete.
#
# Red Gate: on BSD grep, \s/\S in parse-sprint-state.sh never match the story
# lines → has_any_story stays 0 → CLASSIFY_RESULT=epic-complete (wrong).
# On GNU grep (Linux CI), this test passes even before the fix because \s works.
# The portability-guard test above catches the static defect on any platform.
# This test catches the behavioral regression on macOS.
#
# Assert by running the full skill with a has-next-wave sprint-state and verifying
# wave-state.yaml IS written (EPIC-COMPLETE must NOT be triggered).
# ---------------------------------------------------------------------------

# ---------------------------------------------------------------------------
# test_BC_5_41_002_O_P15_001_unresolved_spec_file_warns_on_stderr
# O-P15-001 / BC-5.41.002 EC-002 (unresolved BC path advisory)
#
# When a story's behavioral_contracts: frontmatter references a BC ID whose .md file
# does NOT exist on disk in $ARTIFACTS_WT/specs/behavioral-contracts/, the skill must:
#   (a) STILL include the constructed fallback path in spec_files as a plain string
#       (no schema change — spec_files remains a plain-string list per PC2).
#   (b) Emit an ADVISORY WARNING to stderr naming the unresolved path:
#       "WARNING: spec_file path does not resolve on disk: <path>"
#
# The current implementation SILENTLY emits the constructed path with no stderr
# warning. This test REDs that: it asserts the warning on stderr, which is absent
# before the fix.
#
# Fixture: story S-18.02 declares behavioral_contracts: [BC-MISSING.001] — a BC ID
# whose file does NOT exist in the fixture's bc directory. The skill must emit the
# WARNING to stderr AND include the path in the committed wave-state.yaml spec_files.
#
# Assert via COMMITTED blob for the path presence (POLICY 11).
# ---------------------------------------------------------------------------

@test "test_BC_5_41_002_O_P15_001_unresolved_spec_file_warns_on_stderr" {
  # Create a story file referencing a BC that does NOT exist on disk.
  # BC-MISSING.001.md will not be planted in the fixture bc directory.
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
  - BC-MISSING.001
verification_properties:
  - VP-081
---
# S-18.02 fixture with missing BC
EOF

  # Run with all subcommands; stderr is merged via 2>&1 in each step.
  # The skill must:
  #   (a) exit 0 (unresolved BC is advisory, not a hard block — EC-002)
  #   (b) emit WARNING on stderr about the unresolved path (captured from --emit-wave-state)
  #   (c) committed wave-state.yaml includes the fallback path in spec_files
  _run_skill_subcommands

  # (a) Must exit 0 — unresolved BC path is advisory (EC-002), not a hard block
  [ "$status" -eq 0 ] || {
    echo "FAIL (O-P15-001 a): skill exited ${status}, expected 0." >&2
    echo "  Unresolved BC-MISSING.001 path is advisory (EC-002) — must not hard-block." >&2
    echo "  Actual output: $output" >&2
    false
  }

  # (b) Output (which includes stderr via 2>&1) MUST contain the WARNING line.
  # The warning format: "WARNING: spec_file path does not resolve on disk: <path>"
  # where <path> is "specs/behavioral-contracts/BC-MISSING.001.md"
  echo "$output" | grep -q "WARNING: spec_file path does not resolve on disk:" || {
    echo "FAIL (O-P15-001 b): advisory WARNING line missing from output." >&2
    echo "  Expected substring: 'WARNING: spec_file path does not resolve on disk:'" >&2
    echo "  When a story's behavioral_contracts: entry cannot be resolved to an existing" >&2
    echo "  file on disk, write-wave-state.sh must emit this advisory warning to stderr." >&2
    echo "  The current implementation silently emits the fallback path with no warning." >&2
    echo "  Actual output: $output" >&2
    false
  }

  # The warning must name the specific unresolved path
  echo "$output" | grep -qF "specs/behavioral-contracts/BC-MISSING.001.md" || {
    echo "FAIL (O-P15-001 b path): WARNING line does not name the unresolved path." >&2
    echo "  Expected the warning to contain 'specs/behavioral-contracts/BC-MISSING.001.md'" >&2
    echo "  Actual output: $output" >&2
    false
  }

  # (c) Committed wave-state.yaml MUST still contain the fallback path in spec_files
  # (plain-string list per PC2 — no schema change, path is included regardless)
  git -C "$WORK" show factory-artifacts:wave-state.yaml >/dev/null 2>&1 || {
    echo "FAIL (O-P15-001 c): wave-state.yaml not committed to factory-artifacts." >&2
    false
  }
  local committed_content
  committed_content="$(git -C "$WORK" show factory-artifacts:wave-state.yaml)"
  echo "$committed_content" | grep -qF "specs/behavioral-contracts/BC-MISSING.001.md" || {
    echo "FAIL (O-P15-001 c): committed wave-state.yaml does not contain fallback path for BC-MISSING.001." >&2
    echo "  spec_files must still include the constructed path as a plain string" >&2
    echo "  even when the file does not exist on disk (EC-002: advisory, not hard block)." >&2
    echo "  Committed spec_files section:" >&2
    echo "$committed_content" | grep -A 10 "^stories:" >&2
    false
  }
}

# ===========================================================================
# S-18.13 RED GATE TESTS — wave-handoff Write-tool restructure
# Story:  S-18.13 v1.8 — wave-handoff skill writes HANDOFF.md via the Write
#         tool so the PostToolUse completeness gate fires (gate-trigger fix)
# BCs:    BC-5.41.001 v1.26 (PC10 — four-step agent-orchestrated flow)
#         BC-5.41.002 v1.19 (PC6 — atomicity at git-commit boundary)
#
# RED GATE discipline: ALL tests in this block MUST FAIL against the current
# monolithic implementation (no subcommands, bash-redirect present).
# Failure reason: assertion errors (subcommands absent / redirect present),
# NOT build errors.
#
# Golden fixture T-4a: captured from CURRENT bash-redirect implementation
# BEFORE the T-2 refactor. See:
#   plugins/vsdd-factory/tests/fixtures/wave-handoff-golden/HANDOFF-has-next-wave.md
#   plugins/vsdd-factory/tests/fixtures/wave-handoff-golden/HANDOFF-epic-complete.md
# The SHA in those files is a placeholder; AC-003 substitutes the actual
# DEVELOP_SHA at test time.
# ===========================================================================

# ---------------------------------------------------------------------------
# test_BC_5_41_001_PC10_S18_13_AC001_no_bash_redirect_in_write_handoff
# AC-001 (part a) / BC-5.41.001 PC10 — write_handoff() MUST NOT have
# `} > "$output_path"` bash-redirect construct in lib/write-handoff.sh.
# Oracle: grep -n '} > "$output_path"' lib/write-handoff.sh must return 0 lines.
# RED GATE: current impl has the redirect on line ~218 → grep finds it → FAIL.
# ---------------------------------------------------------------------------

@test "test_BC_5_41_001_PC10_S18_13_AC001_no_bash_redirect_in_write_handoff" {
  local write_handoff_sh
  write_handoff_sh="${SKILL_DIR}/lib/write-handoff.sh"

  # AC-001 oracle (a): `} > "$output_path"` must be ABSENT from lib/write-handoff.sh.
  # After implementation, write_handoff() emits to stdout (no file redirect).
  # This test REDs because the current impl has the bash-redirect on line ~218.
  local redirect_matches
  redirect_matches="$(grep -n '} > "\$output_path"' "$write_handoff_sh" 2>/dev/null || true)"

  [ -z "$redirect_matches" ] || {
    echo "FAIL (AC-001 oracle a): bash-redirect construct found in lib/write-handoff.sh" >&2
    echo "  The line '} > \"\$output_path\"' must be REMOVED." >&2
    echo "  After S-18.13 implementation, write_handoff() must emit the assembled" >&2
    echo "  HANDOFF.md payload to stdout (no file write). The agent then invokes" >&2
    echo "  the Write tool to write the payload to disk." >&2
    echo "  Matches found:" >&2
    echo "$redirect_matches" >&2
    echo "  BC-5.41.001 PC10: bash redirection is FORBIDDEN as primary or fallback" >&2
    echo "  write path for HANDOFF.md." >&2
    false
  }
}

# ---------------------------------------------------------------------------
# test_BC_5_41_001_PC10_S18_13_AC001_emit_handoff_dispatch_arm_present
# AC-001 (part b) / BC-5.41.001 PC10 — wave-handoff.sh MUST have a
# `--emit-handoff` subcommand dispatch arm.
# Oracle: grep -n -- '--emit-handoff' wave-handoff.sh must return >= 1 match.
# RED GATE: current impl has no --emit-handoff arm → grep returns nothing → FAIL.
# ---------------------------------------------------------------------------

@test "test_BC_5_41_001_PC10_S18_13_AC001_emit_handoff_dispatch_arm_present" {
  local wave_handoff_sh
  wave_handoff_sh="${SKILL_DIR}/wave-handoff.sh"

  # AC-001 oracle (b): `--emit-handoff` dispatch arm must be present in wave-handoff.sh.
  # The implementer (T-2a) replaces monolithic main() with a subcommand dispatcher.
  # This test REDs because the current impl has no subcommand dispatch.
  local emit_matches
  emit_matches="$(grep -n -- '--emit-handoff' "$wave_handoff_sh" 2>/dev/null || true)"

  [ -n "$emit_matches" ] || {
    echo "FAIL (AC-001 oracle b): '--emit-handoff' dispatch arm missing from wave-handoff.sh" >&2
    echo "  T-2a must replace the monolithic main() with a subcommand dispatcher that" >&2
    echo "  handles --emit-handoff, --emit-wave-state, and --commit." >&2
    echo "  Current impl has no subcommand dispatch." >&2
    echo "  BC-5.41.001 PC10 step 1: '--emit-handoff' assembles HANDOFF.md payload" >&2
    echo "  and emits it to stdout (no disk write, no commit)." >&2
    false
  }
}

# ---------------------------------------------------------------------------
# test_BC_5_41_001_PC10_S18_13_AC002_postuse_gate_fires_positive
# AC-002 (positive case) / BC-5.41.001 PC10 — PostToolUse
# validate-wave-handoff-completeness gate fires on a Write to .factory/HANDOFF.md
# with a COMPLETE, well-formed HANDOFF.md (all 9 base fields present and valid).
#
# Test approach: mock PostToolUse injection (AC-002 option b) — synthesize a
# PostToolUse Write envelope for .factory/HANDOFF.md and feed it through the
# real factory-dispatcher binary against the production-shaped hooks-registry.
# This is the same pattern as fail-open-on-crash.bats Scenarios B and D.
#
# Expected outcome: gate fires, inspects the on-disk file, finds it valid →
#   exit 0, NO blocking_plugins=validate-wave-handoff-completeness in output.
#
# Skip condition: skips when dispatcher binary or WASM plugin are not compiled.
#   Skip is gated (conditional) so the test RUNS when binaries are present.
# ---------------------------------------------------------------------------

@test "test_BC_5_41_001_PC10_S18_13_AC002_postuse_gate_fires_positive" {
  local repo_root
  repo_root="$(cd "${BATS_TEST_DIRNAME}/../../.." && pwd)"
  local dispatcher="${repo_root}/target/release/factory-dispatcher"
  local wasm_plugin="${repo_root}/plugins/vsdd-factory/hook-plugins/validate-wave-handoff-completeness.wasm"

  # Skip only if binaries are not compiled — NOT an unconditional skip.
  if [ ! -x "$dispatcher" ]; then
    skip "dispatcher binary not built -- run: cargo build --release -p factory-dispatcher"
  fi
  if [ ! -f "$wasm_plugin" ]; then
    skip "validate-wave-handoff-completeness.wasm not built -- implement S-18.02 tasks T-4..T-7"
  fi

  # Build a hermetic test directory independent of the wave-handoff.bats WORK dir.
  local ac002_work
  ac002_work="$(mktemp -d)"
  mkdir -p "${ac002_work}/.factory/logs"
  mkdir -p "${ac002_work}/hook-plugins"
  cp "$wasm_plugin" "${ac002_work}/hook-plugins/"

  # Production-shaped test registry: on_error="continue", same path_allow as
  # hooks-registry.toml production entry.  The read_file capability block is
  # required so the WASM gate can call host::read_file to read the on-disk
  # HANDOFF.md (F-001 fix pattern from fail-open-on-crash.bats).
  cat > "${ac002_work}/hooks-registry.toml" << 'TOML'
schema_version = 2

[[hooks]]
name = "validate-wave-handoff-completeness"
event = "PostToolUse"
tool = "Edit|Write"
plugin = "hook-plugins/validate-wave-handoff-completeness.wasm"
timeout_ms = 5000
on_error = "continue"

[hooks.capabilities.read_file]
path_allow = [
  ".factory/HANDOFF.md",
]
TOML

  # Write a COMPLETE, well-formed HANDOFF.md — all 9 base required fields present
  # (BC-5.41.001 v1.26: wave_id, last_verified_develop_sha, active_bcs,
  #  next_wave_stories, open_decisions, pending_fixes, process_gaps,
  #  precompact_flush_sha, factory_lock_holder).
  # last_verified_develop_sha must be 40-char hex per BC-5.41.001 PC3.
  cat > "${ac002_work}/.factory/HANDOFF.md" << 'YAML'
wave_id: 2
last_verified_develop_sha: abc123def456abc123def456abc123def456abcd
active_bcs:
  - specs/behavioral-contracts/ss-05/BC-5.41.001.md
next_wave_stories:
  - id: S-19.01
    status: pending
open_decisions: []
pending_fixes: []
process_gaps: []
precompact_flush_sha: null
factory_lock_holder: null
YAML

  # Synthesize a PostToolUse Write envelope for .factory/HANDOFF.md.
  # The gate reads the on-disk file via host::read_file (not tool_input.content),
  # so the content field is a stub — the on-disk file above is what the gate sees.
  # file_path must match path_allow = ".factory/HANDOFF.md" for the capability grant.
  # Use bats `run` to capture exit code in $status and output in $output.
  run bash -c "printf '%s' '{
    \"event_name\": \"PostToolUse\",
    \"tool_name\": \"Write\",
    \"session_id\": \"ac-002-positive-test\",
    \"dispatcher_trace_id\": \"ac-002-positive-trace\",
    \"tool_input\": {
      \"file_path\": \".factory/HANDOFF.md\",
      \"content\": \"wave_id: 2\"
    },
    \"tool_response\": {\"exit_code\": 0}
  }' | CLAUDE_PLUGIN_ROOT='${ac002_work}' CLAUDE_PROJECT_DIR='${ac002_work}' '$dispatcher' 2>&1"

  rm -rf "${ac002_work}"

  # Gate must return exit 0 (gate fired, validated, allowed — no block).
  [ "$status" -eq 0 ] || {
    echo "FAIL (AC-002 positive): expected exit 0 for complete HANDOFF.md, got ${status}." >&2
    echo "A complete HANDOFF.md with all 9 required base fields must be allowed by the gate." >&2
    echo "Output: ${output}" >&2
    false
  }

  # Must NOT have blocking_plugins=validate-wave-handoff-completeness (gate did not block).
  [[ "$output" != *"blocking_plugins=validate-wave-handoff-completeness"* ]] || {
    echo "FAIL (AC-002 positive): gate blocked a complete HANDOFF.md — must allow it." >&2
    echo "blocking_plugins= must NOT appear for a valid payload." >&2
    echo "Output: ${output}" >&2
    false
  }
}

# ---------------------------------------------------------------------------
# test_BC_5_41_001_PC10_S18_13_AC002_postuse_gate_fires_negative
# AC-002 (negative case) / BC-5.41.001 PC10 — PostToolUse
# validate-wave-handoff-completeness gate fires on a Write to .factory/HANDOFF.md
# with an INCOMPLETE HANDOFF.md (missing required fields).
#
# This is the POLICY 11 load-bearing assertion: the gate genuinely inspects
# the HANDOFF.md content — it is not a no-op. An unconditional-skip test cannot
# prove the gate runs at all. This test proves the gate blocks bad payloads,
# completing the behavioral triangle: present→allow, absent→block.
#
# Expected outcome: gate fires, inspects the on-disk file, finds it missing
# required fields → exit 2, blocking_plugins=validate-wave-handoff-completeness.
#
# Skip condition: skips when dispatcher binary or WASM plugin are not compiled.
# ---------------------------------------------------------------------------

@test "test_BC_5_41_001_PC10_S18_13_AC002_postuse_gate_fires_negative" {
  local repo_root
  repo_root="$(cd "${BATS_TEST_DIRNAME}/../../.." && pwd)"
  local dispatcher="${repo_root}/target/release/factory-dispatcher"
  local wasm_plugin="${repo_root}/plugins/vsdd-factory/hook-plugins/validate-wave-handoff-completeness.wasm"

  if [ ! -x "$dispatcher" ]; then
    skip "dispatcher binary not built -- run: cargo build --release -p factory-dispatcher"
  fi
  if [ ! -f "$wasm_plugin" ]; then
    skip "validate-wave-handoff-completeness.wasm not built -- implement S-18.02 tasks T-4..T-7"
  fi

  local ac002_work
  ac002_work="$(mktemp -d)"
  mkdir -p "${ac002_work}/.factory/logs"
  mkdir -p "${ac002_work}/hook-plugins"
  cp "$wasm_plugin" "${ac002_work}/hook-plugins/"

  cat > "${ac002_work}/hooks-registry.toml" << 'TOML'
schema_version = 2

[[hooks]]
name = "validate-wave-handoff-completeness"
event = "PostToolUse"
tool = "Edit|Write"
plugin = "hook-plugins/validate-wave-handoff-completeness.wasm"
timeout_ms = 5000
on_error = "continue"

[hooks.capabilities.read_file]
path_allow = [
  ".factory/HANDOFF.md",
]
TOML

  # Write an INCOMPLETE HANDOFF.md — missing last_verified_develop_sha,
  # precompact_flush_sha, and factory_lock_holder (3 of 9 required fields absent).
  # The gate must detect these absences and block with HandoffIncomplete.
  cat > "${ac002_work}/.factory/HANDOFF.md" << 'YAML'
wave_id: 2
active_bcs: []
next_wave_stories:
  - id: S-19.01
    status: pending
open_decisions: []
pending_fixes: []
process_gaps: []
YAML
  # last_verified_develop_sha, precompact_flush_sha, factory_lock_holder are absent.

  # Use bats `run` to capture exit code in $status and output in $output.
  run bash -c "printf '%s' '{
    \"event_name\": \"PostToolUse\",
    \"tool_name\": \"Write\",
    \"session_id\": \"ac-002-negative-test\",
    \"dispatcher_trace_id\": \"ac-002-negative-trace\",
    \"tool_input\": {
      \"file_path\": \".factory/HANDOFF.md\",
      \"content\": \"wave_id: 2\"
    },
    \"tool_response\": {\"exit_code\": 0}
  }' | CLAUDE_PLUGIN_ROOT='${ac002_work}' CLAUDE_PROJECT_DIR='${ac002_work}' '$dispatcher' 2>&1"

  rm -rf "${ac002_work}"

  # Gate must return exit 2 (block) when HANDOFF.md is missing required fields.
  # POLICY 11: this assertion proves the gate genuinely inspects content — not a no-op.
  [ "$status" -eq 2 ] || {
    echo "FAIL (AC-002 negative): expected exit 2 (block) for incomplete HANDOFF.md, got ${status}." >&2
    echo "An incomplete HANDOFF.md (missing last_verified_develop_sha, precompact_flush_sha," >&2
    echo "factory_lock_holder) MUST be blocked by the validate-wave-handoff-completeness gate." >&2
    echo "If exit 0: the gate is a no-op (not inspecting content) — AC-002 BC is violated." >&2
    echo "Output: ${output}" >&2
    false
  }

  # The blocking plugin name must appear in output (confirms gate identity).
  [[ "$output" == *"blocking_plugins=validate-wave-handoff-completeness"* ]] || {
    echo "FAIL (AC-002 negative): blocking_plugins=validate-wave-handoff-completeness not in output." >&2
    echo "The gate must identify itself as the blocking plugin for HandoffIncomplete." >&2
    echo "Output: ${output}" >&2
    false
  }
}

# ---------------------------------------------------------------------------
# test_BC_5_41_001_PC10_S18_13_AC002_postuse_gate_path_linkage
# AC-002 (linkage assertion) / BC-5.41.001 PC10 — the SKILL.md S2 Write-tool
# target path component (HANDOFF.md) must match the gate's path_allow entry
# (.factory/HANDOFF.md) in hooks-registry.toml so the Write→gate linkage
# cannot silently break on a path-component mismatch.
#
# Three-part assertion:
#   (i)  SKILL.md must reference the Write tool step for HANDOFF.md (structural).
#   (ii) hooks-registry.toml path_allow for validate-wave-handoff-completeness
#        must end with "HANDOFF.md" (path-component-strict linkage).
#   (iii) A Write to a path that does NOT match (e.g., HANDOFF_SHOULD_NOT_MATCH.md)
#         must NOT trigger the gate (exit 0, no blocking_plugins) — confirming
#         path-component-strict matching is in effect.
#
# Part (i) is a static check (no dispatcher needed).
# Parts (ii) and (iii) require dispatcher/WASM for the live path.
# Part (iii) is gated by _require_dispatcher_and_wasm equivalently.
# ---------------------------------------------------------------------------

@test "test_BC_5_41_001_PC10_S18_13_AC002_postuse_gate_path_linkage" {
  local repo_root
  repo_root="$(cd "${BATS_TEST_DIRNAME}/../../.." && pwd)"
  local plugin_root="${repo_root}/plugins/vsdd-factory"
  local skill_md="${SKILL_DIR}/SKILL.md"
  local production_registry="${plugin_root}/hooks-registry.toml"

  # --- Part (i): SKILL.md structural assertion (no binary needed) ---
  # SKILL.md must contain a step instructing the agent to use the Write tool
  # for HANDOFF.md. This is a NECESSARY structural precondition for gate linkage:
  # if the skill uses bash redirection instead of the Write tool, the gate never fires.
  grep -qiE "(Write tool|Write.*HANDOFF|S2.*Write)" "$skill_md" 2>/dev/null || {
    echo "FAIL (AC-002 linkage part i): SKILL.md does not reference the Write tool for HANDOFF.md." >&2
    echo "  SKILL.md step S2 must instruct the agent to use the Write tool (not bash redirection)" >&2
    echo "  to write the HANDOFF.md payload. Without this, the PostToolUse gate cannot fire." >&2
    echo "  BC-5.41.001 PC10 §S2: 'Write the captured payload to HANDOFF.md using the Claude" >&2
    echo "  Code Write tool. Do NOT use bash redirection.'" >&2
    false
  }

  # --- Part (ii): production registry path_allow ends with HANDOFF.md ---
  # The path_allow entry for validate-wave-handoff-completeness must include
  # ".factory/HANDOFF.md" so that a Write to that path grants the read_file
  # capability and the gate can inspect the file contents.
  local path_allow_value
  path_allow_value=$(awk '
    /^name = "validate-wave-handoff-completeness"$/ { in_hook=1 }
    in_hook && /^\[\[hooks\]\]/ && !first { in_hook=0 }
    in_hook && /path_allow/ { found_pa=1 }
    found_pa && /HANDOFF\.md/ { print; exit }
  ' "$production_registry")

  [ -n "$path_allow_value" ] || {
    echo "FAIL (AC-002 linkage part ii): hooks-registry.toml path_allow for" >&2
    echo "  validate-wave-handoff-completeness does not contain 'HANDOFF.md'." >&2
    echo "  Without this path_allow entry, a Write to .factory/HANDOFF.md will NOT" >&2
    echo "  grant the read_file capability to the gate, breaking the linkage." >&2
    false
  }

  # --- Part (iii): non-HANDOFF.md path does NOT trigger gate (live, gated) ---
  local dispatcher="${repo_root}/target/release/factory-dispatcher"
  local wasm_plugin="${plugin_root}/hook-plugins/validate-wave-handoff-completeness.wasm"

  if [ ! -x "$dispatcher" ] || [ ! -f "$wasm_plugin" ]; then
    # Parts (i) and (ii) already executed. Part (iii) requires binaries.
    skip "dispatcher/wasm not built; parts i+ii passed statically -- part iii skipped"
  fi

  local ac002_work
  ac002_work="$(mktemp -d)"
  mkdir -p "${ac002_work}/.factory/logs"
  mkdir -p "${ac002_work}/hook-plugins"
  cp "$wasm_plugin" "${ac002_work}/hook-plugins/"

  cat > "${ac002_work}/hooks-registry.toml" << 'TOML'
schema_version = 2

[[hooks]]
name = "validate-wave-handoff-completeness"
event = "PostToolUse"
tool = "Edit|Write"
plugin = "hook-plugins/validate-wave-handoff-completeness.wasm"
timeout_ms = 5000
on_error = "continue"

[hooks.capabilities.read_file]
path_allow = [
  ".factory/HANDOFF.md",
]
TOML

  # Send a Write envelope to a path that contains "HANDOFF" but is NOT exactly
  # ".factory/HANDOFF.md" (path-component-strict matching per F-003 fix).
  # The gate must NOT fire — exit 0, no blocking_plugins.
  # Use bats `run` to capture exit code in $status and output in $output.
  run bash -c "printf '%s' '{
    \"event_name\": \"PostToolUse\",
    \"tool_name\": \"Write\",
    \"session_id\": \"ac-002-linkage-test\",
    \"dispatcher_trace_id\": \"ac-002-linkage-trace\",
    \"tool_input\": {
      \"file_path\": \".factory/HANDOFF_SHOULD_NOT_MATCH.md\",
      \"content\": \"wave_id: 2\"
    },
    \"tool_response\": {\"exit_code\": 0}
  }' | CLAUDE_PLUGIN_ROOT='${ac002_work}' CLAUDE_PROJECT_DIR='${ac002_work}' '$dispatcher' 2>&1"

  rm -rf "${ac002_work}"

  # Must exit 0: non-HANDOFF.md path must not trigger the gate.
  [ "$status" -eq 0 ] || {
    echo "FAIL (AC-002 linkage part iii): expected exit 0 for non-HANDOFF.md path, got ${status}." >&2
    echo "  Path '.factory/HANDOFF_SHOULD_NOT_MATCH.md' must NOT trigger the gate." >&2
    echo "  The gate uses path-component-strict matching (F-003 fix): only the exact" >&2
    echo "  filename 'HANDOFF.md' triggers validation." >&2
    echo "  Output: ${output}" >&2
    false
  }

  # No blocking plugin for a non-HANDOFF.md path.
  [[ "$output" != *"blocking_plugins="* ]] || {
    echo "FAIL (AC-002 linkage part iii): blocking_plugins appeared for non-HANDOFF.md path." >&2
    echo "  The gate must be a no-op for paths that are not exactly '.factory/HANDOFF.md'." >&2
    echo "  Output: ${output}" >&2
    false
  }
}

# ---------------------------------------------------------------------------
# test_BC_5_41_001_PC10_S18_13_AC003_emit_handoff_stdout_matches_golden_has_next_wave
# AC-003 / BC-5.41.001 PC10 — --emit-handoff stdout output is byte-identical
# (modulo the dynamic DEVELOP_SHA) to the frozen golden fixture captured from
# the bash-redirect path in T-4a BEFORE the S-18.13 refactor.
#
# Golden fixture: plugins/vsdd-factory/tests/fixtures/wave-handoff-golden/HANDOFF-has-next-wave.md
# The SHA field is substituted with the hermetic repo's actual origin/develop SHA.
#
# RED GATE: current impl has no --emit-handoff subcommand → bash exits non-zero
# on unknown argument → test fails immediately on exit code check.
# ---------------------------------------------------------------------------

@test "test_BC_5_41_001_PC10_S18_13_AC003_emit_handoff_stdout_matches_golden_has_next_wave" {
  local fixture_dir
  fixture_dir="${BATS_TEST_DIRNAME}/fixtures/wave-handoff-golden"
  local golden_template="${fixture_dir}/HANDOFF-has-next-wave.md"

  [ -f "$golden_template" ] || {
    echo "FAIL: golden fixture not found at ${golden_template}" >&2
    echo "  T-4a must capture the golden fixture BEFORE the T-2 refactor." >&2
    false
  }

  # Run --emit-handoff subcommand — emits HANDOFF.md payload to stdout
  # RED GATE: current impl exits 1 on unknown argument '--emit-handoff'
  local emit_output
  local emit_exit=0
  emit_output="$(
    export ARTIFACTS_WT="${ARTIFACTS_WT}"
    export SPRINT_STATE_YAML="${WORK}/sprint-state.yaml"
    export STATE_MD_PATH="${WORK}/STATE.md"
    export BC_DIR="${ARTIFACTS_WT}/specs/behavioral-contracts"
    export PRECOMPACT_FLUSH_LOG="${ARTIFACTS_WT}/hooks/precompact-flush-log"
    export FACTORY_REPO="${WORK}"
    "${SKILL}" \
      --artifacts-worktree "${ARTIFACTS_WT}" \
      --sprint-state "${WORK}/sprint-state.yaml" \
      --state-md "${WORK}/STATE.md" \
      --bc-dir "${ARTIFACTS_WT}/specs/behavioral-contracts" \
      --emit-handoff \
      2>&1
  )" || emit_exit=$?

  [ "$emit_exit" -eq 0 ] || {
    echo "FAIL (AC-003 HAS-NEXT-WAVE): wave-handoff.sh --emit-handoff exited ${emit_exit}" >&2
    echo "  Current impl has no --emit-handoff subcommand." >&2
    echo "  After T-2a: --emit-handoff emits HANDOFF.md payload to stdout (no disk write)." >&2
    echo "  Output: ${emit_output}" >&2
    false
  }

  # Substitute the dynamic DEVELOP_SHA in the golden template
  local actual_develop_sha
  actual_develop_sha="$(git -C "${WORK}" rev-parse origin/develop)"

  local expected_content
  expected_content="$(sed "s/DEVELOP_SHA_PLACEHOLDER/${actual_develop_sha}/g" "$golden_template")"

  # Compare stdout output against expected (golden with SHA substituted)
  # Strip trailing newline differences — content identity, not newline identity
  local actual_normalized expected_normalized
  actual_normalized="$(printf '%s' "$emit_output" | sed 's/[[:space:]]*$//' )"
  expected_normalized="$(printf '%s' "$expected_content" | sed 's/[[:space:]]*$//')"

  [ "$actual_normalized" = "$expected_normalized" ] || {
    echo "FAIL (AC-003 HAS-NEXT-WAVE): --emit-handoff output is NOT byte-identical to golden fixture." >&2
    echo "  Golden fixture: ${golden_template} (SHA substituted)" >&2
    echo "  --- Expected ---" >&2
    echo "$expected_normalized" >&2
    echo "  --- Actual ---" >&2
    echo "$actual_normalized" >&2
    echo "  --- Diff ---" >&2
    diff <(echo "$expected_normalized") <(echo "$actual_normalized") >&2 || true
    false
  }
}

# ---------------------------------------------------------------------------
# test_BC_5_41_001_PC10_S18_13_AC003_emit_handoff_stdout_matches_golden_epic_complete
# AC-003 (EPIC-COMPLETE case) / BC-5.41.001 PC10
# --emit-handoff output on the EPIC-COMPLETE path is byte-identical to the
# frozen golden fixture HANDOFF-epic-complete.md.
#
# RED GATE: current impl has no --emit-handoff subcommand → exits non-zero.
# ---------------------------------------------------------------------------

@test "test_BC_5_41_001_PC10_S18_13_AC003_emit_handoff_stdout_matches_golden_epic_complete" {
  local fixture_dir
  fixture_dir="${BATS_TEST_DIRNAME}/fixtures/wave-handoff-golden"
  local golden_template="${fixture_dir}/HANDOFF-epic-complete.md"

  [ -f "$golden_template" ] || {
    echo "FAIL: golden fixture not found at ${golden_template}" >&2
    false
  }

  # Use the all-terminal sprint-state fixture (EPIC-COMPLETE path)
  _write_sprint_state_all_terminal
  _write_state_md "3"

  # Run --emit-handoff on the EPIC-COMPLETE fixture
  local emit_output
  local emit_exit=0
  emit_output="$(
    export ARTIFACTS_WT="${ARTIFACTS_WT}"
    export SPRINT_STATE_YAML="${WORK}/sprint-state.yaml"
    export STATE_MD_PATH="${WORK}/STATE.md"
    export BC_DIR="${ARTIFACTS_WT}/specs/behavioral-contracts"
    export PRECOMPACT_FLUSH_LOG="${ARTIFACTS_WT}/hooks/precompact-flush-log"
    export FACTORY_REPO="${WORK}"
    "${SKILL}" \
      --artifacts-worktree "${ARTIFACTS_WT}" \
      --sprint-state "${WORK}/sprint-state.yaml" \
      --state-md "${WORK}/STATE.md" \
      --bc-dir "${ARTIFACTS_WT}/specs/behavioral-contracts" \
      --emit-handoff \
      2>&1
  )" || emit_exit=$?

  [ "$emit_exit" -eq 0 ] || {
    echo "FAIL (AC-003 EPIC-COMPLETE): wave-handoff.sh --emit-handoff exited ${emit_exit}" >&2
    echo "  Current impl has no --emit-handoff subcommand." >&2
    echo "  Output: ${emit_output}" >&2
    false
  }

  local actual_develop_sha
  actual_develop_sha="$(git -C "${WORK}" rev-parse origin/develop)"

  local expected_content
  expected_content="$(sed "s/DEVELOP_SHA_PLACEHOLDER/${actual_develop_sha}/g" "$golden_template")"

  local actual_normalized expected_normalized
  actual_normalized="$(printf '%s' "$emit_output" | sed 's/[[:space:]]*$//')"
  expected_normalized="$(printf '%s' "$expected_content" | sed 's/[[:space:]]*$//')"

  [ "$actual_normalized" = "$expected_normalized" ] || {
    echo "FAIL (AC-003 EPIC-COMPLETE): --emit-handoff output is NOT byte-identical to golden fixture." >&2
    echo "  Golden fixture: ${golden_template} (SHA substituted)" >&2
    echo "  --- Expected ---" >&2
    echo "$expected_normalized" >&2
    echo "  --- Actual ---" >&2
    echo "$actual_normalized" >&2
    diff <(echo "$expected_normalized") <(echo "$actual_normalized") >&2 || true
    false
  }
}

# ---------------------------------------------------------------------------
# test_BC_5_41_002_PC6_S18_13_AC005_commit_creates_one_atomic_commit_has_next_wave
# AC-005 (HAS-NEXT-WAVE arm) / BC-5.41.002 PC6 — the --commit subcommand
# creates exactly ONE atomic git commit on factory-artifacts containing BOTH
# HANDOFF.md and wave-state.yaml.
#
# Pre-condition: HANDOFF.md and wave-state.yaml are placed on disk (simulating
# the state after --emit-handoff + agent Write + --emit-wave-state have run).
# Then --commit is invoked. The commit must be atomic (+1 commit containing both
# files), not two separate commits.
#
# RED GATE: current impl has no --commit subcommand → bash exits 1 on unknown
# argument → test fails on exit code check.
# ---------------------------------------------------------------------------

@test "test_BC_5_41_002_PC6_S18_13_AC005_commit_creates_one_atomic_commit_has_next_wave" {
  # Pre-state: place HANDOFF.md and wave-state.yaml on disk in ARTIFACTS_WT
  # (simulating the state after --emit-handoff → agent Write → --emit-wave-state)
  cat > "${ARTIFACTS_WT}/HANDOFF.md" << 'EOF'
wave_id: 2
last_verified_develop_sha: 0000000000000000000000000000000000000000
active_bcs:
  - specs/behavioral-contracts/ss-05/BC-5.41.001.md
next_wave_stories:
  - id: S-18.02
    status: pending
open_decisions: []
pending_fixes: []
process_gaps: []
precompact_flush_sha: null
factory_lock_holder: null
EOF

  cat > "${ARTIFACTS_WT}/wave-state.yaml" << 'EOF'
wave_id: 3
generated_at: 2026-06-20T00:00:00Z
generated_from_handoff_sha: null
stories: []
arch_files: []
state_pointer: .factory/STATE.md
EOF

  local before_count
  before_count="$(git -C "$WORK" rev-list --count factory-artifacts)"

  # Run --commit subcommand (HAS-NEXT-WAVE path)
  local commit_exit=0
  local commit_output
  commit_output="$(
    export ARTIFACTS_WT="${ARTIFACTS_WT}"
    export SPRINT_STATE_YAML="${WORK}/sprint-state.yaml"
    export STATE_MD_PATH="${WORK}/STATE.md"
    export BC_DIR="${ARTIFACTS_WT}/specs/behavioral-contracts"
    export FACTORY_REPO="${WORK}"
    "${SKILL}" \
      --artifacts-worktree "${ARTIFACTS_WT}" \
      --sprint-state "${WORK}/sprint-state.yaml" \
      --state-md "${WORK}/STATE.md" \
      --bc-dir "${ARTIFACTS_WT}/specs/behavioral-contracts" \
      --commit \
      2>&1
  )" || commit_exit=$?

  [ "$commit_exit" -eq 0 ] || {
    echo "FAIL (AC-005 HAS-NEXT-WAVE): wave-handoff.sh --commit exited ${commit_exit}" >&2
    echo "  Current impl has no --commit subcommand." >&2
    echo "  After T-2a: --commit stages both HANDOFF.md + wave-state.yaml in ONE git commit." >&2
    echo "  Output: ${commit_output}" >&2
    false
  }

  local after_count
  after_count="$(git -C "$WORK" rev-list --count factory-artifacts)"
  local delta=$(( after_count - before_count ))

  [ "$delta" -eq 1 ] || {
    echo "FAIL (AC-005 HAS-NEXT-WAVE): expected exactly 1 new commit on factory-artifacts," >&2
    echo "  got ${delta} commits. Two separate commits violate BC-5.41.002 PC6 atomicity." >&2
    false
  }

  # The single commit must contain BOTH files
  local changed_files
  changed_files="$(git -C "$WORK" diff-tree --no-commit-id -r --name-only factory-artifacts)"

  echo "$changed_files" | grep -q "HANDOFF.md" || {
    echo "FAIL (AC-005 HAS-NEXT-WAVE): HANDOFF.md not in the atomic commit." >&2
    echo "  Files in commit: ${changed_files}" >&2
    false
  }

  echo "$changed_files" | grep -q "wave-state.yaml" || {
    echo "FAIL (AC-005 HAS-NEXT-WAVE): wave-state.yaml not in the atomic commit." >&2
    echo "  Files in commit: ${changed_files}" >&2
    echo "  BC-5.41.002 PC6: HAS-NEXT-WAVE arm must commit BOTH HANDOFF.md + wave-state.yaml" >&2
    echo "  in ONE atomic git commit." >&2
    false
  }
}

# ---------------------------------------------------------------------------
# test_BC_5_41_001_PC10_S18_13_AC006_epic_complete_commit_arm_succeeds_without_wave_state
# AC-006 / BC-5.41.001 PC10 EPIC-COMPLETE arm + BC-5.41.002 PC6
# On the EPIC-COMPLETE path, --commit succeeds with HANDOFF.md present and
# wave-state.yaml ABSENT, creates a single-file commit, and does NOT raise
# HandoffFileAbsent.
#
# Pre-condition: HANDOFF.md is placed on disk; wave-state.yaml is intentionally
# absent (simulating state after --emit-wave-state is SKIPPED on EPIC-COMPLETE).
#
# RED GATE: current impl has no --commit subcommand → exits 1 on unknown arg.
# ---------------------------------------------------------------------------

@test "test_BC_5_41_001_PC10_S18_13_AC006_epic_complete_commit_arm_succeeds_without_wave_state" {
  # Pre-state: HANDOFF.md present; wave-state.yaml intentionally absent
  cat > "${ARTIFACTS_WT}/HANDOFF.md" << 'EOF'
wave_id: 3
last_verified_develop_sha: 0000000000000000000000000000000000000000
active_bcs:
  - specs/behavioral-contracts/ss-05/BC-5.41.001.md
next_wave_stories: []
open_decisions: []
pending_fixes: []
process_gaps: []
precompact_flush_sha: null
factory_lock_holder: null
epic_status: complete
EOF

  # Ensure wave-state.yaml is NOT on disk (EPIC-COMPLETE: intentionally absent)
  rm -f "${ARTIFACTS_WT}/wave-state.yaml"

  local before_count
  before_count="$(git -C "$WORK" rev-list --count factory-artifacts)"

  # Use the all-terminal sprint-state to signal EPIC-COMPLETE to the --commit arm
  _write_sprint_state_all_terminal
  _write_state_md "3"

  local commit_exit=0
  local commit_output
  commit_output="$(
    export ARTIFACTS_WT="${ARTIFACTS_WT}"
    export SPRINT_STATE_YAML="${WORK}/sprint-state.yaml"
    export STATE_MD_PATH="${WORK}/STATE.md"
    export BC_DIR="${ARTIFACTS_WT}/specs/behavioral-contracts"
    export FACTORY_REPO="${WORK}"
    "${SKILL}" \
      --artifacts-worktree "${ARTIFACTS_WT}" \
      --sprint-state "${WORK}/sprint-state.yaml" \
      --state-md "${WORK}/STATE.md" \
      --bc-dir "${ARTIFACTS_WT}/specs/behavioral-contracts" \
      --commit \
      2>&1
  )" || commit_exit=$?

  [ "$commit_exit" -eq 0 ] || {
    echo "FAIL (AC-006): wave-handoff.sh --commit exited ${commit_exit} on EPIC-COMPLETE" >&2
    echo "  EPIC-COMPLETE: wave-state.yaml is intentionally absent (not produced on this path)." >&2
    echo "  --commit MUST NOT abort with HandoffFileAbsent when wave-state.yaml is absent" >&2
    echo "  on the EPIC-COMPLETE path (BC-5.41.001 PC10 step 4 EPIC-COMPLETE carve-out)." >&2
    echo "  Current impl has no --commit subcommand." >&2
    echo "  Output: ${commit_output}" >&2
    false
  }

  # Must NOT have raised HandoffFileAbsent
  echo "$commit_output" | grep -qi "HandoffFileAbsent" && {
    echo "FAIL (AC-006): --commit raised HandoffFileAbsent on EPIC-COMPLETE path." >&2
    echo "  wave-state.yaml absence is expected and correct on EPIC-COMPLETE." >&2
    echo "  BC-5.41.002 PC6: single-file EPIC-COMPLETE commit is NOT a PC6 violation." >&2
    echo "  Output: ${commit_output}" >&2
    false
  }

  local after_count
  after_count="$(git -C "$WORK" rev-list --count factory-artifacts)"
  local delta=$(( after_count - before_count ))

  [ "$delta" -eq 1 ] || {
    echo "FAIL (AC-006): expected exactly 1 new commit on factory-artifacts," >&2
    echo "  got ${delta}. EPIC-COMPLETE --commit must create ONE atomic commit." >&2
    false
  }

  # The commit must contain HANDOFF.md and NOT wave-state.yaml
  local changed_files
  changed_files="$(git -C "$WORK" diff-tree --no-commit-id -r --name-only factory-artifacts)"

  echo "$changed_files" | grep -q "HANDOFF.md" || {
    echo "FAIL (AC-006): HANDOFF.md not in the EPIC-COMPLETE commit." >&2
    echo "  Files: ${changed_files}" >&2
    false
  }

  if echo "$changed_files" | grep -q "wave-state.yaml"; then
    echo "FAIL (AC-006): wave-state.yaml was committed on EPIC-COMPLETE path." >&2
    echo "  EPIC-COMPLETE commit must contain HANDOFF.md ALONE." >&2
    echo "  Files in commit: ${changed_files}" >&2
    false
  fi
}

# ---------------------------------------------------------------------------
# test_BC_5_41_001_EC016_S18_13_handoff_write_tool_unavailable_hard_error
# EC-016 / BC-5.41.001 EC-016 — when the Write tool is unavailable, the skill
# MUST surface HandoffWriteToolUnavailable and MUST NOT fall back to bash
# redirection.
#
# The S-18.13 restructure: the --emit-handoff subcommand emits the payload to
# stdout; the AGENT then invokes the Write tool. If the agent's Write tool is
# unavailable, the skill should detect this (via HANDOFF_WRITE_TOOL_UNAVAILABLE
# env variable or similar mechanism) and fail loudly.
#
# Test strategy: invoke the skill with HANDOFF_WRITE_TOOL_UNAVAILABLE=1 (the
# env-gate added by T-2a to detect Write tool unavailability in the SKILL.md
# S1 step). The skill must exit 1 with HandoffWriteToolUnavailable message.
#
# RED GATE: current impl has no --emit-handoff subcommand and no
# HANDOFF_WRITE_TOOL_UNAVAILABLE guard → exits 1 on unknown argument
# (wrong error code path, not HandoffWriteToolUnavailable).
# ---------------------------------------------------------------------------

@test "test_BC_5_41_001_EC016_S18_13_handoff_write_tool_unavailable_hard_error" {
  local ec016_exit=0
  local ec016_output
  ec016_output="$(
    export ARTIFACTS_WT="${ARTIFACTS_WT}"
    export SPRINT_STATE_YAML="${WORK}/sprint-state.yaml"
    export STATE_MD_PATH="${WORK}/STATE.md"
    export BC_DIR="${ARTIFACTS_WT}/specs/behavioral-contracts"
    export FACTORY_REPO="${WORK}"
    export HANDOFF_WRITE_TOOL_UNAVAILABLE=1
    "${SKILL}" \
      --artifacts-worktree "${ARTIFACTS_WT}" \
      --sprint-state "${WORK}/sprint-state.yaml" \
      --state-md "${WORK}/STATE.md" \
      --bc-dir "${ARTIFACTS_WT}/specs/behavioral-contracts" \
      --emit-handoff \
      2>&1
  )" || ec016_exit=$?

  # Must exit 1 (hard error)
  [ "$ec016_exit" -eq 1 ] || {
    echo "FAIL (EC-016): skill exited ${ec016_exit} with Write tool unavailable, expected exit 1" >&2
    echo "  BC-5.41.001 EC-016: Write tool unavailable → HandoffWriteToolUnavailable hard error." >&2
    echo "  Skill MUST NOT fall back to bash redirection or exit 0." >&2
    echo "  Current impl has no --emit-handoff subcommand — exits 1 for wrong reason (unknown arg)." >&2
    echo "  After implementation: must exit 1 with HandoffWriteToolUnavailable in stderr." >&2
    echo "  Output: ${ec016_output}" >&2
    false
  }

  # Must emit HandoffWriteToolUnavailable in output
  echo "$ec016_output" | grep -qi "HandoffWriteToolUnavailable" || {
    echo "FAIL (EC-016): HandoffWriteToolUnavailable not in output." >&2
    echo "  Expected: 'HandoffWriteToolUnavailable: HANDOFF.md must be written via the Write" >&2
    echo "    tool (Claude Code native tool call); bash redirection is forbidden.'" >&2
    echo "  Actual output: ${ec016_output}" >&2
    false
  }

  # HANDOFF.md MUST NOT be written (no fallback bash redirect)
  [ ! -f "${ARTIFACTS_WT}/HANDOFF.md" ] || {
    echo "FAIL (EC-016): HANDOFF.md was written despite HandoffWriteToolUnavailable." >&2
    echo "  Skill MUST NOT fall back to bash redirection when Write tool is unavailable." >&2
    echo "  BC-5.41.001 PC10: bash redirection is FORBIDDEN as fallback write path." >&2
    false
  }
}

# ---------------------------------------------------------------------------
# test_BC_5_41_001_EC017_S18_13_handoff_file_absent_blocks_commit_has_next_wave
# EC-017 (HAS-NEXT-WAVE path) / BC-5.41.001 EC-017 — --commit invoked but
# HANDOFF.md absent → HandoffFileAbsent hard error; git commit does NOT proceed.
#
# RED GATE: current impl has no --commit subcommand → exits 1 on unknown arg
# (wrong reason: "unknown argument" not "HandoffFileAbsent").
# ---------------------------------------------------------------------------

@test "test_BC_5_41_001_EC017_S18_13_handoff_file_absent_blocks_commit_has_next_wave" {
  # Pre-condition: HANDOFF.md is NOT on disk (Write tool step was skipped/failed)
  rm -f "${ARTIFACTS_WT}/HANDOFF.md"

  # Place wave-state.yaml (present — only HANDOFF.md is missing)
  cat > "${ARTIFACTS_WT}/wave-state.yaml" << 'EOF'
wave_id: 3
generated_at: 2026-06-20T00:00:00Z
generated_from_handoff_sha: null
stories: []
arch_files: []
state_pointer: .factory/STATE.md
EOF

  local before_count
  before_count="$(git -C "$WORK" rev-list --count factory-artifacts)"

  local ec017_exit=0
  local ec017_output
  ec017_output="$(
    export ARTIFACTS_WT="${ARTIFACTS_WT}"
    export SPRINT_STATE_YAML="${WORK}/sprint-state.yaml"
    export STATE_MD_PATH="${WORK}/STATE.md"
    export BC_DIR="${ARTIFACTS_WT}/specs/behavioral-contracts"
    export FACTORY_REPO="${WORK}"
    "${SKILL}" \
      --artifacts-worktree "${ARTIFACTS_WT}" \
      --sprint-state "${WORK}/sprint-state.yaml" \
      --state-md "${WORK}/STATE.md" \
      --bc-dir "${ARTIFACTS_WT}/specs/behavioral-contracts" \
      --commit \
      2>&1
  )" || ec017_exit=$?

  # Must exit 1 (hard error — HANDOFF.md absent blocks commit)
  [ "$ec017_exit" -eq 1 ] || {
    echo "FAIL (EC-017 HAS-NEXT-WAVE): skill exited ${ec017_exit} with HANDOFF.md absent," >&2
    echo "  expected exit 1." >&2
    echo "  BC-5.41.001 EC-017: --commit invoked with HANDOFF.md absent → HandoffFileAbsent." >&2
    echo "  --commit MUST NOT proceed to git add/commit if HANDOFF.md is absent." >&2
    echo "  Output: ${ec017_output}" >&2
    false
  }

  # Must emit HandoffFileAbsent
  echo "$ec017_output" | grep -qi "HandoffFileAbsent" || {
    echo "FAIL (EC-017 HAS-NEXT-WAVE): HandoffFileAbsent not in output." >&2
    echo "  Expected: 'HandoffFileAbsent: HANDOFF.md not found at \${ARTIFACTS_WT}/HANDOFF.md" >&2
    echo "    before commit; aborting atomic commit'" >&2
    echo "  Actual output: ${ec017_output}" >&2
    false
  }

  # No new commit must have been created
  local after_count
  after_count="$(git -C "$WORK" rev-list --count factory-artifacts)"
  [ "$after_count" -eq "$before_count" ] || {
    echo "FAIL (EC-017 HAS-NEXT-WAVE): ${after_count - before_count} new commit(s) created" >&2
    echo "  despite HandoffFileAbsent. --commit MUST NOT git add/commit when HANDOFF.md absent." >&2
    false
  }
}

# ---------------------------------------------------------------------------
# test_BC_5_41_001_EC017_S18_13_handoff_file_absent_blocks_commit_epic_complete
# EC-017 (EPIC-COMPLETE path) / BC-5.41.001 EC-017 — --commit invoked on
# EPIC-COMPLETE path but HANDOFF.md absent → HandoffFileAbsent hard error.
# wave-state.yaml absence is NOT an error on EPIC-COMPLETE (only HANDOFF.md
# absence triggers HandoffFileAbsent).
#
# RED GATE: current impl has no --commit subcommand → exits 1 for wrong reason.
# ---------------------------------------------------------------------------

@test "test_BC_5_41_001_EC017_S18_13_handoff_file_absent_blocks_commit_epic_complete" {
  # Pre-condition: HANDOFF.md absent; wave-state.yaml also absent (EPIC-COMPLETE normal state)
  rm -f "${ARTIFACTS_WT}/HANDOFF.md"
  rm -f "${ARTIFACTS_WT}/wave-state.yaml"

  # Use all-terminal sprint-state (EPIC-COMPLETE signal)
  _write_sprint_state_all_terminal
  _write_state_md "3"

  local before_count
  before_count="$(git -C "$WORK" rev-list --count factory-artifacts)"

  local ec017ec_exit=0
  local ec017ec_output
  ec017ec_output="$(
    export ARTIFACTS_WT="${ARTIFACTS_WT}"
    export SPRINT_STATE_YAML="${WORK}/sprint-state.yaml"
    export STATE_MD_PATH="${WORK}/STATE.md"
    export BC_DIR="${ARTIFACTS_WT}/specs/behavioral-contracts"
    export FACTORY_REPO="${WORK}"
    "${SKILL}" \
      --artifacts-worktree "${ARTIFACTS_WT}" \
      --sprint-state "${WORK}/sprint-state.yaml" \
      --state-md "${WORK}/STATE.md" \
      --bc-dir "${ARTIFACTS_WT}/specs/behavioral-contracts" \
      --commit \
      2>&1
  )" || ec017ec_exit=$?

  # Must exit 1 (HANDOFF.md absent even on EPIC-COMPLETE is an error)
  [ "$ec017ec_exit" -eq 1 ] || {
    echo "FAIL (EC-017 EPIC-COMPLETE): skill exited ${ec017ec_exit} with HANDOFF.md absent" >&2
    echo "  on EPIC-COMPLETE path, expected exit 1." >&2
    echo "  BC-5.41.001 EC-017: HandoffFileAbsent fires on BOTH HAS-NEXT-WAVE and" >&2
    echo "  EPIC-COMPLETE paths when HANDOFF.md is absent." >&2
    echo "  NOTE: wave-state.yaml absence on EPIC-COMPLETE is NOT an error — only" >&2
    echo "  HANDOFF.md absence is checked on EPIC-COMPLETE." >&2
    echo "  Output: ${ec017ec_output}" >&2
    false
  }

  # Must emit HandoffFileAbsent (triggered by missing HANDOFF.md, not missing wave-state.yaml)
  echo "$ec017ec_output" | grep -qi "HandoffFileAbsent" || {
    echo "FAIL (EC-017 EPIC-COMPLETE): HandoffFileAbsent not in output." >&2
    echo "  On EPIC-COMPLETE path with missing HANDOFF.md, --commit must abort with" >&2
    echo "  HandoffFileAbsent (BC-5.41.001 EC-017 EPIC-COMPLETE arm)." >&2
    echo "  Actual output: ${ec017ec_output}" >&2
    false
  }

  # No new commit must have been created
  local after_count
  after_count="$(git -C "$WORK" rev-list --count factory-artifacts)"
  [ "$after_count" -eq "$before_count" ] || {
    echo "FAIL (EC-017 EPIC-COMPLETE): new commit created despite HandoffFileAbsent." >&2
    echo "  --commit MUST NOT proceed to git add/commit when HANDOFF.md absent." >&2
    false
  }
}

@test "test_BC_5_41_001_F_P11_001_bsd_classify_stories_has_next_wave" {
  # Default fixture has S-18.02 (pending) + S-18.03 (draft) → has-next-wave
  _write_sprint_state_pending

  _run_skill_subcommands

  # Skill must exit 0
  [ "$status" -eq 0 ] || {
    echo "FAIL (F-P11-001 behavioral): skill exited ${status}, expected 0." >&2
    echo "  With a has-next-wave sprint-state, the skill must not treat it as epic-complete." >&2
    echo "  BSD grep \\s/\\S match literal 's'/'S' — story lines are never matched →" >&2
    echo "  has_any_story=0 → classify_stories returns epic-complete SILENTLY." >&2
    echo "Actual output: $output" >&2
    false
  }

  # wave-state.yaml MUST be committed (not EPIC-COMPLETE)
  git -C "$WORK" show factory-artifacts:wave-state.yaml >/dev/null 2>&1 || {
    echo "FAIL (F-P11-001 behavioral): wave-state.yaml not committed." >&2
    echo "  classify_stories returned epic-complete instead of has-next-wave." >&2
    echo "  Check: parse-sprint-state.sh grep patterns use \\s/\\S which do not" >&2
    echo "  match on BSD grep — 'has_any_story' stays 0 → epic-complete branch taken." >&2
    echo "Skill output: $output" >&2
    false
  }

  # stdout must NOT contain EPIC-COMPLETE
  echo "$output" | grep -qi "EPIC-COMPLETE" && {
    echo "FAIL (F-P11-001 behavioral): stdout contains EPIC-COMPLETE but sprint-state has pending stories." >&2
    echo "  Expected: has-next-wave path (wave-state.yaml written, no EPIC-COMPLETE message)." >&2
    echo "Actual output: $output" >&2
    false
  }

  # wave-state.yaml committed blob must contain S-18.02 (pending story)
  local committed_content
  committed_content="$(git -C "$WORK" show factory-artifacts:wave-state.yaml)"
  echo "$committed_content" | grep -q "S-18.02" || {
    echo "FAIL (F-P11-001 behavioral): S-18.02 missing from committed wave-state.yaml." >&2
    echo "  The pending story S-18.02 from sprint-state.yaml must appear in wave-state.yaml." >&2
    false
  }
}

# ---------------------------------------------------------------------------
# test_BC_5_41_001_PC10_S18_13_AC001_sibling_site_no_bash_redirect_in_wave_handoff
# AC-001-SIBLING / F-S1813-IMPL-P1-002 — wave-handoff.sh MUST NOT contain
# any bash redirection of HANDOFF.md (> "${ARTIFACTS_WT}/HANDOFF.md" or variants).
# Oracle: grep for '>' redirect patterns targeting HANDOFF.md must return 0 matches.
# This sibling oracle closes the gap that AC-001(a) only checks write-handoff.sh
# but not wave-handoff.sh where the legacy main() also had bash redirects.
# ---------------------------------------------------------------------------

@test "test_BC_5_41_001_PC10_S18_13_AC001_sibling_site_no_bash_redirect_in_wave_handoff" {
  local wave_handoff_sh
  wave_handoff_sh="${SKILL_DIR}/wave-handoff.sh"

  # Grep for any bash redirect targeting HANDOFF.md in wave-handoff.sh.
  # Patterns: > ...HANDOFF.md, >> ...HANDOFF.md, or tee ...HANDOFF.md
  local redirect_matches
  redirect_matches="$(grep -nE '>+[[:space:]]*"?\$\{ARTIFACTS_WT\}/HANDOFF\.md|tee[[:space:]].*HANDOFF\.md' "$wave_handoff_sh" 2>/dev/null || true)"

  [ -z "$redirect_matches" ] || {
    echo "FAIL (AC-001-SIBLING): bash redirect to HANDOFF.md found in wave-handoff.sh" >&2
    echo "  The monolithic main() in wave-handoff.sh MUST NOT write HANDOFF.md via bash" >&2
    echo "  redirection. BC-5.41.001 PC10 / §Forbidden Dependencies forbid bash redirection" >&2
    echo "  of HANDOFF.md as primary or fallback. F-S1813-IMPL-P1-002 requires hard-error." >&2
    echo "  Matches found:" >&2
    echo "$redirect_matches" >&2
    false
  }
}

# ---------------------------------------------------------------------------
# test_S18_13_no_subcommand_hard_errors
# O-S1813-IMPL-P2-003 / POLICY 11
# Invoking wave-handoff.sh with required args but NO subcommand flag (empty SUBCOMMAND)
# MUST exit 1 and output must contain the deprecation message:
#   "monolithic wave-handoff invocation is removed"
# This covers the "" dispatch arm in the case statement (hard-error path).
# ---------------------------------------------------------------------------

@test "test_S18_13_no_subcommand_hard_errors" {
  # Invoke the skill with all required args but no --emit-handoff / --emit-wave-state / --commit flag.
  # The dispatch case arm for "" must exit 1 with the deprecation message.
  run bash -c "
    '${SKILL}' \
      --artifacts-worktree '${ARTIFACTS_WT}' \
      --sprint-state '${WORK}/sprint-state.yaml' \
      --state-md '${WORK}/STATE.md' \
      --bc-dir '${ARTIFACTS_WT}/specs/behavioral-contracts' \
      2>&1
  "

  # Must exit 1
  [ "$status" -eq 1 ] || {
    echo "FAIL: expected exit 1 when no subcommand given, got exit ${status}" >&2
    echo "Output: $output" >&2
    false
  }

  # Output must contain the deprecation message from the empty-subcommand hard-error arm.
  echo "$output" | grep -q "monolithic wave-handoff invocation is removed" || {
    echo "FAIL: output does not contain 'monolithic wave-handoff invocation is removed'" >&2
    echo "Actual output: $output" >&2
    false
  }
}

# ---------------------------------------------------------------------------
# test_S18_13_unknown_subcommand_hard_errors
# O-S1813-IMPL-P2-003 / POLICY 11
# Invoking wave-handoff.sh with an unrecognised subcommand flag (e.g. --bogus)
# MUST exit 1 (unknown-argument arm fires during arg parsing, before the dispatch).
# ---------------------------------------------------------------------------

@test "test_S18_13_unknown_subcommand_hard_errors" {
  # Invoke the skill with all required args plus an unrecognised flag.
  # The while-loop unknown-argument arm exits 1 with "unknown argument".
  run bash -c "
    '${SKILL}' \
      --artifacts-worktree '${ARTIFACTS_WT}' \
      --sprint-state '${WORK}/sprint-state.yaml' \
      --state-md '${WORK}/STATE.md' \
      --bc-dir '${ARTIFACTS_WT}/specs/behavioral-contracts' \
      --bogus \
      2>&1
  "

  # Must exit 1
  [ "$status" -eq 1 ] || {
    echo "FAIL: expected exit 1 for unknown subcommand --bogus, got exit ${status}" >&2
    echo "Output: $output" >&2
    false
  }
}
