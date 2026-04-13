# Release Infrastructure Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add release infrastructure to vsdd-factory (CI, release workflow, tags, releases, release config, skill rewrite) and align secops-factory's `.factory/` to the factory-artifacts orphan branch pattern.

**Architecture:** Four sequential phases. Phase A aligns secops-factory. Phase B adds vsdd-factory infrastructure. Phase C rewrites the release skill. Phase D validates end-to-end.

**Tech Stack:** GitHub Actions, bats, shellcheck, jq, yq, gh CLI, softprops/action-gh-release@v2, git worktrees

**Spec:** `vsdd-factory/.factory/specs/2026-04-13-release-infrastructure-design.md` (on `factory-artifacts` branch)

---

## File Map

### Phase A — secops-factory alignment (working directory: `/Users/jmagady/Dev/secops-factory`)

| Action | File | Purpose |
|--------|------|---------|
| Modify | `.gitignore` | Add `.factory/` |
| Modify | `.github/workflows/ci.yml` | Add factory-artifacts mount step |
| Modify | `.github/workflows/release.yml` | Add factory-artifacts mount step |

### Phase B — vsdd-factory infrastructure (working directory: `/Users/jmagady/Dev/vsdd-factory`)

| Action | File | Purpose |
|--------|------|---------|
| Rename | `.github/workflows/plugin-validation.yml` → `ci.yml` | Consistency + bump checkout |
| Create | `.github/workflows/release.yml` | Tag-triggered release |
| Modify | `.claude-plugin/marketplace.json` | Add version field |

### Phase C — release skill (working directory: `/Users/jmagady/Dev/vsdd-factory`)

| Action | File | Purpose |
|--------|------|---------|
| Rewrite | `plugins/vsdd-factory/skills/release/SKILL.md` | Config-driven release skill |

### Phase B (factory-artifacts branch)

| Action | File | Purpose |
|--------|------|---------|
| Create | `release-config.yaml` | Declarative release manifest |

---

## Phase A: Align secops-factory `.factory/` to orphan branch

### Task 1: Create factory-artifacts orphan branch in secops-factory

**Working directory:** `/Users/jmagady/Dev/secops-factory`

- [ ] **Step 1: Create the orphan branch with .factory contents**

```bash
cd /Users/jmagady/Dev/secops-factory

# Create orphan branch
git checkout --orphan factory-artifacts

# Remove everything from staging
git rm -rf .

# Copy .factory contents back from the main branch
git checkout main -- .factory/

# Move contents up one level (orphan branch root = .factory/ contents)
mv .factory/* .
mv .factory/.* . 2>/dev/null || true
rmdir .factory

# Commit
git add -A
git commit -m "factory: initialize factory-artifacts from main .factory/"

# Go back to main
git checkout main
```

- [ ] **Step 2: Verify the orphan branch exists with correct contents**

```bash
git log --oneline factory-artifacts
```

Expected: Single commit with specs/, plans/, release-config.yaml, ENHANCEMENT-ANALYSIS.md

- [ ] **Step 3: Mount the worktree**

```bash
# Remove the tracked .factory directory from main's working tree
# (we'll add it back as a worktree)
rm -rf .factory

# Mount the orphan branch
git worktree add .factory factory-artifacts
```

- [ ] **Step 4: Verify worktree contents**

```bash
ls .factory/
```

Expected: ENHANCEMENT-ANALYSIS.md, plans/, release-config.yaml, specs/

- [ ] **Step 5: Remove .factory from main's git tracking**

```bash
# Remove .factory/ from git's index (main branch) but keep the worktree files
git rm -rf --cached .factory/
```

- [ ] **Step 6: Add .factory/ to .gitignore**

Add `.factory/` to `/Users/jmagady/Dev/secops-factory/.gitignore`. The file currently contains only `.reference/`. Add `.factory/` before it:

```
.factory/
.reference/
```

- [ ] **Step 7: Commit the removal + gitignore on main**

```bash
git add .gitignore
git commit -m "chore: move .factory to factory-artifacts orphan branch

.factory/ contents now live on the factory-artifacts branch, mounted
as a git worktree. CI and release workflows mount it before reading
release config."
```

- [ ] **Step 8: Push both branches**

```bash
git push origin main
git push origin factory-artifacts
```

---

### Task 2: Update secops-factory CI workflow with factory-artifacts mount

**Working directory:** `/Users/jmagady/Dev/secops-factory`

**Files:**
- Modify: `.github/workflows/ci.yml`

- [ ] **Step 1: Add mount step to each job**

In `.github/workflows/ci.yml`, add the factory-artifacts mount step after each `actions/checkout@v6` step. The mount step is:

```yaml
      - name: Mount factory artifacts
        run: |
          git fetch origin factory-artifacts
          git worktree add .factory origin/factory-artifacts
```

Add this step to all three jobs: `test`, `structure`, and `shellcheck` — after the checkout step in each.

- [ ] **Step 2: Verify YAML syntax**

Run: `python3 -c "import yaml; yaml.safe_load(open('.github/workflows/ci.yml'))"`
Expected: No output (valid YAML).

- [ ] **Step 3: Commit**

```bash
git add .github/workflows/ci.yml
git commit -m "ci: mount factory-artifacts worktree in CI workflow"
```

---

### Task 3: Update secops-factory release workflow with factory-artifacts mount

**Working directory:** `/Users/jmagady/Dev/secops-factory`

**Files:**
- Modify: `.github/workflows/release.yml`

- [ ] **Step 1: Add mount step to both jobs**

In `.github/workflows/release.yml`, add the factory-artifacts mount step after each `actions/checkout@v6` step. Same step as Task 2:

```yaml
      - name: Mount factory artifacts
        run: |
          git fetch origin factory-artifacts
          git worktree add .factory origin/factory-artifacts
```

Add to both the `validate` and `release` jobs.

- [ ] **Step 2: Verify YAML syntax**

Run: `python3 -c "import yaml; yaml.safe_load(open('.github/workflows/release.yml'))"`
Expected: No output (valid YAML).

- [ ] **Step 3: Commit**

```bash
git add .github/workflows/release.yml
git commit -m "ci: mount factory-artifacts worktree in release workflow"
```

---

### Task 4: Push and verify secops-factory CI

**Working directory:** `/Users/jmagady/Dev/secops-factory`

- [ ] **Step 1: Push to main**

```bash
git push origin main
```

- [ ] **Step 2: Watch CI**

```bash
gh run watch --exit-status
```

Expected: All 3 CI jobs pass. The mount step should succeed since `factory-artifacts` branch now exists on remote.

- [ ] **Step 3: Test release workflow still works**

Bump version to 0.5.3, update all version sources, commit, tag, push, verify release creates correctly:

```bash
# Bump plugin.json
jq '.version = "0.5.3"' plugins/secops-factory/.claude-plugin/plugin.json > /tmp/pj && mv /tmp/pj plugins/secops-factory/.claude-plugin/plugin.json

# Bump marketplace.json
jq '.plugins[0].version = "0.5.3"' .claude-plugin/marketplace.json > /tmp/mj && mv /tmp/mj .claude-plugin/marketplace.json

# Bump README badge (sed for regex replacement)
sed -i '' 's/version-0\.5\.2-green/version-0.5.3-green/' README.md

# Add CHANGELOG entry
sed -i '' '/^## \[0\.5\.2\]/i\
## [0.5.3] - 2026-04-13\
\
Align .factory to factory-artifacts orphan branch pattern.\
\
### Changed\
- .factory/ contents moved to factory-artifacts orphan branch\
- CI and release workflows mount factory-artifacts worktree\
- .factory/ added to .gitignore\
' CHANGELOG.md

git add -A
git commit -m "chore: release v0.5.3 — factory-artifacts alignment"
git tag -a v0.5.3 -m "v0.5.3: Factory-artifacts alignment"
git push origin main --tags
```

- [ ] **Step 4: Watch release workflow**

```bash
gh run list --limit 3
# Find the Release run and watch it
gh run watch <release-run-id> --exit-status
```

Expected: Validate job passes (tag matches plugin.json and marketplace.json). Release job creates GitHub Release with CHANGELOG body.

- [ ] **Step 5: Verify release**

```bash
gh release view v0.5.3
```

Expected: Release with CHANGELOG content, not fallback text.

---

## Phase B: vsdd-factory Infrastructure

### Task 5: Rename CI workflow and bump checkout

**Working directory:** `/Users/jmagady/Dev/vsdd-factory`

**Files:**
- Rename: `.github/workflows/plugin-validation.yml` → `.github/workflows/ci.yml`
- Modify: `.github/workflows/ci.yml`

- [ ] **Step 1: Rename the workflow file**

```bash
cd /Users/jmagady/Dev/vsdd-factory
git mv .github/workflows/plugin-validation.yml .github/workflows/ci.yml
```

- [ ] **Step 2: Bump actions/checkout to v6 and add factory-artifacts mount**

Edit `/Users/jmagady/Dev/vsdd-factory/.github/workflows/ci.yml`:

Replace `actions/checkout@v4` with `actions/checkout@v6`.

Add the factory-artifacts mount step after the checkout step:

```yaml
      - name: Mount factory artifacts
        run: |
          git fetch origin factory-artifacts
          git worktree add .factory origin/factory-artifacts
```

- [ ] **Step 3: Verify YAML syntax**

Run: `python3 -c "import yaml; yaml.safe_load(open('/Users/jmagady/Dev/vsdd-factory/.github/workflows/ci.yml'))"`
Expected: No output.

- [ ] **Step 4: Commit**

```bash
cd /Users/jmagady/Dev/vsdd-factory
git add .github/workflows/
git commit -m "ci: rename plugin-validation to ci, bump checkout to v6

Rename for consistency with secops-factory. Bump actions/checkout from
v4 to v6 (Node.js 20 deprecation). Add factory-artifacts mount step."
```

---

### Task 6: Create vsdd-factory release workflow

**Working directory:** `/Users/jmagady/Dev/vsdd-factory`

**Files:**
- Create: `.github/workflows/release.yml`

- [ ] **Step 1: Write release.yml**

Create `/Users/jmagady/Dev/vsdd-factory/.github/workflows/release.yml`:

```yaml
name: Release

on:
  push:
    tags: ["v*"]

permissions:
  contents: write

jobs:
  validate:
    name: Pre-release Validation
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v6

      - name: Mount factory artifacts
        run: |
          git fetch origin factory-artifacts
          git worktree add .factory origin/factory-artifacts

      - name: Install tools
        run: |
          sudo apt-get update
          sudo apt-get install -y bats
          sudo wget -qO /usr/local/bin/yq https://github.com/mikefarah/yq/releases/latest/download/yq_linux_amd64
          sudo chmod +x /usr/local/bin/yq

      - name: Run test suite
        run: cd plugins/vsdd-factory/tests && ./run-all.sh

      - name: Shellcheck hooks
        run: |
          for f in plugins/vsdd-factory/hooks/*.sh plugins/vsdd-factory/bin/*; do
            shellcheck "$f"
          done

      - name: Validate tag matches plugin.json version
        run: |
          TAG_VERSION="${GITHUB_REF_NAME#v}"
          PLUGIN_VERSION=$(jq -r '.version' plugins/vsdd-factory/.claude-plugin/plugin.json)
          if [ "$TAG_VERSION" != "$PLUGIN_VERSION" ]; then
            echo "::error::Tag $GITHUB_REF_NAME does not match plugin.json version $PLUGIN_VERSION"
            exit 1
          fi

      - name: Validate tag matches marketplace.json version
        run: |
          TAG_VERSION="${GITHUB_REF_NAME#v}"
          MKT_VERSION=$(jq -r '.plugins[0].version // empty' .claude-plugin/marketplace.json)
          if [ -n "$MKT_VERSION" ] && [ "$TAG_VERSION" != "$MKT_VERSION" ]; then
            echo "::error::Tag $GITHUB_REF_NAME does not match marketplace.json version $MKT_VERSION"
            exit 1
          fi

  release:
    name: Create GitHub Release
    needs: validate
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v6

      - name: Extract changelog for this version
        id: changelog
        run: |
          VERSION="${GITHUB_REF_NAME#v}"
          # vsdd-factory CHANGELOG format: ## X.Y.Z — Title (no brackets)
          awk "
            /^## ${VERSION} / { found=1; print; next }
            found && /^## / { exit }
            found { print }
          " CHANGELOG.md > /tmp/release-notes.md
          if [ ! -s /tmp/release-notes.md ]; then
            echo "Release ${GITHUB_REF_NAME}" > /tmp/release-notes.md
          fi

      - name: Create GitHub Release
        uses: softprops/action-gh-release@v2
        with:
          body_path: /tmp/release-notes.md
          prerelease: ${{ contains(github.ref_name, '-') }}
```

- [ ] **Step 2: Verify YAML syntax**

Run: `python3 -c "import yaml; yaml.safe_load(open('/Users/jmagady/Dev/vsdd-factory/.github/workflows/release.yml'))"`
Expected: No output.

- [ ] **Step 3: Commit**

```bash
cd /Users/jmagady/Dev/vsdd-factory
git add .github/workflows/release.yml
git commit -m "ci: add release workflow — validation + GitHub Release on tag push

Triggered by v* tags. Mounts factory-artifacts worktree. Validates tag
matches plugin.json and marketplace.json. Creates GitHub Release with
CHANGELOG excerpt. Handles vsdd-factory ## X.Y.Z format."
```

---

### Task 7: Add version to marketplace.json

**Working directory:** `/Users/jmagady/Dev/vsdd-factory`

**Files:**
- Modify: `.claude-plugin/marketplace.json`

- [ ] **Step 1: Add version field**

Edit `/Users/jmagady/Dev/vsdd-factory/.claude-plugin/marketplace.json`. Add `"version": "0.10.2"` after `"name": "vsdd-factory"` in the plugin entry:

```json
{
  "name": "vsdd-factory",
  "owner": {
    "name": "drbothen"
  },
  "metadata": {
    "description": "VSDD dark factory marketplace — spec-driven SDLC pipeline plugins for Claude Code."
  },
  "plugins": [
    {
      "name": "vsdd-factory",
      "version": "0.10.2",
      "source": "./plugins/vsdd-factory",
      "description": "Full VSDD pipeline: brownfield ingest, spec crystallization, decomposition, TDD delivery, adversarial review, holdout eval, formal verification, release.",
      "category": "sdlc",
      "tags": ["vsdd", "spec-driven", "tdd", "factory"]
    }
  ]
}
```

- [ ] **Step 2: Verify JSON**

Run: `jq . /Users/jmagady/Dev/vsdd-factory/.claude-plugin/marketplace.json`
Expected: Valid JSON with version field.

- [ ] **Step 3: Commit**

```bash
cd /Users/jmagady/Dev/vsdd-factory
git add .claude-plugin/marketplace.json
git commit -m "chore: add version field to marketplace.json"
```

---

### Task 8: Create release config on factory-artifacts

**Working directory:** `/Users/jmagady/Dev/vsdd-factory/.factory` (factory-artifacts worktree)

**Files:**
- Create: `release-config.yaml` (on factory-artifacts branch)

- [ ] **Step 1: Write the release config**

Create `/Users/jmagady/Dev/vsdd-factory/.factory/release-config.yaml`:

```yaml
schema: 1

project:
  name: vsdd-factory
  strategy: unified

packages:
  - name: vsdd-factory
    path: plugins/vsdd-factory
    version_sources:
      - file: .claude-plugin/plugin.json
        format: json
        path: "version"
    publish: null

global_version_sources:
  - file: README.md
    format: regex
    path: "version-([0-9.]+)-green"
  - file: .claude-plugin/marketplace.json
    format: json
    path: "plugins[0].version"

pre_release:
  - name: "BATS test suite"
    command: "cd plugins/vsdd-factory/tests && ./run-all.sh"
  - name: "Shellcheck hooks"
    command: "shellcheck plugins/vsdd-factory/hooks/*.sh"
  - name: "Plugin structure validation"
    command: "test -f plugins/vsdd-factory/.claude-plugin/plugin.json"
  - name: "Lobster workflow parsing"
    command: |
      for f in plugins/vsdd-factory/workflows/*.lobster plugins/vsdd-factory/workflows/phases/*.lobster; do
        plugins/vsdd-factory/bin/lobster-parse "$f" '.workflow.name' >/dev/null
      done

quality_gates:
  mode: standard

changelog:
  file: CHANGELOG.md
  format: keep-a-changelog

ci_workflow: .github/workflows/release.yml
```

- [ ] **Step 2: Verify YAML syntax**

Run: `python3 -c "import yaml; yaml.safe_load(open('/Users/jmagady/Dev/vsdd-factory/.factory/release-config.yaml'))"`
Expected: No output.

- [ ] **Step 3: Commit on factory-artifacts branch**

```bash
cd /Users/jmagady/Dev/vsdd-factory/.factory
git add release-config.yaml
git commit -m "chore: add release config for vsdd-factory"
```

---

### Task 9: Push all changes

**Working directory:** `/Users/jmagady/Dev/vsdd-factory`

- [ ] **Step 1: Push main branch**

```bash
cd /Users/jmagady/Dev/vsdd-factory
git push origin main
```

- [ ] **Step 2: Push factory-artifacts branch**

```bash
cd /Users/jmagady/Dev/vsdd-factory/.factory
git push origin factory-artifacts
```

- [ ] **Step 3: Verify CI passes**

```bash
cd /Users/jmagady/Dev/vsdd-factory
gh run watch --exit-status
```

Expected: CI passes with the renamed workflow.

---

### Task 10: Create retroactive git tags

**Working directory:** `/Users/jmagady/Dev/vsdd-factory`

- [ ] **Step 1: Create all 12 annotated tags**

```bash
cd /Users/jmagady/Dev/vsdd-factory
git tag -a v0.1.0  fe38f39 -m "v0.1.0: Initial marketplace"
git tag -a v0.2.0  30fd342 -m "v0.2.0: Wave 1 — Foundation"
git tag -a v0.3.0  7ba1968 -m "v0.3.0: Wave 2 — Skill coverage"
git tag -a v0.4.0  947f5a0 -m "v0.4.0: Wave 3 — Design system"
git tag -a v0.5.0  dabbd8c -m "v0.5.0: Wave 4 — Enforcement layer"
git tag -a v0.6.0  41a7c99 -m "v0.6.0: Wave 5 — Orchestrator"
git tag -a v0.7.0  990c54f -m "v0.7.0: Wave 6 — Runtime helpers"
git tag -a v0.8.0  2d0e6f1 -m "v0.8.0: Wave 7 — Validation infrastructure"
git tag -a v0.9.0  7eb714a -m "v0.9.0: Self-ingest remediation"
git tag -a v0.10.0 864c523 -m "v0.10.0: Commands, hook envelopes, structural tests"
git tag -a v0.10.1 3004317 -m "v0.10.1: Step-file content fill"
git tag -a v0.10.2 26d1de8 -m "v0.10.2: Template path portability"
```

- [ ] **Step 2: Verify**

```bash
git tag -l | wc -l
```

Expected: `12`

- [ ] **Step 3: Push tags**

```bash
git push origin --tags
```

---

### Task 11: Create GitHub Releases

**Working directory:** `/Users/jmagady/Dev/vsdd-factory`

- [ ] **Step 1: Create all 12 releases**

The vsdd-factory CHANGELOG uses `## X.Y.Z — Title` format. Extract each section and create releases. Run sequentially:

```bash
cd /Users/jmagady/Dev/vsdd-factory

# v0.1.0 (last entry — extract to end of file)
awk '/^## 0\.1\.0 /{found=1} found{print}' CHANGELOG.md > /tmp/notes.md
gh release create v0.1.0 --title "v0.1.0: Initial marketplace" --notes-file /tmp/notes.md

# v0.2.0
awk '/^## 0\.2\.0 /{found=1; print; next} found && /^## /{exit} found{print}' CHANGELOG.md > /tmp/notes.md
gh release create v0.2.0 --title "v0.2.0: Wave 1 — Foundation" --notes-file /tmp/notes.md

# v0.3.0
awk '/^## 0\.3\.0 /{found=1; print; next} found && /^## /{exit} found{print}' CHANGELOG.md > /tmp/notes.md
gh release create v0.3.0 --title "v0.3.0: Wave 2 — Skill coverage" --notes-file /tmp/notes.md

# v0.4.0
awk '/^## 0\.4\.0 /{found=1; print; next} found && /^## /{exit} found{print}' CHANGELOG.md > /tmp/notes.md
gh release create v0.4.0 --title "v0.4.0: Wave 3 — Design system" --notes-file /tmp/notes.md

# v0.5.0
awk '/^## 0\.5\.0 /{found=1; print; next} found && /^## /{exit} found{print}' CHANGELOG.md > /tmp/notes.md
gh release create v0.5.0 --title "v0.5.0: Wave 4 — Enforcement layer" --notes-file /tmp/notes.md

# v0.6.0
awk '/^## 0\.6\.0 /{found=1; print; next} found && /^## /{exit} found{print}' CHANGELOG.md > /tmp/notes.md
gh release create v0.6.0 --title "v0.6.0: Wave 5 — Orchestrator" --notes-file /tmp/notes.md

# v0.7.0
awk '/^## 0\.7\.0 /{found=1; print; next} found && /^## /{exit} found{print}' CHANGELOG.md > /tmp/notes.md
gh release create v0.7.0 --title "v0.7.0: Wave 6 — Runtime helpers" --notes-file /tmp/notes.md

# v0.8.0
awk '/^## 0\.8\.0 /{found=1; print; next} found && /^## /{exit} found{print}' CHANGELOG.md > /tmp/notes.md
gh release create v0.8.0 --title "v0.8.0: Wave 7 — Validation infrastructure" --notes-file /tmp/notes.md

# v0.9.0
awk '/^## 0\.9\.0 /{found=1; print; next} found && /^## /{exit} found{print}' CHANGELOG.md > /tmp/notes.md
gh release create v0.9.0 --title "v0.9.0: Self-ingest remediation" --notes-file /tmp/notes.md

# v0.10.0
awk '/^## 0\.10\.0 /{found=1; print; next} found && /^## /{exit} found{print}' CHANGELOG.md > /tmp/notes.md
gh release create v0.10.0 --title "v0.10.0: Commands, hook envelopes, structural tests" --notes-file /tmp/notes.md

# v0.10.1
awk '/^## 0\.10\.1 /{found=1; print; next} found && /^## /{exit} found{print}' CHANGELOG.md > /tmp/notes.md
gh release create v0.10.1 --title "v0.10.1: Step-file content fill" --notes-file /tmp/notes.md

# v0.10.2
awk '/^## 0\.10\.2 /{found=1; print; next} found && /^## /{exit} found{print}' CHANGELOG.md > /tmp/notes.md
gh release create v0.10.2 --title "v0.10.2: Template path portability" --notes-file /tmp/notes.md
```

- [ ] **Step 2: Verify**

```bash
gh release list --limit 15
```

Expected: 12 releases, newest first.

- [ ] **Step 3: Spot-check one release**

```bash
gh release view v0.9.0
```

Expected: Full CHANGELOG body with sections, not fallback text.

---

## Phase C: Release Skill Rewrite

### Task 12: Rewrite the release skill

**Working directory:** `/Users/jmagady/Dev/vsdd-factory`

**Files:**
- Rewrite: `plugins/vsdd-factory/skills/release/SKILL.md`

- [ ] **Step 1: Read the current skill for reference**

Read: `/Users/jmagady/Dev/vsdd-factory/plugins/vsdd-factory/skills/release/SKILL.md`

Note the current structure — we're replacing the entire file.

- [ ] **Step 2: Write the new release skill**

Replace the contents of `/Users/jmagady/Dev/vsdd-factory/plugins/vsdd-factory/skills/release/SKILL.md` with:

```markdown
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

- If `release-config.yaml` has unknown `schema` version → abort with message
  suggesting skill update
- If `quality_gates.mode` is unrecognized → abort with list of valid modes
- If a pre-release check fails → show output, abort, suggest fixing and re-running
- If version bump fails (file not found, bad format) → abort with specifics
- If git push fails → suggest checking remote access, do not retry
- If CI fails → show link to failed run, do not re-tag
```

- [ ] **Step 3: Commit**

```bash
cd /Users/jmagady/Dev/vsdd-factory
git add plugins/vsdd-factory/skills/release/SKILL.md
git commit -m "feat: rewrite release skill — config-driven, project-agnostic

Three modes: bootstrap (init), release, dry-run. Reads release config
from .factory/release-config.yaml. Supports quality gate spectrum from
standard through vsdd-full. Works with any project type."
```

- [ ] **Step 4: Push**

```bash
git push origin main
```

---

## Phase D: Validation

### Task 13: Test release (v0.10.3)

**Working directory:** `/Users/jmagady/Dev/vsdd-factory`

- [ ] **Step 1: Bump version to 0.10.3 in all sources**

```bash
cd /Users/jmagady/Dev/vsdd-factory

# plugin.json
jq '.version = "0.10.3"' plugins/vsdd-factory/.claude-plugin/plugin.json > /tmp/pj && mv /tmp/pj plugins/vsdd-factory/.claude-plugin/plugin.json

# marketplace.json
jq '.plugins[0].version = "0.10.3"' .claude-plugin/marketplace.json > /tmp/mj && mv /tmp/mj .claude-plugin/marketplace.json

# README badge
sed -i '' 's/version-0\.10\.2-green/version-0.10.3-green/' README.md
```

- [ ] **Step 2: Add CHANGELOG entry**

Prepend to CHANGELOG.md, before the `## 0.10.2` line:

```markdown
## 0.10.3 — Release infrastructure and CI/CD

### Added
- **Release workflow** (`.github/workflows/release.yml`) — tag-triggered validation + GitHub Release with CHANGELOG excerpt
- **Release config** (`.factory/release-config.yaml`) — declarative release manifest on factory-artifacts branch
- **Release skill rewrite** — config-driven, 3 modes (init/release/dry-run), quality gate spectrum
- Retroactive git tags and GitHub Releases for all 12 prior versions (v0.1.0 through v0.10.2)
- Version field in marketplace.json for release validation
- Factory-artifacts mount step in CI and release workflows

### Changed
- CI workflow renamed from `plugin-validation.yml` to `ci.yml` for cross-repo consistency
- Bump `actions/checkout` from v4 to v6 (Node.js 20 deprecation)

```

- [ ] **Step 3: Commit, tag, push**

```bash
git add -A
git commit -m "chore: release v0.10.3 — release infrastructure + CI/CD"
git tag -a v0.10.3 -m "v0.10.3: Release infrastructure and CI/CD"
git push origin main --tags
```

- [ ] **Step 4: Watch CI and release workflows**

```bash
gh run list --limit 3
# Watch both runs
gh run watch <ci-run-id> --exit-status
gh run watch <release-run-id> --exit-status
```

Expected: Both pass. No Node.js deprecation warnings.

- [ ] **Step 5: Verify release body**

```bash
gh release view v0.10.3
```

Expected: Full CHANGELOG content with Added/Changed sections, not fallback "Release v0.10.3" text.

- [ ] **Step 6: Final counts**

```bash
git tag -l | wc -l
gh release list --limit 15
```

Expected: 13 tags, 13 releases.
