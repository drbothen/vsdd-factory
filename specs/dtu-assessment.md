---
document_type: dtu-assessment
level: L3
version: "1.0"
status: accepted
producer: architect
timestamp: 2026-04-25T00:00:00
phase: 1b
inputs:
  - .factory/phase-0-ingestion/pass-8-final-synthesis.md
  - .factory/phase-0-ingestion/pass-1-architecture.md
  - .factory/specs/architecture/ARCH-INDEX.md
traces_to: architecture/ARCH-INDEX.md
DTU_REQUIRED: false
---

# DTU Assessment: vsdd-factory

## Summary

| Metric | Value |
|--------|-------|
| External dependencies identified | 5 (all optional or consumer-side) |
| DTU clones recommended | 0 |
| Total clone story points | 0 |
| Estimated Wave 1 capacity needed | 0 points |

**Verdict: DTU_REQUIRED: false**

vsdd-factory is a Claude Code marketplace plugin. It has no third-party SaaS service dependencies that ship behavior contracts requiring behavioral cloning. All 10 subsystems (SS-01 through SS-10) implement first-party behavior. Every external touchpoint is either optional, consumer-side only, or a POSIX/CLI tool whose contract belongs entirely to its own project. No DTU clone development is warranted.

This assessment is final for v1.0. See the Future Re-Assessment Triggers section for conditions under which this decision should be revisited.

---

## Integration Surface Inventory (MANDATORY — all categories required)

### Inbound Data Sources (External → Product)

Systems your product reads from: APIs polled, feeds consumed, webhooks received, sensor data ingested.

None identified — rationale: vsdd-factory does not poll any external API or consume any external feed. The only structured data it receives arrives via the Claude Code harness's hook envelope (stdin JSON per hook event). That envelope is produced by Claude Code itself — the host process — not an external third-party service. The product is a passive receiver of local hook events; it does not reach out to any inbound data provider.

| # | Service | Protocol | Fidelity | DTU? | Justification |
|---|---------|----------|----------|------|---------------|
| — | — | — | — | — | None identified |

### Outbound Operations (Product → External)

Systems your product writes to or triggers: notifications, ticketing, payments, command execution.

None identified — rationale: vsdd-factory writes only to local filesystem paths (`.factory/logs/`, project working tree). The gh CLI is invoked by SS-07 hook scripts (e.g., `capture-pr-activity.sh`) for GitHub operations, but the behavior contracts of gh CLI and the GitHub API belong entirely to GitHub and the gh project — vsdd-factory is a thin invoker, not an implementor of those contracts. There is no vsdd-factory-owned behavior that depends on cloning GitHub's response shapes; any integration test can use real gh or mock at the shell level. No first-class notification, ticketing, or payment outbound path exists.

| # | Service | Protocol | Fidelity | DTU? | Justification |
|---|---------|----------|----------|------|---------------|
| 1 | GitHub (via gh CLI) | CLI subprocess | L1 (API Shape) | No | SS-07 hook scripts invoke `gh` as an OS subprocess. The behavioral contract is GitHub's and gh's, not vsdd-factory's. vsdd-factory ships no behavior logic around GitHub API responses. Integration tests can use real gh or shell mocks — no behavioral clone is required. |

### Identity & Access (Bidirectional — auth flow)

Authentication, authorization, secrets management, credential stores.

None identified — rationale: vsdd-factory has no authentication surface at the dispatcher boundary. Claude Code is the trust boundary (pass-1-architecture.md §4, NFR-SEC cross-cut). The dispatcher runs under the user's local shell session and relies entirely on the OS process model for isolation. There is no OAuth flow, no API key management, and no credential store owned by vsdd-factory. The `protect-secrets.sh` hook (SS-07) is a gate that reads environment variables already present — it does not own an auth protocol.

| # | Service | Protocol | Fidelity | DTU? | Justification |
|---|---------|----------|----------|------|---------------|
| — | — | — | — | — | None identified |

### Persistence & State (Product ↔ Storage)

External databases, caches, object stores, message queues, distributed state.

None identified — rationale: All persistence is local filesystem. The dispatcher writes `dispatcher-internal-YYYY-MM-DD.jsonl` and configured sink files to the user's project directory. There are no external databases, no cloud object stores, no message queues, and no distributed state stores. The sink-otel-grpc driver sends events to `http://localhost:4317` (the local Grafana/Loki stack documented in `tools/observability/`) — this is a locally-operated collector, not a managed third-party SaaS. The pending Datadog and Honeycomb sink drivers (S-4.2, S-4.3, Tier E) are not yet implemented; when they ship they will require re-assessment (see Future Re-Assessment Triggers).

| # | Service | Protocol | Fidelity | DTU? | Justification |
|---|---------|----------|----------|------|---------------|
| — | — | — | — | — | None identified |

### Observability & Export (Product → Monitoring)

Systems your product emits data to: logging aggregators, metrics platforms, tracing, analytics.

Two optional sinks are identified. Neither requires a DTU clone because they are consumer-side integrations: the product is a data producer, not an implementor of Datadog's or Honeycomb's ingestion behavior. The behavior contracts are those services' own, not vsdd-factory's.

*Fidelity signal: can tests run without it? Yes — both sinks are optional, soft-fail (warn-and-skip), and disabled by default.*

| # | Service | Protocol | Fidelity | DTU? | Justification |
|---|---------|----------|----------|------|---------------|
| 1 | Optional OTel collector (local Grafana/Loki stack) | OTLP/gRPC | L1 (API Shape) | No | SS-03 sink-otel-grpc sends events to localhost:4317. This is an operator-controlled local collector, not a managed SaaS. Tests use the in-tree mock OTLP receiver already present in `crates/sink-otel-grpc/tests/`. No behavioral clone needed. |
| 2 | Optional Datadog sink (not yet shipped — S-4.2, Tier E) | HTTP | — | No (deferred) | Sink driver does not exist yet; `sinks/mod.rs::from_config` warns-and-skips on unknown driver types (DRIFT-005). Re-assess when S-4.2 ships. At that point the product would be a consumer of Datadog's log ingestion endpoint — it would not own Datadog's behavior, so DTU would still be unlikely. |
| 3 | Optional Honeycomb sink (not yet shipped — S-4.3, Tier E) | HTTP | — | No (deferred) | Same rationale as Datadog. Re-assess when S-4.3 ships. |

### Enrichment & Lookup (External → Product, on-demand)

External data that augments your product's decisions but isn't the primary data source.

Two optional MCP servers are used by the SS-06 Skill Catalog for research enrichment. Both are optional: the plugin operates fully without them (skills soft-fail gracefully when MCP servers are absent). vsdd-factory ships no behavior logic that depends on the specific response shape of either service beyond what the research-agent skill documents as a soft contract.

| # | Service | Protocol | Fidelity | DTU? | Justification |
|---|---------|----------|----------|------|---------------|
| 1 | Perplexity (optional MCP server) | MCP / HTTPS | L1 (API Shape) | No | Used by the research-agent skill (SS-06) for web search enrichment. Entirely optional — plugin works without it; research skills degrade gracefully. vsdd-factory ships no behavior that depends on cloning Perplexity's search response format. The MCP server is the consumer-side seam. |
| 2 | Context7 (optional MCP server) | MCP / HTTPS | L1 (API Shape) | No | Used by the research-agent skill (SS-06) for library documentation lookup. Same rationale as Perplexity — optional, soft-fail, consumer-side only. No behavioral clone needed. |

---

## Per-Subsystem Audit (ARCH-INDEX.md Subsystem Registry)

Full audit against all 10 subsystems enumerated in ARCH-INDEX.md.

| SS-NN | Name | External Services / Dependencies | DTU Needed? | Rationale |
|-------|------|----------------------------------|-------------|-----------|
| SS-01 | Hook Dispatcher Core | Claude Code harness (stdin envelope consumer) | No | SS-01 is the implementation being onboarded — it IS vsdd-factory. Claude Code is the host process and trust boundary; vsdd-factory consumes its hook envelope as input data, not a third-party API to clone. |
| SS-02 | Hook SDK and Plugin ABI | None | No | Pure first-party Rust crates (`hook-sdk`, `hook-sdk-macros`). No external service dependency. The WASM plugin ABI is an internal contract between SS-01 and SS-02. |
| SS-03 | Observability Sinks | Optional OTel collector (local); optional Datadog + Honeycomb (not yet shipped) | No | sink-file and sink-otel-grpc ship against a local operator-controlled collector. Datadog/Honeycomb drivers are unimplemented (warn-and-skip). vsdd-factory is a producer, not an implementor of these services' ingestion behavior. |
| SS-04 | Plugin Ecosystem | None (WASM sandbox internal) | No | `legacy-bash-adapter.wasm` and `capture-commit-activity.wasm` are first-party WASM plugins. They shell out to OS subprocesses via `exec_subprocess` but do not integrate with any external SaaS. |
| SS-05 | Pipeline Orchestration | LLM provider via Claude Code (consumer) | No | The orchestrator dispatches sub-agents using Claude Code's Agent tool. The LLM provider relationship is Claude Code's concern, not vsdd-factory's. The product does not ship behavior logic that wraps an LLM API directly. |
| SS-06 | Skill Catalog | Optional Perplexity MCP server; optional Context7 MCP server | No (optional, soft-fail) | MCP servers are enrichment-only, entirely optional. Skills degrade gracefully without them. No behavioral clone is needed because vsdd-factory ships no behavior that depends on their specific response shapes. |
| SS-07 | Hook Bash Layer | git (POSIX CLI), gh (GitHub CLI), jq (POSIX CLI) | No | These are standard POSIX/CLI tools invoked as OS subprocesses. Their behavior contracts belong to their respective projects. vsdd-factory does not own or mock these contracts; hooks call them as black-box tools. Shell-level mocks (stub scripts) are sufficient for testing. |
| SS-08 | Templates and Rules | None | No | Pure markdown template files and declarative rule documents. No external service dependency of any kind. |
| SS-09 | Configuration and Activation | Claude Code (host) | No | The activate skill reads the host platform and copies a hooks.json variant. Claude Code is the host process, not an external SaaS. No third-party service involved. |
| SS-10 | CLI Tools and Bin | OS tools (rg, find, git, etc.) | No | bin tools invoke standard OS/POSIX tools as subprocesses. No SaaS dependency. Tool-level mocks (fake binaries on PATH) are sufficient for testing. |

**Result: 10/10 subsystems — DTU not required.**

---

## Dependency Summary

| # | Service | Category | Fidelity | DTU? | Points | Justification |
|---|---------|----------|----------|------|--------|---------------|
| 1 | GitHub via gh CLI | Outbound Operations | L1 | No | 0 | Consumer invocation of CLI tool; GitHub's contract, not ours |
| 2 | OTel collector (local) | Observability & Export | L1 | No | 0 | Locally operated; in-tree mock OTLP receiver sufficient |
| 3 | Datadog (not shipped) | Observability & Export | — | No (deferred) | 0 | Unimplemented; re-assess when S-4.2 ships |
| 4 | Perplexity MCP (optional) | Enrichment & Lookup | L1 | No | 0 | Optional enrichment; soft-fail; not a vsdd-factory behavior contract |
| 5 | Context7 MCP (optional) | Enrichment & Lookup | L1 | No | 0 | Optional enrichment; soft-fail; not a vsdd-factory behavior contract |

**Total DTU clones: 0. Total points: 0.**

---

## Services NOT Requiring DTU

| # | Service | Reason |
|---|---------|--------|
| 1 | GitHub via gh CLI | CLI subprocess seam; GitHub's contract, not vsdd-factory's; shell mocks sufficient |
| 2 | OTel collector (local Grafana/Loki) | Operator-controlled local process; in-tree mock OTLP receiver already exists in crate tests |
| 3 | Datadog sink (not yet shipped) | Unimplemented; warn-and-skip; re-assess at S-4.2 |
| 4 | Honeycomb sink (not yet shipped) | Unimplemented; warn-and-skip; re-assess at S-4.3 |
| 5 | Perplexity MCP server | Optional; soft-fail; enrichment-only; no vsdd-factory behavior depends on response shape |
| 6 | Context7 MCP server | Optional; soft-fail; enrichment-only; no vsdd-factory behavior depends on response shape |
| 7 | Claude Code host harness | vsdd-factory IS the plugin; Claude Code is the host; not a third-party service to clone |

---

## DTU Architecture

Not applicable — DTU_REQUIRED: false. No Docker Compose structure, no environment variable overrides, and no clone development is planned.

---

## Clone Development Approach

Not applicable — no DTU clones are recommended for vsdd-factory v1.0.

---

## Future Re-Assessment Triggers

DTU re-assessment becomes appropriate if vsdd-factory ever introduces any of the following conditions. None apply at v1.0.

| # | Trigger | Example Scenario |
|---|---------|-----------------|
| 1 | First-class integration with a third-party API where vsdd-factory ships owned behavior | A custom Slack bot skill that encodes Slack's event schema and block kit response format as a behavioral contract |
| 2 | A required (non-optional) third-party SaaS dependency | Requiring a specific cloud LLM provider's API directly (bypassing Claude Code's abstraction) |
| 3 | A migration tool whose correctness depends on understanding a specific tool's output format | A Linear or Jira migration skill whose acceptance criteria are defined against Live API response shapes |
| 4 | Datadog sink ships (S-4.2) and integration tests are needed against Datadog's log ingestion endpoint | sink-datadog behavior depends on Datadog's ingest API response codes and retry semantics |
| 5 | Honeycomb sink ships (S-4.3), same rationale as Datadog | sink-honeycomb behavior depends on Honeycomb's Events API |

**At v1.0, triggers 1–5 are all false.** Re-assess at rc.1 when Tier E stories (S-4.1, S-4.2, S-4.3) are in scope.

---

## Decision Log

| Date | Decision | Rationale |
|------|----------|-----------|
| 2026-04-25 | DTU_REQUIRED: false for vsdd-factory v1.0 | Phase 0 gate Q3 confirmed by human (pass-8-final-synthesis.md §13 item 3: "probably no DTU assessment needed for vsdd-factory itself — verify no external service dependency requires it"). pass-1-architecture.md confirms no external SaaS integrations. All 10 ARCH-INDEX subsystems audited; zero subsystems require behavioral cloning of third-party services. All external touchpoints are optional, consumer-side, or POSIX/CLI tools whose contracts belong to their respective projects. |

---

```
PHASE_1_6_A_DTU_ASSESSMENT: COMPLETE
  Verdict: DTU_REQUIRED: false
  File: /Users/jmagady/Dev/vsdd-factory/.factory/specs/dtu-assessment.md
  Subsystems audited: 10
  External integrations identified: 5 (all optional or consumer-side)
  Future re-assessment triggers: 5
```
