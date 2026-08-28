# PR #786 — Code Review Cycle 2 (final before merge)

**PR:** #786 — `fix(bundle): remove mis-bundled policy15-attestation-gate.wasm orphan + release.yml recurrence prevention`
**Branch:** `fix/rc24-orphan-wasm-policy15` → `develop`
**covered_sha:** `29fc003cbba9818999e41d8a575ca78151bd55c1`
**Verdict:** **APPROVE**
**Blocking findings:** 0

---

## Scope of cycle 2

Cycle 1 returned APPROVE with one ADVISORY (CR-001) and one NITPICK (CR-002, not
applicable). The only delta since cycle 1 is a single commit, `29fc003c`, touching
one file with two added lines. This review verifies that delta and re-confirms the
absence of blocking findings.

Commit range reviewed this cycle: `ce7ca4c6..29fc003c`.

## CR-001 — verification (CLOSED)

Cycle 1 finding: the release staging loops relied solely on
`--exclude policy15-attestation-gate` in the `cargo build --target wasm32-wasip1`
invocation. That is the correct root-cause fix, but it is a single point of failure —
if the `--exclude` flag is ever dropped during a future workspace-exclusion edit, the
staging loop would silently copy `policy15-attestation-gate.wasm` into
`hook-plugins/`, reintroducing the orphan that this PR exists to remove. The sibling
crate `read-prefix-fixture` already had belt-and-suspenders coverage (both an
`--exclude` and a named `case` arm); `policy15-attestation-gate` did not.

Fix verified present at both staging sites in
`.github/workflows/release.yml`:

| Line | Job / step | Status |
|------|-----------|--------|
| 182 | `cargo build --target wasm32-wasip1 … --exclude policy15-attestation-gate` (root-cause exclusion, from `ce7ca4c6`) | present |
| 184 | Rationale comment: native CLI binary (`src/main.rs`), no wasm target, invoked by `ci.yml` as a host process | present |
| 215 | `build-dispatcher` → *Stage artifact directory* loop | **added by `29fc003c`** |
| 352 | `commit-binaries` → canonical linux-x64 staging loop | **added by `29fc003c`** |

Both new arms are byte-identical in form to each other and structurally identical to
the pre-existing `read-prefix-fixture.wasm)` arm immediately above them, including the
`; continue ;;` terminator, so control flow is unambiguous.

Correctness checks performed:

1. **Artifact-name accuracy.** `policy15-attestation-gate` is a binary-target crate,
   so a `wasm32-wasip1` build would emit the hyphen-preserved basename
   `policy15-attestation-gate.wasm`. The case pattern matches that name exactly. A
   lib-target stub, if one were ever produced, would be underscore-named
   (`policy15_attestation_gate.wasm`) and is already caught by the outer `*_*.wasm`
   arm — so both possible artifact spellings are covered.
2. **Arm ordering.** The new arms sit after the outer `*_*.wasm` arm. Since the target
   basename contains no underscore, it cannot be shadowed by the earlier arm; ordering
   is therefore correct and non-fragile.
3. **No behavioral change on the happy path.** With `--exclude` in place the artifact
   is never produced, so neither new arm fires during a normal release. The change is
   inert unless the root-cause exclusion regresses — which is exactly the intended
   defense-in-depth semantics, and means the change carries no release-pipeline risk
   of its own.
4. **No collateral edits.** `git show 29fc003c --stat` confirms exactly
   `1 file changed, 2 insertions(+)`. No unrelated changes rode along.
5. **Commit message quality.** Conventional format (`fix(release):`), explains the
   relationship to `ce7ca4c6`, names the mirrored pattern, and cites the finding it
   closes. No AI attribution trailer.

## 8-item checklist

| # | Item | Result |
|---|------|--------|
| 1 | Diff coherence | PASS — every change traces to the orphan-WASM fix or its recurrence prevention. |
| 2 | Description accuracy | PASS — title and body match the diff (this was CR-002 in cycle 1; re-confirmed accurate). |
| 3 | Test coverage | PASS — `crates/factory-dispatcher/tests/bundle_orphan_check.rs` is the governing gate (T-010/T-011 dual-registry orphan detection); the reciprocal anchor comment `# Test gate: …::stage_release_bundle` is present at both staging sites. See OBS-001 below. |
| 4 | Demo evidence | N/A — CI-workflow / bundle-hygiene fix with no user-observable surface. CI job results are the evidence. |
| 5 | Commit quality | PASS — conventional format, clear rationale, no AI attribution. |
| 6 | Diff size | PASS — 2 lines this cycle; whole PR remains small. |
| 7 | Missing changes | PASS — both staging loops covered; no third staging site exists in `release.yml`. |
| 8 | Dependency status | PASS — no upstream PR dependencies; base `develop` merges cleanly (`mergeable: MERGEABLE`). |

## Non-blocking observations

### OBS-001 — [NIT] Test-mirror docstring drift (pre-existing, not introduced by this PR)

`crates/factory-dispatcher/tests/bundle_orphan_check.rs` mirrors only the outer
`*_*.wasm` glob in its `stage_release_bundle` helper; the named inner arms are
documented but not replicated. That is stated explicitly in the helper's docstring
("The inner named arms document…"), so this is a deliberate, documented boundary and
not a defect. However, the T-011 docstring asserts that if `--exclude
read-prefix-fixture` were removed, `read-prefix-fixture.wasm` "would pass through
staging" — a claim the real workflow already contradicts, because a named skip arm for
it exists. This PR adds a structurally identical arm for `policy15-attestation-gate`,
which widens the same pre-existing documentation gap by one entry.

This is **not introduced by this PR** and **does not block merge**. The test still
holds as written (it tests the helper, not the workflow), and the workflow is strictly
safer than the test's stated assumption — the drift is in the conservative direction.
Suggested cleanup for whoever next touches that test file: reword the T-011 and
`stage_release_bundle` docstrings to say the outer glob is the *mirrored* rule and the
named arms are *additional, unmirrored* workflow-side defenses, and add
`policy15-attestation-gate.wasm` to that enumeration.

### OBS-002 — [INFO] CI not yet fully green at review time

At the time of this review the following checks had already passed:
`SAST (Semgrep)`, `policy-15-attestation-location`,
`attestation-gate-non-vacuity-controls`, `platforms-drift`,
`bats-darwin-leg (macos, /bin/bash 3.2)`, `deny-advisories`.

Still running or queued: `validate`, `cargo-host (ubuntu-latest)`,
`cargo-host (macos-latest)`, `bats-full-suite (linux)`,
`bats-wave-handoff (macos)`, and all five `build-dispatcher` legs.

Notably `policy-15-attestation-location` and
`attestation-gate-non-vacuity-controls` — the two checks most directly exercising the
`policy15-attestation-gate` host binary — are already SUCCESS, confirming the
`--exclude` from the wasm build did not break the native CI gate. This is an
informational note, not a finding: standard merge discipline (all required checks
green before merge) applies, and no reviewed content depends on the pending legs.

## What was verified (anti-rubber-stamp statement)

- Ran `grep -n "policy15-attestation-gate" .github/workflows/release.yml` and read all
  four hits in surrounding context (~40 lines at each staging site), confirming the two
  new arms are inside the correct `case` blocks of the correct loops and not merely
  present somewhere in the file.
- Read the full `git show 29fc003c` patch and `--stat` to confirm the delta is exactly
  two insertions in one file with no collateral changes.
- Reasoned about cargo's wasm32-wasip1 artifact naming for bin-vs-lib targets to confirm
  the case pattern matches the name that would actually be emitted, and that the
  underscore variant is already covered by the outer glob.
- Cross-read `crates/factory-dispatcher/tests/bundle_orphan_check.rs` to check whether
  the workflow change desynchronizes the test mirror (it does not — the mirror boundary
  is explicitly documented; see OBS-001).
- Confirmed base-branch mergeability and enumerated CI check state.

## Verdict

**APPROVE** — `covered_sha: 29fc003cbba9818999e41d8a575ca78151bd55c1`

CR-001 is correctly and completely closed at both staging sites. Zero BLOCKING
findings. Zero SUGGESTION findings. One NIT (OBS-001) that is pre-existing and
explicitly out of this PR's scope. Merge once the remaining CI legs report green.
