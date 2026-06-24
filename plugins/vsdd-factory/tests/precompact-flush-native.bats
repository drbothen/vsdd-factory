#!/usr/bin/env bats
# precompact-flush-native.bats — Red Gate bats integration tests for S-18.04a.
#
# Covers bats-level rows from the S-18.04a Red Gate Test Table:
#
#   test_flush_completes_via_git_only_positive
#     AC-001/005/007: positive flush via factory-dispatcher with precompact-flush.wasm;
#     assert (a) factory-artifacts HEAD advances AND (b) precompact-flush-log entry appended;
#     NOT merely absence-of-bash-subprocess (strengthened non-tautological assertion).
#
#   test_git_push_succeeds_with_local_bare_remote
#     AC-001/009: PATH + SSH_AUTH_SOCK in env_allow sufficient for git push to local bare remote.
#
#   test_git_commit_succeeds_with_global_identity_via_home
#     AC-005/017: committer identity via HOME/.gitconfig; GIT_AUTHOR/COMMITTER absent.
#
#   test_flush_commit_lands_on_factory_artifacts_not_main_repo
#     AC-004/005/009/F-R3-006: after flush: factory-artifacts HEAD advanced,
#     main-repo HEAD unchanged; uniform git -C guarantee has bats-level mechanical coverage.
#
#   test_precompact_wasm_invoked_no_bash_subprocess
#     AC-001/014/018: precompact-flush.wasm invoked via factory-dispatcher;
#     assert no bash subprocess in dispatcher log for this plugin;
#     binary_allow = ["git"] only (ADR-028 §Decision 2).
#
# Story: S-18.04a (precompact-flush Native WASM Plugin Core)
# BC:    BC-7.07.001 PC1/PC3/PC4/PC5/PC6b/PC8/INV1/INV3/INV4/Precondition-4
# VP:    VP-082, VP-085
# ADR:   ADR-028 §Decision 1/2/3/8/15/17
#
# AC-019 / F-R3-004 — DISTINCT ROOTS (mandatory per story AC-019):
#   setup() MUST configure _run_dispatcher() with DISTINCT values for
#   CLAUDE_PLUGIN_ROOT and CLAUDE_PROJECT_DIR. Specifically:
#     CLAUDE_PLUGIN_ROOT = $WORK          (plugin directory)
#     CLAUDE_PROJECT_DIR = $WORK/project  (a SUBDIRECTORY of WORK, NOT WORK itself)
#   This de-masking pattern carries forward S-18.04a-prereq AC-005 fix to S-18.04a's
#   own bats file, ensuring positive-flush assertions are non-tautological under the
#   distinct-roots setup. When CLAUDE_PLUGIN_ROOT == CLAUDE_PROJECT_DIR, a regression
#   where write_file accidentally uses plugin_root instead of cwd would pass vacuously.
#
# ALL .factory/-relative path_allow entries in registry stanzas below are rooted at
# $PROJECT_DIR (not $WORK) so allowlist checks continue to pass under distinct-roots.
#
# RED GATE:
#   All tests that invoke precompact-flush.wasm via the dispatcher SKIP if the WASM
#   artifact is not yet compiled — skip != pass (Red Gate is honored).
#   test_flush_commit_lands_on_factory_artifacts_not_main_repo additionally fails
#   if the plugin incorrectly commits to the main repo branch.
#   test_precompact_wasm_invoked_no_bash_subprocess fails if the plugin invokes bash.
#
# Dispatcher binary: target/release/factory-dispatcher
# Plugin WASM:       plugins/vsdd-factory/hook-plugins/precompact-flush.wasm

# ---------------------------------------------------------------------------
# setup / teardown (AC-019: DISTINCT ROOTS)
# ---------------------------------------------------------------------------

setup() {
  REPO_ROOT="$(cd "${BATS_TEST_DIRNAME}/../../.." && pwd)"
  DISPATCHER="$REPO_ROOT/target/release/factory-dispatcher"
  PRECOMPACT_WASM="$REPO_ROOT/plugins/vsdd-factory/hook-plugins/precompact-flush.wasm"

  WORK="$(mktemp -d)"

  # AC-019 F-R3-004: CLAUDE_PROJECT_DIR = $WORK/project (DISTINCT subdirectory of WORK)
  # CLAUDE_PLUGIN_ROOT = $WORK
  # This is the de-masking pattern: PROJECT_DIR != WORK, preventing tautological tests.
  PROJECT_DIR="$WORK/project"

  # Create required directory structure:
  #   $WORK                     — plugin root (CLAUDE_PLUGIN_ROOT)
  #   $WORK/.factory/logs       — dispatcher internal log directory
  #   $WORK/hook-plugins/       — WASM plugin directory
  #   $WORK/project/            — project root (CLAUDE_PROJECT_DIR)
  #   $WORK/project/.factory/   — factory-artifacts worktree mount (distinct from $WORK/.factory)
  #                               NOTE: $PROJECT_DIR/.factory is NOT pre-created here;
  #                               git worktree add requires the target directory to not exist
  #                               (or be empty). The factory-artifacts branch content provides
  #                               .factory/hooks/ via a .gitkeep so write_file can create the log.
  mkdir -p "$WORK/.factory/logs"
  mkdir -p "$WORK/hook-plugins"
  mkdir -p "$PROJECT_DIR"

  # Copy precompact-flush.wasm into WORK's hook-plugins/ if it exists.
  if [ -f "$PRECOMPACT_WASM" ]; then
    cp "$PRECOMPACT_WASM" "$WORK/hook-plugins/precompact-flush.wasm"
  fi

  export CLAUDE_PLUGIN_ROOT="$WORK"
  export CLAUDE_PROJECT_DIR="$PROJECT_DIR"
}

teardown() {
  rm -rf "$WORK"
}

# ---------------------------------------------------------------------------
# Preflight helpers
# ---------------------------------------------------------------------------

# Skip the test if the dispatcher binary is not built.
_require_dispatcher() {
  if [ ! -x "$DISPATCHER" ]; then
    skip "dispatcher binary not built — run: cargo build --release -p factory-dispatcher"
  fi
}

# Skip the test if precompact-flush.wasm is not compiled.
_require_precompact_wasm() {
  _require_dispatcher
  if [ ! -f "$WORK/hook-plugins/precompact-flush.wasm" ]; then
    skip "precompact-flush.wasm not compiled — run: cargo build --target wasm32-wasip1 -p precompact-flush --release && cp ..."
  fi
}

# ---------------------------------------------------------------------------
# Registry helpers
# ---------------------------------------------------------------------------

# Write a registry for precompact-flush native WASM plugin.
# All path_allow entries are rooted at $PROJECT_DIR (not $WORK) per AC-019 F-R3-004.
_write_precompact_registry() {
  cat > "$WORK/hooks-registry.toml" <<EOF
schema_version = 2

[[hooks]]
name = "precompact-flush"
event = "PreCompact"
plugin = "hook-plugins/precompact-flush.wasm"
priority = 100
timeout_ms = 30000
on_error = "continue"
async = false

[hooks.capabilities.read_file]
path_allow = ["${PROJECT_DIR}/.factory/"]

[hooks.capabilities.write_file]
path_allow = ["${PROJECT_DIR}/.factory/"]

[hooks.capabilities.exec_subprocess]
binary_allow = ["git"]
env_allow = ["HOME", "GIT_CONFIG_GLOBAL", "XDG_CONFIG_HOME", "PATH", "SSH_AUTH_SOCK"]
EOF
}

# ---------------------------------------------------------------------------
# Dispatcher invocation helper (AC-019: distinct roots)
# ---------------------------------------------------------------------------

# Run the dispatcher with a given JSON envelope.
# CLAUDE_PLUGIN_ROOT = $WORK (plugin directory; distinct from PROJECT_DIR)
# CLAUDE_PROJECT_DIR = $PROJECT_DIR (project root; subdirectory of WORK)
# Per AC-019 / F-R3-004: these must be distinct to prevent tautological path tests.
_run_dispatcher() {
  local envelope="$1"
  run bash -c "printf '%s' '$envelope' | CLAUDE_PLUGIN_ROOT='$WORK' CLAUDE_PROJECT_DIR='$PROJECT_DIR' HOME='$WORK/home' '$DISPATCHER' 2>&1"
}

# ---------------------------------------------------------------------------
# Git fixture helpers
# ---------------------------------------------------------------------------

# Initialize the git fixture matching the production topology.
# Creates:
#   $PROJECT_DIR    — CLAUDE_PROJECT_DIR = the git repo root (develop branch)
#                     This matches production: CLAUDE_PROJECT_DIR IS a git repo.
#   $BARE_REMOTE    — bare remote (factory-artifacts branch)
#   $FACTORY_ARTS   — factory-artifacts worktree mounted at $PROJECT_DIR/.factory
#
# PRODUCTION TOPOLOGY (critical invariant):
#   In production, CLAUDE_PROJECT_DIR is the project's git repo root (e.g.,
#   /Users/zious/Documents/GITHUB/vsdd-factory). The plugin calls:
#     git -C <cwd> worktree list --porcelain
#   where <cwd> = CLAUDE_PROJECT_DIR. This succeeds because CLAUDE_PROJECT_DIR
#   IS a git repo. The fixture MUST replicate this: PROJECT_DIR must be a git repo,
#   not a plain directory. (Prior fixture had MAIN_REPO=$WORK/main-repo separate
#   from PROJECT_DIR=$WORK/project — that made PROJECT_DIR a non-repo directory,
#   causing `git worktree list` to exit 128 inside the WASM sandbox.)
#
# AC-017 / F-003: The factory-artifacts worktree is mounted at $PROJECT_DIR/.factory
# (NOT at $PROJECT_DIR itself). This matches the production topology: the discovered
# worktree path ends with "/.factory", the AC-017 canonicalize assertion fires, and
# all `git -C <wt>` calls target the real .factory worktree.
#
# AC-019 F-R3-004: CLAUDE_PLUGIN_ROOT=$WORK != CLAUDE_PROJECT_DIR=$PROJECT_DIR
# (WORK is the parent of project; PROJECT_DIR=$WORK/project is the git repo root).
#
# Branch content layout:
#   The factory-artifacts branch stores files WITHOUT the ".factory/" prefix.
#   When checked out at $PROJECT_DIR/.factory, branch-relative paths land correctly:
#     branch "STATE.md" → $PROJECT_DIR/.factory/STATE.md
#     branch "hooks/.gitkeep" → $PROJECT_DIR/.factory/hooks/.gitkeep
#   CLAUDE_PROJECT_DIR-relative plugin I/O of ".factory/STATE.md" resolves to
#   $PROJECT_DIR/.factory/STATE.md. All consistent.
_init_git_fixture() {
  # PROJECT_DIR is the git repo root (= CLAUDE_PROJECT_DIR). Must be a git repo.
  # MAIN_REPO is an alias for PROJECT_DIR for backward compat with test assertions
  # that use $MAIN_REPO (e.g., TC-PF-004 develop-head check).
  MAIN_REPO="$PROJECT_DIR"
  BARE_REMOTE="$WORK/bare-remote.git"
  # FACTORY_ARTS is the directory where the factory-artifacts worktree is mounted.
  # Production topology: worktree is at <cwd>/.factory, i.e., $PROJECT_DIR/.factory.
  # AC-017 canonicalize: discovered path ends with "/.factory" → assertion fires.
  FACTORY_ARTS="$PROJECT_DIR/.factory"

  # Configure git identity in a temp HOME so tests don't need system gitconfig.
  FAKE_HOME="$WORK/home"
  mkdir -p "$FAKE_HOME"
  cat > "$FAKE_HOME/.gitconfig" <<'GITCFG'
[user]
  name = Test Agent
  email = test@factory.local
[init]
  defaultBranch = develop
GITCFG

  # 1. Create the bare remote
  # Use HOME="$FAKE_HOME" so git uses defaultBranch=develop from $FAKE_HOME/.gitconfig.
  HOME="$FAKE_HOME" git init --bare "$BARE_REMOTE" >/dev/null 2>&1

  # 2. Init the main git repo AT PROJECT_DIR (matches production: CLAUDE_PROJECT_DIR IS a git repo).
  # HOME="$FAKE_HOME" ensures defaultBranch=develop is honoured at init time.
  HOME="$FAKE_HOME" git init "$PROJECT_DIR" >/dev/null 2>&1
  git -c user.name="Test Agent" -c user.email="test@factory.local" \
    -C "$PROJECT_DIR" commit --allow-empty -m "init develop" >/dev/null 2>&1
  git -C "$PROJECT_DIR" remote add origin "$BARE_REMOTE" >/dev/null 2>&1

  # 3. Create the factory-artifacts branch in the main repo.
  #    Branch content stores files WITHOUT the ".factory/" prefix — when mounted at
  #    $PROJECT_DIR/.factory, they appear at the correct CLAUDE_PROJECT_DIR-relative paths.
  git -C "$PROJECT_DIR" checkout -b factory-artifacts >/dev/null 2>&1
  # Create content directly in PROJECT_DIR working tree while on factory-artifacts.
  # File paths here are what the branch stores (no leading ".factory/" prefix).
  mkdir -p "$PROJECT_DIR/hooks"
  cat > "$PROJECT_DIR/STATE.md" <<'STATEMD'
---
document_type: state
version: "0.0.1-test"
current_cycle: v1.0-test-cycle
current_step: test-phase/S-18.04a-bats
---

# STATE (bats test fixture)
STATEMD
  touch "$PROJECT_DIR/hooks/.gitkeep"
  git -c user.name="Test Agent" -c user.email="test@factory.local" \
    -C "$PROJECT_DIR" add "STATE.md" "hooks" >/dev/null 2>&1
  git -c user.name="Test Agent" -c user.email="test@factory.local" \
    -C "$PROJECT_DIR" commit -m "init factory-artifacts" >/dev/null 2>&1
  git -C "$PROJECT_DIR" push origin factory-artifacts >/dev/null 2>&1

  # 4. Switch PROJECT_DIR back to develop
  git -C "$PROJECT_DIR" checkout develop >/dev/null 2>&1

  # 5. Mount factory-artifacts as worktree at $PROJECT_DIR/.factory (= $FACTORY_ARTS).
  #    AC-017 F-003: target path must end with "/.factory" so the canonicalize
  #    assertion fires and the AC-017 mismatch check has bats-level coverage.
  #    $PROJECT_DIR/.factory must NOT exist yet (git worktree add requires the target
  #    directory to not exist, or to be an empty dir — do not pre-create it in setup).
  git -C "$PROJECT_DIR" worktree add "$FACTORY_ARTS" factory-artifacts >/dev/null 2>&1

  # 6. Verify the worktree is mounted correctly and path ends with /.factory
  git -C "$PROJECT_DIR" worktree list --porcelain | grep -q "factory-artifacts" || {
    echo "ERROR: factory-artifacts worktree not mounted" >&2
    return 1
  }
  # AC-017 topology check: discovered path must end with /.factory
  git -C "$PROJECT_DIR" worktree list --porcelain | grep "^worktree " | grep -q "/.factory$" || {
    echo "ERROR: factory-artifacts worktree path does not end with /.factory" >&2
    return 1
  }

  export MAIN_REPO BARE_REMOTE FACTORY_ARTS FAKE_HOME
}

# ---------------------------------------------------------------------------
# TC-PF-001: Positive flush via git only (non-tautological)
# AC-001/005/007 / ADR-028 §Decision 3 F-NW-001 / §Decision 8 bats non-tautological
# BC-7.07.001 §INV3 positive-flush-completion
# ---------------------------------------------------------------------------
#
# RED GATE: Plugin not yet compiled; bats test fails to find precompact-flush.wasm.
# After implementation: asserts BOTH (a) factory-artifacts HEAD advances (new commit)
# AND (b) precompact-flush-log has an entry with that SHA.
# NOT tautological: CLAUDE_PROJECT_DIR=$PROJECT_DIR != CLAUDE_PLUGIN_ROOT=$WORK,
# so write_file using plugin_root instead of cwd would fail (distinct-roots guard).

@test "test_flush_completes_via_git_only_positive" {
  _require_precompact_wasm

  _init_git_fixture
  _write_precompact_registry

  # Capture factory-artifacts HEAD before flush
  local head_before
  head_before=$(git -C "$FACTORY_ARTS" rev-parse HEAD)

  # Introduce a pending change on factory-artifacts to ensure flush has something to commit.
  # FACTORY_ARTS = $PROJECT_DIR/.factory; STATE.md is at the worktree root (branch-relative).
  echo "# bats test pending change $(date -u +%Y-%m-%dT%H:%M:%SZ)" \
    >> "$FACTORY_ARTS/STATE.md"

  # Send PreCompact event to dispatcher
  local envelope='{"event_name":"PreCompact","tool_name":"","session_id":"bats-pf-001","dispatcher_trace_id":"bats-trace-001"}'
  _run_dispatcher "$envelope"

  # Dispatcher must exit 0 (precompact-flush.wasm exits 0 on success)
  [ "$status" -eq 0 ]

  # (a) factory-artifacts HEAD must have advanced (new commit created)
  local head_after
  head_after=$(git -C "$FACTORY_ARTS" rev-parse HEAD)
  [ "$head_after" != "$head_before" ]

  # (b) precompact-flush-log must exist and contain an entry with the new SHA.
  # The plugin writes to ".factory/hooks/precompact-flush-log" relative to CLAUDE_PROJECT_DIR.
  # CLAUDE_PROJECT_DIR = $PROJECT_DIR, so log lands at $PROJECT_DIR/.factory/hooks/precompact-flush-log
  # = $FACTORY_ARTS/hooks/precompact-flush-log.
  local log_path="$FACTORY_ARTS/hooks/precompact-flush-log"
  [ -f "$log_path" ]
  grep -q "$head_after" "$log_path"

  # (c) Non-tautological check: the commit lands on factory-artifacts branch,
  #     not on the main-repo develop branch.
  local develop_head
  develop_head=$(git -C "$MAIN_REPO" rev-parse HEAD 2>/dev/null || echo "no-develop")
  [ "$head_after" != "$develop_head" ]
}

# ---------------------------------------------------------------------------
# TC-PF-002: git push succeeds with local bare remote (PATH in env_allow)
# AC-001/009 / ADR-028 §Decision 1 F-NW-001 (PATH+SSH_AUTH_SOCK in env_allow)
# ---------------------------------------------------------------------------
#
# RED GATE: Plugin not yet compiled.
# After implementation: asserts (a) git push exits 0 and (b) bare remote has
# received the commit SHA. This validates that PATH in env_allow is sufficient
# for git push to a local bare remote (simulating SSH-over-PATH git binary lookup).

@test "test_git_push_succeeds_with_local_bare_remote" {
  _require_precompact_wasm

  _init_git_fixture
  _write_precompact_registry

  # Introduce a pending change on factory-artifacts.
  # FACTORY_ARTS = $PROJECT_DIR/.factory; STATE.md is at the worktree root.
  echo "# push test $(date -u +%Y-%m-%dT%H:%M:%SZ)" \
    >> "$FACTORY_ARTS/STATE.md"

  local head_before
  head_before=$(git -C "$FACTORY_ARTS" rev-parse HEAD)

  # Send PreCompact event
  local envelope='{"event_name":"PreCompact","tool_name":"","session_id":"bats-pf-002","dispatcher_trace_id":"bats-trace-002"}'
  _run_dispatcher "$envelope"

  # Dispatcher must exit 0
  [ "$status" -eq 0 ]

  # factory-artifacts HEAD must have advanced
  local head_after
  head_after=$(git -C "$FACTORY_ARTS" rev-parse HEAD)
  [ "$head_after" != "$head_before" ]

  # The bare remote must now have the new SHA on factory-artifacts
  local remote_head
  remote_head=$(git -C "$BARE_REMOTE" rev-parse refs/heads/factory-artifacts)
  [ "$remote_head" = "$head_after" ]
}

# ---------------------------------------------------------------------------
# TC-PF-003: Committer identity via HOME/.gitconfig (no GIT_AUTHOR/COMMITTER vars)
# AC-005/017 / ADR-028 §Decision 10 F-NW2-004
# ---------------------------------------------------------------------------
#
# RED GATE: Plugin not yet compiled.
# After implementation: fixture creates temp $HOME with .gitconfig containing
# user.name + user.email; asserts git commit exits 0 and the commit author
# identity matches .gitconfig (GIT_AUTHOR_NAME/EMAIL absent from env_allow).

@test "test_git_commit_succeeds_with_global_identity_via_home" {
  _require_precompact_wasm

  _init_git_fixture

  # FAKE_HOME already created by _init_git_fixture with user.name + user.email.
  # The plugin's exec_subprocess inherits HOME from env_allow.
  # GIT_AUTHOR_NAME and GIT_AUTHOR_EMAIL are NOT in env_allow — identity must
  # come from HOME/.gitconfig (ADR-028 §Decision 10 F-NW2-004).
  _write_precompact_registry

  # Introduce a pending change.
  # FACTORY_ARTS = $PROJECT_DIR/.factory; STATE.md is at the worktree root.
  echo "# identity test $(date -u +%Y-%m-%dT%H:%M:%SZ)" \
    >> "$FACTORY_ARTS/STATE.md"

  local head_before
  head_before=$(git -C "$FACTORY_ARTS" rev-parse HEAD)

  local envelope='{"event_name":"PreCompact","tool_name":"","session_id":"bats-pf-003","dispatcher_trace_id":"bats-trace-003"}'
  _run_dispatcher "$envelope"

  # Must exit 0 (commit succeeded with identity from HOME/.gitconfig)
  [ "$status" -eq 0 ]

  # factory-artifacts HEAD must advance
  local head_after
  head_after=$(git -C "$FACTORY_ARTS" rev-parse HEAD)
  [ "$head_after" != "$head_before" ]

  # Commit author must be from .gitconfig (not empty)
  local commit_author
  commit_author=$(git -C "$FACTORY_ARTS" log -1 --format="%an <%ae>" HEAD)
  [ "$commit_author" = "Test Agent <test@factory.local>" ]
}

# ---------------------------------------------------------------------------
# TC-PF-004: Flush commit lands on factory-artifacts, not main-repo develop
# AC-004/005/009 / ADR-028 §Decision 17 F-R3-006 / §Decision 3 (uniform git -C)
# ---------------------------------------------------------------------------
#
# RED GATE: Plugin not yet compiled; would fail if any git subprocess omits -C.
# After implementation: verifies that:
#   (a) factory-artifacts branch HEAD advances (commit created there)
#   (b) main-repo develop HEAD is UNCHANGED (no accidental commit to develop)
# This test provides bats-level mechanical coverage for the uniform git -C guarantee.

@test "test_flush_commit_lands_on_factory_artifacts_not_main_repo" {
  _require_precompact_wasm

  _init_git_fixture
  _write_precompact_registry

  # Capture both HEADs before flush
  local develop_head_before factory_head_before
  develop_head_before=$(git -C "$MAIN_REPO" rev-parse HEAD)
  factory_head_before=$(git -C "$FACTORY_ARTS" rev-parse HEAD)

  # Introduce a pending change on factory-artifacts.
  # FACTORY_ARTS = $PROJECT_DIR/.factory; STATE.md is at the worktree root.
  echo "# branch-guard test $(date -u +%Y-%m-%dT%H:%M:%SZ)" \
    >> "$FACTORY_ARTS/STATE.md"

  local envelope='{"event_name":"PreCompact","tool_name":"","session_id":"bats-pf-004","dispatcher_trace_id":"bats-trace-004"}'
  _run_dispatcher "$envelope"

  # Must exit 0
  [ "$status" -eq 0 ]

  # (a) factory-artifacts HEAD MUST advance (flush committed there)
  local factory_head_after
  factory_head_after=$(git -C "$FACTORY_ARTS" rev-parse HEAD)
  [ "$factory_head_after" != "$factory_head_before" ]

  # (b) develop HEAD MUST be UNCHANGED (no accidental -C-less git commit)
  local develop_head_after
  develop_head_after=$(git -C "$MAIN_REPO" rev-parse HEAD)
  [ "$develop_head_after" = "$develop_head_before" ]

  # (c) The new factory-artifacts commit must NOT appear in develop's log
  git -C "$MAIN_REPO" log --oneline HEAD | grep -q "$factory_head_after" && {
    echo "FAIL: factory-artifacts commit $factory_head_after appeared in develop log" >&2
    false
  }
  true
}

# ---------------------------------------------------------------------------
# TC-PF-005: precompact-flush.wasm invoked via factory-dispatcher; no bash subprocess
# AC-001/014/018 / ADR-028 §Decision 2 (binary_allow = ["git"] ONLY)
# BC-7.07.001 PC1 + INV1
# ---------------------------------------------------------------------------
#
# RED GATE: Hook not yet compiled; bats test fails to find precompact-flush.wasm.
# After implementation: verifies that:
#   (a) The plugin is invoked by the dispatcher (sync_plugins=1)
#   (b) No bash subprocess appears in the dispatcher's internal log for this plugin
#   (c) The plugin's binary_allow = ["git"] only — bash is not used for lock renewal

@test "test_precompact_wasm_invoked_no_bash_subprocess" {
  _require_precompact_wasm

  _init_git_fixture
  _write_precompact_registry

  # Introduce a pending change on factory-artifacts.
  # FACTORY_ARTS = $PROJECT_DIR/.factory; STATE.md is at the worktree root.
  echo "# no-bash test $(date -u +%Y-%m-%dT%H:%M:%SZ)" \
    >> "$FACTORY_ARTS/STATE.md"

  local envelope='{"event_name":"PreCompact","tool_name":"","session_id":"bats-pf-005","dispatcher_trace_id":"bats-trace-005"}'
  _run_dispatcher "$envelope"

  # Dispatcher must have run the plugin (sync_plugins=1 or plugins_run=1)
  [[ "$output" == *"sync_plugins=1"* ]] || [[ "$output" == *"plugins_run=1"* ]]

  # The dispatcher output must NOT mention bash subprocess for precompact-flush.
  # The plugin uses exec_subprocess("git", ...) ONLY — never exec_subprocess("bash", ...).
  # If bash appears in the output log for this plugin, binary_allow is being bypassed.
  [[ "$output" != *'"binary":"bash"'* ]]
  [[ "$output" != *"shell_bypass"* ]]

  # Registry must list binary_allow = ["git"] only (no bash entry)
  grep -A 20 'name = "precompact-flush"' "$WORK/hooks-registry.toml" | \
    grep 'binary_allow' | grep -q '"git"'
  ! grep -A 20 'name = "precompact-flush"' "$WORK/hooks-registry.toml" | \
    grep 'binary_allow' | grep -q '"bash"'
}
