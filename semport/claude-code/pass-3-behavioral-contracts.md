# Pass 3 — Behavioral Contracts

24 draft contracts extracted. Confidence: HIGH = from code/JSON, MEDIUM = from README prose.

## Manifest (all HIGH)

- **BC-DRAFT-M01**: Runtime only loads plugin if `./.claude-plugin/plugin.json` exists exactly (`manifest-reference.md:5-9`).
- **BC-DRAFT-M02**: `name` must match `^[a-z][a-z0-9]*(-[a-z0-9]+)*$` (`manifest-reference.md:34-36`).
- **BC-DRAFT-M03**: Custom component paths **supplement**, not replace, defaults; name collisions error (`manifest-reference.md:365-371`).
- **BC-DRAFT-M04**: Paths must be `./`-prefixed, forward-slash, no `../` (`manifest-reference.md:334-350`).

## Command

- **BC-DRAFT-C01** (HIGH): Commands auto-discover from `commands/*.md`.
- **BC-DRAFT-C02** (HIGH): `allowed-tools` constrains the command, supports subcommand globs for Bash (`commit.md:2`).
- **BC-DRAFT-C03** (HIGH): Backtick-bang expressions in command bodies execute at invocation time and embed output into prompt (`commit.md:5-9`).
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
- **BC-DRAFT-S03** (HIGH): SKILL.md description should enumerate triggering phrases explicitly. Strongest observed pattern — `plugin-dev/skills/plugin-structure/SKILL.md:3` literally lists every user phrase.
- **BC-DRAFT-S04** (HIGH): Progressive disclosure via `references/` and `examples/` subdirs.

## Hook (all HIGH)

- **BC-DRAFT-H01**: `hooks.json` = `{description, hooks: {<EventName>: [{matcher?, hooks: [{type, command, timeout?}]}]}}`.
- **BC-DRAFT-H02**: Omitting `matcher` = match all invocations of that event (`hookify/hooks/hooks.json`).
- **BC-DRAFT-H03**: PreToolUse hooks block via exit 2 + stderr (`security_reminder_hook.py:271-273`).
- **BC-DRAFT-H04**: Hook scripts receive stdin JSON `{session_id, tool_name, tool_input,...}` (`security_reminder_hook.py:231-242`).
- **BC-DRAFT-H05**: SessionStart hooks inject prompt text by emitting `{hookSpecificOutput: {hookEventName: "SessionStart", additionalContext: "..."}}` on stdout (`explanatory-output-style/hooks-handlers/session-start.sh:6-13`).
- **BC-DRAFT-H06**: All hook commands must use `${CLAUDE_PLUGIN_ROOT}`; hardcoded paths prohibited.
- **BC-DRAFT-H07**: Hook scripts should fail-open on import/parse errors to not block user work (`hookify/hooks/pretooluse.py:29-32`).

## MCP (HIGH-from-docs only)

- **BC-DRAFT-MCP01**: `.mcp.json` declares `{mcpServers: {<name>: {command, args, env}}}`; servers auto-start on plugin enable. **Zero concrete `.mcp.json` examples in the 13 plugins.**

## Gaps

1. No tests anywhere
2. 4 of 9 lifecycle events unexercised
3. No concrete MCP examples
4. No versioning/migration contract
