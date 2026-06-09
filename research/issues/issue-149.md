# Issue #149 — OTEL telemetry to reduce agent handwaving

**Date:** 2026-06-09
**Repo:** vsdd-factory (self-referential) @ `develop` `82163b7f`
**Issue:** [#149](https://github.com/) — *OTEL telemetry to reduce agent handwaving* (label: none; state: OPEN)
**Research agent:** Claude (vsdd-factory:research-agent)
**Consumer:** orchestrator / issue-triage → GitHub close decision

---

## Restated Question

The operator runs an OTEL dashboard (Grafana) to verify that an AI agent (their "AI GM") "is not making up things and is actually running the mechanics." The dashboard surfaces a telemetry channel **the agent does not have write access to** — unlike logs or output files, which the agent *can* write — so it provides ground truth on whether the agent actually performed an action rather than merely claiming it did (the example image shows a `rag` row "pointing to an actual problem"). The ask: **provide OTEL telemetry that lets a human verify agent behavior out-of-band.**

This is a one-line issue with an attached screenshot. There are no acceptance criteria, no file references, and no scope statement beyond "OTEL telemetry to reduce agent handwaving."

---

## Codebase Grounding (decisive)

vsdd-factory **already ships a complete OTEL observability stack with exactly this property** — a telemetry channel the agent cannot write to, surfaced in Grafana.

### 1. `claude-telemetry` skill — wires Claude Code's native OTel export

`plugins/vsdd-factory/skills/claude-telemetry/SKILL.md` (lines 1–67):

> "Claude Code emits detailed OTel telemetry (every tool call, token count, session event, API request/response summary) when `CLAUDE_CODE_ENABLE_TELEMETRY=1` is set. Our otel-collector (started via `factory-obs up`) accepts this export on port 4318 and forwards the logs to the same Loki that hook events use, tagged `service_name=claude-code` so they're queryable separately."

It manages five env vars written to `.claude/settings.local.json` (`claude-telemetry/SKILL.md:58-67`):
`CLAUDE_CODE_ENABLE_TELEMETRY=1`, `OTEL_METRICS_EXPORTER=otlp`, `OTEL_LOGS_EXPORTER=otlp`, `OTEL_EXPORTER_OTLP_PROTOCOL=http/protobuf`, `OTEL_EXPORTER_OTLP_ENDPOINT=http://localhost:4318`.

### 2. `factory-obs` skill — the 5-service stack that renders it

`plugins/vsdd-factory/skills/factory-obs/SKILL.md` (lines 1–60):

> "a 5-service stack (OTel Collector + Loki + Prometheus + Grafana + Grafana Image Renderer) that ingests `.factory/logs/events-*.jsonl` into Loki and Claude Code's native OTel metrics into Prometheus, and surfaces the data in 7 preconfigured Grafana dashboards."

The stack can watch **multiple factory projects** via a user-level registry (`~/.config/vsdd-factory/watched-factories`).

### 3. `onboard-observability` skill — one-command setup

`plugins/vsdd-factory/skills/onboard-observability/SKILL.md` (lines 1–47): combines `factory-obs register` + the Claude OTel env-var write so a project is wired end-to-end in one invocation.

### The key property the issue asks for is satisfied

The issue's core insight — *"a channel Claude does not have write access to, compared to logs or output files, which it does. That allows easy checking on whether it actually DID it"* — is precisely the architecture already shipped:

- **Hook events** (`.factory/logs/events-*.jsonl`) are emitted by the **factory-dispatcher binary in the WASM/host runtime**, not written by the agent's Edit/Write tools. The agent cannot forge them; they are produced by the enforcement layer as a side effect of real tool calls.
- **Claude Code native OTel** is emitted by the **Claude Code process itself**, exported directly to the collector over OTLP. The agent's text output and file writes cannot fabricate these — token counts, tool-execution events, and permission decisions originate in the runtime.

Both feeds land in Grafana (7 dashboards) where the human inspects them out-of-band — exactly the operator's workflow.

### Prior closure check

No CHANGELOG entry references #149 by number, but the three observability skills are present and mature in `develop`. The capability exists independent of this issue's filing.

---

## External Research — primary sources

`perplexity_research` (reasoning_effort=medium) confirmed Claude Code's native OTel surface against Anthropic/Claude primary docs. Verbatim findings from the synthesis:

- Claude Code exports **"metrics including token usage by model, estimated costs in USD, session"** metrics, **"alongside structured log events covering API requests, tool executions, permission decision[s]."** Named metric example: **`claude_code.cost.usage`** ("tracks estimated USD cost").
- Enabled via **`CLAUDE_CODE_ENABLE_TELEMETRY`** with `OTEL_METRICS_EXPORTER` / `OTEL_LOGS_EXPORTER`; sensitive content is gated behind **`OTEL_LOG_USER_PROMPTS`** and **`OTEL_LOG_TOOL_DETAILS`/`OTEL_LOG_TOOL_CONTENT`** (off by default). This matches the `claude-telemetry` skill's "Non-goals" section, which deliberately leaves those off without explicit consent.
- The "agent **handwav**[ing]" framing — observing what the agent *did* via a tamper-resistant channel rather than trusting its self-report — is an established LLM-agent-observability motivation; the synthesis describes a **"channel that cannot"** be written by the agent as the ground-truth source.

Primary-source URLs returned:

1. Claude Code monitoring/usage docs — https://code.claude.com/docs/en/monitoring-usage
2. Claude Code environment variables — https://code.claude.com/docs/en/env-vars
3. Claude Code costs docs — https://code.claude.com/docs/en/costs
4. Claude Agent SDK observability — https://code.claude.com/docs/en/agent-sdk/observability
5. SigNoz — Claude Code monitoring guide — https://signoz.io/docs/claude-code-monitoring/
6. Honeycomb — "Can Claude Code observe its own code" — https://www.honeycomb.io/blog/can-claude-code-observe-its-own-code
7. ColeMurray/claude-code-otel reference stack — https://github.com/ColeMurray/claude-code-otel
8. Arize — coding agents, telemetry, and the path to self-improving software — https://arize.com/blog/closing-the-loop-coding-agents-telemetry-and-the-path-to-self-improving-software/

All accessed 2026-06-09 via Perplexity `sonar-deep-research`.

---

## Verdict

> **ALREADY-DONE** (with a small optional documentation enhancement) — **Confidence: HIGH**
>
> The exact capability the issue requests — OTEL telemetry on a channel the agent cannot write to, rendered in Grafana to verify the agent actually performed actions — is fully shipped via the `claude-telemetry` + `factory-obs` + `onboard-observability` skill trio. The operator's "AI GM" use case is a direct analog of vsdd-factory's own agent-observability story.
>
> **Recommend CLOSING #149 on GitHub as already-implemented**, with a comment pointing the filer to `/vsdd-factory:onboard-observability` (one-command setup) and the `{service_name="claude-code"}` LogQL query. The screenshot's "channel Claude can't write to" insight is exactly the design rationale already documented in `claude-telemetry/SKILL.md`.
>
> The only residual is **discoverability**, not capability: the issue suggests the filer may not have known these skills exist. If the maintainer wants zero residual, a one-line README/docs pointer ("agent-behavior verification → observability stack") would close the gap — but that is a COULD, not a blocker, and does not keep the issue open.

### Why not VALID-PARTIAL

There is no missing telemetry primitive. Token counts, tool executions, permission decisions, cost, and session events are all already exported and dashboarded. The issue does not name any specific signal that is absent. A VALID-PARTIAL verdict would require identifying a concrete gap; none exists.

---

## Recommended Approach (if maintainer wants the optional doc pointer)

| Item | Detail |
|---|---|
| **Scope** | Documentation-only. Add a discoverability pointer; do NOT build new telemetry. |
| **Key files** | `plugins/vsdd-factory/README.md` (or top-level docs index): add an "Agent-behavior verification / handwaving" bullet → `onboard-observability`. Optionally a one-paragraph "why an out-of-band channel matters" note in `claude-telemetry/SKILL.md`. |
| **Owning agent/skill** | `vsdd-factory:technical-writer` (docs-from-existing-behavior). No code, no hooks. |
| **Risks** | None — additive doc text. |
| **Dependencies** | None. The skills already work. |
| **GitHub action** | Close #149 as completed; link the three skills + the `{service_name="claude-code"}` query. |

---

## Research Methods

| Tool | Queries | Purpose |
|------|---------|---------|
| **Perplexity perplexity_research (PRIMARY)** | 1 (shared with #175) | Claude Code native OTel env vars, emitted metrics/events, agent-handwaving / tamper-resistant-channel rationale — primary-source verification |
| Read | 4 | activate, claude-telemetry, factory-obs, onboard-observability skills |
| Grep | 3 | locate observability skills; confirm no prior #149 closure in CHANGELOG; extract verbatim research findings |
| Glob | 3 | enumerate telemetry/observability skills |
| Training data | 0 areas | All OTel env-var and metric-name claims sourced externally |

**Total MCP tool calls:** 1 (research call shared across #149 + #175 grounding)
**Training data reliance:** LOW — capability claims grounded in repo files (cited with line ranges); external OTel facts verified against Claude Code primary docs.
