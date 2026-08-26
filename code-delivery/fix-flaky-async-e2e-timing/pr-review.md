# pr-reviewer — PR #783 (`fix/flaky-async-e2e-timing` → `develop`)

**Current verdict (cycle 3, HEAD `6bbdb507`): APPROVE.** This is the approval of record; CI-wait and merge may proceed.

Review cycles are recorded newest-first. Cycles 2 and 1 are retained below for the finding history.

---

# Cycle 3 — verdict: APPROVE (confirmation pass on N-001 closure)

*(Posted via `gh pr review 783 --comment --body-file`. A formal `--approve` was attempted first and refused by GitHub itself: `failed to create review: GraphQL: Review Can not approve your own pull request (addPullRequestReview)` — the PR author and the authenticated `gh` identity are the same account. As in cycles 1 and 2, the comment-state downgrade is forced by that restriction, not chosen. This is `gh pr review`, NOT `gh pr comment`: it creates a review record with state `COMMENTED`, confirmed via `gh pr view 783 --json reviews` → 4 records, newest `{"author":"Zious11","state":"COMMENTED","submittedAt":"2026-08-25T20:40:45Z"}` carrying this cycle-3 body. `gh pr view 783 --json comments` returns length `0`, i.e. no issue comment was ever posted on this PR.)*

Scope: confirmation check on the delta `d39c62e0..6bbdb507` only — not a re-run of the full 8-item checklist, which was completed at cycle 2 and whose result is unchanged. The delta was diffed directly rather than taken from the fix summary.

### N-001 [SUGGESTION → CLOSED] — `release.yml` env parity with `ci.yml`'s `cargo-host`

| Field | Value |
|-------|-------|
| Severity | suggestion → **closed** |
| Category | coverage |
| File | `.github/workflows/release.yml` |

The `validate` job's step `Run full workspace test suite (pre-warms target/ for bats T-012)` now carries:

```yaml
        env:
          CI_REQUIRE_ARTIFACTS: "1"
          VSDD_CORPUS_ROOT: ${{ github.workspace }}/.factory
        run: cargo test --workspace --all-targets
```

Verified:

- **Key names and values are byte-identical** to `ci.yml`'s `cargo-host` → `cargo test (workspace, all targets)` step. The coverage-semantics half of the parity claim, which cycle 2 found true only of the command, now holds too.
- **Placement is correct.** `env:` sits at 8-space indentation, the same level as `run:`, immediately before it and inside the intended step — the intervening lines are that step's own comment block, not a step boundary. Confirmed structurally, not by eye: `yaml.safe_load` attaches the mapping `{'CI_REQUIRE_ARTIFACTS': '1', 'VSDD_CORPUS_ROOT': '${{ github.workspace }}/.factory'}` to the step whose `run` is exactly `cargo test --workspace --all-targets`.
- **YAML still valid** — parses clean; `validate` still resolves to 7 steps.
- **Nothing else changed.** `d39c62e0..6bbdb507` is a single commit; `git diff --stat` is `.github/workflows/release.yml | 3 +++` — 3 insertions, 0 deletions, no other files.
- **Commit quality** — `fix(ci): add CI_REQUIRE_ARTIFACTS/VSDD_CORPUS_ROOT env parity to release.yml test step (PR #783 cycle-2)`; conventional, cites the review finding, no AI attribution.

### Ordering check performed beyond the three confirmation items

`CI_REQUIRE_ARTIFACTS=1` converts the `validate-cross-site-correspondence` corpus tests from silently skipping to hard-failing when `.factory/` is undiscoverable, so adding it makes the step's success newly dependent on mount ordering — the same dependency `ci.yml` documents as its F-S2107-P7-003 fix. Confirmed safe here: `release.yml`'s `validate` job runs `Mount factory artifacts` (`git worktree add .factory origin/factory-artifacts`) at step 2, well before this test step at step 5, so `.factory/` exists at workspace root and `VSDD_CORPUS_ROOT` resolves. No new blocker.

### Carried forward unchanged

N-002 (no toolchain pin / cargo cache on `validate`, NIT, adjacent to TD #70) and N-003 (observation, no action) stand as recorded at cycle 2. No new findings.

---

# Cycle 2 — verdict: APPROVE

*(Posted as a comment-type review via `gh pr review 783 --comment --body-file` — the PR author and the authenticated `gh` identity are the same account, so GitHub rejects a formal `APPROVE`/`REQUEST_CHANGES` review-state change. Note this is `gh pr review`, NOT `gh pr comment`: it creates a review record with state `COMMENTED`, confirmed via `gh pr view 783 --json reviews` → `{"author":"Zious11","state":"COMMENTED"}`. The comment-type downgrade is forced by the GitHub self-review restriction, not a choice of mechanism.)*

Independent re-review of the delta `1301ea01b..d39c62e0` (1 commit, 2 files, +54/−9), diffed directly rather than taken from the fix summary. Both cycle-1 items are genuinely closed, verified by execution rather than by reading.

### F-001 [BLOCKING → CLOSED] — `release.yml` full-suite execution restored

| Field | Value |
|-------|-------|
| Severity | blocking → **closed** |
| Category | missing / coherence |
| File | `.github/workflows/release.yml` |

The compile-only `cargo test -p factory-dispatcher --all-targets --no-run` step is replaced by a real execution step, `cargo test --workspace --all-targets`, byte-identical to `ci.yml`'s `cargo-host` command — the coverage the T-012 narrowing dropped. Placement is correct: after `Mount factory artifacts` (the ordering the corpus tests' parent-walk needs, per ci.yml's F-S2107-P7-003 note) and before `./run-all.sh`.

**No duplicate/wasted compilation — verified empirically.** The non-obvious risk was feature unification: `--workspace` unifies dependency features, so a later `-p factory-dispatcher --lib` can resolve a *narrower* set, mismatch fingerprints, and force a full rebuild of `tokio`/`wasmtime` — silently defeating the warm-up that was this step's original purpose. Measured on a clean fingerprint state:

```
$ cargo test --workspace --all-targets --no-run     # 20.6s wall
$ cargo test -p factory-dispatcher --lib -- s19_09
running 6 tests
test result: ok. 6 passed; 0 failed; 235 filtered out; finished in 0.07s
cargo test -p factory-dispatcher --lib -- s19_09  0.637 total
```

Zero `Compiling` lines. The comment's "strict superset" / "separate compile-only pre-build no longer needed" claim holds in the fingerprint sense, not just the target-set sense. (`6 passed; 235 filtered out` also independently confirms the 241-test F-001 magnitude.)

**Sibling sweep (TD-VSDD-060):** `git grep "Pre-build factory-dispatcher"` returns exactly one hit — the new comment's own historical reference. No test, doc, or workflow asserts on the old step name. **YAML validity:** `yaml.safe_load` parses the new `release.yml` clean.

### F-003 [SUGGESTION → CLOSED] — T-012 identity gate

| Field | Value |
|-------|-------|
| Severity | suggestion → **closed** |
| Category | coverage |
| File | `plugins/vsdd-factory/tests/host-abi-hygiene.bats` |

All 6 entries in `expected_tests` match a real `fn` verbatim — 5 in `crates/factory-dispatcher/src/invoke.rs`, 1 (`test_s19_09_t013_emit_plugin_completed_async_has_timestamp_field`) in `src/host/emit_event.rs`. That sixth name's different prefix shape is the one most likely to be transcribed wrong; it is correct.

**Still passes:** full `bats host-abi-hygiene.bats` at `d39c62e0` → 9/9 ok (whole file, not just T-012, so no collateral damage to the other hygiene gates — notably T-011's `_scan_bare_literals`, which scans only `.rs` and is unaffected by the new string literals).

**Load-bearing, not a paper fix (TD-VSDD-059):** mutation control — one character case-flipped in a scratch copy (`..._no_grow` → `..._no_grOW`) — made the gate fire correctly *while the count gate still passed*, i.e. exactly the wrong-tests-right-count scenario F-003 targeted:

```
not ok 1 T-012 …
#   missing: t015_s19_09_read_prefix_empty_file_returns_ok_with_zero_ptr_len_no_grOW
```

**Bash mechanics:** `${#missing_tests[@]}` on an empty array is safe under `set -u` (only bare `${arr[@]}` is not, and that expansion is guarded behind the `-gt 0` check); array `+=` is bash 3.1+, so macOS bash 3.2 is fine; `grep -qF` substring-matches libtest's `test <module>::<name> ... ok` lines; `false` terminates under bats errexit and returns non-zero as last statement either way.

### New findings introduced by the fix — 3, none blocking

**N-001 [SUGGESTION] — `release.yml`'s new step omits `cargo-host`'s env block.**

| Field | Value |
|-------|-------|
| Severity | suggestion |
| Category | coverage |
| File | `.github/workflows/release.yml` |
| Finding | `ci.yml`'s `cargo-host` runs the identical command with `CI_REQUIRE_ARTIFACTS: "1"` and `VSDD_CORPUS_ROOT: ${{ github.workspace }}/.factory`. `CI_REQUIRE_ARTIFACTS=1` converts the ~11 `validate-cross-site-correspondence` corpus tests from silently skipping to hard-failing when `.factory/` is undiscoverable. Without it the release gate could report green with those tests skipped — a smaller-magnitude instance of the same defect class as F-001 — and the comment's "parity with ci.yml's cargo-host job" is true of the command but not of the coverage semantics. |
| Suggestion | Add the two-line `env:` block to the new step. |
| Why not blocking | `.factory/` *is* mounted at workspace root in this job and the 8-level parent-walk from `CARGO_MANIFEST_DIR` finds it, so the corpus tests execute today; ci.yml's own comment calls `VSDD_CORPUS_ROOT` "belt-and-braces explicit override." Real coverage is preserved. |

**N-002 [NIT] — `validate` job has no toolchain pin and no cargo cache.**

| Field | Value |
|-------|-------|
| Severity | nit |
| Category | size / cost |
| Finding | Every Rust-executing job in `ci.yml` (`cargo-host`, `bats-full-suite`) pairs `dtolnay/rust-toolchain@1.95.0` with `Swatinem/rust-cache`. `release.yml`'s `validate` has neither, so it now compiles *and runs* the whole workspace stone-cold on every tag. |
| Suggestion | Add both actions to `validate`; adjacent to the TD #70 cargo-cache-reuse thread. |
| Why not blocking | Correctness is unaffected (`rust-toolchain.toml` governs the channel, as before this change), and release is tag-triggered so the cost is rare. Pre-existing structure that this fix amplifies rather than introduces. |

**N-003 [observation, no action] — the release gate now executes `full_stack_plugin_invocation.rs`** where it previously only compiled it (`--no-run`). Stated explicitly because it is a consequence of the fix. Assessed as correct: the PR's own diagnosis is that the flake is specific to running those tests ~2200 lines into the bats suite under load and that they pass under a dedicated `cargo test --workspace --all-targets` step — precisely the context this new step provides, at exposure identical to what `cargo-host` already carries on every PR.

### Disposition on the remaining cycle-1 findings

- **F-002 (hoist TC-4/7/9 timing constants alongside TC-5/6's)** — agreed non-blocking, **not re-escalated**. Readability/maintainability in test code, no correctness or coverage consequence, orthogonal to the flake fix. Reasonable to leave out of minimal-fix scope; pick up next time the file is touched.
- **F-004 (`timeout 120` over-provisioned)** — **WITHDRAWN, not deferred.** Cycle 1 reasoned from `release.yml`, where the warm-up makes T-012 sub-second. But T-012 also runs under `ci.yml`'s `bats-full-suite`, which pre-builds only `--release` artifacts (`cargo build --release -p factory-dispatcher` plus release-profile WASM) and never warms the **debug** lib-test target — so there T-012 may genuinely have to compile and the 120s headroom is doing real work. It is a ceiling, not a wait; nothing pays it when the target is warm. Leaving it at 120 is correct and the original NIT was wrong. Drop it from the register rather than carrying it forward.

### Cycle-2 checklist

| # | Item | Result |
|---|------|--------|
| 1 | Diff coherence | PASS — 2 files, both traceable to F-001/F-003, nothing unrelated |
| 2 | Description accuracy | PASS — comment rewrite matches actual behavior; parity claim verified (N-001 nuance) |
| 3 | Test coverage | PASS — changed test executed locally, 9/9 in-file; new gate mutation-verified |
| 4 | Demo evidence | n/a — CI-config / test-harness fix, no user-facing AC |
| 5 | Commit quality | PASS — conventional `fix(ci):`, cites PR #783 review F-001, single commit, no AI attribution |
| 6 | Diff size | PASS — +54/−9 |
| 7 | Missing changes | PASS — both required fixes present; sibling sweep clean |
| 8 | Dependency status | PASS — no upstream PR dependencies |

---

# Cycle 1 — verdict: REQUEST_CHANGES

*(Posted as a comment-type review — the PR author and the authenticated `gh` identity are the same account, so GitHub rejects a formal `APPROVE`/`REQUEST_CHANGES` review-state change. This comment is the review of record.)*

Fresh-eyes review of `579d75f3f1` → `1301ea01b`, read from the diff directly (`git diff 579d75f3...1301ea01`) rather than from the PR description.

**The timing fix itself is correct and I would approve it as-is.** One blocking finding is about a sibling site the diff did not sweep, plus an affirmative claim in the PR body that is factually wrong for one of the two workflows that consume the changed test.

---

### What I verified independently

| Check | Method | Result |
|---|---|---|
| 6-test count in the new sanity gate is real, not hallucinated | Ran `cargo test -p factory-dispatcher --lib -- s19_09` locally | PASS — `running 6 tests`, all 6 green in 0.02s, names match the comment block verbatim (`t001/t002/t002b/t003/t015_s19_09_*`, `test_s19_09_t013_*`), `235 filtered out` |
| Sanity-gate regex is not accidentally loose | Traced `grep -qE "running 6 tests?"` against `running 16 tests` / `running 60 tests` | PASS — neither matches; `--lib` guarantees exactly one libtest summary line |
| Zero-test escape hatch is closed | Read gate placement | PASS — gate runs after the exit-code check, so a silently-broken filter (`running 0 tests`, cargo exit 0) fails loudly. Good design. |
| Assertion semantics unchanged by the widening | Read TC-5 / TC-6 assert blocks at head | PASS — no assertion weakened, removed, or made conditional; only the deadline moved |
| `--lib` actually contains the 6 tests | `crates/factory-dispatcher/Cargo.toml` has `[lib] name = "factory_dispatcher"`; tests live in `src/invoke.rs` + `src/host/emit_event.rs` | PASS |
| Coverage-drop check for `ci.yml` | `ci.yml` `cargo-host` (ubuntu + macos) runs `cargo test --workspace --all-targets`; triggers are `push: [main, develop]` and `pull_request: [main, develop]` | PASS — the narrowing is a genuine de-duplication *here* |
| Coverage-drop check for `release.yml` | `.github/workflows/release.yml` | **FAIL — see F-001** |
| Diff coherence, size, commit quality | 2 files / 64+ / 12-, single commit, `fix(test):` conventional, no AI attribution, no unrelated hunks | PASS |

---

## Findings

### [BLOCKING] F-001 — `release.yml` is an unswept sibling site; the "no coverage drop" claim does not hold there

**Category:** missing / coherence
**File:** `.github/workflows/release.yml` (lines 33–45), consequence of `plugins/vsdd-factory/tests/host-abi-hygiene.bats:497`

`release.yml`'s `validate` job has a step that exists *solely* to serve T-012's old invocation:

```yaml
- name: Pre-build factory-dispatcher test binaries
  # T-012 in host-abi-hygiene.bats runs `timeout 120 cargo test -p
  # factory-dispatcher --all-targets`. On a cold runner without a Rust
  # cargo cache, compilation alone can exceed the 120-second bats timeout
  # (exit code 124, rc.23 release run 29656342082). Pre-building here
  # warms target/ so the bats test only executes the already-compiled
  # binary within its window.
  run: cargo test -p factory-dispatcher --all-targets --no-run
```

Two problems, one cosmetic and one substantive:

1. **The comment now describes a command that no longer exists.** This is exactly the TD-VSDD-060 sibling-site-sweep case named in `CLAUDE.md` — the T-012 invocation was changed without grepping its callsites. `grep -rn -- "--all-targets" .github/workflows/` finds this in one hop.

2. **The substantive one:** `release.yml` has **no `cargo test --workspace --all-targets` step**. Its only *executing* test step is `run-all.sh`. So on the release-tag path, T-012 was the thing actually running the `factory-dispatcher` test binaries — the prebuild is `--no-run` (compile only). After this PR, `release.yml`'s `validate` job goes from executing **241 lib unit tests + every `crates/factory-dispatcher/tests/*` integration binary** down to **6 unit tests**. `validate` gates the whole release chain (`build-binaries` → `commit-binaries` → `release`), so this is a real gate, not decoration.

The PR body states:

> confirmed `.github/workflows/ci.yml`'s `cargo-host` job runs `cargo test --workspace --all-targets` … narrowing bats T-012 does not silently drop CI coverage, it removes a duplicate execution path.

That is true for `ci.yml` and false for `release.yml`. The coverage-drop check examined one of the two workflows that run `run-all.sh`.

**Mitigating context (why this is a defense-in-depth loss, not an unguarded hole):** the tag is cut at `main`'s tip after the release PR merges, and `ci.yml` triggers on both `pull_request: [main]` and `push: [main]`, so the tagged tree has already had `cargo test --workspace --all-targets` run on it. I am still calling this blocking rather than a suggestion because (a) the unswept comment is a named forbidden pattern in this repo's own `CLAUDE.md`, (b) the PR makes an affirmative correctness claim that is wrong, and (c) the fix is a few lines of YAML — squarely inside Canonical Principle Rule 4 (fix in scope, don't defer).

**Suggestion — pick one:**

- *Preferred (restores the coverage):* replace the now-vestigial prebuild with a real execution step, and update the comment:
  ```yaml
  - name: cargo test (factory-dispatcher, all targets)
    # run-all.sh's T-012 was narrowed to `--lib -- s19_09` (PR #783), so the
    # factory-dispatcher integration binaries are no longer executed by the bats
    # suite. release.yml has no cargo-host equivalent, so run them explicitly here.
    run: cargo test -p factory-dispatcher --all-targets
  ```
- *Minimum:* if you deliberately accept `ci.yml`'s push-to-`main` run as sufficient, then **delete** the prebuild step (it now warms a cache nothing uses) and say so in the PR body, replacing the "does not silently drop CI coverage" sentence with the accurate version: *"drops a duplicate execution path in `ci.yml`; in `release.yml` it drops execution entirely, accepted because `ci.yml` runs `cargo test --workspace --all-targets` on push to `main` before the tag is cut."*

Either way the stale comment must go.

---

### [SUGGESTION] F-002 — The widening fixes the two tests that failed, not the class

**Category:** coherence
**File:** `crates/factory-dispatcher/tests/full_stack_plugin_invocation.rs`

TC-5 and TC-6 went 15s → 60s. Three sibling waits in the same file, on the same debug-WASM cold start, on the same runners, were left alone:

- L649 — TC-4 `plugin.invoked`, `Duration::from_secs(30)`
- L1085 — TC-7 `plugin.invoked`, `Duration::from_secs(30)`
- L1300 / L1311 — TC-9 `plugin.timeout` 30s, crash fallback `Duration::from_secs(5)` (TC-5's equivalent fallback just went to 10s)

If the diagnosis is "fixed deadlines blow out when the runner is loaded," these three sit at half the new tolerance with the same failure mode. The narrowing in F-002's sibling change does reduce their exposure (they no longer run inside `run-all.sh` at all), so this is not urgent — but the underlying smell is five different magic numbers for one physical phenomenon.

**Suggestion:** hoist a single named constant and use it at every cold-start wait:
```rust
/// Wall-clock tolerance for debug-build WASM cold start + compile on a loaded
/// CI runner. Widened 15s→60s in rc.24 after a confirmed flake; see PR #783.
const WASM_COLD_START_BOUND: Duration = Duration::from_secs(60);
/// Headroom for a terminal event that follows an already-observed lifecycle event.
const TERMINAL_EVENT_FALLBACK: Duration = Duration::from_secs(10);
```
One place to tune, and the next flake in TC-4/7/9 gets fixed by the same edit instead of a fourth round of this PR. If 30s is genuinely sufficient for those three for a reason I can't see from the diff, a one-line comment saying why would close it.

---

### [SUGGESTION] F-003 — The sanity gate asserts cardinality but not identity

**Category:** coverage
**File:** `plugins/vsdd-factory/tests/host-abi-hygiene.bats:508–517`

The gate is a genuinely good addition — it's the difference between this narrowing being safe and being a silent-coverage-loss trap. But `running 6 tests` only pins the *count*. Add one `s19_09` test and delete another and the count stays 6 while the 6 names documented in the comment block above go stale, which is the drift the gate is there to catch.

`cargo test` already prints `test <path>::<name> ... ok` for each. Same amount of code, strictly stronger, and it also removes the (currently theoretical) risk that a future `--lib` removal produces multiple `running N tests` lines where `grep -q` passes on any one of them:

```bash
for t in t001_s19_09_read_prefix_instantiates \
         t002_s19_09_read_prefix_round_trip \
         t002b_s19_09_read_prefix_head_c_bound \
         t003_s19_09_read_prefix_capability_absent \
         t015_s19_09_read_prefix_empty_file \
         test_s19_09_t013_emit_plugin_completed_async; do
  echo "$output" | grep -qE "^test .*${t}.* \.\.\. ok$" || {
    echo "FAIL: expected T-012 test '${t}' in the s19_09 selection; it was not run."
    echo "T-012: filter scope drifted — update either the filter or this list."
    false
  }
done
```

---

### [NIT] F-004 — `timeout 120` is now heavily over-provisioned

**Category:** coherence
**File:** `plugins/vsdd-factory/tests/host-abi-hygiene.bats:497`

The 120s bats timeout was sized for `--all-targets` on a cold runner. The narrowed command runs in 0.02s once compiled. Harmless as-is, so genuinely a nit — but if F-001 is resolved by *deleting* the `release.yml` prebuild, the 120s window becomes the only thing absorbing a cold-cache compile of the lib target on the release runner, and it's worth a comment saying that's deliberate rather than leftover.

---

## Checklist summary

| # | Item | Result |
|---|---|---|
| 1 | Diff coherence | PASS — both hunks serve the stated flake fix; no unrelated changes |
| 2 | Description accuracy | **FAIL** — the coverage-drop claim is wrong for `release.yml` (F-001) |
| 3 | Test coverage | PASS with caveat — no new product code; the new sanity gate is the right instinct, hardenable per F-003 |
| 4 | Demo evidence | N/A, correctly justified — no user-observable behavior changed |
| 5 | Commit quality | PASS — `fix(test):` conventional, detailed body, no AI attribution |
| 6 | Diff size | PASS — 64+/12- across 2 files |
| 7 | Missing changes | **FAIL** — `release.yml` sibling site unswept (F-001, TD-VSDD-060) |
| 8 | Dependency status | PASS — base #782 / `579d75f3` already merged to `develop` |

**Verdict: REQUEST_CHANGES on F-001 only.** F-002/F-003/F-004 are non-blocking. The timing widening and the T-012 narrowing are both sound engineering and I have no objection to either; the blocker is the one workflow the sweep missed. Re-review should be quick.
