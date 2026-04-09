# Pass 4 — Non-Functional Requirements

## Security

`security-guidance/hooks/security_reminder_hook.py:31-126` hardcodes 9 patterns scanned on `Edit|Write|MultiEdit`:

1. **github_actions_workflow** (path-based) — `${{ github.event.* }}` injection in workflows; recommends env-var indirection. Links GitHub's advisory.
2. **child_process_exec** — `child_process.exec`, `exec(`, `execSync(` → recommend execFile.
3. **new_function_injection** — `new Function` → code injection.
4. **eval_injection** — `eval(` → RCE.
5. **react_dangerously_set_html** — `dangerouslySetInnerHTML` → XSS.
6. **document_write_xss** — `document.write` → XSS.
7. **innerHTML_xss** — `.innerHTML =` → XSS.
8. **pickle_deserialization** — `pickle` → RCE.
9. **os_system_injection** — `os.system` → command injection.

Covered CWEs (not labelled in source — gap): CWE-78, CWE-79, CWE-94, CWE-502.

**Kill-switch**: `ENABLE_SECURITY_REMINDER=0` env var disables (`security_reminder_hook.py:219-224`).

**Session-scoped dedup**: `~/.claude/security_warnings_state_<session_id>.json`. Probabilistic 10% GC of state files >30 days old (`:134-156`) — unusual but pragmatic given no daemon.

**Security-as-policy via command frontmatter**: `commit.md:2` is the **only** example of `allowed-tools` subcommand globbing in the corpus. Should be adopted aggressively.

## Fail-open vs fail-closed

Deliberate split, never documented as policy:

- `hookify` fails **open** on import errors (`pretooluse.py:29-32` — logs systemMessage, exits 0).
- `security-guidance` fails **closed** on pattern match, **open** on JSON parse errors (`:235-236`).

Both are defensible; neither is documented as a policy. Worth formalizing.

## Performance

- `hookify` sets `timeout: 10s` on every hook.
- `security-guidance` sets **no timeout** — a buggy pattern would stall every Edit/Write.
- No caching, no parallelism inside plugins.

## Observability

- `security-guidance` writes `/tmp/security-warnings-log.txt` (hardcoded, non-portable — Windows fails, racy across users).
- No structured logging, no trace IDs, no OTel anywhere.
- Silent `try/except` around logging to avoid breaking the user's workflow (acceptable trade but deviates from SOUL.md #4).

## Error handling

- Hook stdin parse failure → "allow" (fail open).
- Pattern match → block.
- No retry, no circuit breakers, no backoff.

## Reliability

`${CLAUDE_PLUGIN_ROOT}` indirection is the **sole** portability mechanism. All 13 plugins comply for command paths. **None comply for data paths — this is the gap.**

## Gaps

1. No declared NFRs anywhere
2. Hardcoded `/tmp/security-warnings-log.txt`
3. No timeout on `security-guidance` hook
4. No structured telemetry
5. No plugin loads external policy (security-guidance hardcodes patterns; hookify is the exception, reading `.claude/hookify.*.local.md`)
