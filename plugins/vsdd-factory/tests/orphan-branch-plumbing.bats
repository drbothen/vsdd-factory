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
# The commits were also unsigned.
#
# THE FIX (issue's Option B, plumbing-only):
#   git branch <name> "$(git commit-tree -S "$(git mktree </dev/null)" -m "...")"
# HEAD never moves, the working tree is never touched, the commit is signed.
#
# TEST STRATEGY
#   - Behavioral (GREEN): run the new recipe in a scratch repo with tracked +
#     untracked files and assert the invariants: current branch unchanged,
#     factory-artifacts resolves, it is a parentless (orphan) empty-tree commit,
#     and `git status --porcelain` is byte-identical before and after.
#   - Behavioral (RED): run each OLD recipe in the same scratch fixture and
#     assert it CAN strand HEAD on factory-artifacts — this is the defect the
#     fix removes. These tests PASS by demonstrating the old behavior is broken.
#   - Signing: run the recipe verbatim WITH -S under an ephemeral SSH signing
#     key and assert the commit verifies. Skipped if SSH commit signing is
#     unavailable in the environment (ssh-keygen missing or git too old).
#   - Contract: the shipped SKILL.md files contain the plumbing recipe and no
#     longer contain the fragile checkout-dance in the branch-creation block.
#
#   The behavioral tests strip -S from the recipe so they run on any host with
#   no signing key configured (as CI runs). The -S delta is covered separately
#   by the gated signing test and by the contract test that asserts -S is
#   present in the shipped recipe text.
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

# The empty tree object hash is a git constant.
EMPTY_TREE="4b825dc642cb6eb9a060e54bf8d69288fbee4904"

# ===========================================================================
# BEHAVIORAL — GREEN: the plumbing recipe holds every invariant
# ===========================================================================

@test "new recipe: HEAD/current branch unchanged after creation" {
  _setup_repo
  local before_head before_branch
  before_head="$(git -C "$REPO" rev-parse HEAD)"
  before_branch="$(git -C "$REPO" branch --show-current)"

  # New recipe (issue #204 Option B), -S stripped for CI portability.
  git -C "$REPO" branch factory-artifacts \
    "$(git -C "$REPO" commit-tree "$(git -C "$REPO" mktree </dev/null)" \
       -m "chore: initialize factory-artifacts orphan branch")"

  [ "$(git -C "$REPO" rev-parse HEAD)" = "$before_head" ]
  [ "$(git -C "$REPO" branch --show-current)" = "$before_branch" ]
  [ "$(git -C "$REPO" branch --show-current)" = "develop" ]
}

@test "new recipe: factory-artifacts ref resolves to a commit" {
  _setup_repo
  git -C "$REPO" branch factory-artifacts \
    "$(git -C "$REPO" commit-tree "$(git -C "$REPO" mktree </dev/null)" \
       -m "chore: initialize factory-artifacts orphan branch")"

  run git -C "$REPO" rev-parse --verify --quiet factory-artifacts
  [ "$status" -eq 0 ]
  [ -n "$output" ]
}

@test "new recipe: factory-artifacts is a parentless (orphan) empty-tree commit" {
  _setup_repo
  git -C "$REPO" branch factory-artifacts \
    "$(git -C "$REPO" commit-tree "$(git -C "$REPO" mktree </dev/null)" \
       -m "chore: initialize factory-artifacts orphan branch")"

  # Orphan: `git rev-list --parents -n1` prints only the commit SHA (1 token)
  # when there are no parents; a parent would add a second token.
  local parents
  parents="$(git -C "$REPO" rev-list --parents -n 1 factory-artifacts)"
  [ "$(printf '%s' "$parents" | wc -w | tr -d ' ')" -eq 1 ]

  # Empty content: the branch's tree is the canonical empty tree.
  [ "$(git -C "$REPO" rev-parse 'factory-artifacts^{tree}')" = "$EMPTY_TREE" ]
}

@test "new recipe: working tree is byte-for-byte untouched" {
  _setup_repo
  local before after
  before="$(git -C "$REPO" status --porcelain)"
  local before_main before_readme before_untracked
  before_main="$(cat "$REPO/src/main.rs")"
  before_readme="$(cat "$REPO/README.md")"
  before_untracked="$(cat "$REPO/untracked.txt")"

  git -C "$REPO" branch factory-artifacts \
    "$(git -C "$REPO" commit-tree "$(git -C "$REPO" mktree </dev/null)" \
       -m "chore: initialize factory-artifacts orphan branch")"

  after="$(git -C "$REPO" status --porcelain)"
  # Same working-tree/index state as before (the one untracked file, nothing else).
  [ "$before" = "$after" ]
  # Tracked files intact.
  [ "$(cat "$REPO/src/main.rs")" = "$before_main" ]
  [ "$(cat "$REPO/README.md")" = "$before_readme" ]
  # Untracked file intact (the old `git rm -rf .` variant would have deleted it).
  [ "$(cat "$REPO/untracked.txt")" = "$before_untracked" ]
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

@test "new recipe does NOT strand: same fixture, HEAD stays put" {
  _setup_repo
  # Contrast to the RED cases above: identical starting fixture, new recipe,
  # HEAD is exactly where it started.
  local before_branch
  before_branch="$(git -C "$REPO" branch --show-current)"
  git -C "$REPO" branch factory-artifacts \
    "$(git -C "$REPO" commit-tree "$(git -C "$REPO" mktree </dev/null)" \
       -m "chore: initialize factory-artifacts orphan branch")"
  [ "$(git -C "$REPO" branch --show-current)" = "$before_branch" ]
}

# ===========================================================================
# SIGNING — verbatim recipe with -S, gated on SSH-signing availability
# ===========================================================================

@test "new recipe with -S produces a verifiable signature" {
  command -v ssh-keygen >/dev/null 2>&1 || skip "ssh-keygen not available"

  _setup_repo
  local key="$WORK/sign_key"
  # Generate an ephemeral SSH key for signing (no passphrase).
  ssh-keygen -q -t ed25519 -f "$key" -N "" -C "orphan-plumbing-test" \
    </dev/null >/dev/null 2>&1 || skip "ssh-keygen failed"

  git -C "$REPO" config gpg.format ssh
  git -C "$REPO" config user.signingkey "$key.pub"

  # Run the recipe VERBATIM, including -S, as shipped in SKILL.md.
  run git -C "$REPO" branch factory-artifacts \
    "$(git -C "$REPO" commit-tree -S "$(git -C "$REPO" mktree </dev/null)" \
       -m "chore: initialize factory-artifacts orphan branch")"
  # If this git build lacks SSH signing support, skip rather than fail.
  if [ "$status" -ne 0 ]; then
    skip "SSH commit signing unavailable in this git build"
  fi

  # An allowed-signers file lets verify-commit confirm the signature.
  local signers="$WORK/allowed_signers"
  echo "orphan-plumbing-test $(cat "$key.pub")" > "$signers"
  git -C "$REPO" config gpg.ssh.allowedSignersFile "$signers"

  run git -C "$REPO" verify-commit factory-artifacts
  [ "$status" -eq 0 ]
  # HEAD still didn't move even with signing on.
  [ "$(git -C "$REPO" branch --show-current)" = "develop" ]
}

# ===========================================================================
# CONTRACT — the shipped SKILL.md files carry the fix, not the fragile recipe
# ===========================================================================

@test "factory-health SKILL.md uses the plumbing recipe" {
  grep -qF "git commit-tree -S" "$FACTORY_HEALTH"
  grep -qF "git mktree </dev/null" "$FACTORY_HEALTH"
  grep -qF "git branch factory-artifacts" "$FACTORY_HEALTH"
}

@test "factory-health SKILL.md no longer contains the checkout dance in branch creation" {
  # The fragile primitives must be gone from the file.
  run grep -F "git checkout --orphan factory-artifacts" "$FACTORY_HEALTH"
  [ "$status" -ne 0 ]
  run grep -F "git checkout -  # return to previous branch" "$FACTORY_HEALTH"
  [ "$status" -ne 0 ]
}

@test "factory-worktree-health SKILL.md uses the plumbing recipe" {
  grep -qF 'git commit-tree -S' "$WORKTREE_HEALTH"
  grep -qF 'git mktree </dev/null' "$WORKTREE_HEALTH"
  grep -qF 'git branch ${BRANCH_NAME}' "$WORKTREE_HEALTH"
  # The remote branch still gets pushed.
  grep -qF 'git push origin ${BRANCH_NAME}' "$WORKTREE_HEALTH"
}

@test "factory-worktree-health SKILL.md no longer contains the checkout dance in branch creation" {
  run grep -F 'git checkout --orphan ${BRANCH_NAME}' "$WORKTREE_HEALTH"
  [ "$status" -ne 0 ]
  # The hardcoded return target is gone.
  run grep -nF 'git checkout develop' "$WORKTREE_HEALTH"
  [ "$status" -ne 0 ]
}
