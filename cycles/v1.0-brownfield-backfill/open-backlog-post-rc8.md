# Open Backlog — post-rc.8 (HISTORICAL SNAPSHOT 2026-05-04)

> **Extracted from STATE.md during D-237 state-hygiene burst 2026-05-05.**
> This was the backlog state as of 2026-05-04 (rc.8 ship). Current active backlog is tracked in tech-debt-register.md and the active cycle's sprint-state.

## Carried over from pre-rc.8 (still open as of 2026-05-04)

- TD-019 ci.yml develop trigger (ci.yml does not run on push to develop; PR-time coverage diverges from release.yml tag-time)
- TD-020 broken bats suites in SKIP_SUITES: codify-lessons, generate-registry, novelty-assessment (2 fail at lines 171/186), state-health — fix-or-delete each; NO new SKIP_SUITES entries without a TD ticket
- TD-021 release.yml fail-fast + cache continue-on-error: set `fail-fast: false` on build-binaries matrix; add `continue-on-error: true` to Cache cargo step
- TD-014 Tier 2/3 retirement (folded into W-16/W-17)
- TD-015 per-invocation telemetry correlation (post-v1.0)
- TD: 1,137 pre-existing STALE input-hashes
- HIGH-W15-001 plugin version drift (1.0.0-rc.1 vs 0.0.1)
- SEC-002/004/005/006 deferred dispositions for v1.0 GA
- Scheduled remote agent 2026-05-22 — independently verifies sync-develop fired correctly on rc.3

## New from Phase B-bis (rc.5..rc.8 work)

- **TD-022 apply-platform.sh bash dependency on Windows.** apply-platform.sh + detect-platform.sh are bash scripts. Windows users need git-bash. Tracked enhancement: rewrite as a `factory-dispatcher activate` subcommand so the dispatcher binary owns activation logic — drops the bash dependency entirely + makes activation truly OS-portable.
- **TD-023 commands-reference.md staleness.** rc.5 deleted the entire `plugins/vsdd-factory/commands/` shim directory (111 files); `docs/guide/commands-reference.md` may still reference patterns from before that. Audit + refresh.
- **TD-024 lessons.md update for Phase B-bis.** Capture in `cycles/v1.0-brownfield-backfill/lessons.md`: (1) `claude plugin validate` doesn't validate marketplace.json source schema; (2) `secrets` context unavailable in job-level `if:`; (3) `[lib] crate-type cdylib` produces underscored wasm names — use `[[bin]] name = "hyphenated"`; (4) same-repo marketplace + git-subdir is empirically broken — cross-repo works.
- **TD-025 generate-marketplace-pr.sh** (optional). The bump-marketplace job duplicates jq + git logic from claude-mp's manual flow. Could extract a small bin/ helper script that both the workflow and an operator-run path can call. Low priority.
- **TD-026 dual maintenance burden.** vsdd-factory release workflow now needs to coordinate with claude-mp's marketplace.json updates. Currently automated via bump-marketplace + CLAUDE_MP_PAT secret. Document the secret rotation policy somewhere (maintainer ops doc).

## Lessons codified during the cycle (needing follow-up in lessons.md)

| Lesson | Where it bit us | Codification |
|---|---|---|
| `claude plugin validate` is plugin-only — it does NOT validate marketplace.json source schema | rc.2..rc.5 shipped a schema-violating `github + path` source that passed validate | Add to release-checklist: "validate" passing is necessary but NOT sufficient; require clean-room install verification |
| `secrets` not available in job-level `if:` | rc.7 hotfix #1 (PR #74) — every release.yml run failed silently with "workflow file issue" | Pattern: preflight step sets output, subsequent steps gate on output |
| Cargo defaults `[lib] name` to underscore-converted package name | block-ai-attribution + capture-pr-activity crashed silently for weeks because registry expected hyphens but cdylib produced underscores | `[[bin]] name = "hyphenated"` is the only path; document in plugin-marketplace-architecture.md (already done) |
| Self-referential same-repo `git-subdir` is broken empirically (works for cross-repo) | rc.6 attempted git-subdir within same repo as marketplace; cache stayed empty | Always split marketplace into a separate repo when nested layout is involved |
| `hooks.json` is per-machine output of activate; tests should assert against `hooks-registry.toml` | rc.7 untracked hooks.json correctly per S-0.4, broke 11 bats suites that asserted against it | Done in rc.8 — bats migrated to registry assertions via `tests/helpers/registry.bash` |
