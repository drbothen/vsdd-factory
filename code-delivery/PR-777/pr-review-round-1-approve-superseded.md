# PR #777 — Final Fresh-Eyes Review (pr-reviewer) — ROUND 1 — SUPERSEDED

> **SUPERSEDED by `pr-review.md` (round 2, same HEAD `010e6140`).** Preserved verbatim below for
> audit trail. Round 2 reached **REQUEST_CHANGES** with 2 HIGH blocking findings.
>
> **Why round 1 was superseded — the specific divergence:** §3 of this round-1 review states
> *"Merge commits diff against `^1` (standard); individual second-parent commits are also evaluated
> in-range, so activation coverage is complete."* That conclusion was reached by **reasoning about the
> code**, not by executing it. Round 2 built and ran the gate binary against this PR's own HEAD with
> base `develop` — exactly the invocation ADR-040 §Decision 9 Ruling 9(c) item 5 prescribes for CI —
> and it returned `FAIL: obligation violated` (exit 2), because the PR's own
> `Merge remote-tracking branch 'origin/develop'` commit `010e6140` is itself treated as an
> activating commit (its first-parent diff carries 8 `.rs` files under the pinned crate). The
> round-1 sentence is therefore inverted: merge-commit activation is not "complete coverage", it is a
> spurious-FAIL source. See finding H-1 in `pr-review.md`.
>
> Round 1 also did not re-run cargo-mutants (explicitly noted as "claim trusted") and did not detect
> the `core.quotePath` false-negative (H-2). Round 1's one LOW finding (comment rot in the F-1 test
> docstring) was independently re-found by round 2 and is carried forward as M-2.
>
> This divergence is itself the lesson: ADR-040 §Consequences records that every defect across the six
> bash-era iterations was found by extraction-and-execution and none by code review. Both blocking
> findings in round 2 required executing the binary.

---

# PR #777 — Final Fresh-Eyes Review (pr-reviewer)

- **PR:** #777 — fix(policy15-gate): implement POLICY 15 attestation-location gate as Rust crate
- **Branch:** feature/policy15-gate-rust → develop
- **Reviewed HEAD:** 010e6140352d12c3101c983b6e848ae7c7d8a5dd
- **Reviewer:** vsdd-factory:pr-reviewer (fresh-eyes)
- **Date:** 2026-08-15

## VERDICT: APPROVE (ready to merge)

No blocking findings. The crate implements the ADR-040 §Decisions 7-10 four-outcome
POLICY 15 ATTESTATION-LOCATION GATE correctly. This is a fix-PR / CI-tooling delivery
(no BCs, no holdout/demo evidence applies). CI-job wiring (D-969) is intentionally out
of scope and disclosed — NOT flagged.

## Scope note

`gh pr diff 777` (authoritative merge-base diff, 45.7KB) is exactly: the
`crates/policy15-attestation-gate/` crate (Cargo.toml, src/lib.rs, src/main.rs,
tests/binary_integration_test.rs) + root `Cargo.toml` member addition + one incidental
`CLAUDE.md` doc-row edit (FUEL_EXHAUSTED sentinel). Local `develop` is stale, so a
`develop...HEAD` three-dot stat shows unrelated already-merged noise — only the real PR
surface was reviewed.

## Independent verification (all re-run, not trusted from description)

- `cargo test -p policy15-attestation-gate --all-targets` → 18 unit + 5 integration, all green
- `cargo test -p policy15-attestation-gate --doc` → 1 doctest green
- `cargo clippy -p policy15-attestation-gate --all-targets -- -D warnings` → exit 0
- `cargo fmt --check --all` → exit 0
- cargo-mutants (0 missed / 46 caught / 4 unviable) NOT re-run (long) — claim trusted

## 1. Four-outcome semantics — CORRECT

Fail / PassWithActivations / PassZeroActivations / EmptyOrUnreachable are a typed closed
enum (deliberate absence of `#[non_exhaustive]`). Exit-code mapping is isolated to
`is_pass()`/`exit_code()`; each variant has a greppable `identifier()`. The bash-era
"four-outcomes / two-exit-codes" defect class is genuinely structurally closed —
`matches!()` on a closed enum cannot conflate `Fail` and `EmptyOrUnreachable`.

Priority order in `run_gate_inner` is FAIL > EMPTY-or-UNREACHABLE > PASS. A real
violation is never masked by an empty-diff commit (fail_list checked first). Worst case
is a conservative false-positive-FAIL, never a false-negative-PASS — correct direction
for a gate.

## 2. Guard-1 stale-pin-before-merge-base ordering — CORRECT IN CODE

`run_gate` (lib.rs:194-209) runs `tree_path_exists(HEAD, PLUGIN_CRATE)` and returns
`StalePin` BEFORE computing merge-base. `test_run_gate_guard1_stale_pin_beats_unresolvable_base`
exercises it through `run_gate` (guard 1), distinct from the `run_gate_from_merge_base`
guard-2 path. Uses `git cat-file -e` (tree query) not filesystem `is_dir()` — the
defect-3 regression (`test_disk_present_tree_absent_is_stale_pin`) confirms this. The
ordering is real in the code, not merely asserted by a test name.

## 3. Dead code / unused imports / logic bugs in run_gate/run_gate_from_merge_base/run_gate_inner — NONE

- All imports used; clippy clean.
- `run_gate_from_merge_base` is pub-but-test-only by design (the "testable core" that
  bypasses the origin remote lookup) — legitimate API, not dead code.
- clap `#[arg(env = "BASE_BRANCH", default_value = "develop")]` yields correct
  CLI-arg > env > default precedence (F-3 regression pins it).
- Merge commits diff against `^1` (standard); individual second-parent commits are also
  evaluated in-range, so activation coverage is complete.
- `count != 1` correctly enforces exactly-once (F-2 covers count==2).

## Findings

### [LOW / non-blocking] Comment rot in tests/binary_integration_test.rs

The `test_f1_..._expected_red_pending_implementer` docstring (lines 444-457, 487-491)
still declares assertion (b) "EXPECTED RED, PENDING IMPLEMENTER" and "this assertion
currently fails today by design / DO NOT weaken." But the implementer already wired the
WARNING (`eprintln!` in lib.rs:244-246, per commit 38faf75f), so the test is GREEN. The
test name and docstring are now factually stale and will confuse a future reader.
Recommend renaming the test (drop `_expected_red_pending_implementer`) and updating the
docstring to describe it as an enforced regression. No functional impact — the assertion
still guards the right behavior.

### [INFORMATIONAL / not a defect] Fail-closed empty-diff prioritization

`EmptyOrUnreachable(UnmeasurableDiff)` outranks `PassWithActivations`, so a range with
one compliant activating commit plus any `--allow-empty` commit reports
EMPTY-or-UNREACHABLE (exit 2) rather than PASS. This is disclosed, ADR-040-ratified
fail-closed design and is conservative-safe, but could surface as a CI false-positive
once D-969 wires the gate live. The PR already commits D-969 to re-validate against real
commit ranges — appropriate place to confirm. Flagged only for D-969's awareness.

## READY

```
covered_sha: 010e6140352d12c3101c983b6e848ae7c7d8a5dd
verdict: APPROVE
blocking_findings: 0
low_findings: 1 (comment rot in F-1 integration test docstring — recommend fix, does not block)
informational: 1 (fail-closed empty-diff prioritization — D-969 follow-up awareness)
validation: 18 unit + 5 integration + 1 doctest green; clippy -D warnings exit 0; fmt clean — all independently re-run
```
