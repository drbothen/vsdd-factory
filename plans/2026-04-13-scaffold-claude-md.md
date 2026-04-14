# Scaffold CLAUDE.md Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add a standalone skill that auto-detects project context and generates a `CLAUDE.md` at the project root for vsdd-factory users.

**Architecture:** A single SKILL.md file that instructs Claude to run four parallel detectors (language/toolchain, build/test/lint, git workflow, project references), assemble results into a CLAUDE.md, present it for confirmation, and write it. No shell scripts or external tooling — pure skill instructions executed by Claude.

**Tech Stack:** Markdown skill definition, Claude Code `@` directive pattern, shell commands for detection (ls, git, jq, grep)

---

### Task 1: Create the scaffold-claude-md skill directory and SKILL.md

**Files:**
- Create: `plugins/vsdd-factory/skills/scaffold-claude-md/SKILL.md`

- [ ] **Step 1: Create the skill directory**

```bash
mkdir -p plugins/vsdd-factory/skills/scaffold-claude-md
```

- [ ] **Step 2: Write the SKILL.md file**

Create `plugins/vsdd-factory/skills/scaffold-claude-md/SKILL.md` with the following content:

```markdown
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
```

- [ ] **Step 3: Verify the file was created and reads correctly**

```bash
cat plugins/vsdd-factory/skills/scaffold-claude-md/SKILL.md | head -5
```

Expected: the frontmatter header with `name: scaffold-claude-md`.

- [ ] **Step 4: Commit**

```bash
git add plugins/vsdd-factory/skills/scaffold-claude-md/SKILL.md
git commit -m "feat(skills): add scaffold-claude-md skill

Auto-detects project language, build/test/lint commands, git workflow,
and project references to generate a project-specific CLAUDE.md."
```

---

### Task 2: Update the activate skill to mention scaffold-claude-md

**Files:**
- Modify: `plugins/vsdd-factory/skills/activate/SKILL.md`

- [ ] **Step 1: Read the current activate skill**

Read `plugins/vsdd-factory/skills/activate/SKILL.md` to confirm current content.

- [ ] **Step 2: Add scaffold-claude-md reference to the confirmation output**

In the `## Procedure` section, after step 4 (Confirm activation), add a step 5:

```markdown
5. **Suggest CLAUDE.md scaffolding.** If no `CLAUDE.md` exists at the project root, print:
   > "Tip: Run `/vsdd-factory:scaffold-claude-md` to auto-generate project-specific build, test, and git instructions for Claude Code."
```

- [ ] **Step 3: Add to the "See also" section**

Add this line to the `## See also` section:

```markdown
- `/vsdd-factory:scaffold-claude-md` — generate a project-specific CLAUDE.md
```

- [ ] **Step 4: Commit**

```bash
git add plugins/vsdd-factory/skills/activate/SKILL.md
git commit -m "feat(skills): activate suggests scaffold-claude-md when no CLAUDE.md exists"
```

---

### Task 3: Update docs/FACTORY.md to mention CLAUDE.md scaffolding

**Files:**
- Modify: `plugins/vsdd-factory/docs/FACTORY.md`

- [ ] **Step 1: Read FACTORY.md and find the project initialization section**

Search for sections related to project setup, initialization, or getting started. Look for headers like "Project Setup", "Getting Started", or the section near repo-initialization references.

- [ ] **Step 2: Add CLAUDE.md scaffolding mention**

In the appropriate project initialization section, add:

```markdown
### Project-Specific Instructions (CLAUDE.md)

The vsdd-factory plugin provides methodology, principles, rules, and agent instructions automatically. Project-specific context — build commands, git workflow, toolchain, and reference links — lives in a `CLAUDE.md` at the project root.

Run `/vsdd-factory:scaffold-claude-md` to auto-detect and generate this file. It inspects your project for language markers, task runners, CI configs, git branch strategy, and documentation, then presents a draft for your approval.

This file is maintained by the project owner and is not managed by the plugin.
```

- [ ] **Step 3: Commit**

```bash
git add plugins/vsdd-factory/docs/FACTORY.md
git commit -m "docs: add CLAUDE.md scaffolding section to FACTORY.md"
```

---

### Task 4: Update docs/VSDD.md to reference the skill

**Files:**
- Modify: `plugins/vsdd-factory/docs/VSDD.md`

- [ ] **Step 1: Read VSDD.md and find the appropriate section**

Search for sections about tooling, setup, or getting started.

- [ ] **Step 2: Add a brief mention**

In the appropriate section, add a line referencing the skill:

```markdown
- **Project context:** `/vsdd-factory:scaffold-claude-md` — auto-detects and generates a project-specific `CLAUDE.md` with build, test, git, and reference information.
```

- [ ] **Step 3: Commit**

```bash
git add plugins/vsdd-factory/docs/VSDD.md
git commit -m "docs: reference scaffold-claude-md skill in VSDD.md"
```

---

### Task 5: Update relevant workflow files

**Files:**
- Modify: `plugins/vsdd-factory/workflows/greenfield.lobster`
- Modify: `plugins/vsdd-factory/workflows/brownfield.lobster`

- [ ] **Step 1: Read the greenfield workflow repo-initialization section**

Read `plugins/vsdd-factory/workflows/greenfield.lobster` lines 32-75 (repo initialization through state initialization).

- [ ] **Step 2: Add scaffold-claude-md as an optional step after repo-initialization in greenfield**

Add a new step between `factory-worktree-gate` and `state-initialization`:

```yaml
    # =========================================================================
    # Project Instructions (CLAUDE.md)
    # Optional: generates project-specific CLAUDE.md if not present.
    # =========================================================================

    - name: scaffold-claude-md
      type: skill
      skill: "skills/scaffold-claude-md/SKILL.md"
      depends_on: [factory-worktree-gate]
      condition: "!file_exists('CLAUDE.md')"
      optional: true
```

- [ ] **Step 3: Update state-initialization dependency in greenfield**

Update `state-initialization` to depend on `scaffold-claude-md` instead of (or in addition to) `factory-worktree-gate`:

```yaml
    - name: state-initialization
      type: agent
      agent: state-manager
      depends_on: [factory-worktree-gate, scaffold-claude-md]
```

Since `scaffold-claude-md` is optional, the workflow continues even if skipped.

- [ ] **Step 4: Add scaffold-claude-md to brownfield workflow**

Read `plugins/vsdd-factory/workflows/brownfield.lobster` and add the same optional step after `factory-worktree-gate` and before `state-initialization`, following the same pattern as greenfield.

- [ ] **Step 5: Commit**

```bash
git add plugins/vsdd-factory/workflows/greenfield.lobster plugins/vsdd-factory/workflows/brownfield.lobster
git commit -m "feat(workflows): add optional scaffold-claude-md step to greenfield and brownfield"
```

---

### Task 6: Test the skill manually

**Files:**
- No files created or modified — validation only

- [ ] **Step 1: Launch Claude Code with the plugin**

```bash
cd /Users/jmagady/Dev/vsdd-factory
claude --plugin-dir ./plugins/vsdd-factory
```

- [ ] **Step 2: Run the skill on a test project**

Navigate to a project with known characteristics (e.g., axiathon which has Cargo.toml, Justfile, develop branch, .factory/) and run:

```
/vsdd-factory:scaffold-claude-md
```

- [ ] **Step 3: Verify detection accuracy**

Check that the generated CLAUDE.md:
- Correctly identifies the language and version (Rust 1.85+)
- Lists build/test/lint commands from the Justfile
- Detects the git-flow workflow (develop branch, conventional commits via lefthook)
- Finds project references (.factory/STATE.md, architecture docs)
- Does NOT include VSDD methodology, SOUL.md references, or rules references

- [ ] **Step 4: Verify the edit flow**

Request a change to the presented CLAUDE.md (e.g., "add a note about nightly rustfmt") and confirm it re-presents and writes correctly.

- [ ] **Step 5: Verify existing file handling**

Run the skill again on the same project. Confirm it detects the existing CLAUDE.md and offers overwrite/cancel.
