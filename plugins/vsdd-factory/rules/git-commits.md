<!-- Implementation rules for SOUL.md principles. SOUL owns "why", this file owns "how". -->

# Git Commit Standards

All commits MUST follow [Conventional Commits](https://www.conventionalcommits.org/).

## Format

```
<type>[optional scope]: <description>

[optional body]

[optional footer(s)]
```

## Required: Type

| Type     | Purpose                      |
|----------|------------------------------|
| feat     | New feature (MINOR version)  |
| fix      | Bug fix (PATCH version)      |
| docs     | Documentation only           |
| style    | Code style (no logic change) |
| refactor | Neither fix nor feature      |
| perf     | Performance improvement      |
| test     | Adding/fixing tests          |
| build    | Build system/dependencies    |
| ci       | CI configuration             |
| chore    | Other non-src/test changes   |

## Required: Description

- Use imperative, present tense ("add" not "added")
- Do NOT capitalize the first letter
- Do NOT end with a period

## Optional: Scope

Enclose in parentheses after type: `feat(api): add endpoint`

## Optional: Body

- Separate from description with a blank line
- Explain motivation and contrast with previous behavior

## Optional: Footer

- `Refs: #123` — Issue references
- `Closes: #123` — Issues closed by commit
- `BREAKING CHANGE:` — Breaking change description

## Breaking Changes

Indicate with either:
1. `!` after type/scope: `feat(api)!: remove endpoint`
2. Footer: `BREAKING CHANGE: endpoint removed and replaced with accounts`

## AI Attribution

Do NOT include in commit messages:
- The "Generated with Claude Code" line
- The "Co-Authored-By: Claude" line
- Any other AI attribution

## GitHub Operations

### Admin bypass is never implicit

NEVER use `--admin` flag on `gh pr merge` or any other branch protection bypass unless the user explicitly grants permission for that specific merge in that moment. Prior permission does not carry forward — each use requires fresh explicit approval. Always ask before using `--admin`. This applies to all repositories under 1898andCo.
