# Pass 2 — Domain Model

_Phase B convergence round 1._

## Changes from Phase A

- Added `"prompt"` hook type alongside `"command"` (`hook-development/SKILL.md:22-34`)
- Added MCP transport taxonomy: stdio, SSE, HTTP, ws (`mcp-integration/SKILL.md:65-165`)
- Added dual-format note for hooks config (plugin wrapper vs settings direct) (`hook-development/SKILL.md:60-119`)
- Added MCP tool naming convention (`mcp-integration/SKILL.md:190-200`)
- Documented default paths for `commands`/`agents`/`hooks`/`mcpServers` (`manifest-reference.md:214,248,263,302`)
- Clarified `hooks` and `mcpServers` accept inline object OR file path (`manifest-reference.md:273-325`)
- Noted tension: `.mcp.json` root structure inconsistent between docs

## Core concepts

| Concept | Definition | Key fields |
|---|---|---|
| **Plugin** | Directory with `.claude-plugin/plugin.json` | `name` (kebab, required), `version`, `description`, `author`, `homepage`, `repository`, `license`, `keywords`, `commands`, `agents`, `hooks`, `mcpServers` |
| **Command** | `commands/*.md` with YAML frontmatter, invoked via `/slash-command` | `description`, `allowed-tools`, `model`, `argument-hint` |
| **Agent** | `agents/*.md` subagent, own prompt/model/tools, invoked via `Task` tool | `name`, `description` (with `<example>` dialogs), `tools` (comma-string), `model` (incl. `inherit`), `color` |
| **Skill** | `skills/<name>/SKILL.md`, auto-activated by description match | `name`, `description` (the trigger), `version`, optional `license` |
| **Hook** | Entry in `hooks.json` (or inline `plugin.json.hooks`) mapping event → matcher → handler | event name, optional `matcher`, `hooks[].type` in `{"command","prompt"}`, `hooks[].command` or `hooks[].prompt`, `hooks[].timeout` |
| **Matcher** | Regex filter on tool name; omitted = match all | e.g. `"Edit\|Write\|MultiEdit"` |
| **MCP server** | External tool provider launched via `.mcp.json` or inline in `plugin.json.mcpServers` | Varies by transport: stdio needs `command`/`args`/`env`; sse/http/ws need `type` + `url` (+ optional `headers`) |
| **MCP tool name** | Auto-generated runtime identifier for server-provided tools | `mcp__plugin_<plugin-name>_<server-name>__<tool-name>`, wildcard-referenceable in `allowed-tools` |
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

Custom paths **supplement** defaults (Phase A BC-DRAFT-M03). Paths must be `./`-prefixed, forward-slash, no `../`.

## Hook types

From `hook-development/SKILL.md:22-59`:

1. **`type: "command"`** — bash/exec command, any event, schema `{type, command, timeout?}`
2. **`type: "prompt"`** — LLM-driven evaluation, schema `{type, prompt, timeout?}`, supported ONLY on `Stop`, `SubagentStop`, `UserPromptSubmit`, `PreToolUse`

## Hook configuration dual format

From `hook-development/SKILL.md:60-119`:

- **Plugin format** (`hooks/hooks.json`): wrapper `{description?, hooks: {<Event>: [...]}}`
- **User settings format** (`~/.claude/settings.json`): direct `{<Event>: [...]}` — no wrapper, no description

Both reference the same event-entry schema inside.

## MCP server transports

From `mcp-integration/SKILL.md:65-165`:

| Transport | Key | Required fields | Auth |
|---|---|---|---|
| stdio | (no `type` or implicit) | `command`, optional `args`, `env` | env vars |
| sse | `"type": "sse"` | `url` | OAuth (automatic) |
| http | `"type": "http"` | `url`, optional `headers` | bearer token |
| ws | `"type": "ws"` | `url`, optional `headers` | bearer token |

**MCP tool naming**: `mcp__plugin_<plugin-name>_<server-name>__<tool-name>`. Wildcard allowed in `allowed-tools` (e.g. `mcp__plugin_asana_asana__*`) but discouraged.

**Startup**: lazy — servers connect on demand, first tool use triggers connection (`mcp-integration/SKILL.md:226-228, 403-405`). Contradicts Phase A claim of "auto-start on enable."

### Open tension: `.mcp.json` root structure

`mcp-integration/SKILL.md:27-37` shows `.mcp.json` WITHOUT a `mcpServers` wrapper (servers at root). Inline form in `plugin.json.mcpServers` DOES use the wrapper. This is either a doc inconsistency or a format-discriminator rule (root-level when file, wrapped when inline). Unresolved — no concrete `.mcp.json` examples exist in any plugin to confirm.

## Lifecycle events

From `plugin-structure/SKILL.md:229`: `PreToolUse`, `PostToolUse`, `Stop`, `SubagentStop`, `SessionStart`, `SessionEnd`, `UserPromptSubmit`, `PreCompact`, `Notification`.

Exhaustive grep of the 5 `hooks.json` files in repo: only `PreToolUse`, `PostToolUse`, `Stop`, `SessionStart`, `UserPromptSubmit` are exercised. `SessionEnd`, `SubagentStop`, `PreCompact`, `Notification` appear ONLY in documentation strings inside `plugin-dev` skills and `plugin-validator.md`, never in a runnable hook configuration. **Four events have zero executable examples.**

## Relationships

```
Plugin ──1:1── plugin.json
       ├──0..N── Command ─── (prompt may invoke) ──► Agent via Task tool
       │                 ─── (prompt may load) ──► Skill via description match
       ├──0..N── Agent  ─── declares allowed Tools, selects model
       ├──0..N── Skill  ─── auto-triggered by description match
       ├──0..N── Hook   ─── subscribes to Event + Matcher → {command|prompt} handler
       └──0..N── MCP server ─── exposes tools as mcp__plugin_<p>_<s>__<t>
```

## Critical distinction

Agents and Skills are disjoint activation paths. Agents = explicit `Task` spawn. Skills = runtime auto-activation from description match. This is the single most important design question when adding a capability.

## Hook lifecycle (HIGH-confidence from code)

1. Event fires in runtime
2. Runtime enumerates enabled plugins, reads each `hooks.json` (or inline `plugin.json.hooks`)
3. For matching entries, for `type:"command"` execs command with `CLAUDE_PLUGIN_ROOT` env + stdin JSON `{session_id, tool_name, tool_input,...}` (`security_reminder_hook.py:231-242`); for `type:"prompt"` invokes LLM with the prompt template
4. Script communicates via: exit code (0 allow, 2 block), stderr (shown to model on block), stdout JSON (e.g. `hookSpecificOutput.additionalContext`)
