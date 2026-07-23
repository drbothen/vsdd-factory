---
document_type: verification-architecture
level: L4
section: verification-architecture
version: "1.11"
status: draft
producer: architect
timestamp: 2026-06-24T00:00:00Z
last_amended: "2026-07-16 (v1.11) — F-P2-001 POLICY 9 re-propagation (architect): VP-095 SS-04 catalog row title corrected to v1.3 H1 verbatim form (host::read_prefix(262144)); v1.10 propagated to VP-095 v1.2 Phase-B title (8192) which v1.3 superseded same-burst 2026-07-16; input-hash f0fab9c→5279415. [Prior: 2026-07-16 (v1.10) — S-19.07 Phase-B POLICY 9 propagation (architect): VP-095 SS-04 catalog row title updated to Phase-B form; proof method integration→unit+static; BC anchor updated to BC-4.13.001 Phase-B + BC-1.17.001 PC-3; §3 integration 34→33, unit-test 46→47; input-hash c9ec678→f0fab9c. [Prior: 2026-07-13 (v1.9) — pass-11 F-P11-001 POLICY 9 propagation (architect): VP-099 SS-07 catalog row title updated to both-ends form ('Starts With ^' → 'Is Fully Anchored (^...$)'); input-hash 893a501→c9ec678. [Prior: 2026-07-08 (v1.8) — E-19 pass-28 VP-096 boundary-wording sync (architect): VP-096 catalog row title updated to exclusive form per BC-4.13.001 §Invariant 9 adjudication (D-781): 'Through Second --- Delimiter' → 'Up To (Excluding) the Second --- Delimiter Line (bytes 0..delimiter_start_offset; opening ---\\n included)'. input-hash 7a7ac8c→893a501. [Prior: 2026-07-06 (v1.7) — F-P3-003 fix (architect): VP-094..VP-101 catalog row titles rewritten verbatim from VP file H1s; all 7 placeholder titles replaced (VP-101 was correct); BC-trace columns corrected per VP source_bc fields. Retroactive POLICY 14 body Changelog row added for v1.6. [Prior: 2026-07-06 (v1.6) — E-19 VP package POLICY 9 propagation (architect): VP-094 added to §SS-05 Provable Properties Catalog (integration; BC-5.42.001; S-19.01); VP-095/096 added to §SS-04 catalog (integration + proptest; BC-4.13.001; S-19.02); VP-097/098/100/101 added to §SS-01 catalog (kani-proof + integration; BC-2.07.001+BC-2.02.011 + BC-2.07.001 + BC-3.08.001+DI-019 + BC-1.17.001; S-19.03/S-19.03/S-19.05/S-19.06); VP-099 added to §SS-07 catalog (integration; no BC; S-19.04). §1 intro: All 93→101 verification properties. §1 VP count invariant: 93 VPs→101 VPs (VP-001..VP-101). §3 Proof Method Coverage Totals: integration 28→34 (+6); kani-proof 4→5 (+1); proptest 4→5 (+1); Total 93→101; arithmetic invariant note updated; VP ID lists updated; Total row range VP-001..VP-093→VP-001..VP-101. §4 integration reference updated 28→34. [Prior: 2026-06-24 (v1.5) — S-18.04b-prereq BC authoring burst (architect): VP-093 added to §SS-01 Provable Properties Catalog (integration; DI-020, DI-025; BC-1.16.001 PC1/PC2/PC3/PC4/PC5/INV1/INV2/INV3/INV5; anchor S-18.04b-prereq — dispatcher git_context injection on PostToolUse Bash git-commit events; exec-free WASM boundary; fail-open on git error). §3 Proof Method Coverage Totals: integration 27→28; Total 92→93. §1 VP count invariant updated 92→93. [Prior: 2026-06-17 (v1.4) — C-P7-001 fix burst (architect): VP-086 row added to §SS-01 Provable Properties Catalog — was present in §3 integration list and §Risk Mitigations but absent from §1 catalog body (91 unique VP IDs vs 92 total). VP-086: Dispatcher Exit-2 Propagation for PreCompact Block-Intent; integration; BC-1.15.001 PC4; SS-01/SS-04; anchor S-18.00. §1 VP count invariant unchanged at 92 (catalog now reflects actual count). [Prior: 2026-06-16 (v1.3) — D-615 E-18 STORY PASS-1 FIX WAVE INTEGRATION BURST (state-manager POLICY 9 propagation): VP-092 added to §SS-06 Provable Properties Catalog (unit-test; DI-020; check-state-health CLAUDE_AUTOCOMPACT_PCT_OVERRIDE advisory check; BC-6.25.001; S-18.10). §3 Proof Method Coverage Totals: unit-test 45→46; Total 91→92. §1 VP count invariant note updated 91→92. [Prior: 2026-06-16 (v1.2) — D-612 INTEGRATION BURST (state-manager POLICY 9 propagation): VP-091 added to §SS-04 Provable Properties Catalog (unit-test; DI-020; validate-heavy-op-delegation always-Continue advisory gate; BC-4.15.001; S-18.06). §3 Proof Method Coverage Totals: unit-test 44→45; Total 90→91. §1 VP count invariant note updated 90→91. [Prior: 2026-06-16 (v1.1) — fix burst (architect): FINDING-1 (MINOR) + O-D607-003 — removed SS-08 from subsystems_affected frontmatter; SS-08 has zero VPs in this document's body (consistent with sibling verification-coverage-matrix.md which correctly omits SS-08). Frontmatter now matches body. [Prior: 2026-06-16 (v1.0) — F2 gate decision: initial creation as a full production-grade architecture deliverable. Sources: VP-INDEX.md v2.29 (86 VPs) + VP-087..VP-090 (4 new E-18 VPs, unstaged). POST-INTEGRATION totals: total_vps=90, unit-test=44, integration=27, manual=10, static-check=1, kani-proof=4, proptest=4. Authored per F2 gate human directive that deferred architecture derived-views be materialized now.]]"
modified:
  - "2026-07-16 (v1.11) — F-P2-001 POLICY 9 re-propagation: VP-095 SS-04 row title corrected to v1.3 form (262144); input-hash f0fab9c→5279415"
  - "2026-07-16 (v1.10) — S-19.07 Phase-B POLICY 9 propagation: VP-095 SS-04 row title→Phase-B form; proof method integration→unit+static; BC anchor updated to BC-4.13.001 Phase-B + BC-1.17.001 PC-3; §3 integration 34→33, unit-test 46→47; input-hash c9ec678→f0fab9c"
  - "2026-07-13 (v1.9) — pass-11 F-P11-001 POLICY 9 propagation: VP-099 SS-07 row title updated to both-ends form; input-hash 893a501→c9ec678"
  - "2026-07-08 (v1.8) — E-19 pass-28 VP-096 boundary-wording sync: VP-096 catalog row title updated to exclusive form per BC-4.13.001 §Invariant 9 (D-781); input-hash 7a7ac8c→893a501"
  - "2026-07-06 (v1.7) — F-P3-003 fix: VP-094..VP-101 catalog row titles rewritten verbatim from VP H1s; BC-trace columns corrected; retroactive v1.6 body Changelog row added (POLICY 14)"
  - "2026-07-06 (v1.6) — E-19 VP package POLICY 9 propagation: VP-094 (SS-05), VP-095/096 (SS-04), VP-097/098/100/101 (SS-01), VP-099 (SS-07) added; §1 count 93→101; §3 integration 28→34, kani-proof 4→5, proptest 4→5, Total 93→101; input-hash 61531bf→7a7ac8c"
  - "2026-06-24 (v1.5) — VP-093 added to SS-01 catalog; integration 27→28; total 92→93"
  - "2026-06-17 (v1.4) — C-P7-001 VP-086 row added to SS-01 Provable Properties Catalog (missing from §1 body; present in §3 and §Risk Mitigations); catalog count now 92 unique VP IDs"
  - "2026-06-16 (v1.3) — D-615 VP-092 added to SS-06 catalog; unit-test 45→46; total 91→92"
  - "2026-06-16 (v1.2) — D-612 VP-091 added to SS-04 catalog; unit-test 44→45; total 90→91"
  - "2026-06-16 (v1.1) — removed SS-08 from subsystems_affected (zero VPs in body; aligns with verification-coverage-matrix.md)"
  - "2026-06-16 (v1.0 initial creation)"
phase: 1b
inputs: [verification-properties/VP-INDEX.md]
input-hash: "015260d"
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

# Verification Architecture

> **Source-of-truth relationship:** VP-INDEX.md is the authoritative VP catalog.
> This document derives from VP-INDEX.md and must be kept in sync via same-burst
> propagation (POLICY 9 / VP-INDEX Propagation Obligation). Any addition, retirement,
> module reassignment, tool change, or phase reassignment in VP-INDEX MUST propagate
> to this document in the same burst.

## §Changelog

| Version | Date | Author | Change |
|---------|------|--------|--------|
| v1.11 | 2026-07-16 | architect | F-P2-001 POLICY 9 re-propagation: VP-095 SS-04 catalog row title corrected to v1.3 H1 verbatim form ('verify-factory-lock Never Receives output_too_large for Any STATE.md Size — Structural Guarantee via host::read_prefix(262144) (BC-1.17.001 PC-3); Large-STATE.md Frontmatter Correctly Parsed from 262144-Byte Prefix'); v1.10 propagated to VP-095 v1.2 Phase-B title (8192) which VP-095 v1.3 superseded same-burst 2026-07-16. input-hash f0fab9c→5279415. |
| v1.10 | 2026-07-16 | architect | S-19.07 Phase-B POLICY 9 propagation: VP-095 SS-04 catalog row title updated to Phase-B form ('verify-factory-lock Never Receives output_too_large for Any STATE.md Size — Structural Guarantee via host::read_prefix(8192) (BC-1.17.001 PC-3); Large-STATE.md Frontmatter Correctly Parsed from 8192-Byte Prefix'); proof method integration→unit+static; BC anchor BC-4.13.001 PC3→BC-4.13.001 Phase-B + BC-1.17.001 PC-3. §3 integration 34→33 (VP-095 removed); unit-test 46→47 (VP-095 added). input-hash c9ec678→f0fab9c. |
| v1.9 | 2026-07-13 | architect | pass-11 F-P11-001 POLICY 9 propagation: VP-099 SS-07 catalog row title updated to both-ends form ('Starts With ^' → 'Is Fully Anchored (^...$)'). input-hash 893a501→c9ec678. |
| v1.8 | 2026-07-08 | architect | E-19 pass-28 VP-096 boundary-wording sync: VP-096 catalog row title updated to exclusive form per BC-4.13.001 §Invariant 9 adjudication (D-781) — 'Through Second --- Delimiter' → 'Up To (Excluding) the Second --- Delimiter Line (bytes 0..delimiter_start_offset; opening ---\n included)'. input-hash 7a7ac8c→893a501. |
| v1.7 | 2026-07-06 | architect | F-P3-003 fix: VP-094..VP-101 catalog row titles rewritten verbatim from VP file H1s (all 7 were placeholder titles unrelated to actual VPs; VP-101 was already correct). BC-trace columns corrected per VP source_bc fields: VP-094→BC-5.42.001; VP-095→BC-4.13.001 PC3; VP-096→BC-4.13.001 INV9; VP-097→BC-2.07.001, BC-2.02.011 EC-001; VP-098→BC-2.07.001; VP-099→—; VP-100→BC-3.08.001 INV6; VP-101→BC-1.17.001. Retroactive POLICY 14 body Changelog row added for v1.6. |
| v1.6 | 2026-07-06 | architect | E-19 VP package POLICY 9 propagation: VP-094..VP-101 rows added to §SS-05/SS-04/SS-01/SS-07 Provable Properties Catalogs (8 VPs; integration 28→34; kani-proof 4→5; proptest 4→5; Total 93→101). Note: row titles contained placeholder values at authorship — corrected at v1.7. |
| v1.5 | 2026-06-24 | architect | S-18.04b-prereq POLICY 9 propagation: VP-093 added to §SS-01 Provable Properties Catalog (integration; DI-020, DI-025; BC-1.16.001 PC1/PC2/PC3/PC4/PC5/INV1/INV2/INV3/INV5; S-18.04b-prereq — dispatcher git_context injection on PostToolUse Bash git-commit events; four-field completeness; fail-open on git error; no injection on non-qualifying events; exec-free WASM boundary; HOST_ABI_VERSION unchanged). §3 integration 27→28; Total 92→93. §1 VP count invariant updated 92→93. |
| v1.4 | 2026-06-17 | architect | C-P7-001 fix: VP-086 row added to SS-01 Provable Properties Catalog — was present in §3 integration list and §Risk Mitigations but absent from §1 catalog body (91 unique VP IDs vs declared total 92). VP-086: Dispatcher Exit-2 Propagation for PreCompact Block-Intent; integration; BC-1.15.001 PC4 (exit-2 block-intent propagated to harness); SS-01/SS-04; anchor S-18.00. Catalog now enumerates all 92 VPs. §3 totals and §1 count invariant (92) unchanged — §3 integration list already included VP-086. |
| v1.3 | 2026-06-16 | state-manager | D-615 POLICY 9 propagation: VP-092 added to SS-06 Provable Properties Catalog (unit-test; DI-020; BC-6.25.001; S-18.10 — check-state-health CLAUDE_AUTOCOMPACT_PCT_OVERRIDE advisory check; never blocks; PC1 absent→ADVISORY; PC2 >80→ADVISORY; PC3 <=80→PASS). §3 unit-test 45→46; Total 91→92. §1 VP count invariant updated 91→92. |
| v1.2 | 2026-06-16 | state-manager | D-612 POLICY 9 propagation: VP-091 added to SS-04 Provable Properties Catalog (unit-test; DI-020; BC-4.15.001; S-18.06 — validate-heavy-op-delegation always-Continue advisory gate). §3 unit-test 44→45; Total 90→91. §1 VP count invariant updated 90→91. |
| v1.1 | 2026-06-16 | architect | FINDING-1 (MINOR) + O-D607-003 — removed SS-08 from `subsystems_affected` frontmatter. SS-08 has zero VPs in this document's §1 body; sibling verification-coverage-matrix.md correctly omits SS-08. Frontmatter now matches body content. |
| v1.0 | 2026-06-16 | architect | Initial creation — F2 gate decision. Sources: VP-INDEX.md v2.29 (86 VPs) + VP-087..VP-090 (4 new E-18 VPs). POST-INTEGRATION totals: total_vps=90, unit-test=44, integration=27, manual=10, static-check=1, kani-proof=4, proptest=4. |

---

## §1 Provable Properties Catalog

All 101 verification properties, organized by subsystem. Each VP entry states: title,
proof method, BC postcondition/invariant anchor, and current status.

> **VP count invariant:** This catalog lists exactly 101 VPs (VP-001..VP-101) across
> all subsystems. The per-method totals in §3 must sum to 101.

---

### SS-01: Hook Dispatcher Core

| VP ID | Title | Proof Method | BC/Invariant Anchor | Status |
|-------|-------|-------------|---------------------|--------|
| VP-001 | Tier Execution Is Sequential; Intra-Tier Is Parallel | integration | DI-001 | draft |
| VP-002 | Plugin Crash or Timeout Does Not Block Sibling Plugins | integration | DI-002 | draft |
| VP-003 | block_intent Is Aggregate; Tier Runs to Completion | unit-test | DI-003 | draft |
| VP-004 | Capability Denial Produces Return Code AND Audit Event | unit-test | DI-004 | draft |
| VP-005 | Shell Interpreters Require Explicit shell_bypass_acknowledged | unit-test | DI-005 | draft |
| VP-006 | Setuid/Setgid Binaries Refused Unconditionally | unit-test | DI-006 | draft |
| VP-007 | Dispatcher Self-Telemetry Is Always-On and Never Panics | unit-test | DI-007 | draft |
| VP-008 | Internal Log Filename Derived from Event Timestamp, Not Wall Clock | unit-test | DI-008 | draft |
| VP-009 | prune_old Removes Only Dispatcher-Internal Files Older Than Threshold | unit-test | DI-009 | draft |
| VP-010 | Plugin Stderr Capped at 4 KiB with Truncation Marker | unit-test | DI-010 | draft |
| VP-014 | Schema Version Mismatch Is a Hard Load Error | unit-test | DI-014 | draft |
| VP-016 | Each Registry Entry Sees Only Its Own plugin_config | unit-test | DI-016 | draft |
| VP-017 | dispatcher_trace_id Present on Every Emitted Event | unit-test | DI-017 | draft |
| VP-018 | Registry Rejects Malformed Configurations at Load Time | unit-test | DI-014 | draft |
| VP-019 | Routing Is Deterministic — Same Input Yields Same Plugin Selection | unit-test | DI-001 | draft |
| VP-020 | Epoch Timeout Rounds Up and Terminates Infinite Loops | unit-test | DI-001, DI-002 | draft |
| VP-021 | Capability Deny-by-Default — Each Capability Requires Explicit Allow | unit-test | DI-004, DI-005 | draft |
| VP-022 | Dispatcher Exit Code Semantics — 0 for Non-Block, 2 for Block | unit-test | DI-014 | draft |
| VP-023 | Wire Format Decoders Reject Truncated Input Without Panic | unit-test | DI-004 | draft |
| VP-024 | Plugin Cache Is Keyed by Path and Invalidated by mtime | unit-test | — | draft |
| VP-025 | Host Function ABI Surface Is Complete and Stable | integration | DI-004 | draft |
| VP-026 | InternalEvent Serializes Flat with No Null Optional Fields | unit-test | DI-017 | draft |
| VP-027 | HookPayload Parsing Is Robust for All Claude Code Envelope Types | unit-test | DI-017 | draft |
| VP-050 | exec_subprocess Timeout Is Enforced — Hung Commands Are Killed | unit-test | DI-002 | draft |
| VP-051 | Dispatcher Startup Flow Writes Parseable JSONL with Correct Envelopes | integration | DI-007, DI-017 | draft |
| VP-052 | Epoch Ticker Shuts Down Cooperatively and Idempotently | unit-test | DI-001 | draft |
| VP-073 | Resolver-Load Purity — resolver WASM module loading is pure | integration | — | draft |
| VP-074 | Resolver-Error Isolation — resolver crash, trap, or timeout must not propagate to dispatcher | kani-proof | DI-002 | draft |
| VP-075 | Context-Injection Determinism — same resolver input always produces same output | proptest | — | draft |
| VP-077 | Dispatcher Partition Correctness (6 properties) | kani-proof | — | draft |
| VP-086 | Dispatcher Exit-2 Propagation for PreCompact Block-Intent | integration | BC-1.15.001 PC4 | draft |
| VP-093 | Dispatcher Injects git_context Into payload.extra on PostToolUse Bash git-commit Events; Fail-Open on Git Error; No Injection on Non-Qualifying Events | integration | BC-1.16.001 PC1/PC2/PC3/PC4/PC5/INV1/INV2/INV3/INV5 | draft |
| VP-097 | path_util::resolve_path_for_allowlist Traversal Defense — .. Sequences Cannot Resolve Outside Allowlist Prefixes | kani-proof | BC-2.07.001, BC-2.02.011 EC-001 | draft |
| VP-098 | Allowlisted-but-Absent File Returns internal.file_not_found Event and NOT_FOUND (-5); Zero CAPABILITY_DENIED False-Positives | integration | BC-2.07.001 | draft |
| VP-100 | Drain-Timer Expiry Emits Exactly One plugin.abandoned Per In-Flight (plugin_name, entry_index); No plugin.completed Follows for Same Trace | integration | BC-3.08.001 INV6 | draft |
| VP-101 | host::read_prefix Returns Byte-Exact Prefix of len <= max_bytes; Never OUTPUT_TOO_LARGE; Absent File Returns NOT_FOUND (-5) | integration | BC-1.17.001 | draft |

---

### SS-02: Hook SDK and Plugin ABI

| VP ID | Title | Proof Method | BC/Invariant Anchor | Status |
|-------|-------|-------------|---------------------|--------|
| VP-038 | SDK HookResult Exit Codes Are Stable — Continue=0, Error=1, Block=2 | unit-test | DI-004 | draft |
| VP-039 | SDK Wire Format Encoding Is Symmetric with Dispatcher Decoding | unit-test | DI-004 | draft |
| VP-040 | SDK HookPayload Round-Trips via Serde and Carries plugin_config | unit-test | DI-016 | draft |
| VP-041 | SDK Panic Handler Extracts Message for All Payload Types | unit-test | DI-002 | draft |
| VP-042 | SDK HostError Code Mapping Is Stable | unit-test | DI-004 | draft |

---

### SS-03: Event Emission (OTel-Aligned)

| VP ID | Title | Proof Method | BC/Invariant Anchor | Status |
|-------|-------|-------------|---------------------|--------|
| VP-011 | Sink submit Must Not Block the Dispatcher | unit-test | DI-011 | draft |
| VP-012 | Sink Failure Affects Only That Sink | unit-test | DI-012 | draft |
| VP-013 | Unknown Sink Driver Types Are Non-Fatal | unit-test | DI-013 | draft |
| VP-028 | Sink Fan-Out — Every Event Reaches Every Configured Accepting Sink | integration | DI-011, DI-012 | draft |
| VP-029 | File Sink Path Template Substitutes {date}, {name}, {project} Correctly | unit-test | DI-008 | draft |
| VP-030 | Sink Shutdown Drains Queued Events Before Closing | unit-test | DI-011 | draft |
| VP-031 | Tag Enrichment Does Not Overwrite Producer Fields | unit-test | DI-012 | draft |
| VP-032 | RoutingFilter Default Accepts All Events; Allow-List Is Whitelist; Deny Applied After Allow | unit-test | DI-011 | draft |
| VP-033 | OTLP LogRecord Mapping Is Correct — type to body, ts_epoch to time_unix_nano | integration | DI-017 | draft |
| VP-034 | OTLP Sink Batch Trigger Thresholds Are Independent | unit-test | DI-011 | draft |
| VP-035 | File Sink Auto-Creates Missing Parent Directories | unit-test | DI-007 | draft |
| VP-036 | Disabled Sink Drops Every Event Without Writing | unit-test | DI-013 | draft |
| VP-037 | OTLP Resource Attributes — Operator Overrides Win Over Auto-Detected Defaults | unit-test | DI-012 | draft |
| VP-079 | Async-Semantics Event Types — Payload Schema Conformance | integration | DI-017, DI-019 | draft |

---

### SS-04: Plugin Ecosystem

| VP ID | Title | Proof Method | BC/Invariant Anchor | Status |
|-------|-------|-------------|---------------------|--------|
| VP-044 | Legacy Bash Adapter Exit Code Mapping Is Correct | unit-test | DI-003 | draft |
| VP-045 | Legacy Bash Adapter Strips plugin_config Before Piping to Bash | unit-test | DI-016 | draft |
| VP-065 | Session-Start Plugin Surface Invariant | integration | — | draft |
| VP-066 | Session-End Plugin Surface Invariant | integration | — | draft |
| VP-067 | Worktree Hook Plugin Surface Invariant | integration | — | draft |
| VP-068 | Tool-Failure Hook Plugin Surface Invariant | integration | — | draft |
| VP-069 | validate-artifact-path Registry-Load Purity | proptest | — | draft |
| VP-070 | validate-artifact-path Path-Pattern Matching Is Pure and Deterministic | kani-proof | — | draft |
| VP-071 | validate-per-story-adversary-convergence Block Invariant | kani-proof | — | draft |
| VP-072 | artifact-path-registry.yaml Single Source of Truth | integration | — | draft |
| VP-076 | Resolver-Capability Confinement | integration | DI-004 | draft |
| VP-083 | Completeness Gate Is No-Op on Wave-1 or Non-HANDOFF.md Writes | unit-test | DI-020 | draft |
| VP-091 | validate-heavy-op-delegation Emits DelegationRecommended Advisory on Pattern-Matching Bash Commands and Returns Continue in All Cases (Never Blocks) | unit-test | DI-020 | draft |
| VP-095 | verify-factory-lock Never Receives output_too_large for Any STATE.md Size — Structural Guarantee via host::read_prefix(262144) (BC-1.17.001 PC-3); Large-STATE.md Frontmatter Correctly Parsed from 262144-Byte Prefix | unit+static | BC-4.13.001 Phase-B + BC-1.17.001 PC-3 | draft |
| VP-096 | extract_frontmatter Purity — Output Byte-Equals File Prefix Up To (Excluding) the Second --- Delimiter Line (bytes 0..delimiter_start_offset; opening ---\n included); Deterministic for Any Input | proptest | BC-4.13.001 INV9 | draft |

---

### SS-04 (also anchoring SS-05 or SS-07 via multi-subsystem VPs)

The following VPs anchor to SS-04 as one of their subsystems; they are listed under
their primary subsystem below but are cross-referenced here for completeness:

- VP-084 (primary SS-05, SS-04): PreCompact Flush Commit Is Lifecycle-Distinct From State-Manager Burst Commit
- VP-081 (primary SS-04, SS-05, SS-07): Wave Cannot Close Without Verified Handoff

---

### SS-05: Pipeline Orchestration

| VP ID | Title | Proof Method | BC/Invariant Anchor | Status |
|-------|-------|-------------|---------------------|--------|
| VP-053 | Lobster Workflow DAG Is Acyclic — No Circular Dependencies | manual | — | draft |
| VP-054 | Workflow Loop Blocks Are Bounded — max_iterations and exit_condition Required | manual | — | draft |
| VP-055 | state-manager Runs Last in Every Burst | manual | — | draft |
| VP-056 | on_failure Semantics — retry → escalate → abort Are Correctly Ordered | manual | — | draft |
| VP-057 | Adversarial Review Convergence — Mis-Anchoring Always Blocks, 3-Clean-Pass Minimum | manual | — | draft |
| VP-061 | Agent Prompt Discipline Rules Are Present in All Three Agent Files | static-check | — | draft |
| VP-062 | S-7.02 Process-Codification Surface Invariant | integration | — | draft |
| VP-063 | RED_RATIO computation correctness | integration | — | draft |
| VP-064 | facade-mode mutation gate enforcement | manual | — | draft |
| VP-081 | Wave Cannot Close Without Verified Handoff (wave_id > 1) | integration | DI-020, DI-021, DI-023 | draft |
| VP-084 | PreCompact Flush Commit Is Lifecycle-Distinct From State-Manager Burst Commit | integration | DI-020, DI-025 | draft |
| VP-087 | wave-state.yaml Is Produced Atomically With HANDOFF.md, Stories List Derives From Real Substrate, BrokenSprintState Blocks on Non-Terminal Stories | integration | DI-023 | draft |
| VP-094 | pr-manager READY-Verdict Covered-SHA Pin, Stale-Verdict Halt, and Release-PR Merge-Strategy Enforcement | integration | BC-5.42.001 | draft |

---

### SS-06: Skill Catalog

| VP ID | Title | Proof Method | BC/Invariant Anchor | Status |
|-------|-------|-------------|---------------------|--------|
| VP-058 | create-adr Atomicity — No Partial Repository State After Failure | integration | — | draft |
| VP-059 | ID Monotonicity — Allocated ADR-NNN is Strictly Greater Than All Existing IDs | proptest | — | draft |
| VP-060 | Bidirectional Supersession — supersedes ↔ superseded_by is Symmetric After Skill Completion | integration | — | draft |
| VP-088 | rehydrate-wave Reads wave-state.yaml From Git (Not Working Tree), Injects Exactly Listed Specs, Blocks on Missing Manifest, No RAG Fallback | integration | DI-023 | draft |
| VP-092 | check-state-health Reads CLAUDE_AUTOCOMPACT_PCT_OVERRIDE from Project-Local settings.json (Global Fallback), Emits ADVISORY When Absent or Value > 80, Emits PASS When Value <= 80, Never Blocks, Always Emits Check Row | unit-test | DI-020 | draft |

---

### SS-07: Hook Bash Layer

| VP ID | Title | Proof Method | BC/Invariant Anchor | Status |
|-------|-------|-------------|---------------------|--------|
| VP-043 | Every hooks-registry.toml Entry Routes Through legacy-bash-adapter.wasm | integration | DI-016 | draft |
| VP-046 | All hooks-registry.toml Entries Correspond to Registered Hook Scripts | manual | DI-014 | draft |
| VP-047 | Validator Hooks Exit 0 (Pass) or 2 (Block) — No Other Codes | manual | DI-003 | draft |
| VP-048 | protect-secrets.sh Fails Closed When jq Is Missing | manual | — | draft |
| VP-049 | Generated hooks-registry.toml Round-Trips Through Registry::load | integration | DI-014 | draft |
| VP-080 | block-ai-attribution PostToolUse arm: detect_attribution correctly identifies all TV-001..011 patterns | proptest | — | draft |
| VP-082 | PreCompact Flush Commits to factory-artifacts Before Compaction Proceeds | integration | DI-021, DI-022, DI-025 | draft |
| VP-085 | PreCompact Flush Hook Is Hermetic | unit-test | DI-021, DI-022, DI-025 | draft |
| VP-078 | CI Lint Invariant — on_error = "block" implies async = false in hooks-registry.toml | integration | — | draft |
| VP-089 | postcompact-reanchor.sh Emits Re-Anchor Block From Git-Sourced STATE.md, Appends Log Entry, Makes No factory-artifacts Commits, Exits 0 on All Error Paths | unit-test | DI-024 | draft |
| VP-090 | precompact-flush-log Pruning — prune to Most-Recent-500 Entries When Count Exceeds 1000 | unit-test | DI-025 | draft |
| VP-099 | hooks-registry Tool-Filter Anchoring Invariant — Every tool= Value Is Fully Anchored (^...$) or Carries # intent: Comment | integration | — | draft |

---

### SS-09: Configuration and Activation

| VP ID | Title | Proof Method | BC/Invariant Anchor | Status |
|-------|-------|-------------|---------------------|--------|
| VP-015 | Per-Project Activation Required Before Dispatcher Can Run | manual | DI-015 | draft |

---

## §2 P0 Properties and P1 Properties

### P0 — Kani Upgrade Candidates (Formal Proof Priority)

These VPs are currently exercised by unit-test or integration methods and are candidates
for promotion to `kani-proof`. Upgrading to Kani provides stronger exhaustive guarantees
for the security-critical or arithmetic-critical properties they cover.

| VP | Property | Rationale for Kani Promotion |
|----|----------|------------------------------|
| VP-020 | Epoch timeout rounds up (div_ceil) | Pure integer arithmetic, bounded input; Kani can exhaustively verify the div_ceil rounding invariant across all u64 values |
| VP-023 | Wire format decoders reject truncated buffers | Security boundary, pure function; Kani can prove no path panics on any truncated input |
| VP-042 | HostError code mapping for all negative i32 | ABI contract, exhaustive verification; Kani can cover all negative i32 values at once |

### P1 — Proptest Upgrade Candidates (Property-Test Priority)

These VPs are currently exercised by unit-test or integration methods and are candidates
for promotion to `proptest`. Proptest strategies extend coverage beyond hand-crafted
fixtures to arbitrary generated inputs.

| VP | Property | Proptest Strategy |
|----|----------|-------------------|
| VP-019 | Routing determinism | proptest over arbitrary HookPayload |
| VP-029 | File sink path template substitution | proptest over arbitrary template strings |
| VP-032 | RoutingFilter semantics | proptest over (event_type, allow, deny) triples |
| VP-059 | ADR ID monotonicity | proptest over arbitrary filesystem ID sets (200 trials) — already proptest; listed for completeness |

---

## §3 Proof Method Coverage Totals

> **Arithmetic invariant:** per-method counts must sum to total_vps (101).
> These totals must equal the VP-INDEX.md §Proof Method Breakdown totals.
> Source of truth: VP-INDEX.md. If VP-INDEX and this table diverge, VP-INDEX wins.

| Proof Method | Count | VP IDs |
|-------------|-------|--------|
| unit-test | 47 | VP-003..014, VP-016..024, VP-026..027, VP-029..032, VP-034..042, VP-044..045, VP-050, VP-052, VP-083, VP-085, VP-089, VP-090, VP-091, VP-092, VP-095 |
| integration | 33 | VP-001, VP-002, VP-025, VP-028, VP-033, VP-043, VP-049, VP-051, VP-058, VP-060, VP-062, VP-063, VP-065, VP-066, VP-067, VP-068, VP-072, VP-073, VP-076, VP-078, VP-079, VP-081, VP-082, VP-084, VP-086, VP-087, VP-088, VP-093, VP-094, VP-098, VP-099, VP-100, VP-101 |
| manual | 10 | VP-015, VP-046..048, VP-053..057, VP-064 |
| static-check | 1 | VP-061 |
| kani-proof | 5 | VP-070, VP-071, VP-074, VP-077, VP-097 |
| proptest | 5 | VP-059, VP-069, VP-075, VP-080, VP-096 |
| **Total** | **101** | **VP-001..VP-101** |

---

## §4 Verification Tooling Selection

### Rust Crates (Dispatcher Core, Plugin Ecosystem, SDK)

**Kani model checker** (`cargo kani`) is selected for properties that are:
- Pure functions with bounded inputs (arithmetic, ABI contracts, partition logic).
- Security-critical with exhaustive verification requirements.
- Currently `kani-proof`: VP-070, VP-071, VP-074, VP-077, VP-097.
- Upgrade candidates (P0): VP-020, VP-023, VP-042.

**cargo-fuzz / proptest** are selected for:
- Determinism and template-substitution properties over arbitrary inputs.
- State machine properties with large input spaces.
- Currently `proptest`: VP-059, VP-069, VP-075, VP-080, VP-096.
- Upgrade candidates (P1): VP-019, VP-029, VP-032.

**Rust unit tests** (`cargo test`) are the default for:
- Pure-function postconditions with hand-crafted representative fixtures.
- Sink behavior, wire format, SDK ABI, and capability enforcement.
- Currently: 47 VPs.

**Integration tests** (bats + Rust integration harnesses) are selected for:
- End-to-end dispatcher pipeline properties (tier ordering, fan-out, startup).
- Hook plugin surface contracts that require the full dispatcher binary.
- Wave-boundary contracts that require a live factory-artifacts git fixture.
- Currently: 33 VPs.

**Manual verification** is selected only for:
- Properties whose verification requires human judgment (workflow DAG structure,
  process codification artifact presence).
- Properties where automation is not yet feasible and the proof cost exceeds benefit.
- Currently: 10 VPs. No additional manual VPs should be added without explicit justification.

**Static-check** (grep / linting) is selected for:
- Structural invariants that are cheapest to enforce via CI grep or ESLint-style tooling.
- Currently: 1 VP (VP-061 — agent prompt discipline rules presence).

---

## §5 Purity Boundary Alignment

The verification strategy is designed around the purity boundary established in the
architecture. Properties that target the **pure core** (deterministic, side-effect-free
functions) are the primary candidates for Kani and proptest. Properties that target the
**effectful shell** (I/O, git, network) use integration or manual methods.

| Layer | Examples | Verification Method |
|-------|----------|-------------------|
| Pure core (Rust crate functions) | Partition logic, path matching, ABI mapping | kani-proof, proptest, unit-test |
| Effectful shell integration | Dispatcher pipeline, sink fan-out, hook plugin surfaces | integration (bats + Rust harnesses) |
| Shell scripts (SS-07) | Bash hook behavior, registry consistency | unit-test (bats), manual |
| Workflow / process artifacts | Lobster DAG acyclicity, agent prompt discipline | manual, static-check |

---

## §6 Risk Mitigations (Architecture-Level)

### R-NNN Addressed VPs

VPs VP-004, VP-005, VP-006, VP-021 (capability enforcement cluster) directly mitigate
the risk of capability bypass at the security boundary.

VP-022 (dispatcher exit code semantics) and VP-086 (exit-2 propagation for PreCompact)
mitigate the risk of silent-no-op blocking failures.

VP-082, VP-084, VP-085 (PreCompact flush cluster) mitigate the risk of context loss at
compaction boundaries — a HIGH-impact failure mode identified in issue #173 (E-18).

VP-081, VP-087, VP-088 (wave-boundary cluster) mitigate the risk of incorrect or
fabricated wave context at rehydration — directly addresses DI-023.

VP-069, VP-070, VP-071, VP-072, VP-073, VP-074, VP-075, VP-076 (resolver cluster)
mitigate the risk of resolver crash propagation to the dispatcher process.
