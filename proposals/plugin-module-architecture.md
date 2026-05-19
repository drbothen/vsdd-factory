---
document_type: proposal
proposal_id: PA-MOD-001
title: "vsdd-factory plugin-module architecture"
status: draft
author: orchestrator (via architect dispatch)
date: 2026-05-17
prerequisite_for: [SK-MCP-001 (spec-kit MCP)]
related_proposals: [SK-MCP-001]
empirical_basis: "User-identified gap during spec-kit MCP design discussion — five disjoint extension surfaces in vsdd-factory require unified plugin abstraction before adding new features cleanly"
target: vsdd-factory upstream
implementation_tiers: [Tier 1 foundation, Tier 2 first plugin (spec-kit), Tier 3 existing feature migration, Tier 4 lifecycle + marketplace]
estimated_effort: "~3-4 weeks all tiers"
priority: HIGH
---

# PA-MOD-001: vsdd-factory Plugin-Module Architecture

## 1. Problem Statement

### 1.1 The Five Disjoint Extension Surfaces

vsdd-factory currently has five independent extension surfaces with no unifying mechanism:

| Surface | Location | How it is registered |
|---------|----------|---------------------|
| **wasm hook plugins** | `hook-plugins/*.wasm` | Hand-edited `hooks-registry.toml` |
| **Sub-agents** | `.claude/agents/<name>.md` | Filesystem convention; no registry |
| **Skills** | `.claude/skills/<plugin>/<name>/` | Filesystem convention; no registry |
| **Workflows** | `workflows/*.lobster` | Filesystem convention; no registry |
| **MCP servers** | `.claude/settings.local.json` `mcpServers` block | Hand-edited JSON; no versioning |

These five surfaces have no shared lifecycle, no version tracking, no install/uninstall protocol, and no discoverability. Today, activating a new domain-specific integration requires manual coordination across all five surfaces simultaneously.

### 1.2 spec-kit as the First Exposed Gap

The spec-kit MCP proposal (SK-MCP-001) defines a domain feature that naturally spans all five surfaces:

- A Rust MCP binary (`spec-kit-mcp`) for agent-callable spec graph tools
- wasm hook plugins that guard spec edits via the factory-dispatcher
- A shared library crate (`vsdd-spec-kit-core`) linked by both binary and hooks
- One or more plugin-specific agents (e.g., `spec-kit-validator`)
- One or more plugin-specific skills (e.g., `validate-spec-graph`)

Without PA-MOD-001, implementing SK-MCP-001 requires:
1. Manually editing `hooks-registry.toml` to add hook entries
2. Manually editing `.claude/settings.local.json` to add the MCP server entry
3. Copying agent and skill markdown files into the agent/skill directories
4. Documenting all five changes in separate locations
5. Writing custom uninstall instructions with no enforcement mechanism

This is not a spec-kit problem — it is a structural problem in vsdd-factory. spec-kit is simply the first feature complex enough to make the gap visible.

### 1.3 Future Plugin Scenarios That Repeat the Problem

| Scenario | Surfaces needed |
|----------|----------------|
| Jira spec-kit (Jira issue ↔ BC traceability) | MCP server + skills + agents |
| GitHub spec-kit (PR ↔ story traceability) | MCP server + skills + hooks |
| Analytics integration (usage telemetry) | hooks + MCP server + agents |
| Custom domain validators | hooks + skills |
| Observability stack (`factory-obs`) | MCP server (Grafana queries) + skills + lifecycle scripts |
| Research cache | skills (already exists) + potential MCP exposure |
| Demo recording infrastructure | skills + agents + lifecycle (VHS binary install) |

Each of these would require the same five-surface manual integration. Without a unifying mechanism, the integration cost grows linearly with each new feature, discoverability remains zero, and uninstall is impossible without diving into five files.

### 1.4 Cost Without PA-MOD-001

- **Each new feature:** touch `hooks-registry.toml` + `settings.local.json` + agent dir + skills dir + workflows dir
- **No discoverability:** no answer to "what plugins are active on this project?"
- **No versioning:** plugin components can drift to incompatible versions across surfaces
- **No clean uninstall:** partial deactivation leaves orphaned hooks or agents
- **No CI validation:** no schema-level enforcement that all surfaces are coherent

---

## 2. Proposed Solution: Plugin-Module Architecture

### 2.1 Core Abstraction: the Plugin Module

A plugin module is a self-describing bundle that declares, in a single manifest, every surface it touches. The factory-dispatcher and install tooling read the manifest and coordinate registration across all surfaces atomically.

The module is the unit of:
- **Installation:** all surfaces activated together
- **Versioning:** one version string covers all components
- **Uninstall:** all surfaces deactivated together
- **Discovery:** a registry scan lists active plugins and their versions

### 2.2 Repository Structure

```
vsdd-factory/
  ├── core/                       # existing — shared library
  ├── dispatcher/                 # existing — factory-dispatcher binary
  ├── crates/                     # existing Rust workspace
  │   ├── factory-dispatcher/
  │   ├── hook-sdk/
  │   └── ...
  ├── plugins/                    # plugin package (existing layout extended)
  │   └── vsdd-factory/           # existing plugin package root
  │       ├── agents/             # existing core agents
  │       ├── skills/             # existing core skills
  │       ├── workflows/          # existing core workflows
  │       ├── hooks/              # existing bash hook scripts
  │       ├── hook-plugins/       # existing wasm hook binaries
  │       ├── hooks-registry.toml # existing aggregate hook registry
  │       ├── config/             # existing config files
  │       ├── plugin-modules/     # NEW — per-module subdirectories
  │       │   ├── spec-kit/       # FIRST module (SK-MCP-001)
  │       │   │   ├── plugin.toml
  │       │   │   ├── bin/
  │       │   │   ├── hook-plugins/
  │       │   │   ├── agents/
  │       │   │   ├── skills/
  │       │   │   └── templates/
  │       │   ├── observability/  # MIGRATED from factory-obs skill
  │       │   ├── research-cache/ # MIGRATED from research-cache-ops skill
  │       │   └── ...
  │       └── plugin-modules-registry.toml  # NEW — aggregate module registry
```

This layout keeps everything inside the existing `plugins/vsdd-factory/` package tree. No new top-level directories. The `plugin-modules/` subdirectory is new; it sits alongside the existing `agents/`, `skills/`, and `hook-plugins/` directories.

### 2.3 Plugin Manifest Format (`plugin.toml`)

Each plugin module declares all its surface contributions in a single `plugin.toml` file. The format is TOML for consistency with the existing `hooks-registry.toml`.

#### 2.3.1 Full Schema

```toml
[plugin]
id = "spec-kit"
version = "0.1.0"
description = "Spec graph integrity tooling for VSDD artifacts"
authors = ["vsdd-factory maintainers"]
license = "MIT"
homepage = "https://github.com/drbothen/vsdd-factory"

# ── Compatibility ────────────────────────────────────────────────────────────
[plugin.requires]
vsdd-factory = ">=1.0.0-rc.18"
rust-version = ">=1.75"

# ── MCP server ───────────────────────────────────────────────────────────────
# Auto-registered in .claude/settings.local.json on `install-plugin`.
[mcp]
binary = "bin/spec-kit-mcp"                # path relative to plugin-modules/spec-kit/
auto-register = true
tool-prefix = "mcp__spec_kit__"
config-schema = "schemas/mcp-config.json"  # optional

# ── Dispatcher hook plugins ──────────────────────────────────────────────────
# Hooks registered AFTER all existing core hooks (priority floor: 500).
# Each key is an event slot; value is the wasm path relative to plugin root.
[dispatcher.hooks]
pre_tool_use_edit  = { plugin = "hook-plugins/validate-spec-edit.wasm",  priority = 500, on_error = "continue" }
post_tool_use_edit = { plugin = "hook-plugins/emit-spec-change.wasm",    priority = 510, on_error = "continue" }

# ── Shared library ───────────────────────────────────────────────────────────
# Cargo workspace crate; built as part of `cargo build --workspace`.
[lib]
crate = "vsdd-spec-kit-core"
target = ["dispatcher", "mcp"]

# ── Plugin-specific agents ───────────────────────────────────────────────────
# Installed as .claude/agents/spec-kit__<name>.md (namespaced).
[agents]
files = ["agents/spec-kit-validator.md"]

# ── Plugin-specific skills ───────────────────────────────────────────────────
# Installed as .claude/skills/spec-kit/<name>/ (namespaced via directory).
[skills]
files = ["skills/validate-spec-graph/"]

# ── Plugin-specific workflows ────────────────────────────────────────────────
[workflows]
files = ["workflows/spec-kit-bootstrap.lobster"]

# ── Configuration templates ───────────────────────────────────────────────────
[templates]
files = ["templates/spec-kit-config.toml"]

# ── Lifecycle hooks ───────────────────────────────────────────────────────────
[lifecycle]
post-install   = "scripts/post-install.sh"
pre-uninstall  = "scripts/pre-uninstall.sh"
upgrade        = "scripts/upgrade.sh"
```

#### 2.3.2 Schema Invariants

- `plugin.id` is the namespace key. All agent/skill files installed by this plugin use `<plugin-id>__` prefix to prevent collision.
- `mcp.auto-register = false` means the MCP server is built but not activated by default (opt-in).
- `dispatcher.hooks` priority values must be >= 500. Core hooks occupy 20–460; this floor prevents module hooks from interfering with core guards.
- `lifecycle` scripts are optional. A plugin with no side effects omits `[lifecycle]`.

#### 2.3.3 Worked Example: `observability` plugin

```toml
[plugin]
id = "observability"
version = "1.0.0"
description = "Local OTel + Loki + Prometheus + Grafana observability stack"

[plugin.requires]
vsdd-factory = ">=1.0.0-rc.16"

# No MCP server — skill-only activation
[mcp]
auto-register = false

[dispatcher.hooks]
session_start = { plugin = "hook-plugins/obs-session-start.wasm", priority = 500, async = true, on_error = "continue" }

[skills]
files = ["skills/factory-obs/", "skills/claude-telemetry/", "skills/onboard-observability/"]

[lifecycle]
post-install  = "scripts/start-stack.sh"
pre-uninstall = "scripts/stop-stack.sh"
```

#### 2.3.4 Worked Example: `research-cache` plugin

```toml
[plugin]
id = "research-cache"
version = "1.0.0"
description = "Perplexity/Context7 query result cache with MCP exposure"

[plugin.requires]
vsdd-factory = ">=1.0.0-rc.18"

[mcp]
binary = "bin/research-cache-mcp"
auto-register = false                # opt-in; high-memory projects may not want it
tool-prefix = "mcp__research_cache__"

[skills]
files = ["skills/research-cache-ops/"]
```

### 2.4 Plugin Discovery and Registration

#### 2.4.1 Build-Time Manifest Aggregation

A `build.rs` script in the plugin package root scans `plugin-modules/*/plugin.toml` at compile time. It generates:

- `plugin-modules-registry.toml` — machine-readable aggregate listing all installed modules, their versions, and their surface registrations
- Updated `hooks-registry.toml` — extended with entries from all module `[dispatcher.hooks]` sections, each tagged with `[hooks.config.plugin_module] = "<id>"` for provenance

The aggregate registry format:

```toml
# plugin-modules-registry.toml — GENERATED by build.rs; do not edit by hand.
schema_version = 1

[[modules]]
id = "spec-kit"
version = "0.1.0"
path = "plugin-modules/spec-kit"
mcp_binary = "plugin-modules/spec-kit/bin/spec-kit-mcp"
agents = ["plugin-modules/spec-kit/agents/spec-kit-validator.md"]
skills = ["plugin-modules/spec-kit/skills/validate-spec-graph/"]
hooks = ["pre_tool_use_edit:plugin-modules/spec-kit/hook-plugins/validate-spec-edit.wasm"]

[[modules]]
id = "observability"
version = "1.0.0"
path = "plugin-modules/observability"
mcp_binary = ""
agents = []
skills = ["plugin-modules/observability/skills/factory-obs/"]
hooks = ["session_start:plugin-modules/observability/hook-plugins/obs-session-start.wasm"]
```

#### 2.4.2 Per-Project Install (`/vsdd-factory:install-plugin`)

The `install-plugin` skill reads `plugin-modules-registry.toml` and performs the following atomic sequence for a named plugin:

1. Read `plugin.toml` for the named module
2. Validate `[plugin.requires]` compatibility
3. Register MCP server in `.claude/settings.local.json` (if `mcp.auto-register = true`)
4. Activate plugin hooks in `hooks-registry.toml` (append entries with provenance tag)
5. Symlink plugin agents into `.claude/agents/` as `<plugin-id>__<name>.md`
6. Symlink plugin skills into `.claude/skills/<plugin-id>/`
7. Record install in `.factory/plugins-state.yaml` (new per-project state file)
8. Run `post-install` lifecycle hook if declared

The install is transactional: if any step fails, previously applied steps are rolled back. `.factory/plugins-state.yaml` tracks installed modules and their versions for idempotency.

#### 2.4.3 Per-Project Uninstall (`/vsdd-factory:uninstall-plugin`)

The `uninstall-plugin` skill reverses all install operations in reverse order:

1. Run `pre-uninstall` lifecycle hook
2. Remove agent symlinks
3. Remove skill symlinks
4. Deactivate plugin hooks in `hooks-registry.toml`
5. Deregister MCP server from `.claude/settings.local.json`
6. Remove entry from `.factory/plugins-state.yaml`

#### 2.4.4 Plugin Listing (`/vsdd-factory:list-plugins`)

The `list-plugins` skill reads `plugin-modules-registry.toml` (available modules) and `.factory/plugins-state.yaml` (installed state) and renders a status table:

```
Plugin Module Status
───────────────────────────────────────────────────────
  spec-kit        v0.1.0   INSTALLED   (MCP + 1 hook + 1 agent + 1 skill)
  observability   v1.0.0   INSTALLED   (1 hook + 3 skills)
  research-cache  v1.0.0   AVAILABLE   (MCP + 1 skill)
  jira            v0.1.0   AVAILABLE   (1 skill)
───────────────────────────────────────────────────────
  2 installed / 4 available
```

---

## 3. Migration of Existing Features to Plugin Modules

### 3.1 Migration Candidate Table

| Current feature | Becomes module | Priority | Notes |
|----------------|----------------|----------|-------|
| `factory-obs` skill (Loki+Grafana+OTel) | `plugin-modules/observability/` | Tier 3 | Start/stop lifecycle maps directly to post-install/pre-uninstall hooks |
| `research-cache-ops` skill | `plugin-modules/research-cache/` | Tier 3 | Expose cache as MCP server (optional auto-register) |
| Jira integration (`jira` skill, ankitpokhrel jira-cli) | `plugin-modules/jira/` | Tier 3 | Currently skill-only; future: MCP server for Jira ops |
| `storybook-mcp-integration` skill | `plugin-modules/storybook-mcp/` | Tier 3 | Already an external MCP; plugin model is cleaner |
| `excalidraw-export` + `create-excalidraw` skills | `plugin-modules/excalidraw/` | Tier 4 | Playwright-based renderer |
| `demo-recording` skill (VHS + Playwright) | `plugin-modules/demo-recording/` | Tier 4 | Lifecycle: install VHS binary |
| `claude-telemetry` + `onboard-observability` | absorbed into `observability` module | Tier 3 | Logically part of observability |

### 3.2 What Stays in Core

These remain as first-class vsdd-factory components, NOT plugin modules:

- Orchestrator agent and all specialist agents (architect, PO, adversary, etc.)
- All core skills (per-story-delivery, wave-gate, create-brief, create-prd, etc.)
- All core workflows (greenfield, brownfield, feature, maintenance)
- Core factory-dispatcher binary and all existing hook plugins under `hook-plugins/`
- Core `hooks-registry.toml` (the non-plugin-module entries)
- All `hooks/*.sh` scripts

The boundary rule: if a component is used on every project regardless of domain, it is core. If a component is domain-specific, optional, or has external service dependencies, it is a plugin module candidate.

### 3.3 Migration Protocol

For each Tier 3/4 migration:

1. Create `plugin-modules/<id>/plugin.toml` manifest
2. Move skill files into `plugin-modules/<id>/skills/`
3. Move any lifecycle scripts into `plugin-modules/<id>/scripts/`
4. Remove the skill from the flat `skills/` directory (or leave a redirect shim that points to the new location for backward compat)
5. Add to `plugin-modules-registry.toml`
6. Update install documentation

Tier 3 migrations happen in parallel with SK-MCP-001 Tier 2/3 development (see Section 7).

---

## 4. Plugin Development Guide

### 4.1 Step-by-Step Authoring Template

**Step 1: Create the module directory**

```
plugins/vsdd-factory/plugin-modules/<plugin-id>/
```

**Step 2: Write `plugin.toml`**

Use the manifest schema from Section 2.3. Start minimal — include only the surfaces your plugin actually needs. Omit `[mcp]`, `[lib]`, `[agents]`, `[workflows]`, and `[lifecycle]` sections entirely if not applicable rather than leaving them as empty tables.

**Step 3: Implement each declared surface**

For wasm hooks: implement using the `hook-sdk` crate, following the existing hook-plugins pattern. Declare the hook in `[dispatcher.hooks]` with a priority >= 500.

For MCP server: implement as a Rust binary crate in the vsdd-factory workspace. The binary path declared in `[mcp] binary` must match the compiled output path.

For agents/skills: author markdown files following the existing agent/skill format. The `plugin-id__` namespace prefix is automatically applied during install — do NOT add it to the filename in the plugin source tree.

**Step 4: Add to Cargo workspace (if binary or lib)**

```toml
# crates/Cargo.toml or Cargo.toml at workspace root
[workspace]
members = [
  # ...existing members...
  "plugins/vsdd-factory/plugin-modules/spec-kit/bin/spec-kit-mcp",
  "plugins/vsdd-factory/plugin-modules/spec-kit/lib/vsdd-spec-kit-core",
]
```

**Step 5: Add CI coverage**

```yaml
# In ci/test.yml or equivalent
- name: Test plugin module
  run: cargo test -p vsdd-spec-kit-core -p spec-kit-mcp
```

**Step 6: Validate plugin.toml schema**

Run (once the build.rs tooling exists):
```bash
vsdd-factory validate-plugin plugins/vsdd-factory/plugin-modules/<plugin-id>/plugin.toml
```

**Step 7: Submit PR**

The `build.rs` manifest aggregation runs in CI. Plugin validation is a required CI check — malformed `plugin.toml` or mismatched surface declarations block merge.

### 4.2 Naming Conventions

| Component | Pattern | Example |
|-----------|---------|---------|
| Plugin module directory | `plugin-modules/<kebab-id>/` | `plugin-modules/spec-kit/` |
| Agent files (source) | `agents/<slug>.md` | `agents/spec-kit-validator.md` |
| Agent files (installed) | `.claude/agents/<plugin-id>__<slug>.md` | `.claude/agents/spec-kit__spec-kit-validator.md` |
| Skill directory (source) | `skills/<slug>/` | `skills/validate-spec-graph/` |
| Skill directory (installed) | `.claude/skills/<plugin-id>/<slug>/` | `.claude/skills/spec-kit/validate-spec-graph/` |
| Hook binary | `hook-plugins/<verb>-<noun>.wasm` | `hook-plugins/validate-spec-edit.wasm` |
| MCP binary | `bin/<plugin-id>-mcp` | `bin/spec-kit-mcp` |

---

## 5. Integration with Existing vsdd-factory Architecture

### 5.1 Build System

The Cargo workspace already spans `crates/` and `plugins/vsdd-factory/`. Plugin module binary and library crates are added as workspace members by their `plugin.toml` declarations. The `build.rs` in the plugin package root orchestrates manifest aggregation:

```rust
// plugins/vsdd-factory/build.rs (pseudocode)
fn main() {
    // 1. Glob plugin-modules/*/plugin.toml
    // 2. Parse each manifest
    // 3. Validate required fields and priority constraints
    // 4. Emit plugin-modules-registry.toml
    // 5. Append plugin hook entries to hooks-registry-plugin-extensions.toml
    //    (merged with core hooks-registry.toml at install time, not at build time)
    // 6. cargo:rerun-if-changed=plugin-modules/*/plugin.toml
}
```

Important: the `hooks-registry.toml` file is NOT overwritten by build.rs. Plugin hook entries are emitted to a separate `hooks-registry-plugin-extensions.toml` that the install tooling merges. This preserves the human-edited source-of-truth property of `hooks-registry.toml` for core hooks.

### 5.2 Distribution Package Layout

The installed vsdd-factory package at `~/.claude/plugins/cache/claude-mp/vsdd-factory/<version>/` gets these additions:

```
vsdd-factory/<version>/
  ├── bin/                         # existing — factory-dispatcher binary
  ├── hooks/                       # existing — bash hook scripts
  ├── hook-plugins/                # existing — core wasm binaries
  ├── hooks-registry.toml          # existing — core + activated plugin hooks
  ├── agents/                      # existing — core agent markdown files
  ├── skills/                      # existing — core skill directories
  ├── workflows/                   # existing — core lobster workflows
  ├── config/                      # existing — artifact-path-registry etc.
  ├── plugin-modules/              # NEW — bundled plugin modules
  │   ├── plugin-modules-registry.toml
  │   ├── spec-kit/
  │   │   ├── plugin.toml
  │   │   ├── bin/spec-kit-mcp
  │   │   ├── hook-plugins/validate-spec-edit.wasm
  │   │   ├── agents/spec-kit-validator.md
  │   │   └── skills/validate-spec-graph/
  │   ├── observability/
  │   │   ├── plugin.toml
  │   │   └── skills/{factory-obs,claude-telemetry,onboard-observability}/
  │   └── research-cache/
  │       ├── plugin.toml
  │       ├── bin/research-cache-mcp
  │       └── skills/research-cache-ops/
```

### 5.3 Settings Integration

When `install-plugin` activates an MCP server, it writes to `.claude/settings.local.json`:

```json
{
  "mcpServers": {
    "spec-kit": {
      "command": "~/.claude/plugins/cache/claude-mp/vsdd-factory/1.0.0-rc.19/plugin-modules/spec-kit/bin/spec-kit-mcp",
      "args": [],
      "_plugin_module": "spec-kit",
      "_plugin_version": "0.1.0"
    }
  }
}
```

The `_plugin_module` and `_plugin_version` fields are bookkeeping annotations used by `uninstall-plugin` to cleanly remove only plugin-installed entries without touching manually-added MCP servers.

### 5.4 Hook Priority Architecture

Core hooks occupy priority 20–460 (as seen in the current `hooks-registry.toml`). The plugin priority floor is 500. This separation is enforced by `build.rs` schema validation and the `lint-registry-async-invariant` hook.

```
Priority bands:
  20–100   Core PreToolUse guards (block-ai-attribution, protect-bc, red-gate, ...)
  110–160  Core tracking + path validation
  210–460  Core PostToolUse validators
  500+     Plugin module hooks (never overlap with core band)
```

A plugin that declares a priority < 500 in `[dispatcher.hooks]` fails schema validation in `build.rs` and is rejected at CI.

---

## 6. Backward Compatibility

### 6.1 Existing MCP Registrations

The per-project MCP registrations for Perplexity, Context7, Tavily, and Playwright remain in `.claude/settings.local.json` as they are today. These are not managed by the plugin module system. The `_plugin_module` annotation field only appears on entries created by `install-plugin`. Existing entries without the annotation are not touched by `uninstall-plugin`.

### 6.2 Existing Hook Registrations

All existing `hooks-registry.toml` entries continue working as-is. Plugin module hooks are appended as additional entries. No existing entries are modified or removed by the plugin module system. The `plugin_module` provenance tag in plugin-generated entries distinguishes them from manually-edited core entries.

### 6.3 Existing Agents and Skills

The existing `plugins/vsdd-factory/agents/` and `plugins/vsdd-factory/skills/` directories are unaffected. The migration of skills into `plugin-modules/` (Tier 3) is opt-in and non-breaking — the skill remains available from the core location until the migration is complete and confirmed. Skills promoted to a plugin module may leave a redirect shim in the original location pointing to the plugin module path.

### 6.4 Namespace Safety

All plugin-installed agent files use `<plugin-id>__<slug>.md` naming. Core agent files use bare `<slug>.md` naming. The `__` separator is reserved for plugin namespace use. A future `validate-agent-namespace` hook can enforce that core agents never use `__` in their filenames.

---

## 7. Implementation Roadmap

### Tier 1: Foundation (~3-5 days)

Goal: infrastructure only, no content migration, no feature activation.

- [ ] Define `plugin.toml` schema as a Rust struct with serde + JSON Schema for validators
- [ ] Implement `build.rs` manifest scanning and `plugin-modules-registry.toml` generation
- [ ] Implement priority-floor validation in `build.rs` (reject plugin hooks priority < 500)
- [ ] Implement `install-plugin` skill (atomic, transactional, rollback on failure)
- [ ] Implement `uninstall-plugin` skill
- [ ] Implement `list-plugins` skill
- [ ] Introduce `.factory/plugins-state.yaml` per-project state file
- [ ] Register `.factory/plugins-state.yaml` in `artifact-path-registry.yaml`
- [ ] Documentation: plugin authoring guide at `plugins/vsdd-factory/plugin-modules/README.md`
- [ ] CI: `validate-plugin-manifests` job that runs `build.rs` in check mode

No existing feature migration. No plugin modules exist yet. Tier 1 ships as a standalone story.

### Tier 2: First Plugin — spec-kit (~1 week)

Goal: prove the plugin model end-to-end by implementing SK-MCP-001 as `plugin-modules/spec-kit/`.

- [ ] Create `plugin-modules/spec-kit/plugin.toml`
- [ ] Implement `vsdd-spec-kit-core` library crate (schema parse + graph traversal)
- [ ] Implement `spec-kit-mcp` binary (MCP server exposing spec-kit-core tools)
- [ ] Implement at least one wasm hook plugin (`validate-spec-edit.wasm`)
- [ ] Author `spec-kit-validator` agent
- [ ] Author `validate-spec-graph` skill
- [ ] End-to-end install/uninstall test
- [ ] Validates: all five surfaces activated and deactivated atomically

Tier 2 runs concurrently with SK-MCP-001 Tier 1/2 development (they are the same work).

### Tier 3: Existing Feature Migration (~2 weeks, parallelizable with SK-MCP-001 Tier 3)

Goal: migrate existing features to reduce core surface area.

- [ ] Migrate `factory-obs` + `claude-telemetry` + `onboard-observability` → `plugin-modules/observability/`
- [ ] Migrate `research-cache-ops` → `plugin-modules/research-cache/` with optional MCP
- [ ] Migrate `jira` skill → `plugin-modules/jira/`
- [ ] Migrate `storybook-mcp-integration` → `plugin-modules/storybook-mcp/`
- [ ] Validate backward compat for each migration (skills still accessible from original paths)

### Tier 4: Lifecycle and Discoverability (~1 week)

Goal: full lifecycle management and optional community plugin registry.

- [ ] Plugin upgrade skill (`upgrade-plugin`) with version-aware migration hooks
- [ ] Hot-reload capability for wasm hook modules (swap without dispatcher restart)
- [ ] `plugins-state.yaml` drift detection (installed version != registry version)
- [ ] Optional: community plugin registry server design (JSON manifest endpoint)

---

## 8. Relationship to SK-MCP-001

PA-MOD-001 is the PREREQUISITE infrastructure. SK-MCP-001 is the FIRST plugin module. They share implementation work — SK-MCP-001's components become `plugin-modules/spec-kit/`.

### Recommended Implementation Order

| Stage | Work | Duration |
|-------|------|----------|
| 1 | PA-MOD-001 Tier 1 (foundation infrastructure) | ~3-5 days |
| 2 | SK-MCP-001 Tier 1 + PA-MOD-001 Tier 2 (spec-kit as plugin-modules/spec-kit/) | ~1-2 weeks |
| 3 | SK-MCP-001 Tier 2 (atomic mutations + dispatcher hook) | ~1 week |
| 4 | PA-MOD-001 Tier 3 + SK-MCP-001 Tier 3 (parallel) | ~2-3 weeks |
| 5 | PA-MOD-001 Tier 4 (lifecycle + discoverability) | ~1 week |

Total estimated: 5-7 weeks for both proposals fully implemented.

### Cross-References Required in SK-MCP-001

SK-MCP-001 should be amended to:
1. State in its frontmatter: `prerequisite: PA-MOD-001 (plugin-module architecture)`
2. Reference `plugins/vsdd-factory/plugin-modules/spec-kit/` as the canonical target path (not a flat placement in `plugins/vsdd-factory/`)
3. Define its hook plugin priority values as >= 500
4. Describe the `install-plugin` skill as the activation mechanism rather than manual `hooks-registry.toml` editing

---

## 9. Risks and Mitigations

| Risk | Likelihood | Impact | Mitigation |
|------|-----------|--------|-----------|
| Plugin manifest format evolves; old plugins break | MEDIUM | HIGH | Manifest schema versioning (`schema_version` field); `vsdd-factory` CI validates against pinned schema; migration notes in CHANGELOG |
| Plugin MCP server crashes; agent dispatch fails silently | LOW | HIGH | MCP server failures are isolated — they do not block agent dispatch; agents fall back to non-MCP paths; `list-plugins` reports MCP server health |
| Two plugins claim the same hook priority slot | MEDIUM | MEDIUM | `build.rs` detects priority collisions at build time and fails CI; conflict resolution: higher-numbered version wins, or explicit ordering in `plugin-modules-registry.toml` |
| Plugin namespace collision (two plugins install `foo` agent) | LOW | MEDIUM | `<plugin-id>__` prefix enforces namespace separation; `build.rs` validates uniqueness across all installed modules |
| `.factory/plugins-state.yaml` drifts from actual installed state | MEDIUM | LOW | `list-plugins` runs a reconciliation check on each invocation; divergence is reported as a warning, not a hard failure |
| Tier 3 migration breaks existing skill invocation | MEDIUM | MEDIUM | Each migrated skill leaves a redirect shim in its original location during a one-release transition period; shims are removed in the following release |
| Plugin authors include malicious code | LOW | CRITICAL | Plugin modules are in-repo; same review process as core code. External/community plugins require explicit trust grant (future Tier 4 feature). |

---

## 10. Open Questions for Human Adjudication

The following questions require a human decision before Tier 1 implementation can be finalized:

**OQ-PA-001:** Should plugin modules ship bundled in the vsdd-factory release package (current proposal), or should they be separately installable from a remote registry (NPM/crates.io model)?

- **Bundled:** simpler distribution, version lockstep with factory, no network dependency
- **Separate registry:** allows community plugins, independent versioning, requires network + trust infrastructure
- **Recommendation:** bundled for Tier 1-3; defer community registry to Tier 4 with a dedicated design

**OQ-PA-002:** Should plugin discovery be runtime (each session scans `plugin-modules-registry.toml` on load) or build-time (hooks compiled in)?

- **Runtime:** more flexible, no rebuild needed to activate/deactivate
- **Build-time:** faster session start, no filesystem scan overhead
- **Recommendation:** runtime for agent/skill discovery (lightweight); build-time for wasm hook activation (hooks require a registry rebuild anyway)

**OQ-PA-003:** Should plugin-specific agents and skills be loaded eagerly (always visible to the LLM) or lazily (loaded only when the plugin is explicitly invoked)?

- **Eager:** agent is always in context, can self-initiate if relevant
- **Lazy:** smaller context budget per session; agent invoked via explicit `install-plugin` mention
- **Recommendation:** skills are always available once installed (low context cost, markdown only); MCP tools are available only when the MCP server is running

**OQ-PA-004:** What is the right model for hook conflict resolution when two plugins register the same event slot at the same priority?

- **First-installed wins:** simple, predictable, but installation order matters
- **Last-installed wins:** allows overrides but may surprise existing users
- **Explicit ordering in registry:** most control, most configuration burden
- **Recommendation:** explicit ordering in `plugin-modules-registry.toml`; CI rejects equal-priority same-slot registrations without an explicit `conflict_resolution` field

---

## Appendix A: plugin.toml Full Schema Reference

```toml
# All fields annotated with [REQUIRED] or [OPTIONAL]

[plugin]
id          = "..."          # [REQUIRED] kebab-case identifier, globally unique in registry
version     = "..."          # [REQUIRED] semver string
description = "..."          # [REQUIRED] one-sentence description
authors     = ["..."]        # [OPTIONAL] list of author strings
license     = "..."          # [OPTIONAL] SPDX license identifier
homepage    = "..."          # [OPTIONAL] URL

[plugin.requires]
vsdd-factory = ">=..."       # [REQUIRED] semver constraint on factory version
rust-version = ">=..."       # [OPTIONAL] only if plugin has a Rust crate

[mcp]
binary        = "..."        # [OPTIONAL] relative path to MCP binary; omit if no MCP
auto-register = true|false   # [OPTIONAL] default false; true auto-adds to settings.local.json
tool-prefix   = "..."        # [OPTIONAL] mcp__<id>__ by default
config-schema = "..."        # [OPTIONAL] path to JSON Schema for MCP config block

[dispatcher.hooks]
# Each key is an event name (snake_case). Value is an inline table.
<event_name> = { plugin = "...", priority = NNN, on_error = "continue|block", async = true|false }
# priority must be >= 500.

[lib]
crate   = "..."              # [OPTIONAL] Cargo crate name; omit if no shared library
target  = ["dispatcher", "mcp"]  # [OPTIONAL] which components link this crate

[agents]
files = ["..."]              # [OPTIONAL] list of agent markdown file paths (relative to plugin root)

[skills]
files = ["..."]              # [OPTIONAL] list of skill directory paths (relative to plugin root)

[workflows]
files = ["..."]              # [OPTIONAL] list of lobster workflow file paths

[templates]
files = ["..."]              # [OPTIONAL] list of template file paths

[lifecycle]
post-install  = "..."        # [OPTIONAL] script run after install completes
pre-uninstall = "..."        # [OPTIONAL] script run before uninstall begins
upgrade       = "..."        # [OPTIONAL] script run during version upgrade
```

---

## Appendix B: `.factory/plugins-state.yaml` Format

```yaml
# .factory/plugins-state.yaml — per-project installed plugin state.
# Managed by install-plugin / uninstall-plugin skills. Do not edit by hand.
schema_version: 1

installed:
  - id: spec-kit
    version: "0.1.0"
    installed_at: "2026-05-17T00:00:00Z"
    factory_version: "1.0.0-rc.19"
    mcp_registered: true
    hooks_activated: ["pre_tool_use_edit"]
    agents_installed: ["spec-kit__spec-kit-validator"]
    skills_installed: ["spec-kit/validate-spec-graph"]

  - id: observability
    version: "1.0.0"
    installed_at: "2026-04-25T00:00:00Z"
    factory_version: "1.0.0-rc.16"
    mcp_registered: false
    hooks_activated: ["session_start"]
    agents_installed: []
    skills_installed: ["observability/factory-obs", "observability/claude-telemetry"]
```
