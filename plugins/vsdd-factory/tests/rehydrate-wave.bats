#!/usr/bin/env bats
# rehydrate-wave.bats — Red Gate tests for the rehydrate-wave skill (S-18.03)
#
# Story:   S-18.03 v1.5 — rehydrate-wave skill — git-sourced scoped rehydration + wave-reset SKILL.md
# BC:      BC-6.24.001 v1.10 (rehydrate-wave skill loads wave-state.yaml and injects
#                              exactly the listed specs into session context — no stale prior-wave specs)
# VP:      VP-088 v1.1 (rehydrate-wave Reads wave-state.yaml From Git (Not Working Tree),
#                       Injects Exactly Listed Specs, Blocks on Missing Manifest, No RAG Fallback)
#
# RED GATE discipline: ALL 9 tests MUST FAIL before any SKILL.md or rehydrate-wave.sh
# implementation exists. Skill is expected at:
#   plugins/vsdd-factory/skills/rehydrate-wave/rehydrate-wave.sh
# Until that file exists, _require_skill skips with an actionable message.
#
# CRITICAL INVARIANT (VP-088 §1 / BC-6.24.001 Inv1): The skill reads wave-state.yaml
# EXCLUSIVELY via `git show factory-artifacts:wave-state.yaml` — never from the working tree.
# Tests MUST set up a real git fixture with a factory-artifacts branch carrying the fixture
# content, matching the wave-handoff.bats ADR-027 fixture discipline.
#
# INJECTED_FILE_COUNT sentinel discipline (VP-088 §2 PC2-SIGNAL):
# All count assertions MUST use the machine-stable sentinel:
#   INJECTED_COUNT=$(output | grep '^INJECTED_FILE_COUNT=' | cut -d= -f2)
# NEVER grep -c "^  - " — presentation-coupled; false-green on list reformatting.
#
# Test naming follows BC-based pattern per VSDD test-writer discipline:
#   test_BC_S_SS_NNN_xxx()
# Mapped to BC-6.24.001 as: test_BC_6_24_001_xxx()
# Story-specific aliases are also used where the story spec prescribes exact test names.
#
# No jq dependency. No Python. POSIX bash + awk + grep + git.

# ---------------------------------------------------------------------------
# Setup / teardown
# ---------------------------------------------------------------------------

setup() {
  PLUGIN_ROOT="$(cd "${BATS_TEST_DIRNAME}/.." && pwd)"
  SKILL_DIR="$PLUGIN_ROOT/skills/rehydrate-wave"
  SKILL="$SKILL_DIR/rehydrate-wave.sh"

  # Create a hermetic git repo in a temp directory.
  # Architecture mirrors wave-handoff.bats ADR-027 fixture discipline:
  #   WORK/               — the "main" working repo (non-bare)
  #   WORK/factory-wt/   — git worktree checked out on the factory-artifacts orphan branch
  #
  # The skill receives:
  #   --artifacts-worktree $ARTIFACTS_WT  (the checked-out factory-artifacts worktree)
  #   --repo $WORK                        (the main repo dir; for git show factory-artifacts:)
  #
  # wave-state.yaml is injected into the factory-artifacts branch via git commit,
  # so `git show factory-artifacts:wave-state.yaml` reads the COMMITTED blob.
  WORK="$(mktemp -d)"
  ARTIFACTS_WT="$WORK/factory-wt"

  git -C "$WORK" init -q -b feature-test
  git -C "$WORK" config user.email "test@example.com"
  git -C "$WORK" config user.name "Test"
  git -C "$WORK" config commit.gpgsign false

  # Create a root commit on the main branch so HEAD is valid
  echo "feature-test root" > "$WORK/root.txt"
  git -C "$WORK" add root.txt
  git -C "$WORK" commit -q -m "feature-test root"

  # Create the factory-artifacts orphan branch
  local saved_branch
  saved_branch="$(git -C "$WORK" branch --show-current)"
  git -C "$WORK" checkout --orphan factory-artifacts -q
  git -C "$WORK" rm -rf . -q 2>/dev/null || true
  echo "factory-artifacts root" > "$WORK/.gitkeep"
  git -C "$WORK" add .gitkeep
  git -C "$WORK" commit -q -m "factory-artifacts init"
  git -C "$WORK" checkout -q "$saved_branch"

  # Add a linked worktree for factory-artifacts
  mkdir -p "$ARTIFACTS_WT"
  git -C "$WORK" worktree add -q "$ARTIFACTS_WT" factory-artifacts
}

teardown() {
  git -C "$WORK" worktree remove --force "$ARTIFACTS_WT" 2>/dev/null || true
  rm -rf "$WORK"
}

# ---------------------------------------------------------------------------
# Preflight helpers
# ---------------------------------------------------------------------------

# Skip if the skill shell script has not been implemented yet.
# This produces the RED GATE "skip" (not a pass) until the implementer creates
# plugins/vsdd-factory/skills/rehydrate-wave/rehydrate-wave.sh
_require_skill() {
  if [ ! -x "$SKILL" ]; then
    skip "rehydrate-wave.sh not yet implemented — S-18.03 T-2 task (implementer): create plugins/vsdd-factory/skills/rehydrate-wave/rehydrate-wave.sh"
  fi
}

# ---------------------------------------------------------------------------
# Git fixture helpers
# ---------------------------------------------------------------------------

# Commit a file to factory-artifacts branch via the ARTIFACTS_WT worktree.
# Usage: _commit_to_factory_artifacts "filename" "content"
_commit_to_factory_artifacts() {
  local filename="$1"
  local content="$2"
  printf '%s\n' "$content" > "$ARTIFACTS_WT/$filename"
  git -C "$ARTIFACTS_WT" add "$filename"
  git -C "$ARTIFACTS_WT" -c user.email="test@example.com" -c user.name="Test" \
    -c commit.gpgsign=false commit -q -m "fixture: add $filename"
}

# Run the rehydrate-wave skill with standard arguments.
# Sets $status and $output (combined stdout+stderr via `run bash -c`).
# The skill must accept:
#   --repo <main-repo-dir>        (for git show factory-artifacts:wave-state.yaml)
#   --artifacts-worktree <path>   (for working-tree fallback detection checks)
# Or any equivalent invocation pattern the implementer chooses.
# Tests use _run_skill to get combined output for assertions.
_run_skill() {
  run bash -c "'$SKILL' \
    --repo '$WORK' \
    --artifacts-worktree '$ARTIFACTS_WT' \
    2>&1"
}

# Extract INJECTED_FILE_COUNT sentinel value from skill output (VP-088 §2 PC2-SIGNAL).
# Usage: count=$(_extract_injected_count "$output")
_extract_injected_count() {
  local skill_output="$1"
  printf '%s\n' "$skill_output" | grep '^INJECTED_FILE_COUNT=' | cut -d= -f2
}

# ---------------------------------------------------------------------------
# test_rehydrate_wave_reads_git_not_working_tree
# AC-001 / BC-6.24.001 postcondition 1 / VP-088 §1
#
# Setup: factory-artifacts:wave-state.yaml has stories: [{id: S-18.02, spec_files: [real.md]}]
#        working-tree .factory/wave-state.yaml has stories: [{id: S-18.STALE}]
# Assert: injected context names real.md; does NOT name S-18.STALE
#
# Red Gate: SKILL.md absent → _require_skill skips (skip != pass); OR stub reads
# working tree → output contains S-18.STALE → refute assertion fails.
# ---------------------------------------------------------------------------

@test "test_rehydrate_wave_reads_git_not_working_tree" {
  _require_skill

  # Plant the AUTHORITATIVE wave-state.yaml on factory-artifacts branch (git blob)
  local git_content
  git_content="$(cat <<'YAML'
wave_id: 2
generated_at: "2026-06-16T00:00:00Z"
generated_from_handoff_sha: "aabbccddeeff00112233445566778899aabbccdd"
state_pointer: ".factory/STATE.md"
arch_files: []
stories:
  - id: S-18.02
    spec_files:
      - real.md
YAML
)"
  _commit_to_factory_artifacts "wave-state.yaml" "$git_content"

  # Plant a STALE working-tree wave-state.yaml (must be ignored by skill)
  # This simulates a locally-edited or stale copy at .factory/wave-state.yaml
  mkdir -p "$WORK/.factory"
  cat > "$WORK/.factory/wave-state.yaml" <<'YAML'
wave_id: 1
state_pointer: ".factory/STATE.md"
arch_files: []
stories:
  - id: S-18.STALE
    spec_files:
      - stale-spec.md
YAML

  _run_skill

  # Assert: injected context names real.md (from the git-committed branch version)
  printf '%s\n' "$output" | grep -qF "real.md" || {
    echo "FAIL (AC-001 / BC-6.24.001 postcondition 1): 'real.md' from factory-artifacts branch"
    echo "  not found in skill output. Skill must read wave-state.yaml via"
    echo "  'git show factory-artifacts:wave-state.yaml', not from working tree."
    echo "Actual output: $output"
    false
  }

  # Refute: stale working-tree spec ID must NOT appear in output
  printf '%s\n' "$output" | grep -qF "S-18.STALE" && {
    echo "FAIL (AC-001 / BC-6.24.001 Inv1): 'S-18.STALE' from working-tree wave-state.yaml"
    echo "  appeared in skill output. Skill must ignore working-tree copy and read"
    echo "  exclusively from factory-artifacts branch via git show."
    echo "Actual output: $output"
    false
  } || true
}

# ---------------------------------------------------------------------------
# test_rehydrate_wave_injects_exactly_union_of_listed_specs
# AC-002 / BC-6.24.001 postcondition 2 / VP-088 §2 PC2 + PC2-SIGNAL
#
# Setup: wave-state.yaml with stories: [{id: S-18.02, spec_files: [A.md, B.md]}],
#        arch_files: [C.md, D.md, E.md], state_pointer: ".factory/STATE.md"
# Assert: INJECTED_FILE_COUNT=6 sentinel present in stdout;
#         each of A.md/B.md/C.md/D.md/E.md/.factory/STATE.md named in output;
#         operator confirmation prompt shown.
#
# Red Gate: skill absent → skip; OR stub exits non-zero OR INJECTED_FILE_COUNT
# missing/wrong → assertion fails for the RIGHT reason.
# ---------------------------------------------------------------------------

@test "test_rehydrate_wave_injects_exactly_union_of_listed_specs" {
  _require_skill

  local git_content
  git_content="$(cat <<'YAML'
wave_id: 2
generated_at: "2026-06-16T00:00:00Z"
generated_from_handoff_sha: "aabbccddeeff00112233445566778899aabbccdd"
state_pointer: ".factory/STATE.md"
arch_files:
  - C.md
  - D.md
  - E.md
stories:
  - id: S-18.02
    spec_files:
      - A.md
      - B.md
YAML
)"
  _commit_to_factory_artifacts "wave-state.yaml" "$git_content"

  _run_skill

  # Assert exit success (not a hard-block)
  [ "$status" -eq 0 ] || {
    echo "FAIL (AC-002): skill exited $status (non-zero); expected 0 on valid manifest."
    echo "Actual output: $output"
    false
  }

  # Assert INJECTED_FILE_COUNT=6 via machine-stable VP-088 §2 PC2-SIGNAL sentinel.
  # MUST NOT use grep -c "^  - " — presentation-coupled; false-green on reformatting.
  local injected_count
  injected_count="$(_extract_injected_count "$output")"
  [ "$injected_count" = "6" ] || {
    echo "FAIL (AC-002 / VP-088 §2 PC2-SIGNAL): INJECTED_FILE_COUNT sentinel."
    echo "  Expected: INJECTED_FILE_COUNT=6"
    echo "  Got sentinel value: '$injected_count'"
    echo "  (A.md + B.md + C.md + D.md + E.md + .factory/STATE.md = 6 files)"
    echo "  Skill must emit 'INJECTED_FILE_COUNT=<n>' line on stdout."
    echo "  DO NOT assert count via grep -c '^  - ' — VP-088 §2 forbids this."
    echo "Actual output: $output"
    false
  }

  # Assert each expected file is named in output
  for expected_file in "A.md" "B.md" "C.md" "D.md" "E.md" ".factory/STATE.md"; do
    printf '%s\n' "$output" | grep -qF "$expected_file" || {
      echo "FAIL (AC-002 / BC-6.24.001 postcondition 2): expected file '$expected_file'"
      echo "  not found in skill output."
      echo "Actual output: $output"
      false
    }
  done

  # Assert operator confirmation prompt is shown (BC-6.24.001 postcondition 5 / Inv4).
  # F-P1-007 tightened: assert the ACTUAL Step 8 sentence — grep -qiE "(confirm|proceed|rehydrat)"
  # was near-tautological because the word "Rehydration" appears in the static header line.
  # Using grep -qF with the exact sentence ensures this fails if the prompt is removed.
  printf '%s\n' "$output" | grep -qF "Confirm rehydration:" || {
    echo "FAIL (AC-002 / BC-6.24.001 postcondition 5 + Inv4): operator confirmation prompt missing."
    echo "  Skill must pause and present a confirmation prompt after listing injected files."
    echo "  Expected to find 'Confirm rehydration:' sentence (Step 8 of rehydrate-wave.sh)."
    echo "Actual output: $output"
    false
  }
}

# ---------------------------------------------------------------------------
# test_rehydrate_wave_does_not_inject_prior_wave_specs
# AC-003 / BC-6.24.001 postcondition 3 / VP-088 §2
#
# Setup: wave-state.yaml references only [S-18.02-foo.md]; project directory
#        also contains S-17.04-bar.md (prior-wave story file).
# Assert: S-17.04-bar.md NOT present in injected set;
#         INJECTED_FILE_COUNT equals manifest count only.
#
# Red Gate: stub injects everything in directory → prior-wave file found → refute fails.
# ---------------------------------------------------------------------------

@test "test_rehydrate_wave_does_not_inject_prior_wave_specs" {
  _require_skill

  local git_content
  git_content="$(cat <<'YAML'
wave_id: 2
generated_at: "2026-06-16T00:00:00Z"
generated_from_handoff_sha: "aabbccddeeff00112233445566778899aabbccdd"
state_pointer: ".factory/STATE.md"
arch_files: []
stories:
  - id: S-18.02
    spec_files:
      - S-18.02-foo.md
YAML
)"
  _commit_to_factory_artifacts "wave-state.yaml" "$git_content"

  # Place a prior-wave story file in the project directory — must NOT be injected
  echo "# Prior wave story — must not be injected" > "$WORK/S-17.04-bar.md"

  _run_skill

  # Assert: prior-wave file S-17.04-bar.md must NOT appear in injected output
  printf '%s\n' "$output" | grep -qF "S-17.04-bar.md" && {
    echo "FAIL (AC-003 / BC-6.24.001 postcondition 3): prior-wave file 'S-17.04-bar.md'"
    echo "  found in skill output. Skill must inject ONLY files listed in wave-state.yaml."
    echo "  Prior-wave files not in the manifest must never be injected."
    echo "Actual output: $output"
    false
  } || true

  # Assert: INJECTED_FILE_COUNT equals manifest count (1 story spec + 0 arch + 1 state = 2)
  local injected_count
  injected_count="$(_extract_injected_count "$output")"
  [ "$injected_count" = "2" ] || {
    echo "FAIL (AC-003 / VP-088 §2 PC2-SIGNAL): INJECTED_FILE_COUNT sentinel mismatch."
    echo "  Expected: INJECTED_FILE_COUNT=2 (S-18.02-foo.md + .factory/STATE.md)"
    echo "  Got sentinel value: '$injected_count'"
    echo "  Count must reflect ONLY manifest files — no additions from filesystem scan."
    echo "Actual output: $output"
    false
  }
}

# ---------------------------------------------------------------------------
# test_rehydrate_wave_always_injects_state_pointer
# AC-004 / BC-6.24.001 postcondition 4
#
# Setup: wave-state.yaml with state_pointer: ".factory/STATE.md" but
#        .factory/STATE.md not in spec_files.
# Assert: .factory/STATE.md appears in injected context output;
#         INJECTED_FILE_COUNT includes it.
#
# Red Gate: stub omits state_pointer → STATE.md not in output → assertion fails.
# ---------------------------------------------------------------------------

@test "test_rehydrate_wave_always_injects_state_pointer" {
  _require_skill

  # state_pointer is present but NOT listed in spec_files — skill must add it regardless
  local git_content
  git_content="$(cat <<'YAML'
wave_id: 2
generated_at: "2026-06-16T00:00:00Z"
generated_from_handoff_sha: "aabbccddeeff00112233445566778899aabbccdd"
state_pointer: ".factory/STATE.md"
arch_files:
  - arch.md
stories:
  - id: S-18.02
    spec_files:
      - story-spec.md
YAML
)"
  _commit_to_factory_artifacts "wave-state.yaml" "$git_content"

  _run_skill

  # Assert: .factory/STATE.md appears in injected context output
  printf '%s\n' "$output" | grep -qF ".factory/STATE.md" || {
    echo "FAIL (AC-004 / BC-6.24.001 postcondition 4): '.factory/STATE.md' not found in"
    echo "  skill output. Skill must ALWAYS inject the state_pointer value (.factory/STATE.md)"
    echo "  regardless of whether it appears in stories[*].spec_files."
    echo "Actual output: $output"
    false
  }

  # Assert INJECTED_FILE_COUNT includes state_pointer (story-spec.md + arch.md + STATE.md = 3)
  local injected_count
  injected_count="$(_extract_injected_count "$output")"
  [ "$injected_count" = "3" ] || {
    echo "FAIL (AC-004 / VP-088 §2 PC2-SIGNAL): INJECTED_FILE_COUNT sentinel mismatch."
    echo "  Expected: INJECTED_FILE_COUNT=3 (story-spec.md + arch.md + .factory/STATE.md)"
    echo "  Got sentinel value: '$injected_count'"
    echo "  state_pointer must be counted in INJECTED_FILE_COUNT."
    echo "Actual output: $output"
    false
  }
}

# ---------------------------------------------------------------------------
# test_rehydrate_wave_warns_on_missing_spec_file_and_continues
# AC-006 / BC-6.24.001 postcondition 6
#
# Setup: wave-state.yaml lists [A.md, missing-spec.md, C.md]; only A.md and C.md exist.
# Assert: output contains warning mentioning missing-spec.md;
#         A.md and C.md are injected; confirmation prompt shown; skill exits without error.
#
# Red Gate: stub hard-blocks on missing → no confirmation prompt → assertion fails.
# ---------------------------------------------------------------------------

@test "test_rehydrate_wave_warns_on_missing_spec_file_and_continues" {
  _require_skill

  local git_content
  git_content="$(cat <<'YAML'
wave_id: 2
generated_at: "2026-06-16T00:00:00Z"
generated_from_handoff_sha: "aabbccddeeff00112233445566778899aabbccdd"
state_pointer: ".factory/STATE.md"
arch_files:
  - C.md
stories:
  - id: S-18.02
    spec_files:
      - A.md
      - missing-spec.md
YAML
)"
  _commit_to_factory_artifacts "wave-state.yaml" "$git_content"

  # Create A.md and C.md but NOT missing-spec.md
  echo "# A spec file" > "$WORK/A.md"
  echo "# C spec file" > "$WORK/C.md"

  _run_skill

  # Assert: skill exits without error (warning path, not hard-block)
  [ "$status" -eq 0 ] || {
    echo "FAIL (AC-006 / BC-6.24.001 postcondition 6): skill exited $status on missing spec."
    echo "  Skill must WARN and continue — not hard-block — when a listed spec file is missing."
    echo "Actual output: $output"
    false
  }

  # Assert: output contains warning about the missing file
  printf '%s\n' "$output" | grep -qF "missing-spec.md" || {
    echo "FAIL (AC-006 / BC-6.24.001 postcondition 6): warning about 'missing-spec.md' not found."
    echo "  Skill must emit a warning naming the missing path when a spec file cannot be found."
    echo "Actual output: $output"
    false
  }

  # Assert: A.md (existing spec) still appears in injected output
  printf '%s\n' "$output" | grep -qF "A.md" || {
    echo "FAIL (AC-006): 'A.md' not found in output after missing-spec.md warning."
    echo "  Remaining files must be injected even when one is missing."
    echo "Actual output: $output"
    false
  }

  # Assert: C.md (existing arch file) still appears in injected output
  printf '%s\n' "$output" | grep -qF "C.md" || {
    echo "FAIL (AC-006): 'C.md' not found in output after missing-spec.md warning."
    echo "  Remaining arch files must be injected even when a story spec file is missing."
    echo "Actual output: $output"
    false
  }

  # Assert: confirmation prompt is still shown (skill continues past warning).
  # F-P1-007 tightened: assert the ACTUAL Step 8 sentence so this fails if the
  # confirmation prompt is removed or changed to a no-op comment.
  printf '%s\n' "$output" | grep -qF "Confirm rehydration:" || {
    echo "FAIL (AC-006 / BC-6.24.001 postcondition 5): operator confirmation prompt missing"
    echo "  after missing-file warning. Skill must still show the confirmation prompt"
    echo "  when it continues past a missing-file warning."
    echo "  Expected to find 'Confirm rehydration:' sentence (Step 8 of rehydrate-wave.sh)."
    echo "Actual output: $output"
    false
  }
}

# ---------------------------------------------------------------------------
# test_rehydrate_wave_hard_blocks_on_missing_manifest
# AC-007 / BC-6.24.001 postcondition 7 / VP-088 §3
#
# Setup: factory-artifacts branch has no wave-state.yaml and no HANDOFF.md with
#        epic_status: complete.
# Assert: output contains "RehydrationError: wave-state.yaml not found on factory-artifacts";
#         no files injected; skill exits with failure (non-zero).
#
# Red Gate: skill absent → skip; OR stub exits 0 → assert_failure fails.
# ---------------------------------------------------------------------------

@test "test_rehydrate_wave_hard_blocks_on_missing_manifest" {
  _require_skill

  # factory-artifacts branch is empty — no wave-state.yaml, no HANDOFF.md
  # (setup() already initialized an empty factory-artifacts orphan branch)

  _run_skill

  # Assert: skill exits with non-zero exit code (hard-block)
  [ "$status" -ne 0 ] || {
    echo "FAIL (AC-007 / BC-6.24.001 postcondition 7 / VP-088 §3): skill exited 0 when"
    echo "  wave-state.yaml is missing. Skill MUST hard-block (non-zero exit) when"
    echo "  wave-state.yaml is absent from factory-artifacts and no EPIC-COMPLETE HANDOFF.md."
    echo "Actual output: $output"
    false
  }

  # Assert: output contains the canonical RehydrationError message
  printf '%s\n' "$output" | grep -qF "RehydrationError: wave-state.yaml not found on factory-artifacts" || {
    echo "FAIL (AC-007 / BC-6.24.001 postcondition 7): canonical RehydrationError message missing."
    echo "  Expected: 'RehydrationError: wave-state.yaml not found on factory-artifacts'"
    echo "  Skill must emit this exact error string when the manifest is absent."
    echo "Actual output: $output"
    false
  }
}

# ---------------------------------------------------------------------------
# test_rehydrate_wave_no_rag_fallback_on_missing_manifest
# AC-008 / BC-6.24.001 postcondition 8 + invariant 3 / VP-088 §3 anti-postcondition
#
# Mirrors test_rehydrate_wave_hard_blocks_on_missing_manifest but additionally
# asserts that the skill emits no file injection output at all (no spec files
# listed in output before the error).
#
# Red Gate: stub injects via RAG → files appear in output → refute fails.
# ---------------------------------------------------------------------------

@test "test_rehydrate_wave_no_rag_fallback_on_missing_manifest" {
  _require_skill

  # factory-artifacts branch is empty — no wave-state.yaml, no HANDOFF.md
  # (setup() already initialized an empty factory-artifacts orphan branch)

  _run_skill

  # Assert: skill exits with non-zero exit code (same as AC-007 check)
  [ "$status" -ne 0 ] || {
    echo "FAIL (AC-008 / BC-6.24.001 postcondition 8 + Inv3 / VP-088 §3 anti-postcondition):"
    echo "  skill exited 0 when wave-state.yaml is missing."
    echo "  No-RAG path requires hard-block; exiting 0 implies files may have been injected."
    echo "Actual output: $output"
    false
  }

  # Assert: no file injection output present before the error.
  # A RAG fallback would list files from the repository spec corpus.
  # The INJECTED_FILE_COUNT sentinel must NOT appear (no injection occurred).
  printf '%s\n' "$output" | grep -q '^INJECTED_FILE_COUNT=' && {
    echo "FAIL (AC-008 / BC-6.24.001 Inv3 / VP-088 §3 anti-postcondition):"
    echo "  INJECTED_FILE_COUNT sentinel found in output when wave-state.yaml is missing."
    echo "  Skill must NOT inject any files via RAG or filesystem scan on missing manifest."
    echo "  The ONLY valid action is the hard-block (RehydrationError)."
    echo "Actual output: $output"
    false
  } || true

  # Assert: RehydrationError is present (same canonical message as AC-007)
  printf '%s\n' "$output" | grep -qF "RehydrationError: wave-state.yaml not found on factory-artifacts" || {
    echo "FAIL (AC-008): RehydrationError message missing from output."
    echo "  When manifest is absent and no EPIC-COMPLETE HANDOFF.md, the skill must"
    echo "  emit 'RehydrationError: wave-state.yaml not found on factory-artifacts'."
    echo "Actual output: $output"
    false
  }
}

# ---------------------------------------------------------------------------
# test_rehydrate_wave_epic_complete_reads_handoff_not_wave_state
# AC-009 / BC-6.24.001 edge case EC-EPIC / VP-088 §4
#
# Setup: no wave-state.yaml on factory-artifacts; HANDOFF.md has
#        epic_status: complete, next_wave_stories: [], arch_files: [C.md]
# Assert: exit 0; output contains "Epic complete — no next-wave stories";
#         .factory/STATE.md and C.md injected; no RehydrationError.
#
# Red Gate: stub emits RehydrationError on missing wave-state.yaml → assert fails.
# ---------------------------------------------------------------------------

@test "test_rehydrate_wave_epic_complete_reads_handoff_not_wave_state" {
  _require_skill

  # No wave-state.yaml — EPIC-COMPLETE path: HANDOFF.md written with epic_status: complete
  local handoff_content
  handoff_content="$(cat <<'YAML'
wave_id: 3
generated_at: "2026-06-16T00:00:00Z"
last_verified_develop_sha: "aabbccddeeff00112233445566778899aabbccdd"
epic_status: complete
next_wave_stories: []
state_pointer: ".factory/STATE.md"
arch_files:
  - C.md
active_bcs:
  - BC-6.24.001
open_decisions: []
pending_fixes: []
process_gaps: []
precompact_flush_sha: null
factory_lock_holder: null
YAML
)"
  _commit_to_factory_artifacts "HANDOFF.md" "$handoff_content"

  _run_skill

  # Assert: exit 0 (EPIC-COMPLETE is not an error)
  [ "$status" -eq 0 ] || {
    echo "FAIL (AC-009 / BC-6.24.001 EC-EPIC / VP-088 §4): skill exited $status on EPIC-COMPLETE."
    echo "  When HANDOFF.md has epic_status: complete, skill must exit 0 — the absence"
    echo "  of wave-state.yaml is EXPECTED on EPIC-COMPLETE and must NOT be an error."
    echo "Actual output: $output"
    false
  }

  # Assert: output contains "Epic complete — no next-wave stories"
  printf '%s\n' "$output" | grep -qF "Epic complete — no next-wave stories" || {
    echo "FAIL (AC-009 / VP-088 §4): canonical 'Epic complete — no next-wave stories' message missing."
    echo "  Skill must emit this exact string when HANDOFF.md has epic_status: complete."
    echo "Actual output: $output"
    false
  }

  # Assert: .factory/STATE.md injected
  printf '%s\n' "$output" | grep -qF ".factory/STATE.md" || {
    echo "FAIL (AC-009 / BC-6.24.001 EC-EPIC): '.factory/STATE.md' not in output on EPIC-COMPLETE."
    echo "  Skill must inject STATE.md from HANDOFF.md state_pointer on EPIC-COMPLETE path."
    echo "Actual output: $output"
    false
  }

  # Assert: C.md (from arch_files) injected
  printf '%s\n' "$output" | grep -qF "C.md" || {
    echo "FAIL (AC-009 / BC-6.24.001 EC-EPIC): 'C.md' from arch_files not in output on EPIC-COMPLETE."
    echo "  Skill must inject arch_files from HANDOFF.md on EPIC-COMPLETE path."
    echo "Actual output: $output"
    false
  }

  # Refute: RehydrationError must NOT appear on EPIC-COMPLETE path
  printf '%s\n' "$output" | grep -qF "RehydrationError" && {
    echo "FAIL (AC-009 / BC-6.24.001 EC-EPIC): 'RehydrationError' found in output on EPIC-COMPLETE."
    echo "  wave-state.yaml absence is EXPECTED on EPIC-COMPLETE — must NOT emit RehydrationError."
    echo "  Skill must read HANDOFF.md instead when wave-state.yaml is absent + epic_status:complete."
    echo "Actual output: $output"
    false
  } || true
}

# ---------------------------------------------------------------------------
# test_rehydrate_wave_deduplicates_spec_files_in_count
# AC-010 / BC-6.24.001 invariant 2 / VP-088 §2 PC2-SIGNAL exact-list semantics
#
# Setup: wave-state.yaml with two stories each listing A.md in spec_files,
#        plus arch_files: [B.md], state_pointer: ".factory/STATE.md"
# Assert: INJECTED_FILE_COUNT=3 (A.md + B.md + STATE.md, deduplicated);
#         A.md appears only once in injected context.
#
# Red Gate: stub double-counts → INJECTED_FILE_COUNT=4 → assert =3 fails.
# ---------------------------------------------------------------------------

@test "test_rehydrate_wave_deduplicates_spec_files_in_count" {
  _require_skill

  # Two stories each list A.md — dedup must produce count=3, not count=4
  local git_content
  git_content="$(cat <<'YAML'
wave_id: 2
generated_at: "2026-06-16T00:00:00Z"
generated_from_handoff_sha: "aabbccddeeff00112233445566778899aabbccdd"
state_pointer: ".factory/STATE.md"
arch_files:
  - B.md
stories:
  - id: S-18.01
    spec_files:
      - A.md
  - id: S-18.02
    spec_files:
      - A.md
YAML
)"
  _commit_to_factory_artifacts "wave-state.yaml" "$git_content"

  _run_skill

  # Assert exit success
  [ "$status" -eq 0 ] || {
    echo "FAIL (AC-010): skill exited $status; expected 0 on valid manifest."
    echo "Actual output: $output"
    false
  }

  # Assert INJECTED_FILE_COUNT=3 via machine-stable VP-088 §2 PC2-SIGNAL sentinel.
  # Set semantics: A.md (deduplicated) + B.md + .factory/STATE.md = 3.
  # A stub that double-counts would produce INJECTED_FILE_COUNT=4.
  local injected_count
  injected_count="$(_extract_injected_count "$output")"
  [ "$injected_count" = "3" ] || {
    echo "FAIL (AC-010 / BC-6.24.001 Inv2 / VP-088 §2 PC2-SIGNAL): deduplication failed."
    echo "  Expected: INJECTED_FILE_COUNT=3 (A.md deduplicated + B.md + .factory/STATE.md)"
    echo "  Got sentinel value: '$injected_count'"
    echo "  INJECTED_FILE_COUNT must reflect SET semantics:"
    echo "  Set(stories[*].spec_files) UNION Set(arch_files) UNION {state_pointer}."
    echo "  Duplicates across multiple stories must be counted once."
    echo "Actual output: $output"
    false
  }

  # Verify A.md appears in the injected set (at least once; dedup doesn't mean absent)
  printf '%s\n' "$output" | grep -qF "A.md" || {
    echo "FAIL (AC-010): 'A.md' not found in injected output — must be present (once)."
    echo "Actual output: $output"
    false
  }
}

# ---------------------------------------------------------------------------
# test_rehydrate_wave_warns_on_empty_stories_list
# F-P1-001 / EC-004 / BC-6.24.001 postcondition 2
#
# Setup: wave-state.yaml with stories: [] (no EPIC-COMPLETE), arch_files: [C.md],
#        state_pointer: ".factory/STATE.md"
# Assert: stderr contains EC-004 warning string; exit 0;
#         INJECTED_FILE_COUNT=2 (arch_files + state_pointer only);
#         C.md and .factory/STATE.md appear in injected output.
#
# Closes adversary LOCAL Pass-1 finding F-P1-001.
# Red Gate: a stub that does NOT emit the warning → grep fails.
# ---------------------------------------------------------------------------

@test "test_rehydrate_wave_warns_on_empty_stories_list" {
  _require_skill

  # Create C.md in WORK so the skill finds it on filesystem (avoids PC6 missing-file warning)
  echo "# C arch file" > "$WORK/C.md"

  local git_content
  git_content="$(cat <<'YAML'
wave_id: 2
generated_at: "2026-06-16T00:00:00Z"
generated_from_handoff_sha: "aabbccddeeff00112233445566778899aabbccdd"
state_pointer: ".factory/STATE.md"
arch_files:
  - C.md
stories: []
YAML
)"
  _commit_to_factory_artifacts "wave-state.yaml" "$git_content"

  # Capture combined stdout+stderr — _run_skill already merges 2>&1.
  _run_skill

  # Assert: skill exits 0 (EC-004 is a warning path, not a hard block)
  [ "$status" -eq 0 ] || {
    echo "FAIL (F-P1-001 / EC-004): skill exited $status; expected 0 on empty stories list."
    echo "  EC-004 is a WARNING, not a hard-block. Skill must exit 0."
    echo "Actual output: $output"
    false
  }

  # Assert: EC-004 warning string present (emitted on stderr, captured via 2>&1)
  printf '%s\n' "$output" | grep -qF "WARNING: wave-state.yaml lists no stories (stories: [] or no spec_files); injecting arch_files + state_pointer only." || {
    echo "FAIL (F-P1-001 / EC-004): EC-004 warning string not found in output."
    echo "  Expected: 'WARNING: wave-state.yaml lists no stories (stories: [] or no spec_files); injecting arch_files + state_pointer only.'"
    echo "  Skill must emit this exact warning on stderr when stories: [] or all spec_files are empty."
    echo "Actual output: $output"
    false
  }

  # Assert: INJECTED_FILE_COUNT=2 (C.md + .factory/STATE.md; no story spec files)
  local injected_count
  injected_count="$(_extract_injected_count "$output")"
  [ "$injected_count" = "2" ] || {
    echo "FAIL (F-P1-001 / EC-004 / VP-088 §2 PC2-SIGNAL): INJECTED_FILE_COUNT sentinel mismatch."
    echo "  Expected: INJECTED_FILE_COUNT=2 (C.md + .factory/STATE.md; stories: [] contributes nothing)"
    echo "  Got sentinel value: '$injected_count'"
    echo "Actual output: $output"
    false
  }

  # Assert: C.md (arch file) appears in injected output
  printf '%s\n' "$output" | grep -qF "C.md" || {
    echo "FAIL (F-P1-001 / EC-004): 'C.md' not found in injected output on empty-stories path."
    echo "  Skill must inject arch_files even when stories: [] (EC-004 warning path)."
    echo "Actual output: $output"
    false
  }

  # Assert: .factory/STATE.md (state_pointer) appears in injected output
  printf '%s\n' "$output" | grep -qF ".factory/STATE.md" || {
    echo "FAIL (F-P1-001 / EC-004): '.factory/STATE.md' not found in injected output on empty-stories path."
    echo "  Skill must inject state_pointer even when stories: [] (EC-004 warning path)."
    echo "Actual output: $output"
    false
  }
}

# ---------------------------------------------------------------------------
# test_rehydrate_wave_warns_on_empty_arch_files
# F-P1-002 / EC-006 / BC-6.24.001 postcondition 2
#
# Setup: wave-state.yaml with one story spec_files: [A.md], arch_files: [],
#        state_pointer: ".factory/STATE.md"
# Assert: stderr contains EC-006 warning string; exit 0;
#         INJECTED_FILE_COUNT=2 (A.md + STATE.md);
#         A.md and .factory/STATE.md injected.
#
# Closes adversary LOCAL Pass-1 finding F-P1-002.
# Red Gate: a stub that does NOT emit the EC-006 warning → grep fails.
# ---------------------------------------------------------------------------

@test "test_rehydrate_wave_warns_on_empty_arch_files" {
  _require_skill

  # Create A.md in WORK so the skill finds it (avoids PC6 missing-file warning)
  echo "# A spec file" > "$WORK/A.md"

  local git_content
  git_content="$(cat <<'YAML'
wave_id: 2
generated_at: "2026-06-16T00:00:00Z"
generated_from_handoff_sha: "aabbccddeeff00112233445566778899aabbccdd"
state_pointer: ".factory/STATE.md"
arch_files: []
stories:
  - id: S-18.02
    spec_files:
      - A.md
YAML
)"
  _commit_to_factory_artifacts "wave-state.yaml" "$git_content"

  # Capture combined stdout+stderr
  _run_skill

  # Assert: skill exits 0 (EC-006 is a warning path, not a hard block)
  [ "$status" -eq 0 ] || {
    echo "FAIL (F-P1-002 / EC-006): skill exited $status; expected 0 on empty arch_files."
    echo "  EC-006 is a WARNING, not a hard-block. Skill must exit 0."
    echo "Actual output: $output"
    false
  }

  # Assert: EC-006 warning string present (emitted on stderr, captured via 2>&1)
  printf '%s\n' "$output" | grep -qF "WARNING: wave-state.yaml lists no arch_files; no architectural context will be injected." || {
    echo "FAIL (F-P1-002 / EC-006): EC-006 warning string not found in output."
    echo "  Expected: 'WARNING: wave-state.yaml lists no arch_files; no architectural context will be injected.'"
    echo "  Skill must emit this exact warning on stderr when arch_files is empty."
    echo "Actual output: $output"
    false
  }

  # Assert: INJECTED_FILE_COUNT=2 (A.md + .factory/STATE.md; no arch files)
  local injected_count
  injected_count="$(_extract_injected_count "$output")"
  [ "$injected_count" = "2" ] || {
    echo "FAIL (F-P1-002 / EC-006 / VP-088 §2 PC2-SIGNAL): INJECTED_FILE_COUNT sentinel mismatch."
    echo "  Expected: INJECTED_FILE_COUNT=2 (A.md + .factory/STATE.md; arch_files: [] contributes nothing)"
    echo "  Got sentinel value: '$injected_count'"
    echo "Actual output: $output"
    false
  }

  # Assert: A.md (story spec file) appears in injected output
  printf '%s\n' "$output" | grep -qF "A.md" || {
    echo "FAIL (F-P1-002 / EC-006): 'A.md' not found in injected output on empty-arch_files path."
    echo "  Skill must inject stories spec_files even when arch_files: [] (EC-006 warning path)."
    echo "Actual output: $output"
    false
  }

  # Assert: .factory/STATE.md (state_pointer) appears in injected output
  printf '%s\n' "$output" | grep -qF ".factory/STATE.md" || {
    echo "FAIL (F-P1-002 / EC-006): '.factory/STATE.md' not found in injected output on empty-arch_files path."
    echo "  Skill must always inject state_pointer (AC-004) even when arch_files: []."
    echo "Actual output: $output"
    false
  }
}

# ---------------------------------------------------------------------------
# test_rehydrate_wave_parses_real_producer_manifest_shape
# F-P1-003 / BC-6.24.001 postcondition 2
#
# Integration test: the fixture is hand-written as a byte-faithful copy of the
# exact YAML that wave-handoff/lib/write-wave-state.sh emits (see producer source
# at plugins/vsdd-factory/skills/wave-handoff/lib/write-wave-state.sh lines 353-356):
#
#   stories_yaml="${stories_yaml}
#   - id: ${sid}
#     status: ${sstatus}
# ${spec_files_yaml}"
#
# Where spec_files_yaml is "    spec_files:\n      - <path>" (4-space indent, 6-space items).
# Critical: the `status:` line appears BETWEEN `id:` and `spec_files:`.
# The consumer's _parse_all_story_spec_files awk must skip `status:` without
# exiting the spec_files parsing block (the status: key was the defect).
#
# Also verifies state_pointer: without surrounding quotes (producer emits unquoted).
#
# Closes adversary LOCAL Pass-1 finding F-P1-003.
# Red Gate: a parser that exits spec_files block on status: → INJECTED_FILE_COUNT wrong.
# ---------------------------------------------------------------------------

@test "test_rehydrate_wave_parses_real_producer_manifest_shape" {
  _require_skill

  # Create the spec files on filesystem so PC6 missing-file warnings don't fire
  mkdir -p "$WORK/specs/behavioral-contracts/ss-06"
  echo "# BC-6.24.001" > "$WORK/specs/behavioral-contracts/ss-06/BC-6.24.001.md"
  echo "# BC-5.41.002" > "$WORK/specs/behavioral-contracts/ss-06/BC-5.41.002.md"
  mkdir -p "$WORK/specs/architecture"
  echo "# ARCH-INDEX" > "$WORK/specs/architecture/ARCH-INDEX.md"

  # Byte-faithful copy of write-wave-state.sh output format.
  # Note: stories_yaml is built with leading newline before "  - id:", so the block is:
  #   stories:
  #   - id: S-18.02
  #     status: pending
  #     spec_files:
  #       - specs/behavioral-contracts/ss-06/BC-6.24.001.md
  #   - id: S-18.03
  #     status: pending
  #     spec_files:
  #       - specs/behavioral-contracts/ss-06/BC-5.41.002.md
  # arch_files similarly starts with leading newline before "  - ".
  # state_pointer is unquoted (no surrounding double-quotes).
  local git_content
  git_content="$(cat <<'YAML'
wave_id: 4
generated_at: 2026-06-17T12:00:00Z
generated_from_handoff_sha: aabbccddeeff00112233445566778899aabbccdd
stories:
  - id: S-18.02
    status: pending
    spec_files:
      - specs/behavioral-contracts/ss-06/BC-6.24.001.md
  - id: S-18.03
    status: pending
    spec_files:
      - specs/behavioral-contracts/ss-06/BC-5.41.002.md
arch_files:
  - specs/architecture/ARCH-INDEX.md
state_pointer: .factory/STATE.md
YAML
)"
  _commit_to_factory_artifacts "wave-state.yaml" "$git_content"

  _run_skill

  # Assert: exit 0 (valid manifest)
  [ "$status" -eq 0 ] || {
    echo "FAIL (F-P1-003): skill exited $status on real-producer-shape manifest."
    echo "  Skill must parse the intervening 'status:' line between 'id:' and 'spec_files:'."
    echo "Actual output: $output"
    false
  }

  # Assert: INJECTED_FILE_COUNT=4
  # (BC-6.24.001.md + BC-5.41.002.md + ARCH-INDEX.md + .factory/STATE.md = 4)
  local injected_count
  injected_count="$(_extract_injected_count "$output")"
  [ "$injected_count" = "4" ] || {
    echo "FAIL (F-P1-003 / VP-088 §2 PC2-SIGNAL): INJECTED_FILE_COUNT mismatch on real-producer shape."
    echo "  Expected: INJECTED_FILE_COUNT=4"
    echo "  Got: '$injected_count'"
    echo "  Parser must not be defeated by the 'status: pending' line between 'id:' and 'spec_files:'."
    echo "  (Producer write-wave-state.sh always emits status: between id: and spec_files:.)"
    echo "Actual output: $output"
    false
  }

  # Assert: each real spec path appears in injected output
  for expected_file in \
    "specs/behavioral-contracts/ss-06/BC-6.24.001.md" \
    "specs/behavioral-contracts/ss-06/BC-5.41.002.md" \
    "specs/architecture/ARCH-INDEX.md" \
    ".factory/STATE.md"
  do
    printf '%s\n' "$output" | grep -qF "$expected_file" || {
      echo "FAIL (F-P1-003): expected file '$expected_file' not found in injected output."
      echo "  The consumer must correctly parse spec_files from real producer-shape YAML."
      echo "Actual output: $output"
      false
    }
  done
}

# ---------------------------------------------------------------------------
# test_rehydrate_wave_handles_inline_empty_spec_files
# F-P1-004 / BC-6.24.001 postcondition 2 / invariant 2
#
# Setup: wave-state.yaml with two stories:
#          - S-18.01 with spec_files: [A.md]
#          - S-18.02 with spec_files: []   (inline-empty list form)
#        arch_files: [B.md], state_pointer: ".factory/STATE.md"
# Assert: INJECTED_FILE_COUNT=3 (A.md + B.md + STATE.md);
#         the empty-spec_files story contributes nothing; no crash; exit 0.
#
# Closes adversary LOCAL Pass-1 finding F-P1-004.
# Red Gate: a parser that crashes on "spec_files: []" → non-zero exit or wrong count.
# ---------------------------------------------------------------------------

@test "test_rehydrate_wave_handles_inline_empty_spec_files" {
  _require_skill

  # Create A.md and B.md on filesystem
  echo "# A spec file" > "$WORK/A.md"
  echo "# B arch file" > "$WORK/B.md"

  local git_content
  git_content="$(cat <<'YAML'
wave_id: 2
generated_at: "2026-06-16T00:00:00Z"
generated_from_handoff_sha: "aabbccddeeff00112233445566778899aabbccdd"
state_pointer: ".factory/STATE.md"
arch_files:
  - B.md
stories:
  - id: S-18.01
    spec_files:
      - A.md
  - id: S-18.02
    spec_files: []
YAML
)"
  _commit_to_factory_artifacts "wave-state.yaml" "$git_content"

  _run_skill

  # Assert: exit 0 (inline-empty spec_files is not an error)
  [ "$status" -eq 0 ] || {
    echo "FAIL (F-P1-004): skill exited $status on inline spec_files: [] form."
    echo "  Empty spec_files list for one story must not cause a crash or hard-block."
    echo "Actual output: $output"
    false
  }

  # Assert: INJECTED_FILE_COUNT=3 (A.md + B.md + .factory/STATE.md;
  #         S-18.02's spec_files: [] contributes nothing)
  local injected_count
  injected_count="$(_extract_injected_count "$output")"
  [ "$injected_count" = "3" ] || {
    echo "FAIL (F-P1-004 / VP-088 §2 PC2-SIGNAL): INJECTED_FILE_COUNT mismatch on inline-empty spec_files."
    echo "  Expected: INJECTED_FILE_COUNT=3 (A.md + B.md + .factory/STATE.md)"
    echo "  Got: '$injected_count'"
    echo "  The story with spec_files: [] must contribute 0 files to the injected set."
    echo "Actual output: $output"
    false
  }

  # Assert: A.md from the non-empty story is still injected
  printf '%s\n' "$output" | grep -qF "A.md" || {
    echo "FAIL (F-P1-004): 'A.md' not found in injected output."
    echo "  The story with spec_files: [A.md] must still inject A.md."
    echo "Actual output: $output"
    false
  }

  # Assert: B.md (arch file) is still injected
  printf '%s\n' "$output" | grep -qF "B.md" || {
    echo "FAIL (F-P1-004): 'B.md' not found in injected output."
    echo "  arch_files must still be injected when one story has spec_files: []."
    echo "Actual output: $output"
    false
  }

  # Assert: .factory/STATE.md (state_pointer) is still injected
  printf '%s\n' "$output" | grep -qF ".factory/STATE.md" || {
    echo "FAIL (F-P1-004): '.factory/STATE.md' not found in injected output."
    echo "  state_pointer must always be injected (AC-004)."
    echo "Actual output: $output"
    false
  }
}

# ---------------------------------------------------------------------------
# test_rehydrate_wave_bare_invocation_uses_defaults
# F-P2-001 / §Library and Framework Requirements — bare-invocation default path
#
# Setup: temp repo with factory-artifacts branch carrying a valid wave-state.yaml.
#        Script invoked with NO --repo / --artifacts-worktree flags and NO env vars,
#        so REPO_DIR defaults to "." and ARTIFACTS_WT defaults to ".factory"
#        (lines 59-60 in rehydrate-wave.sh: REPO_DIR="${REPO_DIR:-.}").
# Assert: script resolves defaults correctly; exit 0; INJECTED_FILE_COUNT correct;
#         expected spec files appear in output.
#
# Closes adversary LOCAL Pass-2 finding F-P2-001.
# Load-bearing: removing the "${REPO_DIR:-.}" defaulting (making REPO_DIR="")
# causes `git -C "" show factory-artifacts:...` to fail → RehydrationError → test fails.
# ---------------------------------------------------------------------------

@test "test_rehydrate_wave_bare_invocation_uses_defaults" {
  _require_skill

  # Commit a valid wave-state.yaml to the factory-artifacts branch of the fixture repo.
  local git_content
  git_content="$(cat <<'YAML'
wave_id: 3
generated_at: "2026-06-25T00:00:00Z"
generated_from_handoff_sha: "aabbccddeeff00112233445566778899aabbccdd"
state_pointer: ".factory/STATE.md"
arch_files:
  - arch-bare.md
stories:
  - id: S-18.03
    spec_files:
      - story-bare.md
YAML
)"
  _commit_to_factory_artifacts "wave-state.yaml" "$git_content"

  # Invoke the skill with NO path flags and NO env vars.
  # Must cd into WORK first so "." resolves to the fixture repo, where
  # `git -C . show factory-artifacts:wave-state.yaml` succeeds.
  # Unset env vars so the bare-invocation defaults are exercised, not inherited env.
  run bash -c "
    cd '$WORK'
    unset REPO_DIR
    unset ARTIFACTS_WT
    '$SKILL' 2>&1
  "

  # Assert: script exited 0 (defaults resolved correctly — no missing-manifest error)
  [ "$status" -eq 0 ] || {
    echo "FAIL (F-P2-001): bare invocation exited $status; expected 0."
    echo "  Script must default REPO_DIR='.' and ARTIFACTS_WT='.factory' when"
    echo "  neither --repo/--artifacts-worktree flags nor env vars are set."
    echo "  'git -C . show factory-artifacts:wave-state.yaml' must succeed from WORK."
    echo "Actual output: $output"
    false
  }

  # Assert: INJECTED_FILE_COUNT=3 (story-bare.md + arch-bare.md + .factory/STATE.md)
  local injected_count
  injected_count="$(_extract_injected_count "$output")"
  [ "$injected_count" = "3" ] || {
    echo "FAIL (F-P2-001 / VP-088 §2 PC2-SIGNAL): INJECTED_FILE_COUNT sentinel mismatch."
    echo "  Expected: INJECTED_FILE_COUNT=3 (story-bare.md + arch-bare.md + .factory/STATE.md)"
    echo "  Got sentinel value: '$injected_count'"
    echo "  Bare-invocation must parse wave-state.yaml and count files correctly."
    echo "Actual output: $output"
    false
  }

  # Assert: expected files appear in injected output
  for expected_file in "story-bare.md" "arch-bare.md" ".factory/STATE.md"; do
    printf '%s\n' "$output" | grep -qF "$expected_file" || {
      echo "FAIL (F-P2-001): expected file '$expected_file' not found in bare-invocation output."
      echo "  Bare invocation must inject the full spec set from wave-state.yaml."
      echo "Actual output: $output"
      false
    }
  done
}

# ---------------------------------------------------------------------------
# test_rehydrate_wave_epic_complete_warns_on_missing_arch_file
# F-P2-003 / BC-6.24.001 EC-EPIC + postcondition 6 / VP-088 §4
#
# Setup: no wave-state.yaml; HANDOFF.md with epic_status: complete,
#        next_wave_stories: [], arch_files: [absent-arch.md] where absent-arch.md
#        is absent from BOTH the filesystem AND factory-artifacts.
# Assert: stderr contains "WARNING: listed spec file not found on filesystem: absent-arch.md";
#         exit 0 (EPIC-COMPLETE path; missing arch_file is a warning, not a hard-block);
#         "Epic complete — no next-wave stories" message still emitted;
#         .factory/STATE.md still injected;
#         INJECTED_FILE_COUNT=1 (STATE.md only; absent-arch.md excluded from count).
#
# Closes adversary LOCAL Pass-2 finding F-P2-003.
# Load-bearing: removing _check_missing_file from the EPIC-COMPLETE path causes
# the WARNING to never be emitted → grep assertion fails.
# ---------------------------------------------------------------------------

@test "test_rehydrate_wave_epic_complete_warns_on_missing_arch_file" {
  _require_skill

  # No wave-state.yaml — EPIC-COMPLETE path triggered by HANDOFF.md.
  # absent-arch.md is listed but NOT created on filesystem or in factory-artifacts.
  local handoff_content
  handoff_content="$(cat <<'YAML'
wave_id: 4
generated_at: "2026-06-25T00:00:00Z"
last_verified_develop_sha: "aabbccddeeff00112233445566778899aabbccdd"
epic_status: complete
next_wave_stories: []
state_pointer: ".factory/STATE.md"
arch_files:
  - absent-arch.md
active_bcs:
  - BC-6.24.001
open_decisions: []
pending_fixes: []
process_gaps: []
precompact_flush_sha: null
factory_lock_holder: null
YAML
)"
  _commit_to_factory_artifacts "HANDOFF.md" "$handoff_content"
  # Deliberately do NOT create absent-arch.md on filesystem or in factory-artifacts.

  _run_skill

  # Assert: exit 0 (missing arch_file is a WARNING, not a hard-block on EPIC-COMPLETE path)
  [ "$status" -eq 0 ] || {
    echo "FAIL (F-P2-003 / BC-6.24.001 EC-EPIC + PC6): skill exited $status on EPIC-COMPLETE"
    echo "  with a missing arch_file. The EPIC-COMPLETE path must WARN and continue"
    echo "  (same behavior as the wave-state path for missing files)."
    echo "Actual output: $output"
    false
  }

  # Assert: warning about absent-arch.md is present on stderr (captured via 2>&1)
  printf '%s\n' "$output" | grep -qF "WARNING: listed spec file not found on filesystem: absent-arch.md" || {
    echo "FAIL (F-P2-003): missing-file warning for 'absent-arch.md' not found in output."
    echo "  Expected: 'WARNING: listed spec file not found on filesystem: absent-arch.md'"
    echo "  The EPIC-COMPLETE path must call _check_missing_file for each arch_file."
    echo "  The same stable warning string used by the wave-state path must be used here."
    echo "Actual output: $output"
    false
  }

  # Assert: "Epic complete — no next-wave stories" message still emitted (EPIC-COMPLETE path OK)
  printf '%s\n' "$output" | grep -qF "Epic complete — no next-wave stories" || {
    echo "FAIL (F-P2-003 / VP-088 §4): 'Epic complete — no next-wave stories' message missing."
    echo "  The missing-arch-file warning must not suppress the EPIC-COMPLETE message."
    echo "Actual output: $output"
    false
  }

  # Assert: .factory/STATE.md (state_pointer) still injected despite missing arch_file
  printf '%s\n' "$output" | grep -qF ".factory/STATE.md" || {
    echo "FAIL (F-P2-003): '.factory/STATE.md' not found in output on EPIC-COMPLETE path."
    echo "  state_pointer must always be injected (AC-004); missing arch_file must not block it."
    echo "Actual output: $output"
    false
  }

  # Assert: INJECTED_FILE_COUNT=1 (STATE.md only; absent-arch.md is still counted in the
  # injected set — the skill builds the set from the manifest regardless of filesystem presence;
  # the warning is advisory only). Actually: the skill adds absent-arch.md to the set and
  # warns; count reflects set members including missing ones (INJECTED_FILE_COUNT counts
  # listed files, not only those confirmed present). Read implementation: INJECTED_SET is
  # built BEFORE _check_missing_file is called; _check_missing_file is advisory-only.
  # So INJECTED_FILE_COUNT = STATE.md + absent-arch.md = 2.
  local injected_count
  injected_count="$(_extract_injected_count "$output")"
  [ "$injected_count" = "2" ] || {
    echo "FAIL (F-P2-003 / VP-088 §2 PC2-SIGNAL): INJECTED_FILE_COUNT sentinel mismatch."
    echo "  Expected: INJECTED_FILE_COUNT=2 (.factory/STATE.md + absent-arch.md)"
    echo "  Got sentinel value: '$injected_count'"
    echo "  _check_missing_file is advisory-only; the file is still in the listed set."
    echo "Actual output: $output"
    false
  }
}

# ---------------------------------------------------------------------------
# test_rehydrate_wave_epic_complete_warns_on_nonempty_next_wave_stories
# F-P2-004 / BC-6.24.001 EC-EPIC / VP-088 §4
#
# Setup: no wave-state.yaml; HANDOFF.md with epic_status: complete BUT
#        next_wave_stories NON-empty (contradictory manifest).
# Assert: stderr contains the canonical contradiction warning string;
#         exit 0 (epic_status:complete is the authoritative discriminator;
#         skill proceeds on EPIC-COMPLETE path per rehydrate-wave.sh lines 220-226);
#         "Epic complete — no next-wave stories" message still emitted;
#         .factory/STATE.md injected;
#         INJECTED_FILE_COUNT emitted.
#
# Closes adversary LOCAL Pass-2 finding F-P2-004.
# Load-bearing: removing the NEXT_WAVE_STORIES non-empty check at lines 220-225
# causes the WARNING to never be emitted → grep assertion fails.
# ---------------------------------------------------------------------------

@test "test_rehydrate_wave_epic_complete_warns_on_nonempty_next_wave_stories" {
  _require_skill

  # No wave-state.yaml — contradictory HANDOFF.md: epic_status:complete + non-empty next_wave_stories.
  local handoff_content
  handoff_content="$(cat <<'YAML'
wave_id: 4
generated_at: "2026-06-25T00:00:00Z"
last_verified_develop_sha: "aabbccddeeff00112233445566778899aabbccdd"
epic_status: complete
next_wave_stories:
  - S-19.01
state_pointer: ".factory/STATE.md"
arch_files: []
active_bcs:
  - BC-6.24.001
open_decisions: []
pending_fixes: []
process_gaps: []
precompact_flush_sha: null
factory_lock_holder: null
YAML
)"
  _commit_to_factory_artifacts "HANDOFF.md" "$handoff_content"

  _run_skill

  # Assert: contradiction warning is present on stderr (captured via 2>&1)
  printf '%s\n' "$output" | grep -qF "WARNING: HANDOFF.md epic_status=complete but next_wave_stories is non-empty; manifest is contradictory." || {
    echo "FAIL (F-P2-004 / VP-088 §4): contradictory-manifest warning not found in output."
    echo "  Expected: 'WARNING: HANDOFF.md epic_status=complete but next_wave_stories is non-empty; manifest is contradictory.'"
    echo "  rehydrate-wave.sh lines 220-225 must emit this warning when epic_status=complete"
    echo "  AND next_wave_stories is non-empty."
    echo "Actual output: $output"
    false
  }

  # Assert: exit 0 — epic_status:complete is the authoritative discriminator (per implementation).
  # The warning is advisory; the skill proceeds on the EPIC-COMPLETE path (exit 0).
  [ "$status" -eq 0 ] || {
    echo "FAIL (F-P2-004 / BC-6.24.001 EC-EPIC): skill exited $status on contradictory manifest."
    echo "  epic_status:complete is the authoritative discriminator (rehydrate-wave.sh line 210)."
    echo "  The skill must proceed on the EPIC-COMPLETE path (exit 0) even on contradictory manifests."
    echo "Actual output: $output"
    false
  }

  # Assert: "Epic complete — no next-wave stories" message still emitted
  # (skill continues on EPIC-COMPLETE path despite the contradiction warning)
  printf '%s\n' "$output" | grep -qF "Epic complete — no next-wave stories" || {
    echo "FAIL (F-P2-004 / VP-088 §4): 'Epic complete — no next-wave stories' message missing"
    echo "  on contradictory manifest. The contradiction warning is advisory-only;"
    echo "  the EPIC-COMPLETE message must still be emitted."
    echo "Actual output: $output"
    false
  }

  # Assert: .factory/STATE.md injected (EPIC-COMPLETE path proceeds normally)
  printf '%s\n' "$output" | grep -qF ".factory/STATE.md" || {
    echo "FAIL (F-P2-004): '.factory/STATE.md' not found in output on contradictory manifest."
    echo "  state_pointer must always be injected (AC-004) on EPIC-COMPLETE path."
    echo "Actual output: $output"
    false
  }

  # Assert: INJECTED_FILE_COUNT is emitted (machine-stable sentinel present)
  local injected_count
  injected_count="$(_extract_injected_count "$output")"
  [ -n "$injected_count" ] || {
    echo "FAIL (F-P2-004 / VP-088 §2 PC2-SIGNAL): INJECTED_FILE_COUNT sentinel missing"
    echo "  on contradictory manifest. Skill must emit this sentinel on all exit-0 paths."
    echo "Actual output: $output"
    false
  }

  # Assert: INJECTED_FILE_COUNT=1 (only .factory/STATE.md; arch_files:[])
  [ "$injected_count" = "1" ] || {
    echo "FAIL (F-P2-004 / VP-088 §2 PC2-SIGNAL): INJECTED_FILE_COUNT mismatch."
    echo "  Expected: INJECTED_FILE_COUNT=1 (.factory/STATE.md; arch_files: [] contributes 0)"
    echo "  Got sentinel value: '$injected_count'"
    echo "Actual output: $output"
    false
  }
}
