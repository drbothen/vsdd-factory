# RC23 Candidate Smoke Test Report

**Date:** 2026-07-18
**Producer:** devops-engineer (authorized by team-lead; D-855 evidence burst)
**Scope:** rc.23-candidate dry-run against develop HEAD `6db4c9fc` (S-19.07 squash-merge 2026-07-17)
**Mode:** DRY-RUN ONLY — no branches, tags, or releases created
**Decision codified:** D-855

---

## Combined Verdict: READY-EXCEPT-KNOWN-BLOCKER

**rc.23-candidate passes all 8 smoke checks with one known, pre-documented blocker.** The blocker (stale linux/windows platform binaries) is resolved automatically by the `release.yml` build-binaries job at tag-push; no pre-release manual action required. Human may authorize the rc.23 cut when ready — all other gate items are PASS.

---

## Check 1 — Release Procedure Readiness

**Verdict: PASS**

- **Next semver:** 1.0.0-rc.23 (CHANGELOG top entry = rc.22; next increment rc.23 is correct).
- **CHANGELOG top entry:** v1.0.0-rc.22 (2026-07-03) — correctly positioned; rc.23 entry will be written at release time.
- **plugin.json version:** bot-written at tag-push time by release.yml step `bump-version`; not manually managed; no stale version risk.
- **release.yml platform matrix:** 5/5 platforms confirmed (darwin-arm64, darwin-x86_64, linux-x86_64, linux-musl, windows-x86_64).
- **TD #69 guardrail:** `.github/workflows/release-branch-guardrail.yml` ACTIVE — release branches MUST be named `release/v<full-semver>` and MUST target `main`.
- **bump-version.sh scope:** CHANGELOG-only (no source file rewrites); safe for conventional-commits workflow.

**Vestigial note (benign — see NEW FINDINGS below):** `release-config.yaml` `global_version_sources` cites a nonexistent root `.claude-plugin/marketplace.json`. This file was removed at rc.7; it now lives in `drbothen/claude-mp`. The citation is benign because `release.yml` derives the release version from the git tag, not from `release-config.yaml` source entries.

---

## Check 2 — POLICY 20 Bundle Hygiene

**Verdict: PASS**

- **WASMs on disk:** 34
- **hooks-registry entries:** 33 (WASM plugins referenced)
- **resolvers-registry WASM:** 1 (`vsdd-context-resolvers.wasm`)
- **Total accounted for:** 33 + 1 = 34 = disk count. Parity confirmed.
- **Orphans (on-disk not referenced):** 0
- **Missing (referenced not on-disk):** 0
- **Dev samples / placeholder stubs:** 0

All POLICY 20 bundle hygiene invariants satisfied.

---

## Check 3 — Committed darwin-arm64 Binary vs Current Registry

**Verdict: PASS**

- **loaded_plugin_count:** 73 (includes `[hooks.capabilities.read_prefix]` from S-19.06/S-19.07 delivery)
- **invoked/completed balance:** 7/7 (all dispatched plugins completed; no abandoned)
- **plugin.crashed:** 0
- **Exit code:** 0
- **Capability gate:** `host::read_prefix` present and loadable in darwin-arm64 binary (rebuilt post-PR #670)
- **hooks-registry.toml parseability on darwin-arm64:** PASS (deny_unknown_fields does NOT fire on rebuilt binary)

---

## Check 4 — Stale Platform Quantification (KNOWN BLOCKER)

**Verdict: CONFIRMED BLOCKER — resolution path documented**

- **linux-x64 committed binary:** rc.22 bundle `a04cb303` (built 2026-07-03 / Jul 4)
- **linux-arm64 committed binary:** rc.22 bundle `a04cb303` (built 2026-07-03 / Jul 4)
- **windows-x64 committed binary:** rc.22 bundle `a04cb303` (built 2026-07-03 / Jul 4)
- **darwin committed binaries:** rebuilt Jul 17 post-PR #670; current source

**Failure mode:** Stale linux/windows binaries contain pre-S-19.06 hook-sdk that uses `deny_unknown_fields` deserialization. When a linux/windows operator's harness loads `hooks-registry.toml` from develop (which declares `[hooks.capabilities.read_prefix]`), the stale binary fails to parse the registry — **total hook-chain outage** on those platforms if consumed directly from develop.

**This is the same blocker documented at D-854 as rc.23 RELEASE-GATE BLOCKER.** Smoke-test confirms the devops characterization.

**Resolution path:** `release.yml` `build-binaries` job rebuilds ALL 5 platform binaries from source at tag-push. Stale committed binaries are replaced by the release pipeline and never ship to operators. **Zero pre-release manual action required.** Human may cut rc.23 at any time; the build pipeline resolves the blocker automatically.

---

## Check 5 — WASM Import Blast Radius

**Verdict: PASS (scoped impact)**

- **WASMs importing `vsdd::read_prefix`:** exactly 1 (`verify-factory-lock.wasm`, rebuilt post-PR #670)
- **Registry entries declaring the capability:** 2
  - `verify-factory-lock` — `path_allow: .factory/STATE.md`
  - `verify-factory-lock-bash` — `path_allow: .factory/STATE.md`
- **Scope:** only `verify-factory-lock` family (2 registrations); no other WASM plugins depend on the new capability
- **Blast radius if capability absent:** only `verify-factory-lock` and `verify-factory-lock-bash` would fail; all other 71 registrations unaffected

---

## Check 6 — Operator-Adjacent Bats vs Fresh `target/release`

**Verdict: 23/23 PASS**

| Suite | Tests | Result |
|-------|-------|--------|
| regression-v1.0 | 9 | PASS |
| read-prefix | 5 | PASS |
| host-abi-hygiene | 9 | PASS |
| **Total** | **23** | **PASS** |

All 23 bats tests pass against the freshly built `target/release` dispatcher binary (darwin-arm64; includes PR #670 `host::read_prefix` implementation). No regression relative to prior release gate.

---

## Check 7 — VSDD_SINK_FILE Opt-In (S-19.05 AC-004)

**Verdict: PASS**

Per S-19.05 AC-004 (release-mode opt-in): `VSDD_SINK_FILE` is honored in **both** debug and release builds.

- **File created:** YES — absolute path provided to sink; file written on dispatch
- **plugin.completed events captured:** 2
- **internal.\* events leaked:** 0 (SEC-003 internal-event exclusion confirmed)

S-19.05 release-mode opt-in behavior confirmed correct.

---

## Check 8 — Plugin Manifest Sanity

**Verdict: PASS**

| Category | Count |
|----------|-------|
| agents | 37 |
| skills | 128 |
| hooks | 49 |
| hook-plugins | 34 |
| hooks-registry entries | 73 |

hooks-registry entry count (73) exceeds `release.yml` floor (30) by a wide margin. All manifest categories at expected production counts. No manifest drift from develop HEAD.

---

## New Findings (Both Benign — E-20 Roster Candidates)

Both findings are pre-existing and do not block rc.23. They are documented here as E-20 roster candidates pending E-20 authorization (D-854 §Pending Human Decisions item 2).

**FINDING-A (benign, devops cleanup):** `release-config.yaml` `global_version_sources` cites a nonexistent root `.claude-plugin/marketplace.json`. This file was removed at rc.7; the canonical location is now `drbothen/claude-mp`. The citation does not affect release.yml execution (which reads version from the git tag directly). Devops cleanup candidate for E-20.

**FINDING-B (benign, polish):** The dispatcher binary emits `"dispatcher_version": "0.0.1"` in internal logs instead of the actual release version. This is a pre-existing limitation (version is not injected at build time from the Cargo workspace version). Affects observability/diagnostics only; no functional impact. Polish candidate for E-20.

---

## Resolution Path Summary

| Item | Status | Action Required |
|------|--------|-----------------|
| Check 1 — Release procedure | PASS | None |
| Check 2 — Bundle hygiene | PASS | None |
| Check 3 — darwin-arm64 binary | PASS | None |
| Check 4 — Stale linux/windows binaries | BLOCKER | **None (release.yml build-binaries auto-resolves at tag-push)** |
| Check 5 — WASM blast radius | PASS | None |
| Check 6 — Bats 23/23 | PASS | None |
| Check 7 — VSDD_SINK_FILE | PASS | None |
| Check 8 — Manifest sanity | PASS | None |
| FINDING-A — vestigial marketplace.json ref | BENIGN | E-20 devops cleanup (not blocking) |
| FINDING-B — dispatcher_version "0.0.1" | BENIGN | E-20 polish (not blocking) |

**Human action required to proceed:** Authorize rc.23 cut (creates `release/v1.0.0-rc.23` branch targeting `main`). The release.yml pipeline handles all remaining steps automatically. See `RELEASING.md` for the canonical procedure.
