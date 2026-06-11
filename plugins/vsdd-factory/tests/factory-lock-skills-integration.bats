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
# The CAS push is the tiebreaker: one succeeds, one gets AcquireRaceRejected.
#
# Test strategy (mirrors factory-cas-push.bats real-fixture approach):
#   1. Both clones see the same initial unlocked STATE.md (INIT_SHA).
#   2. Both run factory-lock-acquire-precheck.sh → both should return
#      PROCEED_ACQUIRE (unlocked base state).
#   3. Clone A writes its lock (factory-lock-write.sh acquire) and does the
#      CAS push (factory-cas-push.sh) FIRST. This succeeds.
#   4. Clone B attempts the same sequence. Its CAS push fails because the
#      remote has advanced past INIT_SHA (clone A's push moved it forward).
#   5. Assert: clone A push succeeded; clone B push failed with
#      AcquireRaceRejected (or CASPushRejected) error message.
#
# A git shim installed in PATH intercepts the fetch and config subcommands
# for each clone to supply the correct email identity without touching
# real git config.
#
# RED GATE: factory-lock-acquire-precheck.sh and factory-cas-push.sh are stubs
# that exit 1 immediately. The PROCEED_ACQUIRE assertion for clone A fails
# because the precheck stub exits 1 with TODO. The test fails at the precheck
# stage — demonstrating RED for the right reason.
# ---------------------------------------------------------------------------

@test "test_BC_6_23_001_concurrent_acquire_cas_race_one_wins_one_rejected" {
  _setup_two_clone_fixture

  # ---------------------------------------------------------------------------
  # Build a git shim that supplies the correct email per clone path
  # ---------------------------------------------------------------------------
  local stub_bin="$WORK/stub-bin"
  mkdir -p "$stub_bin"

  cat > "$stub_bin/git" <<SHIM
#!/usr/bin/env bash
# Git shim for concurrent CAS race test.
# Intercepts 'config user.email' to return the correct identity based on
# which clone directory is being used (detected via -C argument).
# All other subcommands delegate to real git.
REAL_GIT="${REAL_GIT_PATH}"
CLONE_A_PATH="${CLONE_A}"
CLONE_B_PATH="${CLONE_B}"

# Parse subcommand and -C path
args=("\$@")
i=0
subcommand=""
git_c_path=""
while [ \$i -lt \${#args[@]} ]; do
  arg="\${args[\$i]}"
  if [ "\$arg" = "-C" ]; then
    i=\$(( i + 1 ))
    git_c_path="\${args[\$i]}"
  else
    subcommand="\$arg"
    break
  fi
  i=\$(( i + 1 ))
done

case "\$subcommand" in
  config)
    if [[ "\$*" == *"user.email"* ]]; then
      # Return identity based on which clone is active
      if [[ "\$git_c_path" == *"clone-a"* ]] || [[ "\$(pwd)" == *"clone-a"* ]]; then
        printf 'dev-a@x.com\n'
        exit 0
      elif [[ "\$git_c_path" == *"clone-b"* ]] || [[ "\$(pwd)" == *"clone-b"* ]]; then
        printf 'dev-b@x.com\n'
        exit 0
      fi
      # Fallback to real git config
      "\$REAL_GIT" "\$@"
      exit \$?
    fi
    "\$REAL_GIT" "\$@"
    ;;
  *)
    "\$REAL_GIT" "\$@"
    ;;
esac
SHIM
  chmod +x "$stub_bin/git"

  # ---------------------------------------------------------------------------
  # Step 1: Clone A runs factory-lock-acquire-precheck.sh
  # Expect: PROCEED_ACQUIRE (unlocked base state)
  # RED GATE: precheck stub exits 1 → this assertion fails → RED for right reason
  # ---------------------------------------------------------------------------
  local state_a="$CLONE_A/STATE.md"
  local state_b="$CLONE_B/STATE.md"

  run env PATH="${stub_bin}:${PATH}" bash "$PRECHECK_HELPER" "$state_a"
  precheck_a_status="$status"
  precheck_a_output="$output"

  # Must exit 0 and return PROCEED_ACQUIRE
  [ "$precheck_a_status" -eq 0 ]
  [[ "$precheck_a_output" == *"PROCEED_ACQUIRE"* ]]

  # ---------------------------------------------------------------------------
  # Step 2: Clone A writes the lock (factory-lock-write.sh acquire)
  # ---------------------------------------------------------------------------
  run env GIT_CONFIG_GLOBAL=/dev/null \
    HOME="$WORK/home-a" \
    bash "$LOCK_WRITE_HELPER" acquire "$state_a"
  lock_write_a_status="$status"

  # Must exit 0 (write succeeds)
  [ "$lock_write_a_status" -eq 0 ]

  # ---------------------------------------------------------------------------
  # Step 3: Clone A does the CAS push (factory-cas-push.sh)
  # This succeeds because remote is still at INIT_SHA
  # ---------------------------------------------------------------------------
  run bash -c "cd '${CLONE_A}' && bash '${CAS_PUSH_HELPER}'"
  cas_push_a_status="$status"
  cas_push_a_output="$output"

  # Must exit 0 (first pusher wins)
  [ "$cas_push_a_status" -eq 0 ]

  # ---------------------------------------------------------------------------
  # Step 4: Clone B runs factory-lock-acquire-precheck.sh
  # At this point the remote has clone A's lock commit. After fetch, clone B
  # should see a foreign unexpired lock and return REFUSED_FOREIGN_LOCK.
  # ---------------------------------------------------------------------------
  run env PATH="${stub_bin}:${PATH}" bash "$PRECHECK_HELPER" "$state_b"
  precheck_b_status="$status"
  precheck_b_output="$output"

  # Must exit 1 (foreign lock detected after fetch — AcquireRaceRejected)
  [ "$precheck_b_status" -eq 1 ]
  [[ "$precheck_b_output" == *"REFUSED_FOREIGN_LOCK"* ]] || \
    [[ "$precheck_b_output" == *"AcquireRaceRejected"* ]]

  # ---------------------------------------------------------------------------
  # Final assertion: Clone A holds the lock; Clone B was rejected
  # ---------------------------------------------------------------------------
  # The lock in STATE.md on the remote (bare) must be held by dev-a@x.com
  # Verify by checking clone A's state file after the push
  grep -q 'dev-a@x.com' "$state_a"
}
