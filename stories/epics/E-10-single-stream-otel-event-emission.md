---
document_type: epic
epic_id: "E-10"
version: "1.6"
title: "Single-stream OTel-aligned event emission (ADR-015)"
status: draft
anchored_adr: ADR-015
prd_capabilities: [CAP-029, CAP-030]
prd_frs: []
anchor_strategy: adr-driven-migration
priority: P1
target_release: "v1.1"
story_count: 9
subsystems_affected: [SS-01, SS-02, SS-03, SS-04]
producer: story-writer
timestamp: 2026-05-04T00:00:00Z
phase: 2
traces_to: .factory/specs/architecture/decisions/ADR-015-single-stream-otel-schema.md
depends_on: []
last_amended: 2026-05-06
inputs:
  - .factory/specs/architecture/decisions/ADR-015-single-stream-otel-schema.md
input-hash: ""
---

# Epic E-10: Single-stream OTel-aligned event emission (ADR-015)

## Description

Implements the four normative decisions in ADR-015 (ACCEPTED 2026-05-04) across a
six-wave migration plan. ADR-015 supersedes ADR-005 (multi-sink fan-out) and amends
ADR-007 (always-on telemetry). The core problem: plugin-emitted events currently land
in `dispatcher-internal-YYYY-MM-DD.jsonl` alongside lifecycle noise; every downstream
consumer reads `events-YYYY-MM-DD.jsonl`, which is empty. Three known content-defect
bugs (pr.opened vs pr.created mismatch, plugin_version always emitting dispatcher
version, open_to_merge_seconds never emitted) are bundled into the migration.

The epic delivers:

1. **D-15.1** — Single physical stream: all events to `events-*.jsonl`; FileSink is
   the direct writer; Router/SinkRegistry/DlqWriter retired; `dispatcher-internal-*.jsonl`
   gated to `VSDD_DEBUG_LOG=1`.
2. **D-15.2** — OTel-aligned schema: Resource attributes stamped at startup;
   per-event identity fields stamped at emit time; `event.category` derived from a
   compile-time registry; `event.name` in reverse-DNS + `.vN` format.
3. **D-15.3** — Producer-side enrichment contract: host fields win; plugin overrides
   are surfaced via `vsdd.internal.host_field_override.v1` + stderr warning + inline
   `event.host_overrides`; block path gains `vsdd.block.plugin_blocked.v1` audit event.
4. **D-15.4** — Trace propagation: `VSDD_TRACE_ID` + `VSDD_PARENT_SPAN_ID` injected
   into every `exec_subprocess` call unconditionally (dispatcher-side invariants, not
   per-plugin env_allowlist).

## Scope

### In Scope

- Wave 0: read-only audit (Grafana panel inventory, field snapshot)
- Wave 1: host-side FileSink wiring + Resource-attribute enrichment + lifecycle event types
- Wave 2: plugin schema migration (all 11 affected WASM plugins) + dual-emit shims + bug-fix bundle
- Wave 3: Grafana consumer migration + dual-emit shim removal + deprecation announcement
- Wave 4: bash hook parity via `bin/emit-event` enhancement
- Wave 5: crate retirement (`sink-otel-grpc` deleted; `Router`/`SinkRegistry`/`DlqWriter` removed from `sink-core`)
- Three content-defect bug fixes: `pr.opened`→`pr.created` reconciliation,
  `plugin_version` fix, `open_to_merge_seconds` emission

### Out of Scope (Deferred to OQs)

- `observability-config.toml` schema post-migration (OQ-1, SS-03)
- Persistent deprecation registry artifact (OQ-9, SS-03)
- Atomic dual-emit host helper (OQ-8, SS-01)
- FileSink partial-write / atomic-rename recovery (OQ-7, SS-01)
- Windows fallback implementation beyond cascade terminal default (OQ-2)
- Operator-extensible `event-category-registry.toml` (D-15.2.a deferred)
- Tamper-evident audit log / WORM layer
- `factory-obs` / OTel Collector configuration changes

## PRD Capabilities Covered

| Capability ID | Name | Priority |
|--------------|------|----------|
| CAP-029 | Emit structured events to a single observability stream (file path) | P0 |
| CAP-030 | Enrich emitted events with OTel-aligned resource attributes | P1 |

## Subsystem Anchors

- **SS-01 (Hook Dispatcher Core):** owns `main.rs`, `host::emit_event`, FileSink wiring,
  exec_subprocess trace injection, Resource attribute stamping, per-event field stamping,
  event.category registry, lifecycle event emission.
- **SS-02 (Hook SDK and Plugin ABI):** receives an SDK MAJOR semver bump per BC-2.06.001
  to signal D-15.3 host-field-precedence semantics to plugin authors; `crates/hook-sdk/`
  Cargo.toml + CHANGELOG.md modified in Wave 2 (S-10.05 T-7).
- **SS-03 (Event Emission (OTel-Aligned)):** owns `sink-core`, `sink-file`, `sink-otel-grpc`,
  SS-03-observability-sinks.md spec, BC-3.* contracts; Wave 5 spec rewrite is SS-03 work.
- **SS-04 (Plugin Ecosystem):** receives reverse-DNS event-name migration with dual-emit
  shim per BC-4.09.001 — all native WASM plugins under `crates/hook-plugins/` migrate
  event names during the Wave 2→Wave 3 window; legacy emission removed post-Wave-3.

## Migration Wave → Story Mapping

| Wave | Story | Description | Size |
|------|-------|-------------|------|
| Wave 0 | S-10.01 | Read-only audit: Grafana panel inventory + event field snapshot | S |
| Wave 1 | S-10.02 | FileSink single-stream wiring (D-15.1) | M |
| Wave 1 | S-10.03 | Resource-attribute enrichment (D-15.2 Resource fields) | M |
| Wave 1 | S-10.04 | Trace propagation + lifecycle event types (D-15.4 + D-15.3 internals) | M |
| Wave 2 | S-10.05 | Plugin schema migration + dual-emit shims + bug-fix bundle | L |
| Wave 3 | S-10.06 | Consumer migration: Grafana + bin/factory-* tools + OTel collector | M |
| Wave 3 | S-10.07 | Wave 3 sub-tasks: deprecation announcement + operator audit gate | S |
| Wave 4 | S-10.08 | Bash hook parity: bin/emit-event schema alignment | M |
| Wave 5 | S-10.09 | Crate retirement + SS-03 spec rewrite | M |

## Dependency Topology (Intra-epic)

```
S-10.01 (Wave 0) ──→ S-10.02 (Wave 1)
                  ──→ S-10.03 (Wave 1)
                  ──→ S-10.04 (Wave 1)

S-10.02 ──→ S-10.05 (Wave 2)
S-10.03 ──→ S-10.05
S-10.04 ──→ S-10.05

S-10.05 ──→ S-10.06 (Wave 3)
S-10.06 ──→ S-10.07 (Wave 3 sub-tasks)

S-10.07 ──→ S-10.08 (Wave 4)
S-10.08 ──→ S-10.09 (Wave 5)
```

## Open Questions (Escalation References)

| OQ ID | Scope | Description |
|-------|-------|-------------|
| OQ-1 | SS-03 | `observability-config.toml` minimal schema post-migration |
| ~~OQ-2~~ | SS-01 | ~~Platform support matrix for Wave 1 (Windows registry fallback vs stub)~~ RESOLVED 2026-05-04: Wave 1 ships full Windows registry cascade (winreg crate, target-OS-gated). Implementation in S-10.03. |
| ~~OQ-5~~ | SS-03 | ~~Grafana dashboard migration scope and ownership~~ RESOLVED 2026-05-04: Dashboards in `plugins/vsdd-factory/tools/observability/grafana-dashboards/`, versioned-as-code. Migration bundled with S-10.06. |
| OQ-7 | SS-01 | FileSink partial-write recovery semantics |
| OQ-8 | SS-01 | Atomic dual-emit host helper (deferred) |
| OQ-9 | SS-03 | Persistent deprecation registry artifact (post-Wave-3) |
| ~~OQ-W16-011~~ | SS-01/SS-03 | ~~`VSDD_DEBUG_LOG=1` env var vs `debug_log_enabled = true` in `observability-config.toml` — precedence and interaction semantics.~~ RESOLVED D-311 (2026-05-06): 12-factor override semantics chosen. Env var dominates when present; config key governs when absent. BC-1.12.002 amended to v1.1. |

## Architect Routing Decisions (D-311)

These routing decisions were produced by the architect in D-311 to unblock PO Phase 1b/1c
BC authorship. PO MUST follow these decisions when filing BC-1.12.007 and BC-1.12.008.

### BC-1.12.007 — Deprecation/retirement lifecycle routing

**Subsystem:** SS-01
**File path:** `ss-01/BC-1.12.007.md`
**Rationale:** The behavioral invariant being specified is that no production code path
calls `Router`/`SinkRegistry`/`DlqWriter`/`sink-otel-grpc` after Wave 1. This is a
dispatcher-hot-path concern enforced by the SS-01 runtime (`main.rs` call graph), not
by the build system configuration. The `default-members` mutation in the workspace
`Cargo.toml` is a mechanical consequence of the dispatcher's routing decision, not its
subject. SS-03 Wave 5 cleanup (spec rewrite, BC-3.* revision/withdrawal, physical crate
deletion — S-10.09) will have its own SS-03 BC. BC-1.12.007 covers the deprecation
behavioral guarantee (call-graph invariant); BC-3.05.* covers the Wave 5 retirement
physical contract.

**TD-015-a note:** The CI `cargo metadata` re-coupling check is deferred per ADR-015
D-15.1. BC-1.12.007 MUST document this deferral (the check is not a postcondition of
Wave 1; it is a prerequisite for Wave 5 deletion safety).

### BC-1.12.008 — `observability-config.toml` v2 schema routing

**Subsystem:** SS-03
**File path:** `ss-03/BC-3.05.004.md`

> **D-312 Corrigendum (2026-05-06):** D-311 originally assigned `ss-03/BC-3.05.001.md`
> as the target file path for this BC. That assignment collided with the pre-existing
> brownfield BC-3.05.001 (authored 2026-04-25 by codebase-analyzer; covers
> `factory-dispatcher::sinks::mod::load_builds_file_sink_from_parsed_config`). Per
> POLICY 1 (append-only numbering), the pre-existing ID is immutable. The corrected
> assignment is BC-3.05.004 — the next free slot in the BC-3.05.* cluster, which
> SS-03-event-emission.md designates for "observability-config.toml v2 schema
> validation." Existing BC-3.05.001/002/003 have been marked `lifecycle_status:
> retired` / `superseded_by: ADR-015` in D-312.

**Rationale:** The schema definition (what fields are valid, what versions hard-error,
what the precedence rule is for `debug_log_enabled` vs `VSDD_DEBUG_LOG`) is owned by
SS-03. The authoritative text is in SS-03-event-emission.md § `observability-config.toml`
Schema (OQ-1 resolution). ARCH-INDEX Cross-Cutting Concerns assigns observability
ownership to SS-03. SS-01's role is to consume the parsed config, not to own its schema.

**Content note for PO:** EC-007 in BC-1.12.002 was the source of OQ-W16-011. That OQ
is now resolved (D-311). BC-3.05.004 MUST include the resolved two-key gate semantics:
env var dominates when present; `debug_log_enabled` config key governs when env var
absent. Quote from SS-03-event-emission.md as the authoritative source for the
normative precedence text.

### OQ-W16-011 — RESOLVED (D-311)

The `VSDD_DEBUG_LOG` env var vs `debug_log_enabled` config key precedence question is
RESOLVED. See open-questions.md § OQ-W16-011 for the full resolution text. BC-1.12.002
has been amended to v1.1 in D-311 with the resolved two-key gate semantics.

## CHANGELOG

| Version | Date | Change |
|---------|------|--------|
| v1.0 | 2026-05-04 | Initial authoring from ADR-015 migration plan decomposition. |
| v1.1 | 2026-05-04 | Story IDs corrected S-T.NN → S-10.NN (Q1); OQ-2 resolved (full Windows registry in S-10.03, Q4); OQ-5 resolved (Grafana dashboards in grafana-dashboards/ dir, Q6); OQ table updated. |
| v1.2 | 2026-05-06 | D-310 Phase 1a BC authorship complete: 4 new BCs authored (BC-1.12.001..BC-1.12.004 per ADR-015 D-15.1/D-15.2). OQ-W16-011 added to Open Questions table (VSDD_DEBUG_LOG vs debug_log_enabled config precedence; owner SS-01/SS-03 implementer; decision needed by S-10.02). Per TD-VSDD-071 OQ-propagation-to-epic. |
| v1.3 | 2026-05-06 | D-311 architect routing burst. OQ-W16-011 RESOLVED (12-factor override semantics; env var dominates when present; config key governs when absent). BC-1.12.007 routing: SS-01 (ss-01/BC-1.12.007.md) — dispatcher call-graph behavioral invariant. BC-1.12.008 routing: SS-03 (ss-03/BC-3.05.001.md) — config schema domain [ID COLLIDED; see v1.4 corrigendum]. Architect Routing Decisions section added. BC-1.12.002 amended to v1.1. |
| v1.4 | 2026-05-06 | D-312 corrigendum. BC-1.12.008 routing corrected: ss-03/BC-3.05.001.md → ss-03/BC-3.05.004.md. D-311 assigned .001 without verifying the slot; BC-3.05.001/002/003 were pre-existing brownfield BCs (codebase-analyzer, 2026-04-25) covering SinkRegistry loading, multi-sink fan-out, and sink-otel-grpc integration respectively. Per POLICY 1 (append-only numbering), those IDs are immutable. Corrected assignment: BC-3.05.004 (next free slot). Legacy BC-3.05.001/002/003 marked lifecycle_status: retired / superseded_by: ADR-015 in frontmatter (bodies preserved verbatim per POLICY 1). |
| v1.5 | 2026-05-06 | D-314 F-1 fix: capability anchors corrected from CAP-011/CAP-015 (unrelated capabilities — fuel/epoch budgets and brownfield ingestion) to CAP-029/CAP-030 (newly-authored ADR-015-aligned capabilities). CAP-029 = "Emit structured events to a single observability stream (file path)" (P0, ADR-015 D-15.1). CAP-030 = "Enrich emitted events with OTel-aligned resource attributes" (P1, ADR-015 D-15.2). Aspirational hedge note removed. |
| v1.6 | 2026-05-06 | D-323 — F-9 fix: subsystems_affected expanded to [SS-01, SS-02, SS-03, SS-04] reflecting D-318 BC-2.06.001 (SS-02) + BC-4.09.001 (SS-04) authoring; SS-02 + SS-04 Subsystem Anchors added with byte-for-byte ARCH-INDEX canonical names ("Hook SDK and Plugin ABI"; "Plugin Ecosystem"). SS-01 and SS-03 anchor names also corrected to exact ARCH-INDEX canonical form. |
