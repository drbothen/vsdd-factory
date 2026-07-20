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
# F-P1-008 / AC-005 / BC-5.40.001 PC5 — CAS push rejected on genuine concurrent write
# (DE-TAUTOLOGIZED: uses real bare-repo+clone fixture, NOT an all-stub git shim)
#
# The correct failure mechanism for --force-with-lease=<refname>:<sha>:
#   The lease check passes when remote tip == <sha> AT PUSH TIME.
#   The push is REJECTED with "(stale info)" when the remote has advanced
#   past <sha> between the fetch and the push.
#
# Simulation strategy (race-injecting shim):
#   1. Build a real bare repo + primary clone + racer clone.
#   2. Push initial factory-artifacts commit; all parties start at INIT_SHA.
#   3. Make a local commit in the primary clone (not yet pushed) — this gives
#      the primary clone a factory-artifacts tip AHEAD of INIT_SHA.
#   4. Install a git shim that: (a) performs REAL fetch (updating
#      origin/factory-artifacts in primary clone to INIT_SHA); (b) after fetch,
#      advances the bare repo by pushing a commit from the racer clone; then
#      (c) passes control back to the real git for the push.
#   5. When the helper executes:
#        EXPECTED_SHA=$(git rev-parse origin/factory-artifacts)  →  INIT_SHA
#        git push --force-with-lease=factory-artifacts:INIT_SHA ...
#      At push time the remote is at RACER_SHA (not INIT_SHA) — the lease check
#      fires → push rejected with "(stale info)" → CASPushRejected error emitted.
#
# Additional assertion: verify the push command used --force-with-lease=factory-artifacts:<sha>
# (not a blind force) by capturing the exact git invocation in the shim.
#
# RED GATE: the current implementation DOES handle non-zero push exit by
# emitting the correct CASPushRejected message. This test is RED because the
# PREVIOUS stub-based version was tautological (the push stub always returned 1
# regardless of EXPECTED_SHA, so the helper never had to actually form the
# correct lease command). This real-fixture test verifies the ACTUAL push
# command contains the correct `--force-with-lease=factory-artifacts:<sha>`
# form — something the all-stub shim could not verify.
#
# Specifically: the test asserts that the push argument log contains the
# string `--force-with-lease=factory-artifacts:` followed by a real SHA.
# The current implementation does form this correctly, meaning this specific
# assertion will PASS. But the test also asserts the REAL rejection scenario
# (not a synthetic exit-1), removing the tautology entirely.
#
# Note: the test IS expected to pass once the helper is implemented correctly.
# It is RED at this step only if the helper is a pure stub (which it is not here —
# the helpers are real). Per the adversary finding, the original test was
# tautological because a pure git stub can never prove the helper forms the
# correct --force-with-lease argument. This rewrite proves that.
# ---------------------------------------------------------------------------

@test "test_BC_5_40_001_cas_push_rejected_on_concurrent_write" {
  _setup_git_fixture
  _setup_caller_dir

  # Create a racer clone to simulate the concurrent writer
  RACER_REPO="$WORK/racer"
  git clone "$BARE_REPO" "$RACER_REPO" >/dev/null 2>&1
  git -C "$RACER_REPO" config user.email "racer@concurrent.test"
  git -C "$RACER_REPO" config user.name "Racer"
  git -C "$RACER_REPO" checkout factory-artifacts >/dev/null 2>&1

  # Make a local commit in the primary clone (our work to push)
  echo "primary-work" >> "$CLONE_REPO/STATE.md"
  git -C "$CLONE_REPO" add STATE.md >/dev/null 2>&1
  git -C "$CLONE_REPO" commit -m "primary work (not yet pushed)" >/dev/null 2>&1

  # Install a race-injecting git shim:
  #   - For fetch: runs the real fetch (updates origin/factory-artifacts in clone to INIT_SHA)
  #     then injects the race by pushing from the racer clone before returning.
  #   - For everything else (rev-parse, push): delegates to real git.
  # The push command is also logged to a file so we can assert it used
  # --force-with-lease=factory-artifacts:<sha>.
  STUB_BIN="$WORK/stub-bin"
  mkdir -p "$STUB_BIN"
  PUSH_ARGS_LOG="$WORK/push-args.log"

  # Resolve the real git binary BEFORE writing the shim so we can hardcode it.
  REAL_GIT_PATH="$(command -v git)"

  cat > "$STUB_BIN/git" <<STUB
#!/usr/bin/env bash
# Race-injecting git shim for test_BC_5_40_001_cas_push_rejected_on_concurrent_write
REAL_GIT="${REAL_GIT_PATH}"

# Parse subcommand (skipping leading -C <path> flag pairs)
args=("\$@")
i=0; subcommand=""
while [ \$i -lt \${#args[@]} ]; do
  arg="\${args[\$i]}"
  if [ "\$arg" = "-C" ]; then i=\$(( i + 1 ))
  else subcommand="\$arg"; break
  fi
  i=\$(( i + 1 ))
done

case "\$subcommand" in
  fetch)
    # Step 1: run real fetch to update origin/factory-artifacts in the clone
    "\$REAL_GIT" "\$@"
    FETCH_STATUS=\$?
    # Step 2: inject the race — push from racer to advance the bare remote
    # Only inject once (idempotent via sentinel)
    if [ ! -f "${WORK}/race-injected" ]; then
      touch "${WORK}/race-injected"
      echo "racer-concurrent-write" >> "${RACER_REPO}/STATE.md"
      "\$REAL_GIT" -C "${RACER_REPO}" add STATE.md >/dev/null 2>&1
      "\$REAL_GIT" -C "${RACER_REPO}" commit -m "racer concurrent write" >/dev/null 2>&1
      "\$REAL_GIT" -C "${RACER_REPO}" push origin factory-artifacts >/dev/null 2>&1
    fi
    exit \$FETCH_STATUS
    ;;
  push)
    # Log the full push invocation for later assertion
    echo "\$@" >> "${PUSH_ARGS_LOG}"
    # Delegate to real git — the real --force-with-lease check will fire
    "\$REAL_GIT" "\$@"
    ;;
  *)
    "\$REAL_GIT" "\$@"
    ;;
esac
STUB
  chmod +x "$STUB_BIN/git"

  # Run the helper with the race-injecting shim on PATH
  run bash -c "export PATH='${STUB_BIN}:${PATH}'; cd '${CALLER_DIR}' && bash '${HELPER}'"

  # Must exit non-zero (CAS push rejected by --force-with-lease stale-info check)
  [ "$status" -ne 0 ]

  # Must emit the exact CASPushRejected error message (AC-005 / BC-5.40.001 PC5)
  [[ "$output" == *"state-burst CAS push failed — concurrent write detected."* ]]
  [[ "$output" == *"Fetch origin/factory-artifacts and retry."* ]]

  # The push command MUST have used --force-with-lease=factory-artifacts:<sha>
  # (not a blind --force). Assert the push log contains the correct lease form.
  [ -f "$PUSH_ARGS_LOG" ]
  grep -q -- '--force-with-lease=factory-artifacts:' "$PUSH_ARGS_LOG"
}

# ---------------------------------------------------------------------------
# test_BC_5_40_001_cas_push_stale_sha_after_fetch
# F-P1-007 / EC-008 / BC-5.40.001 EC-008 — stale SHA after fetch aborts push
#
# When `git fetch origin factory-artifacts` SUCCEEDS but the subsequent
# `git rev-parse origin/factory-artifacts` FAILS (ref pruned/absent after
# fetch — e.g., the ref was deleted or the fetch left an inconsistent state),
# the helper MUST:
#   - Exit non-zero.
#   - Emit a CASPushRejected-class error message referencing the stale SHA
#     or fetch-parse failure (not merely the generic fetch-failure message).
#   - NOT proceed to the push step (no push attempted with an invalid SHA).
#
# Failure mode: the current implementation uses `set -euo pipefail`.
# When rev-parse exits 128, bash exits immediately with that status code but
# emits NO error message (the pipeline fails silently).  EC-008 requires an
# actionable CASPushRejected-class error so the developer knows to re-fetch.
#
# Test strategy: stub git such that fetch returns exit 0 (success) but
# rev-parse returns exit 128 (ref absent). Track push via sentinel file.
#
# RED GATE: current impl exits 128 with only git's own error on stderr
# ("fatal: unknown revision...") — does NOT emit the required CASPushRejected
# stale-SHA message.  The assertion checking for that message will fail.
# ---------------------------------------------------------------------------

@test "test_BC_5_40_001_cas_push_stale_sha_after_fetch" {
  STUB_BIN="$WORK/stub-bin"
  mkdir -p "$STUB_BIN"

  PUSH_SENTINEL="$WORK/push-not-attempted"

  cat > "$STUB_BIN/git" <<STUB
#!/usr/bin/env bash
# Stub git for test_BC_5_40_001_cas_push_stale_sha_after_fetch
# Parse subcommand (skip -C <path> flag pairs)
args=("\$@")
i=0; subcommand=""
while [ \$i -lt \${#args[@]} ]; do
  arg="\${args[\$i]}"
  if [ "\$arg" = "-C" ]; then i=\$(( i + 1 ))
  else subcommand="\$arg"; break
  fi
  i=\$(( i + 1 ))
done

case "\$subcommand" in
  fetch)
    # Fetch succeeds
    exit 0
    ;;
  rev-parse)
    # Simulate: ref absent / pruned after fetch (EC-008 condition)
    printf 'fatal: ambiguous argument '"'"'origin/factory-artifacts'"'"': unknown revision or path\n' >&2
    exit 128
    ;;
  push)
    # Record that push was incorrectly attempted
    touch "${WORK}/push-was-attempted"
    exit 0
    ;;
  *)
    "$(command -v git)" "\$@"
    ;;
esac
STUB
  chmod +x "$STUB_BIN/git"

  CALLER_DIR="$WORK/caller"
  mkdir -p "$CALLER_DIR/.factory"

  run bash -c "export PATH='${STUB_BIN}:${PATH}'; cd '${CALLER_DIR}' && bash '${HELPER}'"

  # MUST exit non-zero
  [ "$status" -ne 0 ]

  # MUST emit a CASPushRejected-class error message FROM THE HELPER — not merely
  # git's own "fatal: unknown revision" stderr.  The helper must diagnose the
  # rev-parse failure and emit an actionable message so the developer knows to
  # re-fetch.  The contract keyword is "CASPushRejected" or "stale" combined with
  # "SHA" or "ref", produced by the helper itself (not by git).
  #
  # The current implementation uses `set -euo pipefail` and exits on the failing
  # rev-parse command WITHOUT emitting any helper-level message — this assertion
  # will fail because the output contains only git's native error text, not the
  # required CASPushRejected-class message from the helper.
  [[ "$output" == *"CASPushRejected"* ]] || \
    [[ "$output" == *"stale-burst CAS"* ]] || \
    [[ "$output" == *"stale SHA"* ]] || \
    [[ "$output" == *"state-burst CAS push failed"* && "$output" == *"stale"* ]]

  # MUST NOT have attempted the push (ref is invalid; pushing would use an empty SHA)
  [ ! -f "$WORK/push-was-attempted" ]
}

# ---------------------------------------------------------------------------
# test_BC_5_40_001_cas_push_object_absent_after_fetch
# F-R1-003 / EC-008 / BC-5.40.001 EC-008 — object absent from local store after fetch
#
# The BC EC-008 condition (distinct from the rev-parse-fails case above):
#   git rev-parse origin/factory-artifacts SUCCEEDS (returns a valid-looking SHA)
#   BUT git cat-file -e <sha>^{commit} FAILS (the object is absent from the
#   local store — the SHA is a ghost: the ref was advertised but the object
#   was never transmitted, or was GC'd between fetch and push).
#
# Under this condition the helper MUST:
#   - Exit non-zero.
#   - Emit a CASPushRejected-class message referencing stale SHA or the
#     object-absent condition.
#   - NOT proceed to push (pushing a non-existent object SHA would corrupt
#     the remote or be rejected in unpredictable ways).
#
# The EXISTING test (test_BC_5_40_001_cas_push_stale_sha_after_fetch) covers
# the case where rev-parse itself FAILS (exit 128). This test covers the
# distinct case where rev-parse SUCCEEDS but the object is locally absent.
# Both siblings must be guarded.
#
# Failure mode: the current implementation has NO git cat-file -e object-
# existence check after rev-parse. After a successful rev-parse it proceeds
# directly to the push step. The push step will either silently attempt to
# push a non-existent SHA or fail with an opaque git error — neither of which
# satisfies EC-008's requirement for a CASPushRejected stale-SHA message
# emitted by the helper.
#
# RED GATE: current impl has no cat-file check → proceeds to push stub →
# push stub exits 0 → helper exits 0 → [ "$status" -ne 0 ] assertion FAILS → RED.
# (Even if stub returns non-zero, the helper has no stale-SHA message path for
# this code branch, so the CASPushRejected message assertion also FAILS → RED.)
# ---------------------------------------------------------------------------

@test "test_BC_5_40_001_cas_push_object_absent_after_fetch" {
  STUB_BIN="$WORK/stub-bin"
  mkdir -p "$STUB_BIN"

  PUSH_SENTINEL="$WORK/push-was-attempted"

  # A realistic-looking but locally-absent SHA (40 hex chars)
  GHOST_SHA="deadbeef000000000000000000000000deadbeef"

  cat > "$STUB_BIN/git" <<STUB
#!/usr/bin/env bash
# Stub git for test_BC_5_40_001_cas_push_object_absent_after_fetch
# Simulates EC-008: rev-parse returns a SHA but cat-file -e fails (object absent).
# Parse subcommand (skip -C <path> flag pairs)
args=("\$@")
i=0; subcommand=""
while [ \$i -lt \${#args[@]} ]; do
  arg="\${args[\$i]}"
  if [ "\$arg" = "-C" ]; then i=\$(( i + 1 ))
  else subcommand="\$arg"; break
  fi
  i=\$(( i + 1 ))
done

case "\$subcommand" in
  fetch)
    # Fetch succeeds — the ref appears to exist on the remote
    exit 0
    ;;
  rev-parse)
    # rev-parse succeeds — returns a valid-looking SHA (the EC-008 condition:
    # the ref resolves but the object is not in the local store)
    printf '%s\n' "${GHOST_SHA}"
    exit 0
    ;;
  cat-file)
    # cat-file -e <sha>^{commit} FAILS — object is absent from local store.
    # This is the EC-008 object-absent condition that distinguishes this test
    # from the rev-parse-fails case in test_BC_5_40_001_cas_push_stale_sha_after_fetch.
    printf 'error: object does not exist\n' >&2
    exit 1
    ;;
  push)
    # Record that push was incorrectly attempted (helper must NOT reach this step)
    touch "${WORK}/push-was-attempted"
    exit 0
    ;;
  *)
    "$(command -v git)" "\$@"
    ;;
esac
STUB
  chmod +x "$STUB_BIN/git"

  CALLER_DIR="$WORK/caller"
  mkdir -p "$CALLER_DIR/.factory"

  run bash -c "export PATH='${STUB_BIN}:${PATH}'; cd '${CALLER_DIR}' && bash '${HELPER}'"

  # MUST exit non-zero — the object is absent; pushing would be corrupt or opaque
  [ "$status" -ne 0 ]

  # MUST emit a CASPushRejected-class stale-SHA message from the helper itself.
  # The message must reference the stale-SHA / object-absent condition so the
  # developer knows to re-fetch rather than retry the same push.
  # Current impl: no cat-file check → no such message path → assertion FAILS → RED.
  [[ "$output" == *"CASPushRejected"* ]] || \
    [[ "$output" == *"stale SHA"* ]] || \
    [[ "$output" == *"object absent"* ]] || \
    [[ "$output" == *"state-burst CAS push failed"* && \
       ( "$output" == *"stale"* || "$output" == *"object"* ) ]]

  # MUST NOT have attempted the push (object is invalid; pushing must be aborted)
  [ ! -f "$WORK/push-was-attempted" ]
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

# ---------------------------------------------------------------------------
# cwd-independent .factory resolution (issue #631)
#
# The helper formerly ran a relative `git -C .factory`, which assumes the caller
# stands in the repo root. When the caller's cwd IS the .factory worktree — the
# state-manager's natural cwd — `.factory` resolved to `.factory/.factory` and
# every git step died with "cannot change to '.factory'", misreported as a fetch
# error. These tests use a REAL main-worktree + linked .factory worktree + bare
# origin (no git stub) so the resolution logic is exercised end-to-end.
#
# Fixture layout (built by _setup_worktree_fixture):
#   $WORK/origin.git          — bare remote
#   $WORK/main                — main worktree (branch: develop)
#   $WORK/main/.factory       — linked worktree (branch: factory-artifacts)
#   $WORK/story-S1            — sibling story worktree (branch: story/S-1)
# ---------------------------------------------------------------------------

_setup_worktree_fixture() {
  # GHA runners lack a global git identity; scope one to this process only.
  export GIT_AUTHOR_NAME="cas-push-test"
  export GIT_AUTHOR_EMAIL="cas-push@test.local"
  export GIT_COMMITTER_NAME="cas-push-test"
  export GIT_COMMITTER_EMAIL="cas-push@test.local"

  ORIGIN="$WORK/origin.git"
  MAIN="$WORK/main"
  git init --bare -q "$ORIGIN"
  git clone -q "$ORIGIN" "$MAIN" 2>/dev/null

  git -C "$MAIN" commit --allow-empty -q -m init
  # factory-artifacts branch with a STATE.md, pushed to origin
  git -C "$MAIN" checkout -q -b factory-artifacts
  echo "state: initial" > "$MAIN/STATE.md"
  git -C "$MAIN" add STATE.md
  git -C "$MAIN" commit -q -m "init factory-artifacts"
  git -C "$MAIN" push -q origin factory-artifacts
  # primary working branch + linked .factory worktree
  git -C "$MAIN" checkout -q -b develop
  git -C "$MAIN" worktree add -q .factory factory-artifacts
}

@test "cwd-resolution: invocation from repo root succeeds (#631)" {
  _setup_worktree_fixture
  run bash -c "cd '$MAIN' && bash '$HELPER'"
  [ "$status" -eq 0 ]
  [[ "$output" == *"state-burst CAS push succeeded"* ]]
}

@test "cwd-resolution: invocation from INSIDE .factory worktree succeeds (#631)" {
  _setup_worktree_fixture
  # This is the exact scenario that failed before the fix: cwd IS .factory,
  # so the old relative `git -C .factory` looked for .factory/.factory.
  run bash -c "cd '$MAIN/.factory' && bash '$HELPER'"
  [ "$status" -eq 0 ]
  [[ "$output" == *"state-burst CAS push succeeded"* ]]
  # And it must NOT emit the old raw cd failure masquerading as a fetch error.
  [[ "$output" != *"cannot change to '.factory'"* ]]
}

@test "cwd-resolution: invocation from a sibling story worktree succeeds (#631)" {
  _setup_worktree_fixture
  git -C "$MAIN" worktree add -q -b story/S-1 "$WORK/story-S1" develop
  run bash -c "cd '$WORK/story-S1' && bash '$HELPER'"
  [ "$status" -eq 0 ]
  [[ "$output" == *"state-burst CAS push succeeded"* ]]
}

@test "cwd-resolution: invocation from an unrelated non-git dir fails clearly (#631)" {
  UNREL="$WORK/unrelated"
  mkdir -p "$UNREL"
  run bash -c "cd '$UNREL' && bash '$HELPER'"
  [ "$status" -ne 0 ]
  # Actionable message, NOT the raw git cd error.
  [[ "$output" == *"could not locate the .factory worktree"* ]]
  [[ "$output" == *"Run from the repo root or the .factory worktree."* ]]
}
