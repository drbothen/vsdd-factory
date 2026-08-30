---
document_type: epic
epic_id: "E-24"
version: "v1.0"
status: draft
title: "Session Lifecycle Orchestration — wrap, pause, checkpoint, and future session-tooling"
prd_capabilities: [CAP-040]
subsystems_affected: [SS-06]
target_release: "v1.0.0"
story_count: 1
producer: story-writer
timestamp: 2026-08-29T00:00:00Z
phase: F3
cycle: v1.0-brownfield-backfill
depends_on: [E-18]
inputs:
  - .factory/feature-delta/wrap-skill/F1-delta-analysis.md
  - .factory/specs/behavioral-contracts/ss-06/BC-6.28.001.md
input-hash: "88faa0e"
last_amended: "2026-08-29 (v1.0) — initial authoring (story-writer; F3 feature-mode wrap-skill; CAP-040; S-24.01 as first story)"
modified: []
---

# Epic E-24: Session Lifecycle Orchestration

## Description

**This is a HOLDING EPIC** (human-directed, 2026-08-29). E-24 is the home for
`/vsdd-factory:wrap` now and for any future session-lifecycle stories:
automated wrap-on-idle, cross-repo wrap coordination, resume-flow improvements,
and related orchestration enhancements. S-24.01 is the first and currently only
story.

The epic delivers the canonical human-initiated factory session pause-and-resume
workflow. When the human says "wrap", "wrap up", or "wrap the session", the
`/vsdd-factory:wrap` skill executes a deterministic 7-step sequence: halt new work,
verify factory health (routing to compact-state or recover-state as needed), commit
all in-flight WIP to durable story branches, delegate the pipeline-PAUSED STATE.md
update and dated Session Resume Checkpoint to `vsdd-factory:state-manager` (never
editing STATE.md directly, per BC-6.23.001 Invariant 5), release the factory lock,
verify all durability postconditions, and emit a `## Factory Wrapped` report with
resume instructions that cite `/vsdd-factory:rehydrate-wave` before
`/vsdd-factory:next-step`.

The skill itself is a port of the battle-tested user-level `~/.claude/skills/wrap/SKILL.md`
into `plugins/vsdd-factory/skills/wrap/SKILL.md`, with one behavioral reconciliation:
the resume guidance in Step 7 cites `/vsdd-factory:rehydrate-wave` FIRST (mandatory
post-clear per BC-6.24.001), then `/vsdd-factory:next-step`. The local skill omits
`rehydrate-wave`; this ordering is the only behavioral change from the source.

## Trigger / Motivation

The `/wrap` skill has been used in production sessions against this repo (footprint
visible at `STATE.md` `SESSION-WRAP-PAUSE-2026-08-28` and
`SESSION-WRAP-PAUSE-2026-08-29` entries). It is battle-tested. The human requested
it be ported into the plugin so it ships as `vsdd-factory:wrap` and is formally
governed by a behavioral contract (BC-6.28.001 — authored F2, 2026-08-29). F1
analysis (`.factory/feature-delta/wrap-skill/F1-delta-analysis.md`) confirms:
standard scope; 1 new story; 5 story points; P1.

All prerequisite infrastructure is MERGED: S-18.03 (rehydrate-wave), S-18.10
(check-state-health), BC-6.23.001 (factory-lock protocol, active). No new external
dependencies.

## Epic Placement Justification

| Considered Epic | Status | Why NOT |
|----------------|--------|---------|
| E-18 Factory Context Durability | Complete (all 12 stories merged; final S-18.10) | Closed; built the primitives this skill orchestrates |
| E-21 Factory State Data Loss Hardening | S-21.25 is last story; focused on worktree/PR/lock integrity | Wrong scope category |
| E-23 ADR-045 Stable-Anchor Migration | S-23.01–S-23.14; cross-reference migration only | Wrong scope entirely |

E-20 is reserved per POLICY 1 (pre-existing gap). E-22 is dissolved per epic catalog.
E-24 is the next free epic ID confirmed from the STORY-INDEX catalog at time of
creation.

This epic is a HOLDING EPIC: narrow initial scope (S-24.01 wrap skill) but chartered
to absorb future session-lifecycle features rather than opening a new epic per
increment. Human confirmed this as the correct placement (2026-08-29).

## PRD Capabilities Covered

| Capability ID | Name | Priority |
|--------------|------|----------|
| CAP-040 | Human-initiated factory session pause and resume checkpoint orchestration | P1 |

## Acceptance Criteria

| ID | Criterion | Validation Method | Test Scenarios |
|----|-----------|-------------------|----------------|
| EAC-001 | `plugins/vsdd-factory/skills/wrap/SKILL.md` exists, has frontmatter `name: wrap`, and contains exactly 7 numbered steps | Documentary inspection; `plugins/vsdd-factory/tests/skills.bats` structural scan | S-24.01 AC-001 through AC-008 |
| EAC-002 | Step 7 resume guidance cites `/vsdd-factory:rehydrate-wave` before `/vsdd-factory:next-step` (BC-6.28.001 PC-15; BC-6.24.001) | Documentary inspection of SKILL.md Step 7 | S-24.01 AC-007 |
| EAC-003 | No direct STATE.md Write/Edit instruction appears anywhere in the skill body (BC-6.23.001 INV-5; BC-6.28.001 INV-1) | Documentary inspection | S-24.01 AC-009 |

## Stories

| Story ID | Title | Wave | Points | BCs |
|----------|-------|------|--------|-----|
| S-24.01 | vsdd-factory:wrap skill — session pause, checkpoint, and lock-release orchestration | W1 | 5 | BC-6.28.001 |

**Total:** 1 story, 5 story points.

## Dependencies (External)

| System | Capability Needed | Readiness |
|--------|------------------|-----------|
| `vsdd-factory:state-manager` (internal agent) | STATE.md PAUSED transition + Session Resume Checkpoint write (BC-6.23.001 INV-5) | READY — present since E-17 |
| `vsdd-factory:factory-unlock` skill (SS-06) | Factory lock release (BC-6.23.001 PC4) | READY — S-17.x MERGED |
| `vsdd-factory:check-state-health` skill (SS-06) | Factory health gate before pause delegation (BC-6.25.001) | READY — S-18.10 MERGED |
| `vsdd-factory:compact-state` skill (SS-06) | Conditional compaction if NEEDS-COMPACT (EC-002) | READY — S-18.x MERGED |
| `vsdd-factory:recover-state` skill (SS-06) | Recovery if STATE.md corrupted (EC-003) | READY |
| `vsdd-factory:rehydrate-wave` skill (SS-06) | Named in resume guidance Step 7 (BC-6.24.001; PC-15) | READY — S-18.03 MERGED |

**Sequencing rationale:**

- Single story (S-24.01); no intra-epic waves required. Wave assignment is W1 of
  this epic's delivery cycle. The story depends on S-18.03 and S-18.10 (both MERGED
  in E-18), so it has no unsatisfied predecessors.

## Dependency Graph

```
S-24.01 (W1)
  depends_on: S-18.03 (MERGED), S-18.10 (MERGED), BC-6.23.001 (active)
  blocks: none
```

Topological order: W1 only. Acyclic confirmed (no intra-epic arcs).

## Out of Scope

- **Auto-wrap on idle / inactivity timeout:** ADR-026 §Decision 4 precedent; human
  must deliberately invoke `/vsdd-factory:wrap`. Auto-trigger is a future E-24 story.

- **Cross-repo wrap coordination:** Coordinating wrap across multiple open worktrees
  in different repos is a future E-24 story.

- **Resume-flow automation:** Automatically running `rehydrate-wave` + `next-step`
  after a fresh session open is a future E-24 story.

- **bats test harness for the wrap skill:** The human has explicitly decided NOT to
  build a bats test harness for this skill. The F1 analysis proposed `wrap-skill.bats`
  (15 tests, Tier 4); this is HUMAN-DIRECTED NOT TO BUILD. Story verification is
  documentary: ACs trace to BC-6.28.001 postconditions; VP-TBD deferred per
  POLICY 9. See S-24.01 §tdd_mode note.

## Behavioral Contract Traceability

| BC ID | Title (abbreviated) | Story |
|-------|---------------------|-------|
| BC-6.28.001 | `/vsdd-factory:wrap` MUST halt new work, persist in-flight changes, delegate STATE.md PAUSED + checkpoint to state-manager, release lock, verify clean factory-artifacts tree, emit resume guidance naming rehydrate-wave before next-step | S-24.01 |

## Regression Risk Summary

**Risk level: LOW.**

- Purely additive: one new `SKILL.md` file; no existing hooks, Rust crates, or
  hook-registry.toml modified.
- `plugins/vsdd-factory/tests/skills.bats` will scan the new `wrap/SKILL.md`; no
  change to the scan logic, just one more file to pass structural checks.
- `plugins/vsdd-factory/tests/skills-content.bats` will scan the new SKILL.md for
  author-environment leaks; the source skill (`~/.claude/skills/wrap/SKILL.md`)
  was confirmed clean.

## Changelog

| Version | Date | Author | Change |
|---------|------|--------|--------|
| v1.0 | 2026-08-29 | story-writer | Initial creation (F3 feature-mode wrap-skill E-24; CAP-040; BC-6.28.001; 1 story S-24.01; 5 pts; P1; SS-06; HOLDING epic per human direction). |
