# Pass 2 — Domain Model

## Core concepts

| Concept | Definition | Key fields |
|---|---|---|
| **Plugin** | Directory with `.claude-plugin/plugin.json` | `name` (kebab, required), `version`, `description`, `author`, `commands`, `agents`, `hooks`, `mcpServers` |
| **Command** | `commands/*.md` with YAML frontmatter → `/slash-command` | `description`, `allowed-tools`, `model`, `argument-hint` |
| **Agent** | `agents/*.md` subagent, own prompt/model/tools, invoked via `Task` tool | `name`, `description` (with `<example>` dialogs), `tools` (comma-string!), `model` (incl. `inherit`), `color` |
| **Skill** | `skills/<name>/SKILL.md`, **auto-activated** by description match | `name`, `description` (the trigger), `version`, optional `license` |
| **Hook** | Entry in `hooks/hooks.json` mapping event → matcher → command | event name, optional `matcher` (regex on tool name), `hooks[].type: "command"`, `hooks[].command`, `hooks[].timeout` |
| **Matcher** | Regex filter on tool name; omitted = match all | e.g. `"Edit\|Write\|MultiEdit"` |
| **MCP server** | Launched from `.mcp.json` or inline in `plugin.json` | `command`, `args`, `env` with `${CLAUDE_PLUGIN_ROOT}` |
| **Output style** | **NOT a first-class construct**. Emulated via SessionStart hook + `additionalContext`. | — |
| **Progressive disclosure** | Convention: `SKILL.md` lean, heavy docs in `references/`, `examples/` | — |
| **`${CLAUDE_PLUGIN_ROOT}`** | Env var pointing at plugin dir; **only** portable intra-plugin path reference | Mandatory for hook commands, MCP server paths |

## Lifecycle events

From `plugin-structure/SKILL.md:229`: `PreToolUse`, `PostToolUse`, `Stop`, `SubagentStop`, `SessionStart`, `SessionEnd`, `UserPromptSubmit`, `PreCompact`, `Notification`. The 13 plugins only exercise `PreToolUse`, `PostToolUse`, `Stop`, `SessionStart`, `UserPromptSubmit`. **Four events have zero examples.**

## Relationships

```
Plugin ──1:1── plugin.json
       ├──0..N── Command ─── (prompt may invoke) ──► Agent via Task tool
       │                 ─── (prompt may load) ──► Skill via Skill tool
       ├──0..N── Agent  ─── declares allowed Tools, selects model
       ├──0..N── Skill  ─── auto-triggered by description match
       ├──0..N── Hook   ─── subscribes to Event + Matcher → Script (stdin JSON, env)
       └──0..N── MCP server
```

## Critical distinction

Agents and Skills are **disjoint activation paths**. Agents = explicit `Task` spawn. Skills = runtime auto-activation from description match. This is the single most important design question when adding a capability.

## Hook lifecycle (HIGH-confidence from code)

1. Event fires in runtime
2. Runtime enumerates enabled plugins, reads each `hooks.json`
3. For matching entries, execs `command` with `CLAUDE_PLUGIN_ROOT` env + stdin JSON `{session_id, tool_name, tool_input,...}` (evidenced `security_reminder_hook.py:231-242`)
4. Script communicates via: exit code (0 allow, 2 block), stderr (shown to model on block), stdout JSON (e.g. `hookSpecificOutput.additionalContext`)
