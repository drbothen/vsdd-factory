# Research: PreCompact Hook Gating Capability (Issue #173)

**Date:** 2026-06-13
**Type:** general (Claude Code harness capability)
**Status:** complete — HIGH confidence on the core gating question
**Scope:** GATING research for Issue #173 Part B (PreCompact flush design)

---

## CORE QUESTION (verdict first)

**Can a `PreCompact` hook BLOCK / DEFER / DELAY automatic compaction until an external flush completes?**

> **YES — a PreCompact hook CAN block compaction**, via exit code 2 OR `{"decision": "block"}`. This is a **documented, first-class** capability of the official Claude Code hooks system.

**CRITICAL VERSION CAVEAT:** This capability is **version-dependent**. PreCompact blocking was added in **Claude Code v2.1.105**. Before that release, PreCompact was a notification-only / best-effort hook (exit 2 showed stderr to the user but did **not** block). Any design relying on the block MUST pin/assert a minimum Claude Code version ≥ v2.1.105. The marketplace-tarball consumed dispatcher does NOT govern this — the *Claude Code harness version the operator runs* does. Flag this as a hard runtime precondition.

Sources (official): `code.claude.com/docs/en/hooks` "Exit code 2 behavior per event" table → `PreCompact | Yes | Blocks compaction`; "Decision control" table lists `PreCompact` under top-level `decision` supporting `decision: "block"`, `reason`. Version attribution: `github.com/shanraisshan/claude-code-hooks` HOOKS-README ("blocks compaction (since v2.1.105)"); `qiita.com/kai_kou` ("v2.1.105 で新たに PreCompact Hook が追加"). [HIGH]

---

## SUB-QUESTION ANSWERS

### 1. PreCompact semantics — **DOCUMENTED**

- **When it fires:** Immediately **before** context compaction begins (manual `/compact` or auto). [HIGH — official]
- **Matcher field:** Matches on the compaction trigger. Possible values: **`manual`** (user ran `/compact`) and **`auto`** (context-pressure auto-compaction). Omitting the matcher or using `"*"` fires on both. [HIGH — official]
- **JSON input (official, verbatim shape):**
  ```json
  {
    "session_id": "abc123",
    "transcript_path": "/Users/.../transcript.jsonl",
    "cwd": "/Users/...",
    "hook_event_name": "PreCompact",
    "trigger": "manual"   // or "auto"
  }
  ```
  Common fields (`session_id`, `transcript_path`, `cwd`, `permission_mode`, `hook_event_name`) plus the PreCompact-specific **`trigger`**.
- **`custom_instructions`:** **PARTIALLY DOCUMENTED / CONFLICTING.** Multiple secondary sources (disler/claude-code-hooks-mastery; techtaek.com; claude.com blog) state PreCompact also receives **`custom_instructions`** carrying the text the user passed to `/compact` (empty for `auto`). However, two direct fetches of the **official** `code.claude.com/docs/en/hooks` page did **not** surface a `custom_instructions` field in the PreCompact input example — only `trigger`. **Treat `custom_instructions` as likely-present-but-not-officially-confirmed; do NOT build a hard dependency on it.** [MEDIUM]

**Answer:** Documented. Fires before compaction; matcher `manual`/`auto`; receives `trigger` + common fields. **Confidence: HIGH** (core), **MEDIUM** (`custom_instructions`).

### 2. Blocking capability — **YES (DOCUMENTED, v2.1.105+)**

- **`exit 2`** → blocks compaction; stderr is fed back to Claude as the reason. [HIGH — official]
- **`{"decision": "block", "reason": "..."}`** (top-level, exit 0) → blocks compaction with structured reason. [HIGH — official]
- `hookSpecificOutput.permissionDecision` is **NOT** the mechanism here (that is PreToolUse/PermissionRequest). PreCompact uses the **top-level `decision`** field per the official Decision-control table.
- `continue: false` is the **Stop/SubagentStop** mechanism, not PreCompact's gate.

**Answer: YES, PreCompact has a real VETO over compaction** (≥ v2.1.105). This is NOT best-effort-only on current versions. **Confidence: HIGH**, gated on version.

> **Design nuance — block ≠ pause-and-resume.** The hook returns a binary verdict (block / allow) **synchronously**; it does not "delay until an async flush completes" in a first-class way. To make blocking useful for a flush, the flush must run **synchronously inside the hook process** (the hook is a command on stdin/stdout/exit), OR the hook blocks repeatedly until a separately-running flush sets a ready-flag the hook checks. There is no documented "defer-and-callback" primitive — only "block now, allow on a later compaction attempt." Auto-compaction will be re-attempted as context keeps growing, so a block buys time but is not an indefinite hold without the model continuing to push context.

### 3. Retention / steering of WHAT compaction keeps — **NO (UNDOCUMENTED)**

- There is **NO documented mechanism** for a PreCompact hook to inject retention instructions, `additionalContext`, or `hookSpecificOutput` that the harness summarizer is guaranteed to preserve. Two official-page fetches both returned: PreCompact is a **block/allow gate only**, not a context-injection event. [HIGH — official, negative finding]
- The only user-facing steering of compaction content is the **manual `/compact <instructions>`** argument (user-driven, not hook-driven). The Agent **SDK** has a separate `compaction_control.summary_prompt` (different product surface — the Python Agent SDK, not the Claude Code CLI harness hooks).

**Answer: NO / UNDOCUMENTED** for hook-driven retention steering in the CLI harness. **Confidence: HIGH** (that it is not documented).

### 4. Proactive threshold — **YES (env var, with a hard ceiling)**

- **`CLAUDE_AUTOCOMPACT_PCT_OVERRIDE`** (integer 0–100, set in `settings.json` `env` block or shell): triggers auto-compaction at the chosen % of context capacity. e.g. `"75"` = compact at 75%. [HIGH — multiple sources incl. settings guides]
- **Hard ceiling:** You **cannot raise** the threshold above the internal default (~83%). Implementation uses `Math.min(userOverride, defaultThreshold)`. So 50–70% **proactive** banding is achievable (lower = earlier); raising it past ~83% is a no-op. [HIGH]
- **`DISABLE_AUTO_COMPACT=1`** → disables auto-compaction, manual `/compact` still works. **`DISABLE_COMPACT=1`** → disables all compaction (risks hard-limit failure). [HIGH]
- NOTE: this is an **env/settings** knob, not officially enumerated in the public hooks reference; sourced from settings documentation + community guides. Treat exact default (~83%) and clamp behavior as community-verified, not first-party-quoted. [MEDIUM-HIGH]

**Answer: YES — `CLAUDE_AUTOCOMPACT_PCT_OVERRIDE` enables proactive earlier compaction (down to any band ≤ ~83%).** Confidence: HIGH that it exists; MEDIUM on exact clamp value.

### 5. Tool-result clearing (drop vs summarize) — **NO hook-driven mechanism (UNDOCUMENTED)**

- No documented mechanism to selectively CLEAR / drop specific re-fetchable tool outputs from context (vs summarizing). Compaction is described as a whole-context summarize-older-turns operation; there is no per-tool-output eviction primitive, hook-driven or otherwise. [HIGH — negative finding]
- The only "clear" surfaces are **`/clear`** (full session reset — destroys everything, not selective) and delegating verbose ops to **subagents** (see #6) so the output never enters the parent context in the first place.

**Answer: NO selective tool-output clearing.** The architectural workaround is subagent isolation (#6) or whole-session `/clear`. **Confidence: HIGH.**

### 6. Sub-agent context isolation — **YES (DOCUMENTED, strong guarantee)**

- Official: *"Each subagent runs in its own context window... the subagent does that work in its own context and returns only the summary."* *"The verbose output stays in the subagent's context while only the relevant summary returns to your main conversation."* [HIGH — official `code.claude.com/docs/en/sub-agents`]
- *"Each subagent starts with a fresh, isolated context window. It does not see your conversation history..."* The parent receives **only the final output message**, not the subagent's tool-call history or reasoning. [HIGH — official]
- **Cost caveat:** multi-agent workflows use ~4–7x more tokens overall (each subagent opens its own window); Agent Teams ~15x. Isolation protects the PARENT window but multiplies total token spend. [MEDIUM — community, citing Anthropic guidance]

**Answer: YES — subagent (Task/Agent) token usage is isolated from the orchestrator's context window.** Heavy ops delegated to subagents do NOT consume the parent window (only their final summary does). **Confidence: HIGH.**

---

## DESIGN IMPLICATION (3-line summary)

1. **Part B's PreCompact flush is a HARD guarantee — but only on Claude Code ≥ v2.1.105 AND only if the flush runs SYNCHRONOUSLY inside the hook (block via exit 2 / `decision:"block"` until flush completes).** Pin/assert the harness version as a runtime precondition; on older versions PreCompact is best-effort-only with no veto.
2. **You CANNOT steer what compaction keeps via the hook (retention injection is undocumented), and you CANNOT selectively clear tool outputs** — so the strongest design pairs (a) `CLAUDE_AUTOCOMPACT_PCT_OVERRIDE` to trigger early/proactively at a chosen band ≤ ~83%, (b) a synchronous PreCompact flush-gate that persists state to durable storage before allowing compaction, and (c) **subagent isolation** to keep heavy/re-fetchable work out of the orchestrator window entirely (the only reliable "don't-summarize-this" lever).
3. **Recommended architecture:** durable external flush is the source of truth (don't rely on the summarizer preserving anything); PreCompact-block is the *commit barrier* guaranteeing the flush lands before context is lost; subagent delegation + early-threshold compaction are the *pressure-relief* mechanisms that reduce how often the barrier is hit.

---

## Research Methods

| Tool | Queries | Purpose |
|------|---------|---------|
| **Perplexity perplexity_research (PRIMARY)** | 1 | Attempted deep synthesis of PreCompact semantics — **FAILED USEFULLY**: sonar-deep-research's knowledge cutoff (~late 2024) predates Claude Code hooks; it correctly declined to fabricate and flagged the topic as undocumented-to-it. Surfaced the version-sensitivity risk. |
| Perplexity perplexity_search | 3 | Raw URL + snippet ranking on PreCompact exit-code-2/block, autoCompact threshold settings, subagent isolation. These snippets (including official `code.claude.com` doc text) were the load-bearing evidence. |
| WebFetch | 3 | Direct fetch of official `code.claude.com/docs/en/hooks` (PreCompact semantics + exit-2-per-event table + retention check) and `code.claude.com/docs/en/sub-agents` (isolation). First-party verification. |
| Training data | 1 area | Cross-checked hook field names (continue/decision/permissionDecision routing) against model knowledge — explicitly flagged and re-verified against official docs; not relied on for the version-gated facts. |

**Total MCP tool calls:** 4 (1 research + 3 search). Plus 3 WebFetch (official-doc verification).
**Training data reliance:** LOW — the core gating verdict (PreCompact CAN block, since v2.1.105) is sourced from official `code.claude.com` docs cross-validated against 3 independent secondary sources. The deep-research model was explicitly unable to answer (stale cutoff), so ZERO load-bearing claims rest on training data. Negative findings (#3 retention, #5 clearing) are doc-grounded absence-of-mechanism, double-confirmed via two official-page fetches.

### Note on PRIMARY-tool deviation
`perplexity_research` (sonar-deep-research) was invoked FIRST per mandate but returned non-substantive output: its knowledge cutoff predates the Claude Code hooks feature, so it correctly refused to fabricate. The substantive evidence therefore came from `perplexity_search` snippets of the live official docs + direct `WebFetch` of `code.claude.com`. This is the appropriate methodology for a feature that postdates the deep-research model's training window — and is itself a finding: **this topic is fast-moving and version-gated; always verify against the live harness docs, never a model's training data.**

### Conflict log
- **`custom_instructions` field:** secondary sources assert it; official-page fetches did not surface it. Resolved as MEDIUM-confidence "likely present, do not hard-depend." Recommend empirical confirmation against the operator's actual Claude Code version before designing on it.
- **Auto-compact default threshold:** community sources variously cite ~83%, ~75%, and historically ~95%. The threshold has been lowered over releases. Treat the exact number as version-dependent; the *override mechanism* (`CLAUDE_AUTOCOMPACT_PCT_OVERRIDE`, clamped by `Math.min`) is the stable fact.
