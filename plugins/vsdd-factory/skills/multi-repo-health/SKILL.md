---
name: multi-repo-health
description: Scan `.worktrees/` for a multi-repo project layout and report the detected repos with their manifests. Use before multi-repo phase-0 synthesis.
---

# Multi-Repo Health

Wraps `${CLAUDE_PLUGIN_ROOT}/bin/multi-repo-scan` to detect multi-repo layouts.

## Procedure

1. Run `bin/multi-repo-scan --count`. If 0, report "single-repo project" and stop.
2. Run `bin/multi-repo-scan` (JSON). Parse with `jq`.
3. For each detected repo, report:
   - Name and path
   - Manifest type (Cargo.toml, package.json, pyproject.toml, go.mod, or unknown)
   - Current branch (from `git -C <path> rev-parse --abbrev-ref HEAD`)
   - Git status dirty/clean
4. Cross-check against `.factory/stories/` — are there stories targeting each detected repo? Warn if a repo has no stories, or if there are stories targeting an undetected repo.
5. Recommend next action: if multi-repo detected and no prior ingest, suggest `/vsdd-factory:multi-repo-phase-0-synthesis`; otherwise suggest `/vsdd-factory:next-step`.

Read-only. Does not mutate any repo.
