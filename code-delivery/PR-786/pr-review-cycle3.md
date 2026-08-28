# PR Review — #786 `fix/rc24-orphan-wasm-policy15` → `develop` (cycle 3)

**Reviewer:** `vsdd-factory:pr-reviewer` (fresh-eyes, final pre-merge review)
**Verdict:** **REQUEST_CHANGES** — 1 BLOCKING, 2 SUGGESTION, 0 NIT
**covered_sha:** `344f48199c2b797c07624abf829f04570657c62c`
**Diff size:** 3 files, +11 / −1 text lines, plus one binary deletion. Well under the 500-line flag.
**Posting note:** GitHub rejected `gh pr review --request-changes` with `Review Can not request changes on your own pull request (addPullRequestReview)`. Posted as `gh pr review --comment` instead (still a formal review, not `gh pr comment`). The REQUEST_CHANGES verdict stands and is stated in the posted body.

Scope reviewed: full diff at head `344f4819`. All three changed files reviewed against the 8-item checklist. Commits `9facd966` and `ce7ca4c6` (approved in cycles 1-2) re-verified; the two new commits `29fc003c` and `344f4819` reviewed in depth.

---

## Summary of what I verified (not a rubber-stamp)

| Focus | Result |
|-------|--------|
| `release.yml` `--exclude` addition | Correct. YAML parses (`yaml.safe_load` OK, 6 jobs). Extracted `run:` block passes `bash -n`. The relocated trailing comment is valid block-scalar content. |
| `release.yml` defense-in-depth — both staging loops | Complete. `release.yml` has exactly two wasm staging loops (the `cp "$wasm" artifact/` loop in "Stage artifact directory" and the `cp "$wasm" plugins/vsdd-factory/hook-plugins/` loop in "Stage wasm plugins"); both received the `policy15-attestation-gate.wasm)` skip arm. No third staging site exists (grepped all `.wasm` copy/find sites). |
| Orphan removal (`9facd966`) | Correct. Zero registry references in `hooks-registry.toml` / `resolvers-registry.toml`. T-009 enumerates the **git-tracked** set via `git ls-files`, so removing the tracked file is the governing fix. |
| Snapshot fixture banner arithmetic | The **new** top banner is internally correct: file is 433 lines (`wc -l`), `500 - 433 = 67`, `500 - 415 = 85`. Format is byte-structurally identical to the live `.factory/STATE.md` banner. **But see BLOCKING-1** — it is not the only banner in the file. |
| Fixture change — behavioral effect | Empirically tested. Built `validate-state-structure.wasm` fresh from source and ran the real dispatcher against the fixture **with** and **without** the added banner. Output is byte-identical in both cases: exactly one violation (`D-434(e) sub-check 1` — `## Convergence Status` absent), which `pass-real-state-md-snapshot.bats` explicitly tolerates. Zero Phase-1 (banner / dual-margin / trajectory-tail) violations either way. |
| New regressions | None found in `release.yml` or the orphan removal. The ci.yml count-floor guards (`ls hook-plugins/*.wasm` vs `[[bin]]` crates under `crates/hook-plugins/`) are unaffected — `policy15-attestation-gate` lives outside `crates/hook-plugins/`, so the floor is unchanged and the comparison is `-lt`. |
| Commit quality | Conventional format, scoped, clear. Except the subject line of `344f4819` — see BLOCKING-1. |
| Diff size | 11 additions / 1 deletion + one binary removal. Well under the 500-line flag. |

---

## BLOCKING-1 — `344f4819` is a paper-fix (TD-VSDD-059) and introduces a stale banner claim

**File:** `plugins/vsdd-factory/tests/fixtures/validate-state-structure/pass-real-state-md-snapshot/factory/STATE.md`
**Category:** coherence / description
**Severity:** blocking

Three independent problems, all from the same commit.

### (a) The commit cannot fix the F-P2-002 bats failure — that test never reads this fixture

`pass-real-state-md-snapshot.bats` `setup()` does:

```bash
# F-P3-002: auto-copy LIVE STATE.md at run time — eliminates snapshot-vs-live drift class.
cp "$REPO_ROOT/.factory/STATE.md" "$WORK/.factory/STATE.md"
```

The fixture directory is referenced **nowhere** in the suite except a header comment that says so explicitly: *"The frozen fixture at fixtures/validate-state-structure/pass-real-state-md-snapshot/ is retained as a documentation reference for the pass-2 fix-burst baseline."* Every other fixture in that directory is wired via `FIXTURE_SRC=...`; this one is not. There is no generic pass-/fail- fixture driver in `run-all.sh`.

The failure at `29fc003c` was:

```
not ok 1 F-P2-002 PASS: real STATE.md Phase 1 passes — no banner/margin/tail false-positive block
#   `[[ "$output" != *"no SIZE BUDGET banner"* ]]' failed
```

`ci.yml`'s `bats-full-suite` job mounts `origin/factory-artifacts` at `.factory`, so the input to that assertion is the **live** `STATE.md` on the `factory-artifacts` branch. That branch's `STATE.md` had no banner when the run started; it was given one at 18:56 by `fa821eec` on `factory-artifacts` — *"fix(state): add SIZE BUDGET banner to STATE.md — unblock validate-state-structure CI (D-421(c)+D-446(c))"*. **That** commit is the fix. This PR's commit `344f4819` is causally unrelated to it.

Inertness confirmed empirically rather than by inspection alone: freshly-built validator, real dispatcher, fixture content from `344f4819` vs from `29fc003c` — identical single-violation output. The change is a no-op.

### (b) The fixture already had a compliant SIZE BUDGET banner; this adds a duplicate

At `29fc003c` the fixture already contained a banner block with a valid D-446(c) dual-margin form whose actual-count claim matched the file:

```
STATE.md SIZE BUDGET (per D-421(c) + D-422(c) reconciliation):
... Hard cap (500 lines) margin from soft-target = 500 - 415 = 85; margin from actual = 500 - 428 = 72 (D-446(c) dual-margin form).
```

The file now has **two** `STATE.md SIZE BUDGET` banners (line 2 and line 29).

### (c) The pre-existing banner's arithmetic is now wrong, and nothing catches it

The commit grew the file 428 → 433 lines but did not update the second banner, which still asserts `margin from actual = 500 - 428 = 72`. The file is 433 lines. This is a wc-off-by-five inside a banner block — the exact defect class the suite has a dedicated negative fixture for (`fail-banner-wc-off-by-one`).

It goes undetected only because the F-P5-003 banner-block-scoped scan validates the **first** banner block, which is the new one. So the file now presents, in a directory whose purpose is to be the canonical exemplar of correct banner form, a self-contradicting pair of banners — and the sibling site was not swept (TD-VSDD-060).

### Suggested fix (either is ~2 minutes)

**Preferred — revert the commit.** It fixes nothing, and reverting restores a fixture that was already banner-compliant and internally consistent:

```bash
git revert --no-edit 344f4819
```

**Alternative — if the top banner is wanted for live-STATE.md structural parity,** keep it and sweep the sibling so the file has one truth:

```
-  ... Hard cap (500 lines) margin from soft-target = 500 - 415 = 85; margin from actual = 500 - 428 = 72 (D-446(c) dual-margin form).
+  ... Hard cap (500 lines) margin from soft-target = 500 - 415 = 85; margin from actual = 500 - 433 = 67 (D-446(c) dual-margin form).
```

Either way, `344f4819`'s subject — `fix(tests): add SIZE BUDGET banner to pass-real-state-md-snapshot fixture` framed as the F-P2-002 CI fix — should not enter the permanent record asserting a causal link that does not exist. Squash-merge makes the PR body permanent too; see SUGGESTION-2.

---

## SUGGESTION-1 — `ci.yml` sibling parity: the "mirror" comments are now false

**File:** `.github/workflows/ci.yml`
**Category:** missing / coherence

`release.yml`'s staging loops carry the comment *"mirrors `*_*.wasm` outer case arm in release.yml 'Stage artifact directory' and 'Stage wasm plugins' steps"*, and `ci.yml`'s staging step says *"The release.yml 'Stage wasm plugins' step is the canonical staging pattern; this mirrors it for the CI (debug) path."* After this PR the mirror is broken: `ci.yml` has **three** workspace `wasm32-wasip1` builds and **three** staging loops (`cp "$wasm" plugins/vsdd-factory/hook-plugins/` in `cargo-host`, `cp "$wasm" artifact-staging/` in `build-dispatcher`, and `cp "$wasm" plugins/vsdd-factory/hook-plugins/` in `bats-full-suite`), and **none** of the six sites received the `policy15-attestation-gate` treatment that this PR gave `release.yml`.

Why this is not blocking: `plugins/vsdd-factory/hook-plugins/` is gitignored (`.gitignore:64`) and the 37 tracked wasm files got there via the release bot's force-add path (`BINARY_PATHS_REGEX='^plugins/vsdd-factory/(hook-plugins/.*\.wasm|...)$'`). T-009 is tracked-scoped (`git ls-files`), so an untracked CI-staged copy cannot fail it, and the count-floor guards use `-lt` so an extra file is harmless.

Why it is still worth fixing in this PR: the recurrence class this PR exists to close is "a wasm that shouldn't exist gets produced, then swept into `hook-plugins/`." `ci.yml` still produces it. A developer who follows the documented CI staging pattern locally ends up with the stray artifact in their tree, and the next person to read the "mirrors release.yml" comments will be misled. Adding `--exclude policy15-attestation-gate` to the three `ci.yml` workspace builds plus the matching case arm to the three loops is the same mechanical edit already made here, and it keeps the two workflows honest.

There is also no regression test pinning the new defense, whereas the analogous `read-prefix-fixture` defense has one: T-011 (*"POLICY 20 defense proof … proves `--exclude read-prefix-fixture` in release.yml is the governing defense"*). A parallel case in `bundle_orphan_check.rs` for `policy15-attestation-gate.wasm` would give the same standing protection. Worth considering, though the tracked-set assertion in T-009 already covers the committed-orphan outcome.

---

## SUGGESTION-2 — PR body does not describe commits 3 and 4

**Category:** description

The body describes a two-commit PR (`9facd966`, `ce7ca4c6`) and its "Root Cause & Fix Rationale", Traceability, and Rollback sections all enumerate exactly those two. Two commits have landed since:

- `29fc003c` (staging-loop case arms) is not in the narrative sections. The Security Review section's *"Test step expansion (commit 3, release.yml): Strengthens release gate. Env vars are static strings + workspace-scoped GitHub Actions expression"* does not describe this commit at all — there is no env-var or test-step change in the diff.
- `344f4819` (fixture banner) is absent entirely.

The Rollback block lists only two reverts, so following it as written would leave both new commits in place. Since the merge is a squash to `develop`, this body becomes the permanent commit message. Please refresh the narrative sections, the Traceability table, and the rollback command list to match the four-commit reality.

---

## Checklist disposition

| # | Item | Status |
|---|------|--------|
| 1 | Diff coherence — all changes relate to this fix | PASS for commits 1-3. Commit 4 is in-theme but non-functional (BLOCKING-1). |
| 2 | Description accuracy | FAIL — SUGGESTION-2. |
| 3 | Test coverage on changed lines | PASS with a gap. T-009 covers the orphan removal. The `release.yml` guards are CI-config, release-event-only; no T-011-style pin for the new defense (SUGGESTION-1). |
| 4 | Demo evidence | N/A accepted — binary removal + CI config, no observable runtime behavior. Functional evidence is T-009. |
| 5 | Commit quality | PASS except `344f4819`'s causal claim (BLOCKING-1). |
| 6 | Diff size | PASS — 11/-1 + 1 binary. |
| 7 | Missing changes | SUGGESTION-1 (ci.yml sibling sites). |
| 8 | Dependency status | PASS — no upstream PRs; `mergeable: MERGEABLE`. Note the real F-P2-002 unblock lives on `factory-artifacts` (`fa821eec`), outside this PR. |

## CI at time of review

`validate`, `platforms-drift`, `deny-advisories`, `attestation-gate-non-vacuity-controls`, `policy-15-attestation-location`, `bats-darwin-leg`, `bats-wave-handoff (macos)`, semgrep SAST: **success**. `bats-full-suite (linux)`, both `cargo-host` legs, and all five `build-dispatcher` legs were still in progress. `bats-full-suite (linux)` failed at both `ce7ca4c6` and `29fc003c` on F-P2-002; per BLOCKING-1 the input that changed between then and now is `origin/factory-artifacts`, not this PR, so a green result on this run should not be read as evidence that `344f4819` fixed anything.

Commits 1-3 are sound and I would approve them as they stand. The blocking finding is confined to `344f4819` and clears with a one-command revert.
