#!/usr/bin/env bats
# factory-mount-guards.bats — guards for the .factory/ mount ordering hazards.
#
# Issue #205: factory-health/SKILL.md step 2 runs a bare `git worktree add
# .factory factory-artifacts` with NO post-mount assertion. If `.factory`
# already exists as a plain non-empty directory (the #203 onboard-first case),
# that command fails on current git (`fatal: '.factory' already exists`;
# verified 2.50/2.55 — an empty dir mounts cleanly). A nested mount at
# `.factory/.factory` was observed once from that state (#205); the mechanism
# is unconfirmed — the reporter could not reproduce it in isolation, and the
# candidates are a nested add path during error recovery or a process
# re-creating `.factory` mid-mount. The execution tests below FABRICATE that
# layout rather than reproduce it from the bare command. Once nested, step 3's
# `cd .factory && git branch --show-current` reads the PARENT branch, and
# the documented recovery `git worktree remove .factory --force` does not fix
# it because `.factory` is not the worktree — the worktree is `.factory/.factory`.
#
# Issue #203: onboard-observability/SKILL.md runs `factory-obs register` with a
# "When to use: brand new project that just had the plugin installed" while its
# Prerequisites require a `.factory/` ancestor — a contradiction. Running it
# before factory-health creates a plain `.factory/logs/` directory, which is
# exactly the plain-dir that makes the bare mount fail (`already exists`) —
# the state from which #205's nested layout was observed.
#
# THREE KINDS OF TEST HERE:
#   1. Content-contract (grep) tests — RED before the skill edits, GREEN after.
#      They assert the skills carry the required assertion / guard / prereq text.
#   2. Assertion-logic tests — fabricate real git layouts and prove the mount
#      assertion the skill prescribes actually distinguishes healthy from
#      nested / plain-dir / absent. These validate git LOGIC (independent of
#      the prose), so they pass on any git.
#   3. Shipped-block execution tests — EXTRACT the fenced bash blocks from the
#      SKILL.md files and execute that exact text against fabricated layouts.
#      A typo or an inert guard in the shipped snippet fails these, not just
#      the greps (load-bearing closure per TD-VSDD-059).
#
# Traces to: issues #205, #203 (root-cause dispatcher race is #206, separate PR).

setup() {
  PLUGIN_ROOT="$(cd "$BATS_TEST_DIRNAME/.." && pwd)"
  HEALTH="$PLUGIN_ROOT/skills/factory-health/SKILL.md"
  ONBOARD="$PLUGIN_ROOT/skills/onboard-observability/SKILL.md"

  # Scratch git repo with a factory-artifacts orphan branch, reused by the
  # execution tests. Each execution test mounts/creates .factory itself.
  WORK="$(mktemp -d)"
  WORK="$(cd "$WORK" && pwd -P)"   # resolve /var → /private/var on macOS
  REPO="$WORK/repo"
  git init -q -b main "$REPO"
  git -C "$REPO" config user.email test@vsdd.test
  git -C "$REPO" config user.name  "VSDD Test"
  echo root > "$REPO/f"; git -C "$REPO" add f; git -C "$REPO" commit -qm init
  # Create the factory-artifacts orphan branch WITHOUT keeping it checked out,
  # so each execution test is free to `git worktree add` it. (Mirrors the
  # skill's own orphan-branch bootstrap; done inside the repo, then back to main.)
  git -C "$REPO" checkout -q --orphan factory-artifacts
  git -C "$REPO" rm -rf --cached . >/dev/null 2>&1 || true
  git -C "$REPO" commit -q --allow-empty -m "init factory-artifacts"
  git -C "$REPO" checkout -q -f main
}

teardown() {
  git -C "$REPO" worktree prune 2>/dev/null || true
  rm -rf "$WORK"
}

# The canonical mount assertion the skill prescribes. Kept here so the
# execution tests and the content-contract test reference one shape.
# Returns 0 when .factory is the repo-root worktree, non-zero otherwise.
_mount_ok() {
  local top
  top="$(git -C "$1/.factory" rev-parse --show-toplevel 2>/dev/null)" || return 1
  [ "$top" = "$1/.factory" ]
}

# Extract a fenced ```bash block from a SKILL.md: the first block whose body
# matches the given ERE, with the bullet's 2-space indent stripped. Running
# the extracted text binds these tests to the exact snippet the plugin ships.
_extract_block() {
  local file="$1" pat="$2"
  awk -v pat="$pat" '
    /^[[:space:]]*```/ {
      if (inblock) {
        if (buf ~ pat) { printf "%s", buf; exit }
        inblock = 0; buf = ""
      } else {
        inblock = 1
      }
      next
    }
    inblock { line = $0; sub(/^  /, "", line); buf = buf line "\n" }
  ' "$file"
}

# Run an extracted block with cwd set to the repo (subshell contains `exit 1`).
_run_block() {
  local repo="$1" block="$2"
  ( cd "$repo" && eval "$block" )
}

# ============================================================
# Content-contract tests (RED before edits, GREEN after)
# ============================================================

@test "factory-health step 2 has a post-mount assertion on .factory toplevel (#205)" {
  # Must compare the mounted worktree's toplevel against <repo-root>/.factory.
  grep -q 'rev-parse --show-toplevel' "$HEALTH"
  run grep -E '\.factory' "$HEALTH"
  [ "$status" -eq 0 ]
  # The assertion and its repo-root anchor must co-occur (guards against a bare
  # `rev-parse --show-toplevel` that isn't wired to a .factory comparison).
  grep -Eq 'show-toplevel.*\.factory|\.factory.*show-toplevel' "$HEALTH"
}

@test "factory-health warns against blind 'worktree remove --force' on a nested mount (#205)" {
  # On assertion failure the skill must name the nested-mount shape and warn NOT
  # to blindly remove .factory --force (the real worktree is .factory/.factory).
  grep -qi 'nested' "$HEALTH"
  grep -q '.factory/.factory' "$HEALTH"
}

@test "factory-health guards against a pre-existing plain .factory directory before mounting (#205/#203)" {
  # Must handle the plain-dir case (e.g. onboard-created .factory/logs) rather
  # than blindly `git worktree add` into it. mv-aside idiom mirrors the sibling
  # factory-worktree-health skill.
  grep -qi 'plain' "$HEALTH"
}

@test "onboard-observability refuses unless .factory is a mounted worktree (#203)" {
  # Ordering prerequisite: detect .factory is a mounted worktree at repo root;
  # refuse and point at factory-health otherwise.
  grep -q 'rev-parse --show-toplevel' "$ONBOARD"
  grep -qi 'factory-health' "$ONBOARD"
}

@test "onboard-observability 'When to use' no longer claims brand-new-just-installed (#203)" {
  # The old line "Brand new project that just had the vsdd-factory plugin
  # installed" contradicted the .factory prerequisite. It must be qualified so
  # it no longer implies onboarding runs before .factory is mounted.
  when_section="$(awk '/^## When to use/{f=1;next} /^## /{f=0} f' "$ONBOARD")"
  ! printf '%s\n' "$when_section" | grep -qi 'just had the vsdd-factory plugin installed'
}

# ============================================================
# Execution tests — validate the assertion logic (git behavior)
# ============================================================

@test "mount assertion PASSES for a healthy repo-root .factory worktree" {
  cd "$REPO"
  git worktree add -q .factory factory-artifacts >/dev/null
  run _mount_ok "$REPO"
  [ "$status" -eq 0 ]
}

@test "mount assertion FAILS for a nested .factory/.factory mount (#205 shape)" {
  cd "$REPO"
  mkdir .factory                                   # plain dir already present
  git worktree add -q .factory/.factory factory-artifacts >/dev/null
  # git -C .factory resolves to the PARENT toplevel, not <repo>/.factory
  run _mount_ok "$REPO"
  [ "$status" -ne 0 ]
  # And the real worktree is nested one level down — proving why a blind
  # `git worktree remove .factory --force` is the wrong recovery.
  [ -e "$REPO/.factory/.factory/.git" ]
}

@test "mount assertion FAILS for a plain .factory/logs dir (onboard-first, #203 shape)" {
  cd "$REPO"
  mkdir -p .factory/logs
  echo evt > .factory/logs/events-1.jsonl
  run _mount_ok "$REPO"
  [ "$status" -ne 0 ]
  # Distinguish plain dir from worktree: .factory is NOT in the worktree list.
  run bash -c "git -C '$REPO' worktree list --porcelain | grep -qx 'worktree $REPO/.factory'"
  [ "$status" -ne 0 ]
}

@test "mount assertion FAILS when .factory is absent" {
  cd "$REPO"
  run _mount_ok "$REPO"
  [ "$status" -ne 0 ]
}

# ============================================================
# Shipped-block execution tests — the EXACT SKILL.md snippets
# ============================================================

@test "shipped mount block: mounts .factory at repo root from a clean state" {
  local block
  block="$(_extract_block "$HEALTH" 'worktree add \.factory factory-artifacts')"
  [ -n "$block" ]
  run _run_block "$REPO" "$block"
  [ "$status" -eq 0 ]
  run _mount_ok "$REPO"
  [ "$status" -eq 0 ]
}

@test "shipped mount block: plain .factory/logs dir is guarded, mounted over, contents restored (#203)" {
  mkdir -p "$REPO/.factory/logs"
  echo evt > "$REPO/.factory/logs/events-1.jsonl"

  local block
  block="$(_extract_block "$HEALTH" 'worktree add \.factory factory-artifacts')"
  run _run_block "$REPO" "$block"
  [ "$status" -eq 0 ]

  # Mounted at root — the F1 failure mode was `fatal: '.factory' already
  # exists` because the rev-parse --git-dir guard never fired on a plain dir.
  run _mount_ok "$REPO"
  [ "$status" -eq 0 ]
  # Backed-up contents restored into the fresh worktree, backup removed.
  [ -f "$REPO/.factory/logs/events-1.jsonl" ]
  run bash -c "ls -d '$REPO'/.factory-backup-* 2>/dev/null"
  [ "$status" -ne 0 ]
}

@test "shipped mount block: nested .factory/.factory is refused, recovery block heals it, re-run mounts (#205)" {
  # Fabricate the nested corrupt layout with wrapper contents worth preserving.
  mkdir -p "$REPO/.factory/logs"
  echo evt > "$REPO/.factory/logs/events-1.jsonl"
  git -C "$REPO" worktree add -q .factory/.factory factory-artifacts >/dev/null

  local mount_block recovery_block
  mount_block="$(_extract_block "$HEALTH" 'worktree add \.factory factory-artifacts')"
  recovery_block="$(_extract_block "$HEALTH" 'worktree remove \.factory/\.factory')"
  [ -n "$recovery_block" ]

  # Mount block must HALT (not mount over, not mv-aside a live nested worktree).
  run _run_block "$REPO" "$mount_block"
  [ "$status" -ne 0 ]
  [[ "$output" == *"nested"* ]]

  # Recovery: positively confirmed shape → remove nested worktree, set wrapper
  # aside, prune the dangling registration.
  run _run_block "$REPO" "$recovery_block"
  [ "$status" -eq 0 ]
  [ ! -e "$REPO/.factory" ]

  # Re-run of the mount block completes the heal and restores wrapper contents.
  run _run_block "$REPO" "$mount_block"
  [ "$status" -eq 0 ]
  run _mount_ok "$REPO"
  [ "$status" -eq 0 ]
  [ -f "$REPO/.factory/logs/events-1.jsonl" ]
}

@test "shipped recovery block: refuses to touch a layout that is not the confirmed nested shape" {
  # A healthy repo-root mount must survive an accidental recovery invocation.
  git -C "$REPO" worktree add -q .factory factory-artifacts >/dev/null
  local recovery_block
  recovery_block="$(_extract_block "$HEALTH" 'worktree remove \.factory/\.factory')"
  run _run_block "$REPO" "$recovery_block"
  [ "$status" -ne 0 ]
  [[ "$output" == *"not the known nested shape"* ]]
  # Nothing was deleted or moved.
  run _mount_ok "$REPO"
  [ "$status" -eq 0 ]
}

@test "shipped mount block: prunes a stale registration before re-adding (bad prior recovery)" {
  # Simulate the historical bad recovery: worktree mounted, then rm -rf'd
  # without `git worktree remove`, leaving a dangling .git/worktrees entry.
  git -C "$REPO" worktree add -q .factory factory-artifacts >/dev/null
  rm -rf "$REPO/.factory"

  local block
  block="$(_extract_block "$HEALTH" 'worktree add \.factory factory-artifacts')"
  run _run_block "$REPO" "$block"
  [ "$status" -eq 0 ]
  run _mount_ok "$REPO"
  [ "$status" -eq 0 ]
}

@test "shipped onboard-observability prerequisite: halts on plain dir, passes on healthy mount (#203)" {
  local prereq
  prereq="$(_extract_block "$ONBOARD" 'factory-health first')"
  [ -n "$prereq" ]

  # Plain dir (onboard-first shape): the prerequisite must hard-stop.
  mkdir -p "$REPO/.factory/logs"
  run _run_block "$REPO" "$prereq"
  [ "$status" -ne 0 ]
  [[ "$output" == *"factory-health"* ]]

  # Healthy mount: the prerequisite passes.
  rm -rf "$REPO/.factory"
  git -C "$REPO" worktree add -q .factory factory-artifacts >/dev/null
  run _run_block "$REPO" "$prereq"
  [ "$status" -eq 0 ]
}
