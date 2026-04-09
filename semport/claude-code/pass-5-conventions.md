# Pass 5 — Conventions

## plugin.json

All 12 have the same minimal shape:

```json
{"name":"kebab","version":"0.1.0","description":"50-200 chars","author":{"name":"...","email":"..."}}
```

**No plugin** uses `homepage`, `repository`, `license`, `keywords`, or custom `commands`/`agents`/`hooks`/`mcpServers` path overrides. Convention is **"minimal manifest, rely on auto-discovery."**

**Anomaly**: `plugin-dev/` has **no plugin.json at all** (confirmed by Glob of `plugins/**/plugin.json`). Either the runtime tolerates this for some internal case or plugin-dev is a reference bundle not loaded conventionally. **Undocumented.**

## Command frontmatter

```yaml
---
description: <verb-first, ~60 chars>
allowed-tools: <Tool list | Bash(subcmd:*), ...>
argument-hint: <optional>
model: <sonnet | inherit>   # optional
---
```

Kebab-case in YAML keys (`allowed-tools`, `argument-hint`) — counter to JS ecosystem norms.

## Agent frontmatter

```yaml
---
name: kebab-case
description: |
  prose + 2-3 <example>...</example> dialogs
tools: Glob, Grep, LS, Read, ...    # COMMA-SEPARATED BARE STRING, not a YAML list
model: sonnet | inherit
color: green | yellow | ...          # cosmetic only
---
```

## Skill frontmatter

```yaml
---
name: <Title Case OR kebab-case — INCONSISTENT>
description: <long prose enumerating triggering phrases>
version: 0.1.0
license: ...                         # rare
---
```

**Inconsistency**: `plugin-dev/skills/plugin-structure/SKILL.md:2` uses `name: Plugin Structure` (Title Case) but `frontend-design/skills/frontend-design/SKILL.md:2` uses `name: frontend-design` (kebab). **Convention bug in the reference corpus itself.**

Skill `description` is the **trigger** — the strongest empirical pattern is enumerating every user phrase that should activate it (`plugin-structure/SKILL.md:3`).

## hooks.json

Always `"type": "command"`; `timeout` sometimes set (hookify: 10), sometimes omitted (security-guidance).

## Naming

- **kebab-case** universal for: plugin name, command file, agent file, skill directory, script file. Violated only by skill `name:` frontmatter.
- **Hook handler scripts** depart: `pretooluse.py`, `posttooluse.py`, `userpromptsubmit.py`, `stop.py` — lowercased, no separator, matching event name.
- **Hook handler location**: `hooks/` (most) vs `hooks-handlers/` (both output-style plugins). **Inconsistency with no documented canonical.**

## Positive idioms

- Backtick-bang context injection in commands (`commit.md:5-9`).
- Commands invoke same-plugin skills via the `Skill` tool (`hookify.md:9`).
- Agent description embeds `<example>` blocks.
- `${CLAUDE_PLUGIN_ROOT}` everywhere (for command paths).
- Minimal manifests, rely on defaults.

## Anti-patterns / inconsistencies

- Skill `name:` casing drift (Title Case vs kebab).
- `plugin-dev/` has no `plugin.json`.
- `/tmp/security-warnings-log.txt` hardcoded.
- Two hook handler locations without canonical.
- `tools:` as comma-string invites parse bugs.
- No tests anywhere.
- `sys.path` surgery in hookify entry scripts (fragile).
