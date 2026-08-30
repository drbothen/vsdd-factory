---
document_type: epic
level: L3
traces_to: .factory/stories/STORY-INDEX.md
epic_id: "E-17"
version: "v1.3"
status: draft
title: "Factory State Durability and Concurrency — single-writer factory lock/lease (issue #170)"
prd_capabilities: [CAP-031]
subsystems_affected: [SS-04, SS-05, SS-06, SS-07]
target_release: "v1.0.0-rc.18"
# NOTE: target_release above is STALE (predates current rc.24). Wave-5 target release pending human confirmation.
story_count: 7
producer: story-writer
timestamp: 2026-06-10T00:00:00Z
phase: brownfield-backfill
cycle: v1.0-brownfield-backfill
depends_on: []
inputs:
  - .factory/specs/architecture/decisions/ADR-025-single-writer-factory-locklease-prevent-concurrent-session-races-on-factory-artifacts-orphan-branch.md
  - .factory/specs/architecture/decisions/ADR-046-postToolUse-hook-authored-state-md-wall-clock-stamping-and-timestamp-lock-keep-alive.md
  - .factory/specs/behavioral-contracts/ss-04/BC-4.13.001.md
  - .factory/specs/behavioral-contracts/ss-04/BC-4.17.001.md
  - .factory/specs/behavioral-contracts/ss-05/BC-5.40.001.md
  - .factory/specs/behavioral-contracts/ss-06/BC-6.23.001.md
  - .factory/specs/behavioral-contracts/ss-07/BC-7.07.001.md
input-hash: "d29ab2a"
last_amended: "2026-08-30 (v1.3) — frontmatter normalization: add level: L3 (story-writer spec-hygiene sweep)"
modified:
  - "v1.1 2026-06-11: S-17.04 added (ADR-025 v1.4 Decision 11)"
  - "v1.2 2026-08-27: Wave-5 decomposition (D-1124+D-1125): S-17.05 updated (depends_on [S-17.06]); S-17.06 NEW (factory-lock shared fns; 5pts; BC-4.17.001); S-17.07 NEW (precompact-flush identity gate; 5pts; BC-7.07.001). story_count 4→7; total pts 26→44."
---

# Epic E-17: Factory State Durability and Concurrency — single-writer factory lock/lease

## Description

Implements the cross-session single-writer factory lock/lease primitive specified in
ADR-025 v1.2 (D-540 codified). This is the first story chain in the #170→#173→#171
state-durability initiative. The epic closes the gap where two concurrent developer
sessions could race the `factory-artifacts` orphan branch and silently clobber each
other's state commits.

The mechanism is a cooperative advisory lock: a `factory_lock` frontmatter block in
STATE.md (the authoritative lock record), a native-WASM PreToolUse guard that reads
it and blocks mutating tools when a foreign unexpired lock is held, explicit
`/factory-lock` and `/factory-unlock` skills for deliberate acquire/release, a
`state-burst` fetch-then-CAS push upgrade as an independent safety net, and health
status surfacing in `/factory-health` and `/factory-worktree-health`.

## Trigger / Motivation

Research cache confirms the gap with zero relevant hits for lock/flock/mutex/lease
across `plugins/` (issue-170.md VALID-NEW, High confidence, 2026-06-09). The push
path at `skills/state-burst/SKILL.md` is a plain `git push origin factory-artifacts`
— no compare-and-swap, no exclusivity check. `hooks/verify-git-push.sh` explicitly
allows `factory-artifacts` pushes and `--force-with-lease` with no exclusivity guard.

ADR-025 was confirmed by human design review 2026-06-10 and research-agent
APPROVE-WITH-FIXES (5 fixes incorporated in v1.2). All ten decisions are final.
D-540 + D-541 codified by state-manager. Implementation may proceed per
`human_gate_required: false` in ADR-025 frontmatter.

## Epic Placement Justification

E-16 is taken (block-ai-attribution capability extensions). E-17 is the next free
ID under POLICY 1 (append-only numbering per STORY-INDEX confirmed no E-17 row).

The three stories span SS-04 (new WASM guard crate), SS-05 (STATE.md schema +
state-burst CAS push), and SS-06 (new acquire/release skills + health status).
A shared epic is correct: all three stories deliver a single user-visible capability
(CAP-031 cross-session single-writer protection) and share the same ADR anchor,
the same BC family (BC-4.13.001 / BC-5.40.001 / BC-6.23.001), and the same issue #170.

## PRD Capabilities Covered

| Capability ID | Name | Priority |
|--------------|------|----------|
| CAP-031 | Enforce single-writer cross-session exclusivity on factory-artifacts state | P1 |

## Stories

| Story ID | Title | Deliverables | BCs | Wave | Points |
|----------|-------|--------------|-----|------|--------|
| S-17.01 | factory_lock STATE.md schema + state-burst CAS push | D3, D6 | BC-5.40.001 | 1 | 5 |
| S-17.02 | verify-factory-lock WASM guard crate + registry | D1, D2, D9 (unit+guard bats) | BC-4.13.001 | 2 | 8 |
| S-17.03 | /factory-lock + /factory-unlock skills + health status surfacing | D4, D5, D7, D8, D9 (skill bats) | BC-6.23.001 | 3 | 8 |
| S-17.04 | Automatic mid-burst heartbeat renewal wiring — SKILL renew step + verify-lock-renewal.sh PreToolUse gate | D10, D11, D12, D13, D14 | BC-5.40.001 (PC4) | 4 | 5 |
| S-17.06 | factory-lock shared functions — KV-store lock-key extractor, lock-parse crate refactor, verify-factory-lock reuse | shared-fns crate, lock-key extractor API | BC-4.17.001 | 5 | 5 |
| S-17.05 | stamp-state-timestamp PostToolUse hook — hook-authored STATE.md timestamp re-stamp + identity-gated keep-alive (ADR-046) | stamp-state-timestamp WASM plugin | BC-4.17.001, BC-5.40.001 | 5 | 8 |
| S-17.07 | precompact-flush identity gate — identity gate verifying caller context before keep-alive (ADR-046 Wave-5 decomposition) | precompact-flush identity gate | BC-7.07.001 | 5 | 5 |

**Sequencing rationale:**

- Wave 1 (S-17.01): The `factory_lock` schema in STATE.md is the data foundation the
  guard reads and the skills write. The `state-burst` CAS push is independently
  deliverable and improves safety immediately. No dependencies on other stories.

- Wave 2 (S-17.02): The WASM guard reads the `factory_lock` block written by S-17.01's
  schema. The guard crate and registry entries depend on the schema being defined and
  the CAS push pattern being available as a reference. `depends_on: [S-17.01]`.

- Wave 3 (S-17.03): The `/factory-lock` and `/factory-unlock` skills write the
  `factory_lock` block (per S-17.01 schema) and must coexist with the guard
  (S-17.02 enforcement active before skills are delivered creates a safe invariant: no
  skills means no lock writes means no guard firings). The health display reads
  STATE.md which is defined in S-17.01. `depends_on: [S-17.01, S-17.02]`.

## Dependency Graph

```
S-17.01 (schema + CAS push) --> S-17.02 (WASM guard)
                          ╘--> S-17.03 (skills + health)
                                  ^
S-17.02 (guard active) ----------'

S-17.04 (renewal wiring) -- no product deps (renew subcommand merged in S-17.01)

Wave 5 (D-1124 decomposition):
S-17.06 (shared factory-lock fns) --> S-17.05 (stamp-state-timestamp hook)
                                 ╘--> S-17.07 (precompact-flush identity gate)
```

Topological order: S-17.01 → S-17.02 → S-17.03; S-17.04 has no product deps (parallel to any wave ≥2, scheduled wave 4). Wave 5: S-17.06 → {S-17.05, S-17.07} (parallel after S-17.06). No cycles. Acyclic confirmed.

## Out of Scope

- **Decision 9 git-ref CAS upgrade path:** ADR-025 §Decision 9. Server-side ref CAS
  as the primary mechanism requires an empirical GitHub.com CAS verification probe.
  Preserved as a future upgrade for teams requiring fencing tokens or zero-TOCTOU
  acquire-race guarantees.

- **Session-level identity (composite hostname::pid::CLAUDE_SESSION_ID):** Rejected
  in ADR-025 §Decision 3. Git user.email is the identity granularity for v1; self-vs-self
  concurrency is out of scope and mitigated by the blind-push fix (S-17.01).

- **Auto-acquire on first write:** Rejected in ADR-025 §Decision 6. Explicit acquire
  is the correct UX model.

- **Per-story granularity lock:** Rejected in ADR-025 §Alternatives. Whole-factory
  granularity is conservative and simpler.

## Behavioral Contract Traceability

| BC ID | Title | Story |
|-------|-------|-------|
| BC-5.40.001 | factory_lock STATE.md schema + TTL + mid-burst renewal + state-burst CAS push | S-17.01 (PC1-PC3/PC5-PC6); S-17.04 (PC4 enforcement wiring); S-17.05 (PC4 hook actor) |
| BC-4.13.001 | verify-factory-lock WASM PreToolUse guard | S-17.02 |
| BC-6.23.001 | /factory-lock + /factory-unlock + health status | S-17.03 |
| BC-4.17.001 | stamp-state-timestamp PostToolUse hook identity gate | S-17.05 (stamper hook); S-17.06 (shared lock-key extractor crate) |
| BC-7.07.001 | precompact-flush Native WASM Plugin identity gate | S-17.07 (precompact-flush identity gate) |

## Regression Risk Summary

**Risk level: LOW–MEDIUM.**

- S-17.01 modifies only STATE.md frontmatter schema and `state-burst/SKILL.md` push
  logic. No existing WASM crates changed. Regression risk: existing state-burst tests
  must pass; the only behavioral change is the push command (blind → CAS).

- S-17.02 introduces a NEW crate. The new WASM guard registers at PreToolUse for
  Edit|Write|Agent and Bash. If the guard has a bug, `on_error = "continue"` prevents
  it from wedging the factory — the worst case is a missed block (fail-open). Regression
  risk: the guard must NOT fire on absent/expired/self-held locks (all return Continue).

- S-17.03 adds new skills and amends two existing skills (factory-health,
  factory-worktree-health). Amendments add display-only lines; existing skill behavior
  is unchanged. The new skills are net-new (no existing callers). Regression risk:
  existing factory-health bats must pass unchanged.

## Acceptance Criteria

Epic-level ACs are defined per-story in the per-story implementation specs. See:
- S-17.01: `.factory/stories/S-17.01-factory-lock-state-md-schema.md` ACs
- S-17.02: `.factory/stories/S-17.02-verify-factory-lock-wasm-guard.md` ACs
- S-17.03: `.factory/stories/S-17.03-factory-lock-skills.md` ACs
- S-17.04: `.factory/stories/S-17.04-mid-burst-heartbeat-renewal-wiring.md` ACs
- S-17.05: `.factory/stories/S-17.05-stamp-state-timestamp-hook.md` ACs (14 ACs; BC-4.17.001+BC-5.40.001)
- S-17.06: `.factory/stories/S-17.06-factory-lock-shared-functions.md` ACs (BC-4.17.001)
- S-17.07: `.factory/stories/S-17.07-precompact-flush-identity-gate.md` ACs (BC-7.07.001)

## Dependencies (External)

| Dependency | Type | Reason |
|------------|------|--------|
| ADR-025 factory-lock schema | Internal spec | S-17.01-04 implement ADR-025 decisions |
| ADR-046 PostToolUse hook stamping | Internal spec | S-17.05-07 implement ADR-046 decisions |
| BC-5.40.001 factory_lock TTL/renewal | Internal spec | S-17.01/S-17.04/S-17.05 behavioral contracts |
| BC-4.17.001 stamp-state-timestamp guard | Internal spec | S-17.05/S-17.06 behavioral contracts |
| BC-7.07.001 precompact-flush identity gate | Internal spec | S-17.07 behavioral contract |
| factory-artifacts git worktree | Infrastructure | S-17.01 CAS push; S-17.05 PostToolUse re-stamp |

## Changelog

| Version | Date | Author | Change |
|---------|------|--------|--------|
| v1.3 | 2026-08-30 | story-writer | Frontmatter normalization: add level: L3 (story-writer spec-hygiene sweep). |
| v1.2 | 2026-08-27 | state-manager | Wave-5 decomposition cascade Phase D (D-1124+D-1125): S-17.05 v1.1→v1.2 (depends_on []→[S-17.06]; input-hash 4702970→e8b9395; 23 Red Gate tests); S-17.06 NEW (factory-lock shared fns; 5pts; BC-4.17.001; depends_on []; blocks S-17.05+S-17.07; input-hash 372f2eb); S-17.07 NEW (precompact-flush identity gate; 5pts; BC-7.07.001; depends_on [S-17.06]; input-hash 028002a). story_count 4→7; total pts 26→44; DAG S-17.06→{S-17.05,S-17.07}. target_release stale note added. |
| v1.1 | 2026-06-11 | story-writer | S-17.04 added (ADR-025 v1.4 Decision 11; BC-5.40.001 PC4 enforcement wiring; wave 4; 5 pts; depends_on []). story_count 3→4; total pts 21→26. BC-5.40.001 traceability updated. Dependency graph note added for S-17.04 no-deps placement. |
| v1.0 | 2026-06-10 | story-writer | Initial authoring. brownfield-backfill issue #170; ADR-025 v1.2; D-540+D-541 codified. 3 stories S-17.01/02/03 spanning SS-04/SS-05/SS-06. |
