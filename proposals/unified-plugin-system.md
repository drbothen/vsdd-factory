---
document_type: proposal
proposal_id: UNI-PLUG-001
title: "Unified plugin system in factory-dispatcher (hooks + MCP)"
status: draft
author: orchestrator (via architect dispatch)
date: 2026-05-17
amended:
  - date: 2026-05-18
    summary: "(S-15.14 amendment) §6 updated: added OQ-UNI-005 for validate-dispatch-advance priority adjudication when SK-MCP-001 Tier 2 lands. Note added that S-15.14 ships validate-dispatch-advance at priority 154 under migration_source exemption (§5.2); UNI-PLUG-001 500+ floor applies only to new non-migrated plugins."
supersedes: PA-MOD-001 (per user Q1=A dispatcher-only scope decision)
prerequisite_for: [SK-MCP-001]
empirical_basis: "User direction during spec-kit MCP design discussion: 'I want to unify the plugin systems' + Q1=A dispatcher-only narrow scope"
target: vsdd-factory upstream
implementation_tiers: [Tier 1 foundation + hook migration, Tier 2 MCP plugin support, Tier 3 hooks-registry.toml cleanup]
estimated_effort: "~1-2 weeks total"
priority: HIGH (PREREQUISITE for SK-MCP-001 implementation)
sk_mcp_001_amendments_required: 4 cross-references (see §6)
---

# UNI-PLUG-001: Unified Plugin System in factory-dispatcher (Hooks + MCP)

## 1. Executive Summary

The factory-dispatcher currently manages one plugin type: wasm hook plugins, registered
via a hand-edited `hooks-registry.toml`. The spec-kit feature (SK-MCP-001) requires a
second plugin type — an MCP server — and there is no clean way to integrate it today.
Rather than add a parallel second system, this proposal unifies both types under one
registry by extending the dispatcher's plugin infrastructure: a new `plugins/` directory
replaces `hook-plugins/`, each plugin gets a self-describing `plugin.toml` manifest that
may declare a `[hooks]` section, an `[mcp]` section, or both, and a `plugins-registry.toml`
is generated at build time from the individual manifests. The 59 existing hook registrations
(31 distinct wasm binaries) migrate mechanically — their behavior is unchanged. SK-MCP-001
spec-kit becomes the first plugin to declare both `[hooks]` and `[mcp]`, proving the unified
model end-to-end. Estimated foundation effort is ~3-5 days; MCP support is ~3-5 more days;
cleanup is ~2-3 days. Agents, skills, and workflows directories are explicitly out of scope
and unchanged.

---

## 2. Problem Statement

### 2.1 The Existing System: Hooks-Only

factory-dispatcher today has exactly one plugin mechanism:

| Component | Location | How registered |
|-----------|----------|---------------|
| wasm hook plugins | `hook-plugins/*.wasm` | Hand-edited `hooks-registry.toml` |

The registry currently contains 59 hook registrations covering 31 distinct wasm binaries,
dispatched across events (PreToolUse, PostToolUse, SessionStart, SessionEnd, SubagentStop,
Stop, WorktreeCreate, WorktreeRemove). This system works and must not regress.

### 2.2 The Gap: MCP Server Integration

SK-MCP-001 requires two things:

1. A wasm hook plugin (`vsdd-spec-kit-validator.wasm`) that fires as a pre-commit gate on
   spec file edits — this fits the existing hook system.
2. An MCP server binary (`spec-kit-mcp`) that agents can call for spec-graph queries and
   atomic mutations — this has NO existing integration path.

Today, an MCP server is registered by hand-editing `.claude/settings.local.json` on each
project. That approach has four problems:

- **No version tracking.** If vsdd-factory is upgraded (rc.19 → rc.20), the `settings.local.json`
  entry still points to the old version's binary path and silently breaks.
- **No provenance.** There is no way to distinguish a vsdd-factory-managed MCP entry from a
  manually-added one. Uninstall is destructive or manual.
- **No discoverability.** There is no answer to "which MCP servers does vsdd-factory manage
  on this project?"
- **No consistency check.** A project may have the spec-kit hook active but the MCP server
  unregistered, leaving the system in a half-operational state.

### 2.3 Why Not a Second Parallel System?

Adding a separate MCP registry alongside `hooks-registry.toml` would require:
- Maintaining two registry formats, two parsers, two upgrade paths
- Keeping hook and MCP registrations in sync manually when a feature spans both (as spec-kit does)
- Doubling the surface area for registry drift

The correct fix is unification: one registry, one manifest format, two plugin capability
types. This is the minimum change that closes the gap for SK-MCP-001 without creating
long-term maintenance debt.

### 2.4 Scope Boundary

This proposal is narrowly scoped to the factory-dispatcher's plugin infrastructure:

| In scope | Out of scope |
|----------|-------------|
| `hooks-registry.toml` → `plugins-registry.toml` migration | `agents/` directory (unchanged) |
| 31 wasm binaries → `plugins/<id>/` directory layout | `skills/` directory (unchanged) |
| `plugin.toml` manifest supporting `[hooks]` and `[mcp]` | `workflows/` directory (unchanged) |
| Dispatcher session-start MCP auto-registration | Per-project install/uninstall lifecycle |
| SK-MCP-001 as first `[hooks]` + `[mcp]` plugin | Plugin marketplace / community registry |
| Build-time `plugins-registry.toml` aggregation | Hot-reload of running plugins |

---

## 3. Proposed Solution: Unified Plugin System

### 3.1 Installed Package Layout

```
~/.claude/plugins/cache/claude-mp/vsdd-factory/<version>/
  ├── plugins/                              # NEW — unified directory (replaces hook-plugins/)
  │   ├── plugins-registry.toml             # GENERATED by build.rs (replaces hooks-registry.toml as runtime source)
  │   ├── block-ai-attribution/             # MIGRATED from hook-plugins/block-ai-attribution.wasm
  │   │   ├── plugin.toml
  │   │   └── hooks/block-ai-attribution.wasm
  │   ├── legacy-bash-adapter/              # MIGRATED — shared adapter used by ~35 bash-backed hooks
  │   │   ├── plugin.toml
  │   │   └── hooks/legacy-bash-adapter.wasm
  │   ├── validate-stable-anchors/          # MIGRATED
  │   │   ├── plugin.toml
  │   │   └── hooks/validate-stable-anchors.wasm
  │   ├── ... (28 more migrated hook plugins)
  │   └── spec-kit/                         # NEW — SK-MCP-001 first plugin
  │       ├── plugin.toml                   # declares BOTH [hooks] AND [mcp]
  │       ├── hooks/vsdd-spec-kit-validator.wasm
  │       └── bin/spec-kit-mcp
  ├── hook-plugins/                         # DEPRECATED — kept as 1-release shim (rc.19)
  ├── hooks-registry.toml                   # DEPRECATED — dispatcher falls back to this if plugins-registry.toml absent
  ├── agents/                               # UNCHANGED
  ├── skills/                               # UNCHANGED
  ├── workflows/                            # UNCHANGED
  └── config/                              # UNCHANGED
```

The `hook-plugins/` directory and `hooks-registry.toml` file are kept for one release cycle
as a dispatcher fallback. They are removed in the release following migration validation.

### 3.2 plugin.toml Schema

Each plugin declares its capabilities in a single `plugin.toml` file. The format is TOML
for consistency with the existing `hooks-registry.toml`. A plugin may declare `[hooks]`,
`[mcp]`, or both. Sections not applicable to a plugin are simply omitted.

#### Full schema

```toml
[plugin]
id = "spec-kit"                             # [REQUIRED] kebab-case, unique in registry
version = "0.1.0"                           # [REQUIRED] semver string
description = "Spec graph integrity tooling for VSDD artifacts"
required_vsdd_factory = ">=1.0.0-rc.19"    # [REQUIRED] semver constraint

# ── Hook plugin section (optional) ──────────────────────────────────────────
[hooks]
# Each [[hooks.module]] declares one wasm binary and its event registrations.
# Multiple modules per plugin are allowed (one binary per event group is idiomatic).

[[hooks.module]]
file = "hooks/vsdd-spec-kit-validator.wasm"     # path relative to plugin root
events = ["PreToolUse:Edit", "PreToolUse:Write"]
priority = 500          # MUST be >= 500; core hooks occupy 20–460
timeout_ms = 5000
on_error = "block"

[[hooks.module]]
file = "hooks/vsdd-spec-kit-observer.wasm"
events = ["PostToolUse:Edit", "PostToolUse:Write"]
priority = 510
timeout_ms = 5000
on_error = "continue"
async = true

[hooks.module.capabilities.read_file]
path_allow = [".factory/specs"]

# ── MCP server section (optional) ───────────────────────────────────────────
[mcp]
binary = "bin/spec-kit-mcp"                 # path relative to plugin root
auto_register = true                        # true = add to settings.local.json at session-start
tool_prefix = "mcp__spec_kit__"             # tool names as seen by agents
args = ["--factory-root", ".factory"]       # additional CLI args passed to binary
description_override = "Spec graph integrity — query artifacts, trace citations, atomic mutations"
```

#### Schema invariants enforced by build.rs

- `plugin.id` must be unique across all entries in `plugins/`
- `hooks.module[*].priority` must be >= 500 (core hook band is 20–460)
- `hooks.module[*].on_error = "block"` implies `async` must be absent or `false` (preserves the
  `lint-registry-async-invariant` rule that currently applies to `hooks-registry.toml`)
- `mcp.binary` path must exist relative to the plugin root (checked at build time)
- `plugin.id` is used as the MCP server key in `settings.local.json`

#### Worked example: hooks-only plugin (migrated from existing)

```toml
# plugins/block-ai-attribution/plugin.toml

[plugin]
id = "block-ai-attribution"
version = "1.0.0"
description = "Block AI attribution markers in git commits"
required_vsdd_factory = ">=1.0.0-rc.1"

[[hooks.module]]
file = "hooks/block-ai-attribution.wasm"
events = ["PreToolUse:Bash"]
priority = 20
timeout_ms = 5000
on_error = "block"

[hooks.module.capabilities]
env_allow = []
```

#### Worked example: hooks + MCP plugin (SK-MCP-001)

```toml
# plugins/spec-kit/plugin.toml

[plugin]
id = "spec-kit"
version = "0.1.0"
description = "Spec graph integrity tooling for VSDD artifacts"
required_vsdd_factory = ">=1.0.0-rc.19"

[[hooks.module]]
file = "hooks/vsdd-spec-kit-validator.wasm"
events = ["PreToolUse:Edit", "PreToolUse:Write"]
priority = 500
timeout_ms = 10000
on_error = "block"

[hooks.module.capabilities.read_file]
path_allow = [".factory/specs", ".factory/stories"]

[mcp]
binary = "bin/spec-kit-mcp"
auto_register = true
tool_prefix = "mcp__spec_kit__"
args = ["--factory-root", ".factory"]
description_override = "Spec graph integrity — query artifacts, trace citations, atomic mutations"
```

#### Worked example: MCP-only plugin (hypothetical future analytics)

```toml
# plugins/factory-analytics/plugin.toml

[plugin]
id = "factory-analytics"
version = "0.1.0"
description = "Usage telemetry MCP bridge for factory analytics dashboard"
required_vsdd_factory = ">=1.0.0-rc.20"

[mcp]
binary = "bin/factory-analytics-mcp"
auto_register = false     # opt-in; resource-intensive
tool_prefix = "mcp__factory_analytics__"
```

### 3.3 plugins-registry.toml (Build-Generated Aggregate)

`plugins-registry.toml` is generated at build time by `build.rs`. It is the runtime source
of truth for the dispatcher. The dispatcher reads it; it does not read individual
`plugin.toml` files at runtime.

```toml
# plugins-registry.toml — GENERATED by build.rs. Do not edit by hand.
# Source of truth: plugins/*/plugin.toml
schema_version = 1

[[plugin]]
id = "block-ai-attribution"
version = "1.0.0"
plugin_root = "plugins/block-ai-attribution"
hooks = [
  { file = "hooks/block-ai-attribution.wasm", events = ["PreToolUse:Bash"], priority = 20, on_error = "block" }
]

[[plugin]]
id = "legacy-bash-adapter"
version = "1.0.0"
plugin_root = "plugins/legacy-bash-adapter"
hooks = [
  { file = "hooks/legacy-bash-adapter.wasm", events = ["PreToolUse:Edit", "PreToolUse:Write", "PostToolUse:Edit", "PostToolUse:Write"], priority = 0, on_error = "continue" }
]
# Note: legacy-bash-adapter hooks are registered with individual priorities via the
# hooks-registry.toml entries that use it; its plugin.toml priority is advisory only.
# See §3.4 for legacy-bash-adapter handling during migration.

[[plugin]]
id = "spec-kit"
version = "0.1.0"
plugin_root = "plugins/spec-kit"
hooks = [
  { file = "hooks/vsdd-spec-kit-validator.wasm", events = ["PreToolUse:Edit", "PreToolUse:Write"], priority = 500, on_error = "block" }
]
mcp_binary = "plugins/spec-kit/bin/spec-kit-mcp"
mcp_tool_prefix = "mcp__spec_kit__"
mcp_auto_register = true
mcp_args = ["--factory-root", ".factory"]
```

The dispatcher's load sequence on startup:
1. Check for `plugins/plugins-registry.toml` (new path)
2. If absent, fall back to `hooks-registry.toml` (legacy path, deprecated)
3. Log deprecation warning if fallback is used

---

## 4. Dispatcher Behavior

### 4.1 Hook Plugin Handling (Unchanged in Essence)

The dispatcher's existing hook dispatch logic changes only its data source:

- Before: reads `hooks-registry.toml` directly
- After: reads `plugins/plugins-registry.toml`, extracts all `[[plugin]]` entries with `hooks`
  arrays, flattens them into the priority-ordered hook chain for each event

No change to how wasm binaries are loaded, invoked, or their results processed. Existing
hooks that use `legacy-bash-adapter.wasm` continue to work identically — `legacy-bash-adapter`
becomes a plugin with a `plugin.toml` but its wasm binary is unchanged.

**Legacy-bash-adapter migration note:** The 35 hooks that route through `legacy-bash-adapter.wasm`
share a single wasm binary but have individual entries in `hooks-registry.toml` with distinct
priorities, event filters, `[hooks.config]` script_path values, and capabilities tables. During
migration, these entries are preserved in `plugins-registry.toml` as-is — each becomes a
hook entry under the `legacy-bash-adapter` plugin with its original priority, event, and
config fields embedded in an `extended_config` table. Build.rs emits them verbatim from the
existing `hooks-registry.toml` entries tagged with `plugin = "hook-plugins/legacy-bash-adapter.wasm"`.

### 4.2 MCP Plugin Handling (New)

On each session start (dispatching the `SessionStart` event), the dispatcher:

1. Reads `plugins/plugins-registry.toml`
2. For each `[[plugin]]` with `mcp_auto_register = true`, checks project's
   `.claude/settings.local.json` `mcpServers` section
3. If the plugin's `mcp_auto_register = true` entry is absent in `mcpServers`, adds it:

```json
{
  "mcpServers": {
    "spec-kit": {
      "command": "~/.claude/plugins/cache/claude-mp/vsdd-factory/1.0.0-rc.19/plugins/spec-kit/bin/spec-kit-mcp",
      "args": ["--factory-root", ".factory"],
      "_managed_by_vsdd_factory": true,
      "_plugin_version": "0.1.0",
      "_factory_version": "1.0.0-rc.19"
    }
  }
}
```

4. If the entry IS present but the `command` path points to a different factory version
   directory (stale path from prior upgrade), updates the `command` path and `_factory_version`
   annotation in place
5. Emits a session-start telemetry event for each MCP entry added or updated
6. Skips all entries that lack `_managed_by_vsdd_factory: true` — user-added MCP entries
   (Perplexity, Context7, Tavily, Playwright) are never touched

**Path resolution:** The `command` path uses the absolute path to the installed plugin binary.
The dispatcher expands `~` using the home directory at write time, not at read time by
`.claude/settings.local.json`, because MCP server registration requires an absolute binary path.

**Idempotency:** If `command` matches the current factory version's binary path, no write
occurs. The check is a string equality test on the resolved absolute path.

**Removal:** Entries marked `_managed_by_vsdd_factory: true` are removed when:
- The vsdd-factory package detects the plugin is no longer in `plugins-registry.toml`
  (i.e., the plugin was removed from the factory release)
- In future: a per-project deactivation mechanism (out of scope for this proposal)

### 4.3 MCP Auto-Registration: .claude/settings.local.json Precondition

`.claude/settings.local.json` may not exist on a new project. The dispatcher creates it
(with only the `mcpServers` key) if absent. If it exists and already has a `mcpServers`
section, the dispatcher merges into it. The write is atomic: read → merge → write, protected
against concurrent modification by a file lock held for the duration.

---

## 5. Migration of 59 Hook Registrations

The existing `hooks-registry.toml` has 59 `[[hooks]]` entries covering 31 distinct wasm
binaries. Migration is mechanical and preserves all existing behavior.

### 5.1 Migration Strategy per Plugin Type

**Native-WASM plugins (21 entries, ~15 distinct binaries):**
Each gets its own `plugins/<id>/` directory:

```
plugins/
  block-ai-attribution/
    plugin.toml       # [hooks] section with events, priority, capabilities
    hooks/block-ai-attribution.wasm   # binary moved from hook-plugins/
  capture-commit-activity/
    plugin.toml
    hooks/capture-commit-activity.wasm
  ... (13 more native-WASM plugins)
```

The `plugin.toml` for each is mechanically generated from the corresponding `[[hooks]]`
entry in `hooks-registry.toml` — same event, priority, timeout_ms, on_error, async,
capabilities fields.

**Legacy-bash-adapter plugins (~35 entries, 1 shared binary):**
The `legacy-bash-adapter.wasm` binary becomes its own plugin. The 35 hooks that use it
are represented as entries in `plugins-registry.toml` with an `extended_config` section
carrying the `script_path` and capabilities that today live in `[hooks.config]` and
`[hooks.capabilities]` tables:

```toml
# In plugins-registry.toml (generated):
[[plugin]]
id = "convergence-tracker"
version = "1.0.0"
plugin_root = "plugins/convergence-tracker"
hooks = [
  {
    file = "hooks/legacy-bash-adapter.wasm",   # relative to legacy-bash-adapter plugin root
    events = ["PostToolUse:Edit", "PostToolUse:Write"],
    priority = 210,
    timeout_ms = 10000,
    on_error = "continue",
    async = true,
    extended_config = { script_path = "hooks/convergence-tracker.sh" },
    capabilities = {
      env_allow = ["PATH", "HOME", "TMPDIR", "CLAUDE_PROJECT_DIR", "CLAUDE_PLUGIN_ROOT", "VSDD_SESSION_ID"],
      exec_subprocess = { binary_allow = ["bash", "jq"] }
    }
  }
]
```

The `plugin_root` for a legacy-bash-adapter plugin points to a minimal directory:

```
plugins/convergence-tracker/
  plugin.toml     # hooks section declares legacy-bash-adapter.wasm + script_path
  # no hooks/ subdirectory — binary lives in plugins/legacy-bash-adapter/hooks/
```

The `file` path in the generated `plugins-registry.toml` is resolved relative to the
`plugin_root` of the ADAPTER plugin (`plugins/legacy-bash-adapter/`), not the declaring
plugin. Build.rs uses an explicit `adapter_plugin_id` field in `plugin.toml` for this:

```toml
# plugins/convergence-tracker/plugin.toml

[plugin]
id = "convergence-tracker"
version = "1.0.0"
description = "Track convergence metrics on spec edits"
required_vsdd_factory = ">=1.0.0-rc.1"

[[hooks.module]]
adapter_plugin_id = "legacy-bash-adapter"    # resolved to plugins/legacy-bash-adapter/hooks/legacy-bash-adapter.wasm
events = ["PostToolUse:Edit", "PostToolUse:Write"]
priority = 210
timeout_ms = 10000
on_error = "continue"
async = true
script_path = "hooks/convergence-tracker.sh"   # path in the vsdd-factory install tree (not in this plugin dir)

[hooks.module.capabilities]
env_allow = ["PATH", "HOME", "TMPDIR", "CLAUDE_PROJECT_DIR", "CLAUDE_PLUGIN_ROOT", "VSDD_SESSION_ID"]

[hooks.module.capabilities.exec_subprocess]
binary_allow = ["bash", "jq"]
shell_bypass_acknowledged = "legacy-bash-adapter runs unported hooks"
env_allow = ["PATH", "HOME", "TMPDIR", "CLAUDE_PROJECT_DIR", "CLAUDE_PLUGIN_ROOT", "VSDD_SESSION_ID"]
```

### 5.2 Build.rs Migration Script (One-Shot)

The `build.rs` in the vsdd-factory plugin package root performs migration at build time:

```rust
// build.rs — pseudocode for plugin system migration

fn main() {
    // Step 1: Parse hooks-registry.toml (the existing source of truth)
    let legacy_hooks = parse_hooks_registry("plugins/vsdd-factory/hooks-registry.toml");

    // Step 2: Parse all existing plugins/*/plugin.toml files
    let existing_manifests = glob("plugins/vsdd-factory/plugins/*/plugin.toml")
        .map(parse_plugin_manifest)
        .collect();

    // Step 3: For each hook entry in legacy_hooks not already in existing_manifests,
    //         generate a plugin.toml in plugins/vsdd-factory/plugins/<id>/
    for hook in legacy_hooks.iter().filter(|h| !existing_manifests.contains(h.name)) {
        let plugin_dir = format!("plugins/vsdd-factory/plugins/{}", hook.name);
        std::fs::create_dir_all(&plugin_dir)?;
        write_plugin_manifest(&plugin_dir, hook)?;
        // Move wasm binary from hook-plugins/ to plugins/<id>/hooks/ (copy during migration window)
        copy_wasm_binary(&hook.plugin_binary_path, &plugin_dir)?;
    }

    // Step 4: Validate all manifests (priority >= 500 for plugin band; legacy hooks exempt)
    for manifest in existing_manifests.iter().filter(|m| !m.is_legacy_migrated) {
        validate_plugin_priority(&manifest)?;
    }

    // Step 5: Generate plugins-registry.toml from all plugin.toml files
    let registry = aggregate_plugin_manifests(&existing_manifests);
    write_registry("plugins/vsdd-factory/plugins/plugins-registry.toml", &registry)?;

    // Step 6: cargo:rerun-if-changed directives
    println!("cargo:rerun-if-changed=plugins/vsdd-factory/plugins");
    println!("cargo:rerun-if-changed=plugins/vsdd-factory/hooks-registry.toml");
}
```

**Priority floor exception for migrated hooks:** The 59 existing hook registrations use
priorities from 20 to 460. During migration, their priority values are preserved verbatim.
The 500+ floor applies ONLY to NEW plugins added after the migration (i.e., spec-kit and
any future plugins). The `plugin.toml` schema marks migrated hooks with
`migration_source = "hooks-registry.toml"` which suppresses the priority-floor validation.

### 5.3 Behavioral Preservation Invariants

After migration, the dispatcher must preserve:

1. **Priority order:** All 59 hooks fire in the same relative priority order as today.
2. **Event binding:** Each hook fires on exactly the same events (PreToolUse:Bash, PostToolUse:Edit|Write, etc.)
3. **Capability grants:** Every `env_allow`, `binary_allow`, `path_allow` value is preserved.
4. **on_error semantics:** `block` vs `continue` values are unchanged.
5. **async flags:** All existing `async = true` annotations are preserved.
6. **Legacy-bash-adapter config:** `script_path` values are preserved and resolvable.

CI validates invariants 1-6 by running `build.rs` in validation mode and diffing the
generated `plugins-registry.toml` against a snapshot of the expected hook chain derived
from `hooks-registry.toml`.

### 5.4 Migration Window and Rollback

- **rc.19:** Both `hook-plugins/` and `plugins/` directories ship. Dispatcher prefers
  `plugins/plugins-registry.toml` but falls back to `hooks-registry.toml` with a
  deprecation warning if the new registry is absent or fails to parse.
- **rc.20:** `hook-plugins/` directory and `hooks-registry.toml` removed. Dispatcher
  requires `plugins/plugins-registry.toml`.

During the rc.19 window, any regression in the new registry can be diagnosed by inspecting
dispatcher logs. If the new registry produces incorrect hook ordering, the admin can force
fallback to the legacy registry by removing `plugins/plugins-registry.toml` (emergency only).

---

## 6. SK-MCP-001 Cross-Reference Amendments

The existing SK-MCP-001 spec (`spec-kit-mcp.md`) was written before UNI-PLUG-001 existed.
Four amendments are required to align it with the unified plugin system:

### Amendment 1: Add prerequisite frontmatter

Add to SK-MCP-001 frontmatter:
```yaml
prerequisite: UNI-PLUG-001 (unified plugin system)
```

Rationale: SK-MCP-001 Tier 1 cannot be implemented until UNI-PLUG-001 Tier 1 (the plugin
manifest format and `build.rs` aggregator) exists. The dependency is hard.

### Amendment 2: Update canonical target path

SK-MCP-001 Section 2.3 "Distribution" currently shows:
```
~/.claude/plugins/cache/claude-mp/vsdd-factory/<version>/
  bin/
    spec-kit-mcp
  hook-plugins/
    vsdd-spec-kit-validator.wasm
```

After UNI-PLUG-001, the canonical layout is:
```
~/.claude/plugins/cache/claude-mp/vsdd-factory/<version>/
  plugins/
    spec-kit/
      plugin.toml
      bin/spec-kit-mcp
      hooks/vsdd-spec-kit-validator.wasm
```

The flat `bin/` and `hook-plugins/` directories no longer receive new plugins.

### Amendment 3: Hook priority must be >= 500

SK-MCP-001 does not currently specify a hook priority for `vsdd-spec-kit-validator.wasm`.
The amendment must add an explicit priority declaration of 500 or higher. This is enforced
by `build.rs` schema validation for all new (non-migrated) plugins.

Recommended: `priority = 500` (lowest priority in the plugin band, fires after all core hooks).

### Amendment 4: Replace manual registration instructions

SK-MCP-001 Section 2.4 "Auto-Discovery" describes a manual process where the
`devops-engineer` agent adds the MCP server to `.claude/settings.local.json` as part of
a `/vsdd-factory:onboard-observability` or `/vsdd-factory:setup-spec-kit` flow.

After UNI-PLUG-001, the correct mechanism is:
- Declare `auto_register = true` in `plugins/spec-kit/plugin.toml`
- The dispatcher handles registration at session-start automatically
- No manual skill invocation required; no manual `hooks-registry.toml` editing required

The amendment should replace the manual-registration prose with: "spec-kit is registered
automatically by the factory-dispatcher on session-start via the UNI-PLUG-001 unified plugin
system. No manual `settings.local.json` editing is required."

**Recommendation:** Amend SK-MCP-001 in the same session as UNI-PLUG-001 Tier 1
implementation to prevent spec drift. The amendments are editorial (no behavioral change to
SK-MCP-001's tool surface or proof obligations) and can be applied without re-review of
the full SK-MCP-001 spec.

### S-15.14 Priority Note (Amendment I)

vsdd-factory S-15.14 ships `validate-dispatch-advance` registered in hooks-registry.toml
at **priority 154** — inside the core hook band (20–460). This is consistent with UNI-PLUG-001
§5.2: the 500+ floor applies to NEW plugins added after the migration. `validate-dispatch-advance`
is a pre-existing core hook with `migration_source = "hooks-registry.toml"` exemption, so its
priority 154 registration is valid and does not violate the plugin band invariant.

When SK-MCP-001 Tier 2 `vsdd-spec-kit-validator.wasm` lands at priority 500+, the architect
adjudicates whether `validate-dispatch-advance` is:

- **Option A (migrate):** INV-011 logic subsumed by `vsdd-spec-kit-core`; hook priority moves
  to the 500+ plugin band; per-class hook retired. spec-kit becomes the single surface for all
  ops-layer invariants.
- **Option B (defense-in-depth sibling):** Hook remains at priority 154 as an early-gate for
  STATE.md writes; spec-kit at priority 500+ handles the broader spec-layer invariants.
  Both fire independently on `.factory/**/*.md` writes.

This is recorded as **OQ-UNI-005** for human adjudication (see §11).

---

## 7. Backward Compatibility

### 7.1 Existing Hook Registrations

Zero behavioral change. Every one of the 59 existing hook registrations fires in the same
priority order, on the same events, with the same capabilities, with the same on_error
semantics. The migration is a file-layout change and a registry-format change. Wasm binaries
are identical.

### 7.2 Existing User-Added MCP Entries

Perplexity, Context7, Tavily, and Playwright are currently in `.claude/settings.local.json`
as manually-added entries without `_managed_by_vsdd_factory: true`. The dispatcher's
session-start MCP sync logic has a hard invariant: entries lacking `_managed_by_vsdd_factory: true`
are never read, modified, or removed. These four entries are untouched by this proposal.

### 7.3 `lint-registry-async-invariant` Hook

This hook currently fires on writes to `plugins/vsdd-factory/hooks-registry.toml` and
validates the `on_error = "block"` implies `async = false` invariant. After migration:
- The hook must be updated to also fire on writes to `plugins/vsdd-factory/plugins/plugins-registry.toml`
- The invariant itself is unchanged: enforced via `plugin.toml` schema validation in `build.rs`
  AND the lint hook as a defense-in-depth runtime check
- The `path_allow` in the hook's capabilities table must include the new registry path

### 7.4 `hooks-registry.toml` as Canonical Source-of-Truth (Transitional)

During the rc.19 migration window, `hooks-registry.toml` remains the source of truth for
existing hooks. The `build.rs` reads it to generate `plugins-registry.toml`. This means:
- Additions to existing hook behavior (e.g., new script_path for a bash-adapter hook) are
  still made in `hooks-registry.toml` during rc.19
- New plugin registrations (e.g., spec-kit) go directly into `plugins/<id>/plugin.toml`
- `build.rs` merges both sources into `plugins-registry.toml`

In rc.20, `hooks-registry.toml` is removed. All additions go to `plugins/<id>/plugin.toml`.

---

## 8. Implementation Roadmap

### Tier 1: Foundation (~3-5 days)

Goal: plugin manifest format, build.rs aggregator, dispatcher reads new registry.
No behavioral change to existing hooks. No MCP support yet.

- Define `plugin.toml` schema as Rust serde structs with JSON Schema for CI validation
- `build.rs` migration script:
  - Parse `hooks-registry.toml` source of truth
  - Generate `plugins/<id>/plugin.toml` for each of the 31 distinct wasm binaries
  - Copy wasm binaries from `hook-plugins/` to `plugins/<id>/hooks/`
  - Emit `plugins/plugins-registry.toml` as merged aggregate
- Priority-floor validation in `build.rs` (rejects priority < 500 for non-migrated plugins)
- Dispatcher reads `plugins/plugins-registry.toml` (with `hooks-registry.toml` fallback)
- CI job: `validate-plugin-manifests` — runs `build.rs` in check mode, diffs registry
- Update `lint-registry-async-invariant` hook to cover `plugins-registry.toml` path
- Documentation: `plugins/vsdd-factory/plugins/README.md` (plugin authoring guide)

Deliverable: dispatcher functions identically to today; registry format is migrated;
`hooks-registry.toml` is deprecated but present.

### Tier 2: MCP Plugin Support (~3-5 days)

Goal: dispatcher session-start MCP auto-registration; spec-kit as first hooks+MCP plugin.

- Dispatcher reads `mcp_auto_register` entries from `plugins-registry.toml`
- Idempotent `settings.local.json` MCP registration with `_managed_by_vsdd_factory: true`
- Stale path detection: if MCP entry exists but `command` path points to old factory version,
  update the path in place
- `plugins/spec-kit/plugin.toml` declaring `[hooks]` + `[mcp]`
- SK-MCP-001 amendments applied (§6 of this document)
- End-to-end test: session-start triggers spec-kit MCP registration; `spec-kit-mcp` binary
  is callable from an agent; `vsdd-spec-kit-validator.wasm` hook fires on spec edits

Deliverable: spec-kit is the first plugin exercising both plugin types under the unified system.

### Tier 3: Cleanup (~2-3 days)

Goal: remove deprecated legacy infrastructure; update all cross-references.

- Remove `hook-plugins/` directory (wasm binaries now only in `plugins/<id>/hooks/`)
- Remove `hooks-registry.toml`
- Remove dispatcher fallback logic to `hooks-registry.toml`
- Update all vsdd-factory documentation, CLAUDE.md references, and skill text that
  references `hooks-registry.toml` to reference `plugins/<id>/plugin.toml` or
  `plugins/plugins-registry.toml` as appropriate
- Update `artifact-path-registry.yaml` if any `.factory/` paths related to plugins are added

Total: ~8-13 days of implementation work. SK-MCP-001 Tier 1 can begin as soon as
UNI-PLUG-001 Tier 1 is complete.

---

## 9. Relationship to PA-MOD-001

PA-MOD-001 (plugin-module architecture) was the broader exploratory proposal for bundling
five extension surfaces — hooks, MCP, agents, skills, workflows — into unified plugin modules
with install/uninstall lifecycle management. Per user direction Q1=A, the broader scope was
rejected in favor of the dispatcher-only narrow scope defined in this document.

| Dimension | PA-MOD-001 (superseded) | UNI-PLUG-001 (canonical) |
|-----------|------------------------|------------------------|
| Surfaces unified | 5 (hooks + MCP + agents + skills + workflows) | 2 (hooks + MCP) |
| Lifecycle | install/uninstall/upgrade per project | vsdd-factory release ships all plugins |
| Plugin marketplace | designed for future community registry | vsdd-factory-internal only |
| Agents/skills directories | restructured into plugin-modules/ | unchanged |
| Effort | ~3-4 weeks | ~1-2 weeks |
| SK-MCP-001 prerequisite | yes | yes |

PA-MOD-001 stays as REFERENCE at `.factory/proposals/plugin-module-architecture.md` for
future consideration if broader bundling (install/uninstall, agents, skills) becomes
desirable. UNI-PLUG-001 is the canonical implementation target.

Extensibility note: `plugin.toml` is designed to accommodate future PA-MOD-001 surfaces
without breaking schema compatibility. Future `[agents]`, `[skills]`, and `[workflows]`
sections would be additive. Existing hooks-only and hooks+MCP plugins written against
UNI-PLUG-001 would require no manifest changes to coexist with plugins that declare those
future sections.

---

## 10. Risks and Mitigations

| Risk | Likelihood | Impact | Mitigation |
|------|-----------|--------|-----------|
| 59-hook migration introduces ordering regression | LOW | HIGH | `build.rs` validation mode diffs generated registry against expected hook chain; CI blocks if priority order diverges |
| Legacy-bash-adapter `script_path` resolution breaks after binary relocation | MEDIUM | HIGH | `build.rs` verifies all `script_path` values resolve relative to install tree at build time; dispatcher logs resolution errors as FATAL |
| MCP `settings.local.json` write overwrites user-added entry | LOW | HIGH | Hard invariant: only entries with `_managed_by_vsdd_factory: true` are touched; read-before-write merges rather than overwrites |
| `lint-registry-async-invariant` hook misses new registry path | MEDIUM | MEDIUM | Tier 1 scope: update hook `path_allow` and event filter to cover `plugins-registry.toml`; CI regression test on hook |
| Plugin priority collision (two new plugins claim same priority at same event) | MEDIUM | MEDIUM | `build.rs` detects priority conflicts at build time for same-event same-priority declarations; CI blocks |
| Future need for PA-MOD-001 broader bundling forces re-architecture | LOW | LOW | `plugin.toml` schema is designed for additive extension; future `[agents]`, `[skills]` sections add without breaking existing manifests |
| Dispatcher fallback to `hooks-registry.toml` masks broken migration | MEDIUM | MEDIUM | Fallback emits a deprecation WARNING visible in session-start telemetry; monitoring alerts on fallback usage |
| `settings.local.json` file lock contention (concurrent Claude sessions) | LOW | LOW | File is written at session-start before any agent dispatch; concurrent writes are rare; OS-level file locking with retry |

---

## 11. Open Questions for Human Adjudication

**OQ-UNI-001: Build-generated vs hand-written `plugins-registry.toml`?**

- Option A (recommended): Build-script-generated from individual `plugin.toml` manifests.
  Single source of truth. No risk of registry drifting from manifests.
- Option B: Hand-written by plugin authors, with CI validation against individual manifests.
  More explicit control; more maintenance burden.
- **Recommend A.** The aggregate is purely derived from the manifests; generating it is
  deterministic and eliminates a whole class of drift.

**OQ-UNI-002: Plugin hook priority band floor — 500 or 1000?**

- 500 leaves 40 slots between core max (460) and plugin min, which is sufficient for
  core hook growth.
- 1000 creates a clearer separation with a wider buffer zone.
- **Recommend 500.** Core hooks currently top out at 460. A 40-unit buffer is sufficient.
  If core hooks grow past 500 before this proposal lands, the floor moves to 600 — the
  point is enforced separation, not a specific number.

**OQ-UNI-003: Should `auto_register = false` plugins appear in `plugins-registry.toml`?**

- Option A (recommended): Yes — for discoverability. A future `/vsdd-factory:list-plugins`
  can show available MCP servers even if not auto-registered, allowing opt-in activation.
- Option B: No — omit from registry. Simpler; avoids confusion about "registered but not active."
- **Recommend A.** Discoverability value outweighs the marginal complexity. The `auto_register`
  boolean makes opt-in vs opt-out explicit.

**OQ-UNI-004: Should the `plugins/` directory ship with individual `plugin.toml` files, or
only `plugins-registry.toml`?**

- Option A (recommended): Ship both. Individual `plugin.toml` files are useful for debugging
  and human inspection. `plugins-registry.toml` is the runtime source; `plugin.toml` files
  are the authoring source.
- Option B: Ship only `plugins-registry.toml`. Smaller package; less filesystem surface.
- **Recommend A.** The debugging value of human-readable per-plugin manifests is high.
  Package size impact is negligible (each `plugin.toml` is <50 lines of TOML).

**OQ-UNI-005: When SK-MCP-001 Tier 2 ships, should `validate-dispatch-advance` be migrated
under spec-kit (Option A) or kept as a defense-in-depth sibling at priority 154 (Option B)?**

Context: vsdd-factory S-15.14 ships `validate-dispatch-advance` at priority 154 (core hook
band, `migration_source` exemption). The hook enforces INV-011 (STATE.md current_step BC PC
compliance). When `vsdd-spec-kit-validator.wasm` arrives at priority 500+, both hooks would
fire on STATE.md writes — one at 154, one at 500+.

- **Option A (migrate):** Cleaner long-term. spec-kit becomes the single enforcement surface.
  `validate-dispatch-advance` is retired; its INV-011 logic moves into `vsdd-spec-kit-core`.
  Priority 154 slot freed for future core hooks.
- **Option B (defense-in-depth sibling):** Safer during ramp. If spec-kit Tier 2 has early
  false-positive issues, INV-011 remains enforced via the purpose-built hook. Two hooks, same
  invariant — redundancy is explicit and intentional.
- **Preliminary recommendation:** Option B at rc.19 → rc.20 transition (while spec-kit false-
  positive rate is measured); migrate to Option A at rc.21 or when spec-kit achieves
  `block_severity = "HIGH"` hardening with zero false positives on INV-011.
- **Escalation level:** Human adjudication required. The architect recommends Option B for
  the transition window; the human confirms or overrides the migration timing.

---

## Appendix A: Priority Band Reference

Current priority assignments (from `hooks-registry.toml`):

| Band | Range | Examples |
|------|-------|---------|
| Core PreToolUse guards | 20–160 | block-ai-attribution (20), brownfield-discipline (30), red-gate (100), validate-stable-anchors (155) |
| Core PostToolUse validators | 210–460 | convergence-tracker (210), validate-template-compliance (400), lint-registry-async-invariant (160) |
| Core Stop/SubagentStop | 910–960 | session-learning (910), validate-per-story-adversary-convergence (960) |
| **Plugin band (new)** | **500+** | spec-kit (500), future plugins (500+) |

The lint-registry-async-invariant hook's own priority (160) is in the PostToolUse range.
Its file-scope enforcement (only fires when `hooks-registry.toml` or `plugins-registry.toml`
is written) is handled inside the wasm binary, not via the priority mechanism.

---

## Appendix B: Worked Migration Example — `validate-stable-anchors`

**Before (hooks-registry.toml entry):**

```toml
[[hooks]]
name = "validate-stable-anchors"
event = "PreToolUse"
tool = "Edit|Write"
plugin = "hook-plugins/validate-stable-anchors.wasm"
priority = 155
timeout_ms = 5000
on_error = "block"

[hooks.capabilities.read_file]
path_allow = [".factory/specs"]
```

**After (plugins/validate-stable-anchors/plugin.toml):**

```toml
[plugin]
id = "validate-stable-anchors"
version = "1.0.0"
description = "Block volatile line-number citations in spec files (TD-VSDD-091)"
required_vsdd_factory = ">=1.0.0-rc.1"
migration_source = "hooks-registry.toml"   # suppresses priority-floor validation

[[hooks.module]]
file = "hooks/validate-stable-anchors.wasm"
events = ["PreToolUse:Edit", "PreToolUse:Write"]
priority = 155    # preserved from original; below 500 floor, exempt as migrated hook
timeout_ms = 5000
on_error = "block"

[hooks.module.capabilities.read_file]
path_allow = [".factory/specs"]
```

**After (plugins-registry.toml entry, generated):**

```toml
[[plugin]]
id = "validate-stable-anchors"
version = "1.0.0"
plugin_root = "plugins/validate-stable-anchors"
migration_source = "hooks-registry.toml"
hooks = [
  {
    file = "hooks/validate-stable-anchors.wasm",
    events = ["PreToolUse:Edit", "PreToolUse:Write"],
    priority = 155,
    timeout_ms = 5000,
    on_error = "block",
    capabilities = { read_file = { path_allow = [".factory/specs"] } }
  }
]
```

---

## Appendix C: artifact-path-registry.yaml Assessment

The `artifact-path-registry.yaml` does not currently register a `proposal` artifact type.
PA-MOD-001 noted this as a potential write-block issue (the `validate-artifact-path` hook
blocks writes to `.factory/` paths matching no registry entry).

The `proposals/` directory appears in `.factory/proposals/` and does not match any current
registry pattern. Both PA-MOD-001 and this document (UNI-PLUG-001) were written there
successfully, which implies one of:

1. The `validate-artifact-path` hook has a `warn` or `advisory` path for unmapped `.factory/`
   subdirectories that don't match any registered `canonical_path_pattern`
2. The hook's TOML registry read was cached from an earlier session where `proposals/` was
   not a registered path

**Recommendation for human adjudication:** Register `proposal` as an artifact type in
`artifact-path-registry.yaml` to make the path canonical and block-enforced. Suggested entry:

```yaml
- artifact_type: proposal
  canonical_path_pattern: ".factory/proposals/{filename}.md"
  description: Architecture and feature proposals (UNI-PLUG-001, PA-MOD-001, SK-MCP-001, etc.)
  enforcement_level: block
```

This is a one-line addition to `artifact-path-registry.yaml` and closes the ambiguity.
The `validate-artifact-path` hook already handles this file in `path_allow`, so adding
the entry is additive-only with no behavior change for currently-valid writes.

Deferral risk: if the hook's current behavior is `ARTIFACT_PATH_UNREGISTERED` → continue
(not block), then deferral has zero immediate cost. If the hook's behavior is
`ARTIFACT_PATH_UNREGISTERED` → block, then the two existing proposals were written under
a hook misconfiguration and the entry should be added before writing further proposals.
The architect recommends adding the entry now regardless, to make the registry the complete
source of truth for `.factory/` paths.
