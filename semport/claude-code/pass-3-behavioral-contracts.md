# Pass 3 — Behavioral Contracts

_Phase B convergence round 2._

## Changes from Phase A

- Added BC-DRAFT-H08: `type:"prompt"` hook variant (`hook-development/SKILL.md:22-34`)
- Added BC-DRAFT-H09: timeout validation bounds (`validate-hook-schema.sh:131-137`)
- Added BC-DRAFT-MCP02..MCP05: transport taxonomy, tool naming, lazy startup, inline-vs-file config
- Added BC-DRAFT-M05: `hooks`/`mcpServers` accept inline object or file path (`manifest-reference.md:273-325`)
- Added BC-DRAFT-M06: default component paths (`manifest-reference.md:214,248,263,302`)
- Documented TENSION-01: `.mcp.json` root structure inconsistency
- Downgraded BC-DRAFT-MCP01 scope to "inline form only"

### Changes from round 1 (round 2)

- **Contradicted round 1**: BC-DRAFT-H09 default timeout is documented — command 60s, prompt 30s (`hook-development/SKILL.md:491`, not `validate-hook-schema.sh`). Previous GAP closed.
- **Contradicted round 1 for stdio**: MCP stdio startup is eager (process for session lifetime), not lazy (`server-types.md:38-44`). TENSION-02 partially resolved.
- Added BC-DRAFT-H10..H18 (matcher regex, permissionDecision, Stop decision envelope, parallel execution, merge with user hooks, hot-reload prohibition, CLAUDE_ENV_FILE, exit-code taxonomy, env-var catalog).
- Added BC-DRAFT-C06 / C07 (command frontmatter: String-or-Array `allowed-tools`, `disable-model-invocation`, scope triad).
- Added BC-DRAFT-M07 (nested command discovery not automatic).
- Added BC-DRAFT-PS01 / PS02 (Plugin Settings subsystem — new).

Confidence: HIGH = from code/JSON, MEDIUM = from README/skill prose, LOW = inferred.

## Manifest

- **BC-DRAFT-M01** (HIGH): Runtime only loads plugin if `./.claude-plugin/plugin.json` exists exactly (`manifest-reference.md:5-9`).
- **BC-DRAFT-M02** (HIGH): `name` must match `^[a-z][a-z0-9]*(-[a-z0-9]+)*$` (`manifest-reference.md:34-36`).
- **BC-DRAFT-M03** (HIGH): Custom component paths supplement, not replace, defaults; name collisions cause errors (`manifest-reference.md:365-371`).
- **BC-DRAFT-M04** (HIGH): Paths must be `./`-prefixed, forward-slash, no `../` (`manifest-reference.md:334-350`).
- **BC-DRAFT-M05** (HIGH): `hooks` and `mcpServers` fields accept EITHER a relative file path (string) OR an inline object literal (`manifest-reference.md:266-291, 300-325`).
- **BC-DRAFT-M06** (HIGH): Defaults when field omitted — `commands: ./commands`, `agents: ./agents`, `hooks: ./hooks/hooks.json`, `mcpServers: ./.mcp.json` (`manifest-reference.md:214,248,263,302`).
- **BC-DRAFT-M07** (MEDIUM, round 2): Nested subdirectories under `commands/` are NOT auto-discovered; must be explicitly enumerated in the `commands` array (`component-patterns.md:111-121`).

## Command

- **BC-DRAFT-C01** (HIGH): Commands auto-discover from `commands/*.md`.
- **BC-DRAFT-C02** (HIGH): `allowed-tools` constrains the command, supports subcommand globs for Bash (`commit.md:2`). Also accepts fully-qualified MCP tool names like `mcp__plugin_<p>_<s>__<t>` and wildcards `mcp__plugin_<p>_<s>__*` (`mcp-integration/SKILL.md:206-220`). Field type is **String OR Array** (`command-development/SKILL.md:132-143`).
- **BC-DRAFT-C03** (HIGH): Backtick-bang expressions in command bodies execute at invocation time and embed output into the prompt (`commit.md:5-9`).
- **BC-DRAFT-C04** (MEDIUM): `model:` on command sets model for that command.
- **BC-DRAFT-C05** (MEDIUM): `argument-hint` is display-only, not enforcing.
- **BC-DRAFT-C06** (HIGH, round 2): `disable-model-invocation: true` prevents the `SlashCommand` tool from programmatically invoking the command — manual-only (`command-development/SKILL.md:182-193`).
- **BC-DRAFT-C07** (HIGH, round 2): Command discovery has three scopes with distinct `/help` labels — `.claude/commands/` ("project"), `~/.claude/commands/` ("user"/personal), `<plugin>/commands/` ("plugin-name") (`command-development/SKILL.md:55-72`).

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
- **BC-DRAFT-H01b** (HIGH): User-settings `~/.claude/settings.json` hook format OMITS the wrapper — events at top level (`hook-development/SKILL.md:102-118`).
- **BC-DRAFT-H02** (HIGH): Omitting `matcher` = match all invocations of that event (`hookify/hooks/hooks.json`).
- **BC-DRAFT-H03** (HIGH): PreToolUse hooks can block via exit 2 + stderr (`security_reminder_hook.py:271-273`).
- **BC-DRAFT-H04** (HIGH): Hook scripts receive stdin JSON with common fields `{session_id, transcript_path, cwd, permission_mode, hook_event_name}` plus event-specifics: PreToolUse/PostToolUse add `tool_name`, `tool_input`, `tool_result`; UserPromptSubmit adds `user_prompt`; Stop/SubagentStop add `reason` (`hook-development/SKILL.md:302-319`).
- **BC-DRAFT-H05** (HIGH): SessionStart hooks inject prompt text via `{hookSpecificOutput: {hookEventName: "SessionStart", additionalContext: "..."}}` on stdout (`explanatory-output-style/hooks-handlers/session-start.sh:6-13`).
- **BC-DRAFT-H06** (HIGH): Hook commands must use `${CLAUDE_PLUGIN_ROOT}`; hardcoded paths prohibited.
- **BC-DRAFT-H07** (HIGH): Hook scripts should fail-open on import/parse errors (`hookify/hooks/pretooluse.py:29-32`).
- **BC-DRAFT-H08** (HIGH): Handler supports `type:"prompt"` in addition to `type:"command"`. Schema: `{type:"prompt", prompt, timeout?}`. Supported ONLY on events `Stop`, `SubagentStop`, `UserPromptSubmit`, `PreToolUse` (`hook-development/SKILL.md:22-34`).
- **BC-DRAFT-H09** (HIGH, **updated round 2**): Default `timeout` when omitted — **command hooks 60 seconds, prompt hooks 30 seconds** (`hook-development/SKILL.md:491`). Contradicts round 1 which said default was undocumented. Validator additionally warns above 600s (`validate-hook-schema.sh:131-137`).
- **BC-DRAFT-H10** (HIGH, round 2): Matcher is **case-sensitive** regex. Supports exact (`"Write"`), pipe-alternation (`"Read|Write|Edit"`), wildcard (`"*"`), and full regex (`"mcp__.*__delete.*"`) (`hook-development/SKILL.md:386-425`). Flavor unspecified.
- **BC-DRAFT-H11** (HIGH, round 2): PreToolUse hooks emit `{hookSpecificOutput: {permissionDecision: "allow"|"deny"|"ask", updatedInput: {...}}, systemMessage}`. The hook can **rewrite the tool input** before execution, not merely approve/deny (`hook-development/SKILL.md:144-153`).
- **BC-DRAFT-H12** (HIGH, round 2): Stop/SubagentStop hooks emit `{decision: "approve"|"block", reason, systemMessage}` to gate agent termination (`hook-development/SKILL.md:202-209`).
- **BC-DRAFT-H13** (HIGH, round 2): All matching hooks for an event run **in parallel**. Ordering is non-deterministic. Hooks cannot observe each other's output. Cross-hook state sharing is unsafe within one event (`hook-development/SKILL.md:496-518`).
- **BC-DRAFT-H14** (HIGH, round 2): Plugin hooks and user-settings hooks **merge**; both fire on the same event, all in parallel (`hook-development/SKILL.md:383`).
- **BC-DRAFT-H15** (HIGH, round 2, **fills GAP**): Hooks load at session start. Editing `hooks.json`, adding hook scripts, or modifying commands/prompts requires exiting and restarting Claude Code — **no hot-reload** (`hook-development/SKILL.md:572-589`; `component-patterns.md:16-18`).
- **BC-DRAFT-H16** (HIGH, round 2): In SessionStart hooks only, appending shell-export lines to `$CLAUDE_ENV_FILE` persists env vars into the session (`hook-development/SKILL.md:259-263`).
- **BC-DRAFT-H17** (HIGH, round 2): Hook exit-code taxonomy — `0` = success (stdout → transcript), `2` = blocking (stderr → fed back to Claude), any other non-zero = non-blocking error (`hook-development/SKILL.md:294-298`).
- **BC-DRAFT-H18** (HIGH, round 2): Full env-var set available to command hooks: `CLAUDE_PROJECT_DIR`, `CLAUDE_PLUGIN_ROOT`, `CLAUDE_ENV_FILE` (SessionStart only), `CLAUDE_CODE_REMOTE` (set iff remote context) (`hook-development/SKILL.md:322-329`).

## MCP

- **BC-DRAFT-MCP01** (HIGH, from docs): `.mcp.json` file format — inline in `plugin.json.mcpServers` uses `{mcpServers: {<name>: {...}}}` wrapper (`manifest-reference.md:312-325`). Standalone `.mcp.json` example in `mcp-integration/SKILL.md:27-37` omits the wrapper — see TENSION-01.
- **BC-DRAFT-MCP02** (HIGH, from docs): Four MCP transports supported: `stdio` (default; needs `command`/`args`/`env`), `sse` (`type:"sse"` + `url`, OAuth), `http` (`type:"http"` + `url` + optional `headers`, bearer token), `ws` (`type:"ws"` + `url` + optional `headers`) (`mcp-integration/SKILL.md:65-165`).
- **BC-DRAFT-MCP03** (HIGH, from docs): Runtime prefixes MCP-provided tool names as `mcp__plugin_<plugin-name>_<server-name>__<tool-name>`. Commands reference these fully-qualified names in `allowed-tools`; wildcard `mcp__plugin_<p>_<s>__*` permitted but discouraged (`mcp-integration/SKILL.md:190-220`).
- **BC-DRAFT-MCP04** (MEDIUM, **updated round 2**): stdio MCP servers are **eager** — Claude Code spawns the process at init, it runs for the entire Claude Code session, and is terminated on exit (`server-types.md:38-44`). Contradicts round 1 which claimed stdio was lazy. Remote transports (SSE/HTTP/WS) may be lazy; lazy semantics per remote transport still unstated — see TENSION-02 (narrowed).
- **BC-DRAFT-MCP05** (HIGH): Env var substitution in MCP configs supports `${CLAUDE_PLUGIN_ROOT}` AND arbitrary user-shell variables in `env` / `headers` values (`mcp-integration/SKILL.md:167-187`).

## Plugin Settings (round 2, new subsystem)

- **BC-DRAFT-PS01** (HIGH, round 2): Per-project plugin configuration may be stored at `.claude/<plugin-name>.local.md`, structured as YAML frontmatter + markdown body. User-managed, not committed to git (`plugin-settings/SKILL.md:1-60`).
- **BC-DRAFT-PS02** (HIGH, round 2): Hooks, commands, and agents may read this file at runtime to toggle behavior (e.g. hook early-exits on `enabled: false`). This is the ONLY runtime-adjustable config path since hook definitions themselves cannot hot-reload (`plugin-settings/SKILL.md:60-200`).

## Tensions / Open questions

- **TENSION-01**: Is `.mcp.json` root an `mcpServers` wrapper object, or direct name→config map? Still unresolved.
- **TENSION-02** (narrowed round 2): stdio is eager; remote transports may be lazy but not definitively stated per transport.
- **GAP (filled round 2)**: Default `timeout` values — closed: 60s command, 30s prompt.
- **GAP (filled round 2)**: Matcher regex flavor — closed as: case-sensitive regex with pipe alternation, `.*` wildcard support; exact flavor (ECMA/PCRE) still unstated but examples imply ECMA/PCRE-compatible.
- **GAP (filled round 2)**: Hot-reload behavior — closed: no hot-reload, restart required.
- **GAP**: Name collision resolution across plugins (two plugins defining same skill/command name) is undocumented.
- **GAP**: Matcher regex dialect name (ECMA vs PCRE vs Go vs RE2) — never stated.

## Confirmed gaps (unchanged)

1. No tests anywhere
2. 4 of 9 lifecycle events unexercised in any plugin
3. No concrete `.mcp.json` files in any of the 13 plugins
4. No versioning/migration contract
5. ~~No auto-reload / hot-reload behavior documented~~ — **filled round 2**: no hot-reload, by design.
