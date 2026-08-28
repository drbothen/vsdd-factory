# PR Review — #786 `fix/rc24-orphan-wasm-policy15` → `develop`

**Reviewer:** `vsdd-factory:pr-reviewer` (fresh-eyes, final pre-merge review)
**Verdict:** **REQUEST_CHANGES** — 1 BLOCKING (merge-gate; **not** a diff defect)
**covered_sha:** `29fc003cbba9818999e41d8a575ca78151bd55c1`
**Diff size:** 2 files, +6 / −1 text lines, −337,816 bytes binary. Well under the 500-line flag.

> **File note:** written to `pr-reviewer-review.md` rather than `pr-review.md` because a
> concurrent `code-reviewer` cycle-2 pass overwrote `pr-review.md` mid-flight. That file now
> holds the code-reviewer's review, not this one. Two distinct agents, two distinct artifacts —
> worth a filename convention fix (`pr-review-<agent>.md`) so the code-reviewer and pr-reviewer
> outputs stop colliding.

---

## Bottom line

**The diff is correct and I found zero defects in it.** The root-cause analysis in the PR body
is accurate, the removal is safe, and both the `--exclude` and the new staging-loop `case` arms
are syntactically and semantically right. I independently re-derived the root cause from the
diff and from `cargo metadata` and reached the same conclusion the author did.

I am nonetheless issuing REQUEST_CHANGES for one reason: **`cargo-host` is RED on both legs
(macos-latest AND ubuntu-latest) at `29fc003c`, deterministically, and the PR body asserts a
`ci-green` badge.** The failure is provably **not** caused by this diff — it is a regression on
the `factory-artifacts` branch. It must be cleared by `state-manager` before the merge button
is pressed. **No change to this PR's diff is warranted or wanted.** See F-001 for the exact fix
and owner.

---

## Findings

| ID | Severity | Category | Finding |
|----|----------|----------|---------|
| F-001 | **BLOCKING** | test-evidence / merge-gate | `cargo-host` RED on both legs from a `factory-artifacts` STATE.md regression; PR body claims `ci-green`. Not a diff defect. |
| F-002 | ADVISORY | coverage / structural | Bundle staging is still a **denylist** with no reverse orphan assertion, and the bot-commit-to-`main` path that caused this bug has no orphan gate at all. `29fc003c` hardens the instance, not the class. |
| F-003 | ADVISORY | coverage | Nothing pins either new defence. Delete both and no test goes red. Sibling `read-prefix-fixture` is pinned by T-011; `policy15-attestation-gate` is not. |
| F-004 | NITPICK | description | `![Tests](tests-51/51)` and `![CI](ci-green)` badges are unverified/false at `covered_sha`. |

---

### F-001 — BLOCKING — `cargo-host` is RED on both legs; cause is a `factory-artifacts` regression, not this diff

`cargo-host` failed on **both** matrix legs at `29fc003c` (run `33127166291`: macos-latest
`fail` 5m11s, ubuntu-latest `fail` 4m6s). I pulled both job logs — byte-identical failure,
same two tests, same assertions. This is deterministic and cross-platform, not a flake:

```
thread 'tests::test_BC_5_39_005_f_p1_001_real_state_md_banner_wc_passes' panicked at
crates/hook-plugins/validate-state-structure/src/lib.rs:2548:13:
extract_banner_line_count returned None for real STATE.md — F-P1-001 is NOT closed;
banner has no 'N lines (wc-l...)' pattern.

thread 'tests::test_BC_5_39_005_full_validation_against_real_state_md' panicked at
crates/hook-plugins/validate-state-structure/src/lib.rs:2291:9:
validate_banner_wc must return None for real STATE.md; got: Some("no SIZE BUDGET banner
found; STATE.md MUST include an HTML comment banner with 'N lines (wc-l)' claim per
D-421(c)+D-422(c)+D-424(b)+D-428(d)+D-438(a)+D-440(d)+D-442(d)")

test result: FAILED. 63 passed; 2 failed
```

**Why this is not your bug.** `ci.yml`'s `cargo-host` job mounts the *live, mutable*
`origin/factory-artifacts` branch at `.factory/` (`git worktree add .factory
origin/factory-artifacts`, ci.yml:151-153). Both tests read
`CARGO_MANIFEST_DIR/../../../.factory/STATE.md` and skip gracefully when it is absent — so they
only assert when that worktree is mounted, i.e. in CI and on dev machines with the factory
worktree live. Their input is a branch this PR does not touch.

**Exact root cause, bisected on `origin/factory-artifacts`:**

- `1a1dc0d1` (2026-08-25) — *"state(corpus): reconcile … + STATE.md banner wc-l self-count for
  rc.24 CI"* — **added** the D-446(c) dual-margin line ending in `261 lines (wc-l).` This is
  why `develop` CI at `1227d504` (2026-08-25 21:56Z) was green.
- `c3abf5fb` (2026-08-26) — *"factory(pause): session wrap — ADR-046 spec-convergence 1/3,
  rc.24 shipped…"* — **deleted** that line and did not replace it:

```diff
-  Hard cap (500 lines) margin from soft-target = 500 - 415 = 85; margin from actual = 500 - 261 = 239 (D-446(c) dual-margin form). 261 lines (wc-l).
```

`git show origin/factory-artifacts:STATE.md | grep -c "wc-l"` → **0**. Every PR opened against
`develop` after `c3abf5fb` inherits this red. PR #786 is the messenger, not the cause.

**Second-order consequence — flagging to the release owner.** `d39c62e0` / `1227d504` (already
on `develop`, **not** part of this PR) promoted release.yml's validate step from a compile-only
`--no-run` warm-up to a real `cargo test --workspace --all-targets` with
`VSDD_CORPUS_ROOT: ${{ github.workspace }}/.factory`. That restored release-gate parity — and in
doing so coupled the **release pipeline** to the same mutable STATE.md input. As things stand,
**cutting rc.25 will fail at the validate job for this same reason**, independent of this PR.
Fixing F-001 unblocks both.

**Required remediation — owner `vsdd-factory:state-manager`, NOT this PR's author:**

1. Restore the D-446(c) dual-margin + `N lines (wc-l)` self-count line in the STATE.md SIZE
   BUDGET banner on `factory-artifacts`, count refreshed to the file's actual `wc -l`, per
   D-421(c)+D-422(c)+D-424(b)+D-428(d)+D-438(a)+D-440(d)+D-442(d)+D-446(c).
2. Re-run `cargo-host` on `29fc003c`.
3. Green → **APPROVE**, with no diff change.

This is surface-with-routing, not a defer (Companion Principle rule 3): the fix happens in the
same work cycle, by the artifact's owning specialist.

> On the `c3abf5fb` class: a session-wrap commit silently dropping a load-bearing attestation
> line is exactly the failure mode D-449(a) was codified against — a narrative edit degraded a
> mechanical gate's *input* without the gate being invoked. The gate here is a Rust test rather
> than a shell one-liner, but the ply is the same, and it went undetected for a day because the
> `main`→`develop` sync that carried it (`6993138b`) never ran `ci.yml`. Worth an
> L-EDP1-NNN entry at `state-manager`'s discretion.

---

### F-002 — ADVISORY — `29fc003c` hardens the instance; the class is still open

Credit where due: `29fc003c` is a good commit. Two `case` arms, one per staging loop, mirroring
`read-prefix-fixture.wasm` exactly, `; continue ;;` terminator intact, ordering correct (the
basename has no underscore so the outer `*_*.wasm` arm cannot shadow it), and inert on the
happy path since `--exclude` means the artifact is never produced. `git show --stat` confirms
`1 file changed, 2 insertions(+)` — no collateral edits. It closes the single-point-of-failure
concern properly.

It does not close the class, and I want that on the record so the next occurrence isn't a
surprise. Both staging loops remain **denylists**: copy everything, skip a hardcoded list.

```bash
for wasm in target/wasm32-wasip1/release/*.wasm; do
  case "$name" in
    *_*.wasm) … continue ;;
    read-prefix-fixture.wasm) … continue ;;
    policy15-attestation-gate.wasm) … continue ;;   # ← 29fc003c
  esac
  cp "$wasm" artifact/          # everything else ships, registered or not
done
```

Both "Verify registry-declared WASM plugins are staged" steps check **only the forward
direction** — every registry-declared wasm is present. Neither checks the converse: that every
*staged* wasm is registry-declared. That asymmetry is the hole
`policy15-attestation-gate.wasm` fell through, and it is unchanged by this PR. The denylist now
has three entries; entry four will be discovered the same way this one was.

`T-009` (`bundle_orphan_check.rs::test_S_19_04_ac006_T009_hermetic_tracked_bundle_zero_orphans`)
does assert the reverse direction, and it is genuinely well-built — I read it, and the
`.expect()`-over-`filter_map` bypass-mutant reasoning at lines 1610-1625 is real work, not
decoration. But T-009 runs under `cargo test`, i.e. on **PR** events. The actual recurrence path
was: bot commit `89f6f87c` straight onto `main` → merge `6993138b` (*"merge: sync main → develop
after v1.0.0-rc.24 bundle"*) → **no `ci.yml` run for either SHA** (develop's CI history jumps
from `1227d504` to nothing). T-009 never got a chance to fire. The one code path that ships
wasm to operators is the one path with no orphan gate on it.

Suggested structural close (~10 lines, reuses the registry parse already present in the same
step):

```bash
# Reverse orphan assertion: every staged wasm must be registry-declared.
declared_set=" ${hooks_plugins[*]} ${resolver_plugins[*]} "
orphans=()
for f in plugins/vsdd-factory/hook-plugins/*.wasm; do
  n=$(basename "$f")
  [[ "$declared_set" == *" $n "* ]] || orphans+=("$n")
done
if [ "${#orphans[@]}" -gt 0 ]; then
  echo "::error::${#orphans[@]} staged WASM(s) not declared in either registry:"
  printf '  orphan: %s\n' "${orphans[@]}"
  exit 1
fi
```

In commit-binaries' verify step this turns the bot-commit path from "silently ships an orphan"
into "release fails loudly", and demotes the denylist from sole defence to third line of
defence. I'd take it in-scope (CLAUDE.md Rule 4; expanding scope to close a finding correctly
is the sanctioned default, and "adding plumbing that was missing" is explicitly *not* the
redesign that Standing Rule 3 §4 warns against). I defer to `pr-manager` on whether it lands
here or as an immediate follow-up with a real issue anchor — F-001 already gates the merge, so
there is time. What is **not** acceptable is a tech-debt-register entry absent explicit human
direction (Canonical Principle Rule 3).

---

### F-003 — ADVISORY — nothing pins either new defence

`bundle_orphan_check.rs` documents T-011 as:

> *"POLICY 20 defense proof: read-prefix-fixture.wasm (hyphen-named) passes the `*_*.wasm`
> staging glob and is an orphan per both registries; **proves `--exclude read-prefix-fixture`
> in release.yml is the governing defense** (S-19.06)"*

`policy15-attestation-gate` is now in exactly the same position — hyphen-named, so it sails past
the `*_*.wasm` skip arm; a native `src/main.rs` CLI with no wasm target; orphan per both
registries — but has **no** analogous pin. Delete the `--exclude` line *and* both `case` arms
and no test goes red. Only T-009 catches the consequence, and only on a PR event, which F-002
shows is the one event the recurrence path skips.

Suggestion: extend T-011 (or add T-011b) to cover `policy15-attestation-gate.wasm` on the same
pattern. This is TD-VSDD-059 applied to this PR: the *outcome* is gated (T-009), the *mechanism*
is not. Three protections were added across this PR and zero of them are load-bearing under
test.

While there: the T-011 docstring asserts that removing `--exclude read-prefix-fixture` would let
`read-prefix-fixture.wasm` "pass through staging" — which the real workflow already contradicts,
because a named skip arm for it exists. `29fc003c` widens that documentation drift by one entry.
Pre-existing, non-blocking, and drifting in the conservative direction (workflow is safer than
the test claims), but worth correcting when the pin above is added.

---

### F-004 — NITPICK — badges in the PR body overstate the evidence

`![CI](https://img.shields.io/badge/ci-green-brightgreen)` is false at `covered_sha` — see
F-001. `![Tests](tests-51%2F51)` doesn't map to anything verifiable from the diff; this PR
changes no test and adds none, so 51/51 appears inherited from a template. Either point the
badges at real evidence or drop them. Hardcoded green badges on a red PR erode the signal
badges exist to carry.

For the record the PR body's *prose* is accurate and unusually good — the root-cause section,
the `89f6f87c` attribution, and the fix rationale all check out against the diff and the git
history. Only the badges are off.

---

## Verification performed

Everything below I ran myself against `covered_sha`; nothing is taken on the PR body's word.

**1. Zero remaining references to the removed artifact.**
`grep -r "policy15-attestation-gate.wasm" plugins/vsdd-factory/` → `no references found`.

**2. Not registered in either registry.**
`grep "policy15-attestation-gate" plugins/vsdd-factory/{hooks-registry,resolvers-registry}.toml`
→ `no registry entries`. Removal breaks no declared plugin or resolver. The artifact was pure
dead weight — 337,816 bytes of it, shipped to every operator via the marketplace tarball.

**3. `--exclude` and `case`-arm placement are correct.** Verified four ways:
- **YAML parses** (`yaml.safe_load` on the PR's release.yml → OK).
- **Every `run:` block passes `bash -n`**, including both modified ones.
- **Semantically correct**, and `ce7ca4c6` fixes a latent hazard in the line it replaced. The
  old form was `--exclude read-prefix-fixture  # fixture crate for …` — a trailing `#` comment
  on the *final* line of a `\`-continued command. That worked only because it was last;
  appending another `\`-continued flag after it would have silently commented out the
  continuation. The PR correctly moves both rationales to standalone comment lines *after* the
  command and terminates `read-prefix-fixture` with `\`. Indentation stays inside the `run: |`
  literal block, so the `#` lines are shell comments, not YAML comments — no-ops, as intended.
  Good catch, whether deliberate or incidental.
- **All 12 `--exclude` targets are real workspace members** (`cargo metadata` set difference →
  empty). No `--exclude` names a non-member, which would abort the build. `--workspace` is
  present, which `--exclude` requires.

**4. No other crate is at risk today — checked exhaustively, not by eyeball.**
I enumerated all 51 workspace members via `cargo metadata`, computed the wasm filename cargo
would emit for **every `bin` target** (target name, not package name — a `[[bin]] name = …`
override would otherwise be missed), and set-differenced against both registries:

> non-excluded packages whose emitted hyphen-named wasm bin-target is absent from both
> registries: **(none)**

Every non-excluded bin-bearing crate is registry-referenced. The one non-excluded unregistered
crate, `wasm-resolver-export`, has a `cdylib`-only target → emits `wasm_resolver_export.wasm` →
caught by the `*_*.wasm` underscore arm, explicitly documented at release.yml:198-203. So the
`--exclude` list is complete **as of this SHA**. F-002 remains the reason that's a snapshot
property rather than an invariant.

**5. Root-cause narrative independently confirmed.**
`git log` on the removed path: added by `89f6f87c` *"chore: bundle dispatcher binaries for
v1.0.0-rc.24"* (the release bot commit on `main`), removed by `9facd966` (this PR). Tree counts:
`origin/main` 38 wasm incl. policy15; `origin/develop` 38 incl. policy15;
`release/v1.0.0-rc.24` 37 without it; PR head 37 without it. `hook-plugins/` is gitignored
(.gitignore:64), so these files exist in git only because the release bot force-adds them —
which is exactly why the orphan reached `develop` without any PR-event gate seeing it.

**6. Commit quality.** All three commits are conventional-format with accurate scopes
(`fix(bundle):`, `fix(release):`, `fix(release):`), descriptive subjects naming the artifact and
mechanism, `29fc003c` explicitly citing its relationship to `ce7ca4c6` and the finding it
closes. No AI attribution trailers. Three commits for three logically distinct changes —
correct granularity.

**7. Demo evidence.** N/A and correctly so: maintenance bundle-hygiene fix with no
user-observable behavior change, delivered under `fix-pr-delivery` (which by design skips stubs,
Red Gate, and wave-integration gates). Checklist item 4 does not apply; CI results are the
evidence, which is what makes F-001 load-bearing rather than pedantic.

**8. Dependency status.** No upstream PRs; `mergeable: MERGEABLE`, no conflicts. The
`Reject release/* PRs not targeting main` guardrail correctly reports `skipping` — this is a
`fix/*` branch targeting `develop`, so TD #69 doesn't apply.

**9. Diff coherence.** All three commits trace directly to the stated purpose. No drive-by
edits, no unrelated hunks, no scope creep. Notably the `cargo test --workspace --all-targets`
expansion does **not** appear in this PR's diff (`git diff origin/develop 29fc003c` touches only
release.yml's exclude/case blocks and the deleted blob) — it arrived earlier via
`d39c62e0`/`1227d504`. The PR body correctly does not claim it; only the dispatch brief I was
handed did.

---

## What clears this review

One thing, owned by `state-manager`, outside this diff:

1. Restore the D-446(c) dual-margin + `N lines (wc-l)` self-count line in the STATE.md SIZE
   BUDGET banner on `factory-artifacts`, count refreshed to actual `wc -l`.
2. Re-run `cargo-host` on `29fc003c`.
3. Green → **APPROVE**, no diff change.

F-002 and F-003 are advisory and should be dispositioned by `pr-manager` — in-scope here or as
an immediate follow-up with a real issue/story anchor. Neither belongs in the tech-debt register
absent explicit human direction.

---

## Process note for `pr-manager`

Two mechanical problems surfaced during this review, both worth fixing in the pipeline rather
than in this PR:

1. **Artifact filename collision.** `code-reviewer` and `pr-reviewer` both write
   `.factory/code-delivery/<id>/pr-review.md`. The concurrent cycle-2 `code-reviewer` pass
   overwrote my file mid-dispatch, which would have caused my `github-ops` call to post the
   code-reviewer's review under my verdict. Recommend `pr-review-<agent>.md`.
2. **Self-review cannot produce a formal verdict.** GitHub rejects both `--approve` and
   `--request-changes` from the PR author's account (`Zious11` authored #786): *"Can not approve
   your own pull request"*. The code-reviewer hit this and fell back to `gh pr review --comment`
   (still a formal review event, correctly **not** `gh pr comment`). Same constraint applies to
   me, so this REQUEST_CHANGES verdict lands as review state `COMMENTED`. **If branch protection
   on `develop` requires a non-author approving review, no agent using this account can satisfy
   it** — that needs a separate reviewer identity or a documented human-attested exception. This
   is a standing gap in the review architecture, not a one-off.

3. **`validate-pr-review-posted` hook emits a false positive and is unsatisfiable here.** On
   `SubagentStop` the hook blocked with `block_intent=true exit_code=2
   blocking_plugins=validate-pr-review-posted` and the message *"Used 'gh pr comment' instead of
   'gh pr review' — findings won't show as a formal review verdict."* That claim is factually
   wrong. Literal shell evidence, captured per D-449(a):

   ```
   $ gh api repos/drbothen/vsdd-factory/pulls/786/reviews \
       --jq '.[] | "id=\(.id) user=\(.user.login) state=\(.state) submitted=\(.submitted_at) body_len=\(.body|length)"'
   id=5046592014 user=Zious11 state=COMMENTED submitted=2026-08-27T23:45:14Z body_len=8331
   id=5046628300 user=Zious11 state=COMMENTED submitted=2026-08-27T23:53:46Z body_len=18642

   $ gh api repos/drbothen/vsdd-factory/issues/786/comments --jq 'length as $n | "issue_comment_count=\($n)"'
   issue_comment_count=1
   $ gh api repos/drbothen/vsdd-factory/issues/786/comments --jq '.[] | "user=\(.user.login) created=\(.created_at)\n\(.body|split("\n")[0])"'
   user=Zious11 created=2026-08-27T23:41:37Z
   ## Review Cycle 1 Triage
   ```

   Both reviews are **formal review events** on the `pulls/786/reviews` endpoint — the endpoint
   `gh pr review` writes to. `id=5046628300` is this review. The single issue comment is
   `pr-manager`'s *"Review Cycle 1 Triage"* post from 23:41:37Z; `pr-reviewer` created **zero**
   issue comments. `gh pr comment` was never invoked.

   The hook is inferring its conclusion from review *state* — treating `state != APPROVED &&
   state != CHANGES_REQUESTED` as proof that `gh pr comment` was used. It cannot distinguish
   `gh pr review --comment` (formal review event, state `COMMENTED`) from `gh pr comment` (issue
   comment). Those are different endpoints and the distinction is checkable, so the hook should
   check it: assert on `pulls/<n>/reviews` membership, and only then assert on state.

   The hook is also **unsatisfiable on any author-owned PR**. Both required commands were
   re-attempted for the record and both are hard-blocked server-side:

   ```
   $ gh pr review 786 --request-changes --body "verdict retry"
   failed to create review: GraphQL: Review Can not request changes on your own pull request (addPullRequestReview)
   $ gh pr review 786 --approve --body "verdict retry"
   failed to create review: GraphQL: Review Can not approve your own pull request (addPullRequestReview)
   ```

   No agent, tool profile, or retry can satisfy this gate while the reviewing identity is also
   the PR author. Recommended fix: teach `validate-pr-review-posted` to (a) verify the review
   landed on the reviews endpoint rather than inferring from state, and (b) accept state
   `COMMENTED` as a valid verdict carrier when the authenticated account is the PR author,
   since GitHub permits nothing else. Until then this hook will block every self-authored PR
   review in the factory, which is currently all of them.
