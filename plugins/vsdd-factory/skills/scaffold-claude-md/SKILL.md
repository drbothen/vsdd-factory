---
name: scaffold-claude-md
description: Auto-detect project context and generate a CLAUDE.md at the project root. Detects language, build/test/lint commands, git workflow, and project references. Presents results for confirmation before writing.
---

# Scaffold CLAUDE.md

Generate a project-specific `CLAUDE.md` by inspecting the current working directory. This file gives Claude Code the context it needs to build, test, and navigate this project. It does NOT duplicate anything the vsdd-factory plugin already provides (methodology, principles, rules, agent instructions).

## Pre-flight

1. **Check for existing CLAUDE.md.** If one exists at the project root, show it to the user and ask:
   > "CLAUDE.md already exists. Overwrite with a fresh detection, or cancel?"
   If the user cancels, stop.

## Detection

Run all four detectors. Each produces a section of the final CLAUDE.md. If a detector finds nothing, produce a section with a `<!-- TODO: ... -->` placeholder instead of omitting it.

### Detector 1: Language & Toolchain

Scan the project root for marker files and extract version constraints:

| Marker file | Language | Version source |
|-------------|----------|----------------|
| `Cargo.toml` | Rust | `rust-version` field or `rust-toolchain.toml` |
| `package.json` | Node/TypeScript | `engines.node` field |
| `pyproject.toml` | Python | `requires-python` field |
| `setup.py` or `setup.cfg` | Python | `python_requires` |
| `go.mod` | Go | `go` directive |
| `pom.xml` | Java | `maven.compiler.source` |
| `build.gradle` or `build.gradle.kts` | Kotlin/Java | `sourceCompatibility` |

If multiple markers found, list all (monorepo). Extract the minimum version constraint where available. If a workspace file exists (`[workspace]` in Cargo.toml, `workspaces` in package.json), note it as a workspace/monorepo.

### Detector 2: Build, Test & Lint Commands

Check sources in priority order — stop at the first reliable source per command category:

1. **Task runner** — check for `Justfile` first, then `Makefile`:
   - `Justfile`: run `just --list 2>/dev/null` and extract recipe names and descriptions
   - `Makefile`: scan for common targets (`build`, `test`, `lint`, `check`, `ci`, `fmt`, `clean`)

2. **CI config** — scan `.github/workflows/*.yml` for `run:` steps that contain build/test/lint commands. These are the most reliable source of "what the project actually runs."

3. **Toolchain defaults** — fall back based on detected language:
   - Rust: `cargo build`, `cargo test`, `cargo clippy --workspace --all-targets --all-features -- -D warnings`
   - Node: `npm test`, `npm run build`, `npm run lint`
   - Python: `pytest`, `python -m build`, `ruff check .` or `flake8`
   - Go: `go build ./...`, `go test ./...`, `golangci-lint run`

4. **Formatter/linter config** — check for config files that indicate tooling:
   - `rustfmt.toml` or `.rustfmt.toml` → `cargo fmt` (check for nightly requirement)
   - `.prettierrc*` → `npx prettier`
   - `.eslintrc*` or `eslint.config.*` → `npx eslint`
   - `ruff.toml` or `[tool.ruff]` in pyproject.toml → `ruff`
   - `.golangci.yml` → `golangci-lint`

Present commands as a fenced bash block with inline comments explaining each.

### Detector 3: Git Workflow

1. **Default branch:** run `git symbolic-ref refs/remotes/origin/HEAD 2>/dev/null | sed 's|refs/remotes/origin/||'`. Fall back to checking if `main` or `master` exists.

2. **Git-flow detection:** check if a `develop` branch exists locally or remotely (`git branch -a | grep -q develop`). If yes, note "Branch from `develop`, PRs target `develop`."

3. **Branch naming patterns:** run `git branch -a --sort=-committerdate | head -20` and look for patterns (e.g., `feature/`, `fix/`, `story-`, `backlog/`). Report the pattern if consistent.

4. **Commit conventions:** check for:
   - `lefthook.yml` or `.lefthook/` → "Conventional commits enforced by lefthook"
   - `.husky/` → "Commit hooks managed by husky"
   - `commitlint.config.*` → "Conventional commits enforced by commitlint"
   - `.commitlintrc*` → same

5. **AI attribution policy:** check existing CLAUDE.md, CONTRIBUTING.md, or commit hooks for "no AI attribution" rules. If found, include it. If not found, do not assume.

### Detector 4: Project References

Scan for notable directories and files. Build a reference table:

| Check for | If found, add reference |
|-----------|------------------------|
| `.factory/STATE.md` | Factory state |
| `.factory/specs/` with files | Specifications |
| `.factory/stories/` with files | Stories |
| `docs/architecture/` or `docs/arch/` | Architecture docs |
| `_bmad-output/` | Planning artifacts (BMAD) |
| `CONTRIBUTING.md` | Contributing guide |
| `.claude/skills/` with subdirs | Project-local skills |
| `docs/` with `.md` files | Project documentation |

Only include rows for items that actually exist. If nothing found, use placeholder:
```
<!-- TODO: Add references to key project docs, architecture decisions, etc. -->
```

## Assembly

Combine detector outputs into a single CLAUDE.md with this structure:

```markdown
# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Build & Test

[Detector 2 output — fenced bash block with commands]

## Git Workflow

[Detector 3 output — bullet list]

## Project References

[Detector 4 output — markdown table]
```

Notes:
- Language/toolchain info (Detector 1) goes as a one-line note above the Build & Test section (e.g., "Rust 1.85+, Edition 2024. Nightly rustfmt required.")
- Do NOT add `@SOUL.md`, `@.claude/rules/`, or any methodology references — the vsdd-factory plugin provides those via its agents and rules system
- Do NOT add instructions about VSDD methodology, phases, or factory protocol
- Keep it concise — this is a quick-reference file, not documentation

## Presentation

Present the assembled CLAUDE.md to the user in a fenced markdown block. Then ask:

> "Here's the detected CLAUDE.md. Want to change anything before I write it?"

If the user requests changes:
1. Apply the requested edits
2. Re-present the updated file
3. Ask for confirmation again

Repeat until the user approves.

## Write

1. Write the approved content to `CLAUDE.md` at the project root.
2. Report:
   - File path written
   - Count of TODO placeholders remaining (if any)
   - Reminder: "Update this file as your project evolves. Re-run `/vsdd-factory:scaffold-claude-md` anytime to regenerate."
