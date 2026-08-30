---
document_type: delta-analysis-report
feature_name: "vsdd-factory:wrap skill — Session Pause and Checkpoint Orchestration"
created: 2026-08-29
spec_version_at_analysis:
  BC-INDEX: "v5.21"
  ARCH-INDEX: "v3.95"
  STORY-INDEX: "(latest as of 2026-08-29)"
status: draft
intent: "feature"
feature_type: "infrastructure"
scope: "standard"
severity: "N/A"
---

# Delta Analysis Report: vsdd-factory:wrap Skill

## Feature Request

- **Brief:** Port the user-level battle-tested `~/.claude/skills/wrap/SKILL.md` into
  `plugins/vsdd-factory/skills/wrap/SKILL.md` so it ships as `vsdd-factory:wrap`.
- **Requested by:** Human (2026-08-29 direct request)
- **Source skill:** `~/.claude/skills/wrap/SKILL.md` — 7-step procedure, verified against
  this repo; footprint visible at STATE.md SESSION-WRAP-PAUSE-2026-08-28 and
  SESSION-WRAP-PAUSE-2026-08-29 entries.
- **Human-decided reconciliation:** Resume guidance in Step 7 MUST recommend
  `/vsdd-factory:rehydrate-wave` FIRST (mandatory post-clear per BC-6.24.001), THEN
  `/vsdd-factory:next-step`. The local skill currently emits only `next-step`; this
  ordering is the only behavioral change from the source.

---

## Classifications

### Intent Classification

**Classified intent:** `feature`

**Rationale:** Human says "port into the plugin", "integrate", "ships as". This is a new
artifact being introduced; nothing broken, nothing changed in existing behavior. Routes
through full F1–F7 (standard scope).

### Feature Type Classification

**Classified type:** `infrastructure`

**Rationale:** The wrap skill is factory meta-tooling — it orchestrates factory pipeline
state (pause + checkpoint + lock release). No UI, no backend service logic, no product
repository changes. Engine self-improvement in the session-lifecycle category.

### Trivial Scope Classification

| Criterion | Result | Reason |
|-----------|--------|--------|
| Impact boundary: single file/module | FAIL | 2 new files (SKILL.md + bats) + 5 modified index/spec files |
| No new BCs needed | FAIL | BC-6.28.001 required (no existing BC covers session-wrap/pause) |
| No architecture change | PASS | Skills auto-discovered; no structural change |
| No new external dependencies | PASS | No new libraries or external services |
| Regression risk: LOW | PASS | New file only; no existing code paths modified |

**Classified scope:** `standard`

**Rationale:** The "no new BCs needed" criterion fails — a new BC is required to express
testable postconditions under VSDD. Standard scope means full F1–F7 pipeline.

### Severity Classification

**Classified severity:** `N/A` — intent is `feature`, not `bug-fix`.

---

## Impact Assessment

| Dimension | Affected | Details |
|-----------|----------|---------|
| PRD Requirements | 1 new BC | BC-6.28.001 (NEW — SS-06, CAP-040) |
| Architecture | SS-06 count update; 1 new CAP | SS-06: 119→120 skills; CAP-040 appended to capabilities.md |
| UX Screens | 0 | Infrastructure skill; no UI |
| Stories | 1 new story | S-24.01 in new epic E-24 |
| Tests | 1 new bats file | `tests/wrap-skill.bats` (15 tests) |
| Verification Properties | 0 at F1 | Procedure-document skill; VP authorship is F2/F3 scope |
| Regression Baseline | 1,988 existing BCs unchanged | Wrap skill is purely additive |

---

## Subsystem and Capability Anchor

**Subsystem: SS-06 Skill Catalog.** The wrap skill lives at
`plugins/vsdd-factory/skills/wrap/SKILL.md`. SS-06's declared scope per ARCH-INDEX
Subsystem Registry is `plugins/vsdd-factory/skills/` (119 skills, BC prefix BC-6).
Module ownership: module X belongs to SS-06 because SS-06 covers this path per the
Subsystem Registry, and the module's purpose (skill delivery) matches SS-06's scope.

**Capability: NEW — CAP-040.** No existing capability covers human-initiated session-wrap.

| CAP | Why NOT this CAP |
|-----|-----------------|
| CAP-031 — single-writer factory lock/lease | Covers CAS-push acquire/release protocol (BC-6.23.001). Wrap USES this at Step 5 but is a distinct concern: orchestrated session pause vs. raw lock semantics. |
| CAP-032 — wave-boundary checkpoint and PreCompact flush | Covers ADR-026 wave-boundary and the PreCompact hook chain. Wrap DEPENDS on checkpoint outcomes but is human-triggered, not wave-boundary or auto-compaction triggered. |

Proposed CAP-040:

> **CAP-040 — Human-initiated factory session pause and resume checkpoint orchestration.** The `/vsdd-factory:wrap` skill provides the canonical 7-step sequence for safely pausing the factory pipeline: (1) halt new work, (2) verify factory health (routing to compact-state or recover-state as needed), (3) commit WIP on all in-flight story branches, (4) delegate pipeline-PAUSED STATE.md update and Session Resume Checkpoint write to state-manager (BC-6.23.001 Invariant 5 — no direct STATE.md edit), (5) release the factory lock via factory-unlock, (6) verify all durability postconditions (clean factory-artifacts tree, exactly one Session Resume Checkpoint, pipeline:PAUSED), and (7) emit a `## Factory Wrapped` report with resume instructions that cite `/vsdd-factory:rehydrate-wave` before `/vsdd-factory:next-step`. Source: BC-6.28.001; BC-6.23.001 Invariant 5; BC-6.24.001. Spans SS-06. Append-only P1 addition; CAP-039 is the prior entry.

---

## Proposed BC: BC-6.28.001

**Next available BC ID in SS-06:** Highest existing is BC-6.27.001 (pr-manager factory-side
PR protocol; CAP-037; S-21.05). BC-6.28.001 is next per POLICY 1 (append-only).

**Proposed title:**

> `/vsdd-factory:wrap` MUST halt new work, persist all in-flight changes to durable
> branches, delegate STATE.md PAUSED transition and dated Session Resume Checkpoint to
> state-manager (never editing STATE.md directly), release the factory lock, verify a
> clean factory-artifacts working tree, and emit resume guidance that names
> `/vsdd-factory:rehydrate-wave` before `/vsdd-factory:next-step`

**Postconditions:**

| PC# | Postcondition |
|-----|---------------|
| PC-1 | `pipeline:` field in STATE.md frontmatter equals `PAUSED` after wrap completes |
| PC-2 | Exactly one `## Session Resume Checkpoint` section exists in STATE.md; written by state-manager; dated today |
| PC-3 | `git -C .factory status --porcelain` returns empty (factory-artifacts working tree clean) |
| PC-4 | STATE.md written exclusively by `vsdd-factory:state-manager`; wrap skill makes zero direct Write/Edit calls on STATE.md (BC-6.23.001 Invariant 5) |
| PC-5 | Factory lock released, OR checkpoint notes that no lock was held |
| PC-6 | No product-repo or story-worktree changes exist that were intended to be saved but are not (either committed to story branches, or un-committable state documented in checkpoint) |
| PC-7 | Resume guidance cites `/vsdd-factory:rehydrate-wave` before `/vsdd-factory:next-step` (human-decided ordering) |
| PC-8 | Step 7 report includes: pipeline PAUSED at phase/step, checkpoint SHA, WIP branch list (or "none"), lock status, and "Safe to /clear" declaration |

**Invariants:**

| INV# | Invariant |
|------|-----------|
| INV-1 | Skill MUST NOT modify STATE.md directly at any step |
| INV-2 | Skill MUST NOT emit Step 7 report while any PC-1–PC-6 postcondition is unmet |
| INV-3 | Skill MUST NOT spawn new pipeline sub-agents after Step 1 |

---

## Epic + Story Placement

**Epic: NEW — E-24 Session Lifecycle Orchestration**

| Considered epic | Status | Why NOT |
|----------------|--------|---------|
| E-18 Factory Context Durability | Complete (all 13 stories merged; final story S-18.13) | Closed; built the primitives this skill uses |
| E-21 Factory State Data Loss Hardening | S-21.25 is last story; focused on worktree/PR/lock integrity | Wrong scope category |
| E-23 ADR-045 Stable-Anchor Migration | S-23.01–S-23.14; cross-reference migration only | Wrong scope entirely |

E-20 is reserved per POLICY 1 (pre-existing gap). E-22 is dissolved. E-24 is the next
free epic ID.

**Story: S-24.01** (first story in E-24; S-23.14 is the current last story in the catalog)

- **Title:** `vsdd-factory:wrap skill — session pause, checkpoint, and lock-release orchestration`
- **Estimated points:** 5
- **Priority:** P1
- **Dependencies (all satisfied):** S-18.03 (rehydrate-wave, MERGED), S-18.10 (check-state-health, MERGED), BC-6.23.001 (factory-lock, active)

---

## Skill Test Approach

### How Skills Are Tested in This Repo

| Tier | File | What It Tests |
|------|------|---------------|
| 1 — Structural | `tests/skills.bats` | Iron Law, Announce at Start, Red Flags table (discipline skills only) |
| 2 — Content | `tests/skills-content.bats` | Author-environment leaks in skill prose |
| 3 — Script | `tests/rehydrate-wave.bats`, etc. | Skills shipping a shell helper; hermetic git fixture tests |
| 4 — Document content | `tests/wrap-skill.bats` (NEW) | Grep-based structural invariants derived from BC-6.28.001 postconditions |

The wrap skill is a **pure procedure document** (no shell script). Tier 4 applies: a
dedicated `wrap-skill.bats` asserting BC-6.28.001 postconditions against the SKILL.md
content itself.

### Red Gate Approach

Per the writing-skills hard gate: `wrap-skill.bats` MUST be written before
`plugins/vsdd-factory/skills/wrap/SKILL.md` exists. Before SKILL.md exists, every test
fails on missing file. This is the structural RED gate.

### Concrete Tests (15 tests — `tests/wrap-skill.bats`)

Each maps to a BC-6.28.001 postcondition or invariant:

```text
T-01  wrap SKILL.md exists at canonical path
T-02  wrap skill frontmatter name field is 'wrap'
T-03  wrap skill Step 4 delegates to vsdd-factory:state-manager (INV-1)
T-04  wrap skill body contains no direct STATE.md Write/Edit instruction (INV-1 guard)
T-05  wrap skill instructs state-manager to set pipeline: PAUSED (PC-1)
T-06  wrap skill references Session Resume Checkpoint (PC-2)
T-07  wrap skill Step 6 checks factory-artifacts tree via --porcelain (PC-3)
T-08  wrap skill Step 5 invokes factory-unlock (PC-5)
T-09  wrap skill resume guidance mentions rehydrate-wave (PC-7)
T-10  wrap skill resume guidance mentions next-step (PC-7)
T-11  wrap skill resume guidance cites rehydrate-wave before next-step (PC-7 ordering)
T-12  wrap skill Step 7 emits 'Factory Wrapped' report header (PC-8)
T-13  wrap skill Step 7 report includes 'Safe to /clear' declaration (PC-8)
T-14  wrap skill has exactly 7 numbered steps
T-15  wrap skill emits no hardcoded 'product:' literal (skills-content parity)
```

T-11 (ordering assertion) is the non-trivial test — it cannot pass by accident because it
compares line numbers of `rehydrate-wave` and `next-step` in the file, asserting the former
appears first.

---

## Files Changed

### New Files

| File Path | Purpose |
|-----------|---------|
| `plugins/vsdd-factory/skills/wrap/SKILL.md` | Primary deliverable — the ported and reconciled 7-step skill |
| `plugins/vsdd-factory/tests/wrap-skill.bats` | 15-test Red Gate suite for BC-6.28.001 (F3 deliverable) |
| `.factory/specs/behavioral-contracts/ss-06/BC-6.28.001.md` | Behavioral contract for session-wrap (F2 deliverable) |
| `.factory/stories/epics/E-24-session-lifecycle.md` | Epic file (F3 deliverable) |
| `.factory/stories/S-24.01-wrap-skill-session-pause-checkpoint.md` | Story spec (F3 deliverable) |

### Modified Files

| File Path | Change Type | Risk |
|-----------|------------|------|
| `.factory/specs/domain-spec/capabilities.md` | Append CAP-040 entry | LOW |
| `.factory/specs/behavioral-contracts/BC-INDEX.md` | Append BC-6.28.001 row; total_bcs 1988→1989; SS-06 count 589→590 | LOW |
| `.factory/specs/architecture/ARCH-INDEX.md` | SS-06 BC count 589→590; CAP-040 in Document Map | LOW |
| `.factory/stories/STORY-INDEX.md` | Append S-24.01 row | LOW |
| `.factory/specs/architecture/SS-06-skill-catalog.md` | Skill count 119→120; note wrap skill addition | LOW |

### Dependent Files (unchanged but depend on modified files)

| File Path | Depends On | Regression Risk |
|-----------|-----------|----------------|
| `plugins/vsdd-factory/skills/check-state-health/SKILL.md` | Called by wrap Step 2 | LOW |
| `plugins/vsdd-factory/skills/compact-state/SKILL.md` | Called by wrap Step 2 (conditional) | LOW |
| `plugins/vsdd-factory/skills/recover-state/SKILL.md` | Called by wrap Step 2 (conditional) | LOW |
| `plugins/vsdd-factory/skills/factory-unlock/SKILL.md` | Called by wrap Step 5 | LOW |
| `plugins/vsdd-factory/skills/rehydrate-wave/SKILL.md` | Named in wrap Step 7 resume guidance | LOW |
| `plugins/vsdd-factory/skills/next-step/SKILL.md` | Named in wrap Step 7 resume guidance | LOW |
| `plugins/vsdd-factory/agents/state-manager.md` | Delegated to at Step 4 (BC-6.23.001 INV-5) | LOW |
| `plugins/vsdd-factory/templates/session-checkpoints-template.md` | State-manager uses for prior checkpoint archival | LOW |
| `plugins/vsdd-factory/tests/skills.bats` | Scans skills dir; will see new wrap/SKILL.md | LOW |
| `plugins/vsdd-factory/tests/skills-content.bats` | Scans all SKILL.md files for author leaks | LOW |

**plugin.json — NOT MODIFIED.** Skills are auto-discovered from `skills/*/SKILL.md` by
the Claude Code plugin system. The `plugins/vsdd-factory/.claude-plugin/plugin.json`
contains only basic metadata and does not enumerate skills. Confirmed by inspection of
production plugin.json and the precedent of 119 existing skills, none appearing in plugin.json.

---

## Files NOT Changed (Regression Baseline)

All existing artifacts are in the regression baseline. The wrap skill is purely additive.

- `plugins/vsdd-factory/skills/**` (all 119 existing skills) — unchanged
- `plugins/vsdd-factory/hooks/*.sh` (44 bash hooks) — unchanged
- `plugins/vsdd-factory/hooks-registry.toml` — unchanged
- `plugins/vsdd-factory/.claude-plugin/plugin.json` — unchanged
- `crates/**` (all Rust crates including factory-dispatcher) — unchanged; no Rust code touched
- `.factory/specs/behavioral-contracts/ss-06/BC-6.23.001.md` — unchanged (referenced, not modified)
- `.factory/specs/behavioral-contracts/ss-06/BC-6.24.001.md` — unchanged (referenced, not modified)
- All 1,988 existing BCs — unchanged
- All existing bats test files — unchanged; all must continue to pass

---

## Risk Assessment

| Risk Type | Level | Rationale |
|-----------|-------|-----------|
| Regression | LOW | Purely additive feature; no existing code, hooks, or Rust crates modified |
| Architecture | LOW | No structural architecture change; SS-06 count update only |
| Security | LOW | Procedure document; no code execution; no new permissions or capabilities introduced |
| Performance | LOW | Skills are loaded on demand; one additional SKILL.md has negligible impact |

**Local-copy dedup risk (post-merge):** The user-level `~/.claude/skills/wrap/SKILL.md`
and the new plugin-level skill resolve as DIFFERENT slash commands (`/wrap` vs.
`/vsdd-factory:wrap`). They coexist without conflict. Post-merge manual dedup is recommended:

- **Option A (redirect):** Update `~/.claude/skills/wrap/SKILL.md` to a 2-line redirect
  pointing to `vsdd-factory:wrap`. Preserves `/wrap` shorthand.
- **Option B (remove):** Delete `~/.claude/skills/wrap/SKILL.md`. User uses
  `/vsdd-factory:wrap` exclusively.

This is a documented manual post-merge step in the story ACs, not automated.

**ADR NOT warranted.** The wrap skill introduces no new architectural patterns or technology
decisions. ADR-025 (factory lock) and ADR-026 (wave-boundary checkpoint) already govern the
underlying primitives. The resume-guidance ordering (rehydrate-wave before next-step) is a
behavioral postcondition in BC-6.28.001, not an architectural trade-off.

---

## Regression Baseline

- **Total existing BCs:** 1,988
- **Total existing bats tests:** See `plugins/vsdd-factory/tests/run-all.sh` for current count
- **Tests in risk zone:** `skills.bats` (scans skills dir) and `skills-content.bats` (scans
  SKILL.md files for author-environment leaks) — both must pass GREEN after the new
  SKILL.md is added
- **Rust test suite:** Unaffected (`cargo test --workspace` — no Rust crate changes)
- **Risk zone test files:** `plugins/vsdd-factory/tests/skills.bats`,
  `plugins/vsdd-factory/tests/skills-content.bats`

---

## Scope Recommendation

- **Mode:** Feature Mode (standard, F1–F7)
- **Estimated new stories:** 1 (S-24.01)
- **Estimated effort:** 5 story points
- **Can parallelize:** F2 (BC-6.28.001 authorship by product-owner; CAP-040 by business-analyst)
  can run in parallel; F3 (story decomposition + test authorship) follows F2
- **Quick dev routing:** NOT applicable — new BC required (standard scope threshold)

---

## Open Questions

1. **Post-merge dedup preference:** Human to confirm whether to use Option A (redirect
   `~/.claude/skills/wrap/SKILL.md` to vsdd-factory:wrap) or Option B (remove it entirely).
   This affects whether `/wrap` continues to work as a shorthand after merge.

2. **Epic scope of E-24:** Is session-lifecycle E-24 intended to carry only S-24.01 (the
   wrap skill), or should it also include future session-lifecycle features such as
   "auto-wrap on idle" or "wrap + handoff to another developer"? The answer determines
   whether the epic's charter is narrow (wrap only) or a holding epic for future
   session-tooling work.

3. **VP authorship:** BC-6.28.001 is a procedure document, not a pure-function contract.
   Formal verification (Kani/proptest) does not apply. Human to confirm whether a VP
   asserting bats coverage is sufficient (e.g., VP-N: "wrap-skill.bats covers all 8 BC-6.28.001
   postconditions"), or whether the VP should be marked `test-sufficient` without a proof
   harness sketch.
