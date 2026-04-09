# Pass 2 — Domain Model

_Phase B convergence round 2._

## Changes from Phase A

- Added `"prompt"` hook type alongside `"command"` (`hook-development/SKILL.md:22-34`)
- Added MCP transport taxonomy: stdio, SSE, HTTP, ws (`mcp-integration/SKILL.md:65-165`)
- Added dual-format note for hooks config (plugin wrapper vs settings direct) (`hook-development/SKILL.md:60-119`)
- Added MCP tool naming convention (`mcp-integration/SKILL.md:190-200`)
- Documented default paths for `commands`/`agents`/`hooks`/`mcpServers` (`manifest-reference.md:214,248,263,302`)
- Clarified `hooks` and `mcpServers` accept inline object OR file path (`manifest-reference.md:273-325`)
- Noted tension: `.mcp.json` root structure inconsistent between docs

### Changes from round 1 (round 2)

- Added **Plugin Settings** as first-class subsystem — new concept entirely (`plugin-settings/SKILL.md:1-60`).
- Clarified command **scope triad**: project, personal, plugin (`command-development/SKILL.md:55-72`).
- Added command frontmatter field **`disable-model-invocation`** (`command-development/SKILL.md:182-193`).
- Corrected `allowed-tools` type to **String OR Array** (round 1 implied comma-string only) (`command-development/SKILL.md:132-143`).
- Added component-lifecycle Discovery/Activation phases; discovery is init-only, definitively answering hot-reload question (`component-patterns.md:7-26`).
- Recorded: **nested command discovery is not automatic** — subdirectories require explicit custom-path entries (`component-patterns.md:111-121`).
- Expanded hook stdin envelope with `transcript_path`, `cwd`, `permission_mode`, `hook_event_name`, and event-specifics (`hook-development/SKILL.md:302-319`).
- Expanded env-var catalog: `CLAUDE_PROJECT_DIR`, `CLAUDE_PLUGIN_ROOT`, `CLAUDE_ENV_FILE` (SessionStart-only), `CLAUDE_CODE_REMOTE` (`hook-development/SKILL.md:322-329`).
- Resolved TENSION-02 for stdio: stdio MCP servers are **eager** (process runs for entire Claude Code session) (`server-types.md:38-44`). Remote transports (SSE/HTTP/WS) remain potentially lazy.

## Core concepts

| Concept | Definition | Key fields |
|---|---|---|
| **Plugin** | Directory with `.claude-plugin/plugin.json` | `name` (kebab, required), `version`, `description`, `author`, `homepage`, `repository`, `license`, `keywords`, `commands`, `agents`, `hooks`, `mcpServers` |
| **Command** | `commands/*.md` with YAML frontmatter, invoked via `/slash-command` | `description`, `allowed-tools` (String OR Array), `model`, `argument-hint`, `disable-model-invocation` |
| **Command scope** | Discovery location determines label in `/help` | project (`.claude/commands/`), personal (`~/.claude/commands/`), plugin (`<plugin>/commands/`) |
| **Agent** | `agents/*.md` subagent, own prompt/model/tools, invoked via `Task` tool | `name`, `description` (with `<example>` dialogs), `tools` (comma-string), `model` (incl. `inherit`), `color` |
| **Skill** | `skills/<name>/SKILL.md`, auto-activated by description match | `name`, `description` (the trigger), `version`, optional `license` |
| **Hook** | Entry in `hooks.json` (or inline `plugin.json.hooks`) mapping event → matcher → handler | event name, optional `matcher`, `hooks[].type` in `{"command","prompt"}`, `hooks[].command` or `hooks[].prompt`, `hooks[].timeout` |
| **Matcher** | Case-sensitive regex filter on tool name; omitted = match all | e.g. `"Write\|Edit"`, `"*"`, `"mcp__.*__delete.*"` |
| **MCP server** | External tool provider launched via `.mcp.json` or inline in `plugin.json.mcpServers` | Varies by transport: stdio needs `command`/`args`/`env`; sse/http/ws need `type` + `url` (+ optional `headers`) |
| **MCP tool name** | Auto-generated runtime identifier for server-provided tools | `mcp__plugin_<plugin-name>_<server-name>__<tool-name>`, wildcard-referenceable in `allowed-tools` |
| **Plugin Settings** | Per-project plugin config at `.claude/<plugin-name>.local.md` — YAML frontmatter + markdown body; user-managed, not in git (`plugin-settings/SKILL.md:1-60`) | `enabled`, arbitrary keys |
| **Output style** | NOT a first-class construct. Emulated via SessionStart hook + `additionalContext` | — |
| **Progressive disclosure** | Convention: `SKILL.md` lean, heavy docs in `references/`, `examples/` | — |
| **`${CLAUDE_PLUGIN_ROOT}`** | Env var pointing at plugin dir; only portable intra-plugin path reference | Mandatory for hook commands, MCP server paths |

## Default component paths

From `manifest-reference.md:214,248,263,302`:

| Field | Default | Override accepts |
|---|---|---|
| `commands` | `./commands` | String or string[] (dir or file) |
| `agents` | `./agents` | String or string[] |
| `hooks` | `./hooks/hooks.json` | File path OR inline object |
| `mcpServers` | `./.mcp.json` | File path OR inline object |

Custom paths **supplement** defaults. Paths must be `./`-prefixed, forward-slash, no `../`. **Nested command discovery is not automatic** — subdirectories under `commands/` require explicit additional entries in the `commands` array (`component-patterns.md:111-121`).

## Component lifecycle (round 2)

From `component-patterns.md:7-26`:

1. **Discovery** — at Claude Code init only. Scan enabled plugins → read `plugin.json` → discover components in default/custom paths → parse frontmatter → register → initialize (start MCP servers, register hooks).
2. **Activation** — continuous during session. Commands on slash invocation; Agents on Task dispatch; Skills on description match; Hooks on event fire; MCP tools on tool call.

**Implication:** No hot-reload. Editing plugin files requires restarting Claude Code.

## Hook types

From `hook-development/SKILL.md:22-59`:

1. **`type: "command"`** — bash/exec command, any event, schema `{type, command, timeout?}`. Default timeout **60s** (`SKILL.md:491`).
2. **`type: "prompt"`** — LLM-driven evaluation, schema `{type, prompt, timeout?}`, supported ONLY on `Stop`, `SubagentStop`, `UserPromptSubmit`, `PreToolUse`. Default timeout **30s** (`SKILL.md:491`).

## Hook configuration dual format

From `hook-development/SKILL.md:60-119`:

- **Plugin format** (`hooks/hooks.json`): wrapper `{description?, hooks: {<Event>: [...]}}`
- **User settings format** (`~/.claude/settings.json`): direct `{<Event>: [...]}` — no wrapper, no description

Both reference the same event-entry schema inside. Plugin hooks **merge with user hooks** and run in parallel on the same event (`hook-development/SKILL.md:383`).

## Hook I/O envelope (round 2)

**Stdin (all hooks)** (`hook-development/SKILL.md:302-319`):

```
{ session_id, transcript_path, cwd, permission_mode, hook_event_name, ...event-specifics }
```

Event-specific fields: PreToolUse/PostToolUse add `tool_name`, `tool_input`, `tool_result`; UserPromptSubmit adds `user_prompt`; Stop/SubagentStop add `reason`.

**Stdout — standard envelope** (`SKILL.md:282-292`): `{continue?, suppressOutput?, systemMessage?}`.

**Stdout — PreToolUse-specific** (`SKILL.md:144-153`):

```
{ hookSpecificOutput: { permissionDecision: "allow"|"deny"|"ask", updatedInput: {...} }, systemMessage }
```

Note: PreToolUse hooks can **rewrite tool input** via `updatedInput`, not just block.

**Stdout — Stop/SubagentStop-specific** (`SKILL.md:202-209`):

```
{ decision: "approve"|"block", reason, systemMessage }
```

**Exit codes** (`SKILL.md:294-298`): 0 = success (stdout → transcript); 2 = blocking (stderr → Claude); other = non-blocking error.

## Environment variables (round 2)

From `hook-development/SKILL.md:322-329`:

| Var | Scope | Purpose |
|---|---|---|
| `CLAUDE_PROJECT_DIR` | All hooks | Project root path |
| `CLAUDE_PLUGIN_ROOT` | All hooks | Plugin directory (mandatory for portable paths) |
| `CLAUDE_ENV_FILE` | SessionStart only | Append `export FOO=bar` lines to persist env vars into the session |
| `CLAUDE_CODE_REMOTE` | All hooks | Set iff running in remote context |

## Matcher semantics (round 2)

From `hook-development/SKILL.md:386-425`:

- **Case-sensitive**
- Exact: `"Write"`
- Alternation (pipe): `"Read|Write|Edit"`
- Wildcard all: `"*"`
- Full regex: `"mcp__.*__delete.*"`, `"mcp__plugin_asana_.*"`, `"mcp__.*"`

Flavor is unspecified (likely ECMA/PCRE-compatible given regex examples like `.*`).

## Parallel execution (round 2)

All matching hooks for an event run in parallel (`hook-development/SKILL.md:496-518`):

- Hooks cannot see each other's output
- Ordering is non-deterministic
- Hooks must be designed for independence
- State sharing across parallel hooks is unsafe (only sequential cross-event workflows work, e.g. PreToolUse→PostToolUse)

## MCP server transports

From `mcp-integration/SKILL.md:65-165`:

| Transport | Key | Required fields | Auth | Startup |
|---|---|---|---|---|
| stdio | (no `type`) | `command`, optional `args`, `env` | env vars | **Eager** — process spawned at init, runs entire session (`server-types.md:38-44`) |
| sse | `"type": "sse"` | `url` | OAuth (automatic) | Potentially lazy |
| http | `"type": "http"` | `url`, optional `headers` | bearer token | Potentially lazy |
| ws | `"type": "ws"` | `url`, optional `headers` | bearer token | Potentially lazy |

**MCP tool naming**: `mcp__plugin_<plugin-name>_<server-name>__<tool-name>`. Wildcard allowed in `allowed-tools` (e.g. `mcp__plugin_asana_asana__*`) but discouraged.

**Startup** (resolved for stdio; unresolved for remote): stdio is eager (process lifetime = session lifetime, terminated on Claude Code exit). Remote transports may be lazy; lazy-vs-eager semantics not stated per remote transport.

### Open tension: `.mcp.json` root structure

`mcp-integration/SKILL.md:27-37` shows `.mcp.json` WITHOUT a `mcpServers` wrapper. Inline form in `plugin.json.mcpServers` DOES use the wrapper. Still unresolved — no concrete `.mcp.json` examples in any plugin.

## Lifecycle events

From `plugin-structure/SKILL.md:229`: `PreToolUse`, `PostToolUse`, `Stop`, `SubagentStop`, `SessionStart`, `SessionEnd`, `UserPromptSubmit`, `PreCompact`, `Notification`.

Of these, only `PreToolUse`, `PostToolUse`, `Stop`, `SessionStart`, `UserPromptSubmit` have executable examples in the 13 plugins. The other four appear only in documentation.

## Plugin Settings subsystem (round 2)

From `plugin-settings/SKILL.md:1-200`:

- File: `.claude/<plugin-name>.local.md` (project root, gitignored)
- Structure: YAML frontmatter (`enabled`, arbitrary keys) + markdown body
- Purpose: per-project, user-managed plugin config and state
- Readable from: hooks (bash sed/grep), commands (Read tool), agents (instructions)
- Primary use case: toggle hooks on/off without restarting (except: must restart for hooks config changes; but hook *scripts* can read the file at runtime)
- Pattern: hook script sed-parses frontmatter, checks `enabled`, exits 0 early if disabled

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

## Hook lifecycle (HIGH-confidence)

1. Event fires in runtime
2. Runtime enumerates enabled plugins, reads each `hooks.json` (or inline `plugin.json.hooks`)
3. For matching entries, **all matching handlers run in parallel**
4. For `type:"command"` execs command with env vars + stdin JSON envelope
5. For `type:"prompt"` invokes LLM with prompt template and variable substitution (`$TOOL_INPUT`, `$USER_PROMPT`, etc.)
6. Script communicates via exit code (0/2/other), stderr (shown to model on block), stdout JSON (standard envelope + event-specific sections)
7. Plugin hooks merge with user hooks; all run in parallel
