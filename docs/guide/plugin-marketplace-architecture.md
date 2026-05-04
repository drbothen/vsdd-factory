# Plugin & Marketplace Architecture

This document explains how the vsdd-factory plugin is packaged for Claude Code,
why it's structured the way it is, and how to evolve it without re-introducing
the install-time bug that v1.0.0-rc.4 / rc.5 shipped with.

> **TL;DR.** A Claude Code plugin needs two manifest files (`marketplace.json`
> and `plugin.json`) and one of five `source` types. Picking the wrong source
> type silently breaks `claude plugin install` even though `claude plugin
> validate` reports success. We hit that. This doc captures everything we
> learned so we don't hit it again.

---

## 1. Overview: how Claude Code loads a plugin

A Claude Code plugin is a directory containing **at least** a
`.claude-plugin/plugin.json` manifest and one or more of: `skills/`,
`agents/`, `commands/`, `hooks/`, MCP server configs.

Plugins are distributed through a **marketplace** — a separate manifest
(`marketplace.json`) that lists one or more plugins and tells Claude Code
where to fetch each plugin's files from. A marketplace can be its own
GitHub repo, or the same repo as the plugin (our case).

Install flow when a user runs `/plugin install <plugin>@<marketplace>`:

```
1. Marketplace repo cloned to:       ~/.claude/plugins/marketplaces/<marketplace>/
2. Plugin source copied to cache:    ~/.claude/plugins/cache/<marketplace>/<plugin>/<version>/
3. Registry entry written to:        ~/.claude/plugins/installed_plugins.json
4. Plugin loader scans cache dir for SKILL.md, agent .md, hooks.json, etc.
```

**Step 2 is where vsdd-factory used to silently fail.** The recorded
`installPath` in `installed_plugins.json` pointed at a directory that was
never created, because the marketplace.json `source` field used a
schema-violating combination (`source: "github"` + `path:`). The plugin
loader then found zero skills despite the manifest validating "ok".

---

## 2. The 5 valid source types

Per the [official Claude Code marketplace docs](https://code.claude.com/docs/en/plugin-marketplaces.md#plugin-sources),
these are the only forms a `source` field may take:

### 2.1. Relative path (string form)

```json
"source": "./plugins/my-plugin"
```

Use when **the marketplace and plugin live in the same git repo**. The
string is interpreted relative to the marketplace.json file. This is what
vsdd-factory uses today, and what dclaude/wclaude/zclaude use (they pass
`"./"` because their plugin lives at the repo root).

### 2.2. `github` source (object form)

```json
"source": {
  "source": "github",
  "repo": "owner/repo",
  "ref": "main",      // optional — branch, tag, or commit-ish
  "sha": "abc123..."  // optional — pin exact commit
}
```

Use when **the entire external GitHub repo IS the plugin** (plugin files
at repo root). No `path` field. The whole repo gets cloned and treated as
the plugin root. dclaude/wclaude/zclaude could use this form if installed
from external repos; instead they use `"./"` because their marketplace
lives in the same repo as the plugin.

> **CRITICAL:** Adding a `path:` field to a `github` source is INVALID.
> The `github` type has no `path` field per the schema. Claude Code's
> manifest validator does NOT catch this; install runs but the cache copy
> step skips silently. **This is the bug we shipped in rc.4 / rc.5.**
> If you need a subpath inside an external repo, use `git-subdir` (§2.4).

### 2.3. `url` source

```json
"source": {
  "source": "url",
  "url": "https://example.com/path/to/plugin.git",
  "ref": "main"
}
```

Generic git URL form. Same semantics as `github`: the whole cloned tree
is the plugin root. No `path` field.

### 2.4. `git-subdir` source

```json
"source": {
  "source": "git-subdir",
  "url": "https://github.com/owner/monorepo.git",
  "path": "tools/claude-plugin"
}
```

Use when **the plugin lives at a subpath of an external git repo**. Uses
sparse-checkout to clone only that subpath. This is the *correct* form
for "external repo, plugin nested under a subdirectory" — it is what we
mistakenly tried to express via the invalid `github + path` combination.

### 2.5. `npm` source

```json
"source": {
  "source": "npm",
  "package": "@org/plugin-name",
  "version": "^1.2.0"
}
```

For plugins distributed as npm packages.

---

## 3. Decision tree: which source type for our use case?

```
Do you need to pin the install to a specific git ref (branch/tag)?
├── YES → Use github / url / git-subdir OBJECT (all support ref + sha)
│         ├── Plugin at repo root           → github/url object
│         └── Plugin at a subpath           → git-subdir object   ← vsdd-factory today
└── NO  → Is the plugin in the same repo as the marketplace?
         ├── YES → Relative-path STRING:    "source": "./path/to/plugin"
         └── NO  → Use github/url OBJECT (no ref → marketplace's default branch)
```

**vsdd-factory today:** marketplace and plugin live in the same repo
(`drbothen/vsdd-factory`); plugin files at `plugins/vsdd-factory/`. We
follow Git Flow — the GitHub default branch is `develop` (in-flight work)
and tagged releases live on `main`. We need installs to read from `main`,
not from whatever the marketplace clone happens to be on.

That makes us a `same-repo + subpath + needs-ref-pinning` case →
**`git-subdir` with explicit `ref: "main"`**:

```json
"source": {
  "source": "git-subdir",
  "url": "https://github.com/drbothen/vsdd-factory.git",
  "path": "plugins/vsdd-factory",
  "ref": "main"
}
```

We considered the simpler relative-path string form
(`"source": "./plugins/vsdd-factory"`) but rejected it: the string form
has **no field for pinning the ref**. It implicitly reads from the
marketplace clone's checked-out branch — which is fragile under Git Flow,
because if a user re-registers the marketplace and ends up on develop
(the GitHub default), every install would silently start pulling
in-flight work from develop instead of release-stable code from main.

---

## 4. The bug in v1.0.0-rc.4 and v1.0.0-rc.5

`marketplace.json` shipped with this source field:

```json
"source": {
  "source": "github",
  "repo": "drbothen/vsdd-factory",
  "path": "plugins/vsdd-factory",  // ← INVALID on github source
  "ref": "main"
}
```

Symptoms:
- `claude plugin install vsdd-factory@vsdd-factory` reported success
- `claude plugin validate plugins/vsdd-factory` reported success
- `~/.claude/plugins/marketplaces/vsdd-factory/` was correctly cloned
- `~/.claude/plugins/installed_plugins.json` recorded `installPath:
  ~/.claude/plugins/cache/vsdd-factory/vsdd-factory/1.0.0-rc.5` with the
  correct `gitCommitSha`
- **But the recorded installPath directory did not exist.** Cache was empty.
- `/reload-plugins` reported `0 skills`. No `/vsdd-factory:*` slash commands
  appeared. Most agents and all hooks failed to load.

Root cause: the `github` source type schema doesn't include a `path` field.
Claude Code's install code path:
1. Sees `source: "github"`, clones `drbothen/vsdd-factory` to `marketplaces/vsdd-factory/`
2. Looks for the plugin at the marketplace clone root (since `github` source
   plugins are expected to be at root)
3. Doesn't find a `.claude-plugin/plugin.json` at root (it's actually at
   `plugins/vsdd-factory/.claude-plugin/plugin.json`)
4. Records the install metadata anyway and exits without populating cache
5. Plugin loader finds zero files at the recorded `installPath`

**Why `claude plugin validate` didn't catch it:** that command validates
`plugin.json` schema only. There is no first-class `marketplace.json`
schema validator in the CLI. Marketplace schema errors pass through.

---

## 5. The fix in v1.0.0-rc.6

Changed marketplace.json to:

```json
"source": {
  "source": "git-subdir",
  "url": "https://github.com/drbothen/vsdd-factory.git",
  "path": "plugins/vsdd-factory",
  "ref": "main"
}
```

This is the documented pattern for "plugin at a subpath of an external
repo, pinned to a specific ref." It fixes both problems shipped in
rc.4 / rc.5:

1. **Cache populates correctly.** `git-subdir` is the source type that
   officially supports the `path:` field. Sparse-clone of the subpath
   lands in `~/.claude/plugins/cache/<marketplace>/<plugin>/<version>/`
   as expected.
2. **Installs always read from `main`.** Explicit `ref: "main"` pins the
   plugin source independent of which branch the user's marketplace
   clone happens to track. Critical for our Git Flow setup where
   `develop` is the GitHub default but releases live on `main`.

The sibling plugins (dclaude, wclaude, zclaude) use a simpler
`"source": "./"` because their plugin lives at the repo root AND they
don't use Git Flow — their default branch IS their release branch, so
they don't need explicit ref pinning. Different layout, different
appropriate source type.

---

## 6. Repository layout: today and tomorrow

### 6.1. Today (single plugin, same-repo marketplace)

```
vsdd-factory/                                  ← repo root + marketplace root
├── .claude-plugin/
│   └── marketplace.json                       ← lists 1 plugin: source = "./plugins/vsdd-factory"
├── plugins/
│   └── vsdd-factory/                          ← the plugin
│       ├── .claude-plugin/
│       │   └── plugin.json                    ← plugin manifest (name, version)
│       ├── skills/<name>/SKILL.md
│       ├── agents/<name>.md
│       ├── hooks/hooks.json
│       └── ...
└── (everything else: Cargo workspace, docs, tests)
```

User installs via:
```
/plugin install vsdd-factory@vsdd-factory
```

### 6.2. Future: central marketplace for multiple plugins

Eventually we may want a central `drbothen/claude-marketplace` repo that
lists vsdd-factory and any future plugins. Two evolution paths:

**Option A: Marketplace repo references plugins in their own repos**

```
drbothen/claude-marketplace/
└── .claude-plugin/
    └── marketplace.json
        plugins:
        - { name: vsdd-factory,     source: { source: github,     repo: drbothen/vsdd-factory } }
        - { name: secops-factory,   source: { source: github,     repo: drbothen/secops-factory } }
        - { name: my-other-plugin,  source: { source: git-subdir, url: ..., path: tools/foo } }
```

Each plugin lives in its own repo (or a subpath of one). Marketplace just
points at them. Most flexible — each plugin has independent versioning,
release cadence, and ownership. **Use `git-subdir` (not `github + path`)
when a plugin lives at a subpath of an external repo.** Don't repeat our
rc.4/rc.5 mistake.

**Option B: Marketplace repo contains all plugins as subdirectories**

```
drbothen/claude-marketplace/
├── .claude-plugin/
│   └── marketplace.json
│       plugins:
│       - { name: vsdd-factory,   source: "./plugins/vsdd-factory" }
│       - { name: secops-factory, source: "./plugins/secops-factory" }
└── plugins/
    ├── vsdd-factory/
    │   └── .claude-plugin/plugin.json
    └── secops-factory/
        └── .claude-plugin/plugin.json
```

All plugins versioned and shipped together. Simpler if plugins evolve in
lockstep; harder if they have independent release cadences.

For vsdd-factory's likely trajectory (independent from secops-factory),
**Option A is the natural fit when we get there.**

### 6.3. Release channels (stable vs in-flight)

The official docs describe a two-marketplace pattern for projects that
want to expose both stable and pre-release builds. We could adopt this
once the user base is large enough to justify it:

```json
// stable channel (default install path)
{ "name": "vsdd-factory-stable",
  "plugins": [{
    "name": "vsdd-factory",
    "source": { "source": "git-subdir", "url": "...", "path": "...", "ref": "main" }
  }]
}

// edge / nightly channel
{ "name": "vsdd-factory-edge",
  "plugins": [{
    "name": "vsdd-factory",
    "source": { "source": "git-subdir", "url": "...", "path": "...", "ref": "develop" }
  }]
}
```

Users opt in to either channel via different `marketplace add` URLs.
For now, we ship one channel pinned to `main`. The `ref` field gives us
a clean upgrade path the day we want a separate edge channel.

### 6.4. Migration discipline

If/when we move vsdd-factory into a central marketplace repo:

1. **Don't break existing installs.** The old repo's marketplace.json can
   continue to exist and point at the new location, or vsdd-factory can be
   listed in BOTH the old and new marketplaces during a transition.
2. **Keep the version contract.** plugin.json `version` field is the
   source of truth. Releasing v1.1.0 from a new marketplace location must
   still be greater than the last v1.0.x in the old location.
3. **Pin the ref EXPLICITLY in every plugin entry.** Don't rely on
   marketplace clone defaults. Every plugin entry should specify `ref:`
   so the install is reproducible regardless of how the marketplace was
   added.
4. **Re-test on a clean machine.** Marketplace install bugs are silent;
   `claude plugin validate` won't catch them. Spin up a fresh
   `~/.claude/plugins/` (or a different user) and exercise the full
   `/plugin install` → `/reload-plugins` → `/<plugin>:<skill>` flow.

---

## 7. SKILL.md frontmatter reference

For completeness, here's the canonical SKILL.md frontmatter schema we
need to follow. Cribbed from
[official skills docs](https://code.claude.com/docs/en/skills.md):

| Field                        | Type   | Required? | Default                | What it controls |
|------------------------------|--------|-----------|------------------------|------------------|
| `name`                       | string | optional  | directory name         | Stable invocation name (shown in `/<plugin>:<name>`). |
| `description`                | string | recommended | first paragraph of body | What the skill does + when to use. Loaded into context. |
| `when_to_use`                | string | optional  | (none)                 | Trigger phrases / examples. Appended to description. |
| `argument-hint`              | string | optional  | (none)                 | UI hint shown in slash-picker autocomplete. |
| `arguments`                  | string/list | optional | (none)               | Named positional args for `$name` substitution. |
| `disable-model-invocation`   | bool   | optional  | `false`                | If true, model can't auto-invoke; user-only. Description NOT loaded into context. |
| `user-invocable`             | bool   | optional  | `true`                 | If false, hidden from `/` slash picker. Model can still auto-invoke (if not also disabled above). |
| `allowed-tools`              | string/list | optional | (none)               | Tools pre-approved while skill runs. Space-separated string OR YAML list — **NOT comma-separated**. |
| `model`                      | string | optional  | inherit                | Override model for this skill (`opus`, `sonnet`, etc.). |
| `effort`                     | string | optional  | inherit                | Override effort level. |
| `context`                    | string | optional  | (none)                 | `fork` runs the skill in a forked subagent context. |
| `agent`                      | string | optional  | `general-purpose`      | Subagent type when `context: fork`. |
| `paths`                      | string/list | optional | (none)               | Glob patterns; auto-activate skill only on matching files. |

### Common pitfalls (catalogued from rc.5 audit)

1. **Blank lines inside frontmatter break YAML parsing.** Don't write:
   ```yaml
   ---
   name: foo
   description: bar
                                  # ← blank line here breaks parsing
   allowed-tools: Read Bash
   ---
   ```
   Keep the YAML block compact.

2. **`allowed-tools` is space-separated or YAML list, NOT comma-separated.**
   Wrong: `allowed-tools: Read, Write, Edit, Bash` (comma-separated).
   Right: `allowed-tools: Read Write Edit Bash` (space-separated).
   Right: list form with `- Read` / `- Write` per line.

3. **`disable-model-invocation: true` removes the description from context.**
   Use it for state-changing entry points (activate, deactivate, release)
   where the user MUST be the trigger. Don't use it for skills you want
   the model to auto-invoke.

4. **`name:` field is optional but recommended for plugins.** When the
   plugin is installed via a custom path, the directory name and the
   invocation name can drift. An explicit `name:` keeps the slash command
   stable: `/<plugin>:<name>`.

---

## 8. Validation discipline

Before shipping a release that touches plugin or marketplace structure:

```bash
# Validates plugin.json schema (does NOT validate marketplace.json or
# whether install will succeed):
claude plugin validate plugins/vsdd-factory

# Lists installed plugins and their cache install paths. Verify
# the recorded installPath directory ACTUALLY EXISTS on disk:
claude plugin list

# Force a fresh install to a separate cache (clean-room test):
rm -rf ~/.claude/plugins/cache/vsdd-factory ~/.claude/plugins/marketplaces/vsdd-factory
claude plugin uninstall vsdd-factory@vsdd-factory --scope project
# (then reinstall and check whether ~/.claude/plugins/cache/vsdd-factory/vsdd-factory/<version>/
#  actually contains the plugin files)
```

**Rule of thumb:** if `claude plugin list` shows a non-zero version for
the plugin but `~/.claude/plugins/cache/<marketplace>/<plugin>/<version>/`
is empty or missing, the install silently failed (regardless of what
exit codes said). The marketplace.json `source` field is the most likely
culprit.

---

## 9. Reference

- Official source schema: https://code.claude.com/docs/en/plugin-marketplaces.md#plugin-sources
- Plugin caching behavior: https://code.claude.com/docs/en/plugins-reference.md#plugin-caching-and-file-resolution
- Installation scopes: https://code.claude.com/docs/en/plugins-reference.md#plugin-installation-scopes
- SKILL.md frontmatter: https://code.claude.com/docs/en/skills.md
- The CHANGELOG entry where we shipped the fix: see `## 1.0.0-rc.6`
