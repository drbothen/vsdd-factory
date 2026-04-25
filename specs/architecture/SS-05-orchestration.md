---
document_type: architecture-section
level: L3
section: "SS-05-orchestration"
version: "1.0"
status: accepted
producer: architect
timestamp: 2026-04-25T00:00:00
phase: 1.2
inputs:
  - .factory/specs/architecture/ARCH-INDEX.md
  - .factory/phase-0-ingestion/pass-1-architecture.md
  - .factory/phase-0-ingestion/pass-8-final-synthesis.md
traces_to: ARCH-INDEX.md
---

# SS-05: Pipeline Orchestration

## [Section Content]

## Purpose

The Pipeline Orchestration subsystem is the executive control plane of Subsystem B.
It contains the orchestrator agent, 34 specialist sub-agents, and 16 Lobster
workflow definitions (8 mode-level + 8 phase-level). The orchestrator reads
`.lobster` workflow files as structured data, resolves step dependencies, and
dispatches specialist sub-agents via Claude Code's Agent tool. It does not produce
artifacts directly — it delegates all artifact production to sub-agents.

Lobster workflows are YAML data files declaring named steps with `agent`,
`depends_on`, `on_failure`, `max_retries`, `timeout`, and optional `cost_tracking`
fields. This declarative DAG format makes the pipeline introspectable and
modifiable without touching agent prose. The orchestrator is set as the default
agent after `/vsdd-factory:activate`, replacing Claude Code's default assistant
for the lifetime of the session.

Agents are markdown persona-files with frontmatter declaring `name`, `description`,
and optional `tools`, `model`, `color`. Each encodes a specialist role in the VSDD
methodology: architect, business-analyst, adversary, implementer, pr-manager,
codebase-analyzer, spec-steward, consistency-validator, formal-verifier, and
others. The orchestrator spawns them as sub-agents via the Agent tool, passing
context from STATE.md and the current workflow step.

## Modules

| Module / File | Responsibility |
|---|---|
| `plugins/vsdd-factory/agents/orchestrator/orchestrator.md` | Main-thread agent; reads `.lobster` workflow; dispatches sub-agents; does NOT produce artifacts |
| `plugins/vsdd-factory/agents/*.md` (33 files) | Specialist sub-personas; spawned by orchestrator; each encodes a VSDD role |
| `plugins/vsdd-factory/workflows/greenfield.lobster` | Greenfield product workflow (Phase 0–7 full run) |
| `plugins/vsdd-factory/workflows/brownfield.lobster` | Brownfield ingest workflow (Phase 0 deep analysis) |
| `plugins/vsdd-factory/workflows/feature.lobster` | Feature cycle workflow (F1–F7 delta loop) |
| `plugins/vsdd-factory/workflows/maintenance.lobster` | Maintenance and debt-paydown workflow |
| `plugins/vsdd-factory/workflows/discovery.lobster` | Discovery and planning workflow |
| `plugins/vsdd-factory/workflows/planning.lobster` | Roadmap and story decomposition workflow |
| `plugins/vsdd-factory/workflows/multi-repo.lobster` | Multi-repo coordination workflow |
| `plugins/vsdd-factory/workflows/code-delivery.lobster` | Direct code delivery workflow (no spec overhead) |
| `plugins/vsdd-factory/workflows/phases/phase-*.lobster` (8 files) | Per-phase sub-flows invoked by mode-level workflows |

## Public Interface

The orchestration layer's public interface is the set of `/vsdd-factory:*` slash
commands that launch workflows, plus the `.factory/STATE.md` file that carries
pipeline state between sessions.

**Workflow invocation:**
- `/vsdd-factory:activate` — set orchestrator as default agent + wire dispatcher.
- `/vsdd-factory:run-phase <N>` — run a specific VSDD phase sub-flow.
- Mode-level commands: `/vsdd-factory:brownfield-ingest`, etc. — launch the
  corresponding `.lobster` mode workflow.

**Lobster step schema (per step):**
```yaml
steps:
  - name: "architect-design"
    type: agent
    agent: architect
    depends_on: [domain-spec]
    on_failure: retry
    max_retries: 2
    timeout: 600
```

**State contract:** Orchestrator reads `.factory/STATE.md` at startup to determine
current phase, open wave gates, and pending story queue. Sub-agents write back to
`.factory/` via wave-gate hooks gated by SS-07 bash hooks.

**Agent frontmatter schema:**
```yaml
name: architect
description: "System architect for VSDD; owns architecture section files."
tools: [read, write, edit, bash]
model: claude-opus-4-5
color: blue
```

## Internal Structure

The orchestration layer is entirely declarative prose and YAML data — no compiled
code. Control flow:

1. User invokes `/vsdd-factory:activate` → orchestrator set as default agent.
2. User selects a mode → orchestrator reads the corresponding `.lobster` file.
3. Orchestrator resolves `depends_on` DAG → determines executable steps.
4. For each ready step: orchestrator calls Agent tool with the step's `agent` and
   constructed context payload.
5. Sub-agent executes, writes artifacts, returns summary.
6. Orchestrator reads sub-agent output; marks step complete; resolves next ready
   steps; repeats.
7. Wave-gate hooks (SS-07) gate artifact writes; `wave-state` bin tool (SS-10)
   records gate outcomes to STATE.md.

The `on_failure` field controls retry behavior: `retry` (up to `max_retries`),
`skip` (continue pipeline), `abort` (halt the workflow). Cost tracking fields
accumulate token estimates per step for reporting by `factory-dashboard`.

Agent specializations relevant to orchestration (pass-1-architecture.md, lines
51-54): orchestrator (main-thread), adversary (adversarial review with SHA-currency
gate per ADR-013), architect, business-analyst, codebase-analyzer, implementer,
pr-manager, spec-steward, consistency-validator, formal-verifier, test-writer,
story-writer, product-owner, security-reviewer, dtu-validator.

## Dependencies

**Incoming (consumers of SS-05):**
- User / Claude Code session — invokes workflows via slash commands.
- SS-10 (CLI Tools and Bin) — `wave-state` and `lobster-parse` bin tools are
  consumed by the orchestrator and sub-agents to read/write pipeline state.

**Outgoing (SS-05 depends on):**
- SS-06 (Skill Catalog) — orchestrator dispatches sub-agents who invoke skills;
  skills are the atomic units of artifact production.
- SS-07 (Hook Bash Layer) — wave-gate hooks gate sub-agent file writes;
  `handoff-validator` and `pr-manager-completion-guard` are SubagentStop hooks.
- SS-08 (Templates and Rules) — sub-agents render output using templates from the
  template catalog.

## Cross-Cutting

- **Adversarial review (ADR-013):** The adversary agent applies SHA-currency gating
  to every review: it refuses to review artifacts whose source SHA is stale relative
  to the current HEAD. This is opt-in at the project level via
  `templates/verify-sha-currency.sh` (DRIFT-009).
- **Wave-gate bookkeeping:** `warn-pending-wave-gate.sh` (Stop hook) warns if a
  wave gate is open at session end. `update-wave-state-on-merge.sh` (SubagentStop)
  advances wave state after PR merge. Both are in SS-07.
- **State isolation:** Each sub-agent runs in its own Agent tool call with its own
  context window. The orchestrator passes only the minimal context needed per step.
- **Error handling:** Workflow `on_failure` semantics are the primary error
  handling mechanism. Sub-agent crashes surface as Agent tool errors; the
  orchestrator applies the step's `on_failure` policy.
- **Cost visibility:** `cost_tracking: true` on steps enables token-cost
  accumulation in STATE.md, surfaced via `factory-dashboard`.

## Behavioral Contracts

BC shard directory: `.factory/specs/behavioral-contracts/ss-05/`
(target prefix BC-5; ~616 BCs across agents + workflows).

High-level BC groupings: orchestrator dispatch and DAG resolution
(BC-5.001–BC-5.050), Lobster workflow schema invariants (BC-5.051–BC-5.100),
per-agent persona contracts — architect through test-writer
(BC-5.101–BC-5.400), phase sub-flow sequencing (BC-5.401–BC-5.500), wave-gate
and STATE.md write contracts (BC-5.501–BC-5.616).

## ADRs

- ADR-013: Cycle-keyed adversarial review structure — `decisions/ADR-013-adversarial-review-structure.md`

## Drift / Known Issues

- **DRIFT-009 (P2 — low):** Adversary SHA-currency gate is opt-in. `templates/verify-sha-currency.sh`
  exists but `hooks/verify-sha-currency.sh` does not. Projects that have not
  copied the template get no SHA-currency enforcement. Documented in CHANGELOG
  v1.0.0-beta.4; acceptable for 1.0 if opt-in nature is clearly documented.
- **DRIFT-006 (P2 — low):** SessionStart, SessionEnd, SubagentStop events are
  registered in `hooks.json.template` but no plugin-layer hooks react to
  `session.started` / `session.ended`. S-5.1, S-5.2 not shipped (Tier G).
- **S-5.1, S-5.2, S-5.3, S-5.4 NOT SHIPPED:** Session lifecycle and new hook
  event wiring (Tier G, pending Tier E completion).
