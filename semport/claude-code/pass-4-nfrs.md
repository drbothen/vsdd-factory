# Pass 4 — Non-Functional Requirements

_Phase B convergence round 1._

## Security

`security-guidance/hooks/security_reminder_hook.py:31-126` hardcodes **exactly 9** patterns scanned on `Edit|Write|MultiEdit` (exhaustively verified round 1 — Phase A list complete, no missed rules):

1. **github_actions_workflow** (path-based, `:33-35`) — `${{ github.event.* }}` injection in `.github/workflows/*.{yml,yaml}`; recommends env-var indirection. Links GitHub advisory.
2. **child_process_exec** (`:70-71`) — `child_process.exec`, `exec(`, `execSync(` → recommend execFile.
3. **new_function_injection** (`:92-93`) — `new Function` → code injection.
4. **eval_injection** (`:97-98`) — `eval(` → RCE.
5. **react_dangerously_set_html** (`:102-103`) — `dangerouslySetInnerHTML` → XSS.
6. **document_write_xss** (`:107-108`) — `document.write` → XSS.
7. **innerHTML_xss** (`:112-113`) — `.innerHTML =`, `.innerHTML=` → XSS.
8. **pickle_deserialization** (`:117-118`) — bare substring `pickle` → RCE (over-broad; matches `pickleball`).
9. **os_system_injection** (`:122-123`) — `os.system`, `from os import system` → command injection.

Covered CWEs (not labelled in source — gap): CWE-78, CWE-79, CWE-94, CWE-502.

**First-match short-circuit** (`:188-199`): `check_patterns` returns on the first hit; only one reminder per tool call.

**Kill-switch**: `ENABLE_SECURITY_REMINDER=0` env var disables (`security_reminder_hook.py:219-224`).

**Session-scoped dedup**: `~/.claude/security_warnings_state_<session_id>.json`. Probabilistic 10% GC of state files >30 days old (`:134-156`) — unusual but pragmatic given no daemon.

**Security-as-policy via command frontmatter**: `commit.md:2` is the **only** example of `allowed-tools` subcommand globbing in the corpus.

## Validator-enforced NFRs (round 1 — new)

`plugin-dev/skills/hook-development/scripts/validate-hook-schema.sh` enforces a de facto NFR ruleset for hook configuration. Each rule is an enforceable convention:

- **NFR-V01** (`:41-55`): Event name must be one of the nine canonical events `PreToolUse, PostToolUse, UserPromptSubmit, Stop, SubagentStop, SessionStart, SessionEnd, PreCompact, Notification`. Unknown = warning.
- **NFR-V02** (`:70-75`): `matcher` field required (error if missing). **TENSION-03** — contradicts docs and hookify behavior.
- **NFR-V03** (`:77-83`): `hooks` array required.
- **NFR-V04** (`:91-101`): Hook `type` must be `command` or `prompt`; enum-validated.
- **NFR-V05** (`:104-108`): Command hooks must declare `command`.
- **NFR-V06** (`:110-114`): Commands starting with `/` and not containing `${CLAUDE_PLUGIN_ROOT}` trigger a "hardcoded absolute path" warning — **this is the canonical portability rule**.
- **NFR-V07** (`:116-121`): Prompt hooks must declare `prompt`.
- **NFR-V08** (`:123-127`): Prompt hooks supported only on `Stop`, `SubagentStop`, `UserPromptSubmit`, `PreToolUse`; others warn.
- **NFR-V09** (`:131-142`): Timeout must be integer; advisory range `[5, 600]`; outside → warning (not error).
- **NFR-V10** (`:150-158`): Exit taxonomy — errors fail validation (exit 1), warnings pass (exit 0).

## Performance — default-timeout spectrum

Per BC-DRAFT-H09, the runtime assigns timeouts when not specified: **command 60s, prompt 30s**. Observed spectrum across the corpus:

| Plugin | Timeout | Notes |
|---|---|---|
| `hookify` (all hooks) | 10s | Tightest. Relies on fail-open `try/except` in handler. |
| `security-guidance` | (unset) → **60s default** | Round 1 resolves Phase A's gap: `hooks/hooks.json` sets no `timeout`, so the 60s command default applies. "No effective timeout" from Phase A was wrong. |
| `plugin-dev` examples | (unset) → 60s/30s | Docs never set explicit timeouts. |
| Validator advisory | `[5, 600]` | Above/below warns. |

**NFR baseline**: 60s for command hooks, 30s for prompt hooks. Plugins that need tighter budgets (hookify = 10s) must set explicitly. No plugin sets a value above the default.

No caching, no parallelism inside plugins. Pass 3 round 2 confirmed all matching hooks for one event run in parallel at the runtime level (BC-DRAFT-H13) — individual plugins cannot rely on ordering.

## Fail-open vs fail-closed

Deliberate split, never documented as policy:

- `hookify` fails **open** on import errors (`pretooluse.py:29-32` — logs systemMessage, exits 0).
- `security-guidance` fails **closed** on pattern match (exit 2 blocking), **open** on JSON parse errors (`:235-236`).
- `plugin-dev` skill hooks: round 1 survey — `plugin-dev` has no runtime hooks of its own; its `hook-development` skill only ships the validator script and documentation. No additional fail-mode examples.

Two-plugin sample confirms the split is real and unpoliced. **Recommendation**: formalize as "fail-closed on policy violation, fail-open on infrastructure error" and encode in the validator.

## Observability

- `security-guidance` writes `/tmp/security-warnings-log.txt` (`:14`) — hardcoded, non-portable (Windows fails, racy across users, world-readable in `/tmp`). **Anti-pattern.**
- `hookify/core/` round-1 survey: no logging module, no structured logger. Hookify's handlers print via `systemMessage` in the hook response envelope — that is the **only** canonical observability channel in the corpus.
- No structured logging, no trace IDs, no OTel anywhere.
- Silent `try/except` around debug_log (`:23-25`) to avoid breaking the user's workflow (acceptable trade but deviates from SOUL.md #4).

**Canonical logging convention (round 1)**: There is no logging convention. The closest thing is returning `systemMessage` strings in the hook response JSON. Plugin Settings (`.claude/<plugin>.local.md`) is the recommended location for **runtime config**, not logs.

## Error handling

- Hook stdin parse failure → "allow" (fail open).
- Pattern match → block via exit 2 + stderr.
- No retry, no circuit breakers, no backoff.
- Validator exit codes (NFR-V10) are the only formal error taxonomy in the tooling layer.

## Reliability

- `${CLAUDE_PLUGIN_ROOT}` indirection is the **sole** portability mechanism for executable paths. All 13 plugins comply for command paths (enforced advisory by NFR-V06).
- **Data-path gap** (Phase A) — partially resolved round 1: **Plugin Settings at `.claude/<plugin-name>.local.md` (BC-DRAFT-PS01) is the recommended portable config-data path.** Gitignored via `.claude/*.local.md`, user-managed, `chmod 600` recommended. This closes the data-path portability gap for runtime configuration (but not for logs — the `/tmp/security-warnings-log.txt` anti-pattern has no canonical replacement).

## Gaps

1. No declared NFRs anywhere in-source (validator is the de facto spec).
2. Hardcoded `/tmp/security-warnings-log.txt` (no canonical log-path convention).
3. No structured telemetry, no trace IDs.
4. Fail-mode policy unpoliced — validator does not enforce fail-open-vs-closed.
5. Bare `pickle` substring over-matches (false positives).
6. No plugin ships tests for its hooks.
7. CWE labels absent from security reminder messages.
