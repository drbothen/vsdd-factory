# PR #735 Fresh-Eyes Review — fix(skills): factory-artifacts orphan branch via plumbing

**Reviewer:** pr-reviewer (fresh-context, different-model cognitive diversity)
**Base:** develop · **Files:** 3 (+305 / -10) · **Class:** Fix-PR (production-grade bar)
**Verdict:** REQUEST_CHANGES

---

## Summary

The core mechanism is correct and is a genuine improvement over the checkout dance.
`git branch <name> "$(git commit-tree -S "$(git mktree </dev/null)" -m ...)"` builds a
parentless empty-tree commit and points a ref at it without ever moving `HEAD` or
touching the working tree. That eliminates both original defects (stranded `HEAD` from
`git checkout -` after `--orphan`, and destructive `git rm -rf .` / wrong hardcoded
return branch).

Two issues block approval under the production-grade bar, both centered on the newly
**mandatory** `-S` signing flag: it is a robustness regression on an auto-repair path
that runs in varied environments, and the test suite treats the one hard dependency the
recipe introduces as optional — so CI cannot catch the regression.

### Review limitation
The diff contains only a placeholder comment for `tests/orphan-branch-plumbing.bats`
(the 290-line body is not in the provided diff). Findings #2 and #4 concern test
internals and are therefore assessed from the PR description and test-name list, not the
actual test source. They are marked accordingly.

---

## Findings

### [HIGH] Mandatory `-S` is a robustness regression on an auto-repair path
- **Category:** correctness / coherence
- **Where:** both SKILL.md recipes (`git commit-tree -S ...`)
- The OLD `factory-health` recipe used `git commit --allow-empty`, which is *adaptive*:
  it signs iff `commit.gpgsign=true` is configured, and otherwise produces an unsigned
  commit successfully. The NEW recipe uses `git commit-tree -S`, which **forces**
  signing unconditionally. `git commit-tree` does **not** honor `commit.gpgsign`, so
  there is no config path to opt out.
- Consequence: in any environment without a configured signing key (fresh CI runner,
  agent container, or a **downstream consumer of the distributed vsdd-factory plugin**),
  `git commit-tree -S` fails (non-zero, "gpg failed to sign the data" / SSH key load
  error). Where the old recipe *succeeded* (unsigned), the new one now *fails* — and
  this is on `factory-health` / `factory-worktree-health`, which run at session/pipeline
  start as auto-repair. A robustness regression on the repair path is worse than the bug
  being fixed for those environments.
- These SKILL.md files ship in the marketplace plugin and run in environments the author
  does not control, so "this repo mandates signed commits" does not cover all consumers.
- **Suggestion:** Preserve the strand-bug fix but make signing non-fatal for
  unconfigured environments. Options, in order of preference:
  1. Drop `-S` on the empty init marker commit (real content commits on
     `factory-artifacts` are signed later by state-manager); the init commit's signature
     carries marginal value.
  2. Make it config-driven: sign only when a signing key is present (detect via
     `git config --get user.signingkey` / `commit.gpgsign`), else fall back to unsigned —
     replicating the old adaptive behavior while keeping the no-checkout fix.
  3. If mandatory signing is a deliberate hard requirement, document the signing-key
     prerequisite in both SKILL.md files and pair it with #2 below (a hard CI gate).

### [HIGH] Test/production signing asymmetry — CI cannot catch the regression
- **Category:** test-coverage
- **Where:** test 9 ("new recipe with -S produces a verifiable signature") — skips when
  SSH signing is unavailable; production recipe hard-requires `-S`.
- The single hard dependency the recipe introduces (signing) is the one thing the suite
  treats as optional. On a machine without signing configured, tests 1–8/10–13 pass and
  test 9 skips → suite is green, while the actual recipe emitted in SKILL.md would
  **fail** on that same machine. The test matrix is exactly inverted relative to the
  runtime risk.
- **Suggestion:** Resolve jointly with #1. If signing stays mandatory, provision an
  ephemeral signing key in CI and make test 9 a **mandatory** gate (no skip) that
  additionally asserts the recipe *fails cleanly with a clear message* when no key is
  present. If signing becomes config-driven/optional (preferred), add a test for the
  no-key path proving the branch is still created (unsigned) rather than the repair
  aborting.
- *(Assessed from PR description; test body not in diff.)*

### [MEDIUM] commit-tree failure yields `git branch <name> ""` — misleading secondary error, no guard
- **Category:** correctness
- **Where:** both recipes; nested command substitution with no error handling.
- If the inner `git commit-tree` fails (e.g. signing failure per #1), the substitution
  expands to empty and the outer command becomes `git branch factory-artifacts ""`,
  which errors with `fatal: Not a valid object name: ''` — a confusing message that
  points at branch creation, not the real (signing) cause. No branch is created and the
  agent following the SKILL is left to diagnose a misleading error. The one-liner has no
  `set -euo pipefail`, no non-empty check on the captured SHA.
- **Suggestion:** Split into two steps and validate:
  ```bash
  commit=$(git commit-tree "$(git mktree </dev/null)" -m "chore: initialize factory-artifacts orphan branch") \
    || { echo "failed to create init commit" >&2; exit 1; }
  git branch factory-artifacts "$commit"
  ```
  (with `-S` added per the resolution of #1). Capturing to a variable also makes the
  failure attributable.

### [MEDIUM] Behavioral tests likely validate an embedded copy, not the actual SKILL.md recipe
- **Category:** test-coverage
- Tests 1–4 verify behavior of *a* recipe, tests 10/12 assert SKILL.md "uses the plumbing
  recipe," and 11/13 assert the checkout dance is gone. From the names, 10–13 appear to be
  substring/contract matches and 1–4 exercise a recipe string. If tests 1–4 run an embedded
  copy of the recipe rather than the exact fenced block extracted from SKILL.md, then a
  typo introduced into SKILL.md (e.g. a dropped quote, wrong flag) would still pass the
  behavioral tests while shipping a broken recipe — false confidence.
- **Suggestion:** Extract the fenced ```bash``` recipe directly from each SKILL.md and
  execute *that exact text* in the behavioral tests, binding behavior to the shipped
  content. If already done, disregard.
- *(Assessed from PR description; test body not in diff — please confirm.)*

### [LOW] factory-worktree-health checks remote existence but creates a local branch
- **Category:** correctness / edge-case
- The guard is `git ls-remote --heads origin ${BRANCH_NAME}` (remote), but the repair does
  `git branch ${BRANCH_NAME} ...` (local). On a re-run where the local branch already
  exists but the remote does not, `git branch` fails with "already exists." The remote
  guard does not cover local-branch existence.
- **Suggestion:** Guard on local existence too (`git show-ref --verify --quiet refs/heads/${BRANCH_NAME}`)
  before creating, or use `git branch -f`/create-if-absent semantics, then push.

### [LOW] Unquoted `${BRANCH_NAME}` in the worktree-health recipe and push
- **Category:** shell-hygiene
- `git branch ${BRANCH_NAME}` and `git push origin ${BRANCH_NAME}` are unquoted. The value
  is controlled, so no functional bug, but production-grade shell snippets in a distributed
  skill should quote (`"${BRANCH_NAME}"`).

### [ADVISORY] RED reproduction tests (5/6/7) freeze a historical broken recipe
- Tests 5/6/7 embed the old recipes to prove they were broken. This is good regression
  documentation and does not drift (the old recipe is no longer in SKILL.md, per tests
  11/13). But they test now-dead code and will not catch a *different* future regression.
  Acceptable; low maintenance burden. No change required.

### [ADVISORY] No coverage of the push step in factory-worktree-health
- The `git push origin ${BRANCH_NAME}` step (remote interaction) is untested. Push ordering
  itself is correct (create local ref, then push; sequential, no race — a concurrent
  remote branch would cause a safe non-fast-forward rejection, not a clobber). Hard to test
  in CI without a remote fixture; acceptable to leave uncovered.

---

## Checklist verification

1. **Diff coherence** — PASS. All three files serve the single fix.
2. **Description accuracy** — PASS. Description matches the diff; root-cause analysis is
   accurate (`git checkout -` cannot resolve `@{-1}` after `--orphan`).
3. **Test coverage** — PARTIAL. 13 tests cover behavior + RED reproductions + contract.
   Gaps: signing-failure path (#2), no-key branch creation, recipe-extraction binding (#4).
4. **Demo evidence** — N/A for a skill/shell-recipe fix; bats TAP output serves as
   evidence. Acceptable for this class.
5. **Commit quality** — Conventional format, clear message. PASS.
6. **Diff size** — 305 lines, mostly the new test file. Reasonable.
7. **Missing changes** — The plumbing recipe correctly replaces both dances. Mandatory
   `-S` introduces a new gap (#1) not present in the original intent.
8. **Dependency status** — No upstream PR dependency evident.

### Recipe correctness answers to the review questions
- `git mktree </dev/null` → empty tree object; `git commit-tree` with no `-p` → parentless
  orphan commit; `git branch <name> <sha>` → ref only, `HEAD`/worktree untouched. Mechanism
  is correct across all supported git versions (all three commands are long-stable).
- `-S` failure mode is **loud** (non-zero + stderr), not silent — but it cascades into the
  misleading `git branch <name> ""` secondary error (#3), and it is a **regression** vs the
  old adaptive `git commit` behavior (#1).
- Push ordering is correct; no race (#ADVISORY above).

---

## Verdict: REQUEST_CHANGES

Blocking on **#1 (mandatory `-S` regression)** and **#2 (test/production signing
asymmetry)**. The recipe is directionally right and fixes real bugs, but forcing signing on
a distributed auto-repair path — while the test suite makes signing optional — fails the
production-grade bar. Resolve #1 (drop or config-gate `-S`) and #2 (align the test gate),
address #3/#4 (error handling + recipe-extraction binding), and this is a clean approve.
