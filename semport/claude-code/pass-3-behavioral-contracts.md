# Pass 3 — Behavioral Contracts

_Phase B convergence round 1._

## Changes from Phase A

- Added BC-DRAFT-H08: `type:"prompt"` hook variant (`hook-development/SKILL.md:22-34`)
- Added BC-DRAFT-H09: timeout validation bounds (`validate-hook-schema.sh:131-137`)
- Added BC-DRAFT-MCP02..MCP05: transport taxonomy, tool naming, lazy startup, inline-vs-file config
- Added BC-DRAFT-M05: `hooks`/`mcpServers` accept inline object or file path (`manifest-reference.md:273-325`)
- Added BC-DRAFT-M06: default component paths (`manifest-reference.md:214,248,263,302`)
- Documented TENSION-01: `.mcp.json` root structure inconsistency
- Downgraded BC-DRAFT-MCP01 scope to "inline form only"

Confidence: HIGH = from code/JSON, MEDIUM = from README/skill prose, LOW = inferred.

## Manifest

- **BC-DRAFT-M01** (HIGH): Runtime only loads plugin if `./.claude-plugin/plugin.json` exists exactly (`manifest-reference.md:5-9`).
- **BC-DRAFT-M02** (HIGH): `name` must match `^[a-z][a-z0-9]*(-[a-z0-9]+)*$` (`manifest-reference.md:34-36`).
- **BC-DRAFT-M03** (HIGH): Custom component paths supplement, not replace, defaults; name collisions cause errors (`manifest-reference.md:365-371`).
- **BC-DRAFT-M04** (HIGH): Paths must be `./`-prefixed, forward-slash, no `../` (`manifest-reference.md:334-350`).
- **BC-DRAFT-M05** (HIGH): `hooks` and `mcpServers` fields accept EITHER a relative file path (string) OR an inline object literal (`manifest-reference.md:266-291, 300-325`).
- **BC-DRAFT-M06** (HIGH): Defaults when field omitted — `commands: ./commands`, `agents: ./agents`, `hooks: ./hooks/hooks.json`, `mcpServers: ./.mcp.json` (`manifest-reference.md:214,248,263,302`).

## Command

- **BC-DRAFT-C01** (HIGH): Commands auto-discover from `commands/*.md`.
- **BC-DRAFT-C02** (HIGH): `allowed-tools` constrains the command, supports subcommand globs for Bash (`commit.md:2`). Also accepts fully-qualified MCP tool names like `mcp__plugin_<p>_<s>__<t>` and wildcards `mcp__plugin_<p>_<s>__*` (`mcp-integration/SKILL.md:206-220`).
- **BC-DRAFT-C03** (HIGH): Backtick-bang expressions in command bodies execute at invocation time and embed output into the prompt (`commit.md:5-9`).
- **BC-DRAFT-C04** (MEDIUM): `model:` on command sets model for that command.
- **BC-DRAFT-C05** (MEDIUM): `argument-hint` is display-only, not enforcing.

## Agent

- **BC-DRAFT-A01** (HIGH): Agents auto-discover from `agents/*.md`.
- **BC-DRAFT-A02** (HIGH): Agents may declare `tools:` whitelist (`feature-dev/agents/code-architect.md:4`).
- **BC-DRAFT-A03** (HIGH): `model: inherit` delegates to parent session model (`silent-failure-hunter.md:4`).
- **BC-DRAFT-A04** (MEDIUM): Agent description should include 2-3 `<example>` dialogue blocks for auto-selection (`silent-failure-hunter.md:3`).

## Skill

- **BC-DRAFT-S01** (HIGH): Each skill dir needs `SKILL.md`; `README.md` doesn't count (`plugin-structure/SKILL.md:452-454`).
- **BC-DRAFT-S02** (HIGH): Skills auto-activate by matching user intent against `description:`.
- **BC-DRAFT-S03** (HIGH): SKILL.md description should enumerate triggering phrases explicitly (`plugin-dev/skills/plugin-structure/SKILL.md:3`).
- **BC-DRAFT-S04** (HIGH): Progressive disclosure via `references/` and `examples/` subdirs.

## Hook

- **BC-DRAFT-H01** (HIGH): Plugin `hooks.json` format = `{description?, hooks: {<EventName>: [{matcher?, hooks: [<handler>]}]}}` (`hook-development/SKILL.md:60-100`).
- **BC-DRAFT-H01b** (HIGH): User-settings `~/.claude/settings.json` hook format OMITS the `{description, hooks:...}` wrapper — events at top level (`hook-development/SKILL.md:102-118`).
- **BC-DRAFT-H02** (HIGH): Omitting `matcher` = match all invocations of that event (`hookify/hooks/hooks.json`).
- **BC-DRAFT-H03** (HIGH): PreToolUse hooks block via exit 2 + stderr (`security_reminder_hook.py:271-273`).
- **BC-DRAFT-H04** (HIGH): Hook scripts receive stdin JSON `{session_id, tool_name, tool_input,...}` (`security_reminder_hook.py:231-242`).
- **BC-DRAFT-H05** (HIGH): SessionStart hooks inject prompt text by emitting `{hookSpecificOutput: {hookEventName: "SessionStart", additionalContext: "..."}}` on stdout (`explanatory-output-style/hooks-handlers/session-start.sh:6-13`).
- **BC-DRAFT-H06** (HIGH): Hook commands must use `${CLAUDE_PLUGIN_ROOT}`; hardcoded paths prohibited.
- **BC-DRAFT-H07** (HIGH): Hook scripts should fail-open on import/parse errors to not block user work (`hookify/hooks/pretooluse.py:29-32`).
- **BC-DRAFT-H08** (HIGH): Handler supports `type:"prompt"` in addition to `type:"command"`. Schema: `{type:"prompt", prompt, timeout?}`. Supported ONLY on events `Stop`, `SubagentStop`, `UserPromptSubmit`, `PreToolUse` (`hook-development/SKILL.md:22-34`).
- **BC-DRAFT-H09** (MEDIUM): `timeout` field, if present, must be an integer; validator warns above 600 seconds (practical max) (`validate-hook-schema.sh:131-137`). Default when omitted is undocumented in code.

## MCP

- **BC-DRAFT-MCP01** (HIGH, from docs): `.mcp.json` file format — inline in `plugin.json.mcpServers` uses `{mcpServers: {<name>: {...}}}` wrapper (`manifest-reference.md:312-325`). Standalone `.mcp.json` example in `mcp-integration/SKILL.md:27-37` omits the wrapper — see TENSION-01.
- **BC-DRAFT-MCP02** (HIGH, from docs): Four MCP transports supported: `stdio` (default; needs `command`/`args`/`env`), `sse` (`type:"sse"` + `url`, OAuth), `http` (`type:"http"` + `url` + optional `headers`, bearer token), `ws` (`type:"ws"` + `url` + optional `headers`) (`mcp-integration/SKILL.md:65-165`).
- **BC-DRAFT-MCP03** (HIGH, from docs): Runtime prefixes MCP-provided tool names as `mcp__plugin_<plugin-name>_<server-name>__<tool-name>`. Commands reference these fully-qualified names in `allowed-tools`; wildcard `mcp__plugin_<p>_<s>__*` permitted but discouraged (`mcp-integration/SKILL.md:190-220`).
- **BC-DRAFT-MCP04** (MEDIUM, from docs): MCP servers connect **lazily**: "Not all servers connect at startup. First tool use triggers connection" (`mcp-integration/SKILL.md:403-405`). Contradicts the "auto-start on plugin enable" assertion also in the same doc (line 226-228) — see TENSION-02.
- **BC-DRAFT-MCP05** (HIGH): Env var substitution in MCP configs supports `${CLAUDE_PLUGIN_ROOT}` AND arbitrary user-shell variables in `env` / `headers` values (`mcp-integration/SKILL.md:167-187`).

## Tensions / Open questions

- **TENSION-01**: Is `.mcp.json` root an `mcpServers` wrapper object, or direct name→config map? `mcp-integration/SKILL.md:27-37` shows direct; inline form in `manifest-reference.md:312-325` uses wrapper. Zero concrete `.mcp.json` files in the 13 plugins to adjudicate.
- **TENSION-02**: MCP startup — eager on plugin enable (`mcp-integration/SKILL.md:226-228`) vs lazy on first tool use (`mcp-integration/SKILL.md:403-405`). Likely transport-dependent (stdio lazy, SSE eager) but not stated.
- **GAP**: Default `timeout` value when field omitted from a hook handler is documented nowhere in the source.
- **GAP**: Matcher regex flavor (ECMA / PCRE / glob) is never specified. Docs only say "matcher patterns" without semantics.
- **GAP**: Name collision resolution across plugins (two plugins defining same skill/command name) is undocumented.

## Confirmed gaps (unchanged)

1. No tests anywhere
2. 4 of 9 lifecycle events unexercised in any plugin
3. No concrete `.mcp.json` files in any of the 13 plugins
4. No versioning/migration contract
5. No auto-reload / hot-reload behavior documented
