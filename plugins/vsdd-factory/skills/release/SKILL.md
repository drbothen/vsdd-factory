---
name: release
description: >
  Config-driven release pipeline: version bump, CHANGELOG update, git tagging,
  GitHub Release, registry publishing. Reads .factory/release-config.yaml.
  Supports bootstrap (init), release, and dry-run modes.
argument-hint: "[init | <version> | --dry-run]"
---

# Release Pipeline

## When This Skill Runs

When the user wants to cut a release, bootstrap release config for a new repo,
or preview what a release would do. Works with any project type — Claude Code
plugins, Rust crates, Node.js packages, Python packages, Go modules.

## Announce at Start

Before any other action, say verbatim:

> **Release Pipeline** — reading release config from `.factory/release-config.yaml`.

## Factory Worktree Handling

Before reading the release config, ensure `.factory/` is available:

1. If `.factory/release-config.yaml` exists → proceed (worktree already mounted)
2. If `.factory/` directory does not exist:
   - Check if `factory-artifacts` branch exists: `git branch -a | grep factory-artifacts`
   - If yes: `git worktree add .factory origin/factory-artifacts`
   - If no: trigger **Bootstrap Mode** (creates the branch)
3. If `.factory/` exists but `release-config.yaml` is missing → trigger **Bootstrap Mode**

## Mode Detection

Parse `$ARGUMENTS`:
- `init` → **Bootstrap Mode**
- `--dry-run` → **Dry Run Mode**
- Anything else (version string or empty) → **Release Mode**

---

## Bootstrap Mode (`/release init`)

Create a release config for a repo that doesn't have one yet.

### Step 1: Scan for Project Markers

Search the repo root for:

| Marker | Project Type | Version Location | Publish Command |
|--------|-------------|-----------------|-----------------|
| `Cargo.toml` | Rust | `[package].version` | `cargo publish` |
| `package.json` | Node.js | `version` | `npm publish` |
| `pyproject.toml` | Python | `[project].version` | `twine upload dist/*` |
| `plugins/*/.claude-plugin/plugin.json` | Claude Code plugin | `version` | none |
| `go.mod` | Go | git tags only | none |

Also detect:
- Test scripts: `run-all.sh`, `Makefile` with `test` target, CI workflow files
- CHANGELOG: `CHANGELOG.md`, `CHANGELOG`, `CHANGES.md`
- CI workflows: `.github/workflows/*.yml`
- Version badges in README: regex `version-([0-9.]+)`
- Marketplace: `.claude-plugin/marketplace.json` with version field

### Step 2: Propose Config

Generate a `release-config.yaml` based on detected markers. Present it to the
user with explanations of each section.

### Step 3: Confirm and Write

Ask the user to review and confirm. On approval:

1. If `factory-artifacts` branch does not exist, create it:
   ```bash
   git checkout --orphan factory-artifacts
   git rm -rf .
   git commit --allow-empty -m "factory: initialize factory-artifacts branch"
   git checkout main
   git worktree add .factory factory-artifacts
   ```
2. Write `release-config.yaml` to `.factory/`
3. Commit on `factory-artifacts`: `chore: add release config`
4. Push `factory-artifacts`

---

## Release Mode (`/release [version]`)

### Step 1: Read Config

Read `.factory/release-config.yaml`. Validate `schema: 1`.

### Step 2: Determine Version

1. If user provided explicit version (e.g., `/release 1.2.0`) → use it
2. If `.factory/stories/` exists with story files containing `type` frontmatter:
   - Any `type: feat` → MINOR bump
   - Only `type: fix` → PATCH bump
   - Any `breaking_change: true` → MAJOR bump
3. Otherwise → ask: "Current version is **X.Y.Z**. What bump? (major / minor / patch / or type explicit version)"

Get current version from the first entry in `packages[0].version_sources`.

Present proposed version and ask for confirmation before proceeding.

### Step 3: Quality Gates

Read `quality_gates.mode`:

**If `standard`:** Skip to Step 4.

**If `vsdd-partial` or `vsdd-full`:** Check each enabled gate:

| Gate | Config Key | Check |
|------|-----------|-------|
| Convergence | `require_convergence` | `.factory/` contains convergence report with >= `min_convergence_dimensions` dimensions CONVERGED |
| Holdout | `require_holdout` | Holdout satisfaction >= `min_holdout_satisfaction` |
| Formal verification | `require_formal_verification` | `.factory/` contains passing formal verification report |
| Adversarial passes | `require_adversarial_passes` | Adversarial review completed >= N passes |
| Human approval | `require_human_approval` | Ask user: "Quality gates passed. Approve release of vX.Y.Z? (yes/no)" |

If any gate fails, report which gate failed, the expected vs actual value,
and abort. Do not proceed to version bumping.

### Step 4: Pre-release Checks

Run each command in `pre_release` list:

```
Running pre-release check: "BATS test suite"...
✓ BATS test suite passed

Running pre-release check: "Shellcheck hooks"...
✓ Shellcheck hooks passed
```

If any check fails, report the failure output and abort.

### Step 5: Bump Versions

For each entry in `packages[].version_sources` and `global_version_sources`:

- `format: json` → use `jq` to update the field at `path`
- `format: toml` → use `sed` or `toml` tool to update the field at `path`
- `format: yaml` → use `yq` to update the field at `path`
- `format: regex` → use `sed` to replace the captured group in `path` pattern

Report each file updated:
```
Bumped plugins/vsdd-factory/.claude-plugin/plugin.json → 0.10.3
Bumped README.md badge → 0.10.3
Bumped .claude-plugin/marketplace.json → 0.10.3
```

### Step 6: Update CHANGELOG

Check if CHANGELOG already has an entry for this version:
- If yes → add today's date if missing, keep existing content
- If no → generate entry from git log since last tag:
  ```bash
  git log $(git describe --tags --abbrev=0 2>/dev/null)..HEAD \
    --pretty=format:"- %s (%h)" --no-merges
  ```
  Group by conventional commit type (feat/fix/chore/docs/ci).

### Step 7: Commit

```bash
git add -A
git commit -m "chore: release vX.Y.Z"
```

### Step 8: Tag

```bash
git tag -a vX.Y.Z -m "vX.Y.Z: <first line of CHANGELOG entry>"
```

### Step 9: Push

Ask for confirmation: "Push commit + tag to origin? (yes/no)"

```bash
git push origin main
git push origin vX.Y.Z
```

### Step 10: Wait for CI

If `ci_workflow` is set in config:

```bash
gh run list --limit 3
# Find the Release workflow triggered by the tag
gh run watch <run-id> --exit-status
```

Report result: "Release workflow passed ✓" or "Release workflow FAILED — check <url>".

### Step 11: Verify GitHub Release

```bash
gh release view vX.Y.Z
```

If the release was created by CI, verify it has CHANGELOG content (not
fallback text). If no CI created the release, create it:

```bash
gh release create vX.Y.Z --title "vX.Y.Z: <title>" --notes-file /tmp/notes.md
```

### Step 12: Publish (if configured)

For each package with `publish` config:

```bash
cd <package.path>
<publish.pre_publish commands>
<publish.command>
```

Report result for each package.

---

## Dry Run Mode (`/release --dry-run`)

Execute the full Release Mode flow but only print what would happen:

```
DRY RUN — no changes will be made.

1. Version bump: 0.10.2 → 0.10.3 (MINOR)
2. Files to update:
   - plugins/vsdd-factory/.claude-plugin/plugin.json: "version" → "0.10.3"
   - README.md: badge → 0.10.3
   - .claude-plugin/marketplace.json: "plugins[0].version" → "0.10.3"
3. CHANGELOG: would generate entry from 3 commits since v0.10.2
4. Commit: "chore: release v0.10.3"
5. Tag: v0.10.3
6. Push: main + v0.10.3
7. CI workflow: .github/workflows/release.yml would be triggered
8. Publish: none (no publish config)

Quality gates: standard (pre_release checks only)
Pre-release checks that would run:
  - BATS test suite
  - Shellcheck hooks
  - Plugin structure validation
  - Lobster workflow parsing
```

---

## Error Handling

- If `release-config.yaml` has unknown `schema` version → abort with message suggesting skill update
- If `quality_gates.mode` is unrecognized → abort with list of valid modes
- If a pre-release check fails → show output, abort, suggest fixing and re-running
- If version bump fails (file not found, bad format) → abort with specifics
- If git push fails → suggest checking remote access, do not retry
- If CI fails → show link to failed run, do not re-tag
