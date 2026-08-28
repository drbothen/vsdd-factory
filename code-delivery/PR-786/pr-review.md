# PR #786 — Fresh-Eyes Review (CANONICAL — cycle 4, post-revert)

> **This is the current canonical verdict.** Supersedes cycle 2 (`pr-review-cycle2.md`,
> APPROVE at `29fc003c`) and cycle 3 (`pr-review-cycle3.md`, REQUEST_CHANGES at `344f4819`).
> Per-cycle copy of this review is retained at `pr-review-cycle4.md`.

**PR:** fix(bundle): remove mis-bundled policy15-attestation-gate.wasm orphan + release.yml recurrence prevention
**Branch:** `fix/rc24-orphan-wasm-policy15` → `develop`
**Reviewer:** `vsdd-factory:pr-reviewer` (fresh-eyes, final pre-merge review)
**covered_sha:** `82f6d0c997dd8d3878ec79ccabc1902698e1205f`
**Verdict:** **APPROVE** — 0 blocking, 1 suggestion, 0 nit.

Cycle 3 requested changes on commit `344f4819` (fixture edit). That commit has been reverted as
`82f6d0c9`. This review re-verifies the revert and re-audits the net diff.

**GitHub review-state note:** posted via `gh pr review --comment` (formal review event, state
`COMMENTED`), not `--approve`. GitHub structurally refuses `addPullRequestReview` with `APPROVE`
when the authenticated user is the PR author ("Can not approve your own pull request"). Cycles 1–3
landed as `COMMENTED` for the same reason. The APPROVE verdict is authoritative in this document.
`gh pr comment` was **not** used.

---

## Net diff at reviewed SHA

`git diff origin/develop...82f6d0c9 --stat` yields exactly two paths:

| File | Change |
|------|--------|
| `.github/workflows/release.yml` | +6 / −1 |
| `plugins/vsdd-factory/hook-plugins/policy15-attestation-gate.wasm` | Bin 337816 → 0 (deleted) |

The fixture path is **absent from the net diff** — confirming the revert is net-zero.

---

## Verification 1 — Revert cleanliness (cycle-3 blocker)

**VERIFIED CLEAN.**

- `344f4819` added exactly 5 lines to
  `plugins/vsdd-factory/tests/fixtures/validate-state-structure/pass-real-state-md-snapshot/factory/STATE.md`;
  `82f6d0c9` removes exactly those 5 lines (`1 file changed, 5 deletions(-)`).
- `git diff origin/develop 82f6d0c9 -- '*pass-real-state-md-snapshot*'` is **empty** — the fixture is
  byte-identical to `develop`.
- Fixture line 1 is now `---` (frontmatter open), and the file's single `SIZE BUDGET` banner sits at
  line 24. The duplicate-banner defect is gone.

**Independent confirmation of cycle 3's root-cause diagnosis.** `pass-real-state-md-snapshot.bats`
documents at its header (F-P3-002) that `setup()` **auto-copies the live `REPO_ROOT/.factory/STATE.md`
at run time**, and that "the frozen fixture ... is retained as a documentation reference for the
pass-2 fix-burst baseline." The frozen fixture is therefore never read by the test. Two consequences:

1. `344f4819` could not have fixed the F-P2-002 bats failure — it edited a file the test does not read.
2. Reverting it is strictly correct: the fixture's value is as an accurate documentation snapshot, and
   the duplicate banner made it inaccurate.

Confirmed empirically: running the suite in a clean worktree, tests 40–42 (the F-P2-002 tests) skip with
`.factory/STATE.md absent — factory-artifacts worktree not mounted`, i.e. they key off the live file,
not the fixture.

## Verification 2 — No regressions from the revert

**VERIFIED.** The revert touches one test-fixture file that no test reads. `grep -rln
"pass-real-state-md-snapshot"` matches only `validate-state-structure/pass-real-state-md-snapshot.bats`,
whose `setup()` overwrites the content with the live file. No other consumer exists.

## Verification 3 — Commits 9facd966 / ce7ca4c6 / 29fc003c remain sound

**VERIFIED.** Re-audited independently of cycles 1–2.

**`9facd966` (delete the orphan wasm)** — correct.
- `cargo metadata` confirms `policy15-attestation-gate` is a real workspace member whose targets are
  `[lib, bin, test]`; it is a native CLI (`src/main.rs`) invoked as a host process by `ci.yml`, not a
  sandboxed hook plugin.
- Zero entries in `hooks-registry.toml` and `resolvers-registry.toml` → dual-registry orphan confirmed,
  matching the S-19.04 AC-006 / T-009 definition.
- Repo-wide `grep` for `policy15-attestation-gate.wasm` across `*.rs *.toml *.sh *.bats *.yml *.md`
  returns **no** references. Deletion leaves no dangling path.

**`ce7ca4c6` (`--exclude policy15-attestation-gate`)** — correct and correctly targeted.
- The crate name matches the workspace member exactly, so the exclusion is effective (not a silently
  ignored no-op).
- Target analysis shows why this is the *governing* fix: the `lib` target emits
  `policy15_attestation_gate.wasm` (underscore), already skipped by the pre-existing `*_*.wasm` outer
  glob; the `bin` target emits `policy15-attestation-gate.wasm` (hyphen), which passes that glob. The
  `--exclude` removes the artifact at the build step, which is the right layer.
- The inline trailing comment on the former last continuation line was relocated to its own comment
  lines below the command. `release.yml` parses as valid YAML at the reviewed SHA (`yaml.safe_load`
  succeeds; 6 jobs resolved), and the relocated form is valid bash.

**`29fc003c` (defense-in-depth case arms)** — correct, and applied to **both** staging loops.
- Arms added at the "Stage artifact directory" step and the "Stage wasm plugins" (commit-binaries) step
  — 2 arms, matching the 2 loops.

**Anti-drift gate not broken by the comment relocation.** This was the main regression risk, since
`read-prefix-wasm.bats` T-009h greps `release.yml` by pattern count. Recomputed against the reviewed
SHA: `--exclude read-prefix-fixture` = 1 (== workspace build count), `read-prefix-fixture.wasm)` arms
= 2 (== required 2). The new `# read-prefix-fixture: ...` comment line does not contain `--exclude`, so
it adds no spurious match. **Executed `bats read-prefix-wasm.bats` at the reviewed SHA: 8/8 ok,
including T-009h.**

---

## Findings

| # | Severity | Category | Finding | Suggestion |
|---|----------|----------|---------|------------|
| 1 | suggestion | coverage | No POLICY-20-style presence gate for `policy15-attestation-gate`. T-009h hardcodes `read-prefix-fixture` patterns, so the three new defenses are themselves ungated. | Extend/parameterize T-009h over both crate names in a follow-up. |

### Finding 1 detail (non-blocking)

`T-009h` exists precisely to close F-P1-002 ("single-point-defense gap") for `read-prefix-fixture`: it
asserts `--exclude` count == workspace-build count and staging-arm count == 2, with a two-direction
mutation-liveness check. This PR establishes the identical defense pattern for
`policy15-attestation-gate` but adds no corresponding presence gate. A future `release.yml` edit could
silently drop `--exclude policy15-attestation-gate` and no static gate would fire.

Why this is a suggestion and not a blocker:
- The defense is already triple-layered (build exclusion + 2 staging arms).
- A downstream detection gate exists and is *proven* — S-19.04 T-009 orphan-check on `develop` CI is
  exactly what surfaced this bug. A reintroduced orphan would fail CI rather than ship silently.
- The gap is one of prevention *timing*, not of detection coverage.

Recommend a follow-up to generalize T-009h into a table-driven gate over the excluded-crate set, so the
next native-binary crate added to the workspace inherits the gate automatically.

---

## Checklist

| # | Item | Status | Notes |
|---|------|--------|-------|
| 1 | Diff coherence | PASS | Both paths serve the stated orphan-removal + recurrence-prevention goal. No unrelated changes; the out-of-scope fixture edit was reverted. |
| 2 | Description accuracy | PASS | Body documents commits 1–3 and the root cause accurately. Rollback section omits `29fc003c`, but it is inert without the wasm — cosmetic only. |
| 3 | Test coverage | PASS | T-009 covers the orphan removal; T-009h covers the exclusion presence. Verified passing at the reviewed SHA. |
| 4 | Demo evidence | N/A | Binary-artifact removal + CI config change; no user-facing runtime behavior. T-009 passing is the functional evidence. Correctly declared N/A. |
| 5 | Commit quality | PASS | Conventional format throughout; revert uses the standard `git revert` message citing the reverted SHA. |
| 6 | Diff size | PASS | 6 insertions / 1 deletion plus one binary deletion. Far below the 500-line flag. |
| 7 | Missing changes | PASS | Both halves of the stated fix are present. See Finding 1 for the optional hardening delta. |
| 8 | Dependency status | PASS | No upstream PRs; `MERGEABLE` against `develop`. |

## CI

No failing checks at the reviewed SHA (8 pass / 8 pending / 1 skipping / **0 fail**). Passing: SAST
(Semgrep), attestation-gate-non-vacuity-controls, bats-darwin-leg, build-dispatcher (linux-arm64),
deny-advisories, policy-15-attestation-location, platforms-drift, validate.

**Approval is on the diff. Merge remains gated on all required checks going green** — in particular
`bats-full-suite (linux)` and `cargo-host`, which carry T-009 and T-009h.
