---
document_type: session-resume-checkpoint
cycle: v1.0-feature-engine-discipline-pass-1
created: 2026-05-07
producer: state-manager
purpose: enable fresh-session orchestrator to resume without re-reading conversation
decision: D-377
---

# Session Resume Checkpoint — v1.0-feature-engine-discipline-pass-1

## TL;DR (Read This First)

This cycle started as a 3-story engine discipline pass (path governance + per-story
adversarial convergence loop). After those 3 stories merged and F5 pass-2 surfaced
F-P2-001 (convergence hook operationally inert in production despite B3 fix), the
user authorized a mid-cycle scope expansion to build a full WASM-plugin Context
Resolver platform (D-361). That platform is the γ-generic architecture: a
factory-agnostic dispatcher-layer that injects runtime context into hooks via
sandboxed WASM resolvers. It adds 6 more stories under E-12 (S-12.03..S-12.08)
and will be the vehicle that closes F-P2-001 when S-12.08 merges.

**Where we are right now (as of this checkpoint):**
- 4 of 9 total cycle stories merged (S-13.01, S-12.01, S-12.02, S-12.06)
- 2 stories are in-flight in local worktrees with Step 4 complete but NOT yet pushed
  (S-12.03 at `7f37f5a`, S-12.05 at `bbc936e`)
- 3 stories not started (S-12.04, S-12.07, S-12.08)
- F5 pass-2 CRITICAL (15 findings); fix burst BLOCKED until S-12.08 closes F-P2-001
- F5 pass-2 architect-triage not yet authored; will happen after platform delivery
- A NEW cycle (v1.0-feature-plugin-async-semantics-pass-1) was opened at F1
  COMPLETE awaiting human review gate — this checkpoint is for the PAUSED
  engine-discipline-pass-1 cycle only
- Factory-artifacts HEAD: `55df4bc` (D-376 burst)
- develop HEAD: `15432c6` (PR #105, S-12.06)

**Next action for fresh session:** push S-12.03 + S-12.05 branches, dispatch Step 4.5
per-story adversary convergence for both in parallel.

---

## Section 1: Cycle One-Line Summary

Cycle `v1.0-feature-engine-discipline-pass-1` closed two engine governance gaps
in its original scope (artifact path governance and the per-story adversarial
convergence loop), then was expanded mid-cycle when F5 pass-2 found F-P2-001: the
convergence WASM hook is inert in production because no mechanism populates
`plugin_config.stories` at dispatch time. The user authorized the γ-generic
architecture (D-361): a factory-agnostic WASM-plugin Context Resolver platform
added to the dispatcher layer, with the first concrete resolver
(`WaveContextResolver` in a new `vsdd-context-resolvers` crate) consuming
`wave-state.yaml` to deliver the missing context. The cycle now includes 6
additional stories (S-12.03..S-12.08) under existing E-12, the first of which
(S-12.06, HOST_ABI docs) has merged, and the remaining 5 are in various stages
of delivery. F5 pass-2 adversarial review (15 findings, CRITICAL) is suspended
until S-12.08 merges and closes F-P2-001, after which the pass-2 fix burst resumes,
followed by passes-3+ to convergence (3 consecutive NITPICK_ONLY required).

---

## Section 2: Cumulative Cycle State (Numbers)

### Decision Count

| Range | Count | Location |
|-------|-------|----------|
| D-336 to D-377 | 42 decisions | `.factory/cycles/v1.0-feature-engine-discipline-pass-1/decision-log.md` |

D-377 is this checkpoint document itself (allocated in this burst).

### Story Count

| Group | Stories | Status |
|-------|---------|--------|
| Original cycle (Cluster A + B) | S-13.01, S-12.01, S-12.02 | All MERGED |
| F5 fix burst stories (E-14 process-gap) | S-14.01..S-14.05 | factory-artifacts committed only (B6 burst, D-359); not source PRs |
| Mid-cycle amendment platform | S-12.03, S-12.04, S-12.05, S-12.06, S-12.07, S-12.08 | 1 MERGED (S-12.06), 2 in-flight local (S-12.03, S-12.05), 3 not started |

### Behavioral Contracts Added This Cycle

| Phase | BCs Added | IDs |
|-------|-----------|-----|
| F2 (original) | 6 | BC-5.39.001/002 (SS-05); BC-4.10.001/002 + BC-4.11.001 (SS-04); BC-6.22.001 (SS-06) |
| F2-amendment | 7 | BC-1.13.001 (SS-01); BC-4.12.001-005 (SS-04); plus F-P2-002 fix: BC-4.10.001 v1.1 + BC-5.39.001 v1.1 |
| **Total new BCs** | **12** | (BC-4.10.001/002 and BC-5.39.001 were amended, not new) |

### ADRs Added

| ADR | Title | Phase |
|-----|-------|-------|
| ADR-016 | Path Registry as Source of Truth | F2 original |
| ADR-017 | Per-Story Adversary Phasing | F2 original |
| ADR-018 | WASM-Plugin Context Resolvers — Design and Layering | F2-amendment |

### Verification Properties Added

| Phase | VPs Added | IDs |
|-------|-----------|-----|
| F2 (original) | 4 | VP-069 (proptest), VP-070 (kani), VP-071 (kani), VP-072 (integration/bats) |
| F2-amendment | 4 | VP-073 (load purity), VP-074 (error isolation), VP-075 (injection determinism), VP-076 (capability confinement) |
| **Total** | **8** | VP-069 to VP-076 |

### Cycle PRs Merged

| PR | Story/Fix | SHA | Date |
|----|-----------|-----|------|
| #97 | S-13.01 Path Governance Bundle | `2c97cb0` | 2026-05-07 |
| #98 | S-12.01 Per-Story Adversary Workflow | `2e9b670` | 2026-05-07 |
| #99 | S-12.02 Per-Story Adversary Convergence WASM Hook | `e2fd3d4` | 2026-05-07 |
| #100 | F5-B5 Documentation drift fixes | `f018c69` | 2026-05-07 |
| #101 | F5-B1 Critical blockers | `cbcae6f` | 2026-05-07 |
| #102 | F5-B2 Spec-Impl Alignment | `0be9eb7` | 2026-05-07 |
| #103 | F5-B3 Production code correctness | `21ca82a` | 2026-05-07 |
| #104 | F5-B4 Test quality | `1ca6dc4` | 2026-05-07 |
| #105 | S-12.06 HOST_ABI Context Injection Contract | `15432c6` | 2026-05-07 |
| **Total** | **9 PRs merged** | | |

Note: PRs #100 through #104 are the F5 pass-1 fix burst (5 batches). PR #105 is the
first platform story. B6 (E-14 process-gap stories) was committed to factory-artifacts
only (no source PR; factory artifacts only).

### Index Versions at Checkpoint

| Index | Version |
|-------|---------|
| BC-INDEX | v1.18 (total_bcs: 1,943) |
| VP-INDEX | v1.5 (total_vps: 76) |
| STORY-INDEX | v2.30 (90 stories) |
| ARCH-INDEX | v1.9 (18 ADRs) |

---

## Section 3: Major Architectural Decisions

### 3.1 D-361 Mid-Cycle Scope Expansion (2026-05-07)

User authorized mid-cycle expansion of this cycle to include a WASM-plugin Context
Resolver platform. The triggering finding was F-P2-001: the
`validate-per-story-adversary-convergence` hook is operationally inert in production
because F5 pass-1 B3 added consumer plumbing (`list_stories` reads
`plugin_config.stories`) but no producer populates that field at dispatch time.
`wave-state.yaml` is a runtime artifact; the static `hooks-registry.toml`
config is all the dispatcher knows.

The user rejected fast options and chose the maximum-ambition architectural path:
build generic infrastructure that solves data injection for ALL hooks across ALL
factory types, not just the convergence hook.

### 3.2 Architecture Selection: γ-Generic WASM-Plugin Resolvers

Four options were evaluated before the user chose γ-generic:

| Option | Description | Why Rejected |
|--------|-------------|--------------|
| α | Hook reads `wave-state.yaml` directly via `host::read_file` | Conflation; does not generalize; every future hook needs duplicate parsing logic; no data-provider isolation |
| β | Skill writes a manifest before SubagentStop fires | Side-channel coordination; orchestration-layer fix for a dispatcher-layer problem; brittle to skill dispatch ordering |
| γ-compile-time | Factory-specific resolvers compiled into the dispatcher at build time | Couples generic dispatcher binary to factory domain code; breaks factory-agnostic invariant; rebuild required to add a resolver |
| **γ-generic (CHOSEN)** | Sandboxed WASM plugins in a separate registry; dispatcher core is factory-agnostic | Generalizes across all factory types; sandboxed; per-factory resolver crates ship independently |

Key generality requirement (user-directed): the dispatcher MUST be usable by ALL
factory types (vsdd-factory, future content-factory, future PR-review-factory, etc.)
without modification. Per-factory resolvers ship in separate per-factory crates.
Dispatcher core knows nothing about wave/story/cycle/article/PR vocabulary.

### 3.3 Six OD-N Decisions Locked in ADR-018

All six open design decisions from F1-amendment were resolved and codified in ADR-018:

| OD | Question | Decision |
|----|----------|----------|
| OD-1 | Resolver lifecycle | Load at startup with mtime-cache invalidation (same pattern as `plugin_loader.rs`). Per-dispatch would add wasmtime compilation overhead to every hook invocation with `needs_context`. |
| OD-2 | Resolver registration | Separate `resolvers-registry.toml` (NOT extension of `hooks-registry.toml`). Resolvers have different lifecycle (pre-dispatch data providers vs. event handlers), different ABI, different capability profiles. Conflating would create schema ambiguity. |
| OD-3 | Resolver ABI surface | Distinct `ResolverInput` / `ResolverOutput` types, versioned as Resolver ABI v1, independent of Hook ABI v1. Resolvers have no block/continue semantics; return data, not decisions. Reusing `HookPayload` would carry irrelevant fields. |
| OD-4 | Resolver output caching | No caching initially. Resolver invocations are fast WASM calls (microseconds to low milliseconds). Cache invalidation complexity not justified without measured need. |
| OD-5 | Resolver composition | Flat resolver list; no DAG composition initially. First use case (WaveContextResolver) has no dependencies. DAG adds topological sort and cycle-detection complexity without a concrete use case. |
| OD-6 | Resolver discovery | Explicit registration in `resolvers-registry.toml` only; no auto-discovery. Auto-discovery would require naming conventions baked into dispatcher and cannot auto-infer `path_allow` capability declarations. |

### 3.4 F-P2-001 — The Critical Architectural Gap (OPEN)

F-P2-001 from adv-cycle-pass-2.md: the `validate-per-story-adversary-convergence` hook
is operationally inert in production despite pass-1 F-HIGH-3 "fix" in B3. B3 added
consumer plumbing: `RealCallbacks::list_stories` now reads
`plugin_config.stories`. But no producer populates this field. The dispatcher splices
only static `hooks-registry.toml` config into `plugin_config`; `wave-state.yaml`
(which contains the active wave's story list) is a runtime artifact not present in
the registry.

**Resolution path:** S-12.08 (the terminal platform story) will:
1. Update `hooks-registry.toml` to add `needs_context = ["wave-context"]` to the
   convergence hook entry
2. Update `list_stories` to read `plugin_config["wave_context"]["stories"]` (injected
   by WaveContextResolver via S-12.07)
3. Remove the graceful-degrade Continue path; absent context is now a hard Block

F-P2-001 formally closes when S-12.08 merges. The convergence hook will be live end-to-end
for the first time in production. This unblocks the F5 pass-2 fix burst.

### 3.5 NC-1 — Single-Segment Placeholder Semantics

Codified in BC-4.11.001 v1.1 (D-357). The `validate-artifact-path` hook treats a
path segment containing exactly one token as a literal value, not a glob pattern.
This was a user-decided semantic clarification that resolved F-HIGH-6. The spec
previously described "a sequence of segments" which was ambiguous about singleton
behavior.

### 3.6 Bootstrap Pattern Flipping Right-Side-Up

D-354 and D-367 document the bootstrap exception and its resolution:
- S-13.01, S-12.01, S-12.02 were the stories that AUTHORED Step 4.5. They cannot
  go through Step 4.5 because it did not exist during their delivery. D-354 codifies
  this as a one-time bootstrap exception. PG-2 backfill (D-359) created
  `adversary-convergence-state.json` for all three with `exception_type:
  cycle_self_introduction`.
- S-12.03 onward are the FIRST stories required to complete Step 4.5. They authored
  no part of Step 4.5. S-12.06 was the first to actually complete it (D-375,
  6 passes, decay 5→3→2→0→0→0). D-367 records this as "the engine eating its
  own cooking."

### 3.7 pr-manager Early-Exit Defect

The `pr-manager` skill has confirmed early-exit defects at Step 4 (pre-merge check)
and Step 5 (merge execution). Observed 3+ times this session. The orchestrator now
bypasses pr-manager for merge operations and drives `gh pr merge` directly via Bash.
This is PG-6, formalized as S-14.05 under E-14. Future cycles should not expect
pr-manager Steps 4 and 5 to succeed without this bypass.

---

## Section 4: Cycle Progress Map

### 4.1 Original Cycle Stories (E-12 + E-13)

| Story | Title | Epic | Status | PR | SHA |
|-------|-------|------|--------|----|-----|
| S-13.01 | Path Governance Bundle | E-13 | MERGED | #97 | `2c97cb0` |
| S-12.01 | Per-Story Adversary Workflow | E-12 | MERGED | #98 | `2e9b670` |
| S-12.02 | Per-Story Adversary Convergence WASM Hook | E-12 | MERGED | #99 | `e2fd3d4` |

All three carry D-354 bootstrap exception. PG-2 backfill convergence-state.json files
are at:
- `/Users/jmagady/Dev/vsdd-factory/.factory/cycles/v1.0-feature-engine-discipline-pass-1/S-13.01/`
- `/Users/jmagady/Dev/vsdd-factory/.factory/cycles/v1.0-feature-engine-discipline-pass-1/S-12.01/`
- `/Users/jmagady/Dev/vsdd-factory/.factory/cycles/v1.0-feature-engine-discipline-pass-1/S-12.02/`

### 4.2 F5 Pass-1 Fix Burst (B1-B5 + B6)

| Batch | Description | PR | SHA | Status |
|-------|-------------|-----|------|--------|
| B5 (docs) | Documentation drift fixes | #100 | `f018c69` | MERGED |
| B1 (critical) | Critical blockers (ADR slug, VP-071, NC-1, BC hashes) | #101 | `cbcae6f` | MERGED |
| B2 (spec-impl) | BC-4.10.002 PC3 sync, VP-070, S-13.01/S-12.02 terminology | #102 | `0be9eb7` | MERGED |
| B3 (prod code) | list_stories, EC-006 hook.warn, wave-gate identity prefix | #103 | `21ca82a` | MERGED |
| B4 (test quality) | catch_unwind cleanup, perf benchmark, bats split | #104 | `1ca6dc4` | MERGED |
| B6 (E-14 stories) | 5 process-gap stories; PG-2 backfill; factory-artifacts only | n/a | factory-artifacts committed | COMPLETE (no source PR) |

B6 was committed directly to factory-artifacts (D-359). No source code was involved;
E-14 is story specs and `adversary-convergence-state.json` bootstrap files only.

### 4.3 Mid-Cycle Amendment Spec Phases

| Phase | Description | SHA | Status |
|-------|-------------|-----|--------|
| F1-amendment | Delta analysis (this file: F1-platform-amendment-delta-analysis.md) | in factory-artifacts at D-361 | COMPLETE |
| F2-amendment | 6 new BCs + ADR-018 + 4 VPs + PRD v1.2 + F-P2-002 fix | `fc4c947` (factory-artifacts) | COMPLETE |
| F3-amendment | 6 new stories S-12.03..S-12.08 under E-12; E-12 epic v1.0→v1.1 | `d5b838a` (factory-artifacts) | COMPLETE |

Factory-artifacts SHAs above are approximate; run `git -C .factory log --oneline`
to get exact values. The current factory-artifacts HEAD is `55df4bc` (D-376).

### 4.4 Platform Delivery Stories (F4)

| Story | Title | Status | Details |
|-------|-------|--------|---------|
| **S-12.06** | HOST_ABI Context Injection Contract (doc-only) | **MERGED** PR #105 `15432c6` | First platform story. First to complete Step 4.5 (D-375; 6 passes; decay 5→3→2→0→0→0; 3 consecutive NITPICK_ONLY). Section: 393L, factory-agnostic. 5 deferred findings routed to wave-gate/phase-5. |
| **S-12.03** | ContextResolver trait + ResolverRegistry | **Step 4 DONE LOCAL — NOT PUSHED** | Branch `feature/S-12.03-context-resolver-trait` at `7f37f5a`. 4 commits above develop base. 23 resolver-specific tests green. Step 4.5 (per-story adversary convergence) PENDING. |
| **S-12.05** | hook-sdk resolver-authoring extensions | **Step 4 DONE LOCAL — NOT PUSHED** | Branch `feature/S-12.05-hook-sdk-resolver` at `bbc936e`. 5 commits above develop base. 66 tests in S-12.05-specific scope (resolver types + macro tests) green. AC-010 Some(Null) spec-gap resolved. Step 4.5 PENDING. |
| **S-12.04** | WASM resolver loading + lifecycle + error isolation | **NOT STARTED** | Depends on S-12.03 merge. Delivers WASM loading via wasmtime; mtime-cache; trap classification; error isolation. |
| **S-12.07** | vsdd-context-resolvers crate + WaveContextResolver | **NOT STARTED** | Depends on S-12.04 + S-12.05 merge. Delivers the first concrete resolver; reads `wave-state.yaml`; returns `wave_context` payload. |
| **S-12.08** | Migrate convergence hook to consume wave_context | **NOT STARTED** | Depends on S-12.07. Formally closes F-P2-001. Terminal platform story. Unblocks F5 resumption. |

### 4.5 S-12.06 Step 4.5 Convergence Detail

The adversary-convergence-state.json for S-12.06 is at:
`/Users/jmagady/Dev/vsdd-factory/.factory/cycles/v1.0-feature-engine-discipline-pass-1/S-12.06/adversary-convergence-state.json`

State: `convergence_reached: true`, `passes_clean: 3`, `last_classification: NITPICK_ONLY`

Pass trajectory:

| Pass | Classification | Within-Story Findings | Key Issues |
|------|---------------|----------------------|------------|
| 1 | LOW | 5 (4 LOW + 1 NITPICK) | bats filename drift, BC-4.12.002 INV4/OD-5 missing, PC6 undocumented, 3 undocumented HOST_ABI functions |
| 2 | LOW | 3 (2 LOW + 1 NITPICK) | HOST_ABI log naming inconsistency, event_type enumeration unspecified, name/context_key drift |
| 3 | LOW | 2 (1 LOW + 1 NITPICK) | resolver.capability_denied 3-field vs 2-field doc, dispatcher log side-effect undocumented |
| 4 | NITPICK_ONLY | 0 | passes_clean 0→1; all pass-3 fixes verified |
| 5 | NITPICK_ONLY | 0 | passes_clean 1→2; story spec line 275 typo + bats header orphan (both NITPICK, not gating) |
| 6 | NITPICK_ONLY | 0 | passes_clean 2→3; CONVERGENCE REACHED |

5 deferred findings (DEFER-1 through DEFER-5) remain; routed to wave-gate or phase-5:
- DEFER-1: BC-4.12.005 PC4 vs ADR-018 collision-order contradiction (wave-gate)
- DEFER-2: BC-4.12.003 PC2 vs BC-4.12.004 PC2 duplicate event naming (wave-gate)
- DEFER-3: Concurrency model silent on resolver invocation (phase-5)
- DEFER-4: BC-4.12.003 PC4 host log triplet vs HOST_ABI single log() API (wave-gate)
- DEFER-5: BC-4.12.001 INV3/PC4 not surfaced in HOST_ABI section (phase-5); PC5 startup log demoted into this entry

All 6 pass review files are at:
`/Users/jmagady/Dev/vsdd-factory/.factory/cycles/v1.0-feature-engine-discipline-pass-1/S-12.06/adversary-pass-[1-6].md`

### 4.6 F5 Pass-2 Status

F5 pass-2 adversarial review complete (D-360). Classification: CRITICAL. 15 findings.
Full review at:
`/Users/jmagady/Dev/vsdd-factory/.factory/cycles/v1.0-feature-engine-discipline-pass-1/adv-cycle-pass-2.md`

Pass-2 fix burst is BLOCKED. Reason: F-P2-001 cannot be fixed without the platform
(S-12.03..S-12.08 chain). The 14 other pass-2 findings will be addressed in the
post-platform fix burst. No architect-triage for pass-2 has been authored — that
triage happens after platform delivery completes (see Section 7 for full finding
list).

---

## Section 5: Worktree State

Verified at checkpoint time (2026-05-07):

```
/Users/jmagady/Dev/vsdd-factory                          f018c69  [develop]
/Users/jmagady/Dev/vsdd-factory/.factory                 55df4bc  [factory-artifacts]
/Users/jmagady/Dev/vsdd-factory/.worktrees/F5-review-p2  1ca6dc4  (detached HEAD)
/Users/jmagady/Dev/vsdd-factory/.worktrees/S-12.03       7f37f5a  [feature/S-12.03-context-resolver-trait]
/Users/jmagady/Dev/vsdd-factory/.worktrees/S-12.05       bbc936e  [feature/S-12.05-hook-sdk-resolver]
```

### Worktree Notes

**Main worktree (`/Users/jmagady/Dev/vsdd-factory`)**
- Branch: develop
- Local HEAD: `f018c69` (F5-B5 docs)
- Remote origin/develop HEAD: `15432c6` (PR #105)
- Status: STALE — behind origin/develop by 5 commits (PRs #101-#105 not pulled)
- Action required: `git fetch origin && git merge origin/develop` or `git pull`

**`.factory/` worktree**
- Branch: factory-artifacts
- HEAD: `55df4bc` (D-376 state update)
- In sync with origin/factory-artifacts (confirmed by `git branch -vv`)
- No action required

**`.worktrees/F5-review-p2`**
- Detached HEAD at `1ca6dc4` (this was the B4-fix branch; detached after B4 merged)
- This worktree may be cleaned up by the new session; it is no longer active
- Do NOT push or modify

**`.worktrees/S-12.03`**
- Branch: `feature/S-12.03-context-resolver-trait`
- HEAD: `7f37f5a` (4 local commits above develop)
- NOT PUSHED — remote branch does not exist yet
- Commits above develop base:
  - `56db60d` feat(S-12.03): add module stubs
  - `fc41499` test(S-12.03): failing tests (Red Gate)
  - `b23a999` feat(S-12.03): implement register() with duplicate detection
  - `7f37f5a` chore(S-12.03): nightly fmt + proptest-regressions snapshot
- Test status: 23 resolver-specific tests green in factory-dispatcher
  (resolver_registry_test + resolver_determinism_proptest suites)
- Step 4.5 (per-story adversary convergence): PENDING

**`.worktrees/S-12.05`**
- Branch: `feature/S-12.05-hook-sdk-resolver`
- HEAD: `bbc936e` (5 local commits above develop)
- NOT PUSHED — remote branch does not exist yet
- Commits above develop base:
  - `458e6ec` feat(S-12.05): add module stubs
  - `2c2cabd` test(S-12.05): failing tests (Red Gate)
  - `f7c6a84` feat(S-12.05): implement #[resolver] proc-macro body
  - `0e9c186` fix(S-12.05): align wrong_sig.stderr
  - `bbc936e` fix(S-12.05): AC-010 Some(Null) spec-gap resolution + clippy/fmt
- Test status: all tests in workspace green; resolver types + macro tests pass
  (11 + 5 + 14 + 13 in resolver/macro-specific suites); clippy + fmt clean
- Step 4.5 (per-story adversary convergence): PENDING

---

## Section 6: PR History (Cycle-Relevant)

| PR | Title | Branch | Status | Date |
|----|-------|--------|--------|------|
| #105 | [S-12.06] HOST_ABI Context Injection Contract | feature/S-12.06-host-abi-context-injection | MERGED | 2026-05-07 |
| #104 | [F5-B4] Test quality | fix/F5-B4-test-quality | MERGED | 2026-05-07 |
| #103 | [F5-B3] Production code correctness | fix/F5-B3-prod-code | MERGED | 2026-05-07 |
| #102 | [F5-B2] Spec-Impl Alignment | fix/F5-B2-spec-impl-alignment | MERGED | 2026-05-07 |
| #101 | [F5-B1] Fix pass-1 critical blockers | fix/F5-B1-critical-blockers | MERGED | 2026-05-07 |
| #100 | docs(F5-B5): documentation drift fixes | fix/F5-B5-doc-drift | MERGED | 2026-05-07 |
| #99 | [S-12.02] validate-per-story-adversary-convergence WASM Hook | feature/S-12.02-... | MERGED | 2026-05-07 |
| #98 | [S-12.01] Per-Story Adversary Workflow | feature/S-12.01-... | MERGED | 2026-05-07 |
| #97 | feat(S-13.01): path governance bundle | feature/S-13.01-... | MERGED | 2026-05-07 |

No open PRs at checkpoint time. Run `gh pr list --state=open` to confirm.

---

## Section 7: F5 Pass-2 Findings (Pending)

Full review at:
`/Users/jmagady/Dev/vsdd-factory/.factory/cycles/v1.0-feature-engine-discipline-pass-1/adv-cycle-pass-2.md`

**Status of pass-2 fix burst:** NOT STARTED. Blocked on S-12.08. No architect-triage
for pass-2 findings has been authored yet. The triage will happen after platform
delivery completes and S-12.08 merges.

**Finding summary (15 total):**

| ID | Severity | Description | Status |
|----|----------|-------------|--------|
| F-P2-001 | CRITICAL | Convergence hook inert in production — consumer plumbing wired but no producer populates `plugin_config.stories`. B3 fix was partial. | OPEN — closes via S-12.08 |
| F-P2-002 | CRITICAL | BC-4.10.001 + BC-5.39.001 VP-071 traceability rows still have deprecated 'advisory-block' wording (sibling-file regression of F-CRIT-3) | FIXED — applied in F2-amendment burst (D-363); BC-4.10.001 v1.1 + BC-5.39.001 v1.1 |
| F-P2-003 | HIGH | F1 line 391 still references `HookResult::Advisory` pattern (sibling-file miss of pass-1 advisory-block fix) | PENDING fix burst |
| F-P2-004 | HIGH | S-13.01 story spec types still reference `PathDecision`/`ArtifactRegistry` (sibling-file miss of pass-1 F-HIGH-9/F-HIGH-10 fix) | PENDING fix burst |
| F-P2-005 | HIGH | (see adv-cycle-pass-2.md for full detail) | PENDING fix burst |
| F-P2-006 | HIGH | (see adv-cycle-pass-2.md) | PENDING fix burst |
| F-P2-007 | HIGH | (see adv-cycle-pass-2.md) | PENDING fix burst |
| F-P2-008 | HIGH | (see adv-cycle-pass-2.md) | PENDING fix burst |
| F-P2-009 | MEDIUM | (see adv-cycle-pass-2.md) | PENDING fix burst |
| F-P2-010 | MEDIUM | (see adv-cycle-pass-2.md) | PENDING fix burst |
| F-P2-011 | MEDIUM | (see adv-cycle-pass-2.md) | PENDING fix burst |
| F-P2-012 | MEDIUM | (see adv-cycle-pass-2.md) | PENDING fix burst |
| F-P2-013 | LOW (process-gap) | (see adv-cycle-pass-2.md) | PENDING fix burst |
| F-P2-014 | LOW (process-gap) | (see adv-cycle-pass-2.md) | PENDING fix burst |
| F-P2-015 | LOW (process-gap) | (see adv-cycle-pass-2.md) | PENDING fix burst |

**Important:** F-P2-002 was pre-fixed in the F2-amendment integration burst (D-363),
reducing the active pending count from 15 to 14 (1 CRITICAL closed, 13 remaining).
When the fix burst runs post-platform, the first action should verify F-P2-002 is
confirmed closed and the remaining 13 findings are addressed.

**No F5-pass-1-fix-plan equivalent has been authored for pass-2.** The existing
fix-plan at:
`/Users/jmagady/Dev/vsdd-factory/.factory/cycles/v1.0-feature-engine-discipline-pass-1/F5-pass-1-fix-plan.md`
covers pass-1 findings only. Pass-2 architect-triage + fix-plan must be authored
after platform delivery before the pass-2 fix burst runs.

---

## Section 8: Process-Gap Findings Tracked (E-14)

Epic E-14 "Engine Discipline Pass-2" was authored in D-359. 5 stories:

| Story | Priority | Process-Gap | Description |
|-------|----------|-------------|-------------|
| S-14.01 | P0 | PG-2 | Bootstrap convergence-state backfill protocol — formalize the exception procedure used inline for S-13.01/S-12.01/S-12.02 |
| S-14.02 | P1 | PG-1 | F5 dispatch state-manager persist automation — state-manager should extract review text automatically instead of requiring manual transcript scraping |
| S-14.03 | P2 | PG-3 | Pre-F5 placeholder lint — automated lint gate to catch placeholder frontmatter strings before F5 entry (surfaced by D-355-AMEND) |
| S-14.04 | P1 | PG-4 | Policy-rubric auto-injection — adversary should receive the policy rubric automatically without orchestrator manual inclusion |
| S-14.05 | P1 | PG-6 | pr-manager Step-4/Step-5 early-exit codification — document the bypass pattern; orchestrator drives `gh pr merge` directly |

All 5 stories are in DRAFT status under E-14. They are follow-up cycles' work;
they do not block this cycle's F5/F6/F7 completion. S-14.01 P0 bootstrap protocol
was applied inline (the `adversary-convergence-state.json` files with
`bootstrap_annotation` + `exception_type: cycle_self_introduction` for S-13.01,
S-12.01, S-12.02 constitute the inline backfill).

Factory-artifacts paths:
- E-14 epic: `/Users/jmagady/Dev/vsdd-factory/.factory/stories/epics/E-14-engine-discipline-pass-2.md`
- S-14.01..S-14.05 story files: `/Users/jmagady/Dev/vsdd-factory/.factory/stories/S-14.0[1-5]-*.md`

---

## Section 9: Resume Next-Step Recipe

These are the concrete ordered actions for the fresh-session orchestrator:

### Step 0 — Orientation (5 minutes)

1. Read this document fully
2. Run `git worktree list` to verify 5 worktrees are mounted as shown in Section 5
3. Run `gh pr list --state=open` — expect 0 open PRs
4. Run `git -C /Users/jmagady/Dev/vsdd-factory/.factory log -1 --format='%h %s'`
   — should show `55df4bc chore(D-376): S-12.06 merged — Step 9 state update`
5. Run `git log origin/develop --oneline -3` — should show `15432c6` as HEAD

### Step 1 — Sync Main Worktree

The main worktree at `/Users/jmagady/Dev/vsdd-factory` is stale (local develop at
`f018c69`, remote at `15432c6`, 5 commits behind). The new session should NOT work
from the stale local develop. Either:

```bash
git -C /Users/jmagady/Dev/vsdd-factory fetch origin
git -C /Users/jmagady/Dev/vsdd-factory merge origin/develop
```

Or confirm that all work is done from worktrees (S-12.03/S-12.05 branches are already
ahead of `15432c6`; main worktree staleness does not affect those worktrees since they
track `origin/develop` as their upstream reference).

### Step 2 — Push S-12.03 and S-12.05 Branches

These branches exist only locally. Push them before dispatching Step 4.5:

```bash
git -C /Users/jmagady/Dev/vsdd-factory/.worktrees/S-12.03 push origin feature/S-12.03-context-resolver-trait
git -C /Users/jmagady/Dev/vsdd-factory/.worktrees/S-12.05 push origin feature/S-12.05-hook-sdk-resolver
```

### Step 3 — Dispatch S-12.03 Step 4.5 + S-12.05 Step 4.5 in Parallel

Both stories are at Step 4 complete. Step 4.5 is per-story adversarial convergence.
Target: 3 consecutive NITPICK_ONLY passes each.

Use the `vsdd-factory:adversarial-review` skill (or dispatch adversary agent directly)
on each story spec + implementation. Story spec paths:
- S-12.03: `/Users/jmagady/Dev/vsdd-factory/.factory/stories/S-12.03-context-resolver-trait.md`
- S-12.05: `/Users/jmagady/Dev/vsdd-factory/.factory/stories/S-12.05-hook-sdk-resolver-extensions.md`

Convergence state files initialized at:
- `/Users/jmagady/Dev/vsdd-factory/.factory/cycles/v1.0-feature-engine-discipline-pass-1/S-12.03/`
- `/Users/jmagady/Dev/vsdd-factory/.factory/cycles/v1.0-feature-engine-discipline-pass-1/S-12.05/`

After each pass, state-manager persists the review and updates the convergence-state.json.
Red-gate log files are already present in each directory from Step 2 (test authoring).

### Step 4 — Merge S-12.03 and S-12.05

Once each reaches 3 consecutive NITPICK_ONLY, proceed to:
- Step 5 demo recording (per per-story-delivery.md)
- PR creation + AI review + security review
- Merge via `gh pr merge --squash` (bypass pr-manager Steps 4/5 per PG-6)
- State-manager step-9 state update after each merge

### Step 5 — Deliver S-12.04 (after S-12.03 merges)

S-12.04 depends on S-12.03. Delivers WASM resolver loading + lifecycle + error
isolation in `crates/factory-dispatcher/src/resolver_loader.rs`. Anchors: BC-4.12.001,
BC-4.12.004, VP-073, VP-074.

Story file: `/Users/jmagady/Dev/vsdd-factory/.factory/stories/S-12.04-wasm-resolver-loading.md`
(verify filename via STORY-INDEX).

### Step 6 — Deliver S-12.07 (after S-12.04 + S-12.05 merge)

S-12.07 depends on BOTH S-12.04 (WASM loading) AND S-12.05 (hook-sdk resolver types).
Creates the `crates/vsdd-context-resolvers/` crate with `WaveContextResolver`.
Registers in `plugins/vsdd-factory/resolvers-registry.toml`. Anchors: BC-4.12.001-005,
VP-073-076.

### Step 7 — Deliver S-12.08 (after S-12.07 merges) — CRITICAL PATH TERMINUS

S-12.08 closes F-P2-001. Updates:
- `hooks-registry.toml`: adds `needs_context = ["wave-context"]` to convergence hook
- `validate-per-story-adversary-convergence/src/lib.rs`: reads `plugin_config["wave_context"]["stories"]`
- Removes graceful-degrade Continue path for absent stories

The bats end-to-end test (resolver → dispatcher → hook → correct block decision) is
the definitive F-P2-001 closure gate.

After S-12.08 merges, the convergence hook is live end-to-end in production for the
first time.

### Step 8 — Author F5 Pass-2 Architect Triage + Fix-Plan

After S-12.08 merges, dispatch the architect to triage the 14 remaining pass-2
findings (F-P2-001 is closed; F-P2-002 is already fixed). The architect produces a
fix-plan analogous to `F5-pass-1-fix-plan.md`. Save to:
`/Users/jmagady/Dev/vsdd-factory/.factory/cycles/v1.0-feature-engine-discipline-pass-1/F5-pass-2-fix-plan.md`

### Step 9 — Execute F5 Pass-2 Fix Burst

Deliver fix PRs for the 13 remaining pass-2 findings per `vsdd-factory:fix-pr-delivery`.
After all fixes merge, dispatch F5 pass-3 adversarial review. Continue until 3
consecutive NITPICK_ONLY passes.

### Step 10 — F6 Targeted Hardening

After F5 convergence, run `vsdd-factory:phase-f6-targeted-hardening` on the cycle
delta. Focus areas from the pass-1/pass-2 patterns: sibling-file propagation discipline,
BC↔HOST_ABI surface completeness, resolver capability-denied event field completeness.

### Step 11 — F7 Delta Convergence + Human Gate

Run `vsdd-factory:phase-f7-delta-convergence`. Deliver the final convergence report.
Present to user for human review gate before closing the cycle and resuming
E-10 (v1.0-brownfield-backfill pass-9).

---

## Section 10: Open Constraints and Constants

### Operational Constants at Checkpoint

| Constant | Value | Source |
|----------|-------|--------|
| develop HEAD (remote) | `15432c6` | git log origin/develop -1 |
| factory-artifacts HEAD | `55df4bc` | git -C .factory log -1 |
| main HEAD | `fb3e297` | rc.11 bot bundle; behind develop |
| BC-INDEX version | v1.18 | `.factory/specs/behavioral-contracts/BC-INDEX.md` |
| VP-INDEX version | v1.5 | `.factory/specs/verification-properties/VP-INDEX.md` |
| STORY-INDEX version | v2.30 | `.factory/stories/STORY-INDEX.md` |
| ARCH-INDEX version | v1.9 | `.factory/specs/architecture/ARCH-INDEX.md` |
| Total BCs | 1,943 | BC-INDEX |
| Total VPs | 76 | VP-INDEX |
| Total stories | 90 | STORY-INDEX |
| Total epics | 14 | STORY-INDEX |
| Last decision | D-377 | This checkpoint document |

### Known Process Constraints

**pr-manager early-exit bypass:** The `pr-manager` skill has a confirmed defect at
Steps 4 (pre-merge check) and Step 5 (merge execution). It exits early without
completing. Orchestrator must drive `gh pr merge --squash` directly via Bash.
This has been observed 3+ times and is tracked as PG-6 / S-14.05. Do NOT rely on
pr-manager for merge operations this cycle.

**Adversary agent is read-only:** The adversary agent has only Read/Grep/Glob tool
access. It cannot persist its own output. The state-manager must extract the review
text from the adversary's output (conversation or JSONL transcript) and write it to
the appropriate review file. PG-1 / S-14.02 tracks automation of this process.

**Bash hook path-prevent-deadlock pattern:** Multi-line bash compounds sent to hooks
sometimes get blocked by the Bash tool's safety sandbox. Single-command invocations
work reliably. If a compound command blocks, split into sequential single-call Bash
invocations.

**Main worktree stale:** Local develop at `f018c69`; remote at `15432c6`. Any work
starting from the main worktree should first `git pull origin develop`.

**F5 pass-2 fix burst ordering constraint:** F-P2-001 fix (S-12.08) MUST complete
before the fix burst runs. The fix burst depends on the platform being in production
to verify that F-P2-001 is truly closed (the bats end-to-end test is the gate).

**RESOLVER_ABI_VERSION vs HOST_ABI_VERSION:** These are independently versioned
constants. Adding resolver types to hook-sdk constitutes a minor version bump to
hook-sdk per BC-2.06.001. S-12.05 acceptance criteria should include the semver
bump in the hook-sdk crate's `Cargo.toml`.

**v1.0.0-rc.13 tag (remote) is INVALID:** The STATE.md session resume checkpoint
notes: "v1.0.0-rc.13 tag (remote): PINNED at `ba63c9f` — INVALID (validate fails;
user must delete: `git push origin :refs/tags/v1.0.0-rc.13`)". This does not block
cycle delivery but should be cleaned up before the next release.

**Concurrent plugin-async-semantics cycle:** A second feature cycle
(`v1.0-feature-plugin-async-semantics-pass-1`) was opened at F1 COMPLETE and is
awaiting human review gate. That cycle is entirely separate from this one. The
orchestrator should NOT mix work from the two cycles. When the fresh session starts,
confirm which cycle the user wants to work on before dispatching.

---

## Section 11: Estimated Remaining Effort

Honest estimates for the fresh session. Wall-clock times assume single-threaded
sequential agent work with no blocking waits.

| Phase | Activity | Estimate |
|-------|----------|---------|
| S-12.03 Step 4.5 | Per-story adversary convergence (3 NITPICK_ONLY × ~30 min each) | 2-4 hours |
| S-12.05 Step 4.5 | Per-story adversary convergence (parallel with S-12.03) | 2-4 hours |
| S-12.03 + S-12.05 Steps 5-9 | Demo + PR + merge + state update for each | 1-2 hours |
| S-12.04 full per-story-delivery | Steps 1-9 + Step 4.5 | 4-7 hours |
| S-12.07 full per-story-delivery | Steps 1-9 + Step 4.5 (new crate, more complex) | 4-7 hours |
| S-12.08 full per-story-delivery | Steps 1-9 + Step 4.5 (migration; smaller) | 3-5 hours |
| F5 pass-2 architect triage + fix-plan | After S-12.08 merges | 1-2 hours |
| F5 pass-2 fix burst (13 findings) | Fix PRs per vsdd-factory:fix-pr-delivery | 6-8 hours |
| F5 passes-3+ to 3 NITPICK_ONLY | Variable; pass-3+ on post-fix corpus | 4-8 hours |
| F6 targeted hardening | Focused on BC↔HOST_ABI + sibling-file gaps | 4-6 hours |
| F7 convergence + human gate | Convergence report + human review | 1-2 hours |
| State updates between phases | step-9 updates, factory-artifacts commits | 30 min total |
| **Total** | | **32-55 hours agent work** |

Note: S-12.03 and S-12.05 Step 4.5 runs can happen in parallel (they are independent
stories in separate worktrees). This can save 2-4 hours if the orchestrator dispatches
both adversary sessions simultaneously.

The cycle's defining risk to remaining effort is the WASM loading infrastructure
(S-12.04). The `resolver_loader.rs` implementation mirrors `plugin_loader.rs` but
introduces new trap-classification logic and mtime-cache patterns specific to
resolvers. Regression risk is HIGH (R-PLAT-001/R-PLAT-002 in F1-amendment Section 6).
The in-memory registry (S-12.03, already done) de-risks this by separating the trait
from the WASM loading complexity.

---

## Appendix A: Dependency Graph

```
S-12.06 (MERGED — HOST_ABI docs foundation)
  |
  +-- S-12.03 (Step 4 DONE LOCAL — ContextResolver trait + ResolverRegistry)
  |     |
  |     +-- S-12.04 (NOT STARTED — WASM loading + lifecycle)
  |           |
  +-- S-12.05 (Step 4 DONE LOCAL — hook-sdk resolver extensions)
        |     |
        |     +----> S-12.07 (NOT STARTED — vsdd-context-resolvers crate + WaveContextResolver)
        |                 |
        +--------------> S-12.08 (NOT STARTED — convergence hook migration; closes F-P2-001)
                              |
                         [F5 pass-2 fix burst resumption unblocked]
                              |
                         [F5 pass-3+ to 3 NITPICK_ONLY]
                              |
                         [F6 targeted hardening]
                              |
                         [F7 delta convergence + human gate]
                              |
                         [Cycle CLOSED]
```

## Appendix B: Spec File Quick-Reference

| Artifact | Path |
|----------|------|
| ADR-016 (path registry SoT) | `.factory/specs/architecture/decisions/ADR-016-path-registry-source-of-truth.md` |
| ADR-017 (per-story adversary phasing) | `.factory/specs/architecture/decisions/ADR-017-per-story-adversary-phasing.md` |
| ADR-018 (WASM-plugin context resolvers) | `.factory/specs/architecture/decisions/ADR-018-wasm-plugin-context-resolvers.md` |
| BC-1.13.001 (resolver-registry loading) | `.factory/specs/behavioral-contracts/ss-01/BC-1.13.001.md` |
| BC-4.10.001 (per-story adversary hook invariants) | `.factory/specs/behavioral-contracts/ss-04/BC-4.10.001.md` |
| BC-4.10.002 (PC3 log_info) | `.factory/specs/behavioral-contracts/ss-04/BC-4.10.002.md` |
| BC-4.11.001 (NC-1 single-segment semantics) | `.factory/specs/behavioral-contracts/ss-04/BC-4.11.001.md` |
| BC-4.12.001 (resolver lifecycle) | `.factory/specs/behavioral-contracts/ss-04/BC-4.12.001.md` |
| BC-4.12.002 (resolver ABI + payload schema) | `.factory/specs/behavioral-contracts/ss-04/BC-4.12.002.md` |
| BC-4.12.003 (resolver capability model) | `.factory/specs/behavioral-contracts/ss-04/BC-4.12.003.md` |
| BC-4.12.004 (resolver error + crash isolation) | `.factory/specs/behavioral-contracts/ss-04/BC-4.12.004.md` |
| BC-4.12.005 (context-injection merging contract) | `.factory/specs/behavioral-contracts/ss-04/BC-4.12.005.md` |
| BC-5.39.001 (convergence state schema) | `.factory/specs/behavioral-contracts/ss-05/BC-5.39.001.md` |
| BC-5.39.002 (convergence enforcement) | `.factory/specs/behavioral-contracts/ss-05/BC-5.39.002.md` |
| BC-6.22.001 (relocation skill + sequencing) | `.factory/specs/behavioral-contracts/ss-06/BC-6.22.001.md` |
| VP-069 (proptest — path registry) | `.factory/specs/verification-properties/VP-069.md` |
| VP-070 (kani — path matching) | `.factory/specs/verification-properties/VP-070.md` |
| VP-071 (kani — block invariant, v1.2) | `.factory/specs/verification-properties/VP-071.md` |
| VP-072 (integration/bats — path governance) | `.factory/specs/verification-properties/VP-072.md` |
| VP-073 (resolver-load purity) | `.factory/specs/verification-properties/VP-073.md` |
| VP-074 (resolver-error isolation) | `.factory/specs/verification-properties/VP-074.md` |
| VP-075 (context-injection determinism) | `.factory/specs/verification-properties/VP-075.md` |
| VP-076 (resolver-capability confinement) | `.factory/specs/verification-properties/VP-076.md` |
| E-12 epic (engine governance platform) | `.factory/stories/epics/E-12-engine-governance.md` |
| E-13 epic (artifact integrity) | `.factory/stories/epics/E-13-artifact-integrity.md` |
| E-14 epic (engine discipline pass-2) | `.factory/stories/epics/E-14-engine-discipline-pass-2.md` |
| F1 delta analysis (original) | `.factory/cycles/v1.0-feature-engine-discipline-pass-1/F1-delta-analysis.md` |
| F1 platform amendment delta | `.factory/cycles/v1.0-feature-engine-discipline-pass-1/F1-platform-amendment-delta-analysis.md` |
| F5 pass-1 fix plan | `.factory/cycles/v1.0-feature-engine-discipline-pass-1/F5-pass-1-fix-plan.md` |
| adv-cycle-pass-1.md | `.factory/cycles/v1.0-feature-engine-discipline-pass-1/adv-cycle-pass-1.md` |
| adv-cycle-pass-2.md | `.factory/cycles/v1.0-feature-engine-discipline-pass-1/adv-cycle-pass-2.md` |
| S-12.06 adversary convergence state | `.factory/cycles/v1.0-feature-engine-discipline-pass-1/S-12.06/adversary-convergence-state.json` |
