---
name: toolchain-provisioning
description: >
  Dynamically installs the verification toolchain based on the target project's
  language(s). Reads config/verification-toolchains.yaml for the tool manifest
  and the architecture doc for language selection. Executed by the devops-engineer
  agent after architecture is produced (greenfield) or after codebase ingestion (brownfield).
---

# Toolchain Provisioning

## Purpose

The Dark Factory supports multiple languages (Rust, TypeScript, Python, Go, and
more). Each language has its own verification toolchain — proof checkers, fuzzers,
mutation testers, security scanners, property testing libraries, formatters, and
linters. This skill reads the architecture document to determine which language(s)
the project uses, then installs the correct tools from the manifest at
`config/verification-toolchains.yaml`.

## When This Skill Runs

- **After architecture produced (Greenfield):** The architect has chosen the technology
  stack and written it to `.factory/specs/architecture/ARCH-INDEX.md`. The orchestrator
  spawns the devops-engineer to provision the toolchain before implementation begins.
- **After codebase ingestion (Brownfield):** The codebase-analyzer has detected the
  existing language and written it to `.factory/phase-0-ingestion/project-discovery.md`.
  The orchestrator spawns the devops-engineer to provision any missing tools.
- **After spec evolution (Feature Mode):** If a feature introduces a new language
  (e.g., adding a Python SDK to a Rust project), provision the new language's
  toolchain.
- **On demand:** Human requests `provision toolchain for <language>`.

## Precedence Rule

The architect's explicit choices ALWAYS override this manifest's defaults:

1. **Architecture doc** (`.factory/specs/architecture/ARCH-INDEX.md`) — Technology Stack section
2. **Verification architecture** (`.factory/specs/verification-architecture/ARCH-INDEX.md`) — Tooling Selection section
3. **This manifest** (`config/verification-toolchains.yaml`) — defaults per language

If the architect specifies "use mutmut 3.1.0", install that — not the manifest default.
If the architect specifies "skip mutation testing", skip it — even if the manifest says required.
If the architect specifies nothing for a category, use the manifest default.

This prevents the provisioning skill from overriding deliberate architectural decisions.

## Inputs

- `config/verification-toolchains.yaml` — the default tool manifest
- `.factory/specs/architecture/ARCH-INDEX.md` — architect's Technology Stack (takes precedence)
- `.factory/specs/verification-architecture/ARCH-INDEX.md` — architect's Tooling Selection (takes precedence)
- `.factory/phase-0-ingestion/project-discovery.md` — detected language (brownfield)
- Human-specified overrides (highest precedence)

## Workflow

### Step 1: Detect Language(s)

Read the architecture document or project discovery to determine the target language(s):

```
Greenfield: Read .factory/specs/architecture/ARCH-INDEX.md
  → Extract "Technology Stack" or "Language" field
  → May be multiple languages (e.g., Rust API + TypeScript frontend)

Brownfield: Read .factory/phase-0-ingestion/project-discovery.md
  → Extract "Primary language" field
  → Check for additional languages in dependency files

Fallback: Scan project root for detection files
  → Cargo.toml → Rust
  → package.json + tsconfig.json → TypeScript
  → pyproject.toml / setup.py → Python
  → go.mod → Go
```

If multiple languages are detected, provision toolchains for ALL of them.

Write detected languages to `.factory/toolchain-state.yaml`:

```yaml
detected_languages:
  - rust
  - typescript
provisioned_at: YYYY-MM-DDTHH:MM:SSZ
source: architecture  # or: brownfield-discovery, manual, auto-detect
```

### Step 2: Load Architect Overrides

Read the architecture doc and verification architecture for explicit tool choices:

```
If .factory/specs/architecture/ARCH-INDEX.md exists:
  → Extract "Technology Stack" section
  → Extract any pinned tool versions (e.g., "proptest 1.10.0")
  → Extract any excluded tools (e.g., "skip mutation testing for utility modules")

If .factory/specs/verification-architecture.md exists:
  → Extract "Verification Tooling Selection" section
  → These are the architect's EXPLICIT choices — they override manifest defaults
```

Build an overrides map:
```yaml
architect_overrides:
  rust:
    property_testing:
      proptest: { version: "1.10.0" }  # Architect pinned this version
    mutation:
      cargo-mutants: {}  # Architect approved default
    security:
      cargo-deny: { skip: true, reason: "Not needed for internal tool" }
```

### Step 3: Load Tool Manifest

Read `config/verification-toolchains.yaml` and extract the toolchain definition
for each detected language. Merge with architect overrides — overrides win on conflict.

For each language, build the install plan:

```yaml
install_plan:
  rust:
    proof: [kani]
    fuzzing: [cargo-fuzz]
    mutation: [cargo-mutants]
    security: [semgrep, cargo-audit, cargo-deny]
    property_testing: [proptest]
    formatting: [rustfmt]
    linting: [clippy]
  typescript:
    proof: []
    fuzzing: []
    mutation: [stryker]
    security: [semgrep, npm-audit, eslint-security]
    property_testing: [fast-check]
    formatting: [prettier]
    linting: [eslint]
```

### Step 3: Check What's Already Installed

For each tool in the install plan, run its `verify` command to check if it's
already installed:

```bash
# Example: check if kani is installed
kani --version 2>/dev/null && echo "INSTALLED" || echo "MISSING"
```

Produce a delta — tools that need installation vs tools already present.

### Step 4: Install Missing Tools

For each missing tool, execute the install command appropriate to the platform:

**Install command priority:**
1. Language-native package manager first (cargo, npm, pip, go install)
2. Homebrew on macOS if language-native not available
3. System package manager (apt, dnf) as last resort

**For cargo dependencies** (like proptest): These are project dependencies, not
global tools. Add them to the target project's Cargo.toml `[dev-dependencies]`
section instead of installing globally.

**For npm dependencies** (like fast-check, Stryker): Install as devDependencies
in the target project: `npm install --save-dev <package>`.

**For pip dependencies** (like Hypothesis, mutmut): Install in the project's
virtual environment if one exists, otherwise install globally.

After each install, run the verify command to confirm success.

### Step 5: Run Post-Install Steps

Some tools require post-install configuration:
- `kani setup` — downloads the compatible Rust toolchain for Kani
- `rustup component add rustfmt clippy` — installs Rust components
- Stryker may need a `stryker.conf.mjs` configuration file

### Step 6: Write Provisioning Report

Write the results to `.factory/toolchain-state.yaml`:

```yaml
detected_languages:
  - rust
  - typescript
provisioned_at: 2026-03-21T14:30:00Z
source: architecture

  rust:
    proof:
      kani:
        status: installed
        version: "0.53.0"
        verify_output: "Kani 0.53.0"
    fuzzing:
      cargo-fuzz:
        status: installed
        version: "0.12.0"
    mutation:
      cargo-mutants:
        status: installed
        version: "24.11.0"
    security:
      semgrep:
        status: installed
        version: "1.67.0"
      cargo-audit:
        status: installed
        version: "0.20.0"
      cargo-deny:
        status: skipped
        reason: "optional, user declined"
    property_testing:
      proptest:
        status: added_to_cargo_toml
        version: "1.4.0"
    formatting:
      rustfmt:
        status: installed
    linting:
      clippy:
        status: installed

  typescript:
    mutation:
      stryker:
        status: installed
        version: "8.2.0"
    security:
      semgrep:
        status: already_installed
      npm-audit:
        status: builtin
      eslint-security:
        status: installed
    property_testing:
      fast-check:
        status: added_to_package_json
        version: "3.15.0"
    formatting:
      prettier:
        status: installed
    linting:
      eslint:
        status: installed

summary:
  total_tools: 15
  installed: 12
  already_present: 2
  skipped: 1
  failed: 0
```

### Step 7: Validate Toolchain

Run a quick validation that the critical tools work:

```bash
# For each installed tool with a verify command:
kani --version          # Should print version
cargo fuzz --version    # Should print version
cargo mutants --version # Should print version
semgrep --version       # Should print version
```

If any critical (required: true) tool fails validation, report to the orchestrator.

## Integration with Formal Hardening

The formal-verifier agent reads `.factory/toolchain-state.yaml` before executing
formal hardening. This tells it:

- Which proof tools are available (skip Kani if project is Python)
- Which fuzzing tools to use (cargo-fuzz vs go-fuzz vs atheris)
- Which mutation testing tool is installed (cargo-mutants vs Stryker vs mutmut)
- Which security scanners to run (language-appropriate set)
- What kill rate targets apply (from verification-toolchains.yaml)

If a required tool is missing from toolchain-state.yaml, the formal-verifier
requests re-provisioning before proceeding.

## Integration with Brownfield Mode (Codebase Ingestion)

After codebase ingestion detects the existing language, the orchestrator checks if a
toolchain-state.yaml exists. If not, it spawns the devops-engineer with this
skill to provision tools. The verification-gap-analysis (Step 0e) then has
the tools it needs to assess the existing verification posture.

## Multi-Language Projects

For multi-repo projects (DF-012) where different repos use different languages:

1. Read `project.yaml` to determine per-repo languages
2. Provision each language's toolchain
3. Write per-repo tool status in toolchain-state.yaml
4. The formal-verifier runs the correct tools per repo

## Adding a New Language

To add support for a new language:

1. Add a new entry to `config/verification-toolchains.yaml` following the
   existing pattern (detect_files, tools by category, install commands)
2. Add detection logic for the new language's config files
3. No changes needed to this skill — it reads the manifest dynamically

## Storybook MCP Provisioning (DF-037 D18)

For UI/full-stack products, also install Storybook + addon-mcp:

### Detection
If `feature_type in ['ui', 'full-stack']` and package.json exists:

### Installation
```bash
# Install Storybook if not present
npx storybook@latest init  # if .storybook/ doesn't exist

# Install addon-mcp
npm install -D @storybook/addon-mcp
```

### Configuration
Create/update `.storybook/main.ts`:
```typescript
export default {
  addons: ['@storybook/addon-mcp'],
  features: {
    experimentalComponentsManifest: true,
    experimentalCodeExamples: true,
  },
};
```

### MCP Registration
Add to `.factory/mcp-config.yaml`:
```yaml
mcp_servers:
  storybook:
    type: "http"
    url: "http://localhost:6006/mcp"
    condition: "product_type in ['ui', 'full-stack']"
    lifecycle: "start with implementation, stop after release"
    managed_by: "devops-engineer"
```

### Write to toolchain-state.yaml
```yaml
ui_tooling:
  storybook:
    status: installed
    addon_mcp: installed
    mcp_url: "http://localhost:6006/mcp"
```

### Non-React Note
Component manifest feature only supports React. For Vue/Svelte/Angular:
- list-all-documentation, preview-stories, run-story-tests still work
- Component manifest (detailed props/types) unavailable
- Agents read source files directly as fallback

## Quality Gate Criteria

- [ ] All detected languages have entries in verification-toolchains.yaml
- [ ] All required tools for detected languages are installed or explicitly skipped
- [ ] toolchain-state.yaml is written with per-tool status
- [ ] No required tool has status: failed
- [ ] Verify commands pass for all installed tools
- [ ] Storybook + addon-mcp installed for UI/full-stack products (DF-037 D18)
- [ ] MCP server registered in .factory/mcp-config.yaml (DF-037 D18)

### Excalidraw MCP Provisioning (UI Products)

For UI/full-stack products, provision the Excalidraw MCP server for wireframe creation:

1. Register MCP server: `mcporter config add excalidraw --url https://mcp.excalidraw.com`
2. Verify: `mcporter call excalidraw list-tools` returns available tools
3. Install export CLI: `npm install -g @tommywalkie/excalidraw-cli`

**Tools available via Excalidraw MCP:**
- `create-drawing` — create new wireframe/diagram
- `update-drawing` — modify existing drawing
- `export-to-png` — export for embedding in specs and PRs
- `export-to-svg` — export vector format for flow diagrams

**Quality gate:** Excalidraw MCP responds to `list-tools` before proceeding to UX specification.

### Storybook MCP Integration Skill

For UI/full-stack products, invoke the storybook-mcp-integration skill
during toolchain provisioning:
- Skill: `skills/storybook-mcp-integration/SKILL.md`
- Installs Storybook + @storybook/addon-mcp
- Registers MCP server at http://localhost:6006/mcp
- Verifies 6 tools available (list-all-documentation, get-documentation,
  get-documentation-for-story, get-storybook-story-instructions,
  preview-stories, run-story-tests)
