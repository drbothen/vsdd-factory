#!/usr/bin/env bats
# factory-lock-skills-integration.bats — RED-phase (TDD) integration tests for
# BC-6.23.001 AC-011 (state-manager delegation invariant), AC-013 (concurrent
# acquire CAS race tiebreaker).
#
# Traces to: BC-6.23.001 Invariant 5 (no direct STATE.md write in skills),
#            AC-011, AC-013, T-4/T-10 canonical test vectors.
# Story: S-17.03 — /factory-lock + /factory-unlock skills + health status.
#
# RED GATE:
#   test_BC_6_23_001_skill_mds_contain_no_direct_state_write — RED because
#     factory-lock/SKILL.md and factory-unlock/SKILL.md do not exist yet.
#   test_BC_6_23_001_concurrent_acquire_cas_race_one_wins_one_rejected — RED
#     because the /factory-lock skill does not exist yet; the CAS acquire flow
#     is not wired up. Once both helper stubs are implemented and the CAS push
#     is wired through state-manager, this test will become GREEN.
#
# FIXTURE STRATEGY for the CAS race test (T-10):
#   Builds a local bare repo + two clones (dev-a + dev-b), never touching the
#   real .factory/ or origin. Mirrors the factory-cas-push.bats pattern from
#   S-17.01. Both clones see the same unlocked STATE.md. Both run
#   factory-lock-acquire-precheck.sh (should both return PROCEED_ACQUIRE), then
#   race to do CAS-push via factory-cas-push.sh. One succeeds; one gets the
#   CASPushRejected error. This proves the CAS push is the acquire tiebreaker.
#
# Run:
#   bats plugins/vsdd-factory/tests/factory-lock-skills-integration.bats

SKILL_DIR="$(cd "$(dirname "$BATS_TEST_FILENAME")/.." && pwd)/skills"
PRECHECK_HELPER="$(cd "$(dirname "$BATS_TEST_FILENAME")/.." && pwd)/bin/factory-lock-acquire-precheck.sh"
CAS_PUSH_HELPER="$(cd "$(dirname "$BATS_TEST_FILENAME")/.." && pwd)/bin/factory-cas-push.sh"
LOCK_WRITE_HELPER="$(cd "$(dirname "$BATS_TEST_FILENAME")/.." && pwd)/bin/factory-lock-write.sh"

REAL_GIT_PATH="$(command -v git)"

# ---------------------------------------------------------------------------
# setup / teardown
# ---------------------------------------------------------------------------

setup() {
  WORK="$(mktemp -d)"
  WORK="$(cd "$WORK" && pwd -P)"
}

teardown() {
  rm -rf "$WORK"
}

# ---------------------------------------------------------------------------
# Git fixture helper (mirrors factory-cas-push.bats _setup_git_fixture pattern)
# Sets:
#   BARE_REPO   — path to bare.git
#   CLONE_A     — first clone (dev-a@x.com)
#   CLONE_B     — second clone (dev-b@x.com)
#   INIT_SHA    — initial factory-artifacts commit SHA
# ---------------------------------------------------------------------------

_setup_two_clone_fixture() {
  BARE_REPO="$WORK/bare.git"
  CLONE_A="$WORK/clone-a"
  CLONE_B="$WORK/clone-b"

  # Bare repo
  git init --bare "$BARE_REPO" >/dev/null 2>&1

  # Clone A (dev-a)
  git clone "$BARE_REPO" "$CLONE_A" >/dev/null 2>&1
  git -C "$CLONE_A" config user.email "dev-a@x.com"
  git -C "$CLONE_A" config user.name "Dev A"

  # Create factory-artifacts branch with an initial unlocked STATE.md
  git -C "$CLONE_A" checkout -b factory-artifacts >/dev/null 2>&1
  cat > "$CLONE_A/STATE.md" <<'STATE'
---
document_type: state
version: "0.0.1-test"
phase: test
current_step: "test-step"
timestamp: "2026-01-01T00:00:00Z"
---

# STATE (integration fixture)
Unlocked baseline.
STATE
  git -C "$CLONE_A" add STATE.md >/dev/null 2>&1
  git -C "$CLONE_A" commit -m "init factory-artifacts (unlocked)" >/dev/null 2>&1
  git -C "$CLONE_A" push origin factory-artifacts >/dev/null 2>&1

  INIT_SHA="$(git -C "$CLONE_A" rev-parse origin/factory-artifacts)"

  # Clone B (dev-b) — starts from the same initial commit
  git clone "$BARE_REPO" "$CLONE_B" >/dev/null 2>&1
  git -C "$CLONE_B" config user.email "dev-b@x.com"
  git -C "$CLONE_B" config user.name "Dev B"
  git -C "$CLONE_B" checkout factory-artifacts >/dev/null 2>&1
}

# ---------------------------------------------------------------------------
# test_BC_6_23_001_skill_mds_contain_no_direct_state_write
# AC-011 / BC-6.23.001 Invariant 5 — structural grep on both new SKILL.md files
#
# Neither /factory-lock/SKILL.md nor /factory-unlock/SKILL.md MUST contain a
# direct "Write STATE.md" instruction. Both skills are thin orchestrators that
# delegate STATE.md writes to state-manager. This structural test enforces the
# Invariant 5 constraint (TD-VSDD-053 single-writer discipline).
#
# The grep also confirms that the delegation pattern via state-manager IS
# documented in both SKILL.md files (confirming the delegation, not just the
# absence of direct writes).
#
# RED GATE: factory-lock/SKILL.md and factory-unlock/SKILL.md do not yet exist
# (they are S-17.03 creation targets). The [ -f ] assertions fail immediately.
# ---------------------------------------------------------------------------

@test "test_BC_6_23_001_skill_mds_contain_no_direct_state_write" {
  local lock_skill="$SKILL_DIR/factory-lock/SKILL.md"
  local unlock_skill="$SKILL_DIR/factory-unlock/SKILL.md"

  # Both files must exist (created by S-17.03 T-6/T-7)
  [ -f "$lock_skill" ]
  [ -f "$unlock_skill" ]

  # factory-lock/SKILL.md MUST NOT contain a direct "Write STATE.md" instruction
  # (the skill delegates to state-manager; it never calls Write directly on STATE.md)
  if grep -q 'Write STATE\.md' "$lock_skill"; then
    printf 'FAIL: factory-lock/SKILL.md contains a direct Write STATE.md instruction\n' >&2
    printf 'This violates BC-6.23.001 Invariant 5 (state-manager delegation required)\n' >&2
    false
  fi

  # factory-unlock/SKILL.md MUST NOT contain a direct "Write STATE.md" instruction
  if grep -q 'Write STATE\.md' "$unlock_skill"; then
    printf 'FAIL: factory-unlock/SKILL.md contains a direct Write STATE.md instruction\n' >&2
    printf 'This violates BC-6.23.001 Invariant 5 (state-manager delegation required)\n' >&2
    false
  fi

  # Both SKILL.md files MUST document the state-manager delegation path
  # (the instruction must delegate to state-manager, not perform writes directly)
  grep -q 'state-manager' "$lock_skill"
  grep -q 'state-manager' "$unlock_skill"
}

# ---------------------------------------------------------------------------
# test_BC_6_23_001_concurrent_acquire_cas_race_one_wins_one_rejected
# AC-013 / BC-6.23.001 T-4/T-10 — concurrent acquire CAS race tiebreaker
#
# Simulates two concurrent /factory-lock invocations (dev-a@x.com and
# dev-b@x.com) that both see an unlocked STATE.md and both attempt acquire.
# The CAS push is the tiebreaker: one succeeds, one gets CASPushRejected.
#
# Test strategy — models the REAL state-manager acquire sequence:
#   acquire = factory-lock-write.sh acquire → git add → git commit → factory-cas-push.sh
#
# The RACE is deterministically simulated via a race-injecting git shim (same
# pattern as test_BC_5_40_001_cas_push_rejected_on_concurrent_write in
# factory-cas-push.bats). The shim intercepts clone B's `git fetch` inside
# factory-cas-push.sh: after the real fetch completes (B captures EXPECTED_SHA
# = INIT_SHA, reflecting the pre-race remote state), the shim THEN injects
# clone A's push into the bare repo, advancing the remote to SHA_A. When B's
# cas-push then executes `git push --force-with-lease=factory-artifacts:INIT_SHA`,
# the remote is at SHA_A ≠ INIT_SHA → --force-with-lease fires → CASPushRejected.
#
# Without the shim, clone B's cas-push would fetch AFTER clone A pushed, capture
# EXPECTED_SHA=SHA_A, and succeed as a force push (lease check passes because
# remote == EXPECTED_SHA). The shim is required for deterministic race simulation.
#
# Fixture strategy — PRODUCTION-FAITHFUL (.factory path exercised):
#   Each "session" has a session working directory (session-a / session-b) with a
#   .factory symlink pointing at the respective clone (clone-a / clone-b). This
#   mirrors the production layout: main worktree root → .factory/ worktree.
#   factory-cas-push.sh uses `git -C .factory` (hardcoded) for all git operations,
#   so the .factory symlink resolves to the correct clone for each session.
#
# Precheck invocation: factory-lock-acquire-precheck.sh uses bare `git fetch`
# and `git config user.email` (no -C), so it must be called with CWD = a git
# repo. We run it from clone dirs directly (both are factory-artifacts repos
# with `origin` pointing at the bare repo). This is equivalent to the production
# context where the precheck runs from the main project root (also a git repo).
#
# Sequence:
#   1. Both clones run precheck from their clone dirs → both return PROCEED_ACQUIRE.
#   2. Both clones write+commit locally: write lock → git add → git commit.
#   3. Clone A runs CAS push from session-a WITHOUT shim → succeeds (first push).
#   4. Clone B runs CAS push from session-b WITH race-injecting shim:
#        shim fetch: real fetch returns INIT_SHA to B's tracking, THEN injects A's push
#        B captures EXPECTED_SHA=INIT_SHA; pushes with lease=INIT_SHA;
#        remote is now at SHA_A → lease check fires → REJECTED.
#   5. Assert: A push exit 0; B push non-zero + CASPushRejected message.
#   6. Assert: dev-a@x.com holds the lock on the bare remote (clone A's STATE.md).
# ---------------------------------------------------------------------------

@test "test_BC_6_23_001_concurrent_acquire_cas_race_one_wins_one_rejected" {
  _setup_two_clone_fixture

  local state_a="$CLONE_A/STATE.md"
  local state_b="$CLONE_B/STATE.md"

  # ---------------------------------------------------------------------------
  # Build production-faithful session directories:
  #   session-a and session-b each have a .factory symlink → respective clone.
  # factory-cas-push.sh uses `git -C .factory` (hardcoded), so running it from
  # session-a/session-b resolves .factory to the correct clone via the symlink.
  # ---------------------------------------------------------------------------
  local session_a="$WORK/session-a"
  local session_b="$WORK/session-b"
  mkdir -p "$session_a" "$session_b"
  ln -s "$CLONE_A" "$session_a/.factory"
  ln -s "$CLONE_B" "$session_b/.factory"

  # ---------------------------------------------------------------------------
  # Step 1: Both clones run precheck (from clone dirs — git repo CWD required).
  # Remote is at INIT_SHA (unlocked). Both return PROCEED_ACQUIRE.
  # ---------------------------------------------------------------------------
  run bash -c "cd '${CLONE_A}' && bash '${PRECHECK_HELPER}' '${state_a}'"
  [ "$status" -eq 0 ]
  [[ "$output" == *"PROCEED_ACQUIRE"* ]]

  run bash -c "cd '${CLONE_B}' && bash '${PRECHECK_HELPER}' '${state_b}'"
  [ "$status" -eq 0 ]
  [[ "$output" == *"PROCEED_ACQUIRE"* ]]

  # ---------------------------------------------------------------------------
  # Step 2: Both clones write+commit their lock locally (against INIT_SHA base).
  # factory-lock-write.sh ONLY modifies STATE.md — it does NOT commit.
  # The state-manager commit step is modelled explicitly here (TD-VSDD-053:
  # write → commit → CAS push is the atomic acquire sequence).
  #
  # CWD discipline: factory-lock-write.sh resolves the holder via bare
  # `git config user.email` (no -C), so it MUST be invoked with CWD = the
  # relevant clone directory (which has user.email configured repo-locally).
  # This mirrors production context where state-manager runs from the project
  # root (also a git repo with user.email in its local config).
  # ---------------------------------------------------------------------------
  run bash -c "cd '${CLONE_A}' && bash '${LOCK_WRITE_HELPER}' acquire '${state_a}'"
  [ "$status" -eq 0 ]
  git -C "$CLONE_A" add STATE.md >/dev/null 2>&1
  git -C "$CLONE_A" commit -m "acquire lock (dev-a)" >/dev/null 2>&1

  run bash -c "cd '${CLONE_B}' && bash '${LOCK_WRITE_HELPER}' acquire '${state_b}'"
  [ "$status" -eq 0 ]
  git -C "$CLONE_B" add STATE.md >/dev/null 2>&1
  git -C "$CLONE_B" commit -m "acquire lock (dev-b)" >/dev/null 2>&1

  # ---------------------------------------------------------------------------
  # Step 3: Clone A — CAS push from session-a (no shim, first push WINS).
  # factory-cas-push.sh: fetch → EXPECTED_SHA=INIT_SHA → push succeeds.
  # Remote advances to SHA_A (clone A's lock commit).
  # ---------------------------------------------------------------------------
  run bash -c "cd '${session_a}' && bash '${CAS_PUSH_HELPER}'"
  cas_push_a_status="$status"
  cas_push_a_output="$output"

  [ "$cas_push_a_status" -eq 0 ]

  # ---------------------------------------------------------------------------
  # Step 4: Clone B — CAS push from session-b WITH race-injecting shim (LOSES).
  #
  # Shim intercepts clone B's `git fetch` inside factory-cas-push.sh:
  #   - Runs the real fetch (B's tracking: origin/factory-artifacts = INIT_SHA,
  #     because at the moment the shim's REAL fetch runs, A's push happened BEFORE
  #     this step — but wait, A already pushed in Step 3, so B's real fetch WILL
  #     see SHA_A. We need B to see INIT_SHA.
  #
  # Revised shim strategy: stub the FETCH to return exit 0 without actually
  # fetching (B's tracking stays at INIT_SHA from the precheck's fetch in Step 1).
  # Then EXPECTED_SHA = INIT_SHA (B's stale tracking ref). Then push with
  # lease=INIT_SHA but remote is SHA_A → lease fires → REJECTED.
  #
  # This accurately models the race: B fetched during precheck (INIT_SHA), decided
  # PROCEED_ACQUIRE, then wrote+committed, then attempted push — but A beat it to
  # the remote between precheck and push. The fetch inside cas-push.sh is stubbed
  # to represent B not re-fetching (or fetching too quickly before A's push landed),
  # leaving EXPECTED_SHA at INIT_SHA.
  # ---------------------------------------------------------------------------
  local stub_bin="$WORK/stub-bin-b"
  mkdir -p "$stub_bin"

  cat > "$stub_bin/git" <<SHIM
#!/usr/bin/env bash
# Race-injecting git shim for clone B's cas-push in the concurrent race test.
# Stubs 'fetch' to return exit 0 without updating the tracking ref, leaving
# origin/factory-artifacts at INIT_SHA (the value from clone B's precheck fetch).
# All other subcommands delegate to the real git.
REAL_GIT="${REAL_GIT_PATH}"

args=("\$@")
i=0
subcommand=""
while [ \$i -lt \${#args[@]} ]; do
  arg="\${args[\$i]}"
  if [ "\$arg" = "-C" ]; then
    i=\$(( i + 1 ))
  else
    subcommand="\$arg"
    break
  fi
  i=\$(( i + 1 ))
done

case "\$subcommand" in
  fetch)
    # Stub: do NOT fetch — leave B's tracking at INIT_SHA (stale).
    # This models B having fetched during precheck but not re-fetching before push.
    exit 0
    ;;
  *)
    "\$REAL_GIT" "\$@"
    ;;
esac
SHIM
  chmod +x "$stub_bin/git"

  run bash -c "export PATH='${stub_bin}:${PATH}'; cd '${session_b}' && bash '${CAS_PUSH_HELPER}'"
  cas_push_b_status="$status"
  cas_push_b_output="$output"

  # Clone B's push must be rejected (remote has advanced past INIT_SHA)
  [ "$cas_push_b_status" -ne 0 ]

  # Must emit the CASPushRejected message (AC-005 / BC-6.23.001 AC-013)
  [[ "$cas_push_b_output" == *"state-burst CAS push failed — concurrent write detected."* ]] || \
    [[ "$cas_push_b_output" == *"CASPushRejected"* ]] || \
    [[ "$cas_push_b_output" == *"AcquireRaceRejected"* ]]

  # ---------------------------------------------------------------------------
  # Final assertions: clone A holds the lock; clone A succeeded; clone B rejected
  # ---------------------------------------------------------------------------
  grep -q 'dev-a@x.com' "$state_a"
  [ "$cas_push_a_status" -eq 0 ]
  [ "$cas_push_b_status" -ne 0 ]
}
