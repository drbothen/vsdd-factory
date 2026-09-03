---
document_type: adr
adr_id: ADR-050
status: accepted
date: 2026-09-03
subsystems_affected: [SS-01, SS-07]
supersedes: null
superseded_by: null
producer: architect
traces_to: .factory/specs/architecture/ARCH-INDEX.md
---

# ADR-050: CI Darwin-Leg Source-Build Discipline for Registry-Schema Forward Compatibility

## Context

S-25.01 (PR #807) adds a new dispatcher registry `on_error` value, `"block_if_marker"`
(`OnError::BlockIfMarker`, `crates/factory-dispatcher/src/registry.rs`; ADR-048 §Decision 1),
consumed by two brand-new `[[hooks]]` entries in `plugins/vsdd-factory/hooks-registry.toml`
(`validate-unvalidated-mutation-marker` Arm 1/Arm 2; confirmed via
`git diff origin/develop origin/feature/S-25.01 -- hooks-registry.toml` — zero prior
occurrences of either entry name or `block_if_marker` on `develop`; both are wholly new).

CI job `bats-darwin-leg-macos` (`.github/workflows/ci.yml` line ~904, name
`bats-darwin-leg (macos, /bin/bash 3.2)`) fails on this PR. Its "Set up dispatcher binary
for T-001" step copies the COMMITTED, STALE bundled darwin binary
(`plugins/vsdd-factory/hooks/dispatcher/bin/darwin-{arm64,x64}/factory-dispatcher`,
last rebuilt at the v1.0.0-rc.23 release per `git log -1` on that path) into
`target/release/factory-dispatcher` instead of using a dispatcher built from the PR's
own source. That stale binary's `OnError` enum predates the `BlockIfMarker` variant and
carries no `#[serde(other)]` fallback (`#[serde(rename_all = "snake_case")]` only), so
registry load fails outright — `registry parse failed: TOML parse error ... unknown
variant \`block_if_marker\`, expected \`continue\` or \`block\`` — and `T-001` never
reaches its `READY_SHA_MISSING` assertion. All other CI legs pass: `cargo-host
(macos-latest)` (source build, SUCCESS), `bats-full-suite (linux)` (source build,
SUCCESS), and the `build-dispatcher (darwin-arm64)` / `(darwin-x64)` matrix jobs
(SUCCESS) all compile and test S-25.01's own source cleanly.

Root cause is structural, not S-25.01-specific: `plugins/vsdd-factory/hooks/dispatcher/bin/**`
is rebuilt ONLY by `release.yml`'s `commit-binaries` job, as a bot commit onto `main`
at release time, then carried back to `develop` by release.yml's `sync-develop` job
(`RELEASING.md` §"What Happens After Merge"). No feature or fix PR touches
`dispatcher/bin/**` (by convention, documented in `CLAUDE.md`: "the bundle is rebuilt
only by release.yml at release time; feature PRs don't touch dispatcher/bin/**"). This
means the committed darwin bundle is ALWAYS the prior release's binary, one release
behind whatever is in flight on a feature branch. Any future PR that adds a new
registry-schema value (a new `on_error` or `failure_policy` enum variant, or any new
field the current schema's `deny_unknown_fields`/closed-enum `Deserialize` rejects) in
the SAME PR that also wires a registry entry using that value will deterministically
hit this identical false-CI-failure — this is a recurring class, not a one-off.

Confirmed by direct inspection that `bats-darwin-leg-macos`'s OWN header comments frame
its purpose as `/bin/bash` 3.2.x SHELL-SCRIPT interpreter-dialect discipline ("Darwin-leg
interpreter discipline gate (S-19.01 AC-004; ADR-030 §Decision 3)... to detect bash 3.2
regressions before they reach production macOS runners"; "L-BB-simulation-shell-dialect-gap
(D-750): this job closes the gap where scripts failed on macOS CI (bash 3.2.57) but passed
locally (Homebrew 5.x)"), not literal-bundle-artifact validation. The stale-bundle copy step
is a build-time SHORTCUT ("Copy the pre-built darwin binary from the repository bundle to
avoid a full cargo build (~15 min) on each macOS CI run"), not a deliberate "prove the
shipped bundle works" design goal.

Separately confirmed by reading `.github/workflows/release.yml` in full (lines 1-75, the
"Pre-release Validation" job) that release.yml runs `runs-on: ubuntu-latest` only — there
is NO macOS / `/bin/bash` 3.2 validation anywhere in the release pipeline. `bats-darwin-leg-macos`
is therefore the ONLY existing CI check that exercises the committed darwin bundle binary at
all today, and even that exercise is nominal: the existing `--version` probe already
swallows failure (`target/release/factory-dispatcher --version 2>/dev/null || echo
'binary staged'`), so it provides no real integrity signal on the bundle artifact today.

Also confirmed: `ci.yml`'s `build-dispatcher` matrix job (line ~493) already builds
darwin-arm64 and darwin-x64 dispatcher binaries FROM THIS PR'S OWN SOURCE on every PR
(`cargo build --release --target {aarch64,x86_64}-apple-darwin`), runs the workspace test
suite against them (`matrix.run_tests: true` for both darwin entries), and uploads each as
an `actions/upload-artifact@v4` artifact named `factory-dispatcher-darwin-{arm64,x64}`
(14-day retention). The job's own comment states plainly: "nothing downstream downloads
this upload for bundling" — i.e., the artifact is already built, on every PR, and
currently unused.

## Decision

1. `bats-darwin-leg-macos` (`.github/workflows/ci.yml`) adds `needs: build-dispatcher` and,
   in its "Set up dispatcher binary for T-001" step, downloads the matching
   `factory-dispatcher-darwin-{arm64,x64}` artifact (selected by `uname -m`, same arch-detect
   logic already present) via `actions/download-artifact@v4` and stages it at
   `target/release/factory-dispatcher`, INSTEAD OF copying the committed
   `plugins/vsdd-factory/hooks/dispatcher/bin/darwin-*/factory-dispatcher` bundle.

2. A small, separate "bundle smoke-test" step is added to `bats-darwin-leg-macos` (or an
   adjacent job) that invokes the COMMITTED bundle binary directly
   (`plugins/vsdd-factory/hooks/dispatcher/bin/darwin-*/factory-dispatcher --version`) and
   asserts a clean, non-empty, non-error exit — preserving a minimal signal that the
   artifact operators actually receive (wrong-arch, truncated, non-executable) is not
   silently broken, without coupling that check to registry-schema forward-compatibility.
   This step MUST NOT parse `hooks-registry.toml` against the committed binary — that
   coupling is precisely what this ADR removes.

3. General forward-compatibility principle established for all future registry-schema
   additions: a PR MAY land new registry-schema code (a new `on_error`/`failure_policy`
   enum variant, a new field) and a registry entry that USES that new value IN THE SAME
   PR, with NO interim/staged/weakened value and NO two-phase landing choreography,
   because CI (`bats-darwin-leg-macos` included, after this decision) always validates the
   PR's own freshly-built dispatcher against the PR's own registry — never a stale,
   released-and-therefore-necessarily-older bundle.

## Rationale

The alternative of staging the registry entries' `on_error` value at an already-supported
setting (`"continue"` or `"block"`) until a release rebuilds the bundle was evaluated and
rejected: `"continue"` reproduces exactly the D-1135 fail-open-on-crash posture that
ADR-048 was human-directed to reverse (the precise CWE-636 gap ADR-048 closes); `"block"`
reproduces exactly the unconditional-crash-block posture ADR-048 §"Alternatives Considered"
already rejected on self-lock grounds ("creates the unconditional self-lock D-1135 was
designed to prevent... irrecoverable until external process intervention"). Neither
existing `on_error` value is an acceptable interim substitute for `"block_if_marker"`
without either contradicting explicit, recent human direction or reproducing a posture the
architecture record already rejected — so any staging approach forces a security-posture
compromise that this CI-sequencing fix avoids entirely. Fixing the false-CI-failure at its
actual root (test target selection) lets S-25.01 ship with `on_error = "block_if_marker"`
verbatim, exactly as ADR-048 §Decision 1 specifies, on the first attempt.

The `build-dispatcher` artifact-reuse mechanism costs no additional CI compute — the
darwin-arm64/darwin-x64 dispatcher binaries are already built and already uploaded on
every PR; `bats-darwin-leg-macos` merely needs to consume what already exists. This
converts a currently-inert artifact ("nothing downstream downloads this upload") into a
load-bearing one.

## Consequences

### Positive

- S-25.01 (and any future PR) can land new dispatcher registry-schema values and their
  first consuming registry entries in the same PR, without a staged/weakened interim
  `on_error`/`failure_policy` value and without a follow-up "flip the registry" story.
- `bats-darwin-leg-macos` now tests the PR's OWN dispatcher logic under real `/bin/bash`
  3.2 — a strictly more correct target for a PR-validation gate than a version-skewed,
  always-one-release-behind bundle.
- The `build-dispatcher` darwin artifacts (already built, already uploaded, previously
  unconsumed) become load-bearing, eliminating wasted CI compute duplication.
- Closes the general false-CI-failure class for every future registry-schema addition,
  not just S-25.01 — the recurring "chicken-and-egg" release-sequencing problem described
  in this ADR's Context does not recur.

### Negative / Trade-offs

- `bats-darwin-leg-macos` gains a `needs: build-dispatcher` dependency, so its start is
  gated on the matrix job's completion — a wall-clock latency increase for that specific
  check, though not additional compute (the darwin builds already run on every PR
  regardless).
- The committed-bundle "smoke test" (Decision 2) is a narrower integrity check than full
  bats-suite execution against the bundle. It catches gross artifact corruption/wrong-arch
  but does not exercise the bundle's actual gate behavior end-to-end. This is judged
  acceptable because (a) release.yml has no macOS validation of any kind today, so this is
  a net-new signal, not a replacement for a stronger existing one, and (b) the
  released-bundle's actual behavior is exercised in production by every operator on that
  release, with the next release's `bats-full-suite`/`cargo-host` runs (against the NEW
  source, built fresh) providing the load-bearing regression coverage.
- A future contributor unfamiliar with this ADR could reintroduce the stale-bundle-copy
  pattern (e.g., "optimize CI time" by reverting to the committed binary). The CI job
  comment block should cite this ADR to prevent silent regression.

### Status as of 2026-09-03 (v1.0)

**ACCEPTED — Human-Ratified 2026-09-03 (POLICY 22; D-1158).** The human read this ADR in full
and explicitly approved Decisions 1-3 as drafted (POLICY 22 precedent: architect-authored
CI/testing-policy ADRs in this repo — see ADR-030 §Decision 3 / S-19.01 AC-004 as the precedent
establishing the darwin-leg gate's own design). No content in this ADR's Context / Decision /
Rationale / Consequences / Alternatives Considered / Source sections changed as part of this
ratification — status flip only, recorded by state-manager per the ADR-049 precedent (v1.0
frontmatter `status: proposed` -> `accepted`, no other frontmatter field added). Implementation
(the `ci.yml` diff) is routed to `devops-engineer` per the CLAUDE.md Agent Routing Table and is
landing concurrently on `feature/S-25.01` to unblock PR #807's `bats-darwin-leg-macos` CI
failure — the ratification here does not itself constitute the fix; the code-level `ci.yml`
change is the separate implementation artifact this ADR authorizes.

## Alternatives Considered

- **Two-phase registry staging (weakened interim `on_error` value):** Land S-25.01's
  dispatcher code now, but set the two new registry entries' `on_error` to `"continue"`
  or `"block"` until a release rebuilds the bundle, then flip to `"block_if_marker"` in a
  follow-up PR. Rejected: both available interim values reproduce a posture ADR-048
  already, explicitly rejected or reversed (see Rationale above) — this is a genuine
  security/availability trade-off, not a mechanical staging step, and the CI-sequencing
  fix in this ADR avoids the trade-off entirely rather than accepting it temporarily.

- **Omit the new registry entries entirely until post-release, ship dispatcher code only:**
  Rejected: S-25.01's own acceptance tests (`plugins/vsdd-factory/tests/validate-unvalidated-mutation-marker.bats`,
  VP-105-A through VP-105-I) exercise the LIVE registry-wired gate end-to-end and are
  explicitly designed as BC-5.38.001 Red-Gate tests that FAIL until the registry entries
  exist ("Red Gate state: these tests FAIL until T-4/T-5 are complete"). Omitting the
  entries reintroduces Red-Gate failure and blocks S-25.01 by a different mechanism.

- **Rebuild and commit the darwin (and other platform) bundle binaries in-PR:** Rejected:
  violates the documented release-owns-the-bundle convention (`CLAUDE.md`: "feature PRs
  don't touch dispatcher/bin/**"), and would need to repeat for every platform on every
  future registry-schema-adding PR — the class of problem, not a one-off fix.

- **Make `OnError`'s `Deserialize` tolerant of unknown variants (e.g., `#[serde(other)]`
  mapped to a safe default):** Rejected as the PRIMARY fix (though worth independent future
  consideration as defense-in-depth): it cannot help THIS PR regardless, since the
  tolerant deserializer would have to already be compiled into the OLD, already-released
  binary to have any effect — code changes in the current PR cannot retroactively patch an
  already-built, already-committed binary. It also raises its own security question (an
  unknown-variant fallback that defaults toward permissive behavior would itself be a
  fail-open regression of exactly the kind ADR-048 exists to close; a fallback that
  defaults toward `"block"` avoids that but reproduces the self-lock trade-off already
  rejected in ADR-048's Alternatives Considered). Out of scope for this ADR.

## Source / Origin

- `.github/workflows/ci.yml` lines ~493-690 (`build-dispatcher` matrix job, darwin-arm64/
  darwin-x64 entries, `run_tests: true`, `actions/upload-artifact@v4` step,
  `factory-dispatcher-${{ matrix.platform }}` artifact name).
- `.github/workflows/ci.yml` lines ~904-970 (`bats-darwin-leg-macos` job, "Set up dispatcher
  binary for T-001" step, currently `cp`-ing `plugins/vsdd-factory/hooks/dispatcher/bin/darwin-{arm64,x64}/factory-dispatcher`).
- `.github/workflows/release.yml` lines 1-75 (`validate` / "Pre-release Validation" job,
  `runs-on: ubuntu-latest` — no macOS leg in the release pipeline).
- `crates/factory-dispatcher/src/registry.rs` — `OnError` enum (`#[serde(rename_all =
  "snake_case")]`, variants `Continue`/`Block`/`BlockIfMarker`, no `#[serde(other)]`
  fallback).
- `plugins/vsdd-factory/hooks-registry.toml` — `validate-unvalidated-mutation-marker` /
  `validate-unvalidated-mutation-marker-git` entries (new in `feature/S-25.01`, confirmed
  absent on `develop` via `git diff origin/develop origin/feature/S-25.01`).
- `git log -1 -- plugins/vsdd-factory/hooks/dispatcher/bin/darwin-arm64/factory-dispatcher`
  → `chore: bundle dispatcher binaries for v1.0.0-rc.23` (confirms bundle staleness).
- `RELEASING.md` §"What Happens After Merge" (`commit-binaries` bot commit to `main`;
  `sync-develop` back-merge) — confirms the bundle-rebuild-only-at-release convention and
  the mechanism by which `develop` eventually receives the updated bundle.
- `CLAUDE.md` "Dispatcher binary discipline" — "feature PRs don't touch dispatcher/bin/**".
- ADR-048 §Decision 1 (`decisions/ADR-048-...md`) — `on_error = "block_if_marker"`
  registry snippet this ADR keeps unmodified; ADR-048 §"Alternatives Considered" —
  rejection of unconditional `"block"` on self-lock grounds, cited above.
- ADR-039 §Decision 1 — two-axis `on_error`/`failure_policy` model this ADR does not alter.
- ADR-030 §Decision 3 / S-19.01 AC-004 — precedent establishing the `bats-darwin-leg-macos`
  gate's own original design and ADR anchor.
- `.factory/cycles/v1.0-brownfield-backfill/determination-S2501-block-if-marker-bundle-sequencing.md`
  — full analysis this ADR was extracted from.
