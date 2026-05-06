---
pass: 1
date: 2026-05-06
producer: adversary
artifacts_reviewed:
  - .factory/specs/architecture/decisions/ADR-015-single-stream-otel-schema.md
  - .factory/stories/epics/E-10-single-stream-otel-event-emission.md
  - .factory/specs/behavioral-contracts/ss-01/BC-1.11.003.md
  - .factory/specs/behavioral-contracts/ss-01/BC-1.12.001.md
  - .factory/specs/behavioral-contracts/ss-01/BC-1.12.002.md
  - .factory/specs/behavioral-contracts/ss-01/BC-1.12.003.md
  - .factory/specs/behavioral-contracts/ss-01/BC-1.12.004.md
  - .factory/specs/behavioral-contracts/ss-01/BC-1.12.005.md
  - .factory/specs/behavioral-contracts/ss-01/BC-1.12.006.md
  - .factory/specs/behavioral-contracts/ss-01/BC-1.12.007.md
  - .factory/specs/behavioral-contracts/ss-01/BC-1.12.009.md
  - .factory/specs/behavioral-contracts/ss-03/BC-3.05.004.md
  - .factory/stories/S-10.02-adr015-wave1-filesink-single-stream.md
  - .factory/stories/S-10.03-adr015-wave1-resource-attribute-enrichment.md
  - .factory/stories/S-10.04-adr015-wave1-trace-propagation-lifecycle-events.md
  - .factory/stories/S-10.05-adr015-wave2-plugin-schema-migration.md
  - .factory/stories/S-10.09-adr015-wave5-crate-retirement-ss03-rewrite.md
  - .factory/specs/behavioral-contracts/BC-INDEX.md
  - .factory/specs/architecture/ARCH-INDEX.md
  - .factory/stories/STORY-INDEX.md
  - .factory/specs/domain-spec/capabilities.md
  - .factory/specs/domain-spec/invariants.md
  - .factory/specs/open-questions.md
verdict: CRITICAL
---

# Adversary Pass 1 — E-10 Spec Package

## Verdict

CRITICAL — Two CRITICAL semantic-anchoring violations (mis-anchored CAP-IDs in the epic; CAP-003 anchor description directly contradicts ADR-015 single-stream decision in 7 of the 10 BCs). Multiple HIGH findings on POLICY 8 propagation (story-frontmatter coverage gap, BC↔story bidirectional mismatches), POLICY 6 (subsystem coverage missing SS-02 in S-10.05), POLICY 2 (5 domain invariants directly affected by ADR-015 are not cited by any of the 10 BCs), and Axis B (BC-1.12.004 Postcondition 1 cites `main.rs:143` — a `line N` self-reference forbidden by TD-VSDD-091). Epic-wave numbering contradicts itself (E-10 description says "six-wave" while migration plan ships 5 waves Wave 0–Wave 5 = 6 waves but the test count is "wave 17" in story frontmatter). NITPICK_ONLY pass eligibility = 0.

## Findings

### F-1 [CRITICAL]: E-10 epic anchors to capability IDs whose canonical names describe entirely unrelated capabilities

**Artifact:** `/Users/jmagady/Dev/vsdd-factory/.factory/stories/epics/E-10-single-stream-otel-event-emission.md` (frontmatter `prd_capabilities: [CAP-011, CAP-015]`; PRD Capabilities Covered table)
**Policy / Axis:** POLICY 4 (semantic_anchoring_integrity), POLICY 5 (creators_justify_anchors)
**Issue:** The epic claims `CAP-011 = "Emit structured events to the observability stream"` and `CAP-015 = "Enrich events with OTel-aligned resource attributes"`. Per `/Users/jmagady/Dev/vsdd-factory/.factory/specs/domain-spec/capabilities.md` the canonical capabilities are:
- **CAP-011** = "Enforce fuel and epoch budgets on plugin execution" (P1) — bounded fuel cap and epoch deadline for plugin invocations. Has nothing to do with event emission.
- **CAP-015** = "Ingest brownfield codebases via structured multi-pass analysis" (P1) — the brownfield-ingest skill's 7-phase analysis. Has nothing to do with OTel resource attributes.

The epic's hedge note ("treat this section as aspirational; the story-writer anchored to the closest semantically matching capabilities") admits the ambiguity but does NOT resolve it. The closest semantically correct capabilities for ADR-015's subject matter are **CAP-003** ("Stream observability events to multiple configurable sinks", though its name now contradicts D-15.1 — see F-2) and **CAP-027** ("Emit structured events from bash hooks via CLI tool" — relevant for Wave 4). An implementer reading the epic and looking up CAP-011/CAP-015 will find **fuel budgets** and **brownfield ingestion** — completely misleading.
**Evidence:** capabilities.md "**CAP-011 — Enforce fuel and epoch budgets on plugin execution**" and "**CAP-015 — Ingest brownfield codebases via structured multi-pass analysis**"; E-10 epic line 80 "CAP-011 | Emit structured events to the observability stream | P0" and line 81 "CAP-015 | Enrich events with OTel-aligned resource attributes | P1"; epic note line 86 "If the PRD capabilities do not yet enumerate these by these exact IDs, treat this section as aspirational"
**Routing:** PO + architect (capability registry update) — either the capabilities.md must be amended to add new CAP-IDs for ADR-015's subject matter, or the epic must be re-anchored to existing capabilities (CAP-003 + CAP-027 + a new CAP for resource enrichment).
**Suggested fix:** Either (a) author new capabilities CAP-029 "Emit structured events to the observability stream (single-stream)" and CAP-030 "Enrich events with OTel-aligned resource attributes" in capabilities.md and re-anchor the epic, OR (b) remove CAP-011/CAP-015 from the epic's prd_capabilities array and replace with CAP-003 (with explicit justification that ADR-015 simplifies CAP-003's multi-sink wording) plus CAP-010 (always-on self-telemetry, which BC-1.12.002 already anchors to). Mis-anchoring always blocks convergence.

### F-2 [CRITICAL]: 7 of 10 BCs anchor to CAP-003 whose canonical name contradicts the BCs' single-stream invariant

**Artifact:** BC-1.12.001, BC-1.12.003, BC-1.12.004, BC-1.12.005, BC-1.12.007, BC-1.12.009, BC-3.05.004 (Traceability section L2 Capability rows)
**Policy / Axis:** POLICY 4 (semantic_anchoring_integrity)
**Issue:** All seven BCs anchor to `CAP-003 ("Stream observability events to multiple configurable sinks")`. ADR-015 D-15.1 EXPLICITLY retires the multi-sink model (`Router`, `SinkRegistry`, `sink-otel-grpc`, multi-sink stanzas in `observability-config.toml`). The Anchor Justification on each BC says "ADR-015 D-15.1 simplifies CAP-003's multi-sink model to single-stream FileSink" — this admits the contradiction but does not fix it. CAP-003's description also lists "(file, OTel gRPC; HTTP/Datadog/Honeycomb planned for rc.1)" — every one of these is now either retired (sink-otel-grpc) or out of scope (planned sinks superseded by Collector-based fan-out per ADR-015). A reviewer trusting the capability registry as source-of-truth (POLICY 6 spirit) will see the BC anchored to a capability whose stated outcome ("operator sees events in Grafana/Loki or custom endpoint without modifying dispatcher code" via multi-sink fan-out) directly contradicts the BC's actual postconditions (single-stream FileSink only). Per the review axis, "mis-anchor would mislead an implementer into building the wrong thing" — an implementer following CAP-003's existing description would re-build the Router/SinkRegistry path that BC-1.12.001 forbids.
**Evidence:** capabilities.md line 38–41: "**CAP-003 — Stream observability events to multiple configurable sinks** — The dispatcher fans out every internal event to all enabled sink drivers (file, OTel gRPC; HTTP/Datadog/Honeycomb planned for rc.1)"; BC-1.12.001 Traceability "L2 Capability | CAP-003 ... per capabilities.md §CAP-003"; BC-1.12.001 Postcondition 2 "`Router::submit` is NOT called. `SinkRegistry` dispatch is NOT called." Note also CAP-023/CAP-024 (HTTP/Datadog/Honeycomb sinks; per-sink retry/circuit-breaker/DLQ) — both directly contradict ADR-015 D-15.1 D-15.1 Negative consequences and are not marked retired in capabilities.md. The capability registry has not been updated to reflect ADR-015 acceptance.
**Routing:** architect (capabilities.md amendment) — capabilities.md is the source-of-truth per POLICY 6-spirit. CAP-003/CAP-023/CAP-024 must be marked superseded or rewritten before BCs can credibly anchor to them.
**Suggested fix:** Amend capabilities.md to mark CAP-003 description as "[REWRITTEN per ADR-015]: Stream observability events to a single events-*.jsonl file with OTel-Collector-based downstream fan-out" and mark CAP-023/CAP-024 with "**Status:** SUPERSEDED by ADR-015 (multi-sink replaced by external OTel Collector fan-out)". Alternative: introduce a new CAP-029 specifically for single-stream and re-anchor the 7 BCs to it. The current state propagates a stale anchor across 7 BCs and is a content-defect pattern flag (3+ recurrences) per the engine rules.

### F-3 [HIGH]: BC-1.12.004 Postcondition 1 cites `main.rs:143` — `line N` self-reference forbidden by TD-VSDD-091

**Artifact:** `/Users/jmagady/Dev/vsdd-factory/.factory/specs/behavioral-contracts/ss-01/BC-1.12.004.md` Postcondition 1 (Source-of-truth verification block) and Architecture Anchors section
**Policy / Axis:** Axis A (TD-VSDD-091 stable-anchor citations)
**Issue:** Postcondition 1's "Source-of-truth verification (TD-VSDD-093)" block cites `main.rs:143` as the location of the bug ("ADR-015 Context identifies this as a known bug: '`plugin_version` is always the dispatcher's version (`env!("CARGO_PKG_VERSION")`) at `main.rs:143`'"). The Architecture Anchors section also lists `crates/factory-dispatcher/src/main.rs:143`. While these are quoting ADR-015's Source/Origin section (which also cites the line number), TD-VSDD-091 prohibits stable-anchor citations from using `line N` self-references. The line number can drift the moment any code change is introduced upstream — the BC will silently break its own citation. The same issue appears in BC-1.12.001 Architecture Anchors ("`crates/factory-dispatcher/src/sinks/mod.rs` lines 11–15"), BC-1.12.007 Architecture Anchors (same `lines 11–15`), and BC-1.12.004 Architecture Anchors (`host/mod.rs:109-116`, `host/mod.rs:113`). Pattern repeats across 4+ BC files; this is a content defect pattern flag (3+ recurrences → process-gap candidate).
**Evidence:** BC-1.12.004 Postcondition 1 "...at `main.rs:143`"; BC-1.12.004 Architecture Anchors "`crates/factory-dispatcher/src/main.rs:143` — the `plugin_version = env!("CARGO_PKG_VERSION")` bug"; BC-1.12.001 Architecture Anchors "`crates/factory-dispatcher/src/sinks/mod.rs` lines 11–15"; BC-1.12.007 Architecture Anchors "`crates/factory-dispatcher/src/sinks/mod.rs` lines 11–15"; BC-1.12.004 Postcondition 3 SOUL #4 acknowledgment "the `if let Ok(mut events) = self.events.lock()` pattern at `host/mod.rs:113` silently drops"; BC-1.12.001 §TD-VSDD-092 "the pre-Wave-1 `emit_internal` code path at `host/mod.rs:109-116`"
**Routing:** architect / state-manager — replace `line N` references with stable anchors (function/method names + the BC-IDs they correspond to in BC-1.05.036, etc.)
**Suggested fix:** Replace `main.rs:143` with `factory-dispatcher::main::plugin_version_stamp_call_site` (or quote the function/expression name) and replace `host/mod.rs:109-116` and `host/mod.rs:113` with `HostContext::emit_internal` and `HostContext::events::lock_push` respectively. Cross-link to BC-1.05.036 which already governs this code path. Same fix applies to all 4+ instances.

### F-4 [HIGH]: 5+ domain invariants materially impacted by ADR-015 are not cited by any of the 10 BCs

**Artifact:** All 10 BCs Traceability/L2 Domain Invariants rows (each says `TBD`); `/Users/jmagady/Dev/vsdd-factory/.factory/specs/domain-spec/invariants.md`
**Policy / Axis:** POLICY 2 (lift_invariants_to_bcs)
**Issue:** All 10 BCs under review have `L2 Domain Invariants | TBD` (or "TBD (domain spec invariants for emit enrichment not yet cross-referenced)"). The following invariants from `domain-spec/invariants.md` are MATERIALLY IMPACTED by ADR-015 and the BCs:
- **DI-007** "Dispatcher self-telemetry is always-on" — directly amended by ADR-015 (D-15.1 amends ADR-007). BC-1.12.002 governs this exact transition (debug stream gated). DI-007 must be cited here OR re-written to reflect the amendment.
- **DI-008** "Internal log filenames are derived from event timestamps, not wall clock" — still active for the debug stream; BC-1.12.002 uses `dispatcher-internal-YYYY-MM-DD.jsonl` filename pattern. Should be cited.
- **DI-011/DI-012/DI-013** "Sink submit must not block / sink failure isolation / unknown sink driver types non-fatal" — DI-011 and DI-012 are largely VOIDED by D-15.1 (single-sink, no per-sink isolation possible). DI-013 ("unknown sink driver types are non-fatal") interacts with BC-3.05.004 Postcondition 7 (warn-and-skip on unknown keys). Either the DIs must be retired/amended OR the BCs must cite them as enforcers.
- **DI-014** "Schema version mismatch is a hard load error" — directly enforced by BC-3.05.004 Postcondition 4 (schema_version=1 hard-error). BC-3.05.004 MUST cite DI-014.
- **DI-017** "`dispatcher_trace_id` is present on every emitted event" — interacts with BC-1.12.004 Postcondition 1 (`trace_id` field). Note also a naming drift: DI-017 says `dispatcher_trace_id` but BC-1.12.004 / ADR-015 D-15.4 use `trace_id`. ADR-015 v1.7 changelog explicitly canonicalized the name from `dispatcher_trace_id` → `trace_id`; DI-017 has not been updated.

5 orphan invariants is well over the "3+ orphans = HIGH severity with pattern flag" threshold per the engine rules.
**Evidence:** invariants.md DI-007 "Dispatcher self-telemetry is always-on" (Enforcement owner: SS-03 internal_log.rs); invariants.md DI-014 "Schema version mismatch is a hard load error"; invariants.md DI-017 "`dispatcher_trace_id` is present on every emitted event"; ADR-015 v1.7 changelog "trace_id naming canonicalized — `dispatcher_trace_id` → `trace_id` in D-15.2.e"; BC-3.05.004 Traceability "L2 Domain Invariants | TBD"; BC-1.12.002 Traceability "L2 Domain Invariants | TBD"
**Routing:** architect (DI updates) + PO (BC L2 Invariants citations)
**Suggested fix:** (a) Amend DI-007 to note "amended by ADR-015 D-15.1: debug file is opt-in via VSDD_DEBUG_LOG=1; see BC-1.12.002". (b) Amend DI-014 description to allow schema_version=2 (BC-3.05.004 hard-errors on v1, accepts v2). (c) Amend DI-017 field name to `trace_id` (canonical). (d) Cite DI-007/DI-008 in BC-1.12.002, DI-014 in BC-3.05.004, DI-017 in BC-1.12.004. (e) Mark DI-011/DI-012 as superseded by ADR-015 (single sink → per-sink isolation moot).

### F-5 [HIGH]: S-10.05 missing SS-02 in subsystems frontmatter despite modifying crates/hook-sdk/

**Artifact:** `/Users/jmagady/Dev/vsdd-factory/.factory/stories/S-10.05-adr015-wave2-plugin-schema-migration.md` (frontmatter `subsystems: ["SS-01", "SS-04"]`)
**Policy / Axis:** POLICY 6 (architecture_is_subsystem_name_source_of_truth)
**Issue:** S-10.05 declares `subsystems: ["SS-01", "SS-04"]` in frontmatter, but its tasks and File Structure Requirements explicitly modify `crates/hook-sdk/Cargo.toml` and `crates/hook-sdk/CHANGELOG.md` (T-7, AC-008). Per ARCH-INDEX (line 75): SS-02 = "Hook SDK and Plugin ABI" with `crates/hook-sdk/`. BC-1.11.003 (in this story's bcs:) explicitly declares Architecture Module "SS-01 — `crates/factory-dispatcher/...`; SS-02 — `crates/hook-sdk/src/host.rs`". The story's bcs: array references BC-1.11.003 which is half-SS-02. SS-02 should be in the subsystems array. This is a POLICY 6 anchor coverage gap.
**Evidence:** S-10.05 frontmatter line 31 `subsystems: ["SS-01", "SS-04"]`; S-10.05 line 197 Architecture Mapping `vsdd-hook-sdk | crates/hook-sdk/`; S-10.05 line 225 T-7 "Bump vsdd-hook-sdk to MAJOR semver version"; S-10.05 line 266 File Structure Requirements `crates/hook-sdk/Cargo.toml MODIFY`, `crates/hook-sdk/CHANGELOG.md MODIFY`; BC-1.11.003 Traceability "Architecture Module | SS-01 — `crates/factory-dispatcher/src/host/emit_event.rs`; SS-02 — `crates/hook-sdk/src/host.rs`"
**Routing:** story-writer (S-10.05 frontmatter amendment)
**Suggested fix:** Update S-10.05 frontmatter `subsystems: ["SS-01", "SS-02", "SS-04"]`. Add SS-02 row to Architecture Mapping table.

### F-6 [HIGH]: BC-1.11.003 frontmatter capability=CAP-TBD never resolved despite full BC body authoring

**Artifact:** `/Users/jmagady/Dev/vsdd-factory/.factory/specs/behavioral-contracts/ss-01/BC-1.11.003.md` (frontmatter line 16 `capability: "CAP-TBD"`); BC-INDEX row 164
**Policy / Axis:** POLICY 4 (semantic_anchoring_integrity), POLICY 5 (creators_justify_anchors)
**Issue:** BC-1.11.003 has `capability: "CAP-TBD"` in frontmatter and `L2 Capability | CAP-TBD` in Traceability. The BC is fully fleshed out (postconditions, ECs, test vectors, related BCs) and was authored 2026-05-04 by the architect, with story integration in S-10.05 v1.1 (D-313). BC-INDEX also propagates `CAP-TBD`. No explicit blocker prevents resolving the capability — sibling BCs (BC-1.12.001/3/4/5/7/9, BC-3.05.004) all anchor to CAP-003 (a separate problem per F-2), but BC-1.11.003 doesn't even reach for one. Per POLICY 5, "Stop and ask rather than guess" — but this BC has been silently shipped with TBD across two sealed bursts (D-310 → D-313). Stop-and-ask was never triggered. This is a process-gap: agents should not be able to ship BCs to BC-INDEX with CAP-TBD persisting across multiple bursts.
**Evidence:** BC-1.11.003 frontmatter line 16 `capability: "CAP-TBD"`; BC-1.11.003 Traceability "L2 Capability | CAP-TBD"; BC-INDEX line 164 "| CAP-TBD | Wave 2 TBD"; S-10.05 BC table "BC-1.11.003 ... | CAP-TBD"
**Routing:** PO (resolve capability anchor) — `[process-gap]` candidate for orchestrator follow-up since CAP-TBD has survived two sealed bursts.
**Suggested fix:** Anchor BC-1.11.003 to a real capability. Most-natural anchor is the same one CAP question raised in F-2: CAP-003 (with same justification gap), or a new CAP for plugin SDK ergonomics (the `emit_pair` host helper is an SDK ABI extension so CAP-009 "Author and publish WASM hook plugins using the Rust SDK" would be a closer fit). [process-gap] flag: lessons-codification needs a rule that CAP-TBD on a `status: draft` BC cannot persist across more than one burst.

### F-7 [HIGH]: BC-1.12.005 frontmatter phase: 1b but Story Anchor names S-10.02 + S-10.03 (Wave 1 stories), creating wave/phase confusion

**Artifact:** `/Users/jmagady/Dev/vsdd-factory/.factory/specs/behavioral-contracts/ss-01/BC-1.12.005.md` frontmatter and Story Anchor section; S-10.04 narrative (Wave 1 D-15.3 internals)
**Policy / Axis:** POLICY 4 (semantic_anchoring_integrity), Axis I (Cross-BC traceability)
**Issue:** BC-1.12.005 declares Story Anchor "S-10.02 (Wave 1: FileSink wiring + per-event stamping); S-10.03 (Wave 1: Resource-attribute enrichment)". S-10.04 (Wave 1: Trace propagation + lifecycle event types) ALSO covers AC-003 "vsdd.internal.host_field_override.v1 emitted when plugin supplies host-owned field" — this is the EXACT subject of BC-1.12.005. S-10.04 narrative line 47-50 explicitly mentions "visible lifecycle events when host fields are overridden". S-10.04 frontmatter `bcs: [BC-1.12.006]` does NOT include BC-1.12.005, and S-10.04 AC-003 has NO `(traces: BC-1.12.005)` annotation. This is a POLICY 8 propagation gap: BC-1.12.005's host-field-override behavior is implemented in S-10.04 but the BC↔story link is absent in the story body and frontmatter. Either S-10.04 must add BC-1.12.005 to its bcs array and AC-003 trace, OR S-10.04 AC-003 should be removed and folded into S-10.02/S-10.03 (where BC-1.12.005 is currently anchored).
**Evidence:** BC-1.12.005 Story Anchor lines 132-136 names S-10.02 + S-10.03; S-10.04 frontmatter line 24 `behavioral_contracts: - BC-1.12.006` (only one); S-10.04 AC-003 line 80-99 implements the host_field_override 3-channel signal but has no BC trace; S-10.04 task T-2 "Implement host-field override detection at `host::emit_event`"; S-10.04 task T-3 "Implement override rate-limiter"; S-10.04 task T-4 "Emit `vsdd.internal.host_field_override.v1`"
**Routing:** story-writer (S-10.04 frontmatter + AC trace) OR architect (re-anchor BC-1.12.005 Story Anchor to S-10.04 if that is the actual implementation site)
**Suggested fix:** Add BC-1.12.005 to S-10.04 `behavioral_contracts:` array. Add `(traces: BC-1.12.005 postcondition 5)` annotation to AC-003. Update BC-1.12.005 Story Anchor to add S-10.04 alongside S-10.02/S-10.03 (the override visibility spans all three Wave 1 stories).

### F-8 [HIGH]: S-10.04 AC-005 incorrectly cites BC-1.12.003 emission requirement as out-of-scope, but BC-1.12.006 (S-10.04's own BC) gates lifecycle event registry membership

**Artifact:** `/Users/jmagady/Dev/vsdd-factory/.factory/stories/S-10.04-adr015-wave1-trace-propagation-lifecycle-events.md` AC-005 + AC-006
**Policy / Axis:** POLICY 8 (bc_array_changes_propagate_to_body_and_acs), Axis A
**Issue:** S-10.04 AC-005 claims `vsdd.internal.host_id_fallback.v1` is "implemented in S-10.03 but tested here as a lifecycle event type". S-10.03 has BC-1.12.003 in its frontmatter (which mandates the host_id_fallback event in Postcondition 3 + EC-014). But AC-005 in S-10.04 has no BC trace at all, and S-10.04 frontmatter does NOT include BC-1.12.003. AC-006 enumerates 4 new event types and says "category registry from S-10.03" — but the registry is in BC-1.12.004, which is in S-10.03 (and S-10.02) frontmatter. S-10.04 modifies the registry (T-8 "Add registry entries for `vsdd.internal.event_name_deprecated.v1` and `vsdd.block.plugin_blocked.v1`") yet has no link to BC-1.12.004. Result: S-10.04 modifies BC-1.12.003 + BC-1.12.004 functionality without listing them in `behavioral_contracts:`, breaking the story↔BC bidirectional check (POLICY 8).
**Evidence:** S-10.04 frontmatter line 24 `behavioral_contracts: - BC-1.12.006` (only); S-10.04 AC-005 line 111-118 (no BC trace); S-10.04 AC-006 line 120-133 (no BC trace); S-10.04 T-8 line 170 "Add registry entries for `vsdd.internal.event_name_deprecated.v1` and `vsdd.block.plugin_blocked.v1`"; BC-1.12.004 Postcondition 2 (the compile-time registry); BC-1.12.003 EC-014 (terminal fallback emits `vsdd.internal.host_id_fallback.v1`)
**Routing:** story-writer (S-10.04 frontmatter + AC traces)
**Suggested fix:** Add BC-1.12.003 + BC-1.12.004 (and BC-1.12.005 per F-7) to S-10.04 `behavioral_contracts:`. Add `(traces: BC-1.12.003 EC-014)` to AC-005 and `(traces: BC-1.12.004 postcondition 2)` to AC-006.

### F-9 [HIGH]: BC-1.12.007 invariant 1 contradicts BC-1.12.007 postcondition 1 on enforcement modality

**Artifact:** `/Users/jmagady/Dev/vsdd-factory/.factory/specs/behavioral-contracts/ss-01/BC-1.12.007.md` Postcondition 1 vs Invariant 1
**Policy / Axis:** Axis G (Wave-1 call-graph invariant testability)
**Issue:** Postcondition 1 says the call-graph invariant means "Router::submit, SinkRegistry dispatch methods, DlqWriter::write, and any public API of sink-otel-grpc are NOT called from any code path reachable from main.rs in the production dispatcher binary. This includes direct calls AND transitive calls through any helper, wrapper, or trait method." That's a transitively-reachable claim. Invariant 1 says "The call-graph invariant is enforced by the Wave 1 implementation change, NOT by a CI check ... removing the `Router::submit` call from `main.rs` enforces the invariant mechanically. The invariant is testable via static analysis or integration test (assert no events in `sink-otel-grpc`'s output path)." This is contradictory in two ways: (a) "removing the Router::submit call from main.rs" is a single-call enforcement, not a transitive enforcement (a helper could still wrap Router::submit). (b) "Static analysis or integration test" is hand-waving — the BC does not specify WHICH static analysis tool or what call-graph snapshot is normative. Per the review axis G: "BC must specify ... the invariant is testable (production-fn enumeration via grep/static-analysis vs. runtime-only)". Currently the BC is neither concrete enough for static testing nor decisive about runtime testing.

Additionally, EC-001 admits "Per TD-015-a, this re-coupling is NOT currently detected by CI" — meaning the invariant is enforced manually between Wave 1 and Wave 5, but the BC has no concrete checklist or run-book entry. The TD-015-a deferral has no explicit owner or completion-by date in BC-1.12.007.
**Evidence:** BC-1.12.007 Postcondition 1 "...includes direct calls AND transitive calls through any helper, wrapper, or trait method"; BC-1.12.007 Invariant 1 "...enforced by the Wave 1 implementation change, NOT by a CI check ... removing the `Router::submit` call from `main.rs` enforces the invariant mechanically"; BC-1.12.007 EC-001 "Per TD-015-a, this re-coupling is NOT currently detected by CI"; BC-1.12.007 Canonical Test Vector "Static analysis (e.g., `cargo udeps` or custom lint) post-Wave-1 | No active workspace crate has..."
**Routing:** PO (BC body sharpening) or architect (TD-015-a ownership)
**Suggested fix:** (a) Resolve Postcondition 1 vs Invariant 1 contradiction: state explicitly whether the invariant is "no direct call from main.rs" (testable via grep) or "no transitive call from any production-reachable function" (requires call-graph tool — name the tool: `cargo-call-stack`, `rust-analyzer`, etc.). (b) Convert TD-015-a from "deferred" to a tracked open question with explicit owner + decision-by date. (c) Add a Canonical Test Vector that mandates the specific check: `grep -rn "Router::submit\|SinkRegistry::dispatch\|DlqWriter::write" crates/factory-dispatcher/src/ | grep -v "#\[deprecated\]\|#\[allow(dead_code)\]" | wc -l` returns `0`.

### F-10 [HIGH]: BC-1.12.005 EC-008 self-contradicts the rate-limit semantics

**Artifact:** `/Users/jmagady/Dev/vsdd-factory/.factory/specs/behavioral-contracts/ss-01/BC-1.12.005.md` EC-008
**Policy / Axis:** POLICY 4
**Issue:** EC-008 says "`vsdd.internal.host_field_override.v1` itself is written; rate-limiter fires" — this is incoherent. The header ID name "rate-limiter fires" but the prose then says "The lifecycle event is written via normal FileSink path; it is NOT rate-limited against itself; each `(plugin.name, field_name)` pair fires at most once per invocation regardless of how many domain events triggered the override". Read as written: the EC describes BOTH "rate-limiter fires" AND "is NOT rate-limited" — which one is the expected behavior? An implementer cannot tell what to write a test for. The intent appears to be "the lifecycle event itself is not subject to a meta-rate-limit" but the EC ID ("rate-limiter fires") suggests the opposite.
**Evidence:** BC-1.12.005 EC-008 line 152 `EC-008 | vsdd.internal.host_field_override.v1 itself is written; rate-limiter fires | The lifecycle event is written via normal FileSink path; it is NOT rate-limited against itself`
**Routing:** PO (BC clarification)
**Suggested fix:** Rewrite EC-008 description to "The lifecycle event itself is not subject to a meta-rate-limit; the per-`(plugin.name, field_name)` rate-limit governs original emission" and rename the row's ID prose to remove the ambiguous "rate-limiter fires" phrase.

### F-11 [HIGH]: BC-3.05.004 Postcondition 4 conflicts with EC-004 on schema_version > 2 handling

**Artifact:** `/Users/jmagady/Dev/vsdd-factory/.factory/specs/behavioral-contracts/ss-03/BC-3.05.004.md` Postcondition 4 + Invariant 1 + EC-004
**Policy / Axis:** POLICY 4, Axis D (OQ-W16-011 RESOLVED enforcement — partial: this is also a schema-version completeness gap)
**Issue:** Postcondition 4 says "A config file with `schema_version = 1` (or any integer less than 2) MUST hard-error at load time." Invariant 1 says "`schema_version = 2` is the only accepted version. Any other integer (including 1) hard-errors with a migration hint." Both imply schema_version != 2 → hard-error (so schema_version=3 hard-errors). But EC-004 ("schema_version = 3 (future version, not yet known)") says "Treated as unknown ... hard-error or warn-and-use-defaults depending on implementation policy. **Note:** this edge case is a known unresolved detail — the current spec mandates `schema_version = 2` exactly. Future schema versions require a new BC." This contradicts Invariant 1's "Any other integer (including 1) hard-errors". The BC has not resolved its own behavior for schema_version > 2. Per POLICY 4, the spec must be deterministic. Per Axis F (collectively exhaustive states), the schema_version domain is partitioned into {1, 2, >2} but the {>2} state has UNRESOLVED behavior.
**Evidence:** BC-3.05.004 Postcondition 4 lines 84-93 ("A config file with `schema_version = 1` (or any integer less than 2) MUST hard-error..."); Invariant 1 line 148 ("`schema_version = 2` is the only accepted version. Any other integer (including 1) hard-errors with a migration hint"); EC-004 lines 210-211 ("Treated as unknown (version > 2 is not currently supported); hard-error or warn-and-use-defaults depending on implementation policy. **Note:** this edge case is a known unresolved detail")
**Routing:** PO (BC postcondition clarification)
**Suggested fix:** Resolve EC-004 to one of: (a) "schema_version > 2 hard-errors with `unknown future schema version` message" (most consistent with Invariant 1), or (b) "schema_version > 2 warns and uses v2 defaults". Update Invariant 1 OR EC-004 so they don't contradict.

### F-12 [HIGH]: BC-1.12.009 four-state taxonomy is mathematically incomplete (missing 5th state)

**Artifact:** `/Users/jmagady/Dev/vsdd-factory/.factory/specs/behavioral-contracts/ss-01/BC-1.12.009.md` Postcondition 4 + Invariant 1
**Policy / Axis:** Axis F (Dual-emit four-state classification: mutually exclusive AND collectively exhaustive)
**Issue:** Per ADR-015 D-15.2.e changelog v1.5 ("addressed pass 5 adversary findings F-1: added fifth state to D-15.2.e covering orphaned-half consumer degradation rule"), the ADR explicitly defines a FIVE-STATE classification (non-paired, paired-old, paired-new, post-Wave-3 absent, orphaned-half). BC-1.12.009 reduces this to a "four-state consumer taxonomy" but explicitly cites the ADR's five-state in the Architecture Anchors row (line 230 "the authoritative five-state identity contract (non-paired, paired-old, paired-new, post-Wave-3 absent, orphaned-half); this BC normalizes it to the four-state consumer taxonomy"). However, the BC also documents a "Non-paired (baseline state, also the post-Wave-3 state)" inside Postcondition 4 — making the "four-state" actually FIVE states (1: paired-current, 2: paired-deprecated, 3: orphaned-deprecated-half, 4: orphaned-current-half, 5: non-paired/post-Wave-3). The "four-state" naming is misleading.

Worse, Postcondition 4 says any event "falls into exactly one of four states" — but then describes FIVE states (the four numbered + "Non-paired (baseline state)"). This violates the BC's own claim of mutual-exclusivity-and-collective-exhaustiveness. An implementer reading the BC will see "exactly one of four" and miss the fifth (non-paired) baseline.

Additionally: Invariant 1 says "`event.deprecated_by` and `event.replaces_deprecated_alias` are mutually exclusive on any single event". EC-005 says "Both `event.deprecated_by` AND `event.replaces_deprecated_alias` present on same event | Malformed (violates Invariant 1); consumer MUST treat as orphaned half". So MALFORMED events DO occur in practice — adding a 6th state (Malformed) — which is also not in the four-state count.
**Evidence:** BC-1.12.009 line 230 Architecture Anchors "the authoritative five-state identity contract"; BC-1.12.009 line 96 "Any event in `events-*.jsonl` falls into exactly one of four states"; BC-1.12.009 lines 99-141 list 4 numbered states then add "Non-paired (baseline state, also the post-Wave-3 state)"; BC-1.12.009 EC-005 "Both `event.deprecated_by` AND `event.replaces_deprecated_alias` present on same event | Malformed (violates Invariant 1); consumer MUST treat as orphaned half"; BC-1.12.009 H1 line 28 includes "four-state event classification (paired-current / paired-deprecated / orphaned-current-half / orphaned-deprecated-half)" — H1 omits non-paired
**Routing:** PO (BC body and H1 sync)
**Suggested fix:** Either (a) rename to "five-state taxonomy" and explicitly enumerate all 5 (paired-current, paired-deprecated, orphaned-deprecated-half, orphaned-current-half, non-paired), updating H1 accordingly; OR (b) keep "four-state" but explicitly call non-paired the "baseline (default)" not a state. Reconcile EC-005 (malformed) by classifying malformed → orphaned-half explicitly in Postcondition 4 (currently only in EC).

### F-13 [HIGH]: H1 of BC-1.12.002 says "VSDD_DEBUG_LOG=1 env var or debug_log_enabled=true config key" but ADR/OQ-W16-011 specifies precedence (env dominates), not OR

**Artifact:** `/Users/jmagady/Dev/vsdd-factory/.factory/specs/behavioral-contracts/ss-01/BC-1.12.002.md` H1 line 28
**Policy / Axis:** POLICY 7 (bc_h1_is_title_source_of_truth), Axis D
**Issue:** BC-1.12.002 H1: "...gated by VSDD_DEBUG_LOG=1 env var or debug_log_enabled=true config key; off by default in release builds; ADR-007 always-on guarantee amended". The use of "or" in H1 is too lax. OQ-W16-011 resolution (D-311) specifies 12-factor override semantics: env var DOMINATES when present; config key GOVERNS when env var absent. The BC body Description, Postconditions 2-3, Invariants 1-3, and EC-007/EC-007b correctly reflect this precedence. But the H1 claims a flat "or" relationship which is technically true only in the union sense (either condition enables the gate) and obscures the precedence rule.

POLICY 7 says "BC file H1 heading is the authoritative BC title. All downstream references must match. Enrichment goes INTO the H1." The downstream references (BC-INDEX line 166, S-10.02 BC table line 188) carry the same ambiguous H1 verbatim, so the imprecision propagates. BC-3.05.004 H1 (line 29) is more precise: "...two-key debug-stream gate (VSDD_DEBUG_LOG=1 env var dominates; debug_log_enabled config key governs when env var absent)". BC-1.12.002 H1 should match this level of precision.
**Evidence:** BC-1.12.002 line 28 H1 ("...VSDD_DEBUG_LOG=1 env var or debug_log_enabled=true config key..."); BC-3.05.004 line 29 H1 ("...two-key debug-stream gate (VSDD_DEBUG_LOG=1 env var dominates; debug_log_enabled config key governs when env var absent)"); BC-INDEX line 166 (carries the imprecise H1 verbatim); OQ-W16-011 resolution `Option chosen: (c) — 12-factor override semantics: env var dominates when present; config key governs when env var absent.`
**Routing:** PO (H1 amendment) + state-manager (BC-INDEX sync)
**Suggested fix:** Update BC-1.12.002 H1 to match BC-3.05.004's precision: "...two-key debug-stream gate (VSDD_DEBUG_LOG=1 env var dominates when present; debug_log_enabled config key governs when env var absent)..." and propagate to BC-INDEX in same burst (POLICY 7 same-burst sync).

### F-14 [MEDIUM]: E-10 epic story_count: 9 but Wave→Story Mapping table shows 9 rows, while Migration Plan claims "six-wave migration plan" (5 actual waves Wave 0..Wave 5)

**Artifact:** `/Users/jmagady/Dev/vsdd-factory/.factory/stories/epics/E-10-single-stream-otel-event-emission.md` Description line 31, Wave→Story Mapping table line 99
**Policy / Axis:** POLICY 4 (semantic_anchoring_integrity, internal-consistency)
**Issue:** Epic Description line 31 says "...across a six-wave migration plan." Wave 0 (S-10.01), Wave 1 (S-10.02-04), Wave 2 (S-10.05), Wave 3 (S-10.06-07), Wave 4 (S-10.08), Wave 5 (S-10.09) = 6 waves. That part is correct. But ADR-015 Migration Plan only enumerates Wave 0 through Wave 5 (6 waves) — also correct. However, the Wave→Story Mapping table in the epic shows 9 rows (S-10.01-09) — also correct. So far consistent. BUT: each E-10 story frontmatter has `wave: 17` (e.g., S-10.02 line 26 `wave: 17`). Wave 17 refers to the orchestrator's overall wave-numbering scheme (project-wide Wave 17 of cycle v1.0-brownfield-backfill). This is potentially confusing — the term "wave" in frontmatter is the project-cycle wave; the term "wave" in narrative text and Migration Plan is an ADR-015 migration wave (0-5). The two namespaces collide on the same word with no disambiguation in the story bodies.
**Evidence:** S-10.02 frontmatter line 26 `wave: 17`; S-10.02 narrative line 47 "Wave: Wave 1"; S-10.02 narrative line 48 "Depends on: S-10.01"; epic line 99 "Wave 0 | S-10.01"; epic line 31 "across a six-wave migration plan"; ADR-015 Migration Plan section enumerates Wave 0..Wave 5
**Routing:** story-writer (story body disambiguation)
**Suggested fix:** Add a one-liner to each E-10 story body explaining: "Note: `wave: 17` in frontmatter refers to the project-cycle wave (orchestrator scheduling). The 'Wave 1/2/3/4/5' references in this story body refer to ADR-015 migration waves (per ADR-015 Migration Plan)." OR rename the frontmatter field to `cycle_wave: 17` to disambiguate.

### F-15 [MEDIUM]: BC-1.12.001 Postcondition 3 SOUL #4 acknowledgment grammar conflict — claims "silent discard is no longer the failure mode on the critical path" but BC-1.05.036 EC-011 silent-drop is preserved

**Artifact:** `/Users/jmagady/Dev/vsdd-factory/.factory/specs/behavioral-contracts/ss-01/BC-1.12.001.md` Postcondition 3
**Policy / Axis:** POLICY 4, TD-VSDD-093 (closure-narrative source-of-truth validation)
**Issue:** BC-1.12.001 Postcondition 3 says: "After the FileSink rewire, the `emit_event` → `FileSink::write` path uses the write-failure cascade per BC-1.11.002 (NOT silent discard) — the cascade writes to the fallback file and emits a stderr warning. **Silent discard is no longer the failure mode on the critical path.**" But BC-1.12.004 Postcondition 3 (SOUL #4 acknowledgment) and BC-1.12.001's own §TD-VSDD-092 verification section both say: "The pre-Wave-1 `emit_internal` code path at `host/mod.rs:109-116` discards IO errors from `log.write` via a best-effort pattern — this discard is acknowledged and intentional per BC-1.05.036 Postcondition 6." AND: "the `events.lock().push(event)` at `host/mod.rs:113-115`: uses `if let Ok(mut events) = self.events.lock()` which silently drops on Mutex poison. Acknowledged ... known-limitation per BC-1.05.036 EC-011 / OQ-W16-004. This is the ONLY known silent-discard in this path; it is documented, not new."

So silent discard IS preserved (Mutex poison case, in-memory events queue) — contradicting the bold claim "Silent discard is no longer the failure mode on the critical path." The claim is true for FileSink IO errors but not true for Mutex poison. The spec needs to clarify SCOPE: the claim applies to the file-write path, not the in-memory events queue.
**Evidence:** BC-1.12.001 Postcondition 3 line 82 "Silent discard is no longer the failure mode on the critical path"; BC-1.12.001 §TD-VSDD-092 line 198 "The pre-Wave-1 `emit_internal` code path at `host/mod.rs:109-116` discards IO errors from `log.write` via a best-effort pattern — this discard is acknowledged and intentional"; BC-1.12.004 §TD-VSDD-092 lines 287-290 "...silently drops on Mutex poison. Acknowledged ... known-limitation per BC-1.05.036 EC-011 / OQ-W16-004. This is the ONLY known silent-discard in this path"
**Routing:** PO (BC body sharpening)
**Suggested fix:** Amend BC-1.12.001 Postcondition 3 to: "Silent discard is no longer the failure mode for FileSink::write IO errors on the critical path. The pre-existing in-memory events-queue Mutex-poison silent-drop (BC-1.05.036 EC-011) is preserved as a documented known-limitation."

### F-16 [MEDIUM]: S-10.05 lists "11 affected WASM plugins" but plugin inventory in story body lists 11 with "Verify count matches ADR-015's 11 currently-impacted WASM plugins" — ADR-015 has no such "11" enumeration

**Artifact:** `/Users/jmagady/Dev/vsdd-factory/.factory/stories/S-10.05-adr015-wave2-plugin-schema-migration.md` Architecture Mapping (line 198)
**Policy / Axis:** POLICY 4 (semantic_anchoring_integrity), Axis B (TD-VSDD-093 closure-narrative source-of-truth validation)
**Issue:** S-10.05 line 200 says: "(Verify count matches ADR-015 '11 currently-impacted WASM plugins'.)". I grep'd ADR-015 — there is NO mention of "11 plugins" anywhere in the ADR. The ADR refers to "every native WASM plugin" generically. The "11" count appears to be an unsourced claim. EC-005 line 215 also says "Additional plugins discovered in S-10.01 audit beyond the 11 listed" — but S-10.01 has not yet executed (S-10.01 is Wave 0 measurement, status: draft, not yet run). The "11" is therefore an unverified fabrication, not a closure-narrative-validated count.

This is exactly the kind of source-of-truth drift TD-VSDD-093 was created to catch. The story body asserts "ADR-015 says 11"; the ADR says nothing of the kind. An implementer reading S-10.05 will trust the count, fail to find an ADR-015 enumeration, and either invent one or get confused.
**Evidence:** S-10.05 line 198 Architecture Mapping "**Plugin inventory (11 affected WASM plugins — verify from S-10.01 audit output):** Per ADR-015 Context 'every plugin-emitted domain event,'..."; S-10.05 line 204 "(Verify count matches ADR-015 '11 currently-impacted WASM plugins'.)"; S-10.05 EC-005 line 215 "Additional plugins discovered in S-10.01 audit beyond the 11 listed"; ADR-015 (grep "11" returns no plugin-count enumeration); ADR-015 Context "every plugin-emitted domain event" (no count)
**Routing:** story-writer (S-10.05 amendment)
**Suggested fix:** Replace "11 affected WASM plugins" claim with "Plugin inventory: TBD (to be enumerated from S-10.01 audit output)". Remove the "(Verify count matches ADR-015...)" parenthetical since ADR-015 makes no such claim. Defer plugin enumeration to S-10.01 completion.

### F-17 [MEDIUM]: BC-1.12.003 EC-002 (CI=true detection) inconsistency between BC and S-10.03 EC-002

**Artifact:** BC-1.12.003 EC-011 + S-10.03 EC-002
**Policy / Axis:** POLICY 4
**Issue:** BC-1.12.003 EC-011 says: "`CI=true` in environment | `deployment.environment.name` = `"ci"`". S-10.03 EC-002 says: "`CI` env var set to non-`"true"` string (e.g. '1') | Only the literal `"true"` value maps to `"ci"`; other values → `"local-dev"`". This is potentially correct (the BC asserts the happy path; the story EC asserts the strict-string check) but it's not consistent — many CI systems set `CI=1` (GitHub Actions actually sets `CI=true`, but Travis sets `CI=true` AND `CONTINUOUS_INTEGRATION=true`, GitLab sets `CI=true`, but generic shell scripts often use `CI=1`). The spec mandates strict literal `"true"` only. ADR-015 D-15.2 Resource attributes table line 180 says: "`deployment.environment.name` | `"ci"` if `CI=true` else `"local-dev"`" — also strict. But the test for `CI=1` (a common pattern) failing silently to `local-dev` is potentially surprising. This is a Postcondition completeness issue that should be at minimum called out as an Invariant in BC-1.12.003.
**Evidence:** BC-1.12.003 EC-011 line 188 "`CI=true` in environment | `deployment.environment.name` = `"ci"`"; S-10.03 EC-002 line 211 "`CI` env var set to non-`"true"` string (e.g. '1') | Only the literal `"true"` value maps to `"ci"`"; ADR-015 D-15.2 line 180 "`deployment.environment.name` | `"ci"` if `CI=true` else `"local-dev"`"
**Routing:** PO (BC EC clarification)
**Suggested fix:** Add an explicit BC-1.12.003 Invariant: "`deployment.environment.name = 'ci'` requires the EXACT string `'true'` in the `CI` env var. `CI=1`, `CI=yes`, `CI=on` all map to `'local-dev'`." Add a test vector witnessing this strict semantics.

### F-18 [MEDIUM]: BC-3.05.004 cites BC-1.12.002 as "BC-3.05.004 is the schema contract; BC-1.12.002 is the runtime contract. Both must be consistent" — but BC-1.12.002 frontmatter capability is CAP-010 while BC-3.05.004 is CAP-003

**Artifact:** BC-3.05.004 Related BCs section + BC-1.12.002 frontmatter
**Policy / Axis:** POLICY 4, POLICY 5
**Issue:** BC-3.05.004 says it must be "consistent" with BC-1.12.002. But the two BCs have different capability anchors — BC-1.12.002 → CAP-010 (always-on dispatcher self-telemetry); BC-3.05.004 → CAP-003 (multi-sink streaming). The capability frame for "the same gate semantics" is split across two unrelated capabilities. If the gate semantics are truly one shared invariant (one runtime + one schema view), they should anchor to the same capability OR share a justification text explaining why two capabilities are needed. The current setup makes capability-traceability searches fragmented: an audit on CAP-010 won't find BC-3.05.004; an audit on CAP-003 won't find BC-1.12.002.
**Evidence:** BC-3.05.004 line 169 "BC-3.05.004 is the schema contract; BC-1.12.002 is the runtime contract. Both must be consistent: the two-key gate semantics are the same in both"; BC-1.12.002 frontmatter line 16 `capability: "CAP-010"`; BC-3.05.004 frontmatter line 17 `capability: "CAP-003"`
**Routing:** PO + architect (capability anchor reconciliation)
**Suggested fix:** Align BC-3.05.004 to CAP-010 (since it governs the same debug-stream gate) OR amend the Capability Anchor Justification on BC-3.05.004 to explicitly cross-reference CAP-010 and explain the dual-capability anchoring choice (both apply: CAP-003 for the events stream config; CAP-010 for the always-on/debug-stream config). Same fix applies to F-2 above.

### F-19 [MEDIUM]: ARCH-INDEX SS-01 BC count says 114 (106+8 new) but BC-INDEX total_bcs is 1929 — math inconsistency claim

**Artifact:** ARCH-INDEX line 74 + line 85; BC-INDEX line 11
**Policy / Axis:** POLICY 9 (vp_index propagation, by analogy applied to BC-INDEX↔ARCH-INDEX coherence)
**Issue:** ARCH-INDEX line 74 SS-01 BC count: "114 (106 prior + 4 Phase 1a BC-1.12.001–004 + 4 Phase 1b BC-1.12.005/006/007/009 per ADR-015 D-15.1/D-15.2/D-15.3)". 106+4+4 = 114 ✓. ARCH-INDEX line 76 SS-03: "52 (51 prior + 1 Phase 1b BC-3.05.004 v2 schema validation per ADR-015 D-15.4)". 51+1 = 52 ✓. ARCH-INDEX line 85: "**Total BCs: 1,929 (per BC-INDEX; +4 from ADR-015 D-15.1/D-15.2 Phase 1a SS-01 additions BC-1.12.001–BC-1.12.004; +5 from Phase 1b SS-01/SS-03 additions BC-1.12.005/006/007/009 + BC-3.05.004)**". So +4+5 = +9 from ADR-015. BC-INDEX line 11 says total_bcs: 1929. The total assumes prior baseline was 1920. From the BC-INDEX changelog (line 21 "Total BCs 1920→1924" for D-310 Phase 1a +4, then implied 1924+5=1929 for D-313). 1920 baseline + 9 = 1929 ✓. Math is consistent on this axis.

HOWEVER: BC-INDEX line 29 also says: "+3 new SS-01 BCs (BC-1.11.001 VSDD_TRACE_ID injection, BC-1.11.002 FileSink partial-write recovery, BC-1.11.003 atomic dual-emit emit_pair). SS-01 count 103→106." So the SS-01 +3 BC-1.11.001/002/003 is included in the "106 prior" baseline. But BC-1.11.003 is one of the 10 BCs under review and is included in BC-INDEX. ARCH-INDEX SS-01 line 74 explanation "106 prior + 4 + 4 = 114" — the "+4 Phase 1b" enumerates BC-1.12.005/006/007/009 (no BC-1.12.008 because the slot was vacated per D-312 corrigendum). The math is correct but only because BC-1.12.008 was renumbered to BC-3.05.004 (a different subsystem) — that means the SS-01 Phase 1b additions are 4 (BC-1.12.005/006/007/009), and SS-03 Phase 1b addition is 1 (BC-3.05.004), totalling 5. Check: ARCH-INDEX SS-01 = 4 SS-01 Phase 1b ✓. ARCH-INDEX SS-03 = 1 Phase 1b ✓. Total 4+5 = 9 ✓.

The arithmetic is correct, but the description in ARCH-INDEX line 85 collapses Phase 1a (+4) and Phase 1b (+5) without separately tracing BC-1.12.008's renumbering to BC-3.05.004 — a reader cannot follow the renumbering history from ARCH-INDEX alone. Per Axis B (closure-narrative source-of-truth validation), the renumbering should be traced in ARCH-INDEX's narrative.
**Evidence:** ARCH-INDEX lines 74, 76, 85; BC-INDEX line 11 (total_bcs: 1929), line 17 (D-312 BC-3.05.004 row note), line 21 (D-310 Phase 1a +4); E-10 epic line 205 (D-312 corrigendum: "BC-1.12.008 routing corrected: ss-03/BC-3.05.001.md → ss-03/BC-3.05.004.md")
**Routing:** state-manager (ARCH-INDEX narrative amendment)
**Suggested fix:** Add a footnote to ARCH-INDEX line 85 noting "(BC-1.12.008 was originally proposed as SS-01 in D-311 routing; renumbered to BC-3.05.004 (SS-03) in D-312 corrigendum per POLICY 1 ID-collision rule. SS-01 thus has +4 Phase 1b (no BC-1.12.008); SS-03 has +1 Phase 1b (BC-3.05.004))." This makes the renumbering traceable from the index narrative.

### F-20 [MEDIUM]: BC-1.11.003 EC-004 specifies "no-op shim" semantics for post-Wave-3 `emit_pair` use, but the post-Wave-3 ABI surface should not include `emit_pair` at all per ADR-015 Wave 3 closure

**Artifact:** BC-1.11.003 EC-004
**Policy / Axis:** POLICY 4, ADR-015 internal consistency (Axis C)
**Issue:** BC-1.11.003 EC-004 says: "Plugin calls `emit_pair` after Wave 3 shim removal (post-migration) | `emit_pair` remains available but is a no-op shim that calls `emit_event` with only the `new_event` payload; the `old_event` is silently dropped; a stderr warning is emitted that the plugin is using `emit_pair` post-migration". This contradicts ADR-015 §"Wave 3" which states "Remove dual-emit shims (as a sub-task of Wave 3 or immediately after)" and Wave 3 closure announcement event "the dispatcher MUST emit one `vsdd.internal.event_name_deprecated.v1` event per deprecated old-name namespace". Post-Wave-3, the dual-emit migration is OVER. Maintaining `emit_pair` as a "no-op shim that silently drops the old_event" seems to contradict the SOUL #4 principle (silent drop is precisely what we're trying to avoid). Also: ADR-015 OQ-8 resolution says "Wave 3 shim removal eliminates the need entirely post-migration. Legacy two-call shims continue to work until Wave 3."

Why would post-Wave-3 `emit_pair` exist at all? If a plugin has migrated to `emit_pair`, post-Wave-3 the plugin should be calling `emit_event` directly with the new name. If the plugin is on `emit_pair` post-Wave-3, the SDK should have warned at Wave 3 and removed the API. Keeping a "silently drops old_event" shim is a SOUL #4 anti-pattern.
**Evidence:** BC-1.11.003 EC-004 line 173 ("`emit_pair` remains available but is a no-op shim that calls `emit_event` with only the `new_event` payload; the `old_event` is silently dropped"); ADR-015 Migration Plan Wave 3 line 630 ("Remove dual-emit shims (as a sub-task of Wave 3 or immediately after)"); ADR-015 OQ-8 resolution line 852 ("Wave 3 shim removal eliminates the need entirely post-migration")
**Routing:** architect (BC body amendment + ADR cross-check)
**Suggested fix:** Either (a) clarify that `emit_pair` is REMOVED from the SDK post-Wave-3 (so EC-004 is "compile error post-Wave-3, plugin must use `emit_event` directly") and amend BC-1.11.003 §Legacy Two-Call Migration Path; OR (b) replace "silently dropped" with "explicit error returned from `emit_pair` so the plugin author sees the deprecation". The current "silent-drop with stderr warning" is SOUL #4 borderline — stderr warnings are an unreliable observability channel for plugin authors writing log analytics.

### F-21 [LOW]: Multiple BCs use `(pending intent verification)`-eligible language without the tag

**Artifact:** BC-1.12.001 Capability Anchor Justification, BC-1.12.003 Capability Anchor Justification, BC-1.12.004 Capability Anchor Justification, BC-1.12.005 Capability Anchor Justification
**Policy / Axis:** POLICY 5 (creators_justify_anchors), partial-fix regression discipline (S-7.01)
**Issue:** Each of the 4 BCs uses near-identical Capability Anchor Justification text: "...ADR-015 D-15.1 simplifies CAP-003's multi-sink model to single-stream FileSink; the capability remains the canonical anchor because the outcome (events reaching the observability stream for downstream consumers) is unchanged." This is the same justification phrase × 4. Per POLICY 5 ("Stop and ask rather than guess"), each BC should justify its anchor independently against capabilities.md. The repeated phrase suggests cut-and-paste — the BCs may not have individually re-derived the anchor justification. This is borderline LOW (might be intentional consistency) but per "intent adjudication rule" should be flagged for orchestrator/human adjudication: are these BCs intentionally sharing a justification template, or is this evidence of mechanical citation?
**Evidence:** BC-1.12.001 line 176; BC-1.12.003 line 219; BC-1.12.004 line 264; BC-1.12.005 line 180; BC-1.12.007 line 184 (all four contain near-identical text pattern about CAP-003 multi-sink simplification)
**Routing:** PO (intent verification)
**Suggested fix:** Either (a) confirm the shared template is intentional (template-driven anchoring) and add a note in BC-INDEX or PR description, OR (b) differentiate the justifications to reflect each BC's specific anchor relationship to CAP-003. (pending intent verification) — the orchestrator or human adjudicates.

### F-22 [LOW]: BC-1.12.003 H1 says "all 15 OTel Resource attributes" but ADR-015 D-15.2 table actually lists 15 fields including schema_url

**Artifact:** BC-1.12.003 H1 line 28 + Postcondition 1 table
**Policy / Axis:** POLICY 7
**Issue:** BC-1.12.003 H1 says "all 15 OTel Resource attributes". Counting ADR-015 D-15.2 Resource attributes table: service.name, service.namespace, service.instance.id, service.version, deployment.environment.name, host.name, host.id, os.type, process.pid, vcs.repository.url.full, vcs.repository.name, vcs.provider.name, vcs.owner.name, worktree.id, schema_url = 15 ✓. Count is correct. BUT BC-1.12.003 Postcondition 1 also lists 15 in its table. Math is consistent. This is a NON-finding — actually a successful audit. (Posting as LOW to acknowledge the verify pass.) [Note: I am keeping this row in the report as a positive verification entry — convertible to "Observation" in a polish pass.]

## Per-Policy Compliance

| # | Policy | Status | Notes |
|---|--------|--------|-------|
| 1 | append_only_numbering | PASS | BC-3.05.001/002/003 retired with `superseded_by: ADR-015` per D-312 corrigendum (POLICY 1 honored — bodies preserved verbatim, IDs not reused). BC-1.12.008 slot intentionally vacated per D-312 corrigendum. SS-01 BC sequence BC-1.12.001-007/009 honors append-only. No findings. |
| 2 | lift_invariants_to_bcs | FAIL (HIGH) | F-4: 5+ DIs (DI-007, DI-008, DI-011, DI-012, DI-013, DI-014, DI-017) materially affected by ADR-015 are not cited in any of 10 BCs. All 10 BCs have `L2 Domain Invariants | TBD`. Pattern flag (3+ orphans). |
| 3 | state_manager_runs_last | PASS (with caveat) | BC-INDEX changelog rows D-310/D-311/D-312/D-313 show state-manager updates following architect/PO authoring. ARCH-INDEX SS-01/SS-03 counts updated to 114/52 reflecting D-313. STORY-INDEX v2.17 contains E-10 BC populations. State-manager appears to have run last in each burst. No regressions detected. |
| 4 | semantic_anchoring_integrity | FAIL (CRITICAL) | F-1: epic anchored to CAP-011/CAP-015 which describe unrelated capabilities. F-2: 7 BCs anchored to CAP-003 whose canonical name contradicts ADR-015. F-7/F-8: BC↔Story bidirectional links missing for BC-1.12.005/003/004 in S-10.04. F-12: BC-1.12.009 H1 names "four-state" but body has 5 states. F-13: BC-1.12.002 H1 imprecise relative to OQ-W16-011 resolution. |
| 5 | creators_justify_anchors | FAIL (HIGH) | F-6: BC-1.11.003 has CAP-TBD persisting across two sealed bursts. F-21: 4 BCs use near-identical CAP-003 justification text suggesting mechanical citation. F-1/F-2: epic and BCs anchor to capabilities without justification cross-checked against capabilities.md. |
| 6 | architecture_is_subsystem_name_source_of_truth | FAIL (HIGH) | F-5: S-10.05 missing SS-02 in subsystems despite modifying crates/hook-sdk/. ARCH-INDEX subsystem names verified: "SS-01 Hook Dispatcher Core", "SS-03 Event Emission (OTel-Aligned)", "SS-04 Plugin Ecosystem" all match BCs. |
| 7 | bc_h1_is_title_source_of_truth | FAIL (HIGH) | F-13: BC-1.12.002 H1 imprecise vs OQ-W16-011 resolution; BC-INDEX propagates the same imprecise H1. F-12: BC-1.12.009 H1 says "four-state" when body specifies 5. |
| 8 | bc_array_changes_propagate_to_body_and_acs | FAIL (HIGH) | F-7: BC-1.12.005 implementation in S-10.04 not reflected in S-10.04 frontmatter. F-8: S-10.04 modifies BC-1.12.003/004 functionality without listing them in frontmatter. |
| 9 | vp_index_is_vp_catalog_source_of_truth | N/A | All 10 BCs have `(TBD — to be assigned after S-10.NN story authoring)` in VP Anchors / Verification Properties. No VPs in scope yet; VP-INDEX propagation check is N/A. F-19 documents BC-INDEX↔ARCH-INDEX arithmetic check (analog axis): math is correct but renumbering history (BC-1.12.008 → BC-3.05.004) is not narrated in ARCH-INDEX. MEDIUM. |
| 10 | demo_evidence_story_scoped | N/A | Spec-package review pre-implementation. |
| 11 | no_test_tautologies | N/A | Spec-package review pre-implementation. |
| 12 | bc_tv_emitter_consistency | PASS | Sampled BC-1.12.004 Canonical Test Vectors against Postconditions: no TV-row marks a field excluded that conflicts with the BC's postcondition. BC-1.12.005 EC-002 ("Plugin supplies NO host-owned fields | event.host_overrides absent") matches Invariant 1. No findings. |

## Per-Axis Compliance

| Axis | Status | Notes |
|------|--------|-------|
| A. TD-VSDD-091 stable-anchor citations | FAIL (HIGH) | F-3: Multiple `line N` self-references in BC-1.12.001/004/007 (`main.rs:143`, `host/mod.rs:109-116`, `sinks/mod.rs lines 11-15`, `host/mod.rs:113`). Pattern flag (4+ recurrences). |
| B. TD-VSDD-093 closure-narrative source-of-truth validation | FAIL (MEDIUM) | F-16: S-10.05 cites "ADR-015 11 plugins" claim but ADR-015 has no such enumeration. Sourced verification failed. F-19: BC-INDEX↔ARCH-INDEX arithmetic correct but renumbering history (BC-1.12.008→BC-3.05.004) not narrated in ARCH-INDEX. F-15: BC-1.12.001 Postcondition 3 makes a claim ("Silent discard is no longer the failure mode on the critical path") that is contradicted by its own §TD-VSDD-092 verification section. |
| C. ADR-015 internal consistency (D-15.1, D-15.2, D-15.3, D-15.4) | FAIL (HIGH) | F-20: BC-1.11.003 EC-004 post-Wave-3 `emit_pair` semantics conflict with ADR-015 Wave 3 shim-removal. F-12: BC-1.12.009 four-state vs ADR-015 D-15.2.e five-state taxonomy. F-14: epic narrative "six-wave" vs frontmatter wave: 17 disambiguation gap. |
| D. OQ-W16-011 RESOLVED enforcement | PASS (with H1 caveat F-13) | BC-1.12.002 body, BC-3.05.004 body, ADR-015 D-15.1, and SS-03-event-emission.md all consistently implement 12-factor override semantics. EC-007/EC-007b in BC-1.12.002 honor "MUST" (not "MAY") per D-311 amendment. F-13: H1 imprecision is the only gap. |
| E. OQ-W16-012 RESOLVED enforcement | PASS | OQ-W16-012 resolution (mark BC-3.05.001/002/003 superseded NOW in D-312) verified: BC-INDEX line 17 confirms status update; OQ-W16-012 resolution text confirms architect action; BC-3.05.004 Related BCs section explicitly notes "BC-3.05.001/002/003 — RETIRED ... DO NOT USE as a behavioral reference". All three retired BCs correctly cross-referenced. |
| F. Dual-emit four-state classification (BC-1.12.009) | FAIL (HIGH) | F-12: "Four-state" naming vs five actual states (paired-current, paired-deprecated, orphaned-deprecated-half, orphaned-current-half, non-paired); plus malformed (EC-005) is a 6th state. Mutually exclusive AND collectively exhaustive claim of Postcondition 4 is false as stated. |
| G. Wave-1 call-graph invariant testability (BC-1.12.007) | FAIL (HIGH) | F-9: Postcondition 1 claims transitive-reachability scope; Invariant 1 claims direct-call enforcement; testability hand-waved between "static analysis or integration test"; TD-015-a CI check deferred without explicit owner/decision-by date. |
| H. CAP-008 anchor justification (BC-1.12.006) | PASS | BC-1.12.006 anchored to CAP-008 ("Gate tool calls with pre-execution behavioral checks (PreToolUse hooks)") with explicit justification: "the block-audit event is the observable evidence that the gating capability fired ... CAP-008's block enforcement is invisible to auditors without this event." Per capabilities.md CAP-008 line 49, the description matches. POLICY 5 satisfied. |
| I. Cross-BC traceability (BC-1.12.004 → S-10.02 + S-10.03; BC-1.12.007 → S-10.02 + S-10.09) | PASS (with F-8 caveat) | BC-1.12.004 Story Anchor names S-10.02 + S-10.03; both stories have BC-1.12.004 in frontmatter ✓. BC-1.12.007 Story Anchor names S-10.02 + S-10.09; both stories have BC-1.12.007 in frontmatter ✓. F-8: S-10.04 ALSO modifies registry from BC-1.12.004 but doesn't trace it. |

## Convergence Status

VERDICT: CRITICAL — convergence pass-counter MUST be RESET to 0.

Pass 1 of N. NITPICK_ONLY pass-counter = 0.

## Summary of routing

| Routing | Findings |
|---------|----------|
| PO | F-6, F-7, F-8, F-9, F-10, F-11, F-12, F-13, F-15, F-17, F-18, F-21, F-22 |
| architect | F-1 (capability registry update), F-2 (capabilities.md amendment), F-3 (line-N citations), F-4 (DI updates), F-18 (capability anchor reconciliation), F-20 (ADR cross-check) |
| story-writer | F-5 (S-10.05 SS-02), F-7 (S-10.04 frontmatter), F-8 (S-10.04 frontmatter+ACs), F-14 (story body wave disambiguation), F-16 (S-10.05 plugin count) |
| state-manager | F-13 (BC-INDEX H1 sync), F-19 (ARCH-INDEX renumbering narrative) |
| process-gap follow-up | F-3 (line-N pattern across 4+ BCs); F-6 (CAP-TBD persisting across bursts) |

## Process-gap candidates (S-7.02)

- **[process-gap]** F-3: `line N` citations forbidden by TD-VSDD-091 found in 4+ BC files. Per "3+ recurrences = process gap" rule, codification follow-up: a hook or rule-file enforcement that BC bodies cannot contain `:NNN` or `lines NNN-NNN` patterns. Suggested rule-file location: `.factory/rules/lessons-codification.md` or a new BC-author-template guard.
- **[process-gap]** F-6: BC-1.11.003 shipped with `capability: "CAP-TBD"` across two sealed bursts (D-310 → D-313). Per POLICY 5 "Stop and ask rather than guess", agents should not be able to advance a `status: draft` BC across burst boundaries with CAP-TBD persisting. Suggested rule: a same-burst gate that flags CAP-TBD for resolution before burst seal.
- **[process-gap]** F-21: Mechanical-citation pattern across 4 BC bodies sharing the same Capability Anchor Justification text suggests cut-and-paste. Per POLICY 5, each anchor justification should be re-derived. Suggested rule: an adversary-prompt amendment to flag near-identical justification text across siblings as POLICY 5 LOW with intent-verification escalation.
