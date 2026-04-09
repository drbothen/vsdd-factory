# Pass 6 — Final Synthesis: Anthropic claude-code plugin ecosystem

_Phase C definitive synthesis. Supersedes the Phase A pass-6-synthesis.md. Built from converged passes 0-5 (Phase B, 2-3 rounds each)._

---

## 1. Executive Summary

The `anthropics/claude-code` repository is **not a runtime** — it is a documentation + reference-plugin monorepo hosting 13 plugin folders (138 files, ~26k lines of markdown, ~1.1k lines of Python, ~1.5k lines of shell, 285 lines of JSON) plus two ancillary script directories for repo maintenance. There is no root build system, no tests anywhere, and no shared library across plugins. The Claude Code CLI that actually loads these plugins lives outside the repo. Everything in the tree is either (a) a loadable reference plugin, (b) a conceptual skill bundle that teaches plugin authoring (`plugin-dev/`, which notably has **no `plugin.json`** of its own), or (c) GitHub-issue automation unrelated to the runtime.

The load-bearing facts are: plugins are **static file trees discovered once at session init** (`plugin-structure/references/component-patterns.md:7-26`); there is **no hot reload** — editing any file requires restarting Claude Code; the **only runtime-adjustable configuration surface** is the "Plugin Settings" overlay file at `.claude/<plugin-name>.local.md`; hooks run **in parallel** on every matching event with no cross-handler state and non-deterministic order; and plugin-format hooks and user-settings-format hooks **merge** at Discovery into one combined event-to-handler map. These five facts together pin the entire runtime model.

Three-round Phase B convergence produced substantive corrections over the Phase A broad sweep: (a) the Plugin Settings subsystem was elevated to first-class with **two** adopters (hookify for config, ralph-wiggum for cross-hook state) after round 1 incorrectly called hookify the "lone adopter"; (b) agent frontmatter has **strict regex/enum validation** for `name`, `model`, `color` (color is required); (c) PreToolUse hooks can **rewrite tool input** via `permissionDecision`+`updatedInput`, not just allow/deny; (d) MCP remote transports support a **`headersHelper`** script for dynamic JWT/HMAC/short-lived auth; (e) skill frontmatter `name:` convention is **Title Case as the majority** (8/10), reversing round 1's claim; (f) four unresolved tensions (TENSION-01..04) and two never-documented gaps (name collision resolution, regex dialect name) remain after three rounds with no new source evidence.

---

## 2. System Architecture

### 2.1 Canonical plugin layout

From `plugin-dev/skills/plugin-structure/SKILL.md:22-37` and `references/manifest-reference.md:5-9`:

```
<plugin-name>/
├── .claude-plugin/plugin.json   REQUIRED, exact path. Absent => plugin not loadable.
├── commands/*.md                default ./commands (override in plugin.json, SUPPLEMENTS)
├── agents/*.md                  default ./agents
├── skills/<name>/SKILL.md       REQUIRED per skill; optional references/, examples/, scripts/
├── hooks/hooks.json             default ./hooks/hooks.json; inline object OR file path
├── .mcp.json                    default ./.mcp.json; inline object OR file path
└── README.md                    prose only
```

Custom path fields in `plugin.json` **supplement** the defaults, they do not replace them (`manifest-reference.md:365-371`). Nested subdirectories under `commands/` are **not** auto-discovered (`component-patterns.md:111-121`). All path values must be `./`-prefixed, forward-slash, with no `../`.

### 2.2 Component lifecycle — Discovery vs Activation

The single most load-bearing fact in the ecosystem is the two-phase lifecycle from `plugin-structure/references/component-patterns.md:7-26`:

**Phase 1 — Discovery** (executes ONCE at Claude Code session init):
1. Scan enabled plugins.
2. Read each `plugin.json`.
3. Enumerate component directories using the defaults-plus-supplements rule.
4. Parse frontmatter on commands, agents, skills.
5. Register commands, agents, skills, hooks into in-memory registries.
6. **Start stdio MCP servers eagerly** (process lifetime = session lifetime).
7. Merge plugin `hooks/hooks.json` (wrapper format) with user `~/.claude/settings.json` (direct format) into one combined event→handler-list map.

**Phase 2 — Activation** (continuous during session):
- Commands fire on `/slash-command` invocation.
- Agents fire on `Task` tool dispatch.
- Skills fire on description match against user intent.
- Hooks fire when their subscribed event fires.
- MCP tools fire on tool invocation.
- **No re-scan. No hot reload.** Edit any file = restart required.

### 2.3 Plugin composition model

Six architectural patterns (A–F from Phase A) plus one added in Phase B (G) plus one refinement (H). Each is a distinct, identifiable shape a plugin can take:

- **A. Command-and-agents orchestration** — `feature-dev`, `code-review`, `pr-review-toolkit`, `agent-sdk-dev`. A single command spawns one or more agents via the `Task` tool; agents carry `model`/`tools`/`color` frontmatter.
- **B. Skill-only** — `claude-opus-4-5-migration`, `frontend-design`. Just `skills/<name>/SKILL.md`, auto-activated by description match.
- **C. Hook-as-output-style** — `explanatory-output-style`, `learning-output-style`. SessionStart hooks emit `{hookSpecificOutput:{additionalContext:"..."}}`. The deprecated "output style" concept is reimplemented as a SessionStart hook.
- **D. Security/policy hook** — `security-guidance`. PreToolUse matches `Edit|Write|MultiEdit`, exits 2 to block. With `permissionDecision`+`updatedInput` (round 2), this pattern can now *rewrite* the invocation; no in-repo plugin exercises the rewrite form yet.
- **E. Runtime Python package** — `hookify` is the only example. Entry scripts in `hooks/` bootstrap `sys.path` from `${CLAUDE_PLUGIN_ROOT}` then import `hookify.core.*`. Only `hookify/core/` is real code (610 LOC); `hookify/matchers/` and `hookify/utils/` are empty `__init__.py` placeholders (corrected in Phase B round 1).
- **F. Loop interception** — `ralph-wiggum`. Stop hook continues iterating until a cancel file exists. Uses Plugin Settings as cross-iteration state store.
- **G. Plugin Settings overlay** (round 1, new) — `.claude/<plugin-name>.local.md` (YAML frontmatter + markdown body) is the **only** runtime-adjustable surface. Hookify adopts it for config; ralph-wiggum adopts it for stateful iteration tracking. Read at runtime by hooks (`sed`/`grep`), commands (`Read` tool), and agents (instructions). Gitignored, `chmod 600`. Source: `plugin-dev/skills/plugin-settings/SKILL.md:1-60`.
- **H. MCP transport taxonomy** — plugins declare MCP servers via inline `plugin.json.mcpServers` or a separate `.mcp.json`. Four transports: `stdio`, `sse`, `http`, `ws`. stdio is eager; remote transports may be lazy. Authentication: env vars (stdio), OAuth-automatic (sse/http), static `headers` with `${ENV_VAR}` substitution, or dynamic `headersHelper` script (round 3).

### 2.4 Hook invocation data flow

Updated for parallel execution, dual handler types, and permission-decision envelopes:

```
Event fires (e.g. PreToolUse)
  |
  v
Runtime consults merged event->handler map built at Discovery
  (plugin hooks + user-settings hooks, both flattened)
  |
  v
For each entry whose matcher regex matches tool_name
  (case-sensitive; exact, pipe-alt "A|B", wildcard "*", or ".*" regex;
   omitting matcher = match-all per docs -- see TENSION-03):
  |
  +-- type:"command"  -> spawn subprocess
  |                      stdin = JSON envelope:
  |                        { session_id, transcript_path, cwd,
  |                          permission_mode, hook_event_name,
  |                          ...event-specifics }
  |                      env += CLAUDE_PROJECT_DIR, CLAUDE_PLUGIN_ROOT,
  |                             (SessionStart only) CLAUDE_ENV_FILE,
  |                             CLAUDE_CODE_REMOTE
  |                      default timeout 60s
  |
  +-- type:"prompt"   -> LLM-evaluated prompt string
                         (supported ONLY on Stop, SubagentStop,
                          UserPromptSubmit, PreToolUse)
                         default timeout 30s
  |
  v
ALL matching handlers run IN PARALLEL.
No cross-handler state. Non-deterministic order.
  |
  v
Per handler, stdout parsed as JSON envelope:
  common:       { continue?, suppressOutput?, systemMessage? }
  PreToolUse:   { hookSpecificOutput: { permissionDecision: "allow"|"deny"|"ask",
                                         updatedInput: {...} }, systemMessage }
  Stop/SubagentStop: { decision: "approve"|"block", reason, systemMessage }
  SessionStart: { hookSpecificOutput: { hookEventName: "SessionStart",
                                         additionalContext: "..." } }
Exit codes: 0 success, 2 blocking (stderr -> Claude),
            other = non-blocking error.
```

Key consequence: because Discovery owns the plugin-hook + user-hook merge, **hot-reload is fundamentally impossible** without re-running Discovery. This is the structural reason for Plugin Settings: it is the sole architectural escape hatch from the immutable Discovery model.

### 2.5 Configuration layering

From lowest (immutable, session-lifetime) to highest (dynamic, per-invocation):

1. `plugin.json` defaults — immutable, Discovery-time.
2. `plugin.json` path overrides — immutable, Discovery-time (supplement semantics).
3. User `~/.claude/settings.json` hooks — immutable, Discovery-time, merged with plugin hooks.
4. Per-command frontmatter — fixed at parse time.
5. Per-agent frontmatter — fixed at parse time, regex/enum validated.
6. **Plugin Settings `.claude/<plugin-name>.local.md`** — runtime, per-invocation, user-editable with no restart required.

### 2.6 Deployment topology

None in the traditional sense. No daemon, no server, no package manager. Plugins are static file trees. Discovery is one-shot at CLI init. All runtime dynamism comes from (a) parallel hook races, (b) MCP remote transports, (c) Plugin Settings overlay reads. Restart Claude Code = full reset.

---

## 3. Domain Model (Concepts)

### 3.1 Core concept catalog

| Concept | Definition | Key fields |
|---|---|---|
| **Plugin** | Directory containing `.claude-plugin/plugin.json` | `name` (kebab, required), `version`, `description`, `author`, `homepage`, `repository`, `license`, `keywords`, `commands`, `agents`, `hooks`, `mcpServers` |
| **Command** | `commands/*.md` with YAML frontmatter, invoked via `/slash-command` | `description`, `allowed-tools` (String OR Array), `model`, `argument-hint`, `disable-model-invocation` |
| **Command scope** | Discovery location determines label in `/help` | project (`.claude/commands/`), personal (`~/.claude/commands/`), plugin (`<plugin>/commands/`) |
| **Agent** | `agents/*.md` subagent with own prompt/model/tools, invoked via `Task` tool | `name` (req, 3-50 chars, regex), `description` (req, 10-5000 chars, with `<example>` dialogs), `model` (req, enum), `color` (req, enum), `tools` (opt, array or comma-string — TENSION-04); system prompt 20-10000 chars |
| **Skill** | `skills/<name>/SKILL.md`, auto-activated by description match | `name`, `description` (trigger text), `version`, optional `license` |
| **Hook** | Entry in `hooks.json` (or inline `plugin.json.hooks`) mapping event → matcher → handler | event name, optional `matcher` (TENSION-03), `hooks[].type` in `{"command","prompt"}`, `hooks[].command` or `hooks[].prompt`, optional `timeout` |
| **Matcher** | Case-sensitive regex filter on tool name; flavor unspecified | e.g. `"Write\|Edit"`, `"*"`, `"mcp__.*__delete.*"` |
| **MCP server** | External tool provider via `.mcp.json` or inline `plugin.json.mcpServers` | stdio: `command`/`args`/`env`; remote: `type` + `url` + optional `headers` / `headersHelper` |
| **MCP `headersHelper`** | Script path producing dynamic auth headers as JSON on stdout | JWT, HMAC, short-lived tokens; composable with static `headers` |
| **MCP tool name** | Auto-generated runtime identifier | `mcp__plugin_<plugin-name>_<server-name>__<tool-name>` |
| **Plugin Settings** | Per-project plugin config at `.claude/<plugin-name>.local.md` — YAML frontmatter + markdown body; user-managed, gitignored, `chmod 600` | `enabled`, arbitrary keys |
| **Output style** | NOT a first-class construct — emulated via SessionStart + `additionalContext` | — |
| **Progressive disclosure** | Convention: `SKILL.md` lean (~1,500–2,000 words), heavy docs in `references/`, `examples/` | — |
| **`${CLAUDE_PLUGIN_ROOT}`** | Env var pointing at plugin dir | Mandatory for portable intra-plugin paths |

### 3.2 Lifecycle events (9 total, 5 exercised)

From `plugin-dev/skills/plugin-structure/SKILL.md:229`:

| Event | Exercised in corpus | Where |
|---|---|---|
| `PreToolUse` | yes | `security-guidance`, `hookify`, example `bash_command_validator_example.py` |
| `PostToolUse` | yes | `hookify` |
| `UserPromptSubmit` | yes | `hookify` |
| `Stop` | yes | `hookify`, `ralph-wiggum` |
| `SessionStart` | yes | `explanatory-output-style`, `learning-output-style` |
| `SubagentStop` | **no** | — |
| `SessionEnd` | **no** | — |
| `PreCompact` | **no** | — |
| `Notification` | **no** | — |

Four of nine events have no executable example in any of the 13 plugins. This is an ingest-capped coverage gap, not necessarily a spec gap.

### 3.3 Relationships

```
Plugin --1:1-- plugin.json
       |--0..N-- Command --- (prompt may invoke) ----> Agent via Task tool
       |                 --- (prompt may load)    ----> Skill via description match
       |--0..N-- Agent  --- declares allowed Tools, selects model
       |--0..N-- Skill  --- auto-triggered by description match
       |--0..N-- Hook   --- subscribes to Event + Matcher -> {command|prompt} handler
       |--0..N-- MCP server --- exposes tools as mcp__plugin_<p>_<s>__<t>
       +--0..1-- PluginSettings (.claude/<name>.local.md, project-scope, user-managed)
```

**Critical disjointness**: Agents and Skills are disjoint activation paths. Agents are *explicit* `Task` spawns from a command or another agent. Skills are *runtime auto-activation* from description match. A system that needs "deterministic, addressable subroutine" picks Agent; one that needs "when the user mentions X, load guidance" picks Skill.

---

## 4. Behavioral Contracts (Catalog)

40+ draft contracts organized by subsystem. All are HIGH confidence unless marked otherwise.

### 4.1 Manifest

| ID | Confidence | Contract | Source |
|---|---|---|---|
| BC-DRAFT-M01 | HIGH | Runtime loads plugin only if `./.claude-plugin/plugin.json` exists exactly | `plugin-structure/SKILL.md:22-37` |
| BC-DRAFT-M02 | HIGH | `name` matches `^[a-z][a-z0-9]*(-[a-z0-9]+)*$` | `manifest-reference.md` |
| BC-DRAFT-M03 | HIGH | Custom component paths **supplement**, not replace, defaults | `manifest-reference.md:365-371` |
| BC-DRAFT-M04 | HIGH | Paths must be `./`-prefixed, forward-slash, no `../` | `manifest-reference.md` |
| BC-DRAFT-M05 | HIGH | `hooks` and `mcpServers` fields accept either a relative file path OR an inline object | `manifest-reference.md:273-325` |
| BC-DRAFT-M06 | HIGH | Defaults: `commands: ./commands`, `agents: ./agents`, `hooks: ./hooks/hooks.json`, `mcpServers: ./.mcp.json` | `manifest-reference.md:214,248,263,302` |
| BC-DRAFT-M07 | MEDIUM | Nested subdirs under `commands/` are NOT auto-discovered | `component-patterns.md:111-121` |

### 4.2 Command

| ID | Confidence | Contract | Source |
|---|---|---|---|
| BC-DRAFT-C01 | HIGH | Commands auto-discover from `commands/*.md` | `command-development/SKILL.md` |
| BC-DRAFT-C02 | HIGH | `allowed-tools` supports subcommand globs for Bash and MCP tool names with wildcards; field is **String OR Array** | `command-development/SKILL.md:132-143` |
| BC-DRAFT-C03 | HIGH | Backtick-bang expressions execute at invocation; `$1..$N`, `$ARGUMENTS` expand | `command-development/SKILL.md:200-263` |
| BC-DRAFT-C04 | MEDIUM | `model:` on a command sets model for that command | `command-development/SKILL.md` |
| BC-DRAFT-C05 | MEDIUM | `argument-hint` is display-only | `command-development/SKILL.md` |
| BC-DRAFT-C06 | HIGH | `disable-model-invocation: true` prevents programmatic `SlashCommand` invocation | `command-development/SKILL.md:182-193` |
| BC-DRAFT-C07 | HIGH | Command discovery has three scopes (project/personal/plugin) with distinct `/help` labels; subdirs append to label, e.g. `(project:ci)` | `command-development/SKILL.md:55-72` |

### 4.3 Agent

| ID | Confidence | Contract | Source |
|---|---|---|---|
| BC-DRAFT-A01 | HIGH | Agents auto-discover from `agents/*.md` | `agent-development/SKILL.md` |
| BC-DRAFT-A02 | HIGH | Agents may declare a `tools:` whitelist — format ambiguous (TENSION-04) | `agent-development/SKILL.md:149`, `pr-review-toolkit/agents/silent-failure-hunter.md:4` |
| BC-DRAFT-A03 | HIGH | `model: inherit` delegates to parent session model | `agent-development/SKILL.md` |
| BC-DRAFT-A04 | MEDIUM | Description should include 2-3 `<example>` dialogue blocks | `agent-development/SKILL.md` |
| BC-DRAFT-A05 | HIGH | `name` must be 3-50 chars, `[a-z0-9-]`, start/end alphanumeric, no underscores. `description` 10-5000 chars. System prompt body 20-10000 chars | `agent-development/SKILL.md:64-80,264-273` (prose; regex `^[a-z0-9]([a-z0-9-]*[a-z0-9])?$` is a faithful reconstruction, not a verbatim extract) |
| BC-DRAFT-A06 | HIGH | `model` enum `{inherit,sonnet,opus,haiku}`. `color` **required**, enum `{blue,cyan,green,yellow,magenta,red}` | `agent-development/SKILL.md:128-141,351-357` |

### 4.4 Skill

| ID | Confidence | Contract | Source |
|---|---|---|---|
| BC-DRAFT-S01 | HIGH | Each skill dir needs `SKILL.md`; `README.md` does not count | `skill-development/SKILL.md` |
| BC-DRAFT-S02 | HIGH | Skills auto-activate by matching user intent against `description:` | `skill-development/SKILL.md` |
| BC-DRAFT-S03 | HIGH | Description should enumerate triggering phrases in **third person** — "This skill should be used when..." | `skill-development/SKILL.md:162-182` |
| BC-DRAFT-S04 | HIGH | Progressive disclosure via `references/` and `examples/`. Three-level loading: metadata always in context, SKILL.md body on trigger, bundled resources on demand | `skill-development/SKILL.md:77-86` |

### 4.5 Hook

| ID | Confidence | Contract | Source |
|---|---|---|---|
| BC-DRAFT-H01 | HIGH | Plugin `hooks.json` format = `{description?, hooks: {<EventName>: [{matcher?, hooks: [<handler>]}]}}` | `hook-development/SKILL.md:60-119` |
| BC-DRAFT-H01b | HIGH | User-settings `~/.claude/settings.json` omits the wrapper — events at top level | `examples/settings/*.json` |
| BC-DRAFT-H02 | HIGH (narrowed) | Omitting `matcher` = match-all per prose docs. **TENSION-03**: `validate-hook-schema.sh:70-75` errors when matcher missing | `hook-development/SKILL.md` |
| BC-DRAFT-H03 | HIGH | PreToolUse hooks can block via exit 2 + stderr | `hook-development/SKILL.md` |
| BC-DRAFT-H04 | HIGH | Stdin JSON common fields: `{session_id, transcript_path, cwd, permission_mode, hook_event_name}` + event-specifics | `hook-development/SKILL.md:302-319` |
| BC-DRAFT-H05 | HIGH | SessionStart hooks inject prompt text via `{hookSpecificOutput: {hookEventName: "SessionStart", additionalContext: "..."}}` | `hook-development/SKILL.md` |
| BC-DRAFT-H06 | HIGH | Hook commands must use `${CLAUDE_PLUGIN_ROOT}`; hardcoded absolute paths warned | `validate-hook-schema.sh:111-114` |
| BC-DRAFT-H07 | HIGH | Hook scripts should fail-open on import/parse errors | `hookify/hooks/pretooluse.py:29-32` |
| BC-DRAFT-H08 | HIGH | `type:"prompt"` supported ONLY on `Stop`, `SubagentStop`, `UserPromptSubmit`, `PreToolUse` | `hook-development/SKILL.md:22-34` |
| BC-DRAFT-H09 | HIGH | Default `timeout`: command 60s, prompt 30s. Validator warns outside `[5, 600]` | `validate-hook-schema.sh:131-142` |
| BC-DRAFT-H10 | HIGH | Matcher is **case-sensitive** regex: exact, pipe-alt, wildcard, full regex. Dialect name unstated | `hook-development/SKILL.md:386-425` |
| BC-DRAFT-H11 | HIGH | PreToolUse hooks emit `{hookSpecificOutput: {permissionDecision: "allow"|"deny"|"ask", updatedInput: {...}}, systemMessage}` — can **rewrite tool input** | `hook-development/SKILL.md:144-153` |
| BC-DRAFT-H12 | HIGH | Stop/SubagentStop hooks emit `{decision: "approve"|"block", reason, systemMessage}` | `hook-development/SKILL.md:202-209` |
| BC-DRAFT-H13 | HIGH | All matching hooks for an event run **in parallel**, non-deterministic order, no cross-hook state | `hook-development/SKILL.md:496-518` |
| BC-DRAFT-H14 | HIGH | Plugin hooks and user-settings hooks **merge** at Discovery | `hook-development/SKILL.md:60-119` |
| BC-DRAFT-H15 | HIGH | Hooks load at session start — **no hot reload** | `plugin-settings/examples/example-settings.md:159` |
| BC-DRAFT-H16 | HIGH | SessionStart hooks can persist env vars via `$CLAUDE_ENV_FILE` | `hook-development/SKILL.md:322-329` |
| BC-DRAFT-H17 | HIGH | Exit codes: 0 success, 2 blocking, other non-blocking error | `hook-development/SKILL.md` |
| BC-DRAFT-H18 | HIGH | Env vars: `CLAUDE_PROJECT_DIR`, `CLAUDE_PLUGIN_ROOT`, `CLAUDE_ENV_FILE` (SessionStart only), `CLAUDE_CODE_REMOTE` | `hook-development/SKILL.md:322-329` |

### 4.6 MCP

| ID | Confidence | Contract | Source |
|---|---|---|---|
| BC-DRAFT-MCP01 | HIGH | Inline in `plugin.json.mcpServers` uses `{mcpServers:{...}}` wrapper; standalone `.mcp.json` example omits wrapper — **TENSION-01** | `mcp-integration/SKILL.md:27-37` |
| BC-DRAFT-MCP02 | HIGH | Four MCP transports: `stdio`, `sse`, `http`, `ws` | `mcp-integration/SKILL.md:65-165` |
| BC-DRAFT-MCP03 | HIGH | Tool names prefixed `mcp__plugin_<plugin-name>_<server-name>__<tool-name>` | `mcp-integration/SKILL.md:190-200` |
| BC-DRAFT-MCP04 | MEDIUM | stdio MCP servers are **eager** — process lifetime = session lifetime. Remote may be lazy (TENSION-02 narrowed) | `mcp-integration/references/server-types.md:38-44` |
| BC-DRAFT-MCP05 | HIGH | Env var substitution supports `${CLAUDE_PLUGIN_ROOT}` and arbitrary user-shell vars in `env` / `headers` | `mcp-integration/SKILL.md` |
| BC-DRAFT-MCP06 | HIGH | `headersHelper` = executable script emitting JSON headers on stdout at each request; JWT/HMAC/time-based; composable with static `headers`; mTLS not directly supported | `mcp-integration/references/authentication.md:233-258` |

### 4.7 Plugin Settings

| ID | Confidence | Contract | Source |
|---|---|---|---|
| BC-DRAFT-PS01 | HIGH | Per-project config at `.claude/<plugin-name>.local.md`. YAML frontmatter + markdown body. User-managed, gitignored (`.claude/*.local.md` and `.local.json`), `chmod 600` recommended | `plugin-settings/SKILL.md:1-60` |
| BC-DRAFT-PS02 | HIGH | Hooks, commands, and agents may read at runtime to toggle behavior. **Only** runtime-adjustable config path | `plugin-settings/SKILL.md:1-60` |

---

## 5. Non-Functional Requirements

### 5.1 Security

`security-guidance/hooks/security_reminder_hook.py:31-126` hardcodes **exactly 9** detection patterns on `Edit|Write|MultiEdit`:

1. `github_actions_workflow` — `${{ github.event.* }}` injection (CWE-94 class)
2. `child_process_exec` — `child_process.exec*` (CWE-78)
3. `new_function_injection` — `new Function` (CWE-94)
4. `eval_injection` — `eval(` (CWE-94)
5. `react_dangerously_set_html` — `dangerouslySetInnerHTML` (CWE-79)
6. `document_write_xss` — `document.write` (CWE-79)
7. `innerHTML_xss` — `.innerHTML =` (CWE-79)
8. `pickle_deserialization` — bare substring `pickle` (CWE-502; over-broad, matches `pickleball`)
9. `os_system_injection` — `os.system` (CWE-78)

CWE labels absent from reminder messages. First-match short-circuit. Kill switch: `ENABLE_SECURITY_REMINDER=0`. Session-scoped dedup with probabilistic GC. **`commit-commands/commands/commit.md:2` is the only example in the corpus of `allowed-tools` subcommand globbing** — canonical compile-time blast-radius control.

### 5.2 Validator-enforced NFRs (the de facto spec)

`plugin-dev/skills/hook-development/scripts/validate-hook-schema.sh`:

| ID | Rule | Line range |
|---|---|---|
| NFR-V01 | Event name ∈ 9 canonical events; unknown = warning | `:41-55` |
| NFR-V02 | `matcher` field required (error). **TENSION-03** | `:70-75` |
| NFR-V03 | `hooks` array required | `:77-83` |
| NFR-V04 | `type` ∈ {`command`, `prompt`}; enum | `:91-101` |
| NFR-V05 | Command hooks must declare `command` | `:104-108` |
| NFR-V06 | Commands starting with `/` lacking `${CLAUDE_PLUGIN_ROOT}` warn — **canonical portability rule** | `:110-114` |
| NFR-V07 | Prompt hooks must declare `prompt` | `:116-121` |
| NFR-V08 | Prompt hooks only on `Stop`, `SubagentStop`, `UserPromptSubmit`, `PreToolUse` | `:123-127` |
| NFR-V09 | Timeout integer; advisory `[5,600]`; outside warns | `:131-142` |
| NFR-V10 | Errors exit 1, warnings exit 0 | `:150-158` |

### 5.3 Performance — the timeout spectrum

| Plugin | Timeout | Notes |
|---|---|---|
| `hookify` (all 4 events) | **10s** | Tightest; relies on fail-open try/except |
| `security-guidance` | **60s** (unset, inherits default) | Round 1 corrected Phase A's "no effective timeout" |
| `plugin-dev` examples | 60s command / 30s prompt (unset) | |
| Validator advisory | `[5, 600]` | |

**Baseline NFR**: 60s command, 30s prompt. No plugin exceeds default.

### 5.4 Observability

**`systemMessage` in hook response JSON is the ONLY canonical observability channel.** No structured logger, no trace IDs, no OTel. **Anti-pattern**: `security-guidance` writes `/tmp/security-warnings-log.txt` (hardcoded, non-portable, racy). No logging convention exists.

### 5.5 Reliability / fail modes

Deliberate but unpoliced split: `hookify` fails **open** on import errors; `security-guidance` fails **closed** on pattern match, **open** on JSON parse errors. The correct policy — **fail-closed on policy violation, fail-open on infrastructure error** — is implicit but never documented. Validator enforces shape (NFR-V01..V10) but not behavior.

### 5.6 Portability

`${CLAUDE_PLUGIN_ROOT}` is the sole portability mechanism for executable paths (enforced advisory by NFR-V06). All 13 plugins comply. For data paths, Plugin Settings is the recommended portable location — but only for config, not logs.

---

## 6. Conventions

### 6.1 plugin.json convention

Minimal shape universal:

```json
{"name":"kebab","version":"0.1.0","description":"50-200 chars","author":{"name":"...","email":"..."}}
```

No plugin uses `homepage`, `repository`, `license`, `keywords`, or path overrides. **Anomaly**: `plugin-dev/` has no `plugin.json`.

### 6.2 Command frontmatter

```yaml
---
description: <verb-first, ~60 chars>
allowed-tools: <Tool list | Bash(subcmd:*), ...>   # String OR Array
argument-hint: <optional>
model: <sonnet | inherit>
disable-model-invocation: <bool>
---
```

**Round 2 retraction**: `plugin-dev/skills/command-development/references/` does NOT exist; round 1 claim of "7 files" retracted.

### 6.3 Agent frontmatter

```yaml
---
name: kebab-case              # 3-50 chars, [a-z0-9-], start/end alnum, no underscores
description: |
  prose + 2-3 <example>...</example> dialogs   # 10-5000 chars
tools: Glob, Grep, LS, Read   # TENSION-04 — array per docs, comma-string in examples
model: inherit                # enum: {inherit, sonnet, opus, haiku}
color: green                  # REQUIRED enum: {blue, cyan, green, yellow, magenta, red}
---
<system prompt body: 20-10000 chars>
```

- **CONV-A01**: `name` regex `^[a-z0-9]([a-z0-9-]*[a-z0-9])?$`
- **CONV-A02**: `description` 10-5000 chars, system prompt 20-10000 chars
- **CONV-A03**: `model` enum
- **CONV-A04**: `color` required, enum

### 6.4 Skill frontmatter

```yaml
---
name: <Title Case is the majority (8/10)>
description: <third-person prose enumerating trigger phrases>
version: 0.1.0
---
```

- **CONV-S01**: Description in **third person**.
- **CONV-S02 (round 2 REVERSED)**: Title Case is the **majority** (8/10), kebab is the outlier. Reversed vs round 1.
- **CONV-S03 (round 2)**: Body tone must be **imperative/infinitive**, NOT second-person (`skill-development/SKILL.md:160`).
- **CONV-S04 (round 2)**: Size budget **1,500-2,000 words** for SKILL.md body (`skill-development/SKILL.md:190`). Detail offloads to `references/`.

### 6.5 Naming — the four-way regime

- **kebab-case**: plugin name, command file, agent file, skill directory, script file
- **Title Case**: skill `name:` frontmatter (majority)
- **eventname-no-separator**: hook entry scripts (`pretooluse.py`)
- **snake_case**: Python internals (`hookify/core/config_loader.py`)

Hook handler **location**: `hooks/` (most) vs `hooks-handlers/` (both output-style plugins). No canonical.

### 6.6 Positive idioms

- Backtick-bang context injection (`commit.md:5-9`)
- Commands invoke same-plugin skills via `Skill` tool (`hookify.md:9`)
- Agent description embeds `<example>` blocks
- `${CLAUDE_PLUGIN_ROOT}` everywhere
- Minimal manifests, rely on defaults
- `type:"prompt"` hooks — lightweight LLM-driven guardrails
- `permissionDecision` rewrite pattern — PreToolUse can rewrite tool input
- `headersHelper` auth script for MCP
- Plugin Settings as runtime config — **2 adopters** (hookify config, ralph-wiggum state)
- `.claude/*.local.md` + `.local.json` gitignore convention
- Third-person skill descriptions, imperative body tone, 1,500-2,000 word budget

### 6.7 Anti-patterns and latent issues

- `plugin-dev/` has no `plugin.json`
- `/tmp/security-warnings-log.txt` hardcoded
- Two hook handler directory conventions
- Agent `tools:` comma-string (TENSION-04)
- Bare `pickle` substring match — over-broad
- CWE labels missing from security reminders
- No canonical logging path
- Fail-mode policy unpoliced
- No hot-reload confirmed at `plugin-settings/examples/example-settings.md:159`
- **No tests anywhere** — exhaustively confirmed
- **(round 2 latent) Plugin Settings dual-purpose conflation** — same envelope for user config AND machine state; user editing a state file silently corrupts the plugin.

---

## 7. Tensions and Gaps

### 7.1 Tensions

| ID | Description | Status |
|---|---|---|
| **TENSION-01** | `.mcp.json` root structure (wrapper vs direct). No concrete files in corpus | **Unresolved** — true source-level unknown |
| **TENSION-02** | stdio eager vs lazy | **Narrowed**: stdio confirmed eager; remote transports may be lazy but not per-transport stated |
| **TENSION-03** | `matcher` required by validator, optional per docs and hookify | **Unresolved** — validator and docs disagree |
| **TENSION-04** | Agent `tools:` array vs comma-string | **Unresolved** — both forms in the wild |

### 7.2 Gaps

| Gap | Type |
|---|---|
| Name collision resolution across plugins | **True unknown** |
| Matcher regex dialect (ECMA / PCRE / RE2 / Go) | **True unknown** |
| No tests anywhere in corpus | Ingest-cap |
| 4 of 9 lifecycle events unexercised | Ingest-cap |
| No concrete `.mcp.json` files in 13 plugins | Ingest-cap |
| No versioning / migration contract | **True unknown** |
| Fail-mode policy unpoliced | **True unknown** |
| No canonical logging path | **True unknown** |

---

## 8. Lessons for vsdd-factory

**Highest-value section.** Organized P0/P1/P2/P3 with (a) what vsdd-factory does today, (b) what the reference does, (c) the gap, (d) specific actions.

### P0 — Must fix before next release

**L-P0-01. Agent frontmatter does not validate `name`, `model`, or `color`.**
- **Today**: Agents declare frontmatter informally; no linter asserts Anthropic constraints.
- **Reference**: BC-DRAFT-A05/A06 — `name` regex 3-50 chars, `model` enum, `color` **required** enum. `validate-agent.sh` enforces.
- **Gap**: Silent drift could break on Claude Code updates.
- **Actions**:
  1. Audit all `agents/*.md` against BC-DRAFT-A05/A06.
  2. Add a CI lint mirroring `validate-agent.sh`.
  3. Make it part of `just check` / lefthook.

**L-P0-02. TENSION-04 — agent `tools:` comma-string vs JSON array.**
- **Today**: Likely inconsistent.
- **Reference**: Both forms work; JSON array is documented.
- **Actions**: Standardize on JSON array. Add lint flagging comma-string.

**L-P0-03. Hooks relying on hardcoded paths instead of `${CLAUDE_PLUGIN_ROOT}`.**
- **Today**: Any hook using absolute/relative-from-cwd paths.
- **Reference**: NFR-V06 + BC-DRAFT-H06 mandate `${CLAUDE_PLUGIN_ROOT}`.
- **Actions**: Grep all `hooks.json` and `hooks/` dirs; replace with `${CLAUDE_PLUGIN_ROOT}`. Adopt `validate-hook-schema.sh` as lint.

**L-P0-04. `plugin.json` present on every loadable factory plugin.**
- **Today**: May mirror `plugin-dev/`'s "reference bundle, no manifest" pattern.
- **Reference**: Absence = not loadable (BC-DRAFT-M01).
- **Actions**: Inventory all plugins; flag missing `plugin.json`. Document loadable-vs-reference distinction.

### P1 — Should adopt

**L-P1-01. Adopt Plugin Settings for `/vsdd-factory:activate` and runtime toggles.**
- **Today**: No runtime-adjustable surface. Every config change = restart.
- **Reference**: Pattern G — `.claude/<plugin-name>.local.md` is the ONLY way to change plugin behavior without restart. hookify (config) + ralph-wiggum (state).
- **Actions**:
  1. Create `.claude/vsdd-factory.local.md` template with `activation_enabled`, `current_phase`, `current_wave`, `paused_hooks[]`, `dry_run`, `log_level`.
  2. Update `.gitignore` to include `.claude/*.local.md` **and** `.claude/*.local.json`.
  3. Document `chmod 600`.
  4. **Do not conflate config and state** (see L-P3-03).
  5. Teach factory hooks and skills to read it at runtime.

**L-P1-02. Rewrite spec-steward as a `permissionDecision`+`updatedInput` hook.**
- **Today**: Spec-steward blocks edits to green VPs via exit 2 (deny only).
- **Reference**: BC-DRAFT-H11 — PreToolUse can emit `{permissionDecision:"allow"|"deny"|"ask", updatedInput:{...}}` to **rewrite** the call.
- **Actions**:
  1. Upgrade spec-steward to emit structured envelopes.
  2. Add "supersede, don't mutate" rewrite path for green VPs.
  3. Add `permissionDecision: "ask"` for borderline cases.

**L-P1-03. Adopt `type:"prompt"` hooks for LLM-driven guardrails.**
- **Today**: All hooks are `type:"command"` subprocess scripts.
- **Reference**: BC-DRAFT-H08 — prompt hooks are LLM-evaluated, supported on Stop/SubagentStop/UserPromptSubmit/PreToolUse, 30s default.
- **Actions**: Identify the 3-5 factory hooks that are "ask Claude to check X"; convert to `type:"prompt"`.

**L-P1-04. Third-person skill descriptions with enumerated trigger phrases.**
- **Today**: Likely second-person imperative.
- **Reference**: CONV-S01 + CONV-S03 — frontmatter `description` third person enumerating triggers; body tone imperative/infinitive.
- **Actions**:
  1. Rewrite every skill `description:` in third person.
  2. Rewrite bodies in imperative form.
  3. Enforce via lint.

**L-P1-05. Adopt the 1,500-2,000 word SKILL.md budget with `references/` offload.**
- **Today**: Monolithic skill files likely.
- **Reference**: CONV-S04 + BC-DRAFT-S04 — progressive disclosure three-level loading.
- **Actions**: Audit SKILL.md word counts; split overflows to `references/`.

**L-P1-06. Adopt validator-inspired lint rules (NFR-V01..V10).**
- **Today**: No hooks-schema lint.
- **Reference**: `validate-hook-schema.sh` enforces event enum, type enum, prompt-event scoping, portability warning, timeout bounds.
- **Actions**:
  1. Copy `validate-hook-schema.sh` into `.claude/rules/` or `scripts/`.
  2. Add to `just check`.
  3. **Decide TENSION-03**: either require `matcher` or permit omission.

### P2 — Worth considering

**L-P2-01. Title Case skill names.** CONV-S02 reversal — Title Case is the majority (8/10). Low priority, cosmetic, improves consistency.

**L-P2-02. Mount `commit.md`-style `allowed-tools` subcommand globs.** Canonical blast-radius control.

**L-P2-03. Formalize fail-open vs fail-closed policy.** Add to SOUL.md: "fail-closed on policy violation, fail-open on infrastructure error". Lint to enforce.

**L-P2-04. Dual-format hook authoring guide (plugin wrapper vs user direct).** Document in factory-protocol.md.

**L-P2-05. `headersHelper` pattern for future MCP servers with short-lived tokens.** Future-proofing.

### P3 — Document as known divergences

**L-P3-01. `plugin-dev`-style "reference bundle without plugin.json".** Useful for internal spec bundles. Document as intentional.

**L-P3-02. Reject `/tmp/*.log` anti-pattern.** Choose canonical logs path like `${CLAUDE_PROJECT_DIR}/.claude/logs/<plugin>.log` or `.factory/logs/<plugin>/`. Add to SOUL.md / bash.md.

**L-P3-03. Plugin Settings dual-purpose conflation — one semantic per file.**
- **Reference anti-pattern**: hookify uses `.claude/hookify.local.md` as user-editable config; ralph-wiggum uses `.claude/ralph-loop.local.md` as machine-written state. A user editing a state file silently corrupts ralph-wiggum.
- **vsdd-factory divergence**: Explicitly pick one semantic per `.local.md` file. Preferred split:
  - `.claude/vsdd-factory.local.md` = user config only
  - `.claude/vsdd-factory-state.local.json` = machine state (never intended for human editing)
- **Actions**:
  1. Document the split in factory-protocol.md.
  2. Warn users against editing `*-state.local.json`.
  3. Add header to machine-written files: `# DO NOT EDIT — managed by vsdd-factory`.

**L-P3-04. No tests anywhere in the reference corpus.** Corverax SOUL.md #4 (silent failures) mandates tests. Document the divergence.

**L-P3-05. Hook handler directory — pick `hooks/` and stick with it.** Majority, matches default path. Lint against `hooks-handlers/`.

### Summary of the top 20 actionable findings

| # | ID | Title | Priority |
|---|---|---|---|
| 1 | L-P0-01 | Validate agent frontmatter per BC-A05/A06 | P0 |
| 2 | L-P0-02 | Standardize agent `tools:` on JSON array | P0 |
| 3 | L-P0-03 | Enforce `${CLAUDE_PLUGIN_ROOT}` in hook commands | P0 |
| 4 | L-P0-04 | `plugin.json` present on every loadable plugin | P0 |
| 5 | L-P1-01 | Adopt Plugin Settings for activation/runtime toggles | P1 |
| 6 | L-P1-02 | Rewrite spec-steward as `permissionDecision` hook | P1 |
| 7 | L-P1-03 | Adopt `type:"prompt"` hooks for LLM guardrails | P1 |
| 8 | L-P1-04 | Third-person skill descriptions + imperative body | P1 |
| 9 | L-P1-05 | 1,500-2,000 word SKILL.md budget | P1 |
| 10 | L-P1-06 | Adopt validator-inspired hook-schema lint | P1 |
| 11 | L-P2-01 | Title Case skill frontmatter names | P2 |
| 12 | L-P2-02 | Tighten `allowed-tools` to subcommand globs | P2 |
| 13 | L-P2-03 | Formalize fail-closed-policy / fail-open-infra | P2 |
| 14 | L-P2-04 | Document plugin-wrapper vs user-direct hook formats | P2 |
| 15 | L-P2-05 | `headersHelper` pattern for future MCP servers | P2 |
| 16 | L-P3-01 | Document "reference bundle without plugin.json" | P3 |
| 17 | L-P3-02 | Canonical logs path — reject `/tmp/*.log` | P3 |
| 18 | L-P3-03 | Plugin Settings: one semantic per file | P3 |
| 19 | L-P3-04 | Document "we require tests, reference doesn't" | P3 |
| 20 | L-P3-05 | Standardize `hooks/` (not `hooks-handlers/`) | P3 |

---

## 9. Convergence Metadata

**Round counts per pass**: Pass 0 = 2; Pass 1 = 2; Pass 2 = **3** (converged); Pass 3 = **3** (converged); Pass 4 = 2; Pass 5 = **2 (bounded final)**.

**Contradictions corrected during convergence**:
1. Pass 0 round 1: `hookify/matchers/` and `hookify/utils/` are empty `__init__.py` placeholders, not populated modules.
2. Pass 2 round 2: TENSION-02 resolved for stdio — confirmed eager.
3. Pass 2 round 2: `allowed-tools` type is String OR Array (not Array only).
4. Pass 3 round 2: `security-guidance` timeout inherits 60s default (not "no effective timeout").
5. Pass 5 round 2: CONV-S02 **direction reversed** — Title Case is the majority (8/10).
6. Pass 5 round 2: Plugin Settings adopter count corrected 1 → 2 (hookify + ralph-wiggum). Dual-purpose conflation identified as latent anti-pattern.
7. Pass 5 round 2: Retracted "7 references" claim for `command-development/` — directory does not exist.

**Residual gaps**: TENSION-01, TENSION-03, TENSION-04, name collision resolution, matcher regex dialect name, no tests anywhere, 4 unexercised lifecycle events, no concrete `.mcp.json` files, no versioning/migration contract, fail-mode policy unpoliced, no canonical logging path, Plugin Settings dual-purpose conflation.
