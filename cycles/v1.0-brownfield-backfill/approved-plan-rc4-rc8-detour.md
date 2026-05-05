# Approved Plan — post-rc.4 / rc.5..rc.8 detour (HISTORICAL)

> **Status:** COMPLETE. This plan governed Phase A through Phase B-bis.
> Extracted from STATE.md during D-237 state-hygiene burst 2026-05-05.
> Current active plan is in STATE.md (post-rc.11 reality).

## Phase A ✅ COMPLETE — Pre-W16 hardening sprint (PR #65, develop @ 844b0e9)

TD-013/016/017/018 closed. bats-orphan-detection CI gate live; run-all.sh glob-discovery; main branch protection restored; workspace clippy clean; 3 stale .sh dupes deleted.

## Phase B ✅ COMPLETE — v1.0.0-rc.4 cut (PRs #66 + #67; tag e93fef7)

rc.4 shipped 2026-05-03T10:12:41Z. 2 recovery sub-phases: first release.yml failed (TD-016 glob + 4 broken suites); PR #67 skip-list hotfix; force-retagged; second run all 9 jobs SUCCESS after windows-x64 cache-save flake recovered via rerun. TD-020/021 registered.

## Phase B-bis ✅ COMPLETE — Release machinery hardening (rc.5..rc.8; PRs #68/69/70/71/72/73/74/75/76/77)

The `/plugin install` flow had been silently broken since rc.2 — `marketplace.json` used a schema-violating source field (`github + path`), so cache never populated and zero skills loaded. This was masked by the legacy committed `hooks.json` and pre-rc.4 install paths. rc.5..rc.8 cut to fix it end-to-end. Each rc surfaced the next defect under it:

- **rc.5** (PR #68; tag a9b78ce → c87c1a9): collapsed `commands/` shim layer into skills (111 files removed); added `disable-model-invocation: true` to activate/deactivate skills (block model auto-invoke of state-changing entry points).
- **rc.6** (PR #70/71; tag → c87c1a9 → ...): switched marketplace source to `git-subdir + ref=main`. Schema-correct but empirically still broken in self-referential same-repo install.
- **rc.7** (PR #72/73; multiple force-retags after hotfixes #74 and #75):
  - Split marketplace into separate repo (`drbothen/claude-mp`) — fix that actually works
  - Untracked `marketplace.json` from this repo (lives in claude-mp now)
  - Added bump-marketplace auto-PR job (gated by `CLAUDE_MP_PAT` secret)
  - Migrated 2 broken WASM plugins (`block-ai-attribution`, `capture-pr-activity`) from `cdylib` to `[[bin]]` pattern so they emit hyphenated `.wasm` names matching the registry
  - Hotfix #1: dropped invalid `secrets` context from job-level `if:` in release.yml (was preventing all release.yml runs)
  - Hotfix #2: re-tracked `hooks.json` as a workaround when bats suite broke without it (dual tracked+gitignored hack)
- **rc.8** (PR #76/77; tag SHIPPED CLEAN on first attempt 2026-05-04T14:49Z):
  - Migrated 13 bats suites (27 @test blocks) to assert against `hooks-registry.toml` (canonical source) instead of `hooks.json` (per-machine activation output)
  - Added `tests/helpers/registry.bash` with `registry_has_hook` + `registry_has_script` helpers
  - Untracked `hooks.json` cleanly (S-0.4 design intent fully restored)
  - bump-marketplace auto-PR job confirmed working — claude-mp PR #2 auto-opened on rc.8 ship

**End state (after rc.8):**
- All 19 WASM hook plugins load successfully (was 17/19 pre-rc.7)
- `/plugin install vsdd-factory@claude-mp` populates cache correctly (was failing silently since rc.2)
- All 10 hook events fire end-to-end via the dispatcher
- bats suite stable across the activate boundary
- Release pipeline shipped clean on rc.8 (first time since rc.4)

**Lessons codified — see `cycles/v1.0-brownfield-backfill/lessons.md`:**
- `claude plugin validate` only validates plugin.json; does NOT validate marketplace.json source schema or attempt a dry-run install. Workflow-level errors pass through silently.
- `secrets` context is NOT available in job-level `if:` in GitHub Actions — only step-level. Pattern: preflight step that sets a `configured=true|false` output, gate subsequent steps on it.
- `[lib] crate-type = ["cdylib"]` produces underscored .wasm names (Cargo's identifier rule); `[[bin]] name = "hyphenated"` is the only way to get hyphenated wasm output.
- "Same-repo marketplace + plugin at subpath" with `git-subdir` is empirically broken; cross-repo `git-subdir` works (verified by Semgrep + dclaude/wclaude/zclaude).

## Phase C — v1.0.0 GA cut (rc.9..rc.11 extension — SUPERSEDES ORIGINAL rc.8 PLAN)

Originally "~7 days burn-in from rc.4 ship date." That timeline extended through rc.9..rc.11:
- **rc.9** (PRs siblings — PowerShell native ports)
- **rc.10** (skill-body version-ref cleanup)
- **rc.11** (single-commit burst protocol + TD-020 sweep; PRs #89/#90/#91 merged 2026-05-04)

Phase C burn-in clock restarts from rc.11 ship date 2026-05-04. ~7 days → GA cut target ~2026-05-11.

## Phase D — W-16 Spec Foundation

E-9 epic for Tier 2 native WASM migration covering 23 `validate-*.sh` hooks (~7 batched stories). Key decisions: D-9.1 rewrite-clean strategy; D-9.2 subprocess capability; D-9.3 story granularity by capability cluster.

## Phase E — W-16 Implementation

Per-story-delivery per W-15 patterns. Wave gate. Cut v1.1.0.

## Phase F / G — W-17 Spec + Implementation (REVISED)

E-10 epic (ADR-015 single-stream OTel event emission, 9 stories S-10.01..S-10.09) — ELEVATED ahead of E-9 Burst 2 per D-236 (2026-05-05).

## Phase H — v1.3.0 Cleanup

Delete legacy-bash-adapter. All hooks native.
