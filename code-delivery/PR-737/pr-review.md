# PR #737 Re-Review — head bf0f13e7

**Title:** fix(skills): assert .factory mounts at repo root; gate onboard-observability on the mount
**Repo:** drbothen/vsdd-factory
**Reviewer:** pr-reviewer (fresh-context, different-model cognitive diversity)
**Prior review:** 2026-07-22, prior head ~e828-era (F1–F5)

## Verdict: REQUEST_CHANGES

APPROVE requires BOTH HIGH findings (F1, F2) to be structurally FIXED. F1 is
UNADDRESSED (the flagged flawed line is present verbatim in this head) and F2 is
only PARTIAL (destructive `rm -rf .factory` + missing `git worktree prune`
persist). Verdict criteria not met.

---

## Per-finding closure status

### F1 (HIGH) — plain-dir guard uses `rev-parse --git-dir`, a no-op in its target case
**Status: UNADDRESSED**

The PR added a *post-mount* assertion using `--show-toplevel`, but the
**pre-mount plain-dir guard** — the exact thing F1 flagged — still uses the
flawed `rev-parse --git-dir` test, unchanged:

```bash
if [ -e .factory ] && ! git -C .factory rev-parse --git-dir >/dev/null 2>&1; then
  mv .factory .factory-backup-$(date +%s)   # plain dir, not a worktree
fi
git worktree add .factory factory-artifacts
```

Failure scenario (the guard's stated target case — a plain `.factory/logs/`
left by `/onboard-observability`): `.factory` is a subdirectory *inside* the
repo working tree. `git -C .factory rev-parse --git-dir` therefore walks up,
discovers the parent repo's `.git`, prints it, and **exits 0**. The negation
`! git ... rev-parse --git-dir` is false, so the guard body never runs and the
plain dir is NOT moved aside. `git worktree add .factory factory-artifacts`
then fails with `fatal: '.factory' already exists` — precisely the bug the PR
claims to fix. `rev-parse --git-dir` returns exit 0 for BOTH a real worktree
AND a plain in-repo dir, so it can never distinguish them.

The fix suggested in the prior review (use `--show-toplevel` equality, e.g.
`[ "$(git -C .factory rev-parse --show-toplevel 2>/dev/null)" != "$(git rev-parse --show-toplevel)/.factory" ]`)
was applied only to the separate post-mount assertion, not to the guard. The
guard remains inert.

Note: the new content-contract test (test 3) only greps for the word `plain`
in the SKILL — it does NOT execute the guard, so it cannot detect that the
guard is inert. The 4 execution tests validate the `--show-toplevel` assertion
logic, never the `rev-parse --git-dir` guard. Passing tests here do not
constitute a load-bearing closure (TD-VSDD-059).

### F2 (HIGH) — destructive `rm -rf .factory` recovery, string-comparison fragility, dangling registration
**Status: UNADDRESSED / PARTIAL (not a clean FIX)**

New nested-mount recovery block:

```bash
git worktree remove .factory/.factory --force
rm -rf .factory        # now a leftover plain dir; the guard above re-handles it
```

Improvement: the `rm -rf .factory` is now reached only after the post-mount
assertion fails, rather than being gated inline on a comparison. But the
structural risk F2 named persists:

1. **String-comparison fragility still gates a destructive op.** The gating
   assertion is itself a raw string equality
   `[ "$(git -C .factory rev-parse --show-toplevel)" = "$(git rev-parse --show-toplevel)/.factory" ]`.
   On a symlink/path-canonicalization mismatch this can FALSE-FAIL on a
   *healthy* repo-root worktree, routing a human/agent into the recovery block
   where `rm -rf .factory` then deletes the contents of a live worktree. The
   recovery has no positive confirmation that the layout is actually nested
   (e.g. `test -e .factory/.factory/.git`) before the destructive `rm`. The
   test suite even acknowledges this canonicalization hazard by doing
   `pwd -P` (/var→/private/var) in setup — the production skill has no such
   safeguard.

2. **Dangling registration / missing `git worktree prune` is UNADDRESSED.**
   The recovery contains no `git worktree prune`. If `.factory` itself carries
   a stale worktree registration (the corrupt state #205 describes),
   `rm -rf .factory` leaves a dangling entry in `.git/worktrees` and the next
   `git worktree add .factory` can fail with "already registered". F2 called
   this out explicitly; there is no prune step anywhere in the diff.

3. The trailing comment "the guard above re-handles it" is misleading given
   F1: the guard is inert, so re-running does not mv-aside — the mount only
   succeeds incidentally because `.factory` was just `rm`'d to absent.

### F3 (MEDIUM) — `|| echo ABORT` doesn't halt
**Status: UNADDRESSED**

```bash
[ "$(git -C .factory rev-parse --show-toplevel)" = "$(git rev-parse --show-toplevel)/.factory" ] \
  || echo "ABORT: .factory did not mount at the repo root"
```

Still `|| echo`. This prints and continues; step 3 (`cd .factory && git branch
--show-current`) then reads the parent branch of a nested/plain layout and the
check reports healthy — the exact silent-misbehavior the PR set out to prevent.
Needs `{ echo "ABORT..."; exit 1; }` (or equivalent hard stop).

### F4 (MEDIUM) — worktree tests sharing $REPO as flake source
**Status: FIXED**

`setup()` now creates a fresh `WORK="$(mktemp -d)"` / `REPO="$WORK/repo"` per
test (bats runs `setup()` before each test), and `teardown()` runs
`git -C "$REPO" worktree prune` + `rm -rf "$WORK"`. Each execution test mounts
its own `.factory` in an isolated repo, so the healthy / nested / plain-dir /
absent tests no longer contend for a shared `$REPO`. Load-bearing structural
change; closure accepted.

### F5 (MEDIUM) — backup-restore is comment-only
**Status: UNADDRESSED**

```bash
git worktree add .factory factory-artifacts
# If a backup was made, restore any contents (e.g. logs/) into the worktree:
#   cp -R .factory-backup-*/. .factory/ 2>/dev/null || true
#   rm -rf .factory-backup-*
```

Restore remains commented out. Any data moved to `.factory-backup-<ts>` by the
guard (if the guard ever fired) is silently orphaned; the user is left with a
`.factory-backup-*` dir and no automatic restoration.

---

## New findings at this head

- **N1 (LOW):** Recovery `rm -rf .factory` lacks a positive nested-layout
  confirmation. Add `test -e .factory/.factory/.git` (or a worktree-list check)
  before the destructive `rm`, and add `git worktree prune` after, so a
  false-failed assertion cannot destroy a healthy worktree (ties to F2).
- **N2 (LOW):** onboard-observability Prerequisite 1 presents the mount check
  as a bare `[ ... ]` with prose "abort with ..." but no `|| { echo ...; exit 1; }`.
  Same halt-doesn't-halt shape as F3; as an executable snippet it does not
  enforce the abort.

## Checklist notes

- Diff coherence: all changes relate to the #205/#203 mount-ordering hazard. OK.
- Description accuracy: the "What changed" body claims a working plain-dir
  guard; the shipped guard is inert (F1). Description overstates the fix.
- Test coverage: content-contract tests grep for prose only and cannot detect
  the inert guard or the non-halting abort; execution tests validate
  `--show-toplevel` logic but not the `rev-parse --git-dir` guard or the
  `rm -rf` recovery path. The two HIGH defects are untested.
- Diff size: 205/-3 — reasonable.

## Summary table

| ID | Sev | Status |
|----|-----|--------|
| F1 | HIGH | UNADDRESSED |
| F2 | HIGH | UNADDRESSED / PARTIAL |
| F3 | MEDIUM | UNADDRESSED |
| F4 | MEDIUM | FIXED |
| F5 | MEDIUM | UNADDRESSED |
| N1 | LOW | new |
| N2 | LOW | new |

**REQUEST_CHANGES** — both HIGH findings unresolved.
