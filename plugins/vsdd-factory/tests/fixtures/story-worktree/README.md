# fixtures/story-worktree — S-21.04 test fixture scaffold

## Purpose

Fixture scaffolding for `story-worktree-write-path-discipline.bats` (S-21.04 gate harness).

Tests exercise BC-6.26.001 v1.6 PC2a (sub-cases a and b) / PC2b (stray files + non-directory
inode) / PC2c: the `find`-based teardown preflight that must run before every `git worktree
remove` on a story worktree, including the v1.6 non-directory path (EC-008/T-6).

## Fixture shape

The fixture is created **dynamically** in a `$(mktemp -d)` temp directory by the bats
`setup()` function — there is no persistent on-disk state to check out. The fixture is
fully cleaned up by `teardown()` via `chmod -R 755 + rm -rf "$WORK"` (the chmod handles
T-004's chmod 000 subdirectory).

A "story-worktree fixture" simulates the state of `.worktrees/<STORY-ID>/` just before
step G teardown. Five fixture configurations are used by the five tests:

```
T-001 (stray file present — PC2b):
  $WORK/
    story-worktree/               ← simulated .worktrees/S-021/
      .factory/
        stories/
          S-021-DELIVERY.md       ← stray .md file: written via CWD-relative path
        engine-config.yaml        ← stray non-.md file: makes -type f any-file-type load-bearing
    canonical-factory/            ← simulated canonical .factory/ mount
    worktree-remove.log           ← sentinel (must remain empty on PREFLIGHT BLOCKED)

T-002 (EC-005 + EC-003 — PC2a sub-cases a and b):
  Part 1 (EC-005): story-worktree/ has NO .factory/ directory → PC2a sub-case (a)
  Part 2 (EC-003): story-worktree/.factory/ exists but is empty → PC2a sub-case (b)
  worktree-remove.log ← sentinel (must have ≥2 entries after both runs)

T-003 (relocate-then-retry — PC2b → PC2a):
  Same as T-001, then stray file moved to canonical-factory/ before retry.
  Re-run shows empty shadow tree → teardown proceeds.

T-004 (find error — PC2c HALT):
  $WORK/
    story-worktree/
      .factory/
        locked-subdir/    ← chmod 000: find encounters permission denied, exits 1
    worktree-remove.log   ← sentinel (must remain empty on PC2c HALT)

T-005 (regular file at .factory — PC2b non-directory, EC-008/T-6):
  $WORK/
    story-worktree/
      .factory             ← REGULAR FILE (not a directory) at worktree root
    worktree-remove.log    ← sentinel (must remain empty on PC2b BLOCKED)
  [ ! -d .factory ] would be TRUE (wrong: authorizes teardown)
  [ ! -e .factory ] is FALSE (correct: path is occupied by non-directory inode)
  find NOT invoked; PREFLIGHT BLOCKED (non-directory case); exit non-zero.
```

## Stray file anatomy

The canonical stray file used across tests:

```
Shadow path:    $WORK/story-worktree/.factory/stories/S-021-DELIVERY.md
Canonical path: $WORK/canonical-factory/stories/S-021-DELIVERY.md
```

The stray file represents a DELIVERY ledger written via a CWD-relative path
(`.factory/stories/S-021-DELIVERY.md`) from inside the story worktree — the primary
failure mode documented in BC-6.26.001 §Description and issue #523.

## Test-double approach

This story requires no external CLI stubs (unlike S-21.03's `gh`/`git` stubs). The
`find` command runs directly on the fixture filesystem. The harness helper function
`_run_teardown_preflight()` (defined inline in the .bats file) implements the
BC-6.26.001 PC2a/PC2b/PC2c logic using an anti-tautology extraction gate:

1. Extract the find command verbatim from step-g-cleanup.md §G.1 (the line matching
   `find ... .factory ... -type f` without `2>/dev/null`).
2. Substitute `<worktree-path>` with the fixture path and `eval` the extracted command.
3. If `.factory/` is absent → PC2a sub-case (a): proceed (REMOVE_LOG written, return 0).
4. If `find` exits non-zero → PC2c HALT: surface exit code + stderr, return 1.
5. If `find` output is non-empty → PC2b BLOCKED: emit PREFLIGHT BLOCKED message + stray
   paths + Option A/Option B + retry mandate, return 1.
6. If `find` exits 0, empty output → PC2a sub-case (b): proceed (REMOVE_LOG written, return 0).

The anti-tautology gate catches two classes of doc-mutant through different mechanisms:

- A `-type d` mutant (replacing `-type f` with `-type d`) is caught by the **extraction
  grep**, not by changed find semantics. The filter `grep -- '-type f'` inside
  `_run_teardown_preflight` does not match a `-type d` line, leaving `find_cmd_line` empty.
  The function returns 1 with `HARNESS FAIL: could not extract conformant find command...`.
  T-001 fails because `PREFLIGHT BLOCKED` is absent from output; T-002 fails because
  `worktree-remove-invoked` is absent. The load-bearing gate is the extraction grep.

- A `-name '*.md'` mutant (restricting find to `.md` files only) is caught by the **non-.md
  output assertion** in T-001. `engine-config.yaml` (a `.yaml` stray file in the T-001
  fixture) would not be returned by a `-name '*.md'`-restricted find command, so
  `grep -q 'engine-config.yaml'` fails. The load-bearing gate is the non-.md artifact
  assertion, not changed find semantics.

A harness hardcoding its own `find ... -type f 2>/dev/null || true` would not catch either
class of doc-mutant.

This approach follows the W1 S-21.03 precedent of inline bash harness helpers operating
against a minimal fixture setup, adapted from CLI-stub fixtures (S-21.03) to filesystem
fixtures (S-21.04) because the preflight mechanism is `find` (filesystem-direct).

## EC coverage table

| EC ID | Description | Test |
|-------|-------------|------|
| EC-003 | `find` returns empty (dir exists, no files) | T-002 part 2 (explicit EC-003 variant) |
| EC-005 | Story worktree has no `.factory/` directory at all | T-002 part 1 |
| EC-004 | `find` returns stray file (shadow .factory/ has content) | T-001, T-003 (first pass) |
| EC-007/EC-008 | Regular file (not directory) at `.factory/` path → PC2b BLOCKED without find | T-005 (BC-6.26.001 v1.6 T-6) |
| PC2c   | `find` exits non-zero for non-path-absent reason | T-004 (chmod 000 subdir) |

## POLICY 21 note

If shim scripts are added to `tests/fixtures/` in the future, they fall under the v1.4.9
fixtures exemption (test infrastructure only; not shipped in release bundles).
