# Release Infrastructure Design — vsdd-factory

**Date:** 2026-04-13
**Status:** Approved
**Scope:** vsdd-factory infrastructure + release skill rewrite + secops-factory alignment
**Prerequisite:** secops-factory release infrastructure (completed, validated through v0.5.2)

## Problem

vsdd-factory is at v0.10.2 with 18 commits but has zero git tags, zero GitHub
Releases, and no release workflow. The existing `plugin-validation.yml` CI
covers tests but uses deprecated `actions/checkout@v4`. The marketplace.json
has no version field. The existing `release` skill assumes a full VSDD pipeline
and cannot be used for non-VSDD projects or the plugin itself.

Additionally, secops-factory tracks `.factory/` directly on main, which is
inconsistent with vsdd-factory's pattern of using a `factory-artifacts` orphan
branch. Both repos need alignment: `.factory/` should always be a worktree
mounted from `factory-artifacts`, and CI/release workflows must mount it
before reading release config.

## Decisions

- **`.factory/` convention:** Always lives on a `factory-artifacts` orphan
  branch, mounted as a worktree locally and checked out in CI. Both repos
  follow this pattern.
- **CI strategy:** Rename `plugin-validation.yml` to `ci.yml`, bump checkout
  to v6, preserve all existing checks, add factory-artifacts mount step.
- **Release workflow:** Same pattern as secops-factory — tag-triggered with
  version match validation and auto GitHub Release creation. Mounts
  factory-artifacts to read release config for validation.
- **Release config:** Lives at `.factory/release-config.yaml` on the
  `factory-artifacts` branch. Same schema as secops-factory.
- **Release skill:** Rewrite to be config-driven and project-type agnostic.
  Three modes: bootstrap, release, dry-run. Quality gate spectrum from
  `standard` through `vsdd-full`. Skill mounts `.factory/` if not present.
- **Retroactive tags/releases:** All 12 versions get annotated tags and
  GitHub Releases.

## Deliverables

### 0. Align secops-factory `.factory/` to orphan branch pattern

Move secops-factory's `.factory/` contents (specs, plans, release-config,
ENHANCEMENT-ANALYSIS) from main to a `factory-artifacts` orphan branch:

1. Create `factory-artifacts` orphan branch in secops-factory
2. Move `.factory/` contents to the orphan branch
3. Remove `.factory/` from main's tracked files
4. Add `.factory/` to `.gitignore`
5. Update secops-factory CI and release workflows to mount factory-artifacts:
   ```yaml
   - name: Mount factory artifacts
     run: |
       git fetch origin factory-artifacts
       git worktree add .factory origin/factory-artifacts
   ```
6. Verify CI still passes (release config readable from mounted worktree)

### 1. Rename + Extend CI Workflow

Rename `.github/workflows/plugin-validation.yml` to `.github/workflows/ci.yml`.

Changes:
- Bump `actions/checkout` from v4 to v6
- Add factory-artifacts mount step before validation
- Preserve all existing checks: bats tests (hooks + bin + skills), shell
  syntax check, JSON validation (marketplace, plugin, hooks), lobster parsing
- Keep single-job structure (matches existing pattern)

### 2. Create Release Workflow (`.github/workflows/release.yml`)

Same structure as secops-factory v0.5.2 (with the fixed awk extraction).

**Note:** vsdd-factory CHANGELOG uses `## X.Y.Z — Title` format (no brackets,
em-dash, no date) while secops-factory uses `## [X.Y.Z] - Date`. The awk
extraction pattern in release.yml must handle the vsdd-factory format:
match on `## ${VERSION}` (without brackets) instead of `## \[${VERSION}\]`.

- Trigger: `push.tags: ["v*"]`
- Job 1 (validate): mount factory-artifacts, bats tests, shellcheck, lobster
  parsing, tag matches `plugin.json` version, tag matches `marketplace.json`
  version
- Job 2 (release): extract CHANGELOG section, create GitHub Release via
  `softprops/action-gh-release@v2`, prerelease detection via `-` in tag
- Uses `actions/checkout@v6`

### 3. Add Version to marketplace.json

Add `"version": "0.10.2"` to the plugin entry in
`.claude-plugin/marketplace.json`.

### 4. Create Release Config (`.factory/release-config.yaml`)

Lives on the `factory-artifacts` branch. Mounted into `.factory/` by CI
workflows and the release skill at runtime.

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

### 5. Retroactive Git Tags

12 annotated tags on their respective commits:

| Tag       | Commit    | Message                                       |
|-----------|-----------|-----------------------------------------------|
| `v0.1.0`  | `fe38f39` | Initial marketplace                           |
| `v0.2.0`  | `30fd342` | Wave 1: Foundation                            |
| `v0.3.0`  | `7ba1968` | Wave 2: Skill coverage                        |
| `v0.4.0`  | `947f5a0` | Wave 3: Design system                         |
| `v0.5.0`  | `dabbd8c` | Wave 4: Enforcement layer                     |
| `v0.6.0`  | `41a7c99` | Wave 5: Orchestrator                          |
| `v0.7.0`  | `990c54f` | Wave 6: Runtime helpers                       |
| `v0.8.0`  | `2d0e6f1` | Wave 7: Validation infrastructure             |
| `v0.9.0`  | `7eb714a` | Self-ingest remediation                       |
| `v0.10.0` | `864c523` | Commands, hook envelopes, structural tests    |
| `v0.10.1` | `3004317` | Step-file content fill                        |
| `v0.10.2` | `26d1de8` | Template path portability                     |

### 6. GitHub Releases

12 releases via `gh release create`, each with the corresponding CHANGELOG
entry as the release body.

### 7. Release Skill Rewrite

Replace `plugins/vsdd-factory/skills/release/SKILL.md` with a config-driven,
project-type agnostic release skill.

#### Factory Worktree Handling

The release skill must handle the `.factory/` worktree:
- If `.factory/` is already mounted (developer has worktree set up) → use it
- If `.factory/` is not present → attempt to mount from `factory-artifacts`:
  `git worktree add .factory origin/factory-artifacts`
- If no `factory-artifacts` branch exists → bootstrap mode creates it

#### Mode 1: Bootstrap (`/vsdd-factory:release init`)

Triggered when no `.factory/release-config.yaml` exists, or user explicitly
runs `init`.

**Behavior:**
1. Ensure `.factory/` worktree is mounted (create `factory-artifacts` orphan
   branch if it doesn't exist)
2. Scan repo for project markers:
   - `Cargo.toml` → Rust (version in `[package].version`, publish: `cargo publish`)
   - `package.json` → Node.js (version in `version`, publish: `npm publish`)
   - `pyproject.toml` → Python (version in `[project].version`, publish: `twine upload`)
   - `plugins/*/.claude-plugin/plugin.json` → Claude Code plugin (no registry)
   - `go.mod` → Go (version from git tags only)
3. Detect version strings, test scripts, CI workflows, CHANGELOG
4. Propose `release-config.yaml` based on findings
5. Present to user for confirmation/edits
6. Write config to `.factory/release-config.yaml` on `factory-artifacts` branch
7. Commit to `factory-artifacts`

#### Mode 2: Release (`/vsdd-factory:release [version]`)

Default mode when `.factory/release-config.yaml` exists.

**Version determination:**
1. Explicit version from user → use it
2. `.factory/stories/` with typed story files → derive (feat→MINOR, fix→PATCH,
   breaking→MAJOR)
3. Otherwise → prompt user for bump type (major/minor/patch/explicit)

**Execution sequence:**
1. Ensure `.factory/` worktree is mounted
2. Read `.factory/release-config.yaml`
3. Check quality gates (if `vsdd-full` or `vsdd-partial`):
   - Convergence reports vs `min_convergence_dimensions`
   - Holdout satisfaction vs `min_holdout_satisfaction`
   - Formal verification status
   - Adversarial pass count vs `require_adversarial_passes`
   - Human approval gate
   - Abort with specifics if any gate fails
4. Run `pre_release` checks — abort if any fails
5. Bump all `version_sources` (per-package + global) on main
6. Update CHANGELOG entry on main (keep existing content, add date; generate
   from git log if no entry exists)
7. Commit to main: `chore: release vX.Y.Z`
8. Create annotated git tag
9. Push main + tag
10. If `ci_workflow` set, watch workflow run and report status
11. Verify/create GitHub Release
12. Run `publish` commands for each package (if configured)
13. Each step confirms before executing (skip with `--yes`)

#### Mode 3: Dry Run (`/vsdd-factory:release --dry-run`)

Runs full Mode 2 flow but only prints actions. No files modified, no git
operations.

#### Quality Gate Modes

| Mode           | Gates                                            | Use Case              |
|----------------|--------------------------------------------------|-----------------------|
| `standard`     | `pre_release` commands only                      | Plugins, simple repos |
| `vsdd-partial` | Convergence + selected subset                    | Partial VSDD adoption |
| `vsdd-full`    | Convergence, holdout, formal verification,       | Full VSDD pipeline    |
|                | adversarial passes, human approval               |                       |

#### Removed from Current Skill

- Hard dependency on `STORY-INDEX.md` and `develop` branch
- Demo GIF attachment
- Hardcoded registry commands (`cargo publish`, `npm publish`, etc.)
- Quality evidence section in CHANGELOG template

#### Preserved from Current Skill

- Semver determination from story types
- CHANGELOG generation from git log
- Git tag + push flow
- GitHub Release creation
- Post-release README updates
- CI wait-and-watch

### 8. Test Release (v0.10.3)

Bump all version sources to 0.10.3, add CHANGELOG entry covering this
infrastructure work, commit, tag, push — validates the full release pipeline
end-to-end (same pattern as secops-factory v0.5.2 validation).

## Release Config Schema Reference

Same schema as secops-factory spec. Both repos use identical schema version 1.
The only difference is where the config file lives:
- Both repos: `.factory/release-config.yaml` on `factory-artifacts` branch
- Accessed via worktree mount (local) or `git worktree add` (CI)

## Out of Scope

- CI workflow templates/seeding mechanism for new repos
- Applying release config to jira-cli
- Marketplace pinning to specific refs

## Implementation Order

Phase A — secops-factory alignment:
1. Create `factory-artifacts` orphan branch in secops-factory
2. Move `.factory/` contents from main to `factory-artifacts`
3. Remove `.factory/` from main, add to `.gitignore`
4. Update secops-factory CI + release workflows with mount step
5. Push, verify CI passes

Phase B — vsdd-factory infrastructure:
6. Rename `plugin-validation.yml` to `ci.yml`, bump checkout, add mount step
7. Create `release.yml` with mount step
8. Add version to `marketplace.json`
9. Create `.factory/release-config.yaml` on `factory-artifacts`
10. Commit all main-branch changes, push
11. Create 12 retroactive tags, push
12. Create 12 GitHub Releases

Phase C — release skill rewrite:
13. Rewrite `release` skill SKILL.md
14. Commit, push

Phase D — validation:
15. Bump to v0.10.3, tag, push — validate full pipeline
16. Verify CI + Release workflows pass, release body correct
