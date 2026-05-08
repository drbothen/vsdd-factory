---
document_type: architecture-index
level: L3
version: "1.15"
status: accepted
producer: architect
timestamp: 2026-05-07T00:00:00
phase: F2
inputs:
  - .factory/phase-0-ingestion/pass-8-final-synthesis.md
  - .factory/phase-0-ingestion/pass-1-architecture.md
  - .factory/legacy-design-docs/2026-04-24-v1.0-factory-plugin-kit-design.md
  - .factory/phase-0-ingestion/pass-0-inventory.md
  - .factory/phase-0-ingestion/pass-2-domain-model.md
  - .factory/phase-0-ingestion/pass-5-conventions.md
traces_to: phase-1-spec-crystallization
deployment_topology: single-service
changelog:
  - date: 2026-05-07
    change: "F2 pass-4 fix burst close (2026-05-07): ADR-019 v1.5→v1.6 (§Consequences drain-window line 215 symbolic ASYNC_DRAIN_WINDOW_MS; F-P4-004). ARCH-INDEX v1.14→v1.15. last_amended: 2026-05-07 (async-semantics F2 pass-4 fix burst: ADR-019 v1.6; F-P4-001 BC-INDEX re-tally confirmed in ARCH-INDEX)."
  - date: 2026-05-07
    change: "F2 pass-3 fix burst close + user-correction (2026-05-07): DI-NN placeholder resolved to DI-019 in ADR-019 §Consequences and VP-079 (all references). ADR-019 v1.5 confirmed at close. SS-09-config-activation.md v1.2 (stale text replaced in-place). SS-07-hook-bash.md v1.2 (schema_version 1→2 in-place). last_amended: 2026-05-07 (async-semantics F2 pass-3 fix burst close: DI-019 placeholder resolved; SS-09 v1.2; SS-07 v1.2; ADR-019 v1.5)."
  - date: 2026-05-07
    change: "F2 pass-3 fix burst revision (2026-05-07): Subsystem Registry BC counts re-tallied to authoritative frontmatter subsystem (not directory). BC-7.06.001 (ss-07/ dir, frontmatter SS-01): SS-07 197→196, SS-01 116→117. BC-8.29.001/002/003 + BC-8.30.002 (ss-08/ dir, frontmatter SS-05): SS-08 218→214, SS-05 648→652. Total BCs unchanged at 1,947. Obsolete directory-based footnote removed. ADR-019 v1.4→v1.5 (drain window cites DI-019 as canonical constant source). VP-079 v1.2→v1.3 (Property 5 and drain-window references cite DI-019 as canonical source; DI-019 added to Traceability)."
  - date: 2026-05-07
    change: "F2 pass-2 fix burst close (2026-05-07): ADR-019 v1.2→v1.3 (sync/async classification rationale added to §Consequences for warn-pending-wave-gate/regression-gate SYNC deliberate; track-agent-start/stop/session-learning added to async list; BC-7.06.001 Invariant 6 cross-reference added). last_amended: 2026-05-07 (async-semantics F2 pass-2 fix burst: ADR-019 v1.3 §Consequences sync)."
  - date: 2026-05-07
    change: "F2 pass-1 fix burst (2026-05-07): ADR-019 v1.0→v1.1 (placeholder BC IDs resolved; forward ref BC-9.01.006 added to SS-09 subsystem assignments). SS-09-config-activation.md v1.0→v1.1 (async:true refs removed; schema_version 1→2). SS-07-hook-bash.md v1.0→v1.1 (schema_version 1→2). last_amended: 2026-05-07 (F2 pass-1 fix burst: 3 arch doc amendments, BC-9.01.006 forward ref resolved in ADR-019)."
  - date: 2026-05-07
    change: "F2 async-semantics (2026-05-07): +1 ADR (ADR-019 Plugin Async Semantics at Registry Layer — SS-01, SS-07, SS-09 affected). Supersedes: null. ADR-019 is next-available after ADR-018. SS-01 BC count 115→116 (+BC-1.14.001 dispatcher partition contract). SS-07 BC count 196→197 (+BC-7.06.001 registry schema v2 + CI lint). Total BCs 1943→1945."
  - date: 2026-05-07
    change: "D-362 F2-amendment WASM-plugin Context Resolver platform: +1 ADR (ADR-018 WASM-plugin Context Resolvers — Design and Layering — SS-01, SS-04). SS-01 BC count 114→115 (+BC-1.13.001 resolver-registry loading + pre-dispatch context injection); SS-04 BC count 34→39 (+BC-4.12.001-005 resolver lifecycle/ABI/capabilities/error-isolation/merging). Total BCs 1937→1943. ADR-018 is next-available after ADR-017."
  - date: 2026-05-07
    change: "D-340 F2 spec evolution for engine discipline pass-1: +2 ADRs (ADR-016 artifact path registry SoT — SS-04/SS-06; ADR-017 per-story adversary three-perimeter model — SS-04/SS-05). SS-04 BC count 31→34 (+BC-4.10.001/002 + BC-4.11.001); SS-05 BC count 646→648 (+BC-5.39.001/002); SS-06 BC count 585→586 (+BC-6.22.001). Total BCs 1931→1937. ADR-016 and ADR-017 are next-available after ADR-015."
  - date: 2026-05-06
    change: "D-336 E-10 pass-8 fix-cycle (architect): version bump v1.6 → v1.7. F-2 fix: Cross-Cutting Concerns line 152 — `dispatcher_trace_id` → `trace_id` per DI-017 v1.1 / ADR-015 v1.7 canonicalization. F-3 fix: Schema versioning row — differentiated per-config (hooks-registry=1, observability-config=2 per ADR-015 D-15.1); added SS-03 to subsystem list; cited BC-3.05.004 PC4 migration hint and DI-014. last_updated: 2026-05-06."
  - date: 2026-05-06
    change: "D-333 E-10 pass-6 fix-cycle seal: version bump v1.5 → v1.6. F-1 fix: renumbering-history paragraph (line 96) D-15.4 → D-15.1 — same-document sibling-paragraph drift from D-331's primary fix at SS-03 row (line 85) now fully closed. No BC subsystem count changes."
  - date: 2026-05-06
    change: "D-331 E-10 pass-5 fix-cycle seal: version bump v1.4 → v1.5. F-2 propagation: BC-3.05.004 narrative in Subsystem Registry SS-03 row updated D-15.4 → D-15.1 (OQ-1 resolved in SS-03-event-emission.md). No BC subsystem count changes."
  - date: 2026-05-06
    change: "D-327 E-10 ↔ rc.12 format-alignment seal: version bump v1.3 → v1.4. No BC subsystem count changes (4 BCs amended, not added). Courtesy bump to record rc.12 alignment cycle sealed (audit 119e70e → D-326 7afc64d → D-327). Engine baseline: v1.0.0-rc.12 (4cf59bc on develop)."
  - date: 2026-05-06
    change: "D-324 E-10 pass-3 fix burst seal: version bump v1.2 → v1.3. No substantive ARCH-INDEX changes this burst; BC subsystem counts unchanged (no new BCs in D-322). Courtesy bump to record D-324 seal."
  - date: 2026-05-06
    change: "D-321 E-10 pass-2 fix burst seal: version bump v1.1 → v1.2. SS-02 BC count 25→26 (+BC-2.06.001); SS-04 BC count 30→31 (+BC-4.09.001). Total BCs 1929→1931. See cycles/v1.0-brownfield-backfill/E-10-pass-2.md."
---

# Architecture Index: vsdd-factory

> **Context Engineering:** This is a lightweight index (~350 tokens). Agents load
> ONLY the section files they need. See the Document Map for per-section consumer
> guidance.

## Document Map

| Section | File | Primary Consumer | Purpose |
|---------|------|-----------------|---------|
| SS-01 Hook Dispatcher Core | SS-01-hook-dispatcher.md | implementer, story-writer | Module catalog, host fns, sandbox model, Rust crates |
| SS-02 Hook SDK and Plugin ABI | SS-02-hook-sdk.md | implementer, plugin authors | SDK API, manifest schema, capability declarations |
| SS-03 Event Emission (OTel-Aligned) | SS-03-event-emission.md | implementer, story-writer | Single-stream FileSink, OTel schema, host enrichment, write-failure cascade. Old file SS-03-observability-sinks.md is SUPERSEDED (see ADR-015). |
| SS-04 Plugin Ecosystem | SS-04-plugin-ecosystem.md | implementer, story-writer | legacy-bash-adapter, capture-commit-activity, Tier E/F lifecycle plugin crates |
| SS-05 Pipeline Orchestration | SS-05-orchestration.md | orchestrator, story-writer | Agents, Lobster workflows, pipeline phase structure |
| SS-06 Skill Catalog | SS-06-skill-catalog.md | story-writer, skill authors | 119 skills, SKILL.md contract, output routing |
| SS-07 Hook Bash Layer | SS-07-hook-bash.md | implementer, formal-verifier | 44 bash hooks, registry bindings, exit code semantics |
| SS-08 Templates and Rules | SS-08-templates-rules.md | story-writer, implementer | 108 templates, 9 rules, template compliance contracts |
| SS-09 Configuration and Activation | SS-09-config-activation.md | implementer, story-writer | hooks.json variants, activation skill, CI platform config |
| SS-10 CLI Tools and Bin | SS-10-cli-tools.md | implementer, story-writer | 12 bin tools, 110 slash-command bindings |

### Future Sections (Deferred)

The following cross-cutting documents were planned but are currently deferred.
The per-subsystem SS-NN files above collectively cover their content:

| Deferred File | Covered By |
|---------------|-----------|
| system-overview.md | ARCH-INDEX §Subsystem Registry + §Component Dependency Map |
| module-decomposition.md | SS-01..SS-10 section files + §Subsystem Registry |
| dependency-graph.md | §Component Dependency Map (Mermaid) in this file |
| api-surface.md | SS-01-hook-dispatcher.md, SS-02-hook-sdk.md |
| verification-architecture.md | VP-INDEX.md + SS-07-hook-bash.md |
| purity-boundary-map.md | SS-01..SS-04 section files (purity notes per module) |
| tooling-selection.md | VP-INDEX §Kani Upgrade Candidates + §Property-Test Upgrade Candidates |
| verification-coverage-matrix.md | VP-INDEX §Full Index (scope column = module mapping) |

## Cross-References

| If you need... | Read these together |
|----------------|-------------------|
| BC renumbering mapping | Subsystem Registry below (SS-NN → BC-S range) |
| Implementation plan for a module | SS-NN section file for that subsystem + §Subsystem Registry |
| Verification plan for a module | VP-INDEX.md + SS-07-hook-bash.md (for hook VPs) |
| Story decomposition input | SS-NN section files for relevant subsystems + §Component Dependency Map |

## Subsystem Registry

> **Source of truth** for subsystem names and IDs. BC frontmatter `subsystem:`,
> BC-INDEX subsystem column, story `subsystems:` fields, and PRD subsystem
> references MUST all use the exact Name from this table.

BC counts are shown by **authoritative subsystem** (BC frontmatter `subsystem:` field), not by directory location.

| SS-ID | Name | Section File | Implementing Modules / Folders | BC-S Prefix | BCs | Phase |
|-------|------|--------------|-------------------------------|-------------|-----|-------|
| SS-01 | Hook Dispatcher Core | SS-01-hook-dispatcher.md | `crates/factory-dispatcher/src/{main,registry,routing,executor,invoke,engine,plugin_loader,payload}.rs` | BC-1 | 117 (116 by directory + 1 reanchored: BC-7.06.001 frontmatter subsystem=SS-01 per F-P1-006) | Phase 1 |
| SS-02 | Hook SDK and Plugin ABI | SS-02-hook-sdk.md | `crates/hook-sdk/`, `crates/hook-sdk-macros/` | BC-2 | 26 (includes 2 D-183 BCs: BC-2.02.011 host::write_file, BC-2.02.012 HookPayload SubagentStop fields; +1 D-219 BC-2.02.013 host::run_subprocess — WITHDRAWN D-224; +1 D-321 BC-2.06.001 SDK semver bump) | Phase 1 |
| SS-03 | Event Emission (OTel-Aligned) | SS-03-event-emission.md | `crates/sink-core/` (FileSink; Router/SinkRegistry deprecated), `crates/sink-file/`, `crates/factory-dispatcher/src/{host/emit_event,internal_log,sinks}.rs` (`sink-otel-grpc/` deprecated Wave 1) | BC-3 | 53 (51 prior + 1 Phase 1b BC-3.05.004 v2 schema validation per ADR-015 D-15.1 (OQ-1 resolved in SS-03-event-emission.md) + 1 F2 pass-1 fix burst BC-3.08.001 async-semantics event catalog) | Phase 1 |
| SS-04 | Plugin Ecosystem | SS-04-plugin-ecosystem.md | `crates/hook-plugins/legacy-bash-adapter/`, `crates/hook-plugins/capture-commit-activity/`, `crates/hook-plugins/capture-pr-activity/` [PLANNED S-3.02], `crates/hook-plugins/block-ai-attribution/` [PLANNED S-3.03], `crates/hook-plugins/session-start-telemetry/` [PLANNED S-5.01], `crates/hook-plugins/session-end-telemetry/` [PLANNED S-5.02], `crates/hook-plugins/worktree-hooks/` [PLANNED S-5.03], `crates/hook-plugins/tool-failure-hooks/` [PLANNED S-5.04], `crates/hook-plugins/validate-per-story-adversary-convergence/` [PLANNED], `crates/hook-plugins/validate-artifact-path/` [PLANNED] | BC-4 | 39 (+1 D-321 BC-4.09.001; +3 D-340 BC-4.10.001/002 + BC-4.11.001; +5 D-362 BC-4.12.001-005 resolver platform) | Phase 1 |
| SS-05 | Pipeline Orchestration | SS-05-orchestration.md | `plugins/vsdd-factory/agents/`, `plugins/vsdd-factory/workflows/*.lobster`, `plugins/vsdd-factory/workflows/phases/` | BC-5 | 652 (648 by directory + 4 reanchored: BC-8.29.001/002/003 + BC-8.30.002 frontmatter subsystem=SS-05; files remain in ss-08/ per POLICY 1) | Phase 1 |
| SS-06 | Skill Catalog | SS-06-skill-catalog.md | `plugins/vsdd-factory/skills/` (119 skills, 581 markdown files) | BC-6 | 586 (+1 D-340 BC-6.22.001 relocate-artifact skill) | Phase 1 |
| SS-07 | Hook Bash Layer | SS-07-hook-bash.md | `plugins/vsdd-factory/hooks/*.sh` (44 scripts), `plugins/vsdd-factory/hooks-registry.toml` | BC-7 | 196 (197 by directory − 1 reanchored: BC-7.06.001 is in ss-07/ but frontmatter subsystem=SS-01 per F-P1-006) | Phase 1 |
| SS-08 | Templates and Rules | SS-08-templates-rules.md | `plugins/vsdd-factory/templates/` (108 files), `plugins/vsdd-factory/rules/` (9 files) | BC-8 | 214 (218 by directory − 4 reanchored: BC-8.29.001/002/003 + BC-8.30.002 frontmatter subsystem=SS-05; files remain in ss-08/ per POLICY 1) | Phase 1 |
| SS-09 | Configuration and Activation | SS-09-config-activation.md | `plugins/vsdd-factory/hooks/hooks.json*`, `plugins/vsdd-factory/.claude-plugin/plugin.json`, `ci/platforms.yaml`, `scripts/generate-registry-from-hooks-json.sh` | BC-9 | 6 (+1 F2 pass-1 fix burst BC-9.01.006 envelope-sync invariant) | Phase 1 |
| SS-10 | CLI Tools and Bin | SS-10-cli-tools.md | `plugins/vsdd-factory/bin/` (12 tools), `plugins/vsdd-factory/commands/` (110 files), `scripts/` | BC-10 | 58 | Phase 1 |

**Total BCs: 1,947 (per BC-INDEX v1.22; counts above are by authoritative frontmatter subsystem).** Cross-subsystem file placements (POLICY 1 append-only): BC-7.06.001 in ss-07/ → SS-01 (F-P1-006 reanchor); BC-8.29.001/002/003 + BC-8.30.002 in ss-08/ → SS-05 (historical allocation). The total is invariant under both directory-based and frontmatter-based tallying.

**Renumbering history — BC-1.12.008 → BC-3.05.004 (D-311/D-312):** BC-1.12.008 was originally proposed as an SS-01 routing target in D-311; renumbered to BC-3.05.004 (SS-03) in D-312 corrigendum per POLICY 1 ID-collision rule (BC-3.05.001/002/003 already existed as brownfield BCs authored by codebase-analyzer on 2026-04-25; BC-3.05.004 was the next free slot). Consequence: SS-01 has +4 Phase 1a additions (BC-1.12.001–BC-1.12.004) and +4 Phase 1b additions (BC-1.12.005/006/007/009; no BC-1.12.008 ID exists). SS-03 has +1 Phase 1b addition (BC-3.05.004 v2 schema validation per ADR-015 D-15.1). OQ-W16-012 filed-and-resolved in D-312.

### Subsystem Registry Design Notes

The 10-subsystem layout reflects two first-class groups:

- **SS-01 through SS-04** are **Subsystem A** (Rust compiled artifacts). They share one Cargo workspace, one release binary per platform, and the WASM plugin ABI contract.
- **SS-05 through SS-10** are **Subsystem B** (VSDD orchestration framework). They share one plugin manifest, one marketplace entry, and the Lobster workflow format.

**Deviations from the suggested 10-subsystem template:**

- SS-05 was renamed from "Pipeline Orchestration" to "Pipeline Orchestration" — kept, but agents were moved in (34 agents belong here alongside workflows because agents are the steps' executors; splitting them would orphan the orchestrator).
- SS-06 "Skill Catalog" is the largest single BC surface (585 BCs as of v1.0.0-beta.6; see Subsystem Registry table for current count). It is intentionally a standalone subsystem because skills have independent behavioral contracts per skill (each SKILL.md is a discrete unit of behavior).
- SS-09 "Configuration and Activation" is narrower than the suggested name — it covers only the activation plumbing and CI variant generation, not general config. `hooks-registry.toml` routing lives in SS-07 (Hook Bash Layer) because it is the routing table for that layer.
- SS-10 merges "CLI Tools" and "Bin" because all 12 bin tools are CLI-invocable and the commands/ slash-command bindings are just thin wrappers around skills.

## Component Dependency Map

```mermaid
graph TD
    CC["Claude Code (harness)"]
    SS09["SS-09: Configuration and Activation"]
    SS01["SS-01: Hook Dispatcher Core"]
    SS02["SS-02: Hook SDK and Plugin ABI"]
    SS03["SS-03: Event Emission (OTel-Aligned)"]
    SS04["SS-04: Plugin Ecosystem"]
    SS05["SS-05: Pipeline Orchestration"]
    SS06["SS-06: Skill Catalog"]
    SS07["SS-07: Hook Bash Layer"]
    SS08["SS-08: Templates and Rules"]
    SS10["SS-10: CLI Tools and Bin"]

    CC -->|"reads hooks.json"| SS09
    SS09 -->|"generated variants wire to"| SS01
    SS01 -->|"loads + executes WASM via"| SS02
    SS01 -->|"writes all events to (single-stream)"| SS03
    SS01 -->|"invokes plugins from"| SS04
    SS04 -->|"shells out via exec_subprocess to"| SS07
    SS05 -->|"dispatches sub-agents who call"| SS06
    SS05 -->|"gate writes via"| SS07
    SS06 -->|"renders output from"| SS08
    SS06 -->|"emits events via"| SS10
    SS10 -->|"reads/writes state in"| SS05
```

**Strict dependency direction:** data flows down (CC → SS09 → SS01 → SS02/SS03/SS04 → SS07). The orchestration stack (SS-05/06/07/08/10) is a separate plane. SS-07 sits at the intersection: bash hooks are invoked by both SS-04 (via legacy-bash-adapter) and SS-05/SS-06 (as gates on tool use).

## Cross-Cutting Concerns

| Concern | Owner | Mechanism |
|---------|-------|-----------|
| Observability (single-stream) | SS-03 | `events-YYYY-MM-DD.jsonl` — all lifecycle + domain events (ADR-015). Debug file `dispatcher-internal-*.jsonl` opt-in via `VSDD_DEBUG_LOG=1`; ADR-007 amended. |
| Capability enforcement | SS-01 | Deny-by-default; cap-gated host fns emit denial event + return code |
| Schema versioning | SS-01, SS-09, SS-03 | per-config: `hooks-registry.toml` schema_version=2 (post-ADR-019; v1→v2 hard-error, no compat shim); `observability-config.toml` schema_version=2 (post-ADR-015 D-15.1; v1→v2 hard-errors with migration hint per BC-3.05.004 PC4); other TOML configs schema_version=1; mismatch = hard error per DI-014 |
| Trace correlation | SS-01 | `trace_id` (UUID v4) propagated on every emitted event (renamed from `dispatcher_trace_id` per DI-017 / ADR-015 v1.7) |
| Platform selection | SS-09 | Activation skill copies `hooks.json.<platform>` (ADR-009) |
| Error non-blocking | SS-01 | Registry/payload/engine errors → `internal.dispatcher_error` → exit 0 |
| Bash hook compatibility | SS-04 | `legacy-bash-adapter.wasm` via `exec_subprocess` (ADR-012) |
| Secrets protection | SS-07 | `protect-secrets.sh` PreToolUse gate + env_allow deny-by-default |

## Architecture Decisions

| ID | Decision Summary | Subsystems | File |
|----|-----------------|------------|------|
| ADR-001 | Compiled Rust dispatcher per platform | SS-01, SS-09 | decisions/ADR-001-rust-dispatcher.md |
| ADR-002 | WASM (wasmtime) plugin ABI | SS-01, SS-02, SS-04 | decisions/ADR-002-wasm-plugin-abi.md |
| ADR-003 | WASI preview 1 for v1.0; preview 2 deferred | SS-02, SS-04 | decisions/ADR-003-wasi-preview1.md |
| ADR-004 | TOML for all configuration files | SS-01, SS-09 | decisions/ADR-004-toml-config.md |
| ADR-005 | Multi-sink observability natively in dispatcher — **SUPERSEDED by [ADR-015](decisions/ADR-015-single-stream-otel-schema.md)** | SS-01, SS-03 | decisions/ADR-005-multi-sink-observability.md |
| ADR-006 | HOST_ABI_VERSION as separate semver constant | SS-01, SS-02 | decisions/ADR-006-host-abi-version.md |
| ADR-007 | Always-on dispatcher self-telemetry — **AMENDED by [ADR-015](decisions/ADR-015-single-stream-otel-schema.md)** | SS-01, SS-03 | decisions/ADR-007-always-on-telemetry.md |
| ADR-008 | Parallel-within-tier, sequential-between-tier execution | SS-01 | decisions/ADR-008-parallel-within-tier.md |
| ADR-009 | Activation-skill-driven platform binary selection | SS-09 | decisions/ADR-009-activation-platform-selection.md |
| ADR-010 | StoreData-typed linker for host functions (invoke.rs pattern) | SS-01, SS-02 | decisions/ADR-010-storedata-linker.md |
| ADR-011 | Dual hooks.json + hooks-registry.toml during migration | SS-07, SS-09 | decisions/ADR-011-dual-hook-routing-tables.md |
| ADR-012 | Legacy-bash-adapter as universal current router | SS-04, SS-07 | decisions/ADR-012-legacy-bash-adapter-router.md |
| ADR-013 | Cycle-keyed adversarial review structure | SS-05, SS-06 | decisions/ADR-013-adversarial-review-structure.md |
| ADR-014 | Tier-2 native WASM migration (latency-primary gate + bundle-size advisory + 30MB hard kill-switch) | SS-02, SS-04, SS-07 | decisions/ADR-014-tier-2-native-wasm-migration.md |
| ADR-015 | Single-stream OTel schema + producer-side enrichment — **ACCEPTED 2026-05-04; supersedes ADR-005, amends ADR-007** | SS-01, SS-03 | decisions/ADR-015-single-stream-otel-schema.md |
| ADR-016 | Artifact path registry as single source of truth for `.factory/` canonical paths — **ACCEPTED 2026-05-07; D-340 F2** | SS-04, SS-06 | decisions/ADR-016-artifact-path-registry-sot.md |
| ADR-017 | Per-story adversarial convergence gate — three-perimeter model and WASM hook phasing — **ACCEPTED 2026-05-07; D-340 F2** | SS-04, SS-05 | decisions/ADR-017-per-story-adversary-phasing.md |
| ADR-018 | WASM-plugin Context Resolvers — design and layering for factory-agnostic runtime context injection via sandboxed WASM-plugin resolvers — **ACCEPTED 2026-05-07; D-362 F2-amendment** | SS-01, SS-04 | decisions/ADR-018-wasm-plugin-context-resolvers.md |
| ADR-019 | Plugin Async Semantics Belong at the Registry Layer — hard cut to registry-layer `async: bool` per-plugin field; envelope uniformly synchronous; dispatcher partition (sync_group/async_group); CI lint `on_error=block ⇒ async=false` — **ACCEPTED 2026-05-07; F2 async-semantics; v1.6 (F2 pass-4 fix burst close: §Consequences drain-window line 215 `+ 100ms` → `+ ASYNC_DRAIN_WINDOW_MS` symbolic constant; F-P4-004 closed)** | SS-01, SS-07, SS-09 | decisions/ADR-019-plugin-async-semantics-at-registry-layer.md |

## Phase 1.4 BC Renumbering Map

This table enables downstream Phase 1.4 migration from `BC-AUDIT-NNN` to `BC-S.SS.NNN`.

| Subsystem | Name | Pass-0 source files | Approx BC-AUDIT range | Target BC-S prefix |
|-----------|------|--------------------|-----------------------|-------------------|
| SS-01 | Hook Dispatcher Core | pass-3-behavioral-contracts.md, pass-3-behavioral-contracts-deep-r1.md | BC-AUDIT-001 – BC-AUDIT-086 | BC-1 |
| SS-02 | Hook SDK and Plugin ABI | pass-3-behavioral-contracts.md (hook-sdk section) | BC-AUDIT-087 – BC-AUDIT-111 | BC-2 |
| SS-03 | Event Emission (OTel-Aligned) | pass-3-behavioral-contracts.md (sink-core, sink-file; sink-otel-grpc deprecated) | BC-AUDIT-112 – BC-AUDIT-161 | BC-3 |
| SS-04 | Plugin Ecosystem | pass-3-behavioral-contracts.md (hook-plugins section) | BC-AUDIT-162 – BC-AUDIT-191 | BC-4 |
| SS-05 | Pipeline Orchestration | pass-3-deep-agents.md, pass-3-deep-workflows.md | BC-AUDIT-787 – BC-AUDIT-1003 | BC-5 |
| SS-06 | Skill Catalog | pass-3-deep-skills-batch-1.md, batch-2.md, batch-3.md | BC-AUDIT-192 – BC-AUDIT-744 | BC-6 |
| SS-07 | Hook Bash Layer | pass-3-deep-hooks.md | BC-AUDIT-1004 – BC-AUDIT-1179 | BC-7 |
| SS-08 | Templates and Rules | pass-3-deep-templates-tools-rules.md (templates + rules) | BC-AUDIT-1180 – BC-AUDIT-1309 | BC-8 |
| SS-09 | Configuration and Activation | pass-3-behavioral-contracts.md (activation, CI config) | BC-AUDIT-744 – BC-AUDIT-763 | BC-9 |
| SS-10 | CLI Tools and Bin | pass-3-deep-templates-tools-rules.md (bin tools + commands) | BC-AUDIT-1310 – BC-AUDIT-1452 | BC-10 |
