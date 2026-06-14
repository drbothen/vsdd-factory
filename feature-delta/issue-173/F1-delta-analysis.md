---
document_type: feature-delta-analysis
feature: issue-173
phase: F1
title: "Wave-boundary checkpoint+reset and lossless intra-wave compaction (PreCompact flush + WASM gates)"
issue: "#173"
producer: architect
timestamp: 2026-06-13T00:00:00Z
status: draft
research_file: .factory/research/issues/issue-173.md
composes_with: ["#171", "#170"]
---

# F1 Delta Analysis — Issue #173
## Wave-boundary checkpoint+reset and lossless intra-wave compaction

---

## 0. Prerequisite Research Disposition

The research file at `.factory/research/issues/issue-173.md` resolves the issue's central open
question before this F1 analysis begins. Key finding:

**PreCompact CAN block compaction** (exit 2 or `decision: block`) — confirmed against live
`code.claude.com/docs/en/hooks` (2026-06-09). This upgrades the PreCompact flush from
"best-effort" to a **hard guarantee**. It also revealed a previously-unmentioned `PostCompact`
event (fires after compaction, cannot block) that is the correct home for post-compaction
re-hydration, a piece the issue missed.

All four open questions in the issue body (§5 below) are now partially or fully answered.

---

## 1. Impact Boundary

### 1.1 Subsystems Touched

| Subsystem | Impact | Rationale |
|-----------|--------|-----------|
| SS-04 Hook Dispatcher / Plugin Fleet | **NEW + MODIFY** | New PreCompact and PostCompact event registrations require dispatcher support. New WASM crates for handoff-completeness gate and delegation guard. Existing WASM crates (handoff-validator, regression-gate, validate-stable-anchors) are reuse/extend candidates. |
| SS-05 State Manager / STATE.md schema | **MODIFY** | HANDOFF.md schema (or STATE.md extension) for verified wave handoff. `state-burst` skill extended to run in a context the PreCompact hook invokes. Wave-reset skill is new (SS-05 scope). |
| SS-06 Skills / Workflow Layer | **MODIFY** | New `wave-reset` skill (or extend `recover-state`). Extend `wave-gate` to block wave-close until handoff-completeness gate passes. Extend `compact-state` SKILL.md to disambiguate it from context compaction. Extend `check-state-health` to report on handoff completeness and PreCompact hook wiring. |
| SS-07 Hooks Scripts / Shell Layer | **NEW** | `precompact-flush.sh` — effectful shell hook on PreCompact event. `postcompact-reanchor.sh` (advisory) on PostCompact event. Both follow the `check-factory-commit.sh` / `factory-branch-guard.sh` pattern. |
| SS-01 Dispatcher Binary | **POTENTIALLY MODIFY** | If the current dispatcher binary does not emit PreCompact/PostCompact events, it must be updated. This is the hardest dependency (see §5a). A dispatcher binary change requires a release to propagate to the operator-level cache. |
| SS-08 Documentation | **MODIFY** | Terminology disambiguation: "context compaction" vs "state compaction". At minimum: compact-state skill header, factory-health skill output, STATE.md template, and the issue's acceptance criterion about docs disambiguation. |

### 1.2 ADRs Touched

| ADR | Status | Touch Type |
|-----|--------|------------|
| ADR-025 (single-writer factory lock/lease) | Active, v1.6 | Compose — the PreCompact flush must renew the factory lock as part of its flush payload. The handoff-completeness gate and wave-reset interact with the lock-held-check. No ADR-025 amendment needed at F1; assess again in F3. |
| ADR-019 (plugin async semantics) | Active | Compose — PreCompact hooks must be `async: false` in the registry (blocking semantics require sync-group). Existing tier classification applies. |
| **NEW ADR-026** | Proposed | Wave-boundary checkpoint/reset + PreCompact integration. Records the architecture decision to use PreCompact blocking as the intra-wave compaction guarantee, hard session reset as the primary cross-wave mechanism, and `wave-state.yaml` curated manifest as the rehydration vehicle. See §2 for full proposed scope. |

### 1.3 Stories / Epics Touched

| Artifact | Touch Type | Notes |
|----------|------------|-------|
| E-17 (factory-state-durability-concurrency) | Composition anchor | E-17 is the parent story-chain for #170. Issue #173 composes with E-17 (specifically S-17.01 CAS push and S-17.02 lock guard are prerequisites for the PreCompact flush). Does NOT extend E-17 — see §1.4 for epic decision. |
| S-17.04 (heartbeat renewal) | Dependency | The PreCompact flush must invoke the same lock-renewal logic as S-17.04's SKILL renew step before committing state. At F3 story-writer must add `depends_on: [S-17.04]` to wave-1 of this feature's stories. |
| STORY-INDEX | Modify | New E-18 epic row + 5–7 new story rows. |
| ARCH-INDEX | Modify | ADR-026 registration. Subsystem registry unchanged (no new subsystems). |

### 1.4 OUT OF SCOPE

The following are explicitly excluded from this feature:

- Changing the factory lock/lease mechanism (E-17/ADR-025 scope). PreCompact flush USES the lock; does not redesign it.
- Implementing the deferred-item revalidation gate from #171. Issue #171 is a compose-with relationship (deferred process-gaps belong in the wave handoff payload) but is its own feature with a separate F1.
- RAG over the spec corpus (see §5d — curated `wave-state.yaml` manifest is the chosen rehydration vehicle; RAG is deferred unless the manifest approach is found insufficient in F3).
- `custom_instructions` retention-pinning on auto-compaction. Research confirmed this is unreliable on auto-triggered events. Explicitly not a load-bearing path.
- Changing the bats test runner or Cargo workspace structure (dispatcher binary update is gated on §5a investigation).
- Per-AC demo evidence for existing stories (demo-recorder scope, not this feature).

---

## 2. New Artifact List

### 2.1 Proposed ADR: ADR-026

**Title:** Wave-boundary checkpoint/reset and PreCompact integration

**Scope:** 6–8 decisions:
1. PreCompact shell flush as the intra-wave compaction guarantee (blocking exit 2).
2. PostCompact advisory re-anchor (cannot block; re-reads STATE.md pointer).
3. Hard session reset as the primary cross-wave mechanism (NOT continuous compaction).
4. `wave-state.yaml` curated manifest as the scoped-rehydration vehicle (NOT RAG).
5. `HANDOFF.md` on `factory-artifacts` as the verified wave-close checkpoint artifact (schema, required fields, verification rules, anti-fabrication cross-check against git/tests).
6. WASM for the checkpoint-completeness gate (deterministic, parse-heavy, tamper-resistant); shell for the flush (effectful, git/FS).
7. Proactive threshold value (human decision — see §5c; recommend 70%).
8. `on_error = continue` for PreCompact flush (fail-open; if flush crashes, compaction proceeds rather than wedging the session — correct because the flush's goal is durability, not blocking an unsafe compaction).

**Depends on:** Pre-investigation of §5a (dispatcher binary support for PreCompact/PostCompact). ADR-026 cannot be finalized until that question is answered. The ADR must pin the minimum harness version that emits these events.

### 2.2 New WASM Gates (New Crates)

| Artifact | Crate Path | Class | Event | Tool Matcher |
|----------|-----------|-------|-------|--------------|
| `validate-wave-handoff-completeness.wasm` | `crates/hook-plugins/validate-wave-handoff-completeness/` | Blocks wave-close (Wave-gate skill's Gate 1 step) via advisory block-mode | PostToolUse on Write/Edit to `HANDOFF.md` OR invoked by wave-gate skill check | — |
| `validate-heavy-op-delegation.wasm` | `crates/hook-plugins/validate-heavy-op-delegation/` | PreToolUse nudge/block for heavy ops that should delegate to sub-agents | PreToolUse | `Bash` (heuristic: command pattern matches known heavy ops — cargo test --release, grep -r, large find) |

NOTE: `handoff-validator.wasm` (currently on `SubagentStop`) is a reuse candidate for
validating sub-agent handoff outputs. The new `validate-wave-handoff-completeness.wasm`
targets a different trigger surface (the wave-close checkpoint artifact, not sub-agent
stop events). Assess whether they can share a crate in F3 (may be separate concerns).

### 2.3 New Shell Hooks

| Artifact | Path | Event | Priority | on_error |
|----------|------|-------|----------|----------|
| `precompact-flush.sh` | `plugins/vsdd-factory/hooks/precompact-flush.sh` | PreCompact | TBD (highest in PreCompact tier, first and only entry) | continue (see ADR-026 Decision 8) |
| `postcompact-reanchor.sh` | `plugins/vsdd-factory/hooks/postcompact-reanchor.sh` | PostCompact | TBD | continue (PostCompact cannot block anyway) |

Both follow the `check-factory-commit.sh` family pattern: bash with `set -euo pipefail`,
hermetic (reads STATE.md + git only, no in-context reasoning), invoke `state-burst` logic
directly (not via sub-agent).

### 2.4 New Skills

| Skill | Action | Replaces / Extends |
|-------|--------|-------------------|
| `wave-reset` | Cross-wave reset workflow: (1) verify handoff-completeness gate is CLEAN, (2) recommend/trigger session clear, (3) re-hydrate from `wave-state.yaml` manifest | New skill under SS-06. Extends `recover-state`'s rehydration logic but is wave-aware. |
| `wave-handoff` | Write the verified wave-close `HANDOFF.md` — structured fields, git/test cross-check, process-gap carry-forward | New skill under SS-06. Complementary to `wave-gate`. |

### 2.5 New Schema: HANDOFF.md

Required fields (to be formally specified in ADR-026):
- `wave_id` — closing wave number
- `last_verified_develop_sha` — cross-checked against `git rev-parse origin/develop`
- `active_bcs` — list of BC IDs that are in-scope for the next wave
- `next_wave_stories` — story IDs with status
- `open_decisions` — each citing a commit hash, test ID, or file path (not memory)
- `pending_fixes` — open findings not yet closed (anti-fabrication: each cites a PR or issue ref)
- `process_gaps` — carry-forward from #171 mechanism
- `precompact_flush_sha` — last SHA written by `precompact-flush.sh` (thread of continuity)

### 2.6 Doc Disambiguation Targets

| Document | What to Change |
|----------|---------------|
| `plugins/vsdd-factory/skills/compact-state/SKILL.md` | Add header callout: "This is the *state* compaction skill — it slims `STATE.md` as a *file* operation. It does NOT interact with the harness context window. For context-window compaction integration, see the `PreCompact` hook." |
| `plugins/vsdd-factory/skills/check-state-health/SKILL.md` | Add diagnostic row for `HANDOFF.md` completeness status and PreCompact hook registration status. |
| `CLAUDE.md` (this repo) | Add terminology clarification box in "Factory Hook Diagnostics" section: "Context compaction" (harness window) vs "State compaction" (compact-state file op) vs "PreCompact flush" (the new hook bridging them). |
| `plugins/vsdd-factory/hooks-registry.toml` | Add comment block above new `[[hooks]]` PreCompact entries explaining the event taxonomy. |

---

## 3. Affected Stories — Provisional Decomposition

This sketch is for sizing only. Story-writer produces the actual stories in F3.

### Epic Recommendation: NEW E-18

**Reason:** E-17's declared scope is "single-writer factory lock/lease — prevent concurrent
session races" (CAP-031 only). Issue #173 is a different capability class: context-durability
and wave-boundary checkpoint/reset (a new CAP, likely CAP-032). E-17 has 4 stories and is
tracking toward completion (S-17.01/02/03 all MERGED; S-17.04 draft). Extending E-17 with
5–7 new stories from a different domain would break the epic's stated scope and its
PRD capability traceability. **E-18 is the correct next free ID per POLICY 1.**

Sub-systems affected: SS-04, SS-05, SS-06, SS-07, and potentially SS-01.

### 3.1 Story Sketch

| Provisional ID | Title | Part | Subsystem(s) | Wave | Est. Points |
|----------------|-------|------|--------------|------|-------------|
| S-18.01 | ADR-026 + HANDOFF.md schema + wave-handoff skill | A | SS-05, SS-06 | 1 | 5 |
| S-18.02 | validate-wave-handoff-completeness WASM gate crate + registry | A | SS-04 | 2 | 8 |
| S-18.03 | wave-reset skill + wave-state.yaml scoped rehydration | A | SS-05, SS-06 | 3 | 8 |
| S-18.04 | precompact-flush.sh shell hook + registry (blocking PreCompact) | B | SS-07, SS-04 | 2 | 5 |
| S-18.05 | postcompact-reanchor.sh advisory hook (PostCompact re-anchor) | B | SS-07, SS-04 | 3 | 3 |
| S-18.06 | validate-heavy-op-delegation WASM gate + output-size nudge | C | SS-04 | 3 | 5 |
| S-18.07 | Terminology disambiguation + docs update | A/B/C | SS-08 | 4 | 2 |

**Estimated total: 36 points, 7 stories, 4 waves.**

**Dependency chain:**
```
S-18.01 (schema + ADR) --> S-18.02 (handoff WASM gate)
                       ╘-> S-18.04 (PreCompact flush shell)
S-18.02 + S-18.04  --> S-18.03 (wave-reset skill)
S-18.03            --> S-18.05 (PostCompact re-anchor)
S-18.03            --> S-18.06 (delegation guard)
S-18.06            --> S-18.07 (docs/disambiguation)
```

Depends on E-17: `S-18.04` (precompact-flush.sh) must invoke lock-renewal logic from
`S-17.04`. `S-18.01` schema must reference `factory_lock` frontmatter from `S-17.01`.
Add `depends_on: [S-17.04]` to S-18.04 at story-writer authoring time.

**NOTE on wave gating:** If §5a reveals the dispatcher binary does NOT yet emit
PreCompact/PostCompact, a pre-story "dispatcher binary update" may need to precede S-18.04
and S-18.05. This could add 1 story (SS-01, ~5pts) and bump all downstream waves by 1.
Flag this as a conditional story; the F1 estimate assumes the harness already emits these
events (as the live docs indicate) and only the PLUGIN registration is absent.

---

## 4. Regression Risk

### 4.1 Existing Hook Chain

| Risk | Severity | Mitigation |
|------|----------|-----------|
| PreCompact flush deadlock: `precompact-flush.sh` blocks compaction but flush itself needs in-context reasoning (e.g., "what wave is this?") | HIGH | Keep the flush hermetic — read only from `STATE.md` + git, never from in-context state. The flush determines wave context from STATE.md frontmatter `current_wave:` field, not from the LLM's active context. |
| Blocking PreCompact on crash (`on_error = block`) would wedge the session permanently if the shell hook crashes | HIGH | Use `on_error = continue`. If the flush crashes, compaction proceeds unblocked. Durability is best-effort on crash; wedge is never correct. |
| `validate-wave-handoff-completeness` blocks wave-close prematurely on single-wave / short runs where no HANDOFF.md exists | MEDIUM | Gate must be a no-op (Continue) when `HANDOFF.md` does not exist and the wave count is 1. Gate only fires when transitioning between waves (wave N > 1 or `HANDOFF.md` already exists on factory-artifacts). |
| WASM fuel budget exhaustion on `validate-wave-handoff-completeness` if HANDOFF.md grows large | MEDIUM | Apply the same per-field extract strategy as existing WASM gates (parse frontmatter fields, not the full body). Cap HANDOFF.md body at 200 lines (similar to STATE.md soft limit). |
| Single-commit-per-burst discipline (TD-VSDD-053): `precompact-flush.sh` fires as a shell hook mid-burst and commits to `factory-artifacts`, potentially creating a second commit in the same burst | MEDIUM | The flush commit must be a separate lifecycle (PreCompact fires between LLM turns, not inside a burst). Validate that the flush does not create a second `.factory/` commit in the same burst-log entry. Bats test required: fire PreCompact → flush commits → confirm subsequent state-burst is still the "single commit" of the burst. |
| `validate-heavy-op-delegation` WASM gate false-positives blocking legitimate Bash commands | MEDIUM | Start with advisory mode (emit finding to stderr, do not block). Promote to blocking only after calibration in F3 adversarial review. |
| Priority conflicts in the PreCompact event tier: two new hooks (precompact-flush.sh, postcompact-reanchor.sh) both register at a new event tier with no existing priority assignment | LOW | First-in-tier means priority ordering is unconstrained; assign reasonable values (precompact-flush priority=100, postcompact-reanchor priority=100 — only one entry each). Document in hooks-registry.toml. |
| CI WASM floor-count gate inflation: two new WASM crates added under `crates/hook-plugins/` raise the expected floor-count | LOW | Standard procedure — update the CI floor-count check at the same commit that adds the new crates. `validate-wave-handoff-completeness` and `validate-heavy-op-delegation` are both `[[bin]]`-bearing crates; they count. `factory-lock-parse` is lib-only and does NOT count per ADR-025 v1.6 precedent. |

### 4.2 Dispatcher Fuel Budgets

The new WASM gates add to the total plugin fleet. Current fleet: 34 PostToolUse + 19 PreToolUse
+ misc = ~66 event registrations. Adding 2 new WASM registrations is within normal growth bounds.
PreCompact and PostCompact are new event tiers with zero existing entries — no priority collisions.
Fuel budgets: set `timeout_ms = 5000` consistent with existing WASM gates.

### 4.3 Single-Commit-Per-Burst Discipline

The PreCompact hook fires on a harness-internal event, not on an LLM tool call. It is NOT
a state-manager burst. The `state-burst` invoked by `precompact-flush.sh` commits to
`factory-artifacts`, but this commit is a separate lifecycle (context-management event, not
a pipeline burst). The TD-VSDD-053 "single-commit-per-burst" constraint applies to
state-manager bursts (the 5-commit A/B/C/D/E sequence). Clarify this boundary in ADR-026
and in `precompact-flush.sh` comments to prevent a future adversary from flagging the
PreCompact commit as a TD-VSDD-053 violation.

### 4.4 Release Requirement

The new `[[hooks]] event = "PreCompact"` and `event = "PostCompact"` entries in
`hooks-registry.toml` only become active in the operator-level cache after a release cut.
Develop-branch edits do not affect the cached plugin (per CLAUDE.md self-referential note).
E-18 stories must be sequenced to ship in a single rc cut — partial shipping (e.g., the
shell hooks without the WASM gates) would create a regression where the flush fires but the
wave-close gate does not block.

---

## 5. Critical Dependency / Open-Question Register

These are genuine human or architecture decisions. Do NOT silently resolve.

### OQ-18-001 — Dispatcher binary support for PreCompact/PostCompact (GATING)

**Question:** Does the current factory-dispatcher binary (the Rust binary in `crates/`) already
emit `PreCompact` and `PostCompact` events to plugins? The Claude Code harness emits these
events; the question is whether the vsdd-factory dispatcher's plugin invocation layer passes
them through to WASM/shell plugins, or whether a dispatcher binary update is required first.

**Why it blocks:** S-18.04 (precompact-flush.sh) and S-18.05 (postcompact-reanchor.sh) are
entirely blocked if the dispatcher does not pass these events. A dispatcher binary update
requires a source change in `crates/`, a new rc cut, and a release before the operator-level
cache picks it up.

**Recommended resolution path:** Before story-writer authors S-18 stories in F3, architect
or devops-engineer inspects `crates/factory-dispatcher/src/invoke.rs` (or equivalent dispatch
routing) for PreCompact/PostCompact event handling. If absent, add a pre-story S-18.00 to
the E-18 wave-1 scope.

**Research note (issue-173.md):** The live Claude Code docs confirm the harness emits PreCompact
and PostCompact. The gap question is specific to the vsdd-factory dispatcher routing layer.

**ESCALATION REQUIRED:** Human decision on whether to immediately inspect the dispatcher source
or treat this as a conditional story with a flag. Do not silently assume the dispatcher already
handles these events.

---

### OQ-18-002 — Auto wave-boundary reset vs prompt-the-human

**Question:** Should the wave-boundary session reset be automatic (the `wave-reset` skill or
orchestrator triggers `/clear` or a session reset programmatically) or should it prompt the
human ("wave N is closed and checkpointed — please clear the session and start wave N+1")?

**Tradeoffs:**

| Mechanism | Pro | Con |
|-----------|-----|-----|
| Auto-reset (orchestrator self-clears) | Zero manual step; enforced by gate chain | Orchestrator clearing its own context is a destructive, irreversible action. If the handoff is incomplete or wrong, the in-session state is lost. Risk of false-positive resets. |
| Prompt-the-human | Human verifies the HANDOFF.md looks correct before clearing; recoverable if wrong | Adds manual friction at every wave boundary; humans may skip it under pressure. |
| Hybrid (prompt with auto-fallback after N minutes) | Best of both | More complex; adds a timer/cron dependency. |

**Research consensus:** Favors prompt-the-human for the initial implementation (lower risk;
matches the factory's "human in the loop" model for destructive actions per CLAUDE.md git
safety rules). Auto-reset is a Phase 2 optimization after the checkpoint/handoff mechanism
is validated.

**HUMAN DECISION REQUIRED.**

---

### OQ-18-003 — Proactive compaction threshold value and per-autonomy-level configurability

**Question:** What threshold (50% vs 70% of effective context window) should trigger a proactive
state-burst, and should this be configurable per autonomy level (fully-autonomous runs vs
human-in-the-loop)?

**Research finding (issue-173.md):** Research consensus is **70%** of effective window capacity
(not absolute token limit), not 50%. Reasoning: 50% triggers too early and adds overhead on
short runs; 70% leaves enough headroom for the flush to complete without racing the auto-compact.
The proactive threshold is a settings/config value (not a hook), consistent with the issue's
own framing.

**Recommendation:** Default to 70%, expose as a `precompact_threshold_pct` key in the factory
settings (e.g., `.claude/settings.json` or `STATE.md` frontmatter). Per-autonomy-level
configurability is a future optimization — v1 uses a single global default.

**HUMAN DECISION REQUIRED:** Confirm 70% default or override. Configurability scope
(global-only vs per-autonomy-level) is a human call; architect recommends global-only for v1.

---

### OQ-18-004 — Scoped rehydration: curated wave-state.yaml manifest vs RAG over spec corpus

**Question:** After a wave-boundary reset, how does the next session know which specs/stories
to load? Options:
- (A) **Curated `wave-state.yaml` manifest** — the `wave-handoff` skill explicitly lists the
  next wave's stories + their spec dependencies in `wave-state.yaml` (already exists as a
  concept in the factory). Deterministic, auditable, no external infrastructure.
- (B) **RAG over the spec corpus** — semantic retrieval of relevant specs on session start.
  Flexible but requires infrastructure, introduces non-determinism, and is harder to audit.

**Architect recommendation:** Option A (curated manifest) for v1. Rationale:
- `wave-state.yaml` already exists in the factory's vocabulary (`wave-status` skill).
- Deterministic rehydration matches the factory's verification-grade requirement — RAG
  introduces the same hallucination risk that wave-boundary resets are designed to eliminate.
- The spec corpus is structured (BC-INDEX, STORY-INDEX, wave assignment in story frontmatter)
  — a curated manifest can be generated mechanically from the existing index structure.
- RAG remains a Phase 2 option if the manifest approach proves too rigid for large epics.

**HUMAN DECISION REQUIRED:** Confirm Option A or override. If Option B (RAG) is chosen, a
new F1 research spike is needed to select and validate the retrieval infrastructure.

---

## 6. Terminology Collision — Disambiguation Plan

The issue correctly identifies this collision. Current state (verified):

| Term | Current Meaning | Correct Canonical Term |
|------|----------------|------------------------|
| "context compaction" | Harness summarizing/clearing the LLM context window (auto or manual /compact) | "Context compaction" — keep this term, make it explicit |
| "state compaction" | `compact-state` skill slimming STATE.md to <200 lines (file operation) | "State compaction" or "STATE.md compaction" — keep, but scope more carefully |
| "PreCompact flush" | (New) The new `precompact-flush.sh` hook that fires before context compaction | "PreCompact flush" — new term, introduce clearly |
| "wave-boundary reset" | (New) Hard session clear at each wave close | "Wave-boundary reset" — new term |

The three disambiguation targets listed in §2.6 are the minimum doc changes. S-18.07 is the
story that lands them.

---

## 7. Epic Decision Summary

**Recommendation: New E-18 (not extend E-17).**

| Criterion | Assessment |
|-----------|------------|
| E-17 scope match | E-17 is "single-writer factory lock/lease" (CAP-031). Issue #173 is "context-durability and wave-boundary reset" (candidate CAP-032). Different capability class. |
| E-17 stories remaining | S-17.04 is the only open story (draft, wave 4). E-17 is nearly done. Adding 7 more stories breaks the epic's stated story_count and PRD capability traceability. |
| Subsystem overlap | E-17 spans SS-04/SS-05/SS-06. E-18 spans SS-04/SS-05/SS-06/SS-07 (and potentially SS-01). Overlap is real but not grounds for combining (same subsystems serve different capabilities). |
| POLICY 1 compliance | E-18 is the next free ID. STORY-INDEX confirms no E-18 row exists. Append-only numbering satisfied. |
| Compose relationship | E-18 depends on E-17 (S-17.04 heartbeat renewal), expressed as story-level `depends_on` in S-18.04. Epic-level dependency is documented in §3. |

---

## 8. Rough Estimate

| Stories | Points | Waves |
|---------|--------|-------|
| 7 (S-18.01..S-18.07) | 36 pts | 4 |
| +1 conditional (S-18.00 dispatcher binary update) | +5 pts | Prepend to wave 1 |
| **Total (conditional)** | **36–41 pts** | **4–5 waves** |

Comparable to E-17 (26 pts, 4 stories) but with broader scope (new event tier + wave lifecycle
redesign). S-18.01 (ADR authoring + schema) is the critical path for all downstream stories.

---

## 9. Document Map

| Artifact | Path |
|----------|------|
| This delta analysis | `.factory/feature-delta/issue-173/F1-delta-analysis.md` |
| Issue research (prerequisite) | `.factory/research/issues/issue-173.md` |
| Issue research #171 | `.factory/research/issues/issue-171.md` |
| E-17 epic (dependency anchor) | `.factory/stories/epics/E-17-factory-state-durability-concurrency.md` |
| ADR-025 v1.6 (dependency) | `.factory/specs/architecture/decisions/ADR-025-single-writer-factory-locklease-prevent-concurrent-session-races-on-factory-artifacts-orphan-branch.md` |
| ADR-019 (async semantics, compose) | `.factory/specs/architecture/decisions/ADR-019-plugin-async-semantics-at-registry-layer.md` |
| hooks-registry.toml (impact) | `plugins/vsdd-factory/hooks-registry.toml` |
| ARCH-INDEX.md (subsystem registry) | `.factory/specs/architecture/ARCH-INDEX.md` |
| STORY-INDEX.md | `.factory/stories/STORY-INDEX.md` |

---

*Produced by architect agent, F1 phase, feature-mode. Story-writer produces actual stories in F3 after OQ-18-001 through OQ-18-004 are resolved. Do NOT commit this artifact — state-manager owns .factory/ commits.*
