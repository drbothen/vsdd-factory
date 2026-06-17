---
document_type: epic
epic_id: "E-18"
version: "v1.0"
status: draft
title: "Factory Context Durability — wave-boundary checkpoint, PreCompact flush, and lossless intra-wave compaction (issue #173)"
prd_capabilities: [CAP-032]
subsystems_affected: [SS-01, SS-04, SS-05, SS-06, SS-07]
target_release: "v1.0.0-rc.23"
story_count: 12
producer: story-writer
timestamp: 2026-06-16T00:00:00Z
phase: F3
cycle: v1.0-feature-context-durability-E18
depends_on: [E-17]
inputs:
  - .factory/feature-delta/issue-173/F1-delta-analysis.md
  - .factory/specs/architecture/decisions/ADR-026-wave-boundary-checkpoint-reset-and-lossless-intra-wave-compaction.md
  - .factory/specs/behavioral-contracts/ss-04/BC-4.14.001.md
  - .factory/specs/behavioral-contracts/ss-04/BC-4.15.001.md
  - .factory/specs/behavioral-contracts/ss-05/BC-5.41.001.md
  - .factory/specs/behavioral-contracts/ss-05/BC-5.41.002.md
  - .factory/specs/behavioral-contracts/ss-05/BC-5.41.003.md
  - .factory/specs/behavioral-contracts/ss-06/BC-6.24.001.md
  - .factory/specs/behavioral-contracts/ss-06/BC-6.25.001.md
  - .factory/specs/behavioral-contracts/ss-07/BC-7.07.001.md
  - .factory/specs/behavioral-contracts/ss-07/BC-7.07.002.md
  - .factory/specs/behavioral-contracts/ss-01/BC-1.15.001.md
input-hash: "c2426d5"
---

# Epic E-18: Factory Context Durability — wave-boundary checkpoint, PreCompact flush, and lossless intra-wave compaction

## Description

Implements the full wave-boundary checkpoint, PreCompact flush, and lossless intra-wave
compaction system specified in ADR-026 v1.5 (D-576 codified). This is the third story
chain in the #170→#173→#171 state-durability initiative. The epic closes the gap where
Claude Code's automatic context-window compaction (triggered by the `CLAUDE_AUTOCOMPACT_PCT_OVERRIDE`
env var) could silently discard load-bearing pipeline state in the middle of a multi-wave
delivery cycle — violating DI-020 (wave/phase boundary transitions must not lose
load-bearing pipeline state).

The mechanism is a cooperative durability envelope: a `HANDOFF.md` wave-boundary
checkpoint validated by a WASM completeness gate on every write, a `precompact-flush.sh`
PostCompact hook that commits the current STATE.md snapshot before compaction erases
it from context, a `postcompact-reanchor.sh` advisory hook that re-anchors the session
after compaction from the git-sourced STATE.md, a `rehydrate-wave` skill for
git-sourced scoped context rehydration, and supporting gate-automation stories
(pure-parse invariant gate, F2 process-gap lesson gate).

## Trigger / Motivation

Research cache (issue-173.md VALID-NEW, High confidence, 2026-06-14) confirms the gap:
Claude Code's `CLAUDE_AUTOCOMPACT_PCT_OVERRIDE` threshold can trigger mid-session
compaction silently. After compaction, context is truncated to a summarized snapshot
that may omit critical in-flight pipeline state (active BCs, pending_fixes, open
decisions, wave progress). The factory has no mechanism to detect or recover from this.

ADR-026 was confirmed by research-agent APPROVE (12 decisions; 2026-06-14). All
decisions are final per human gate. Implementation proceeds per `human_gate_required:
false` in ADR-026 frontmatter.

## Epic Placement Justification

E-17 is taken (factory lock/lease). E-18 is the next free ID under POLICY 1
(append-only numbering; STORY-INDEX confirmed no E-18 row at time of creation).

The 12 stories span SS-01 (dispatcher routing), SS-04 (new WASM gates), SS-05
(HANDOFF.md schema + skill orchestration), SS-06 (rehydrate-wave + check-state-health
skills + terminology docs), and SS-07 (precompact-flush.sh + postcompact-reanchor.sh
bash hooks). A shared epic is correct: all stories deliver a single user-visible
capability (CAP-032 lossless context-window transitions) and share the same ADR anchor
(ADR-026), the same BC family (BC-4.14.001 / BC-4.15.001 / BC-5.41.001-003 /
BC-6.24.001 / BC-6.25.001 / BC-7.07.001-002 / BC-1.15.001), and issue #173.

## PRD Capabilities Covered

| Capability ID | Name | Priority |
|--------------|------|----------|
| CAP-032 | Guarantee lossless context-window transitions via wave-boundary checkpoint and PreCompact flush | P0 |

## Stories

| Story ID | Title | Wave | Points | BCs |
|----------|-------|------|--------|-----|
| S-18.00 | Dispatcher PreCompact/PostCompact Routing + check-harness-version.sh | W1 | 8 | BC-1.15.001 |
| S-18.01 | HANDOFF.md Schema + wave-handoff Skill; wave-state.yaml Atomic Production | W2 | 13 | BC-5.41.001, BC-5.41.002 |
| S-18.04a | precompact-flush.sh Core | W2 | 13 | BC-7.07.001 |
| S-18.02 | validate-wave-handoff-completeness WASM Gate Crate | W3 | 8 | BC-4.14.001 |
| S-18.04b | validate-burst-log / validate-dispatch-advance PreCompact Exemption + precompact-flush-prune.sh | W3 | 8 | BC-5.41.003 |
| S-18.05 | postcompact-reanchor.sh advisory hook — PostCompact re-anchor from git-sourced STATE.md | W3 | 5 | BC-7.07.002 |
| S-18.03 | rehydrate-wave skill — git-sourced scoped rehydration + wave-reset SKILL.md | W4 | 8 | BC-6.24.001 |
| S-18.06 | validate-heavy-op-delegation WASM gate — advisory DelegationRecommended on heavy Bash operations | W5 | 8 | BC-4.15.001 |
| S-18.07 | E-18 terminology disambiguation docs — compact-state vs PreCompact flush; SKILL.md cross-references | W6 | 3 | (doc-only) |
| S-18.08 | O-P8-002 pure-parse invariant consistency gate — consistency-validator scan of BCs declaring pure-parse | W7 | 5 | (gate-enforcement) |
| S-18.10 | check-state-health CLAUDE_AUTOCOMPACT_PCT_OVERRIDE settings.json Verification | W7 | 5 | BC-6.25.001 |
| S-18.09 | F2 process-gap lesson gate checks — machine-stable assertions, stale-term detector, BC-precondition registry-block-shape validator | W8 | 5 | (gate-enforcement) |

**Total:** 12 stories, 89 story points.

**Sequencing rationale:**

- Wave 1 (S-18.00): Establishes dispatcher routing for PreCompact/PostCompact events and the harness-version check. No dependencies on other E-18 stories.

- Wave 2 (S-18.01, S-18.04a): HANDOFF.md schema and precompact-flush.sh are the data/hook foundations. S-18.01 produces the schema the gate validates. S-18.04a produces the flush hook and append-log that S-18.04b's exemption logic reads. Both depend only on S-18.00.

- Wave 3 (S-18.02, S-18.04b, S-18.05): WASM gate (reads schema from S-18.01), WASM exemption (reads log format from S-18.04a), and postcompact-reanchor hook (depends on S-18.00 routing). Run in parallel.

- Wave 4 (S-18.03): rehydrate-wave skill depends on S-18.04a (log), S-18.04b (exemption confirmed), S-18.06, S-18.07.

- Wave 5 (S-18.06): validate-heavy-op-delegation WASM gate. Depends on S-18.03 (wave ordering).

- Wave 6 (S-18.07): Terminology disambiguation docs. Depends on S-18.03, S-18.04a/b, S-18.05 all being in final state.

- Wave 7 (S-18.08, S-18.10): Consistency gate (scans all E-18 BCs) and check-state-health verification. Both depend on S-18.07 or earlier stories.

- Wave 8 (S-18.09): F2 process-gap lesson gate. Depends on S-18.08 (wave 7) so must be in a successor wave. Terminal story for E-18.

## Dependency Graph

```
S-18.00 (W1)
    ├─→ S-18.01 (W2)
    │       └─→ S-18.02 (W3)
    │               └─→ S-18.08 (W7)
    ├─→ S-18.04a (W2)
    │       └─→ S-18.04b (W3)
    │               └─→ S-18.03 (W4)
    │                       └─→ S-18.06 (W5)
    │                               └─→ S-18.07 (W6)
    │                                       └─→ S-18.08 (W7) ─→ S-18.09 (W8)
    └─→ S-18.05 (W3)
            └─→ S-18.07 (W6) ──────────────────────────────────────────'
S-18.10 (W7) depends on S-18.07
```

Topological order: W1→W2→W3→W4→W5→W6→W7→W8. No cycles. Acyclic confirmed.

## Out of Scope

- **Auto-rehydration on compaction detection:** ADR-026 §Decision 4 explicitly defers
  auto-rehydration to v2. Manual `/rehydrate-wave` skill invocation is v1.

- **Multi-session concurrent compaction:** Out of scope; E-17 (single-writer lock) is
  the prerequisite. E-18 assumes single-writer model from E-17.

- **HANDOFF.md schema versioning:** ADR-026 §Decision 7 pins to v1.0 schema; versioning
  migration is a future concern.

## Behavioral Contract Traceability

| BC ID | Title | Story |
|-------|-------|-------|
| BC-1.15.001 | Dispatcher routes PostCompact and PreCompact hook events | S-18.00 |
| BC-5.41.001 | wave-handoff skill — HANDOFF.md write + wave-state.yaml production | S-18.01 |
| BC-5.41.002 | wave-gate skill — HANDOFF.md presence validation + wave close | S-18.01 |
| BC-4.14.001 | validate-wave-handoff-completeness WASM gate | S-18.02 |
| BC-7.07.001 | precompact-flush.sh commits STATE.md snapshot before compaction | S-18.04a |
| BC-5.41.003 | PreCompact flush commits exempt from MULTI_COMMIT_CHAIN_NOT_ALLOWED | S-18.04b |
| BC-7.07.002 | postcompact-reanchor.sh re-anchors session after compaction | S-18.05 |
| BC-6.24.001 | rehydrate-wave skill — git-sourced scoped context rehydration | S-18.03 |
| BC-4.15.001 | validate-heavy-op-delegation WASM gate — DelegationRecommended advisory | S-18.06 |
| BC-6.25.001 | check-state-health CLAUDE_AUTOCOMPACT_PCT_OVERRIDE verification | S-18.10 |

## Regression Risk Summary

**Risk level: MEDIUM.**

- New WASM crates (S-18.02, S-18.04b, S-18.06) use `on_error = "continue"` (fail-open).
  Worst-case regression: gate misses a block — factory continues unblocked. No false-positive
  block risk on clean operations.
- New bash hooks (S-18.04a, S-18.05) run PostCompact, a new event type. Regression risk
  confined to PostCompact event path; existing PreToolUse/PostToolUse paths unchanged.
- S-18.01 adds HANDOFF.md schema to wave-handoff skill. Existing skill behavior unchanged
  for non-handoff writes. Wave-gate skill gains new HANDOFF.md validation step.
- S-18.07 and S-18.08/S-18.09 are read-only gate automation (doc update + scan bats tests)
  with no behavioral changes to existing hooks.

## Changelog

| Version | Date | Author | Change |
|---------|------|--------|--------|
| v1.0 | 2026-06-16 | story-writer | Initial creation (story-side fix burst M-005). E-18 context-durability epic for issue #173; ADR-026; D-576 codified. 12 stories S-18.00..S-18.10 spanning SS-01/04/05/06/07. 8 waves; 89 pts. CAP-032 anchor. |
