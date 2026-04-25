# Scaffold CLAUDE.md — Design Spec

## Summary

A new standalone skill (`scaffold-claude-md`) that auto-detects project context and generates a `CLAUDE.md` at the project root. Focused purely on project-specific context — build commands, git workflow, toolchain, and reference links — since the vsdd-factory plugin already provides methodology, principles, rules, and agent instructions.

## Problem

When a user activates vsdd-factory on a project, the plugin provides the full VSDD methodology (agents, rules, hooks, skills). But Claude Code still lacks project-specific context: how to build, test, lint, what the git workflow is, where key docs live. This information currently lives only in the developer's head.

Axiathon's `.claude/` setup demonstrates the value of a lightweight `CLAUDE.md` that captures this context. The gap is that vsdd-factory has no mechanism to scaffold one.

## Scope

**In scope:**
- New skill: `scaffold-claude-md`
- Four auto-detectors (language/toolchain, build/test/lint, git workflow, project references)
- CLAUDE.md generation with TODO placeholders for undetected sections
- Existing file handling (overwrite/merge/cancel)
- Updates to activate skill, docs, and relevant workflows

**Out of scope:**
- Copying SOUL.md or rules (already provided by the plugin)
- Methodology instructions (handled by orchestrator/agents)
- Workspace state snapshots (stale too fast)
- CI/CD pipeline details (Claude can read these on demand)

## Skill Definition

- **Name:** `scaffold-claude-md`
- **Location:** `plugins/vsdd-factory/skills/scaffold-claude-md/SKILL.md`
- **Invocation:** `/vsdd-factory:scaffold-claude-md`

## Detection Engine

Four independent detectors run in parallel against the current working directory. Each produces a section of the CLAUDE.md. Missing data gets a `<!-- TODO -->` placeholder.

### Detector 1: Language & Toolchain

- Scan for marker files:
  - `Cargo.toml` → Rust
  - `package.json` → Node/TypeScript
  - `pyproject.toml` / `setup.py` → Python
  - `go.mod` → Go
  - `pom.xml` / `build.gradle` → Java/Kotlin
- If multiple found, list all (monorepo case)
- Extract version constraints where available (e.g., `rust-version` from `Cargo.toml`, `engines` from `package.json`)

### Detector 2: Build, Test & Lint Commands

- Check for `Justfile` → extract recipes (`just check`, `just ci`, `just test`)
- Check for `Makefile` → extract common targets
- Fall back to toolchain defaults (`cargo build`, `npm test`, `pytest`, etc.)
- Scan `.github/workflows/` or other CI configs for actual commands used — most reliable source
- Check for formatters/linters: `rustfmt.toml`, `.prettierrc`, `.eslintrc`, `ruff.toml`, `.golangci.yml`

### Detector 3: Git Workflow

- Default branch: `git symbolic-ref refs/remotes/origin/HEAD` or fall back to `main`/`master`
- Check if `develop` branch exists (git-flow indicator)
- Check for branch naming patterns from recent branches
- Check for `lefthook.yml`, `.husky/`, or other commit hook configs

### Detector 4: Project References

- Scan for architecture docs, READMEs in subdirectories
- Check for existing `.factory/` directory and `STATE.md`
- Check for existing `.claude/skills/` (project-local skills)
- Check for planning artifacts (`_bmad-output/`, `docs/architecture/`, etc.)

## Output Format

The generated `CLAUDE.md` follows a lightweight structure — project-specific context only:

```markdown
# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Build & Test

\`\`\`bash
cargo build                        # Build all crates
cargo test                         # Run all tests
cargo clippy --workspace -- -D warnings  # Lint
just check                         # Quick pre-commit
\`\`\`

## Git Workflow

- Default branch: `main`
- Branch from `develop`, PRs target `develop`
- Conventional commits enforced by lefthook
- Branch naming: `feature/story-X.X-desc`, `fix/issue-123-desc`

## Project References

| Topic | Location |
|-------|----------|
| Factory state | `.factory/STATE.md` |
| Architecture | `docs/architecture/` |
```

Key decisions:
- No `@SOUL.md` or `@rules/` references — loaded by the plugin's agents and rules system
- No methodology instructions — orchestrator and AGENT-SOUL.md handle that
- TODO placeholders for anything detection couldn't determine
- Focused purely on "how do I build and navigate this specific project"

## Skill Flow

1. **Check for existing CLAUDE.md** — if one exists, show it and ask: "CLAUDE.md already exists. Overwrite (replace entirely), or cancel?" Merge is not offered — the file is short enough that overwrite-and-edit is simpler than attempting to diff/merge sections.
2. **Run all four detectors** in parallel (they're independent)
3. **Assemble the CLAUDE.md** from detector results, using TODO placeholders for gaps
4. **Present the full file** to the user, grouped by section with clear headers
5. **Ask for confirmation** — "Want to change anything before I write this?"
6. **Apply edits** if requested, re-present, and confirm again
7. **Write `CLAUDE.md`** to project root
8. **Report** — confirm the file path and remind about TODO placeholders if any remain

## Updates to Existing Files

| File | Change |
|------|--------|
| `skills/activate/SKILL.md` | Add a note after activation: "Run `/vsdd-factory:scaffold-claude-md` to generate project-specific instructions" |
| `docs/VSDD.md` | Add scaffold-claude-md to the tooling/setup section |
| `docs/FACTORY.md` | Mention CLAUDE.md scaffolding in the project initialization section |
| Relevant `.lobster` workflow files | Add scaffold-claude-md as an optional step in project setup workflows |

Specific workflow files to update will be identified during implementation planning.

## Non-Goals

- This skill does NOT replace or duplicate any existing vsdd-factory functionality
- This skill does NOT write agent instructions, methodology docs, or rules
- This skill does NOT persist state — the CLAUDE.md is a snapshot that the user maintains
