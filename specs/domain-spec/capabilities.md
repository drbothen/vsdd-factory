---
document_type: domain-spec-section
level: L2
section: capabilities
version: "1.10"
status: accepted
producer: business-analyst
timestamp: 2026-04-25T00:00:00
last_amended: 2026-07-19
phase: 1.3
inputs:
  - .factory/phase-0-ingestion/pass-2-domain-model.md
  - .factory/phase-0-ingestion/pass-8-final-synthesis.md
  - .factory/legacy-design-docs/2026-04-24-v1.0-factory-plugin-kit-design.md
  - .factory/specs/architecture/ARCH-INDEX.md
input-hash: "85c749e"
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

## CHANGELOG

| Version | Date | Change |
|---------|------|--------|
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
