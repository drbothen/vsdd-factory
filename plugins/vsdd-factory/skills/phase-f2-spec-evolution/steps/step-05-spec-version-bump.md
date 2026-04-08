# Step 5: Spec Version Bump

Apply spec versioning rules to determine and record the version bump.

## Inputs

- PRD delta from Step 2
- Architecture delta from Step 3 (if applicable)
- Current spec version
- Spec versioning rules from `workflows/skills/spec-versioning/SKILL.md`

## Actions

1. Determine version bump type:
   - MAJOR: architectural rework, removed features, breaking changes
   - MINOR: new features, new requirements (most common for Feature Mode)
   - PATCH: wording fixes, edge case additions, clarifications
2. Update the spec version in the PRD frontmatter
3. Write a changelog entry to `.factory/spec-changelog.md` using `templates/spec-changelog-template.md`

## Outputs

- Updated spec version in PRD frontmatter
- New changelog entry in `.factory/spec-changelog.md`

## Completion Criteria

- Version bump follows semver rules correctly
- Changelog entry includes: summary, new requirements, modified requirements, impact assessment
