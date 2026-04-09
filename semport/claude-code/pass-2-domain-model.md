# Pass 2 — Domain Model

_Phase B convergence round 3 — **CONVERGED**._

## Changes from Phase A

- Added `"prompt"` hook type alongside `"command"` (`hook-development/SKILL.md:22-34`)
- Added MCP transport taxonomy: stdio, SSE, HTTP, ws (`mcp-integration/SKILL.md:65-165`)
- Added dual-format note for hooks config (plugin wrapper vs settings direct) (`hook-development/SKILL.md:60-119`)
- Added MCP tool naming convention (`mcp-integration/SKILL.md:190-200`)
- Documented default paths for `commands`/`agents`/`hooks`/`mcpServers` (`manifest-reference.md:214,248,263,302`)
- Clarified `hooks` and `mcpServers` accept inline object OR file path (`manifest-reference.md:273-325`)
- Noted tension: `.mcp.json` root structure inconsistent between docs

### Changes from round 1 (round 2)

- Added **Plugin Settings** as first-class subsystem (`plugin-settings/SKILL.md:1-60`).
- Clarified command **scope triad**: project, personal, plugin (`command-development/SKILL.md:55-72`).
- Added command frontmatter field **`disable-model-invocation`** (`command-development/SKILL.md:182-193`).
- Corrected `allowed-tools` type to **String OR Array** (`command-development/SKILL.md:132-143`).
- Added component-lifecycle Discovery/Activation phases; discovery is init-only, definitively answering hot-reload question (`component-patterns.md:7-26`).
- Recorded: **nested command discovery is not automatic** (`component-patterns.md:111-121`).
- Expanded hook stdin envelope with `transcript_path`, `cwd`, `permission_mode`, `hook_event_name`, event-specifics (`hook-development/SKILL.md:302-319`).
- Expanded env-var catalog: `CLAUDE_PROJECT_DIR`, `CLAUDE_PLUGIN_ROOT`, `CLAUDE_ENV_FILE`, `CLAUDE_CODE_REMOTE` (`hook-development/SKILL.md:322-329`).
- Resolved TENSION-02 for stdio: stdio MCP servers are **eager** (process runs for entire Claude Code session) (`server-types.md:38-44`).

### Changes from round 2 (round 3)

- Added **agent frontmatter validation rules** and enumerated value sets (`agent-development/SKILL.md:60-160,264-285`). `name` regex-validated 3-50 chars lowercase+digits+hyphens, must start/end alphanumeric. `model` enum: `inherit|sonnet|opus|haiku`. `color` enum **required**: `blue|cyan|green|yellow|magenta|red`. `tools` typed as **array** (e.g. `["Read","Write","Grep"]`), not comma-string — tension with in-repo examples which use comma-string (see TENSION-04).
- Added **`headersHelper`** MCP field for dynamic authentication (`mcp-integration/references/authentication.md:233-258`). A shell script path that emits JSON headers on stdout; enables short-lived tokens, JWT, HMAC signing, time-based auth. Not supported for mTLS directly (must wrap in stdio server).
- **TENSION-03 discovered**: `validate-hook-schema.sh:70-75` treats `matcher` as **required** (errors on missing), but `hook-development/SKILL.md` and `hookify/hooks/hooks.json` treat it as optional. Runtime behavior unconfirmed.
- Declared convergence after round 3. Remaining gaps (name collisions across plugins, regex flavor name, `.mcp.json` root structure) have no new evidence in source.

## Core concepts

| Concept | Definition | Key fields |
|---|---|---|
| **Plugin** | Directory with `.claude-plugin/plugin.json` | `name` (kebab, required), `version`, `description`, `author`, `homepage`, `repository`, `license`, `keywords`, `commands`, `agents`, `hooks`, `mcpServers` |
| **Command** | `commands/*.md` with YAML frontmatter, invoked via `/slash-command` | `description`, `allowed-tools` (String OR Array), `model`, `argument-hint`, `disable-model-invocation` |
| **Command scope** | Discovery location determines label in `/help` | project (`.claude/commands/`), personal (`~/.claude/commands/`), plugin (`<plugin>/commands/`) |
| **Agent** | `agents/*.md` subagent, own prompt/model/tools, invoked via `Task` tool | `name` (required, 3-50 chars, `^[a-z0-9][a-z0-9-]*[a-z0-9]$`), `description` (required, 10-5000 chars, includes `<example>` dialogs), `model` (required, enum `inherit\|sonnet\|opus\|haiku`), `color` (required, enum `blue\|cyan\|green\|yellow\|magenta\|red`), `tools` (optional, array or comma-string — see TENSION-04), system prompt 20-10000 chars |
| **Skill** | `skills/<name>/SKILL.md`, auto-activated by description match | `name`, `description` (the trigger), `version`, optional `license` |
| **Hook** | Entry in `hooks.json` (or inline `plugin.json.hooks`) mapping event → matcher → handler | event name, optional `matcher` (see TENSION-03), `hooks[].type` in `{"command","prompt"}`, `hooks[].command` or `hooks[].prompt`, `hooks[].timeout` |
| **Matcher** | Case-sensitive regex filter on tool name; omitted = match all (docs) / required (validator — TENSION-03) | e.g. `"Write\|Edit"`, `"*"`, `"mcp__.*__delete.*"` |
| **MCP server** | External tool provider launched via `.mcp.json` or inline in `plugin.json.mcpServers` | Varies by transport: stdio needs `command`/`args`/`env`; sse/http/ws need `type` + `url` (+ optional `headers`, optional `headersHelper`) |
| **MCP `headersHelper`** | Script path producing dynamic auth headers as JSON on stdout (round 3) | Used for JWT, HMAC, short-lived tokens; mutually composable with static `headers` |
| **MCP tool name** | Auto-generated runtime identifier for server-provided tools | `mcp__plugin_<plugin-name>_<server-name>__<tool-name>`, wildcard-referenceable in `allowed-tools` |
| **Plugin Settings** | Per-project plugin config at `.claude/<plugin-name>.local.md` — YAML frontmatter + markdown body; user-managed, not in git (`plugin-settings/SKILL.md:1-60`) | `enabled`, arbitrary keys |
| **Output style** | NOT a first-class construct. Emulated via SessionStart hook + `additionalContext` | — |
| **Progressive disclosure** | Convention: `SKILL.md` lean, heavy docs in `references/`, `examples/` | — |
| **`${CLAUDE_PLUGIN_ROOT}`** | Env var pointing at plugin dir; only portable intra-plugin path reference | Mandatory for hook commands, MCP server paths |

## Agent frontmatter validation (round 3)

From `agent-development/SKILL.md:60-160,264-285`:

| Field | Required | Constraint |
|---|---|---|
| `name` | yes | 3-50 chars, lowercase alnum + hyphens, must start and end alphanumeric, no underscores |
| `description` | yes | 10-5000 chars; must include `<example>` blocks with `<commentary>` |
| `model` | yes | enum: `inherit`, `sonnet`, `opus`, `haiku` |
| `color` | yes | enum: `blue`, `cyan`, `green`, `yellow`, `magenta`, `red` |
| `tools` | no | array of tool names; default = all tools |

System prompt body: 20-10000 characters, second-person, structured (responsibilities / process / output format / edge cases).

**TENSION-04:** `agent-development/SKILL.md:149` prescribes `tools` as JSON array. Reference plugin `silent-failure-hunter.md:4` uses comma-string. Both forms may be accepted by the runtime; authoritative format unconfirmed.

## Default component paths

From `manifest-reference.md:214,248,263,302`:

| Field | Default | Override accepts |
|---|---|---|
| `commands` | `./commands` | String or string[] (dir or file) |
| `agents` | `./agents` | String or string[] |
| `hooks` | `./hooks/hooks.json` | File path OR inline object |
| `mcpServers` | `./.mcp.json` | File path OR inline object |

Custom paths **supplement** defaults. Paths must be `./`-prefixed, forward-slash, no `../`. **Nested command discovery is not automatic** (`component-patterns.md:111-121`).

## Component lifecycle

From `component-patterns.md:7-26`:

1. **Discovery** — at Claude Code init only. Scan enabled plugins → read `plugin.json` → discover components → parse frontmatter → register → initialize (start MCP servers, register hooks).
2. **Activation** — continuous during session. Commands on slash invocation; Agents on Task dispatch; Skills on description match; Hooks on event fire; MCP tools on tool call.

**Implication:** No hot-reload. Editing plugin files requires restarting Claude Code.

## Hook types

From `hook-development/SKILL.md:22-59`:

1. **`type: "command"`** — bash/exec command, any event, schema `{type, command, timeout?}`. Default timeout **60s**.
2. **`type: "prompt"`** — LLM-driven evaluation, schema `{type, prompt, timeout?}`, supported ONLY on `Stop`, `SubagentStop`, `UserPromptSubmit`, `PreToolUse`. Default timeout **30s**.

## Hook configuration dual format

From `hook-development/SKILL.md:60-119`:

- **Plugin format** (`hooks/hooks.json`): wrapper `{description?, hooks: {<Event>: [...]}}`
- **User settings format** (`~/.claude/settings.json`): direct `{<Event>: [...]}` — no wrapper, no description

Both reference the same event-entry schema inside. Plugin hooks **merge with user hooks** and run in parallel on the same event.

## Hook I/O envelope

**Stdin (all hooks)** (`hook-development/SKILL.md:302-319`):

```
{ session_id, transcript_path, cwd, permission_mode, hook_event_name, ...event-specifics }
```

Event-specific: PreToolUse/PostToolUse add `tool_name`, `tool_input`, `tool_result`; UserPromptSubmit adds `user_prompt`; Stop/SubagentStop add `reason`.

**Stdout — standard envelope**: `{continue?, suppressOutput?, systemMessage?}`.

**Stdout — PreToolUse-specific** (`SKILL.md:144-153`):

```
{ hookSpecificOutput: { permissionDecision: "allow"|"deny"|"ask", updatedInput: {...} }, systemMessage }
```

PreToolUse hooks can **rewrite tool input** via `updatedInput`, not just block.

**Stdout — Stop/SubagentStop-specific** (`SKILL.md:202-209`):

```
{ decision: "approve"|"block", reason, systemMessage }
```

**Exit codes**: 0 = success; 2 = blocking (stderr → Claude); other = non-blocking error.

## Environment variables

| Var | Scope | Purpose |
|---|---|---|
| `CLAUDE_PROJECT_DIR` | All hooks | Project root path |
| `CLAUDE_PLUGIN_ROOT` | All hooks | Plugin directory (mandatory for portable paths) |
| `CLAUDE_ENV_FILE` | SessionStart only | Append `export FOO=bar` lines to persist env vars into session |
| `CLAUDE_CODE_REMOTE` | All hooks | Set iff running in remote context |

## Matcher semantics

From `hook-development/SKILL.md:386-425`:

- **Case-sensitive**
- Exact / pipe-alternation / wildcard `"*"` / full regex with `.*`

Flavor unspecified. **TENSION-03**: `validate-hook-schema.sh:70-75` errors when `matcher` is missing; docs and examples treat it as optional. Runtime behavior unconfirmed.

## Parallel execution

All matching hooks for an event run in parallel (`hook-development/SKILL.md:496-518`). Hooks cannot see each other's output; ordering non-deterministic; state sharing across parallel hooks unsafe.

## MCP server transports

| Transport | Key | Required fields | Auth | Startup |
|---|---|---|---|---|
| stdio | (no `type`) | `command`, optional `args`, `env` | env vars | **Eager** |
| sse | `"type": "sse"` | `url` | OAuth (automatic) | Potentially lazy |
| http | `"type": "http"` | `url`, optional `headers`, optional `headersHelper` | bearer token / dynamic | Potentially lazy |
| ws | `"type": "ws"` | `url`, optional `headers` | bearer token | Potentially lazy |

**MCP tool naming**: `mcp__plugin_<plugin-name>_<server-name>__<tool-name>`.

**Authentication methods** (from `authentication.md`):

1. **OAuth (automatic)** — SSE/HTTP; Claude Code handles 2.0 flow.
2. **Bearer tokens / API keys** — static via `headers` with `${ENV_VAR}` substitution.
3. **stdio env** — credentials via `env` block.
4. **`headersHelper` (round 3)** — script path emitting JSON headers on stdout; for JWT, HMAC signatures, time-based tokens, short-lived credentials.

### TENSION-01: `.mcp.json` root structure (unresolved after 3 rounds)

`mcp-integration/SKILL.md:27-37` shows `.mcp.json` WITHOUT a `mcpServers` wrapper. Inline form in `plugin.json.mcpServers` DOES use the wrapper. No concrete `.mcp.json` examples in any plugin.

## Lifecycle events

From `plugin-structure/SKILL.md:229`: `PreToolUse`, `PostToolUse`, `Stop`, `SubagentStop`, `SessionStart`, `SessionEnd`, `UserPromptSubmit`, `PreCompact`, `Notification`.

Only 5 of 9 have executable examples in the 13 plugins.

## Plugin Settings subsystem

From `plugin-settings/SKILL.md:1-545`:

- File: `.claude/<plugin-name>.local.md` (project root, gitignored)
- Structure: YAML frontmatter (`enabled`, arbitrary keys) + markdown body
- Purpose: per-project, user-managed plugin config and state
- Readable from: hooks (bash sed/grep), commands (Read tool), agents (instructions)
- Primary use case: toggle hooks on/off without restarting
- Recommended file permissions: `chmod 600`
- Gitignore: `.claude/*.local.md`

## Relationships

```
Plugin ──1:1── plugin.json
       ├──0..N── Command ─── (prompt may invoke) ──► Agent via Task tool
       │                 ─── (prompt may load) ──► Skill via description match
       ├──0..N── Agent  ─── declares allowed Tools, selects model
       ├──0..N── Skill  ─── auto-triggered by description match
       ├──0..N── Hook   ─── subscribes to Event + Matcher → {command|prompt} handler
       ├──0..N── MCP server ─── exposes tools as mcp__plugin_<p>_<s>__<t>
       └──0..1── PluginSettings (.claude/<name>.local.md, project-scope, user-managed)
```

## Critical distinction

Agents and Skills are disjoint activation paths. Agents = explicit `Task` spawn. Skills = runtime auto-activation from description match.

## Convergence declaration

Pass 2 has converged after round 3. Remaining gaps (name collision resolution across plugins, regex flavor name, `.mcp.json` root structure) have no new evidence in source after three rounds of deepening.
