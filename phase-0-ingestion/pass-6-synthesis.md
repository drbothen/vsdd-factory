# Pass 6: Synthesis & Validation — vsdd-factory

**Date:** 2026-04-25
**Reads:** Pass 0 inventory + Pass 1 architecture + Pass 2 domain model + Pass 3 behavioral contracts + Pass 4 NFR catalog + Pass 5 conventions.

## Executive summary

vsdd-factory is a **two-subsystem self-referential project** that ships as a single Claude Code marketplace plugin (`vsdd-factory@1.0.0-beta.4`). It delivers (a) a Verified Spec-Driven Development (VSDD) orchestration framework — 119 skills, 34 agents, 16 workflows, 108 templates, 44 bash hooks driving an autonomous SDLC pipeline — AND (b) a Rust hook dispatcher (10,226 LOC across 7 crates) that replaces the v0.79.x bash-hook dispatch layer with a compiled, cross-platform, WASM-plugin-based hook execution engine. The Rust dispatcher was greenfielded specifically to fix a Claude Code harness bug (`PostToolUse:Bash` matcher de-duplication) that v0.79.0–v0.79.4 could not work around in config alone.

The codebase has high internal consistency, dense Rust test coverage (180 tests across 18 source files plus ~1262 bats), and unusually thoughtful comments — many of which read as ADRs and explain not just what the code does but why. The single greatest risk surface is **drift between the design doc and the code-as-built**, and the plugin layer's depth (1270 skill subdirectories, 581 markdown files in `skills/`) was not exhaustively enumerated in this broad sweep.

## Key findings (top 5)

1. **Two subsystems, one repo, one version.** The Rust workspace (`crates/`) and the Claude Code plugin (`plugins/vsdd-factory/`) both stamp version `1.0.0-beta.4`. The plugin layer invokes the dispatcher binary at runtime via `hooks.json` — coupling is mediated by the activate skill, which copies the right `hooks.json.<platform>` into place and verifies the dispatcher binary.

2. **Self-reference is deliberate.** The product (VSDD framework) is what the engineering team uses to develop the product. The dispatcher's PostToolUse hooks gate the very file writes that produce these synthesis documents. (The validate-novelty-assessment.sh hook fired on every Write of these pass files this session.)

3. **Migration window is in flight.** As of v1.0.0-beta.4, ALL 45 entries in `hooks-registry.toml` route through `legacy-bash-adapter.wasm` — there are no native WASM ports active in production. The native ports (S-3.1 capture-commit-activity, S-3.2 capture-pr-activity, S-3.3 block-ai-attribution) are scoped for Tier E (post-beta.1 stable). `capture-commit-activity` exists as a stub crate (20 LOC) but has not yet replaced the legacy entry.

4. **Sink resilience is partially shipped.** The dispatcher has the constants (`internal.sink_circuit_opened`, `internal.sink_queue_full`, etc.) and the trait scaffolding, plus working file + otel-grpc drivers. The retry / circuit-breaker / dead-letter / HTTP / Datadog / Honeycomb features are scoped for Tier E (S-4.x) and have not shipped.

5. **Internal documentation is dense and load-bearing.** `.factory/specs/2026-04-24-v1.0-factory-plugin-kit-design.md` is a complete-design master with 5 ADRs and 7+ resolved open questions. `EPIC.md` decomposes 41 stories with explicit dependency tiers. `CHANGELOG.md` is 215KB and includes commit-level provenance per release. These artifacts are reliable and should be treated as primary inputs by downstream skills (create-prd, create-domain-spec, etc.).

## Confidence assessment

| Area | Confidence | Basis |
|---|---|---|
| Architecture | **HIGH** | All 7 crates + plugin layer mapped from source. Mermaid diagrams cite specific files/lines. |
| Domain Model | **HIGH (Half A) / MEDIUM (Half B)** | Half A (Rust runtime) entities pulled from typed code. Half B (Plugin layer) entities pulled from frontmatter convention; per-skill invariants thin. |
| Behavioral Contracts | **HIGH (Subsystem A) / MEDIUM (Subsystem B)** | 78/86 BCs are HIGH confidence (test-asserted). Subsystem B BCs are sparse (~7 cited; ~113 skills not yet quality-gated). |
| NFRs | **HIGH** | 76 NFRs cite specific config values or constants. Drift items explicitly flagged. |
| Conventions | **HIGH** | Sampled across both subsystems; consistency assessment table identifies pending vs universal patterns. |
| Drift / inconsistencies | **HIGH for the documented drift; UNKNOWN for the not-yet-walked plugin tree** | Drift items have explicit code-vs-design citations. Skill-by-skill walk has not happened — additional drift may exist there. |

## Drift report — design-doc-vs-code mismatches

### DRIFT-001: `read_file` host fn at StoreData-typed linker is a CAPABILITY_DENIED stub
- **Design says:** `read_file(path, max_bytes, timeout_ms) → Option<Bytes>` with capability gate via `path_allow`.
- **Code as built:** `crates/factory-dispatcher/src/host/read_file.rs` has the full implementation registered against `Linker<HostContext>`. `crates/factory-dispatcher/src/invoke.rs:447–474` (the StoreData-typed linker actually used in invocation) registers a stub returning `codes::CAPABILITY_DENIED` unconditionally with the comment "read_file isn't reachable by any in-tree plugin yet."
- **Severity:** MEDIUM. Behavior matches design (denied, with explicit code), but a future plugin needing read access will not find the path enabled even if `[hooks.capabilities.read_file] path_allow = [...]` is set.
- **Action:** Fix is mechanical (mirror the host/read_file.rs impl into invoke.rs's `setup_host_on_store_data`). The host/read_file.rs unit tests should not be deleted.

### DRIFT-002: `internal.sink_*` events declared as constants but never emitted
- **Design says:** Sink failures emit `internal.sink_error`; queue overflow emits `internal.sink_queue_full`; circuit opens/closes emit `internal.sink_circuit_*`.
- **Code as built:** Constants `INTERNAL_SINK_ERROR`, `INTERNAL_SINK_QUEUE_FULL`, `INTERNAL_SINK_CIRCUIT_OPENED`, `INTERNAL_SINK_CIRCUIT_CLOSED` declared in `internal_log.rs:67–70`. Failures are recorded into `Mutex<Vec<SinkFailure>>` per-sink (via `sink-file::SinkFailure` and `sink-otel-grpc`) but never converted to events. Wiring lives in S-4.4 (Tier E).
- **Severity:** MEDIUM. Drift acknowledged in source comments. Operators won't see sink degradation in their event stream until S-4.4 lands.

### DRIFT-003: Per-sink dedicated thread vs design's "shared runtime once S-1.6 lands"
- **Design says:** S-1.6 introduces a dispatcher-wide tokio runtime that all sinks can share.
- **Code as built:** S-1.6 has shipped (`Tier B.x` per EPIC.md). `sink-file/lib.rs` and `sink-otel-grpc/lib.rs` BOTH still spin up their own dedicated OS thread each, with their own `current_thread` runtime. Doc comments say "Swap to a shared `Handle` as a one-line edit once S-1.6 lands" but the swap has not been made.
- **Severity:** LOW (works correctly; just less efficient).

### DRIFT-004: Two parallel hook-routing tables (`hooks.json` and `hooks-registry.toml`)
- **Design implication:** v1.0 should be the single source of truth via `hooks-registry.toml`.
- **Code as built:** Both `hooks.json` (45-ish entries from v0.79.x) AND `hooks-registry.toml` (45 entries auto-generated by `scripts/generate-registry-from-hooks-json.sh`) exist. The legacy `hooks.json` is the file Claude Code currently reads on activation; the dispatcher consumes `hooks-registry.toml`. Activate skill writes `hooks.json.<platform>` over `hooks.json` — that template points only at the dispatcher, but legacy `hooks.json` is still referenced by older flows.
- **Severity:** MEDIUM (transitional; documented in registry header "DO NOT HAND-EDIT during the v0.79.x → v1.0 migration").

### DRIFT-005: HTTP / Datadog / Honeycomb sinks declared in design but not implemented
- **Design says:** v1.0 supports `file`, `http`, `otel-grpc`, `datadog`, `honeycomb` sink types.
- **Code as built:** Only `file` and `otel-grpc` are shipped (sink-core, sink-file, sink-otel-grpc crates). Unknown driver types are warned and skipped (BC-AUDIT-042). HTTP / Datadog / Honeycomb scoped for Tier E (S-4.1, S-4.2, S-4.3).
- **Severity:** LOW (graceful unknown-type handling; design clearly stages these as rc.1, not beta.1).

### DRIFT-006: Phase 5 events (SessionStart / SessionEnd / WorktreeCreate / WorktreeRemove / PostToolUseFailure) not yet wired
- **Design says:** v1.0 final wires these.
- **Code as built:** `hooks.json.template` *does* register the dispatcher for SessionStart, SessionEnd, SubagentStop, etc. — so these are routable. But there are no plugin-layer hooks listening for `session.started` / `session.ended` events. CHANGELOG declares them in scope for `1.0.0` but not yet shipped (we're at beta.4).
- **Severity:** LOW (planned; clear timeline).

### DRIFT-007: `dispatcher.shutting_down` constant defined but never emitted
- **Code:** `internal_log.rs:58` declares `DISPATCHER_SHUTTING_DOWN`. `main.rs` does not emit it on exit. Reasonable in a per-event short-lived process.
- **Severity:** LOW (cosmetic; could be a spec-level "remove or wire" follow-up).

### DRIFT-008: `plugin.loaded` / `plugin.load_failed` constants never emitted from plugin_loader.rs
- **Code:** Constants declared in `internal_log.rs`. `plugin_loader::PluginCache::get_or_compile` is what would emit them. Sampled the executor's load-error fallback (`emit_lifecycle` for `PluginResult::Crashed`) but no dedicated `plugin.loaded` event fires.
- **Severity:** LOW.

### DRIFT-009: Adversary SHA-currency gate is opt-in (verify-sha-currency.sh template)
- **Design / CHANGELOG:** v1.0.0-beta.4 ships the verify-sha-currency.sh template + state-burst skill. The hook gate is no-op when project hasn't installed the verify-sha-currency hook.
- **Severity:** LOW. Working-as-intended for a new feature; just document the opt-in clearly.

### DRIFT-010: 26 unported bash hooks block Windows users
- **Design:** Windows users get full support only for the 4 native-WASM ports (which themselves are not yet shipped; see DRIFT-006 + Tier E).
- **Code as built:** All 44 hook scripts route through legacy-bash-adapter, which requires git-bash on Windows.
- **Severity:** MEDIUM-HIGH for Windows users. Not a regression, but the gap has widened relative to design's beta.1 ambition.

## Story-coverage map

Cross-referencing `.factory/stories/v1.0/` (41 stories) against shipped state from CHANGELOG and code presence:

| Story | Title | Tier | Status | Evidence |
|---|---|---|---|---|
| S-0.1 | bump-version.sh prerelease support | A | **SHIPPED** | CHANGELOG (v1.0.0-beta.1+); script exists at `scripts/bump-version.sh` |
| S-0.2 | Release workflow prerelease handling | A | **SHIPPED** | CHANGELOG; CI runs are producing tagged prereleases |
| S-0.3 | Activation skill platform detection | A | **SHIPPED** | `skills/activate/SKILL.md` + detect-platform.sh |
| S-0.4 | hooks.json.template + CI generation | A | **SHIPPED** | Template + 5 platform variants in repo; `scripts/generate-hooks-json.sh` |
| S-0.5 | Docs scaffolding | A | **SHIPPED** | `docs/guide/` has 30 reference docs |
| S-1.1 | Cargo workspace + CI scaffolding | B.0 | **SHIPPED** | Cargo.toml + 7 workspace member crates; `ci/platforms.yaml` |
| S-1.2 | factory-dispatcher core (stdin, TOML load, routing) | B.x | **SHIPPED** | `crates/factory-dispatcher/src/{main,registry,routing,payload}.rs`; 9+ tests |
| S-1.3 | hook-sdk crate | B.x | **SHIPPED** | `crates/hook-sdk` + macros; 20 tests; published-ready |
| S-1.4 | Host function surface | B.x | **SHIPPED** (with DRIFT-001 caveat on read_file at invoke.rs) | `crates/factory-dispatcher/src/host/*.rs`; 18 host integration tests |
| S-1.5 | wasmtime integration + epoch/fuel | B.x | **SHIPPED** | `crates/factory-dispatcher/src/{engine,invoke}.rs`; tested |
| S-1.6 | tokio + parallel-within-tier | B.x | **SHIPPED** | `crates/factory-dispatcher/src/executor.rs`; spawn_blocking pattern; 4 tests |
| S-1.7 | dispatcher-internal.jsonl | B.x | **SHIPPED** | `crates/factory-dispatcher/src/internal_log.rs`; 8 tests; 17 event constants |
| S-1.8 | sink-file driver | B.x | **SHIPPED** | `crates/sink-file/`; 17 tests; daily rotation working |
| S-1.9 | sink-otel-grpc driver | B.x | **SHIPPED** | `crates/sink-otel-grpc/`; 13 tests; mock OTLP receiver in integration test |
| S-2.1 | legacy-bash-adapter WASM plugin | C | **SHIPPED** | `crates/hook-plugins/legacy-bash-adapter/`; 14 tests; CHANGELOG v1.0.0-beta.1 |
| S-2.2 | hooks-registry.toml auto-generation | C | **SHIPPED** | `scripts/generate-registry-from-hooks-json.sh`; 6 generate-registry bats; idempotent |
| S-2.3 | Cross-platform CI matrix | C | **SHIPPED** | 5-platform builds in `ci/platforms.yaml`; `scripts/check-platforms-drift.py` |
| S-2.4 | Binary commit automation | C | **SHIPPED** | per-platform binaries committed under `plugins/vsdd-factory/hooks/dispatcher/bin/<platform>/` |
| S-2.5 | hook-sdk publish to crates.io (0.1.0) | C | **PARTIAL** | CHANGELOG v1.0.0-beta.1: "SDK publish dry-run clean" but real `cargo publish` deferred |
| S-2.6 | Activation skill integrates with hooks.json variants | C | **SHIPPED** | activate skill confirmed working in CHANGELOG; detect-platform + apply-platform helpers |
| S-2.7 | Regression test suite validation | C | **SHIPPED** | `regression-v1.0.bats` (11 tests); CHANGELOG v1.0.0-beta.1 |
| S-2.8 | 1.0.0-beta.1 release gate | D | **SHIPPED** | tagged 1.0.0-beta.1 → … → beta.4 |
| S-3.1 | Port capture-commit-activity to WASM | E | **NOT SHIPPED** (stub only) | `crates/hook-plugins/capture-commit-activity/src/lib.rs` is a 20-LOC stub returning 0 |
| S-3.2 | Port capture-pr-activity to WASM | E | **NOT SHIPPED** | No crate yet under `hook-plugins/` for this hook |
| S-3.3 | Port block-ai-attribution to WASM | E | **NOT SHIPPED** | No crate yet |
| S-3.4 | emit_event as host function refactor | E | **PARTIAL** | The host fn IS implemented (`host/emit_event.rs` + `invoke.rs::emit_event`); but bash hooks still call `bin/emit-event` shell tool. Refactor incomplete. |
| S-4.1 | sink-http driver | E | **NOT SHIPPED** | No crate; warn-and-skip handler in sinks/mod.rs |
| S-4.2 | sink-datadog driver | E | **NOT SHIPPED** | No crate; warn-and-skip |
| S-4.3 | sink-honeycomb driver | E | **NOT SHIPPED** | No crate; warn-and-skip |
| S-4.4 | Per-sink retry + circuit breaker | E | **NOT SHIPPED** | Constants declared, never emitted; SinkFailure recorded but not retried |
| S-4.5 | Dead letter queue | E | **NOT SHIPPED** | Design has `[sinks.*.dead_letter]` block; no impl |
| S-4.6 | Per-sink routing filters + tag enrichment | E | **PARTIAL** | RoutingFilter implemented in sink-core; tag enrichment scoped to sink-core via `tags` field; per-sink wiring partial |
| S-4.7 | E2E observability integration tests | E | **NOT SHIPPED** | sinks_otel_grpc.rs + sinks_file_integration.rs cover unit/integration but full E2E pending S-4.x sinks shipping |
| S-4.8 | 1.0.0-rc.1 release gate | F | **NOT REACHED** | Pending Tier E completion |
| S-5.1 | SessionStart hook | G | **NOT SHIPPED** | hooks.json.template registers dispatcher on SessionStart but no plugin reacts |
| S-5.2 | SessionEnd hook | G | **NOT SHIPPED** | same |
| S-5.3 | WorktreeCreate / WorktreeRemove | G | **NOT SHIPPED** | no plugin |
| S-5.4 | PostToolUseFailure | G | **NOT SHIPPED** | no plugin |
| S-5.5 | Migration guide (0.79.x → 1.0) | G | **PARTIAL** | `docs/guide/migrating-from-0.79.md` skeleton exists per CHANGELOG; full guide TBD |
| S-5.6 | Semver commitment docs | G | **NOT SHIPPED** | scoped for 1.0.0 final |
| S-5.7 | 1.0.0 release gate | H | **NOT REACHED** | Pending Tier G |

**Tier completion summary:** Tier A (5/5), Tier B.0 (1/1), Tier B.x (8/8), Tier C (6/7 — S-2.5 partial), Tier D (1/1) → **beta.1 gate met; beta.2/3/4 follow-ups landed**. Tier E in flight (1/11 partial: S-3.4); Tiers F/G/H not reached. The 18 stories at Tier E + Tier G have NOT shipped — that's the open backlog.

## Lessons for vsdd-factory (P0/P1/P2/P3)

### P0 — must address before next backwards-compat-breaking release

- **L-P0-001:** Wire `read_file` impl through to the StoreData-typed linker (eliminate DRIFT-001). Safe to do without bumping HOST_ABI_VERSION.
- **L-P0-002:** Eliminate the parallel-routing-table state. Either retire `hooks.json` (operate entirely from `hooks-registry.toml`) or document `hooks.json` as the bootstrap-only path. Today's state (DRIFT-004) is fragile.
- **L-P0-003:** Backfill BCs for the 24+ validate-* hooks that gate writes. The validate-novelty-assessment.sh hook is itself unspecified by a BC — it caught my output structure 3 times before I matched it. That's a living example of "the hook is the spec; the spec doesn't exist."

### P1 — meaningfully improves quality / reduces risk

- **L-P1-001:** Wire `internal.sink_*` event emission (S-4.4 work). Without these, operators have no audit trail of sink degradation.
- **L-P1-002:** Apply `#[deny(missing_docs)]` to factory-dispatcher and hook-sdk (currently only sink crates have it). Code is well-documented anyway; enforcement prevents regression.
- **L-P1-003:** Add a "subsystem coverage" check to `brownfield-ingest`: when ingesting a multi-subsystem repo (Rust + plugin layer like this one), the analysis must explicitly enumerate which subsystem each pass covered. This Phase 0 ingestion almost missed Subsystem B's depth.
- **L-P1-004:** Surface `dispatcher.shutting_down` and `plugin.loaded` events (DRIFT-007, DRIFT-008). They're already declared; emission is a 1-line addition.
- **L-P1-005:** Adopt the per-validator BC pattern. Each of the 24+ validate-* hooks is a standalone behavioral contract; today they're documented only by their script body.
- **L-P1-006:** Document the `legacy-bash-adapter` wind-down plan. Each hook ported reduces adapter reach; the plan should track which hooks port next, latency-driven.

### P2 — quality improvements (worth doing but not blocking)

- **L-P2-001:** Migrate per-sink dedicated threads to a shared dispatcher tokio runtime (DRIFT-003). Comments already promise this.
- **L-P2-002:** Catalog the 110 slash commands and their target skills as a navigable index. Today they're a flat directory.
- **L-P2-003:** Plugin manifest version (`plugin.json`) and Cargo workspace package versions are NOT linked structurally. They drift in lockstep by convention but have no compile-time gate. A pre-release script could enforce.
- **L-P2-004:** Test naming sometimes diverges from the descriptive-sentence convention (most tests good, some short). Linter-level enforcement.
- **L-P2-005:** Centralize the "negative-error-code" set: `codes::*` lives in factory-dispatcher::host but the SDK's `host::HostError::from_code` mirrors it. Drift between them = silent breakage. Consider extracting to a `host-codes` crate or pinning by integration test.

### P3 — nice-to-haves, future direction

- **L-P3-001:** WASI preview-2 migration (declared as v2.0 in design ADR-003).
- **L-P3-002:** Plugin signing / marketplace discovery infra (declared non-goal for v1.0).
- **L-P3-003:** seccomp / AppArmor / Landlock sandboxing on Linux (v2+ per design).
- **L-P3-004:** Arg-filtering for `exec_subprocess` (today's gate is binary_allow only; arg-level is post-1.0 if attack scenarios materialize per design Q4).
- **L-P3-005:** Replace `bin/emit-event` shell tool with a typed Rust binary that calls into the SDK's host-fn shim (S-3.4 ambition).

## Cross-pass consistency checks

- **Pass 2 ↔ Pass 3 (entities ↔ contracts):** All Half-A entities (HookPayload, Registry, RegistryEntry, PluginResult, etc.) have BCs. ✅
- **Pass 1 ↔ Pass 4 (architecture ↔ NFRs):** Architecture-promised parallel-within-tier is reflected in NFR-PERF/SCALE catalog. ✅
- **Pass 2 ↔ Pass 5 (entities ↔ conventions):** Builder pattern + thiserror + deny_unknown_fields applied uniformly to entity types. ✅
- **Pass 3 ↔ Pass 4 (contracts ↔ NFRs):** All declared NFR caps (STDERR_CAP_BYTES, MAX_OUTPUT_BYTES, BASH_TIMEOUT_MS, default fuel_cap) have asserting BCs. ✅
- **Pass 4 ↔ Pass 6 (NFRs ↔ drift):** Pending NFRs in Pass 4 line up with not-shipped stories in Pass 6 — the absences are consistent across both views. ✅
- **Half B coverage gap is consistent across passes 2/3/5:** All three flagged the plugin tree as under-walked relative to depth.

## Inconsistencies found across passes

- **Pass 0 reported 119 SKILL.md files; Pass 5 reported 581 markdown files in skills/.** The two numbers are not contradictory (one counts SKILL.md only, the other counts all .md including nested step files), but the relationship was not stated explicitly in either pass. Pass 0 is the canonical "skills" count.
- **Agent count: Pass 0 says 34 (33 .md files + 1 orchestrator/ directory); Pass 1 says 33+orchestrator; Pass 2 lists 34 names.** All consistent if you accept that orchestrator has its own subdirectory but counts as one identity. ✅
- **Hook count: Pass 0 says 44 (top-level hooks/*.sh); Pass 2 categorizes "30+ existing hooks" per the design doc.** 44 vs 30+ — the 30 figure is from the v0.79.x baseline; current count is 44 (some validators added later). Document explicitly: 44 is now-state, 30 is design-time baseline.

## Remaining gaps for deepening rounds

These should be the targets for Phase B convergence rounds:

### Subsystem B coverage gaps (high priority)
- **GAP-A:** Per-skill quality-gate BC extraction. 113 skills not yet quality-gated (only 6 sampled in Pass 3). Skills with explicit "Quality Gate" sections: brownfield-ingest, code-delivery, deliver-story, wave-gate, release, create-prd. Each likely has 3–8 BCs.
- **GAP-B:** Per-validator hook BC extraction. 24+ validate-* hooks collapsed into BC-AUDIT-068. Each is a discrete contract.
- **GAP-C:** Workflow `.lobster` step semantics. 16 workflows × 5–50 steps each; preconditions and on_failure semantics are not yet enumerated.
- **GAP-D:** Template-vs-skill compliance map. Which template each skill writes to, with reverse cross-reference. Today templates/ is a flat 108-file directory.

### Subsystem A coverage gaps
- **GAP-E:** Plugin-loader detail. PluginCache invalidation + caching semantics not deeply read this pass.
- **GAP-F:** sinks/router.rs (Router placeholder) — what's it for, what's it currently doing.
- **GAP-G:** SDK-side `host` module — the SDK's plugin-author-facing host fn shims (host.rs, ffi.rs) sample-read only.
- **GAP-H:** legacy-bash-adapter behavior under stderr/stdout interleaving and exit-code edge cases.
- **GAP-I:** sink-otel-grpc batching + flush behavior under load.

### Cross-cutting gaps
- **GAP-J:** Bin tools' contracts (`emit-event`, `factory-query`, `factory-replay`, `wave-state`, `lobster-parse`) — their I/O surfaces inform observability ingestion.
- **GAP-K:** docs/guide/ 30-file reference cross-walk against actual code state. Are the docs current with beta.4?
- **GAP-L:** ci/ directory contents (only 1 file: `platforms.yaml`) — full CI workflow definitions live in `.github/workflows/` (per `.github/` directory presence). Not opened this pass.
- **GAP-M:** rules/ directory under plugins (10+ rule files). Not deeply walked.

## Convergence report

| Subsystem / Pass | Coverage | Convergence priority |
|---|---|---|
| Subsystem A — Rust dispatcher core | DENSE | already converged for routing/registry/executor; 1–2 deepening rounds for plugin_loader + sinks/router |
| Subsystem A — host fns | DENSE | already converged; deepen on read_file to address DRIFT-001 |
| Subsystem A — sink drivers | MEDIUM-DENSE | 1 round on otel-grpc batching + retry semantics |
| Subsystem A — SDK | MEDIUM | 1 round on host module + ffi |
| Subsystem A — legacy-bash-adapter | MEDIUM | 1 round on edge cases |
| Subsystem B — agents | THIN | 2–3 rounds; 34 agents × per-agent quality gates |
| Subsystem B — skills | THIN | 3–5 rounds; 119 skills × quality gates + step decomposition |
| Subsystem B — workflows | THIN | 1–2 rounds; 16 workflows × step semantics |
| Subsystem B — hooks (bash) | THIN | 2–3 rounds; per-validator BCs |
| Subsystem B — bin tools | THIN | 1 round on observability tooling I/O contracts |
| Subsystem B — templates | UNINSPECTED | 1 round to map template → producing-skill → consuming-artifact |
| Cross-cutting — design doc cross-ref | DONE in Pass 6 drift report | done |
| Cross-cutting — release / CI | THIN | 1 round on `.github/workflows/` + scripts/ |

**Recommended deepening order:** First tier (priority): per-skill BC extraction (GAP-A) and per-validator BC extraction (GAP-B), because they pad out Subsystem B's BC count by ~150 more. Second tier: workflow semantics (GAP-C) and template map (GAP-D). Third tier: Subsystem A deepening on the remaining drift items.

## Recommendations for downstream skills

- **create-prd / create-domain-spec:** Use `.factory/specs/2026-04-24-v1.0-factory-plugin-kit-design.md` as primary input. Its 5 ADRs + Open-Questions resolutions ARE the design intent. Cross-check the present pass-6 drift report before assuming any feature exists in code.
- **create-architecture:** Pass 1's component catalog + layer diagram is the starting point. Pass 5's pattern catalog informs the "design patterns" section.
- **decompose-stories:** EPIC.md already exists and is high-quality. Use it as input, not a thing to recreate.
- **semport-analyze:** Not directly applicable (no language port in flight). Could be used downstream if a future operator wants to port the dispatcher to Go.
- **adversary:** Best targets for adversarial review are the drift items (DRIFT-001 read_file stub, DRIFT-002 sink event emission absence) and the Subsystem B coverage gaps.

## State Checkpoint
```yaml
pass: 6
status: complete
key_findings: 5
drift_items: 10
story_coverage_assessed: 41/41
shipped_stories: 22 (full) + 4 (partial)
not_shipped_stories: 15
remaining_gaps_for_deepening: 13 (GAP-A through GAP-M)
cross_pass_consistency_checks: 6 (5 pass, 1 reconciled)
inconsistencies_found: 3 (all reconciled with clarifying note)
timestamp: 2026-04-25
next_phase: B (convergence deepening)
```

## Novelty Assessment

| Field | Value |
|-------|-------|
| **Pass** | 6 |
| **Novelty score** | SUBSTANTIVE — first synthesis for this repo |
| **Trajectory** | First sweep across all 6 prior passes. 10 drift items identified, 41-row story coverage map produced, 13 deepening-round gaps catalogued, 5 P0/P1 lessons surfaced. No prior pass-6 artifact existed. |
| **Verdict** | FINDINGS_REMAIN — Subsystem B is significantly under-walked (per GAP-A through GAP-D). Convergence rounds remain to close the gaps. CONVERGENCE_REACHED is not declarable until Phase B convergence loops complete on all 6 prior passes. |
