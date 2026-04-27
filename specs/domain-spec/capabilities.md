---
document_type: domain-spec-section
level: L2
section: capabilities
version: "1.0"
status: accepted
producer: business-analyst
timestamp: 2026-04-25T00:00:00
phase: 1.3
inputs:
  - .factory/phase-0-ingestion/pass-2-domain-model.md
  - .factory/phase-0-ingestion/pass-8-final-synthesis.md
  - .factory/legacy-design-docs/2026-04-24-v1.0-factory-plugin-kit-design.md
  - .factory/specs/architecture/ARCH-INDEX.md
input-hash: "8021f73"
traces_to: L2-INDEX.md
---

# Capabilities

> **Sharded L2 section (DF-021).** Navigate via `L2-INDEX.md`.
> CAP-NNN = user-facing things vsdd-factory enables. Each maps to SS-NN
> subsystems and a verifiable user outcome. Priority: P0/P1/P2.

## P0 Capabilities — Must-Have for Release

**CAP-001 — Run a self-orchestrating LLM-driven SDLC pipeline**
The orchestrator agent reads `.lobster` workflow files and autonomously dispatches specialist sub-agents through all 8 SDLC phases (brief → domain-spec → PRD → architecture → stories → TDD delivery → adversarial review → convergence).
Subsystems: SS-05, SS-06. Outcome: user runs `/vsdd-factory:run-phase` and the pipeline produces spec + code + tests without manual agent handoffs.
Source: pass-8 §2; design doc "Decisions" §1. Justification: this is the product's core value proposition.

**CAP-002 — Hook Claude Code tool calls with sandboxed WASM plugins**
Every Claude Code tool invocation (Bash, Edit, Write, etc.) triggers the dispatcher, which routes to matching WASM plugins by event type and tool-regex. Plugins run in wasmtime with bounded fuel + epoch timeout.
Subsystems: SS-01, SS-02, SS-04. Outcome: a plugin can block a tool call (exit 2) or allow it (exit 0) with sub-10ms overhead.
Source: design doc "Decisions" §3; pass-1 §Layer Structure. Justification: grounded in the core architectural decision for WASM-sandboxed hooks.

**CAP-003 — Stream observability events to multiple configurable sinks**
The dispatcher fans out every internal event to all enabled sink drivers (file, OTel gRPC; HTTP/Datadog/Honeycomb planned for rc.1). Sinks are independently configured via `observability-config.toml`.
Subsystems: SS-01, SS-03, SS-10. Outcome: operator sees events in Grafana/Loki or custom endpoint without modifying dispatcher code.
Source: design doc "Decisions" §4; pass-8 §ADR-005. Justification: grounded in the multi-sink observability design decision.

**CAP-007 — Deploy and activate the plugin on any supported platform**
The activate skill detects the operator's OS+arch, copies the matching per-platform `hooks.json`, and verifies the dispatcher binary. Supported platforms: darwin-arm64, darwin-x64, linux-x64, linux-arm64, windows-x64.
Subsystems: SS-06, SS-09. Outcome: `/vsdd-factory:activate` succeeds on all five platforms without manual path configuration.
Source: design doc Q5 resolution; pass-8 §ADR-007. Justification: cross-platform support is a primary product differentiator.
<!-- [process-gap] F-007 fix (Wave 5 SS-06 re-anchor): SS-06 added — the activate/deactivate skills (BC-6.01.003-006, BC-6.03.001-009) implement platform detection and activation logic in SS-06 (Skill Catalog). Wave 3 F-007 precedent: when story.subsystems ⊄ CAP.subsystems, expand the CAP. Both S-0.03 and S-2.06 declare subsystems: ["SS-06", "SS-09"]. SS-01 expansion reverted at Wave 5 pass-1 fix burst per HIGH-002 — no SS-01 BC anchored to activate-skill stories; dispatcher binary dependency is consumed (S-2.04 produces it), not implemented in SS-01. -->

**CAP-008 — Gate tool calls with pre-execution behavioral checks (PreToolUse hooks)**
Bash hooks registered as PreToolUse can block dangerous commands before execution: secret exposure, branch protection violations, AI attribution injection, destructive command patterns.
Subsystems: SS-01, SS-02, SS-04, SS-07. Outcome: a `git push --force` on `main` is blocked before Claude Code executes it.
<!-- [process-gap] F-102 fix (Wave 3 SS-04 pass-2): SS-02 added — the SDK's HookResult exit-code contract (BC-2.01.002) is part of the gating capability; native WASM plugins use SS-02 to return HookResult::Block. Per Wave 1+2 F-007 precedent (CAP-003, CAP-010 expanded). -->
Source: pass-2 §Hook (bash); pass-3-deep-hooks.md. Justification: behavioral gating is a first-class VSDD safety primitive.

**CAP-009 — Author and publish WASM hook plugins using the Rust SDK**
The `vsdd-hook-sdk` crate provides the `#[hook]` macro, `HookPayload`, `HookResult`, and all `vsdd::*` host function bindings. A third-party plugin author can add a dependency and ship a `.wasm` without touching the dispatcher.
Subsystems: SS-02. Outcome: `cargo build --target wasm32-wasip1` produces a drop-in `.wasm` loadable by the dispatcher.
Source: pass-1 §hook-sdk; design doc "WASM plugin ABI". Justification: grounded in the language-agnostic plugin ABI decision (ADR-002).

**CAP-010 — Always-on dispatcher self-telemetry independent of sink config**
`dispatcher-internal-YYYY-MM-DD.jsonl` is written for every invocation regardless of whether any sink is configured or healthy. 30-day rotation.
Subsystems: SS-01, SS-03, SS-10. Outcome: an operator with a misconfigured OTel sink can still audit hook invocations via the internal log.
Source: design doc Q6 Option B; `internal_log.rs`. Justification: grounded in the always-on telemetry ADR (ADR-007).
<!-- [process-gap] F-007: SS-01 added as dominant implementer (internal_log.rs is in
     crates/factory-dispatcher/src/). A subsystem-tag drift sweep across all 28 CAPs is
     recommended — an architect/spec-steward should verify SS assignments for each CAP
     against ARCH-INDEX.md Subsystem Registry to catch similar drift on other entries. -->

**CAP-013 — Capture post-execution activity (PostToolUse hooks)**
Bash hooks registered as PostToolUse capture commit metadata, PR activity, and tool errors for audit and observability purposes.
Subsystems: SS-01, SS-04, SS-07. Outcome: every `git commit` during a session produces a `commit.made` event in the event log.
<!-- [process-gap] F-103 fix (Wave 3 SS-04 pass-2): SS-01 added — dispatcher routing of PostToolUseFailure (and PostToolUse) lives in SS-01; the capability spans SS-01 routing + SS-04 plugin + SS-07 bash-layer. Per Wave 1+2 F-007 precedent (CAP-003, CAP-010 expanded). -->
Source: pass-2 §Hook (bash) capture category. Justification: audit capture is part of the VSDD governance layer.

**CAP-014 — Decompose product specs into verified behavioral contracts (BCs)**
Skills produce `BC-S.SS.NNN`-identified behavioral contracts from stories, grounding every acceptance criterion in a traceable spec artifact.
Subsystems: SS-05, SS-06, SS-08. Outcome: every story's acceptance criteria links to at least one BC-S.SS.NNN identifier.
Source: pass-2 §BehavioralContract; pass-8 §11. Justification: BC traceability is the "verified" in VSDD.

**CAP-016 — Drive TDD delivery with red/green/refactor gate enforcement**
The deliver-story skill enforces a mandatory red-gate (failing test must exist) before the green-gate (minimum implementation), then refactor + review + demo + merge.
Subsystems: SS-05, SS-06, SS-08. Outcome: a story cannot merge without a test that was red before implementation. SS-08 contribution: the story-template `tdd_mode:` field (BC-8.30.001) enables the pipeline to distinguish strict TDD stories from facade-mode stories at the template/artifact level.
Source: pass-2 §Skill deliver-story; pass-8 §story lifecycle state machine. Justification: TDD discipline is a core VSDD delivery constraint.

**CAP-028 — Install and update the plugin via Claude Code marketplace**
The plugin is distributed via `.claude-plugin/marketplace.json` and installs through Claude Code's standard plugin mechanism. Version is co-stamped across `plugin.json`, `CHANGELOG`, and binary bundles.
Subsystems: SS-06, SS-09. <!-- Expanded SS-09 → SS-06,SS-09 per Wave 6 F-005 sanctioned per Wave 3 F-007 precedent (FR-029 activation skill consumes marketplace-installed plugin) --> <!-- F-101 (Wave 6 pass-2): SS-06 enforcer-BC pending — install/update flows through SS-06 activate skill (BC-6.12.x family per FR-029). Specific BC IDs TBD when SS-06 BC backfill closes. Mirrors CAP-007 line 46 inline-comment pattern. --> <!-- F-002 (Wave 7 pass-1): SS-10 target-module declarations on Wave 7 stories (S-0.02 Release.yml, S-4.08/S-5.07 bump-version.sh invocations) are SECONDARY architectural module (per ARCH-INDEX:83 scripts/ wildcard) NOT primary CAP-028 subsystem; primary subsystems remain SS-06,SS-09. F-007/F-005 sanctioned-template-anchor pattern. --> Outcome: `/plugin install vsdd-factory` succeeds and reports `1.0.0-beta.4`.
Source: pass-8 §2 "Plugin (Claude Code marketplace plugin)". Justification: marketplace distribution is the product's delivery channel.

## P1 Capabilities — Should-Have

**CAP-004 — Enforce per-PR behavioral contract traceability**
Every PR must contain evidence that new BCs are covered by tests and demo recordings before merge. The pr-manager-completion-guard and handoff-validator hooks enforce this at SubagentStop.
Subsystems: SS-05, SS-06. Outcome: a PR missing BC→test→demo traceability is blocked at merge.
Source: pass-2 §Hook lifecycle category; pass-8 §L-P0-003.

**CAP-005 — Run adversarial review with information asymmetry**
The adversary agent uses a different model family and fresh context window (no prior conversation) to review artifacts. SHA-currency gating ensures the adversary sees the latest commits.
Subsystems: SS-05, SS-06. Outcome: adversary finds issues that the original author's context-loaded session would miss.
Source: pass-8 §"Key differentiators" (e); design doc §Staged release.

**CAP-006 — Decompose specs into wave-scheduled stories with parallel execution support**
The decompose-stories and wave-scheduling skills produce story batches with explicit tier dependency declarations, enabling parallelism within a wave and strict sequencing between waves.
Subsystems: SS-05, SS-06. Outcome: the orchestrator produces a wave plan where stories in the same tier can be worked concurrently.
Source: pass-2 §Story tier mapping; pass-8 §story coverage rollup.

**CAP-011 — Enforce fuel and epoch budgets on plugin execution**
Every plugin invocation has a bounded fuel cap (default 10M operations) and epoch deadline (derived from `timeout_ms`). Exceeded limits produce `Timeout{Epoch}` or `Timeout{Fuel}` outcomes, never hung processes.
Subsystems: SS-01. Outcome: a runaway plugin is killed within `timeout_ms + EPOCH_TICK_MS (10ms)`.
Source: pass-2 §Engine + EpochTicker; `invoke.rs`. Source: design doc §WASM plugin ABI.

**CAP-012 — Recover from workflow interruption (crash recovery)**
The feature workflow and brownfield-ingest skill checkpoint state to `.factory/STATE.md` and can resume from the last known-good wave rather than starting over.
Subsystems: SS-05. Outcome: a mid-workflow crash loses at most the current story's in-flight work.
Source: pass-8 §NFR catalog (reliability); pass-2 §WorkflowState.

**CAP-015 — Ingest brownfield codebases via structured multi-pass analysis**
The brownfield-ingest skill runs a 7-phase analysis (inventory → architecture → domain model → behavioral contracts → NFRs → conventions → synthesis) with convergence tracking until novelty reaches NITPICK.
Subsystems: SS-06. Outcome: an existing codebase produces a 1,800+ BC catalog with 97%+ confirmation rate.
Source: pass-8 §"Key differentiators" (f); PHASE_0_INGEST declaration.

**CAP-017 — Create and manage formal ADR records**
The create-architecture skill produces fleshed-out `ADR-NNN-*.md` decision records; the create-adr skill (S-6.01) scaffolds new ADR records with frontmatter, ID allocation, and ARCH-INDEX registration. Together they provide the full ADR lifecycle: scaffold (create-adr) → flesh out (create-architecture / architect agent) → accept → supersede.
Subsystems: SS-06, SS-08, SS-10. Outcome: every major design decision has an ADR file with rationale and consequences.
Source: pass-8 §§4, 11 (ADR promotion path); pass-1 §Architecture Decisions.

**CAP-018 — Validate spec consistency across all artifact layers**
The consistency-validator agent cross-checks that entity definitions in the domain spec, BC identifiers in stories, and test assertions in code all refer to the same terms and constraints.
Subsystems: SS-05, SS-06. <!-- F-302 (Wave 6 pass-4): SS-05 added — consistency-validator agent (BC-5.05.007-010) is the implementer of CAP-018; PRD §8:1108 already lists SS-05+SS-06. Per Wave 3 F-007 sanctioned-template-anchor pattern. --> Outcome: a renamed entity surfaces as a consistency violation before a PR merges.
Source: pass-2 §Agent (consistency-validator); pass-8 §L-P0-003.

**CAP-019 — Generate domain specs from product briefs**
The create-domain-spec skill synthesizes the L2 domain specification (entities, invariants, events, capabilities, edge cases) from a product brief and optional brownfield ingestion output.
Subsystems: SS-06, SS-08. Outcome: a product brief produces a sharded L2 domain spec with CAP/DI/DE/DEC identifiers.
Source: this very artifact. Justification: self-referential — the skill that produced this file.

**CAP-020 — Produce and maintain a PRD with NFR catalog**
The create-prd skill iterates from brief to PRD master + 4 supplements (NFR catalog, error taxonomy, interface definitions, test vectors).
Subsystems: SS-06. Outcome: a complete PRD with 76+ NFRs across 8 categories.
Source: pass-2 §Skill create-prd; pass-8 §11 (create-prd guidance).

**CAP-021 — Perform formal verification of pure domain logic**
The formal-verify skill applies Kani, proptest, and fuzz tools to the pure core (routing, payload parsing, registry validation) to produce machine-checked provable properties.
Subsystems: SS-06. Outcome: a provable property catalog with Kani harness coverage.
Source: pass-8 §11; ARCH-INDEX.md §Verification Architecture.

## P2 Capabilities — Nice-to-Have

**CAP-022 — Port hook plugins from bash to native WASM**
Individual bash hook scripts can be replaced with typed WASM plugins (Tier E: S-3.1/3.2/3.3) to eliminate the legacy-bash-adapter dependency for that hook and unlock Windows native support.
Subsystems: SS-04, SS-06. Outcome: a ported hook works on Windows without git-bash.
Source: pass-8 §Story coverage rollup Tier E; DRIFT-010.

**CAP-023 — Ship advanced observability sinks (HTTP, Datadog, Honeycomb)**
Planned sink drivers (S-4.1/4.2/4.3) allow direct event forwarding without a local OTel collector. Unknown sink driver types warn-and-skip today.
Subsystems: SS-01, SS-03. Outcome: an operator forwards events to Datadog with zero local disk footprint.
Source: pass-8 §ADR-005; design doc §Multi-instance, multi-backend observability.

**CAP-024 — Per-sink retry, circuit breaker, and dead-letter queue**
S-4.4/4.5: each sink driver retries failed sends with exponential backoff, trips a circuit breaker on sustained failure, and routes dropped events to a `dead-letter-<sink>-<date>.jsonl`.
Subsystems: SS-01, SS-03, SS-10. Outcome: a Datadog outage doesn't lose events — they land in the DLQ.
Source: pass-8 §DRIFT-002; design doc §Multi-instance observability sinks.

<!-- [process-gap] CAP subsystem drift sweep confirmed across 4 CAPs (CAP-003, CAP-010,
CAP-023, CAP-024) during Wave 2 SS-03 adversarial pass-1 fix burst. Recommend
architect/business-analyst run a comprehensive 28-CAP audit before Wave 3 to
surface any remaining cross-subsystem coverage gaps. -->

**CAP-025 — Generate semantic port translations between language implementations**
The semport-analyze skill translates a component catalog from one language to another, preserving behavioral contracts across the port.
Subsystems: SS-06, SS-08. Outcome: a Go port of the dispatcher starts from a semantically-equivalent component catalog, not a blank file.
Source: pass-8 §11 (semport-analyze guidance); pass-2 §Skill semport-analyze.

**CAP-026 — Manage multi-repo health and cross-repo traceability**
The multi-repo-health skill scans multiple repositories for spec drift, BC coverage gaps, and version mismatches across a portfolio.
Subsystems: SS-06. Outcome: a portfolio dashboard surfaces which repos are behind on spec crystallization.
Source: pass-2 §Skill (multi-repo); pass-8 §convention catalog.

**CAP-027 — Emit structured events from bash hooks via CLI tool**
The `bin/emit-event` CLI tool normalizes event emission from bash hooks, writing to the internal log and configured sinks. This bridges the legacy bash layer until native WASM ports complete.
Subsystems: SS-07, SS-10. Outcome: a bash hook emits a `hook.block` event that appears in Grafana.
Source: pass-2 §Event (logical); pass-8 §L-P0-004.
