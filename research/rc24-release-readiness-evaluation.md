# Release-Readiness Evaluation — `v1.0.0-rc.24` candidate

**Range analyzed:** `v1.0.0-rc.23..develop` (HEAD `27c56c01`)
**Commit count:** 51
**Evaluator:** read-only analysis (no code modified, no branches, no tags)
**Date:** 2026-08-25
**develop STATE.md version:** 8.65 · cycle `v1.0-brownfield-backfill` · pipeline **PAUSED** (Wave-7 held pending ADR-045 ratification)

---

## 1. Executive summary + recommendation

**Recommendation: SHIP `v1.0.0-rc.24` — WITH NOTES.**

The dominant reason to cut rc.24 is **security**: the range clears **five RUSTSEC advisories** (two of them affecting the production WASM sandbox runtime — a `FilePerms` host-filesystem-escape and a cross-engine type-confusion) and adds a **SHA-pinned `cargo deny check advisories` CI gate** so the advisory list can never silently drift again. These fixes exist on `develop` today (`Cargo.lock` confirms `wasmtime`/`wasmtime-wasi` at `46.0.2`) but are **not yet in operators' hands** — the marketplace tarball at rc.23 still ships the vulnerable `44.0.x` runtime. Additionally, the **fuel-cap raise 10M→20M** (ADR-042) is committed but by design **only becomes effective in a cut binary** — it is inert at the operator level until rc.24 ships. Both are ship-motivating: releasing is the delivery mechanism.

The **WITH-NOTES** qualifier is required for three reasons, none of which is a defect in shipped code:
1. **develop's HEAD `ci` run currently concludes `failure`.** The failures are isolated to the `bats-full-suite (linux)` job and are both non-product: (a) an environment gap where `cargo metadata --locked` / `cargo-deny` are unavailable inside the bats job (the authoritative `cargo-host` and `deny-advisories` jobs both pass GREEN and confirm the dependency resolution), and (b) a live-factory-artifacts snapshot test drifting because story `S-21.16` is in STORY-INDEX but not yet in `sprint-state.yaml` (mid-cycle bookkeeping). Per `RELEASING.md` Step 0, develop `ci` must read `success` before cutting — so these need a fix or a documented waiver first.
2. **`plugin.json` is still at `1.0.0-rc.23`** — the version bump + CHANGELOG drain are the mechanical release steps still outstanding (the `## [Unreleased]` section is already fully written, which is good).
3. Several **HIGH-severity security items remain OPEN** (`[C-1]`/`[C-2]`/`[C-4]` exec_subprocess CWE findings; ADR-043 not ratified) and a **`[P0-followup]` POLICY 15 branch-protection** gap persists. These are **carry-forward**, not release-blocking — they predate rc.23, are unrelated to the net-new content in this range, and require human/architect action.

Net: this is a **security-and-hardening release**. Nothing in the 51 commits is a breaking change to the hook API, registry schema semantics, or skill signatures. The only work to actually cut it is the standard `RELEASING.md` bump/drain/PR-to-main flow, plus resolving (or explicitly waiving) the two non-product bats-full-suite failures.

---

## 2. Per-category commit breakdown (51 total)

| Category | Count | Notable commits (SHA / PR) |
|----------|------:|----------------------------|
| **SECURITY** | 2 | `97fb07fa` #781 (wasmtime 44→46.0.2 + cargo-deny gate, 5 advisories); `700b4dd3` #770 (wasmtime-wasi 44.0.1→44.0.3, CVE-2026-47261) |
| **FEATURES** (new gates/capabilities) | 7 | `27c56c01` #780 (S-21.10 failure_policy schema); `19cb57e6` #777 (POLICY 15 gate crate); `e94767bc` #776 (S-21.07 cross-site-correspondence hook); `2e8087af` #775 (S-21.09 path-staging restore + parity gate); `ebf9fb6d` #761 (S-21.03 trunk-assertion); `a4a79f09` #760 (S-21.02 post-rebase diff-integrity gate); `7bb0e797` #759 (S-21.01 path-staging guard) |
| **BUG FIXES** (dispatcher/hooks/config/obs) | 15 | `62fbcf1a` #774 (fuel cap 10M→20M + block_reason disambiguation); `a6a15e1d` #779 (policy15 empty-range inert-skip); `948f0fb1` #763 (E-21 W1 wave-gate follow-ups); `651db073` #738; `4445ce19` #728; `c6fa6eee` #722; `144b797f` #721; `10565565` #719; `5648164d` #731; `a7280b7c` #730; `8a397904` #726; `7025537c` #716; `ae263781` #742; `b6231a88` #720; `00ee5576` (#524, red-gate-log removal) |
| **TOOLING/HOOKS** (agent/skill/workflow/bin/template behavior) | 19 | `d3d5f232` #532 (adversary ground-truth verification); `dc110e03` #531 / `c7e3ab9f` #530 / `aaa8d754` #529 (TDD-agent hardening); `97723f06` #739 (orchestrator startup tools); `188ae734` #735 / `d50922c2` #737 / `be80c250` #727 / `9d3abda0` #526 (factory-worktree resolution); `15a85f43` #718 / `e628b884` #715 (compute-input-hash); config-path registrations #754/#723/#717; `aa594c9a` #714 (CHANGELOG story-template task); demo/test-writer fixes #528/#691 |
| **DEPENDENCIES** (JS, non-prod) | 2 | `e9f56d73` #744 (dompurify 3.4.6→3.4.12); `88054fe7` #745 (immutable 4.3.8→4.3.9) |
| **DOCS / TESTS** | 3 | `9f23ebdd` #740 (state-manager idempotency docs); `26508e83` #743 / `8f17eea1` #725 (live-snapshot test skips) |
| **CI/CD** | 1 | `84a441a0` #778 (wire POLICY 15 gate as required-check job) |
| **MERGE** | 2 | `69e8de30` (#524); `584b0518` (sync main→develop post-rc.23 bundle) |

> Note: category boundaries between "bug fix" and "tooling/hooks" are judgment calls (many `fix(hooks:)` commits harden validator behavior). Counts sum to 51.

---

## 3. Security posture — the strongest ship argument

The security improvement vs rc.23 is **material and multi-layered.**

### 3.1 Advisories cleared (5)

| Advisory | Component | old → new | Nature | PR |
|----------|-----------|-----------|--------|----|
| RUSTSEC-2026-0188 / CVE-2026-58494 | `wasmtime` (WASM sandbox runtime) | 44.0.x → **46.0.2** | `FilePerms` capability bypass — WASM guest could read/overwrite **host files** via hard-link + rename circumventing the capability check | #781 |
| RUSTSEC-2026-0222 | `wasmtime` | 44.0.x → **46.0.2** | cross-engine type-index confusion when a module is shared across independently-created `Engine`s | #781 |
| RUSTSEC-2026-0204 | `crossbeam-epoch` (transitive, async runtime) | 0.9.18 → 0.9.20 | invalid pointer deref in GC epoch mechanism under concurrency | #781 |
| RUSTSEC-2026-0190 | `anyhow` | → 1.0.104 | unsoundness in `Error::downcast_mut` (wrong concrete-type mutable ref) | #781 |
| RUSTSEC-2025-0052 | `httpmock`/`async-std` (**dev-dependency only**) | 0.7 → 0.8.3 | unmaintained `async-std` dropped; test-binary only, no production path | #781 |
| CVE-2026-47261 / GHSA-2r75-cxrj-cmph | `wasmtime-wasi` | 44.0.1 → 44.0.3 (earlier), now 46.0.2 | WASI adapter fix (landed first in #770, superseded by the 46.0.2 bump) | #770 |

The two `wasmtime` advisories are the significant ones: the dispatcher runs **every hook plugin** inside this sandbox, so a `FilePerms` host-filesystem escape is directly on the operator's critical path. `Cargo.lock` on develop confirms both `wasmtime` and `wasmtime-wasi` resolve to **46.0.2**.

### 3.2 New standing defense

A **SHA-pinned `cargo deny check advisories` CI job** (`deny-advisories`, confirmed GREEN on HEAD) now runs on every PR to `develop`/`main`, gating merges on new advisories against the full dependency tree. This closes the systemic gap where the five advisories above sat in the tree across multiple rc cycles undetected. `deny.toml` is present at repo root.

### 3.3 Assessment

Security posture is **materially improved vs rc.23** and is the primary justification for cutting rc.24: the fixes are real, verified in `Cargo.lock`, CI-gated going forward, and **undelivered to operators until a release is cut** (the marketplace cache still ships the vulnerable 44.0.x runtime). This is a delivery-motivated release.

---

## 4. Risk / breaking-changes analysis

| Item | Type | Operator impact | Blocking? |
|------|------|-----------------|-----------|
| **Fuel cap 10M → 20M** (`62fbcf1a` #774, ADR-042) | Behavior change (limits) | **Effective only in the cut binary** — inert at operator level until rc.24 ships. Reduces `FUEL_EXHAUSTED` false-blocks on large cycle artifacts. **Caveat (per CLAUDE.md): 20M does NOT eliminate exhaustion** on the largest artifacts (`decision-log.md`, `burst-log.md`, `lessons.md`); size budgets + compaction remain the real remedy. Also adds fuel-vs-epoch `block_reason` disambiguation so operators can tell `FUEL_EXHAUSTED` from timeout. | No — strictly permissive |
| **`failure_policy` registry schema extension** (`27c56c01` #780, ADR-039 Phase 1) | Schema addition | New optional per-plugin `failure_policy` TOML field (`fail-closed`\|`fail-open`); **absent field defaults to `fail-open`** (= current behavior). Unknown values rejected at parse time. **Schema only — no enforcement change this phase.** Backward-compatible. | No |
| **New WASM hook plugins** (cross-site-correspondence #776, path-staging #775/#759, post-rebase diff-integrity #760, POLICY 15 gate #777) | New gates | New PostToolUse/PreToolUse validators fire on `.factory/` writes and merge pre-checks. Could surface **new advisory/block signals** on operator factory workflows that previously passed silently. All are `.factory/`-governance-scoped; no effect on non-factory consumers. | No — but exercise on a real factory session before broad rollout |
| **POLICY 15 gate wired as required-check** (`84a441a0` #778) | CI change | Affects this repo's PR gating only; not shipped to operators. | No |
| **Registry bundle count** | Bundle | New WASM plugins added to `hooks-registry.toml`; the bundle-count floor guard enforces parity. Operators receive additional plugins. | No |
| **JS dep bumps** (dompurify, immutable) | Dependency | Confined to `skills/visual-companion`; no dispatcher/runtime impact. | No |
| **Hook API / skill signatures** | — | **No breaking changes** to hook ABI, registry schema semantics, or skill invocation signatures in this range. | No |

**Overall risk: LOW.** The changes are additive (new gates, permissive limit raise, optional schema field, security bumps). The main operator-facing behavioral novelty is that new `.factory/` governance gates may emit signals they didn't before — recommend one real factory-session smoke test before announcing.

---

## 5. Dependency delta

| Dependency | old → new | Class | Driver |
|------------|-----------|-------|--------|
| `wasmtime` | 44.0.x → **46.0.2** | production (sandbox runtime) | security (RUSTSEC-2026-0188, -0222) |
| `wasmtime-wasi` | 44.0.1 → 44.0.3 → **46.0.2** | production (WASI adapter) | security (CVE-2026-47261; then major bump) |
| `crossbeam-epoch` | 0.9.18 → 0.9.20 | transitive (async runtime) | security (RUSTSEC-2026-0204) |
| `anyhow` | → 1.0.104 | production (error propagation) | security (RUSTSEC-2026-0190) |
| `httpmock` | 0.7 → 0.8.3 | **dev-dependency** | security (RUSTSEC-2025-0052; drops async-std) |
| `dompurify` | 3.4.6 → 3.4.12 | JS (skills/visual-companion) | dependabot |
| `immutable` | 4.3.8 → 4.3.9 | JS (skills/visual-companion) | dependabot |

The `wasmtime` 44→46 major bump is the headline; verified resolved in `Cargo.lock`.

---

## 6. CI / test status

- **Authoritative gates GREEN on HEAD (`27c56c01`, run 32050509355):** `cargo-host (ubuntu-latest)`, `cargo-host (macos-latest)`, `deny-advisories`, all 5 `build-dispatcher` platform legs (darwin-arm64/x64, linux-x64/arm64, windows-x64), `validate`, `platforms-drift`, `bats-darwin-leg (macos)`, `bats-wave-handoff (macos)`, `attestation-gate-non-vacuity-controls`.
- **`ci` workflow overall = `failure`**, isolated to **`bats-full-suite (linux)`**, from exactly **two** `not ok` tests — **neither a shipped-code defect:**
  1. `not ok 3 — AC-003: cargo metadata --locked resolves wasmtime-wasi to >= 46.0.2` (`s21-12-version-and-deny-gate.bats:112`). Fails with `jq: parse error` because `cargo metadata --locked` did not produce JSON **in the bats job environment** (which lacks the full cargo/cargo-deny toolchain — the sibling AC-004 test explicitly `skip`s with "cargo-deny not installed"). The **actual** version resolution is correct and is proven GREEN by the `cargo-host` + `deny-advisories` jobs. This is a **test-harness environment miswiring**, not a version problem.
  2. `not ok 12 — test_real_production_file_completeness_and_status_fidelity` (`sprint-state-format.bats:1394`, BC-5.41.004 PC4). Fails because `S-21.16` appears in STORY-INDEX (non-retired) but not in `sprint-state.yaml`. This is a **live factory-artifacts snapshot** assertion tripping on **mid-cycle bookkeeping** (E-21 Wave-7 is in flight / PAUSED per STATE.md v8.65), not on release code. Related skips exist (#743, #725) but this particular live-data assertion is not guarded.
- **Known test-skip flags / flaky areas** relevant to a release call: `VSDD_SKIP_PRODUCTION_STATE_MD_TEST` (removed in #776, production validators now always run), live `STATE.md` snapshot skips gated on worktree mount (#743), live `sprint-state.yaml` assertions skipped under CI by design (#725) — yet the PC4 completeness assertion above still runs and fails on live data; `jq`/`cargo-deny` availability differences between the `bats-full-suite` and `cargo-host` jobs.

**Consequence for release:** Per `RELEASING.md` Step 0, `gh run list --workflow=ci --branch=develop --limit=1` must return `success`; today it returns `failure`. Even though both failures are non-product, the release should be cut only after either (a) fixing the two bats tests (guard AC-003 behind cargo-metadata availability like AC-004 already is; skip/refresh the live sprint-state completeness assertion), or (b) recording an explicit human waiver documenting that the authoritative `cargo-host`/`deny-advisories`/`build-dispatcher` legs are green. Note the last fully-green `ci` push run was #779 (`a6a15e1d`); the AC-003 failure was introduced by the S-21.12 test itself (#781) and the sprint-state failure is live-data drift.

---

## 7. CHANGELOG + release-process readiness

- **`CHANGELOG.md` `## [Unreleased]` is already fully populated** with structured `### Added / ### Fixed / ### Security` entries covering S-21.07, S-21.09, S-21.10, and the S-21.12 security clearances (incl. all five advisory IDs). This is the strongest process-readiness signal — the CHANGELOG story-template task (#714) is working. **The Security subsection is release-quality as written.**
- **`.factory/release-config.yaml` exists** (schema 1, unified strategy). One known vestigial issue: `global_version_sources` still cites the removed root `.claude-plugin/marketplace.json` — documented as benign in the rc.23 CHANGELOG "Deferred" (FINDING-A; `release.yml` reads the version from the git tag, not this file). Not blocking.
- **`plugin.json` version = `1.0.0-rc.23`** — not yet bumped.
- **`RELEASING.md`** is the canonical procedure and is intact.

### Steps remaining to actually cut rc.24
1. **Resolve or waive** the two `bats-full-suite (linux)` failures so develop `ci` reads `success` (Step 0 gate).
2. `git checkout -b release/v1.0.0-rc.24 origin/develop` (branch from develop, per invariant).
3. `scripts/bump-version.sh 1.0.0-rc.24 "<title>"` → bumps `plugin.json` (and README badge).
4. **Drain `## [Unreleased]`** into a `## 1.0.0-rc.24 — <title> (2026-08-25)` heading directly below `## [Unreleased]` (leave `[Unreleased]` present + empty). The content is already written; this is a move, not authoring.
5. Commit CHANGELOG; push `release/v1.0.0-rc.24`.
6. `gh pr create --base main` (guardrail enforces base=main + branch name).
7. Wait for CI green on the release PR, then `gh pr merge <N> --merge --delete-branch` (**`--merge`, never `--squash`**).
8. `git tag -a v1.0.0-rc.24 ... origin/main` at main's new tip; `git push origin v1.0.0-rc.24`.
9. Watch `release.yml` (5-platform binary rebuild + marketplace bump PR at `drbothen/claude-mp`); merge the marketplace PR.

---

## 8. Outstanding items (tagged)

| Item | Severity / source | Disposition |
|------|-------------------|-------------|
| Two `bats-full-suite (linux)` failures (AC-003 env miswiring; sprint-state live-data drift on S-21.16) | CI (this analysis) | **RELEASE-BLOCKING** — gate is `RELEASING.md` Step 0 (develop `ci` must be green). Non-product, so a fix or explicit human waiver clears it. |
| `plugin.json` version bump + CHANGELOG drain | Release mechanics | **RELEASE-BLOCKING** (part of cutting the release itself — see §7 steps 3–4). |
| `[C-1]` CWE-706 exec_subprocess `binary_allow` prefix check inert | HIGH SECURITY (D-972, OPEN 2026-08-11) | **CARRY-FORWARD** — predates rc.23; ADR-043 v1.5 **not ratified** (needs architect/human). Not introduced by this range. |
| `[C-2]` CWE-362 TOCTOU resolve-then-check in exec_subprocess | HIGH SECURITY (D-972, OPEN) | **CARRY-FORWARD** — routed to security-reviewer pre-ADR-043. |
| `[C-4]` CWE-284 arbitrary binary exec via misconfigured prefix list | HIGH SECURITY (D-972, OPEN) | **CARRY-FORWARD** — routed to product-owner + implementer. |
| `[C-5]` CWE-284 no per-entry resource-limit isolation | MEDIUM SECURITY (D-972, OPEN) | **CARRY-FORWARD**. |
| `[P0-followup]` POLICY 15 gate runs but not a REQUIRED status check (branch protection) | OPEN 2026-08-16 — human/admin action | **CARRY-FORWARD** — closes when an admin configures branch protection; repo-governance, not operator-facing. |
| ADR-045 (stable-anchor cross-reference architecture) PROPOSED, human ratification required; Wave-7 pre-TDD cascade HELD | Pipeline state (STATE.md v8.65) | **CARRY-FORWARD** — E-21 Wave-7 is unfinished spec-convergence work; unrelated to the merged, CI-green content in this release range. Does not gate rc.24. |
| ADR-043 not ratified (blocks [C-1]/[C-2] structural fixes) | Architect gate | **CARRY-FORWARD**. |
| `release-config.yaml` vestigial `marketplace.json` cite; dispatcher `dispatcher_version: 0.0.1` telemetry | rc.23 Deferred (FINDING-A/B) | **CARRY-FORWARD** — benign, E-20 devops polish. |

**Key point on the security carry-forwards:** the `[C-1]..[C-5]` exec_subprocess findings are HIGH but concern a *different* subsystem (subprocess binary allow-listing) than what rc.24 fixes (the wasmtime sandbox). They were already open at rc.23 and are gated on ADR-043 ratification (a human decision). They do **not** block rc.24 and shipping rc.24 does **not** worsen them — it strictly improves the sandbox-runtime posture in parallel.

---

## 9. Bottom line

Cut **rc.24**. It delivers a real, verified, previously-undelivered security improvement (5 advisories, sandbox-runtime host-escape fix, new advisory CI gate) plus an operator-facing fuel-cap relief that is inert until a binary is cut. Risk is LOW and additive; no breaking changes. Before tagging: clear the two non-product `bats-full-suite (linux)` failures (fix or waive) so develop `ci` is green per `RELEASING.md` Step 0, then run the standard bump → CHANGELOG-drain → release-PR-to-main → tag flow. Carry the open exec_subprocess HIGH findings and the POLICY 15 branch-protection follow-up forward — they are human/architect-gated and independent of this release.
