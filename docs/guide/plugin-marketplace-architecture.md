# Plugin & Marketplace Architecture

This document explains how the vsdd-factory plugin is packaged for Claude Code,
why it's structured the way it is, and the install-time bug we shipped through
v1.0.0-rc.2 to rc.6 trying to land a "right-looking" but empirically broken
configuration.

> **TL;DR.** A Claude Code plugin needs a `plugin.json` manifest and a
> `marketplace.json` somewhere that points at it. The two manifests can live
> in the same repo or different repos. Empirically, **same-repo nested
> layouts (marketplace at root + plugin at `plugins/<name>/`) silently fail
> the cache-populate step on `claude plugin install`**, regardless of whether
> the source is `github+path` (schema-invalid), `git-subdir+path+ref`
> (schema-valid but still broken in self-reference), or anything else with
> `path:`. The fix that actually works is to **split the marketplace into a
> separate repo** that points at the plugin via `git-subdir+ref`. We did
> that in v1.0.0-rc.7 — vsdd-factory's marketplace now lives at
> [drbothen/claude-mp](https://github.com/drbothen/claude-mp).

---

## 1. Overview: how Claude Code loads a plugin

A Claude Code plugin is a directory containing **at least** a
`.claude-plugin/plugin.json` manifest and one or more of: `skills/`,
`agents/`, `commands/`, `hooks/`, MCP server configs.

Plugins are distributed through a **marketplace** — a separate manifest
(`marketplace.json`) that lists one or more plugins and tells Claude Code
where to fetch each plugin's files from. The marketplace can live in the
same repo as the plugin (the "monorepo with plugins/" pattern shown in
the docs walkthrough) or in a separate repo (what every successful sibling
plugin does).

Install flow when a user runs `/plugin install <plugin>@<marketplace>`:

```
1. Marketplace repo cloned to:       ~/.claude/plugins/marketplaces/<marketplace>/
2. Plugin source copied to cache:    ~/.claude/plugins/cache/<marketplace>/<plugin>/<version>/
3. Registry entry written to:        ~/.claude/plugins/installed_plugins.json
4. Plugin loader scans cache dir for SKILL.md, agent .md, hooks.json, etc.
```

**Step 2 is where vsdd-factory used to silently fail.** The recorded
`installPath` in `installed_plugins.json` pointed at a directory that was
never created. The plugin loader then found zero skills despite
`claude plugin validate` reporting success. We traced this to a
self-referential layout problem: when the marketplace repo IS the plugin
repo and the plugin lives at a subpath, Claude Code's install pipeline
clones the marketplace, records the install metadata, then fails to copy
the subpath into the cache version dir. Sometimes a `temp_github_*` orphan
clone gets left behind as evidence of partial work.

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
vsdd-factory used in beta.3 - rc.1, and what the docs walkthrough shows.
**Empirically WORKS** for our nested layout, but has no `ref` field for
explicit branch pinning — the install reads from whatever the marketplace
clone is checked out to (typically the GitHub default branch).

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
at repo root). No `path` field per the schema. The whole repo gets cloned
and treated as the plugin root.

> **CRITICAL:** Adding a `path:` field to a `github` source is INVALID
> per the schema. `github` has no `path` field. Claude Code's manifest
> validator does NOT catch this. **This is the bug we shipped in
> rc.2 - rc.5.** If the plugin is at a subpath, use `git-subdir` (§2.4)
> — but see §4 below for why that ALSO didn't fix things in our case.

### 2.3. `url` source

```json
"source": {
  "source": "url",
  "url": "https://example.com/path/to/plugin.git",
  "ref": "main"
}
```

Generic git URL form. Same semantics as `github`: the whole cloned tree
is the plugin root. No `path` field per the schema.

### 2.4. `git-subdir` source

```json
"source": {
  "source": "git-subdir",
  "url": "https://github.com/owner/monorepo.git",
  "path": "tools/claude-plugin",
  "ref": "main",
  "sha": "abc123..."
}
```

Documented use: **plugin lives at a subpath of an external git repo**.
Sparse-clones only the subpath. Used successfully in production by
Semgrep, Railway, and ~8 other plugins in Anthropic's official marketplace.

> **CAUTION (vsdd-factory rc.6 lesson):** Empirically, `git-subdir` is
> only verified to work when the URL repo is **different** from the
> repo the marketplace.json itself lives in. We tried `git-subdir` with
> `url:` pointing at the same repo as the marketplace and it failed
> the cache-populate step in exactly the same way `github+path` did.
> Whether this is a Claude Code bug specific to self-referential
> `git-subdir`, or a documented-but-misunderstood limitation, isn't
> clear from the docs. The defensive answer: don't put marketplace.json
> and the `git-subdir` target in the same repo.

### 2.5. `npm` source

```json
"source": {
  "source": "npm",
  "package": "@org/plugin-name",
  "version": "^1.2.0"
}
```

For plugins distributed as npm packages. Not used by vsdd-factory.

---

## 3. Decision tree: which source type for our use case?

```
Is the marketplace.json in the SAME repo as the plugin source files?
├── YES (monorepo):
│   └── Use a relative-path string:   "source": "./path/to/plugin"
│       ▶ No ref pinning available; relies on marketplace clone's branch
│       ▶ AVOID object forms with path: even git-subdir fails here
│
└── NO (separate marketplace repo): ◀ vsdd-factory today
    └── Plugin at repo root?
        ├── YES → Use github/url object:   {github, repo, ref?, sha?}
        └── NO  → Use git-subdir object:   {git-subdir, url, path, ref?, sha?}
            ▶ Supports explicit ref pinning (good for Git Flow)
            ▶ Sparse-checkout reduces install size
            ▶ This is what claude-mp uses to point at vsdd-factory
```

**vsdd-factory today (post-rc.7):**

- Marketplace lives at `drbothen/claude-mp` — its own tiny repo
- Plugin lives at `drbothen/vsdd-factory` at the subpath `plugins/vsdd-factory/`
- claude-mp's `marketplace.json` has:
  ```json
  "source": {
    "source": "git-subdir",
    "url": "https://github.com/drbothen/vsdd-factory.git",
    "path": "plugins/vsdd-factory",
    "ref": "main"
  }
  ```
- This is a "separate marketplace repo + plugin at subpath" layout, which
  is the well-tested `git-subdir` happy path (same shape Semgrep ships).

---

## 4. The full saga: what broke and why

The user-visible symptom across rc.2 - rc.6 was always the same:
`/plugin install vsdd-factory@vsdd-factory` reported success, but
`/reload-plugins` reported `0 skills` and `/vsdd-factory:*` slash commands
didn't appear. Nothing in the install logs flagged a problem.

### Timeline

| Version | marketplace.json source | Result |
|---|---|---|
| beta.3 - rc.1 | `"./plugins/vsdd-factory"` | ✅ Worked |
| rc.2 - rc.5 | `{github, repo, path, ref}` | ❌ Cache empty, 0 skills |
| rc.6 | `{git-subdir, url, path, ref}` | ❌ Cache STILL empty, 0 skills |
| rc.7 | (marketplace.json removed; lives at claude-mp) | ✅ Works |

### Root cause progression

**rc.2's regression** (the original break): commit `e2303b9` changed source
from the working string form to `github+path`, intending to pin to main
branch under Git Flow. The `github` source type doesn't support `path:` per
the schema. Claude Code's installer cloned the marketplace repo, recorded
the install metadata with the correct `gitCommitSha`, but skipped the
subpath-extract step silently. `claude plugin validate` passed (it only
validates plugin.json, not marketplace.json source semantics).

**rc.6's failed fix**: switched source to `git-subdir+path+ref` per docs
recommendation. Schema-valid this time. Empirically still broken: cache
directory still never created. Hypothesis confirmed via the orphan
`temp_github_*` clone left in `~/.claude/plugins/cache/` — the install
DID clone the source repo to a temp directory, but failed to extract the
`plugins/vsdd-factory/` subpath into the final versioned cache path.

The trigger appears to be **self-referential same-repo install with a
subpath**: when the marketplace clone and the source repo are byte-for-byte
the same repo at the same ref, AND the source uses `path:` to indicate a
subpath of that repo, the cache-populate step fails. Whether this is a
known limitation, a Claude Code install-pipeline bug, or an artifact of
some cycle-detection logic isn't documented anywhere.

**rc.7's actual fix**: split the marketplace into its own repo
(`drbothen/claude-mp`). The `git-subdir` source now points at a
**different repo** (`drbothen/vsdd-factory`) than the marketplace lives
in. This is the well-tested cross-repo install path used by every
successful plugin we surveyed.

### Why `claude plugin validate` failed to catch any of this

The `validate` command validates `plugin.json` schema only. It does NOT
validate marketplace.json source semantics, does NOT attempt a dry-run
install, and does NOT verify cache-populate succeeds. Marketplace schema
errors and install-pipeline failures pass through silently.

> **Validation discipline going forward:** after every release, do an
> empirical clean-room install on a fresh state and verify the cache
> directory is populated AND `/reload-plugins` reports a non-zero skill
> count. See §8 for the checklist.

---

## 5. Repository layout: today and tomorrow

### 5.1. Today (post-rc.7)

```
drbothen/claude-mp/                            ← marketplace repo (tiny)
├── .claude-plugin/
│   └── marketplace.json                       ← lists 1 plugin: source = git-subdir
├── README.md
└── LICENSE

drbothen/vsdd-factory/                         ← plugin repo (development monorepo)
├── plugins/
│   └── vsdd-factory/                          ← the plugin
│       ├── .claude-plugin/
│       │   └── plugin.json                    ← plugin manifest (name, version)
│       ├── skills/<name>/SKILL.md
│       ├── agents/<name>.md
│       ├── hooks/hooks.json
│       └── ...
├── crates/                                    ← Cargo workspace (dispatcher, hooks)
├── docs/                                      ← user + dev docs
├── tests/                                     ← integration tests
├── .github/workflows/                         ← release.yml, ci.yml
└── (NO marketplace.json at this repo's root — moved to claude-mp)
```

User installs via:
```
/plugin marketplace add drbothen/claude-mp
/plugin install vsdd-factory@claude-mp
```

### 5.2. Future: adding more plugins to claude-mp

The marketplace is built to accept additional plugins. Each new plugin
appends to `claude-mp/.claude-plugin/marketplace.json`'s `plugins[]`
array. Each plugin lives in its own repo, gets its own `git-subdir`
entry in the marketplace, gets its own version field that's bumped when
that plugin ships a release.

```json
// future claude-mp marketplace.json
{
  "name": "claude-mp",
  "owner": { "name": "drbothen" },
  "plugins": [
    {
      "name": "vsdd-factory",
      "version": "1.0.0",
      "source": { "source": "git-subdir", "url": "...vsdd-factory.git", "path": "plugins/vsdd-factory", "ref": "main" }
    },
    {
      "name": "secops-factory",
      "version": "0.5.3",
      "source": { "source": "git-subdir", "url": "...secops-factory.git", "path": "plugins/secops-factory", "ref": "main" }
    },
    {
      "name": "another-plugin",
      "version": "1.0.0",
      "source": { "source": "github", "repo": "drbothen/another-plugin", "ref": "main" }
    }
  ]
}
```

Plugins evolve independently in their own repos. The marketplace catalog
is small (one tiny repo) and updates cheaply when a plugin ships a release.

### 5.3. Release-channel pattern (future option)

The official docs describe a two-marketplace pattern for projects that
want stable + edge channels. We could adopt this once the user base is
large enough:

```json
// stable channel (default)
{ "name": "claude-mp",
  "plugins": [{
    "name": "vsdd-factory",
    "source": { "source": "git-subdir", "url": "...", "path": "...", "ref": "main" }
  }]
}

// edge / nightly channel — separate repo (e.g., drbothen/claude-mp-edge)
{ "name": "claude-mp-edge",
  "plugins": [{
    "name": "vsdd-factory",
    "source": { "source": "git-subdir", "url": "...", "path": "...", "ref": "develop" }
  }]
}
```

Users opt in to either channel via different `marketplace add` URLs.
Today claude-mp ships only stable. The `ref` field gives us a clean
upgrade path the day we want a separate edge channel.

### 5.4. Migration discipline for future plugins

If/when adding a new plugin to claude-mp:

1. **Pin `ref:` explicitly** — never rely on the source repo's default branch.
2. **Validate empirically** — run `/plugin install` on a clean state and
   verify the cache directory is populated AND skills surface.
3. **Bump the marketplace version field** when the underlying plugin ships
   a release, so users see the new version available.
4. **Re-test on a clean machine** after structural changes — marketplace
   install bugs are silent, `claude plugin validate` won't catch them.

---

## 6. Slash command UX

Plugin skills are namespaced as `/<plugin-name>:<skill-name>` per the
official docs. **However**, the slash picker auto-disambiguates: if you
type `/foo` and only one installed plugin has a skill named `foo`, the
picker shows it with a `(plugin-name)` badge and accepting it invokes the
namespaced version. No explicit prefix needed in the common case.

This bare-form behavior is **undocumented**. It works in Claude Code
2.1.x and is consistent across recent versions, but no release note or
docs page describes it formally. Treat it as a UX convenience, not a
stable API contract:

- **Documented**: `/<plugin-name>:<skill-name>` form always works
- **Undocumented but observed**: bare `/<skill-name>` form works when
  there's no name collision across installed plugins
- **Unknown**: collision behavior — what the picker does when two plugins
  define the same skill name. Avoid this case by giving each plugin's
  skills distinctive names.

For plugin-author hygiene: don't depend on the bare form in your docs
or test instructions. Tell users `/vsdd-factory:create-adr`, not
`/create-adr` — the explicit form is guaranteed to work.

---

## 7. SKILL.md frontmatter reference

For completeness, here's the canonical SKILL.md frontmatter schema we
need to follow. From the
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

### Common pitfalls (catalogued from rc.5/rc.7 audits)

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
# Validates plugin.json schema only (does NOT validate marketplace.json
# source semantics or whether install will succeed):
claude plugin validate plugins/vsdd-factory

# Lists installed plugins and their cache install paths:
claude plugin list

# Empirical clean-room install (the gate that ACTUALLY catches bugs
# the validator misses):
claude plugin marketplace remove claude-mp           # if installed
rm -rf ~/.claude/plugins/marketplaces/claude-mp
rm -rf ~/.claude/plugins/cache/claude-mp
rm -rf ~/.claude/plugins/cache/temp_github_*         # any orphans
claude plugin marketplace add drbothen/claude-mp
claude plugin install vsdd-factory@claude-mp
# In a Claude Code session:
#   /reload-plugins
# Then verify:
ls ~/.claude/plugins/cache/claude-mp/vsdd-factory/<version>/   # must contain files
find ~/.claude/plugins/cache/claude-mp -name SKILL.md | wc -l  # must be > 0
```

**Rules of thumb:**
- If `claude plugin list` shows a non-zero version but
  `~/.claude/plugins/cache/<marketplace>/<plugin>/<version>/` is empty
  or missing, the install silently failed. Marketplace.json source field
  shape is the most likely culprit.
- If `temp_github_*` directories appear in
  `~/.claude/plugins/cache/`, the install pipeline started cloning but
  didn't complete. Treat as a failed install even if the registry says
  success.
- `/reload-plugins` reports a "skills" count that appears to be
  delta-since-last-load, not total active. To verify total count,
  check the cache directory directly or look at the available-skills
  list in a fresh session.

---

## 9. Reference

- Official source schema: https://code.claude.com/docs/en/plugin-marketplaces.md#plugin-sources
- Plugin caching behavior: https://code.claude.com/docs/en/plugins-reference.md#plugin-caching-and-file-resolution
- Installation scopes: https://code.claude.com/docs/en/plugins-reference.md#plugin-installation-scopes
- SKILL.md frontmatter: https://code.claude.com/docs/en/skills.md
- The CHANGELOG entry where we shipped the actual fix: see `## 1.0.0-rc.7`
