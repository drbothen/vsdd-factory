# Pass 5 — Conventions

_Phase B convergence round 1._

## plugin.json

All 12 plugins with a manifest use the same minimal shape:

```json
{"name":"kebab","version":"0.1.0","description":"50-200 chars","author":{"name":"...","email":"..."}}
```

**No plugin** uses `homepage`, `repository`, `license`, `keywords`, or custom `commands`/`agents`/`hooks`/`mcpServers` path overrides. Convention: **"minimal manifest, rely on auto-discovery."**

**Anomaly**: `plugin-dev/` has **no plugin.json at all**. Either the runtime tolerates this or plugin-dev is a reference bundle not loaded conventionally. **Undocumented.**

## Command frontmatter

```yaml
---
description: <verb-first, ~60 chars>
allowed-tools: <Tool list | Bash(subcmd:*), ...>   # String OR Array (BC-C02)
argument-hint: <optional>
model: <sonnet | inherit>                           # optional
disable-model-invocation: <bool>                    # optional, round 2 addition
---
```

Kebab-case in YAML keys (`allowed-tools`, `argument-hint`, `disable-model-invocation`) — counter to JS ecosystem norms but consistent across the corpus. Field `disable-model-invocation` (BC-DRAFT-C06) prevents programmatic `SlashCommand` tool invocation; use for dangerous or interactive commands.

## Agent frontmatter (expanded round 1)

```yaml
---
name: kebab-case                    # 3-50 chars, [a-z0-9-], start/end alnum, no underscores
description: |
  prose + 2-3 <example>...</example> dialogs   # 10-5000 chars
tools: Glob, Grep, LS, Read, ...    # TENSION-04: array per docs, comma-string in examples
model: inherit                      # enum: {inherit, sonnet, opus, haiku}
color: green                        # REQUIRED enum: {blue, cyan, green, yellow, magenta, red}
---
<system prompt body: 20-10000 chars>
```

Round-1 additions from pass 3 round 3:

- **CONV-A01**: `name` regex `^[a-z0-9]([a-z0-9-]*[a-z0-9])?$`, 3-50 chars (BC-DRAFT-A05; `agent-development/SKILL.md:69-80,264-273`).
- **CONV-A02**: `description` length bounds 10-5000 chars, system prompt 20-10000 chars (BC-DRAFT-A05).
- **CONV-A03**: `model` enumerated `{inherit, sonnet, opus, haiku}` (BC-DRAFT-A06; `agent-development/SKILL.md:128-141`).
- **CONV-A04**: `color` is **required** and enumerated `{blue, cyan, green, yellow, magenta, red}` (BC-DRAFT-A06; `:351-357`). Phase A flagged color as "cosmetic only" — round 1 corrects: cosmetic purpose, but schema-mandatory.

## Skill frontmatter

```yaml
---
name: <Title Case OR kebab-case — INCONSISTENT>
description: <third-person prose enumerating triggering phrases>
version: 0.1.0
license: ...                         # rare
---
```

- **CONV-S01**: Skill `description` should be written in **third person** — "This skill should be used when..." — not second-person imperative (`skill-development/SKILL.md:162-182`, BC-DRAFT-S03 round 3). This is a convention, not a schema rule, but the skill-development skill teaches it explicitly.
- **CONV-S02 (Inconsistency, unchanged)**: `plugin-dev/skills/plugin-structure/SKILL.md:2` uses `name: Plugin Structure` (Title Case) while `frontend-design/skills/frontend-design/SKILL.md:2` uses kebab. Round-1 resurvey of every SKILL.md in the corpus confirms this is still the only casing drift — most skills use kebab, a minority under `plugin-dev/skills/` use Title Case. **Convention bug in the reference corpus itself.**

Skill `description` is the **trigger** — the strongest empirical pattern is enumerating every user phrase that should activate it.

## hooks.json

Always `"type": "command"` or `"type": "prompt"` (round 2). `timeout` sometimes set (hookify: 10), sometimes omitted (security-guidance → inherits 60s default). Validator (`validate-hook-schema.sh`) is the de facto schema reference — see pass 4 NFR-V01..V10.

## Naming

- **kebab-case** universal for: plugin name, command file, agent file, skill directory, script file. Violated only by skill `name:` frontmatter.
- **Hook handler scripts** depart: `pretooluse.py`, `posttooluse.py`, `userpromptsubmit.py`, `stop.py` — lowercased, no separator, matching event name.
- **Python package modules** (`hookify/core/`, round-1 survey): snake_case (`base.py`, `config.py`, `dispatch.py` style). Standard Python — creates a **three-way naming regime** across the corpus: kebab-case (plugin metadata), eventname-no-separator (hook entry scripts), snake_case (Python internals).
- **Hook handler location**: `hooks/` (most) vs `hooks-handlers/` (both output-style plugins). Round-1 check of `hook-development/SKILL.md`: **no canonical guidance issued**. Inconsistency remains unresolved.

## Positive idioms (expanded round 1)

Original five:

- Backtick-bang context injection in commands (`commit.md:5-9`).
- Commands invoke same-plugin skills via the `Skill` tool (`hookify.md:9`).
- Agent description embeds `<example>` blocks.
- `${CLAUDE_PLUGIN_ROOT}` everywhere (for command paths).
- Minimal manifests, rely on defaults.

Round 1 additions (from pass 3 rounds 2-3):

- **`type:"prompt"` hooks** as a lightweight alternative to command hooks — no script file required, prompt text runs through the model. Constrained to four events (NFR-V08).
- **`permissionDecision` rewrite pattern** — PreToolUse hooks can return `updatedInput` to mutate the tool invocation (BC-DRAFT-H11). Stronger than block/allow.
- **`headersHelper` auth script** — MCP servers delegate header generation to an executable for JWT/HMAC/time-based tokens (BC-DRAFT-MCP06). Composable with static `headers`.
- **Plugin Settings as runtime config** — `.claude/<plugin-name>.local.md` with YAML frontmatter + markdown body is the **only** canonical runtime-adjustable config path (BC-DRAFT-PS01/PS02). Hookify is the lone adopter.
- **`.claude/*.local.md` gitignore convention** — pairs with Plugin Settings; keeps per-user overrides out of version control. Recommended `chmod 600`.
- **Third-person skill descriptions** — CONV-S01 above.

## Anti-patterns / inconsistencies (expanded round 1)

Original seven:

- Skill `name:` casing drift (Title Case vs kebab).
- `plugin-dev/` has no `plugin.json`.
- `/tmp/security-warnings-log.txt` hardcoded.
- Two hook handler locations without canonical.
- `tools:` as comma-string invites parse bugs.
- No tests anywhere.
- `sys.path` surgery in hookify entry scripts (fragile).

Round 1 additions:

- **TENSION-03 (pass 3 round 3)**: `matcher` is required by the validator (`validate-hook-schema.sh:70-75`, pass 4 NFR-V02) but treated as optional by docs and hookify. Either the validator or the docs is wrong.
- **TENSION-04 (pass 3 round 3)**: Agent `tools:` field format is JSON array per `agent-development/SKILL.md:149` but comma-string per `silent-failure-hunter.md:4` and similar. No plugin uses the array form in practice.
- **Bare `pickle` substring match** (`security_reminder_hook.py:118`) — over-broad; any file mentioning `pickleball` triggers a false positive.
- **CWE labels missing** from security reminders — violates SOUL.md #5 "Errors Are Domain Knowledge."
- **No canonical logging path** — a plugin needing to log is forced to invent (and `/tmp/security-warnings-log.txt` is the only existing example, itself an anti-pattern).
- **Fail-mode policy unpoliced** — validator enforces shape but not fail-open/fail-closed policy (pass 4 Observation).
- **No hot-reload** (BC-DRAFT-H15) — anti-pattern for developer experience, not an inconsistency but a notable convention.
