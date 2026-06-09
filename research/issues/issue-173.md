# Issue #173 — Wave-boundary checkpoint+reset & lossless intra-wave compaction (PreCompact flush + WASM gates)

**Date:** 2026-06-09
**Issue:** [#173](https://github.com/drbothen/vsdd-factory/issues/173) — `feat(context): enforce wave-boundary checkpoint+reset and lossless intra-wave compaction (PreCompact flush + WASM gates)`
**Label:** enhancement
**Repo state:** `develop` @ `82163b7f`, plugin `v1.0.0-rc.20`
**Research agent:** Claude (vsdd-factory research-agent)

---

## Restated proposal

The factory externalizes state to `STATE.md` + the `factory-artifacts` branch but does not use that externalized state as a *deliberate context-management* mechanism. Long autonomous runs exhaust the context window and rely on reactive harness auto-compaction mid-wave, which silently loses pipeline continuity. The issue proposes a two-part enforced strategy: **(A) cross-wave** — write a *verified* handoff to external state at each wave close, then hard-reset the session and re-hydrate only the next wave's slice; **(B) intra-wave** — make auto-compaction lossless via a NEW `PreCompact` hook that flushes wave-critical state + pins retention instructions, plus tool-result clearing, sub-agent isolation, and a proactive ~50–70% threshold. Both parts enforced via hooks split along the factory's existing WASM-gate / shell-effect convention.

---

## Codebase grounding

### What exists today (verified)

| Capability | Status | Evidence |
|---|---|---|
| `PreCompact` hook | **ABSENT** | `grep -ri PreCompact` across the whole repo → **0 matches**. `hooks-registry.toml` registers `SessionStart`, `SessionEnd`, `WorktreeCreate`, plus the PreToolUse/PostToolUse fleet — no compaction event of any kind. |
| `compact-state` skill | EXISTS — *file* operation only | `plugins/vsdd-factory/skills/compact-state/SKILL.md` extracts historical STATE.md sections into `cycles/<cycle>/*.md`, slims STATE.md to <200 lines. Operates on the **file**, not the context window. Confirms the issue's "terminology collision" — `compact-state` ≠ context compaction. |
| `check-state-health` skill | EXISTS | `plugins/vsdd-factory/skills/check-state-health/SKILL.md` — HEALTHY/WARNING/NEEDS-COMPACT diagnostic on STATE.md. |
| `state-burst` skill | EXISTS | `plugins/vsdd-factory/skills/state-burst/SKILL.md` — single-commit burst + `git -C .factory push origin factory-artifacts` (line 159). This is the "flush" primitive the issue wants a PreCompact hook to invoke. |
| `recover-state` / `next-step` skills | EXIST | `skills/recover-state/`, `skills/next-step/` — provide resume bones, but nothing enforces a checkpoint *at the wave boundary* or a scoped re-hydration after reset. |
| Wave-boundary checkpoint enforcement | **ABSENT** | No hook/skill blocks wave-close until a verified handoff exists; `validate-wave-gate-*.sh` hooks gate wave *prerequisites/completeness*, not a context-checkpoint. |
| WASM gate convention | CONFIRMED | The hook fleet splits cleanly: WASM for deterministic parse-heavy validators, shell for effectful guards (`check-factory-commit.sh`, `factory-branch-guard.sh`, `red-gate.sh`). The issue's proposed hook split matches this established convention. |

**Conclusion:** The building blocks the issue names (`state-burst` flush, `STATE.md` externalization, WASM-validator convention, sub-agent isolation) all exist. The **two load-bearing pieces are entirely absent**: (1) any `PreCompact` integration with the harness, and (2) any wave-boundary checkpoint/reset enforcement. Prior-work grep of `CHANGELOG.md` and the active `decision-log.md` shows no PreCompact / context-compaction / wave-reset work.

---

## External research (primary sources)

### (1) Claude Code `PreCompact` hook semantics — CAN block (current docs)

**This is the issue's central open question ("can a PreCompact hook block/defer compaction, or only run alongside?") and the answer materially changes the design.**

Verified against the **live** official hooks reference (`https://code.claude.com/docs/en/hooks`, fetched 2026-06-09):

- **`PreCompact` CAN block compaction.** The decision-control table states: `PreCompact` → "Can block?" = **Yes**, "What happens on exit 2" = **"Blocks compaction"**. It also accepts top-level `decision: "block"` with a `reason` field. So the flush can be a **hard guarantee**, not best-effort — exit 2 (or `decision: block`) defers compaction until the flush succeeds.
- **Matcher values:** `manual` (user `/compact`) and `auto` (harness-triggered). A hook can target either.
- **A NEW `PostCompact` event also exists** (not mentioned in the issue): fires *after* compaction completes, **cannot block** (exit 2 → stderr-to-user only). This is the natural home for a **post-compaction re-hydration / verification** step (re-anchor STATE.md pointer, re-assert wave-critical facts).
- **`custom_instructions` caveat (conflicting sources — flag):** An older snapshot of the hooks reference (surfaced via Perplexity deep-research, citing `code.claude.com/docs/en/hooks`) documented that PreCompact receives `trigger` + `custom_instructions`, and that for `auto` triggers **`custom_instructions` is empty**. The *current* live page (WebFetch 2026-06-09) does **not** list a `custom_instructions` input for PreCompact at all. **Net:** retention-instruction injection via `custom_instructions` on an *auto* compaction is **not reliably available** — the issue's Part-B Step 1 "supply retention instructions to the summarizer" mechanism is uncertain and must not be the load-bearing path. The robust path is **flush-to-external-state on PreCompact (blocking) + re-hydrate on SessionStart/PostCompact** — both fully supported.
- PreCompact supports `command`, `http`, `mcp_tool` hook types (per the older reference) — i.e., it can shell out to a flush command, consistent with the issue's "shell for effectful flush" routing.

> Conflict resolution: where the two primary snapshots disagree, the **live page wins** (later + more specific per VSDD source-of-truth precedence). The live page is decisive that PreCompact **can block**; it is silent (not contradictory) on `custom_instructions`. Treat retention-pinning as best-effort-only and design the flush as the guarantee.

Sources: `https://code.claude.com/docs/en/hooks` (live, 2026-06-09); `https://code.claude.com/docs/en/hooks-guide`; `https://platform.claude.com/docs/en/build-with-claude/compaction`.

### (2) Anthropic context-engineering guidance — reset-at-boundary + external memory

- Anthropic's *Effective context engineering for AI agents* emphasizes **curate, don't dump** — token-budget allocation and **external memory systems** over raw context expansion. (`https://www.anthropic.com/engineering/effective-context-engineering-for-ai-agents`)
- The balance of evidence (Perplexity Sonar deep-research synthesis over Anthropic + LangGraph + Microsoft Agent Framework docs) favors **hard session resets at task/wave boundaries as the primary cross-boundary strategy**, supplemented by external state and *judicious* intra-task compaction. Rationale: resets avoid **stacking multiple lossy summarization passes** and enable **deterministic replay** from a checkpoint, independent of opaque provider compaction logic. The precondition (state externalized) is **already met** by the factory — which is exactly the condition under which reset beats continuous compaction.
- **Proactive compaction at ~70% of effective window capacity** (not absolute token limit) is the consensus sweet spot: it prevents context rot while avoiding the last-second (~95%) auto-compaction that stacks lossy summaries. (Anthropic context-engineering blog; Microsoft Agent Framework compaction docs `https://learn.microsoft.com/en-us/agent-framework/agents/conversations/compaction`.)
- **Hallucinated-state risk** from lossy summarizers (e.g., "all tests passed" when only some did) is a documented failure mode; for a *verified* pipeline this is amplified — the factory has already observed fabricated SHAs (jira-cli evidence in the issue). Verified-against-git/tests checkpoints directly mitigate this. (`https://platform.claude.com/docs/en/test-and-evaluate/strengthen-guardrails/reduce-hallucinations`; community: dev.to AI-agent-failure-modes, producttalk context-rot.)

### (3) Tool-result clearing & sub-agent isolation

- Clearing re-fetchable bulky tool outputs (test logs, file dumps) is preferred over summarizing them — they can be re-read on demand, and summarization is where drift enters. (Anthropic context-engineering blog; Redis context-engineering best-practices `https://redis.io/blog/context-engineering-best-practices-for-an-emerging-discipline/`.)
- Routing heavy ops (large test runs, broad greps, research) to **sub-agents** keeps their cost out of the orchestrator window — already a factory strength; the issue proposes *enforcing* it via a PreToolUse delegation guard. (`https://www.anthropic.com/engineering/multi-agent-research-system`.)

---

## Verdict

> **VALID-NEW** — Confidence: **High**

The two load-bearing pieces (a `PreCompact` hook of any kind, and wave-boundary checkpoint/reset enforcement) are verifiably absent (0 grep hits, no registry entry, no prior CHANGELOG/decision-log work). The proposal is technically sound and well-aligned with current Claude Code primary docs and Anthropic guidance. **One issue assumption is now resolved more favorably than the issue assumed:** PreCompact **can** hard-block compaction (live docs), so the flush can be a guarantee rather than best-effort. **One assumption is weaker than stated:** retention-instruction injection via `custom_instructions` is unreliable on auto-compaction (current docs omit it / older docs say it's empty for `auto`), so it must not be the load-bearing mechanism.

---

## Recommended approach + scope (zero re-research)

### Architecture decision needed first (route: architect)
Adopt the research consensus: **reset-at-wave-boundary is primary; intra-wave PreCompact flush is the safety net.** Record as an ADR (the issue composes with #171 deferred-process-gaps, which belong in the handoff). The proactive-threshold value (50% vs 70%) is the one genuine human/architect decision — recommend **70%** per consensus, configurable per autonomy level.

### Part A — cross-wave checkpoint → reset → scoped rehydrate
- **New WASM gate: `validate-wave-handoff-completeness`** (class: `regression-gate.wasm` / `pr-manager-completion-guard.wasm`). Blocks wave-close until a structured `HANDOFF.md` on `factory-artifacts` has all required fields (decisions w/ commit-hash|test-id|file-path cites, pending fixes, open process-gaps, last *verified* develop SHA, active BCs, next-wave story list). Reuse/extend the existing `handoff-validator` if present.
- **Verification rule (anti-fabrication):** handoff claims cross-checked against git/tests, not memory — directly mitigates the jira-cli fabricated-SHA failure mode. Pair with the existing `verify-sha-currency.sh` template logic.
- **Reset + scoped rehydrate:** orchestrator-level skill (`wave-reset` or extend `recover-state`) that loads **only** the next wave's slice (its stories + the specs they touch), curated via `wave-state.yaml` manifest (preferred over RAG for determinism). Rely on prompt caching on the stable prefix (CLAUDE.md + architecture + STATE.md pointer).

### Part B — intra-wave lossless compaction
- **New shell hook: `precompact-flush.sh`** on event `PreCompact` (effectful: git/FS). Invokes a `state-burst` flush, then `exit 2` / `decision:block` **only if the flush is required and incomplete** — using the now-confirmed blocking capability to defer compaction until state is persisted. **Do NOT rely on `custom_instructions` for retention** (unreliable on auto). Register in `hooks-registry.toml` under a new `[[hooks]] event = "PreCompact"` block; add the `command`/script-path entry mirroring the legacy-bash-adapter pattern.
- **New `PostCompact` hook (advisory):** re-anchor the STATE.md pointer and re-assert wave-critical facts after compaction (cannot block; advisory re-hydration). This is the piece the issue missed — `PostCompact` is the correct home for post-compaction verification.
- **PreToolUse delegation guard (WASM):** nudge/enforce heavy-op → sub-agent.
- **PostToolUse output-size guard (WASM or shell):** nudge tool-result clearing.
- **Proactive threshold:** settings/config, not a hook (per the issue) — configure the auto-compact band.

### Risks
- **`custom_instructions` unreliability on auto-compaction** (documented above) — the single biggest design risk; mitigate by making the flush the guarantee and retention best-effort.
- **Blocking PreCompact deadlock:** if `precompact-flush.sh` blocks but the flush itself needs context that's about to be compacted, you can wedge a session. Keep the flush hermetic (reads STATE.md + git, not in-context reasoning).
- **WASM cannot do git/FS** (sandboxed) — flush MUST stay shell; only the *completeness gate* is WASM (WASM decides → shell acts). Matches the issue's own caveat and the factory convention.
- **Single-wave / short runs:** ensure reset/checkpoint scales with wave boundaries, not every step (AC in the issue) — no new friction for short pipelines.

### Test strategy
- Bats: PreCompact shell flush fires + commits on a simulated compaction event; blocking exit-2 path defers when handoff incomplete.
- WASM unit tests for the handoff-completeness gate (required-fields present/absent; fabricated-SHA detection).
- Integration: simulate a mid-wave compaction; assert no load-bearing SHA/decision is lost (re-hydrate matches pre-compact state).

### Dependencies
- Confirm the installed Claude Code harness build actually emits `PreCompact`/`PostCompact` (live docs say yes as of 2026-06-09; pin the harness version in the ADR). The operator-level marketplace cache must pick up the new hooks via a release (develop edits don't affect the cached plugin).
- Composes with **#171** (deferred process-gaps belong in the handoff) and reduces blast radius of the **#170** fabrication/race class.

---

## Research Methods

| Tool | Queries | Purpose |
|------|---------|---------|
| **Perplexity perplexity_research (PRIMARY)** | 1 | Claude Code PreCompact semantics + Anthropic context-engineering best practices (deep multi-source) |
| Perplexity perplexity_ask | 0 (shared w/ #174) | — |
| WebFetch | 2 | Live `code.claude.com/docs/en/hooks` — PreCompact/PostCompact blocking capability, matcher values, event list |
| Read / Grep / Glob | ~12 | Codebase grounding: hooks-registry, compact-state, check-state-health, state-burst, PreCompact absence, prior-work CHANGELOG/decision-log |
| Training data | 0 load-bearing | All blocking-capability / event-name / threshold claims verified against primary docs this session |

**Total MCP tool calls (this issue):** 1 research + 2 WebFetch (+ shared ask). **Training data reliance:** LOW — the pivotal "PreCompact can block" fact was verified against the live primary doc, overriding an older conflicting snapshot.
