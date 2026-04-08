---
name: release
description: >
  Release pipeline: semver determination, CHANGELOG generation, git tagging,
  GitHub Release creation, registry publishing, and post-release docs update.
---

# Release Pipeline

## When This Skill Runs

After convergence is achieved and human approval is granted. The orchestrator
spawns the devops-engineer to execute the release.

## Prerequisites

- Convergence report shows all dimensions CONVERGED
- Human has approved deployment via the convergence human-approval gate
- `gh` CLI installed and authenticated
- CI/CD release workflow exists at `.github/workflows/release.yml`

## Workflow

### Step 1: Determine Semver Version

Read story types from `STORY-INDEX.md` (or `.factory/stories/`):

```
Version determination rules:
  - Any story with "feat" type       -> MINOR bump
  - Only stories with "fix" type     -> PATCH bump
  - Any story flagged breaking_change -> MAJOR bump
  - First release (no prior tags)    -> 1.0.0 (or 0.1.0 if pre-stable)
```

Check current version:
```bash
git describe --tags --abbrev=0 2>/dev/null || echo "no prior release"
```

Present the proposed version to the orchestrator for confirmation before
proceeding.

### Step 2: Generate CHANGELOG.md

Parse git log since last tag (or initial commit if first release):

```bash
git log $(git describe --tags --abbrev=0 2>/dev/null)..HEAD \
  --pretty=format:"%s (%h)" --no-merges
```

Group entries by commit type:

```markdown
# Changelog

## [vX.Y.Z] - YYYY-MM-DD

### Features
- feat(STORY-NNN): [description] (#PR_NUMBER)

### Bug Fixes
- fix(FIX-P4-NNN): [description] (#PR_NUMBER)

### Security
- fix(FIX-P5-NNN): [description] (#PR_NUMBER)

### Breaking Changes
- [description with migration guide]

### Quality Evidence
- Convergence: 7/7 dimensions CONVERGED after N adversarial passes
- Tests: NNN passing, NN% coverage
- Mutation kill rate: NN%
- Holdout satisfaction: 0.NN
- Formal proofs: N/N verified
- Security findings: 0 CRIT/HIGH
```

Commit the CHANGELOG:
```bash
git add CHANGELOG.md
git commit -m "docs: update CHANGELOG for vX.Y.Z"
```

### Step 3: Tag and Push

```bash
git tag -a vX.Y.Z -m "Release vX.Y.Z: [summary]"
git push origin develop    # CHANGELOG commit
git push origin vX.Y.Z     # tag triggers release.yml workflow
```

### Step 4: Wait for Release CI

```bash
# Wait for the release workflow triggered by the tag push
gh run watch
```

If the release build fails:
1. Diagnose the failure from CI logs
2. Fix via the fix-pr-delivery flow
3. Delete the failed tag, re-tag after fix merges
4. Re-push the tag

### Step 5: Create GitHub Release

```bash
gh release create vX.Y.Z \
  --title "vX.Y.Z: [release title]" \
  --notes-file .factory/release/CHANGELOG-vX.Y.Z.md \
  --attach .factory/demo-evidence/final-journey/*.gif
```

The release includes:
- Release notes from CHANGELOG
- Built binaries (uploaded by release.yml CI)
- Demo GIF from final journey recording
- Convergence summary

### Step 6: Post-Release Updates

1. **Update README.md version badge:**
   ```markdown
   ![Version](https://img.shields.io/badge/version-vX.Y.Z-blue)
   ```

2. **Update installation instructions** (if version-specific):
   ```bash
   cargo install product-name@X.Y.Z
   # or
   npm install product-name@X.Y.Z
   ```

3. **Commit docs update:**
   ```bash
   git add README.md
   git commit -m "docs: update version badge and install instructions for vX.Y.Z"
   git push origin develop
   ```

### Step 7: Registry Publishing (Handled by CI)

The `release.yml` workflow handles publishing to registries:
- **Rust:** `cargo publish` to crates.io
- **Node.js:** `npm publish` to npm
- **Python:** `twine upload` to PyPI
- **Docker:** `docker push` to GHCR

If CI publishing fails, the devops-engineer can publish manually after
diagnosing the issue.

## Release Notes Template

Use `templates/release-notes-template.md` for the GitHub Release body.
Include:
- Highlights (2-3 sentence summary)
- Features, bug fixes, security fixes
- Quality evidence table
- Demo GIF
- Breaking changes with migration guide
- Link to convergence report

## Output Artifacts

| Artifact | Path | Description |
|----------|------|-------------|
| CHANGELOG entry | `CHANGELOG.md` | Versioned release notes |
| Git tag | `vX.Y.Z` | Annotated release tag |
| GitHub Release | `gh release` | Release with binaries + demo evidence |
| README update | `README.md` | Version badge + install instructions |

## Quality Gate Criteria

- [ ] Semver version determined from story types and confirmed by orchestrator
- [ ] CHANGELOG.md generated and committed
- [ ] Git tag created with annotated message
- [ ] release.yml CI passed
- [ ] GitHub Release created with binaries and demo evidence
- [ ] README.md version badge updated
- [ ] Registry publishing succeeded (if applicable)
