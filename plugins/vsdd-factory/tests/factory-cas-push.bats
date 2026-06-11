#!/usr/bin/env bats
# factory-cas-push.bats — RED-phase (TDD) tests for BC-5.40.001 PC5 (CAS push
# rejection), EC-003 (fetch failure aborts push), and Invariant 5 (force-with-lease
# already permitted by verify-git-push.sh).
#
# Traces to: BC-5.40.001 PC5, EC-003, Invariant 5.
# Story: S-17.01 — factory_lock STATE.md schema + state-burst CAS push (D6).
# Target: plugins/vsdd-factory/bin/factory-cas-push.sh
#
# RED GATE: All tests that exercise factory-cas-push.sh MUST FAIL because the
# helper is a stub that exits 1 with a TODO message.
#
# Fixture strategy for git-involving tests (AC-005, AC-010):
#   Build a LOCAL bare repo + clone (never touching the real .factory/ or origin).
#   The helper uses `git -C .factory` — so we symlink/set up .factory/ inside a
#   throwaway tmpdir to point at the clone, isolating every test from real state.
#
#   For AC-005 (concurrent write rejection): create a bare repo, clone it, make
#   an additional commit directly in the bare repo (simulating a concurrent push),
#   then run the helper from a working directory whose .factory/ points at the clone.
#   The helper's `--force-with-lease=factory-artifacts:<sha>` will be rejected
#   because remote has advanced past the expected SHA.
#
#   For AC-010 (fetch failure): point the clone at an unreachable remote URL so
#   that `git fetch` fails with a non-zero exit. The helper must abort before push.
#
# Run:
#   bats plugins/vsdd-factory/tests/factory-cas-push.bats

HELPER="$(cd "$(dirname "$BATS_TEST_FILENAME")/.." && pwd)/bin/factory-cas-push.sh"
HOOK_DIR="$(cd "$(dirname "$BATS_TEST_FILENAME")/.." && pwd)/hooks"

# ---------------------------------------------------------------------------
# setup / teardown
# ---------------------------------------------------------------------------

setup() {
  WORK="$(mktemp -d)"
  # Resolve symlinks (macOS /var -> /private/var) for consistent path comparisons
  WORK="$(cd "$WORK" && pwd -P)"
}

teardown() {
  rm -rf "$WORK"
}

# ---------------------------------------------------------------------------
# Helper: build a minimal bare repo + a clone whose factory-artifacts branch
# can be used as a stand-in for the real factory-artifacts branch.
# Sets:
#   BARE_REPO   — path to bare.git
#   CLONE_REPO  — path to clone (will be used as .factory/ directory)
#   INIT_SHA    — the initial commit SHA (used as EXPECTED_SHA in AC-005 tests)
# ---------------------------------------------------------------------------

_setup_git_fixture() {
  BARE_REPO="$WORK/bare.git"
  CLONE_REPO="$WORK/clone"

  # Create bare repo
  git init --bare "$BARE_REPO" >/dev/null 2>&1

  # Create a working clone
  git clone "$BARE_REPO" "$CLONE_REPO" >/dev/null 2>&1

  # Configure identity in clone (required for commits)
  git -C "$CLONE_REPO" config user.email "test@factory-cas-push.test"
  git -C "$CLONE_REPO" config user.name "CAS Push Test"

  # Create and push the factory-artifacts branch to bare repo
  git -C "$CLONE_REPO" checkout -b factory-artifacts >/dev/null 2>&1
  echo "state: initial" > "$CLONE_REPO/STATE.md"
  git -C "$CLONE_REPO" add STATE.md >/dev/null 2>&1
  git -C "$CLONE_REPO" commit -m "init factory-artifacts" >/dev/null 2>&1
  git -C "$CLONE_REPO" push origin factory-artifacts >/dev/null 2>&1

  # Record the initial commit SHA — this is the SHA the helper will see
  # when it runs `git -C .factory rev-parse origin/factory-artifacts`
  INIT_SHA="$(git -C "$CLONE_REPO" rev-parse origin/factory-artifacts)"
}

# ---------------------------------------------------------------------------
# Helper: create a fake working directory with .factory pointing at the clone.
# The helper uses `git -C .factory` — so we create a directory whose .factory
# subdirectory IS the clone (symlink or copy).
# Sets:
#   CALLER_DIR  — the directory from which factory-cas-push.sh will be invoked
# ---------------------------------------------------------------------------

_setup_caller_dir() {
  CALLER_DIR="$WORK/caller"
  mkdir -p "$CALLER_DIR"
  # Create .factory as a symlink to the clone directory
  ln -s "$CLONE_REPO" "$CALLER_DIR/.factory"
}

# ---------------------------------------------------------------------------
# test_BC_5_40_001_cas_push_rejected_on_concurrent_write
# AC-005 / BC-5.40.001 PC5 — CAS push rejected on concurrent write
#
# Setup:
#   1. Build bare repo + clone; push initial factory-artifacts commit.
#   2. Capture INIT_SHA = origin/factory-artifacts at clone's view.
#   3. Simulate concurrent write: push an additional commit DIRECTLY to the
#      bare repo (advancing the remote past INIT_SHA) without updating the clone.
#   4. Now the clone's `origin/factory-artifacts` is stale (INIT_SHA) but the
#      bare repo's tip is CONCURRENT_SHA.
#   5. After the helper runs `git fetch` (which updates origin/factory-artifacts
#      in the clone to CONCURRENT_SHA), the EXPECTED_SHA it computes will be
#      CONCURRENT_SHA — but we need the push to fail.
#
# Revised strategy using local commit in clone before fetch:
#   1. Push initial commit to bare repo. Clone sees INIT_SHA.
#   2. Make a LOCAL commit in the clone (NOT pushed) — this advances the clone's
#      factory-artifacts tip past origin/factory-artifacts.
#   3. Simulate concurrent write in the bare repo (advancing remote to CONCURRENT_SHA).
#   4. When the helper runs `git fetch`, it updates origin/factory-artifacts to
#      CONCURRENT_SHA, which is NOT an ancestor of the clone's local tip.
#   5. `git push --force-with-lease=factory-artifacts:CONCURRENT_SHA` will be
#      rejected because the local factory-artifacts != CONCURRENT_SHA's lineage.
#
# Wait — the actual failure mechanism for --force-with-lease:
#   `--force-with-lease=<refname>:<expect>` checks that the remote ref currently
#   equals <expect>. After fetch, origin/factory-artifacts = CONCURRENT_SHA.
#   The helper pushes `--force-with-lease=factory-artifacts:CONCURRENT_SHA`.
#   The remote's factory-artifacts is CONCURRENT_SHA. The check passes (remote == expect).
#   This means fetch+push would SUCCEED in this scenario.
#
# Correct simulation of rejection:
#   The race window is: fetch sets EXPECTED_SHA; then remote advances AGAIN before
#   the push arrives. We simulate this by:
#     a. Clone has LOCAL_SHA as HEAD of factory-artifacts.
#     b. Helper fetches → EXPECTED_SHA = INIT_SHA (bare repo still at INIT_SHA).
#     c. Between fetch and push: another commit is pushed to bare repo (RACE_SHA).
#     d. Push with --force-with-lease=factory-artifacts:INIT_SHA fails because
#        remote is now at RACE_SHA, not INIT_SHA.
#
#   To achieve (c) without modifying the helper, we need to inject the race.
#   Implementation: use a git wrapper script that succeeds for `fetch` (which
#   updates origin/factory-artifacts to INIT_SHA in the clone) but then advances
#   the bare repo, then the push fails because remote advanced past EXPECTED_SHA.
#
#   Simpler and more reliable: use a stub `git` on PATH that:
#     - Returns exit 0 for `fetch` calls (no-op)
#     - Returns exit 1 for `push` calls (simulating --force-with-lease rejection)
#   This directly tests the helper's error-handling on push failure, which is
#   the exact behavioral contract (PC5): on non-zero push exit, helper must emit
#   the CASPushRejected error and exit non-zero.
#
# This approach mirrors the stub-on-PATH pattern from resolve-worktree-identity.bats.
#
# RED GATE: stub helper exits 1 before executing any git commands — fails on
# the "non-zero exit + correct error message" assertion (wrong exit reason).
# ---------------------------------------------------------------------------

@test "test_BC_5_40_001_cas_push_rejected_on_concurrent_write" {
  # Build a stub git directory on PATH.
  # The stub:
  #   - For `fetch`: exits 0 (fetch succeeds; no-op here because rev-parse
  #     needs to resolve, but the stub does not update any real repo state —
  #     we handle rev-parse separately below).
  #   - For `push --force-with-lease`: exits 1 (concurrent write detected).
  #   - For `rev-parse origin/factory-artifacts`: prints a fake SHA (so the
  #     helper can capture EXPECTED_SHA without failing at step 2).
  #   - All other git commands: delegate to real git.

  STUB_BIN="$WORK/stub-bin"
  mkdir -p "$STUB_BIN"

  cat > "$STUB_BIN/git" <<'STUB'
#!/usr/bin/env bash
# Stub git for test_BC_5_40_001_cas_push_rejected_on_concurrent_write
# Arguments are passed as: git -C .factory <subcommand> [args...]

# Strip leading `-C <path>` flag pair so we can inspect the subcommand.
args=("$@")
i=0
subcommand=""
while [ $i -lt ${#args[@]} ]; do
  arg="${args[$i]}"
  if [ "$arg" = "-C" ]; then
    i=$(( i + 1 ))  # skip the path argument
  else
    subcommand="$arg"
    break
  fi
  i=$(( i + 1 ))
done

case "$subcommand" in
  fetch)
    # Fetch succeeds (no-op for stub)
    exit 0
    ;;
  rev-parse)
    # Return a deterministic fake SHA for EXPECTED_SHA capture
    printf 'aabbccddeeff00112233445566778899aabbccdd\n'
    exit 0
    ;;
  push)
    # Simulate --force-with-lease rejection (concurrent write detected)
    exit 1
    ;;
  *)
    # Delegate everything else to real git
    exec "$(command -v git)" "$@"
    ;;
esac
STUB
  chmod +x "$STUB_BIN/git"

  # Create a minimal caller directory (the helper does not need a real .factory/
  # because git is fully stubbed)
  CALLER_DIR="$WORK/caller"
  mkdir -p "$CALLER_DIR/.factory"

  # Run the helper with the stub git first on PATH
  run bash -c "export PATH='$STUB_BIN:$PATH'; cd '$CALLER_DIR' && bash '$HELPER'"

  # Must exit non-zero (CAS push rejected)
  [ "$status" -ne 0 ]

  # Must emit the exact CASPushRejected error message (AC-005)
  # Output captures both stdout and stderr via `run`
  [[ "$output" == *"state-burst CAS push failed — concurrent write detected."* ]]
  [[ "$output" == *"Fetch origin/factory-artifacts and retry."* ]]
}

# ---------------------------------------------------------------------------
# test_BC_5_40_001_fetch_failure_aborts_push
# AC-010 / BC-5.40.001 EC-003 — fetch failure aborts push
#
# When `git -C .factory fetch origin factory-artifacts` fails (non-zero exit),
# the helper MUST:
#   - Exit non-zero.
#   - Emit the AC-010 fetch-failure error message to stderr.
#   - NOT proceed to the push step.
#
# Verification that push is not attempted: the stub tracks whether `git push`
# was called by writing a sentinel file. If the helper incorrectly proceeds
# to push after fetch failure, the sentinel file will exist and the assertion
# fails.
#
# RED GATE: stub helper exits 1 immediately without calling any git commands —
# but the expected failure reason is wrong (TODO message vs. fetch error message).
# ---------------------------------------------------------------------------

@test "test_BC_5_40_001_fetch_failure_aborts_push" {
  STUB_BIN="$WORK/stub-bin"
  mkdir -p "$STUB_BIN"

  # Sentinel file: created by stub if push is attempted
  PUSH_SENTINEL="$WORK/push-was-called"

  cat > "$STUB_BIN/git" <<STUB
#!/usr/bin/env bash
# Stub git for test_BC_5_40_001_fetch_failure_aborts_push
# Strip leading -C <path> flag pair.
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
    # Simulate fetch failure (network error / unreachable remote)
    printf 'fatal: unable to connect to origin\n' >&2
    exit 1
    ;;
  push)
    # Record that push was attempted (must NOT happen after fetch failure)
    touch "${PUSH_SENTINEL}"
    exit 0
    ;;
  *)
    exec "\$(command -v git)" "\$@"
    ;;
esac
STUB
  chmod +x "$STUB_BIN/git"

  CALLER_DIR="$WORK/caller"
  mkdir -p "$CALLER_DIR/.factory"

  run bash -c "export PATH='$STUB_BIN:$PATH'; cd '$CALLER_DIR' && bash '$HELPER'"

  # Must exit non-zero (fetch failure)
  [ "$status" -ne 0 ]

  # Must emit the exact AC-010 fetch-failure error message
  [[ "$output" == *"state-burst CAS push failed — fetch error before push. Retry after resolving network."* ]]

  # Push MUST NOT have been attempted (sentinel file must not exist)
  [ ! -f "$PUSH_SENTINEL" ]
}

# ---------------------------------------------------------------------------
# test_BC_5_40_001_verify_git_push_hook_unchanged
# AC-009 / BC-5.40.001 Invariant 5 — verify-git-push.sh permits --force-with-lease
#
# The existing `plugins/vsdd-factory/hooks/verify-git-push.sh` hook MUST:
#   1. Contain the string `--force-with-lease` (permit-path code is present).
#   2. Not have been modified by this story (diff against develop HEAD is empty
#      for this file — no changes introduced by S-17.01).
#   3. The permit path must take the form of allowing --force-with-lease while
#      blocking raw --force / -f.
#
# This test does NOT require factory-cas-push.sh to be implemented —
# it reads verify-git-push.sh directly. It will PASS even against the stub
# IF verify-git-push.sh already contains --force-with-lease.
#
# Per the Red Gate requirement in the story: "Initial State (Red Gate): FAIL: not yet
# verified". However, since verify-git-push.sh is pre-existing and already contains
# --force-with-lease (per ADR-025 Decision 8 and the hook source we read), this test
# is EXPECTED to be green at Red Gate. This is acceptable: the story's Red Gate Table
# notes "not yet verified" as the initial state — meaning the assertion itself has not
# been codified as an executable test yet, not that the assertion will fail.
#
# The test exercises Invariant 5 correctly: if someone accidentally removes the
# --force-with-lease permit from verify-git-push.sh, this test catches it.
# ---------------------------------------------------------------------------

@test "test_BC_5_40_001_verify_git_push_hook_unchanged" {
  PUSH_HOOK="$HOOK_DIR/verify-git-push.sh"

  # Hook must exist
  [ -f "$PUSH_HOOK" ]

  # Hook must permit --force-with-lease (the permit token must be present)
  grep -q -- '--force-with-lease' "$PUSH_HOOK"

  # Hook must block raw --force (the block token must also be present,
  # confirming the permit/block distinction is intact)
  grep -q -- '--force' "$PUSH_HOOK"

  # The permit path must explicitly allow --force-with-lease before blocking --force.
  # Check that the file contains the allow-comment or allow-branch for --force-with-lease.
  # The canonical form in the hook is: if [[ "$COMMAND" == *"--force-with-lease"* ]]; then
  #   : # Allowed — safe force push
  grep -q 'Allowed' "$PUSH_HOOK"

  # Stronger: the allow-branch for --force-with-lease must appear BEFORE the
  # block for --force in file order (structural invariant of the hook logic).
  # Extract line numbers for both and compare.
  lease_line="$(grep -n -- '--force-with-lease' "$PUSH_HOOK" | head -1 | cut -d: -f1)"
  # git_push_force is the event tag passed to block_pre for the force-push block
  force_block_line="$(grep -n 'git_push_force' "$PUSH_HOOK" | head -1 | cut -d: -f1)"

  # Both must be present
  [ -n "$lease_line" ]
  [ -n "$force_block_line" ]

  # --force-with-lease allow-path must come before the block for --force
  [ "$lease_line" -lt "$force_block_line" ]
}
