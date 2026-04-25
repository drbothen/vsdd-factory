---
document_type: bc-id-mapping
level: ops
producer: codebase-analyzer
phase: 1.4a
timestamp: 2026-04-25T00:00:00
inputs:
  - .factory/specs/architecture/ARCH-INDEX.md
  - .factory/phase-0-ingestion/pass-3-behavioral-contracts.md
  - .factory/phase-0-ingestion/pass-3-behavioral-contracts-deep-r1.md
  - .factory/phase-0-ingestion/pass-3-deep-skills-batch-1.md
  - .factory/phase-0-ingestion/pass-3-deep-skills-batch-2.md
  - .factory/phase-0-ingestion/pass-3-deep-skills-batch-3.md
  - .factory/phase-0-ingestion/pass-3-deep-agents.md
  - .factory/phase-0-ingestion/pass-3-deep-hooks.md
  - .factory/phase-0-ingestion/pass-3-deep-workflows.md
  - .factory/phase-0-ingestion/pass-3-deep-templates-tools-rules.md
  - .factory/phase-0-ingestion/pass-3-deep-rust-tests.md
total_bcs: 1851
---

# BC ID Mapping: BC-AUDIT-NNN to BC-S.SS.NNN

This document maps every brownfield-audit BC ID (`BC-AUDIT-001..2389`) to its
canonical subsystem-scoped ID (`BC-S.SS.NNN`) per the Subsystem Registry in
`.factory/specs/architecture/ARCH-INDEX.md`.

- `BC-S` is the subsystem-prefix integer (1..10) per the `BC-S Prefix` column of the registry.
- `SS` is the section number within that subsystem (01..NN, see taxonomy below).
- `NNN` is the sequential 001..NNN within each `BC-S.SS` group.

The mapping is exhaustive: 1,851 source BCs → 1,851 unique target IDs.

## Subsystem distribution

| Subsystem | Name | BC-S | Count |
|-----------|------|------|-------|
| SS-01 | Hook Dispatcher Core | BC-1 | 99 |
| SS-02 | Hook SDK and Plugin ABI | BC-2 | 22 |
| SS-03 | Observability Sinks | BC-3 | 49 |
| SS-04 | Plugin Ecosystem | BC-4 | 13 |
| SS-05 | Pipeline Orchestration | BC-5 | 627 |
| SS-06 | Skill Catalog | BC-6 | 571 |
| SS-07 | Hook Bash Layer | BC-7 | 192 |
| SS-08 | Templates and Rules | BC-8 | 215 |
| SS-09 | Configuration and Activation | BC-9 | 5 |
| SS-10 | CLI Tools and Bin | BC-10 | 58 |
| **Total** | | | **1,851** |

## Subsystem section taxonomy

Sections are designed so each section groups 5–50 BCs covering one coherent
sub-area. Section numbers may be non-contiguous within a subsystem to leave
room for future additions and to keep related sections clustered.

### SS-01 — Hook Dispatcher Core (BC-1)

| Section | Name | Count |
|---------|------|-------|
| BC-1.01 | Registry and routing schema | 15 |
| BC-1.02 | Hook payload parsing | 5 |
| BC-1.03 | Plugin execution and invocation | 16 |
| BC-1.04 | Engine and ticker | 3 |
| BC-1.05 | Host functions and capabilities | 34 |
| BC-1.06 | Internal log and always-on telemetry | 10 |
| BC-1.07 | End-to-end and regression contracts | 6 |
| BC-1.08 | Architectural and cross-cutting contracts | 6 |
| BC-1.09 | Plugin cache and loader semantics | 4 |

### SS-02 — Hook SDK and Plugin ABI (BC-2)

| Section | Name | Count |
|---------|------|-------|
| BC-2.01 | HookResult and HookPayload SDK contracts | 4 |
| BC-2.02 | SDK host module and FFI | 10 |
| BC-2.04 | HookPayload SDK parsing | 5 |
| BC-2.05 | SDK __internal helpers | 3 |

### SS-03 — Observability Sinks (BC-3)

| Section | Name | Count |
|---------|------|-------|
| BC-3.01 | Sink registry and base contracts | 9 |
| BC-3.02 | File sink behavior | 16 |
| BC-3.03 | OTLP gRPC sink batching and lifecycle | 13 |
| BC-3.04 | Sink router pass-through and extension point | 2 |
| BC-3.05 | Sink integration via dispatcher | 3 |
| BC-3.06 | sink-core base traits and submit/flush | 6 |

### SS-04 — Plugin Ecosystem (BC-4)

| Section | Name | Count |
|---------|------|-------|
| BC-4.01 | legacy-bash-adapter contracts | 6 |
| BC-4.02 | legacy-bash-adapter stderr and exit-code edges | 6 |
| BC-4.03 | capture-commit-activity plugin | 1 |

### SS-05 — Pipeline Orchestration (BC-5)

Sections 01–10 cover the workflow class contracts and per-agent BCs. Sections
20–35 cover per-workflow-file BCs (one section per `.lobster` file).

| Section | Name | Count |
|---------|------|-------|
| BC-5.01 | Workflow .lobster protocol contracts | 11 |
| BC-5.02 | Orchestrator agent and sequence companions | 13 |
| BC-5.03 | UX and accessibility agents | 18 |
| BC-5.04 | Adversary and review agents | 7 |
| BC-5.05 | Spec, architecture, and quality agents | 21 |
| BC-5.06 | Business and story authoring agents | 15 |
| BC-5.07 | Implementation, test, and engineering agents | 49 |
| BC-5.08 | Ops, GitHub, and PR agents | 24 |
| BC-5.09 | Validation and research agents | 19 |
| BC-5.10 | State manager agent | 5 |
| BC-5.20 | Phase 0: Codebase Ingestion workflow | 20 |
| BC-5.21 | Phase 1: Spec Crystallization workflow | 19 |
| BC-5.22 | Phase 2: Story Decomposition workflow | 19 |
| BC-5.23 | Phase 3: TDD Implementation workflow | 21 |
| BC-5.24 | Phase 4: Holdout Evaluation workflow | 8 |
| BC-5.25 | Phase 5: Adversarial Refinement workflow | 9 |
| BC-5.26 | Phase 6: Formal Hardening workflow | 14 |
| BC-5.27 | Phase 7: Convergence workflow | 24 |
| BC-5.28 | Greenfield mode workflow (greenfield.lobster) | 78 |
| BC-5.29 | Brownfield mode workflow (brownfield.lobster) | 31 |
| BC-5.30 | Feature mode workflow (feature.lobster) | 87 |
| BC-5.31 | Code Delivery workflow (code-delivery.lobster) | 28 |
| BC-5.32 | Discovery mode workflow (discovery.lobster) | 34 |
| BC-5.33 | Maintenance mode workflow (maintenance.lobster) | 39 |
| BC-5.34 | Multi-Repo mode workflow (multi-repo.lobster) | 9 |
| BC-5.35 | Planning mode workflow (planning.lobster) | 5 |

### SS-06 — Skill Catalog (BC-6)

Section 01 holds the broad-sweep BCs that asserted skill quality-gate
conventions before per-skill extraction. Section 02 holds the meta-class
contracts (frontmatter shape, invocation surface). Sections 03–19 group the
119 individual skills by functional theme so each section has 4–68 BCs.

| Section | Name | Count |
|---------|------|-------|
| BC-6.01 | Skill quality-gate contracts (broad-sweep) | 6 |
| BC-6.02 | Skill class meta-contracts (frontmatter, invocation, output) | 12 |
| BC-6.03 | Activation and deactivation skills | 9 |
| BC-6.04 | Adversarial and review skills | 37 |
| BC-6.05 | Brownfield, discovery, research skills | 58 |
| BC-6.06 | State and convergence skills | 46 |
| BC-6.07 | Spec creation and validation skills | 68 |
| BC-6.08 | Demo, UX, and design skills | 55 |
| BC-6.09 | DTU (Digital Twin Universe) skills | 6 |
| BC-6.10 | Story delivery skills | 20 |
| BC-6.11 | Factory operations and dashboards skills | 54 |
| BC-6.12 | Feature-mode scoping rules | 4 |
| BC-6.13 | Telemetry and analytics integration skills | 14 |
| BC-6.14 | Artifact and verification skills | 31 |
| BC-6.15 | Brainstorming and writing skills | 11 |
| BC-6.16 | Phase orchestration and mode skills | 43 |
| BC-6.17 | Feature-mode phase skills (f1-f7) | 46 |
| BC-6.18 | PR and release skills | 19 |
| BC-6.19 | Onboarding and setup skills | 32 |

### SS-07 — Hook Bash Layer (BC-7)

| Section | Name | Count |
|---------|------|-------|
| BC-7.01 | Bash hook script contracts (broad-sweep) | 7 |
| BC-7.02 | Validator hook class contracts | 9 |
| BC-7.03 | Routing hooks (PreToolUse, PostToolUse, lifecycle) | 93 |
| BC-7.04 | Validator hook scripts (validate-* and verify-*) | 83 |

### SS-08 — Templates and Rules (BC-8)

Sections 01–18 cover templates (one section per template cluster). Sections
19–27 cover one rules file per section.

| Section | Name | Count |
|---------|------|-------|
| BC-8.01 | Spec hierarchy templates (L1 through L4) | 17 |
| BC-8.02 | Story, epic, and cycle templates | 7 |
| BC-8.03 | Architecture templates | 11 |
| BC-8.04 | Adversarial-review templates | 12 |
| BC-8.05 | Holdout-evaluation templates | 6 |
| BC-8.06 | Convergence and traceability templates | 6 |
| BC-8.07 | Brownfield, discovery, extraction templates | 10 |
| BC-8.08 | Demo evidence templates | 4 |
| BC-8.09 | Verification and report templates | 5 |
| BC-8.10 | DTU (Digital Twin Universe) templates | 3 |
| BC-8.11 | UX templates | 4 |
| BC-8.12 | Design system and UI quality templates | 11 |
| BC-8.13 | Spec lifecycle templates (changelog, drift, withdrawal, gates) | 4 |
| BC-8.14 | State and workflow templates | 10 |
| BC-8.15 | Code-delivery and PR templates | 2 |
| BC-8.16 | Discovery, project, config templates | 14 |
| BC-8.17 | Skill and agent file templates | 3 |
| BC-8.18 | verify-sha-currency template-distributed hook | 1 |
| BC-8.19 | Rules index (_index.md include order) | 1 |
| BC-8.20 | Rules: bash (no stderr suppression, no eval, dependency checks) | 10 |
| BC-8.21 | Rules: factory-protocol (.factory worktree, governance) | 8 |
| BC-8.22 | Rules: git-commits (Conventional Commits, no AI attribution) | 9 |
| BC-8.23 | Rules: rust (forbid unsafe, edition, lints) | 9 |
| BC-8.24 | Rules: spec-format (4-level hierarchy, BC/VP shape) | 12 |
| BC-8.25 | Rules: step-decomposition (VSDD phase 0-7 protocol) | 12 |
| BC-8.26 | Rules: story-completeness (14-check audit) | 16 |
| BC-8.27 | Rules: worktree-protocol (branch hierarchy, merge protocol) | 8 |

### SS-09 — Configuration and Activation (BC-9)

| Section | Name | Count |
|---------|------|-------|
| BC-9.01 | Release and platform variant contracts | 5 |

> Note: SS-09 has only 5 BCs because most activation behavior was extracted as
> per-skill BCs in SS-06.03 (`activate` / `deactivate` skills) rather than as
> separate config-layer BCs. The 5 BCs here cover release semver, CHANGELOG
> protocol, marketplace bot commit, 5-platform CI matrix, and hooks.json
> gitignore policy from pass-3-behavioral-contracts.md section L.

### SS-10 — CLI Tools and Bin (BC-10)

| Section | Name | Count |
|---------|------|-------|
| BC-10.01 | compute-input-hash CLI | 5 |
| BC-10.02 | emit-event CLI | 7 |
| BC-10.03 | factory-dashboard CLI | 5 |
| BC-10.04 | factory-obs CLI | 6 |
| BC-10.05 | factory-query CLI | 6 |
| BC-10.06 | factory-replay CLI | 4 |
| BC-10.07 | factory-report CLI | 4 |
| BC-10.08 | factory-sla CLI | 5 |
| BC-10.09 | lobster-parse CLI | 3 |
| BC-10.10 | multi-repo-scan CLI | 4 |
| BC-10.11 | research-cache CLI | 5 |
| BC-10.12 | wave-state CLI | 4 |

## Cross-cutting BC notes

The following BCs describe a behavior that crosses subsystem boundaries; they
are assigned to the subsystem that owns the **primary responsibility** (typically
the producer of the contract), per the orchestrator instruction.

- `BC-AUDIT-082` (dispatcher exit code is 2 iff at least one block_intent recorded) is the
  dispatcher-side contract for cross-subsystem block propagation; assigned to
  SS-01.08, with implications for SS-04 (legacy-bash-adapter ↔ dispatcher exit-code
  contract) and SS-07 (bash hooks `exit 2` semantics).
- `BC-AUDIT-068` (validate-* family on PostToolUse) is the broad-sweep cataloging
  of 24 validators and is assigned to SS-07.01 alongside the per-validator detail
  in SS-07.04.
- `BC-AUDIT-104..107` (hook capability model, native-vs-legacy ABI) are dispatcher-side
  contracts that bind hook bash layer; classified under SS-07.02 (validator hook
  class contracts) because they describe what the bash layer must declare.
- `BC-AUDIT-061..062` (registry-generation script) bind SS-09 to SS-07; assigned to
  SS-01.07 (regression contracts) because the test asserts dispatcher round-trip.

## Master mapping table

The `Old ID` column is the verbatim BC-AUDIT identifier, the `New ID` is the
canonical `BC-S.SS.NNN`, the `Source` columns cite the originating pass-3 file
and the line of the BC heading, and the `Title` is the truncated headline of
the contract (max 100 chars).

| Old ID | Source file | Source line | New ID | Subsystem | Section name | Title (truncated) |
|--------|-------------|-------------|--------|-----------|--------------|-------------------|
| BC-AUDIT-001 | pass-3-behavioral-contracts.md | 30 | BC-1.01.001 | SS-01 | Registry and routing schema | Registry rejects unknown schema version |
| BC-AUDIT-002 | pass-3-behavioral-contracts.md | 36 | BC-1.01.002 | SS-01 | Registry and routing schema | Registry rejects invalid tool regex at load time |
| BC-AUDIT-003 | pass-3-behavioral-contracts.md | 42 | BC-1.01.003 | SS-01 | Registry and routing schema | Registry rejects unknown entry fields (typo guard) |
| BC-AUDIT-004 | pass-3-behavioral-contracts.md | 48 | BC-1.01.004 | SS-01 | Registry and routing schema | Relative plugin paths resolve against registry file's parent directory |
| BC-AUDIT-005 | pass-3-behavioral-contracts.md | 54 | BC-1.01.005 | SS-01 | Registry and routing schema | Plugin filter requires event match AND (no tool OR tool regex matches) |
| BC-AUDIT-006 | pass-3-behavioral-contracts.md | 60 | BC-1.01.006 | SS-01 | Registry and routing schema | Tiers ordered ascending by priority, registry order preserved within tier |
| BC-AUDIT-2300 | pass-3-deep-rust-tests.md | 101 | BC-1.01.007 | SS-01 | Registry and routing schema | factory-dispatcher::registry::parses_minimal_registry: minimum-viable registry parses with one ho... |
| BC-AUDIT-2301 | pass-3-deep-rust-tests.md | 112 | BC-1.01.008 | SS-01 | Registry and routing schema | factory-dispatcher::registry::config_defaults_to_empty_table_when_absent: missing [hooks.config] ... |
| BC-AUDIT-2302 | pass-3-deep-rust-tests.md | 123 | BC-1.01.009 | SS-01 | Registry and routing schema | factory-dispatcher::registry::config_block_parses_into_entry: [hooks.config] supports nested tabl... |
| BC-AUDIT-2303 | pass-3-deep-rust-tests.md | 134 | BC-1.01.010 | SS-01 | Registry and routing schema | factory-dispatcher::registry::defaults_applied_when_missing: omitted entry timeouts/fuel/priority... |
| BC-AUDIT-2304 | pass-3-deep-rust-tests.md | 145 | BC-1.01.011 | SS-01 | Registry and routing schema | factory-dispatcher::registry::rejects_unknown_on_error_value: on_error="shout" (or any non-{block... |
| BC-AUDIT-2305 | pass-3-deep-rust-tests.md | 156 | BC-1.01.012 | SS-01 | Registry and routing schema | factory-dispatcher::registry::accepts_capabilities_block: [hooks.capabilities] + nested [hooks.ca... |
| BC-AUDIT-2306 | pass-3-deep-rust-tests.md | 167 | BC-1.01.013 | SS-01 | Registry and routing schema | factory-dispatcher::registry::overrides_priority_per_entry: per-entry priority field overrides re... |
| BC-AUDIT-2307 | pass-3-deep-rust-tests.md | 178 | BC-1.01.014 | SS-01 | Registry and routing schema | factory-dispatcher::registry::load_returns_not_found_for_missing_path: missing registry file prod... |
| BC-AUDIT-2308 | pass-3-deep-rust-tests.md | 189 | BC-1.01.015 | SS-01 | Registry and routing schema | factory-dispatcher::routing::group_returns_empty_for_no_matches: no-match payload yields empty ti... |
| BC-AUDIT-007 | pass-3-behavioral-contracts.md | 66 | BC-1.02.001 | SS-01 | Hook payload parsing | HookPayload requires non-empty event_name and session_id |
| BC-AUDIT-008 | pass-3-behavioral-contracts.md | 72 | BC-1.02.002 | SS-01 | Hook payload parsing | HookPayload accepts both `event_name` and `hook_event_name` |
| BC-AUDIT-2309 | pass-3-deep-rust-tests.md | 200 | BC-1.02.003 | SS-01 | Hook payload parsing | factory-dispatcher::payload::parses_pretooluse: PreToolUse envelope deserializes with tool_input ... |
| BC-AUDIT-2310 | pass-3-deep-rust-tests.md | 211 | BC-1.02.004 | SS-01 | Hook payload parsing | factory-dispatcher::payload::parses_posttooluse_with_response: PostToolUse envelope carries tool_... |
| BC-AUDIT-2311 | pass-3-deep-rust-tests.md | 222 | BC-1.02.005 | SS-01 | Hook payload parsing | factory-dispatcher::payload::accepts_session_event_without_tool_name: SessionStart parses with to... |
| BC-AUDIT-009 | pass-3-behavioral-contracts.md | 80 | BC-1.03.001 | SS-01 | Plugin execution and invocation | Plugin in infinite loop times out via epoch interruption |
| BC-AUDIT-010 | pass-3-behavioral-contracts.md | 86 | BC-1.03.002 | SS-01 | Plugin execution and invocation | Plugin in tight arithmetic loop runs out of fuel |
| BC-AUDIT-011 | pass-3-behavioral-contracts.md | 92 | BC-1.03.003 | SS-01 | Plugin execution and invocation | Plugin trap (e.g., `unreachable`) reports as Crashed |
| BC-AUDIT-012 | pass-3-behavioral-contracts.md | 98 | BC-1.03.004 | SS-01 | Plugin execution and invocation | Normal plugin returns Ok with exit_code 0 + fuel consumption recorded |
| BC-AUDIT-013 | pass-3-behavioral-contracts.md | 104 | BC-1.03.005 | SS-01 | Plugin execution and invocation | stderr captured per plugin and truncated at 4 KiB with marker |
| BC-AUDIT-014 | pass-3-behavioral-contracts.md | 110 | BC-1.03.006 | SS-01 | Plugin execution and invocation | Empty stderr is omitted from lifecycle events (noise reduction) |
| BC-AUDIT-015 | pass-3-behavioral-contracts.md | 116 | BC-1.03.007 | SS-01 | Plugin execution and invocation | Tier execution preserves between-tier order |
| BC-AUDIT-016 | pass-3-behavioral-contracts.md | 122 | BC-1.03.008 | SS-01 | Plugin execution and invocation | Plugins within a tier execute concurrently |
| BC-AUDIT-017 | pass-3-behavioral-contracts.md | 128 | BC-1.03.009 | SS-01 | Plugin execution and invocation | `block_intent` set only when on_error=block AND plugin asks to block |
| BC-AUDIT-018 | pass-3-behavioral-contracts.md | 134 | BC-1.03.010 | SS-01 | Plugin execution and invocation | Per-plugin `plugin_config` spliced into HookPayload before invocation |
| BC-AUDIT-019 | pass-3-behavioral-contracts.md | 140 | BC-1.03.011 | SS-01 | Plugin execution and invocation | WASI exit(N) maps to PluginResult::Ok with exit_code N |
| BC-AUDIT-2335 | pass-3-deep-rust-tests.md | 486 | BC-1.03.012 | SS-01 | Plugin execution and invocation | factory-dispatcher::executor (integration)::parallel_happy_path_five_plugins_one_tier: 5 plugins ... |
| BC-AUDIT-2336 | pass-3-deep-rust-tests.md | 497 | BC-1.03.013 | SS-01 | Plugin execution and invocation | factory-dispatcher::executor (integration)::crash_does_not_affect_siblings: one Crashed plugin do... |
| BC-AUDIT-2337 | pass-3-deep-rust-tests.md | 508 | BC-1.03.014 | SS-01 | Plugin execution and invocation | factory-dispatcher::executor (integration)::parallel_timeout_does_not_cascade: hang plugin times ... |
| BC-AUDIT-2338 | pass-3-deep-rust-tests.md | 519 | BC-1.03.015 | SS-01 | Plugin execution and invocation | factory-dispatcher::executor (integration)::multi_tier_runs_in_priority_order: tier 10 plugin exe... |
| BC-AUDIT-2339 | pass-3-deep-rust-tests.md | 530 | BC-1.03.016 | SS-01 | Plugin execution and invocation | factory-dispatcher::executor (integration)::empty_tier_set_returns_zero_exit_code: empty tier lis... |
| BC-AUDIT-020 | pass-3-behavioral-contracts.md | 148 | BC-1.04.001 | SS-01 | Engine and ticker | Engine builds with epoch interruption + fuel + reference types |
| BC-AUDIT-021 | pass-3-behavioral-contracts.md | 154 | BC-1.04.002 | SS-01 | Engine and ticker | Epoch ticker advances epoch every 10ms; cooperative shutdown |
| BC-AUDIT-022 | pass-3-behavioral-contracts.md | 160 | BC-1.04.003 | SS-01 | Engine and ticker | timeout_ms_to_epochs rounds up |
| BC-AUDIT-023 | pass-3-behavioral-contracts.md | 168 | BC-1.05.001 | SS-01 | Host functions and capabilities | exec_subprocess denies when no exec_subprocess capability |
| BC-AUDIT-024 | pass-3-behavioral-contracts.md | 174 | BC-1.05.002 | SS-01 | Host functions and capabilities | exec_subprocess denies binaries not on allow-list |
| BC-AUDIT-025 | pass-3-behavioral-contracts.md | 180 | BC-1.05.003 | SS-01 | Host functions and capabilities | exec_subprocess denies shell interpreters without shell_bypass_acknowledged |
| BC-AUDIT-026 | pass-3-behavioral-contracts.md | 186 | BC-1.05.004 | SS-01 | Host functions and capabilities | exec_subprocess refuses setuid/setgid binaries categorically (Unix) |
| BC-AUDIT-027 | pass-3-behavioral-contracts.md | 192 | BC-1.05.005 | SS-01 | Host functions and capabilities | exec_subprocess returns OUTPUT_TOO_LARGE when result exceeds buffer |
| BC-AUDIT-028 | pass-3-behavioral-contracts.md | 198 | BC-1.05.006 | SS-01 | Host functions and capabilities | exec_subprocess result envelope is `i32_LE \| u32_LE_stdout_len \| stdout \| u32_LE_stderr_len \|... |
| BC-AUDIT-029 | pass-3-behavioral-contracts.md | 204 | BC-1.05.007 | SS-01 | Host functions and capabilities | env host fn denies env var not on allow-list |
| BC-AUDIT-030 | pass-3-behavioral-contracts.md | 210 | BC-1.05.008 | SS-01 | Host functions and capabilities | env host fn returns 0 when var allowed but unset |
| BC-AUDIT-031 | pass-3-behavioral-contracts.md | 216 | BC-1.05.009 | SS-01 | Host functions and capabilities | read_file at the StoreData-typed linker layer is currently a CAPABILITY_DENIED stub |
| BC-AUDIT-032 | pass-3-behavioral-contracts.md | 222 | BC-1.05.010 | SS-01 | Host functions and capabilities | Context getters (session_id, dispatcher_trace_id, plugin_root, plugin_version, cwd) always return... |
| BC-AUDIT-033 | pass-3-behavioral-contracts.md | 228 | BC-1.05.011 | SS-01 | Host functions and capabilities | log host fn emits `plugin.log` internal event with level mapped to {trace,debug,info,warn,error} |
| BC-AUDIT-034 | pass-3-behavioral-contracts.md | 234 | BC-1.05.012 | SS-01 | Host functions and capabilities | emit_event filters out reserved field names from plugin payload |
| BC-AUDIT-2314 | pass-3-deep-rust-tests.md | 255 | BC-1.05.013 | SS-01 | Host functions and capabilities | factory-dispatcher::host::emit_event::decode_single_pair: length-prefixed key/value buffer with o... |
| BC-AUDIT-2315 | pass-3-deep-rust-tests.md | 266 | BC-1.05.014 | SS-01 | Host functions and capabilities | factory-dispatcher::host::emit_event::decode_multiple_pairs: 3-pair buffer round-trips with order... |
| BC-AUDIT-2316 | pass-3-deep-rust-tests.md | 277 | BC-1.05.015 | SS-01 | Host functions and capabilities | factory-dispatcher::host::emit_event::decode_empty_buffer_yields_empty_vec: empty input → empty... |
| BC-AUDIT-2317 | pass-3-deep-rust-tests.md | 288 | BC-1.05.016 | SS-01 | Host functions and capabilities | factory-dispatcher::host::emit_event::decode_rejects_truncated_key_length: <4-byte buffer trigger... |
| BC-AUDIT-2318 | pass-3-deep-rust-tests.md | 299 | BC-1.05.017 | SS-01 | Host functions and capabilities | factory-dispatcher::host::emit_event::decode_rejects_truncated_key_bytes: declared key_len exceed... |
| BC-AUDIT-2319 | pass-3-deep-rust-tests.md | 310 | BC-1.05.018 | SS-01 | Host functions and capabilities | factory-dispatcher::host::emit_event::reserved_fields_rejected: every name in RESERVED_FIELDS is ... |
| BC-AUDIT-2320 | pass-3-deep-rust-tests.md | 321 | BC-1.05.019 | SS-01 | Host functions and capabilities | factory-dispatcher::host::emit_event::non_reserved_field_accepted: non-reserved keys (commit_sha,... |
| BC-AUDIT-2321 | pass-3-deep-rust-tests.md | 332 | BC-1.05.020 | SS-01 | Host functions and capabilities | factory-dispatcher::host::log::level_mapping_matches_sdk: level u32 0..=4 maps to {trace,debug,in... |
| BC-AUDIT-2322 | pass-3-deep-rust-tests.md | 343 | BC-1.05.021 | SS-01 | Host functions and capabilities | factory-dispatcher::host::read_file::denies_when_no_capability_block: no Capabilities.read_file b... |
| BC-AUDIT-2323 | pass-3-deep-rust-tests.md | 354 | BC-1.05.022 | SS-01 | Host functions and capabilities | factory-dispatcher::host::read_file::reads_allowed_file: file under path_allow with size <= max_b... |
| BC-AUDIT-2324 | pass-3-deep-rust-tests.md | 365 | BC-1.05.023 | SS-01 | Host functions and capabilities | factory-dispatcher::host::read_file::rejects_path_outside_allow_list: file outside any allow-list... |
| BC-AUDIT-2325 | pass-3-deep-rust-tests.md | 376 | BC-1.05.024 | SS-01 | Host functions and capabilities | factory-dispatcher::host::read_file::rejects_file_exceeding_max_bytes: file size > max_bytes → ... |
| BC-AUDIT-2326 | pass-3-deep-rust-tests.md | 387 | BC-1.05.025 | SS-01 | Host functions and capabilities | factory-dispatcher::host::read_file::relative_path_resolves_under_plugin_root: relative path join... |
| BC-AUDIT-2327 | pass-3-deep-rust-tests.md | 398 | BC-1.05.026 | SS-01 | Host functions and capabilities | factory-dispatcher::host::exec_subprocess::allows_shell_with_acknowledgment: shell_bypass_acknowl... |
| BC-AUDIT-2328 | pass-3-deep-rust-tests.md | 409 | BC-1.05.027 | SS-01 | Host functions and capabilities | factory-dispatcher::host::exec_subprocess::stdin_bytes_reach_subprocess: non-empty stdin_bytes is... |
| BC-AUDIT-2329 | pass-3-deep-rust-tests.md | 420 | BC-1.05.028 | SS-01 | Host functions and capabilities | factory-dispatcher::host::exec_subprocess::binary_allow_matches_basename: allow-list compares aga... |
| BC-AUDIT-2330 | pass-3-deep-rust-tests.md | 431 | BC-1.05.029 | SS-01 | Host functions and capabilities | factory-dispatcher::host::exec_subprocess::is_shell_detects_interpreters: SHELL_NAMES set is bash... |
| BC-AUDIT-2331 | pass-3-deep-rust-tests.md | 442 | BC-1.05.030 | SS-01 | Host functions and capabilities | factory-dispatcher::host::exec_subprocess::decode_args_round_trip: encoded args buffer round-trip... |
| BC-AUDIT-2332 | pass-3-deep-rust-tests.md | 453 | BC-1.05.031 | SS-01 | Host functions and capabilities | factory-dispatcher::host::exec_subprocess::decode_args_rejects_truncated_buffer: declared length ... |
| BC-AUDIT-2333 | pass-3-deep-rust-tests.md | 464 | BC-1.05.032 | SS-01 | Host functions and capabilities | factory-dispatcher::host::exec_subprocess::timeout_enforced: command exceeding timeout_ms is kill... |
| BC-AUDIT-2340 | pass-3-deep-rust-tests.md | 541 | BC-1.05.033 | SS-01 | Host functions and capabilities | factory-dispatcher::host_functions (integration)::setup_linker_registers_every_vsdd_import: setup... |
| BC-AUDIT-2341 | pass-3-deep-rust-tests.md | 552 | BC-1.05.034 | SS-01 | Host functions and capabilities | factory-dispatcher::host_functions (integration)::wat_module_importing_host_functions_instantiate... |
| BC-AUDIT-035 | pass-3-behavioral-contracts.md | 242 | BC-1.06.001 | SS-01 | Internal log and always-on telemetry | Internal log writes are best-effort; never panic; never propagate |
| BC-AUDIT-036 | pass-3-behavioral-contracts.md | 248 | BC-1.06.002 | SS-01 | Internal log and always-on telemetry | Daily rotation by event timestamp produces separate files per UTC date |
| BC-AUDIT-037 | pass-3-behavioral-contracts.md | 254 | BC-1.06.003 | SS-01 | Internal log and always-on telemetry | Internal log auto-creates missing parent directories |
| BC-AUDIT-038 | pass-3-behavioral-contracts.md | 260 | BC-1.06.004 | SS-01 | Internal log and always-on telemetry | prune_old removes only `dispatcher-internal-*.jsonl` files older than threshold |
| BC-AUDIT-039 | pass-3-behavioral-contracts.md | 266 | BC-1.06.005 | SS-01 | Internal log and always-on telemetry | prune_old is no-op when log dir missing |
| BC-AUDIT-040 | pass-3-behavioral-contracts.md | 272 | BC-1.06.006 | SS-01 | Internal log and always-on telemetry | InternalEvent fields flatten to top-level JSON (no nested `fields`) |
| BC-AUDIT-2312 | pass-3-deep-rust-tests.md | 233 | BC-1.06.007 | SS-01 | Internal log and always-on telemetry | factory-dispatcher::internal_log::writes_jsonl_events_with_expected_shape: 10 events with trace_i... |
| BC-AUDIT-2313 | pass-3-deep-rust-tests.md | 244 | BC-1.06.008 | SS-01 | Internal log and always-on telemetry | factory-dispatcher::internal_log::skips_serializing_none_optional_fields: None-valued optional fi... |
| BC-AUDIT-2342 | pass-3-deep-rust-tests.md | 563 | BC-1.06.009 | SS-01 | Internal log and always-on telemetry | factory-dispatcher::internal_log (integration)::startup_flow_writes_parseable_jsonl: 4-event disp... |
| BC-AUDIT-2343 | pass-3-deep-rust-tests.md | 574 | BC-1.06.010 | SS-01 | Internal log and always-on telemetry | factory-dispatcher::internal_log (integration)::write_is_best_effort_when_path_is_a_file: log_dir... |
| BC-AUDIT-059 | pass-3-behavioral-contracts.md | 394 | BC-1.07.001 | SS-01 | End-to-end and regression contracts | All 30+ existing bash hooks fire via legacy-bash-adapter on Linux/macOS |
| BC-AUDIT-060 | pass-3-behavioral-contracts.md | 400 | BC-1.07.002 | SS-01 | End-to-end and regression contracts | `commit.made` events fire reliably on real Claude Code git commit |
| BC-AUDIT-061 | pass-3-behavioral-contracts.md | 406 | BC-1.07.003 | SS-01 | End-to-end and regression contracts | Generated hooks-registry.toml round-trips through Registry::load |
| BC-AUDIT-062 | pass-3-behavioral-contracts.md | 412 | BC-1.07.004 | SS-01 | End-to-end and regression contracts | registry-generation script is idempotent |
| BC-AUDIT-2344 | pass-3-deep-rust-tests.md | 585 | BC-1.07.005 | SS-01 | End-to-end and regression contracts | factory-dispatcher::loads_legacy_registry::every_entry_routes_through_legacy_bash_adapter: every ... |
| BC-AUDIT-2345 | pass-3-deep-rust-tests.md | 596 | BC-1.07.006 | SS-01 | End-to-end and regression contracts | factory-dispatcher::loads_legacy_registry::every_entry_carries_a_script_path: every entry has plu... |
| BC-AUDIT-081 | pass-3-behavioral-contracts.md | 536 | BC-1.08.001 | SS-01 | Architectural and cross-cutting contracts | dispatcher exits 0 on registry/payload/engine errors (non-blocking) |
| BC-AUDIT-082 | pass-3-behavioral-contracts.md | 542 | BC-1.08.002 | SS-01 | Architectural and cross-cutting contracts | dispatcher exit code is 2 iff at least one block_intent recorded |
| BC-AUDIT-083 | pass-3-behavioral-contracts.md | 548 | BC-1.08.003 | SS-01 | Architectural and cross-cutting contracts | dispatcher uses current_thread tokio runtime (not multi-threaded pool) |
| BC-AUDIT-084 | pass-3-behavioral-contracts.md | 554 | BC-1.08.004 | SS-01 | Architectural and cross-cutting contracts | dispatcher uses CLAUDE_PROJECT_DIR for cwd, falling back to current_dir |
| BC-AUDIT-085 | pass-3-behavioral-contracts.md | 560 | BC-1.08.005 | SS-01 | Architectural and cross-cutting contracts | dispatcher injects CLAUDE_PLUGIN_ROOT into base_host_ctx.plugin_root |
| BC-AUDIT-086 | pass-3-behavioral-contracts.md | 566 | BC-1.08.006 | SS-01 | Architectural and cross-cutting contracts | dispatcher projects whole process env into env_view (capability gate enforced at host fn call time) |
| BC-AUDIT-119 | pass-3-behavioral-contracts-deep-r1.md | 350 | BC-1.09.001 | SS-01 | Plugin cache and loader semantics | PluginCache key is `path` only; invalidation is mtime-driven |
| BC-AUDIT-120 | pass-3-behavioral-contracts-deep-r1.md | 357 | BC-1.09.002 | SS-01 | Plugin cache and loader semantics | PluginCache.get_or_compile is thread-safe via Mutex<HashMap> |
| BC-AUDIT-121 | pass-3-behavioral-contracts-deep-r1.md | 364 | BC-1.09.003 | SS-01 | Plugin cache and loader semantics | PluginCache has no eviction policy — entries live for the dispatcher's process lifetime |
| BC-AUDIT-122 | pass-3-behavioral-contracts-deep-r1.md | 373 | BC-1.09.004 | SS-01 | Plugin cache and loader semantics | Missing plugin path returns NotFound; corrupt bytes return Compile; IO errors carry path context |
| BC-AUDIT-050 | pass-3-behavioral-contracts.md | 336 | BC-2.01.001 | SS-02 | HookResult and HookPayload SDK contracts | HookResult serialization is tagged with `outcome` field |
| BC-AUDIT-051 | pass-3-behavioral-contracts.md | 342 | BC-2.01.002 | SS-02 | HookResult and HookPayload SDK contracts | HookResult exit codes Continue=0 / Block=2 / Error=1 |
| BC-AUDIT-052 | pass-3-behavioral-contracts.md | 348 | BC-2.01.003 | SS-02 | HookResult and HookPayload SDK contracts | HOST_ABI_VERSION is 1 in both crates |
| BC-AUDIT-053 | pass-3-behavioral-contracts.md | 354 | BC-2.01.004 | SS-02 | HookResult and HookPayload SDK contracts | SDK HookPayload has `plugin_config` field defaulting to Null |
| BC-AUDIT-125 | pass-3-behavioral-contracts-deep-r1.md | 398 | BC-2.02.001 | SS-02 | SDK host module and FFI | Plugin-author API surface is `vsdd_hook_sdk::host::*`; raw FFI is private (`mod ffi;`) |
| BC-AUDIT-126 | pass-3-behavioral-contracts-deep-r1.md | 405 | BC-2.02.002 | SS-02 | SDK host module and FFI | Bounded host calls are mandatory — `read_file` and `exec_subprocess` REQUIRE `timeout_ms` and a... |
| BC-AUDIT-127 | pass-3-behavioral-contracts-deep-r1.md | 412 | BC-2.02.003 | SS-02 | SDK host module and FFI | HostError code mapping: -1 = CapabilityDenied, -2 = Timeout, -3 = OutputTooLarge, -4 = InvalidArg... |
| BC-AUDIT-128 | pass-3-behavioral-contracts-deep-r1.md | 419 | BC-2.02.004 | SS-02 | SDK host module and FFI | SubprocessResult envelope decoding is paranoid — rejects truncated input rather than panicking |
| BC-AUDIT-129 | pass-3-behavioral-contracts-deep-r1.md | 426 | BC-2.02.005 | SS-02 | SDK host module and FFI | SDK-side `read_string` re-call protocol — host returns required size; SDK reallocates and re-calls |
| BC-AUDIT-130 | pass-3-behavioral-contracts-deep-r1.md | 433 | BC-2.02.006 | SS-02 | SDK host module and FFI | SDK ffi.rs uses `#[link(wasm_import_module = "vsdd")]` on wasm32 targets, host stubs on others |
| BC-AUDIT-2353 | pass-3-deep-rust-tests.md | 688 | BC-2.02.007 | SS-02 | SDK host module and FFI | hook-sdk::host::encode_fields_uses_length_prefix: encode_fields([(k,v)]) lays out key_len\|key\|v... |
| BC-AUDIT-2354 | pass-3-deep-rust-tests.md | 699 | BC-2.02.008 | SS-02 | SDK host module and FFI | hook-sdk::host::encode_args_round_trip: encode_args matches the same length-prefix shape with no ... |
| BC-AUDIT-2355 | pass-3-deep-rust-tests.md | 710 | BC-2.02.009 | SS-02 | SDK host module and FFI | hook-sdk::host::decode_subprocess_result_parses_envelope: SubprocessResult envelope `i32 \| u32 \... |
| BC-AUDIT-2356 | pass-3-deep-rust-tests.md | 721 | BC-2.02.010 | SS-02 | SDK host module and FFI | hook-sdk::host::log_levels_are_stable: LogLevel discriminants 0..=4 are pinned (Trace=0, Debug=1,... |
| BC-AUDIT-2357 | pass-3-deep-rust-tests.md | 732 | BC-2.04.001 | SS-02 | HookPayload SDK parsing | hook-sdk::payload::pretooluse_payload_deserializes: full envelope parses with tool_input populate... |
| BC-AUDIT-2358 | pass-3-deep-rust-tests.md | 743 | BC-2.04.002 | SS-02 | HookPayload SDK parsing | hook-sdk::payload::posttooluse_payload_with_response: SDK payload includes typed access to tool_r... |
| BC-AUDIT-2359 | pass-3-deep-rust-tests.md | 754 | BC-2.04.003 | SS-02 | HookPayload SDK parsing | hook-sdk::payload::lifecycle_payload_without_tool_name: SessionStart parses with tool_name="" and... |
| BC-AUDIT-2360 | pass-3-deep-rust-tests.md | 765 | BC-2.04.004 | SS-02 | HookPayload SDK parsing | hook-sdk::payload::payload_round_trip_via_serde: serialize → deserialize preserves event_name a... |
| BC-AUDIT-2361 | pass-3-deep-rust-tests.md | 776 | BC-2.04.005 | SS-02 | HookPayload SDK parsing | hook-sdk::payload::plugin_config_passes_through_when_present: plugin_config field arrives populat... |
| BC-AUDIT-2350 | pass-3-deep-rust-tests.md | 655 | BC-2.05.001 | SS-02 | SDK __internal helpers | hook-sdk::__internal::panic_message_extracts_static_str: panic of `&str` is extracted into the pa... |
| BC-AUDIT-2351 | pass-3-deep-rust-tests.md | 666 | BC-2.05.002 | SS-02 | SDK __internal helpers | hook-sdk::__internal::panic_message_extracts_string: panic of `String` is extracted |
| BC-AUDIT-2352 | pass-3-deep-rust-tests.md | 677 | BC-2.05.003 | SS-02 | SDK __internal helpers | hook-sdk::__internal::panic_message_falls_back_for_unknown_types: non-string panic payloads retur... |
| BC-AUDIT-041 | pass-3-behavioral-contracts.md | 280 | BC-3.01.001 | SS-03 | Sink registry and base contracts | Empty SinkRegistry submit/flush/shutdown is no-op |
| BC-AUDIT-042 | pass-3-behavioral-contracts.md | 286 | BC-3.01.002 | SS-03 | Sink registry and base contracts | Unknown sink type warns to stderr but does not fail config load |
| BC-AUDIT-043 | pass-3-behavioral-contracts.md | 292 | BC-3.01.003 | SS-03 | Sink registry and base contracts | Sink schema_version != 1 is a hard error |
| BC-AUDIT-044 | pass-3-behavioral-contracts.md | 298 | BC-3.01.004 | SS-03 | Sink registry and base contracts | RoutingFilter empty = pass-through, allow non-empty = whitelist, deny applied after allow |
| BC-AUDIT-045 | pass-3-behavioral-contracts.md | 304 | BC-3.01.005 | SS-03 | Sink registry and base contracts | SinkEvent serializes flat (transparent over Map) |
| BC-AUDIT-046 | pass-3-behavioral-contracts.md | 310 | BC-3.01.006 | SS-03 | Sink registry and base contracts | file sink path template substitutes `{date}`, `{name}`, `{project}` and rejects unknown placeholders |
| BC-AUDIT-047 | pass-3-behavioral-contracts.md | 316 | BC-3.01.007 | SS-03 | Sink registry and base contracts | file sink mpsc bounded at default 1000; submit is non-blocking via try_send |
| BC-AUDIT-048 | pass-3-behavioral-contracts.md | 322 | BC-3.01.008 | SS-03 | Sink registry and base contracts | file sink failures recorded into Mutex<Vec<SinkFailure>> |
| BC-AUDIT-049 | pass-3-behavioral-contracts.md | 328 | BC-3.01.009 | SS-03 | Sink registry and base contracts | otel-grpc sink loads with unreachable endpoint (lazy connect) |
| BC-AUDIT-2368 | pass-3-deep-rust-tests.md | 857 | BC-3.02.001 | SS-03 | File sink behavior | sink-file::template_date_only: `{date}` substitutes to YYYY-MM-DD |
| BC-AUDIT-2369 | pass-3-deep-rust-tests.md | 868 | BC-3.02.002 | SS-03 | File sink behavior | sink-file::template_name_only: `{name}` substitutes to the sink's operator-assigned name |
| BC-AUDIT-2370 | pass-3-deep-rust-tests.md | 879 | BC-3.02.003 | SS-03 | File sink behavior | sink-file::template_project_basename: `{project}` substitutes to the basename of project_dir |
| BC-AUDIT-2371 | pass-3-deep-rust-tests.md | 890 | BC-3.02.004 | SS-03 | File sink behavior | sink-file::template_all_placeholders: `{project}/{name}/{date}` interpolates all three with trail... |
| BC-AUDIT-2372 | pass-3-deep-rust-tests.md | 901 | BC-3.02.005 | SS-03 | File sink behavior | sink-file::template_no_project_yields_empty_basename: template uses {project} but None passed →... |
| BC-AUDIT-2373 | pass-3-deep-rust-tests.md | 912 | BC-3.02.006 | SS-03 | File sink behavior | sink-file::template_unbalanced_brace_treated_literally: opening `{` without closing `}` is treate... |
| BC-AUDIT-2374 | pass-3-deep-rust-tests.md | 923 | BC-3.02.007 | SS-03 | File sink behavior | sink-file::auto_creates_parent_directory: nested non-existent parent dirs are mkdir-p'd on first ... |
| BC-AUDIT-2375 | pass-3-deep-rust-tests.md | 934 | BC-3.02.008 | SS-03 | File sink behavior | sink-file::jsonl_append_preserves_three_events: 3 sequential events produce 3 lines in submission... |
| BC-AUDIT-2376 | pass-3-deep-rust-tests.md | 945 | BC-3.02.009 | SS-03 | File sink behavior | sink-file::routing_filter_drops_excluded_events: allow=["commit.made"] passes 2 of 3 events through |
| BC-AUDIT-2377 | pass-3-deep-rust-tests.md | 956 | BC-3.02.010 | SS-03 | File sink behavior | sink-file::tag_enrichment_writes_tags_onto_every_event: configured tags `env=prod,team=factory` l... |
| BC-AUDIT-2378 | pass-3-deep-rust-tests.md | 967 | BC-3.02.011 | SS-03 | File sink behavior | sink-file::tag_enrichment_does_not_overwrite_producer_fields: tag with key="type" does NOT clobbe... |
| BC-AUDIT-2379 | pass-3-deep-rust-tests.md | 978 | BC-3.02.012 | SS-03 | File sink behavior | sink-file::disabled_sink_drops_every_event: enabled=false → no file written, no events accepted |
| BC-AUDIT-2380 | pass-3-deep-rust-tests.md | 989 | BC-3.02.013 | SS-03 | File sink behavior | sink-file::read_only_path_records_failure_without_panic: read-only target dir → SinkFailure rec... |
| BC-AUDIT-2381 | pass-3-deep-rust-tests.md | 1000 | BC-3.02.014 | SS-03 | File sink behavior | sink-file::backpressure_fills_queue_and_increments_counter: queue_depth=2 + 500 submitted events ... |
| BC-AUDIT-2382 | pass-3-deep-rust-tests.md | 1011 | BC-3.02.015 | SS-03 | File sink behavior | sink-file::shutdown_drains_queued_events: shutdown() drains pending events; post-shutdown submit ... |
| BC-AUDIT-2383 | pass-3-deep-rust-tests.md | 1022 | BC-3.02.016 | SS-03 | File sink behavior | sink-file::config_deserializes_from_toml: minimal TOML config parses with queue_depth defaulting ... |
| BC-AUDIT-137 | pass-3-behavioral-contracts-deep-r1.md | 489 | BC-3.03.001 | SS-03 | OTLP gRPC sink batching and lifecycle | Batch trigger thresholds are independent — `size` (default 100) AND `interval_ms` (default 5000... |
| BC-AUDIT-138 | pass-3-behavioral-contracts-deep-r1.md | 499 | BC-3.03.002 | SS-03 | OTLP gRPC sink batching and lifecycle | Send failure protocol — drop the gRPC client on error; rebuild on next batch (self-healing tran... |
| BC-AUDIT-139 | pass-3-behavioral-contracts-deep-r1.md | 506 | BC-3.03.003 | SS-03 | OTLP gRPC sink batching and lifecycle | Connection lifecycle — endpoint validated EAGERLY at constructor; channel built LAZILY in worke... |
| BC-AUDIT-140 | pass-3-behavioral-contracts-deep-r1.md | 513 | BC-3.03.004 | SS-03 | OTLP gRPC sink batching and lifecycle | Worker thread owns its own current_thread tokio runtime on a dedicated OS thread |
| BC-AUDIT-141 | pass-3-behavioral-contracts-deep-r1.md | 520 | BC-3.03.005 | SS-03 | OTLP gRPC sink batching and lifecycle | Producer-side `submit` is fully non-blocking via `try_send`; overflow increments `queue_full_coun... |
| BC-AUDIT-142 | pass-3-behavioral-contracts-deep-r1.md | 527 | BC-3.03.006 | SS-03 | OTLP gRPC sink batching and lifecycle | `flush()` is a synchronous oneshot round-trip; producer blocks on `rx.blocking_recv()` until the ... |
| BC-AUDIT-143 | pass-3-behavioral-contracts-deep-r1.md | 536 | BC-3.03.007 | SS-03 | OTLP gRPC sink batching and lifecycle | Shutdown drains and joins the worker thread; idempotent post-`accepts` rejection |
| BC-AUDIT-2384 | pass-3-deep-rust-tests.md | 1035 | BC-3.03.008 | SS-03 | OTLP gRPC sink batching and lifecycle | sink-otel-grpc::event_to_log_record_maps_reserved_fields: SinkEvent → LogRecord lifts type→bo... |
| BC-AUDIT-2385 | pass-3-deep-rust-tests.md | 1046 | BC-3.03.009 | SS-03 | OTLP gRPC sink batching and lifecycle | sink-otel-grpc::event_attributes_flatten_non_reserved_fields: non-reserved fields flatten to OTLP... |
| BC-AUDIT-2386 | pass-3-deep-rust-tests.md | 1057 | BC-3.03.010 | SS-03 | OTLP gRPC sink batching and lifecycle | sink-otel-grpc::event_to_log_record_nested_value_serialized_to_string: nested JSON values are str... |
| BC-AUDIT-2387 | pass-3-deep-rust-tests.md | 1068 | BC-3.03.011 | SS-03 | OTLP gRPC sink batching and lifecycle | sink-otel-grpc::event_to_log_record_missing_type_yields_empty_body: producer-bug missing-type yie... |
| BC-AUDIT-2388 | pass-3-deep-rust-tests.md | 1079 | BC-3.03.012 | SS-03 | OTLP gRPC sink batching and lifecycle | sink-otel-grpc::event_to_log_record_missing_ts_yields_zero_timestamp: missing ts_epoch → time_u... |
| BC-AUDIT-2389 | pass-3-deep-rust-tests.md | 1090 | BC-3.03.013 | SS-03 | OTLP gRPC sink batching and lifecycle | sink-otel-grpc::resource_attributes_merge_defaults_with_config: operator overrides win over auto-... |
| BC-AUDIT-123 | pass-3-behavioral-contracts-deep-r1.md | 382 | BC-3.04.001 | SS-03 | Sink router pass-through and extension point | Router is currently a thin pass-through wrapper around SinkRegistry |
| BC-AUDIT-124 | pass-3-behavioral-contracts-deep-r1.md | 389 | BC-3.04.002 | SS-03 | Sink router pass-through and extension point | Router exists as the future extension point for S-4.x retry / circuit-breaker / batching / routin... |
| BC-AUDIT-2334 | pass-3-deep-rust-tests.md | 475 | BC-3.05.001 | SS-03 | Sink integration via dispatcher | factory-dispatcher::sinks::mod::load_builds_file_sink_from_parsed_config: ObservabilityConfig wit... |
| BC-AUDIT-2346 | pass-3-deep-rust-tests.md | 607 | BC-3.05.002 | SS-03 | Sink integration via dispatcher | factory-dispatcher::sinks_file_integration::registry_fans_events_to_file_sinks_with_filter_and_ta... |
| BC-AUDIT-2347 | pass-3-deep-rust-tests.md | 618 | BC-3.05.003 | SS-03 | Sink integration via dispatcher | factory-dispatcher::sinks_otel_grpc (integration)::ten_events_arrive_with_correct_attribute_mappi... |
| BC-AUDIT-2362 | pass-3-deep-rust-tests.md | 789 | BC-3.06.001 | SS-03 | sink-core base traits and submit/flush | sink-core::routing_filter_default_accepts_everything: empty allow + empty deny → every event pa... |
| BC-AUDIT-2363 | pass-3-deep-rust-tests.md | 800 | BC-3.06.002 | SS-03 | sink-core base traits and submit/flush | sink-core::sink_event_event_type_accessor_reads_type_field: SinkEvent.event_type() returns the "t... |
| BC-AUDIT-2364 | pass-3-deep-rust-tests.md | 811 | BC-3.06.003 | SS-03 | sink-core base traits and submit/flush | sink-core::sink_event_event_type_missing_returns_none: no "type" field → event_type() returns None |
| BC-AUDIT-2365 | pass-3-deep-rust-tests.md | 822 | BC-3.06.004 | SS-03 | sink-core base traits and submit/flush | sink-core::sink_event_event_type_non_string_returns_none: "type" set to non-string Value → even... |
| BC-AUDIT-2366 | pass-3-deep-rust-tests.md | 833 | BC-3.06.005 | SS-03 | sink-core base traits and submit/flush | sink-core::sink_config_common_defaults_enabled_true: minimal SinkConfigCommon TOML defaults enabl... |
| BC-AUDIT-2367 | pass-3-deep-rust-tests.md | 844 | BC-3.06.006 | SS-03 | sink-core base traits and submit/flush | sink-core::routing_filter_allow_case_sensitive: allow-list compares case-sensitively (Commit.Made... |
| BC-AUDIT-054 | pass-3-behavioral-contracts.md | 362 | BC-4.01.001 | SS-04 | legacy-bash-adapter contracts | legacy-bash-adapter requires non-empty `plugin_config.script_path` |
| BC-AUDIT-055 | pass-3-behavioral-contracts.md | 368 | BC-4.01.002 | SS-04 | legacy-bash-adapter contracts | legacy-bash-adapter strips plugin_config to Null before piping payload to bash |
| BC-AUDIT-056 | pass-3-behavioral-contracts.md | 374 | BC-4.01.003 | SS-04 | legacy-bash-adapter contracts | legacy-bash-adapter maps bash exit codes to HookResult |
| BC-AUDIT-057 | pass-3-behavioral-contracts.md | 380 | BC-4.01.004 | SS-04 | legacy-bash-adapter contracts | legacy-bash-adapter caps combined output at 1 MiB |
| BC-AUDIT-058 | pass-3-behavioral-contracts.md | 386 | BC-4.01.005 | SS-04 | legacy-bash-adapter contracts | legacy-bash-adapter caps wall-clock at 60_000ms (backstop only) |
| BC-AUDIT-2349 | pass-3-deep-rust-tests.md | 642 | BC-4.01.006 | SS-04 | legacy-bash-adapter contracts | hook-plugins::legacy-bash-adapter::passes_payload_bytes_to_bash_with_plugin_config_stripped: re-s... |
| BC-AUDIT-131 | pass-3-behavioral-contracts-deep-r1.md | 442 | BC-4.02.001 | SS-04 | legacy-bash-adapter stderr and exit-code edges | Adapter forwards stdout AND stderr to host log via `host::log_info` / `host::log_warn` (per-strea... |
| BC-AUDIT-132 | pass-3-behavioral-contracts-deep-r1.md | 449 | BC-4.02.002 | SS-04 | legacy-bash-adapter stderr and exit-code edges | Adapter exit-code mapping: 0 → Continue, 2 → Block (reason=first stderr line OR synthetic), o... |
| BC-AUDIT-133 | pass-3-behavioral-contracts-deep-r1.md | 459 | BC-4.02.003 | SS-04 | legacy-bash-adapter stderr and exit-code edges | Adapter's plugin_config.script_path validation is checked BEFORE any subprocess invocation |
| BC-AUDIT-134 | pass-3-behavioral-contracts-deep-r1.md | 466 | BC-4.02.004 | SS-04 | legacy-bash-adapter stderr and exit-code edges | Adapter strips plugin_config to Null before piping to bash — bash hooks predate the field |
| BC-AUDIT-135 | pass-3-behavioral-contracts-deep-r1.md | 473 | BC-4.02.005 | SS-04 | legacy-bash-adapter stderr and exit-code edges | Adapter resolves relative `script_path` under `${CLAUDE_PLUGIN_ROOT}`; absolute paths bypass the ... |
| BC-AUDIT-136 | pass-3-behavioral-contracts-deep-r1.md | 480 | BC-4.02.006 | SS-04 | legacy-bash-adapter stderr and exit-code edges | Adapter's wall-clock cap (BASH_TIMEOUT_MS = 60_000) is a backstop; real per-call deadline = dispa... |
| BC-AUDIT-2348 | pass-3-deep-rust-tests.md | 631 | BC-4.03.001 | SS-04 | capture-commit-activity plugin | hook-plugins::capture-commit-activity::on_hook_returns_zero_in_stub: stub on_hook returns 0 (pre-... |
| BC-AUDIT-108 | pass-3-behavioral-contracts-deep-r1.md | 259 | BC-5.01.001 | SS-05 | Workflow .lobster protocol contracts | A `.lobster` file is YAML at the top level with a single `workflow:` key |
| BC-AUDIT-109 | pass-3-behavioral-contracts-deep-r1.md | 266 | BC-5.01.002 | SS-05 | Workflow .lobster protocol contracts | Workflow `defaults:` block sets default `on_failure`, `max_retries`, `timeout` for unspecified steps |
| BC-AUDIT-110 | pass-3-behavioral-contracts-deep-r1.md | 273 | BC-5.01.003 | SS-05 | Workflow .lobster protocol contracts | Step taxonomy: `type:` enumerated as `skill`, `agent`, `gate`, `loop`, `human-approval`, `sub-wor... |
| BC-AUDIT-111 | pass-3-behavioral-contracts-deep-r1.md | 288 | BC-5.01.004 | SS-05 | Workflow .lobster protocol contracts | Step ordering is by `depends_on:` topological resolution (NOT array position) |
| BC-AUDIT-112 | pass-3-behavioral-contracts-deep-r1.md | 295 | BC-5.01.005 | SS-05 | Workflow .lobster protocol contracts | Steps SHALL declare `condition:` for conditional execution; condition is a string expression eval... |
| BC-AUDIT-113 | pass-3-behavioral-contracts-deep-r1.md | 306 | BC-5.01.006 | SS-05 | Workflow .lobster protocol contracts | Failure handling — `on_failure: escalate` is the workflow default; per-step override via `on_fa... |
| BC-AUDIT-114 | pass-3-behavioral-contracts-deep-r1.md | 313 | BC-5.01.007 | SS-05 | Workflow .lobster protocol contracts | `loop:` blocks are bounded; require `max_iterations` and `exit_condition` |
| BC-AUDIT-115 | pass-3-behavioral-contracts-deep-r1.md | 320 | BC-5.01.008 | SS-05 | Workflow .lobster protocol contracts | `human-approval` steps declare `approval: { prompt, artifacts, timeout }` |
| BC-AUDIT-116 | pass-3-behavioral-contracts-deep-r1.md | 327 | BC-5.01.009 | SS-05 | Workflow .lobster protocol contracts | `agent` steps with `model_tier:` override the default agent model assignment |
| BC-AUDIT-117 | pass-3-behavioral-contracts-deep-r1.md | 334 | BC-5.01.010 | SS-05 | Workflow .lobster protocol contracts | `agent` steps declare `context: { include: [...], exclude: [...] }` to enforce information-asymme... |
| BC-AUDIT-118 | pass-3-behavioral-contracts-deep-r1.md | 341 | BC-5.01.011 | SS-05 | Workflow .lobster protocol contracts | Sub-workflow invocation: `type: sub-workflow` with `sub_workflow: "<filename>.lobster"` |
| BC-AUDIT-879 | pass-3-deep-agents.md | 765 | BC-5.02.001 | SS-05 | Orchestrator agent and sequence companions | orchestrator: never writes any files — delegates all writes |
| BC-AUDIT-880 | pass-3-deep-agents.md | 773 | BC-5.02.002 | SS-05 | Orchestrator agent and sequence companions | orchestrator: never delegates to itself |
| BC-AUDIT-881 | pass-3-deep-agents.md | 781 | BC-5.02.003 | SS-05 | Orchestrator agent and sequence companions | orchestrator: never skips per-story delivery sub-steps |
| BC-AUDIT-882 | pass-3-deep-agents.md | 789 | BC-5.02.004 | SS-05 | Orchestrator agent and sequence companions | orchestrator: never composes PR bodies or gh commands |
| BC-AUDIT-883 | pass-3-deep-agents.md | 797 | BC-5.02.005 | SS-05 | Orchestrator agent and sequence companions | orchestrator: state-manager runs LAST in every burst |
| BC-AUDIT-884 | pass-3-deep-agents.md | 805 | BC-5.02.006 | SS-05 | Orchestrator agent and sequence companions | orchestrator: never sets runTimeoutSeconds below 300 |
| BC-AUDIT-885 | pass-3-deep-agents.md | 813 | BC-5.02.007 | SS-05 | Orchestrator agent and sequence companions | orchestrator: input-hash drift check before Phase 1/2/3/7 human approval |
| BC-AUDIT-886 | pass-3-deep-agents.md | 821 | BC-5.02.008 | SS-05 | Orchestrator agent and sequence companions | orchestrator: prepends `cd <project-path> &&` and uses absolute paths in every dispatch |
| BC-AUDIT-887 | pass-3-deep-agents.md | 829 | BC-5.02.009 | SS-05 | Orchestrator agent and sequence companions | orchestrator: workspace resolution at session start (not from env var) |
| BC-AUDIT-888 | pass-3-deep-agents.md | 837 | BC-5.02.010 | SS-05 | Orchestrator agent and sequence companions | orchestrator: 3-clean-passes minimum for adversarial convergence |
| BC-AUDIT-890 | pass-3-deep-agents.md | 853 | BC-5.02.011 | SS-05 | Orchestrator agent and sequence companions | orchestrator: split bursts of >8 artifacts into create + integrate sub-bursts |
| BC-AUDIT-891 | pass-3-deep-agents.md | 861 | BC-5.02.012 | SS-05 | Orchestrator agent and sequence companions | orchestrator: heartbeat is read-only (no spawning, no writes) |
| BC-AUDIT-892 | pass-3-deep-agents.md | 869 | BC-5.02.013 | SS-05 | Orchestrator agent and sequence companions | orchestrator: pipeline resume requires factory-worktree-health BEFORE STATE.md read |
| BC-AUDIT-800 | pass-3-deep-agents.md | 31 | BC-5.03.001 | SS-05 | UX and accessibility agents | accessibility-auditor: WCAG criterion citation is mandatory for every finding |
| BC-AUDIT-801 | pass-3-deep-agents.md | 39 | BC-5.03.002 | SS-05 | UX and accessibility agents | accessibility-auditor: read-only — never modifies source |
| BC-AUDIT-802 | pass-3-deep-agents.md | 47 | BC-5.03.003 | SS-05 | UX and accessibility agents | accessibility-auditor: skip cleanly when product has no UI |
| BC-AUDIT-803 | pass-3-deep-agents.md | 55 | BC-5.03.004 | SS-05 | UX and accessibility agents | accessibility-auditor: automated tools run before manual review |
| BC-AUDIT-804 | pass-3-deep-agents.md | 63 | BC-5.03.005 | SS-05 | UX and accessibility agents | accessibility-auditor: cannot load architecture files |
| BC-AUDIT-838 | pass-3-deep-agents.md | 383 | BC-5.03.006 | SS-05 | UX and accessibility agents | demo-recorder: output strictly to docs/demo-evidence/<STORY-ID>/ |
| BC-AUDIT-839 | pass-3-deep-agents.md | 391 | BC-5.03.007 | SS-05 | UX and accessibility agents | demo-recorder: VHS for CLI, Playwright for web — never plain text captures |
| BC-AUDIT-840 | pass-3-deep-agents.md | 399 | BC-5.03.008 | SS-05 | UX and accessibility agents | demo-recorder: both success AND error paths recorded per AC |
| BC-AUDIT-841 | pass-3-deep-agents.md | 407 | BC-5.03.009 | SS-05 | UX and accessibility agents | demo-recorder: every recording links to a specific AC via AC-NNN naming |
| BC-AUDIT-842 | pass-3-deep-agents.md | 415 | BC-5.03.010 | SS-05 | UX and accessibility agents | demo-recorder: VHS tapes use Wait+Line, not Sleep |
| BC-AUDIT-957 | pass-3-deep-agents.md | 1473 | BC-5.03.011 | SS-05 | UX and accessibility agents | ux-designer: every screen traces to a PRD requirement |
| BC-AUDIT-958 | pass-3-deep-agents.md | 1481 | BC-5.03.012 | SS-05 | UX and accessibility agents | ux-designer: every interaction has both success AND error paths |
| BC-AUDIT-959 | pass-3-deep-agents.md | 1489 | BC-5.03.013 | SS-05 | UX and accessibility agents | ux-designer: sharded UX (UX-INDEX + screen + flow files), never monolithic |
| BC-AUDIT-960 | pass-3-deep-agents.md | 1497 | BC-5.03.014 | SS-05 | UX and accessibility agents | ux-designer: WCAG 2.1 AA documented per screen |
| BC-AUDIT-967 | pass-3-deep-agents.md | 1565 | BC-5.03.015 | SS-05 | UX and accessibility agents | visual-reviewer: analyzes recordings, never source |
| BC-AUDIT-968 | pass-3-deep-agents.md | 1573 | BC-5.03.016 | SS-05 | UX and accessibility agents | visual-reviewer: 4-dimensional satisfaction scoring (functional / visual / timing / completeness) |
| BC-AUDIT-969 | pass-3-deep-agents.md | 1581 | BC-5.03.017 | SS-05 | UX and accessibility agents | visual-reviewer: blank/missing demos report BLOCKED with satisfaction 0.0 |
| BC-AUDIT-970 | pass-3-deep-agents.md | 1589 | BC-5.03.018 | SS-05 | UX and accessibility agents | visual-reviewer: distinguishes intentional changes from regressions |
| BC-AUDIT-805 | pass-3-deep-agents.md | 77 | BC-5.04.001 | SS-05 | Adversary and review agents | adversary: cannot see prior adversarial reviews (information wall) |
| BC-AUDIT-806 | pass-3-deep-agents.md | 85 | BC-5.04.002 | SS-05 | Adversary and review agents | adversary: every finding tagged with HIGH/MEDIUM/LOW confidence |
| BC-AUDIT-807 | pass-3-deep-agents.md | 93 | BC-5.04.003 | SS-05 | Adversary and review agents | adversary: mis-anchoring always blocks convergence |
| BC-AUDIT-808 | pass-3-deep-agents.md | 101 | BC-5.04.004 | SS-05 | Adversary and review agents | adversary: minimum 3 clean passes, max 10 before human escalation |
| BC-AUDIT-809 | pass-3-deep-agents.md | 109 | BC-5.04.005 | SS-05 | Adversary and review agents | adversary: max 3 self-validation iterations per pass (AgenticAKM) |
| BC-AUDIT-810 | pass-3-deep-agents.md | 117 | BC-5.04.006 | SS-05 | Adversary and review agents | adversary: returns findings as chat text, never writes files |
| BC-AUDIT-930 | pass-3-deep-agents.md | 1221 | BC-5.04.007 | SS-05 | Adversary and review agents | spec-reviewer: never re-reports adversary findings |
| BC-AUDIT-811 | pass-3-deep-agents.md | 131 | BC-5.05.001 | SS-05 | Spec, architecture, and quality agents | architect: every module gets a purity boundary classification |
| BC-AUDIT-812 | pass-3-deep-agents.md | 139 | BC-5.05.002 | SS-05 | Spec, architecture, and quality agents | architect: every VP has a viable proof strategy and feasibility note |
| BC-AUDIT-813 | pass-3-deep-agents.md | 147 | BC-5.05.003 | SS-05 | Spec, architecture, and quality agents | architect: ARCH-INDEX must declare deployment_topology |
| BC-AUDIT-814 | pass-3-deep-agents.md | 155 | BC-5.05.004 | SS-05 | Spec, architecture, and quality agents | architect: DTU assessment is mandatory and covers all 6 categories |
| BC-AUDIT-815 | pass-3-deep-agents.md | 163 | BC-5.05.005 | SS-05 | Spec, architecture, and quality agents | architect: VP-INDEX changes propagate in same burst to verification-architecture.md and verificat... |
| BC-AUDIT-816 | pass-3-deep-agents.md | 171 | BC-5.05.006 | SS-05 | Spec, architecture, and quality agents | architect: VP-locking is 5-step protocol, after which VP is immutable |
| BC-AUDIT-830 | pass-3-deep-agents.md | 307 | BC-5.05.007 | SS-05 | Spec, architecture, and quality agents | consistency-validator: 80 criteria, none skipped |
| BC-AUDIT-831 | pass-3-deep-agents.md | 315 | BC-5.05.008 | SS-05 | Spec, architecture, and quality agents | consistency-validator: index-first validation discipline |
| BC-AUDIT-832 | pass-3-deep-agents.md | 323 | BC-5.05.009 | SS-05 | Spec, architecture, and quality agents | consistency-validator: gate fails when blocking findings exist |
| BC-AUDIT-833 | pass-3-deep-agents.md | 331 | BC-5.05.010 | SS-05 | Spec, architecture, and quality agents | consistency-validator: mis-anchoring is never an "Observation" |
| BC-AUDIT-862 | pass-3-deep-agents.md | 605 | BC-5.05.011 | SS-05 | Spec, architecture, and quality agents | formal-verifier: VP withdrawal requires architect approval |
| BC-AUDIT-931 | pass-3-deep-agents.md | 1229 | BC-5.05.012 | SS-05 | Spec, architecture, and quality agents | spec-reviewer: SR-NNN ID space, distinct from ADV-NNN and CR-NNN |
| BC-AUDIT-932 | pass-3-deep-agents.md | 1237 | BC-5.05.013 | SS-05 | Spec, architecture, and quality agents | spec-reviewer: cannot see implementation details (information wall) |
| BC-AUDIT-933 | pass-3-deep-agents.md | 1245 | BC-5.05.014 | SS-05 | Spec, architecture, and quality agents | spec-reviewer: 6-category finding taxonomy |
| BC-AUDIT-934 | pass-3-deep-agents.md | 1259 | BC-5.05.015 | SS-05 | Spec, architecture, and quality agents | spec-steward: never modifies spec content — governance only |
| BC-AUDIT-935 | pass-3-deep-agents.md | 1267 | BC-5.05.016 | SS-05 | Spec, architecture, and quality agents | spec-steward: every spec change requires version bump |
| BC-AUDIT-936 | pass-3-deep-agents.md | 1275 | BC-5.05.017 | SS-05 | Spec, architecture, and quality agents | spec-steward: locked VP enforcement (immutable after lock) |
| BC-AUDIT-937 | pass-3-deep-agents.md | 1283 | BC-5.05.018 | SS-05 | Spec, architecture, and quality agents | spec-steward: append-only IDs and immutable filename slugs |
| BC-AUDIT-948 | pass-3-deep-agents.md | 1389 | BC-5.05.019 | SS-05 | Spec, architecture, and quality agents | technical-writer: documents only current code, never aspirational |
| BC-AUDIT-949 | pass-3-deep-agents.md | 1397 | BC-5.05.020 | SS-05 | Spec, architecture, and quality agents | technical-writer: never modifies source/tests/configs |
| BC-AUDIT-950 | pass-3-deep-agents.md | 1405 | BC-5.05.021 | SS-05 | Spec, architecture, and quality agents | technical-writer: gaps in source documentation explicitly listed |
| BC-AUDIT-817 | pass-3-deep-agents.md | 185 | BC-5.06.001 | SS-05 | Business and story authoring agents | business-analyst: never invents capabilities — must ground in product brief |
| BC-AUDIT-818 | pass-3-deep-agents.md | 193 | BC-5.06.002 | SS-05 | Business and story authoring agents | business-analyst: produces sharded L2 (L2-INDEX + section files), never monolithic |
| BC-AUDIT-819 | pass-3-deep-agents.md | 201 | BC-5.06.003 | SS-05 | Business and story authoring agents | business-analyst: include all template sections (mark N/A with justification) |
| BC-AUDIT-820 | pass-3-deep-agents.md | 209 | BC-5.06.004 | SS-05 | Business and story authoring agents | business-analyst: every ASM has a validation method; every R-NNN has a mitigation |
| BC-AUDIT-908 | pass-3-deep-agents.md | 1021 | BC-5.06.005 | SS-05 | Business and story authoring agents | product-owner: BC-S.SS.NNN numbering scheme |
| BC-AUDIT-909 | pass-3-deep-agents.md | 1029 | BC-5.06.006 | SS-05 | Business and story authoring agents | product-owner: BC H1 heading is title source of truth |
| BC-AUDIT-910 | pass-3-deep-agents.md | 1037 | BC-5.06.007 | SS-05 | Business and story authoring agents | product-owner: append-only IDs and slugs |
| BC-AUDIT-911 | pass-3-deep-agents.md | 1045 | BC-5.06.008 | SS-05 | Business and story authoring agents | product-owner: every domain invariant lifted to a BC |
| BC-AUDIT-912 | pass-3-deep-agents.md | 1053 | BC-5.06.009 | SS-05 | Business and story authoring agents | product-owner: same-burst anchor-back when creating BCs |
| BC-AUDIT-913 | pass-3-deep-agents.md | 1061 | BC-5.06.010 | SS-05 | Business and story authoring agents | product-owner: subsystem ID from ARCH-INDEX, never names |
| BC-AUDIT-943 | pass-3-deep-agents.md | 1343 | BC-5.06.011 | SS-05 | Business and story authoring agents | story-writer: one file per story, never monolithic |
| BC-AUDIT-944 | pass-3-deep-agents.md | 1351 | BC-5.06.012 | SS-05 | Business and story authoring agents | story-writer: every AC traces to a BC clause; six context-engineering sections mandatory |
| BC-AUDIT-945 | pass-3-deep-agents.md | 1359 | BC-5.06.013 | SS-05 | Business and story authoring agents | story-writer: no story exceeds 13 points or 20-30% agent context window |
| BC-AUDIT-946 | pass-3-deep-agents.md | 1367 | BC-5.06.014 | SS-05 | Business and story authoring agents | story-writer: BC array changes propagate to body and ACs in same atomic commit |
| BC-AUDIT-947 | pass-3-deep-agents.md | 1375 | BC-5.06.015 | SS-05 | Business and story authoring agents | story-writer: dependency graph must be acyclic |
| BC-AUDIT-821 | pass-3-deep-agents.md | 223 | BC-5.07.001 | SS-05 | Implementation, test, and engineering agents | code-reviewer: cannot see adversarial reviews (information wall) |
| BC-AUDIT-822 | pass-3-deep-agents.md | 231 | BC-5.07.002 | SS-05 | Implementation, test, and engineering agents | code-reviewer: every finding classified into exactly one of 6 categories |
| BC-AUDIT-823 | pass-3-deep-agents.md | 239 | BC-5.07.003 | SS-05 | Implementation, test, and engineering agents | code-reviewer: pass 2+ never re-reports prior findings |
| BC-AUDIT-824 | pass-3-deep-agents.md | 247 | BC-5.07.004 | SS-05 | Implementation, test, and engineering agents | code-reviewer: convergence verdict line is exact format |
| BC-AUDIT-825 | pass-3-deep-agents.md | 261 | BC-5.07.005 | SS-05 | Implementation, test, and engineering agents | codebase-analyzer: 6-pass protocol with per-pass output files |
| BC-AUDIT-826 | pass-3-deep-agents.md | 269 | BC-5.07.006 | SS-05 | Implementation, test, and engineering agents | codebase-analyzer: never returns inline findings on Write denial |
| BC-AUDIT-827 | pass-3-deep-agents.md | 277 | BC-5.07.007 | SS-05 | Implementation, test, and engineering agents | codebase-analyzer: convergence requires binary novelty (SUBSTANTIVE / NITPICK) |
| BC-AUDIT-828 | pass-3-deep-agents.md | 285 | BC-5.07.008 | SS-05 | Implementation, test, and engineering agents | codebase-analyzer: convergence bounds — min 2 rounds, max 5 before escalation |
| BC-AUDIT-829 | pass-3-deep-agents.md | 293 | BC-5.07.009 | SS-05 | Implementation, test, and engineering agents | codebase-analyzer: state checkpoint at end of every pass |
| BC-AUDIT-850 | pass-3-deep-agents.md | 491 | BC-5.07.010 | SS-05 | Implementation, test, and engineering agents | dtu-validator: never modifies clone source — spawns implementer for fixes |
| BC-AUDIT-852 | pass-3-deep-agents.md | 513 | BC-5.07.011 | SS-05 | Implementation, test, and engineering agents | dx-engineer: never logs API key values — only names + pass/fail |
| BC-AUDIT-853 | pass-3-deep-agents.md | 521 | BC-5.07.012 | SS-05 | Implementation, test, and engineering agents | dx-engineer: blocks pipeline when any of 3 model families unreachable |
| BC-AUDIT-854 | pass-3-deep-agents.md | 529 | BC-5.07.013 | SS-05 | Implementation, test, and engineering agents | dx-engineer: tool installation requires security-reviewer audit |
| BC-AUDIT-855 | pass-3-deep-agents.md | 537 | BC-5.07.014 | SS-05 | Implementation, test, and engineering agents | dx-engineer: SHA pinning of dependencies and Docker images |
| BC-AUDIT-856 | pass-3-deep-agents.md | 551 | BC-5.07.015 | SS-05 | Implementation, test, and engineering agents | e2e-tester: never mocks internal components |
| BC-AUDIT-857 | pass-3-deep-agents.md | 559 | BC-5.07.016 | SS-05 | Implementation, test, and engineering agents | e2e-tester: BC-NNN traceable test naming |
| BC-AUDIT-858 | pass-3-deep-agents.md | 567 | BC-5.07.017 | SS-05 | Implementation, test, and engineering agents | e2e-tester: tests are idempotent and clean up |
| BC-AUDIT-859 | pass-3-deep-agents.md | 575 | BC-5.07.018 | SS-05 | Implementation, test, and engineering agents | e2e-tester: writes tests, not implementation code |
| BC-AUDIT-860 | pass-3-deep-agents.md | 589 | BC-5.07.019 | SS-05 | Implementation, test, and engineering agents | formal-verifier: never marks VP verified without running proof to completion |
| BC-AUDIT-861 | pass-3-deep-agents.md | 597 | BC-5.07.020 | SS-05 | Implementation, test, and engineering agents | formal-verifier: cannot see adversarial reviews (information wall) |
| BC-AUDIT-863 | pass-3-deep-agents.md | 613 | BC-5.07.021 | SS-05 | Implementation, test, and engineering agents | formal-verifier: mutation kill rate enforced per module-criticality tier |
| BC-AUDIT-864 | pass-3-deep-agents.md | 621 | BC-5.07.022 | SS-05 | Implementation, test, and engineering agents | formal-verifier: fuzz targets run ≥5 minutes with no crashes |
| BC-AUDIT-865 | pass-3-deep-agents.md | 629 | BC-5.07.023 | SS-05 | Implementation, test, and engineering agents | formal-verifier: purity boundary audit catches I/O in pure-core |
| BC-AUDIT-869 | pass-3-deep-agents.md | 673 | BC-5.07.024 | SS-05 | Implementation, test, and engineering agents | holdout-evaluator: cannot read source code, specs, or prior reviews |
| BC-AUDIT-870 | pass-3-deep-agents.md | 681 | BC-5.07.025 | SS-05 | Implementation, test, and engineering agents | holdout-evaluator: gate criteria — mean ≥0.85, every critical scenario ≥0.60 |
| BC-AUDIT-871 | pass-3-deep-agents.md | 689 | BC-5.07.026 | SS-05 | Implementation, test, and engineering agents | holdout-evaluator: 0.0–1.0 satisfaction scoring per scenario |
| BC-AUDIT-872 | pass-3-deep-agents.md | 697 | BC-5.07.027 | SS-05 | Implementation, test, and engineering agents | holdout-evaluator: read-only — no Write tool |
| BC-AUDIT-873 | pass-3-deep-agents.md | 711 | BC-5.07.028 | SS-05 | Implementation, test, and engineering agents | implementer: never writes code without a failing test (Red Gate) |
| BC-AUDIT-874 | pass-3-deep-agents.md | 719 | BC-5.07.029 | SS-05 | Implementation, test, and engineering agents | implementer: minimum code per test (TDD discipline) |
| BC-AUDIT-875 | pass-3-deep-agents.md | 727 | BC-5.07.030 | SS-05 | Implementation, test, and engineering agents | implementer: micro-commit per passing test, squash before PR |
| BC-AUDIT-876 | pass-3-deep-agents.md | 735 | BC-5.07.031 | SS-05 | Implementation, test, and engineering agents | implementer: respects purity boundary map |
| BC-AUDIT-877 | pass-3-deep-agents.md | 743 | BC-5.07.032 | SS-05 | Implementation, test, and engineering agents | implementer: HALT only on blocker, impossibility, or 3 consecutive failures |
| BC-AUDIT-878 | pass-3-deep-agents.md | 751 | BC-5.07.033 | SS-05 | Implementation, test, and engineering agents | implementer: status reporting in {DONE, DONE_WITH_CONCERNS, NEEDS_CONTEXT, BLOCKED} |
| BC-AUDIT-893 | pass-3-deep-agents.md | 883 | BC-5.07.034 | SS-05 | Implementation, test, and engineering agents | performance-engineer: never modifies source code — measurement only |
| BC-AUDIT-894 | pass-3-deep-agents.md | 891 | BC-5.07.035 | SS-05 | Implementation, test, and engineering agents | performance-engineer: capture baseline BEFORE changes |
| BC-AUDIT-895 | pass-3-deep-agents.md | 899 | BC-5.07.036 | SS-05 | Implementation, test, and engineering agents | performance-engineer: numerical thresholds only, never qualitative |
| BC-AUDIT-896 | pass-3-deep-agents.md | 907 | BC-5.07.037 | SS-05 | Implementation, test, and engineering agents | performance-engineer: every NFR-NNN gets a compliance row |
| BC-AUDIT-919 | pass-3-deep-agents.md | 1121 | BC-5.07.038 | SS-05 | Implementation, test, and engineering agents | security-reviewer: cite CWE/CVE for every finding |
| BC-AUDIT-920 | pass-3-deep-agents.md | 1129 | BC-5.07.039 | SS-05 | Implementation, test, and engineering agents | security-reviewer: 4-tier severity (CRITICAL/HIGH/MEDIUM/LOW) |
| BC-AUDIT-921 | pass-3-deep-agents.md | 1137 | BC-5.07.040 | SS-05 | Implementation, test, and engineering agents | security-reviewer: cannot see implementer reasoning (information wall) |
| BC-AUDIT-922 | pass-3-deep-agents.md | 1145 | BC-5.07.041 | SS-05 | Implementation, test, and engineering agents | security-reviewer: never dismiss without documented reasoning |
| BC-AUDIT-923 | pass-3-deep-agents.md | 1153 | BC-5.07.042 | SS-05 | Implementation, test, and engineering agents | security-reviewer: supply chain audit ANY finding blocks installation |
| BC-AUDIT-924 | pass-3-deep-agents.md | 1161 | BC-5.07.043 | SS-05 | Implementation, test, and engineering agents | security-reviewer: posts via gh pr review, never gh pr comment (per-story) |
| BC-AUDIT-951 | pass-3-deep-agents.md | 1419 | BC-5.07.044 | SS-05 | Implementation, test, and engineering agents | test-writer: never writes implementation code |
| BC-AUDIT-952 | pass-3-deep-agents.md | 1427 | BC-5.07.045 | SS-05 | Implementation, test, and engineering agents | test-writer: BC-NNN-traceable test naming required |
| BC-AUDIT-953 | pass-3-deep-agents.md | 1435 | BC-5.07.046 | SS-05 | Implementation, test, and engineering agents | test-writer: Red Gate must be verified — all tests fail before implementation |
| BC-AUDIT-954 | pass-3-deep-agents.md | 1443 | BC-5.07.047 | SS-05 | Implementation, test, and engineering agents | test-writer: never writes vacuously true tests |
| BC-AUDIT-955 | pass-3-deep-agents.md | 1451 | BC-5.07.048 | SS-05 | Implementation, test, and engineering agents | test-writer: property-based tests generate ≥1000 random cases |
| BC-AUDIT-956 | pass-3-deep-agents.md | 1459 | BC-5.07.049 | SS-05 | Implementation, test, and engineering agents | test-writer: uses canonical test vectors from BCs when available |
| BC-AUDIT-834 | pass-3-deep-agents.md | 345 | BC-5.08.001 | SS-05 | Ops, GitHub, and PR agents | data-engineer: every migration has both up and down scripts |
| BC-AUDIT-835 | pass-3-deep-agents.md | 353 | BC-5.08.002 | SS-05 | Ops, GitHub, and PR agents | data-engineer: every field has a privacy classification before schema finalization |
| BC-AUDIT-836 | pass-3-deep-agents.md | 361 | BC-5.08.003 | SS-05 | Ops, GitHub, and PR agents | data-engineer: pure validation logic never touches DB I/O |
| BC-AUDIT-837 | pass-3-deep-agents.md | 369 | BC-5.08.004 | SS-05 | Ops, GitHub, and PR agents | data-engineer: every schema traces to a BC-NNN data contract |
| BC-AUDIT-843 | pass-3-deep-agents.md | 429 | BC-5.08.005 | SS-05 | Ops, GitHub, and PR agents | devops-engineer: never commits secrets |
| BC-AUDIT-844 | pass-3-deep-agents.md | 437 | BC-5.08.006 | SS-05 | Ops, GitHub, and PR agents | devops-engineer: all GitHub Actions pinned to SHA, never tag |
| BC-AUDIT-845 | pass-3-deep-agents.md | 445 | BC-5.08.007 | SS-05 | Ops, GitHub, and PR agents | devops-engineer: develop branch protected with CI status checks |
| BC-AUDIT-846 | pass-3-deep-agents.md | 453 | BC-5.08.008 | SS-05 | Ops, GitHub, and PR agents | devops-engineer: .factory mounted as git worktree on factory-artifacts orphan branch |
| BC-AUDIT-847 | pass-3-deep-agents.md | 461 | BC-5.08.009 | SS-05 | Ops, GitHub, and PR agents | devops-engineer: worktree-per-story discipline (.worktrees/STORY-NNN) |
| BC-AUDIT-866 | pass-3-deep-agents.md | 643 | BC-5.08.010 | SS-05 | Ops, GitHub, and PR agents | github-ops: executes only — never makes decisions |
| BC-AUDIT-867 | pass-3-deep-agents.md | 651 | BC-5.08.011 | SS-05 | Ops, GitHub, and PR agents | github-ops: returns full stdout + stderr unmodified |
| BC-AUDIT-868 | pass-3-deep-agents.md | 659 | BC-5.08.012 | SS-05 | Ops, GitHub, and PR agents | github-ops: retry once on transient errors, then report |
| BC-AUDIT-889 | pass-3-deep-agents.md | 845 | BC-5.08.013 | SS-05 | Ops, GitHub, and PR agents | orchestrator: dispatches state-manager directly for .factory/ commits — never devops-engineer |
| BC-AUDIT-897 | pass-3-deep-agents.md | 921 | BC-5.08.014 | SS-05 | Ops, GitHub, and PR agents | pr-manager: 9-step coordinator, never exits mid-flow |
| BC-AUDIT-898 | pass-3-deep-agents.md | 929 | BC-5.08.015 | SS-05 | Ops, GitHub, and PR agents | pr-manager: delegates all gh/git commands to github-ops |
| BC-AUDIT-899 | pass-3-deep-agents.md | 937 | BC-5.08.016 | SS-05 | Ops, GitHub, and PR agents | pr-manager: max 10 review convergence cycles before human escalation |
| BC-AUDIT-900 | pass-3-deep-agents.md | 945 | BC-5.08.017 | SS-05 | Ops, GitHub, and PR agents | pr-manager: never merges with failing CI checks or unmerged dependency PRs |
| BC-AUDIT-901 | pass-3-deep-agents.md | 953 | BC-5.08.018 | SS-05 | Ops, GitHub, and PR agents | pr-manager: max 3 CI fix cycles before human escalation |
| BC-AUDIT-902 | pass-3-deep-agents.md | 967 | BC-5.08.019 | SS-05 | Ops, GitHub, and PR agents | pr-reviewer: cannot see .factory/ artifacts (information wall) |
| BC-AUDIT-903 | pass-3-deep-agents.md | 975 | BC-5.08.020 | SS-05 | Ops, GitHub, and PR agents | pr-reviewer: posts via `gh pr review`, never `gh pr comment` |
| BC-AUDIT-904 | pass-3-deep-agents.md | 983 | BC-5.08.021 | SS-05 | Ops, GitHub, and PR agents | pr-reviewer: spawns `github-ops` (exact name) for posting |
| BC-AUDIT-905 | pass-3-deep-agents.md | 991 | BC-5.08.022 | SS-05 | Ops, GitHub, and PR agents | pr-reviewer: 3-tier severity classification (BLOCKING / WARNING / NIT) |
| BC-AUDIT-906 | pass-3-deep-agents.md | 999 | BC-5.08.023 | SS-05 | Ops, GitHub, and PR agents | pr-reviewer: no rubber-stamping — explain what was verified |
| BC-AUDIT-907 | pass-3-deep-agents.md | 1007 | BC-5.08.024 | SS-05 | Ops, GitHub, and PR agents | pr-reviewer: demo evidence in `.gif`/`.webm`, not `.txt` |
| BC-AUDIT-848 | pass-3-deep-agents.md | 475 | BC-5.09.001 | SS-05 | Validation and research agents | dtu-validator: never use production API keys with write access |
| BC-AUDIT-849 | pass-3-deep-agents.md | 483 | BC-5.09.002 | SS-05 | Validation and research agents | dtu-validator: fidelity thresholds enforced per L-tier |
| BC-AUDIT-851 | pass-3-deep-agents.md | 499 | BC-5.09.003 | SS-05 | Validation and research agents | dtu-validator: drift >5% triggers stale flag and fix story |
| BC-AUDIT-914 | pass-3-deep-agents.md | 1075 | BC-5.09.004 | SS-05 | Validation and research agents | research-agent: every claim cited; never relies on training data alone |
| BC-AUDIT-915 | pass-3-deep-agents.md | 1083 | BC-5.09.005 | SS-05 | Validation and research agents | research-agent: library versions verified against registries, never training data |
| BC-AUDIT-916 | pass-3-deep-agents.md | 1091 | BC-5.09.006 | SS-05 | Validation and research agents | research-agent: mandatory Research Methods section per report |
| BC-AUDIT-917 | pass-3-deep-agents.md | 1099 | BC-5.09.007 | SS-05 | Validation and research agents | research-agent: never overwrites prior research — appends new dated file |
| BC-AUDIT-918 | pass-3-deep-agents.md | 1107 | BC-5.09.008 | SS-05 | Validation and research agents | research-agent: no source code modification, no Bash |
| BC-AUDIT-925 | pass-3-deep-agents.md | 1175 | BC-5.09.009 | SS-05 | Validation and research agents | session-reviewer: T1 read-only, NEVER writes files |
| BC-AUDIT-926 | pass-3-deep-agents.md | 1183 | BC-5.09.010 | SS-05 | Validation and research agents | session-reviewer: 8-dimensional analysis required |
| BC-AUDIT-927 | pass-3-deep-agents.md | 1191 | BC-5.09.011 | SS-05 | Validation and research agents | session-reviewer: actionable proposals, not vague observations |
| BC-AUDIT-928 | pass-3-deep-agents.md | 1199 | BC-5.09.012 | SS-05 | Validation and research agents | session-reviewer: no information walls — sees everything |
| BC-AUDIT-929 | pass-3-deep-agents.md | 1207 | BC-5.09.013 | SS-05 | Validation and research agents | session-reviewer: tracks own cost; flags >5% of pipeline run cost |
| BC-AUDIT-961 | pass-3-deep-agents.md | 1511 | BC-5.09.014 | SS-05 | Validation and research agents | validate-extraction: behavioral and metric phases must be split |
| BC-AUDIT-962 | pass-3-deep-agents.md | 1519 | BC-5.09.015 | SS-05 | Validation and research agents | validate-extraction: every numeric claim has a (claimed, recounted, delta) triple |
| BC-AUDIT-963 | pass-3-deep-agents.md | 1527 | BC-5.09.016 | SS-05 | Validation and research agents | validate-extraction: max 3 refinement iterations (AgenticAKM) |
| BC-AUDIT-964 | pass-3-deep-agents.md | 1535 | BC-5.09.017 | SS-05 | Validation and research agents | validate-extraction: 4-tier per-item disposition (VERIFIED / INACCURATE / HALLUCINATED / UNVERIFI... |
| BC-AUDIT-965 | pass-3-deep-agents.md | 1543 | BC-5.09.018 | SS-05 | Validation and research agents | validate-extraction: never modifies source code |
| BC-AUDIT-966 | pass-3-deep-agents.md | 1551 | BC-5.09.019 | SS-05 | Validation and research agents | validate-extraction: >50% hallucination rate triggers Level 3 escalation |
| BC-AUDIT-938 | pass-3-deep-agents.md | 1297 | BC-5.10.001 | SS-05 | State manager agent | state-manager: git access scoped to .factory/ only |
| BC-AUDIT-939 | pass-3-deep-agents.md | 1305 | BC-5.10.002 | SS-05 | State manager agent | state-manager: never writes spec documents or source code |
| BC-AUDIT-940 | pass-3-deep-agents.md | 1313 | BC-5.10.003 | SS-05 | State manager agent | state-manager: STATE.md cap of 200 lines (hook blocks at 500) |
| BC-AUDIT-941 | pass-3-deep-agents.md | 1321 | BC-5.10.004 | SS-05 | State manager agent | state-manager: worktree preconditions verified before any .factory/ creation |
| BC-AUDIT-942 | pass-3-deep-agents.md | 1329 | BC-5.10.005 | SS-05 | State manager agent | state-manager: wave-gate remediation uses Single Canonical SHA + Two-Commit Protocol |
| BC-AUDIT-1300 | pass-3-deep-workflows.md | 37 | BC-5.20.001 | SS-05 | Phase 0: Codebase Ingestion workflow | phase-0-codebase-ingestion: identity |
| BC-AUDIT-1301 | pass-3-deep-workflows.md | 47 | BC-5.20.002 | SS-05 | Phase 0: Codebase Ingestion workflow | phase-0-codebase-ingestion: entry-point |
| BC-AUDIT-1302 | pass-3-deep-workflows.md | 58 | BC-5.20.003 | SS-05 | Phase 0: Codebase Ingestion workflow | phase-0-codebase-ingestion: terminal-step |
| BC-AUDIT-1303 | pass-3-deep-workflows.md | 69 | BC-5.20.004 | SS-05 | Phase 0: Codebase Ingestion workflow | phase-0-codebase-ingestion: DAG integrity |
| BC-AUDIT-1304 | pass-3-deep-workflows.md | 78 | BC-5.20.005 | SS-05 | Phase 0: Codebase Ingestion workflow | phase-0-codebase-ingestion: failure semantics |
| BC-AUDIT-1305 | pass-3-deep-workflows.md | 89 | BC-5.20.006 | SS-05 | Phase 0: Codebase Ingestion workflow | phase-0:source-acquisition |
| BC-AUDIT-1306 | pass-3-deep-workflows.md | 99 | BC-5.20.007 | SS-05 | Phase 0: Codebase Ingestion workflow | phase-0:backup-source-acquisition |
| BC-AUDIT-1307 | pass-3-deep-workflows.md | 107 | BC-5.20.008 | SS-05 | Phase 0: Codebase Ingestion workflow | phase-0:broad-sweep |
| BC-AUDIT-1308 | pass-3-deep-workflows.md | 115 | BC-5.20.009 | SS-05 | Phase 0: Codebase Ingestion workflow | phase-0:backup-broad-sweep |
| BC-AUDIT-1309 | pass-3-deep-workflows.md | 122 | BC-5.20.010 | SS-05 | Phase 0: Codebase Ingestion workflow | phase-0:convergence-deepening |
| BC-AUDIT-1310 | pass-3-deep-workflows.md | 129 | BC-5.20.011 | SS-05 | Phase 0: Codebase Ingestion workflow | phase-0:backup-convergence-deepening |
| BC-AUDIT-1311 | pass-3-deep-workflows.md | 135 | BC-5.20.012 | SS-05 | Phase 0: Codebase Ingestion workflow | phase-0:coverage-audit |
| BC-AUDIT-1312 | pass-3-deep-workflows.md | 142 | BC-5.20.013 | SS-05 | Phase 0: Codebase Ingestion workflow | phase-0:backup-coverage-audit |
| BC-AUDIT-1313 | pass-3-deep-workflows.md | 147 | BC-5.20.014 | SS-05 | Phase 0: Codebase Ingestion workflow | phase-0:extraction-validation |
| BC-AUDIT-1314 | pass-3-deep-workflows.md | 154 | BC-5.20.015 | SS-05 | Phase 0: Codebase Ingestion workflow | phase-0:backup-extraction-validation |
| BC-AUDIT-1315 | pass-3-deep-workflows.md | 159 | BC-5.20.016 | SS-05 | Phase 0: Codebase Ingestion workflow | phase-0:final-synthesis |
| BC-AUDIT-1316 | pass-3-deep-workflows.md | 166 | BC-5.20.017 | SS-05 | Phase 0: Codebase Ingestion workflow | phase-0:backup-final-synthesis |
| BC-AUDIT-1317 | pass-3-deep-workflows.md | 171 | BC-5.20.018 | SS-05 | Phase 0: Codebase Ingestion workflow | phase-0:phase-0-gate |
| BC-AUDIT-1318 | pass-3-deep-workflows.md | 178 | BC-5.20.019 | SS-05 | Phase 0: Codebase Ingestion workflow | phase-0:input-hash-drift-check |
| BC-AUDIT-1319 | pass-3-deep-workflows.md | 185 | BC-5.20.020 | SS-05 | Phase 0: Codebase Ingestion workflow | phase-0:human-approval |
| BC-AUDIT-1320 | pass-3-deep-workflows.md | 200 | BC-5.21.001 | SS-05 | Phase 1: Spec Crystallization workflow | phase-1-spec-crystallization: identity |
| BC-AUDIT-1321 | pass-3-deep-workflows.md | 206 | BC-5.21.002 | SS-05 | Phase 1: Spec Crystallization workflow | phase-1: entry-point |
| BC-AUDIT-1322 | pass-3-deep-workflows.md | 212 | BC-5.21.003 | SS-05 | Phase 1: Spec Crystallization workflow | phase-1: terminal-step |
| BC-AUDIT-1323 | pass-3-deep-workflows.md | 218 | BC-5.21.004 | SS-05 | Phase 1: Spec Crystallization workflow | phase-1: DAG integrity |
| BC-AUDIT-1324 | pass-3-deep-workflows.md | 224 | BC-5.21.005 | SS-05 | Phase 1: Spec Crystallization workflow | phase-1: failure semantics |
| BC-AUDIT-1325 | pass-3-deep-workflows.md | 231 | BC-5.21.006 | SS-05 | Phase 1: Spec Crystallization workflow | phase-1:create-brief |
| BC-AUDIT-1326 | pass-3-deep-workflows.md | 238 | BC-5.21.007 | SS-05 | Phase 1: Spec Crystallization workflow | phase-1:backup-create-brief |
| BC-AUDIT-1327 | pass-3-deep-workflows.md | 243 | BC-5.21.008 | SS-05 | Phase 1: Spec Crystallization workflow | phase-1:create-domain-spec |
| BC-AUDIT-1328 | pass-3-deep-workflows.md | 250 | BC-5.21.009 | SS-05 | Phase 1: Spec Crystallization workflow | phase-1:backup-create-domain-spec |
| BC-AUDIT-1329 | pass-3-deep-workflows.md | 255 | BC-5.21.010 | SS-05 | Phase 1: Spec Crystallization workflow | phase-1:create-prd |
| BC-AUDIT-1330 | pass-3-deep-workflows.md | 262 | BC-5.21.011 | SS-05 | Phase 1: Spec Crystallization workflow | phase-1:backup-create-prd |
| BC-AUDIT-1331 | pass-3-deep-workflows.md | 267 | BC-5.21.012 | SS-05 | Phase 1: Spec Crystallization workflow | phase-1:create-architecture |
| BC-AUDIT-1332 | pass-3-deep-workflows.md | 274 | BC-5.21.013 | SS-05 | Phase 1: Spec Crystallization workflow | phase-1:backup-create-architecture |
| BC-AUDIT-1333 | pass-3-deep-workflows.md | 279 | BC-5.21.014 | SS-05 | Phase 1: Spec Crystallization workflow | phase-1:prd-revision |
| BC-AUDIT-1334 | pass-3-deep-workflows.md | 286 | BC-5.21.015 | SS-05 | Phase 1: Spec Crystallization workflow | phase-1:backup-prd-revision |
| BC-AUDIT-1335 | pass-3-deep-workflows.md | 291 | BC-5.21.016 | SS-05 | Phase 1: Spec Crystallization workflow | phase-1:spec-gate |
| BC-AUDIT-1336 | pass-3-deep-workflows.md | 298 | BC-5.21.017 | SS-05 | Phase 1: Spec Crystallization workflow | phase-1:adversarial-spec-review |
| BC-AUDIT-1337 | pass-3-deep-workflows.md | 305 | BC-5.21.018 | SS-05 | Phase 1: Spec Crystallization workflow | phase-1:input-hash-drift-check |
| BC-AUDIT-1338 | pass-3-deep-workflows.md | 311 | BC-5.21.019 | SS-05 | Phase 1: Spec Crystallization workflow | phase-1:human-approval |
| BC-AUDIT-1340 | pass-3-deep-workflows.md | 322 | BC-5.22.001 | SS-05 | Phase 2: Story Decomposition workflow | phase-2-story-decomposition: identity |
| BC-AUDIT-1341 | pass-3-deep-workflows.md | 327 | BC-5.22.002 | SS-05 | Phase 2: Story Decomposition workflow | phase-2: entry-point |
| BC-AUDIT-1342 | pass-3-deep-workflows.md | 331 | BC-5.22.003 | SS-05 | Phase 2: Story Decomposition workflow | phase-2: terminal-step |
| BC-AUDIT-1343 | pass-3-deep-workflows.md | 335 | BC-5.22.004 | SS-05 | Phase 2: Story Decomposition workflow | phase-2: DAG integrity |
| BC-AUDIT-1344 | pass-3-deep-workflows.md | 340 | BC-5.22.005 | SS-05 | Phase 2: Story Decomposition workflow | phase-2: failure semantics |
| BC-AUDIT-1345 | pass-3-deep-workflows.md | 347 | BC-5.22.006 | SS-05 | Phase 2: Story Decomposition workflow | phase-2:define-epics |
| BC-AUDIT-1346 | pass-3-deep-workflows.md | 350 | BC-5.22.007 | SS-05 | Phase 2: Story Decomposition workflow | phase-2:backup-define-epics |
| BC-AUDIT-1347 | pass-3-deep-workflows.md | 353 | BC-5.22.008 | SS-05 | Phase 2: Story Decomposition workflow | phase-2:create-stories |
| BC-AUDIT-1348 | pass-3-deep-workflows.md | 356 | BC-5.22.009 | SS-05 | Phase 2: Story Decomposition workflow | phase-2:backup-create-stories |
| BC-AUDIT-1349 | pass-3-deep-workflows.md | 359 | BC-5.22.010 | SS-05 | Phase 2: Story Decomposition workflow | phase-2:dependency-graph |
| BC-AUDIT-1350 | pass-3-deep-workflows.md | 362 | BC-5.22.011 | SS-05 | Phase 2: Story Decomposition workflow | phase-2:backup-dependency-graph |
| BC-AUDIT-1351 | pass-3-deep-workflows.md | 365 | BC-5.22.012 | SS-05 | Phase 2: Story Decomposition workflow | phase-2:wave-schedule |
| BC-AUDIT-1352 | pass-3-deep-workflows.md | 368 | BC-5.22.013 | SS-05 | Phase 2: Story Decomposition workflow | phase-2:backup-wave-schedule |
| BC-AUDIT-1353 | pass-3-deep-workflows.md | 371 | BC-5.22.014 | SS-05 | Phase 2: Story Decomposition workflow | phase-2:holdout-scenarios |
| BC-AUDIT-1354 | pass-3-deep-workflows.md | 374 | BC-5.22.015 | SS-05 | Phase 2: Story Decomposition workflow | phase-2:backup-holdout-scenarios |
| BC-AUDIT-1355 | pass-3-deep-workflows.md | 377 | BC-5.22.016 | SS-05 | Phase 2: Story Decomposition workflow | phase-2:decomposition-gate |
| BC-AUDIT-1356 | pass-3-deep-workflows.md | 381 | BC-5.22.017 | SS-05 | Phase 2: Story Decomposition workflow | phase-2:adversarial-story-review |
| BC-AUDIT-1357 | pass-3-deep-workflows.md | 385 | BC-5.22.018 | SS-05 | Phase 2: Story Decomposition workflow | phase-2:input-hash-drift-check |
| BC-AUDIT-1358 | pass-3-deep-workflows.md | 388 | BC-5.22.019 | SS-05 | Phase 2: Story Decomposition workflow | phase-2:human-approval |
| BC-AUDIT-1360 | pass-3-deep-workflows.md | 397 | BC-5.23.001 | SS-05 | Phase 3: TDD Implementation workflow | phase-3-tdd-implementation: identity |
| BC-AUDIT-1361 | pass-3-deep-workflows.md | 400 | BC-5.23.002 | SS-05 | Phase 3: TDD Implementation workflow | phase-3: entry-point |
| BC-AUDIT-1362 | pass-3-deep-workflows.md | 403 | BC-5.23.003 | SS-05 | Phase 3: TDD Implementation workflow | phase-3: terminal-step |
| BC-AUDIT-1363 | pass-3-deep-workflows.md | 408 | BC-5.23.004 | SS-05 | Phase 3: TDD Implementation workflow | phase-3: DAG integrity |
| BC-AUDIT-1364 | pass-3-deep-workflows.md | 411 | BC-5.23.005 | SS-05 | Phase 3: TDD Implementation workflow | phase-3: failure semantics |
| BC-AUDIT-1365 | pass-3-deep-workflows.md | 415 | BC-5.23.006 | SS-05 | Phase 3: TDD Implementation workflow | phase-3:create-worktree |
| BC-AUDIT-1366 | pass-3-deep-workflows.md | 418 | BC-5.23.007 | SS-05 | Phase 3: TDD Implementation workflow | phase-3:backup-create-worktree |
| BC-AUDIT-1367 | pass-3-deep-workflows.md | 421 | BC-5.23.008 | SS-05 | Phase 3: TDD Implementation workflow | phase-3:generate-stubs |
| BC-AUDIT-1368 | pass-3-deep-workflows.md | 424 | BC-5.23.009 | SS-05 | Phase 3: TDD Implementation workflow | phase-3:backup-generate-stubs |
| BC-AUDIT-1369 | pass-3-deep-workflows.md | 427 | BC-5.23.010 | SS-05 | Phase 3: TDD Implementation workflow | phase-3:failing-tests |
| BC-AUDIT-1370 | pass-3-deep-workflows.md | 431 | BC-5.23.011 | SS-05 | Phase 3: TDD Implementation workflow | phase-3:backup-failing-tests |
| BC-AUDIT-1371 | pass-3-deep-workflows.md | 434 | BC-5.23.012 | SS-05 | Phase 3: TDD Implementation workflow | phase-3:implement |
| BC-AUDIT-1372 | pass-3-deep-workflows.md | 438 | BC-5.23.013 | SS-05 | Phase 3: TDD Implementation workflow | phase-3:backup-implement |
| BC-AUDIT-1373 | pass-3-deep-workflows.md | 441 | BC-5.23.014 | SS-05 | Phase 3: TDD Implementation workflow | phase-3:record-demos |
| BC-AUDIT-1374 | pass-3-deep-workflows.md | 444 | BC-5.23.015 | SS-05 | Phase 3: TDD Implementation workflow | phase-3:backup-record-demos |
| BC-AUDIT-1375 | pass-3-deep-workflows.md | 447 | BC-5.23.016 | SS-05 | Phase 3: TDD Implementation workflow | phase-3:pr-lifecycle |
| BC-AUDIT-1376 | pass-3-deep-workflows.md | 450 | BC-5.23.017 | SS-05 | Phase 3: TDD Implementation workflow | phase-3:backup-pr-lifecycle |
| BC-AUDIT-1377 | pass-3-deep-workflows.md | 453 | BC-5.23.018 | SS-05 | Phase 3: TDD Implementation workflow | phase-3:cleanup |
| BC-AUDIT-1378 | pass-3-deep-workflows.md | 456 | BC-5.23.019 | SS-05 | Phase 3: TDD Implementation workflow | phase-3:backup-cleanup |
| BC-AUDIT-1379 | pass-3-deep-workflows.md | 460 | BC-5.23.020 | SS-05 | Phase 3: TDD Implementation workflow | phase-3:implementation-gate |
| BC-AUDIT-1380 | pass-3-deep-workflows.md | 464 | BC-5.23.021 | SS-05 | Phase 3: TDD Implementation workflow | phase-3:input-hash-drift-check |
| BC-AUDIT-1390 | pass-3-deep-workflows.md | 473 | BC-5.24.001 | SS-05 | Phase 4: Holdout Evaluation workflow | phase-4-holdout-evaluation: identity |
| BC-AUDIT-1391 | pass-3-deep-workflows.md | 476 | BC-5.24.002 | SS-05 | Phase 4: Holdout Evaluation workflow | phase-4: entry-point |
| BC-AUDIT-1392 | pass-3-deep-workflows.md | 479 | BC-5.24.003 | SS-05 | Phase 4: Holdout Evaluation workflow | phase-4: terminal-step |
| BC-AUDIT-1393 | pass-3-deep-workflows.md | 482 | BC-5.24.004 | SS-05 | Phase 4: Holdout Evaluation workflow | phase-4: DAG integrity |
| BC-AUDIT-1394 | pass-3-deep-workflows.md | 485 | BC-5.24.005 | SS-05 | Phase 4: Holdout Evaluation workflow | phase-4: failure semantics |
| BC-AUDIT-1395 | pass-3-deep-workflows.md | 488 | BC-5.24.006 | SS-05 | Phase 4: Holdout Evaluation workflow | phase-4:scenario-rotation |
| BC-AUDIT-1396 | pass-3-deep-workflows.md | 492 | BC-5.24.007 | SS-05 | Phase 4: Holdout Evaluation workflow | phase-4:holdout-evaluation |
| BC-AUDIT-1397 | pass-3-deep-workflows.md | 495 | BC-5.24.008 | SS-05 | Phase 4: Holdout Evaluation workflow | phase-4:holdout-gate |
| BC-AUDIT-1400 | pass-3-deep-workflows.md | 505 | BC-5.25.001 | SS-05 | Phase 5: Adversarial Refinement workflow | phase-5-adversarial-refinement: identity |
| BC-AUDIT-1401 | pass-3-deep-workflows.md | 508 | BC-5.25.002 | SS-05 | Phase 5: Adversarial Refinement workflow | phase-5: entry-point |
| BC-AUDIT-1402 | pass-3-deep-workflows.md | 511 | BC-5.25.003 | SS-05 | Phase 5: Adversarial Refinement workflow | phase-5: terminal-step |
| BC-AUDIT-1403 | pass-3-deep-workflows.md | 514 | BC-5.25.004 | SS-05 | Phase 5: Adversarial Refinement workflow | phase-5: DAG integrity |
| BC-AUDIT-1404 | pass-3-deep-workflows.md | 517 | BC-5.25.005 | SS-05 | Phase 5: Adversarial Refinement workflow | phase-5: failure semantics |
| BC-AUDIT-1405 | pass-3-deep-workflows.md | 520 | BC-5.25.006 | SS-05 | Phase 5: Adversarial Refinement workflow | phase-5:adversarial-review-loop |
| BC-AUDIT-1406 | pass-3-deep-workflows.md | 524 | BC-5.25.007 | SS-05 | Phase 5: Adversarial Refinement workflow | phase-5:adversary-code-review (nested) |
| BC-AUDIT-1407 | pass-3-deep-workflows.md | 528 | BC-5.25.008 | SS-05 | Phase 5: Adversarial Refinement workflow | phase-5:triage-and-fix (nested) |
| BC-AUDIT-1408 | pass-3-deep-workflows.md | 531 | BC-5.25.009 | SS-05 | Phase 5: Adversarial Refinement workflow | phase-5:gemini-secondary-review |
| BC-AUDIT-1410 | pass-3-deep-workflows.md | 541 | BC-5.26.001 | SS-05 | Phase 6: Formal Hardening workflow | phase-6-formal-hardening: identity |
| BC-AUDIT-1411 | pass-3-deep-workflows.md | 544 | BC-5.26.002 | SS-05 | Phase 6: Formal Hardening workflow | phase-6: entry-point |
| BC-AUDIT-1412 | pass-3-deep-workflows.md | 547 | BC-5.26.003 | SS-05 | Phase 6: Formal Hardening workflow | phase-6: terminal-step |
| BC-AUDIT-1413 | pass-3-deep-workflows.md | 550 | BC-5.26.004 | SS-05 | Phase 6: Formal Hardening workflow | phase-6: DAG integrity |
| BC-AUDIT-1414 | pass-3-deep-workflows.md | 553 | BC-5.26.005 | SS-05 | Phase 6: Formal Hardening workflow | phase-6: failure semantics |
| BC-AUDIT-1415 | pass-3-deep-workflows.md | 556 | BC-5.26.006 | SS-05 | Phase 6: Formal Hardening workflow | phase-6:kani-proofs |
| BC-AUDIT-1416 | pass-3-deep-workflows.md | 559 | BC-5.26.007 | SS-05 | Phase 6: Formal Hardening workflow | phase-6:backup-kani-proofs |
| BC-AUDIT-1417 | pass-3-deep-workflows.md | 562 | BC-5.26.008 | SS-05 | Phase 6: Formal Hardening workflow | phase-6:fuzz-testing |
| BC-AUDIT-1418 | pass-3-deep-workflows.md | 565 | BC-5.26.009 | SS-05 | Phase 6: Formal Hardening workflow | phase-6:backup-fuzz-testing |
| BC-AUDIT-1419 | pass-3-deep-workflows.md | 568 | BC-5.26.010 | SS-05 | Phase 6: Formal Hardening workflow | phase-6:mutation-testing |
| BC-AUDIT-1420 | pass-3-deep-workflows.md | 571 | BC-5.26.011 | SS-05 | Phase 6: Formal Hardening workflow | phase-6:backup-mutation-testing |
| BC-AUDIT-1421 | pass-3-deep-workflows.md | 574 | BC-5.26.012 | SS-05 | Phase 6: Formal Hardening workflow | phase-6:security-scan |
| BC-AUDIT-1422 | pass-3-deep-workflows.md | 577 | BC-5.26.013 | SS-05 | Phase 6: Formal Hardening workflow | phase-6:backup-security-scan |
| BC-AUDIT-1423 | pass-3-deep-workflows.md | 580 | BC-5.26.014 | SS-05 | Phase 6: Formal Hardening workflow | phase-6:hardening-gate |
| BC-AUDIT-1430 | pass-3-deep-workflows.md | 590 | BC-5.27.001 | SS-05 | Phase 7: Convergence workflow | phase-7-convergence: identity |
| BC-AUDIT-1431 | pass-3-deep-workflows.md | 593 | BC-5.27.002 | SS-05 | Phase 7: Convergence workflow | phase-7: entry-point |
| BC-AUDIT-1432 | pass-3-deep-workflows.md | 596 | BC-5.27.003 | SS-05 | Phase 7: Convergence workflow | phase-7: terminal-step |
| BC-AUDIT-1433 | pass-3-deep-workflows.md | 599 | BC-5.27.004 | SS-05 | Phase 7: Convergence workflow | phase-7: DAG integrity |
| BC-AUDIT-1434 | pass-3-deep-workflows.md | 602 | BC-5.27.005 | SS-05 | Phase 7: Convergence workflow | phase-7: failure semantics |
| BC-AUDIT-1435 | pass-3-deep-workflows.md | 605 | BC-5.27.006 | SS-05 | Phase 7: Convergence workflow | phase-7:spec-convergence |
| BC-AUDIT-1436 | pass-3-deep-workflows.md | 608 | BC-5.27.007 | SS-05 | Phase 7: Convergence workflow | phase-7:backup-spec-convergence |
| BC-AUDIT-1437 | pass-3-deep-workflows.md | 611 | BC-5.27.008 | SS-05 | Phase 7: Convergence workflow | phase-7:test-convergence |
| BC-AUDIT-1438 | pass-3-deep-workflows.md | 614 | BC-5.27.009 | SS-05 | Phase 7: Convergence workflow | phase-7:backup-test-convergence |
| BC-AUDIT-1439 | pass-3-deep-workflows.md | 617 | BC-5.27.010 | SS-05 | Phase 7: Convergence workflow | phase-7:implementation-convergence |
| BC-AUDIT-1440 | pass-3-deep-workflows.md | 620 | BC-5.27.011 | SS-05 | Phase 7: Convergence workflow | phase-7:backup-implementation-convergence |
| BC-AUDIT-1441 | pass-3-deep-workflows.md | 623 | BC-5.27.012 | SS-05 | Phase 7: Convergence workflow | phase-7:verification-convergence |
| BC-AUDIT-1442 | pass-3-deep-workflows.md | 626 | BC-5.27.013 | SS-05 | Phase 7: Convergence workflow | phase-7:backup-verification-convergence |
| BC-AUDIT-1443 | pass-3-deep-workflows.md | 629 | BC-5.27.014 | SS-05 | Phase 7: Convergence workflow | phase-7:visual-convergence |
| BC-AUDIT-1444 | pass-3-deep-workflows.md | 632 | BC-5.27.015 | SS-05 | Phase 7: Convergence workflow | phase-7:backup-visual-convergence |
| BC-AUDIT-1445 | pass-3-deep-workflows.md | 635 | BC-5.27.016 | SS-05 | Phase 7: Convergence workflow | phase-7:performance-convergence |
| BC-AUDIT-1446 | pass-3-deep-workflows.md | 638 | BC-5.27.017 | SS-05 | Phase 7: Convergence workflow | phase-7:backup-performance-convergence |
| BC-AUDIT-1447 | pass-3-deep-workflows.md | 641 | BC-5.27.018 | SS-05 | Phase 7: Convergence workflow | phase-7:documentation-convergence |
| BC-AUDIT-1448 | pass-3-deep-workflows.md | 644 | BC-5.27.019 | SS-05 | Phase 7: Convergence workflow | phase-7:backup-documentation-convergence |
| BC-AUDIT-1449 | pass-3-deep-workflows.md | 648 | BC-5.27.020 | SS-05 | Phase 7: Convergence workflow | phase-7:convergence-gate |
| BC-AUDIT-1450 | pass-3-deep-workflows.md | 652 | BC-5.27.021 | SS-05 | Phase 7: Convergence workflow | phase-7:input-hash-drift-check |
| BC-AUDIT-1451 | pass-3-deep-workflows.md | 655 | BC-5.27.022 | SS-05 | Phase 7: Convergence workflow | phase-7:convergence-demo |
| BC-AUDIT-1452 | pass-3-deep-workflows.md | 658 | BC-5.27.023 | SS-05 | Phase 7: Convergence workflow | phase-7:visual-review |
| BC-AUDIT-1453 | pass-3-deep-workflows.md | 661 | BC-5.27.024 | SS-05 | Phase 7: Convergence workflow | phase-7:human-approval |
| BC-AUDIT-1460 | pass-3-deep-workflows.md | 673 | BC-5.28.001 | SS-05 | Greenfield mode workflow (greenfield.lobster) | greenfield-vsdd: identity |
| BC-AUDIT-1461 | pass-3-deep-workflows.md | 677 | BC-5.28.002 | SS-05 | Greenfield mode workflow (greenfield.lobster) | greenfield: entry-point |
| BC-AUDIT-1462 | pass-3-deep-workflows.md | 680 | BC-5.28.003 | SS-05 | Greenfield mode workflow (greenfield.lobster) | greenfield: terminal-step |
| BC-AUDIT-1463 | pass-3-deep-workflows.md | 684 | BC-5.28.004 | SS-05 | Greenfield mode workflow (greenfield.lobster) | greenfield: DAG integrity |
| BC-AUDIT-1464 | pass-3-deep-workflows.md | 688 | BC-5.28.005 | SS-05 | Greenfield mode workflow (greenfield.lobster) | greenfield: failure semantics |
| BC-AUDIT-1465 | pass-3-deep-workflows.md | 691 | BC-5.28.006 | SS-05 | Greenfield mode workflow (greenfield.lobster) | greenfield: cost monitoring (workflow-level) |
| BC-AUDIT-1466 | pass-3-deep-workflows.md | 696 | BC-5.28.007 | SS-05 | Greenfield mode workflow (greenfield.lobster) | greenfield:repo-initialization |
| BC-AUDIT-1467 | pass-3-deep-workflows.md | 699 | BC-5.28.008 | SS-05 | Greenfield mode workflow (greenfield.lobster) | greenfield:factory-worktree-health |
| BC-AUDIT-1468 | pass-3-deep-workflows.md | 702 | BC-5.28.009 | SS-05 | Greenfield mode workflow (greenfield.lobster) | greenfield:factory-worktree-gate |
| BC-AUDIT-1469 | pass-3-deep-workflows.md | 705 | BC-5.28.010 | SS-05 | Greenfield mode workflow (greenfield.lobster) | greenfield:scaffold-claude-md |
| BC-AUDIT-1470 | pass-3-deep-workflows.md | 708 | BC-5.28.011 | SS-05 | Greenfield mode workflow (greenfield.lobster) | greenfield:state-initialization |
| BC-AUDIT-1471 | pass-3-deep-workflows.md | 711 | BC-5.28.012 | SS-05 | Greenfield mode workflow (greenfield.lobster) | greenfield:adaptive-planning |
| BC-AUDIT-1472 | pass-3-deep-workflows.md | 714 | BC-5.28.013 | SS-05 | Greenfield mode workflow (greenfield.lobster) | greenfield:phase-1-spec-crystallization |
| BC-AUDIT-1473 | pass-3-deep-workflows.md | 717 | BC-5.28.014 | SS-05 | Greenfield mode workflow (greenfield.lobster) | greenfield:architect-feasibility-review |
| BC-AUDIT-1474 | pass-3-deep-workflows.md | 720 | BC-5.28.015 | SS-05 | Greenfield mode workflow (greenfield.lobster) | greenfield:prd-revision |
| BC-AUDIT-1475 | pass-3-deep-workflows.md | 723 | BC-5.28.016 | SS-05 | Greenfield mode workflow (greenfield.lobster) | greenfield:phase-1-dtu-assessment |
| BC-AUDIT-1476 | pass-3-deep-workflows.md | 726 | BC-5.28.017 | SS-05 | Greenfield mode workflow (greenfield.lobster) | greenfield:phase-1-gene-transfusion-assessment |
| BC-AUDIT-1477 | pass-3-deep-workflows.md | 729 | BC-5.28.018 | SS-05 | Greenfield mode workflow (greenfield.lobster) | greenfield:phase-1-cicd-setup |
| BC-AUDIT-1478 | pass-3-deep-workflows.md | 732 | BC-5.28.019 | SS-05 | Greenfield mode workflow (greenfield.lobster) | greenfield:phase-1-design-system-bootstrap |
| BC-AUDIT-1479 | pass-3-deep-workflows.md | 735 | BC-5.28.020 | SS-05 | Greenfield mode workflow (greenfield.lobster) | greenfield:phase-1-design-system-approval |
| BC-AUDIT-1480 | pass-3-deep-workflows.md | 738 | BC-5.28.021 | SS-05 | Greenfield mode workflow (greenfield.lobster) | greenfield:phase-1-multi-variant-design |
| BC-AUDIT-1481 | pass-3-deep-workflows.md | 741 | BC-5.28.022 | SS-05 | Greenfield mode workflow (greenfield.lobster) | greenfield:phase-1-multi-variant-approval |
| BC-AUDIT-1482 | pass-3-deep-workflows.md | 744 | BC-5.28.023 | SS-05 | Greenfield mode workflow (greenfield.lobster) | greenfield:phase-1-heuristic-evaluation |
| BC-AUDIT-1483 | pass-3-deep-workflows.md | 747 | BC-5.28.024 | SS-05 | Greenfield mode workflow (greenfield.lobster) | greenfield:phase-1-consistency-audit |
| BC-AUDIT-1484 | pass-3-deep-workflows.md | 750 | BC-5.28.025 | SS-05 | Greenfield mode workflow (greenfield.lobster) | greenfield:phase-1-gate |
| BC-AUDIT-1485 | pass-3-deep-workflows.md | 754 | BC-5.28.026 | SS-05 | Greenfield mode workflow (greenfield.lobster) | greenfield:phase-1d-adversarial-spec-review |
| BC-AUDIT-1486 | pass-3-deep-workflows.md | 757 | BC-5.28.027 | SS-05 | Greenfield mode workflow (greenfield.lobster) | greenfield:phase-1d-spec-review-gemini |
| BC-AUDIT-1487 | pass-3-deep-workflows.md | 760 | BC-5.28.028 | SS-05 | Greenfield mode workflow (greenfield.lobster) | greenfield:phase-1-state-backup |
| BC-AUDIT-1488 | pass-3-deep-workflows.md | 763 | BC-5.28.029 | SS-05 | Greenfield mode workflow (greenfield.lobster) | greenfield:phase-1-human-approval |
| BC-AUDIT-1489 | pass-3-deep-workflows.md | 766 | BC-5.28.030 | SS-05 | Greenfield mode workflow (greenfield.lobster) | greenfield:multi-repo-topology-check |
| BC-AUDIT-1490 | pass-3-deep-workflows.md | 769 | BC-5.28.031 | SS-05 | Greenfield mode workflow (greenfield.lobster) | greenfield:multi-repo-human-confirmation |
| BC-AUDIT-1491 | pass-3-deep-workflows.md | 772 | BC-5.28.032 | SS-05 | Greenfield mode workflow (greenfield.lobster) | greenfield:multi-repo-transition |
| BC-AUDIT-1492 | pass-3-deep-workflows.md | 775 | BC-5.28.033 | SS-05 | Greenfield mode workflow (greenfield.lobster) | greenfield:multi-repo-state-migration |
| BC-AUDIT-1493 | pass-3-deep-workflows.md | 778 | BC-5.28.034 | SS-05 | Greenfield mode workflow (greenfield.lobster) | greenfield:phase-2-story-decomposition |
| BC-AUDIT-1494 | pass-3-deep-workflows.md | 781 | BC-5.28.035 | SS-05 | Greenfield mode workflow (greenfield.lobster) | greenfield:phase-2-consistency-check |
| BC-AUDIT-1495 | pass-3-deep-workflows.md | 784 | BC-5.28.036 | SS-05 | Greenfield mode workflow (greenfield.lobster) | greenfield:phase-2-adversarial-review |
| BC-AUDIT-1496 | pass-3-deep-workflows.md | 787 | BC-5.28.037 | SS-05 | Greenfield mode workflow (greenfield.lobster) | greenfield:phase-2-consistency-audit |
| BC-AUDIT-1497 | pass-3-deep-workflows.md | 790 | BC-5.28.038 | SS-05 | Greenfield mode workflow (greenfield.lobster) | greenfield:phase-2-gate |
| BC-AUDIT-1498 | pass-3-deep-workflows.md | 793 | BC-5.28.039 | SS-05 | Greenfield mode workflow (greenfield.lobster) | greenfield:phase-2-spec-review-gemini |
| BC-AUDIT-1499 | pass-3-deep-workflows.md | 796 | BC-5.28.040 | SS-05 | Greenfield mode workflow (greenfield.lobster) | greenfield:phase-2-state-backup |
| BC-AUDIT-1500 | pass-3-deep-workflows.md | 799 | BC-5.28.041 | SS-05 | Greenfield mode workflow (greenfield.lobster) | greenfield:phase-2-human-approval |
| BC-AUDIT-1501 | pass-3-deep-workflows.md | 802 | BC-5.28.042 | SS-05 | Greenfield mode workflow (greenfield.lobster) | greenfield:dx-engineer-preflight |
| BC-AUDIT-1502 | pass-3-deep-workflows.md | 805 | BC-5.28.043 | SS-05 | Greenfield mode workflow (greenfield.lobster) | greenfield:dx-engineer-preflight-gate |
| BC-AUDIT-1503 | pass-3-deep-workflows.md | 808 | BC-5.28.044 | SS-05 | Greenfield mode workflow (greenfield.lobster) | greenfield:pre-phase-4-dtu-gate |
| BC-AUDIT-1504 | pass-3-deep-workflows.md | 811 | BC-5.28.045 | SS-05 | Greenfield mode workflow (greenfield.lobster) | greenfield:pre-phase-4-cicd-gate |
| BC-AUDIT-1505 | pass-3-deep-workflows.md | 814 | BC-5.28.046 | SS-05 | Greenfield mode workflow (greenfield.lobster) | greenfield:phase-3-per-story-delivery |
| BC-AUDIT-1506 | pass-3-deep-workflows.md | 818 | BC-5.28.047 | SS-05 | Greenfield mode workflow (greenfield.lobster) | greenfield:phase-3-dtu-validation |
| BC-AUDIT-1507 | pass-3-deep-workflows.md | 821 | BC-5.28.048 | SS-05 | Greenfield mode workflow (greenfield.lobster) | greenfield:phase-3-consistency-audit |
| BC-AUDIT-1508 | pass-3-deep-workflows.md | 824 | BC-5.28.049 | SS-05 | Greenfield mode workflow (greenfield.lobster) | greenfield:phase-3-gate |
| BC-AUDIT-1509 | pass-3-deep-workflows.md | 827 | BC-5.28.050 | SS-05 | Greenfield mode workflow (greenfield.lobster) | greenfield:phase-4-scenario-rotation |
| BC-AUDIT-1510 | pass-3-deep-workflows.md | 830 | BC-5.28.051 | SS-05 | Greenfield mode workflow (greenfield.lobster) | greenfield:phase-4-dtu-startup |
| BC-AUDIT-1511 | pass-3-deep-workflows.md | 833 | BC-5.28.052 | SS-05 | Greenfield mode workflow (greenfield.lobster) | greenfield:phase-4-holdout-evaluation |
| BC-AUDIT-1512 | pass-3-deep-workflows.md | 836 | BC-5.28.053 | SS-05 | Greenfield mode workflow (greenfield.lobster) | greenfield:phase-4-gate |
| BC-AUDIT-1513 | pass-3-deep-workflows.md | 839 | BC-5.28.054 | SS-05 | Greenfield mode workflow (greenfield.lobster) | greenfield:phase-4-demo-recording |
| BC-AUDIT-1514 | pass-3-deep-workflows.md | 842 | BC-5.28.055 | SS-05 | Greenfield mode workflow (greenfield.lobster) | greenfield:phase-5-adversarial-refinement |
| BC-AUDIT-1515 | pass-3-deep-workflows.md | 845 | BC-5.28.056 | SS-05 | Greenfield mode workflow (greenfield.lobster) | greenfield:phase-5-gemini-review |
| BC-AUDIT-1516 | pass-3-deep-workflows.md | 848 | BC-5.28.057 | SS-05 | Greenfield mode workflow (greenfield.lobster) | greenfield:phase-4b-holdout-regression |
| BC-AUDIT-1517 | pass-3-deep-workflows.md | 851 | BC-5.28.058 | SS-05 | Greenfield mode workflow (greenfield.lobster) | greenfield:phase-6-formal-hardening |
| BC-AUDIT-1518 | pass-3-deep-workflows.md | 854 | BC-5.28.059 | SS-05 | Greenfield mode workflow (greenfield.lobster) | greenfield:phase-5-fix-delivery |
| BC-AUDIT-1519 | pass-3-deep-workflows.md | 857 | BC-5.28.060 | SS-05 | Greenfield mode workflow (greenfield.lobster) | greenfield:phase-6-dtu-adversarial |
| BC-AUDIT-1520 | pass-3-deep-workflows.md | 860 | BC-5.28.061 | SS-05 | Greenfield mode workflow (greenfield.lobster) | greenfield:phase-5-gate |
| BC-AUDIT-1521 | pass-3-deep-workflows.md | 863 | BC-5.28.062 | SS-05 | Greenfield mode workflow (greenfield.lobster) | greenfield:phase-6-heuristic-evaluation |
| BC-AUDIT-1522 | pass-3-deep-workflows.md | 866 | BC-5.28.063 | SS-05 | Greenfield mode workflow (greenfield.lobster) | greenfield:phase-6-ui-completeness-final |
| BC-AUDIT-1523 | pass-3-deep-workflows.md | 869 | BC-5.28.064 | SS-05 | Greenfield mode workflow (greenfield.lobster) | greenfield:phase-6-responsive-final |
| BC-AUDIT-1524 | pass-3-deep-workflows.md | 872 | BC-5.28.065 | SS-05 | Greenfield mode workflow (greenfield.lobster) | greenfield:phase-6-ui-quality-gate |
| BC-AUDIT-1525 | pass-3-deep-workflows.md | 875 | BC-5.28.066 | SS-05 | Greenfield mode workflow (greenfield.lobster) | greenfield:phase-6-ui-fix-delivery |
| BC-AUDIT-1526 | pass-3-deep-workflows.md | 878 | BC-5.28.067 | SS-05 | Greenfield mode workflow (greenfield.lobster) | greenfield:phase-7-convergence |
| BC-AUDIT-1527 | pass-3-deep-workflows.md | 881 | BC-5.28.068 | SS-05 | Greenfield mode workflow (greenfield.lobster) | greenfield:phase-6-gate |
| BC-AUDIT-1528 | pass-3-deep-workflows.md | 884 | BC-5.28.069 | SS-05 | Greenfield mode workflow (greenfield.lobster) | greenfield:phase-6-final-demo |
| BC-AUDIT-1529 | pass-3-deep-workflows.md | 887 | BC-5.28.070 | SS-05 | Greenfield mode workflow (greenfield.lobster) | greenfield:phase-6-visual-review |
| BC-AUDIT-1530 | pass-3-deep-workflows.md | 890 | BC-5.28.071 | SS-05 | Greenfield mode workflow (greenfield.lobster) | greenfield:phase-6-state-backup |
| BC-AUDIT-1531 | pass-3-deep-workflows.md | 893 | BC-5.28.072 | SS-05 | Greenfield mode workflow (greenfield.lobster) | greenfield:phase-6-human-approval |
| BC-AUDIT-1532 | pass-3-deep-workflows.md | 896 | BC-5.28.073 | SS-05 | Greenfield mode workflow (greenfield.lobster) | greenfield:release |
| BC-AUDIT-1533 | pass-3-deep-workflows.md | 899 | BC-5.28.074 | SS-05 | Greenfield mode workflow (greenfield.lobster) | greenfield:steady-state-handoff |
| BC-AUDIT-1534 | pass-3-deep-workflows.md | 902 | BC-5.28.075 | SS-05 | Greenfield mode workflow (greenfield.lobster) | greenfield:post-feature-validation |
| BC-AUDIT-1535 | pass-3-deep-workflows.md | 905 | BC-5.28.076 | SS-05 | Greenfield mode workflow (greenfield.lobster) | greenfield:session-review |
| BC-AUDIT-1536 | pass-3-deep-workflows.md | 908 | BC-5.28.077 | SS-05 | Greenfield mode workflow (greenfield.lobster) | greenfield:session-review-approval |
| BC-AUDIT-1537 | pass-3-deep-workflows.md | 911 | BC-5.28.078 | SS-05 | Greenfield mode workflow (greenfield.lobster) | greenfield:process-review-decisions |
| BC-AUDIT-1540 | pass-3-deep-workflows.md | 922 | BC-5.29.001 | SS-05 | Brownfield mode workflow (brownfield.lobster) | brownfield-vsdd: identity |
| BC-AUDIT-1541 | pass-3-deep-workflows.md | 925 | BC-5.29.002 | SS-05 | Brownfield mode workflow (brownfield.lobster) | brownfield: entry-point |
| BC-AUDIT-1542 | pass-3-deep-workflows.md | 928 | BC-5.29.003 | SS-05 | Brownfield mode workflow (brownfield.lobster) | brownfield: terminal-step |
| BC-AUDIT-1543 | pass-3-deep-workflows.md | 931 | BC-5.29.004 | SS-05 | Brownfield mode workflow (brownfield.lobster) | brownfield: DAG integrity |
| BC-AUDIT-1544 | pass-3-deep-workflows.md | 935 | BC-5.29.005 | SS-05 | Brownfield mode workflow (brownfield.lobster) | brownfield: failure semantics |
| BC-AUDIT-1545 | pass-3-deep-workflows.md | 940 | BC-5.29.006 | SS-05 | Brownfield mode workflow (brownfield.lobster) | brownfield:environment-setup |
| BC-AUDIT-1546 | pass-3-deep-workflows.md | 944 | BC-5.29.007 | SS-05 | Brownfield mode workflow (brownfield.lobster) | brownfield:environment-gate |
| BC-AUDIT-1547 | pass-3-deep-workflows.md | 947 | BC-5.29.008 | SS-05 | Brownfield mode workflow (brownfield.lobster) | brownfield:repo-verification |
| BC-AUDIT-1548 | pass-3-deep-workflows.md | 951 | BC-5.29.009 | SS-05 | Brownfield mode workflow (brownfield.lobster) | brownfield:factory-worktree-health |
| BC-AUDIT-1549 | pass-3-deep-workflows.md | 954 | BC-5.29.010 | SS-05 | Brownfield mode workflow (brownfield.lobster) | brownfield:factory-worktree-gate |
| BC-AUDIT-1550 | pass-3-deep-workflows.md | 957 | BC-5.29.011 | SS-05 | Brownfield mode workflow (brownfield.lobster) | brownfield:scaffold-claude-md |
| BC-AUDIT-1551 | pass-3-deep-workflows.md | 960 | BC-5.29.012 | SS-05 | Brownfield mode workflow (brownfield.lobster) | brownfield:state-initialization |
| BC-AUDIT-1552 | pass-3-deep-workflows.md | 963 | BC-5.29.013 | SS-05 | Brownfield mode workflow (brownfield.lobster) | brownfield:phase-0-codebase-ingestion |
| BC-AUDIT-1553 | pass-3-deep-workflows.md | 967 | BC-5.29.014 | SS-05 | Brownfield mode workflow (brownfield.lobster) | brownfield:phase-0-artifact-backup |
| BC-AUDIT-1554 | pass-3-deep-workflows.md | 970 | BC-5.29.015 | SS-05 | Brownfield mode workflow (brownfield.lobster) | brownfield:phase-0-gate |
| BC-AUDIT-1555 | pass-3-deep-workflows.md | 974 | BC-5.29.016 | SS-05 | Brownfield mode workflow (brownfield.lobster) | brownfield:phase-0-human-approval |
| BC-AUDIT-1556 | pass-3-deep-workflows.md | 977 | BC-5.29.017 | SS-05 | Brownfield mode workflow (brownfield.lobster) | brownfield:post-phase-0-routing |
| BC-AUDIT-1557 | pass-3-deep-workflows.md | 981 | BC-5.29.018 | SS-05 | Brownfield mode workflow (brownfield.lobster) | brownfield:brownfield-market-intel |
| BC-AUDIT-1558 | pass-3-deep-workflows.md | 984 | BC-5.29.019 | SS-05 | Brownfield mode workflow (brownfield.lobster) | brownfield:brownfield-market-review |
| BC-AUDIT-1559 | pass-3-deep-workflows.md | 987 | BC-5.29.020 | SS-05 | Brownfield mode workflow (brownfield.lobster) | brownfield:detect-cross-language-porting |
| BC-AUDIT-1560 | pass-3-deep-workflows.md | 990 | BC-5.29.021 | SS-05 | Brownfield mode workflow (brownfield.lobster) | brownfield:semport-translation |
| BC-AUDIT-1561 | pass-3-deep-workflows.md | 993 | BC-5.29.022 | SS-05 | Brownfield mode workflow (brownfield.lobster) | brownfield:semport-validation-gate |
| BC-AUDIT-1562 | pass-3-deep-workflows.md | 996 | BC-5.29.023 | SS-05 | Brownfield mode workflow (brownfield.lobster) | brownfield:brownfield-design-system-extract |
| BC-AUDIT-1563 | pass-3-deep-workflows.md | 999 | BC-5.29.024 | SS-05 | Brownfield mode workflow (brownfield.lobster) | brownfield:brownfield-design-system-approval |
| BC-AUDIT-1564 | pass-3-deep-workflows.md | 1002 | BC-5.29.025 | SS-05 | Brownfield mode workflow (brownfield.lobster) | brownfield:brownfield-to-greenfield-transition |
| BC-AUDIT-1565 | pass-3-deep-workflows.md | 1006 | BC-5.29.026 | SS-05 | Brownfield mode workflow (brownfield.lobster) | brownfield:greenfield-pipeline |
| BC-AUDIT-1566 | pass-3-deep-workflows.md | 1009 | BC-5.29.027 | SS-05 | Brownfield mode workflow (brownfield.lobster) | brownfield:multi-repo-handoff-check |
| BC-AUDIT-1567 | pass-3-deep-workflows.md | 1013 | BC-5.29.028 | SS-05 | Brownfield mode workflow (brownfield.lobster) | brownfield:multi-repo-pipeline |
| BC-AUDIT-1568 | pass-3-deep-workflows.md | 1016 | BC-5.29.029 | SS-05 | Brownfield mode workflow (brownfield.lobster) | brownfield:session-review |
| BC-AUDIT-1569 | pass-3-deep-workflows.md | 1019 | BC-5.29.030 | SS-05 | Brownfield mode workflow (brownfield.lobster) | brownfield:session-review-approval |
| BC-AUDIT-1570 | pass-3-deep-workflows.md | 1022 | BC-5.29.031 | SS-05 | Brownfield mode workflow (brownfield.lobster) | brownfield:process-review-decisions |
| BC-AUDIT-1580 | pass-3-deep-workflows.md | 1033 | BC-5.30.001 | SS-05 | Feature mode workflow (feature.lobster) | feature-vsdd: identity |
| BC-AUDIT-1581 | pass-3-deep-workflows.md | 1036 | BC-5.30.002 | SS-05 | Feature mode workflow (feature.lobster) | feature: entry-point |
| BC-AUDIT-1582 | pass-3-deep-workflows.md | 1039 | BC-5.30.003 | SS-05 | Feature mode workflow (feature.lobster) | feature: terminal-step |
| BC-AUDIT-1583 | pass-3-deep-workflows.md | 1042 | BC-5.30.004 | SS-05 | Feature mode workflow (feature.lobster) | feature: DAG integrity |
| BC-AUDIT-1584 | pass-3-deep-workflows.md | 1050 | BC-5.30.005 | SS-05 | Feature mode workflow (feature.lobster) | feature: failure semantics |
| BC-AUDIT-1585 | pass-3-deep-workflows.md | 1055 | BC-5.30.006 | SS-05 | Feature mode workflow (feature.lobster) | feature:factory-worktree-health |
| BC-AUDIT-1586 | pass-3-deep-workflows.md | 1058 | BC-5.30.007 | SS-05 | Feature mode workflow (feature.lobster) | feature:factory-worktree-gate |
| BC-AUDIT-1587 | pass-3-deep-workflows.md | 1061 | BC-5.30.008 | SS-05 | Feature mode workflow (feature.lobster) | feature:feature-cycle-init |
| BC-AUDIT-1588 | pass-3-deep-workflows.md | 1064 | BC-5.30.009 | SS-05 | Feature mode workflow (feature.lobster) | feature:environment-check |
| BC-AUDIT-1589 | pass-3-deep-workflows.md | 1067 | BC-5.30.010 | SS-05 | Feature mode workflow (feature.lobster) | feature:feature-market-intel |
| BC-AUDIT-1590 | pass-3-deep-workflows.md | 1070 | BC-5.30.011 | SS-05 | Feature mode workflow (feature.lobster) | feature:feature-market-review |
| BC-AUDIT-1591 | pass-3-deep-workflows.md | 1073 | BC-5.30.012 | SS-05 | Feature mode workflow (feature.lobster) | feature:establish-demo-baseline |
| BC-AUDIT-1592 | pass-3-deep-workflows.md | 1077 | BC-5.30.013 | SS-05 | Feature mode workflow (feature.lobster) | feature:phase-f1-delta-analysis |
| BC-AUDIT-1593 | pass-3-deep-workflows.md | 1080 | BC-5.30.014 | SS-05 | Feature mode workflow (feature.lobster) | feature:phase-f1-state-backup |
| BC-AUDIT-1594 | pass-3-deep-workflows.md | 1083 | BC-5.30.015 | SS-05 | Feature mode workflow (feature.lobster) | feature:phase-f1-gate |
| BC-AUDIT-1595 | pass-3-deep-workflows.md | 1086 | BC-5.30.016 | SS-05 | Feature mode workflow (feature.lobster) | feature:phase-f1-human-approval |
| BC-AUDIT-1596 | pass-3-deep-workflows.md | 1089 | BC-5.30.017 | SS-05 | Feature mode workflow (feature.lobster) | feature:quick-dev-single-story |
| BC-AUDIT-1597 | pass-3-deep-workflows.md | 1092 | BC-5.30.018 | SS-05 | Feature mode workflow (feature.lobster) | feature:quick-dev-regression |
| BC-AUDIT-1598 | pass-3-deep-workflows.md | 1095 | BC-5.30.019 | SS-05 | Feature mode workflow (feature.lobster) | feature:quick-dev-f7-lite |
| BC-AUDIT-1599 | pass-3-deep-workflows.md | 1098 | BC-5.30.020 | SS-05 | Feature mode workflow (feature.lobster) | feature:quick-dev-f7-human-approval |
| BC-AUDIT-1600 | pass-3-deep-workflows.md | 1101 | BC-5.30.021 | SS-05 | Feature mode workflow (feature.lobster) | feature:quick-dev-release |
| BC-AUDIT-1601 | pass-3-deep-workflows.md | 1104 | BC-5.30.022 | SS-05 | Feature mode workflow (feature.lobster) | feature:quick-dev-state-backup |
| BC-AUDIT-1602 | pass-3-deep-workflows.md | 1107 | BC-5.30.023 | SS-05 | Feature mode workflow (feature.lobster) | feature:bugfix-demo-baseline |
| BC-AUDIT-1603 | pass-3-deep-workflows.md | 1110 | BC-5.30.024 | SS-05 | Feature mode workflow (feature.lobster) | feature:bugfix-single-story-delivery |
| BC-AUDIT-1604 | pass-3-deep-workflows.md | 1113 | BC-5.30.025 | SS-05 | Feature mode workflow (feature.lobster) | feature:bugfix-build-verification |
| BC-AUDIT-1605 | pass-3-deep-workflows.md | 1116 | BC-5.30.026 | SS-05 | Feature mode workflow (feature.lobster) | feature:bugfix-holdout-scoped |
| BC-AUDIT-1606 | pass-3-deep-workflows.md | 1119 | BC-5.30.027 | SS-05 | Feature mode workflow (feature.lobster) | feature:bugfix-f5-scoped |
| BC-AUDIT-1607 | pass-3-deep-workflows.md | 1122 | BC-5.30.028 | SS-05 | Feature mode workflow (feature.lobster) | feature:bugfix-f6-scoped |
| BC-AUDIT-1608 | pass-3-deep-workflows.md | 1125 | BC-5.30.029 | SS-05 | Feature mode workflow (feature.lobster) | feature:bugfix-f6-a11y |
| BC-AUDIT-1609 | pass-3-deep-workflows.md | 1128 | BC-5.30.030 | SS-05 | Feature mode workflow (feature.lobster) | feature:bugfix-demo-comparison |
| BC-AUDIT-1610 | pass-3-deep-workflows.md | 1131 | BC-5.30.031 | SS-05 | Feature mode workflow (feature.lobster) | feature:bugfix-f7-verification |
| BC-AUDIT-1611 | pass-3-deep-workflows.md | 1134 | BC-5.30.032 | SS-05 | Feature mode workflow (feature.lobster) | feature:bugfix-f7-human-approval |
| BC-AUDIT-1612 | pass-3-deep-workflows.md | 1137 | BC-5.30.033 | SS-05 | Feature mode workflow (feature.lobster) | feature:bugfix-release |
| BC-AUDIT-1613 | pass-3-deep-workflows.md | 1140 | BC-5.30.034 | SS-05 | Feature mode workflow (feature.lobster) | feature:bugfix-state-backup |
| BC-AUDIT-1614 | pass-3-deep-workflows.md | 1143 | BC-5.30.035 | SS-05 | Feature mode workflow (feature.lobster) | feature:bugfix-post-monitoring |
| BC-AUDIT-1615 | pass-3-deep-workflows.md | 1146 | BC-5.30.036 | SS-05 | Feature mode workflow (feature.lobster) | feature:phase-f2-spec-evolution |
| BC-AUDIT-1616 | pass-3-deep-workflows.md | 1149 | BC-5.30.037 | SS-05 | Feature mode workflow (feature.lobster) | feature:phase-f2-dtu-reassessment |
| BC-AUDIT-1617 | pass-3-deep-workflows.md | 1152 | BC-5.30.038 | SS-05 | Feature mode workflow (feature.lobster) | feature:phase-f2-gene-transfusion-assessment |
| BC-AUDIT-1618 | pass-3-deep-workflows.md | 1155 | BC-5.30.039 | SS-05 | Feature mode workflow (feature.lobster) | feature:phase-f2-ux-design |
| BC-AUDIT-1619 | pass-3-deep-workflows.md | 1158 | BC-5.30.040 | SS-05 | Feature mode workflow (feature.lobster) | feature:phase-f2-design-system-bootstrap |
| BC-AUDIT-1620 | pass-3-deep-workflows.md | 1161 | BC-5.30.041 | SS-05 | Feature mode workflow (feature.lobster) | feature:phase-f2-multi-variant |
| BC-AUDIT-1621 | pass-3-deep-workflows.md | 1164 | BC-5.30.042 | SS-05 | Feature mode workflow (feature.lobster) | feature:phase-f2-multi-variant-approval |
| BC-AUDIT-1622 | pass-3-deep-workflows.md | 1167 | BC-5.30.043 | SS-05 | Feature mode workflow (feature.lobster) | feature:phase-f2-a11y-review |
| BC-AUDIT-1623 | pass-3-deep-workflows.md | 1170 | BC-5.30.044 | SS-05 | Feature mode workflow (feature.lobster) | feature:phase-f2-adversarial-review |
| BC-AUDIT-1624 | pass-3-deep-workflows.md | 1173 | BC-5.30.045 | SS-05 | Feature mode workflow (feature.lobster) | feature:phase-f2-spec-review-gemini |
| BC-AUDIT-1625 | pass-3-deep-workflows.md | 1176 | BC-5.30.046 | SS-05 | Feature mode workflow (feature.lobster) | feature:phase-f2-state-backup |
| BC-AUDIT-1626 | pass-3-deep-workflows.md | 1179 | BC-5.30.047 | SS-05 | Feature mode workflow (feature.lobster) | feature:phase-f2-gate |
| BC-AUDIT-1627 | pass-3-deep-workflows.md | 1182 | BC-5.30.048 | SS-05 | Feature mode workflow (feature.lobster) | feature:phase-f2-human-approval |
| BC-AUDIT-1628 | pass-3-deep-workflows.md | 1185 | BC-5.30.049 | SS-05 | Feature mode workflow (feature.lobster) | feature:phase-f3-incremental-stories |
| BC-AUDIT-1629 | pass-3-deep-workflows.md | 1188 | BC-5.30.050 | SS-05 | Feature mode workflow (feature.lobster) | feature:phase-f3-spec-review-gemini |
| BC-AUDIT-1630 | pass-3-deep-workflows.md | 1191 | BC-5.30.051 | SS-05 | Feature mode workflow (feature.lobster) | feature:phase-f3-state-backup |
| BC-AUDIT-1631 | pass-3-deep-workflows.md | 1194 | BC-5.30.052 | SS-05 | Feature mode workflow (feature.lobster) | feature:phase-f3-gate |
| BC-AUDIT-1632 | pass-3-deep-workflows.md | 1197 | BC-5.30.053 | SS-05 | Feature mode workflow (feature.lobster) | feature:phase-f3-human-approval |
| BC-AUDIT-1633 | pass-3-deep-workflows.md | 1200 | BC-5.30.054 | SS-05 | Feature mode workflow (feature.lobster) | feature:toolchain-preflight |
| BC-AUDIT-1634 | pass-3-deep-workflows.md | 1203 | BC-5.30.055 | SS-05 | Feature mode workflow (feature.lobster) | feature:phase-f4-delta-implementation |
| BC-AUDIT-1635 | pass-3-deep-workflows.md | 1207 | BC-5.30.056 | SS-05 | Feature mode workflow (feature.lobster) | feature:phase-f4-state-backup |
| BC-AUDIT-1636 | pass-3-deep-workflows.md | 1210 | BC-5.30.057 | SS-05 | Feature mode workflow (feature.lobster) | feature:phase-f4-gate |
| BC-AUDIT-1637 | pass-3-deep-workflows.md | 1213 | BC-5.30.058 | SS-05 | Feature mode workflow (feature.lobster) | feature:build-verification |
| BC-AUDIT-1638 | pass-3-deep-workflows.md | 1216 | BC-5.30.059 | SS-05 | Feature mode workflow (feature.lobster) | feature:build-gate |
| BC-AUDIT-1639 | pass-3-deep-workflows.md | 1219 | BC-5.30.060 | SS-05 | Feature mode workflow (feature.lobster) | feature:holdout-evaluation |
| BC-AUDIT-1640 | pass-3-deep-workflows.md | 1222 | BC-5.30.061 | SS-05 | Feature mode workflow (feature.lobster) | feature:holdout-gate |
| BC-AUDIT-1641 | pass-3-deep-workflows.md | 1225 | BC-5.30.062 | SS-05 | Feature mode workflow (feature.lobster) | feature:phase-f5-scoped-adversarial |
| BC-AUDIT-1642 | pass-3-deep-workflows.md | 1228 | BC-5.30.063 | SS-05 | Feature mode workflow (feature.lobster) | feature:phase-f5-state-backup |
| BC-AUDIT-1643 | pass-3-deep-workflows.md | 1231 | BC-5.30.064 | SS-05 | Feature mode workflow (feature.lobster) | feature:phase-f6-targeted-hardening |
| BC-AUDIT-1644 | pass-3-deep-workflows.md | 1234 | BC-5.30.065 | SS-05 | Feature mode workflow (feature.lobster) | feature:phase-f6-security-scan |
| BC-AUDIT-1645 | pass-3-deep-workflows.md | 1237 | BC-5.30.066 | SS-05 | Feature mode workflow (feature.lobster) | feature:phase-f6-dtu-adversarial |
| BC-AUDIT-1646 | pass-3-deep-workflows.md | 1240 | BC-5.30.067 | SS-05 | Feature mode workflow (feature.lobster) | feature:phase-f6-a11y-recheck |
| BC-AUDIT-1647 | pass-3-deep-workflows.md | 1243 | BC-5.30.068 | SS-05 | Feature mode workflow (feature.lobster) | feature:phase-f6-fix-delivery |
| BC-AUDIT-1648 | pass-3-deep-workflows.md | 1246 | BC-5.30.069 | SS-05 | Feature mode workflow (feature.lobster) | feature:phase-f6-gate |
| BC-AUDIT-1649 | pass-3-deep-workflows.md | 1249 | BC-5.30.070 | SS-05 | Feature mode workflow (feature.lobster) | feature:phase-f6-state-backup |
| BC-AUDIT-1650 | pass-3-deep-workflows.md | 1252 | BC-5.30.071 | SS-05 | Feature mode workflow (feature.lobster) | feature:phase-f6-demo-recording |
| BC-AUDIT-1651 | pass-3-deep-workflows.md | 1255 | BC-5.30.072 | SS-05 | Feature mode workflow (feature.lobster) | feature:phase-f6-visual-regression |
| BC-AUDIT-1652 | pass-3-deep-workflows.md | 1258 | BC-5.30.073 | SS-05 | Feature mode workflow (feature.lobster) | feature:phase-f7-heuristic-evaluation |
| BC-AUDIT-1653 | pass-3-deep-workflows.md | 1261 | BC-5.30.074 | SS-05 | Feature mode workflow (feature.lobster) | feature:phase-f7-ui-completeness-final |
| BC-AUDIT-1654 | pass-3-deep-workflows.md | 1264 | BC-5.30.075 | SS-05 | Feature mode workflow (feature.lobster) | feature:phase-f7-responsive-final |
| BC-AUDIT-1655 | pass-3-deep-workflows.md | 1267 | BC-5.30.076 | SS-05 | Feature mode workflow (feature.lobster) | feature:phase-f7-ui-quality-gate |
| BC-AUDIT-1656 | pass-3-deep-workflows.md | 1270 | BC-5.30.077 | SS-05 | Feature mode workflow (feature.lobster) | feature:phase-f7-ui-fix-delivery |
| BC-AUDIT-1657 | pass-3-deep-workflows.md | 1273 | BC-5.30.078 | SS-05 | Feature mode workflow (feature.lobster) | feature:phase-f7-delta-convergence |
| BC-AUDIT-1658 | pass-3-deep-workflows.md | 1276 | BC-5.30.079 | SS-05 | Feature mode workflow (feature.lobster) | feature:phase-f7-gate |
| BC-AUDIT-1659 | pass-3-deep-workflows.md | 1279 | BC-5.30.080 | SS-05 | Feature mode workflow (feature.lobster) | feature:phase-f7-state-backup |
| BC-AUDIT-1660 | pass-3-deep-workflows.md | 1282 | BC-5.30.081 | SS-05 | Feature mode workflow (feature.lobster) | feature:phase-f7-human-approval |
| BC-AUDIT-1661 | pass-3-deep-workflows.md | 1285 | BC-5.30.082 | SS-05 | Feature mode workflow (feature.lobster) | feature:release |
| BC-AUDIT-1662 | pass-3-deep-workflows.md | 1288 | BC-5.30.083 | SS-05 | Feature mode workflow (feature.lobster) | feature:feature-cycle-handoff |
| BC-AUDIT-1663 | pass-3-deep-workflows.md | 1291 | BC-5.30.084 | SS-05 | Feature mode workflow (feature.lobster) | feature:post-feature-validation |
| BC-AUDIT-1664 | pass-3-deep-workflows.md | 1294 | BC-5.30.085 | SS-05 | Feature mode workflow (feature.lobster) | feature:session-review |
| BC-AUDIT-1665 | pass-3-deep-workflows.md | 1297 | BC-5.30.086 | SS-05 | Feature mode workflow (feature.lobster) | feature:session-review-approval |
| BC-AUDIT-1666 | pass-3-deep-workflows.md | 1300 | BC-5.30.087 | SS-05 | Feature mode workflow (feature.lobster) | feature:process-review-decisions |
| BC-AUDIT-1670 | pass-3-deep-workflows.md | 1311 | BC-5.31.001 | SS-05 | Code Delivery workflow (code-delivery.lobster) | per-story-delivery: identity |
| BC-AUDIT-1671 | pass-3-deep-workflows.md | 1314 | BC-5.31.002 | SS-05 | Code Delivery workflow (code-delivery.lobster) | code-delivery: entry-point |
| BC-AUDIT-1672 | pass-3-deep-workflows.md | 1317 | BC-5.31.003 | SS-05 | Code Delivery workflow (code-delivery.lobster) | code-delivery: terminal-step |
| BC-AUDIT-1673 | pass-3-deep-workflows.md | 1320 | BC-5.31.004 | SS-05 | Code Delivery workflow (code-delivery.lobster) | code-delivery: DAG integrity |
| BC-AUDIT-1674 | pass-3-deep-workflows.md | 1323 | BC-5.31.005 | SS-05 | Code Delivery workflow (code-delivery.lobster) | code-delivery: failure semantics |
| BC-AUDIT-1675 | pass-3-deep-workflows.md | 1332 | BC-5.31.006 | SS-05 | Code Delivery workflow (code-delivery.lobster) | code-delivery:create-worktree |
| BC-AUDIT-1676 | pass-3-deep-workflows.md | 1335 | BC-5.31.007 | SS-05 | Code Delivery workflow (code-delivery.lobster) | code-delivery:generate-stubs |
| BC-AUDIT-1677 | pass-3-deep-workflows.md | 1339 | BC-5.31.008 | SS-05 | Code Delivery workflow (code-delivery.lobster) | code-delivery:write-tests |
| BC-AUDIT-1678 | pass-3-deep-workflows.md | 1343 | BC-5.31.009 | SS-05 | Code Delivery workflow (code-delivery.lobster) | code-delivery:red-gate |
| BC-AUDIT-1679 | pass-3-deep-workflows.md | 1347 | BC-5.31.010 | SS-05 | Code Delivery workflow (code-delivery.lobster) | code-delivery:implement |
| BC-AUDIT-1680 | pass-3-deep-workflows.md | 1351 | BC-5.31.011 | SS-05 | Code Delivery workflow (code-delivery.lobster) | code-delivery:per-story-adversarial-review |
| BC-AUDIT-1681 | pass-3-deep-workflows.md | 1355 | BC-5.31.012 | SS-05 | Code Delivery workflow (code-delivery.lobster) | code-delivery:e2e-tests |
| BC-AUDIT-1682 | pass-3-deep-workflows.md | 1358 | BC-5.31.013 | SS-05 | Code Delivery workflow (code-delivery.lobster) | code-delivery:storybook-story-generation |
| BC-AUDIT-1683 | pass-3-deep-workflows.md | 1361 | BC-5.31.014 | SS-05 | Code Delivery workflow (code-delivery.lobster) | code-delivery:storybook-component-tests |
| BC-AUDIT-1684 | pass-3-deep-workflows.md | 1365 | BC-5.31.015 | SS-05 | Code Delivery workflow (code-delivery.lobster) | code-delivery:per-story-ui-quality-gate |
| BC-AUDIT-1685 | pass-3-deep-workflows.md | 1369 | BC-5.31.016 | SS-05 | Code Delivery workflow (code-delivery.lobster) | code-delivery:demo-recording |
| BC-AUDIT-1686 | pass-3-deep-workflows.md | 1372 | BC-5.31.017 | SS-05 | Code Delivery workflow (code-delivery.lobster) | code-delivery:squash-and-push |
| BC-AUDIT-1687 | pass-3-deep-workflows.md | 1375 | BC-5.31.018 | SS-05 | Code Delivery workflow (code-delivery.lobster) | code-delivery:create-pr |
| BC-AUDIT-1688 | pass-3-deep-workflows.md | 1378 | BC-5.31.019 | SS-05 | Code Delivery workflow (code-delivery.lobster) | code-delivery:ai-pr-review |
| BC-AUDIT-1689 | pass-3-deep-workflows.md | 1382 | BC-5.31.020 | SS-05 | Code Delivery workflow (code-delivery.lobster) | code-delivery:security-review |
| BC-AUDIT-1690 | pass-3-deep-workflows.md | 1386 | BC-5.31.021 | SS-05 | Code Delivery workflow (code-delivery.lobster) | code-delivery:pr-review-convergence |
| BC-AUDIT-1691 | pass-3-deep-workflows.md | 1390 | BC-5.31.022 | SS-05 | Code Delivery workflow (code-delivery.lobster) | code-delivery:brownfield-full-regression |
| BC-AUDIT-1692 | pass-3-deep-workflows.md | 1394 | BC-5.31.023 | SS-05 | Code Delivery workflow (code-delivery.lobster) | code-delivery:brownfield-codeowners-check |
| BC-AUDIT-1693 | pass-3-deep-workflows.md | 1397 | BC-5.31.024 | SS-05 | Code Delivery workflow (code-delivery.lobster) | code-delivery:wait-for-ci |
| BC-AUDIT-1694 | pass-3-deep-workflows.md | 1400 | BC-5.31.025 | SS-05 | Code Delivery workflow (code-delivery.lobster) | code-delivery:dependency-merge-check |
| BC-AUDIT-1695 | pass-3-deep-workflows.md | 1403 | BC-5.31.026 | SS-05 | Code Delivery workflow (code-delivery.lobster) | code-delivery:merge-pr |
| BC-AUDIT-1696 | pass-3-deep-workflows.md | 1407 | BC-5.31.027 | SS-05 | Code Delivery workflow (code-delivery.lobster) | code-delivery:delivery-human-approval |
| BC-AUDIT-1697 | pass-3-deep-workflows.md | 1410 | BC-5.31.028 | SS-05 | Code Delivery workflow (code-delivery.lobster) | code-delivery:cleanup-worktree |
| BC-AUDIT-1700 | pass-3-deep-workflows.md | 1421 | BC-5.32.001 | SS-05 | Discovery mode workflow (discovery.lobster) | autonomous-discovery: identity |
| BC-AUDIT-1701 | pass-3-deep-workflows.md | 1425 | BC-5.32.002 | SS-05 | Discovery mode workflow (discovery.lobster) | discovery: entry-point |
| BC-AUDIT-1702 | pass-3-deep-workflows.md | 1428 | BC-5.32.003 | SS-05 | Discovery mode workflow (discovery.lobster) | discovery: terminal-step |
| BC-AUDIT-1703 | pass-3-deep-workflows.md | 1431 | BC-5.32.004 | SS-05 | Discovery mode workflow (discovery.lobster) | discovery: DAG integrity |
| BC-AUDIT-1704 | pass-3-deep-workflows.md | 1434 | BC-5.32.005 | SS-05 | Discovery mode workflow (discovery.lobster) | discovery: failure semantics |
| BC-AUDIT-1705 | pass-3-deep-workflows.md | 1439 | BC-5.32.006 | SS-05 | Discovery mode workflow (discovery.lobster) | discovery:load-discovery-config |
| BC-AUDIT-1706 | pass-3-deep-workflows.md | 1442 | BC-5.32.007 | SS-05 | Discovery mode workflow (discovery.lobster) | discovery:state-init |
| BC-AUDIT-1707 | pass-3-deep-workflows.md | 1445 | BC-5.32.008 | SS-05 | Discovery mode workflow (discovery.lobster) | discovery:feature-research |
| BC-AUDIT-1708 | pass-3-deep-workflows.md | 1448 | BC-5.32.009 | SS-05 | Discovery mode workflow (discovery.lobster) | discovery:state-backup-feature-research |
| BC-AUDIT-1709 | pass-3-deep-workflows.md | 1451 | BC-5.32.010 | SS-05 | Discovery mode workflow (discovery.lobster) | discovery:customer-feedback-ingestion |
| BC-AUDIT-1710 | pass-3-deep-workflows.md | 1454 | BC-5.32.011 | SS-05 | Discovery mode workflow (discovery.lobster) | discovery:competitive-monitoring |
| BC-AUDIT-1711 | pass-3-deep-workflows.md | 1457 | BC-5.32.012 | SS-05 | Discovery mode workflow (discovery.lobster) | discovery:usage-analytics |
| BC-AUDIT-1712 | pass-3-deep-workflows.md | 1460 | BC-5.32.013 | SS-05 | Discovery mode workflow (discovery.lobster) | discovery:state-backup-ingestion |
| BC-AUDIT-1713 | pass-3-deep-workflows.md | 1463 | BC-5.32.014 | SS-05 | Discovery mode workflow (discovery.lobster) | discovery:intelligence-synthesis |
| BC-AUDIT-1714 | pass-3-deep-workflows.md | 1466 | BC-5.32.015 | SS-05 | Discovery mode workflow (discovery.lobster) | discovery:state-backup-synthesis |
| BC-AUDIT-1715 | pass-3-deep-workflows.md | 1469 | BC-5.32.016 | SS-05 | Discovery mode workflow (discovery.lobster) | discovery:feature-scoring-value |
| BC-AUDIT-1716 | pass-3-deep-workflows.md | 1473 | BC-5.32.017 | SS-05 | Discovery mode workflow (discovery.lobster) | discovery:feature-scoring-feasibility |
| BC-AUDIT-1717 | pass-3-deep-workflows.md | 1477 | BC-5.32.018 | SS-05 | Discovery mode workflow (discovery.lobster) | discovery:feature-scoring-novelty |
| BC-AUDIT-1718 | pass-3-deep-workflows.md | 1481 | BC-5.32.019 | SS-05 | Discovery mode workflow (discovery.lobster) | discovery:feature-debate |
| BC-AUDIT-1719 | pass-3-deep-workflows.md | 1485 | BC-5.32.020 | SS-05 | Discovery mode workflow (discovery.lobster) | discovery:product-research |
| BC-AUDIT-1720 | pass-3-deep-workflows.md | 1488 | BC-5.32.021 | SS-05 | Discovery mode workflow (discovery.lobster) | discovery:product-scoring |
| BC-AUDIT-1721 | pass-3-deep-workflows.md | 1491 | BC-5.32.022 | SS-05 | Discovery mode workflow (discovery.lobster) | discovery:deduplication |
| BC-AUDIT-1722 | pass-3-deep-workflows.md | 1495 | BC-5.32.023 | SS-05 | Discovery mode workflow (discovery.lobster) | discovery:state-backup-scoring |
| BC-AUDIT-1723 | pass-3-deep-workflows.md | 1498 | BC-5.32.024 | SS-05 | Discovery mode workflow (discovery.lobster) | discovery:generate-report |
| BC-AUDIT-1724 | pass-3-deep-workflows.md | 1501 | BC-5.32.025 | SS-05 | Discovery mode workflow (discovery.lobster) | discovery:state-backup-report |
| BC-AUDIT-1725 | pass-3-deep-workflows.md | 1504 | BC-5.32.026 | SS-05 | Discovery mode workflow (discovery.lobster) | discovery:discovery-notifications |
| BC-AUDIT-1726 | pass-3-deep-workflows.md | 1507 | BC-5.32.027 | SS-05 | Discovery mode workflow (discovery.lobster) | discovery:discovery-review |
| BC-AUDIT-1727 | pass-3-deep-workflows.md | 1510 | BC-5.32.028 | SS-05 | Discovery mode workflow (discovery.lobster) | discovery:route-approved-ideas |
| BC-AUDIT-1728 | pass-3-deep-workflows.md | 1514 | BC-5.32.029 | SS-05 | Discovery mode workflow (discovery.lobster) | discovery:state-final |
| BC-AUDIT-1729 | pass-3-deep-workflows.md | 1517 | BC-5.32.030 | SS-05 | Discovery mode workflow (discovery.lobster) | discovery:execute-product-ideas |
| BC-AUDIT-1730 | pass-3-deep-workflows.md | 1520 | BC-5.32.031 | SS-05 | Discovery mode workflow (discovery.lobster) | discovery:execute-feature-ideas |
| BC-AUDIT-1731 | pass-3-deep-workflows.md | 1523 | BC-5.32.032 | SS-05 | Discovery mode workflow (discovery.lobster) | discovery:session-review |
| BC-AUDIT-1732 | pass-3-deep-workflows.md | 1526 | BC-5.32.033 | SS-05 | Discovery mode workflow (discovery.lobster) | discovery:session-review-approval |
| BC-AUDIT-1733 | pass-3-deep-workflows.md | 1529 | BC-5.32.034 | SS-05 | Discovery mode workflow (discovery.lobster) | discovery:process-review-decisions |
| BC-AUDIT-1740 | pass-3-deep-workflows.md | 1540 | BC-5.33.001 | SS-05 | Maintenance mode workflow (maintenance.lobster) | maintenance-sweep: identity |
| BC-AUDIT-1741 | pass-3-deep-workflows.md | 1543 | BC-5.33.002 | SS-05 | Maintenance mode workflow (maintenance.lobster) | maintenance: entry-point |
| BC-AUDIT-1742 | pass-3-deep-workflows.md | 1546 | BC-5.33.003 | SS-05 | Maintenance mode workflow (maintenance.lobster) | maintenance: terminal-step |
| BC-AUDIT-1743 | pass-3-deep-workflows.md | 1549 | BC-5.33.004 | SS-05 | Maintenance mode workflow (maintenance.lobster) | maintenance: DAG integrity |
| BC-AUDIT-1744 | pass-3-deep-workflows.md | 1552 | BC-5.33.005 | SS-05 | Maintenance mode workflow (maintenance.lobster) | maintenance: failure semantics |
| BC-AUDIT-1745 | pass-3-deep-workflows.md | 1557 | BC-5.33.006 | SS-05 | Maintenance mode workflow (maintenance.lobster) | maintenance:load-config |
| BC-AUDIT-1746 | pass-3-deep-workflows.md | 1560 | BC-5.33.007 | SS-05 | Maintenance mode workflow (maintenance.lobster) | maintenance:state-init |
| BC-AUDIT-1747 | pass-3-deep-workflows.md | 1563 | BC-5.33.008 | SS-05 | Maintenance mode workflow (maintenance.lobster) | maintenance:dependency-audit-scan |
| BC-AUDIT-1748 | pass-3-deep-workflows.md | 1566 | BC-5.33.009 | SS-05 | Maintenance mode workflow (maintenance.lobster) | maintenance:dependency-audit-analysis |
| BC-AUDIT-1749 | pass-3-deep-workflows.md | 1569 | BC-5.33.010 | SS-05 | Maintenance mode workflow (maintenance.lobster) | maintenance:state-backup-sweep-1 |
| BC-AUDIT-1750 | pass-3-deep-workflows.md | 1572 | BC-5.33.011 | SS-05 | Maintenance mode workflow (maintenance.lobster) | maintenance:doc-drift-scan |
| BC-AUDIT-1751 | pass-3-deep-workflows.md | 1575 | BC-5.33.012 | SS-05 | Maintenance mode workflow (maintenance.lobster) | maintenance:state-backup-sweep-2 |
| BC-AUDIT-1752 | pass-3-deep-workflows.md | 1578 | BC-5.33.013 | SS-05 | Maintenance mode workflow (maintenance.lobster) | maintenance:pattern-consistency-scan |
| BC-AUDIT-1753 | pass-3-deep-workflows.md | 1581 | BC-5.33.014 | SS-05 | Maintenance mode workflow (maintenance.lobster) | maintenance:state-backup-sweep-3 |
| BC-AUDIT-1754 | pass-3-deep-workflows.md | 1584 | BC-5.33.015 | SS-05 | Maintenance mode workflow (maintenance.lobster) | maintenance:holdout-freshness-check |
| BC-AUDIT-1755 | pass-3-deep-workflows.md | 1587 | BC-5.33.016 | SS-05 | Maintenance mode workflow (maintenance.lobster) | maintenance:state-backup-sweep-4 |
| BC-AUDIT-1756 | pass-3-deep-workflows.md | 1590 | BC-5.33.017 | SS-05 | Maintenance mode workflow (maintenance.lobster) | maintenance:performance-regression-scan |
| BC-AUDIT-1757 | pass-3-deep-workflows.md | 1593 | BC-5.33.018 | SS-05 | Maintenance mode workflow (maintenance.lobster) | maintenance:state-backup-sweep-5 |
| BC-AUDIT-1758 | pass-3-deep-workflows.md | 1596 | BC-5.33.019 | SS-05 | Maintenance mode workflow (maintenance.lobster) | maintenance:dtu-fidelity-drift |
| BC-AUDIT-1759 | pass-3-deep-workflows.md | 1599 | BC-5.33.020 | SS-05 | Maintenance mode workflow (maintenance.lobster) | maintenance:state-backup-sweep-6 |
| BC-AUDIT-1760 | pass-3-deep-workflows.md | 1602 | BC-5.33.021 | SS-05 | Maintenance mode workflow (maintenance.lobster) | maintenance:spec-coherence |
| BC-AUDIT-1761 | pass-3-deep-workflows.md | 1606 | BC-5.33.022 | SS-05 | Maintenance mode workflow (maintenance.lobster) | maintenance:state-backup-sweep-7 |
| BC-AUDIT-1762 | pass-3-deep-workflows.md | 1609 | BC-5.33.023 | SS-05 | Maintenance mode workflow (maintenance.lobster) | maintenance:tech-debt-register |
| BC-AUDIT-1763 | pass-3-deep-workflows.md | 1612 | BC-5.33.024 | SS-05 | Maintenance mode workflow (maintenance.lobster) | maintenance:state-backup-sweep-8 |
| BC-AUDIT-1764 | pass-3-deep-workflows.md | 1615 | BC-5.33.025 | SS-05 | Maintenance mode workflow (maintenance.lobster) | maintenance:accessibility-regression |
| BC-AUDIT-1765 | pass-3-deep-workflows.md | 1619 | BC-5.33.026 | SS-05 | Maintenance mode workflow (maintenance.lobster) | maintenance:state-backup-sweep-9 |
| BC-AUDIT-1766 | pass-3-deep-workflows.md | 1622 | BC-5.33.027 | SS-05 | Maintenance mode workflow (maintenance.lobster) | maintenance:design-drift-scan |
| BC-AUDIT-1767 | pass-3-deep-workflows.md | 1625 | BC-5.33.028 | SS-05 | Maintenance mode workflow (maintenance.lobster) | maintenance:state-backup-sweep-10 |
| BC-AUDIT-1768 | pass-3-deep-workflows.md | 1628 | BC-5.33.029 | SS-05 | Maintenance mode workflow (maintenance.lobster) | maintenance:risk-assumption-monitoring |
| BC-AUDIT-1769 | pass-3-deep-workflows.md | 1631 | BC-5.33.030 | SS-05 | Maintenance mode workflow (maintenance.lobster) | maintenance:state-backup-sweep-11 |
| BC-AUDIT-1770 | pass-3-deep-workflows.md | 1634 | BC-5.33.031 | SS-05 | Maintenance mode workflow (maintenance.lobster) | maintenance:maintenance-report |
| BC-AUDIT-1771 | pass-3-deep-workflows.md | 1637 | BC-5.33.032 | SS-05 | Maintenance mode workflow (maintenance.lobster) | maintenance:fix-pr-delivery |
| BC-AUDIT-1772 | pass-3-deep-workflows.md | 1641 | BC-5.33.033 | SS-05 | Maintenance mode workflow (maintenance.lobster) | maintenance:maintenance-demo-recording |
| BC-AUDIT-1773 | pass-3-deep-workflows.md | 1644 | BC-5.33.034 | SS-05 | Maintenance mode workflow (maintenance.lobster) | maintenance:notifications |
| BC-AUDIT-1774 | pass-3-deep-workflows.md | 1648 | BC-5.33.035 | SS-05 | Maintenance mode workflow (maintenance.lobster) | maintenance:state-final |
| BC-AUDIT-1775 | pass-3-deep-workflows.md | 1651 | BC-5.33.036 | SS-05 | Maintenance mode workflow (maintenance.lobster) | maintenance:maintenance-gate |
| BC-AUDIT-1776 | pass-3-deep-workflows.md | 1654 | BC-5.33.037 | SS-05 | Maintenance mode workflow (maintenance.lobster) | maintenance:session-review |
| BC-AUDIT-1777 | pass-3-deep-workflows.md | 1657 | BC-5.33.038 | SS-05 | Maintenance mode workflow (maintenance.lobster) | maintenance:session-review-approval |
| BC-AUDIT-1778 | pass-3-deep-workflows.md | 1660 | BC-5.33.039 | SS-05 | Maintenance mode workflow (maintenance.lobster) | maintenance:process-review-decisions |
| BC-AUDIT-1790 | pass-3-deep-workflows.md | 1671 | BC-5.34.001 | SS-05 | Multi-Repo mode workflow (multi-repo.lobster) | multi-repo-vsdd: identity |
| BC-AUDIT-1791 | pass-3-deep-workflows.md | 1674 | BC-5.34.002 | SS-05 | Multi-Repo mode workflow (multi-repo.lobster) | multi-repo: entry-point |
| BC-AUDIT-1792 | pass-3-deep-workflows.md | 1677 | BC-5.34.003 | SS-05 | Multi-Repo mode workflow (multi-repo.lobster) | multi-repo: terminal-step (primary track) |
| BC-AUDIT-1793 | pass-3-deep-workflows.md | 1680 | BC-5.34.004 | SS-05 | Multi-Repo mode workflow (multi-repo.lobster) | multi-repo: DAG integrity (primary track) |
| BC-AUDIT-1794 | pass-3-deep-workflows.md | 1683 | BC-5.34.005 | SS-05 | Multi-Repo mode workflow (multi-repo.lobster) | multi-repo: failure semantics |
| BC-AUDIT-1795 | pass-3-deep-workflows.md | 1686 | BC-5.34.006 | SS-05 | Multi-Repo mode workflow (multi-repo.lobster) | multi-repo: cross-repo information asymmetry walls |
| BC-AUDIT-1796 | pass-3-deep-workflows.md | 1696 | BC-5.34.007 | SS-05 | Multi-Repo mode workflow (multi-repo.lobster) | multi-repo:environment-setup |
| BC-AUDIT-1797 | pass-3-deep-workflows.md | 1699 | BC-5.34.008 | SS-05 | Multi-Repo mode workflow (multi-repo.lobster) | multi-repo:read-project-manifest |
| BC-AUDIT-1798 | pass-3-deep-workflows.md | 1702 | BC-5.34.009 | SS-05 | Multi-Repo mode workflow (multi-repo.lobster) | multi-repo:compute-repo-waves |
| BC-AUDIT-1790-PLN | pass-3-deep-workflows.md | 1763 | BC-5.35.001 | SS-05 | Planning mode workflow (planning.lobster) | adaptive-planning: identity |
| BC-AUDIT-1791-PLN | pass-3-deep-workflows.md | 1766 | BC-5.35.002 | SS-05 | Planning mode workflow (planning.lobster) | planning: entry-point |
| BC-AUDIT-1792-PLN | pass-3-deep-workflows.md | 1769 | BC-5.35.003 | SS-05 | Planning mode workflow (planning.lobster) | planning: terminal-step |
| BC-AUDIT-1793-PLN | pass-3-deep-workflows.md | 1772 | BC-5.35.004 | SS-05 | Planning mode workflow (planning.lobster) | planning: DAG integrity |
| BC-AUDIT-1794-PLN | pass-3-deep-workflows.md | 1775 | BC-5.35.005 | SS-05 | Planning mode workflow (planning.lobster) | planning: failure semantics |
| BC-AUDIT-070 | pass-3-behavioral-contracts.md | 466 | BC-6.01.001 | SS-06 | Skill quality-gate contracts (broad-sweep) | brownfield-ingest enforces strict-binary novelty |
| BC-AUDIT-071 | pass-3-behavioral-contracts.md | 472 | BC-6.01.002 | SS-06 | Skill quality-gate contracts (broad-sweep) | brownfield-ingest "Iron Law" — no round completion without honest convergence check |
| BC-AUDIT-072 | pass-3-behavioral-contracts.md | 478 | BC-6.01.003 | SS-06 | Skill quality-gate contracts (broad-sweep) | activate skill requires platform detection success |
| BC-AUDIT-073 | pass-3-behavioral-contracts.md | 484 | BC-6.01.004 | SS-06 | Skill quality-gate contracts (broad-sweep) | activate skill copies hooks.json.<platform> to hooks.json then verifies dispatcher binary |
| BC-AUDIT-074 | pass-3-behavioral-contracts.md | 490 | BC-6.01.005 | SS-06 | Skill quality-gate contracts (broad-sweep) | activate skill writes platform + plugin version + activated_at to .claude/settings.local.json |
| BC-AUDIT-075 | pass-3-behavioral-contracts.md | 496 | BC-6.01.006 | SS-06 | Skill quality-gate contracts (broad-sweep) | activate drift warns on cross-host re-activation |
| BC-AUDIT-087 | pass-3-behavioral-contracts-deep-r1.md | 104 | BC-6.02.001 | SS-06 | Skill class meta-contracts (frontmatter, invocation, output) | SKILL.md frontmatter requires `name` and `description`; both are non-empty strings |
| BC-AUDIT-088 | pass-3-behavioral-contracts-deep-r1.md | 111 | BC-6.02.002 | SS-06 | Skill class meta-contracts (frontmatter, invocation, output) | SKILL.md description supports YAML block scalar (`>` folded) for multi-line text |
| BC-AUDIT-089 | pass-3-behavioral-contracts-deep-r1.md | 118 | BC-6.02.003 | SS-06 | Skill class meta-contracts (frontmatter, invocation, output) | Skill invocation surface is `/vsdd-factory:<skill-name>` slash command |
| BC-AUDIT-090 | pass-3-behavioral-contracts-deep-r1.md | 125 | BC-6.02.004 | SS-06 | Skill class meta-contracts (frontmatter, invocation, output) | Skills with `disable-model-invocation: true` are dispatcher-only — model cannot self-invoke |
| BC-AUDIT-091 | pass-3-behavioral-contracts-deep-r1.md | 132 | BC-6.02.005 | SS-06 | Skill class meta-contracts (frontmatter, invocation, output) | Skills with `allowed-tools:` whitelist restrict tool surface inside the skill body |
| BC-AUDIT-092 | pass-3-behavioral-contracts-deep-r1.md | 139 | BC-6.02.006 | SS-06 | Skill class meta-contracts (frontmatter, invocation, output) | "Announce at Start" protocol — verbatim opening line per skill |
| BC-AUDIT-093 | pass-3-behavioral-contracts-deep-r1.md | 146 | BC-6.02.007 | SS-06 | Skill class meta-contracts (frontmatter, invocation, output) | Skills SHALL link to template files via `${CLAUDE_PLUGIN_ROOT}/templates/...` references |
| BC-AUDIT-094 | pass-3-behavioral-contracts-deep-r1.md | 153 | BC-6.02.008 | SS-06 | Skill class meta-contracts (frontmatter, invocation, output) | Skill quality gates expressed as a "Hard Gate" or "Iron Law" prose section |
| BC-AUDIT-095 | pass-3-behavioral-contracts-deep-r1.md | 160 | BC-6.02.009 | SS-06 | Skill class meta-contracts (frontmatter, invocation, output) | Skill bodies MAY include a "Red Flags" table to enumerate fabrication / shortcut anti-patterns |
| BC-AUDIT-096 | pass-3-behavioral-contracts-deep-r1.md | 167 | BC-6.02.010 | SS-06 | Skill class meta-contracts (frontmatter, invocation, output) | Skills that dispatch sub-agents declare a "Canonical Source" or single-source-of-truth playbook r... |
| BC-AUDIT-097 | pass-3-behavioral-contracts-deep-r1.md | 174 | BC-6.02.011 | SS-06 | Skill class meta-contracts (frontmatter, invocation, output) | Skills with `argument-hint:` declare inline `$ARGUMENTS[N]` / `$ARGUMENTS` semantics |
| BC-AUDIT-098 | pass-3-behavioral-contracts-deep-r1.md | 181 | BC-6.02.012 | SS-06 | Skill class meta-contracts (frontmatter, invocation, output) | Skill output paths follow `${CLAUDE_PLUGIN_ROOT}` / `.factory/` placement convention |
| BC-AUDIT-200 | pass-3-deep-skills-batch-1.md | 65 | BC-6.03.001 | SS-06 | Activation and deactivation skills | activate: skill identity contract |
| BC-AUDIT-201 | pass-3-deep-skills-batch-1.md | 74 | BC-6.03.002 | SS-06 | Activation and deactivation skills | activate: aborts on unsupported platform |
| BC-AUDIT-202 | pass-3-deep-skills-batch-1.md | 83 | BC-6.03.003 | SS-06 | Activation and deactivation skills | activate: drift warning on host change |
| BC-AUDIT-203 | pass-3-deep-skills-batch-1.md | 92 | BC-6.03.004 | SS-06 | Activation and deactivation skills | activate: writes activation block with three named fields |
| BC-AUDIT-204 | pass-3-deep-skills-batch-1.md | 101 | BC-6.03.005 | SS-06 | Activation and deactivation skills | activate: dry-run mode performs no writes |
| BC-AUDIT-205 | pass-3-deep-skills-batch-1.md | 110 | BC-6.03.006 | SS-06 | Activation and deactivation skills | activate: applies per-platform variant via apply-platform.sh |
| BC-AUDIT-305 | pass-3-deep-skills-batch-1.md | 1106 | BC-6.03.007 | SS-06 | Activation and deactivation skills | deactivate: skill identity (inverse of activate) |
| BC-AUDIT-306 | pass-3-deep-skills-batch-1.md | 1115 | BC-6.03.008 | SS-06 | Activation and deactivation skills | deactivate: sanity-check before clobbering |
| BC-AUDIT-307 | pass-3-deep-skills-batch-1.md | 1124 | BC-6.03.009 | SS-06 | Activation and deactivation skills | deactivate: empty-file disposition asks user |
| BC-AUDIT-206 | pass-3-deep-skills-batch-1.md | 123 | BC-6.04.001 | SS-06 | Adversarial and review skills | adversarial-review: skill identity contract |
| BC-AUDIT-207 | pass-3-deep-skills-batch-1.md | 132 | BC-6.04.002 | SS-06 | Adversarial and review skills | adversarial-review: announces verbatim before any other action |
| BC-AUDIT-208 | pass-3-deep-skills-batch-1.md | 141 | BC-6.04.003 | SS-06 | Adversarial and review skills | adversarial-review: minimum 3 consecutive clean passes for convergence |
| BC-AUDIT-209 | pass-3-deep-skills-batch-1.md | 150 | BC-6.04.004 | SS-06 | Adversarial and review skills | adversarial-review: filename collision guard pre-flight |
| BC-AUDIT-210 | pass-3-deep-skills-batch-1.md | 159 | BC-6.04.005 | SS-06 | Adversarial and review skills | adversarial-review: policy rubric auto-loading from policies.yaml |
| BC-AUDIT-211 | pass-3-deep-skills-batch-1.md | 168 | BC-6.04.006 | SS-06 | Adversarial and review skills | adversarial-review: post-adversary persistence via state-manager |
| BC-AUDIT-212 | pass-3-deep-skills-batch-1.md | 177 | BC-6.04.007 | SS-06 | Adversarial and review skills | adversarial-review: trajectory monotonicity (findings never increase) |
| BC-AUDIT-213 | pass-3-deep-skills-batch-1.md | 190 | BC-6.04.008 | SS-06 | Adversarial and review skills | agent-file-review: skill identity contract |
| BC-AUDIT-214 | pass-3-deep-skills-batch-1.md | 199 | BC-6.04.009 | SS-06 | Adversarial and review skills | agent-file-review: token budget thresholds (PASS/WARN/FAIL) |
| BC-AUDIT-215 | pass-3-deep-skills-batch-1.md | 208 | BC-6.04.010 | SS-06 | Adversarial and review skills | agent-file-review: 15-check list runs all checks |
| BC-AUDIT-216 | pass-3-deep-skills-batch-1.md | 217 | BC-6.04.011 | SS-06 | Adversarial and review skills | agent-file-review: tool profile match against openclaw.json (FAIL on mismatch) |
| BC-AUDIT-217 | pass-3-deep-skills-batch-1.md | 226 | BC-6.04.012 | SS-06 | Adversarial and review skills | agent-file-review: batch summary mode |
| BC-AUDIT-251 | pass-3-deep-skills-batch-1.md | 564 | BC-6.04.013 | SS-06 | Adversarial and review skills | code-delivery: skill identity (post-convergence delivery) |
| BC-AUDIT-252 | pass-3-deep-skills-batch-1.md | 573 | BC-6.04.014 | SS-06 | Adversarial and review skills | code-delivery: pre-push test gate via before_tool_call hook |
| BC-AUDIT-253 | pass-3-deep-skills-batch-1.md | 582 | BC-6.04.015 | SS-06 | Adversarial and review skills | code-delivery: per-AC demo evidence with both success and error paths |
| BC-AUDIT-254 | pass-3-deep-skills-batch-1.md | 591 | BC-6.04.016 | SS-06 | Adversarial and review skills | code-delivery: 4-model-family review (4th model in pr-reviewer) |
| BC-AUDIT-255 | pass-3-deep-skills-batch-1.md | 600 | BC-6.04.017 | SS-06 | Adversarial and review skills | code-delivery: review convergence loop (max 10 cycles) |
| BC-AUDIT-256 | pass-3-deep-skills-batch-1.md | 609 | BC-6.04.018 | SS-06 | Adversarial and review skills | code-delivery: autonomy-level-driven merge decision |
| BC-AUDIT-404 | pass-3-deep-skills-batch-2.md | 91 | BC-6.04.019 | SS-06 | Adversarial and review skills | fix-pr-delivery: Identity — streamlined fix PR flow with same rigor minus stubs/Red Gate/wave g... |
| BC-AUDIT-405 | pass-3-deep-skills-batch-2.md | 100 | BC-6.04.020 | SS-06 | Adversarial and review skills | fix-pr-delivery: Branch and PR title naming uses `fix/FIX-P[phase]-NNN` and `fix(FIX-P[phase]-NNN... |
| BC-AUDIT-406 | pass-3-deep-skills-batch-2.md | 109 | BC-6.04.021 | SS-06 | Adversarial and review skills | fix-pr-delivery: Demo recording is conditional on behavior-changing fixes |
| BC-AUDIT-407 | pass-3-deep-skills-batch-2.md | 118 | BC-6.04.022 | SS-06 | Adversarial and review skills | fix-pr-delivery: Max 10 review cycles before convergence or exhaustion |
| BC-AUDIT-408 | pass-3-deep-skills-batch-2.md | 127 | BC-6.04.023 | SS-06 | Adversarial and review skills | fix-pr-delivery: Hardening fixes re-run only failing checks, not all checks |
| BC-AUDIT-409 | pass-3-deep-skills-batch-2.md | 136 | BC-6.04.024 | SS-06 | Adversarial and review skills | fix-pr-delivery: Output is fix PR merged to develop with worktree cleaned up |
| BC-AUDIT-426 | pass-3-deep-skills-batch-2.md | 313 | BC-6.04.025 | SS-06 | Adversarial and review skills | holdout-eval: Identity — runs holdout evaluation with strict information asymmetry, returns sat... |
| BC-AUDIT-427 | pass-3-deep-skills-batch-2.md | 322 | BC-6.04.026 | SS-06 | Adversarial and review skills | holdout-eval: Iron Law — evaluator MUST NOT see specs, source, BCs, architecture, or prior reviews |
| BC-AUDIT-428 | pass-3-deep-skills-batch-2.md | 331 | BC-6.04.027 | SS-06 | Adversarial and review skills | holdout-eval: Gate is mean satisfaction ≥ 0.85 AND every critical scenario ≥ 0.60 |
| BC-AUDIT-429 | pass-3-deep-skills-batch-2.md | 340 | BC-6.04.028 | SS-06 | Adversarial and review skills | holdout-eval: Output written to .factory/holdout-scenarios/evaluations/wave-<N>/ |
| BC-AUDIT-516 | pass-3-deep-skills-batch-2.md | 1246 | BC-6.04.029 | SS-06 | Adversarial and review skills | phase-4-holdout-evaluation: Identity — Phase 4 entry point with scenario rotation + holdout-eva... |
| BC-AUDIT-517 | pass-3-deep-skills-batch-2.md | 1255 | BC-6.04.030 | SS-06 | Adversarial and review skills | phase-4-holdout-evaluation: Gate — adversary tier (GPT-5.4 not Claude), mean ≥0.85, no must-p... |
| BC-AUDIT-518 | pass-3-deep-skills-batch-2.md | 1264 | BC-6.04.031 | SS-06 | Adversarial and review skills | phase-4-holdout-evaluation: Direct command is /vsdd-factory:holdout-eval |
| BC-AUDIT-679 | pass-3-deep-skills-batch-3.md | 801 | BC-6.04.032 | SS-06 | Adversarial and review skills | session-review: identity, trigger, primary agent |
| BC-AUDIT-680 | pass-3-deep-skills-batch-3.md | 810 | BC-6.04.033 | SS-06 | Adversarial and review skills | session-review: 8 analysis dimensions |
| BC-AUDIT-681 | pass-3-deep-skills-batch-3.md | 819 | BC-6.04.034 | SS-06 | Adversarial and review skills | session-review: 10 proposal categories with routing |
| BC-AUDIT-682 | pass-3-deep-skills-batch-3.md | 828 | BC-6.04.035 | SS-06 | Adversarial and review skills | session-review: 72h non-blocking timeout |
| BC-AUDIT-683 | pass-3-deep-skills-batch-3.md | 837 | BC-6.04.036 | SS-06 | Adversarial and review skills | session-review: cross-run pattern database + benchmarks |
| BC-AUDIT-684 | pass-3-deep-skills-batch-3.md | 846 | BC-6.04.037 | SS-06 | Adversarial and review skills | session-review: failure-mode safety (incomplete logs / missing cost / corrupt pattern db) |
| BC-AUDIT-231 | pass-3-deep-skills-batch-1.md | 368 | BC-6.05.001 | SS-06 | Brownfield, discovery, research skills | brownfield-ingest: skill identity (broad-then-converge protocol) |
| BC-AUDIT-232 | pass-3-deep-skills-batch-1.md | 377 | BC-6.05.002 | SS-06 | Brownfield, discovery, research skills | brownfield-ingest: announces verbatim and creates phase A/B/B5/B6/C TodoWrite entries |
| BC-AUDIT-233 | pass-3-deep-skills-batch-1.md | 386 | BC-6.05.003 | SS-06 | Brownfield, discovery, research skills | brownfield-ingest: source acquisition into .reference/ |
| BC-AUDIT-234 | pass-3-deep-skills-batch-1.md | 395 | BC-6.05.004 | SS-06 | Brownfield, discovery, research skills | brownfield-ingest: strict-binary novelty (NITPICK token only) |
| BC-AUDIT-235 | pass-3-deep-skills-batch-1.md | 404 | BC-6.05.005 | SS-06 | Brownfield, discovery, research skills | brownfield-ingest: minimum 2 deepening rounds, no fixed maximum |
| BC-AUDIT-236 | pass-3-deep-skills-batch-1.md | 413 | BC-6.05.006 | SS-06 | Brownfield, discovery, research skills | brownfield-ingest: honest convergence clause in every round prompt |
| BC-AUDIT-237 | pass-3-deep-skills-batch-1.md | 422 | BC-6.05.007 | SS-06 | Brownfield, discovery, research skills | brownfield-ingest: Phase B.5 coverage audit is mandatory |
| BC-AUDIT-238 | pass-3-deep-skills-batch-1.md | 431 | BC-6.05.008 | SS-06 | Brownfield, discovery, research skills | brownfield-ingest: Phase B.6 extraction validation with behavioral/metric split |
| BC-AUDIT-261 | pass-3-deep-skills-batch-1.md | 662 | BC-6.05.009 | SS-06 | Brownfield, discovery, research skills | competitive-monitoring: skill identity contract |
| BC-AUDIT-262 | pass-3-deep-skills-batch-1.md | 671 | BC-6.05.010 | SS-06 | Brownfield, discovery, research skills | competitive-monitoring: urgency classification HIGH/MEDIUM/LOW |
| BC-AUDIT-263 | pass-3-deep-skills-batch-1.md | 680 | BC-6.05.011 | SS-06 | Brownfield, discovery, research skills | competitive-monitoring: VERIFIED/UNVERIFIED flagging on findings |
| BC-AUDIT-302 | pass-3-deep-skills-batch-1.md | 1075 | BC-6.05.012 | SS-06 | Brownfield, discovery, research skills | customer-feedback-ingestion: skill identity (read-only, no customer interaction) |
| BC-AUDIT-303 | pass-3-deep-skills-batch-1.md | 1084 | BC-6.05.013 | SS-06 | Brownfield, discovery, research skills | customer-feedback-ingestion: 5 categorization buckets with priority |
| BC-AUDIT-304 | pass-3-deep-skills-batch-1.md | 1093 | BC-6.05.014 | SS-06 | Brownfield, discovery, research skills | customer-feedback-ingestion: deduplication via 0.80 semantic similarity threshold |
| BC-AUDIT-327 | pass-3-deep-skills-batch-1.md | 1328 | BC-6.05.015 | SS-06 | Brownfield, discovery, research skills | discovery-engine: skill identity (autonomous opportunity research) |
| BC-AUDIT-328 | pass-3-deep-skills-batch-1.md | 1337 | BC-6.05.016 | SS-06 | Brownfield, discovery, research skills | discovery-engine: 2 modes (Feature Discovery vs Product Discovery) |
| BC-AUDIT-329 | pass-3-deep-skills-batch-1.md | 1346 | BC-6.05.017 | SS-06 | Brownfield, discovery, research skills | discovery-engine: 7-dimension scoring with weights summing to 1.00 |
| BC-AUDIT-330 | pass-3-deep-skills-batch-1.md | 1355 | BC-6.05.018 | SS-06 | Brownfield, discovery, research skills | discovery-engine: routing thresholds (auto-brief vs backlog vs registry vs urgent) |
| BC-AUDIT-331 | pass-3-deep-skills-batch-1.md | 1368 | BC-6.05.019 | SS-06 | Brownfield, discovery, research skills | disposition-pass: skill identity (Pass 9, vision-lens re-examination) |
| BC-AUDIT-332 | pass-3-deep-skills-batch-1.md | 1377 | BC-6.05.020 | SS-06 | Brownfield, discovery, research skills | disposition-pass: 4-bucket mandatory categorization |
| BC-AUDIT-333 | pass-3-deep-skills-batch-1.md | 1386 | BC-6.05.021 | SS-06 | Brownfield, discovery, research skills | disposition-pass: every disposition tied to named vision section |
| BC-AUDIT-334 | pass-3-deep-skills-batch-1.md | 1395 | BC-6.05.022 | SS-06 | Brownfield, discovery, research skills | disposition-pass: parallelism in batches of 10 with --all |
| BC-AUDIT-335 | pass-3-deep-skills-batch-1.md | 1404 | BC-6.05.023 | SS-06 | Brownfield, discovery, research skills | disposition-pass: vision SHA tracked in rollup header for staleness |
| BC-AUDIT-438 | pass-3-deep-skills-batch-2.md | 433 | BC-6.05.024 | SS-06 | Brownfield, discovery, research skills | intelligence-synthesis: Identity — correlates market/feedback/competitive/analytics into scored... |
| BC-AUDIT-439 | pass-3-deep-skills-batch-2.md | 442 | BC-6.05.025 | SS-06 | Brownfield, discovery, research skills | intelligence-synthesis: Market research is the only required input; works with partial data |
| BC-AUDIT-440 | pass-3-deep-skills-batch-2.md | 451 | BC-6.05.026 | SS-06 | Brownfield, discovery, research skills | intelligence-synthesis: Themes formed by semantic clustering across sources, not per-source listing |
| BC-AUDIT-441 | pass-3-deep-skills-batch-2.md | 460 | BC-6.05.027 | SS-06 | Brownfield, discovery, research skills | intelligence-synthesis: Insights scored on 7 dimensions including evidence_strength |
| BC-AUDIT-442 | pass-3-deep-skills-batch-2.md | 469 | BC-6.05.028 | SS-06 | Brownfield, discovery, research skills | intelligence-synthesis: Routing — composite ≥0.7 AND evidence ≥0.6 → Brief; URGENT compet... |
| BC-AUDIT-443 | pass-3-deep-skills-batch-2.md | 478 | BC-6.05.029 | SS-06 | Brownfield, discovery, research skills | intelligence-synthesis: Output is insights-YYYY-MM-DD.md with frontmatter and per-insight detail |
| BC-AUDIT-451 | pass-3-deep-skills-batch-2.md | 569 | BC-6.05.030 | SS-06 | Brownfield, discovery, research skills | market-intelligence-assessment: Identity — mandatory pre-spec gate producing GO/CAUTION/STOP |
| BC-AUDIT-452 | pass-3-deep-skills-batch-2.md | 578 | BC-6.05.031 | SS-06 | Brownfield, discovery, research skills | market-intelligence-assessment: 5 parallel research tracks via research-agent + Perplexity |
| BC-AUDIT-453 | pass-3-deep-skills-batch-2.md | 587 | BC-6.05.032 | SS-06 | Brownfield, discovery, research skills | market-intelligence-assessment: Recommendation criteria — GO requires pain confirmed + market v... |
| BC-AUDIT-454 | pass-3-deep-skills-batch-2.md | 596 | BC-6.05.033 | SS-06 | Brownfield, discovery, research skills | market-intelligence-assessment: Depth scaled by input level L0-L4; L4 has auto-GO without human gate |
| BC-AUDIT-455 | pass-3-deep-skills-batch-2.md | 605 | BC-6.05.034 | SS-06 | Brownfield, discovery, research skills | market-intelligence-assessment: Output is market-intel.md; STOP override is recorded with reasoning |
| BC-AUDIT-456 | pass-3-deep-skills-batch-2.md | 614 | BC-6.05.035 | SS-06 | Brownfield, discovery, research skills | market-intelligence-assessment: Quality Gate — assumptions explicitly flagged for human validation |
| BC-AUDIT-470 | pass-3-deep-skills-batch-2.md | 766 | BC-6.05.036 | SS-06 | Brownfield, discovery, research skills | multi-repo-phase-0-synthesis: Identity — synthesizes per-repo ingestion outputs into unified pr... |
| BC-AUDIT-471 | pass-3-deep-skills-batch-2.md | 775 | BC-6.05.037 | SS-06 | Brownfield, discovery, research skills | multi-repo-phase-0-synthesis: 8 sequential synthesis steps with named agent per step |
| BC-AUDIT-472 | pass-3-deep-skills-batch-2.md | 784 | BC-6.05.038 | SS-06 | Brownfield, discovery, research skills | multi-repo-phase-0-synthesis: Adversary review uses information asymmetry wall — cannot see raw... |
| BC-AUDIT-473 | pass-3-deep-skills-batch-2.md | 793 | BC-6.05.039 | SS-06 | Brownfield, discovery, research skills | multi-repo-phase-0-synthesis: All T1 writes routed through state-manager |
| BC-AUDIT-474 | pass-3-deep-skills-batch-2.md | 802 | BC-6.05.040 | SS-06 | Brownfield, discovery, research skills | multi-repo-phase-0-synthesis: Quality Gate — unified project-context.md exists with cross-repo ... |
| BC-AUDIT-475 | pass-3-deep-skills-batch-2.md | 811 | BC-6.05.041 | SS-06 | Brownfield, discovery, research skills | multi-repo-phase-0-synthesis: Failure — incomplete per-repo ingestion halts synthesis |
| BC-AUDIT-574 | pass-3-deep-skills-batch-2.md | 1834 | BC-6.05.042 | SS-06 | Brownfield, discovery, research skills | planning-research: Identity — domain/market/technical research via Perplexity + Context7 |
| BC-AUDIT-575 | pass-3-deep-skills-batch-2.md | 1843 | BC-6.05.043 | SS-06 | Brownfield, discovery, research skills | planning-research: Cross-reference findings across ≥2 independent sources; date-stamp all findings |
| BC-AUDIT-576 | pass-3-deep-skills-batch-2.md | 1852 | BC-6.05.044 | SS-06 | Brownfield, discovery, research skills | planning-research: Output is research-report.md + research-sources.md |
| BC-AUDIT-577 | pass-3-deep-skills-batch-2.md | 1861 | BC-6.05.045 | SS-06 | Brownfield, discovery, research skills | planning-research: Quality Gate — sources cited with URLs+dates, uncertainties flagged, ≥2 so... |
| BC-AUDIT-578 | pass-3-deep-skills-batch-2.md | 1870 | BC-6.05.046 | SS-06 | Brownfield, discovery, research skills | planning-research: Failure — MCP tools unavailable → use training data with explicit "UNVERIF... |
| BC-AUDIT-647 | pass-3-deep-skills-batch-3.md | 485 | BC-6.05.047 | SS-06 | Brownfield, discovery, research skills | research: identity & sub-agent fork |
| BC-AUDIT-648 | pass-3-deep-skills-batch-3.md | 494 | BC-6.05.048 | SS-06 | Brownfield, discovery, research skills | research: domain vs general routing by first arg |
| BC-AUDIT-649 | pass-3-deep-skills-batch-3.md | 503 | BC-6.05.049 | SS-06 | Brownfield, discovery, research skills | research: pre-run cache scan + post-run index update + factory commit |
| BC-AUDIT-650 | pass-3-deep-skills-batch-3.md | 516 | BC-6.05.050 | SS-06 | Brownfield, discovery, research skills | research-cache-ops: identity & wraps research-cache binary |
| BC-AUDIT-651 | pass-3-deep-skills-batch-3.md | 525 | BC-6.05.051 | SS-06 | Brownfield, discovery, research skills | research-cache-ops: six operations (stats/key/has/get/put/clear) |
| BC-AUDIT-652 | pass-3-deep-skills-batch-3.md | 534 | BC-6.05.052 | SS-06 | Brownfield, discovery, research skills | research-cache-ops: SHA-256 deterministic key + research-agent integration pattern |
| BC-AUDIT-673 | pass-3-deep-skills-batch-3.md | 743 | BC-6.05.053 | SS-06 | Brownfield, discovery, research skills | semport-analyze: identity, two modes, reference resolution |
| BC-AUDIT-674 | pass-3-deep-skills-batch-3.md | 752 | BC-6.05.054 | SS-06 | Brownfield, discovery, research skills | semport-analyze: incremental protocol is delta-only (5 steps) |
| BC-AUDIT-675 | pass-3-deep-skills-batch-3.md | 761 | BC-6.05.055 | SS-06 | Brownfield, discovery, research skills | semport-analyze: full mode runs codebase-analyzer 6-pass protocol |
| BC-AUDIT-676 | pass-3-deep-skills-batch-3.md | 770 | BC-6.05.056 | SS-06 | Brownfield, discovery, research skills | semport-analyze: language idiom mapping table (Python/TS/Rust pairs) |
| BC-AUDIT-677 | pass-3-deep-skills-batch-3.md | 779 | BC-6.05.057 | SS-06 | Brownfield, discovery, research skills | semport-analyze: validate-extraction agent post-pass (max 3 iterations) |
| BC-AUDIT-678 | pass-3-deep-skills-batch-3.md | 788 | BC-6.05.058 | SS-06 | Brownfield, discovery, research skills | semport-analyze: outputs and post-skill report |
| BC-AUDIT-239 | pass-3-deep-skills-batch-1.md | 444 | BC-6.06.001 | SS-06 | State and convergence skills | check-input-drift: skill identity contract |
| BC-AUDIT-240 | pass-3-deep-skills-batch-1.md | 453 | BC-6.06.002 | SS-06 | State and convergence skills | check-input-drift: scan via single binary, not inline shell loops |
| BC-AUDIT-241 | pass-3-deep-skills-batch-1.md | 462 | BC-6.06.003 | SS-06 | State and convergence skills | check-input-drift: mandatory resolve step after scan |
| BC-AUDIT-242 | pass-3-deep-skills-batch-1.md | 471 | BC-6.06.004 | SS-06 | State and convergence skills | check-input-drift: cluster-drift triage before bulk --update |
| BC-AUDIT-243 | pass-3-deep-skills-batch-1.md | 484 | BC-6.06.005 | SS-06 | State and convergence skills | check-state-health: skill identity (read-only diagnostic) |
| BC-AUDIT-244 | pass-3-deep-skills-batch-1.md | 493 | BC-6.06.006 | SS-06 | State and convergence skills | check-state-health: 7 numbered checks executed in order |
| BC-AUDIT-245 | pass-3-deep-skills-batch-1.md | 502 | BC-6.06.007 | SS-06 | State and convergence skills | check-state-health: stale-phase detection patterns |
| BC-AUDIT-246 | pass-3-deep-skills-batch-1.md | 511 | BC-6.06.008 | SS-06 | State and convergence skills | check-state-health: content-routing antipattern catalog |
| BC-AUDIT-257 | pass-3-deep-skills-batch-1.md | 622 | BC-6.06.009 | SS-06 | State and convergence skills | compact-state: skill identity (extract historical content from STATE.md) |
| BC-AUDIT-258 | pass-3-deep-skills-batch-1.md | 631 | BC-6.06.010 | SS-06 | State and convergence skills | compact-state: 7-pattern extraction map |
| BC-AUDIT-259 | pass-3-deep-skills-batch-1.md | 640 | BC-6.06.011 | SS-06 | State and convergence skills | compact-state: never-deletes safety guarantee |
| BC-AUDIT-260 | pass-3-deep-skills-batch-1.md | 649 | BC-6.06.012 | SS-06 | State and convergence skills | compact-state: post-compaction STATE.md <200 lines + verify |
| BC-AUDIT-273 | pass-3-deep-skills-batch-1.md | 782 | BC-6.06.013 | SS-06 | State and convergence skills | convergence-check: skill identity (Phase 7, 7-dimension validation) |
| BC-AUDIT-274 | pass-3-deep-skills-batch-1.md | 791 | BC-6.06.014 | SS-06 | State and convergence skills | convergence-check: iron law all-7-must-pass |
| BC-AUDIT-275 | pass-3-deep-skills-batch-1.md | 800 | BC-6.06.015 | SS-06 | State and convergence skills | convergence-check: per-dimension pass criteria |
| BC-AUDIT-276 | pass-3-deep-skills-batch-1.md | 809 | BC-6.06.016 | SS-06 | State and convergence skills | convergence-check: writes report at .factory/cycles/<current>/convergence-report.md |
| BC-AUDIT-277 | pass-3-deep-skills-batch-1.md | 822 | BC-6.06.017 | SS-06 | State and convergence skills | convergence-tracking: skill identity (quantitative metrics-driven) |
| BC-AUDIT-278 | pass-3-deep-skills-batch-1.md | 831 | BC-6.06.018 | SS-06 | State and convergence skills | convergence-tracking: spec convergence formula (Novelty < 0.15 + median severity decay) |
| BC-AUDIT-279 | pass-3-deep-skills-batch-1.md | 840 | BC-6.06.019 | SS-06 | State and convergence skills | convergence-tracking: tier-based mutation kill rate thresholds |
| BC-AUDIT-280 | pass-3-deep-skills-batch-1.md | 849 | BC-6.06.020 | SS-06 | State and convergence skills | convergence-tracking: convergence index formula (CI(i)) |
| BC-AUDIT-619 | pass-3-deep-skills-batch-3.md | 217 | BC-6.06.021 | SS-06 | State and convergence skills | recover-state: identity, dry-run, and backup |
| BC-AUDIT-620 | pass-3-deep-skills-batch-3.md | 226 | BC-6.06.022 | SS-06 | State and convergence skills | recover-state: artifact directory probe table is exhaustive |
| BC-AUDIT-621 | pass-3-deep-skills-batch-3.md | 235 | BC-6.06.023 | SS-06 | State and convergence skills | recover-state: phase decision tree is total and ordered |
| BC-AUDIT-622 | pass-3-deep-skills-batch-3.md | 244 | BC-6.06.024 | SS-06 | State and convergence skills | recover-state: requires user approval before write |
| BC-AUDIT-623 | pass-3-deep-skills-batch-3.md | 253 | BC-6.06.025 | SS-06 | State and convergence skills | recover-state: documented limitations are honored (no fabrication) |
| BC-AUDIT-697 | pass-3-deep-skills-batch-3.md | 979 | BC-6.06.026 | SS-06 | State and convergence skills | state-burst: identity & defect-class context |
| BC-AUDIT-698 | pass-3-deep-skills-batch-3.md | 988 | BC-6.06.027 | SS-06 | State and convergence skills | state-burst: announces protocol verbatim |
| BC-AUDIT-699 | pass-3-deep-skills-batch-3.md | 997 | BC-6.06.028 | SS-06 | State and convergence skills | state-burst: Stage 1 with `15fa97e6` placeholder + tense rule |
| BC-AUDIT-700 | pass-3-deep-skills-batch-3.md | 1006 | BC-6.06.029 | SS-06 | State and convergence skills | state-burst: Stage 2 global SHA replace + backfill commit |
| BC-AUDIT-701 | pass-3-deep-skills-batch-3.md | 1015 | BC-6.06.030 | SS-06 | State and convergence skills | state-burst: refuses 3rd commit + recovery via `git reset --soft HEAD~2` |
| BC-AUDIT-702 | pass-3-deep-skills-batch-3.md | 1024 | BC-6.06.031 | SS-06 | State and convergence skills | state-burst: documented bypass paths |
| BC-AUDIT-703 | pass-3-deep-skills-batch-3.md | 1037 | BC-6.06.032 | SS-06 | State and convergence skills | state-update: identity & internal-only contract |
| BC-AUDIT-704 | pass-3-deep-skills-batch-3.md | 1046 | BC-6.06.033 | SS-06 | State and convergence skills | state-update: 4-step procedure (read → frontmatter → history → commit) |
| BC-AUDIT-705 | pass-3-deep-skills-batch-3.md | 1055 | BC-6.06.034 | SS-06 | State and convergence skills | state-update: enumerates 5 pipeline statuses + 7 phase IDs |
| BC-AUDIT-773 | pass-3-deep-skills-batch-3.md | 1723 | BC-6.06.035 | SS-06 | State and convergence skills | wave-gate: identity, allowed tools, pre-flight |
| BC-AUDIT-774 | pass-3-deep-skills-batch-3.md | 1732 | BC-6.06.036 | SS-06 | State and convergence skills | wave-gate: announces protocol verbatim + TodoWrite per gate |
| BC-AUDIT-775 | pass-3-deep-skills-batch-3.md | 1741 | BC-6.06.037 | SS-06 | State and convergence skills | wave-gate: gate sequence is load-bearing (1→2→3→4→5→6, stop on first failure) |
| BC-AUDIT-776 | pass-3-deep-skills-batch-3.md | 1750 | BC-6.06.038 | SS-06 | State and convergence skills | wave-gate: 8-row red-flag rationalization table |
| BC-AUDIT-777 | pass-3-deep-skills-batch-3.md | 1759 | BC-6.06.039 | SS-06 | State and convergence skills | wave-gate: GATE_CHECK telemetry lines (validated by hook) |
| BC-AUDIT-778 | pass-3-deep-skills-batch-3.md | 1768 | BC-6.06.040 | SS-06 | State and convergence skills | wave-gate: outputs and post-pass guidance |
| BC-AUDIT-779 | pass-3-deep-skills-batch-3.md | 1781 | BC-6.06.041 | SS-06 | State and convergence skills | wave-scheduling: identity & topo-sort algorithm |
| BC-AUDIT-780 | pass-3-deep-skills-batch-3.md | 1790 | BC-6.06.042 | SS-06 | State and convergence skills | wave-scheduling: parallel groups (≤2 S/M or ≤1 L/XL per group) |
| BC-AUDIT-781 | pass-3-deep-skills-batch-3.md | 1799 | BC-6.06.043 | SS-06 | State and convergence skills | wave-scheduling: pipeline overlap (Wave N+1 stubs while Wave N implements) |
| BC-AUDIT-782 | pass-3-deep-skills-batch-3.md | 1808 | BC-6.06.044 | SS-06 | State and convergence skills | wave-scheduling: wave-schedule.md output + quality gate |
| BC-AUDIT-783 | pass-3-deep-skills-batch-3.md | 1817 | BC-6.06.045 | SS-06 | State and convergence skills | wave-scheduling: failure modes (cycle/missing dep/no roots) |
| BC-AUDIT-784 | pass-3-deep-skills-batch-3.md | 1830 | BC-6.06.046 | SS-06 | State and convergence skills | wave-status: identity & read-only contract |
| BC-AUDIT-264 | pass-3-deep-skills-batch-1.md | 693 | BC-6.07.001 | SS-06 | Spec creation and validation skills | conform-to-template: skill identity (additive only — never deletes) |
| BC-AUDIT-265 | pass-3-deep-skills-batch-1.md | 702 | BC-6.07.002 | SS-06 | Spec creation and validation skills | conform-to-template: refuses table-column changes and section reordering |
| BC-AUDIT-266 | pass-3-deep-skills-batch-1.md | 711 | BC-6.07.003 | SS-06 | Spec creation and validation skills | conform-to-template: user approval gate before write |
| BC-AUDIT-267 | pass-3-deep-skills-batch-1.md | 720 | BC-6.07.004 | SS-06 | Spec creation and validation skills | conform-to-template: post-conformance re-validation reports before/after |
| BC-AUDIT-268 | pass-3-deep-skills-batch-1.md | 733 | BC-6.07.005 | SS-06 | Spec creation and validation skills | consistency-validation: skill identity (cross-document validator) |
| BC-AUDIT-269 | pass-3-deep-skills-batch-1.md | 742 | BC-6.07.006 | SS-06 | Spec creation and validation skills | consistency-validation: 36 numbered rules executed in order |
| BC-AUDIT-270 | pass-3-deep-skills-batch-1.md | 751 | BC-6.07.007 | SS-06 | Spec creation and validation skills | consistency-validation: index-first validation precedes detail loading (DF-021) |
| BC-AUDIT-271 | pass-3-deep-skills-batch-1.md | 760 | BC-6.07.008 | SS-06 | Spec creation and validation skills | consistency-validation: BC clause reverse-coverage severity (Rule 25) |
| BC-AUDIT-272 | pass-3-deep-skills-batch-1.md | 769 | BC-6.07.009 | SS-06 | Spec creation and validation skills | consistency-validation: NFR-to-Story severity by priority tier (Rule 27) |
| BC-AUDIT-281 | pass-3-deep-skills-batch-1.md | 862 | BC-6.07.010 | SS-06 | Spec creation and validation skills | create-architecture: skill identity + iron law (verification feasibility) |
| BC-AUDIT-282 | pass-3-deep-skills-batch-1.md | 871 | BC-6.07.011 | SS-06 | Spec creation and validation skills | create-architecture: ADR style for every decision |
| BC-AUDIT-283 | pass-3-deep-skills-batch-1.md | 880 | BC-6.07.012 | SS-06 | Spec creation and validation skills | create-architecture: sharded output (ARCH-INDEX + section files) |
| BC-AUDIT-284 | pass-3-deep-skills-batch-1.md | 889 | BC-6.07.013 | SS-06 | Spec creation and validation skills | create-architecture: VP files written to verification-properties/ |
| BC-AUDIT-285 | pass-3-deep-skills-batch-1.md | 902 | BC-6.07.014 | SS-06 | Spec creation and validation skills | create-brief: skill identity + hard gate |
| BC-AUDIT-286 | pass-3-deep-skills-batch-1.md | 911 | BC-6.07.015 | SS-06 | Spec creation and validation skills | create-brief: factory-health prerequisite + research check |
| BC-AUDIT-287 | pass-3-deep-skills-batch-1.md | 920 | BC-6.07.016 | SS-06 | Spec creation and validation skills | create-brief: questions one-at-a-time, multiple choice when possible |
| BC-AUDIT-288 | pass-3-deep-skills-batch-1.md | 929 | BC-6.07.017 | SS-06 | Spec creation and validation skills | create-brief: writes product-brief.md with 8 named sections + state-update |
| BC-AUDIT-289 | pass-3-deep-skills-batch-1.md | 942 | BC-6.07.018 | SS-06 | Spec creation and validation skills | create-domain-spec: skill identity (sharded L2 spec) |
| BC-AUDIT-290 | pass-3-deep-skills-batch-1.md | 951 | BC-6.07.019 | SS-06 | Spec creation and validation skills | create-domain-spec: 3-pass extraction (structural + behavioral + context) |
| BC-AUDIT-291 | pass-3-deep-skills-batch-1.md | 960 | BC-6.07.020 | SS-06 | Spec creation and validation skills | create-domain-spec: sharded output structure (5 named files) |
| BC-AUDIT-292 | pass-3-deep-skills-batch-1.md | 973 | BC-6.07.021 | SS-06 | Spec creation and validation skills | create-excalidraw: skill identity (programmatic .excalidraw JSON generation) |
| BC-AUDIT-293 | pass-3-deep-skills-batch-1.md | 982 | BC-6.07.022 | SS-06 | Spec creation and validation skills | create-excalidraw: deterministic IDs (not random UUIDs) |
| BC-AUDIT-294 | pass-3-deep-skills-batch-1.md | 991 | BC-6.07.023 | SS-06 | Spec creation and validation skills | create-excalidraw: arrow points property required (workaround for export bug) |
| BC-AUDIT-295 | pass-3-deep-skills-batch-1.md | 1004 | BC-6.07.024 | SS-06 | Spec creation and validation skills | create-prd: skill identity + hard gate |
| BC-AUDIT-296 | pass-3-deep-skills-batch-1.md | 1013 | BC-6.07.025 | SS-06 | Spec creation and validation skills | create-prd: each BC must be testable, unambiguous, complete |
| BC-AUDIT-297 | pass-3-deep-skills-batch-1.md | 1022 | BC-6.07.026 | SS-06 | Spec creation and validation skills | create-prd: 3 named PRD supplements |
| BC-AUDIT-298 | pass-3-deep-skills-batch-1.md | 1031 | BC-6.07.027 | SS-06 | Spec creation and validation skills | create-prd: BC reference repos integration (Source line in BC traceability) |
| BC-AUDIT-299 | pass-3-deep-skills-batch-1.md | 1044 | BC-6.07.028 | SS-06 | Spec creation and validation skills | create-story: skill identity + hard gate |
| BC-AUDIT-300 | pass-3-deep-skills-batch-1.md | 1053 | BC-6.07.029 | SS-06 | Spec creation and validation skills | create-story: 7 plan-failure patterns block proceeding |
| BC-AUDIT-301 | pass-3-deep-skills-batch-1.md | 1062 | BC-6.07.030 | SS-06 | Spec creation and validation skills | create-story: forbidden dependencies + version pin enforcement |
| BC-AUDIT-308 | pass-3-deep-skills-batch-1.md | 1137 | BC-6.07.031 | SS-06 | Spec creation and validation skills | decompose-stories: skill identity + iron law (BC traceability) |
| BC-AUDIT-309 | pass-3-deep-skills-batch-1.md | 1146 | BC-6.07.032 | SS-06 | Spec creation and validation skills | decompose-stories: 13-point story size limit (must split before implementation) |
| BC-AUDIT-310 | pass-3-deep-skills-batch-1.md | 1155 | BC-6.07.033 | SS-06 | Spec creation and validation skills | decompose-stories: dependency graph acyclicity verified programmatically |
| BC-AUDIT-311 | pass-3-deep-skills-batch-1.md | 1164 | BC-6.07.034 | SS-06 | Spec creation and validation skills | decompose-stories: 5 named output artifacts + holdout scenarios |
| BC-AUDIT-421 | pass-3-deep-skills-batch-2.md | 262 | BC-6.07.035 | SS-06 | Spec creation and validation skills | guided-brief-creation: Identity — staged elicitation from raw idea to product brief |
| BC-AUDIT-422 | pass-3-deep-skills-batch-2.md | 271 | BC-6.07.036 | SS-06 | Spec creation and validation skills | guided-brief-creation: Hard gate — must complete brief before any PRD/architecture/implementation |
| BC-AUDIT-423 | pass-3-deep-skills-batch-2.md | 280 | BC-6.07.037 | SS-06 | Spec creation and validation skills | guided-brief-creation: Capture-don't-interrupt rule preserves human creative flow |
| BC-AUDIT-424 | pass-3-deep-skills-batch-2.md | 289 | BC-6.07.038 | SS-06 | Spec creation and validation skills | guided-brief-creation: Output is product-brief.md (and elicitation-notes.md if applicable) |
| BC-AUDIT-425 | pass-3-deep-skills-batch-2.md | 298 | BC-6.07.039 | SS-06 | Spec creation and validation skills | guided-brief-creation: Failure mode — contradictory requirements halt elicitation for human res... |
| BC-AUDIT-689 | pass-3-deep-skills-batch-3.md | 899 | BC-6.07.040 | SS-06 | Spec creation and validation skills | spec-drift: identity & forked Explore agent |
| BC-AUDIT-690 | pass-3-deep-skills-batch-3.md | 908 | BC-6.07.041 | SS-06 | Spec creation and validation skills | spec-drift: scans 4 spec dirs + checks naming + finds orphans |
| BC-AUDIT-691 | pass-3-deep-skills-batch-3.md | 917 | BC-6.07.042 | SS-06 | Spec creation and validation skills | spec-drift: writes spec-drift-report.md to current cycle |
| BC-AUDIT-692 | pass-3-deep-skills-batch-3.md | 930 | BC-6.07.043 | SS-06 | Spec creation and validation skills | spec-versioning: identity & semver scheme |
| BC-AUDIT-693 | pass-3-deep-skills-batch-3.md | 939 | BC-6.07.044 | SS-06 | Spec creation and validation skills | spec-versioning: bump-type rules (MAJOR/MINOR/PATCH) |
| BC-AUDIT-694 | pass-3-deep-skills-batch-3.md | 948 | BC-6.07.045 | SS-06 | Spec creation and validation skills | spec-versioning: per-story spec_version + drift detection |
| BC-AUDIT-695 | pass-3-deep-skills-batch-3.md | 957 | BC-6.07.046 | SS-06 | Spec creation and validation skills | spec-versioning: L4 immutability rules + locked-VP enforcement |
| BC-AUDIT-696 | pass-3-deep-skills-batch-3.md | 966 | BC-6.07.047 | SS-06 | Spec creation and validation skills | spec-versioning: failure-mode safety (inconsistent versions / locked-VP modified / unparseable) |
| BC-AUDIT-726 | pass-3-deep-skills-batch-3.md | 1260 | BC-6.07.048 | SS-06 | Spec creation and validation skills | traceability-extension: identity & chain semantics |
| BC-AUDIT-727 | pass-3-deep-skills-batch-3.md | 1269 | BC-6.07.049 | SS-06 | Spec creation and validation skills | traceability-extension: 7 extension rules (IDs new, links append-only, deprecated stays) |
| BC-AUDIT-728 | pass-3-deep-skills-batch-3.md | 1278 | BC-6.07.050 | SS-06 | Spec creation and validation skills | traceability-extension: architecture-section-level references (DF-021) |
| BC-AUDIT-729 | pass-3-deep-skills-batch-3.md | 1287 | BC-6.07.051 | SS-06 | Spec creation and validation skills | traceability-extension: chain verification command |
| BC-AUDIT-751 | pass-3-deep-skills-batch-3.md | 1505 | BC-6.07.052 | SS-06 | Spec creation and validation skills | validate-brief: identity & step-file note |
| BC-AUDIT-752 | pass-3-deep-skills-batch-3.md | 1514 | BC-6.07.053 | SS-06 | Spec creation and validation skills | validate-brief: structure check requires 6 sections each meeting minimums |
| BC-AUDIT-753 | pass-3-deep-skills-batch-3.md | 1523 | BC-6.07.054 | SS-06 | Spec creation and validation skills | validate-brief: bloat check (<500/<800/<1500 token bands) |
| BC-AUDIT-754 | pass-3-deep-skills-batch-3.md | 1532 | BC-6.07.055 | SS-06 | Spec creation and validation skills | validate-brief: implementation-leakage tech-name scanner |
| BC-AUDIT-755 | pass-3-deep-skills-batch-3.md | 1541 | BC-6.07.056 | SS-06 | Spec creation and validation skills | validate-brief: information density anti-patterns (4 categories + thresholds) |
| BC-AUDIT-756 | pass-3-deep-skills-batch-3.md | 1550 | BC-6.07.057 | SS-06 | Spec creation and validation skills | validate-brief: market intel cross-check + report file + overall verdict |
| BC-AUDIT-757 | pass-3-deep-skills-batch-3.md | 1563 | BC-6.07.058 | SS-06 | Spec creation and validation skills | validate-consistency: identity & frontmatter |
| BC-AUDIT-758 | pass-3-deep-skills-batch-3.md | 1572 | BC-6.07.059 | SS-06 | Spec creation and validation skills | validate-consistency: 7 cross-file checks (BC/VP/Story/Architecture/Counts/Status/Naming) |
| BC-AUDIT-759 | pass-3-deep-skills-batch-3.md | 1581 | BC-6.07.060 | SS-06 | Spec creation and validation skills | validate-consistency: report format with Failures/Warnings/All Passed |
| BC-AUDIT-760 | pass-3-deep-skills-batch-3.md | 1594 | BC-6.07.061 | SS-06 | Spec creation and validation skills | validate-template-compliance: identity & three scopes (file/dir/all) |
| BC-AUDIT-761 | pass-3-deep-skills-batch-3.md | 1603 | BC-6.07.062 | SS-06 | Spec creation and validation skills | validate-template-compliance: template resolution by document_type then path |
| BC-AUDIT-762 | pass-3-deep-skills-batch-3.md | 1612 | BC-6.07.063 | SS-06 | Spec creation and validation skills | validate-template-compliance: 3-level compliance check (frontmatter/sections/tables) |
| BC-AUDIT-763 | pass-3-deep-skills-batch-3.md | 1621 | BC-6.07.064 | SS-06 | Spec creation and validation skills | validate-template-compliance: report format (per-file detail + summary table + aggregate counts) |
| BC-AUDIT-764 | pass-3-deep-skills-batch-3.md | 1630 | BC-6.07.065 | SS-06 | Spec creation and validation skills | validate-template-compliance: documented limitations (no content quality, no value validation) |
| BC-AUDIT-765 | pass-3-deep-skills-batch-3.md | 1643 | BC-6.07.066 | SS-06 | Spec creation and validation skills | validate-workflow: identity & static-only contract |
| BC-AUDIT-766 | pass-3-deep-skills-batch-3.md | 1652 | BC-6.07.067 | SS-06 | Spec creation and validation skills | validate-workflow: 6 checks (required fields/agent/skill/depends_on/dup names/top-level) |
| BC-AUDIT-767 | pass-3-deep-skills-batch-3.md | 1661 | BC-6.07.068 | SS-06 | Spec creation and validation skills | validate-workflow: collects all errors (no early bail) + exit code |
| BC-AUDIT-317 | pass-3-deep-skills-batch-1.md | 1226 | BC-6.08.001 | SS-06 | Demo, UX, and design skills | demo-recording: skill identity (CLI/web/API/library) |
| BC-AUDIT-318 | pass-3-deep-skills-batch-1.md | 1235 | BC-6.08.002 | SS-06 | Demo, UX, and design skills | demo-recording: 5 detection signals → demo type → tool |
| BC-AUDIT-319 | pass-3-deep-skills-batch-1.md | 1244 | BC-6.08.003 | SS-06 | Demo, UX, and design skills | demo-recording: target sizes (WebM <2MB, GIF <5MB, total <25MB) |
| BC-AUDIT-320 | pass-3-deep-skills-batch-1.md | 1253 | BC-6.08.004 | SS-06 | Demo, UX, and design skills | demo-recording: every AC has user-observable behavior covered + visual review |
| BC-AUDIT-321 | pass-3-deep-skills-batch-1.md | 1266 | BC-6.08.005 | SS-06 | Demo, UX, and design skills | design-drift-detection: skill identity (Sweep 10, UI products only) |
| BC-AUDIT-322 | pass-3-deep-skills-batch-1.md | 1275 | BC-6.08.006 | SS-06 | Demo, UX, and design skills | design-drift-detection: emergent pattern threshold (>=3 instances → propose) |
| BC-AUDIT-323 | pass-3-deep-skills-batch-1.md | 1284 | BC-6.08.007 | SS-06 | Demo, UX, and design skills | design-drift-detection: graceful skip when no design system |
| BC-AUDIT-324 | pass-3-deep-skills-batch-1.md | 1297 | BC-6.08.008 | SS-06 | Demo, UX, and design skills | design-system-bootstrap: skill identity (greenfield + brownfield + feature) |
| BC-AUDIT-325 | pass-3-deep-skills-batch-1.md | 1306 | BC-6.08.009 | SS-06 | Demo, UX, and design skills | design-system-bootstrap: minimal bootstrap fallback when no brand guidelines |
| BC-AUDIT-326 | pass-3-deep-skills-batch-1.md | 1315 | BC-6.08.010 | SS-06 | Demo, UX, and design skills | design-system-bootstrap: WCAG AA contrast validation (accessibility-auditor) |
| BC-AUDIT-342 | pass-3-deep-skills-batch-1.md | 1479 | BC-6.08.011 | SS-06 | Demo, UX, and design skills | excalidraw-export: skill identity (reference-only batch PNG export) |
| BC-AUDIT-343 | pass-3-deep-skills-batch-1.md | 1488 | BC-6.08.012 | SS-06 | Demo, UX, and design skills | excalidraw-export: arrow points workaround documented (must have `points`) |
| BC-AUDIT-476 | pass-3-deep-skills-batch-2.md | 826 | BC-6.08.013 | SS-06 | Demo, UX, and design skills | multi-variant-design: Identity — generates 2-3 variants per complex screen scored by 4 agents o... |
| BC-AUDIT-477 | pass-3-deep-skills-batch-2.md | 835 | BC-6.08.014 | SS-06 | Demo, UX, and design skills | multi-variant-design: Top variant + runner-up presented for human selection or synthesis |
| BC-AUDIT-478 | pass-3-deep-skills-batch-2.md | 844 | BC-6.08.015 | SS-06 | Demo, UX, and design skills | multi-variant-design: Output is SCR-NNN-variants.md per complex screen |
| BC-AUDIT-479 | pass-3-deep-skills-batch-2.md | 853 | BC-6.08.016 | SS-06 | Demo, UX, and design skills | multi-variant-design: Failure — score deadlock (within 0.05) presents both with dimension break... |
| BC-AUDIT-614 | pass-3-deep-skills-batch-3.md | 168 | BC-6.08.017 | SS-06 | Demo, UX, and design skills | record-demo: identity & template usage |
| BC-AUDIT-615 | pass-3-deep-skills-batch-3.md | 177 | BC-6.08.018 | SS-06 | Demo, UX, and design skills | record-demo: per-AC evidence capture (CLI vs web) |
| BC-AUDIT-616 | pass-3-deep-skills-batch-3.md | 186 | BC-6.08.019 | SS-06 | Demo, UX, and design skills | record-demo: writes demo-report.md with per-AC table |
| BC-AUDIT-617 | pass-3-deep-skills-batch-3.md | 195 | BC-6.08.020 | SS-06 | Demo, UX, and design skills | record-demo: tool-unavailable fallback never skips evidence |
| BC-AUDIT-618 | pass-3-deep-skills-batch-3.md | 204 | BC-6.08.021 | SS-06 | Demo, UX, and design skills | record-demo: commits evidence to factory-artifacts |
| BC-AUDIT-653 | pass-3-deep-skills-batch-3.md | 547 | BC-6.08.022 | SS-06 | Demo, UX, and design skills | responsive-validation: identity, agents, conditional UI gating |
| BC-AUDIT-654 | pass-3-deep-skills-batch-3.md | 556 | BC-6.08.023 | SS-06 | Demo, UX, and design skills | responsive-validation: 4 mandatory breakpoints (375/768/1024/1440) |
| BC-AUDIT-655 | pass-3-deep-skills-batch-3.md | 565 | BC-6.08.024 | SS-06 | Demo, UX, and design skills | responsive-validation: critical-failure list is blocking |
| BC-AUDIT-656 | pass-3-deep-skills-batch-3.md | 574 | BC-6.08.025 | SS-06 | Demo, UX, and design skills | responsive-validation: screenshot evidence + per-screen pass/fail matrix |
| BC-AUDIT-657 | pass-3-deep-skills-batch-3.md | 583 | BC-6.08.026 | SS-06 | Demo, UX, and design skills | responsive-validation: failure modes (resize/screenshot/auth) |
| BC-AUDIT-706 | pass-3-deep-skills-batch-3.md | 1068 | BC-6.08.027 | SS-06 | Demo, UX, and design skills | storybook-mcp-integration: identity & UI-conditional invocation |
| BC-AUDIT-707 | pass-3-deep-skills-batch-3.md | 1077 | BC-6.08.028 | SS-06 | Demo, UX, and design skills | storybook-mcp-integration: install + config + register procedure |
| BC-AUDIT-708 | pass-3-deep-skills-batch-3.md | 1086 | BC-6.08.029 | SS-06 | Demo, UX, and design skills | storybook-mcp-integration: 6 MCP tools mapped to agent roles |
| BC-AUDIT-709 | pass-3-deep-skills-batch-3.md | 1095 | BC-6.08.030 | SS-06 | Demo, UX, and design skills | storybook-mcp-integration: T1/T2/T3 access pattern via DF-023+DF-027 |
| BC-AUDIT-710 | pass-3-deep-skills-batch-3.md | 1104 | BC-6.08.031 | SS-06 | Demo, UX, and design skills | storybook-mcp-integration: self-healing loop with 10-iteration cap |
| BC-AUDIT-711 | pass-3-deep-skills-batch-3.md | 1113 | BC-6.08.032 | SS-06 | Demo, UX, and design skills | storybook-mcp-integration: reuse-first enforcement before new components |
| BC-AUDIT-712 | pass-3-deep-skills-batch-3.md | 1122 | BC-6.08.033 | SS-06 | Demo, UX, and design skills | storybook-mcp-integration: non-React fallback (manifest only) |
| BC-AUDIT-734 | pass-3-deep-skills-batch-3.md | 1340 | BC-6.08.034 | SS-06 | Demo, UX, and design skills | ui-completeness-check: identity, agents, UI gating, zero-gap rule |
| BC-AUDIT-735 | pass-3-deep-skills-batch-3.md | 1349 | BC-6.08.035 | SS-06 | Demo, UX, and design skills | ui-completeness-check: 3 pipeline points with strictness gradient |
| BC-AUDIT-736 | pass-3-deep-skills-batch-3.md | 1358 | BC-6.08.036 | SS-06 | Demo, UX, and design skills | ui-completeness-check: ui-traceability.yaml schema (screens with components/interactions/responsi... |
| BC-AUDIT-737 | pass-3-deep-skills-batch-3.md | 1367 | BC-6.08.037 | SS-06 | Demo, UX, and design skills | ui-completeness-check: 7-axis gap detection (screens/components/states/interactions/responsive/a1... |
| BC-AUDIT-738 | pass-3-deep-skills-batch-3.md | 1376 | BC-6.08.038 | SS-06 | Demo, UX, and design skills | ui-completeness-check: state coverage (D4) — 4 async states + per-component-type required states |
| BC-AUDIT-739 | pass-3-deep-skills-batch-3.md | 1385 | BC-6.08.039 | SS-06 | Demo, UX, and design skills | ui-completeness-check: 100% fidelity target + fix story generation |
| BC-AUDIT-740 | pass-3-deep-skills-batch-3.md | 1398 | BC-6.08.040 | SS-06 | Demo, UX, and design skills | ui-quality-gate: identity, agents, UI conditional |
| BC-AUDIT-741 | pass-3-deep-skills-batch-3.md | 1407 | BC-6.08.041 | SS-06 | Demo, UX, and design skills | ui-quality-gate: comprehensive checklist across 5 dimensions |
| BC-AUDIT-742 | pass-3-deep-skills-batch-3.md | 1416 | BC-6.08.042 | SS-06 | Demo, UX, and design skills | ui-quality-gate: 4 strictness levels (per-story/wave/build/convergence) |
| BC-AUDIT-743 | pass-3-deep-skills-batch-3.md | 1425 | BC-6.08.043 | SS-06 | Demo, UX, and design skills | ui-quality-gate: failure → FIX-UI-NNN routing |
| BC-AUDIT-744 | pass-3-deep-skills-batch-3.md | 1434 | BC-6.08.044 | SS-06 | Demo, UX, and design skills | ui-quality-gate: performance targets (D8 LCP/FID/CLS/TTI/bundle/images) |
| BC-AUDIT-745 | pass-3-deep-skills-batch-3.md | 1443 | BC-6.08.045 | SS-06 | Demo, UX, and design skills | ui-quality-gate: gate-report.md structure (Gate Level + Result + Checklist + Failures + Perf table) |
| BC-AUDIT-746 | pass-3-deep-skills-batch-3.md | 1456 | BC-6.08.046 | SS-06 | Demo, UX, and design skills | ux-heuristic-evaluation: identity, conditional, four pipeline points |
| BC-AUDIT-747 | pass-3-deep-skills-batch-3.md | 1465 | BC-6.08.047 | SS-06 | Demo, UX, and design skills | ux-heuristic-evaluation: Nielsen 10 heuristics with explicit subchecks |
| BC-AUDIT-748 | pass-3-deep-skills-batch-3.md | 1474 | BC-6.08.048 | SS-06 | Demo, UX, and design skills | ux-heuristic-evaluation: cognitive walkthrough per key task |
| BC-AUDIT-749 | pass-3-deep-skills-batch-3.md | 1483 | BC-6.08.049 | SS-06 | Demo, UX, and design skills | ux-heuristic-evaluation: 0.7 threshold + remediation flagging |
| BC-AUDIT-750 | pass-3-deep-skills-batch-3.md | 1492 | BC-6.08.050 | SS-06 | Demo, UX, and design skills | ux-heuristic-evaluation: report path + failure modes |
| BC-AUDIT-768 | pass-3-deep-skills-batch-3.md | 1674 | BC-6.08.051 | SS-06 | Demo, UX, and design skills | visual-companion: identity, prerequisites, optional setup |
| BC-AUDIT-769 | pass-3-deep-skills-batch-3.md | 1683 | BC-6.08.052 | SS-06 | Demo, UX, and design skills | visual-companion: server lifecycle (start/url/state-dir/stop, 30-min auto-exit) |
| BC-AUDIT-770 | pass-3-deep-skills-batch-3.md | 1692 | BC-6.08.053 | SS-06 | Demo, UX, and design skills | visual-companion: write-loop discipline (Write tool, semantic filenames, no reuse) |
| BC-AUDIT-771 | pass-3-deep-skills-batch-3.md | 1701 | BC-6.08.054 | SS-06 | Demo, UX, and design skills | visual-companion: visual-vs-terminal decision rule |
| BC-AUDIT-772 | pass-3-deep-skills-batch-3.md | 1710 | BC-6.08.055 | SS-06 | Demo, UX, and design skills | visual-companion: excalidraw mode + composed views (screen.json manifest) |
| BC-AUDIT-336 | pass-3-deep-skills-batch-1.md | 1417 | BC-6.09.001 | SS-06 | DTU (Digital Twin Universe) skills | dtu-creation: skill identity (build behavioral clones) |
| BC-AUDIT-337 | pass-3-deep-skills-batch-1.md | 1426 | BC-6.09.002 | SS-06 | DTU (Digital Twin Universe) skills | dtu-creation: fidelity level driven by SUT usage |
| BC-AUDIT-338 | pass-3-deep-skills-batch-1.md | 1435 | BC-6.09.003 | SS-06 | DTU (Digital Twin Universe) skills | dtu-creation: clone validation via contract tests + Schemathesis |
| BC-AUDIT-339 | pass-3-deep-skills-batch-1.md | 1448 | BC-6.09.004 | SS-06 | DTU (Digital Twin Universe) skills | dtu-validate: skill identity (independent reimplementation comparison) |
| BC-AUDIT-340 | pass-3-deep-skills-batch-1.md | 1457 | BC-6.09.005 | SS-06 | DTU (Digital Twin Universe) skills | dtu-validate: criticality-driven candidacy |
| BC-AUDIT-341 | pass-3-deep-skills-batch-1.md | 1466 | BC-6.09.006 | SS-06 | DTU (Digital Twin Universe) skills | dtu-validate: divergence in CRITICAL = blocking |
| BC-AUDIT-312 | pass-3-deep-skills-batch-1.md | 1177 | BC-6.10.001 | SS-06 | Story delivery skills | deliver-story: skill identity (dispatcher, not implementer) |
| BC-AUDIT-313 | pass-3-deep-skills-batch-1.md | 1186 | BC-6.10.002 | SS-06 | Story delivery skills | deliver-story: 9-step dispatch sequence with exit conditions |
| BC-AUDIT-314 | pass-3-deep-skills-batch-1.md | 1195 | BC-6.10.003 | SS-06 | Story delivery skills | deliver-story: Red Gate verification in step 3 (mandatory) |
| BC-AUDIT-315 | pass-3-deep-skills-batch-1.md | 1204 | BC-6.10.004 | SS-06 | Story delivery skills | deliver-story: verification discipline — never trust agent reports |
| BC-AUDIT-316 | pass-3-deep-skills-batch-1.md | 1213 | BC-6.10.005 | SS-06 | Story delivery skills | deliver-story: context discipline mapping per specialist |
| BC-AUDIT-430 | pass-3-deep-skills-batch-2.md | 355 | BC-6.10.006 | SS-06 | Story delivery skills | implementation-readiness: Identity — gate between planning and building, validates spec package... |
| BC-AUDIT-431 | pass-3-deep-skills-batch-2.md | 364 | BC-6.10.007 | SS-06 | Story delivery skills | implementation-readiness: Validation runs 8 dimensions in parallel, not sequential |
| BC-AUDIT-432 | pass-3-deep-skills-batch-2.md | 373 | BC-6.10.008 | SS-06 | Story delivery skills | implementation-readiness: PRD bloat check — narrative padding in requirements is a finding |
| BC-AUDIT-433 | pass-3-deep-skills-batch-2.md | 382 | BC-6.10.009 | SS-06 | Story delivery skills | implementation-readiness: Context budget warns when total exceeds 60% of agent context window |
| BC-AUDIT-434 | pass-3-deep-skills-batch-2.md | 391 | BC-6.10.010 | SS-06 | Story delivery skills | implementation-readiness: PRD implementation leakage scan flags premature tech decisions |
| BC-AUDIT-435 | pass-3-deep-skills-batch-2.md | 400 | BC-6.10.011 | SS-06 | Story delivery skills | implementation-readiness: PRD information density — Critical >10, Warning 5-10, Pass <5 issues ... |
| BC-AUDIT-436 | pass-3-deep-skills-batch-2.md | 409 | BC-6.10.012 | SS-06 | Story delivery skills | implementation-readiness: Story tokens 300-800; total context ≤60% of agent window |
| BC-AUDIT-437 | pass-3-deep-skills-batch-2.md | 418 | BC-6.10.013 | SS-06 | Story delivery skills | implementation-readiness: Output is readiness-report.md with READY\|CONCERNS\|NOT_READY verdict |
| BC-AUDIT-590 | pass-3-deep-skills-batch-2.md | 1996 | BC-6.10.014 | SS-06 | Story delivery skills | post-feature-validation: Identity — monitors post-ship feedback at 7/30/90-day intervals; entir... |
| BC-AUDIT-591 | pass-3-deep-skills-batch-2.md | 2005 | BC-6.10.015 | SS-06 | Story delivery skills | post-feature-validation: Verdict thresholds — SUCCESS / PARTIAL / MISS based on adoption + feed... |
| BC-AUDIT-592 | pass-3-deep-skills-batch-2.md | 2014 | BC-6.10.016 | SS-06 | Story delivery skills | post-feature-validation: Default success criteria — adoption ≥0.10, positive ratio ≥0.6, er... |
| BC-AUDIT-593 | pass-3-deep-skills-batch-2.md | 2023 | BC-6.10.017 | SS-06 | Story delivery skills | post-feature-validation: Output is feature-impact-[name]-YYYY-MM-DD.md with adoption + feedback +... |
| BC-AUDIT-594 | pass-3-deep-skills-batch-2.md | 2032 | BC-6.10.018 | SS-06 | Story delivery skills | post-feature-validation: Feeds back into discovery — calibration data, new pain points, evidenc... |
| BC-AUDIT-595 | pass-3-deep-skills-batch-2.md | 2041 | BC-6.10.019 | SS-06 | Story delivery skills | post-feature-validation: Quality Gate — only runs when enabled, evidence-based verdict, actiona... |
| BC-AUDIT-596 | pass-3-deep-skills-batch-2.md | 2050 | BC-6.10.020 | SS-06 | Story delivery skills | post-feature-validation: Failure — feedback channels unavailable → analyze available data and... |
| BC-AUDIT-344 | pass-3-deep-skills-batch-1.md | 1501 | BC-6.11.001 | SS-06 | Factory operations and dashboards skills | factory-cycles-bootstrap: skill identity (flat → cycle-keyed migration) |
| BC-AUDIT-345 | pass-3-deep-skills-batch-1.md | 1510 | BC-6.11.002 | SS-06 | Factory operations and dashboards skills | factory-cycles-bootstrap: archives via `git mv` (preserves history) |
| BC-AUDIT-346 | pass-3-deep-skills-batch-1.md | 1519 | BC-6.11.003 | SS-06 | Factory operations and dashboards skills | factory-cycles-bootstrap: writes .factory/current-cycle pointer |
| BC-AUDIT-347 | pass-3-deep-skills-batch-1.md | 1532 | BC-6.11.004 | SS-06 | Factory operations and dashboards skills | factory-dashboard: skill identity (read-only diagnostic) |
| BC-AUDIT-348 | pass-3-deep-skills-batch-1.md | 1541 | BC-6.11.005 | SS-06 | Factory operations and dashboards skills | factory-dashboard: missing files produce "not initialized" notices, not errors |
| BC-AUDIT-349 | pass-3-deep-skills-batch-1.md | 1550 | BC-6.11.006 | SS-06 | Factory operations and dashboards skills | factory-dashboard: --factory PATH and --days N options |
| BC-AUDIT-350 | pass-3-deep-skills-batch-1.md | 1563 | BC-6.11.007 | SS-06 | Factory operations and dashboards skills | factory-health: skill identity (auto-repairing worktree validator) |
| BC-AUDIT-351 | pass-3-deep-skills-batch-1.md | 1572 | BC-6.11.008 | SS-06 | Factory operations and dashboards skills | factory-health: 8 sequential checks with auto-repair on missing structures |
| BC-AUDIT-352 | pass-3-deep-skills-batch-1.md | 1581 | BC-6.11.009 | SS-06 | Factory operations and dashboards skills | factory-health: STATE.md size thresholds (200/500 lines) |
| BC-AUDIT-353 | pass-3-deep-skills-batch-1.md | 1594 | BC-6.11.010 | SS-06 | Factory operations and dashboards skills | factory-obs: skill identity (manage local observability stack) |
| BC-AUDIT-354 | pass-3-deep-skills-batch-1.md | 1603 | BC-6.11.011 | SS-06 | Factory operations and dashboards skills | factory-obs: 9-arg subcommand surface |
| BC-AUDIT-355 | pass-3-deep-skills-batch-1.md | 1612 | BC-6.11.012 | SS-06 | Factory operations and dashboards skills | factory-obs: multi-factory watched-list at ~/.config/vsdd-factory/watched-factories |
| BC-AUDIT-356 | pass-3-deep-skills-batch-1.md | 1621 | BC-6.11.013 | SS-06 | Factory operations and dashboards skills | factory-obs: env override port allowlist |
| BC-AUDIT-357 | pass-3-deep-skills-batch-1.md | 1634 | BC-6.11.014 | SS-06 | Factory operations and dashboards skills | factory-worktree-health: skill identity (blocking precondition) |
| BC-AUDIT-358 | pass-3-deep-skills-batch-1.md | 1643 | BC-6.11.015 | SS-06 | Factory operations and dashboards skills | factory-worktree-health: workspace isolation guard (Step 0) |
| BC-AUDIT-359 | pass-3-deep-skills-batch-1.md | 1652 | BC-6.11.016 | SS-06 | Factory operations and dashboards skills | factory-worktree-health: 5-step sync state evaluation matrix |
| BC-AUDIT-360 | pass-3-deep-skills-batch-1.md | 1661 | BC-6.11.017 | SS-06 | Factory operations and dashboards skills | factory-worktree-health: dual-worktree check for multi-repo mode |
| BC-AUDIT-417 | pass-3-deep-skills-batch-2.md | 220 | BC-6.11.018 | SS-06 | Factory operations and dashboards skills | generate-pdf: Identity — convert markdown to 1898 & Co. branded PDF via pandoc + weasyprint |
| BC-AUDIT-418 | pass-3-deep-skills-batch-2.md | 229 | BC-6.11.019 | SS-06 | Factory operations and dashboards skills | generate-pdf: Required frontmatter fields are title, author, date |
| BC-AUDIT-419 | pass-3-deep-skills-batch-2.md | 238 | BC-6.11.020 | SS-06 | Factory operations and dashboards skills | generate-pdf: Output PDF defaults to <input>.pdf in same directory |
| BC-AUDIT-420 | pass-3-deep-skills-batch-2.md | 247 | BC-6.11.021 | SS-06 | Factory operations and dashboards skills | generate-pdf: Errors must be reported with specific solutions per error class |
| BC-AUDIT-444 | pass-3-deep-skills-batch-2.md | 494 | BC-6.11.022 | SS-06 | Factory operations and dashboards skills | jira: Identity is reference-only documentation for ankitpokhrel/jira-cli |
| BC-AUDIT-445 | pass-3-deep-skills-batch-2.md | 509 | BC-6.11.023 | SS-06 | Factory operations and dashboards skills | maintenance-sweep: Identity — periodic sweeps + cleanup PRs through standard quality gates |
| BC-AUDIT-446 | pass-3-deep-skills-batch-2.md | 518 | BC-6.11.024 | SS-06 | Factory operations and dashboards skills | maintenance-sweep: 9 sweep types run in parallel after STARTED commit |
| BC-AUDIT-447 | pass-3-deep-skills-batch-2.md | 527 | BC-6.11.025 | SS-06 | Factory operations and dashboards skills | maintenance-sweep: Dependency audit splits T3 (run scans) and T2 (analyze) |
| BC-AUDIT-448 | pass-3-deep-skills-batch-2.md | 536 | BC-6.11.026 | SS-06 | Factory operations and dashboards skills | maintenance-sweep: Performance regression — >25% triggers PR; 10-25% logs trend; <10% no action |
| BC-AUDIT-449 | pass-3-deep-skills-batch-2.md | 545 | BC-6.11.027 | SS-06 | Factory operations and dashboards skills | maintenance-sweep: Auto-PR quality gate same as Feature Mode (regression + holdout + adversarial ... |
| BC-AUDIT-450 | pass-3-deep-skills-batch-2.md | 554 | BC-6.11.028 | SS-06 | Factory operations and dashboards skills | maintenance-sweep: Output is sweep-report-YYYY-MM-DD.md plus per-sweep findings files |
| BC-AUDIT-462 | pass-3-deep-skills-batch-2.md | 682 | BC-6.11.029 | SS-06 | Factory operations and dashboards skills | model-routing: Identity — LiteLLM model tier assignment reference |
| BC-AUDIT-463 | pass-3-deep-skills-batch-2.md | 691 | BC-6.11.030 | SS-06 | Factory operations and dashboards skills | model-routing: Iron rule — Adversary MUST use adversary tier (GPT-5.4), never Claude |
| BC-AUDIT-464 | pass-3-deep-skills-batch-2.md | 700 | BC-6.11.031 | SS-06 | Factory operations and dashboards skills | model-routing: Three-tier fallback — primary → standard fallback → reasoning fallback (for ... |
| BC-AUDIT-465 | pass-3-deep-skills-batch-2.md | 709 | BC-6.11.032 | SS-06 | Factory operations and dashboards skills | model-routing: Compounding correctness — pause if budget forces downgrade in P3-P5 |
| BC-AUDIT-466 | pass-3-deep-skills-batch-2.md | 724 | BC-6.11.033 | SS-06 | Factory operations and dashboards skills | multi-repo-health: Identity — scan .worktrees/ for multi-repo layout, report repos with manifests |
| BC-AUDIT-467 | pass-3-deep-skills-batch-2.md | 733 | BC-6.11.034 | SS-06 | Factory operations and dashboards skills | multi-repo-health: Read-only — does not mutate any repo |
| BC-AUDIT-468 | pass-3-deep-skills-batch-2.md | 742 | BC-6.11.035 | SS-06 | Factory operations and dashboards skills | multi-repo-health: Single-repo path — count == 0 reports "single-repo project" and stops |
| BC-AUDIT-469 | pass-3-deep-skills-batch-2.md | 751 | BC-6.11.036 | SS-06 | Factory operations and dashboards skills | multi-repo-health: Story-repo cross-check warns when stories reference undetected repo or repo la... |
| BC-AUDIT-579 | pass-3-deep-skills-batch-2.md | 1885 | BC-6.11.037 | SS-06 | Factory operations and dashboards skills | policy-add: Identity — register new governance policy in .factory/policies.yaml with sequential ID |
| BC-AUDIT-580 | pass-3-deep-skills-batch-2.md | 1894 | BC-6.11.038 | SS-06 | Factory operations and dashboards skills | policy-add: Policy name must be snake_case and unique across registry |
| BC-AUDIT-581 | pass-3-deep-skills-batch-2.md | 1903 | BC-6.11.039 | SS-06 | Factory operations and dashboards skills | policy-add: Severity HIGH or MEDIUM only; HIGH violations block convergence |
| BC-AUDIT-582 | pass-3-deep-skills-batch-2.md | 1912 | BC-6.11.040 | SS-06 | Factory operations and dashboards skills | policy-add: Enforced_by + scope each must have ≥1 entry; custom policies must include verificat... |
| BC-AUDIT-583 | pass-3-deep-skills-batch-2.md | 1921 | BC-6.11.041 | SS-06 | Factory operations and dashboards skills | policy-add: Output appends to policies.yaml; runs validate after; reports next steps |
| BC-AUDIT-584 | pass-3-deep-skills-batch-2.md | 1930 | BC-6.11.042 | SS-06 | Factory operations and dashboards skills | policy-add: Prerequisite — policies.yaml must exist; otherwise run policy-registry init first |
| BC-AUDIT-585 | pass-3-deep-skills-batch-2.md | 1945 | BC-6.11.043 | SS-06 | Factory operations and dashboards skills | policy-registry: Identity — view/validate/manage governance policy registry |
| BC-AUDIT-586 | pass-3-deep-skills-batch-2.md | 1954 | BC-6.11.044 | SS-06 | Factory operations and dashboards skills | policy-registry: Init copies template + populates 9 baseline governance policies |
| BC-AUDIT-587 | pass-3-deep-skills-batch-2.md | 1963 | BC-6.11.045 | SS-06 | Factory operations and dashboards skills | policy-registry: Validate checks ID/name uniqueness, snake_case, severity ∈ {HIGH,MEDIUM}, lint... |
| BC-AUDIT-588 | pass-3-deep-skills-batch-2.md | 1972 | BC-6.11.046 | SS-06 | Factory operations and dashboards skills | policy-registry: List shows summary table with #/Policy/Severity/Enforced By/Lint Hook |
| BC-AUDIT-589 | pass-3-deep-skills-batch-2.md | 1981 | BC-6.11.047 | SS-06 | Factory operations and dashboards skills | policy-registry: Adversarial review auto-loads policies.yaml as rubric |
| BC-AUDIT-610 | pass-3-deep-skills-batch-3.md | 128 | BC-6.11.048 | SS-06 | Factory operations and dashboards skills | quick-dev-routing: identity & qualification gate |
| BC-AUDIT-611 | pass-3-deep-skills-batch-3.md | 137 | BC-6.11.049 | SS-06 | Factory operations and dashboards skills | quick-dev-routing: multi-goal detection precedes routing |
| BC-AUDIT-612 | pass-3-deep-skills-batch-3.md | 146 | BC-6.11.050 | SS-06 | Factory operations and dashboards skills | quick-dev-routing: compressed pipeline preserves regression + adversary + human merge |
| BC-AUDIT-613 | pass-3-deep-skills-batch-3.md | 155 | BC-6.11.051 | SS-06 | Factory operations and dashboards skills | quick-dev-routing: writes routing-decision.md and falls back on regression failure |
| BC-AUDIT-785 | pass-3-deep-skills-batch-3.md | 1843 | BC-6.11.052 | SS-06 | Factory operations and dashboards skills | worktree-manage: identity & 3 commands (create/list/cleanup) |
| BC-AUDIT-786 | pass-3-deep-skills-batch-3.md | 1852 | BC-6.11.053 | SS-06 | Factory operations and dashboards skills | worktree-manage: create produces `.worktrees/STORY-NNN/` on `feature/STORY-NNN-<desc>` |
| BC-AUDIT-787 | pass-3-deep-skills-batch-3.md | 1861 | BC-6.11.054 | SS-06 | Factory operations and dashboards skills | worktree-manage: cleanup refuses dirty + warns unmerged |
| BC-AUDIT-400 | pass-3-deep-skills-batch-2.md | 49 | BC-6.12.001 | SS-06 | Feature-mode scoping rules | feature-mode-scoping-rules: Identity is reference doc consumed by F1-F7 phase skills |
| BC-AUDIT-401 | pass-3-deep-skills-batch-2.md | 58 | BC-6.12.002 | SS-06 | Feature-mode scoping rules | feature-mode-scoping-rules: Regression scope is the FULL test suite, never scoped |
| BC-AUDIT-402 | pass-3-deep-skills-batch-2.md | 67 | BC-6.12.003 | SS-06 | Feature-mode scoping rules | feature-mode-scoping-rules: Adversarial review covers NEW + MODIFIED + DEPENDENT, never previous ... |
| BC-AUDIT-403 | pass-3-deep-skills-batch-2.md | 76 | BC-6.12.004 | SS-06 | Feature-mode scoping rules | feature-mode-scoping-rules: Scope is immutable after F1 (Rule 6) |
| BC-AUDIT-218 | pass-3-deep-skills-batch-1.md | 239 | BC-6.13.001 | SS-06 | Telemetry and analytics integration skills | analytics-integration: skill identity contract (optional, no-op when not configured) |
| BC-AUDIT-219 | pass-3-deep-skills-batch-1.md | 248 | BC-6.13.002 | SS-06 | Telemetry and analytics integration skills | analytics-integration: feature health classification thresholds |
| BC-AUDIT-220 | pass-3-deep-skills-batch-1.md | 257 | BC-6.13.003 | SS-06 | Telemetry and analytics integration skills | analytics-integration: error severity classification |
| BC-AUDIT-221 | pass-3-deep-skills-batch-1.md | 266 | BC-6.13.004 | SS-06 | Telemetry and analytics integration skills | analytics-integration: digest output path and quality gate |
| BC-AUDIT-247 | pass-3-deep-skills-batch-1.md | 524 | BC-6.13.005 | SS-06 | Telemetry and analytics integration skills | claude-telemetry: skill identity (manage 5 OTEL_* env vars) |
| BC-AUDIT-248 | pass-3-deep-skills-batch-1.md | 533 | BC-6.13.006 | SS-06 | Telemetry and analytics integration skills | claude-telemetry: 3 modes (on/off/status) |
| BC-AUDIT-249 | pass-3-deep-skills-batch-1.md | 542 | BC-6.13.007 | SS-06 | Telemetry and analytics integration skills | claude-telemetry: prunes legacy temporality key |
| BC-AUDIT-250 | pass-3-deep-skills-batch-1.md | 551 | BC-6.13.008 | SS-06 | Telemetry and analytics integration skills | claude-telemetry: prominent restart reminder after `on` |
| BC-AUDIT-484 | pass-3-deep-skills-batch-2.md | 910 | BC-6.13.009 | SS-06 | Telemetry and analytics integration skills | onboard-observability: Identity — registers project + writes Claude OTel env vars; idempotent |
| BC-AUDIT-485 | pass-3-deep-skills-batch-2.md | 919 | BC-6.13.010 | SS-06 | Telemetry and analytics integration skills | onboard-observability: Required announce-at-start verbatim message |
| BC-AUDIT-486 | pass-3-deep-skills-batch-2.md | 928 | BC-6.13.011 | SS-06 | Telemetry and analytics integration skills | onboard-observability: Aborts if no .factory/ ancestor or factory-obs binary missing |
| BC-AUDIT-487 | pass-3-deep-skills-batch-2.md | 937 | BC-6.13.012 | SS-06 | Telemetry and analytics integration skills | onboard-observability: Writes exactly 5 OTEL_* env vars; preserves all other keys |
| BC-AUDIT-488 | pass-3-deep-skills-batch-2.md | 946 | BC-6.13.013 | SS-06 | Telemetry and analytics integration skills | onboard-observability: Idempotency — register dedupes on absolute path; jq merge overwrites onl... |
| BC-AUDIT-489 | pass-3-deep-skills-batch-2.md | 955 | BC-6.13.014 | SS-06 | Telemetry and analytics integration skills | onboard-observability: Non-goals — does not start/stop Docker stack, does not unregister, does ... |
| BC-AUDIT-222 | pass-3-deep-skills-batch-1.md | 279 | BC-6.14.001 | SS-06 | Artifact and verification skills | artifact-detection: skill identity contract (universal pipeline front-end) |
| BC-AUDIT-223 | pass-3-deep-skills-batch-1.md | 288 | BC-6.14.002 | SS-06 | Artifact and verification skills | artifact-detection: 5-tier readiness classification (L0-L4) |
| BC-AUDIT-224 | pass-3-deep-skills-batch-1.md | 297 | BC-6.14.003 | SS-06 | Artifact and verification skills | artifact-detection: format detection flags FR-NNN legacy for migration |
| BC-AUDIT-225 | pass-3-deep-skills-batch-1.md | 306 | BC-6.14.004 | SS-06 | Artifact and verification skills | artifact-detection: writes 3 routing artifacts |
| BC-AUDIT-226 | pass-3-deep-skills-batch-1.md | 315 | BC-6.14.005 | SS-06 | Artifact and verification skills | artifact-detection: failure modes (no .factory/, corruption, legacy) |
| BC-AUDIT-410 | pass-3-deep-skills-batch-2.md | 151 | BC-6.14.006 | SS-06 | Artifact and verification skills | formal-verify: Identity — Phase 6 quality gate runs Kani + cargo-fuzz + cargo-mutants + semgrep |
| BC-AUDIT-411 | pass-3-deep-skills-batch-2.md | 160 | BC-6.14.007 | SS-06 | Artifact and verification skills | formal-verify: Iron Law — every VP needs passing proof + saturated fuzz + meeting kill rate |
| BC-AUDIT-412 | pass-3-deep-skills-batch-2.md | 169 | BC-6.14.008 | SS-06 | Artifact and verification skills | formal-verify: Fuzz saturation requires ≥5 minutes per target with stable coverage |
| BC-AUDIT-413 | pass-3-deep-skills-batch-2.md | 178 | BC-6.14.009 | SS-06 | Artifact and verification skills | formal-verify: Mutation kill rate target is ≥90% |
| BC-AUDIT-414 | pass-3-deep-skills-batch-2.md | 187 | BC-6.14.010 | SS-06 | Artifact and verification skills | formal-verify: Security scan clean = zero CRITICAL and zero HIGH |
| BC-AUDIT-415 | pass-3-deep-skills-batch-2.md | 196 | BC-6.14.011 | SS-06 | Artifact and verification skills | formal-verify: Output is formal-verification-report.md at .factory/cycles/<current>/ |
| BC-AUDIT-416 | pass-3-deep-skills-batch-2.md | 205 | BC-6.14.012 | SS-06 | Artifact and verification skills | formal-verify: Missing tools must be reported, never silently skipped |
| BC-AUDIT-490 | pass-3-deep-skills-batch-2.md | 970 | BC-6.14.013 | SS-06 | Artifact and verification skills | perf-check: Identity — bench regression + resource profiling + budget compliance |
| BC-AUDIT-491 | pass-3-deep-skills-batch-2.md | 979 | BC-6.14.014 | SS-06 | Artifact and verification skills | perf-check: Regression threshold — flag > 10% vs baseline |
| BC-AUDIT-492 | pass-3-deep-skills-batch-2.md | 988 | BC-6.14.015 | SS-06 | Artifact and verification skills | perf-check: Default budgets — startup <100ms; binary <50MB; debug build <60s; tests <120s |
| BC-AUDIT-493 | pass-3-deep-skills-batch-2.md | 997 | BC-6.14.016 | SS-06 | Artifact and verification skills | perf-check: Output is performance-report.md with PASS\|WARN\|FAIL gate |
| BC-AUDIT-494 | pass-3-deep-skills-batch-2.md | 1006 | BC-6.14.017 | SS-06 | Artifact and verification skills | perf-check: No benchmarks → report and recommend creating them |
| BC-AUDIT-624 | pass-3-deep-skills-batch-3.md | 266 | BC-6.14.018 | SS-06 | Artifact and verification skills | register-artifact: identity & justification |
| BC-AUDIT-625 | pass-3-deep-skills-batch-3.md | 275 | BC-6.14.019 | SS-06 | Artifact and verification skills | register-artifact: type identification by path pattern (4-row table) |
| BC-AUDIT-626 | pass-3-deep-skills-batch-3.md | 284 | BC-6.14.020 | SS-06 | Artifact and verification skills | register-artifact: idempotent (refuses duplicate ID) |
| BC-AUDIT-627 | pass-3-deep-skills-batch-3.md | 293 | BC-6.14.021 | SS-06 | Artifact and verification skills | register-artifact: refuses to create INDEX file (separation of concerns) |
| BC-AUDIT-628 | pass-3-deep-skills-batch-3.md | 302 | BC-6.14.022 | SS-06 | Artifact and verification skills | register-artifact: batch mode aggregates results |
| BC-AUDIT-713 | pass-3-deep-skills-batch-3.md | 1135 | BC-6.14.023 | SS-06 | Artifact and verification skills | systematic-debugging: identity + hard gate |
| BC-AUDIT-714 | pass-3-deep-skills-batch-3.md | 1144 | BC-6.14.024 | SS-06 | Artifact and verification skills | systematic-debugging: 4-phase sequence (root cause → pattern → hypothesis → implementation) |
| BC-AUDIT-715 | pass-3-deep-skills-batch-3.md | 1153 | BC-6.14.025 | SS-06 | Artifact and verification skills | systematic-debugging: Phase 4.5 — 3+ failed fixes = STOP and escalate |
| BC-AUDIT-716 | pass-3-deep-skills-batch-3.md | 1162 | BC-6.14.026 | SS-06 | Artifact and verification skills | systematic-debugging: 8-row red-flag rationalization table |
| BC-AUDIT-717 | pass-3-deep-skills-batch-3.md | 1171 | BC-6.14.027 | SS-06 | Artifact and verification skills | systematic-debugging: BC-aware mode + status-protocol reporting |
| BC-AUDIT-730 | pass-3-deep-skills-batch-3.md | 1300 | BC-6.14.028 | SS-06 | Artifact and verification skills | track-debt: identity & three commands (add/list/resolve) |
| BC-AUDIT-731 | pass-3-deep-skills-batch-3.md | 1309 | BC-6.14.029 | SS-06 | Artifact and verification skills | track-debt: add assigns next TD-NNN with full metadata |
| BC-AUDIT-732 | pass-3-deep-skills-batch-3.md | 1318 | BC-6.14.030 | SS-06 | Artifact and verification skills | track-debt: register format (Active vs Resolved sections) |
| BC-AUDIT-733 | pass-3-deep-skills-batch-3.md | 1327 | BC-6.14.031 | SS-06 | Artifact and verification skills | track-debt: when-to-add catalogue (6 sources) |
| BC-AUDIT-227 | pass-3-deep-skills-batch-1.md | 328 | BC-6.15.001 | SS-06 | Brainstorming and writing skills | brainstorming: skill identity + hard gate |
| BC-AUDIT-228 | pass-3-deep-skills-batch-1.md | 337 | BC-6.15.002 | SS-06 | Brainstorming and writing skills | brainstorming: 6 named techniques and selection logic |
| BC-AUDIT-229 | pass-3-deep-skills-batch-1.md | 346 | BC-6.15.003 | SS-06 | Brainstorming and writing skills | brainstorming: every idea goes through process even when "obvious" |
| BC-AUDIT-230 | pass-3-deep-skills-batch-1.md | 355 | BC-6.15.004 | SS-06 | Brainstorming and writing skills | brainstorming: report output and quality gate |
| BC-AUDIT-788 | pass-3-deep-skills-batch-3.md | 1874 | BC-6.15.005 | SS-06 | Brainstorming and writing skills | writing-skills: identity & TDD-for-skills mapping |
| BC-AUDIT-789 | pass-3-deep-skills-batch-3.md | 1883 | BC-6.15.006 | SS-06 | Brainstorming and writing skills | writing-skills: hard gate (NO SKILL WITHOUT FAILING TEST FIRST) |
| BC-AUDIT-790 | pass-3-deep-skills-batch-3.md | 1892 | BC-6.15.007 | SS-06 | Brainstorming and writing skills | writing-skills: when-to-create vs when-not catalogues |
| BC-AUDIT-791 | pass-3-deep-skills-batch-3.md | 1901 | BC-6.15.008 | SS-06 | Brainstorming and writing skills | writing-skills: SKILL.md structure with 6 mandatory sections |
| BC-AUDIT-792 | pass-3-deep-skills-batch-3.md | 1910 | BC-6.15.009 | SS-06 | Brainstorming and writing skills | writing-skills: CSO description rules (Use when… , no workflow summary, <500 chars) |
| BC-AUDIT-793 | pass-3-deep-skills-batch-3.md | 1919 | BC-6.15.010 | SS-06 | Brainstorming and writing skills | writing-skills: red-green-refactor cycle for skills + bulletproofing table |
| BC-AUDIT-794 | pass-3-deep-skills-batch-3.md | 1928 | BC-6.15.011 | SS-06 | Brainstorming and writing skills | writing-skills: vsdd-factory conventions + checklist |
| BC-AUDIT-457 | pass-3-deep-skills-batch-2.md | 630 | BC-6.16.001 | SS-06 | Phase orchestration and mode skills | mode-decision-guide: Identity — reference doc for orchestrator mode detection |
| BC-AUDIT-458 | pass-3-deep-skills-batch-2.md | 639 | BC-6.16.002 | SS-06 | Phase orchestration and mode skills | mode-decision-guide: Feature Mode threshold — <30% files changed AND <50% components AND ≤2 c... |
| BC-AUDIT-459 | pass-3-deep-skills-batch-2.md | 648 | BC-6.16.003 | SS-06 | Phase orchestration and mode skills | mode-decision-guide: Greenfield switchover — ≥30% files OR ≥50% components OR breaking inte... |
| BC-AUDIT-460 | pass-3-deep-skills-batch-2.md | 657 | BC-6.16.004 | SS-06 | Phase orchestration and mode skills | mode-decision-guide: Bug fix minimal route — F1 → F4 → F5 → F7, skip F2/F3 |
| BC-AUDIT-461 | pass-3-deep-skills-batch-2.md | 666 | BC-6.16.005 | SS-06 | Phase orchestration and mode skills | mode-decision-guide: Human override always wins over auto-detection |
| BC-AUDIT-480 | pass-3-deep-skills-batch-2.md | 868 | BC-6.16.006 | SS-06 | Phase orchestration and mode skills | next-step: Identity — read STATE.md and propose next workflow step, do not execute |
| BC-AUDIT-481 | pass-3-deep-skills-batch-2.md | 877 | BC-6.16.007 | SS-06 | Phase orchestration and mode skills | next-step: STATE.md missing → directs user to factory-health and stops |
| BC-AUDIT-482 | pass-3-deep-skills-batch-2.md | 886 | BC-6.16.008 | SS-06 | Phase orchestration and mode skills | next-step: Does not execute — proposal only |
| BC-AUDIT-483 | pass-3-deep-skills-batch-2.md | 895 | BC-6.16.009 | SS-06 | Phase orchestration and mode skills | next-step: Uses lobster-parse to enumerate workflow steps with dependencies |
| BC-AUDIT-495 | pass-3-deep-skills-batch-2.md | 1021 | BC-6.16.010 | SS-06 | Phase orchestration and mode skills | phase-0-codebase-ingestion: Identity — Phase 0 entry point delegating to brownfield-ingest sub-... |
| BC-AUDIT-496 | pass-3-deep-skills-batch-2.md | 1030 | BC-6.16.011 | SS-06 | Phase orchestration and mode skills | phase-0-codebase-ingestion: Gate — context doc, criticality, BCs (origin: recovered), coverage ... |
| BC-AUDIT-497 | pass-3-deep-skills-batch-2.md | 1039 | BC-6.16.012 | SS-06 | Phase orchestration and mode skills | phase-0-codebase-ingestion: Direct work skill is /vsdd-factory:brownfield-ingest <path> |
| BC-AUDIT-498 | pass-3-deep-skills-batch-2.md | 1054 | BC-6.16.013 | SS-06 | Phase orchestration and mode skills | phase-1-prd-revision: Identity — PO revises PRD per architect feasibility report; max 3 iterations |
| BC-AUDIT-499 | pass-3-deep-skills-batch-2.md | 1063 | BC-6.16.014 | SS-06 | Phase orchestration and mode skills | phase-1-prd-revision: Skip when feasibility report says "validated — no issues" |
| BC-AUDIT-500 | pass-3-deep-skills-batch-2.md | 1072 | BC-6.16.015 | SS-06 | Phase orchestration and mode skills | phase-1-prd-revision: 3-round deadlock escalates to human with both positions |
| BC-AUDIT-501 | pass-3-deep-skills-batch-2.md | 1081 | BC-6.16.016 | SS-06 | Phase orchestration and mode skills | phase-1-prd-revision: Quality Gate — every concern addressed or contested with rationale |
| BC-AUDIT-502 | pass-3-deep-skills-batch-2.md | 1096 | BC-6.16.017 | SS-06 | Phase orchestration and mode skills | phase-1-spec-crystallization: Identity — Phase 1 entry point spanning brief → architecture |
| BC-AUDIT-503 | pass-3-deep-skills-batch-2.md | 1105 | BC-6.16.018 | SS-06 | Phase orchestration and mode skills | phase-1-spec-crystallization: Gate — IDs unique, VPs cover security boundaries, purity map comp... |
| BC-AUDIT-504 | pass-3-deep-skills-batch-2.md | 1114 | BC-6.16.019 | SS-06 | Phase orchestration and mode skills | phase-1-spec-crystallization: Sub-workflow is workflows/phases/phase-1-spec-crystallization.lobster |
| BC-AUDIT-505 | pass-3-deep-skills-batch-2.md | 1129 | BC-6.16.020 | SS-06 | Phase orchestration and mode skills | phase-1d-adversarial-spec-review: Identity — adversary reviews spec package with fresh context |
| BC-AUDIT-506 | pass-3-deep-skills-batch-2.md | 1138 | BC-6.16.021 | SS-06 | Phase orchestration and mode skills | phase-1d-adversarial-spec-review: Adversary reviews 7 categories — ambiguity, missing edges, im... |
| BC-AUDIT-507 | pass-3-deep-skills-batch-2.md | 1147 | BC-6.16.022 | SS-06 | Phase orchestration and mode skills | phase-1d-adversarial-spec-review: Findings triaged C/H/M/L; cross-doc sync check before re-review |
| BC-AUDIT-508 | pass-3-deep-skills-batch-2.md | 1156 | BC-6.16.023 | SS-06 | Phase orchestration and mode skills | phase-1d-adversarial-spec-review: Convergence — adversary reports "CONVERGENCE REACHED — find... |
| BC-AUDIT-509 | pass-3-deep-skills-batch-2.md | 1165 | BC-6.16.024 | SS-06 | Phase orchestration and mode skills | phase-1d-adversarial-spec-review: Quality Gate — different model family + fresh context every p... |
| BC-AUDIT-510 | pass-3-deep-skills-batch-2.md | 1180 | BC-6.16.025 | SS-06 | Phase orchestration and mode skills | phase-2-story-decomposition: Identity — Phase 2 entry point delegating to decompose-stories sub... |
| BC-AUDIT-511 | pass-3-deep-skills-batch-2.md | 1189 | BC-6.16.026 | SS-06 | Phase orchestration and mode skills | phase-2-story-decomposition: Gate — every BC traces to ≥1 story, no placeholder ACs, no cycle... |
| BC-AUDIT-512 | pass-3-deep-skills-batch-2.md | 1198 | BC-6.16.027 | SS-06 | Phase orchestration and mode skills | phase-2-story-decomposition: Direct command is /vsdd-factory:decompose-stories |
| BC-AUDIT-513 | pass-3-deep-skills-batch-2.md | 1213 | BC-6.16.028 | SS-06 | Phase orchestration and mode skills | phase-3-tdd-implementation: Identity — per-story TDD delivery via deliver-story sub-workflow |
| BC-AUDIT-514 | pass-3-deep-skills-batch-2.md | 1222 | BC-6.16.029 | SS-06 | Phase orchestration and mode skills | phase-3-tdd-implementation: Gate — Red Gate passed, all tests pass, demos cover ACs, PR merged,... |
| BC-AUDIT-515 | pass-3-deep-skills-batch-2.md | 1231 | BC-6.16.030 | SS-06 | Phase orchestration and mode skills | phase-3-tdd-implementation: Prerequisites — Phase 2 complete, story status `ready`, all depende... |
| BC-AUDIT-519 | pass-3-deep-skills-batch-2.md | 1279 | BC-6.16.031 | SS-06 | Phase orchestration and mode skills | phase-5-adversarial-refinement: Identity — multi-model adversarial loop until novelty=0 |
| BC-AUDIT-520 | pass-3-deep-skills-batch-2.md | 1288 | BC-6.16.032 | SS-06 | Phase orchestration and mode skills | phase-5-adversarial-refinement: Gate — novelty=0, all findings addressed/accepted, ≥3 clean p... |
| BC-AUDIT-521 | pass-3-deep-skills-batch-2.md | 1297 | BC-6.16.033 | SS-06 | Phase orchestration and mode skills | phase-5-adversarial-refinement: Direct command is /vsdd-factory:adversarial-review implementation |
| BC-AUDIT-522 | pass-3-deep-skills-batch-2.md | 1312 | BC-6.16.034 | SS-06 | Phase orchestration and mode skills | phase-6-formal-hardening: Identity — Phase 6 entry point applying 4 verification techniques |
| BC-AUDIT-523 | pass-3-deep-skills-batch-2.md | 1321 | BC-6.16.035 | SS-06 | Phase orchestration and mode skills | phase-6-formal-hardening: Gate — all proofs pass, fuzz 5min/target zero crashes, mutation >90%,... |
| BC-AUDIT-524 | pass-3-deep-skills-batch-2.md | 1330 | BC-6.16.036 | SS-06 | Phase orchestration and mode skills | phase-6-formal-hardening: Prerequisites — cargo-kani, cargo-fuzz, cargo-mutants, semgrep installed |
| BC-AUDIT-525 | pass-3-deep-skills-batch-2.md | 1345 | BC-6.16.037 | SS-06 | Phase orchestration and mode skills | phase-7-convergence: Identity — 7-dimensional convergence assessment |
| BC-AUDIT-526 | pass-3-deep-skills-batch-2.md | 1354 | BC-6.16.038 | SS-06 | Phase orchestration and mode skills | phase-7-convergence: Gate — all 7 dimensions CONVERGED, traceability matrix, demo verified by v... |
| BC-AUDIT-527 | pass-3-deep-skills-batch-2.md | 1363 | BC-6.16.039 | SS-06 | Phase orchestration and mode skills | phase-7-convergence: Outcome — CONVERGED → release; NOT CONVERGED → loop back to Phase 3 |
| BC-AUDIT-658 | pass-3-deep-skills-batch-3.md | 596 | BC-6.16.040 | SS-06 | Phase orchestration and mode skills | run-phase: identity & resolution rules |
| BC-AUDIT-659 | pass-3-deep-skills-batch-3.md | 605 | BC-6.16.041 | SS-06 | Phase orchestration and mode skills | run-phase: validates workflow before execution |
| BC-AUDIT-660 | pass-3-deep-skills-batch-3.md | 614 | BC-6.16.042 | SS-06 | Phase orchestration and mode skills | run-phase: topological execution honors depends_on |
| BC-AUDIT-661 | pass-3-deep-skills-batch-3.md | 623 | BC-6.16.043 | SS-06 | Phase orchestration and mode skills | run-phase: STATE.md update + final summary |
| BC-AUDIT-528 | pass-3-deep-skills-batch-2.md | 1378 | BC-6.17.001 | SS-06 | Feature-mode phase skills (f1-f7) | phase-f1-delta-analysis: Identity — analyzes feature request against existing artifacts to dete... |
| BC-AUDIT-529 | pass-3-deep-skills-batch-2.md | 1387 | BC-6.17.002 | SS-06 | Feature-mode phase skills (f1-f7) | phase-f1-delta-analysis: Components classified NEW/MODIFIED/DEPENDENT by architect |
| BC-AUDIT-530 | pass-3-deep-skills-batch-2.md | 1396 | BC-6.17.003 | SS-06 | Feature-mode phase skills (f1-f7) | phase-f1-delta-analysis: Intent classification — feature, enhancement, bug-fix maps to F1-F7 vs... |
| BC-AUDIT-531 | pass-3-deep-skills-batch-2.md | 1405 | BC-6.17.004 | SS-06 | Feature-mode phase skills (f1-f7) | phase-f1-delta-analysis: Trivial scope — single module + no new BCs + no arch change + no new d... |
| BC-AUDIT-532 | pass-3-deep-skills-batch-2.md | 1414 | BC-6.17.005 | SS-06 | Feature-mode phase skills (f1-f7) | phase-f1-delta-analysis: Severity (bug-fix only) — CRITICAL triggers expedited flow with skippe... |
| BC-AUDIT-533 | pass-3-deep-skills-batch-2.md | 1423 | BC-6.17.006 | SS-06 | Feature-mode phase skills (f1-f7) | phase-f1-delta-analysis: Output is delta-analysis.md + affected-files.txt (+ affected-repos.txt f... |
| BC-AUDIT-534 | pass-3-deep-skills-batch-2.md | 1432 | BC-6.17.007 | SS-06 | Feature-mode phase skills (f1-f7) | phase-f1-delta-analysis: Quality Gate — feature_type, intent, scope, severity (if bug-fix), BC-... |
| BC-AUDIT-535 | pass-3-deep-skills-batch-2.md | 1447 | BC-6.17.008 | SS-06 | Feature-mode phase skills (f1-f7) | phase-f2-spec-evolution: Identity — incremental spec updates (PRD + arch + VPs), delta only |
| BC-AUDIT-536 | pass-3-deep-skills-batch-2.md | 1456 | BC-6.17.009 | SS-06 | Feature-mode phase skills (f1-f7) | phase-f2-spec-evolution: PRD delta appends new BCs continuing BC-S.SS.NNN sequence; modified BCs ... |
| BC-AUDIT-537 | pass-3-deep-skills-batch-2.md | 1465 | BC-6.17.010 | SS-06 | Feature-mode phase skills (f1-f7) | phase-f2-spec-evolution: UX delta + accessibility review run only when feature_type ∈ ['ui', 'f... |
| BC-AUDIT-538 | pass-3-deep-skills-batch-2.md | 1474 | BC-6.17.011 | SS-06 | Feature-mode phase skills (f1-f7) | phase-f2-spec-evolution: Spec version bump per semver — MAJOR/MINOR/PATCH |
| BC-AUDIT-539 | pass-3-deep-skills-batch-2.md | 1483 | BC-6.17.012 | SS-06 | Feature-mode phase skills (f1-f7) | phase-f2-spec-evolution: Adversary reviews ONLY the delta (PRD + arch + VP + UX), not unchanged s... |
| BC-AUDIT-540 | pass-3-deep-skills-batch-2.md | 1492 | BC-6.17.013 | SS-06 | Feature-mode phase skills (f1-f7) | phase-f2-spec-evolution: Quality Gate — BC-S.SS.NNN format, append-only, acyclic deps, version ... |
| BC-AUDIT-541 | pass-3-deep-skills-batch-2.md | 1501 | BC-6.17.014 | SS-06 | Feature-mode phase skills (f1-f7) | phase-f2-spec-evolution: Failure — missing F1 output halts F2; CRITICAL after 3 rounds escalate... |
| BC-AUDIT-542 | pass-3-deep-skills-batch-2.md | 1516 | BC-6.17.015 | SS-06 | Feature-mode phase skills (f1-f7) | phase-f3-incremental-stories: Identity — adds new stories integrated into existing dependency g... |
| BC-AUDIT-543 | pass-3-deep-skills-batch-2.md | 1525 | BC-6.17.016 | SS-06 | Feature-mode phase skills (f1-f7) | phase-f3-incremental-stories: Story IDs continue existing sequence; per-file STORY-NNN.md, not mo... |
| BC-AUDIT-544 | pass-3-deep-skills-batch-2.md | 1534 | BC-6.17.017 | SS-06 | Feature-mode phase skills (f1-f7) | phase-f3-incremental-stories: Cycle detection via Kahn's algorithm topological sort |
| BC-AUDIT-545 | pass-3-deep-skills-batch-2.md | 1543 | BC-6.17.018 | SS-06 | Feature-mode phase skills (f1-f7) | phase-f3-incremental-stories: DTU clones stories placed in Wave 1; gene transfusion stories flagged |
| BC-AUDIT-546 | pass-3-deep-skills-batch-2.md | 1552 | BC-6.17.019 | SS-06 | Feature-mode phase skills (f1-f7) | phase-f3-incremental-stories: Quality Gate — IDs continue, per-file, BC-S.SS.NNN, testable AC, ... |
| BC-AUDIT-547 | pass-3-deep-skills-batch-2.md | 1567 | BC-6.17.020 | SS-06 | Feature-mode phase skills (f1-f7) | phase-f4-delta-implementation: Identity — TDD scoped to new stories with full regression as saf... |
| BC-AUDIT-548 | pass-3-deep-skills-batch-2.md | 1576 | BC-6.17.021 | SS-06 | Feature-mode phase skills (f1-f7) | phase-f4-delta-implementation: Establish regression baseline before any new code; if any fail, STOP |
| BC-AUDIT-549 | pass-3-deep-skills-batch-2.md | 1585 | BC-6.17.022 | SS-06 | Feature-mode phase skills (f1-f7) | phase-f4-delta-implementation: Per-story delivery uses code-delivery.lobster sub-workflow with 11... |
| BC-AUDIT-550 | pass-3-deep-skills-batch-2.md | 1594 | BC-6.17.023 | SS-06 | Feature-mode phase skills (f1-f7) | phase-f4-delta-implementation: Wave Integration Gate — full tests + adversary + security + hold... |
| BC-AUDIT-551 | pass-3-deep-skills-batch-2.md | 1603 | BC-6.17.024 | SS-06 | Feature-mode phase skills (f1-f7) | phase-f4-delta-implementation: Regression failure — fix the implementation, not the test |
| BC-AUDIT-552 | pass-3-deep-skills-batch-2.md | 1612 | BC-6.17.025 | SS-06 | Feature-mode phase skills (f1-f7) | phase-f4-delta-implementation: Quality Gate — regression baseline + Two-Step Red Gate + full re... |
| BC-AUDIT-553 | pass-3-deep-skills-batch-2.md | 1621 | BC-6.17.026 | SS-06 | Feature-mode phase skills (f1-f7) | phase-f4-delta-implementation: No human gate — automated quality gate |
| BC-AUDIT-554 | pass-3-deep-skills-batch-2.md | 1636 | BC-6.17.027 | SS-06 | Feature-mode phase skills (f1-f7) | phase-f5-scoped-adversarial: Identity — adversary reviews only delta files, fresh context, diff... |
| BC-AUDIT-555 | pass-3-deep-skills-batch-2.md | 1645 | BC-6.17.028 | SS-06 | Feature-mode phase skills (f1-f7) | phase-f5-scoped-adversarial: Review package excludes prior reviews, implementation rationale, sem... |
| BC-AUDIT-556 | pass-3-deep-skills-batch-2.md | 1654 | BC-6.17.029 | SS-06 | Feature-mode phase skills (f1-f7) | phase-f5-scoped-adversarial: 5 review categories — spec fidelity, regression risk, convention, ... |
| BC-AUDIT-557 | pass-3-deep-skills-batch-2.md | 1663 | BC-6.17.030 | SS-06 | Feature-mode phase skills (f1-f7) | phase-f5-scoped-adversarial: Severity scale CRITICAL/HIGH/MEDIUM/LOW/COSMETIC; convergence at nov... |
| BC-AUDIT-558 | pass-3-deep-skills-batch-2.md | 1672 | BC-6.17.031 | SS-06 | Feature-mode phase skills (f1-f7) | phase-f5-scoped-adversarial: Secondary review (Gemini/review-tier) optional for security-critical... |
| BC-AUDIT-559 | pass-3-deep-skills-batch-2.md | 1681 | BC-6.17.032 | SS-06 | Feature-mode phase skills (f1-f7) | phase-f5-scoped-adversarial: Output convergence-summary.md; F5 fixes through code-delivery.lobste... |
| BC-AUDIT-560 | pass-3-deep-skills-batch-2.md | 1690 | BC-6.17.033 | SS-06 | Feature-mode phase skills (f1-f7) | phase-f5-scoped-adversarial: Quality Gate — delta scope only, fresh context, different model fa... |
| BC-AUDIT-561 | pass-3-deep-skills-batch-2.md | 1705 | BC-6.17.034 | SS-06 | Feature-mode phase skills (f1-f7) | phase-f6-targeted-hardening: Identity — Kani+fuzz+mutation scoped to delta; regression+security... |
| BC-AUDIT-562 | pass-3-deep-skills-batch-2.md | 1714 | BC-6.17.035 | SS-06 | Feature-mode phase skills (f1-f7) | phase-f6-targeted-hardening: Hardening scope per-tool varies — Kani/fuzz/mutation/Semgrep delta... |
| BC-AUDIT-563 | pass-3-deep-skills-batch-2.md | 1723 | BC-6.17.036 | SS-06 | Feature-mode phase skills (f1-f7) | phase-f6-targeted-hardening: Mutation kill rate ≥90% on changed files (≥95% for security-crit... |
| BC-AUDIT-564 | pass-3-deep-skills-batch-2.md | 1732 | BC-6.17.037 | SS-06 | Feature-mode phase skills (f1-f7) | phase-f6-targeted-hardening: Information asymmetry wall — formal-verifier cannot see F5 adversa... |
| BC-AUDIT-565 | pass-3-deep-skills-batch-2.md | 1741 | BC-6.17.038 | SS-06 | Feature-mode phase skills (f1-f7) | phase-f6-targeted-hardening: Quality Gate — proofs pass, fuzz clean, mutation 90% (95% critical... |
| BC-AUDIT-566 | pass-3-deep-skills-batch-2.md | 1756 | BC-6.17.039 | SS-06 | Feature-mode phase skills (f1-f7) | phase-f7-delta-convergence: Identity — 5-dimensional convergence on delta + full regression val... |
| BC-AUDIT-567 | pass-3-deep-skills-batch-2.md | 1765 | BC-6.17.040 | SS-06 | Feature-mode phase skills (f1-f7) | phase-f7-delta-convergence: 5 dimensions — Spec novelty<0.15, Test mutation≥90%, Impl verific... |
| BC-AUDIT-568 | pass-3-deep-skills-batch-2.md | 1774 | BC-6.17.041 | SS-06 | Feature-mode phase skills (f1-f7) | phase-f7-delta-convergence: Regression validation is binary pass/fail, not "convergence" |
| BC-AUDIT-569 | pass-3-deep-skills-batch-2.md | 1783 | BC-6.17.042 | SS-06 | Feature-mode phase skills (f1-f7) | phase-f7-delta-convergence: Cost-benefit — flag MAXIMUM_VIABLE_REFINEMENT_REACHED if cost > P(f... |
| BC-AUDIT-570 | pass-3-deep-skills-batch-2.md | 1792 | BC-6.17.043 | SS-06 | Feature-mode phase skills (f1-f7) | phase-f7-delta-convergence: Traceability chain extended (append, not replace) with new BC→VP→... |
| BC-AUDIT-571 | pass-3-deep-skills-batch-2.md | 1801 | BC-6.17.044 | SS-06 | Feature-mode phase skills (f1-f7) | phase-f7-delta-convergence: Final human authorization gate; failure routes to specific phase |
| BC-AUDIT-572 | pass-3-deep-skills-batch-2.md | 1810 | BC-6.17.045 | SS-06 | Feature-mode phase skills (f1-f7) | phase-f7-delta-convergence: Quality Gate — all 5 dims pass, regression passes, traceability ext... |
| BC-AUDIT-573 | pass-3-deep-skills-batch-2.md | 1819 | BC-6.17.046 | SS-06 | Feature-mode phase skills (f1-f7) | phase-f7-delta-convergence: Max 5 convergence cycles before cost-benefit escalation |
| BC-AUDIT-600 | pass-3-deep-skills-batch-3.md | 30 | BC-6.18.001 | SS-06 | PR and release skills | pr-create: identity & invocation contract |
| BC-AUDIT-601 | pass-3-deep-skills-batch-3.md | 39 | BC-6.18.002 | SS-06 | PR and release skills | pr-create: gathers story context before generating body |
| BC-AUDIT-602 | pass-3-deep-skills-batch-3.md | 48 | BC-6.18.003 | SS-06 | PR and release skills | pr-create: PR body must follow templated structure with mermaid + traceability table |
| BC-AUDIT-603 | pass-3-deep-skills-batch-3.md | 57 | BC-6.18.004 | SS-06 | PR and release skills | pr-create: PR creation targets develop with feat-prefixed title |
| BC-AUDIT-604 | pass-3-deep-skills-batch-3.md | 66 | BC-6.18.005 | SS-06 | PR and release skills | pr-create: post-creation report includes URL + next steps |
| BC-AUDIT-605 | pass-3-deep-skills-batch-3.md | 79 | BC-6.18.006 | SS-06 | PR and release skills | pr-review-triage: identity & dispatch role |
| BC-AUDIT-606 | pass-3-deep-skills-batch-3.md | 88 | BC-6.18.007 | SS-06 | PR and release skills | pr-review-triage: classification table is complete and exhaustive |
| BC-AUDIT-607 | pass-3-deep-skills-batch-3.md | 97 | BC-6.18.008 | SS-06 | PR and release skills | pr-review-triage: size-blocking finding STOPS pr-manager |
| BC-AUDIT-608 | pass-3-deep-skills-batch-3.md | 106 | BC-6.18.009 | SS-06 | PR and release skills | pr-review-triage: ten-cycle escalation cap |
| BC-AUDIT-609 | pass-3-deep-skills-batch-3.md | 115 | BC-6.18.010 | SS-06 | PR and release skills | pr-review-triage: writes review-findings.md with cycle row + triage table |
| BC-AUDIT-629 | pass-3-deep-skills-batch-3.md | 315 | BC-6.18.011 | SS-06 | PR and release skills | release: identity, modes, factory worktree pre-flight |
| BC-AUDIT-630 | pass-3-deep-skills-batch-3.md | 324 | BC-6.18.012 | SS-06 | PR and release skills | release: announces protocol verbatim |
| BC-AUDIT-631 | pass-3-deep-skills-batch-3.md | 333 | BC-6.18.013 | SS-06 | PR and release skills | release: bootstrap detects markers across five project types |
| BC-AUDIT-632 | pass-3-deep-skills-batch-3.md | 342 | BC-6.18.014 | SS-06 | PR and release skills | release: version bump determined from story types when no explicit version |
| BC-AUDIT-633 | pass-3-deep-skills-batch-3.md | 351 | BC-6.18.015 | SS-06 | PR and release skills | release: quality-gate modes (standard/vsdd-partial/vsdd-full) |
| BC-AUDIT-634 | pass-3-deep-skills-batch-3.md | 360 | BC-6.18.016 | SS-06 | PR and release skills | release: per-format version-bump dispatch (json/toml/yaml/regex) |
| BC-AUDIT-635 | pass-3-deep-skills-batch-3.md | 369 | BC-6.18.017 | SS-06 | PR and release skills | release: tag, push (with confirm), CI watch, gh-release verify |
| BC-AUDIT-636 | pass-3-deep-skills-batch-3.md | 378 | BC-6.18.018 | SS-06 | PR and release skills | release: dry-run produces complete plan with no side effects |
| BC-AUDIT-637 | pass-3-deep-skills-batch-3.md | 387 | BC-6.18.019 | SS-06 | PR and release skills | release: error-handling catalog |
| BC-AUDIT-638 | pass-3-deep-skills-batch-3.md | 400 | BC-6.19.001 | SS-06 | Onboarding and setup skills | repo-initialization: identity & delegation reference |
| BC-AUDIT-639 | pass-3-deep-skills-batch-3.md | 409 | BC-6.19.002 | SS-06 | Onboarding and setup skills | repo-initialization: workspace-isolation guard refuses dark-factory cwd |
| BC-AUDIT-640 | pass-3-deep-skills-batch-3.md | 418 | BC-6.19.003 | SS-06 | Onboarding and setup skills | repo-initialization: develop branch is the protected default |
| BC-AUDIT-641 | pass-3-deep-skills-batch-3.md | 427 | BC-6.19.004 | SS-06 | Onboarding and setup skills | repo-initialization: factory-artifacts orphan worktree pre-check (NOT dark-factory) |
| BC-AUDIT-642 | pass-3-deep-skills-batch-3.md | 436 | BC-6.19.005 | SS-06 | Onboarding and setup skills | repo-initialization: architect signal table for multi-vs-single-repo |
| BC-AUDIT-643 | pass-3-deep-skills-batch-3.md | 445 | BC-6.19.006 | SS-06 | Onboarding and setup skills | repo-initialization: multi-repo creates .factory-project/ + project.yaml + per-service repos |
| BC-AUDIT-644 | pass-3-deep-skills-batch-3.md | 454 | BC-6.19.007 | SS-06 | Onboarding and setup skills | repo-initialization: dx-engineer environment setup (DF-027) |
| BC-AUDIT-645 | pass-3-deep-skills-batch-3.md | 463 | BC-6.19.008 | SS-06 | Onboarding and setup skills | repo-initialization: CI/CD setup is deferred to post-architecture |
| BC-AUDIT-646 | pass-3-deep-skills-batch-3.md | 472 | BC-6.19.009 | SS-06 | Onboarding and setup skills | repo-initialization: outputs |
| BC-AUDIT-662 | pass-3-deep-skills-batch-3.md | 636 | BC-6.19.010 | SS-06 | Onboarding and setup skills | scaffold-claude-md: identity & overwrite confirmation |
| BC-AUDIT-663 | pass-3-deep-skills-batch-3.md | 645 | BC-6.19.011 | SS-06 | Onboarding and setup skills | scaffold-claude-md: four detectors run in priority order |
| BC-AUDIT-664 | pass-3-deep-skills-batch-3.md | 654 | BC-6.19.012 | SS-06 | Onboarding and setup skills | scaffold-claude-md: CLAUDE.md does NOT duplicate plugin methodology |
| BC-AUDIT-665 | pass-3-deep-skills-batch-3.md | 663 | BC-6.19.013 | SS-06 | Onboarding and setup skills | scaffold-claude-md: present-confirm-write loop |
| BC-AUDIT-666 | pass-3-deep-skills-batch-3.md | 676 | BC-6.19.014 | SS-06 | Onboarding and setup skills | sdk-generation: identity & trigger triad |
| BC-AUDIT-667 | pass-3-deep-skills-batch-3.md | 685 | BC-6.19.015 | SS-06 | Onboarding and setup skills | sdk-generation: contract validation gates generation |
| BC-AUDIT-668 | pass-3-deep-skills-batch-3.md | 694 | BC-6.19.016 | SS-06 | Onboarding and setup skills | sdk-generation: language idiom enforcement (TS async, Py snake_case, Go errors) |
| BC-AUDIT-669 | pass-3-deep-skills-batch-3.md | 703 | BC-6.19.017 | SS-06 | Onboarding and setup skills | sdk-generation: tool-format dispatch (OpenAPI/protobuf/GraphQL) |
| BC-AUDIT-670 | pass-3-deep-skills-batch-3.md | 712 | BC-6.19.018 | SS-06 | Onboarding and setup skills | sdk-generation: contract-test integration (Pact / Specmatic / Schemathesis / openapi-diff) |
| BC-AUDIT-671 | pass-3-deep-skills-batch-3.md | 721 | BC-6.19.019 | SS-06 | Onboarding and setup skills | sdk-generation: contract evolution (semver + breaking detection) |
| BC-AUDIT-672 | pass-3-deep-skills-batch-3.md | 730 | BC-6.19.020 | SS-06 | Onboarding and setup skills | sdk-generation: outputs and quality gate |
| BC-AUDIT-685 | pass-3-deep-skills-batch-3.md | 859 | BC-6.19.021 | SS-06 | Onboarding and setup skills | setup-env: identity & frontmatter |
| BC-AUDIT-686 | pass-3-deep-skills-batch-3.md | 868 | BC-6.19.022 | SS-06 | Onboarding and setup skills | setup-env: tool-check tables (8 required + 8 optional) |
| BC-AUDIT-687 | pass-3-deep-skills-batch-3.md | 877 | BC-6.19.023 | SS-06 | Onboarding and setup skills | setup-env: MCP env-var prefix check + git config (rerere on) |
| BC-AUDIT-688 | pass-3-deep-skills-batch-3.md | 886 | BC-6.19.024 | SS-06 | Onboarding and setup skills | setup-env: factory-health invocation + final missing-tools list |
| BC-AUDIT-718 | pass-3-deep-skills-batch-3.md | 1184 | BC-6.19.025 | SS-06 | Onboarding and setup skills | toolchain-provisioning: identity & 4 trigger points |
| BC-AUDIT-719 | pass-3-deep-skills-batch-3.md | 1193 | BC-6.19.026 | SS-06 | Onboarding and setup skills | toolchain-provisioning: precedence rule (architect > verification > manifest > human) |
| BC-AUDIT-720 | pass-3-deep-skills-batch-3.md | 1202 | BC-6.19.027 | SS-06 | Onboarding and setup skills | toolchain-provisioning: language detection cascade |
| BC-AUDIT-721 | pass-3-deep-skills-batch-3.md | 1211 | BC-6.19.028 | SS-06 | Onboarding and setup skills | toolchain-provisioning: install-priority (lang-native → brew → system) + pkg-mgr per type |
| BC-AUDIT-722 | pass-3-deep-skills-batch-3.md | 1220 | BC-6.19.029 | SS-06 | Onboarding and setup skills | toolchain-provisioning: writes detailed toolchain-state.yaml |
| BC-AUDIT-723 | pass-3-deep-skills-batch-3.md | 1229 | BC-6.19.030 | SS-06 | Onboarding and setup skills | toolchain-provisioning: integration with formal-hardening + multi-repo + new-language |
| BC-AUDIT-724 | pass-3-deep-skills-batch-3.md | 1238 | BC-6.19.031 | SS-06 | Onboarding and setup skills | toolchain-provisioning: Storybook + Excalidraw MCP for UI products |
| BC-AUDIT-725 | pass-3-deep-skills-batch-3.md | 1247 | BC-6.19.032 | SS-06 | Onboarding and setup skills | toolchain-provisioning: quality-gate criteria |
| BC-AUDIT-063 | pass-3-behavioral-contracts.md | 420 | BC-7.01.001 | SS-07 | Bash hook script contracts (broad-sweep) | block-ai-attribution blocks git commit messages with AI attribution |
| BC-AUDIT-064 | pass-3-behavioral-contracts.md | 427 | BC-7.01.002 | SS-07 | Bash hook script contracts (broad-sweep) | capture-commit-activity (PostToolUse:Bash) emits `commit.made` on successful commits |
| BC-AUDIT-065 | pass-3-behavioral-contracts.md | 433 | BC-7.01.003 | SS-07 | Bash hook script contracts (broad-sweep) | regression-gate (PostToolUse) fails when bash command interrupted |
| BC-AUDIT-066 | pass-3-behavioral-contracts.md | 439 | BC-7.01.004 | SS-07 | Bash hook script contracts (broad-sweep) | protect-secrets (PreToolUse:Bash + PreToolUse:Read) blocks reads of dotenv / credentials |
| BC-AUDIT-067 | pass-3-behavioral-contracts.md | 445 | BC-7.01.005 | SS-07 | Bash hook script contracts (broad-sweep) | check-factory-commit warns when committing in `.factory/` without STATE.md update |
| BC-AUDIT-068 | pass-3-behavioral-contracts.md | 451 | BC-7.01.006 | SS-07 | Bash hook script contracts (broad-sweep) | validate-* family (24 validators on PostToolUse:Edit\|Write or all) |
| BC-AUDIT-069 | pass-3-behavioral-contracts.md | 458 | BC-7.01.007 | SS-07 | Bash hook script contracts (broad-sweep) | track-agent-{start,stop} (PreToolUse:Agent / SubagentStop) records agent lifecycle |
| BC-AUDIT-099 | pass-3-behavioral-contracts-deep-r1.md | 192 | BC-7.02.001 | SS-07 | Validator hook class contracts | Hooks read JSON envelope from stdin and parse with jq |
| BC-AUDIT-100 | pass-3-behavioral-contracts-deep-r1.md | 199 | BC-7.02.002 | SS-07 | Validator hook class contracts | Hook exit code semantics: 0 = pass/allow, 2 = block, with stderr diagnostic |
| BC-AUDIT-101 | pass-3-behavioral-contracts-deep-r1.md | 206 | BC-7.02.003 | SS-07 | Validator hook class contracts | Hooks emit `hook.block` event on block via `${CLAUDE_PLUGIN_ROOT}/bin/emit-event` |
| BC-AUDIT-102 | pass-3-behavioral-contracts-deep-r1.md | 213 | BC-7.02.004 | SS-07 | Validator hook class contracts | Hook scoping uses `case "$FILE_PATH"` glob narrowing — early `exit 0` on irrelevant files |
| BC-AUDIT-103 | pass-3-behavioral-contracts-deep-r1.md | 220 | BC-7.02.005 | SS-07 | Validator hook class contracts | Hook latency budget is sub-100ms; deterministic; LLM-free |
| BC-AUDIT-104 | pass-3-behavioral-contracts-deep-r1.md | 227 | BC-7.02.006 | SS-07 | Validator hook class contracts | factory-dispatcher routing binds hooks via `[[hooks]]` entry: `name`, `event`, optional `tool` re... |
| BC-AUDIT-105 | pass-3-behavioral-contracts-deep-r1.md | 234 | BC-7.02.007 | SS-07 | Validator hook class contracts | Validator hooks at `tool = "Edit\|Write"` regex run on EVERY post-edit / post-write event regardl... |
| BC-AUDIT-106 | pass-3-behavioral-contracts-deep-r1.md | 241 | BC-7.02.008 | SS-07 | Validator hook class contracts | Hook capability model: every legacy-routed hook declares `[hooks.capabilities.exec_subprocess]` w... |
| BC-AUDIT-107 | pass-3-behavioral-contracts-deep-r1.md | 248 | BC-7.02.009 | SS-07 | Validator hook class contracts | Native (non-legacy) hook plugins MUST link `vsdd-hook-sdk` and use the `#[hook]` macro (not curre... |
| BC-AUDIT-1000 | pass-3-deep-hooks.md | 39 | BC-7.03.001 | SS-07 | Routing hooks (PreToolUse, PostToolUse, lifecycle) | block-ai-attribution: identity & registry binding |
| BC-AUDIT-1001 | pass-3-deep-hooks.md | 52 | BC-7.03.002 | SS-07 | Routing hooks (PreToolUse, PostToolUse, lifecycle) | block-ai-attribution: substring gate on `git commit` |
| BC-AUDIT-1002 | pass-3-deep-hooks.md | 64 | BC-7.03.003 | SS-07 | Routing hooks (PreToolUse, PostToolUse, lifecycle) | block-ai-attribution: blocks Co-Authored-By with AI tool name |
| BC-AUDIT-1003 | pass-3-deep-hooks.md | 75 | BC-7.03.004 | SS-07 | Routing hooks (PreToolUse, PostToolUse, lifecycle) | block-ai-attribution: blocks "Generated with Claude Code" / generated-by-AI / noreply email |
| BC-AUDIT-1004 | pass-3-deep-hooks.md | 85 | BC-7.03.005 | SS-07 | Routing hooks (PreToolUse, PostToolUse, lifecycle) | block-ai-attribution: jq absence is graceful no-op |
| BC-AUDIT-1005 | pass-3-deep-hooks.md | 98 | BC-7.03.006 | SS-07 | Routing hooks (PreToolUse, PostToolUse, lifecycle) | brownfield-discipline: identity & registry binding |
| BC-AUDIT-1006 | pass-3-deep-hooks.md | 110 | BC-7.03.007 | SS-07 | Routing hooks (PreToolUse, PostToolUse, lifecycle) | brownfield-discipline: blocks edits to `.reference/**` |
| BC-AUDIT-1007 | pass-3-deep-hooks.md | 120 | BC-7.03.008 | SS-07 | Routing hooks (PreToolUse, PostToolUse, lifecycle) | brownfield-discipline: jq missing is hard error (rare among hooks) |
| BC-AUDIT-1008 | pass-3-deep-hooks.md | 132 | BC-7.03.009 | SS-07 | Routing hooks (PreToolUse, PostToolUse, lifecycle) | capture-commit-activity: identity & registry binding |
| BC-AUDIT-1009 | pass-3-deep-hooks.md | 145 | BC-7.03.010 | SS-07 | Routing hooks (PreToolUse, PostToolUse, lifecycle) | capture-commit-activity: skips non-Bash, interrupted, or non-zero exits |
| BC-AUDIT-1010 | pass-3-deep-hooks.md | 155 | BC-7.03.011 | SS-07 | Routing hooks (PreToolUse, PostToolUse, lifecycle) | capture-commit-activity: matches `git commit` as real subcommand and parses preamble |
| BC-AUDIT-1011 | pass-3-deep-hooks.md | 165 | BC-7.03.012 | SS-07 | Routing hooks (PreToolUse, PostToolUse, lifecycle) | capture-commit-activity: detects `--amend` and emits `amended="true"` |
| BC-AUDIT-1012 | pass-3-deep-hooks.md | 175 | BC-7.03.013 | SS-07 | Routing hooks (PreToolUse, PostToolUse, lifecycle) | capture-commit-activity: emits `commit.made` event via emit-event helper |
| BC-AUDIT-1013 | pass-3-deep-hooks.md | 187 | BC-7.03.014 | SS-07 | Routing hooks (PreToolUse, PostToolUse, lifecycle) | capture-pr-activity: identity & registry binding |
| BC-AUDIT-1014 | pass-3-deep-hooks.md | 199 | BC-7.03.015 | SS-07 | Routing hooks (PreToolUse, PostToolUse, lifecycle) | capture-pr-activity: emits `pr.opened` from `gh pr create` stdout |
| BC-AUDIT-1015 | pass-3-deep-hooks.md | 209 | BC-7.03.016 | SS-07 | Routing hooks (PreToolUse, PostToolUse, lifecycle) | capture-pr-activity: emits `pr.merged` and computes `open_to_merge_seconds` |
| BC-AUDIT-1016 | pass-3-deep-hooks.md | 219 | BC-7.03.017 | SS-07 | Routing hooks (PreToolUse, PostToolUse, lifecycle) | capture-pr-activity: log directory resolution order |
| BC-AUDIT-1017 | pass-3-deep-hooks.md | 231 | BC-7.03.018 | SS-07 | Routing hooks (PreToolUse, PostToolUse, lifecycle) | check-factory-commit: identity & registry binding (with semantic mismatch) |
| BC-AUDIT-1018 | pass-3-deep-hooks.md | 244 | BC-7.03.019 | SS-07 | Routing hooks (PreToolUse, PostToolUse, lifecycle) | check-factory-commit: emits additionalContext when .factory commit lacks STATE.md |
| BC-AUDIT-1019 | pass-3-deep-hooks.md | 256 | BC-7.03.020 | SS-07 | Routing hooks (PreToolUse, PostToolUse, lifecycle) | convergence-tracker: identity & registry binding |
| BC-AUDIT-1020 | pass-3-deep-hooks.md | 268 | BC-7.03.021 | SS-07 | Routing hooks (PreToolUse, PostToolUse, lifecycle) | convergence-tracker: enforces trajectory monotonicity (warn-only) |
| BC-AUDIT-1021 | pass-3-deep-hooks.md | 278 | BC-7.03.022 | SS-07 | Routing hooks (PreToolUse, PostToolUse, lifecycle) | convergence-tracker: BLOCKS premature CONVERGENCE_REACHED with novelty>0.15 |
| BC-AUDIT-1022 | pass-3-deep-hooks.md | 288 | BC-7.03.023 | SS-07 | Routing hooks (PreToolUse, PostToolUse, lifecycle) | convergence-tracker: BLOCKS CONVERGENCE_REACHED when CRIT or HIGH count > 0 |
| BC-AUDIT-1023 | pass-3-deep-hooks.md | 298 | BC-7.03.024 | SS-07 | Routing hooks (PreToolUse, PostToolUse, lifecycle) | convergence-tracker: BLOCKS CONVERGENCE_REACHED with <3 consecutive clean passes |
| BC-AUDIT-1024 | pass-3-deep-hooks.md | 308 | BC-7.03.025 | SS-07 | Routing hooks (PreToolUse, PostToolUse, lifecycle) | convergence-tracker: warns on zero findings on first pass |
| BC-AUDIT-1025 | pass-3-deep-hooks.md | 320 | BC-7.03.026 | SS-07 | Routing hooks (PreToolUse, PostToolUse, lifecycle) | destructive-command-guard: identity & registry binding |
| BC-AUDIT-1026 | pass-3-deep-hooks.md | 332 | BC-7.03.027 | SS-07 | Routing hooks (PreToolUse, PostToolUse, lifecycle) | destructive-command-guard: blocks `rm -rf` on catastrophic roots (8 patterns) |
| BC-AUDIT-1027 | pass-3-deep-hooks.md | 342 | BC-7.03.028 | SS-07 | Routing hooks (PreToolUse, PostToolUse, lifecycle) | destructive-command-guard: blocks `rm -rf` on protected paths |
| BC-AUDIT-1028 | pass-3-deep-hooks.md | 352 | BC-7.03.029 | SS-07 | Routing hooks (PreToolUse, PostToolUse, lifecycle) | destructive-command-guard: blocks `rm` on source-of-truth files (8 SOT files) |
| BC-AUDIT-1029 | pass-3-deep-hooks.md | 362 | BC-7.03.030 | SS-07 | Routing hooks (PreToolUse, PostToolUse, lifecycle) | destructive-command-guard: blocks clobbering redirects to SOT files |
| BC-AUDIT-1030 | pass-3-deep-hooks.md | 372 | BC-7.03.031 | SS-07 | Routing hooks (PreToolUse, PostToolUse, lifecycle) | destructive-command-guard: blocks `find -delete\|-exec rm` on protected paths |
| BC-AUDIT-1031 | pass-3-deep-hooks.md | 382 | BC-7.03.032 | SS-07 | Routing hooks (PreToolUse, PostToolUse, lifecycle) | destructive-command-guard: blocks irreversible git operations (8 patterns) |
| BC-AUDIT-1032 | pass-3-deep-hooks.md | 392 | BC-7.03.033 | SS-07 | Routing hooks (PreToolUse, PostToolUse, lifecycle) | destructive-command-guard: blocks `--no-verify` and `--no-gpg-sign` |
| BC-AUDIT-1033 | pass-3-deep-hooks.md | 402 | BC-7.03.034 | SS-07 | Routing hooks (PreToolUse, PostToolUse, lifecycle) | destructive-command-guard: blocks `git rm` on living spec/story files |
| BC-AUDIT-1034 | pass-3-deep-hooks.md | 411 | BC-7.03.035 | SS-07 | Routing hooks (PreToolUse, PostToolUse, lifecycle) | destructive-command-guard: blocks gh shared-state destructive ops (4 codes) |
| BC-AUDIT-1035 | pass-3-deep-hooks.md | 420 | BC-7.03.036 | SS-07 | Routing hooks (PreToolUse, PostToolUse, lifecycle) | destructive-command-guard: blocks curl\|wget pipe-to-shell |
| BC-AUDIT-1036 | pass-3-deep-hooks.md | 430 | BC-7.03.037 | SS-07 | Routing hooks (PreToolUse, PostToolUse, lifecycle) | destructive-command-guard: blocks recursive chmod/chown on protected paths |
| BC-AUDIT-1037 | pass-3-deep-hooks.md | 441 | BC-7.03.038 | SS-07 | Routing hooks (PreToolUse, PostToolUse, lifecycle) | factory-branch-guard: identity & registry binding |
| BC-AUDIT-1038 | pass-3-deep-hooks.md | 453 | BC-7.03.039 | SS-07 | Routing hooks (PreToolUse, PostToolUse, lifecycle) | factory-branch-guard: blocks if .factory/ is not a worktree |
| BC-AUDIT-1039 | pass-3-deep-hooks.md | 463 | BC-7.03.040 | SS-07 | Routing hooks (PreToolUse, PostToolUse, lifecycle) | factory-branch-guard: blocks if worktree on wrong branch |
| BC-AUDIT-1040 | pass-3-deep-hooks.md | 472 | BC-7.03.041 | SS-07 | Routing hooks (PreToolUse, PostToolUse, lifecycle) | factory-branch-guard: skips paths outside .factory tree |
| BC-AUDIT-1041 | pass-3-deep-hooks.md | 483 | BC-7.03.042 | SS-07 | Routing hooks (PreToolUse, PostToolUse, lifecycle) | handoff-validator: identity & registry binding |
| BC-AUDIT-1042 | pass-3-deep-hooks.md | 496 | BC-7.03.043 | SS-07 | Routing hooks (PreToolUse, PostToolUse, lifecycle) | handoff-validator: warns on empty subagent result |
| BC-AUDIT-1043 | pass-3-deep-hooks.md | 506 | BC-7.03.044 | SS-07 | Routing hooks (PreToolUse, PostToolUse, lifecycle) | handoff-validator: warns on suspiciously short result (<40 chars) |
| BC-AUDIT-1044 | pass-3-deep-hooks.md | 517 | BC-7.03.045 | SS-07 | Routing hooks (PreToolUse, PostToolUse, lifecycle) | pr-manager-completion-guard: identity & registry binding |
| BC-AUDIT-1045 | pass-3-deep-hooks.md | 529 | BC-7.03.046 | SS-07 | Routing hooks (PreToolUse, PostToolUse, lifecycle) | pr-manager-completion-guard: counts STEP_COMPLETE emissions; passes if ≥8 |
| BC-AUDIT-1046 | pass-3-deep-hooks.md | 539 | BC-7.03.047 | SS-07 | Routing hooks (PreToolUse, PostToolUse, lifecycle) | pr-manager-completion-guard: BLOCKED status is legitimate early exit |
| BC-AUDIT-1047 | pass-3-deep-hooks.md | 548 | BC-7.03.048 | SS-07 | Routing hooks (PreToolUse, PostToolUse, lifecycle) | pr-manager-completion-guard: blocks with step-specific continuation hint |
| BC-AUDIT-1048 | pass-3-deep-hooks.md | 560 | BC-7.03.049 | SS-07 | Routing hooks (PreToolUse, PostToolUse, lifecycle) | protect-bc: identity & registry binding |
| BC-AUDIT-1049 | pass-3-deep-hooks.md | 573 | BC-7.03.050 | SS-07 | Routing hooks (PreToolUse, PostToolUse, lifecycle) | protect-bc: denies edits to green BCs at .factory/specs/behavioral-contracts/BC-*.md |
| BC-AUDIT-1050 | pass-3-deep-hooks.md | 583 | BC-7.03.051 | SS-07 | Routing hooks (PreToolUse, PostToolUse, lifecycle) | protect-bc: jq-missing fail-closed (exit 1) |
| BC-AUDIT-1051 | pass-3-deep-hooks.md | 593 | BC-7.03.052 | SS-07 | Routing hooks (PreToolUse, PostToolUse, lifecycle) | protect-bc: missing file or non-BC path → allow |
| BC-AUDIT-1052 | pass-3-deep-hooks.md | 605 | BC-7.03.053 | SS-07 | Routing hooks (PreToolUse, PostToolUse, lifecycle) | protect-secrets: identity & dual registry binding |
| BC-AUDIT-1053 | pass-3-deep-hooks.md | 618 | BC-7.03.054 | SS-07 | Routing hooks (PreToolUse, PostToolUse, lifecycle) | protect-secrets: blocks Read of `.env`/`.envrc`/`.env.*` (excludes `.example`/`.sample`/`.template`) |
| BC-AUDIT-1054 | pass-3-deep-hooks.md | 628 | BC-7.03.055 | SS-07 | Routing hooks (PreToolUse, PostToolUse, lifecycle) | protect-secrets: blocks shell content reads of .env files |
| BC-AUDIT-1055 | pass-3-deep-hooks.md | 638 | BC-7.03.056 | SS-07 | Routing hooks (PreToolUse, PostToolUse, lifecycle) | protect-secrets: blocks copy/move of real .env (allows template→real) |
| BC-AUDIT-1056 | pass-3-deep-hooks.md | 647 | BC-7.03.057 | SS-07 | Routing hooks (PreToolUse, PostToolUse, lifecycle) | protect-secrets: blocks tar/zip including .env |
| BC-AUDIT-1057 | pass-3-deep-hooks.md | 656 | BC-7.03.058 | SS-07 | Routing hooks (PreToolUse, PostToolUse, lifecycle) | protect-secrets: blocks `echo\|printf` of secret-shaped variables |
| BC-AUDIT-1058 | pass-3-deep-hooks.md | 666 | BC-7.03.059 | SS-07 | Routing hooks (PreToolUse, PostToolUse, lifecycle) | protect-secrets: blocks env\|grep for secret-shaped names |
| BC-AUDIT-1059 | pass-3-deep-hooks.md | 678 | BC-7.03.060 | SS-07 | Routing hooks (PreToolUse, PostToolUse, lifecycle) | protect-vp: identity & registry binding |
| BC-AUDIT-1060 | pass-3-deep-hooks.md | 689 | BC-7.03.061 | SS-07 | Routing hooks (PreToolUse, PostToolUse, lifecycle) | protect-vp: denies edits to green VPs at .factory/specs/verification-properties/VP-*.md |
| BC-AUDIT-1061 | pass-3-deep-hooks.md | 701 | BC-7.03.062 | SS-07 | Routing hooks (PreToolUse, PostToolUse, lifecycle) | purity-check: identity & registry binding |
| BC-AUDIT-1062 | pass-3-deep-hooks.md | 713 | BC-7.03.063 | SS-07 | Routing hooks (PreToolUse, PostToolUse, lifecycle) | purity-check: scopes to pure/core/kernel paths or *_pure.rs / *.pure.ts |
| BC-AUDIT-1063 | pass-3-deep-hooks.md | 722 | BC-7.03.064 | SS-07 | Routing hooks (PreToolUse, PostToolUse, lifecycle) | purity-check: detects 16 forbidden side-effect patterns and warns |
| BC-AUDIT-1064 | pass-3-deep-hooks.md | 734 | BC-7.03.065 | SS-07 | Routing hooks (PreToolUse, PostToolUse, lifecycle) | red-gate: identity & registry binding |
| BC-AUDIT-1065 | pass-3-deep-hooks.md | 746 | BC-7.03.066 | SS-07 | Routing hooks (PreToolUse, PostToolUse, lifecycle) | red-gate: skips test files always |
| BC-AUDIT-1066 | pass-3-deep-hooks.md | 755 | BC-7.03.067 | SS-07 | Routing hooks (PreToolUse, PostToolUse, lifecycle) | red-gate: scopes to known source extensions |
| BC-AUDIT-1067 | pass-3-deep-hooks.md | 765 | BC-7.03.068 | SS-07 | Routing hooks (PreToolUse, PostToolUse, lifecycle) | red-gate: state file absent → skip |
| BC-AUDIT-1068 | pass-3-deep-hooks.md | 774 | BC-7.03.069 | SS-07 | Routing hooks (PreToolUse, PostToolUse, lifecycle) | red-gate: blocks edits to files not in `red[]` (path normalization 4-way) |
| BC-AUDIT-1069 | pass-3-deep-hooks.md | 788 | BC-7.03.070 | SS-07 | Routing hooks (PreToolUse, PostToolUse, lifecycle) | red-gate: jq parse error → fail-closed exit 1 |
| BC-AUDIT-1070 | pass-3-deep-hooks.md | 800 | BC-7.03.071 | SS-07 | Routing hooks (PreToolUse, PostToolUse, lifecycle) | regression-gate: identity & registry binding |
| BC-AUDIT-1071 | pass-3-deep-hooks.md | 812 | BC-7.03.072 | SS-07 | Routing hooks (PreToolUse, PostToolUse, lifecycle) | regression-gate: matches 9 test runners |
| BC-AUDIT-1072 | pass-3-deep-hooks.md | 821 | BC-7.03.073 | SS-07 | Routing hooks (PreToolUse, PostToolUse, lifecycle) | regression-gate: pass/fail derivation prefers exit_code; falls back to interrupted |
| BC-AUDIT-1073 | pass-3-deep-hooks.md | 831 | BC-7.03.074 | SS-07 | Routing hooks (PreToolUse, PostToolUse, lifecycle) | regression-gate: writes state file with status, timestamp, command |
| BC-AUDIT-1074 | pass-3-deep-hooks.md | 840 | BC-7.03.075 | SS-07 | Routing hooks (PreToolUse, PostToolUse, lifecycle) | regression-gate: warns on pass→fail transition |
| BC-AUDIT-1075 | pass-3-deep-hooks.md | 851 | BC-7.03.076 | SS-07 | Routing hooks (PreToolUse, PostToolUse, lifecycle) | session-learning: identity & registry binding |
| BC-AUDIT-1076 | pass-3-deep-hooks.md | 863 | BC-7.03.077 | SS-07 | Routing hooks (PreToolUse, PostToolUse, lifecycle) | session-learning: appends timestamped marker to .factory/sidecar-learning.md |
| BC-AUDIT-1077 | pass-3-deep-hooks.md | 873 | BC-7.03.078 | SS-07 | Routing hooks (PreToolUse, PostToolUse, lifecycle) | session-learning: skips when .factory/ absent |
| BC-AUDIT-1078 | pass-3-deep-hooks.md | 884 | BC-7.03.079 | SS-07 | Routing hooks (PreToolUse, PostToolUse, lifecycle) | track-agent-start: identity & registry binding |
| BC-AUDIT-1079 | pass-3-deep-hooks.md | 896 | BC-7.03.080 | SS-07 | Routing hooks (PreToolUse, PostToolUse, lifecycle) | track-agent-start: emits agent.start with subagent + best-effort story_id |
| BC-AUDIT-1080 | pass-3-deep-hooks.md | 908 | BC-7.03.081 | SS-07 | Routing hooks (PreToolUse, PostToolUse, lifecycle) | track-agent-stop: identity & registry binding |
| BC-AUDIT-1081 | pass-3-deep-hooks.md | 919 | BC-7.03.082 | SS-07 | Routing hooks (PreToolUse, PostToolUse, lifecycle) | track-agent-stop: classifies result as ok\|empty\|blocked, emits agent.stop |
| BC-AUDIT-1082 | pass-3-deep-hooks.md | 931 | BC-7.03.083 | SS-07 | Routing hooks (PreToolUse, PostToolUse, lifecycle) | update-wave-state-on-merge: identity & registry binding |
| BC-AUDIT-1083 | pass-3-deep-hooks.md | 942 | BC-7.03.084 | SS-07 | Routing hooks (PreToolUse, PostToolUse, lifecycle) | update-wave-state-on-merge: scopes to pr-manager + successful merge signal |
| BC-AUDIT-1084 | pass-3-deep-hooks.md | 951 | BC-7.03.085 | SS-07 | Routing hooks (PreToolUse, PostToolUse, lifecycle) | update-wave-state-on-merge: appends story to wave_data.stories_merged via python YAML |
| BC-AUDIT-1085 | pass-3-deep-hooks.md | 961 | BC-7.03.086 | SS-07 | Routing hooks (PreToolUse, PostToolUse, lifecycle) | update-wave-state-on-merge: flips gate_status to pending when wave fully merged |
| BC-AUDIT-1086 | pass-3-deep-hooks.md | 973 | BC-7.03.087 | SS-07 | Routing hooks (PreToolUse, PostToolUse, lifecycle) | verify-git-push: identity & registry binding |
| BC-AUDIT-1087 | pass-3-deep-hooks.md | 985 | BC-7.03.088 | SS-07 | Routing hooks (PreToolUse, PostToolUse, lifecycle) | verify-git-push: blocks --force / -f, allows --force-with-lease |
| BC-AUDIT-1088 | pass-3-deep-hooks.md | 995 | BC-7.03.089 | SS-07 | Routing hooks (PreToolUse, PostToolUse, lifecycle) | verify-git-push: blocks pushes to main\|master\|develop |
| BC-AUDIT-1089 | pass-3-deep-hooks.md | 1005 | BC-7.03.090 | SS-07 | Routing hooks (PreToolUse, PostToolUse, lifecycle) | verify-git-push: emits additionalContext on allowed pushes |
| BC-AUDIT-1090 | pass-3-deep-hooks.md | 1016 | BC-7.03.091 | SS-07 | Routing hooks (PreToolUse, PostToolUse, lifecycle) | warn-pending-wave-gate: identity & registry binding |
| BC-AUDIT-1091 | pass-3-deep-hooks.md | 1028 | BC-7.03.092 | SS-07 | Routing hooks (PreToolUse, PostToolUse, lifecycle) | warn-pending-wave-gate: stderr warning when any wave has gate_status: pending |
| BC-AUDIT-1175 | pass-3-deep-hooks.md | 1870 | BC-7.03.093 | SS-07 | Routing hooks (PreToolUse, PostToolUse, lifecycle) | verify-git-push: identity confirmation (already covered in BC-1086 routing section) |
| BC-AUDIT-1092 | pass-3-deep-hooks.md | 1042 | BC-7.04.001 | SS-07 | Validator hook scripts (validate-* and verify-*) | validate-anchor-capabilities-union: identity & registry binding |
| BC-AUDIT-1093 | pass-3-deep-hooks.md | 1054 | BC-7.04.002 | SS-07 | Validator hook scripts (validate-* and verify-*) | validate-anchor-capabilities-union: scopes to .factory/stories/S-*.md or STORY-*.md |
| BC-AUDIT-1094 | pass-3-deep-hooks.md | 1063 | BC-7.04.003 | SS-07 | Validator hook scripts (validate-* and verify-*) | validate-anchor-capabilities-union: blocks when frontmatter caps ≠ union over BC capability fields |
| BC-AUDIT-1095 | pass-3-deep-hooks.md | 1073 | BC-7.04.004 | SS-07 | Validator hook scripts (validate-* and verify-*) | validate-anchor-capabilities-union: missing BC files warn-only |
| BC-AUDIT-1096 | pass-3-deep-hooks.md | 1084 | BC-7.04.005 | SS-07 | Validator hook scripts (validate-* and verify-*) | validate-bc-title: identity & registry binding |
| BC-AUDIT-1097 | pass-3-deep-hooks.md | 1094 | BC-7.04.006 | SS-07 | Validator hook scripts (validate-* and verify-*) | validate-bc-title: scopes to behavioral-contracts/BC-*.md, skips BC-INDEX |
| BC-AUDIT-1098 | pass-3-deep-hooks.md | 1102 | BC-7.04.007 | SS-07 | Validator hook scripts (validate-* and verify-*) | validate-bc-title: blocks when BC H1 != BC-INDEX row title |
| BC-AUDIT-1099 | pass-3-deep-hooks.md | 1114 | BC-7.04.008 | SS-07 | Validator hook scripts (validate-* and verify-*) | validate-changelog-monotonicity: identity & registry binding |
| BC-AUDIT-1100 | pass-3-deep-hooks.md | 1125 | BC-7.04.009 | SS-07 | Validator hook scripts (validate-* and verify-*) | validate-changelog-monotonicity: scopes to .factory/*.md, skips known no-changelog files |
| BC-AUDIT-1101 | pass-3-deep-hooks.md | 1133 | BC-7.04.010 | SS-07 | Validator hook scripts (validate-* and verify-*) | validate-changelog-monotonicity: blocks duplicate version rows |
| BC-AUDIT-1102 | pass-3-deep-hooks.md | 1142 | BC-7.04.011 | SS-07 | Validator hook scripts (validate-* and verify-*) | validate-changelog-monotonicity: blocks rows with date older above newer (non-decreasing) |
| BC-AUDIT-1103 | pass-3-deep-hooks.md | 1152 | BC-7.04.012 | SS-07 | Validator hook scripts (validate-* and verify-*) | validate-changelog-monotonicity: cross-checks frontmatter version against top changelog row |
| BC-AUDIT-1104 | pass-3-deep-hooks.md | 1163 | BC-7.04.013 | SS-07 | Validator hook scripts (validate-* and verify-*) | validate-demo-evidence-story-scoped: identity & registry binding |
| BC-AUDIT-1105 | pass-3-deep-hooks.md | 1173 | BC-7.04.014 | SS-07 | Validator hook scripts (validate-* and verify-*) | validate-demo-evidence-story-scoped: blocks flat-level demo-evidence files |
| BC-AUDIT-1106 | pass-3-deep-hooks.md | 1185 | BC-7.04.015 | SS-07 | Validator hook scripts (validate-* and verify-*) | validate-factory-path-root: identity & registry binding |
| BC-AUDIT-1107 | pass-3-deep-hooks.md | 1196 | BC-7.04.016 | SS-07 | Validator hook scripts (validate-* and verify-*) | validate-factory-path-root: blocks paths through .worktrees/<X>/.factory/ |
| BC-AUDIT-1108 | pass-3-deep-hooks.md | 1208 | BC-7.04.017 | SS-07 | Validator hook scripts (validate-* and verify-*) | validate-finding-format: identity & registry binding |
| BC-AUDIT-1109 | pass-3-deep-hooks.md | 1218 | BC-7.04.018 | SS-07 | Validator hook scripts (validate-* and verify-*) | validate-finding-format: blocks legacy ADV-NNN and ADV-PN-NNN formats |
| BC-AUDIT-1110 | pass-3-deep-hooks.md | 1228 | BC-7.04.019 | SS-07 | Validator hook scripts (validate-* and verify-*) | validate-finding-format: blocks legacy STORY-NNN-FIX-NNN format |
| BC-AUDIT-1111 | pass-3-deep-hooks.md | 1239 | BC-7.04.020 | SS-07 | Validator hook scripts (validate-* and verify-*) | validate-index-self-reference: identity & registry binding |
| BC-AUDIT-1112 | pass-3-deep-hooks.md | 1249 | BC-7.04.021 | SS-07 | Validator hook scripts (validate-* and verify-*) | validate-index-self-reference: warns when index lacks current pass/burst row |
| BC-AUDIT-1113 | pass-3-deep-hooks.md | 1261 | BC-7.04.022 | SS-07 | Validator hook scripts (validate-* and verify-*) | validate-input-hash: identity & registry binding |
| BC-AUDIT-1114 | pass-3-deep-hooks.md | 1272 | BC-7.04.023 | SS-07 | Validator hook scripts (validate-* and verify-*) | validate-input-hash: skips intentional placeholders [live-state] and [pending-recompute] |
| BC-AUDIT-1115 | pass-3-deep-hooks.md | 1281 | BC-7.04.024 | SS-07 | Validator hook scripts (validate-* and verify-*) | validate-input-hash: warns on missing/template/null hash |
| BC-AUDIT-1116 | pass-3-deep-hooks.md | 1290 | BC-7.04.025 | SS-07 | Validator hook scripts (validate-* and verify-*) | validate-input-hash: BLOCKS on hash length != 7 |
| BC-AUDIT-1117 | pass-3-deep-hooks.md | 1300 | BC-7.04.026 | SS-07 | Validator hook scripts (validate-* and verify-*) | validate-input-hash: BLOCKS on non-hex chars |
| BC-AUDIT-1118 | pass-3-deep-hooks.md | 1310 | BC-7.04.027 | SS-07 | Validator hook scripts (validate-* and verify-*) | validate-input-hash: warns on stored ≠ computed (advisory) |
| BC-AUDIT-1119 | pass-3-deep-hooks.md | 1321 | BC-7.04.028 | SS-07 | Validator hook scripts (validate-* and verify-*) | validate-novelty-assessment: identity & registry binding |
| BC-AUDIT-1120 | pass-3-deep-hooks.md | 1332 | BC-7.04.029 | SS-07 | Validator hook scripts (validate-* and verify-*) | validate-novelty-assessment: scopes to adversarial review pass files; excludes index/findings |
| BC-AUDIT-1121 | pass-3-deep-hooks.md | 1340 | BC-7.04.030 | SS-07 | Validator hook scripts (validate-* and verify-*) | validate-novelty-assessment: blocks when section is missing |
| BC-AUDIT-1122 | pass-3-deep-hooks.md | 1349 | BC-7.04.031 | SS-07 | Validator hook scripts (validate-* and verify-*) | validate-novelty-assessment: requires Pass / Novelty score / Verdict / Trajectory fields |
| BC-AUDIT-1123 | pass-3-deep-hooks.md | 1360 | BC-7.04.032 | SS-07 | Validator hook scripts (validate-* and verify-*) | validate-pr-description-completeness: identity & registry binding |
| BC-AUDIT-1124 | pass-3-deep-hooks.md | 1369 | BC-7.04.033 | SS-07 | Validator hook scripts (validate-* and verify-*) | validate-pr-description-completeness: scopes to code-delivery/<STORY>/pr-description.md |
| BC-AUDIT-1125 | pass-3-deep-hooks.md | 1377 | BC-7.04.034 | SS-07 | Validator hook scripts (validate-* and verify-*) | validate-pr-description-completeness: blocks missing 6 required H2 sections |
| BC-AUDIT-1126 | pass-3-deep-hooks.md | 1386 | BC-7.04.035 | SS-07 | Validator hook scripts (validate-* and verify-*) | validate-pr-description-completeness: blocks unresolved {placeholder} tokens |
| BC-AUDIT-1127 | pass-3-deep-hooks.md | 1398 | BC-7.04.036 | SS-07 | Validator hook scripts (validate-* and verify-*) | validate-pr-merge-prerequisites: identity & registry binding |
| BC-AUDIT-1128 | pass-3-deep-hooks.md | 1409 | BC-7.04.037 | SS-07 | Validator hook scripts (validate-* and verify-*) | validate-pr-merge-prerequisites: scopes to github-ops merge dispatches |
| BC-AUDIT-1129 | pass-3-deep-hooks.md | 1418 | BC-7.04.038 | SS-07 | Validator hook scripts (validate-* and verify-*) | validate-pr-merge-prerequisites: blocks when 3 evidence files missing |
| BC-AUDIT-1130 | pass-3-deep-hooks.md | 1428 | BC-7.04.039 | SS-07 | Validator hook scripts (validate-* and verify-*) | validate-pr-merge-prerequisites: missing delivery dir is warn-only (graceful early pipeline) |
| BC-AUDIT-1131 | pass-3-deep-hooks.md | 1439 | BC-7.04.040 | SS-07 | Validator hook scripts (validate-* and verify-*) | validate-pr-review-posted: identity & registry binding |
| BC-AUDIT-1132 | pass-3-deep-hooks.md | 1449 | BC-7.04.041 | SS-07 | Validator hook scripts (validate-* and verify-*) | validate-pr-review-posted: scopes to pr-reviewer / pr-review-triage |
| BC-AUDIT-1133 | pass-3-deep-hooks.md | 1457 | BC-7.04.042 | SS-07 | Validator hook scripts (validate-* and verify-*) | validate-pr-review-posted: blocks when pr-review.md not written |
| BC-AUDIT-1134 | pass-3-deep-hooks.md | 1466 | BC-7.04.043 | SS-07 | Validator hook scripts (validate-* and verify-*) | validate-pr-review-posted: blocks `gh pr comment` fallback (not a review verdict) |
| BC-AUDIT-1135 | pass-3-deep-hooks.md | 1475 | BC-7.04.044 | SS-07 | Validator hook scripts (validate-* and verify-*) | validate-pr-review-posted: blocks when no formal review posted |
| BC-AUDIT-1136 | pass-3-deep-hooks.md | 1487 | BC-7.04.045 | SS-07 | Validator hook scripts (validate-* and verify-*) | validate-state-index-status-coherence: identity & registry binding |
| BC-AUDIT-1137 | pass-3-deep-hooks.md | 1498 | BC-7.04.046 | SS-07 | Validator hook scripts (validate-* and verify-*) | validate-state-index-status-coherence: scopes to STATE.md or cycles/*/INDEX.md |
| BC-AUDIT-1138 | pass-3-deep-hooks.md | 1506 | BC-7.04.047 | SS-07 | Validator hook scripts (validate-* and verify-*) | validate-state-index-status-coherence: WARNS (exit 1) when STATE.convergence_status drifts from c... |
| BC-AUDIT-1139 | pass-3-deep-hooks.md | 1518 | BC-7.04.048 | SS-07 | Validator hook scripts (validate-* and verify-*) | validate-state-pin-freshness: identity & registry binding |
| BC-AUDIT-1140 | pass-3-deep-hooks.md | 1527 | BC-7.04.049 | SS-07 | Validator hook scripts (validate-* and verify-*) | validate-state-pin-freshness: blocks when STATE pin != artifact frontmatter version (5 fields) |
| BC-AUDIT-1141 | pass-3-deep-hooks.md | 1539 | BC-7.04.050 | SS-07 | Validator hook scripts (validate-* and verify-*) | validate-state-size: identity & registry binding |
| BC-AUDIT-1142 | pass-3-deep-hooks.md | 1548 | BC-7.04.051 | SS-07 | Validator hook scripts (validate-* and verify-*) | validate-state-size: warns at >200 lines, blocks at >500 unless reducing |
| BC-AUDIT-1143 | pass-3-deep-hooks.md | 1560 | BC-7.04.052 | SS-07 | Validator hook scripts (validate-* and verify-*) | validate-story-bc-sync: identity & registry binding |
| BC-AUDIT-1144 | pass-3-deep-hooks.md | 1571 | BC-7.04.053 | SS-07 | Validator hook scripts (validate-* and verify-*) | validate-story-bc-sync: blocks frontmatter BC missing from body BC table |
| BC-AUDIT-1145 | pass-3-deep-hooks.md | 1580 | BC-7.04.054 | SS-07 | Validator hook scripts (validate-* and verify-*) | validate-story-bc-sync: blocks frontmatter BC missing AC trace annotation |
| BC-AUDIT-1146 | pass-3-deep-hooks.md | 1589 | BC-7.04.055 | SS-07 | Validator hook scripts (validate-* and verify-*) | validate-story-bc-sync: blocks body-table BC missing from frontmatter array |
| BC-AUDIT-1147 | pass-3-deep-hooks.md | 1600 | BC-7.04.056 | SS-07 | Validator hook scripts (validate-* and verify-*) | validate-subsystem-names: identity & registry binding |
| BC-AUDIT-1148 | pass-3-deep-hooks.md | 1610 | BC-7.04.057 | SS-07 | Validator hook scripts (validate-* and verify-*) | validate-subsystem-names: scopes to BC-*.md and STORY-*.md, requires ARCH-INDEX |
| BC-AUDIT-1149 | pass-3-deep-hooks.md | 1618 | BC-7.04.058 | SS-07 | Validator hook scripts (validate-* and verify-*) | validate-subsystem-names: blocks BC `subsystem:` not in canonical SS-NN list |
| BC-AUDIT-1150 | pass-3-deep-hooks.md | 1627 | BC-7.04.059 | SS-07 | Validator hook scripts (validate-* and verify-*) | validate-subsystem-names: blocks story `subsystems:` array members not in canonical list |
| BC-AUDIT-1151 | pass-3-deep-hooks.md | 1638 | BC-7.04.060 | SS-07 | Validator hook scripts (validate-* and verify-*) | validate-table-cell-count: identity & registry binding |
| BC-AUDIT-1152 | pass-3-deep-hooks.md | 1647 | BC-7.04.061 | SS-07 | Validator hook scripts (validate-* and verify-*) | validate-table-cell-count: scopes to .factory/*.md |
| BC-AUDIT-1153 | pass-3-deep-hooks.md | 1654 | BC-7.04.062 | SS-07 | Validator hook scripts (validate-* and verify-*) | validate-table-cell-count: blocks rows with pipe count != header pipe count |
| BC-AUDIT-1154 | pass-3-deep-hooks.md | 1666 | BC-7.04.063 | SS-07 | Validator hook scripts (validate-* and verify-*) | validate-template-compliance: identity & registry binding |
| BC-AUDIT-1155 | pass-3-deep-hooks.md | 1677 | BC-7.04.064 | SS-07 | Validator hook scripts (validate-* and verify-*) | validate-template-compliance: skips INDEX/yaml/json/current-cycle files |
| BC-AUDIT-1156 | pass-3-deep-hooks.md | 1685 | BC-7.04.065 | SS-07 | Validator hook scripts (validate-* and verify-*) | validate-template-compliance: resolves template by document_type, then path pattern (15 patterns) |
| BC-AUDIT-1157 | pass-3-deep-hooks.md | 1694 | BC-7.04.066 | SS-07 | Validator hook scripts (validate-* and verify-*) | validate-template-compliance: blocks missing frontmatter keys vs template |
| BC-AUDIT-1158 | pass-3-deep-hooks.md | 1703 | BC-7.04.067 | SS-07 | Validator hook scripts (validate-* and verify-*) | validate-template-compliance: blocks missing H2 sections vs template |
| BC-AUDIT-1159 | pass-3-deep-hooks.md | 1715 | BC-7.04.068 | SS-07 | Validator hook scripts (validate-* and verify-*) | validate-vp-consistency: identity & registry binding |
| BC-AUDIT-1160 | pass-3-deep-hooks.md | 1725 | BC-7.04.069 | SS-07 | Validator hook scripts (validate-* and verify-*) | validate-vp-consistency: scopes to VP-INDEX.md, verification-architecture.md, verification-covera... |
| BC-AUDIT-1161 | pass-3-deep-hooks.md | 1733 | BC-7.04.070 | SS-07 | Validator hook scripts (validate-* and verify-*) | validate-vp-consistency: blocks VPs in INDEX missing from verification-architecture.md |
| BC-AUDIT-1162 | pass-3-deep-hooks.md | 1742 | BC-7.04.071 | SS-07 | Validator hook scripts (validate-* and verify-*) | validate-vp-consistency: blocks VPs in INDEX missing from coverage-matrix |
| BC-AUDIT-1163 | pass-3-deep-hooks.md | 1750 | BC-7.04.072 | SS-07 | Validator hook scripts (validate-* and verify-*) | validate-vp-consistency: blocks VP-INDEX summary totals != row counts |
| BC-AUDIT-1164 | pass-3-deep-hooks.md | 1759 | BC-7.04.073 | SS-07 | Validator hook scripts (validate-* and verify-*) | validate-vp-consistency: blocks coverage-matrix column sum != VP-INDEX summary total |
| BC-AUDIT-1165 | pass-3-deep-hooks.md | 1768 | BC-7.04.074 | SS-07 | Validator hook scripts (validate-* and verify-*) | validate-vp-consistency: blocks VPs in coverage-matrix missing from VP-INDEX |
| BC-AUDIT-1166 | pass-3-deep-hooks.md | 1779 | BC-7.04.075 | SS-07 | Validator hook scripts (validate-* and verify-*) | validate-wave-gate-completeness: identity & registry binding |
| BC-AUDIT-1167 | pass-3-deep-hooks.md | 1789 | BC-7.04.076 | SS-07 | Validator hook scripts (validate-* and verify-*) | validate-wave-gate-completeness: scopes to wave-state.yaml writes; finds newly-passed waves |
| BC-AUDIT-1168 | pass-3-deep-hooks.md | 1798 | BC-7.04.077 | SS-07 | Validator hook scripts (validate-* and verify-*) | validate-wave-gate-completeness: blocks gate_status:passed without gate_report |
| BC-AUDIT-1169 | pass-3-deep-hooks.md | 1807 | BC-7.04.078 | SS-07 | Validator hook scripts (validate-* and verify-*) | validate-wave-gate-completeness: blocks when gate_report file not found |
| BC-AUDIT-1170 | pass-3-deep-hooks.md | 1816 | BC-7.04.079 | SS-07 | Validator hook scripts (validate-* and verify-*) | validate-wave-gate-completeness: blocks gate report missing evidence for any of 6 gates |
| BC-AUDIT-1171 | pass-3-deep-hooks.md | 1828 | BC-7.04.080 | SS-07 | Validator hook scripts (validate-* and verify-*) | validate-wave-gate-prerequisite: identity & registry binding |
| BC-AUDIT-1172 | pass-3-deep-hooks.md | 1839 | BC-7.04.081 | SS-07 | Validator hook scripts (validate-* and verify-*) | validate-wave-gate-prerequisite: adversary dispatches go through SHA-currency hook |
| BC-AUDIT-1173 | pass-3-deep-hooks.md | 1849 | BC-7.04.082 | SS-07 | Validator hook scripts (validate-* and verify-*) | validate-wave-gate-prerequisite: worker agents go through gate prerequisite check |
| BC-AUDIT-1174 | pass-3-deep-hooks.md | 1859 | BC-7.04.083 | SS-07 | Validator hook scripts (validate-* and verify-*) | validate-wave-gate-prerequisite: skips non-worker, non-adversary subagents |
| BC-AUDIT-1800 | pass-3-deep-templates-tools-rules.md | 42 | BC-8.01.001 | SS-08 | Spec hierarchy templates (L1 through L4) | product-brief-template: governs L1 Product Brief artifact identity |
| BC-AUDIT-1801 | pass-3-deep-templates-tools-rules.md | 49 | BC-8.01.002 | SS-08 | Spec hierarchy templates (L1 through L4) | product-brief-template: required frontmatter fields |
| BC-AUDIT-1802 | pass-3-deep-templates-tools-rules.md | 56 | BC-8.01.003 | SS-08 | Spec hierarchy templates (L1 through L4) | product-brief-template: required sections |
| BC-AUDIT-1803 | pass-3-deep-templates-tools-rules.md | 65 | BC-8.01.004 | SS-08 | Spec hierarchy templates (L1 through L4) | L2-domain-spec-template: deprecated monolithic L2 template kept for reference |
| BC-AUDIT-1804 | pass-3-deep-templates-tools-rules.md | 72 | BC-8.01.005 | SS-08 | Spec hierarchy templates (L1 through L4) | L2-domain-spec-template: required sections (legacy 10-section schema) |
| BC-AUDIT-1805 | pass-3-deep-templates-tools-rules.md | 81 | BC-8.01.006 | SS-08 | Spec hierarchy templates (L1 through L4) | L2-domain-spec-index-template: sharded L2 index identity |
| BC-AUDIT-1806 | pass-3-deep-templates-tools-rules.md | 88 | BC-8.01.007 | SS-08 | Spec hierarchy templates (L1 through L4) | L2-domain-spec-index-template: required sections |
| BC-AUDIT-1807 | pass-3-deep-templates-tools-rules.md | 97 | BC-8.01.008 | SS-08 | Spec hierarchy templates (L1 through L4) | L2-domain-spec-section-template: minimal per-section shard |
| BC-AUDIT-1808 | pass-3-deep-templates-tools-rules.md | 106 | BC-8.01.009 | SS-08 | Spec hierarchy templates (L1 through L4) | prd-template: governs L3 PRD identity |
| BC-AUDIT-1809 | pass-3-deep-templates-tools-rules.md | 113 | BC-8.01.010 | SS-08 | Spec hierarchy templates (L1 through L4) | prd-template: required sections (1 through 7) |
| BC-AUDIT-1810 | pass-3-deep-templates-tools-rules.md | 122 | BC-8.01.011 | SS-08 | Spec hierarchy templates (L1 through L4) | prd-supplement-error-taxonomy-template: PRD error-taxonomy supplement |
| BC-AUDIT-1811 | pass-3-deep-templates-tools-rules.md | 131 | BC-8.01.012 | SS-08 | Spec hierarchy templates (L1 through L4) | prd-supplement-interface-definitions-template: PRD CLI/JSON/Config supplement |
| BC-AUDIT-1812 | pass-3-deep-templates-tools-rules.md | 140 | BC-8.01.013 | SS-08 | Spec hierarchy templates (L1 through L4) | prd-supplement-nfr-catalog-template: PRD non-functional requirements supplement |
| BC-AUDIT-1813 | pass-3-deep-templates-tools-rules.md | 149 | BC-8.01.014 | SS-08 | Spec hierarchy templates (L1 through L4) | prd-supplement-test-vectors-template: canonical test vectors supplement |
| BC-AUDIT-1814 | pass-3-deep-templates-tools-rules.md | 158 | BC-8.01.015 | SS-08 | Spec hierarchy templates (L1 through L4) | behavioral-contract-template: per-BC structural contract |
| BC-AUDIT-1815 | pass-3-deep-templates-tools-rules.md | 165 | BC-8.01.016 | SS-08 | Spec hierarchy templates (L1 through L4) | behavioral-contract-template: optional anchor sections |
| BC-AUDIT-1816 | pass-3-deep-templates-tools-rules.md | 174 | BC-8.01.017 | SS-08 | Spec hierarchy templates (L1 through L4) | L4-verification-property-template: VP-NNN identity + lifecycle |
| BC-AUDIT-1817 | pass-3-deep-templates-tools-rules.md | 185 | BC-8.02.001 | SS-08 | Story, epic, and cycle templates | story-template: governs STORY-NNN identity |
| BC-AUDIT-1818 | pass-3-deep-templates-tools-rules.md | 192 | BC-8.02.002 | SS-08 | Story, epic, and cycle templates | story-template: required sections (10 mandatory) |
| BC-AUDIT-1819 | pass-3-deep-templates-tools-rules.md | 199 | BC-8.02.003 | SS-08 | Story, epic, and cycle templates | story-template: optional planning + ASM/R + lifecycle frontmatter blocks |
| BC-AUDIT-1820 | pass-3-deep-templates-tools-rules.md | 208 | BC-8.02.004 | SS-08 | Story, epic, and cycle templates | story-index-template: STORY-INDEX identity |
| BC-AUDIT-1821 | pass-3-deep-templates-tools-rules.md | 217 | BC-8.02.005 | SS-08 | Story, epic, and cycle templates | epic-template: EPIC-XXX identity |
| BC-AUDIT-1822 | pass-3-deep-templates-tools-rules.md | 226 | BC-8.02.006 | SS-08 | Story, epic, and cycle templates | epic-index-template: EPIC-INDEX identity + epic-to-capability mapping |
| BC-AUDIT-1823 | pass-3-deep-templates-tools-rules.md | 235 | BC-8.02.007 | SS-08 | Story, epic, and cycle templates | cycle-manifest-template: per-cycle manifest identity |
| BC-AUDIT-1824 | pass-3-deep-templates-tools-rules.md | 246 | BC-8.03.001 | SS-08 | Architecture templates | architecture-template: governs L3 architecture document identity |
| BC-AUDIT-1825 | pass-3-deep-templates-tools-rules.md | 253 | BC-8.03.002 | SS-08 | Architecture templates | architecture-template: Part 1 sections (1–9, system + data + integration) |
| BC-AUDIT-1826 | pass-3-deep-templates-tools-rules.md | 260 | BC-8.03.003 | SS-08 | Architecture templates | architecture-template: Part 2 verification architecture (10–14) |
| BC-AUDIT-1827 | pass-3-deep-templates-tools-rules.md | 267 | BC-8.03.004 | SS-08 | Architecture templates | architecture-template: Part 3 module specifications (15) |
| BC-AUDIT-1828 | pass-3-deep-templates-tools-rules.md | 276 | BC-8.03.005 | SS-08 | Architecture templates | architecture-index-template: ARCH-INDEX governs sharded architecture index |
| BC-AUDIT-1829 | pass-3-deep-templates-tools-rules.md | 285 | BC-8.03.006 | SS-08 | Architecture templates | architecture-section-template: per-section ARCH-NN shard |
| BC-AUDIT-1830 | pass-3-deep-templates-tools-rules.md | 294 | BC-8.03.007 | SS-08 | Architecture templates | architecture-feasibility-report-template: pre-architecture feasibility check |
| BC-AUDIT-1831 | pass-3-deep-templates-tools-rules.md | 303 | BC-8.03.008 | SS-08 | Architecture templates | verification-architecture-template: verification-arch shard |
| BC-AUDIT-1832 | pass-3-deep-templates-tools-rules.md | 312 | BC-8.03.009 | SS-08 | Architecture templates | verification-coverage-matrix-template: coverage matrix shard |
| BC-AUDIT-1833 | pass-3-deep-templates-tools-rules.md | 321 | BC-8.03.010 | SS-08 | Architecture templates | verification-gap-analysis-template: brownfield verification gap report |
| BC-AUDIT-1834 | pass-3-deep-templates-tools-rules.md | 330 | BC-8.03.011 | SS-08 | Architecture templates | recovered-architecture-template: brownfield recovered architecture |
| BC-AUDIT-1835 | pass-3-deep-templates-tools-rules.md | 341 | BC-8.04.001 | SS-08 | Adversarial-review templates | adversarial-review-template: per-pass adversarial review identity |
| BC-AUDIT-1836 | pass-3-deep-templates-tools-rules.md | 350 | BC-8.04.002 | SS-08 | Adversarial-review templates | adversarial-review-index-template: per-pass index of findings |
| BC-AUDIT-1837 | pass-3-deep-templates-tools-rules.md | 359 | BC-8.04.003 | SS-08 | Adversarial-review templates | adversarial-finding-template: per-finding ADV-N identity |
| BC-AUDIT-1838 | pass-3-deep-templates-tools-rules.md | 368 | BC-8.04.004 | SS-08 | Adversarial-review templates | findings-tracker-template: cycle-level findings tracker |
| BC-AUDIT-1839 | pass-3-deep-templates-tools-rules.md | 377 | BC-8.04.005 | SS-08 | Adversarial-review templates | fix-template: per-fix FIX-P[N]-NNN identity |
| BC-AUDIT-1840 | pass-3-deep-templates-tools-rules.md | 386 | BC-8.04.006 | SS-08 | Adversarial-review templates | convergence-trajectory-template: pass-by-pass finding trajectory |
| BC-AUDIT-1841 | pass-3-deep-templates-tools-rules.md | 395 | BC-8.04.007 | SS-08 | Adversarial-review templates | review-findings-template: PR-review findings per story |
| BC-AUDIT-1842 | pass-3-deep-templates-tools-rules.md | 404 | BC-8.04.008 | SS-08 | Adversarial-review templates | code-review-template: code-reviewer per-pass output |
| BC-AUDIT-1843 | pass-3-deep-templates-tools-rules.md | 413 | BC-8.04.009 | SS-08 | Adversarial-review templates | agent-file-review-template: agent persona doc review |
| BC-AUDIT-1844 | pass-3-deep-templates-tools-rules.md | 422 | BC-8.04.010 | SS-08 | Adversarial-review templates | adversary-prompt-templates: subdir governs phase-specific adversary prompt scaffolds |
| BC-AUDIT-1845 | pass-3-deep-templates-tools-rules.md | 429 | BC-8.04.011 | SS-08 | Adversarial-review templates | adversary-prompt-templates: required Review Focus + Not-Reviewing sections |
| BC-AUDIT-1846 | pass-3-deep-templates-tools-rules.md | 436 | BC-8.04.012 | SS-08 | Adversarial-review templates | adversary-prompt-templates: previous-findings handlebars template |
| BC-AUDIT-1847 | pass-3-deep-templates-tools-rules.md | 447 | BC-8.05.001 | SS-08 | Holdout-evaluation templates | holdout-scenario-template: HS-NNN scenario identity |
| BC-AUDIT-1848 | pass-3-deep-templates-tools-rules.md | 456 | BC-8.05.002 | SS-08 | Holdout-evaluation templates | holdout-scenario-index-template: HS-INDEX scenario catalog |
| BC-AUDIT-1849 | pass-3-deep-templates-tools-rules.md | 465 | BC-8.05.003 | SS-08 | Holdout-evaluation templates | evaluation-per-scenario-template: HS-NNN per-scenario evaluation |
| BC-AUDIT-1850 | pass-3-deep-templates-tools-rules.md | 474 | BC-8.05.004 | SS-08 | Holdout-evaluation templates | evaluation-index-template: per-pass holdout evaluation index |
| BC-AUDIT-1851 | pass-3-deep-templates-tools-rules.md | 483 | BC-8.05.005 | SS-08 | Holdout-evaluation templates | evaluation-summary-template: holdout evaluation final summary |
| BC-AUDIT-1852 | pass-3-deep-templates-tools-rules.md | 492 | BC-8.05.006 | SS-08 | Holdout-evaluation templates | holdout-evaluation-report-template: cycle-level holdout report |
| BC-AUDIT-1853 | pass-3-deep-templates-tools-rules.md | 503 | BC-8.06.001 | SS-08 | Convergence and traceability templates | convergence-report-template: 7-dimension pipeline convergence scorecard |
| BC-AUDIT-1854 | pass-3-deep-templates-tools-rules.md | 510 | BC-8.06.002 | SS-08 | Convergence and traceability templates | convergence-report-template: 7 named dimensions |
| BC-AUDIT-1855 | pass-3-deep-templates-tools-rules.md | 519 | BC-8.06.003 | SS-08 | Convergence and traceability templates | consistency-report-template: 10-section L1→L4 consistency validation |
| BC-AUDIT-1856 | pass-3-deep-templates-tools-rules.md | 528 | BC-8.06.004 | SS-08 | Convergence and traceability templates | consistency-validation-report-template: minimal consistency-validation gate output |
| BC-AUDIT-1857 | pass-3-deep-templates-tools-rules.md | 537 | BC-8.06.005 | SS-08 | Convergence and traceability templates | traceability-matrix-template: forward + reverse L1→Proof traceability |
| BC-AUDIT-1858 | pass-3-deep-templates-tools-rules.md | 546 | BC-8.06.006 | SS-08 | Convergence and traceability templates | traceability-matrices-template: multi-axis traceability collection (BC/VP/NFR/clause/edge/gap) |
| BC-AUDIT-1859 | pass-3-deep-templates-tools-rules.md | 557 | BC-8.07.001 | SS-08 | Brownfield, discovery, extraction templates | project-context-template: brownfield project context summary |
| BC-AUDIT-1860 | pass-3-deep-templates-tools-rules.md | 566 | BC-8.07.002 | SS-08 | Brownfield, discovery, extraction templates | conventions-template: brownfield conventions extraction |
| BC-AUDIT-1861 | pass-3-deep-templates-tools-rules.md | 575 | BC-8.07.003 | SS-08 | Brownfield, discovery, extraction templates | extraction-validation-template: brownfield extraction validation |
| BC-AUDIT-1862 | pass-3-deep-templates-tools-rules.md | 584 | BC-8.07.004 | SS-08 | Brownfield, discovery, extraction templates | gene-transfusion-assessment-template: brownfield gene-transfusion candidate assessment |
| BC-AUDIT-1863 | pass-3-deep-templates-tools-rules.md | 593 | BC-8.07.005 | SS-08 | Brownfield, discovery, extraction templates | domain-research-template: L2 domain-research report |
| BC-AUDIT-1864 | pass-3-deep-templates-tools-rules.md | 602 | BC-8.07.006 | SS-08 | Brownfield, discovery, extraction templates | research-index-template: per-cycle research index |
| BC-AUDIT-1865 | pass-3-deep-templates-tools-rules.md | 611 | BC-8.07.007 | SS-08 | Brownfield, discovery, extraction templates | discovery-report-template: discovery-engine periodic report |
| BC-AUDIT-1866 | pass-3-deep-templates-tools-rules.md | 620 | BC-8.07.008 | SS-08 | Brownfield, discovery, extraction templates | idea-brief-template: pre-brief idea capture |
| BC-AUDIT-1867 | pass-3-deep-templates-tools-rules.md | 629 | BC-8.07.009 | SS-08 | Brownfield, discovery, extraction templates | feature-request-template: feature-mode FR-NNN identity |
| BC-AUDIT-1868 | pass-3-deep-templates-tools-rules.md | 638 | BC-8.07.010 | SS-08 | Brownfield, discovery, extraction templates | delta-analysis-report-template: feature-mode delta analysis |
| BC-AUDIT-1869 | pass-3-deep-templates-tools-rules.md | 649 | BC-8.08.001 | SS-08 | Demo evidence templates | demo-evidence-report-template: per-product demo evidence rollup |
| BC-AUDIT-1870 | pass-3-deep-templates-tools-rules.md | 658 | BC-8.08.002 | SS-08 | Demo evidence templates | demo-tape-template: VHS .tape demo recording template |
| BC-AUDIT-1871 | pass-3-deep-templates-tools-rules.md | 667 | BC-8.08.003 | SS-08 | Demo evidence templates | demo-playwright-template: Playwright per-AC video+screenshot demo |
| BC-AUDIT-1872 | pass-3-deep-templates-tools-rules.md | 676 | BC-8.08.004 | SS-08 | Demo evidence templates | demo-ci-workflow-template: GitHub Actions demo-generation workflow |
| BC-AUDIT-1873 | pass-3-deep-templates-tools-rules.md | 687 | BC-8.09.001 | SS-08 | Verification and report templates | formal-verification-template: formal-verify pass output |
| BC-AUDIT-1874 | pass-3-deep-templates-tools-rules.md | 696 | BC-8.09.002 | SS-08 | Verification and report templates | fuzz-report-template: fuzz testing per-target report |
| BC-AUDIT-1875 | pass-3-deep-templates-tools-rules.md | 705 | BC-8.09.003 | SS-08 | Verification and report templates | performance-report-template: perf-check pass output |
| BC-AUDIT-1876 | pass-3-deep-templates-tools-rules.md | 714 | BC-8.09.004 | SS-08 | Verification and report templates | security-review-template: security-review per-pass output |
| BC-AUDIT-1877 | pass-3-deep-templates-tools-rules.md | 723 | BC-8.09.005 | SS-08 | Verification and report templates | security-scan-report-template: static-analysis security scan |
| BC-AUDIT-1878 | pass-3-deep-templates-tools-rules.md | 734 | BC-8.10.001 | SS-08 | DTU (Digital Twin Universe) templates | dtu-assessment-template: DTU assessment for SUT |
| BC-AUDIT-1879 | pass-3-deep-templates-tools-rules.md | 743 | BC-8.10.002 | SS-08 | DTU (Digital Twin Universe) templates | dtu-clone-spec-template: per-service DTU clone specification |
| BC-AUDIT-1880 | pass-3-deep-templates-tools-rules.md | 752 | BC-8.10.003 | SS-08 | DTU (Digital Twin Universe) templates | dtu-fidelity-report-template: DTU clone fidelity report |
| BC-AUDIT-1881 | pass-3-deep-templates-tools-rules.md | 763 | BC-8.11.001 | SS-08 | UX templates | ux-spec-template: deprecated monolithic UX spec |
| BC-AUDIT-1882 | pass-3-deep-templates-tools-rules.md | 772 | BC-8.11.002 | SS-08 | UX templates | ux-spec-index-template: sharded UX-spec index |
| BC-AUDIT-1883 | pass-3-deep-templates-tools-rules.md | 781 | BC-8.11.003 | SS-08 | UX templates | ux-spec-screen-template: per-screen SCR-NNN spec |
| BC-AUDIT-1884 | pass-3-deep-templates-tools-rules.md | 790 | BC-8.11.004 | SS-08 | UX templates | ux-spec-flow-template: per-flow FLOW-NNN spec |
| BC-AUDIT-1885 | pass-3-deep-templates-tools-rules.md | 801 | BC-8.12.001 | SS-08 | Design system and UI quality templates | design-system/: subdir governs design-token + component-contract + pattern catalog |
| BC-AUDIT-1886 | pass-3-deep-templates-tools-rules.md | 808 | BC-8.12.002 | SS-08 | Design system and UI quality templates | design-system/constraints.yaml: global UI generation rules |
| BC-AUDIT-1887 | pass-3-deep-templates-tools-rules.md | 815 | BC-8.12.003 | SS-08 | Design system and UI quality templates | design-system/tokens/: 7 token JSON catalogs |
| BC-AUDIT-1888 | pass-3-deep-templates-tools-rules.md | 822 | BC-8.12.004 | SS-08 | Design system and UI quality templates | design-system/components/: registry + 11 component contracts |
| BC-AUDIT-1889 | pass-3-deep-templates-tools-rules.md | 829 | BC-8.12.005 | SS-08 | Design system and UI quality templates | design-system/patterns/: 3 cross-component pattern catalogs |
| BC-AUDIT-1890 | pass-3-deep-templates-tools-rules.md | 838 | BC-8.12.006 | SS-08 | Design system and UI quality templates | ui-quality/: subdir governs UI quality gate + report templates |
| BC-AUDIT-1891 | pass-3-deep-templates-tools-rules.md | 845 | BC-8.12.007 | SS-08 | Design system and UI quality templates | ui-quality/gate-report-template: 4-gate-level UI quality gate |
| BC-AUDIT-1892 | pass-3-deep-templates-tools-rules.md | 852 | BC-8.12.008 | SS-08 | Design system and UI quality templates | ui-quality/heuristic-evaluation-template: 10-heuristic UX evaluation |
| BC-AUDIT-1893 | pass-3-deep-templates-tools-rules.md | 859 | BC-8.12.009 | SS-08 | Design system and UI quality templates | ui-quality/responsive-report-template: 4-breakpoint responsive validation |
| BC-AUDIT-1894 | pass-3-deep-templates-tools-rules.md | 866 | BC-8.12.010 | SS-08 | Design system and UI quality templates | ui-quality/completeness-report-template: UI completeness fidelity report |
| BC-AUDIT-1895 | pass-3-deep-templates-tools-rules.md | 875 | BC-8.12.011 | SS-08 | Design system and UI quality templates | ui-traceability-template: UI element → story → component → test → visual evidence matrix |
| BC-AUDIT-1896 | pass-3-deep-templates-tools-rules.md | 886 | BC-8.13.001 | SS-08 | Spec lifecycle templates (changelog, drift, withdrawal, gates) | spec-changelog-template: spec-versioning changelog |
| BC-AUDIT-1897 | pass-3-deep-templates-tools-rules.md | 895 | BC-8.13.002 | SS-08 | Spec lifecycle templates (changelog, drift, withdrawal, gates) | spec-drift-report-template: spec-drift report |
| BC-AUDIT-1898 | pass-3-deep-templates-tools-rules.md | 904 | BC-8.13.003 | SS-08 | Spec lifecycle templates (changelog, drift, withdrawal, gates) | vp-withdrawal-template: green-VP retirement record |
| BC-AUDIT-1899 | pass-3-deep-templates-tools-rules.md | 913 | BC-8.13.004 | SS-08 | Spec lifecycle templates (changelog, drift, withdrawal, gates) | design-drift-template: design-drift detection report |
| BC-AUDIT-1900 | pass-3-deep-templates-tools-rules.md | 924 | BC-8.14.001 | SS-08 | State and workflow templates | state-template: STATE.md pipeline-state identity |
| BC-AUDIT-1901 | pass-3-deep-templates-tools-rules.md | 933 | BC-8.14.002 | SS-08 | State and workflow templates | state-manager-checklist-template: wave-gate remediation-burst checklist |
| BC-AUDIT-1902 | pass-3-deep-templates-tools-rules.md | 942 | BC-8.14.003 | SS-08 | State and workflow templates | burst-log-template: state-burst log |
| BC-AUDIT-1903 | pass-3-deep-templates-tools-rules.md | 951 | BC-8.14.004 | SS-08 | State and workflow templates | session-checkpoints-template: cycle session resume checkpoints |
| BC-AUDIT-1904 | pass-3-deep-templates-tools-rules.md | 960 | BC-8.14.005 | SS-08 | State and workflow templates | session-review-template: post-cycle session review |
| BC-AUDIT-1905 | pass-3-deep-templates-tools-rules.md | 969 | BC-8.14.006 | SS-08 | State and workflow templates | lessons-template: cycle lessons-learned |
| BC-AUDIT-1906 | pass-3-deep-templates-tools-rules.md | 978 | BC-8.14.007 | SS-08 | State and workflow templates | blocking-issues-resolved-template: cycle blockers-resolved log |
| BC-AUDIT-1907 | pass-3-deep-templates-tools-rules.md | 987 | BC-8.14.008 | SS-08 | State and workflow templates | wave-schedule-template: per-cycle wave schedule |
| BC-AUDIT-1908 | pass-3-deep-templates-tools-rules.md | 996 | BC-8.14.009 | SS-08 | State and workflow templates | wave-state-template: wave-state.yaml lifecycle tracker schema |
| BC-AUDIT-1909 | pass-3-deep-templates-tools-rules.md | 1005 | BC-8.14.010 | SS-08 | State and workflow templates | red-gate-log-template: TDD red-gate verification log |
| BC-AUDIT-1910 | pass-3-deep-templates-tools-rules.md | 1016 | BC-8.15.001 | SS-08 | Code-delivery and PR templates | pr-description-template: per-story PR description |
| BC-AUDIT-1911 | pass-3-deep-templates-tools-rules.md | 1025 | BC-8.15.002 | SS-08 | Code-delivery and PR templates | release-notes-template: per-version release notes |
| BC-AUDIT-1912 | pass-3-deep-templates-tools-rules.md | 1036 | BC-8.16.001 | SS-08 | Discovery, project, config templates | autonomy-config-template: budget + protected-agents schema |
| BC-AUDIT-1913 | pass-3-deep-templates-tools-rules.md | 1045 | BC-8.16.002 | SS-08 | Discovery, project, config templates | merge-config-template: code-delivery autonomy + branch + PR config |
| BC-AUDIT-1914 | pass-3-deep-templates-tools-rules.md | 1054 | BC-8.16.003 | SS-08 | Discovery, project, config templates | policies-template: declarative governance policy registry schema |
| BC-AUDIT-1915 | pass-3-deep-templates-tools-rules.md | 1063 | BC-8.16.004 | SS-08 | Discovery, project, config templates | discovery-config-template: discovery-engine ingestion config |
| BC-AUDIT-1916 | pass-3-deep-templates-tools-rules.md | 1072 | BC-8.16.005 | SS-08 | Discovery, project, config templates | project-manifest-template: multi-repo project.yaml schema |
| BC-AUDIT-1917 | pass-3-deep-templates-tools-rules.md | 1081 | BC-8.16.006 | SS-08 | Discovery, project, config templates | reference-manifest-template: .reference/ rebuild manifest |
| BC-AUDIT-1918 | pass-3-deep-templates-tools-rules.md | 1090 | BC-8.16.007 | SS-08 | Discovery, project, config templates | factory-project-state-template: multi-repo project-level STATE.md |
| BC-AUDIT-1919 | pass-3-deep-templates-tools-rules.md | 1099 | BC-8.16.008 | SS-08 | Discovery, project, config templates | factory-project-structure-template: .factory-project/ multi-repo directory structure |
| BC-AUDIT-1920 | pass-3-deep-templates-tools-rules.md | 1108 | BC-8.16.009 | SS-08 | Discovery, project, config templates | tech-debt-register-template: project tech-debt register |
| BC-AUDIT-1921 | pass-3-deep-templates-tools-rules.md | 1117 | BC-8.16.010 | SS-08 | Discovery, project, config templates | sweep-report-template: maintenance sweep report |
| BC-AUDIT-1922 | pass-3-deep-templates-tools-rules.md | 1126 | BC-8.16.011 | SS-08 | Discovery, project, config templates | project-justfile-template: per-project justfile bootstrap |
| BC-AUDIT-1923 | pass-3-deep-templates-tools-rules.md | 1135 | BC-8.16.012 | SS-08 | Discovery, project, config templates | implementation-readiness-template: pre-implementation readiness gate |
| BC-AUDIT-1924 | pass-3-deep-templates-tools-rules.md | 1144 | BC-8.16.013 | SS-08 | Discovery, project, config templates | brief-validation-template: brief-quality gate report |
| BC-AUDIT-1925 | pass-3-deep-templates-tools-rules.md | 1153 | BC-8.16.014 | SS-08 | Discovery, project, config templates | module-criticality-template: module criticality classification |
| BC-AUDIT-1926 | pass-3-deep-templates-tools-rules.md | 1164 | BC-8.17.001 | SS-08 | Skill and agent file templates | skill-execution-template: SKILL.md (execution variant) shape |
| BC-AUDIT-1927 | pass-3-deep-templates-tools-rules.md | 1173 | BC-8.17.002 | SS-08 | Skill and agent file templates | skill-delegation-template: SKILL.md (delegation variant) shape |
| BC-AUDIT-1928 | pass-3-deep-templates-tools-rules.md | 1182 | BC-8.17.003 | SS-08 | Skill and agent file templates | agents-md-template: AGENTS.md shape |
| BC-AUDIT-1929 | pass-3-deep-templates-tools-rules.md | 1191 | BC-8.18.001 | SS-08 | verify-sha-currency template-distributed hook | verify-sha-currency.sh: state-manager burst-hygiene gate (template-distributed; opt-in, NOT regis... |
| BC-AUDIT-2200 | pass-3-deep-templates-tools-rules.md | 1652 | BC-8.19.001 | SS-08 | Rules index (_index.md include order) | rules/_index.md: rule include-order via @-references |
| BC-AUDIT-2201 | pass-3-deep-templates-tools-rules.md | 1661 | BC-8.20.001 | SS-08 | Rules: bash (no stderr suppression, no eval, dependency checks) | rules/bash.md: SHALL NOT suppress stderr with `2>/dev/null` in production scripts |
| BC-AUDIT-2202 | pass-3-deep-templates-tools-rules.md | 1668 | BC-8.20.002 | SS-08 | Rules: bash (no stderr suppression, no eval, dependency checks) | rules/bash.md: SHALL NOT use `eval` in shell helpers |
| BC-AUDIT-2203 | pass-3-deep-templates-tools-rules.md | 1675 | BC-8.20.003 | SS-08 | Rules: bash (no stderr suppression, no eval, dependency checks) | rules/bash.md: justfile recipes MUST guard optional tools with `command -v` check |
| BC-AUDIT-2204 | pass-3-deep-templates-tools-rules.md | 1682 | BC-8.20.004 | SS-08 | Rules: bash (no stderr suppression, no eval, dependency checks) | rules/bash.md: test files MUST verify tool dependencies at the top before any assertions |
| BC-AUDIT-2205 | pass-3-deep-templates-tools-rules.md | 1689 | BC-8.20.005 | SS-08 | Rules: bash (no stderr suppression, no eval, dependency checks) | rules/bash.md: negative assertions MUST verify the search tool ran successfully |
| BC-AUDIT-2206 | pass-3-deep-templates-tools-rules.md | 1696 | BC-8.20.006 | SS-08 | Rules: bash (no stderr suppression, no eval, dependency checks) | rules/bash.md: literal-string matching SHALL use `grep -F` |
| BC-AUDIT-2207 | pass-3-deep-templates-tools-rules.md | 1703 | BC-8.20.007 | SS-08 | Rules: bash (no stderr suppression, no eval, dependency checks) | rules/bash.md: test headers SHALL state accurate test counts |
| BC-AUDIT-2208 | pass-3-deep-templates-tools-rules.md | 1710 | BC-8.20.008 | SS-08 | Rules: bash (no stderr suppression, no eval, dependency checks) | rules/bash.md: file-path references SHALL be validated by structural tests |
| BC-AUDIT-2209 | pass-3-deep-templates-tools-rules.md | 1717 | BC-8.20.009 | SS-08 | Rules: bash (no stderr suppression, no eval, dependency checks) | rules/bash.md: every `2>/dev/null` MUST carry a `# STDERR-EXEMPT: <rationale>` tag |
| BC-AUDIT-2210 | pass-3-deep-templates-tools-rules.md | 1724 | BC-8.20.010 | SS-08 | Rules: bash (no stderr suppression, no eval, dependency checks) | rules/bash.md: `just ci` MUST run the same commands as `.github/workflows/ci.yml` |
| BC-AUDIT-2211 | pass-3-deep-templates-tools-rules.md | 1733 | BC-8.21.001 | SS-08 | Rules: factory-protocol (.factory worktree, governance) | rules/factory-protocol.md: `.factory/` is a git worktree on the orphan `factory-artifacts` branch |
| BC-AUDIT-2212 | pass-3-deep-templates-tools-rules.md | 1740 | BC-8.21.002 | SS-08 | Rules: factory-protocol (.factory worktree, governance) | rules/factory-protocol.md: `.factory/` directory layout (canonical 8-section structure) |
| BC-AUDIT-2213 | pass-3-deep-templates-tools-rules.md | 1747 | BC-8.21.003 | SS-08 | Rules: factory-protocol (.factory worktree, governance) | rules/factory-protocol.md: all `.factory/` changes commit to `factory-artifacts`, NOT main/develop |
| BC-AUDIT-2214 | pass-3-deep-templates-tools-rules.md | 1754 | BC-8.21.004 | SS-08 | Rules: factory-protocol (.factory worktree, governance) | rules/factory-protocol.md: file lifecycle classification (Living / Accumulating / Cycle-scoped / ... |
| BC-AUDIT-2215 | pass-3-deep-templates-tools-rules.md | 1761 | BC-8.21.005 | SS-08 | Rules: factory-protocol (.factory worktree, governance) | rules/factory-protocol.md: NEVER put target project source code in `.factory/` |
| BC-AUDIT-2216 | pass-3-deep-templates-tools-rules.md | 1768 | BC-8.21.006 | SS-08 | Rules: factory-protocol (.factory worktree, governance) | rules/factory-protocol.md: NEVER modify `.factory/` files from main/develop branch |
| BC-AUDIT-2217 | pass-3-deep-templates-tools-rules.md | 1775 | BC-8.21.007 | SS-08 | Rules: factory-protocol (.factory worktree, governance) | rules/factory-protocol.md: STATE.md is the single source of truth for pipeline progress |
| BC-AUDIT-2218 | pass-3-deep-templates-tools-rules.md | 1782 | BC-8.21.008 | SS-08 | Rules: factory-protocol (.factory worktree, governance) | rules/factory-protocol.md: specs are the product, code is disposable (SOUL.md #3 reified) |
| BC-AUDIT-2219 | pass-3-deep-templates-tools-rules.md | 1791 | BC-8.22.001 | SS-08 | Rules: git-commits (Conventional Commits, no AI attribution) | rules/git-commits.md: all commits MUST follow Conventional Commits |
| BC-AUDIT-2220 | pass-3-deep-templates-tools-rules.md | 1798 | BC-8.22.002 | SS-08 | Rules: git-commits (Conventional Commits, no AI attribution) | rules/git-commits.md: commit type SHALL be one of 10 known values (feat/fix/docs/style/refactor/p... |
| BC-AUDIT-2221 | pass-3-deep-templates-tools-rules.md | 1805 | BC-8.22.003 | SS-08 | Rules: git-commits (Conventional Commits, no AI attribution) | rules/git-commits.md: description uses imperative present tense, lowercase initial, no period |
| BC-AUDIT-2222 | pass-3-deep-templates-tools-rules.md | 1812 | BC-8.22.004 | SS-08 | Rules: git-commits (Conventional Commits, no AI attribution) | rules/git-commits.md: scope (optional) is parenthesized after type — `feat(api):` |
| BC-AUDIT-2223 | pass-3-deep-templates-tools-rules.md | 1819 | BC-8.22.005 | SS-08 | Rules: git-commits (Conventional Commits, no AI attribution) | rules/git-commits.md: body separated from description with blank line; explains motivation + prev... |
| BC-AUDIT-2224 | pass-3-deep-templates-tools-rules.md | 1826 | BC-8.22.006 | SS-08 | Rules: git-commits (Conventional Commits, no AI attribution) | rules/git-commits.md: footers — `Refs:`, `Closes:`, `BREAKING CHANGE:` |
| BC-AUDIT-2225 | pass-3-deep-templates-tools-rules.md | 1833 | BC-8.22.007 | SS-08 | Rules: git-commits (Conventional Commits, no AI attribution) | rules/git-commits.md: breaking changes — `!` after type/scope OR `BREAKING CHANGE:` footer |
| BC-AUDIT-2226 | pass-3-deep-templates-tools-rules.md | 1840 | BC-8.22.008 | SS-08 | Rules: git-commits (Conventional Commits, no AI attribution) | rules/git-commits.md: NEVER include AI attribution in commit messages |
| BC-AUDIT-2227 | pass-3-deep-templates-tools-rules.md | 1847 | BC-8.22.009 | SS-08 | Rules: git-commits (Conventional Commits, no AI attribution) | rules/git-commits.md: NEVER use `gh pr merge --admin` without explicit per-merge user permission |
| BC-AUDIT-2228 | pass-3-deep-templates-tools-rules.md | 1856 | BC-8.23.001 | SS-08 | Rules: rust (forbid unsafe, edition, lints) | rules/rust.md: every application crate MUST declare `#![forbid(unsafe_code)]` |
| BC-AUDIT-2229 | pass-3-deep-templates-tools-rules.md | 1863 | BC-8.23.002 | SS-08 | Rules: rust (forbid unsafe, edition, lints) | rules/rust.md: NO `unwrap()` in production code — use `?` or `expect("actionable msg")` |
| BC-AUDIT-2230 | pass-3-deep-templates-tools-rules.md | 1870 | BC-8.23.003 | SS-08 | Rules: rust (forbid unsafe, edition, lints) | rules/rust.md: NEVER block the async runtime — use `spawn_blocking` for CPU work, `tokio::time:... |
| BC-AUDIT-2231 | pass-3-deep-templates-tools-rules.md | 1877 | BC-8.23.004 | SS-08 | Rules: rust (forbid unsafe, edition, lints) | rules/rust.md: type design — newtypes for IDs, validated constructors at trust boundaries, `#[n... |
| BC-AUDIT-2232 | pass-3-deep-templates-tools-rules.md | 1884 | BC-8.23.005 | SS-08 | Rules: rust (forbid unsafe, edition, lints) | rules/rust.md: error handling — thiserror enums, per-crate `pub type Result<T>`, sanitize `Disp... |
| BC-AUDIT-2233 | pass-3-deep-templates-tools-rules.md | 1891 | BC-8.23.006 | SS-08 | Rules: rust (forbid unsafe, edition, lints) | rules/rust.md: module structure — `lib.rs` is pure re-export barrel; impl in domain modules |
| BC-AUDIT-2234 | pass-3-deep-templates-tools-rules.md | 1898 | BC-8.23.007 | SS-08 | Rules: rust (forbid unsafe, edition, lints) | rules/rust.md: dependencies declared at workspace level; Edition 2024, MSRV 1.85+; clippy warning... |
| BC-AUDIT-2235 | pass-3-deep-templates-tools-rules.md | 1905 | BC-8.23.008 | SS-08 | Rules: rust (forbid unsafe, edition, lints) | rules/rust.md: testing — unit/`#[cfg(test)]` in same file, integration in `tests/`, property in... |
| BC-AUDIT-2236 | pass-3-deep-templates-tools-rules.md | 1912 | BC-8.23.009 | SS-08 | Rules: rust (forbid unsafe, edition, lints) | rules/rust.md: architecture — strictly acyclic dependency graph; no circular deps between crates |
| BC-AUDIT-2237 | pass-3-deep-templates-tools-rules.md | 1921 | BC-8.24.001 | SS-08 | Rules: spec-format (4-level hierarchy, BC/VP shape) | rules/spec-format.md: 4-level spec hierarchy (L1 brief / L2 domain / L3 BC / L4 VP) |
| BC-AUDIT-2238 | pass-3-deep-templates-tools-rules.md | 1928 | BC-8.24.002 | SS-08 | Rules: spec-format (4-level hierarchy, BC/VP shape) | rules/spec-format.md: BC numbering — `BC-S.SS.NNN` (S=subsystem, SS=section, NNN=contract) |
| BC-AUDIT-2239 | pass-3-deep-templates-tools-rules.md | 1935 | BC-8.24.003 | SS-08 | Rules: spec-format (4-level hierarchy, BC/VP shape) | rules/spec-format.md: BC file format SHALL contain Subsystem/Section/Contract/Preconditions/Postc... |
| BC-AUDIT-2240 | pass-3-deep-templates-tools-rules.md | 1942 | BC-8.24.004 | SS-08 | Rules: spec-format (4-level hierarchy, BC/VP shape) | rules/spec-format.md: BC-INDEX.md format — table with ID/Title/Subsystem/Status/Stories columns |
| BC-AUDIT-2241 | pass-3-deep-templates-tools-rules.md | 1949 | BC-8.24.005 | SS-08 | Rules: spec-format (4-level hierarchy, BC/VP shape) | rules/spec-format.md: VP numbering — sequential `VP-NNN` |
| BC-AUDIT-2242 | pass-3-deep-templates-tools-rules.md | 1956 | BC-8.24.006 | SS-08 | Rules: spec-format (4-level hierarchy, BC/VP shape) | rules/spec-format.md: VP file format SHALL contain Property/Type/Scope/Verification Method/Status... |
| BC-AUDIT-2243 | pass-3-deep-templates-tools-rules.md | 1963 | BC-8.24.007 | SS-08 | Rules: spec-format (4-level hierarchy, BC/VP shape) | rules/spec-format.md: green VPs are IMMUTABLE — modification requires new VP supersedes old |
| BC-AUDIT-2244 | pass-3-deep-templates-tools-rules.md | 1970 | BC-8.24.008 | SS-08 | Rules: spec-format (4-level hierarchy, BC/VP shape) | rules/spec-format.md: architecture is sharded into ARCH-NN sections, NOT a monolith |
| BC-AUDIT-2245 | pass-3-deep-templates-tools-rules.md | 1977 | BC-8.24.009 | SS-08 | Rules: spec-format (4-level hierarchy, BC/VP shape) | rules/spec-format.md: ARCH-NN section template — Overview/Decisions/Components/Data Flow/Constr... |
| BC-AUDIT-2246 | pass-3-deep-templates-tools-rules.md | 1984 | BC-8.24.010 | SS-08 | Rules: spec-format (4-level hierarchy, BC/VP shape) | rules/spec-format.md: PRD supplements live in `.factory/specs/prd-supplements/` (5 named files) |
| BC-AUDIT-2247 | pass-3-deep-templates-tools-rules.md | 1991 | BC-8.24.011 | SS-08 | Rules: spec-format (4-level hierarchy, BC/VP shape) | rules/spec-format.md: STORY-NNN file format — Epic/Description/AC/BCs/VPs/Tasks/Strategy/Depend... |
| BC-AUDIT-2248 | pass-3-deep-templates-tools-rules.md | 1998 | BC-8.24.012 | SS-08 | Rules: spec-format (4-level hierarchy, BC/VP shape) | rules/spec-format.md: BC retirement requires updating ALL 5 artifacts in same burst |
| BC-AUDIT-2249 | pass-3-deep-templates-tools-rules.md | 2007 | BC-8.25.001 | SS-08 | Rules: step-decomposition (VSDD phase 0-7 protocol) | rules/step-decomposition.md: VSDD pipeline has 8 phases numbered 0–7 |
| BC-AUDIT-2250 | pass-3-deep-templates-tools-rules.md | 2014 | BC-8.25.002 | SS-08 | Rules: step-decomposition (VSDD phase 0-7 protocol) | rules/step-decomposition.md: phase numbers are sequential integers; no fractional phases ("3.5") |
| BC-AUDIT-2251 | pass-3-deep-templates-tools-rules.md | 2021 | BC-8.25.003 | SS-08 | Rules: step-decomposition (VSDD phase 0-7 protocol) | rules/step-decomposition.md: every phase has exactly two skill entry points (work skill + phase e... |
| BC-AUDIT-2252 | pass-3-deep-templates-tools-rules.md | 2028 | BC-8.25.004 | SS-08 | Rules: step-decomposition (VSDD phase 0-7 protocol) | rules/step-decomposition.md: 4-layer orchestration architecture (lobster → phase entry-point sk... |
| BC-AUDIT-2253 | pass-3-deep-templates-tools-rules.md | 2035 | BC-8.25.005 | SS-08 | Rules: step-decomposition (VSDD phase 0-7 protocol) | rules/step-decomposition.md: step IDs are LOWERCASE ALPHABETIC ONLY — `step-a-`, `step-b-`, `st... |
| BC-AUDIT-2254 | pass-3-deep-templates-tools-rules.md | 2042 | BC-8.25.006 | SS-08 | Rules: step-decomposition (VSDD phase 0-7 protocol) | rules/step-decomposition.md: step file structure includes `_shared-context.md` + per-step files |
| BC-AUDIT-2255 | pass-3-deep-templates-tools-rules.md | 2049 | BC-8.25.007 | SS-08 | Rules: step-decomposition (VSDD phase 0-7 protocol) | rules/step-decomposition.md: lobster step `name:` MUST match the step file ID (without `step-` pr... |
| BC-AUDIT-2256 | pass-3-deep-templates-tools-rules.md | 2056 | BC-8.25.008 | SS-08 | Rules: step-decomposition (VSDD phase 0-7 protocol) | rules/step-decomposition.md: `_shared-context.md` holds constraints applying to ALL steps in the ... |
| BC-AUDIT-2257 | pass-3-deep-templates-tools-rules.md | 2063 | BC-8.25.009 | SS-08 | Rules: step-decomposition (VSDD phase 0-7 protocol) | rules/step-decomposition.md: content completeness — no content loss on decomposition |
| BC-AUDIT-2258 | pass-3-deep-templates-tools-rules.md | 2070 | BC-8.25.010 | SS-08 | Rules: step-decomposition (VSDD phase 0-7 protocol) | rules/step-decomposition.md: phase sub-workflow lobster pattern — step + state-manager backup +... |
| BC-AUDIT-2259 | pass-3-deep-templates-tools-rules.md | 2077 | BC-8.25.011 | SS-08 | Rules: step-decomposition (VSDD phase 0-7 protocol) | rules/step-decomposition.md: forbidden practices (no fractional phases / numeric step IDs / sub-s... |
| BC-AUDIT-2260 | pass-3-deep-templates-tools-rules.md | 2084 | BC-8.25.012 | SS-08 | Rules: step-decomposition (VSDD phase 0-7 protocol) | rules/step-decomposition.md: verification — lobster-parse + path resolution + content completen... |
| BC-AUDIT-2261 | pass-3-deep-templates-tools-rules.md | 2093 | BC-8.26.001 | SS-08 | Rules: story-completeness (14-check audit) | rules/story-completeness.md: 14-check audit before marking a story ready for implementation |
| BC-AUDIT-2262 | pass-3-deep-templates-tools-rules.md | 2100 | BC-8.26.002 | SS-08 | Rules: story-completeness (14-check audit) | rules/story-completeness.md: check 1 — source-of-truth alignment (line-by-line vs architecture ... |
| BC-AUDIT-2263 | pass-3-deep-templates-tools-rules.md | 2107 | BC-8.26.003 | SS-08 | Rules: story-completeness (14-check audit) | rules/story-completeness.md: check 2 — every file in project structure has Deliverable section ... |
| BC-AUDIT-2264 | pass-3-deep-templates-tools-rules.md | 2114 | BC-8.26.004 | SS-08 | Rules: story-completeness (14-check audit) | rules/story-completeness.md: check 3 — technical gotchas documented in Dev Notes (API quirks, v... |
| BC-AUDIT-2265 | pass-3-deep-templates-tools-rules.md | 2121 | BC-8.26.005 | SS-08 | Rules: story-completeness (14-check audit) | rules/story-completeness.md: check 4 — CI/CD workflows complete (workflow YAML deliverables, se... |
| BC-AUDIT-2266 | pass-3-deep-templates-tools-rules.md | 2128 | BC-8.26.006 | SS-08 | Rules: story-completeness (14-check audit) | rules/story-completeness.md: check 5 — README/user-facing-docs deliverable covers what-it-is/in... |
| BC-AUDIT-2267 | pass-3-deep-templates-tools-rules.md | 2135 | BC-8.26.007 | SS-08 | Rules: story-completeness (14-check audit) | rules/story-completeness.md: check 6 — hosting/infra decisions explicit (org/repo/visibility/br... |
| BC-AUDIT-2268 | pass-3-deep-templates-tools-rules.md | 2142 | BC-8.26.008 | SS-08 | Rules: story-completeness (14-check audit) | rules/story-completeness.md: check 7 — license stated explicitly + consistent across 5 surfaces |
| BC-AUDIT-2269 | pass-3-deep-templates-tools-rules.md | 2149 | BC-8.26.009 | SS-08 | Rules: story-completeness (14-check audit) | rules/story-completeness.md: check 8 — generated output specified (format/sort-order/edge-cases... |
| BC-AUDIT-2270 | pass-3-deep-templates-tools-rules.md | 2156 | BC-8.26.010 | SS-08 | Rules: story-completeness (14-check audit) | rules/story-completeness.md: check 9 — test fixtures defined with directory/config/expected-beh... |
| BC-AUDIT-2271 | pass-3-deep-templates-tools-rules.md | 2163 | BC-8.26.011 | SS-08 | Rules: story-completeness (14-check audit) | rules/story-completeness.md: check 10 — shell/script rules addressed (or `bash.md` excluded if ... |
| BC-AUDIT-2272 | pass-3-deep-templates-tools-rules.md | 2170 | BC-8.26.012 | SS-08 | Rules: story-completeness (14-check audit) | rules/story-completeness.md: check 11 — `.claude/rules/_index.md` references EXACTLY the rules ... |
| BC-AUDIT-2273 | pass-3-deep-templates-tools-rules.md | 2177 | BC-8.26.013 | SS-08 | Rules: story-completeness (14-check audit) | rules/story-completeness.md: check 12 — internal consistency (crate names / license text / file... |
| BC-AUDIT-2274 | pass-3-deep-templates-tools-rules.md | 2184 | BC-8.26.014 | SS-08 | Rules: story-completeness (14-check audit) | rules/story-completeness.md: check 13 — project-specific vs generic separation (tool deliverabl... |
| BC-AUDIT-2275 | pass-3-deep-templates-tools-rules.md | 2191 | BC-8.26.015 | SS-08 | Rules: story-completeness (14-check audit) | rules/story-completeness.md: check 14 — prerequisites listed (manual steps, repo creation, bran... |
| BC-AUDIT-2276 | pass-3-deep-templates-tools-rules.md | 2198 | BC-8.26.016 | SS-08 | Rules: story-completeness (14-check audit) | rules/story-completeness.md: process — read end-to-end, run each check, fix gaps one at a time ... |
| BC-AUDIT-2277 | pass-3-deep-templates-tools-rules.md | 2207 | BC-8.27.001 | SS-08 | Rules: worktree-protocol (branch hierarchy, merge protocol) | rules/worktree-protocol.md: branch hierarchy — main (releases) > develop (integration) > featur... |
| BC-AUDIT-2278 | pass-3-deep-templates-tools-rules.md | 2214 | BC-8.27.002 | SS-08 | Rules: worktree-protocol (branch hierarchy, merge protocol) | rules/worktree-protocol.md: story worktrees live in `.worktrees/STORY-NNN/` |
| BC-AUDIT-2279 | pass-3-deep-templates-tools-rules.md | 2221 | BC-8.27.003 | SS-08 | Rules: worktree-protocol (branch hierarchy, merge protocol) | rules/worktree-protocol.md: worktree creation — `git worktree add .worktrees/STORY-NNN -b featu... |
| BC-AUDIT-2280 | pass-3-deep-templates-tools-rules.md | 2228 | BC-8.27.004 | SS-08 | Rules: worktree-protocol (branch hierarchy, merge protocol) | rules/worktree-protocol.md: micro-commits per test pass; commit format `feat(STORY-NNN): <desc>` ... |
| BC-AUDIT-2281 | pass-3-deep-templates-tools-rules.md | 2235 | BC-8.27.005 | SS-08 | Rules: worktree-protocol (branch hierarchy, merge protocol) | rules/worktree-protocol.md: merge protocol — tests pass, PR to develop, adversarial+code review... |
| BC-AUDIT-2282 | pass-3-deep-templates-tools-rules.md | 2242 | BC-8.27.006 | SS-08 | Rules: worktree-protocol (branch hierarchy, merge protocol) | rules/worktree-protocol.md: wave integration — full test suite, adversarial review of wave diff... |
| BC-AUDIT-2283 | pass-3-deep-templates-tools-rules.md | 2249 | BC-8.27.007 | SS-08 | Rules: worktree-protocol (branch hierarchy, merge protocol) | rules/worktree-protocol.md: `.factory/` worktree is PERMANENT — never remove it |
| BC-AUDIT-2284 | pass-3-deep-templates-tools-rules.md | 2256 | BC-8.27.008 | SS-08 | Rules: worktree-protocol (branch hierarchy, merge protocol) | rules/worktree-protocol.md: cleanup rules — remove worktrees promptly, never force-remove with ... |
| BC-AUDIT-076 | pass-3-behavioral-contracts.md | 504 | BC-9.01.001 | SS-09 | Release and platform variant contracts | bump-version.sh accepts semver prerelease format (1.0.0-beta.N, 1.0.0-rc.N) |
| BC-AUDIT-077 | pass-3-behavioral-contracts.md | 510 | BC-9.01.002 | SS-09 | Release and platform variant contracts | chore commit (operator-staged) modifies only CHANGELOG.md |
| BC-AUDIT-078 | pass-3-behavioral-contracts.md | 516 | BC-9.01.003 | SS-09 | Release and platform variant contracts | release workflow's bot commit atomically writes binaries + plugin.json + marketplace.json |
| BC-AUDIT-079 | pass-3-behavioral-contracts.md | 522 | BC-9.01.004 | SS-09 | Release and platform variant contracts | 5-platform CI matrix is the build matrix; drift gated by check-platforms-drift.py |
| BC-AUDIT-080 | pass-3-behavioral-contracts.md | 528 | BC-9.01.005 | SS-09 | Release and platform variant contracts | hooks.json is gitignored; hooks.json.template + per-platform variants are committed |
| BC-AUDIT-2100 | pass-3-deep-templates-tools-rules.md | 1214 | BC-10.01.001 | SS-10 | compute-input-hash CLI | compute-input-hash: input-hash drift detection + remediation tool |
| BC-AUDIT-2101 | pass-3-deep-templates-tools-rules.md | 1221 | BC-10.01.002 | SS-10 | compute-input-hash CLI | compute-input-hash: I/O — argv command form, prints hash to stdout, diagnostics to stderr |
| BC-AUDIT-2102 | pass-3-deep-templates-tools-rules.md | 1228 | BC-10.01.003 | SS-10 | compute-input-hash CLI | compute-input-hash: input resolution against ARTIFACT_DIR + .factory/ search bases |
| BC-AUDIT-2103 | pass-3-deep-templates-tools-rules.md | 1235 | BC-10.01.004 | SS-10 | compute-input-hash CLI | compute-input-hash: refuses to hash with missing inputs |
| BC-AUDIT-2104 | pass-3-deep-templates-tools-rules.md | 1242 | BC-10.01.005 | SS-10 | compute-input-hash CLI | compute-input-hash: exit codes — 0 success/match, 1 usage/missing/scan-update-failed, 2 drift/s... |
| BC-AUDIT-2105 | pass-3-deep-templates-tools-rules.md | 1251 | BC-10.02.001 | SS-10 | emit-event CLI | emit-event: failure-tolerant structured event emitter |
| BC-AUDIT-2106 | pass-3-deep-templates-tools-rules.md | 1258 | BC-10.02.002 | SS-10 | emit-event CLI | emit-event: I/O — argv key=value pairs become JSON top-level fields; stdin ignored; no stdout/s... |
| BC-AUDIT-2107 | pass-3-deep-templates-tools-rules.md | 1265 | BC-10.02.003 | SS-10 | emit-event CLI | emit-event: auto-injects `ts`, `ts_epoch`, `schema_version=1` |
| BC-AUDIT-2108 | pass-3-deep-templates-tools-rules.md | 1272 | BC-10.02.004 | SS-10 | emit-event CLI | emit-event: auto-injects session_id from VSDD_SESSION_ID > CLAUDE_SESSION_ID |
| BC-AUDIT-2109 | pass-3-deep-templates-tools-rules.md | 1279 | BC-10.02.005 | SS-10 | emit-event CLI | emit-event: log-dir resolution — VSDD_LOG_DIR > main-worktree/.factory/logs > cwd/.factory/logs |
| BC-AUDIT-2110 | pass-3-deep-templates-tools-rules.md | 1286 | BC-10.02.006 | SS-10 | emit-event CLI | emit-event: VSDD_TELEMETRY=off short-circuit (line-1 kill switch) |
| BC-AUDIT-2111 | pass-3-deep-templates-tools-rules.md | 1293 | BC-10.02.007 | SS-10 | emit-event CLI | emit-event: atomic JSONL append exploits POSIX PIPE_BUF guarantee |
| BC-AUDIT-2112 | pass-3-deep-templates-tools-rules.md | 1302 | BC-10.03.001 | SS-10 | factory-dashboard CLI | factory-dashboard: live pipeline dashboard markdown renderer |
| BC-AUDIT-2113 | pass-3-deep-templates-tools-rules.md | 1309 | BC-10.03.002 | SS-10 | factory-dashboard CLI | factory-dashboard: I/O — CLI flags `--days N` (default 7), `--factory PATH` (default ./.factory) |
| BC-AUDIT-2114 | pass-3-deep-templates-tools-rules.md | 1316 | BC-10.03.003 | SS-10 | factory-dashboard CLI | factory-dashboard: STATE.md size warnings (>500 lines block, >200 lines info) |
| BC-AUDIT-2115 | pass-3-deep-templates-tools-rules.md | 1323 | BC-10.03.004 | SS-10 | factory-dashboard CLI | factory-dashboard: Health checks section probes existence of key paths |
| BC-AUDIT-2116 | pass-3-deep-templates-tools-rules.md | 1330 | BC-10.03.005 | SS-10 | factory-dashboard CLI | factory-dashboard: never crashes on missing dependencies (degrades gracefully) |
| BC-AUDIT-2117 | pass-3-deep-templates-tools-rules.md | 1339 | BC-10.04.001 | SS-10 | factory-obs CLI | factory-obs: lifecycle manager for the local observability docker-compose stack |
| BC-AUDIT-2118 | pass-3-deep-templates-tools-rules.md | 1346 | BC-10.04.002 | SS-10 | factory-obs CLI | factory-obs: 9 subcommands (up, regenerate, down, reset, status, logs, dashboard, register, unreg... |
| BC-AUDIT-2119 | pass-3-deep-templates-tools-rules.md | 1353 | BC-10.04.003 | SS-10 | factory-obs CLI | factory-obs: registry resolution — VSDD_OBS_REGISTRY > XDG_CONFIG_HOME/vsdd-factory/watched-fac... |
| BC-AUDIT-2120 | pass-3-deep-templates-tools-rules.md | 1360 | BC-10.04.004 | SS-10 | factory-obs CLI | factory-obs: docker-compose-safe subdir name = `<basename>-<8-char-shasum>` |
| BC-AUDIT-2121 | pass-3-deep-templates-tools-rules.md | 1367 | BC-10.04.005 | SS-10 | factory-obs CLI | factory-obs: register validates absolute path + .factory/ subdir presence; dedups; seeds header o... |
| BC-AUDIT-2122 | pass-3-deep-templates-tools-rules.md | 1374 | BC-10.04.006 | SS-10 | factory-obs CLI | factory-obs: exit-code semantics — 0 success, 1 usage/validation/missing-deps, 127 docker-compo... |
| BC-AUDIT-2123 | pass-3-deep-templates-tools-rules.md | 1383 | BC-10.05.001 | SS-10 | factory-query CLI | factory-query: canned queries against the observability event log |
| BC-AUDIT-2124 | pass-3-deep-templates-tools-rules.md | 1390 | BC-10.05.002 | SS-10 | factory-query CLI | factory-query: 6 subcommands (top, recent, grep, hooks, stats, reasons, help) |
| BC-AUDIT-2125 | pass-3-deep-templates-tools-rules.md | 1397 | BC-10.05.003 | SS-10 | factory-query CLI | factory-query: shared flag surface (--days N, --limit N, --severity, --type, --tsv) |
| BC-AUDIT-2126 | pass-3-deep-templates-tools-rules.md | 1404 | BC-10.05.004 | SS-10 | factory-query CLI | factory-query: portable date helpers handle BSD + GNU date |
| BC-AUDIT-2127 | pass-3-deep-templates-tools-rules.md | 1411 | BC-10.05.005 | SS-10 | factory-query CLI | factory-query: stats output enumerates total/blocks/warns/actions/unique_reasons/unique_hooks + l... |
| BC-AUDIT-2128 | pass-3-deep-templates-tools-rules.md | 1418 | BC-10.05.006 | SS-10 | factory-query CLI | factory-query: exit codes — 0 normal/empty results, 1 missing jq / unknown flag / unknown subco... |
| BC-AUDIT-2129 | pass-3-deep-templates-tools-rules.md | 1427 | BC-10.06.001 | SS-10 | factory-replay CLI | factory-replay: reconstructs a session's hook activity from the event log |
| BC-AUDIT-2130 | pass-3-deep-templates-tools-rules.md | 1434 | BC-10.06.002 | SS-10 | factory-replay CLI | factory-replay: 3 subcommands (sessions, show, latest, help) |
| BC-AUDIT-2131 | pass-3-deep-templates-tools-rules.md | 1441 | BC-10.06.003 | SS-10 | factory-replay CLI | factory-replay: pairing rule — sort by ts_epoch, group by session_id, latest events at top |
| BC-AUDIT-2132 | pass-3-deep-templates-tools-rules.md | 1448 | BC-10.06.004 | SS-10 | factory-replay CLI | factory-replay: render format — `ts  severity  hook  reason  context` |
| BC-AUDIT-2133 | pass-3-deep-templates-tools-rules.md | 1457 | BC-10.07.001 | SS-10 | factory-report CLI | factory-report: markdown-formatted summary of the observability event log |
| BC-AUDIT-2134 | pass-3-deep-templates-tools-rules.md | 1464 | BC-10.07.002 | SS-10 | factory-report CLI | factory-report: 3 subcommands (daily, weekly, range, help) |
| BC-AUDIT-2135 | pass-3-deep-templates-tools-rules.md | 1471 | BC-10.07.003 | SS-10 | factory-report CLI | factory-report: report shape — Summary table + Top reasons + Hook activity + (Wave merges) + (S... |
| BC-AUDIT-2136 | pass-3-deep-templates-tools-rules.md | 1478 | BC-10.07.004 | SS-10 | factory-report CLI | factory-report: portable BSD/GNU days-between calculation |
| BC-AUDIT-2137 | pass-3-deep-templates-tools-rules.md | 1487 | BC-10.08.001 | SS-10 | factory-sla CLI | factory-sla: agent.start/agent.stop pairing for subagent SLA tracking |
| BC-AUDIT-2138 | pass-3-deep-templates-tools-rules.md | 1494 | BC-10.08.002 | SS-10 | factory-sla CLI | factory-sla: 3 subcommands (durations, summary, open, help) |
| BC-AUDIT-2139 | pass-3-deep-templates-tools-rules.md | 1501 | BC-10.08.003 | SS-10 | factory-sla CLI | factory-sla: pairing implemented as O(n) awk stack per (session, subagent) key |
| BC-AUDIT-2140 | pass-3-deep-templates-tools-rules.md | 1508 | BC-10.08.004 | SS-10 | factory-sla CLI | factory-sla: percentile computation in awk (p50/p90/p99 + min/max/mean) |
| BC-AUDIT-2141 | pass-3-deep-templates-tools-rules.md | 1515 | BC-10.08.005 | SS-10 | factory-sla CLI | factory-sla: `open` surfaces orphan starts so silent agent failures are visible |
| BC-AUDIT-2142 | pass-3-deep-templates-tools-rules.md | 1524 | BC-10.09.001 | SS-10 | lobster-parse CLI | lobster-parse: thin yq + jq wrapper for .lobster YAML workflow files |
| BC-AUDIT-2143 | pass-3-deep-templates-tools-rules.md | 1531 | BC-10.09.002 | SS-10 | lobster-parse CLI | lobster-parse: I/O — argv positional `<file.lobster>` + optional `[jq-expression]` (default `.`... |
| BC-AUDIT-2144 | pass-3-deep-templates-tools-rules.md | 1538 | BC-10.09.003 | SS-10 | lobster-parse CLI | lobster-parse: missing yq/jq error to stderr + exit 1; missing file = exit 1; missing arg = exit 2 |
| BC-AUDIT-2145 | pass-3-deep-templates-tools-rules.md | 1547 | BC-10.10.001 | SS-10 | multi-repo-scan CLI | multi-repo-scan: detect multi-repo layout under .worktrees/ and emit JSON dependency report |
| BC-AUDIT-2146 | pass-3-deep-templates-tools-rules.md | 1554 | BC-10.10.002 | SS-10 | multi-repo-scan CLI | multi-repo-scan: 3 modes — `json` (default), `--list`, `--count` |
| BC-AUDIT-2147 | pass-3-deep-templates-tools-rules.md | 1561 | BC-10.10.003 | SS-10 | multi-repo-scan CLI | multi-repo-scan: manifest detection priority — Cargo.toml > package.json > pyproject.toml > go.... |
| BC-AUDIT-2148 | pass-3-deep-templates-tools-rules.md | 1568 | BC-10.10.004 | SS-10 | multi-repo-scan CLI | multi-repo-scan: empty directory = `{"repos": [], "count": 0}` not error |
| BC-AUDIT-2149 | pass-3-deep-templates-tools-rules.md | 1577 | BC-10.11.001 | SS-10 | research-cache CLI | research-cache: SHA-256-keyed disk cache for research-agent queries |
| BC-AUDIT-2150 | pass-3-deep-templates-tools-rules.md | 1584 | BC-10.11.002 | SS-10 | research-cache CLI | research-cache: 6 subcommands (get, put, has, key, clear, stats) |
| BC-AUDIT-2151 | pass-3-deep-templates-tools-rules.md | 1591 | BC-10.11.003 | SS-10 | research-cache CLI | research-cache: query normalization (whitespace-collapse + trim) before hashing |
| BC-AUDIT-2152 | pass-3-deep-templates-tools-rules.md | 1598 | BC-10.11.004 | SS-10 | research-cache CLI | research-cache: stats output `entries=N bytes=M dir=PATH` |
| BC-AUDIT-2153 | pass-3-deep-templates-tools-rules.md | 1605 | BC-10.11.005 | SS-10 | research-cache CLI | research-cache: clear is idempotent and removes only `*.json` (not the dir itself) |
| BC-AUDIT-2154 | pass-3-deep-templates-tools-rules.md | 1614 | BC-10.12.001 | SS-10 | wave-state CLI | wave-state: read-only query against .factory/stories/sprint-state.yaml |
| BC-AUDIT-2155 | pass-3-deep-templates-tools-rules.md | 1621 | BC-10.12.002 | SS-10 | wave-state CLI | wave-state: 4 subcommands (current, stories, ready, summary) |
| BC-AUDIT-2156 | pass-3-deep-templates-tools-rules.md | 1628 | BC-10.12.003 | SS-10 | wave-state CLI | wave-state: schema fallback — `.current_wave // .active_wave // 1`; both wave-shapes supported |
| BC-AUDIT-2157 | pass-3-deep-templates-tools-rules.md | 1635 | BC-10.12.004 | SS-10 | wave-state CLI | wave-state: ready exit code — 0 if all ready, 1 otherwise |
