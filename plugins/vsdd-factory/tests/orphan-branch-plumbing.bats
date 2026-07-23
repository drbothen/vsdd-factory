#!/usr/bin/env bats
# orphan-branch-plumbing.bats — behavioral + contract tests for issue #204:
# the factory-artifacts orphan branch must be created with plumbing
# (git commit-tree + git mktree + git branch), never with a checkout dance.
#
# THE BUG (#204): both factory-health and factory-worktree-health created the
# orphan branch by switching HEAD onto it (git checkout --orphan), clearing the
# index, committing, then switching back (git checkout - / git checkout develop).
# The return step is unchecked. When it fails — e.g. `git checkout -` cannot
# resolve the previous ref, or the working-tree files that became untracked
# under the orphan would be overwritten — HEAD is silently STRANDED on
# factory-artifacts and subsequent pipeline work commits to the wrong branch.
#
# THE FIX (issue's Option B, plumbing-only):
#   commit=$(git commit-tree "$(git mktree </dev/null)" -m "...") || { ...; exit 1; }
#   git branch <name> "$commit"
# HEAD never moves and the working tree is never touched. The init marker
# commit is deliberately UNSIGNED: `git commit-tree` does not honor
# `commit.gpgsign`, so a hard -S would turn the auto-repair path into a hard
# failure on any host without a signing key (fresh CI runner, agent container,
# downstream plugin consumer). Content commits are signed later by
# state-manager under the repo's normal commit config.
#
# TEST STRATEGY
#   - The behavioral tests EXTRACT the fenced ```bash recipe from the shipped
#     SKILL.md files and execute that exact text — a typo introduced into
#     either SKILL.md fails the behavioral tests, not just a substring check.
#   - Behavioral (GREEN): run the extracted recipe in a scratch repo with
#     tracked + untracked files and assert the invariants: current branch
#     unchanged, factory-artifacts resolves, it is a parentless (orphan)
#     empty-tree commit, and `git status --porcelain` is byte-identical
#     before and after.
#   - Behavioral (RED): run each OLD recipe in the same scratch fixture and
#     assert it CAN strand HEAD on factory-artifacts — this is the defect the
#     fix removes. These tests PASS by demonstrating the old behavior is broken.
#   - Keyless environment: run the extracted recipe with global/system git
#     config masked (no signing key reachable) and assert the branch is still
#     created, unsigned — the repair path must not require a key.
#   - Error path: run the extracted recipe in a repo where commit-tree cannot
#     succeed (no committer identity) and assert it fails cleanly with the
#     recipe's own message and creates NO branch (no `git branch <name> ""`).
#   - Contract: the shipped SKILL.md files contain the plumbing recipe (no -S),
#     the worktree variant carries the local-existence guard and quoted
#     ${BRANCH_NAME}, and neither file retains the fragile checkout dance.
#
# Run:
#   bats plugins/vsdd-factory/tests/orphan-branch-plumbing.bats

setup() {
  PLUGIN_ROOT="$(cd "$BATS_TEST_DIRNAME/.." && pwd)"
  FACTORY_HEALTH="$PLUGIN_ROOT/skills/factory-health/SKILL.md"
  WORKTREE_HEALTH="$PLUGIN_ROOT/skills/factory-worktree-health/SKILL.md"

  WORK="$(mktemp -d)"
  # Resolve symlinks (macOS /var -> /private/var) for stable path comparisons.
  WORK="$(cd "$WORK" && pwd -P)"
}

teardown() {
  rm -rf "$WORK"
}

# ---------------------------------------------------------------------------
# Extract the branch-creation recipe from a SKILL.md: the first fenced ```bash
# block that contains `commit-tree`, with the bullet's 2-space indent stripped.
# Binding the behavioral tests to this extracted text means the tests exercise
# the exact recipe the plugin ships.
# ---------------------------------------------------------------------------
_extract_recipe() {
  local file="$1"
  awk '
    /^[[:space:]]*```/ {
      if (inblock) {
        if (buf ~ /commit-tree/) { printf "%s", buf; exit }
        inblock = 0; buf = ""
      } else {
        inblock = 1
      }
      next
    }
    inblock { line = $0; sub(/^  /, "", line); buf = buf line "\n" }
  ' "$file"
}

# Run an extracted recipe inside a repo (subshell; recipe `exit 1` is contained).
_run_recipe() {
  local repo="$1" recipe="$2"
  ( cd "$repo" && eval "$recipe" )
}

# Same, with BRANCH_NAME bound (factory-worktree-health parameterizes on it).
_run_recipe_named() {
  local repo="$1" recipe="$2" name="$3"
  ( cd "$repo" && BRANCH_NAME="$name" && eval "$recipe" )
}

# ---------------------------------------------------------------------------
# Build a scratch repo on branch `develop` with one seed commit, a tracked
# file, and an untracked file. Commit signing is DISABLED so the fixture is
# CI-portable (no key required). Sets REPO and STARTING_BRANCH.
# ---------------------------------------------------------------------------
_setup_repo() {
  REPO="$WORK/repo"
  git init -q -b develop "$REPO"
  git -C "$REPO" config user.email "test@orphan-plumbing.test"
  git -C "$REPO" config user.name "Orphan Plumbing Test"
  git -C "$REPO" config commit.gpgsign false

  mkdir -p "$REPO/src"
  echo "fn main() {}" > "$REPO/src/main.rs"
  echo "# project"     > "$REPO/README.md"
  git -C "$REPO" add -A
  git -C "$REPO" commit -q -m "seed develop with tracked files"

  # An untracked file present at recipe time — the class of working-tree state
  # that the old checkout-dance could clobber or refuse to restore over.
  echo "scratch notes" > "$REPO/untracked.txt"

  STARTING_BRANCH="$(git -C "$REPO" branch --show-current)"
}

# Bare origin for the factory-worktree-health recipe's push step.
_setup_remote() {
  ORIGIN="$WORK/origin.git"
  git init -q --bare "$ORIGIN"
  git -C "$REPO" remote add origin "$ORIGIN"
}

# The empty tree object hash is a git constant.
EMPTY_TREE="4b825dc642cb6eb9a060e54bf8d69288fbee4904"

# ===========================================================================
# EXTRACTION SANITY — the recipes are actually present to extract
# ===========================================================================

@test "factory-health SKILL.md yields an extractable branch-creation recipe" {
  recipe="$(_extract_recipe "$FACTORY_HEALTH")"
  [ -n "$recipe" ]
  [[ "$recipe" == *"commit-tree"* ]]
  [[ "$recipe" == *"git branch factory-artifacts"* ]]
}

@test "factory-worktree-health SKILL.md yields an extractable branch-creation recipe" {
  recipe="$(_extract_recipe "$WORKTREE_HEALTH")"
  [ -n "$recipe" ]
  [[ "$recipe" == *"commit-tree"* ]]
  [[ "$recipe" == *'git push origin "${BRANCH_NAME}"'* ]]
}

# ===========================================================================
# BEHAVIORAL — GREEN: the SHIPPED recipe holds every invariant
# ===========================================================================

@test "shipped recipe: HEAD/current branch unchanged after creation" {
  _setup_repo
  local before_head before_branch recipe
  before_head="$(git -C "$REPO" rev-parse HEAD)"
  before_branch="$(git -C "$REPO" branch --show-current)"
  recipe="$(_extract_recipe "$FACTORY_HEALTH")"

  run _run_recipe "$REPO" "$recipe"
  [ "$status" -eq 0 ]

  [ "$(git -C "$REPO" rev-parse HEAD)" = "$before_head" ]
  [ "$(git -C "$REPO" branch --show-current)" = "$before_branch" ]
  [ "$(git -C "$REPO" branch --show-current)" = "develop" ]
}

@test "shipped recipe: factory-artifacts ref resolves to a commit" {
  _setup_repo
  run _run_recipe "$REPO" "$(_extract_recipe "$FACTORY_HEALTH")"
  [ "$status" -eq 0 ]

  run git -C "$REPO" rev-parse --verify --quiet factory-artifacts
  [ "$status" -eq 0 ]
  [ -n "$output" ]
}

@test "shipped recipe: factory-artifacts is a parentless (orphan) empty-tree commit" {
  _setup_repo
  run _run_recipe "$REPO" "$(_extract_recipe "$FACTORY_HEALTH")"
  [ "$status" -eq 0 ]

  # Orphan: `git rev-list --parents -n1` prints only the commit SHA (1 token)
  # when there are no parents; a parent would add a second token.
  local parents
  parents="$(git -C "$REPO" rev-list --parents -n 1 factory-artifacts)"
  [ "$(printf '%s' "$parents" | wc -w | tr -d ' ')" -eq 1 ]

  # Empty content: the branch's tree is the canonical empty tree.
  [ "$(git -C "$REPO" rev-parse 'factory-artifacts^{tree}')" = "$EMPTY_TREE" ]
}

@test "shipped recipe: working tree is byte-for-byte untouched" {
  _setup_repo
  local before after
  before="$(git -C "$REPO" status --porcelain)"
  local before_main before_readme before_untracked
  before_main="$(cat "$REPO/src/main.rs")"
  before_readme="$(cat "$REPO/README.md")"
  before_untracked="$(cat "$REPO/untracked.txt")"

  run _run_recipe "$REPO" "$(_extract_recipe "$FACTORY_HEALTH")"
  [ "$status" -eq 0 ]

  after="$(git -C "$REPO" status --porcelain)"
  # Same working-tree/index state as before (the one untracked file, nothing else).
  [ "$before" = "$after" ]
  # Tracked files intact.
  [ "$(cat "$REPO/src/main.rs")" = "$before_main" ]
  [ "$(cat "$REPO/README.md")" = "$before_readme" ]
  # Untracked file intact (the old `git rm -rf .` variant would have deleted it).
  [ "$(cat "$REPO/untracked.txt")" = "$before_untracked" ]
}

@test "shipped worktree-health recipe: creates the branch locally and pushes it" {
  _setup_repo
  _setup_remote
  local before_branch recipe
  before_branch="$(git -C "$REPO" branch --show-current)"
  recipe="$(_extract_recipe "$WORKTREE_HEALTH")"

  run _run_recipe_named "$REPO" "$recipe" "factory-artifacts"
  [ "$status" -eq 0 ]

  # Local branch created; HEAD untouched.
  run git -C "$REPO" rev-parse --verify --quiet factory-artifacts
  [ "$status" -eq 0 ]
  [ "$(git -C "$REPO" branch --show-current)" = "$before_branch" ]
  # Pushed: the bare origin now has the ref.
  run git -C "$ORIGIN" show-ref --verify --quiet refs/heads/factory-artifacts
  [ "$status" -eq 0 ]
}

@test "shipped worktree-health recipe: re-run with existing local branch succeeds (local-existence guard)" {
  _setup_repo
  _setup_remote
  local recipe
  recipe="$(_extract_recipe "$WORKTREE_HEALTH")"

  # First run creates local + remote. Simulate the review's re-run case by
  # deleting only the REMOTE ref (Step 1's ls-remote check is remote-only,
  # so the skill re-enters this recipe while the local branch still exists).
  run _run_recipe_named "$REPO" "$recipe" "factory-artifacts"
  [ "$status" -eq 0 ]
  git -C "$ORIGIN" update-ref -d refs/heads/factory-artifacts
  local existing
  existing="$(git -C "$REPO" rev-parse factory-artifacts)"

  # Re-run: without the guard this died with "branch already exists".
  run _run_recipe_named "$REPO" "$recipe" "factory-artifacts"
  [ "$status" -eq 0 ]
  # The existing local branch was reused, not recreated...
  [ "$(git -C "$REPO" rev-parse factory-artifacts)" = "$existing" ]
  # ...and the push restored the remote ref.
  run git -C "$ORIGIN" show-ref --verify --quiet refs/heads/factory-artifacts
  [ "$status" -eq 0 ]
}

# ===========================================================================
# BEHAVIORAL — RED: the OLD recipes CAN strand HEAD. These tests document the
# defect the fix removes; they PASS by proving the old behavior is broken.
# ===========================================================================

@test "old factory-health recipe strands HEAD (git checkout - cannot resolve)" {
  _setup_repo
  # Reproduce the pre-fix factory-health recipe verbatim (SKILL.md:22-25).
  # `git checkout -` resolves the previous branch from the reflog's @{-1};
  # after `git checkout --orphan` there is no such entry to return to, so the
  # return fails and HEAD is left on factory-artifacts.
  git -C "$REPO" checkout -q --orphan factory-artifacts
  git -C "$REPO" rm -rf --cached . >/dev/null 2>&1 || true
  git -C "$REPO" commit -q --allow-empty \
    -m "chore: initialize factory-artifacts orphan branch"
  run git -C "$REPO" checkout -    # unchecked return — the defect
  # The return failed...
  [ "$status" -ne 0 ]
  # ...and HEAD is STRANDED on factory-artifacts, not back on develop.
  [ "$(git -C "$REPO" branch --show-current)" = "factory-artifacts" ]
  [ "$(git -C "$REPO" branch --show-current)" != "$STARTING_BRANCH" ]
}

@test "old factory-worktree-health recipe destroys untracked working-tree files" {
  _setup_repo
  # Reproduce the pre-fix factory-worktree-health recipe (SKILL.md:86-90),
  # minus the network push. `git rm -rf .` (no --cached) deletes the working
  # copy of every tracked path from disk on the orphan branch. The hardcoded
  # `git checkout develop` restores the tracked files, but any work that ran
  # against the working tree between the two checkouts saw them gone — and an
  # untracked file is simply lost to the rm.
  [ -f "$REPO/src/main.rs" ]
  git -C "$REPO" checkout -q --orphan factory-artifacts
  git -C "$REPO" rm -rf . >/dev/null 2>&1 || true
  # Tracked files are gone from disk at this point (mid-recipe window).
  [ ! -f "$REPO/src/main.rs" ]
  [ ! -f "$REPO/README.md" ]
}

@test "old factory-worktree-health recipe: hardcoded return lands on develop, not the actual start branch" {
  _setup_repo
  # Start on a FEATURE branch, not develop. The old recipe's hardcoded
  # `git checkout develop` silently moves the session to develop regardless of
  # where it began — a wrong-branch return even when it doesn't outright fail.
  git -C "$REPO" checkout -q -b feature/my-work
  local start="feature/my-work"
  [ "$(git -C "$REPO" branch --show-current)" = "$start" ]

  git -C "$REPO" checkout -q --orphan factory-artifacts
  git -C "$REPO" rm -rf . >/dev/null 2>&1 || true
  git -C "$REPO" commit -q --allow-empty -m "chore: initialize factory-artifacts branch"
  git -C "$REPO" checkout -q develop   # hardcoded return target

  [ "$(git -C "$REPO" branch --show-current)" = "develop" ]
  [ "$(git -C "$REPO" branch --show-current)" != "$start" ]
}

@test "shipped recipe does NOT strand: same fixture, HEAD stays put" {
  _setup_repo
  # Contrast to the RED cases above: identical starting fixture, shipped
  # recipe, HEAD is exactly where it started.
  local before_branch
  before_branch="$(git -C "$REPO" branch --show-current)"
  run _run_recipe "$REPO" "$(_extract_recipe "$FACTORY_HEALTH")"
  [ "$status" -eq 0 ]
  [ "$(git -C "$REPO" branch --show-current)" = "$before_branch" ]
}

# ===========================================================================
# ENVIRONMENT NEUTRALITY — the repair path must not require a signing key
# ===========================================================================

@test "shipped recipe succeeds in a keyless environment (no signing config reachable)" {
  _setup_repo
  local recipe
  recipe="$(_extract_recipe "$FACTORY_HEALTH")"

  # Mask global + system git config so no user.signingkey / commit.gpgsign /
  # gpg.format can leak in from the host — the fresh-CI-runner shape.
  run bash -c "cd '$REPO' && GIT_CONFIG_GLOBAL=/dev/null GIT_CONFIG_SYSTEM=/dev/null bash -c '$(printf '%s' "$recipe" | sed "s/'/'\\\\''/g")'"
  [ "$status" -eq 0 ]

  # Branch created; commit is unsigned (%G? prints N for no signature).
  run git -C "$REPO" rev-parse --verify --quiet factory-artifacts
  [ "$status" -eq 0 ]
  [ "$(git -C "$REPO" log -1 --format=%G? factory-artifacts)" = "N" ]
}

@test "shipped recipe fails cleanly when commit-tree cannot succeed — no half-made branch" {
  # A repo where commit-tree CANNOT succeed: user.useConfigOnly forbids the
  # username@hostname ident auto-detection and no user.email is configured.
  # The recipe's guard must surface its own message and NOT run
  # `git branch factory-artifacts ""` (which would emit a misleading
  # "not a valid object name" error).
  REPO="$WORK/noident"
  git init -q -b develop "$REPO"
  git -C "$REPO" -c user.email=seed@x -c user.name=seed commit -q --allow-empty -m seed
  git -C "$REPO" config user.useConfigOnly true

  local recipe
  recipe="$(_extract_recipe "$FACTORY_HEALTH")"
  run bash -c "cd '$REPO' && GIT_CONFIG_GLOBAL=/dev/null GIT_CONFIG_SYSTEM=/dev/null bash -c '$(printf '%s' "$recipe" | sed "s/'/'\\\\''/g")'"
  [ "$status" -ne 0 ]
  # The failure is attributed by the recipe's own message...
  [[ "$output" == *"failed to create factory-artifacts init commit"* ]]
  # ...not by a misleading empty-object branch error, and no branch exists.
  [[ "$output" != *"not a valid object name"* ]]
  run git -C "$REPO" show-ref --verify --quiet refs/heads/factory-artifacts
  [ "$status" -ne 0 ]
}

# ===========================================================================
# CONTRACT — the shipped SKILL.md files carry the fix, not the fragile recipe
# ===========================================================================

@test "factory-health SKILL.md uses the plumbing recipe without mandatory -S" {
  grep -qF "git commit-tree" "$FACTORY_HEALTH"
  grep -qF "git mktree </dev/null" "$FACTORY_HEALTH"
  grep -qF "git branch factory-artifacts" "$FACTORY_HEALTH"
  # -S would make the auto-repair path fail on keyless hosts; it must be gone.
  run grep -F "commit-tree -S" "$FACTORY_HEALTH"
  [ "$status" -ne 0 ]
  # The commit-tree failure guard is present (no `git branch <name> ""`).
  grep -qF "failed to create factory-artifacts init commit" "$FACTORY_HEALTH"
}

@test "factory-health SKILL.md no longer contains the checkout dance in branch creation" {
  # The fragile primitives must be gone from the file.
  run grep -F "git checkout --orphan factory-artifacts" "$FACTORY_HEALTH"
  [ "$status" -ne 0 ]
  run grep -F "git checkout -  # return to previous branch" "$FACTORY_HEALTH"
  [ "$status" -ne 0 ]
}

@test "factory-worktree-health SKILL.md uses the guarded plumbing recipe" {
  grep -qF 'git commit-tree' "$WORKTREE_HEALTH"
  grep -qF 'git mktree </dev/null' "$WORKTREE_HEALTH"
  run grep -F "commit-tree -S" "$WORKTREE_HEALTH"
  [ "$status" -ne 0 ]
  # Local-existence guard (Step 1's ls-remote check is remote-only).
  grep -qF 'git show-ref --verify --quiet "refs/heads/${BRANCH_NAME}"' "$WORKTREE_HEALTH"
  # Quoted expansions and the failure guard.
  grep -qF 'git branch "${BRANCH_NAME}" "$commit"' "$WORKTREE_HEALTH"
  grep -qF 'git push origin "${BRANCH_NAME}"' "$WORKTREE_HEALTH"
  grep -qF 'failed to create ${BRANCH_NAME} init commit' "$WORKTREE_HEALTH"
}

@test "factory-worktree-health SKILL.md no longer contains the checkout dance in branch creation" {
  run grep -F 'git checkout --orphan ${BRANCH_NAME}' "$WORKTREE_HEALTH"
  [ "$status" -ne 0 ]
  # The hardcoded return target is gone.
  run grep -nF 'git checkout develop' "$WORKTREE_HEALTH"
  [ "$status" -ne 0 ]
}
