# Not Portable — Dark-Factory Extensions That Don't Map to Claude Code

Dark-factory ships 20 OpenClaw TypeScript runtime extensions. Most of them have been ported to Claude Code primitives in Waves 4–6. These four do not — they require integration points that Claude Code's plugin API does not currently expose, and shipping a half-implementation would be worse than an honest gap.

## 1. Cost tracker (`cost-tracker.ts`)

**What it did**: Intercepts every model call, tracks per-agent token usage and USD cost, enforces a pipeline budget (`budgetMaxUsd`), and downgrades models when approaching the limit (e.g., Opus → Sonnet → Haiku).

**Why it doesn't port**: Claude Code's plugin hooks fire on tool calls, not on model calls. There is no `PreModelCall` or `PostModelCall` event. A plugin can observe `Bash`, `Edit`, `Write`, `Task` etc., but cannot see token counts or model selection for the main thread or sub-agents.

**Partial workaround**: a plugin could ship a wrapper binary that users invoke via `claude code` with custom env vars, and parse the resulting session JSON afterward. That's out of scope for a normal plugin.

**Recommendation**: rely on `claude /cost` (built-in) for per-session cost visibility. If you need hard budget enforcement, use Anthropic Console spend limits at the API key level.

## 2. Attention heatmap (`attention-heatmap.ts`)

**What it did**: Logged which files/paths the main thread touched over the course of a session and produced a heatmap to surface drift and over-focus.

**Why it doesn't port**: same as above — requires visibility into the main thread's file-touch history that's not exposed via hooks. A PostToolUse hook on Edit|Write can observe writes but not reads (no hook fires on Read), so any resulting "heatmap" would be biased toward writes and miss the actual attention pattern.

**Partial workaround**: the `PostToolUse` hook on Edit|Write could append to a log file; a separate skill could render it. Low value compared to the engineering cost.

**Recommendation**: skip. Session review (`/vsdd-factory:session-review`) provides similar narrative value through synthesis rather than telemetry.

## 3. Tiered context (`tiered-context.ts`)

**What it did**: Automatically tiered context documents by relevance — loads a small "hot" core of docs, pulls in "warm" docs on demand, and leaves "cold" docs on disk until referenced.

**Why it doesn't port**: Claude Code has its own context management and doesn't expose a pluggable document loader. Attempting to shadow it from a plugin would fight the built-in behavior.

**Workaround**: the pattern is already partially supported by skills that use progressive disclosure (the `skills/` directory is lazy-loaded by design). Ship docs at different locations and let skills pull them in as needed.

**Recommendation**: design new documentation with progressive disclosure in mind — keep high-level orientation in `docs/`, domain deep-dives in `docs/deep/`, and reference data in tool-accessible formats like the `workflows/*.lobster` corpus.

## 4. Full sidecar learning synthesis (`sidecar-learning.ts`)

**What it did**: At session end, analyzed the session transcript for lessons, decisions, and follow-ups, and produced a structured sidecar-learning document without user prompting.

**Why it only partially ports**: Claude Code's `Stop` hook fires with minimal session context — it can mark that a session ended, but cannot read the transcript or synthesize from it. The plugin ships `hooks/session-learning.sh` which writes a timestamped stub to `.factory/sidecar-learning.md`, and the `session-review` skill (Wave 2) provides the synthesis on explicit invocation.

**Workaround shipped**: automatic marker (hook) + manual synthesis (skill). Users run `/vsdd-factory:session-review` when they want the full synthesis.

**Recommendation**: use the shipped combo. If Anthropic adds a `Stop` hook variant with transcript access in the future, the gap closes.

---

## Summary

| Dark-factory extension | Claude Code status | Mitigation |
|---|---|---|
| `cost-tracker.ts` | Not portable | Use `/cost` built-in + API-level spend limits |
| `attention-heatmap.ts` | Not portable | Use `session-review` skill synthesis |
| `tiered-context.ts` | Not portable | Rely on built-in context management + progressive docs |
| `sidecar-learning.ts` (full) | Partially ported | `hooks/session-learning.sh` + `skills/session-review` |

All 16 other OpenClaw extensions were successfully ported as Claude Code hooks (Wave 4) or bin helpers (Wave 6).
