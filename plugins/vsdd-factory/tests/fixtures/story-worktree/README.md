# fixtures/story-worktree — S-21.04 test fixture scaffold

## Purpose

Fixture scaffolding for `story-worktree-write-path-discipline.bats` (S-21.04 gate harness).

Tests exercise BC-6.26.001 PC2a / PC2b: the `find`-based teardown preflight that must run
before every `git worktree remove` on a story worktree.

## Fixture shape

The fixture is created **dynamically** in a `$(mktemp -d)` temp directory by the bats
`setup()` function — there is no persistent on-disk state to check out. The fixture is
fully cleaned up by `teardown()` via `rm -rf "$WORK"`.

A "story-worktree fixture" simulates the state of `.worktrees/<STORY-ID>/` just before
step G teardown. Three fixture configurations are used by the three tests:

```
T-001 (stray file present):
  $WORK/
    story-worktree/               ← simulated .worktrees/S-021/
      .factory/
        stories/
          S-021-DELIVERY.md       ← stray file: written to shadow tree via CWD-relative path
    canonical-factory/            ← simulated canonical .factory/ mount
    worktree-remove.log           ← mock invocation log (must remain empty on PREFLIGHT BLOCKED)

T-002 (empty shadow tree):
  $WORK/
    story-worktree/               ← simulated .worktrees/S-021/ — no .factory/ content
    canonical-factory/
    worktree-remove.log           ← mock invocation log (must contain "worktree-remove-invoked")

T-003 (relocate-then-retry):
  Same as T-001, then stray file moved to canonical-factory/ before retry.
  Re-run shows empty shadow tree → teardown proceeds.
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
BC-6.26.001 PC2a/PC2b logic:

1. `find "$worktree_path/.factory" -type f 2>/dev/null`
2. If output is non-empty: print `PREFLIGHT BLOCKED` message with stray paths; do NOT
   write to `$REMOVE_LOG` (git worktree remove NOT called)
3. If output is empty: write `worktree-remove-invoked` to `$REMOVE_LOG` (teardown proceeds)

This approach follows the W1 S-21.03 precedent of inline bash harness helpers operating
against a minimal fixture setup, adapted from CLI-stub fixtures (S-21.03) to filesystem
fixtures (S-21.04) because the preflight mechanism is `find` (filesystem-direct) rather
than `gh`/`git` (CLI tools requiring stubs).

## POLICY 21 exemption

Test-double shims in `tests/fixtures/` are POLICY 21-exempt per v1.4.9: these files are
test infrastructure only and do not ship in release bundles. No `.sh` files are placed
outside `tests/fixtures/` by this story.
