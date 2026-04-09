# Pass 5 — Conventions

_Phase B convergence round 2 (bounded final)._

Round 2 found 3 SUBSTANTIVE corrections to round 1 (CONV-S02 direction reversal, Plugin Settings adopter count, CONV-S03/S04 body tone + size budget), 1 accuracy retraction (command-development has no references/ dir), and 1 latent anti-pattern (Plugin Settings dual-purpose conflation). Pass 4 separately converged with zero substantive findings.

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

**(r2-fix)** Round 1 asserted `plugin-dev/skills/command-development/references/` contains 7 files. **No references directory exists for command-development** (`plugins/plugin-dev/skills/command-development/` contains only `SKILL.md`). Round 1 claim retracted.

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

- **CONV-A01**: `name` regex `^[a-z0-9]([a-z0-9-]*[a-z0-9])?$`, 3-50 chars (BC-DRAFT-A05).
- **CONV-A02**: `description` 10-5000 chars, system prompt 20-10000 chars.
- **CONV-A03**: `model` enumerated `{inherit, sonnet, opus, haiku}`.
- **CONV-A04**: `color` is **required** and enumerated `{blue, cyan, green, yellow, magenta, red}`.

## Skill frontmatter

```yaml
---
name: <Title Case is the majority; kebab-case is the outlier>
description: <third-person prose enumerating triggering phrases>
version: 0.1.0
license: ...                         # rare
---
```

- **CONV-S01**: Skill `description` should be written in **third person** — "This skill should be used when..." — not second-person imperative (`skill-development/SKILL.md:162-182`).

- **CONV-S02 (r2-fix — DIRECTION REVERSED)**: Exhaustive corpus audit: 10 SKILL.md files total. **8 use Title Case, 2 use kebab-case.** Round 1 said Title Case was "only under `plugin-dev/skills/`" — **WRONG**. `plugins/hookify/skills/writing-rules/SKILL.md:2` is `name: Writing Hookify Rules` (Title Case) and is not under plugin-dev.

  | File | `name:` value | Case |
  |---|---|---|
  | `plugin-dev/skills/skill-development/SKILL.md:2` | `Skill Development` | Title |
  | `plugin-dev/skills/command-development/SKILL.md:2` | `Command Development` | Title |
  | `plugin-dev/skills/plugin-structure/SKILL.md:2` | `Plugin Structure` | Title |
  | `plugin-dev/skills/plugin-settings/SKILL.md:2` | `Plugin Settings` | Title |
  | `plugin-dev/skills/mcp-integration/SKILL.md:2` | `MCP Integration` | Title |
  | `plugin-dev/skills/hook-development/SKILL.md:2` | `Hook Development` | Title |
  | `plugin-dev/skills/agent-development/SKILL.md:2` | `Agent Development` | Title |
  | `hookify/skills/writing-rules/SKILL.md:2` | `Writing Hookify Rules` | **Title (not plugin-dev)** |
  | `frontend-design/skills/frontend-design/SKILL.md:2` | `frontend-design` | kebab |
  | `claude-opus-4-5-migration/skills/claude-opus-4-5-migration/SKILL.md:2` | `claude-opus-4-5-migration` | kebab |

  Title Case is the **majority** convention (8/10, 80%). The skill-development skill teaches `name: Skill Name` (Title Case) as the canonical example at `skill-development/SKILL.md:166`. **Title Case is the norm; kebab is the outlier.** Direction reversed vs round 1.

- **CONV-S03 (r2-add)**: Skill **body tone** must be **imperative/infinitive form, NOT second person** (`skill-development/SKILL.md:160`). "To accomplish X, do Y" is canonical; "You should do X" / "If you need to do X" are anti-patterns. Distinct from CONV-S01 (frontmatter description tone); CONV-S01 = third-person frontmatter, CONV-S03 = imperative body.

- **CONV-S04 (r2-add)**: Skill **size budget** — target **1,500-2,000 words for SKILL.md body** (`skill-development/SKILL.md:190`). Detail must be offloaded to `references/patterns.md`, `references/advanced.md`, `references/migration.md`, `references/api-reference.md` (progressive disclosure). Explicit validation step at `:220` "Test progressive disclosure".

Skill `description` is the **trigger** — the strongest empirical pattern is enumerating every user phrase that should activate it.

## hooks.json

Always `"type": "command"` or `"type": "prompt"`. `timeout` sometimes set (hookify: 10), sometimes omitted (security-guidance → inherits 60s default). Validator (`validate-hook-schema.sh`) is the de facto schema reference — see pass 4 NFR-V01..V10.

## Naming

- **kebab-case** universal for: plugin name, command file, agent file, skill directory, script file.
- **Skill `name:` frontmatter** — NOT kebab. Title Case is the majority (CONV-S02).
- **Hook handler scripts** depart: `pretooluse.py`, `posttooluse.py`, `userpromptsubmit.py`, `stop.py` — lowercased, no separator, matching event name.
- **Python package modules** (`hookify/core/`): snake_case. Creates a **four-way naming regime**: kebab-case (plugin metadata + directories), Title Case (skill frontmatter names), eventname-no-separator (hook entry scripts), snake_case (Python internals).
- **Hook handler location**: `hooks/` (most) vs `hooks-handlers/` (both output-style plugins). **No canonical guidance.** Inconsistency remains unresolved.

## Positive idioms (expanded round 1+2)

Original five:

- Backtick-bang context injection in commands (`commit.md:5-9`).
- Commands invoke same-plugin skills via the `Skill` tool (`hookify.md:9`).
- Agent description embeds `<example>` blocks.
- `${CLAUDE_PLUGIN_ROOT}` everywhere.
- Minimal manifests, rely on defaults.

Round 1+2 additions:

- **`type:"prompt"` hooks** — lightweight alternative to command hooks. Constrained to four events (NFR-V08).
- **`permissionDecision` rewrite pattern** — PreToolUse hooks can return `updatedInput` to mutate the tool invocation (BC-DRAFT-H11).
- **`headersHelper` auth script** — MCP servers delegate header generation for JWT/HMAC/time-based tokens (BC-DRAFT-MCP06).
- **Plugin Settings as runtime config** — `.claude/<plugin-name>.local.md`, the **only** canonical runtime-adjustable config path.
  - **(r2-fix)** Round 1 claimed "hookify is the lone adopter" — **WRONG**. **Two plugins adopt the pattern**:
    1. **hookify** — `.claude/hookify.local.md` for config.
    2. **ralph-wiggum** — `.claude/ralph-loop.local.md` for cross-hook state persistence (referenced at `ralph-wiggum/commands/help.md:49,69`, `ralph-wiggum/hooks/stop-hook.sh:13`, `ralph-wiggum/commands/cancel-ralph.md:3,11,16,17`, `ralph-wiggum/scripts/setup-ralph-loop.sh:54,57,140,164`). Ralph uses the file as **stateful iteration across Stop hook invocations**, not config.
  - `plugin-settings/examples/example-settings.md` teaches four template shapes: basic config, advanced config, agent state file, and feature flag pattern — establishing Plugin Settings as a **general-purpose key-value store**, not just runtime config.
- **`.claude/*.local.md` + `.claude/*.local.json` gitignore convention** — both extensions should be gitignored (`plugin-settings/examples/example-settings.md:140-143`). Round 1 had only `.local.md`. Recommended `chmod 600`.
- **Third-person skill descriptions** (CONV-S01).
- **Imperative skill bodies** (CONV-S03).
- **1,500-2,000 word SKILL.md budget with references/ offload** (CONV-S04).

## Anti-patterns / inconsistencies (expanded round 1+2)

- Skill `name:` casing drift — **(r2-fix reversed direction)**: kebab is the outlier (2/10), Title Case is the majority (8/10). See CONV-S02.
- `plugin-dev/` has no `plugin.json`.
- `/tmp/security-warnings-log.txt` hardcoded.
- Two hook handler locations without canonical.
- `tools:` as comma-string invites parse bugs (TENSION-04).
- **No tests anywhere** — (r2-confirm) exhaustive glob confirms zero matches.
- `sys.path` surgery in hookify entry scripts.
- **TENSION-03** (matcher required by validator, optional per docs) — unresolved after round 2.
- **TENSION-04** (agent `tools:` array vs comma-string) — unresolved after round 2.
- **Bare `pickle` substring match** — over-broad.
- **CWE labels missing** from security reminders.
- **No canonical logging path**.
- **Fail-mode policy unpoliced** — validator enforces shape but not fail-open/fail-closed.
- **No hot-reload** — (r2-confirm) `plugin-settings/examples/example-settings.md:159` explicitly states "Changes require Claude Code restart - hooks can't be hot-swapped."
- **(r2-add) Plugin Settings dual-purpose conflation** — the Plugin Settings pattern is used for two distinct purposes: (a) user-editable config (hookify) and (b) machine-written state files (ralph-wiggum loop counter, multi-agent-swarm task tracking). No convention distinguishes them, and both share the same `.claude/*.local.md` gitignore envelope. A state file silently overwritten by the user editing it will corrupt the owning plugin. **Latent anti-pattern.**

## Round 2 convergence note

Round 2 produced 3 SUBSTANTIVE findings. The Plugin Settings correction itself spawned a latent anti-pattern that round 3 would be justified to investigate — but round 2 is the bounded final round per protocol. Remaining residuals: TENSION-03, TENSION-04, hook handler location, Plugin Settings dual-purpose convention. All documented; none blocks downstream synthesis.
