# fix: rc.24 CI-green prep (test/CI/dependency-only)

**Type:** Fix PR (release-prep, no product logic changed)
**Target release:** v1.0.0-rc.24
**Branch:** `fix/rc24-ci-green` → `develop`

Makes `develop` CI-green ahead of cutting v1.0.0-rc.24. All three commits are
test-harness, CI-gate, or dependency-lockfile fixes — no application/dispatcher
logic changed.

---

## What Changed

### 1. `test(ci): stop merging cargo metadata stderr into jq in wasmtime version gate`
Fixes an AC-003 false-fail in
`plugins/vsdd-factory/tests/s21-12-version-and-deny-gate.bats`: the wasmtime
version-gate test was merging `cargo metadata`'s stderr into the stdout stream
consumed by `jq`, so any stderr noise (warnings, progress output) corrupted the
JSON payload and caused spurious gate failures unrelated to the actual
wasmtime version check.

**Behavior change disclosed (per review cycle 1 feedback):** beyond the
stderr/stdout separation, this commit also converts a hard-fail into a
`skip` when `cargo metadata --locked` itself fails or returns no output (an
environment-gap case, e.g. a missing/broken toolchain in the CI runner) —
this is a deliberate loosening of the gate's failure mode for that specific
environment-gap condition, not just a stream-corruption fix. Review cycle 1
found the `exit != 0` arm of that skip guard was unreachable under bats'
`errexit` semantics; fixed in cycle 2 (see Review Convergence below) by
wrapping the capture in `set +e`/`set -e`, matching the existing AC-004
pattern in the same file.

### 2. `test(ci): treat superseded stories as terminal in sprint-state PC4 gate`
Extends the PC4 completeness/fidelity exclusion in
`plugins/vsdd-factory/tests/sprint-state-format.bats` to also treat
`**SUPERSEDED**` stories as terminal — the same class of exclusion already
applied to `retired` stories. Without this, superseded stories were incorrectly
counted against PC4 completeness/fidelity, producing false-fail gate blocks.

### 3. `fix(security): bump h2 0.4.14 → 0.4.19 — clear RUSTSEC-2026-0258 DoS`
Cargo.lock-only bump of `h2` to clear **RUSTSEC-2026-0258** (unbounded empty
DATA frames DoS), reachable in the production `factory-dispatcher` binary via
the sink crates' `reqwest`/`tonic` → `hyper` → `h2` chain. A peer could queue
empty DATA frames without limit, driving unbounded memory growth on streams
that are not actively drained. Fixed upstream in h2 0.4.16+; this bump was
caught by the `cargo deny check advisories` CI gate (added in #781) failing
clean against 0.4.14 during `release.yml`'s fresh dependency resolution.
`cargo update -p h2 --precise 0.4.19` — no `Cargo.toml` edit required. CHANGELOG
`[Unreleased]` Security entry added.

**Incidental, benign:** `windows-sys` 0.52 → 0.61.2 unification also lands in
`Cargo.lock` as a side effect of the h2 bump — it was already present via
tokio's `^0.61` requirement elsewhere in the tree; pinning it back was out of
range and unnecessary.

---

## Architecture Changes

N/A — no architecture/subsystem changes. This PR touches only test-harness
`.bats` files and `Cargo.lock` (transitive dependency pin). No `crates/`
production source, no `ARCH-INDEX.md` subsystem, no ADR is affected.

## Story Dependencies

N/A — this is not a story PR (no `STORY-NNN` behind it); it is release-prep
CI-hygiene + security-dependency work with no `depends_on` entry in
`STORY-INDEX.md`. No upstream/downstream story PRs gate this merge.

## Spec Traceability

N/A — no BC/AC/VP is being implemented or amended by this PR. Traceability
for the underlying defects is the CI gates themselves:
- Commit 1 traces to `plugins/vsdd-factory/tests/s21-12-version-and-deny-gate.bats`
  AC-003 (wasmtime version gate).
- Commit 2 traces to `plugins/vsdd-factory/tests/sprint-state-format.bats`
  PC4 completeness/fidelity gate.
- Commit 3 traces to RUSTSEC-2026-0258 (h2 DoS advisory) via the
  `cargo deny check advisories` CI gate added in #781.

## Test Evidence

| Suite | Result |
|-------|--------|
| bats `s21-12-version-and-deny-gate.bats` | Fixed AC-003 false-fail (stderr/jq corruption); expected PASS |
| bats `sprint-state-format.bats` | 14/14 pass (locally verified, combined with companion factory-artifacts commits) |
| `validate-state-structure` | 65 pass / 0 fail (locally verified) |
| `validate-cross-site-correspondence` | 231 pass / 0 fail (locally verified) |
| `cargo deny check advisories` | Expected CLEAN against h2 0.4.19 (RUSTSEC-2026-0258 cleared) |

No new tests were added — these are fixes to existing test-harness gate logic
plus a lockfile-only dependency bump; no new product behavior requires new
test coverage. Full `cargo test --workspace --all-targets` +
`plugins/vsdd-factory/tests/run-all.sh` are run by CI on the PR (see below)
and by the authoritative post-merge `develop` push-run.

## Demo Evidence

N/A — no user-observable behavior changed (test-harness fixes + transitive
dependency bump only). Per the `fix-pr-delivery` skill: demo recording is
required only for behavior-changing fixes (output, error messages, CLI flags,
API responses, security *restrictions*, or other user-observable behavior).
This fix touches none of those — it corrects false-fail CI gates and clears a
transitive dependency advisory with no dispatcher-observable behavior change.

## Security Advisory Cleared

| Advisory | Package | Before | After | Severity | Status |
|----------|---------|--------|-------|----------|--------|
| RUSTSEC-2026-0258 | h2 | 0.4.14 | 0.4.19 | DoS (unbounded memory growth) | CLEARED |

`cargo deny check advisories` is expected to pass clean against this lockfile.

---

## Companion State (already on origin, not part of this PR)

CI mounts the `factory-artifacts` orphan branch fresh on every run. The
following two commits are **already merged to `origin/factory-artifacts`** and
are required alongside this PR's changes to achieve a fully green CI corpus,
but they live outside this PR's diff (this repo's `.factory/` content is owned
by `state-manager`, not `pr-manager` or story/fix branches):

- `6f3217de` — `fix(sprint-state): correct terminal-partition placement + full-graph depth ordering`
- `1a1dc0d1` — `state(corpus): reconcile S-21.11 STORY-INDEX hash parity + STATE.md banner wc-l self-count for rc.24 CI`

A comprehensive corpus sweep confirmed that this branch's 3 commits, combined
with the above 2 already-landed `factory-artifacts` commits, resolve **all 6**
release-gating CI failures identified ahead of rc.24, with no remaining
open-ended drift.

**Locally verified green (combined set):**

| Suite | Result |
|-------|--------|
| bats `sprint-state-format.bats` | 14/14 pass |
| `validate-state-structure` | 65 pass / 0 fail |
| `validate-cross-site-correspondence` | 231 pass / 0 fail |

---

## Risk Assessment

- **Blast radius:** Test harness (2 `.bats` files) + `Cargo.lock` dependency
  pin. No production Rust source (`crates/`) changed.
- **User impact:** None — no behavior change in the dispatcher binary logic;
  the h2 bump is a transitive dependency version bump that closes a DoS
  vector, it does not change dispatcher API surface or CLI behavior.
- **Risk level:** LOW.

## Security Review

**Verdict: APPROVE** (fresh independent pass by `security-reviewer`, dispatched
against the actual PR diff — not the commit-message claims).

### Verification performed
- Diffed all 4 changed files directly; confirmed no product-logic files
  touched (only `Cargo.lock`, `CHANGELOG.md`, and 2 `.bats` files).
- Built isolated worktrees for both `develop` (pre-PR) and
  `fix/rc24-ci-green`, and independently re-ran `cargo deny check advisories`
  + `cargo audit` against each:
  - `develop`: `cargo deny check advisories` **FAILS** — RUSTSEC-2026-0258
    confirmed present, reachable via `reqwest`/`tonic` → `hyper` →
    `h2 v0.4.14` in `factory-dispatcher`'s non-dev dependency graph.
  - PR branch: `cargo deny check advisories` **passes clean**; `cargo audit`
    shows zero h2-related findings. h2 resolves to 0.4.19, above the
    advisory's fix floor of 0.4.16.

### Findings

| ID | Title | Severity | Disposition |
|----|-------|----------|-------------|
| SEC-001 | RUSTSEC-2026-0258 (h2 unbounded empty DATA frames DoS, CWE-400 / GHSA-q83h-524g-xf6h) | was HIGH (production-reachable) → MITIGATED | Fixed, confirmed by independent tool re-run (0.4.19 ≥ 0.4.16 fix floor, Cargo.lock-only, no other h2 version remains) |
| SEC-002 | windows-sys 0.52.0/0.61.2 coexistence | LOW / informational | No advisory exists at either version; no action required |
| SEC-003 | Pre-existing `event-listener 5.4.1` unsound warning (RUSTSEC-2026-0221), dev-dep only via httpmock | LOW | Present identically on `develop` and this branch; not introduced or worsened by this PR — out of scope |
| SEC-004 | Bats test changes (wasmtime jq-stderr fix; SUPERSEDED-terminal exclusion) | informational | Confirmed NOT a weakening of the actual version-floor/completeness enforcement — `cargo-deny`/advisories CI jobs remain the authoritative backstop; SUPERSEDED exclusion is governance/process scope, no CWE applicable |

No CRITICAL or HIGH findings remain outstanding.

## Holdout Evaluation

N/A — test/CI/dependency-only change; no product behavior evaluated at wave
gate for this PR.

## Adversarial Review

N/A — evaluated at Phase 5 for story-level product changes; not applicable to
this CI-hygiene / dependency-lockfile fix PR.

---

## Review Convergence

pr-reviewer's `gh pr review --request-changes` could not land as a formal
GitHub review *state* (PR author and authenticated `gh` identity are the
same account — GitHub API rejects self-review-state changes); verdicts are
tracked via PR comment + this table instead. Full findings:
[`.factory/code-delivery/rc24-ci-green/pr-review.md`](../rc24-ci-green/pr-review.md)
and PR comment
[#issuecomment-5413884144](https://github.com/drbothen/vsdd-factory/pull/782#issuecomment-5413884144).

| Cycle | Verdict | Blocking | Suggestions | Nits | Disposition |
|-------|---------|----------|--------------|------|--------------|
| 1 | REQUEST_CHANGES | 1 — F-782-001: `s21-12-version-and-deny-gate.bats` AC-003 skip-path unreachable under bats `errexit` (TD-VSDD-059 paper-fix) | F-782-002: PR body under-describes commit 1's hard-fail→skip behavior change · F-782-003: `sprint-state-format.bats` SUPERSEDED exclusion unanchored to status column + case-inconsistent vs. `retired` | F-782-004: ASSERT 2 exclusion is a harmless no-op (reviewer: "fine to keep") | F-782-001 + F-782-003 routed to test-writer for in-worktree fix (production-grade-default: no deferral on a cheap in-scope completeness-gate hole); F-782-002 fixed directly in this PR description; F-782-004 accepted as-is per reviewer's own disposition |

---

## Pre-Merge Checklist

- [ ] All CI status checks passing on the PR
- [ ] Security review confirms h2 0.4.19 clears RUSTSEC-2026-0258 with no new
      advisories introduced
- [ ] pr-reviewer convergence to 0 blocking findings
- [ ] Post-merge `develop` push-run CI (mounts `origin/factory-artifacts`
      fresh, runs full corpus + bats including the push-only sprint-state
      canary) confirmed GREEN — this is the authoritative gate for rc.24
      readiness
