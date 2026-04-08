---
name: release
description: Release workflow — merge develop to main, tag version, generate changelog, create GitHub release. Only run after convergence check passes (Phase 6 complete).
argument-hint: "[version]"
disable-model-invocation: true
allowed-tools: Read, Write, Edit, Bash, Glob, Grep, AskUserQuestion
---

# Release

Merge `develop` into `main`, tag, and publish a GitHub release.

## Templates

Read and follow the output format in:
- `.claude/templates/release-notes-template.md` — release notes structure
- `.claude/templates/cycle-manifest-template.md` — cycle completion record

## Prerequisites

1. **Convergence check must pass.** Read `.factory/cycles/<current>/convergence-report.md` and verify all 7 dimensions are CONVERGED. If not, abort and tell the user what's missing.

2. **All PRs merged to develop.** No open PRs targeting develop.
   ```bash
   gh pr list --base develop --state open
   ```

3. **All tests pass on develop.**
   ```bash
   git checkout develop && cargo test --release
   ```

## Input

`$ARGUMENTS` — version number (e.g., `1.0.0`). Must follow semver.

If no version provided, read the convergence report cycle name (e.g., `v1.0.0-greenfield`) and extract the version. Ask the user to confirm.

## Process

### 1. Version Validation

- Verify version follows semver (MAJOR.MINOR.PATCH)
- Check no existing tag with this version: `git tag -l "v<version>"`
- If tag exists, abort

### 2. Generate Changelog

Collect changes from develop since last release tag (or since beginning if first release):

```bash
git log main..develop --oneline --no-merges
```

Group by conventional commit type:
- **Features** (feat)
- **Bug Fixes** (fix)
- **Performance** (perf)
- **Breaking Changes** (feat! / fix! / BREAKING CHANGE footer)
- **Other** (docs, refactor, test, chore)

Write to `CHANGELOG.md` (on develop, before merge):

```markdown
# Changelog

## [<version>] - <YYYY-MM-DD>

### Features
- <description> (<commit hash>)

### Bug Fixes
- ...

### Breaking Changes
- ...
```

Commit: `chore: update changelog for v<version>`

### 3. Merge to Main

```bash
git checkout main
git merge develop --no-ff -m "release: v<version>"
```

Use `--no-ff` to preserve the merge commit as a release boundary.

### 4. Tag

```bash
git tag -a "v<version>" -m "Release v<version>"
```

### 5. Update Factory State

Write cycle manifest to `.factory/cycles/<current>/cycle-manifest.md`:
- Stories delivered
- BCs implemented
- VPs verified
- Spec changes
- Lessons learned

Update STATE.md: phase → `release`, pipeline → `COMPLETED`.

Commit factory artifacts.

### 6. Push

**Ask the user before pushing.** Show what will be pushed:

```
Ready to push:
  main     → origin/main (merge commit + tag)
  develop  → origin/develop (changelog)
  v<version> tag

Push now? (y/n)
```

If approved:
```bash
git push origin main develop --tags
```

### 7. Create GitHub Release

```bash
gh release create "v<version>" \
  --title "v<version>" \
  --notes-file CHANGELOG.md \
  --target main
```

### 8. Post-Release

- Tell the user the release URL
- Remind about post-release tasks:
  - Update CLAUDE.md if architecture changed
  - Close related Jira tickets
  - Notify stakeholders

## Output

```
Release v<version> complete:
  Tag:     v<version>
  Release: <GitHub release URL>
  Stories: <N> delivered
  BCs:     <N> implemented
  
  develop and main are now in sync.
  Next cycle starts from develop.
```
