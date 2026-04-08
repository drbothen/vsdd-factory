<!-- Audit checklist for story self-containment. SOUL owns "why", this file owns "how to verify". -->

# Story Self-Containment Audit

When creating or reviewing a story spec — especially one targeting a standalone deliverable (new repo, new tool, new crate) — run this checklist before marking the story as ready for implementation.

The goal: **an implementer can execute the story without leaving the file**, except for following reference links to specs, architecture docs, or related stories.

## Audit Checklist

Run these checks in order. Each must pass.

### 1. Source of truth alignment

Do embedded configs, dependency rules, and crate lists match the architecture docs they reference? Compare line by line — stale data is the #1 gap.

### 2. All deliverable files specified

Every file in the project structure section must have either:
- A **Deliverable section** with complete file content, OR
- A **Task** that describes its implementation (for code files)

If a file appears in the structure but has neither, it's a gap.

### 3. Technical gotchas documented

Are known pitfalls, quirks, and non-obvious decisions documented in Dev Notes? Examples:
- API quirks (cargo subcommand argv duplication)
- Library version-specific behavior (cargo_metadata feature resolution)
- Platform differences (Windows archive format)

The implementer should not discover these mid-implementation.

### 4. CI/CD workflows complete

- Are workflow YAML files provided as deliverables with full content?
- Are they aligned with the org's existing CI patterns? Document divergences and justify them.
- Are required secrets, branch protection rules, and manual setup steps listed as prerequisites?

### 5. README / user-facing docs

For tools or libraries, a README deliverable must cover:
- What it is (one-liner)
- Installation (all methods)
- Quick start
- Config format reference
- CLI reference (all subcommands, all flags)
- Exit codes
- Integration examples (CI, pre-commit, justfile)
- License

### 6. Hosting / infra decisions explicit

State these — don't imply them:
- GitHub org and repo name
- Repo visibility (public/private)
- Branch strategy (Git Flow vs trunk-based)
- Branch protection / rulesets (with exact required status checks)
- Required secrets (with setup instructions)

### 7. License stated

Not implied by file names. Explicitly chosen and consistent across:
- Story frontmatter or decisions section
- `Cargo.toml` `license` field
- `LICENSE` file reference in project structure
- README license section
- SOUL.md / rules references

### 8. Generated output specified

If the tool generates files (e.g., `generate` subcommand), specify:
- Exact output format with example
- Sort order and comment conventions
- Edge case behavior (empty workspace, zero deps)
- Exit codes for the generation path

### 9. Test fixtures defined

Test scenarios must include:
- Directory structure with file names
- Config file content
- Expected behavior (exit code, output messages)
- What the fixture tests (which AC it validates)

"Create a fixture for violations" is insufficient. "Create `violation-workspace/` with `api` depending on `server` where `server` is forbidden, expecting exit 1 and output containing `api depends on server`" is sufficient.

### 10. Shell / script rules addressed

If the project has no shell scripts, explicitly state that and exclude `bash.md` from rules. Don't leave the implementer wondering which rules files apply.

### 11. Rules index complete

The `.claude/rules/_index.md` deliverable must reference exactly the rules files that exist — no more, no fewer.

### 12. Internal consistency

Read all deliverables end-to-end and verify:
- Crate names match everywhere (no axiathon names in generic tool examples)
- License text is consistent
- File paths in project structure match deliverable headings
- Config option names match between schema docs, CLI skeleton, and example configs
- Badge URLs, repo URLs, and org names are consistent

### 13. Project-specific vs generic separation

If the story covers both a generic tool AND its integration into a specific project:
- Tool deliverables use generic example names
- Project-specific config is in a separate task/section
- The tool has no hardcoded knowledge of the consuming project

### 14. Prerequisites listed

Manual steps that must happen before Phase 1 are called out in a Prerequisites section:
- Repo creation
- Branch protection setup
- Secret configuration
- External account setup (crates.io, npm, etc.)

## Process

1. Read the full story end-to-end
2. Run each check above against the story content
3. For each failure, determine if research is needed or if the fix is straightforward
4. Fix gaps one at a time — get approval on approach before applying
5. After all gaps are fixed, do a final consistency pass (check #12) across the entire file
6. The story is ready when all 14 checks pass
