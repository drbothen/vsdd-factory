# Pass 3 — Behavioral Contracts

_Phase B convergence round 3 — **CONVERGED**._

## Changes from Phase A

- Added BC-DRAFT-H08: `type:"prompt"` hook variant (`hook-development/SKILL.md:22-34`)
- Added BC-DRAFT-H09: timeout validation bounds (`validate-hook-schema.sh:131-137`)
- Added BC-DRAFT-MCP02..MCP05: transport taxonomy, tool naming, lazy startup, inline-vs-file config
- Added BC-DRAFT-M05: `hooks`/`mcpServers` accept inline object or file path
- Added BC-DRAFT-M06: default component paths
- Documented TENSION-01: `.mcp.json` root structure inconsistency

### Changes from round 1 (round 2)

- **Contradicted round 1**: BC-DRAFT-H09 default timeout is documented — command 60s, prompt 30s.
- **Contradicted round 1 for stdio**: MCP stdio startup is eager. TENSION-02 partially resolved.
- Added BC-DRAFT-H10..H18 (matcher regex, permissionDecision, Stop decision envelope, parallel execution, hook merge, no-hot-reload, CLAUDE_ENV_FILE, exit-code taxonomy, env-var catalog).
- Added BC-DRAFT-C06 / C07 (command frontmatter: String-or-Array `allowed-tools`, `disable-model-invocation`, scope triad).
- Added BC-DRAFT-M07 (nested command discovery not automatic).
- Added BC-DRAFT-PS01 / PS02 (Plugin Settings subsystem).

### Changes from round 2 (round 3)

- Added **BC-DRAFT-A05** (agent frontmatter validation regex and length bounds).
- Added **BC-DRAFT-A06** (agent `model` / `color` enumerated values; `color` required).
- Added **BC-DRAFT-MCP06** (`headersHelper` dynamic auth script).
- **TENSION-03 raised**: `validate-hook-schema.sh:70-75` requires `matcher`; docs and example plugins treat as optional. BC-DRAFT-H02 scope narrowed.
- **TENSION-04 raised**: agent `tools` is array per `agent-development/SKILL.md:149` but comma-string per in-repo examples.
- Declared convergence after round 3.

Confidence: HIGH = from code/JSON, MEDIUM = from README/skill prose, LOW = inferred.

## Manifest

- **BC-DRAFT-M01** (HIGH): Runtime only loads plugin if `./.claude-plugin/plugin.json` exists exactly.
- **BC-DRAFT-M02** (HIGH): `name` must match `^[a-z][a-z0-9]*(-[a-z0-9]+)*$`.
- **BC-DRAFT-M03** (HIGH): Custom component paths supplement, not replace, defaults.
- **BC-DRAFT-M04** (HIGH): Paths must be `./`-prefixed, forward-slash, no `../`.
- **BC-DRAFT-M05** (HIGH): `hooks` and `mcpServers` fields accept EITHER a relative file path OR an inline object literal.
- **BC-DRAFT-M06** (HIGH): Defaults — `commands: ./commands`, `agents: ./agents`, `hooks: ./hooks/hooks.json`, `mcpServers: ./.mcp.json`.
- **BC-DRAFT-M07** (MEDIUM, round 2): Nested subdirectories under `commands/` are NOT auto-discovered.

## Command

- **BC-DRAFT-C01** (HIGH): Commands auto-discover from `commands/*.md`.
- **BC-DRAFT-C02** (HIGH): `allowed-tools` supports subcommand globs for Bash. Accepts MCP tool names and wildcards. Field type is **String OR Array**.
- **BC-DRAFT-C03** (HIGH): Backtick-bang expressions execute at invocation time. Positional args `$1`, `$2`, ... and `$ARGUMENTS` expand in command body (`command-development/SKILL.md:200-263`).
- **BC-DRAFT-C04** (MEDIUM): `model:` on command sets model for that command.
- **BC-DRAFT-C05** (MEDIUM): `argument-hint` is display-only.
- **BC-DRAFT-C06** (HIGH, round 2): `disable-model-invocation: true` prevents programmatic `SlashCommand` invocation.
- **BC-DRAFT-C07** (HIGH, round 2): Command discovery has three scopes with distinct `/help` labels. Subdirectories get appended to the label, e.g. `(project:ci)`.

## Agent

- **BC-DRAFT-A01** (HIGH): Agents auto-discover from `agents/*.md`.
- **BC-DRAFT-A02** (HIGH): Agents may declare `tools:` whitelist. Format ambiguous: `agent-development/SKILL.md:149` prescribes JSON array; in-repo example `silent-failure-hunter.md:4` uses comma-string. Runtime may accept both (TENSION-04).
- **BC-DRAFT-A03** (HIGH): `model: inherit` delegates to parent session model.
- **BC-DRAFT-A04** (MEDIUM): Agent description should include 2-3 `<example>` dialogue blocks.
- **BC-DRAFT-A05** (HIGH, round 3): Agent `name` must be 3-50 characters, lowercase alphanumeric and hyphens only, must start and end with alphanumeric, no underscores (`agent-development/SKILL.md:69-80,264-273`). `description` 10-5000 chars. System prompt body 20-10000 chars.
- **BC-DRAFT-A06** (HIGH, round 3): Agent `model` is an enumerated value set `{inherit, sonnet, opus, haiku}`. Agent `color` is **required** and enumerated `{blue, cyan, green, yellow, magenta, red}` (`agent-development/SKILL.md:128-141,351-357`).

## Skill

- **BC-DRAFT-S01** (HIGH): Each skill dir needs `SKILL.md`; `README.md` doesn't count.
- **BC-DRAFT-S02** (HIGH): Skills auto-activate by matching user intent against `description:`.
- **BC-DRAFT-S03** (HIGH): SKILL.md description should enumerate triggering phrases explicitly, in third person ("This skill should be used when...") (`skill-development/SKILL.md:162-182`).
- **BC-DRAFT-S04** (HIGH): Progressive disclosure via `references/` and `examples/` subdirs. Three-level loading: metadata always in context, SKILL.md body on trigger, bundled resources on demand (`skill-development/SKILL.md:77-86`).

## Hook

- **BC-DRAFT-H01** (HIGH): Plugin `hooks.json` format = `{description?, hooks: {<EventName>: [{matcher?, hooks: [<handler>]}]}}`.
- **BC-DRAFT-H01b** (HIGH): User-settings `~/.claude/settings.json` hook format OMITS the wrapper — events at top level.
- **BC-DRAFT-H02** (HIGH, **narrowed round 3**): Omitting `matcher` = match all invocations of that event per prose docs and `hookify/hooks/hooks.json`. **TENSION-03**: `validate-hook-schema.sh:70-75` reports an error when `matcher` is missing. Runtime behavior unconfirmed.
- **BC-DRAFT-H03** (HIGH): PreToolUse hooks can block via exit 2 + stderr.
- **BC-DRAFT-H04** (HIGH): Hook scripts receive stdin JSON with common fields `{session_id, transcript_path, cwd, permission_mode, hook_event_name}` plus event-specifics.
- **BC-DRAFT-H05** (HIGH): SessionStart hooks inject prompt text via `{hookSpecificOutput: {hookEventName: "SessionStart", additionalContext: "..."}}`.
- **BC-DRAFT-H06** (HIGH): Hook commands must use `${CLAUDE_PLUGIN_ROOT}`; hardcoded paths prohibited (enforced as warning by `validate-hook-schema.sh:111-114`).
- **BC-DRAFT-H07** (HIGH): Hook scripts should fail-open on import/parse errors.
- **BC-DRAFT-H08** (HIGH): Handler supports `type:"prompt"` in addition to `type:"command"`. Supported ONLY on `Stop`, `SubagentStop`, `UserPromptSubmit`, `PreToolUse` (validator enforces at `validate-hook-schema.sh:123-127`).
- **BC-DRAFT-H09** (HIGH, updated round 2): Default `timeout` — command 60s, prompt 30s. Validator warns above 600s and below 5s (`validate-hook-schema.sh:131-142`).
- **BC-DRAFT-H10** (HIGH, round 2): Matcher is **case-sensitive** regex. Supports exact, pipe-alternation, wildcard, full regex. Flavor unspecified.
- **BC-DRAFT-H11** (HIGH, round 2): PreToolUse hooks emit `{hookSpecificOutput: {permissionDecision: "allow"|"deny"|"ask", updatedInput: {...}}, systemMessage}`. Can **rewrite tool input**.
- **BC-DRAFT-H12** (HIGH, round 2): Stop/SubagentStop hooks emit `{decision: "approve"|"block", reason, systemMessage}`.
- **BC-DRAFT-H13** (HIGH, round 2): All matching hooks run **in parallel**, non-deterministic order, no cross-hook state.
- **BC-DRAFT-H14** (HIGH, round 2): Plugin hooks and user-settings hooks **merge**.
- **BC-DRAFT-H15** (HIGH, round 2): Hooks load at session start — **no hot-reload**. Restart required.
- **BC-DRAFT-H16** (HIGH, round 2): SessionStart hooks can persist env vars via `$CLAUDE_ENV_FILE`.
- **BC-DRAFT-H17** (HIGH, round 2): Exit-code taxonomy — 0 success, 2 blocking, other non-blocking error.
- **BC-DRAFT-H18** (HIGH, round 2): Full env-var set: `CLAUDE_PROJECT_DIR`, `CLAUDE_PLUGIN_ROOT`, `CLAUDE_ENV_FILE` (SessionStart only), `CLAUDE_CODE_REMOTE`.

## MCP

- **BC-DRAFT-MCP01** (HIGH): Inline in `plugin.json.mcpServers` uses `{mcpServers: {<name>: {...}}}` wrapper. Standalone `.mcp.json` example in `mcp-integration/SKILL.md:27-37` omits the wrapper — see TENSION-01.
- **BC-DRAFT-MCP02** (HIGH): Four MCP transports: `stdio`, `sse`, `http`, `ws`.
- **BC-DRAFT-MCP03** (HIGH): Tool names prefixed `mcp__plugin_<plugin-name>_<server-name>__<tool-name>`.
- **BC-DRAFT-MCP04** (MEDIUM, updated round 2): stdio MCP servers are **eager** — process lifetime = session lifetime. Remote transports may be lazy (TENSION-02 narrowed).
- **BC-DRAFT-MCP05** (HIGH): Env var substitution supports `${CLAUDE_PLUGIN_ROOT}` and arbitrary user-shell variables in `env` / `headers`.
- **BC-DRAFT-MCP06** (HIGH, round 3): MCP remote-transport servers may declare a `headersHelper` field pointing at an executable script that emits JSON headers on stdout at each request (`authentication.md:233-258`). Intended for JWT, HMAC signatures, short-lived tokens, time-based headers. Composable with static `headers`. mTLS not directly supported.

## Plugin Settings

- **BC-DRAFT-PS01** (HIGH, round 2): Per-project plugin configuration at `.claude/<plugin-name>.local.md`. YAML frontmatter + markdown body. User-managed, gitignored (`.claude/*.local.md`). Recommended permissions: `chmod 600`.
- **BC-DRAFT-PS02** (HIGH, round 2): Hooks, commands, and agents may read this file at runtime to toggle behavior. ONLY runtime-adjustable config path.

## Tensions / Open questions

- **TENSION-01**: `.mcp.json` root wrapper vs direct. Unresolved after 3 rounds.
- **TENSION-02** (narrowed round 2): stdio is eager; remote transports may be lazy but not per-transport specified.
- **TENSION-03** (new round 3): `matcher` optional per docs and examples, but `validate-hook-schema.sh:70-75` reports error when missing.
- **TENSION-04** (new round 3): Agent `tools` field is JSON array per `agent-development/SKILL.md:149` but comma-string per in-repo examples.
- **GAP (filled round 2)**: Default `timeout` values — 60s command / 30s prompt.
- **GAP (filled round 2)**: Matcher regex flavor — case-sensitive, pipe alternation, `.*`; exact dialect name unstated.
- **GAP (filled round 2)**: Hot-reload — none, restart required.
- **GAP (unfilled, 3 rounds)**: Name collision resolution across plugins. Undocumented.
- **GAP (unfilled, 3 rounds)**: Matcher regex dialect name (ECMA / PCRE / RE2). Never stated.

## Confirmed gaps (unchanged)

1. No tests anywhere
2. 4 of 9 lifecycle events unexercised in any plugin
3. No concrete `.mcp.json` files in any of the 13 plugins
4. No versioning/migration contract

## Convergence declaration

Pass 3 has converged after round 3. Three SUBSTANTIVE findings added (A05, A06, MCP06) plus two new tensions (TENSION-03, TENSION-04). Two GAPs remain unfilled after three rounds with no new evidence in source. A fourth round would yield only nitpicks.
