## PR Review — fresh-eyes diff review

**Review verdict: APPROVE**
covered_sha: 1db2ab6509d00b257e0cb006cdcff42508584968

Reviewed as a fresh-context reviewer against the diff, PR description, and CI evidence only. No blocking findings. One SUGGESTION and two NITs below.

---

### What I verified independently

| Check | Result |
|---|---|
| `Cargo.toml` pins | Exactly 2 lines changed: `wasmtime = "46.0.3"`, `wasmtime-wasi = "46.0.3"`. Correct. |
| Diff scope | Exactly 2 files (`Cargo.toml`, `Cargo.lock`). 78 insertions / 78 deletions — perfectly symmetric, consistent with pure `version` + `checksum` swaps. Zero source-file changes. |
| Lockfile cascade consistency | Enumerated every `wasmtime*` / `cranelift*` / `pulley*` / `wiggle*` / `cap-*` package in the head-commit lockfile: **all 18 wasmtime-family crates at 46.0.3**, cranelift family at 0.133.3, cap-std family at 3.4.6, pulley + wiggle at 46.0.3. **Zero residual `46.0.2` / `0.133.2` / `3.4.5` entries.** |
| Sibling-pin sweep (TD-VSDD-060) | `git grep` across all `*.toml` at head: no per-crate `wasmtime*` version pin outside the workspace table — every member crate inherits via `workspace = true`. No orphaned pin left behind. |
| Multiple lockfiles | Only one tracked `Cargo.lock` in the repo. No second workspace to cascade. |
| Advisory applicability (independent of PR body) | Fetched both advisories from `rustsec/advisory-db` directly. `RUSTSEC-2026-0268` → `patched = [">= 46.0.3, < 47.0.0", ">= 47.0.4"]`. `RUSTSEC-2026-0269` → `patched = [">= 24.0.13, <25", ">= 36.0.14, <37", ">= 46.0.3, < 47.0.0", ">= 47.0.4"]`. **46.0.3 satisfies both.** The PR's "defer 47.x" rationale is sound — 47.x would have required 47.0.4, a larger migration for no additional security benefit. |
| No advisory suppression | `deny.toml` at head has `ignore = []`. The green `deny-advisories` check is therefore a genuine clean pass against the live RustSec DB, not a suppressed one. This is the strongest single piece of evidence in the PR. |
| Branch currency | Head is exactly one commit on top of current `origin/develop` (`9ab5a6f6`). Linear, `MERGEABLE`, no stale-base risk. |
| Commit quality | Single commit, conventional `fix(deps):` format, advisory IDs in the subject, no AI attribution. Clean. |
| Diff size | 156 changed lines — far under the 500-line review threshold. |
| Regression risk | Patch-level bump within `46.0.x`; no API surface touched (`Config`, fuel, epoch, preopens all unchanged upstream). `deny-advisories`, `validate`, Semgrep SAST, and 2 bats legs already green. |
| Yanked-`chacha20` note | Confirmed pre-existing: `chacha20 0.10.1` is present in the lockfile but **not touched by this diff**. Correctly scoped out of this PR — agreed, not a blocker here. |

---

### Findings

#### 1. [SUGGESTION] — operator-level remediation is not actually complete on merge; PR body reads as if it is

| Field | Value |
|---|---|
| Severity | suggestion |
| Category | description |

The diff is correct and I'm not asking for a code change. But the PR body's framing overstates what merging accomplishes, and a human reader would reasonably draw the wrong conclusion about exposure.

The body states "**Advisories Cleared by This PR**", "WASM sandbox escape and heap-DoS vectors closed", and under Blast Radius lists "Systems affected: `factory-dispatcher` binary". What merging to `develop` actually achieves is clearing the advisories **for source builds of the workspace**. The dispatcher binaries that operators execute are committed artifacts:

```
plugins/vsdd-factory/hooks/dispatcher/bin/darwin-arm64/factory-dispatcher
plugins/vsdd-factory/hooks/dispatcher/bin/darwin-x64/factory-dispatcher
plugins/vsdd-factory/hooks/dispatcher/bin/linux-arm64/factory-dispatcher
plugins/vsdd-factory/hooks/dispatcher/bin/linux-x64/factory-dispatcher
plugins/vsdd-factory/hooks/dispatcher/bin/windows-x64/factory-dispatcher.exe
```

This PR does not rebuild them — correctly so, since the release pipeline owns the binary bundle. That means the vulnerable wasmtime **46.0.2 remains statically linked into the binaries operators run** until a new release cross-compiles them.

This is not hypothetical: `v1.0.0-rc.24` is **already tagged** (`2f97d5e9`), and it was cut from a tree pinned at 46.0.2. So the current latest release ships the unpatched CVSS-HIGH sandbox escape, and it stays that way until rc.25. For a project whose stated threat model is *executing untrusted hook-plugin WASM*, that residual window is the part a reader most needs to see.

**Suggested fix** — no code change; amend the body and attach the follow-up so it can't get lost:

1. Under Risk Assessment, add: *"Effective scope: source builds only on merge. Bundled dispatcher binaries and the marketplace cache still embed wasmtime 46.0.2 until rc.25 is cut — v1.0.0-rc.24 (already tagged) ships unpatched."*
2. Change the two "CLEARED" claims to "cleared at source; operator-level clearance pending rc.25".
3. Add an explicit release follow-up item to cut rc.25 promptly, rather than letting it ride to the next scheduled release. This mirrors the already-documented `DEFAULT_FUEL_CAP` / ADR-042 pattern where a develop-branch change was not effective at operator level until a release was cut.

#### 2. [NIT] — RUSTSEC-2026-0269 is filed against `wasmtime`, not `wasmtime-wasi`

| Field | Value |
|---|---|
| Severity | nit |
| Category | description |

The title and body attribute 0269 to `wasmtime-wasi` ("wasmtime-wasi filesystem sandbox escape"). In the RustSec DB the advisory's `package` field is **`wasmtime`** — `crates/wasmtime-wasi/RUSTSEC-2026-0269.md` returns 404. The underlying defect plausibly lives in the WASI filesystem implementation, so the prose isn't wrong about mechanism, but the advisory attribution is. No action needed on the diff — bumping both crates in lockstep is correct regardless, since they are version-locked siblings and the lock cascade confirms it. Worth correcting if the body is edited for finding #1 anyway.

#### 3. [NIT] — "CVSS 8.8" is unsourced against the advisory record

| Field | Value |
|---|---|
| Severity | nit |
| Category | description |

The body cites CVSS 8.8 for 0269. The advisory carries a CVSS **4.0** vector (`AV:N/AC:L/AT:P/PR:L/UI:P/VC:H/VI:H/VA:L/SC:H/SI:H/SA:L`), not a v3.1 score. The HIGH classification is well-supported by that vector (high confidentiality + integrity impact, plus high subsequent-system impact — exactly what a sandbox escape looks like), so the severity call is right. Just cite the CVSS 4.0 vector or the GHSA's own rating rather than a bare "8.8", so the number is traceable.

---

### CI status at time of review

`deny-advisories` **SUCCESS** (the decisive check), plus `validate`, `SAST (Semgrep)`, `platforms-drift`, `policy-15-attestation-location`, `attestation-gate-non-vacuity-controls`, `bats-darwin-leg`, `bats-wave-handoff` all SUCCESS.

Still **IN_PROGRESS**: `cargo-host (ubuntu-latest)`, `cargo-host (macos-latest)`, `bats-full-suite (linux)`, and all 5 `build-dispatcher` legs. My approval is on the diff's correctness; the usual all-green gate should still be satisfied before merge — in particular the 5 `build-dispatcher` legs, since they are the real cross-platform proof that 46.0.3 compiles everywhere the release pipeline targets.

Note the body's pre-merge checklist already ticks `cargo test` / `cargo clippy` / `cargo fmt` as passing based on a local run. Two of those CI legs had not finished when I reviewed, so treat those ticks as local-only evidence until `cargo-host` goes green on both runners.

---

### Verdict

**APPROVE.** Minimal, correctly-scoped, internally consistent security patch. The lockfile cascade is complete with no residual old versions, the advisory applicability was independently confirmed against the RustSec DB, and `deny.toml` carries no suppression — so the green advisory gate is real. No blocking findings.

The one thing I'd genuinely like actioned before this is considered "done" is finding #1: merging closes the source-level exposure, but operators stay on unpatched binaries until rc.25 is cut. That's a release-scheduling action, not a diff change — but it should be attached somewhere durable rather than assumed.

---

## Posting provenance (governance record)

| Field | Value |
|---|---|
| Posted | Yes — verified on GitHub |
| Command | `gh pr review 804 --comment --body-file <this content>` |
| Result | exit 0, no stdout (normal for `--comment`) |
| Confirmed state | review by `Zious11`, state `COMMENTED`, `2026-08-31T23:34:06Z` |
| covered_sha | `1db2ab6509d00b257e0cb006cdcff42508584968` (matches `headRefOid`) |

**Why `--comment` and not `--approve`:** PR #804 is self-authored (author `Zious11` is the same GitHub account as the reviewer). GitHub returns HTTP 422 for `--approve` / `--request-changes` on a self-authored PR, so the verdict cannot be posted as a formal approval state. Per human governance ruling, the verdict is posted via `gh pr review --comment` with the verdict stated in the body's first lines.

Note this is `gh pr review --comment` (a formal review record, state `COMMENTED`) — **not** `gh pr comment` (a plain issue comment). The distinction matters: the `validate-pr-review-posted` guard's warning text targets `gh pr comment`, which was not used here.

**Known hook false-positive:** the `validate-pr-review-posted` SubagentStop plugin blocks on absence of an `--approve` / `--request-changes` invocation. For self-authored PRs this is the documented false-positive. Nothing was bypassed to resolve it — no `--no-verify`, no `--force`, no dispatcher bypass. The block is surfaced to the human rather than worked around.
