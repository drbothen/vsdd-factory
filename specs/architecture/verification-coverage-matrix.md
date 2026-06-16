---
document_type: architecture-section
level: L4
section: verification-coverage-matrix
version: "1.1"
status: draft
producer: architect
timestamp: 2026-06-16T00:00:00Z
last_amended: "2026-06-16 (v1.1) — D-612 INTEGRATION BURST (state-manager POLICY 9 propagation): VP-091 added to SS-04 module table (unit-test; SS-04; DI-020; BC-4.15.001 — validate-heavy-op-delegation always-Continue advisory gate). SS-04 subtotal U 3→4, row total 12→13. Grand Total U 44→45, row total 90→91. Per-tool arithmetic 4+4+45+27+10+1=91 VERIFIED. Per-subsystem row-sum 31+5+14+13+12+4+11+1=91 VERIFIED. [Prior: 2026-06-16 (v1.0) — F2 gate decision: initial creation as a full production-grade architecture deliverable. Sources: VP-INDEX.md v2.29 (86 VPs) + VP-087..VP-090 (4 new E-18 VPs, unstaged). POST-INTEGRATION totals: total_vps=90, unit-test=44, integration=27, manual=10, static-check=1, kani-proof=4, proptest=4. Every VP assigned to its authoritative module per VP-INDEX.md scope column and VP file frontmatter. Authored per F2 gate human directive that deferred architecture derived-views be materialized now.]"
modified:
  - "2026-06-16 (v1.1) — D-612 VP-091 added to SS-04 module; SS-04 U 3→4, total 12→13; grand total 90→91"
  - "2026-06-16 (v1.0 initial creation)"
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
> total row. The per-tool column sums (K+P+U+I+M+S) must equal 91. This invariant
> must be verified on every update to this document.

## §Changelog

| Version | Date | Author | Change |
|---------|------|--------|--------|
| v1.1 | 2026-06-16 | state-manager | D-612 POLICY 9 propagation: VP-091 added to SS-04 module table (unit-test; SS-04; DI-020; BC-4.15.001 — validate-heavy-op-delegation always-Continue advisory gate). SS-04 subtotal U 3→4, row total 12→13. Grand Total U 44→45, row total 90→91. Per-tool arithmetic 4+4+45+27+10+1=91 VERIFIED. Per-subsystem row-sum 31+5+14+13+12+4+11+1=91 VERIFIED. |
| v1.0 | 2026-06-16 | architect | Initial creation — F2 gate decision. Sources: VP-INDEX.md v2.29 (86 VPs) + VP-087..VP-090 (4 new E-18 VPs). POST-INTEGRATION totals: total_vps=90, unit-test=44, integration=27, manual=10, static-check=1, kani-proof=4, proptest=4. |

---

## §1 VP-to-Module Coverage Table

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
VP-086 per assignment notes in §2 below).

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
| **SS-01 subtotal** | | | **2** | **1** | **22** | **6** | **0** | **0** |

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
| **SS-04 subtotal** | | | **2** | **1** | **4** | **6** | **0** | **0** |

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
| **SS-05 subtotal** | | | **0** | **0** | **0** | **5** | **6** | **1** |

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
| **SS-06 subtotal** | | | **0** | **1** | **0** | **3** | **0** | **0** |

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
| **SS-07 subtotal** | | | **0** | **1** | **3** | **4** | **3** | **0** |

---

### Module: plugins/vsdd-factory/.claude-plugin, hooks.json (SS-09 — Configuration and Activation)

| VP ID | Title (abbreviated) | Subsystems | K | P | U | I | M | S |
|-------|---------------------|-----------|---|---|---|---|---|---|
| VP-015 | Per-Project Activation Required Before Dispatcher Can Run | SS-09 | | | | | ✓ | |
| **SS-09 subtotal** | | | **0** | **0** | **0** | **0** | **1** | **0** |

---

## §2 Module Assignment Judgment Calls

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

---

## §3 Grand Totals

Each VP counted exactly once in the row for its primary subsystem. The grand-total
per-tool column sums equal 91 (total_vps POST-INTEGRATION).

| Subsystem | K | P | U | I | M | S | Row Total |
|-----------|---|---|---|---|---|---|-----------|
| SS-01 | 2 | 1 | 22 | 6 | 0 | 0 | 31 |
| SS-02 | 0 | 0 | 5 | 0 | 0 | 0 | 5 |
| SS-03 | 0 | 0 | 11 | 3 | 0 | 0 | 14 |
| SS-04 | 2 | 1 | 4 | 6 | 0 | 0 | 13 |
| SS-05 | 0 | 0 | 0 | 5 | 6 | 1 | 12 |
| SS-06 | 0 | 1 | 0 | 3 | 0 | 0 | 4 |
| SS-07 | 0 | 1 | 3 | 4 | 3 | 0 | 11 |
| SS-09 | 0 | 0 | 0 | 0 | 1 | 0 | 1 |
| **Grand Total** | **4** | **4** | **45** | **27** | **10** | **1** | **91** |

**Per-tool arithmetic check:** 4 + 4 + 45 + 27 + 10 + 1 = **91** ✓

**Per-subsystem row-sum check:** 31 + 5 + 14 + 13 + 12 + 4 + 11 + 1 = **91** ✓

**Per-tool column matches VP-INDEX.md POST-INTEGRATION targets:**
- kani-proof: **4** ✓ (VP-070, VP-071, VP-074, VP-077)
- proptest: **4** ✓ (VP-059, VP-069, VP-075, VP-080)
- unit-test: **45** ✓ (42 from VP-INDEX v2.29 + VP-089 + VP-090 + VP-091)
- integration: **27** ✓ (25 from VP-INDEX v2.29 + VP-087 + VP-088)
- manual: **10** ✓ (unchanged from VP-INDEX v2.29)
- static-check: **1** ✓ (unchanged from VP-INDEX v2.29)

**SS-01 row detail:** K=2 (VP-074, VP-077), P=1 (VP-075), U=22 (VP-003..010,
VP-014, VP-016..024, VP-026..027, VP-050, VP-052), I=6 (VP-001, VP-002, VP-025,
VP-051, VP-073, VP-086), M=0, S=0. Row sum = 2+1+22+6+0+0 = **31** ✓

All 91 VPs are accounted for with no omissions and no double-counts.
