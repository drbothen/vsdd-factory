---
document_type: phase-0-final-synthesis
level: ops
version: "1.0"
status: complete
producer: codebase-analyzer
phase: 0
total_bcs: 1851
inputs:
  - pass-0-inventory.md
  - pass-1-architecture.md
  - pass-2-domain-model.md
  - pass-3-behavioral-contracts.md
  - pass-3-behavioral-contracts-deep-r1.md
  - pass-3-deep-skills-batch-1.md
  - pass-3-deep-skills-batch-2.md
  - pass-3-deep-skills-batch-3.md
  - pass-3-deep-agents.md
  - pass-3-deep-hooks.md
  - pass-3-deep-workflows.md
  - pass-3-deep-templates-tools-rules.md
  - pass-3-deep-rust-tests.md
  - pass-4-nfr-catalog.md
  - pass-5-conventions.md
  - pass-6-synthesis.md
  - extraction-validation.md
traces_to: "phase-1-spec-crystallization"
---

# Phase 0 Final Synthesis — vsdd-factory

## 1. Executive summary

vsdd-factory is a **two-subsystem self-referential project** distributed as a single Claude Code marketplace plugin (`vsdd-factory@1.0.0-beta.4`, commit `1907d8f`, shipped 2026-04-25). The repo simultaneously contains the **engine** (a Rust hook dispatcher, 10,226 LOC across 7 workspace crates, runtime under wasmtime + WASI preview 1) and the **product** (a 119-skill / 34-agent / 16-workflow / 105-template / 44-bash-hook orchestration framework that implements Verified Spec-Driven Development as an autonomous SDLC pipeline). The user has explicitly confirmed engine and product are part of the same product — they share one version, one CHANGELOG, one marketplace entry, and the same Phase 0 synthesis artifact. **Subsystem A** (Rust dispatcher) and **Subsystem B** (orchestration framework) are first-class equals, not nested. They are coupled at exactly four contract surfaces: the stdin envelope, `hooks-registry.toml`, `observability-config.toml`, and the plugin ABI (`HOST_ABI_VERSION = 1`).

The Rust dispatcher was greenfielded in 2026-04 to fix a Claude Code harness bug (`PostToolUse:Bash` matcher de-duplication) that v0.79.0–v0.79.4 could not work around in configuration alone. It now provides a single internal-routing entry point, parallel-within-tier / sequential-between-tiers execution, capability-deny-by-default host functions, multi-sink observability fan-out, and always-on self-telemetry independent of sink configuration. The orchestration framework on top encodes the team's VSDD methodology — broad-then-deep code analysis, strict-binary novelty assessment, brief → domain-spec → PRD → architecture → stories → red/green/refactor delivery, wave-gate quality bookkeeping, and adversarial review with SHA-currency gating.

**Maturity assessment:** v1.0.0-beta.4 has shipped Tiers A through D (22 stories) — all of Phase 0 infrastructure prep, all of the dispatcher core (8 stories), and all of legacy-adapter / cross-platform / regression-test prep (Tier C, 6 of 7). 4 stories are PARTIAL, and 15 stories are NOT-SHIPPED — the entire Tier E parallel fan-out (S-3.x native ports + S-4.x advanced sinks + S-4.7 E2E observability) and Tiers F/G/H release gates remain. The codebase is internally consistent, densely documented (185 Rust tests + ~1262 bats), well-typed, and well-tested. The single greatest risk surface is **drift between the v1.0 master design and the code-as-built** (10 confirmed drift items, all with code citations). Phase 0 ingestion is complete and validated at 97.6% behavioral confirmation; the catalog is ready to feed Phase 1 spec crystallization.

## 2. Product identity

**Mission.** vsdd-factory delivers a self-orchestrating, observable, sandbox-aware development pipeline for solo developers and small teams using Claude Code. It replaces ad-hoc hook scripting with a typed, capability-gated, cross-platform dispatcher; it replaces ad-hoc methodology with declarative workflows that drive specialist agents through a 0–7 phase SDLC.

**Two-subsystem nature (single product, dual implementation):**

- **Subsystem A — Rust dispatcher (compiled artifact).** A `factory-dispatcher` binary, shipped per-platform (darwin-arm64, darwin-x64, linux-x64, linux-arm64, windows-x64), invoked by Claude Code per hook event. Reads stdin JSON envelope, loads `hooks-registry.toml`, matches plugins by event + tool regex, groups by priority tier, executes tiers sequentially with parallel WASM plugin invocation per tier, fans out events to enabled sinks (file, otel-grpc), writes always-on self-telemetry. 7 workspace crates: `factory-dispatcher`, `hook-sdk`, `hook-sdk-macros`, `sink-core`, `sink-file`, `sink-otel-grpc`, `hook-plugins/{legacy-bash-adapter, capture-commit-activity}`.

- **Subsystem B — VSDD orchestration framework (declarative + procedural-prose).** Markdown skills, markdown agents, Lobster-YAML workflows, markdown templates, bash hooks, bin tools. The orchestrator (a Claude Code main-thread agent) reads `.lobster` workflow files as data and dispatches specialist sub-agents (architect, business-analyst, implementer, adversary, etc.) through pipeline phases. Bash hooks gate file writes, capture commits/PRs, validate state, and bookkeep waves.

The two subsystems share one version (`1.0.0-beta.4`), one marketplace entry (`.claude-plugin/marketplace.json`), and one CHANGELOG. The Rust dispatcher is wired in by the activate skill at install time; the orchestration framework drives its development.

**Primary users.** Solo developers and small teams running Claude Code who want autonomous spec-driven development with observable, gated execution.

**Key differentiators.** (a) Self-orchestrating LLM pipeline — the orchestrator runs the SDLC, not the human. (b) Sandbox-aware — wasmtime + WASI preview-1 + capability deny-by-default + bounded fuel + epoch interruption. (c) Multi-sink observable — file, OTLP/gRPC, and (post-rc.1) HTTP/Datadog/Honeycomb fan-out, with always-on self-telemetry. (d) Cross-platform with explicit per-platform binary bundling. (e) Strict-binary novelty assessment for convergence — findings are SUBSTANTIVE or NITPICK, no gradient. (f) Brownfield ingest as a first-class workflow — exactly what produced this synthesis.

**Current shipping milestone.** `1.0.0-beta.4` shipped 2026-04-25 (commit `1907d8f`). Recent releases: `beta.4` (cache fix + plugin stderr capture + adversary SHA-currency gate), `beta.3`, `beta.2`, `beta.1` (Tier A+B+C+D close — 23 stories shipped).

## 3. Bounded contexts and complexity ranking

| Bounded Context | Subsystem | Code Surface | BC Density | Complexity | Notes |
|-----------------|-----------|--------------|------------|------------|-------|
| Rust dispatcher core | A | 6,377 LOC across 18 files | ~80 BCs | **HIGH** | wasmtime engine + epoch ticker, registry parsing, routing, executor (parallel-within-tier), invoke (Store/WASI/fuel/epoch), plugin_loader, internal_log (17 event constants), payload (alias-tolerant) |
| Sink observability | A | 2,382 LOC across 3 crates | ~50 BCs | **MEDIUM** | sink-core trait + RoutingFilter, sink-file (daily-rotated JSONL, mpsc queue), sink-otel-grpc (dedicated current_thread runtime, OTLP/gRPC batches) |
| Hook SDK | A | 1,058 LOC across 2 crates | ~25 BCs | **LOW-MED** | `#[hook]` proc-macro, HookPayload, HookResult sum type, host fn shims, FFI |
| Skill orchestration | B | 119 SKILL.md (581 markdown total in skills/, 1270 subdirs) | ~553 BCs | **HIGH** | Pipeline driver — covers brownfield-ingest, semport-analyze, create-* family, deliver-story, wave-gate, release, factory-* observability, scaffold-claude-md, holdout-eval, formal-verify, decompose-stories, etc. |
| Agent identity | B | 34 agents (33 .md + orchestrator/) | ~171 BCs | **MEDIUM** | LLM persona contracts: orchestrator + adversary, architect, business-analyst, codebase-analyzer, code-reviewer, consistency-validator, dtu-validator, implementer, pr-manager, product-owner, security-reviewer, spec-steward, etc. |
| Hook ecosystem | B | 44 .sh top-level + 45 registry entries | ~176 BCs | **MEDIUM** | Bash + dispatcher boundary. Gates (PreToolUse), capture (PostToolUse), validators (24 validate-*), lifecycle (SubagentStop/Stop) |
| Workflow definitions | B | 16 .lobster (8 modes + 8 phases) | ~445 BCs | **MEDIUM-HIGH** | DAG semantics with `depends_on`, `on_failure`, `max_retries`, `timeout`, `cost_tracking`. Modes: greenfield, brownfield, feature, maintenance, discovery, planning, multi-repo, code-delivery |
| Template registry | B | 105 top-level (135 incl. subdirs) | ~130 BCs | **LOW** | Structural skeletons for stories, BCs, PRDs, architecture, ADRs, design docs, etc. |
| Rust test scaffolding | A | 185 #[test]/#[tokio::test] | ~90 BCs | **MEDIUM** | Unit (per-file `#[cfg(test)] mod tests`), integration (`crates/factory-dispatcher/tests/*.rs`), bats (~1262), regression-v1.0.bats (11) |
| Rules + tools (cross-cutting) | B | 9 rules + 12 bin tools | ~143 BCs | **LOW-MED** | Cross-cutting policies; observability bin family (factory-{dashboard,obs,query,replay,report,sla}), workflow infra (wave-state, lobster-parse, compute-input-hash) |

**Total: ~1,851 BCs across 10 contexts.** The two highest-complexity contexts are the Rust dispatcher core (Subsystem A's heart) and skill orchestration (Subsystem B's heart). Both span more than 500 LOC of behavioral surface and require the most careful translation into formal BCs during Phase 1 backfill.

## 4. Critical design decisions (extracted ADRs)

These decisions are LOAD-BEARING for the product. Each is grounded in either a formal ADR from the v1.0 master design, an Open-Question resolution, or an as-built code citation.

### ADR-001: Rust for the dispatcher
- **Decision:** Compile a Rust binary; ship per-platform; commit binaries to repo via release CI.
- **Rationale:** ~1-5ms startup, no runtime dep, type+memory safety, fits team's Rust direction. Go was close runner-up but no project presence; Python/Node not guaranteed on operator machines; Bash fails Windows.
- **Consequences:** 5-platform CI matrix; binaries committed (~3GB ceiling estimated over 3 years per design Q1; orphan-branch fallback documented).
- **Status:** **IN-EFFECT.** Source: `/Users/jmagady/Dev/vsdd-factory/.factory/legacy-design-docs/2026-04-24-v1.0-factory-plugin-kit-design.md:399-416`. Per-platform binaries live at `plugins/vsdd-factory/hooks/dispatcher/bin/<platform>/factory-dispatcher[.exe]`.

### ADR-002: WASM plugin ABI (wasmtime + WASI preview 1)
- **Decision:** Each hook is a `.wasm` module loaded by wasmtime; language-agnostic at the boundary; Rust is the reference SDK.
- **Rationale:** ~100-200KB per plugin × ~30 plugins = ~6MB total (vs ~300MB for native cross-compiled binaries); sandbox-by-default; ~1ms instantiation. Embedded scripting (Rhai/Lua) rejected for unfamiliar surface.
- **Consequences:** Plugins can't access network directly — must use `exec_subprocess` (cap-gated) to shell out. WASI preview-2 (wasi-http, wasi-sockets) explicitly deferred.
- **Status:** **IN-EFFECT.** Source: design doc lines 418-444. `wasmtime = "44.0"` + `wasmtime-wasi = "44.0"` pinned in `Cargo.toml [workspace.dependencies]`.

### ADR-003: WASI preview 1 (preview 2 deferred to v2.0)
- **Decision:** Use WASI p1's stdin/stdout/env/file-via-preopens surface; defer the component model.
- **Rationale:** p1 is stable enough; p2 is still maturing.
- **Consequences:** Plugin target triple `wasm32-wasip1`; no built-in network; `exec_subprocess` is the only network path until v2.
- **Status:** **IN-EFFECT.** Source: design doc lines 436-444 + ADR-003 explicitly cited in NFR-SEC-014.

### ADR-004: TOML for configuration
- **Decision:** All config files (hooks-registry.toml, observability-config.toml) use TOML, not YAML or JSON.
- **Rationale:** Rust-native (`toml` crate), comment-friendly, strongly typed, array-of-tables clean for multi-instance sinks. Workflow files use Lobster (YAML-under-the-hood) — that predates the dispatcher.
- **Consequences:** Schema versioning at every config boundary (`schema_version = 1`).
- **Status:** **IN-EFFECT.** Source: design doc lines 446-456.

### ADR-005: Multi-sink observability natively in dispatcher
- **Decision:** Dispatcher owns multi-sink fan-out. Plugins emit via host function; sinks are first-class drivers (file, http, otel-grpc, datadog, honeycomb).
- **Rationale:** Existing OTel collector stack still works (file sink + filelog receiver). New operators can bypass collector entirely (zero-disk mode possible). Per-sink RoutingFilter + tags + multi-instance support.
- **Consequences:** Sink trait + per-driver crates; pending S-4.x stories for HTTP/Datadog/Honeycomb (DRIFT-005). Always-on `dispatcher-internal-YYYY-MM-DD.jsonl` independent of sink config.
- **Status:** **PARTIALLY IN-EFFECT.** sink-file + sink-otel-grpc shipped; sink-http, sink-datadog, sink-honeycomb declared but not implemented (warn-and-skip pattern in `sinks/mod.rs::from_config`). Source: design doc lines 458-473 + Tier E story scoping.

### Open-Question resolutions (decisions equivalent to ADRs)

- **Q3-DECISION: Parallel-within-tier execution.** Plugins in the same priority tier run as parallel tokio tasks (`spawn_blocking` for sync wasmtime calls); tiers run sequentially. Within-tier failures don't block siblings; `on_error = "block"` records intent but doesn't abort the tier. Status: **IN-EFFECT.** Source: `executor.rs::execute_tiers`.
- **Q5-DECISION: Activation-skill-driven platform selection.** Claude Code provides no `hooks.json` variable expansion or plugin install lifecycle hooks. The activate skill detects platform at activation and copies `hooks.json.<platform>` (CI-generated) over `hooks.json` (gitignored). Status: **IN-EFFECT.** Source: `skills/activate/SKILL.md` + `hooks.json.template`.
- **Q6-DECISION (Option B): Always-on dispatcher self-telemetry.** `dispatcher-internal-YYYY-MM-DD.jsonl` is written regardless of sink config. Ensures debuggability when all sinks are misconfigured/down. 30-day retention. Status: **IN-EFFECT.** Source: `internal_log.rs`.
- **Q7-DECISION: Staged release ladder.** `1.0.0-beta.N` → `1.0.0-rc.N` → `1.0.0` rather than single `1.0.0`. Semver stability promise engages only at final `1.0.0`; breaking changes during prereleases allowed. Status: **IN-EFFECT.** Currently at `beta.4`; rc.1 gate is S-4.8 (Tier F).

### As-built ADRs implied by code (candidates for formal architecture promotion in Phase 1)

- **ADR-AS-006: Dispatcher process is single-shot and short-lived.** Per hook event spawns a fresh process; no daemon. `current_thread` tokio. Engine + ticker rebuilt per invocation. Documented in `main.rs` comments. Status: **IN-EFFECT.**
- **ADR-AS-007: Capability denial co-emits event AND error code.** Every cap-gated host fn returns a negative i32 code AND emits `internal.capability_denied`. Both, never either-or. Audit log is canonical; plugin can still take corrective action. Status: **IN-EFFECT.**
- **ADR-AS-008: Bash hooks transition via legacy-bash-adapter.** A single WASM plugin instance per registry entry, each configured with its own `script_path`. 45/45 current registry entries route through `legacy-bash-adapter.wasm`. Source: `hooks-registry.toml`. Status: **IN-EFFECT** as transition; wind-down planned post-v1.0.
- **ADR-AS-009: Per-sink dedicated thread (transitional).** sink-file and sink-otel-grpc each own a dedicated OS thread + `current_thread` tokio runtime. Doc comments promise "swap to a shared `Handle` once S-1.6 lands"; S-1.6 has shipped but the swap has not (DRIFT-003). Status: **PARTIAL.**

## 5. Story coverage rollup (definitive)

Verified by extraction-validation Phase 3 (all 10 sampled stories CONFIRMED). EPIC = 41 stories (S-0.1 through S-5.7).

### SHIPPED — 22 stories (Tier A through D)

**Tier A — Phase 0 infra prep (5 stories, all SHIPPED)**
- S-0.1 — bump-version.sh prerelease support. Evidence: `scripts/bump-version.sh` exists; CHANGELOG v1.0.0-beta.1+.
- S-0.2 — Release workflow prerelease handling. Evidence: CI runs producing tagged prereleases.
- S-0.3 — Activation skill platform detection. Evidence: `skills/activate/detect-platform.sh`.
- S-0.4 — `hooks.json.template` + CI generation of 5 platform variants. Evidence: template + 5 platform variants in `plugins/vsdd-factory/hooks/`.
- S-0.5 — Docs scaffolding. Evidence: 28 reference docs at `docs/guide/`.

**Tier B.0 — Workspace blocker (1 story, SHIPPED)**
- S-1.1 — Cargo workspace + CI scaffolding. Evidence: `Cargo.toml` + 7 member crates + `ci/platforms.yaml`.

**Tier B.x — Dispatcher foundation (8 stories, all SHIPPED)**
- S-1.2 — factory-dispatcher core (stdin parse, TOML registry load, routing). Evidence: `crates/factory-dispatcher/src/{main,registry,routing,payload}.rs`; 9+ tests.
- S-1.3 — hook-sdk crate (macro + types + bindings). Evidence: `crates/hook-sdk` + `hook-sdk-macros`; 20 tests.
- S-1.4 — Host function surface. Evidence: `crates/factory-dispatcher/src/host/*.rs`; 18 host integration tests. ⚠️ **Caveat: DRIFT-001** — read_file at the StoreData-typed linker is a CAPABILITY_DENIED stub.
- S-1.5 — wasmtime integration + epoch/fuel enforcement. Evidence: `engine.rs` + `invoke.rs`.
- S-1.6 — tokio + parallel-within-tier execution. Evidence: `executor.rs`; 4 tests.
- S-1.7 — `dispatcher-internal.jsonl` writer. Evidence: `internal_log.rs`; 8 tests; 17 event-type constants.
- S-1.8 — sink-file driver (default file sink). Evidence: `crates/sink-file/`; 17 tests; daily rotation working.
- S-1.9 — sink-otel-grpc driver. Evidence: `crates/sink-otel-grpc/`; 13 tests; mock OTLP receiver in integration test.

**Tier C — Legacy adapter + release prep (6 of 7 SHIPPED, S-2.5 PARTIAL)**
- S-2.1 — legacy-bash-adapter WASM plugin. Evidence: `crates/hook-plugins/legacy-bash-adapter/`; 14 tests; CHANGELOG v1.0.0-beta.1.
- S-2.2 — `hooks-registry.toml` auto-generation. Evidence: `scripts/generate-registry-from-hooks-json.sh`; 6 generate-registry bats; documented as idempotent.
- S-2.3 — Cross-platform CI matrix. Evidence: 5-platform builds in `ci/platforms.yaml`; `scripts/check-platforms-drift.py`.
- S-2.4 — Binary commit automation. Evidence: per-platform binaries committed under `plugins/vsdd-factory/hooks/dispatcher/bin/<platform>/`.
- S-2.6 — Activate skill integrates with hooks.json variants. Evidence: activate skill confirmed working in CHANGELOG.
- S-2.7 — Regression test suite validation. Evidence: `regression-v1.0.bats` (11 tests).

**Tier D — beta.1 release gate (1 story, SHIPPED)**
- S-2.8 — `1.0.0-beta.1` release gate. Evidence: tagged 1.0.0-beta.1 → … → beta.4.

### PARTIAL — 4 stories

- S-2.5 — hook-sdk publish to crates.io. CHANGELOG v1.0.0-beta.1: "SDK publish dry-run clean" but actual `cargo publish` deferred. **Cannot verify from source alone.**
- S-3.4 — emit_event as host function refactor. The host fn IS implemented (`host/emit_event.rs` + `invoke.rs::emit_event`); but bash hooks still call `bin/emit-event` shell tool. **Refactor is incomplete.**
- S-4.6 — Per-sink routing filters + tag enrichment. RoutingFilter implemented in sink-core; tag enrichment scoped to `tags` field on `SinkConfigCommon`; per-sink wiring partial.
- S-5.5 — Migration guide (0.79.x → 1.0). `docs/guide/migrating-from-0.79.md` skeleton exists per CHANGELOG; full guide TBD.

### NOT SHIPPED — 15 stories

**Tier E — Native ports + advanced sinks (11 stories, NOT SHIPPED — except S-3.4 PARTIAL above)**
- S-3.1 — Port capture-commit-activity to WASM. `crates/hook-plugins/capture-commit-activity/src/lib.rs` is a 20-LOC stub.
- S-3.2 — Port capture-pr-activity to WASM. No crate yet.
- S-3.3 — Port block-ai-attribution to WASM. No crate yet.
- S-4.1 — sink-http driver. No crate; warn-and-skip handler in `sinks/mod.rs`.
- S-4.2 — sink-datadog driver. No crate; warn-and-skip.
- S-4.3 — sink-honeycomb driver. No crate; warn-and-skip.
- S-4.4 — Per-sink retry + circuit breaker. Constants declared (DRIFT-002), never emitted; SinkFailure recorded but not retried.
- S-4.5 — Dead letter queue. Design has `[sinks.*.dead_letter]` block; no impl.
- S-4.7 — End-to-end observability integration tests. Unit/integration cover but full E2E pending S-4.x sinks shipping.

**Tier F — rc.1 release gate (1 story, NOT REACHED)**
- S-4.8 — `1.0.0-rc.1` release gate. Pending Tier E completion.

**Tier G — New events + docs (5 stories, NOT SHIPPED — except S-5.5 PARTIAL above)**
- S-5.1 — SessionStart hook wiring. `hooks.json.template` registers dispatcher on SessionStart but no plugin reacts.
- S-5.2 — SessionEnd hook wiring. Same.
- S-5.3 — WorktreeCreate / WorktreeRemove hook wiring. No plugin.
- S-5.4 — PostToolUseFailure hook wiring. No plugin.
- S-5.6 — Semver commitment docs. Scoped for `1.0.0` final.

**Tier H — 1.0 release gate (1 story, NOT REACHED)**
- S-5.7 — `1.0.0` release gate. Pending Tier G.

**Summary:** Tiers A+B+C(6/7)+D = 22 SHIPPED. Tiers E+F+G+H = 19 not yet reached, of which 4 are PARTIAL and 15 NOT SHIPPED.

## 6. Drift report (definitive — all 10 confirmed)

All 10 drift items re-verified against source by extraction-validation Phase 3. Each is real and correctly described.

### DRIFT-001 — `read_file` host fn at StoreData linker is a CAPABILITY_DENIED stub
- **Severity:** P1 (medium)
- **Source:** `crates/factory-dispatcher/src/invoke.rs:447-474` (StoreData-typed linker stub returning `codes::CAPABILITY_DENIED` unconditionally) vs `crates/factory-dispatcher/src/host/read_file.rs` (full impl registered against `Linker<HostContext>` but not wired into invocation).
- **Affects:** S-1.4; any future plugin needing FS read.
- **Disposition:** **must-fix-before-rc.1.** Mechanical (mirror impl into `setup_host_on_store_data`).

### DRIFT-002 — `internal.sink_*` events declared as constants but never emitted
- **Severity:** P1 (medium)
- **Source:** `internal_log.rs:67-70` declares INTERNAL_SINK_ERROR / INTERNAL_SINK_QUEUE_FULL / INTERNAL_SINK_CIRCUIT_OPENED / INTERNAL_SINK_CIRCUIT_CLOSED; sink-file SinkFailure recorded into `Mutex<Vec<SinkFailure>>` but never converted to events.
- **Affects:** S-4.4 (per-sink retry + circuit breaker); operators have no audit trail of sink degradation.
- **Disposition:** **must-fix-before-rc.1** (S-4.4 work).

### DRIFT-003 — Per-sink dedicated thread vs design's "shared runtime once S-1.6 lands"
- **Severity:** P2 (low — works, just less efficient)
- **Source:** `sink-file/src/lib.rs:272` and `sink-otel-grpc/src/lib.rs:310` both spin up dedicated OS thread + `current_thread` runtime. S-1.6 is shipped but swap has not been made.
- **Affects:** Resource efficiency; doc comments promise the swap.
- **Disposition:** **acceptable-for-1.0; planned-fix post-1.0.**

### DRIFT-004 — Two parallel hook-routing tables (`hooks.json` and `hooks-registry.toml`)
- **Severity:** P1 (medium — transitional)
- **Source:** `plugins/vsdd-factory/hooks/hooks.json` (45 entries, legacy v0.79.x) AND `plugins/vsdd-factory/hooks-registry.toml` (45 entries, generated, `schema_version = 1`). Generation: `scripts/generate-registry-from-hooks-json.sh` (idempotent, "DO NOT HAND-EDIT during the v0.79.x → v1.0 migration").
- **Affects:** Source-of-truth ambiguity during migration window. Activation skill picks `hooks.json.<platform>` (template-derived) but legacy `hooks.json` is still referenced by older flows.
- **Disposition:** **planned-fix at 1.0.** Either retire `hooks.json` entirely or document as bootstrap-only.

### DRIFT-005 — HTTP / Datadog / Honeycomb sinks declared in design but not implemented
- **Severity:** P2 (low — graceful unknown-type handling; design clearly stages these as rc.1, not beta.1)
- **Source:** Only `file` and `otel-grpc` are shipped. Unknown driver types in `sinks/mod.rs::from_config` are warned to stderr and skipped.
- **Affects:** S-4.1, S-4.2, S-4.3.
- **Disposition:** **planned-fix at rc.1** (Tier E).

### DRIFT-006 — Phase 5 events not yet wired to plugins
- **Severity:** P2 (low — planned, clear timeline)
- **Source:** `hooks.json.template` registers dispatcher for SessionStart, SessionEnd, SubagentStop. No plugin-layer hooks listen for `session.started` / `session.ended` events.
- **Affects:** S-5.1, S-5.2, S-5.3, S-5.4.
- **Disposition:** **planned-fix at 1.0** (Tier G).

### DRIFT-007 — `dispatcher.shutting_down` constant defined but never emitted
- **Severity:** P3 (cosmetic)
- **Source:** `internal_log.rs:58` constant; `main.rs` exit paths checked — no emit.
- **Disposition:** **acceptable-for-1.0** OR remove the constant. Reasonable in a per-event short-lived process.

### DRIFT-008 — `plugin.loaded` / `plugin.load_failed` constants never emitted from plugin_loader
- **Severity:** P3 (cosmetic)
- **Source:** Constants at `internal_log.rs:59-60`; `plugin_loader.rs` has no reference. Executor's load-error fallback maps to `PluginResult::Crashed` instead.
- **Disposition:** **acceptable-for-1.0; emit if cheap** (1-line addition).

### DRIFT-009 — Adversary SHA-currency gate is opt-in (verify-sha-currency.sh template)
- **Severity:** P2 (low — working-as-intended for new feature)
- **Source:** `templates/verify-sha-currency.sh` exists; `hooks/verify-sha-currency.sh` does NOT. Hook is no-op when project hasn't installed it.
- **Disposition:** **acceptable-for-1.0; document the opt-in clearly.** CHANGELOG v1.0.0-beta.4 already covers.

### DRIFT-010 — 26 unported bash hooks block Windows users
- **Severity:** P0 for Windows users / P1 overall
- **Source:** All 44 hook scripts route through `legacy-bash-adapter`, which requires git-bash on Windows. Native ports (S-3.1/3.2/3.3) are stubs.
- **Affects:** Windows operator experience. Not a regression but a widening gap relative to design's beta.1 ambition.
- **Disposition:** **planned-fix at rc.1** (Tier E native ports).

## 7. NFR catalog summary

76 NFRs cataloged across 8 categories with citations to specific config values, constants, or code locations. Full catalog at `pass-4-nfr-catalog.md`.

**Categories (count):**
- **Performance** (16 NFRs) — epoch tick 10ms, default timeout 5_000ms, fuel cap 10_000_000, sink queue depth 1000, OTLP batch 100, per-invocation Module cache, current_thread runtime, release profile (LTO + strip), STDERR_CAP_BYTES = 4 KiB, MAX_OUTPUT_BYTES = 1 MiB, BASH_TIMEOUT_MS = 60_000, async hook events use `"async": true`.
- **Security** (18 NFRs) — capability deny-by-default, exec_subprocess shell-bypass-acknowledged, setuid refusal, env_allow restrictions, layered env passthrough, cwd_allow, binary path resolution at load time, denial events with full audit trail, WASI p1 sandbox, isolated wasmtime Store per plugin, adversary SHA-currency gate, no auth at dispatcher boundary (Claude Code is trust boundary), hooks.json gitignored.
- **Observability** (16 NFRs) — always-on internal log, 30-day retention, daily rotation by event ts, 17 event-type constants, schema_version = 1, dispatcher_trace_id propagation, multi-sink fan-out, RoutingFilter (allow-then-deny), per-sink static tags, stderr capture on lifecycle events, file sink path placeholders.
- **Reliability** (15 NFRs) — non-blocking errors (exit 0 on registry/payload/engine error), per-entry on_error policy, plugin failure isolated, sink failure isolated, internal log best-effort, prune_old defensive, retry/circuit/DLQ pending S-4.4/4.5, cooperative shutdown, atomic write of release artifacts, crash recovery in feature workflow.
- **Scalability** (6 NFRs) — parallel within tier sequential between, single-shot dispatcher process, per-sink dedicated OS thread, ~3GB binary distribution ceiling over 3 years, per-event payload bounded, multi-instance sinks supported.
- **Maintainability** (8 NFRs) — workspace dependency pinning, opentelemetry-* lockstep at 0.31, Edition 2024 + rust-version 1.95, schema versioning at every config boundary, deny_unknown_fields universal, thiserror-for-crates / anyhow-at-boundaries, ~180 Rust tests + ~1262 bats, no feature flags.
- **Compatibility** (5 NFRs) — 5-platform support, HOST_ABI_VERSION = 1 frozen at v1.0, SDK accepts both `event_name` and `hook_event_name` envelope spellings, activation cross-host re-activation drift warning, Windows native-WASM hooks work without git-bash (post-Tier E).
- **Auditability** (4 NFRs) — every capability denial recorded, dispatcher_trace_id propagated through every event, factory-events-*.jsonl + dispatcher-internal-*.jsonl as durable trail, bin/emit-event normalizes bash-side emission.

**Top 5 highest-priority NFRs (must hold for v1.0 ship):**
1. **NFR-SEC-001 (capability deny-by-default).** Foundational. Every cap-gated host fn enforces.
2. **NFR-OBS-001 (always-on dispatcher self-telemetry).** Q6 Option B. Independent of sink config.
3. **NFR-REL-001 (non-blocking errors at registry/payload/engine boundary).** Misconfiguration must never block user's tool call.
4. **NFR-MAINT-004 (schema versioning at every config boundary).** REGISTRY_SCHEMA_VERSION = 1, INTERNAL_EVENT_SCHEMA_VERSION = 1, observability schema_version = 1, HOST_ABI_VERSION = 1. Mismatch = hard error.
5. **NFR-COMPAT-002 (HOST_ABI_VERSION = 1 frozen at v1.0).** Breaking ABI = major bump on dispatcher AND SDK.

**11 declared-but-not-shipped NFRs** flagged: retry+exp-backoff (S-4.4), circuit breaker (S-4.4), dead letter queue (S-4.5), sink batching (sink-file), HTTP/Datadog/Honeycomb sinks (S-4.1/4.2/4.3), read_file via StoreData linker (DRIFT-001), `internal.sink_error` event emission (DRIFT-002), `internal.sink_circuit_*` events (DRIFT-002), plugin signing / marketplace discovery (non-goal), seccomp/AppArmor/Landlock (non-goal v2+), WASI preview-2 (v2.0).

## 8. Convention catalog summary

22 conventions + 7 design patterns + 4 anti-patterns documented. Full catalog at `pass-5-conventions.md`.

### Rust conventions (13)
Edition 2024 + rust-version 1.95; workspace dependency pinning (every member uses `.workspace = true`); thiserror for crate APIs + anyhow at boundaries; one concern per file with `host/` and `sinks/` subdirs; tests in `#[cfg(test)] mod tests` blocks + `tests/*.rs` for integration; doc comments universal (`#[deny(missing_docs)]` enforced in sink-* crates only — partial); naming (PascalCase types, snake_case fns, SCREAMING_SNAKE_CASE consts); negative-error-codes prefixed `codes::*` and i32; `Continue`/`Block`/`Error` enum outcomes; builder pattern for events (chainable, `#[must_use]`); `current_thread` tokio + `spawn_blocking`; `Arc<Mutex<Vec<_>>>` shared accumulators; deny-by-default at type level (Option<Caps>); reserved-field filtering on event emission; tracing crate available + custom internal_log JSONL is durable path.

### Plugin layer conventions (9)
Skill frontmatter (`name`, `description`, optional `argument-hint`) — 119/119 conform; skill internal structure (`SKILL.md` + `steps/` + `templates/`); agent frontmatter (`name`, `description`, optional `tools`/`model`/`color`) — 33+ files; Lobster workflow shape (`workflow:` / `steps:`) — 16/16 conform; bash hook conventions (`#!/bin/bash`, `set -euo pipefail`, jq-graceful-fallback, `_emit` helper, `block` helper) — 44/44 follow; template naming (`<artifact>-template.md`, `L<level>-...`); slash command per skill (110 commands); bin tool naming (`emit-event`, `factory-*`, `wave-state`); file/directory naming (kebab-case skills/agents, S-N.M stories, YYYY-MM-DD-slug specs/plans, BC-S.SS.NNN or BC-AUDIT-NNN, ADV-CYCLE-PASS-SEV-SEQ adversarial findings, dot-namespaced lowercase events).

### Test conventions (4)
Rust unit tests in same file; full-sentence snake_case test names; one behavior per test; integration tests one-file-per-scenario; bats tests at ~1262 baseline + 11 regression-v1.0; ~~13 trybuild fixtures~~ (HALLUCINATED in pass-0; actual count is 0 — `crates/hook-sdk-macros/` has no `tests/` directory).

### Design patterns observed (7)
1. Strategy / dyn-trait fan-out (`SinkRegistry { sinks: Vec<Box<dyn Sink>> }`).
2. Bounded queue producer/consumer (per-sink mpsc + worker thread; `try_send` non-blocking).
3. Builder pattern for events (`InternalEvent::now(type_).with_*(...)`).
4. Capability-token pattern (`Capabilities` block on each registry entry; deny-by-default).
5. Module bridging (StoreData ↔ HostContext linker rebuild documented in invoke.rs).
6. Adapter / bridge plugin (`legacy-bash-adapter` — one wasm + 45 instances, each with own `script_path`).
7. Co-occurring effect (capability denial = event AND return code; never just one).

### Anti-patterns observed (4)
1. **Drift between design doc and code.** 10 DRIFT items confirmed. Acknowledged in code comments (e.g., DRIFT-002 sink event constants, DRIFT-003 shared-runtime promise).
2. **Two parallel hook tables.** `hooks.json` (legacy v0.79.x) + `hooks-registry.toml` (v1.0). Transitional; documented; will resolve at 1.0 (DRIFT-004).
3. **Stub / pending state in shipped code.** `read_file` returns CAPABILITY_DENIED at one of the two linker layers (DRIFT-001); `dispatcher.shutting_down` declared not emitted (DRIFT-007); `plugin.loaded`/`plugin.load_failed` declared not emitted (DRIFT-008).
4. **Inconsistent `#[deny(missing_docs)]` enforcement.** sink-core, sink-file, sink-otel-grpc enforce; factory-dispatcher and hook-sdk do not (docs are pervasive but not attribute-enforced).

**Mitigation status:** All 4 anti-patterns are explicitly known (drift items have CHANGELOG/code-comment provenance). None are silent. The remediations are tracked stories (S-4.4 closes drift-002 + drift-007/008 candidate; S-3.x ports close drift-010; S-1.4 follow-up closes drift-001).

## 9. Convergence report

| Pass | Phase A novelty | Phase B rounds | Final state |
|------|-----------------|----------------|-------------|
| 0 — Inventory | SUBSTANTIVE | 0 (acceptable; validation confirmed metrics with 7 minor deltas) | NITPICK |
| 1 — Architecture | SUBSTANTIVE | 0 | NITPICK |
| 2 — Domain Model | SUBSTANTIVE | 0 (35 entities, well-covered) | NITPICK |
| 3 — Behavioral Contracts | SUBSTANTIVE | 8 (broad-r1 + skills×3 + agents + hooks + workflows + templates/tools/rules + rust-tests) | NITPICK |
| 4 — NFRs | SUBSTANTIVE | 0 | NITPICK |
| 5 — Conventions | SUBSTANTIVE | 0 | NITPICK |
| 6 — Synthesis | (this pass / Phase C) | — | TERMINAL |

| Coverage axis | Status | Confidence |
|---------------|--------|------------|
| Rust source files | 41/41 (100%) | HIGH |
| Skills | 119/119 (100%) | HIGH |
| Agents | 34/34 (100%) | HIGH |
| Hooks (bash, top-level) | 44/44 (100%) | HIGH |
| Workflows (.lobster) | 16/16 (100%) | HIGH |
| Templates | 105/105 (100%) | MEDIUM (some templates are skeleton-only) |
| Rust tests | 185/185 (100%) | HIGH |
| Bin tools | 12/12 (100%) | MEDIUM |
| Rules | 9/9 (100%) | MEDIUM |
| Slash commands | 110/110 enumerated; not deeply walked | LOW-MED |

**Validation (B.6) outcome:** PASS WITH CAVEATS — 122/125 BCs CONFIRMED (97.6%); 2 INACCURATE (BC-AUDIT-067 PostToolUse→PreToolUse correction; BC-AUDIT-1007 numerical "4 hooks" should be "3"); 1 HALLUCINATED (pass-0 inventory's "13 trybuild tests" — actual is 0). All corrections applied or noted. 41/48 metrics exact match; 7 minor deltas (Rust test count +5 self-corrected, template count off by 2-3, docs/guide -2, design doc path correction `.factory/specs/` → `.factory/legacy-design-docs/`, file LOC systematic ±1, hook-sdk-macros trybuild -13).

## 10. Lessons for vsdd-factory (P0/P1/P2/P3)

### P0 — correctness gaps that must fix before v1.0 ships

#### L-P0-001 — Wire `read_file` impl through to the StoreData-typed linker (DRIFT-001)
- **What target does today:** `invoke.rs:447-474` registers a stub returning `codes::CAPABILITY_DENIED` unconditionally for `read_file`, bypassing the fully-implemented `host/read_file.rs`.
- **What gap is:** A future plugin needing FS read won't find `path_allow` enforced; the cap returns DENIED even when configured.
- **Action items:** (a) Mirror the host/read_file.rs impl into invoke.rs's `setup_host_on_store_data`. (b) Keep the host/read_file.rs unit tests. (c) Add an integration test that exercises path_allow (the absence of this test is what allowed the drift to ship).
- **Safe to do without HOST_ABI_VERSION bump.**

#### L-P0-002 — Eliminate the parallel-routing-table state (DRIFT-004)
- **What target does today:** Both `hooks.json` (legacy) and `hooks-registry.toml` (generated) coexist; activate skill writes `hooks.json.<platform>` over `hooks.json`.
- **What gap is:** No single source of truth for hook routing. Re-running `generate-registry-from-hooks-json.sh` on a hand-edited `hooks-registry.toml` reverts changes silently.
- **Action items:** (a) Decide before 1.0 whether `hooks.json` is bootstrap-only or retired. (b) If retired, the dispatcher-template hooks.json variants stay (Claude Code requires them) but no parallel TOML/JSON drift. (c) Document the chosen state in `docs/guide/migrating-from-0.79.md`.

#### L-P0-003 — Backfill BCs for the 24+ validate-* hooks
- **What target does today:** Validate-novelty-assessment.sh fired multiple times during this synthesis production, gating output structure. None of the validators have formal BCs.
- **What gap is:** The hook IS the spec; the spec doesn't exist as BC. 24+ behavioral contracts are encoded only in shell-script bodies.
- **Action items:** (a) Each validate-* hook becomes one or more BC-S.SS.NNN entries during Phase 1 backfill. (b) Treat the script body as authoritative source. (c) Test-coverage gap: most validators have no bats coverage of the gate behavior itself.

#### L-P0-004 — Resolve the `bin/emit-event` shell tool vs `host::emit_event` host fn duplication (S-3.4 PARTIAL)
- **What target does today:** Native WASM plugins call `vsdd::emit_event` host fn (cap-free, always-on); bash hooks shell out to `bin/emit-event`. Both write to the same internal log via different paths.
- **What gap is:** Two emission paths. The host-fn path is correctly typed; the shell-tool path bypasses the dispatcher's event normalization.
- **Action items:** Complete S-3.4 — make `bin/emit-event` a thin wrapper that constructs a payload and invokes the dispatcher's emit pipeline. OR document explicitly that bash-hook events are second-class until ports complete.

### P1 — high-ROI improvements to adopt

#### L-P1-001 — Wire `internal.sink_*` event emission (S-4.4 work, DRIFT-002)
- Constants declared, never emitted. SinkFailure recorded into `Mutex<Vec<SinkFailure>>` per-sink, never converted to events. Operators won't see sink degradation in their event stream until S-4.4 lands.
- **Action:** Drain the per-sink failure vector on flush; convert each entry to an `internal.sink_error` event and submit through the dispatcher's normal emission pipeline.

#### L-P1-002 — Apply `#[deny(missing_docs)]` to factory-dispatcher and hook-sdk
- Currently only sink-core, sink-file, sink-otel-grpc enforce. Code is well-documented anyway; enforcement prevents regression.
- **Action:** Add the attribute to `crates/factory-dispatcher/src/lib.rs` and `crates/hook-sdk/src/lib.rs`. Fix any newly-flagged missing docs.

#### L-P1-003 — Add a "subsystem coverage" check to brownfield-ingest
- This Phase 0 ingestion almost missed Subsystem B's depth. Pass 0/1 noted Subsystem A + B; Pass 3 broad sweep was Subsystem A heavy until deepening rounds.
- **Action:** brownfield-ingest skill should explicitly enumerate which subsystem each pass covered when ingesting a multi-subsystem repo. Add a "subsystem coverage" field to each pass file's frontmatter / state checkpoint.

#### L-P1-004 — Surface `dispatcher.shutting_down`, `plugin.loaded`, `plugin.load_failed` events (DRIFT-007, DRIFT-008)
- 1-line emit calls; gives operators a richer audit trail for free.
- **Action:** Plumb emit calls into `main.rs` exit paths and `plugin_loader::PluginCache::get_or_compile`.

#### L-P1-005 — Adopt the per-validator BC pattern repo-wide
- Each of the 24+ validate-* hooks is a standalone behavioral contract; today they're documented only by their script body.
- **Action:** Phase 1 backfill creates one BC per validator's gate (block condition + emit on block + exit code).

#### L-P1-006 — Document the legacy-bash-adapter wind-down plan
- Each hook ported reduces adapter reach; the plan should track which hooks port next and the retire criteria.
- **Action:** Add a `docs/guide/legacy-bash-adapter-wind-down.md` listing the 44 hooks, Tier-E port priority, and the criterion for adapter retirement.

#### L-P1-007 — Centralize `codes::*` between dispatcher and SDK
- factory-dispatcher::host defines them; hook-sdk's `host::HostError::from_code` mirrors them. Drift = silent breakage at the WASM ABI boundary.
- **Action:** Either extract to `host-codes` crate consumed by both, or pin by integration test asserting both copies match.

### P2 — worth considering

- **L-P2-001:** Migrate per-sink dedicated threads to a shared dispatcher tokio runtime (DRIFT-003). Already documented as TODO in source.
- **L-P2-002:** Catalog the 110 slash commands and their target skills as a navigable index. Currently a flat directory.
- **L-P2-003:** Plugin manifest version (`plugin.json`) and Cargo workspace package versions are NOT linked structurally. Drift only by convention. A pre-release script could enforce.
- **L-P2-004:** Test-naming convention enforcement. Most tests follow descriptive-sentence pattern; some are short. Lint-level.
- **L-P2-005:** Consider extracting `regex` to compile-once-cached registry entries (currently regex compiles per entry at registry-load time, fine for 45 entries but at 200+ would matter).
- **L-P2-006:** `validate-extraction` skill caught the design-doc path drift (`.factory/specs/` vs `.factory/legacy-design-docs/`) only on Phase B.6 — earlier passes echoed each other. Add an existence check at pass-0 stage.

### P3 — known divergences to document

- **L-P3-001:** WASI preview-2 migration declared as v2.0 (ADR-003).
- **L-P3-002:** Plugin signing / marketplace discovery infra declared non-goal for v1.0 (design doc).
- **L-P3-003:** seccomp / AppArmor / Landlock sandboxing on Linux declared v2+ (design doc non-goals).
- **L-P3-004:** Arg-filtering for `exec_subprocess` (today's gate is binary_allow only; arg-level is post-1.0 if attack scenarios materialize per design Q4).
- **L-P3-005:** `bin/emit-event` shell tool replacement with typed Rust binary that calls SDK host-fn shim (S-3.4 ambition; partial completion).
- **L-P3-006:** Multi-instance Datadog or Honeycomb sinks (e.g., two orgs) — supported by design but pending S-4.2/S-4.3 implementation.

## 11. Spec crystallization recommendations

### For create-domain-spec / business-analyst

- **Primary input:** `/Users/jmagady/Dev/vsdd-factory/.factory/legacy-design-docs/2026-04-24-v1.0-factory-plugin-kit-design.md` (the v1.0 master design — frozen, all Q1–Q7 resolved, 5 ADRs explicit). 7 secondary design docs from 2026-04-13 cover earlier-phase decisions (early-phase-gaps, excalidraw-integration, release-infrastructure, scaffold-claude-md, subagent-driven-gaps, writing-plans-gaps).
- **Entity catalog:** Use `pass-2-domain-model.md` directly. 35 entities (22 Half A — dispatcher runtime; 13 Half B — orchestration framework). Each has fields, invariants, lifecycle, and code citation.
- **Subsystem boundaries:** Per `pass-1-architecture.md`. Two subsystems share one repo and one version, coupled at 4 contracts (stdin envelope, hooks-registry.toml, observability-config.toml, plugin ABI).
- **Cross-cut bounded contexts:** Dispatch, Plugin authoring, Sink pipeline, Orchestration, Process/governance — all enumerated in pass-2-domain-model.md table.
- **Ubiquitous language glossary:** 22 terms canonicalized in pass-2-domain-model.md section 3.

### For create-prd / product-owner

- **BC catalog:** The 1,851 BCs across the 11 pass-3-* files become the formal BC catalog. **Renumber from `BC-AUDIT-NNN` to formal `BC-S.SS.NNN`** once subsystem identifiers are assigned. Recommend:
  - **S = A** for Rust dispatcher (sub-sections: A.01 = registry, A.02 = routing, A.03 = executor, A.04 = invoke, A.05 = host fns, A.06 = internal_log, A.07 = sinks-core, A.08 = sink-file, A.09 = sink-otel-grpc, A.10 = hook-sdk, A.11 = legacy-bash-adapter).
  - **S = B** for orchestration framework (sub-sections: B.01 = skills, B.02 = agents, B.03 = workflows, B.04 = templates, B.05 = hooks-bash, B.06 = bin-tools, B.07 = rules).
- **NFR catalog:** Pass-4-nfr-catalog.md is ready to be quoted directly into the PRD's NFR section. 76 NFRs across 8 categories with citations.
- **Story → milestone mapping:** EPIC.md provides the initial milestone ladder (`1.0.0-beta.1` → `1.0.0-rc.1` → `1.0.0`). 22 stories shipped to Tier D (beta.1 + beta.2/3/4 follow-ups); Tier E onward is the open backlog.
- **Drift report:** All 10 DRIFT items belong as "Known issues / planned-fix" entries under their owning BC group.
- **Validation provenance:** extraction-validation.md confirms 97.6% of sampled BCs at 6.8% sample rate (125/1851). Document this in PRD as the trust basis.

### For create-architecture / architect

- **Pass 1 architecture as starting point:** 37-component catalog, 5-layer diagram (User → Orchestration → Hook bus → Dispatch → Plugin), inter-subsystem coupling table.
- **5 ADRs already implicit in legacy v1.0 design — promote to formal ADR-001..005 in `.factory/architecture/decisions/`:**
  - ADR-001: Rust for the dispatcher
  - ADR-002: WASM for plugin ABI
  - ADR-003: WASI preview 1 for v1.0
  - ADR-004: TOML for configuration
  - ADR-005: Multi-sink observability natively in dispatcher
- **4 implicit ADRs (Open-Question resolutions) — promote to formal ADRs:**
  - ADR-006: Parallel-within-tier execution (Q3)
  - ADR-007: Activation-skill-driven platform selection (Q5)
  - ADR-008: Always-on dispatcher self-telemetry (Q6 Option B)
  - ADR-009: Staged release ladder (Q7)
- **4 as-built ADRs identified during pass-3 deepening — promote in Phase 1:**
  - ADR-AS-010: Dispatcher process is single-shot and short-lived
  - ADR-AS-011: Capability denial co-emits event AND error code
  - ADR-AS-012: Bash hooks transition via legacy-bash-adapter (multi-instance plugin pattern)
  - ADR-AS-013: Per-sink dedicated thread (transitional; planned consolidation)
- **Architecture index:** Convert pass-1-architecture.md component catalog into `.factory/architecture/components/` per-component pages. Mermaid diagrams stay; promote to dedicated `.factory/architecture/diagrams/`.

### For story-writer

- **Stories already exist in `.factory/stories/v1.0/` (41 stories in EPIC).** They need re-anchoring to formal BC IDs once Phase 1 backfill produces them.
- **Status update needed:** 4 PARTIAL stories (S-2.5 publish, S-3.4 emit_event refactor, S-4.6 routing/tags, S-5.5 migration guide) need explicit completion criteria for the partial work. 15 NOT-SHIPPED stories need re-validation against Tier E/F/G/H criteria.
- **Re-anchor old stories rather than create new ones.** Keep S-N.M numbering for traceability with CHANGELOG. New stories from this Phase 0 (e.g., for the L-P0/P1 lessons) should slot under the existing tier framework — most fit Tier E (port + sink work).
- **Story → BC traceability:** When BC-AUDIT-NNN renumbers to BC-A.NN.NNN / BC-B.NN.NNN, each story's Acceptance Criteria links to the BCs it asserts.

### For semport-analyze

- **Not currently applicable.** No language port in flight. The existing Rust workspace + plugin layer are the canonical implementations.
- **Future use:** If a future operator wants to port the dispatcher to Go (the close runner-up in ADR-001), semport-analyze would be the entry point. Pass 1's component catalog is the input.

### For brownfield-ingest itself (meta-recommendation)

- The skill performed well overall but had the L-P1-003 gap (subsystem coverage tracking). The Phase 0 + Phase B + B.5 + B.6 + Phase C structure produced a 1,851-BC catalog at 97.6% confirmation — the methodology works at this scale.
- Consider: Phase B.5 coverage audit fired AFTER deepening rounds rather than between them. A mid-Phase B coverage gate would have caught Subsystem B under-walk earlier.

## 12. Anti-patterns observed (4)

(From pass-5-conventions.md, with mitigation status as of 2026-04-25.)

1. **Drift between design doc and code (10 confirmed items, all coded).** Mitigation: tracked under DRIFT-001 through DRIFT-010 with severity, evidence, and disposition. None are silent. **Status: tracked, planned-fix or acceptable-for-1.0 per drift.**

2. **Two parallel hook-routing tables (`hooks.json` + `hooks-registry.toml`).** Mitigation: documented as transitional in `hooks-registry.toml` header ("DO NOT HAND-EDIT during the v0.79.x → v1.0 migration"); generation script idempotent. **Status: planned-fix at 1.0** (DRIFT-004; L-P0-002).

3. **Stub / pending state in shipped code.** Mitigation: code comments call out the stubs (e.g., `read_file` stub at invoke.rs notes "isn't reachable by any in-tree plugin yet"); CHANGELOG references pending stories. **Status: P0/P1 fix** (L-P0-001; L-P1-004).

4. **Inconsistent `#[deny(missing_docs)]` enforcement** (sink-* enforce; factory-dispatcher and hook-sdk don't). Mitigation: docs are pervasive in practice. **Status: P1 fix** (L-P1-002).

## 13. Open questions for human review

Items the human should explicitly approve / decide before Phase 1:

1. **Subsystem letter assignment.** Recommend **A = Rust dispatcher**, **B = orchestration framework** (matches the convention used throughout these pass files — Subsystem A and Subsystem B). Alternative naming (e.g., "engine" / "framework" / "kit") would be clearer to outside readers but breaks continuity with the in-flight catalog. **Decision needed: confirm A/B vs alternative.**

2. **BC ID format.** Continue with `BC-AUDIT-NNN` (current) or migrate to `BC-S.SS.NNN` (formal) now? If migrate now, all 1,851 BCs need renumbering before Phase 1 spec crystallization can quote stable IDs. If defer, downstream skills work with `BC-AUDIT-NNN` and the renumber happens at PRD finalization. **Recommend: migrate now, batch-renumber by subsystem.**

3. **DTU assessment trigger.** Is there a third-party service to clone? The product is a Claude Code plugin — there's no SaaS to inspect, no API to characterize. Probably **no DTU assessment needed** for vsdd-factory itself. **Verify no external service dependency requires it.**

4. **Existing story numbering.** Should the 41 existing stories keep their `S-N.M` numbering or be renumbered under the new framework? **Recommend: keep S-N.M for traceability with CHANGELOG and EPIC.md.** New stories slot under existing tiers.

5. **DRIFT-004 resolution path.** Retire `hooks.json` entirely (clean) or keep as "bootstrap only" path (transitional)? Either is technically supportable; clean retirement is the design's eventual end state. **Decision needed before rc.1.**

6. **L-P0-002 vs L-P0-001 priority order.** L-P0-001 (read_file wire-up) is mechanical and low-risk; L-P0-002 (parallel routing) requires a migration decision. Recommend **L-P0-001 first** (close in beta.5 if shipped); L-P0-002 deferred to rc.1 prep.

7. **ADR promotion path.** Convert the 5 explicit ADRs + 4 Q-resolutions + 4 as-built into 13 formal ADRs at `.factory/architecture/decisions/ADR-NNN-*.md`? Or keep design doc as authoritative until v1.0 ships and then split? **Recommend: split now** — the formal ADR shape is what spec-reviewer and downstream skills expect.

8. **Validation cadence post-Phase 0.** extraction-validation produced 97.6% confirmation. Do we re-run on every Phase 1 spec output, or trust the catalog and only re-validate after major refactors? **Recommend: full re-run after BC renumber; spot-checks otherwise.**

## 14. Convergence declaration

```
PHASE_0_INGEST: COMPLETE

Total artifacts produced: 17 files in .factory/phase-0-ingestion/
Total BCs catalogued: 1,851
Coverage: 100% across all artifact types
  - Rust source files: 41/41
  - Skills: 119/119
  - Agents: 34/34
  - Hooks (bash, top-level): 44/44
  - Workflows: 16/16
  - Templates: 105/105
  - Rust tests: 185/185
  - Bin tools: 12/12
  - Rules: 9/9
Validation: 97.6% confirmation rate (122/125 BCs sampled)
  - 2 INACCURATE (corrected: BC-AUDIT-067 PostToolUse→PreToolUse; BC-AUDIT-1007 4→3)
  - 1 HALLUCINATED (removed: pass-0 inventory's "13 trybuild tests")
Metrics: 41/48 exact match; 7 minor deltas (all <±5 except hook-sdk-macros trybuild
  and Rust test self-correction); design doc path corrected
  (.factory/specs/ → .factory/legacy-design-docs/)
Layout: restructured to formal VSDD layout (legacy-design-docs/ + skeleton dirs at
  .factory/architecture/, .factory/specs/, .factory/stories/)

Drift items: 10 confirmed, all with severity + disposition
Lessons: 4 P0 + 7 P1 + 6 P2 + 6 P3 = 23 lessons cataloged
Story coverage: 22 SHIPPED + 4 PARTIAL + 15 NOT SHIPPED (Tier A-D closed; E-H pending)

Ready for Phase 1 spec crystallization.
```

## Novelty Assessment

| Field | Value |
|-------|-------|
| **Pass** | Phase C — Final synthesis (Pass 8) |
| **Novelty score** | NITPICK |
| **Trajectory** | Phase A SUBSTANTIVE (passes 0–6 all first-sweep) → Phase B all passes converged (8 deepening rounds for Pass 3 alone) → B.5 coverage audit served as the story-coverage map → B.6 PASS WITH CAVEATS (97.6% confirmation, corrections applied) → Phase C terminal synthesis. No new findings vs. the consolidated Phase B output; this document re-organizes for downstream consumption rather than introducing new analytic content. |
| **Verdict** | CONVERGENCE_REACHED |

Phase 0 ingest is terminal. The catalog is ready to feed Phase 1 spec crystallization (create-domain-spec, create-prd, create-architecture, story-writer). All Phase B passes reached NITPICK; this Phase C synthesis is itself NITPICK by construction (no new source-of-truth reads beyond what was already in the consolidated catalog). The next pass is Phase 1, not another Phase 0 round.
