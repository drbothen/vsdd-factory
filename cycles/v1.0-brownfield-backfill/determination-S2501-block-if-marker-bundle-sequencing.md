---
document_type: determination
producer: architect
date: 2026-09-03
story: S-25.01
pr: 807
status: recommendation
related_adr: ADR-050
---

# Determination — S-25.01 (PR #807) `block_if_marker` / Bundle-Version-Skew Landing Sequence

## 1. Verified Diagnosis

The prompt's diagnosis was independently verified against the actual repository state
(commands and outputs below are the literal evidence; no narrative-only claims per POLICY 22).

**S-25.01's registry change is wholly new, not a modification of an existing entry:**

```
$ git show origin/develop:plugins/vsdd-factory/hooks-registry.toml | grep -c "validate-unvalidated-mutation-marker"
0
$ git diff origin/develop origin/feature/S-25.01 -- plugins/vsdd-factory/hooks-registry.toml | grep -E "^\+.*on_error"
+on_error = "block_if_marker"   (Arm 1: validate-unvalidated-mutation-marker, tool=^Agent$)
+on_error = "block_if_marker"   (Arm 2: validate-unvalidated-mutation-marker-git, tool=^Bash$)
```

Both `[[hooks]]` entries and both `on_error = "block_if_marker"` occurrences are entirely
new to `hooks-registry.toml` in `feature/S-25.01` — there is nothing to "roll back" on
`develop`; this is a net-new gate, not a regression of an existing one.

**The dispatcher's `OnError` enum has no forward-compatibility fallback:**

```rust
// crates/factory-dispatcher/src/registry.rs
#[derive(... Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum OnError {
    #[default]
    Continue,
    Block,
    BlockIfMarker,   // NEW in feature/S-25.01 — ADR-048 §Decision 1
}
```

No `#[serde(other)]` arm. An older binary's `OnError` enum (pre-S-25.01) has only
`Continue`/`Block`; parsing a registry containing the string `"block_if_marker"` fails the
WHOLE registry load (not just that one entry) with `unknown variant \`block_if_marker\`,
expected \`continue\` or \`block\``.

**The bundled darwin binary is stale by exactly one release:**

```
$ git log --oneline -1 -- plugins/vsdd-factory/hooks/dispatcher/bin/darwin-arm64/factory-dispatcher
80e5cd7b chore: bundle dispatcher binaries for v1.0.0-rc.23
```

Confirmed pre-S-25.01: `develop`'s bundled darwin binaries were last rebuilt at the
v1.0.0-rc.23 release, per the documented convention (`CLAUDE.md`: "the bundle is rebuilt
only by release.yml at release time; feature PRs don't touch dispatcher/bin/**"; confirmed
in `.github/workflows/release.yml`'s `commit-binaries`/`sync-develop` jobs and
`RELEASING.md` §"What Happens After Merge").

**CI job `bats-darwin-leg-macos` copies that stale committed binary, not a source build:**

```yaml
# .github/workflows/ci.yml ~line 946
- name: Set up dispatcher binary for T-001
  run: |
    ...
    DARWIN_BIN="plugins/vsdd-factory/hooks/dispatcher/bin/darwin-${arch}/factory-dispatcher"
    cp "${DARWIN_BIN}" target/release/factory-dispatcher
```

PR #807's own CI checks (`gh pr view 807 --json statusCheckRollup`) confirm the exact
failure signature: every OTHER leg that builds from source passes —
`cargo-host (macos-latest)` SUCCESS, `bats-full-suite (linux)` SUCCESS,
`build-dispatcher (darwin-arm64)`/`(darwin-x64)` SUCCESS — only
`bats-darwin-leg (macos, /bin/bash 3.2)` is FAILURE, and only because its dispatcher setup
step sources the stale bundle rather than a fresh build.

**Diagnosis fully confirmed.** The chicken-and-egg framing in the prompt is accurate: the
registry can't ship `block_if_marker` without the code, the code can't take effect for
operators until a release rebuilds the bundle, and the CI leg that happens to test the
stale bundle blocks the PR that would let that release happen.

## 2. Evaluation of the Four Candidate Resolutions

### Candidate 1 — Two-phase landing (registry-value staging)

Evaluated staging the two new registry entries' `on_error` at an already-supported value
(`"continue"` or `"block"`) until a release rebuilds the bundle, then flipping to
`"block_if_marker"` in a follow-up PR. **Rejected** after full analysis — not because
staging is mechanically wrong, but because BOTH candidate interim values reproduce a
posture ADR-048 already, explicitly, disposed of:

- `on_error = "continue"` (fail-open on crash) is the LITERAL D-1135 posture. ADR-048's own
  Context section states the human EXPLICITLY DIRECTED its reversal ("Human-directed
  redesign (2026-08-31): reverse D-1135 fail-open-on-crash ratification") specifically
  because it reintroduces the CWE-636 gap this whole ADR exists to close (crash + marker
  exists → silent allow). Staging with `"continue"`, even temporarily, ships exactly what
  the human said not to ship.
- `on_error = "block"` (unconditional block on any crash, marker or not) is explicitly
  evaluated and REJECTED in ADR-048's own "Alternatives Considered" section: "creates the
  unconditional self-lock D-1135 was designed to prevent (crash with no marker → still
  block → irrecoverable until external process intervention)." Its blast radius is also
  categorically broader than either interim option needs to be — it would block EVERY
  Agent dispatch project-wide (not just the specific quarantined mutation) on ANY crash of
  this one WASM plugin, for any reason, marker or not.

I confirmed this is not merely a theoretical concern: because these two registry entries
gate `^Agent$` (Arm 1) and `^Bash$` (Arm 2) broadly, `on_error = "block"`'s self-lock
would halt this very multi-agent factory's ability to dispatch ANY specialist agent while
the interim window is open, on a transient WASM crash unrelated to any real quarantine
condition. That is a materially worse operational posture than "the feature doesn't exist
yet," which is what `develop` has today.

I also verified this would NOT be caught by S-25.01's own test suite either way — the nine
`validate-unvalidated-mutation-marker.bats` acceptance tests (VP-105-A through VP-105-I)
all exercise the NORMAL (non-crash) gate-evaluation path and are indifferent to `on_error`'s
specific value; `on_error` only matters on the crash/fuel/timeout fallback path, which none
of them simulate. So staging would be "clean" by S-25.01's own green-gate signal while
silently reintroducing one of the two ADR-048-rejected postures. That is precisely the kind
of TDD-blind-spot regression the production-grade default in `CLAUDE.md` warns against.

**Conclusion: Candidate 1 forces a real security/availability trade-off that the ADR
record already litigated and rejected on both sides. It should not be adopted as a
"mechanical" staging step — see §3 for the preferred resolution that avoids the dilemma
entirely.**

### Candidate 2 — Forward-compatible `OnError` deserializer

Evaluated adding `#[serde(other)]` (or similar) to `OnError` so a future binary tolerates
an unrecognized `on_error` string. **Two problems, both fatal for THIS PR:**

1. It cannot help S-25.01 at all. The deserializer patch would need to already be compiled
   into the OLD, already-released darwin binary to have any effect on this PR's CI run.
   Code changes in `feature/S-25.01` cannot retroactively alter an already-built,
   already-committed binary — by the time a future release ships a tolerant deserializer,
   THIS problem (S-25.01 vs. rc.23's binary) is already resolved by that same release
   rebuilding the bundle anyway.
2. Its OWN security shape mirrors the Candidate-1 dilemma: a fallback that defaults an
   unknown `on_error` toward permissive behavior (`Continue`) is itself a fail-open
   regression of exactly the class ADR-048 was built to close (an operator on an
   in-between binary would silently downgrade any FUTURE `on_error` hardening to
   fail-open without ever knowing). A fallback that defaults toward `Block` reproduces the
   self-lock trade-off ADR-048's Alternatives Considered already rejected. There is no
   "safe default" direction for an unknown enum variant in a system whose entire design
   principle is that fail-open and fail-closed are NOT interchangeable defaults.

**Conclusion: worth flagging as an independent, optional defense-in-depth idea for a
future ADR (not this one — it's out of scope for the current problem and carries its own
unresolved security-default question), but it does not solve the immediate blocker and is
not part of this recommendation.**

### Candidate 3 — Darwin-leg test-design fix (RECOMMENDED)

Evaluated whether `bats-darwin-leg-macos`/T-001 should build from source (or skip/xfail
on version skew) instead of testing the stale bundle. Investigation found this to be
clearly the correct fix, for reasons not fully visible from the CI job's surface framing:

- The job's OWN header comments frame its purpose as `/bin/bash` 3.2.x SHELL-SCRIPT
  interpreter-dialect discipline ("Darwin-leg interpreter discipline gate... to detect
  bash 3.2 regressions"; "L-BB-simulation-shell-dialect-gap... this job closes the gap
  where scripts failed on macOS CI (bash 3.2.57) but passed locally (Homebrew 5.x)"), NOT
  a deliberate "prove the exact shipped bundle works" contract. The stale-bundle copy is
  explicitly a BUILD-TIME SHORTCUT ("avoid a full cargo build (~15 min)"), not a design
  requirement.
- I confirmed `.github/workflows/release.yml`'s own "Pre-release Validation" job runs
  `runs-on: ubuntu-latest` ONLY — there is no macOS / bash-3.2 validation anywhere in the
  release pipeline. So today, `bats-darwin-leg-macos` is the ONLY CI check that ever
  touches the committed darwin bundle at all, and even that touch is nominal: its existing
  `--version` probe already swallows failure (`... || echo 'binary staged'`), so it
  provides essentially no real bundle-integrity signal today.
- I confirmed `ci.yml`'s `build-dispatcher` matrix job ALREADY builds darwin-arm64 and
  darwin-x64 dispatchers from the PR's OWN source on every PR, runs the workspace test
  suite against them, and uploads each as an `actions/upload-artifact@v4` artifact named
  `factory-dispatcher-darwin-{arm64,x64}` — with the job's own comment stating "nothing
  downstream downloads this upload for bundling." This is an already-built, already-tested,
  currently-unused artifact sitting exactly where darwin-leg needs it.

**This is the recommended fix.** See §3 and ADR-050 for the full design.

### Candidate 4 — Rebuild+commit bundle binaries in-PR

Evaluated rebuilding and committing fresh darwin (and other-platform) binaries directly in
S-25.01. **Rejected:** violates the documented release-owns-the-bundle convention
(`CLAUDE.md`: "feature PRs don't touch dispatcher/bin/**"), and — more importantly for the
"general problem" framing the prompt asks about — it does not generalize: every future PR
that adds a new registry-schema value would need to repeat this for every platform, which
is the class of problem, not a fix for it.

## 3. Recommendation

**Adopt Candidate 3.** `bats-darwin-leg-macos` should consume the freshly-built,
already-produced `build-dispatcher` darwin artifact instead of the committed bundle. This
is the ONLY candidate that:

1. Lets S-25.01 ship `on_error = "block_if_marker"` verbatim, on the first attempt, exactly
   as ADR-048 §Decision 1 specifies — no registry staging, no interim security-posture
   compromise, no follow-up "flip the value" story.
2. Costs no additional CI compute (the darwin binaries are already built on every PR; they
   are simply not yet consumed by this job).
3. Permanently closes the GENERAL problem the prompt asks about: every future PR that adds
   a new `on_error`/`failure_policy` enum variant, or any other registry-schema addition
   the current schema's `deny_unknown_fields`/closed-enum `Deserialize` would reject, will
   no longer hit this false-CI-failure — because CI will always be testing that PR's own
   freshly-built dispatcher, never a stale, necessarily-older released bundle.
4. Does not require weighing D-1135's already-reversed fail-open posture or ADR-048's
   already-rejected self-lock posture against each other — it sidesteps the dilemma
   Candidate 1 forces, rather than picking a side of it.

I have drafted this as **ADR-050** (`.factory/specs/architecture/decisions/ADR-050-ci-darwin-leg-source-build-discipline-for-registry-schema-forward-compatibility.md`,
status `proposed`, registered in ARCH-INDEX.md's Architecture Decisions table, subsystems
SS-01/SS-07, template-compliance-validated PASS) so there is concrete, ratifiable content
rather than only a routing recommendation. ADR-050 does NOT modify ADR-048 or ADR-039 —
S-25.01's dispatcher code and registry entries are unchanged by this determination.

### Is this a general, documented pattern? Yes — captured in ADR-050 §Decision 3

Any future PR may land a new registry-schema value (a new `on_error`/`failure_policy`
variant, a new field) together with its first consuming registry entry in the SAME PR,
with no two-phase landing choreography required, because `bats-darwin-leg-macos` (once
ADR-050 is implemented) always validates the PR's own dispatcher against the PR's own
registry. This retires the "chicken-and-egg" release-sequencing hazard as a recurring
class, not just for S-25.01.

## 4. Concrete Step List, Owner Routing, and Ratification Requirements

| # | Change | File(s) | Owner (per CLAUDE.md Agent Routing Table) | ADR/BC impact |
|---|--------|---------|---------|---------|
| 1 | Add `needs: build-dispatcher` to `bats-darwin-leg-macos`; replace the "Set up dispatcher binary for T-001" `cp` step with `actions/download-artifact@v4` pulling `factory-dispatcher-darwin-{arm64,x64}` (arch-selected via the existing `uname -m` branch), staged to `target/release/factory-dispatcher` | `.github/workflows/ci.yml` (~lines 904-970) | `devops-engineer` | Implements ADR-050 §Decision 1 |
| 2 | Add a small, separate "bundle smoke-test" step invoking the COMMITTED `plugins/vsdd-factory/hooks/dispatcher/bin/darwin-*/factory-dispatcher --version` and asserting a clean, non-empty exit — MUST NOT parse `hooks-registry.toml` against it | `.github/workflows/ci.yml` (same job, or an adjacent lightweight job) | `devops-engineer` | Implements ADR-050 §Decision 2 |
| 3 | Update the job's header comment to cite ADR-050 (prevents a future contributor from silently reverting to the stale-bundle-copy pattern "to save CI time") | `.github/workflows/ci.yml` | `devops-engineer` | Documentation only |
| 4 | Ratify ADR-050 (POLICY 22) — human decision; on ratification, state-manager flips ADR-050 frontmatter `status: proposed` → `accepted`, records D-NNN in the decision-log, and updates ARCH-INDEX's Architecture Decisions row status text | `.factory/specs/architecture/decisions/ADR-050-...md`, `.factory/STATE.md` decision-log, `ARCH-INDEX.md` | Human ratification → `state-manager` executes the bookkeeping | POLICY 22 — see below |
| 5 | Re-run PR #807's CI after step 1-2 land on `develop`/rebase onto a branch carrying them, confirm `bats-darwin-leg-macos` goes GREEN with the real `block_if_marker` registry entries unmodified | — | `pr-manager` (existing PR lifecycle owner for #807) | — |

**No changes required to:** `plugins/vsdd-factory/hooks-registry.toml`, `crates/factory-dispatcher/src/registry.rs`, ADR-048, ADR-039, any S-25.01 BC/VP/story file, or the dispatcher bundle binaries themselves. S-25.01 ships as originally designed.

**Sequencing dependency:** step 1-3 (the `ci.yml` change) must land and be present on
whatever branch S-25.01's CI runs against BEFORE PR #807 can go green — i.e., either (a)
a small standalone infra PR lands the `ci.yml` change to `develop` first, and S-25.01
rebases/merges `develop` to pick it up, or (b) the `ci.yml` change is included directly in
PR #807 itself (also acceptable — it's a CI-only change, orthogonal to S-25.01's own ACs,
and PR #807 is the PR that surfaces the need). Given PR #807 is already open and blocked
on exactly this leg, routing the `ci.yml` fix as an ADDITIONAL commit on `feature/S-25.01`
(option b) is the faster path and avoids a second PR round-trip — **recommended**, subject
to devops-engineer/pr-manager confirming it doesn't conflict with S-25.01's own scope
discipline (this is a CI-infrastructure fix that happens to be required to unblock this
PR, not a change to S-25.01's behavioral scope, so it should be a clearly-separated commit
within the PR, not interleaved with S-25.01's own TDD micro-commits).

### Does this need POLICY 22 / a release in the loop?

- **POLICY 22 (human ratification):** YES for ADR-050. It is a new architectural decision
  (test-target provenance for a standing CI gate, plus the general forward-compatibility
  principle for all future registry-schema evolution) and this repo's established
  convention (see ADR-030 §Decision 3 / S-19.01 AC-004, and every ADR-048 revision) is
  that architect-authored ADRs require human ratification before `status: accepted`. I
  have NOT self-ratified it — it is `status: proposed` pending human review.
- **A release:** NOT required to unblock PR #807. Unlike Candidate 1 (which depended on a
  release rebuilding the bundle before a follow-up flip), Candidate 3 requires no release
  at all — it only requires the `ci.yml` change to be present on the branch under test. PR
  #807 can go green as soon as steps 1-2 land, without waiting for any release cycle.
