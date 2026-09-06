---
document_type: domain-spec-section
level: L2
section: capabilities
version: "1.18"
status: accepted
producer: business-analyst
timestamp: 2026-04-25T00:00:00
last_amended: "2026-09-05 (v1.18) — S-25.02 F2 fix-burst (product-owner; F-S2502-F2-009, POLICY 19): CAP-043 §Source de-loaded of its volatile ADR-047 v1.6 version pin (stable §Decision 8a anchor retained); added BC-1.18.011 (new governed one-time B2 migration BC) and ADR-051 Decision 10 to the citation list. No CAP count change (still 43)."
phase: 1.3
inputs:
  - .factory/phase-0-ingestion/pass-2-domain-model.md
  - .factory/phase-0-ingestion/pass-8-final-synthesis.md
  - .factory/legacy-design-docs/2026-04-24-v1.0-factory-plugin-kit-design.md
  - .factory/specs/architecture/ARCH-INDEX.md
  - .factory/specs/architecture/decisions/ADR-051-layer-2-two-mechanism-size-triggered-shard-rotation-append-logs-and-bc-index-sharding.md
input-hash: "8ea0a46"
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

**CAP-002 — Hook Claude Code tool calls and session/worktree lifecycle events with sandboxed WASM plugins**
Every Claude Code tool invocation (Bash, Edit, Write, etc.) and every session/worktree lifecycle event (SessionStart, SessionEnd, WorktreeCreate, WorktreeRemove) triggers the dispatcher, which routes to matching WASM plugins by event type and tool-regex. Plugins run in wasmtime with bounded fuel + epoch timeout.
Subsystems: SS-01, SS-02, SS-04. Outcome: a plugin can block a tool call (exit 2) or allow it (exit 0) with sub-10ms overhead; lifecycle events emit structured telemetry.
Source: design doc "Decisions" §3; pass-1 §Layer Structure. Justification: grounded in the core architectural decision for WASM-sandboxed hooks. Lifecycle events use the same dispatcher + WASM-plugin sandbox as tool calls — splitting into a separate CAP would over-fragment the capability registry and introduce traceability ambiguity.
<!-- [arch-decision] Decision C (S-5.01 adversarial pass-1, 2026-04-28): CAP-002 widened from "tool calls" to include "session/worktree lifecycle events". SessionStart is not a tool call but uses the same dispatcher + WASM sandbox; introducing a separate CAP would require dual-anchor for every lifecycle plugin story and orphan the traceability chain. Architect confirmed Option 1 (widen). -->

**CAP-003 — Stream observability events to multiple configurable sinks**
The dispatcher fans out every internal event to all enabled sink drivers (file, OTel gRPC; HTTP/Datadog/Honeycomb planned for rc.1). Sinks are independently configured via `observability-config.toml`.
Subsystems: SS-01, SS-03, SS-10. Outcome: operator sees events in Grafana/Loki or custom endpoint without modifying dispatcher code.
Source: design doc "Decisions" §4; pass-8 §ADR-005. Justification: grounded in the multi-sink observability design decision.
**Status:** REWRITTEN per ADR-015 D-15.1 (multi-sink model retired in favor of single events-*.jsonl stream + external OTel Collector fan-out).
**Current canonical wording:** Stream observability events to a single `events-*.jsonl` file path; downstream multi-sink fan-out delegated to an external OTel Collector.

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
<!-- F-009 (Wave 8 pass-1): docs-stories S-0.05/S-5.05/S-5.06 anchored to CAP-014 via SS-08 methodology BCs (BC-8.22.001/26.001/26.006); content spans CAP-007/CAP-028 semantics — see story-body Stretch-Anchor Disclosures. 6 v1.1 BC candidates BC-8.31.003-008 registered in story v1.1 candidate tables (story-table-only; no on-disk BC files; BC-8.31.001-002 gap-numbered for future expansion). (Updated Wave 14 pass-2: BC-8.31.008 added by S-5.05 v1.5; count corrected from 7 to 6.) -->

**CAP-016 — Drive TDD delivery with red/green/refactor gate enforcement**
The deliver-story skill enforces a mandatory red-gate (failing test must exist) before the green-gate (minimum implementation), then refactor + review + demo + merge.
Subsystems: SS-05, SS-06, SS-08. Outcome: a story cannot merge without a test that was red before implementation. SS-08 contribution: the story-template `tdd_mode:` field (BC-8.30.001) enables the pipeline to distinguish strict TDD stories from facade-mode stories at the template/artifact level.
Source: pass-2 §Skill deliver-story; pass-8 §story lifecycle state machine. Justification: TDD discipline is a core VSDD delivery constraint.

**CAP-028 — Install and update the plugin via Claude Code marketplace**
The plugin is distributed via `.claude-plugin/marketplace.json` and installs through Claude Code's standard plugin mechanism. Version is co-stamped across `plugin.json`, `CHANGELOG`, and binary bundles.
Subsystems: SS-06, SS-09. <!-- Expanded SS-09 → SS-06,SS-09 per Wave 6 F-005 sanctioned per Wave 3 F-007 precedent (FR-029 activation skill consumes marketplace-installed plugin) --> <!-- F-101 (Wave 6 pass-2): SS-06 enforcer-BC pending — install/update flows through SS-06 activate skill (BC-6.12.x family per FR-029). Specific BC IDs TBD when SS-06 BC backfill closes. Mirrors CAP-007 line 46 inline-comment pattern. --> <!-- F-002 (Wave 7 pass-1): SS-10 target-module declarations on Wave 7 stories (S-0.02 Release.yml, S-4.08/S-5.07 bump-version.sh invocations) are SECONDARY architectural module (per ARCH-INDEX:83 scripts/ wildcard) NOT primary CAP-028 subsystem; primary subsystems remain SS-06,SS-09. F-007/F-005 sanctioned-template-anchor pattern. --> Outcome: `/plugin install vsdd-factory` succeeds and reports `1.0.0-beta.4`.
Source: pass-8 §2 "Plugin (Claude Code marketplace plugin)". Justification: marketplace distribution is the product's delivery channel.

**CAP-029 — Emit structured events to a single observability stream (file path)**
The dispatcher writes every user-facing domain event as a JSONL record to a single `events-YYYY-MM-DD.jsonl` file via FileSink. Router, SinkRegistry, and DlqWriter are retired; all downstream multi-sink fan-out is delegated to an external OTel Collector that reads the file. The `dispatcher-internal-*.jsonl` debug stream is gated to `VSDD_DEBUG_LOG=1` env var or `debug_log_enabled = true` in `observability-config.toml`.
Subsystems: SS-01, SS-03. Outcome: every dispatched hook event appears as a parseable JSONL line in `events-*.jsonl` with no Router/SinkRegistry indirection.
Source: ADR-015 D-15.1 (single-stream FileSink design). Justification: CAP-003 described the now-retired multi-sink model; CAP-029 captures the ADR-015 replacement as a first-class capability.

## P1 Capabilities — Should-Have

**CAP-030 — Enrich emitted events with OTel-aligned resource attributes**
At dispatcher startup, a Resource attribute block of 15 OTel-aligned fields (per ADR-015 D-15.2 Resource attributes table — the authoritative enumeration; see ADR-015 §D-15.2 "Resource attributes") is stamped once and attached to every emitted event. Per-event identity fields (`trace_id`, `event.id`, `event.category`, `event.name` in reverse-DNS + `.vN` format) are stamped at emit time by the host.

**CHANGELOG / Errata:**
D-318 (2026-05-06): The original 15-field enumeration in this CAP description (authored 2026-05-06 in D-314) does not match ADR-015 D-15.2's canonical 15-field enumeration. CAP-030 is authoritatively defined by ADR-015 D-15.2 — see "Resource attributes" table in ADR-015. The original enumeration above is historical; do not rely on it. The authoritative 15-field set is: `service.name`, `service.namespace`, `service.instance.id`, `service.version`, `deployment.environment.name`, `host.name`, `host.id`, `os.type`, `process.pid`, `vcs.repository.url.full`, `vcs.repository.name`, `vcs.provider.name`, `vcs.owner.name`, `worktree.id`, `schema_url`.

Subsystems: SS-01, SS-03. Outcome: every event in `events-*.jsonl` carries a complete OTel-aligned resource block enabling correlation across Grafana/Loki/Honeycomb without post-processing enrichment.
Source: ADR-015 D-15.2 (OTel-aligned schema; 15-attribute Resource block). Justification: no existing capability described the Resource attribute enrichment contract; ADR-015 D-15.2 adds it as a first-class normative decision.

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
Every plugin invocation has a bounded fuel cap (default 20M operations, per ADR-042 §Decision 1) and epoch deadline (derived from `timeout_ms`). Exceeded limits produce `Timeout{Epoch}` or `Timeout{Fuel}` outcomes, never hung processes.
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
**Status:** SUPERSEDED by ADR-015 D-15.1 (multi-sink replaced by external OTel Collector fan-out; per-sink retry/circuit-breaker no longer in scope).

**CAP-024 — Per-sink retry, circuit breaker, and dead-letter queue**
S-4.4/4.5: each sink driver retries failed sends with exponential backoff, trips a circuit breaker on sustained failure, and routes dropped events to a `dead-letter-<sink>-<date>.jsonl`.
Subsystems: SS-01, SS-03, SS-10. Outcome: a Datadog outage doesn't lose events — they land in the DLQ.
Source: pass-8 §DRIFT-002; design doc §Multi-instance observability sinks.
**Status:** SUPERSEDED by ADR-015 D-15.1 (per-sink DLQ no longer applicable to single-stream FileSink; FileSink uses BC-1.11.002 partial-write recovery instead).

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

**CAP-031 — Enforce single-writer cross-session exclusivity on factory-artifacts state**
A cooperative advisory lock prevents two developers from concurrently mutating the `factory-artifacts` orphan branch. The `verify-factory-lock` WASM PreToolUse guard blocks mutating tools (Edit, Write, Agent dispatch, `git push` to `factory-artifacts`) when another developer holds an unexpired lock; reads always pass. Lock state lives in `STATE.md` frontmatter (`factory_lock.holder`, `locked_at`, `expires_at`). Lock acquisition uses fetch-then-`--force-with-lease` CAS push (closing the primary TOCTOU acquire-race, CWE-367). TTL is 45 minutes with mid-burst renewal; an expired or absent lock is treated as unlocked. Fail-open on guard crash preserves the guard as advisory/efficiency-class (Kleppmann §8); the CAS push is the independent safety net. Force-unlock (`/factory-unlock --force`) is audited via `factory.lock.stolen`.
Subsystems: SS-04, SS-05, SS-06. Outcome: Developer A running `/factory-lock` blocks Developer B's Edit/Write/Agent dispatch until the lock expires or is released, with an actionable refusal message naming holder, expiry, and break-glass command.
Source: ADR-025 v1.2 (issue #170); D-540.

**CAP-032 — Guarantee lossless context-window transitions via wave-boundary checkpoint and PreCompact flush**
The factory provides durable continuity across two failure classes: (a) wave-boundary session resets, where the human clears the Claude Code context at the end of wave N; and (b) mid-wave auto-compaction events, where the harness summarizes the context window without factory coordination. Wave-boundary continuity is guaranteed by requiring a verified `HANDOFF.md` artifact on the `factory-artifacts` branch before a wave may be declared closed — the file carries anti-fabrication cross-checks (git-verified SHAs, filesystem-verified BC paths) so no field can be fabricated from in-context memory. Mid-wave compaction losslessness is guaranteed by the `precompact-flush.sh` hook, which fires synchronously on the `PreCompact` harness event, commits `STATE.md` and wave-critical state to `factory-artifacts` before compaction proceeds, and blocks compaction (exit 2) if the commit fails. After compaction, `postcompact-reanchor.sh` fires on `PostCompact` (advisory only; cannot block) and emits a re-anchor block restating pipeline identity and `last_verified_develop_sha` from the durable external source — for the self-referential engine this is `current_cycle:` + `current_step:` read from STATE.md on the `factory-artifacts` branch; for product pipelines this is the wave-group ordinal derived from the topo-sort of `sprint-state.yaml` pending/draft entries. No `current_wave:` field is emitted or read by the re-anchor hook; that field does not exist in STATE.md frontmatter (DI-023). A WASM gate (`validate-wave-handoff-completeness`) enforces handoff completeness on `HANDOFF.md` writes. Session rehydration after a wave reset uses a curated `wave-state.yaml` manifest (deterministic; no RAG). Requires harness ≥ v2.1.105.
Subsystems: SS-01, SS-04, SS-05, SS-06, SS-07. Outcome: A new session started after wave N rehydrates exactly the specs and stories needed for wave N+1 from a verified, git-committed checkpoint. Mid-wave compaction events lose no load-bearing state (SHAs, decisions, active BC IDs). Fabricated-SHA risk class is directly eliminated via anti-fabrication cross-checks in HANDOFF.md.
Source: ADR-026 (issue #173); E-18. (First authored against ADR-026 v1.0 — informational, non-load-bearing.)

**CAP-033 — Enforce pr-manager merge-operation integrity — READY-verdict SHA pinning, stale-verdict detection, and release-branch merge-strategy guard**
Three mechanically-enforced guards close distinct unsafe-merge windows: (a) the `pr-manager-completion-guard` SubagentStop hook blocks READY verdicts lacking a `covered_sha: <40-hex>` field (`READY_SHA_MISSING`), preventing the orchestrator from ever acting on a verdict that cannot be staleness-checked; (b) `check-stale-verdict.sh` compares the live PR HEAD (`gh pr view --json headRefOid`) against the `covered_sha` embedded in the verdict immediately before any `gh pr merge` call, exiting non-zero with `STALE_READY_VERDICT` if the HEAD has advanced since review; (c) `enforce-merge-strategy.sh` wraps `gh pr merge` and forces `--merge` on branches matching `^release/v`, emitting `RELEASE_PR_SQUASH_FORBIDDEN` if `--squash` or `--rebase` is supplied for a release branch.
Subsystems: SS-05, SS-07. Outcome: the orchestrator cannot merge on a stale READY verdict; release PRs are mechanically guaranteed to use `--merge`; the RELEASING.md `--merge` requirement is enforced at the script layer rather than by convention.
Source: D-749 (L-BB-merge-race-ready-report-stale-head) + D-750 (L-BB-release-pr-squash-merge-not-mechanically-enforced); BC-5.42.001; S-19.01. Justification: no existing capability covers merge-operation integrity at the pr-manager level; CAP-004 covers BC→test→demo traceability (distinct concern); append-only P1 addition at next free ID.

**CAP-034 — Enforce factory artifact nested-worktree path exclusivity (E-21 INV-E21-001) — dual-layer defense: WASM Bash guard + skill-doc merge pre-check**
Two complementary enforcement layers prevent `.factory/**` paths from appearing in product-branch diffs. Layer 1 (invariant): new `validate-factory-path-staging` WASM PreToolUse plugin fires on `^Bash$` and blocks `git add`/`git stage` of `.factory/` paths on product branches, preventing dual-tracking at staging time. Layer 2 (safety-net): live surface = undocumented ad-hoc orchestrator/operator `git pull`/`git merge` Bash on the main product checkout (not documented in any single agent protocol file); enforcement site = `per-story-delivery.md` §Main-Checkout Sync Protocol (S-21.01 Layer-2 deliverable). Server-side origination is the primary threat vector: contributor PRs adding `.factory/`-pathed files merged server-side bypass Layer 1 entirely; the subsequent `git pull` on the main checkout is the clobber vector Layer 2 guards against. Agents excluded from Layer-2 hosting: `pr-manager` (merges server-side via `gh pr merge`, never runs a local Bash merge — BC-5.43.001 PC3); `state-manager` (operates exclusively via `git -C .factory` on the factory-artifacts worktree, never touches the main product checkout).
Subsystems: SS-04, SS-05. Outcome: `.factory/**` paths cannot enter a product-branch diff via either the staging path or the merge path; pre-existing dual-tracking is intercepted before merge even if it preceded the runtime guard.
Source: ADR-031 Decision 2+7; E-21 INV-E21-001; BC-4.16.001 (SS-04 Layer 1), BC-5.43.001 (SS-05 Layer 2); S-21.01. Justification: no existing capability covers nested-worktree path exclusivity at the staging or merge boundary; CAP-031 covers single-writer lock semantics (distinct concern); append-only P1 addition at next free ID.

**CAP-035 — Post-rebase diff-integrity gate — detect and surface silent ORT production-code drops before force-push (E-21 INV-E21-005)**
After any `git rebase`, `git rebase --continue`, or `git pull --rebase` on a feature branch, a mandatory diff-integrity gate runs before `git push --force-with-lease`. The gate runs `git diff origin/develop --stat`, identifies files with net-negative line counts that were also modified by recently-merged sibling stories on `origin/develop`, and requires per-file confirmation that each such delta is intentional before proceeding. Any unverified net-negative delta in a sibling-touched file halts the force-push with `UnverifiedNetNegativeDelta`. Closes the ORT 3-way merge silent-drop failure mode (issue #365) where a clean rebase can silently delete production lines when adjacent regions of the same file are modified by two branches.
Subsystems: SS-05. Outcome: a silent ORT drop of production lines is detected before reaching origin/develop; the force-push is blocked until the delta is manually verified or the dropped lines are restored.
Source: ADR-031 Decision 6+7; E-21 INV-E21-005; BC-5.44.001; S-21.02. Justification: no existing capability covers post-rebase diff-integrity assertion; CAP-033 covers READY-verdict SHA pinning and merge-strategy guard (distinct concern); append-only P1 addition at next free ID.

**CAP-036 — Story-worktree factory artifact write-path discipline and teardown preflight (E-21 INV-E21-002 + INV-E21-004)**
Two mandatory agent disciplines govern story-worktree interactions with the `.factory/` subtree. PC1 (write-path anchoring): all `.factory/**` writes must use canonical absolute paths anchored to `$(git -C .factory rev-parse --show-toplevel)`, never CWD-relative paths derived from the story-worktree root, which silently misdirects writes to the shadow `.factory/` directory present under every `git worktree add` path. PC2 (teardown preflight): before `git worktree remove <story-worktree>`, a `find <worktree-path>/.factory -type f 2>/dev/null` preflight must confirm the shadow `.factory/` directory contains no tracked files. Both enforced via skill-doc mandate (BC-6.26.001); no new WASM plugin required (POLICY 21 satisfied).
Subsystems: SS-06. Outcome: story-worktree write operations cannot misdirect factory artifact mutations to shadow `.factory/` paths; worktree teardown cannot silently delete committed factory artifacts.
Source: ADR-031 Decision 4+7; E-21 INV-E21-002+INV-E21-004; BC-6.26.001; S-21.04. Justification: no existing capability covers story-worktree write-path discipline or teardown preflight; append-only P1 addition at next free ID.

**CAP-037 — Factory worktree branch integrity — dispatch-preamble assertion and factory-side PR restoration protocol (E-21 INV-E21-003)**
Two mandatory protocol elements ensure the `.factory/` worktree is always on `factory-artifacts` before any write. Dispatch-preamble assertion: every state-manager dispatch must begin with `ASSERT: git -C .factory branch --show-current == "factory-artifacts"` before any `.factory/**` write. Factory-side PR restore sequence: after any `gh pr merge` that was submitted from a chore branch on `factory-artifacts`, the responsible agent must execute the 5-step restore sequence — (1) `git -C .factory checkout factory-artifacts`, (2) `git -C .factory pull --ff-only`, (3) delete local chore branch, (4) delete remote chore branch, (5) final branch assertion. Both enforced via skill-doc mandate (BC-6.27.001); no new WASM plugin required (POLICY 21 satisfied). Explicitly distinct from CAP-031 (cooperative lock/lease per ADR-025): CAP-031 prevents concurrent developer races; this capability prevents single-developer writes to the wrong branch after a factory-side PR merge.
Subsystems: SS-06. Outcome: `.factory/**` writes cannot misdirect to an inactive chore branch; factory-side PR merges are always followed by a verified return to `factory-artifacts`.
Source: ADR-031 Decision 5+7; E-21 INV-E21-003; BC-6.27.001; S-21.05. Justification: no existing capability covers the dispatch-preamble branch assertion or factory-side PR restore sequence; CAP-031 distinction noted; append-only P1 addition at next free ID.

**CAP-038 — Factory PR trunk ancestry integrity — post-create baseRefName assertion and post-merge ancestry guard (E-21 INV-E21-006)**
Two mandatory post-action assertions in the pr-manager 9-step lifecycle protect against
trunk-ancestry drift. Post-create assertion (Step 3): immediately after `gh pr create`,
the pr-manager agent MUST verify `gh pr view --json baseRefName --jq '.baseRefName'`
equals the configured trunk branch (e.g., `develop`). If the assertion fails, the
story is stopped before any review or merge proceeds. Post-merge ancestry guard
(Step 9): immediately after `gh pr merge` resolves with `state=MERGED`, the agent MUST
verify `git merge-base --is-ancestor <merge-commit-sha> origin/<trunk>` exits 0. If
the assertion fails, a P0 data-error signal is raised and state-manager is alerted
before the story delivery ledger is updated. Both assertions are skill-doc mandates on
BC-6.10.002 (amendment); no new WASM plugin required (POLICY 21 satisfied). Explicitly
distinct from CAP-033 (READY-verdict SHA pinning + stale-verdict detection +
release-branch merge-strategy guard, which targets the merge-operation itself rather
than post-create and post-merge integrity assertions).
Subsystems: SS-05. Outcome: PRs with a mismatched base branch are detected before review
proceeds; PRs whose merge commits do not land on trunk are detected before delivery is
declared complete.
Source: ADR-031 Decision 8; E-21 INV-E21-006; BC-6.10.002 (amendment); S-21.03.
Justification: CAP-033 does not cover the baseRefName post-create check or the
post-merge ancestry assertion; append-only P1 addition at next free ID.

**CAP-039 — Break-glass operator override for self-locking PreToolUse `^Agent$` gates**
An environment-variable override, `VSDD_BREAK_GLASS_GATE_BYPASS`, read directly by the
dispatcher's gate-evaluation path (`crates/factory-dispatcher/src/executor.rs::execute_tiers`)
for exactly two named `legacy-bash-adapter.wasm`-hosted validator plugins registered on
`event = "PreToolUse"`, `tool = "^Agent$"` — `validate-wave-gate-prerequisite` and
`validate-pr-merge-prerequisites`. A fail-closed validator gate on the `Agent` tool is a
self-lock hazard class (Kubernetes/GKE/OPA Gatekeeper admission-webhook precedent): if either
gate wedges (miscalibrated `timeout_ms`, hang, unmodeled failure mode), no subsequent `Agent`
dispatch can occur — including the dispatch needed to fix the miscalibration. Setting the
variable does not require a working `Agent` dispatch (it is read from process environment, a
path that never routes through the dispatcher's own `Agent`-tool gate), satisfying the
self-lock-prevention property. Authentication is by-possession of shell/process-environment
access to the machine running the session — this factory has no separate credential layer for
hook bypass. The override is per-invocation only: it does not mutate `hooks-registry.toml` and
does not disable the plugin's own validation logic. Every activation is mandatorily audited via
a structured `break_glass.activated` event to the dispatcher-internal JSONL log (gate name(s),
timestamp, dispatch trace UUID) — an unaudited bypass would reintroduce the same "silent
approval" failure mode (CWE-636 lineage) that CAP-011's fail-closed enforcement exists to close,
at a different layer.
Subsystems: SS-01. Outcome: an operator whose `Agent` dispatch is wedged by a fail-closed
`validate-wave-gate-prerequisite` or `validate-pr-merge-prerequisites` block can set
`VSDD_BREAK_GLASS_GATE_BYPASS` in their shell and immediately dispatch again, with the bypass
event durably recorded in the audit log; an unset (or non-matching) variable leaves normal
fail-closed behavior fully intact.
Source: ADR-039 §Decision 3 v1.10 amendment (break-glass minimum-viable definition); BC-1.03.018;
S-21.11. Justification: no existing capability covers an operator escape hatch for a
self-locking PreToolUse `Agent`-dispatch gate. CAP-002 covers the dispatcher's normal
block/allow hook-routing decision (this capability overrides that decision, not a member of
it); CAP-008 covers Bash-tool PreToolUse gating specifically (destructive commands, secrets,
AI attribution) — a distinct tool and distinct check class from the two named `^Agent$` gates;
CAP-011 covers fuel/epoch budget enforcement itself (the mechanism this capability provides an
escape hatch FROM, not a restatement of it); CAP-031 already uses "break-glass" terminology for
`/factory-unlock --force` but governs a distinct concern (single-writer lock/lease exclusivity
on `factory-artifacts`, ADR-025), not validator-gate self-lock escape. Append-only P1 addition
at next free ID.

**CAP-040 — Human-initiated factory session pause and resume checkpoint orchestration**
The `/vsdd-factory:wrap` skill provides the canonical 7-step sequence for safely pausing the
factory pipeline: (1) halt new sub-agent spawning, (2) verify factory health (routing to
`/vsdd-factory:compact-state` or `/vsdd-factory:recover-state` as needed), (3) commit WIP on
all in-flight story branches to durable remote branches — never the default branch; un-committable
state is documented in the checkpoint instead of forced, (4) delegate the pipeline-PAUSED STATE.md
transition and dated Session Resume Checkpoint write to `vsdd-factory:state-manager` — BC-6.23.001
Invariant 5 mandates that the skill NEVER edits STATE.md directly, (5) release the factory lock
via `/vsdd-factory:factory-unlock` if held (skip silently if no lock held), (6) verify all
durability postconditions (clean `factory-artifacts` working tree via `git -C .factory status
--porcelain`, exactly one `## Session Resume Checkpoint` section in STATE.md, `pipeline: PAUSED`
in frontmatter, banner `wc-l` accurate per BC-5.39.005), and (7) emit a `## Factory Wrapped`
report with resume instructions that cite `/vsdd-factory:rehydrate-wave` BEFORE
`/vsdd-factory:next-step` — mandatory post-clear ordering per BC-6.24.001.
Subsystems: SS-06. Outcome: after `/vsdd-factory:wrap` completes, the session can be `/clear`ed
or closed with zero data loss — a fresh session running `/vsdd-factory:rehydrate-wave` then
`/vsdd-factory:next-step` resumes from the durable checkpoint on `factory-artifacts` without
any reliance on in-session memory.
Source: BC-6.28.001; BC-6.23.001 Invariant 5; BC-6.24.001; BC-5.39.005 (banner seal discipline).
Justification: no existing capability covers human-initiated session-wrap orchestration as an
end-to-end sequence. CAP-031 covers the raw lock acquire/release protocol (BC-6.23.001) — a
primitive this capability USES but does not define. CAP-032 covers wave-boundary checkpoint and
PreCompact flush (BC-6.24.001) — triggered by session clear and harness events respectively, not
by explicit human invocation. This capability covers the skill that orchestrates all of the above
primitives into a single deterministic safe-pause sequence at the human's request. Append-only P1
addition; CAP-039 is the prior entry.

**CAP-041 — Validation Integrity: INDETERMINATE Outcome, Durable Mutation Marker, and Next-Advance Gate**
When a fail-closed WASM validator plugin cannot complete — due to fuel exhaustion
(`Trap::OutOfFuel`), epoch timeout (`Trap::Interrupt`), or host output too large
(`PluginResult::Ok{exit_code:0}` with `host_output_too_large_seen == true`) — the dispatcher
classifies the outcome as INDETERMINATE rather than silently passing. This closes the
pre-Layer-1 CWE-754 (Improper Check for Exceptional Conditions) vulnerability where all three
causes produced a false-PASS signal.
For plugins with `failure_policy = "fail-closed"` (Layer 1 Cohort A: `validate-pr-merge-prerequisites`,
`validate-wave-gate-prerequisite`, `validate-factory-path-staging`): (a) the dispatcher emits a
`plugin.indeterminate` event to the OTel-aligned event log, (b) writes an atomic durable marker
at `.factory/unvalidated-mutation.marker` (TOML format: `timestamp`, `plugin_name`,
`artifact_path`, `cause` (fuel|epoch|output-too-large), `trace_id`), and (c) arms the
next-advance gate: the two registered `validate-unvalidated-mutation-marker.wasm` entries block
the subsequent `^Agent$` PreToolUse dispatch AND any `git commit`/`git push` Bash PreToolUse
dispatch until the marker is cleared.
For plugins with `failure_policy = "fail-open"` or absent `failure_policy` (default — all ~76
current production plugins): INDETERMINATE is advisory-only. Only the `plugin.indeterminate`
event is emitted; no marker is written and no gate is triggered. The canonical backward-
compatibility guard test `test_BC_1_18_004_fail_open_default_preserves_advisory_behavior`
MUST NOT be deleted (ADR-047 §Decision 7 preservation obligation).
The marker is cleared by: (Condition A) the same named plugin producing a subsequent PASS
(`DispatchOutcome::Pass`, exit_code=0, `host_output_too_large_seen=false`) on the same
artifact — `delete_marker_if_pass` performs an idempotent `fs::remove_file`; or (Condition B)
operator manual deletion via `rm .factory/unvalidated-mutation.marker` — a fully supported
escape hatch requiring only shell access. Both conditions unblock both gate arms simultaneously.
A FAIL re-validation preserves the marker; INDETERMINATE re-validation overwrites it
(last-writer-wins, single-marker policy).
Subsystems: SS-01 (dispatcher executor, marker write/clear, gate registration), SS-04
(factory-artifacts path hosting the marker), SS-07 (registry `failure_policy` field routing).
Outcome: after Layer 1, the dispatcher can never silently approve a mutation whose authoritative
validator ran out of resources — the mutation is quarantined (marker + gate) until either the
validator succeeds on a re-run or the operator explicitly overrides via `rm`. The fail-open path
(~76 existing plugins) is unaffected: only Cohort A validators (exactly three human-confirmed
entries) are gated in S-25.01. This satisfies NIST SA-8(24) Fail Secure for the bounded
fail-closed cohort.
Source: ADR-047 (§Decision 1 — outcome trichotomy; §Decision 2 — `failure_policy` reuse;
§Decision 3 — durable marker format; §Decision 4 — next-advance gate Arm 1; §Decision 5 —
marker-clear protocol; §Decision 7 — backward-compatibility contract; §Decision 8a — Cohort A
three-validator enumeration; §Decision 9 — git commit/push Arm 2 extension);
BC-1.18.001 (fail-closed INDETERMINATE + marker write); BC-1.18.002 (next-advance gate both
arms); BC-1.18.003 (marker-clear protocol); BC-1.18.004 (fail-open advisory-only anchor);
BC-3.08.001 Event 8 (SS-03 wire-format catalog for `plugin.indeterminate`); S-25.01.
Justification: no existing capability covers the INDETERMINATE outcome class, durable mutation
marker, or next-advance gate. CAP-003 covers the event-stream observability sink (the
`plugin.indeterminate` event routes there, but the gate and marker lifecycle are distinct
concerns not covered by sink observability). CAP-011 covers fuel/epoch budget enforcement as a
block-or-pass decision axis — the INDETERMINATE path is a separate "cannot complete" axis that
ADR-047 adds orthogonally to ADR-039's block/advisory axes. CAP-039 covers the break-glass
override for self-locking gates — a complementary escape hatch for a different failure mode
(gate itself wedges) rather than the marker quarantine this capability defines. Append-only P1
addition; CAP-040 is the prior entry.

**CAP-042 — `last_amended` Write-Path Durable Fix: current-entry-only scalar, `changelog:` prepend discipline, sanctioned migration/rotation tooling, and bash-adapter fuel-budget relief**
D-1149 (2026-09-02) performed a one-time, human-authorized mitigation that split a `last_amended`
frontmatter mega-line (up to 323,499 chars) into a slim current-entry-plus-pointer form on five
files (`STORY-INDEX.md`, `BC-INDEX.md`, `ARCH-INDEX.md`, `VP-INDEX.md`, `STATE.md`), moving the
removed tail into `*-amendment-history.md` sidecars — but the root cause (every state-manager
burst PREPENDING the new entry and wrapping the entire prior value inline as a nested
`[Prior: ...]` bracket chain) remained unfixed, so the field would regrow without bound. This
capability is the durable cure, ratified by ADR-049 (Human-Ratified 2026-09-02, POLICY 22,
Decision Option 2): (a) `last_amended:` becomes current-entry-only on every future write, on all
five files plus every other `.factory/` artifact carrying the field — the burst overwrites it
with a single-line, D-1144-escaped scalar holding ONLY the new entry, never reading or
bracket-wrapping the prior value; (b) the entry displaced from `last_amended` is instead
PREPENDED as one new `changelog:` sequence list item (`ARCH-INDEX.md`/`BC-INDEX.md`/`VP-INDEX.md`
already carry `changelog:`; `STORY-INDEX.md` gains it, completing the D-448(b)/D-414(c)
deferral; `STATE.md` relies on its existing body-level `## Decisions Log`/`## Phase Progress`
instead of a frontmatter `changelog:`) — existing `changelog:` items are left byte-for-byte
untouched, list-item append rather than single-scalar concatenation; (c) a sanctioned,
platform-agnostic Rust `bin/` binary (POLICY 21 `no_new_shell_scripts`) performs the one-time
migration (adding `changelog:` to `STORY-INDEX.md`, confirming current-entry-only shape on all
five files, remediating the pre-existing D-1144 unescaped-double-quote defect in
`BC-INDEX.md`/`ARCH-INDEX.md`/`STATE.md`) and, as a safety-net utility, rotates an
over-long `changelog:` sequence into a per-cycle archive; (d) the write-path fix eliminates the
743-fuel-timeouts/day symptom (2026-09-02) by ensuring the bash-adapter-hosted WASM validators
that scan these five files never again encounter an unbounded single physical line. The AC-002
validator-compatibility audit (ADR-049 findings 1-6) established that zero production validator
code changes are required — `changelog:` is inert to every current reader, and arm_e's Class E1/E2
checks never fire on any of these five files today.
Subsystems: SS-04 (bash-adapter-hosted WASM validators; fuel-budget relief), SS-05 (state-manager
agent + orchestration write-path discipline), SS-06 (`state-burst` skill canonical write-path),
SS-10 (the new sanctioned migration/rotation `bin/` tool). Outcome: no future state-manager burst
can ever regrow a `last_amended` mega-line on any governed file, no future fuel-exhaustion
incident of the 2026-09-02 class recurs, and the one-time D-1149 sidecar-and-migration debt is
retired via a sanctioned tool rather than a repeat ad hoc POL-3 exception.
Source: ADR-049 (Decision 1-7; Rationale; Alternatives Considered); D-1149 + `L-BB-D1149`
(`.factory/cycles/v1.0-brownfield-backfill/decision-log.md`); S-15.03 §Scope Extension
(`last_amended` Write-Path Durable Fix), AC-001..AC-010; BC-5.45.001 (write-path invariant);
BC-10.13.001 (migration/rotation tool); BC-4.18.001 (fuel-budget relief). Justification: no
existing capability covers frontmatter write-path discipline for history-bearing fields. CAP-031
covers single-writer lock semantics on `factory-artifacts` (distinct concern — this capability
governs the SHAPE of one field's write, not who may write). CAP-032 covers wave-boundary
checkpoint/PreCompact-flush continuity (distinct concern — durability across context-window
transitions, not per-field write discipline). CAP-011 covers fuel/epoch budget enforcement as a
block-or-pass decision axis (distinct concern — this capability removes the ROOT CAUSE of the
payload growth that exhausts that budget on these five files, rather than changing the budget or
its enforcement). Append-only P1 addition; CAP-041 is the prior entry.

**CAP-043 — Artifact Sharding Layer 2: Size-Triggered Shard Rotation for Cycle Append-Logs and
BC-INDEX Structured-Catalog Sharding**
Extends Layer 1's INDETERMINATE detection (CAP-041) with a root-cause prevention mechanism: a
native (non-WASM), dispatcher-mediated PreToolUse gate intercepts every `Edit`/`Write`/`MultiEdit`
against a registered sharded artifact, computes the projected post-write byte size, and — if the
projected size would exceed a calibrated `shard_cap_bytes` — performs a roll-before-write (seal
the current shard by rename, create a fresh empty current file, atomically publish the updated
shard index) and returns `HookResult::Block` with an explicit, actionable retry instruction
(transparent write-redirection is not implementable under `HookResult`'s
`Continue`/`Block`/`Error`-only contract). No shard is ever observed over cap by any downstream
reader; no LLM-side size awareness is required or permitted. The cap formula —
`shard_cap_bytes <= (PRACTICAL_FUEL_CEILING / WORST_CASE_FUEL_PER_BYTE) - MAX_SINGLE_RECORD_BYTES
- SAFETY_MARGIN`, evaluated per-artifact as the MINIMUM across every Cohort B validator that reads
it — bounds every sharded artifact's current shard below the size at which the three Cohort B
fuel-exhausting validators (`validate-burst-log`'s Edit/Write/MultiEdit arm, `regression-gate`,
`convergence-tracker`) would fuel-exhaust, eliminating the INDETERMINATE root cause BY
CONSTRUCTION rather than merely detecting it after the fact (CAP-041's scope). This capability has
two mechanisms for two artifact shapes: mechanism A shards four append-only cycle logs
(`decision-log.md`, `burst-log.md`, `lessons.md`, `session-checkpoints.md`) via a stable-current-
filename addressing scheme (the canonical filename is always the latest/active shard; sealed
shards are renamed away with a `<stem>.<seq:04>.md` suffix) requiring zero code change from
shard-unaware readers; mechanism B shards `BC-INDEX.md` — a structured catalog, not an append-only
log — via two sub-mechanisms: B1 reuses the already-shipped `rotate_changelog` primitive
(CAP-042) to automatically rotate the frontmatter `changelog:` array (BC-INDEX's dominant size
driver, 177,305 of 539,713 bytes measured 2026-09-05) once it overflows a configured item count,
and B2 splits the file's ten already-existing `### SS-NN` per-subsystem body sections into
individually-addressable shard files, keyed by the already-authoritative BC-S-prefix→SS-NN mapping
(ARCH-INDEX Subsystem Registry, POLICY 6) for zero-lookup first-level addressing. A mandatory
one-time backfill-split retroactively shards the four EXISTING append-log files (each already
5-19× over the calibrated cap as of 2026-09-05) — without it, this capability would only prevent
future overflow and never shrink the artifacts already producing the majority of observed
INDETERMINATE events, an incomplete delivery of its own stated purpose.
Subsystems: SS-01 (native `shard_manager.rs` dispatcher module; cap-check, roll-before-write,
shard-index and shard-manifest publication), SS-07 (Cohort B `failure_policy` flip on
`hooks-registry.toml`, sequenced on this capability's postconditions holding AND the one-time
backfill-split completing). Outcome: the artifacts responsible for the majority of Layer-1
`plugin.indeterminate` production events (BC-INDEX.md alone: 45.2% of the first 708 events
observed) are structurally bounded below the Cohort B fuel-exhaustion boundary, making it safe —
for the first time — to flip `validate-burst-log`/`regression-gate`/`convergence-tracker` to
`failure_policy = "fail-closed"` without reintroducing the self-inflicted-DoS risk ADR-047 §8a's
Cohort A/B partition was designed to avoid.
Source: ADR-051 (Decisions 1-10; Rationale; Alternatives Considered); ADR-047 §Decision 8b
(ratified future phase this capability elaborates) and §Decision 8a (Cohort B partition,
plugin-name corrected); ADR-039 §Decision 3 (calibration-precedes-fail-closed-flip ordering
constraint Decision 9 depends on); ADR-049 §Decision 6 (`rotate_changelog` primitive B1 reuses);
D-1166 (human widest-scope decision covering both mechanisms in one story); BC-1.18.005 (cap
formula + size-trigger); BC-1.18.006 (roll-before-write + atomic index publish); BC-1.18.007
(retention/compaction policy); BC-1.18.008 (one-time backfill-split); BC-1.18.009 (BC-INDEX
changelog rotation, B1); BC-1.18.010 (BC-INDEX per-subsystem sharding, B2); BC-1.18.011 (governed
one-time B2 migration); BC-7.08.001 (Cohort B fail-closed flip); S-25.02. Justification: no
existing capability covers artifact-size-triggered
shard rotation. CAP-041 covers the INDETERMINATE outcome's DETECTION-and-quarantine path (marker
write, next-advance gate) for whichever validators already went fail-closed — it does not touch
input size at all, and explicitly anticipates this capability as its own "Ratified Future Phase
Layer 2" (ADR-047 §Decision 8b). CAP-042 covers ONE specific unbounded-growth vector
(`last_amended` frontmatter mega-lines) with a write-path discipline fix scoped to five index
files' `last_amended` field; this capability generalizes the size-triggered-rotation PATTERN to
whole-artifact byte-size bounding across a different, broader set of artifacts (cycle append-logs
in full, plus BC-INDEX's body content, not merely one frontmatter field) and reuses CAP-042's
`rotate_changelog` primitive as a component rather than duplicating it. CAP-011 covers fuel/epoch
budget enforcement as a per-invocation block-or-pass decision axis (distinct concern — this
capability prevents the INPUT that would exhaust that budget from ever existing, rather than
changing the budget or the pass/fail decision once exhaustion occurs). Append-only P1 addition;
CAP-042 is the prior entry.

## CHANGELOG

| Version | Date | Change |
|---------|------|--------|
| v1.18 | 2026-09-05 | S-25.02 F2 fix-burst (product-owner; adversary pass-1 findings F-S2502-F2-009 + F-S2502-F2-002 routed): CAP-043 §Source de-loaded of its volatile `ADR-047 v1.6` version pin per TD-VSDD-091/POLICY 19 — the stable `§Decision 8a` section anchor already resolves the citation without the version token, mirroring the CAP-032/`ADR-026` precedent (v1.6 row, same file). Citation list extended to `ADR-051 (Decisions 1-10...)` (was 1-9) and to the new `BC-1.18.011` (governed one-time B2 BC-INDEX migration BC, authored in the same fix-burst). No CAP-043 body-text semantic change and no CAP count change (still 43). |
| v1.17 | 2026-09-05 | F2 spec-evolution, S-25.02 activation (product-owner, orchestrator-dispatched): authored CAP-043 (P1 — Artifact Sharding Layer 2: size-triggered shard rotation for cycle append-logs [mechanism A] and BC-INDEX structured-catalog sharding [mechanism B: B1 changelog rotation + B2 per-subsystem body sharding]; SS-01/SS-07; ADR-051 §Decisions 1-9; ADR-047 §D8a/§D8b; BC-1.18.005–010; BC-7.08.001; S-25.02). Extends CAP-041's detection-and-quarantine model with a root-cause-prevention mechanism. Distinguishes from CAP-041 (INDETERMINATE detection/quarantine — does not touch input size), CAP-042 (one specific frontmatter-field write-path fix — this capability generalizes the pattern and reuses CAP-042's `rotate_changelog` primitive), CAP-011 (fuel/epoch budget enforcement — this capability prevents the oversized input rather than changing the budget). CAP count advance 42→43. |
| v1.16 | 2026-09-02 | S-15.03 Phase B (product-owner, orchestrator-dispatched): authored CAP-042 (P1 — `last_amended` Write-Path Durable Fix: current-entry-only scalar, `changelog:` prepend discipline, sanctioned migration/rotation tooling, and bash-adapter fuel-budget relief; SS-04/SS-05/SS-06/SS-10; ADR-049 §Decision 1-7; BC-5.45.001/BC-10.13.001/BC-4.18.001; S-15.03). Closes the D-1149 mitigation-not-cure gap (`L-BB-D1149`). Distinguishes from CAP-031 (lock semantics), CAP-032 (wave-boundary/PreCompact continuity), CAP-011 (fuel/epoch budget enforcement — this capability removes the root-cause payload growth rather than changing the budget). CAP count advance 41→42. |
| v1.15 | 2026-08-30 | F2 validation-integrity-layer1 (product-owner, orchestrator-dispatched): authored CAP-041 (P1 — Validation Integrity: INDETERMINATE Outcome, Durable Mutation Marker, and Next-Advance Gate; SS-01/SS-04/SS-07; ADR-047 §D1–D9; BC-1.18.001–004; BC-3.08.001 Event 8; S-25.01). Closes pre-Layer-1 CWE-754 false-PASS vulnerability. Cohort A = 3 human-confirmed fail-closed validators in S-25.01 Layer 1. Distinguishes from CAP-003 (sink observability), CAP-011 (fuel/epoch budget enforcement), CAP-039 (break-glass gate bypass). CAP count advance 40→41. |
| v1.14 | 2026-08-29 | F2 feature-mode wrap-skill (product-owner, orchestrator-dispatched): authored CAP-040 (P1 — human-initiated factory session pause and resume checkpoint orchestration; SS-06; BC-6.28.001; BC-6.23.001 Invariant 5; BC-6.24.001; BC-5.39.005). Distinguishes from CAP-031 (raw lock acquire/release protocol) and CAP-032 (wave-boundary checkpoint / PreCompact flush). CAP count advance 39→40. |
| v1.13 | 2026-08-20 | S-21.25 adversarial pass-2 fix (LOW; brownfield cycle v1.0-brownfield-backfill, product-owner, orchestrator-dispatched): CAP-011 body's ADR-042 section cite corrected "§Decision 2" → "§Decision 1". ADR-042 §Decision 1 ("New fuel budget value: 20,000,000 (20M), derivation from measured data") is the section that actually sets the 20M default; §Decision 2 covers a different concern ("Raise is global … not per-plugin"). The v1.12 fix (immediately below) introduced the wrong section number while correcting the stale "10M" figure; this row aligns the cite with `crates/factory-dispatcher/src/invoke.rs`'s `DEFAULT_FUEL_CAP` doc comment and BC-1.03.019 Precondition 2/Architecture Anchors, both of which already cite "§Decision 1" correctly. No capability semantics, subsystem mapping, or outcome statement altered — precision fix only. |
| v1.12 | 2026-08-20 | F-S2125-P1-007 fix (LOW, pre-existing; S-21.25 adversarial review, brownfield cycle v1.0-brownfield-backfill, architect, orchestrator-dispatched): CAP-011 body corrected "default 10M operations" → "default 20M operations (per ADR-042 §Decision 2)". The 10M figure predated ADR-042's fuel-cap raise and had gone stale; BC-1.03.019 anchors to CAP-011, making the staleness load-bearing. No capability semantics, subsystem mapping, or outcome statement altered — precision fix only. |
| v1.11 | 2026-08-19 | S-21.11 expanded-scope BC coverage burst (product-owner, orchestrator-directed): authored CAP-039 (P1 — break-glass operator override for the two self-locking PreToolUse `^Agent$` validator gates; SS-01; ADR-039 §Decision 3 v1.10 amendment; BC-1.03.018; S-21.11). Distinguished from CAP-002 (normal hook block/allow decision), CAP-008 (Bash-tool PreToolUse gating), CAP-011 (the fuel/epoch enforcement this capability bypasses), and CAP-031 (factory-lock break-glass — same term, distinct concern). CAP count advance 38→39. |
| v1.10 | 2026-07-19 | F-P6-001 (architect): CAP-034 Layer-2 sentence corrected from pre-F-P2-001 framing (named orchestrator/pr-manager/state-manager as Layer-2 hosts — retracted at ADR-031 v1.3) to current framing: live surface = undocumented ad-hoc orchestrator/operator git pull/merge Bash on the main product checkout; enforcement site = per-story-delivery.md §Main-Checkout Sync Protocol (S-21.01 Layer-2 deliverable); pr-manager explicitly excluded (merges server-side via gh pr merge — BC-5.43.001 PC3); state-manager explicitly excluded (operates via git -C .factory only, never touches main checkout); server-side origination documented as primary threat vector. TD-VSDD-060 sweep: one hit at CAP-034 line 220 (fixed this burst); CAP-035..038 and all other E-21 text clean. |
| v1.9 | 2026-07-19 | E-21 factory state data-loss hardening: authored CAP-034 (P1 — nested-worktree path exclusivity, two-layer defense; SS-04+SS-05; ADR-031; BC-4.16.001+BC-5.43.001; S-21.01), CAP-035 (P1 — post-rebase diff-integrity gate; SS-05; ADR-031; BC-5.44.001; S-21.02), CAP-036 (P1 — story-worktree write-path discipline+teardown preflight; SS-06; ADR-031; BC-6.26.001; S-21.04), CAP-037 (P1 — factory worktree branch integrity; SS-06; ADR-031; BC-6.27.001; S-21.05), CAP-038 (P1 — factory PR trunk ancestry integrity, post-create baseRefName + post-merge ancestry guard; SS-05; ADR-031; BC-6.10.002 amendment; S-21.03) [added v1.9 via F-P1 adjudication]. CAP count advance 33→38. |
| v1.8 | 2026-07-06 | F-P3-015/F-P3-016 capability-mapping: authored CAP-033 (P1 — pr-manager merge-operation integrity; READY-verdict SHA pinning + stale-verdict detection + release-branch merge-strategy guard; SS-05+SS-07; D-749+D-750; BC-5.42.001; S-19.01). CAP count advance 32→33. |
| v1.7 | 2026-06-15 | F-P18-O1 cosmetic fix: CHANGELOG display rows reordered into monotonic descending order (newest-first) to prevent a scrambled sequence from masking future missing-row defects. No row content, version number, or date was altered. All versions v1.0–v1.6 confirmed present. |
| v1.6 | 2026-06-14 | O-P8-001 cite-stability fix (F2 adversarial pass-8): CAP-032 body `Source:` line migrated from volatile-pin `ADR-026 v1.0 (issue #173); E-18.` to stable anchor form `ADR-026 (issue #173); E-18.` with informational non-load-bearing parenthetical per TD-VSDD-091 / POLICY 19 spirit. Changelog row v1.4 historical version mention preserved as authoring-time record (non-normative). |
| v1.5 | 2026-06-14 | F-P3-005 sibling-sweep fix (F2 adversarial pass-3, E-18): CAP-032 PostCompact re-anchor description updated to remove phantom `current_wave` field. Re-anchor now restates pipeline identity as `current_cycle:` + `current_step:` (engine) or sprint-state.yaml topo-sort wave-group ordinal (product pipelines), sourced from STATE.md on `factory-artifacts`. Explicit normative note added: no `current_wave:` field is emitted or read (DI-023). |
| v1.4 | 2026-06-14 | F2 E-18 context-durability: Authored CAP-032 (P0 — wave-boundary checkpoint/reset and lossless intra-wave compaction; ADR-026 v1.0; issue #173). Spans SS-01/SS-04/SS-05/SS-06/SS-07. |
| v1.3 | 2026-06-10 | D-540 / issue #170. Authored CAP-031 (P0 — single-writer cross-session factory lock/lease; ADR-025 v1.2). Spans SS-04/SS-05/SS-06. |
| v1.2 | 2026-05-06 | D-318 F-1 fix: CAP-030 enumeration corrected to reference ADR-015 D-15.2 authoritatively. Original enumeration preserved as historical record per POLICY 1. Errata note appended to CAP-030 documenting divergence and providing the authoritative 15-field set (`service.name`, `service.namespace`, `service.instance.id`, `service.version`, `deployment.environment.name`, `host.name`, `host.id`, `os.type`, `process.pid`, `vcs.repository.url.full`, `vcs.repository.name`, `vcs.provider.name`, `vcs.owner.name`, `worktree.id`, `schema_url`). |
| v1.1 | 2026-05-06 | D-314 F-1/F-2 fix. Authored CAP-029 (P0 — single-stream FileSink; ADR-015 D-15.1) and CAP-030 (P1 — OTel resource enrichment; ADR-015 D-15.2). Marked CAP-003 REWRITTEN per ADR-015 D-15.1 (original description preserved per POLICY 1 append-only). Marked CAP-023 and CAP-024 SUPERSEDED per ADR-015 D-15.1 (original descriptions preserved per POLICY 1 append-only). |
| v1.0 | 2026-04-25 | Initial authoring from domain spec crystallization (Phase 1.3). 28 capabilities (CAP-001–CAP-028). |
