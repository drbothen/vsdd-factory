---
document_type: verification-coverage-matrix
level: L4
section: verification-coverage-matrix
version: "1.7"
status: draft
producer: architect
timestamp: 2026-06-24T00:00:00Z
last_amended: "2026-07-16 (v1.7) — S-19.07 Phase-B POLICY 9 propagation (architect): VP-095 SS-04 row title updated to Phase-B form; ✓ moved from I column to U column; SS-04 subtotal U 4→5, I 7→6; Grand Total U 46→47, I 34→33; input-hash c9ec678→f0fab9c. [Prior: 2026-07-13 (v1.6) — pass-11 F-P11-001 POLICY 9 propagation (architect): VP-099 SS-07 row title updated to both-ends form ('Starts With ^' → 'Is Fully Anchored (^...$)'); input-hash 893a501→c9ec678. [Prior: 2026-07-08 (v1.5) — E-19 pass-28 VP-096 boundary-wording sync (architect): VP-096 SS-04 row title updated to exclusive form per BC-4.13.001 §Invariant 9 adjudication (D-781): 'Byte-Exact Prefix Through Second --- Delimiter' → 'Byte-Exact Prefix Up To (Excluding) Second --- Delimiter Line'. input-hash 7a7ac8c→893a501. [Prior: 2026-07-06 (v1.4) — E-19 VP package POLICY 9 propagation (architect): VP-094 added to SS-05 module table (I; BC-5.42.001); VP-095/096 added to SS-04 (I + P; BC-4.13.001); VP-097/098/100/101 added to SS-01 (K + I + I + I; BC-2.07.001+BC-2.02.011 + BC-2.07.001 + BC-3.08.001+DI-019 + BC-1.17.001); VP-099 added to SS-07 (I; no BC). SS-01 subtotal K 2→3 (+VP-097), I 7→10 (+VP-098/100/101), row total 32→36. SS-04 subtotal P 1→2 (+VP-096), I 6→7 (+VP-095), row total 13→15. SS-05 subtotal I 5→6 (+VP-094), row total 12→13. SS-07 subtotal I 4→5 (+VP-099), row total 11→12. Grand Total K 4→5, P 4→5, I 28→34, row total 93→101. Per-tool arithmetic 5+5+46+34+10+1=101 VERIFIED. Per-subsystem row-sum 36+5+14+15+13+5+12+1=101 VERIFIED. §2 VP-100 judgment call added (SS-01 primary; SS-03 secondary). §3 Grand Totals updated. [Prior: 2026-06-24 (v1.3) — S-18.04b-prereq BC authoring burst (architect POLICY 9 propagation): VP-093 added to SS-01 module table (integration; SS-01; DI-020, DI-025; BC-1.16.001 — dispatcher git_context injection on PostToolUse Bash git-commit events; four-field completeness; fail-open on git error; no injection on non-qualifying events). SS-01 subtotal I 6→7, row total 31→32. Grand Total I 27→28, row total 92→93. Per-tool arithmetic 4+4+46+28+10+1=93 VERIFIED. Per-subsystem row-sum 32+5+14+13+12+5+11+1=93 VERIFIED. [Prior: 2026-06-16 (v1.2) — D-615 E-18 STORY PASS-1 FIX WAVE INTEGRATION BURST (state-manager POLICY 9 propagation): VP-092 added to SS-06 module table (unit-test; SS-06; DI-020; BC-6.25.001 — check-state-health CLAUDE_AUTOCOMPACT_PCT_OVERRIDE advisory check; never blocks). SS-06 subtotal U 0→1, row total 4→5. Grand Total U 45→46, row total 91→92. Per-tool arithmetic 4+4+46+27+10+1=92 VERIFIED. Per-subsystem row-sum 31+5+14+13+12+5+11+1=92 VERIFIED. [Prior: 2026-06-16 (v1.1) — D-612 INTEGRATION BURST (state-manager POLICY 9 propagation): VP-091 added to SS-04 module table (unit-test; SS-04; DI-020; BC-4.15.001 — validate-heavy-op-delegation always-Continue advisory gate). SS-04 subtotal U 3→4, row total 12→13. Grand Total U 44→45, row total 90→91. Per-tool arithmetic 4+4+45+27+10+1=91 VERIFIED. Per-subsystem row-sum 31+5+14+13+12+4+11+1=91 VERIFIED. [Prior: 2026-06-16 (v1.0) — F2 gate decision: initial creation as a full production-grade architecture deliverable. Sources: VP-INDEX.md v2.29 (86 VPs) + VP-087..VP-090 (4 new E-18 VPs, unstaged). POST-INTEGRATION totals: total_vps=90, unit-test=44, integration=27, manual=10, static-check=1, kani-proof=4, proptest=4. Every VP assigned to its authoritative module per VP-INDEX.md scope column and VP file frontmatter. Authored per F2 gate human directive that deferred architecture derived-views be materialized now.]]"
modified:
  - "2026-07-16 (v1.7) — S-19.07 Phase-B POLICY 9 propagation: VP-095 SS-04 row title→Phase-B form; ✓ I→U; SS-04 subtotal U 4→5, I 7→6; Grand Total U 46→47, I 34→33; input-hash c9ec678→f0fab9c"
  - "2026-07-13 (v1.6) — pass-11 F-P11-001 POLICY 9 propagation: VP-099 SS-07 row title updated to both-ends form; input-hash 893a501→c9ec678"
  - "2026-07-08 (v1.5) — E-19 pass-28 VP-096 boundary-wording sync: VP-096 SS-04 row title updated to exclusive form per BC-4.13.001 §Invariant 9 (D-781); input-hash 7a7ac8c→893a501"
  - "2026-07-06 (v1.4) — E-19 VP package POLICY 9 propagation: VP-094 (SS-05), VP-095/096 (SS-04), VP-097/098/100/101 (SS-01), VP-099 (SS-07) added; K 4→5, P 4→5, I 28→34, Total 93→101; §2 VP-100 judgment call added; input-hash 61531bf→7a7ac8c"
  - "2026-06-24 (v1.3) — VP-093 added to SS-01 module; SS-01 I 6→7, total 31→32; grand total 92→93"
  - "2026-06-16 (v1.2) — D-615 VP-092 added to SS-06 module; SS-06 U 0→1, total 4→5; grand total 91→92"
  - "2026-06-16 (v1.1) — D-612 VP-091 added to SS-04 module; SS-04 U 3→4, total 12→13; grand total 90→91"
  - "2026-06-16 (v1.0 initial creation)"
phase: 1b
inputs: [verification-properties/VP-INDEX.md]
input-hash: "f0fab9c"
traces_to: VP-INDEX.md
subsystems_affected:
  - SS-01
  - SS-02
  - SS-03
  - SS-04
  - SS-05
  - SS-06
  - SS-07
  - SS-09
---

# Verification Coverage Matrix

> **Source-of-truth relationship:** VP-INDEX.md is the authoritative VP catalog.
> This matrix derives from VP-INDEX.md §Full Index (scope column). Any change to
> VP-INDEX — VP addition, retirement, module reassignment, tool change, or phase
> reassignment — MUST propagate to this matrix in the same burst (POLICY 9 /
> VP-INDEX Propagation Obligation).
>
> **Module assignment rule:** Each VP is assigned to exactly one primary subsystem.
> When a VP lists multiple subsystems, the PRIMARY subsystem is the first-listed
> subsystem in VP-INDEX.md §Full Index Scope column, consistent with the VP file's
> frontmatter `scope:` field. Multi-subsystem VPs appear in exactly one module table;
> their additional subsystem affiliations are noted in the Subsystems column.
>
> **Grand-total arithmetic invariant:** Each VP is counted exactly once in the grand-
> total row. The per-tool column sums (K+P+U+I+M+S) must equal 101. This invariant
> must be verified on every update to this document.

## §Changelog

| Version | Date | Author | Change |
|---------|------|--------|--------|
| v1.7 | 2026-07-16 | architect | S-19.07 Phase-B POLICY 9 propagation: VP-095 SS-04 row title updated to Phase-B form ('verify-factory-lock Never Receives output_too_large for Any STATE.md Size — Structural Guarantee via host::read_prefix(8192) (BC-1.17.001 PC-3); Large-STATE.md Frontmatter Correctly Parsed from 8192-Byte Prefix'); ✓ moved from I column to U column (proof_method integration→unit+static per VP-095 v1.2 amendment). SS-04 subtotal U 4→5, I 7→6, row total 15 unchanged. Grand Total U 46→47, I 34→33, total 101 unchanged. Per-tool arithmetic 5+5+47+33+10+1=101 VERIFIED. Per-subsystem row-sum 36+5+14+15+13+5+12+1=101 VERIFIED. input-hash c9ec678→f0fab9c. |
| v1.6 | 2026-07-13 | architect | pass-11 F-P11-001 POLICY 9 propagation: VP-099 SS-07 row title updated to both-ends form ('Starts With ^' → 'Is Fully Anchored (^...$)'). input-hash 893a501→c9ec678. |
| v1.5 | 2026-07-08 | architect | E-19 pass-28 VP-096 boundary-wording sync: VP-096 SS-04 row title updated to exclusive form per BC-4.13.001 §Invariant 9 adjudication (D-781) — 'Byte-Exact Prefix Through Second --- Delimiter' → 'Byte-Exact Prefix Up To (Excluding) Second --- Delimiter Line'. input-hash 7a7ac8c→893a501. |
| v1.4 | 2026-07-06 | architect | E-19 VP package POLICY 9 propagation: VP-094 added to SS-05 (I; BC-5.42.001; S-19.01); VP-095/096 added to SS-04 (I + P; BC-4.13.001; S-19.02); VP-097/098/100/101 added to SS-01 (K + I + I + I; BC-2.07.001+BC-2.02.011 + BC-2.07.001 + BC-3.08.001+DI-019 + BC-1.17.001); VP-099 added to SS-07 (I; no BC; S-19.04). All 8 abbreviated titles corrected from prior placeholder values. SS-01 subtotal K 2→3, I 7→10, row 32→36. SS-04 subtotal P 1→2, I 6→7, row 13→15. SS-05 subtotal I 5→6, row 12→13. SS-07 subtotal I 4→5, row 11→12. Grand Total K 4→5, P 4→5, I 28→34, total 93→101. Per-tool arithmetic 5+5+46+34+10+1=101 VERIFIED. Per-subsystem row-sum 36+5+14+15+13+5+12+1=101 VERIFIED. §2 VP-100 judgment call added (SS-01 primary; SS-03 secondary). |
| v1.3 | 2026-06-24 | architect | S-18.04b-prereq POLICY 9 propagation: VP-093 added to SS-01 module table (integration; SS-01; DI-020, DI-025; BC-1.16.001 — dispatcher git_context injection on PostToolUse Bash git-commit events; four-field injection; fail-open on git error; no injection on non-qualifying events; exec-free WASM boundary; HOST_ABI_VERSION unchanged; anchor S-18.04b-prereq). SS-01 subtotal I 6→7, row total 31→32. Grand Total I 27→28, row total 92→93. Per-tool arithmetic 4+4+46+28+10+1=93 VERIFIED. Per-subsystem row-sum 32+5+14+13+12+5+11+1=93 VERIFIED. |
| v1.2 | 2026-06-16 | state-manager | D-615 POLICY 9 propagation: VP-092 added to SS-06 module table (unit-test; SS-06; DI-020; BC-6.25.001 — check-state-health CLAUDE_AUTOCOMPACT_PCT_OVERRIDE advisory check; never blocks; PC1 absent→ADVISORY; PC2 >80→ADVISORY; PC3 <=80→PASS). SS-06 subtotal U 0→1, row total 4→5. Grand Total U 45→46, row total 91→92. Per-tool arithmetic 4+4+46+27+10+1=92 VERIFIED. Per-subsystem row-sum 31+5+14+13+12+5+11+1=92 VERIFIED. |
| v1.1 | 2026-06-16 | state-manager | D-612 POLICY 9 propagation: VP-091 added to SS-04 module table (unit-test; SS-04; DI-020; BC-4.15.001 — validate-heavy-op-delegation always-Continue advisory gate). SS-04 subtotal U 3→4, row total 12→13. Grand Total U 44→45, row total 90→91. Per-tool arithmetic 4+4+45+27+10+1=91 VERIFIED. Per-subsystem row-sum 31+5+14+13+12+4+11+1=91 VERIFIED. |
| v1.0 | 2026-06-16 | architect | Initial creation — F2 gate decision. Sources: VP-INDEX.md v2.29 (86 VPs) + VP-087..VP-090 (4 new E-18 VPs). POST-INTEGRATION totals: total_vps=90, unit-test=44, integration=27, manual=10, static-check=1, kani-proof=4, proptest=4. |

---

## §1 Coverage by Module (VP-to-Module Table)

Column key:
- **K** = kani-proof
- **P** = proptest
- **U** = unit-test
- **I** = integration
- **M** = manual
- **S** = static-check

---

### Module: crates/factory-dispatcher (SS-01 — Hook Dispatcher Core)

VPs whose primary subsystem is SS-01. Includes multi-subsystem VPs where SS-01 is
first-listed (VP-007, VP-008, VP-009, VP-026, VP-051, VP-073, VP-074, VP-075, VP-077,
VP-086, VP-100 per assignment notes in §2 below).

| VP ID | Title (abbreviated) | Subsystems | K | P | U | I | M | S |
|-------|---------------------|-----------|---|---|---|---|---|---|
| VP-001 | Tier Execution Is Sequential; Intra-Tier Is Parallel | SS-01 | | | | ✓ | | |
| VP-002 | Plugin Crash or Timeout Does Not Block Sibling Plugins | SS-01 | | | | ✓ | | |
| VP-003 | block_intent Is Aggregate; Tier Runs to Completion | SS-01 | | | ✓ | | | |
| VP-004 | Capability Denial Produces Return Code AND Audit Event | SS-01 | | | ✓ | | | |
| VP-005 | Shell Interpreters Require Explicit shell_bypass_acknowledged | SS-01 | | | ✓ | | | |
| VP-006 | Setuid/Setgid Binaries Refused Unconditionally | SS-01 | | | ✓ | | | |
| VP-007 | Dispatcher Self-Telemetry Is Always-On and Never Panics | SS-01, SS-03 | | | ✓ | | | |
| VP-008 | Internal Log Filename Derived from Event Timestamp | SS-01, SS-03 | | | ✓ | | | |
| VP-009 | prune_old Removes Only Dispatcher-Internal Files | SS-01, SS-03 | | | ✓ | | | |
| VP-010 | Plugin Stderr Capped at 4 KiB with Truncation Marker | SS-01 | | | ✓ | | | |
| VP-014 | Schema Version Mismatch Is a Hard Load Error | SS-01 | | | ✓ | | | |
| VP-016 | Each Registry Entry Sees Only Its Own plugin_config | SS-01 | | | ✓ | | | |
| VP-017 | dispatcher_trace_id Present on Every Emitted Event | SS-01 | | | ✓ | | | |
| VP-018 | Registry Rejects Malformed Configurations at Load Time | SS-01 | | | ✓ | | | |
| VP-019 | Routing Is Deterministic — Same Input Yields Same Plugin Selection | SS-01 | | | ✓ | | | |
| VP-020 | Epoch Timeout Rounds Up and Terminates Infinite Loops | SS-01 | | | ✓ | | | |
| VP-021 | Capability Deny-by-Default | SS-01 | | | ✓ | | | |
| VP-022 | Dispatcher Exit Code Semantics — 0 Non-Block, 2 Block | SS-01 | | | ✓ | | | |
| VP-023 | Wire Format Decoders Reject Truncated Input Without Panic | SS-01, SS-02 | | | ✓ | | | |
| VP-024 | Plugin Cache Is Keyed by Path and Invalidated by mtime | SS-01 | | | ✓ | | | |
| VP-025 | Host Function ABI Surface Is Complete and Stable | SS-01, SS-02 | | | | ✓ | | |
| VP-026 | InternalEvent Serializes Flat with No Null Optional Fields | SS-01, SS-03 | | | ✓ | | | |
| VP-027 | HookPayload Parsing Is Robust for All Envelope Types | SS-01 | | | ✓ | | | |
| VP-050 | exec_subprocess Timeout Is Enforced — Hung Commands Are Killed | SS-01 | | | ✓ | | | |
| VP-051 | Dispatcher Startup Flow Writes Parseable JSONL | SS-01, SS-03 | | | | ✓ | | |
| VP-052 | Epoch Ticker Shuts Down Cooperatively and Idempotently | SS-01 | | | ✓ | | | |
| VP-073 | Resolver-Load Purity | SS-01, SS-04 | | | | ✓ | | |
| VP-074 | Resolver-Error Isolation | SS-01, SS-04 | ✓ | | | | | |
| VP-075 | Context-Injection Determinism | SS-01, SS-04 | | ✓ | | | | |
| VP-077 | Dispatcher Partition Correctness (6 properties) | SS-01 | ✓ | | | | | |
| VP-086 | Dispatcher Exit-2 Propagation for PreCompact Block-Intent | SS-01, SS-04 | | | | ✓ | | |
| VP-093 | Dispatcher Injects git_context Into payload.extra on PostToolUse Bash git-commit Events; Fail-Open on Git Error | SS-01 | | | | ✓ | | |
| VP-097 | path_util::resolve_path_for_allowlist Traversal Defense — .. Sequences Cannot Escape Allowlist Prefixes | SS-01 | ✓ | | | | | |
| VP-098 | Allowlisted-but-Absent File Returns NOT_FOUND (-5); Zero CAPABILITY_DENIED False-Positives | SS-01 | | | | ✓ | | |
| VP-100 | Drain-Timer Expiry Emits Exactly One plugin.abandoned Per In-Flight; No plugin.completed Follows for Same Trace | SS-01, SS-03 | | | | ✓ | | |
| VP-101 | host::read_prefix Returns Byte-Exact Prefix; Never OUTPUT_TOO_LARGE; Absent Returns NOT_FOUND (-5) | SS-01 | | | | ✓ | | |
| **SS-01 subtotal** | | | **3** | **1** | **22** | **10** | **0** | **0** |

---

### Module: crates/hook-sdk (SS-02 — Hook SDK and Plugin ABI)

| VP ID | Title (abbreviated) | Subsystems | K | P | U | I | M | S |
|-------|---------------------|-----------|---|---|---|---|---|---|
| VP-038 | SDK HookResult Exit Codes Are Stable | SS-02 | | | ✓ | | | |
| VP-039 | SDK Wire Format Encoding Is Symmetric | SS-02 | | | ✓ | | | |
| VP-040 | SDK HookPayload Round-Trips via Serde | SS-02 | | | ✓ | | | |
| VP-041 | SDK Panic Handler Extracts Message | SS-02 | | | ✓ | | | |
| VP-042 | SDK HostError Code Mapping Is Stable | SS-02 | | | ✓ | | | |
| **SS-02 subtotal** | | | **0** | **0** | **5** | **0** | **0** | **0** |

---

### Module: crates/sink-core, crates/sink-file (SS-03 — Event Emission)

| VP ID | Title (abbreviated) | Subsystems | K | P | U | I | M | S |
|-------|---------------------|-----------|---|---|---|---|---|---|
| VP-011 | Sink submit Must Not Block the Dispatcher | SS-03 | | | ✓ | | | |
| VP-012 | Sink Failure Affects Only That Sink | SS-03 | | | ✓ | | | |
| VP-013 | Unknown Sink Driver Types Are Non-Fatal | SS-03 | | | ✓ | | | |
| VP-028 | Sink Fan-Out — Every Event Reaches Every Accepting Sink | SS-03 | | | | ✓ | | |
| VP-029 | File Sink Path Template Substitutes {date}, {name}, {project} | SS-03 | | | ✓ | | | |
| VP-030 | Sink Shutdown Drains Queued Events | SS-03 | | | ✓ | | | |
| VP-031 | Tag Enrichment Does Not Overwrite Producer Fields | SS-03 | | | ✓ | | | |
| VP-032 | RoutingFilter Default Accepts All Events | SS-03 | | | ✓ | | | |
| VP-033 | OTLP LogRecord Mapping Is Correct | SS-03 | | | | ✓ | | |
| VP-034 | OTLP Sink Batch Trigger Thresholds Are Independent | SS-03 | | | ✓ | | | |
| VP-035 | File Sink Auto-Creates Missing Parent Directories | SS-03 | | | ✓ | | | |
| VP-036 | Disabled Sink Drops Every Event Without Writing | SS-03 | | | ✓ | | | |
| VP-037 | OTLP Resource Attributes — Operator Overrides Win | SS-03 | | | ✓ | | | |
| VP-079 | Async-Semantics Event Types — Payload Schema Conformance | SS-03 | | | | ✓ | | |
| **SS-03 subtotal** | | | **0** | **0** | **11** | **3** | **0** | **0** |

---

### Module: crates/hook-plugins/* (SS-04 — Plugin Ecosystem)

VPs where SS-04 is the first-listed subsystem. Does not include multi-subsystem VPs
where SS-01 or SS-05 is listed first (see assignment notes in §2).

| VP ID | Title (abbreviated) | Subsystems | K | P | U | I | M | S |
|-------|---------------------|-----------|---|---|---|---|---|---|
| VP-044 | Legacy Bash Adapter Exit Code Mapping | SS-04, SS-07 | | | ✓ | | | |
| VP-045 | Legacy Bash Adapter Strips plugin_config | SS-04 | | | ✓ | | | |
| VP-065 | Session-Start Plugin Surface Invariant | SS-04 | | | | ✓ | | |
| VP-066 | Session-End Plugin Surface Invariant | SS-04 | | | | ✓ | | |
| VP-067 | Worktree Hook Plugin Surface Invariant | SS-04 | | | | ✓ | | |
| VP-068 | Tool-Failure Hook Plugin Surface Invariant | SS-04 | | | | ✓ | | |
| VP-069 | validate-artifact-path Registry-Load Purity | SS-04 | | ✓ | | | | |
| VP-070 | validate-artifact-path Path-Pattern Matching | SS-04 | ✓ | | | | | |
| VP-071 | validate-per-story-adversary-convergence Block Invariant | SS-04 | ✓ | | | | | |
| VP-072 | artifact-path-registry.yaml Single Source of Truth | SS-04 | | | | ✓ | | |
| VP-076 | Resolver-Capability Confinement | SS-04 | | | | ✓ | | |
| VP-083 | Completeness Gate Is No-Op on Wave-1 or Non-HANDOFF.md Writes | SS-04 | | | ✓ | | | |
| VP-091 | validate-heavy-op-delegation Emits DelegationRecommended Advisory (Never Blocks) | SS-04 | | | ✓ | | | |
| VP-095 | verify-factory-lock Never Receives output_too_large for Any STATE.md Size — Structural Guarantee via host::read_prefix(8192) (BC-1.17.001 PC-3); Large-STATE.md Frontmatter Correctly Parsed from 8192-Byte Prefix | SS-04 | | | ✓ | | | |
| VP-096 | extract_frontmatter Purity — Byte-Exact Prefix Up To (Excluding) Second --- Delimiter Line; Deterministic | SS-04 | | ✓ | | | | |
| **SS-04 subtotal** | | | **2** | **2** | **5** | **6** | **0** | **0** |

---

### Module: plugins/vsdd-factory/agents, workflows (SS-05 — Pipeline Orchestration)

VPs where SS-05 is the first-listed or primary subsystem.

| VP ID | Title (abbreviated) | Subsystems | K | P | U | I | M | S |
|-------|---------------------|-----------|---|---|---|---|---|---|
| VP-053 | Lobster Workflow DAG Is Acyclic | SS-05 | | | | | ✓ | |
| VP-054 | Workflow Loop Blocks Are Bounded | SS-05 | | | | | ✓ | |
| VP-055 | state-manager Runs Last in Every Burst | SS-05 | | | | | ✓ | |
| VP-056 | on_failure Semantics — retry → escalate → abort | SS-05 | | | | | ✓ | |
| VP-057 | Adversarial Review Convergence | SS-05 | | | | | ✓ | |
| VP-061 | Agent Prompt Discipline Rules Present in All Three Agent Files | SS-05 | | | | | | ✓ |
| VP-062 | S-7.02 Process-Codification Surface Invariant | SS-05, SS-07, SS-08 | | | | ✓ | | |
| VP-063 | RED_RATIO computation correctness | SS-05 | | | | ✓ | | |
| VP-064 | facade-mode mutation gate enforcement | SS-05, SS-06 | | | | | ✓ | |
| VP-081 | Wave Cannot Close Without Verified Handoff (wave_id > 1) | SS-04, SS-05, SS-07 | | | | ✓ | | |
| VP-084 | PreCompact Flush Commit Is Lifecycle-Distinct | SS-05, SS-04 | | | | ✓ | | |
| VP-087 | wave-state.yaml Produced Atomically With HANDOFF.md | SS-05 | | | | ✓ | | |
| VP-094 | pr-manager READY-Verdict Covered-SHA Pin, Stale-Verdict Halt, Release-PR Merge-Strategy Enforcement | SS-05 | | | | ✓ | | |
| **SS-05 subtotal** | | | **0** | **0** | **0** | **6** | **6** | **1** |

> **Assignment note (VP-081):** VP-081 lists scope SS-04, SS-05, SS-07. The primary
> owning subsystem is SS-05 (Pipeline Orchestration) because the behavioral contract
> is BC-5.41.001 — a wave-gate orchestration step. The WASM gate (SS-04) and shell
> script (SS-07) are components invoked by the orchestration step.
>
> **Assignment note (VP-084):** VP-084 lists scope SS-05, SS-04. The lifecycle-
> distinctness invariant (BC-5.41.003) governs when the MULTI_COMMIT_CHAIN_NOT_ALLOWED
> detector is suppressed — an orchestration policy. SS-05 is primary.

---

### Module: plugins/vsdd-factory/skills (SS-06 — Skill Catalog)

| VP ID | Title (abbreviated) | Subsystems | K | P | U | I | M | S |
|-------|---------------------|-----------|---|---|---|---|---|---|
| VP-058 | create-adr Atomicity | SS-06 | | | | ✓ | | |
| VP-059 | ID Monotonicity — Allocated ADR-NNN | SS-06 | | ✓ | | | | |
| VP-060 | Bidirectional Supersession | SS-06 | | | | ✓ | | |
| VP-088 | rehydrate-wave Reads wave-state.yaml From Git | SS-06 | | | | ✓ | | |
| VP-092 | check-state-health CLAUDE_AUTOCOMPACT_PCT_OVERRIDE Advisory Check (Never Blocks) | SS-06 | | | ✓ | | | |
| **SS-06 subtotal** | | | **0** | **1** | **1** | **3** | **0** | **0** |

---

### Module: plugins/vsdd-factory/hooks/*.sh, hooks-registry.toml (SS-07 — Hook Bash Layer)

VPs where SS-07 is the first-listed subsystem. Note VP-043 lists SS-07, SS-01 —
SS-07 is primary because the property tests the registry file (hooks-registry.toml),
not the dispatcher routing engine.

| VP ID | Title (abbreviated) | Subsystems | K | P | U | I | M | S |
|-------|---------------------|-----------|---|---|---|---|---|---|
| VP-043 | Every hooks-registry.toml Entry Routes Through legacy-bash-adapter.wasm | SS-07, SS-01 | | | | ✓ | | |
| VP-046 | All hooks-registry.toml Entries Correspond to Registered Hook Scripts | SS-07 | | | | | ✓ | |
| VP-047 | Validator Hooks Exit 0 or 2 — No Other Codes | SS-07 | | | | | ✓ | |
| VP-048 | protect-secrets.sh Fails Closed When jq Is Missing | SS-07 | | | | | ✓ | |
| VP-049 | Generated hooks-registry.toml Round-Trips Through Registry::load | SS-07, SS-09 | | | | ✓ | | |
| VP-078 | CI Lint Invariant — on_error=block implies async=false | SS-07, SS-01 | | | | ✓ | | |
| VP-080 | block-ai-attribution PostToolUse arm: detect_attribution | SS-07 | | ✓ | | | | |
| VP-082 | PreCompact Flush Commits to factory-artifacts | SS-07, SS-04 | | | | ✓ | | |
| VP-085 | PreCompact Flush Hook Is Hermetic | SS-07 | | | ✓ | | | |
| VP-089 | postcompact-reanchor.sh Emits Re-Anchor Block | SS-07 | | | ✓ | | | |
| VP-090 | precompact-flush-log Pruning | SS-07 | | | ✓ | | | |
| VP-099 | hooks-registry Tool-Filter Anchoring Invariant — Every tool= Value Is Fully Anchored (^...$) or Carries # intent: Comment | SS-07 | | | | ✓ | | |
| **SS-07 subtotal** | | | **0** | **1** | **3** | **5** | **3** | **0** |

---

### Module: plugins/vsdd-factory/.claude-plugin, hooks.json (SS-09 — Configuration and Activation)

| VP ID | Title (abbreviated) | Subsystems | K | P | U | I | M | S |
|-------|---------------------|-----------|---|---|---|---|---|---|
| VP-015 | Per-Project Activation Required Before Dispatcher Can Run | SS-09 | | | | | ✓ | |
| **SS-09 subtotal** | | | **0** | **0** | **0** | **0** | **1** | **0** |

---

## §2 Module Assignment Judgment Calls and Coverage Gaps

The following VP assignments required explicit judgment due to multi-subsystem scope.
All decisions defer to VP-INDEX.md §Full Index Scope column as authoritative; the
rationale below documents the reasoning applied when scope order determines primary.

| VP | Scope Column | Primary Assignment | Rationale |
|----|-------------|-------------------|-----------|
| VP-007 | SS-01, SS-03 | SS-01 | Dispatcher self-telemetry invariant; unit-test exercises dispatcher emit path |
| VP-008 | SS-01, SS-03 | SS-01 | Internal log filename logic lives in dispatcher crate |
| VP-009 | SS-01, SS-03 | SS-01 | prune_old is a dispatcher-crate function |
| VP-023 | SS-01, SS-02 | SS-01 | Wire format decoder is tested at dispatcher boundary |
| VP-025 | SS-01, SS-02 | SS-01 | Host ABI completeness is a dispatcher guarantee |
| VP-026 | SS-01, SS-03 | SS-01 | InternalEvent is a dispatcher-core struct |
| VP-043 | SS-07, SS-01 | SS-07 | Property tests hooks-registry.toml (SS-07 file), not dispatcher routing engine |
| VP-044 | SS-04, SS-07 | SS-04 | legacy-bash-adapter WASM lives in crates/hook-plugins/ (SS-04 territory) |
| VP-049 | SS-07, SS-09 | SS-07 | Property tests the generated TOML file structure (SS-07 owns the file) |
| VP-051 | SS-01, SS-03 | SS-01 | Dispatcher startup sequence is the module under test |
| VP-062 | SS-05, SS-07, SS-08 | SS-05 | Process-codification artifact is an orchestration pipeline invariant |
| VP-064 | SS-05, SS-06 | SS-05 | facade-mode mutation gate enforcement is a pipeline orchestration step |
| VP-073 | SS-01, SS-04 | SS-01 | Resolver-load purity: the dispatcher loads resolver modules; dispatcher is subject |
| VP-074 | SS-01, SS-04 | SS-01 | Kani proof targets dispatcher process boundary (error isolation) |
| VP-075 | SS-01, SS-04 | SS-01 | Context-injection determinism at dispatcher boundary |
| VP-078 | SS-07, SS-01 | SS-07 | CI lint invariant checks hooks-registry.toml; SS-07 owns the file |
| VP-080 | SS-07 | SS-07 | VP-INDEX §Full Index lists SS-07; behavioral arm VP follows hook-layer convention |
| VP-081 | SS-04, SS-05, SS-07 | SS-05 | Primary contract is BC-5.41.001 wave-gate orchestration step (SS-05) |
| VP-082 | SS-07, SS-04 | SS-07 | precompact-flush.sh is a SS-07 shell script |
| VP-084 | SS-05, SS-04 | SS-05 | Lifecycle-distinctness invariant (BC-5.41.003) is an orchestration policy |
| VP-086 | SS-01, SS-04 | SS-01 | Dispatcher exit-2 propagation: dispatcher binary is the module under test |
| VP-100 | SS-01, SS-03 | SS-01 | Drain-timer expiry is a dispatcher-core async-drain behavior (SS-01 primary); async-semantics event schema (SS-03) is secondary — the plugin.abandoned event must conform to BC-3.08.001 schema, but the causal mechanism is dispatcher drain-timer |

---

## §3 Grand Totals — Summary by Method

Each VP counted exactly once in the row for its primary subsystem. The grand-total
per-tool column sums equal 101 (total_vps POST-INTEGRATION).

| Subsystem | K | P | U | I | M | S | Row Total |
|-----------|---|---|---|---|---|---|-----------|
| SS-01 | 3 | 1 | 22 | 10 | 0 | 0 | 36 |
| SS-02 | 0 | 0 | 5 | 0 | 0 | 0 | 5 |
| SS-03 | 0 | 0 | 11 | 3 | 0 | 0 | 14 |
| SS-04 | 2 | 2 | 4 | 7 | 0 | 0 | 15 |
| SS-05 | 0 | 0 | 0 | 6 | 6 | 1 | 13 |
| SS-06 | 0 | 1 | 1 | 3 | 0 | 0 | 5 |
| SS-07 | 0 | 1 | 3 | 5 | 3 | 0 | 12 |
| SS-09 | 0 | 0 | 0 | 0 | 1 | 0 | 1 |
| **Grand Total** | **5** | **5** | **47** | **33** | **10** | **1** | **101** |

**Per-tool arithmetic check:** 5 + 5 + 47 + 33 + 10 + 1 = **101** ✓

**Per-subsystem row-sum check:** 36 + 5 + 14 + 15 + 13 + 5 + 12 + 1 = **101** ✓

**Per-tool column matches VP-INDEX.md POST-INTEGRATION targets:**
- kani-proof: **5** ✓ (VP-070, VP-071, VP-074, VP-077, VP-097)
- proptest: **5** ✓ (VP-059, VP-069, VP-075, VP-080, VP-096)
- unit-test: **47** ✓ (42 from VP-INDEX v2.29 + VP-089 + VP-090 + VP-091 + VP-092 + VP-095)
- integration: **33** ✓ (25 from VP-INDEX v2.29 + VP-087 + VP-088 + VP-093 + VP-094 + VP-098 + VP-099 + VP-100 + VP-101)
- manual: **10** ✓ (unchanged from VP-INDEX v2.29)
- static-check: **1** ✓ (unchanged from VP-INDEX v2.29)

**SS-01 row detail:** K=3 (VP-074, VP-077, VP-097), P=1 (VP-075), U=22 (VP-003..010,
VP-014, VP-016..024, VP-026..027, VP-050, VP-052), I=10 (VP-001, VP-002, VP-025,
VP-051, VP-073, VP-086, VP-093, VP-098, VP-100, VP-101), M=0, S=0. Row sum = 3+1+22+10+0+0 = **36** ✓

All 101 VPs are accounted for with no omissions and no double-counts.

---

## §4 Domain Invariant Verification Map

Maps each active domain invariant to the VPs that directly verify it. VP assignment is
authoritative in VP-INDEX.md §Full Index (BC/Invariant Anchor column). DI descriptions
derive from `.factory/specs/domain-spec/invariants.md`.

| DI | Description (brief) | Verified By (VP IDs) | Priority |
|----|---------------------|----------------------|----------|
| DI-001 | Tiers execute sequentially; plugins within a tier execute in parallel | VP-001, VP-019, VP-020, VP-052 | P0 |
| DI-002 | A plugin crash or timeout does not block sibling plugins | VP-002, VP-020, VP-050, VP-074 | P0 |
| DI-003 | block_intent is aggregate; tier runs to completion regardless | VP-003, VP-044, VP-047 | P0 |
| DI-004 | Capability denial always produces a return code AND audit event | VP-004, VP-021, VP-023, VP-025, VP-076 | P0 |
| DI-005 | Shell interpreters require explicit shell_bypass_acknowledged | VP-005, VP-021 | P0 |
| DI-006 | Setuid/setgid binaries refused unconditionally on Unix | VP-006 | P0 |
| DI-007 | Dispatcher self-telemetry is always-on | VP-007, VP-035, VP-051 | P1 |
| DI-008 | Internal log filenames derived from event timestamps, not wall clock | VP-008, VP-029 | P1 |
| DI-009 | Internal logs pruned to 30 days at dispatcher start | VP-009 | P1 |
| DI-010 | Plugin stderr capped at 4 KiB with truncation marker | VP-010 | P1 |
| DI-011 | Sink submit must not block the dispatcher | VP-011, VP-028, VP-030, VP-032, VP-034 | P1 |
| DI-012 | A sink failure affects only that sink | VP-012, VP-028, VP-031, VP-037 | P1 |
| DI-013 | Unknown sink driver types are non-fatal | VP-013, VP-036 | P1 |
| DI-014 | Schema version mismatch is a hard load error | VP-014, VP-018, VP-022, VP-046, VP-049 | P0 |
| DI-015 | Per-project activation required before dispatcher can run | VP-015 | P1 |
| DI-016 | Each registry entry sees only its own plugin_config | VP-016, VP-043, VP-045 | P0 |
| DI-017 | trace_id present on every emitted event; wire-format exclusivity | VP-017, VP-026, VP-027, VP-033, VP-051, VP-079 | P1 |
| DI-018 | (not active — captured as KL-005) | — | — |
| DI-019 | ASYNC_DRAIN_WINDOW_MS = 100 ms (runtime constant) | VP-079, VP-100 | P1 |
| DI-020 | Wave/phase boundary transitions must not lose load-bearing pipeline state | VP-081, VP-082, VP-083, VP-084, VP-092, VP-093 | P0 |
| DI-021 | Handoff claims cross-checked against verifiable external ground truth | VP-081, VP-082, VP-085 | P0 |
| DI-022 | PreCompact flush derives state exclusively from durable persisted sources | VP-082, VP-085 | P0 |
| DI-023 | Wave/phase identity derives from real persisted substrate; no phantom fields | VP-081, VP-087, VP-088 | P0 |
| DI-024 | PostCompact re-anchor is best-effort; not in CAP-032 continuity-guarantee chain | VP-089 | P1 |
| DI-025 | PreCompact flush commits lifecycle-orthogonal to state-manager burst commits | VP-082, VP-084, VP-085, VP-090, VP-093 | P0 |
