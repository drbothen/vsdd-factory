# Review Findings — PR #783 (`fix/flaky-async-e2e-timing`)

## Convergence Tracking

| Cycle | Findings | Blocking | Fixed | Remaining |
|-------|----------|----------|-------|-----------|
| 1 | 4 (F-001..F-004) | 1 (F-001) | 2 (F-001, F-003) | 0 blocking, 2 non-blocking left as-is (F-002, F-004) |
| 2 | 3 new (N-001..N-003) | 0 | 1 (N-001, folded in) | 0 blocking — **APPROVE**. N-002 tracked as follow-up (not this PR's scope); N-003 observation only; F-002 left as-is; F-004 withdrawn by reviewer |

## Cycle 1 — pr-reviewer verdict: REQUEST_CHANGES

Posted at https://github.com/drbothen/vsdd-factory/pull/783#pullrequestreview-5023625990
(comment-type review; self-approve/request-changes state blocked since the
`gh` identity matches the PR author).

Reviewed diff: `579d75f3f1` → `1301ea01b`.

### F-001 [BLOCKING] — `release.yml` unswept sibling site (TD-VSDD-060)

`release.yml`'s `validate` job had no independent `cargo test --workspace
--all-targets` execution step — its "Pre-build factory-dispatcher test
binaries" step ran `--no-run` (compile only), and the only step that
actually executed test assertions was `./run-all.sh` (bats), which drives
T-012. Narrowing T-012 from `--all-targets` to `--lib -- s19_09` therefore
silently dropped the release-tag validate-gate's effective coverage from
241 lib tests + all integration binaries down to 6 tests, with no
compensating execution path (unlike `ci.yml`, where the separate
`cargo-host` job provides that coverage independently). The PR's original
claim — "narrowing bats T-012 does not silently drop CI coverage" — was
true for `ci.yml` and false for `release.yml`.

**Disposition:** Routed to devops-engineer (CI/CD domain owner per
CLAUDE.md Agent Routing Table). Fixed in commit `d39c62e0797712db2183317b6475b23499b9470a`:
converted the "Pre-build" step from `--no-run` (compile-only) into a real
`cargo test --workspace --all-targets` execution step, restoring coverage
parity with `ci.yml`. `--all-targets` is a strict superset of `--lib --
s19_09`, so it still warms `target/` for T-012 as a side effect (no
duplicate compile). Comment rewritten to describe current T-012 behavior
and the rationale for the new independent step. Flake-safe: only possible
because commit 1 (this PR) already widened the async e2e timing ceilings to
60s, so restoring full execution does not reintroduce the original flake.

### F-002 [SUGGESTION] — non-blocking, left as-is

TC-4 (L649) and TC-7 (L1085) remain at 30s, TC-9 at 30s/5s — all on the same
debug-WASM cold-start class as the widened TC-5/TC-6. Suggests hoisting a
shared named constant (e.g. `WASM_COLD_START_BOUND`) instead of five magic
numbers. Not required — these tests were not observed to flake, and this
PR's mandate is fixing the confirmed flake, not a speculative preemptive
widening of tests with no failure history. Left as a follow-up opportunity,
not a tracked tech-debt entry (no human-directed deferral was requested;
simply out of this fix's minimal scope).

### F-003 [SUGGESTION, fixed] — sanity gate strengthened to assert identity

The new T-012 sanity gate originally asserted only a test *count* ("running
6 tests"), which would not catch an add/delete pair that kept the count at
6 while silently changing which tests run. **Disposition:** Fixed alongside
F-001 by devops-engineer in the same commit — added a per-test-name
assertion for all 6 documented `s19_09` test names, verified against the
actual `cargo test` output format before committing.

### F-004 [NIT] — non-blocking, left as-is

`timeout 120` in the bats T-012 gate is now over-provisioned for the
narrowed (~0.02s) test run. Cosmetic; skipped as trivial/non-blocking per
explicit scope instruction to devops-engineer.

## Cycle 2 — pr-reviewer verdict: APPROVE

Posted 2026-08-25T20:28:59Z (comment-type review; self-approve blocked).
Reviewed diff: `1301ea01b` → `d39c62e0` (2 files, +54/-9).

Both cycle-1 items independently re-verified with load-bearing evidence
(re-diffed directly, ran the changed tests locally including a mutation
control on T-012's identity gate, confirmed byte-identical command parity
with `ci.yml`, confirmed zero duplicate compilation empirically).

### N-001 [SUGGESTION, fixed] — env var parity

`release.yml`'s new `cargo test --workspace --all-targets` step omitted
`CI_REQUIRE_ARTIFACTS: "1"` and `VSDD_CORPUS_ROOT: ${{ github.workspace
}}/.factory`, which `ci.yml`'s equivalent `cargo-host` step sets. Without
`CI_REQUIRE_ARTIFACTS=1`, ~11 corpus tests in `validate-cross-site-correspondence`
would silently skip instead of hard-fail if `.factory/` became
undiscoverable — the same defect class as F-001, at smaller magnitude. Not
escalated to blocking by the reviewer (coverage is preserved today since
`.factory/` is mounted and discoverable), but folded in as cheap fail-loud
insurance. **Disposition:** routed to devops-engineer, fixed by adding the
matching `env:` block to the same step.

### N-002 [NIT] — follow-up, not this PR's scope

`release.yml`'s `validate` job lacks a toolchain pin + `Swatinem/rust-cache`
(unlike `ci.yml`'s Rust-executing jobs), so it now compiles+runs the full
workspace cold on every tag. Correctness unaffected; cost concern adjacent
to TD #70 (cargo-cache-reuse). Tracked as a follow-up, not folded into this
fix (separable infra/perf concern, human-directed deferral not required
since it's a legitimate scope boundary, not an issue this PR's diff caused
directly — pre-existing gap in `validate`'s job config, exposed rather than
introduced by this PR).

### N-003 [observation] — no action

Release gate now executes `full_stack_plugin_invocation.rs` for real
(previously `--no-run` compile-only). Reviewer confirmed this is correct
and desirable — exposure identical to what `cargo-host` already carries on
every PR, and it's precisely the coverage this PR exists to restore.

### F-002 and F-004 final disposition

F-002 (hoist timing constants): left as-is, reviewer agreed non-blocking.
F-004 (`timeout 120` over-provisioned): **withdrawn** by the reviewer in
cycle 2 — `ci.yml`'s `bats-full-suite` job never warms the debug lib-test
target the way `release.yml` now does, so the 120s ceiling does real work
there. Original NIT was incorrect; dropped from the register.

## Cycle 2 fold-in (N-001) — fixed, commit `6bbdb507`

devops-engineer added the exact `env:` block (`CI_REQUIRE_ARTIFACTS: "1"`,
`VSDD_CORPUS_ROOT: ${{ github.workspace }}/.factory`) to `release.yml`'s
test-execution step, matching `ci.yml`'s `cargo-host` step exactly. 3-line
diff, YAML validity confirmed, no divergence (plain fast-forward push).
New HEAD: `6bbdb507` (was `d39c62e0`).

Quick re-glance dispatched (not a full cycle-3 review) per the reviewer's
own disposition that this fold-in "cannot break anything currently green"
and only needs confirmation of correct placement/values.
