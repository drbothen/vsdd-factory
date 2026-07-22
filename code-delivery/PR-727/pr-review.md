# Fresh-Eyes PR Review — PR #727

**PR:** fix(tooling): resolve .factory worktree cwd-independently in factory-cas-push.sh
**Branch:** fix/cas-push-worktree-cwd → develop
**Issue:** #631
**CI:** GREEN (bats-full-suite, bats-darwin-leg, bats-wave-handoff, cargo-host ubuntu+macos, build-dispatcher all targets, SAST Semgrep)

## Verdict: REQUEST_CHANGES

The core fix is sound and solves the reported cwd-dependency bug with a clean, backward-compatible fast path. Blocking on one MAJOR defect in the new fallback extraction (path-with-spaces truncation) plus a latent fragility, both on the exact code path this PR introduces.

---

## Findings

### [BLOCKING / MAJOR] `awk '{print $2}'` truncates worktree paths containing spaces

```sh
MAIN_WT="$(git worktree list --porcelain 2>/dev/null | awk '/^worktree /{print $2; exit}')" || MAIN_WT=""
```

`git worktree list --porcelain` emits `worktree <absolute-path>` with the path **raw and unescaped** — only the `-z` variant is safe for special characters; `--porcelain` is not. `awk` splits on whitespace, so `$2` captures only the first token of the path. If the main worktree lives at e.g. `/Users/zious/Application Support/vsdd-factory`, `MAIN_WT` becomes `/Users/zious/Application`, `[ -d "$MAIN_WT/.factory" ]` is false, and the script prints the "could not locate" error and exits 1.

This is precisely the fallback the PR exists to add, so the defect hides behind the fast path in the common case and in CI (fixtures use no-space paths). It fails the moment the repo is cloned under a space-containing path — plausible on macOS, which this project explicitly targets. The PR carefully quotes `"$FACTORY_DIR"` everywhere to survive spaces, then truncates the path during extraction, which defeats the quoting.

**Fix (trivial):**
```sh
| awk '/^worktree /{ sub(/^worktree /, ""); print; exit }'
```
or `substr($0, 10)`. Per the project production-grade default, fix in scope rather than ship as "good enough for our path."

### [SUGGESTION / MINOR] `awk … exit` under `set -o pipefail` can wipe a valid `MAIN_WT`

`awk` calls `exit` on first match, closing the pipe read end early. If `git worktree list` is still writing, it takes SIGPIPE (exit 141); with `pipefail` the pipeline reports non-zero, the assignment "fails," and `|| MAIN_WT=""` overwrites the already-captured path with empty — routing execution into the error/exit-1 branch even though resolution succeeded.

Practically unreachable: worktree-list output for a handful of worktrees is well under the ~64KB pipe buffer, so `git` completes and exits 0 before `awk` closes the pipe. Only bites with hundreds of worktrees. Flagged as latent fragility on the critical path. Cleanest hardening: parse without early `exit`, or use `git rev-parse --path-format=absolute --git-common-dir` and take its parent, sidestepping porcelain parsing entirely.

### [NIT] Fast path `[ -d ".factory" ]` does not verify the dir is the factory worktree

Accepts any directory literally named `.factory` in cwd without confirming it is the factory-artifacts worktree. Low risk here (`.factory` is a gitignored worktree mount, not committed content). Noted for completeness.

---

## Could Not Fully Verify

- **New bats tests.** The 4 tests and `_setup_worktree_fixture` are elided in the diff (`[... omitted for brevity]`), so git-identity setup, worktree wiring, and `$WORK` isolation across tests are unassessable from what was provided. Before merge, the test source should be reviewed, and a test asserting resolution works when the main worktree path contains a **space** should be added (a no-space fixture will not catch the BLOCKING finding above).

## Verified Good

- **Path tracing:** cwd == `.factory` → fast path false → fallback derives `<main>/.factory` correctly. Repo-root and sibling-worktree cwds resolve correctly.
- **First-entry-is-main:** `git worktree list` always emits the main worktree first from any worktree — the `awk … exit` first-match assumption is a guaranteed git property.
- **`|| MAIN_WT=""` vs `set -e`:** correctly prevents `set -e` abort when not in a git repo (modulo the SIGPIPE edge above).
- **Backward compat:** repo-root fast path yields `git -C ".factory"`, byte-identical to the old `git -C .factory` (quoting inert for a no-space literal). No regression.
- **`$FACTORY_DIR` quoting:** quoted at all four call sites.
- **`$(pwd)` in error path:** best-effort in the failure branch; a `pwd` failure yields at worst an empty interpolation, not a functional problem.

---

**Bottom line:** Correct, no-regression fix for the cwd-dependency bug. Blocking only on the space-truncation defect — a one-line fix on the very path this PR introduces — and requesting the elided test source (with a space-path case) for verification.
