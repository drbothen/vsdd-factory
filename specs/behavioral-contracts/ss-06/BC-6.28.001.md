---
document_type: behavioral-contract
level: L3
version: "1.2"
status: active
producer: product-owner
timestamp: 2026-08-29T00:00:00Z
phase: F2
inputs:
  - .factory/feature-delta/wrap-skill/F1-delta-analysis.md
  - plugins/vsdd-factory/agents/state-manager.md
  - .factory/specs/behavioral-contracts/ss-06/BC-6.23.001.md
  - .factory/specs/behavioral-contracts/ss-06/BC-6.24.001.md
input-hash: "36c33f3"
traces_to: .factory/specs/prd.md
origin: greenfield
extracted_from: null
subsystem: "SS-06"
capability: "CAP-040"
lifecycle_status: active
introduced: v1.0-brownfield-backfill
modified:
  - "2026-08-29 (v1.1) — Finding-4 correction: replace bats-harness verification-vehicle language with documentary-verification statement per human-directed decision 2026-08-29 (product-owner; consistency-audit)."
  - "2026-08-29 (v1.2) — POL-14 auto-promotion: lifecycle_status draft→active on S-24.01 PR #802 squash-merged 9ab5a6f6 (D-1132)."
deprecated: null
deprecated_by: null
replacement: null
retired: null
removed: null
removal_reason: null
bc_id: BC-6.28.001
section: "6.28"
last_amended: "2026-08-29 (v1.2) — POL-14 auto-promotion: lifecycle_status draft→active on S-24.01 PR #802 squash-merged 9ab5a6f6 (D-1132)."
---

# BC-6.28.001: `/vsdd-factory:wrap` MUST halt new work, persist all in-flight changes to durable branches, delegate STATE.md PAUSED transition and dated Session Resume Checkpoint to state-manager (never editing STATE.md directly), release the factory lock, verify a clean factory-artifacts working tree, and emit resume guidance that names `/vsdd-factory:rehydrate-wave` before `/vsdd-factory:next-step`

## Description

The `/vsdd-factory:wrap` skill is the canonical 7-step procedure for safely pausing the factory
pipeline so the session can be cleared or closed with zero data loss. It orchestrates three
existing primitives — the factory-lock release protocol (BC-6.23.001), the Session Resume
Checkpoint write (state-manager machinery), and the resume-ordering constraint
(BC-6.24.001) — into a single deterministic sequence that any session can execute. The skill
never writes STATE.md directly; all STATE.md mutations are delegated to
`vsdd-factory:state-manager` (BC-6.23.001 Invariant 5 / TD-VSDD-053 single-writer discipline).
After wrap completes, the sole resumption path is: fresh session → `/vsdd-factory:rehydrate-wave`
→ `/vsdd-factory:next-step` — no in-session memory is required.

## Preconditions

1. The human has explicitly invoked `/vsdd-factory:wrap`. There is no auto-wrap on
   inactivity, idle timeout, or session close — the operator must deliberately run the skill.

2. The factory pipeline is in a non-terminal state: a phase has started (`pipeline:` is not
   `COMPLETE` or `CLOSED`) and STATE.md is reachable on the `factory-artifacts` branch.

3. `vsdd-factory:state-manager` is dispatchable from within this session (state-manager is the
   sole authorized writer of STATE.md per BC-6.23.001 Invariant 5; the wrap skill cannot
   complete without delegating to it).

4. `.factory/STATE.md` is parseable with a valid YAML frontmatter block. If STATE.md is
   missing or corrupted, Step 2 of the skill routes to `/vsdd-factory:recover-state` and
   obtains human approval before continuing.

## Postconditions

### STATE.md Frontmatter Mutations (state-manager delegated, never direct)

**PC-1** — `pipeline:` frontmatter field equals `PAUSED` after wrap completes.

**PC-2** — `timestamp:` frontmatter field refreshed to an ISO-8601 timestamp of the pause
instant (e.g., `2026-08-29T18:00:00Z`).

**PC-3** — `phase:` frontmatter string begins with `PAUSED <YYYY-MM-DD>.` and captures the
pipeline position at pause time (e.g., `"PAUSED 2026-08-29. E-17 Wave-5 DELIVERY COMPLETE. ..."`).

**PC-4** — `current_step:` frontmatter field updated verbatim to describe the
`SESSION-WRAP-PAUSE-<YYYY-MM-DD>` step, per the verbatim-strict chain
(D-441/D-442/D-443/D-444/D-449): no meta-commentary, no clause-reordering, no
justification-suffix injection beyond the step description itself.

**PC-5** — `last_amended:` frontmatter field updated with today's date, version bump token,
agent identity `state-manager`, and a single-line description of the pause burst (e.g.,
`"2026-08-29 (v9.31) — SESSION-WRAP-PAUSE-2026-08-29 (state-manager; ...)"`).

**PC-6** — `version:` frontmatter field bumped by exactly one point (e.g., `"9.30"` →
`"9.31"`). No double-bump in a single burst (TD-VSDD-053).

**PC-7** — STATE.md banner comment `wc-l` claim matches the actual `wc -l .factory/STATE.md`
line count after the pause commit. The dual-margin form `margin from actual = 500 - <actual_wc_l>
= <margin>` must be accurate (BC-5.39.005 banner seal discipline — stale wc-l claims are
HIGH-severity defects).

### Session Resume Checkpoint

**PC-8** — Exactly ONE `## Session Resume Checkpoint` section exists in STATE.md after wrap
completes. The prior checkpoint is archived to `cycles/<current_cycle>/session-checkpoints.md`
before the new one is written (content-routing discipline). The new checkpoint must be dated
today and capture all six minimum fields:
  (a) date and current pipeline position (phase, step, story/wave, what is next),
  (b) convergence counter if in an adversarial/convergence loop (streak, pass count),
  (c) in-flight work (stories mid-TDD, PRs awaiting review/CI, sub-agent steps abandoned
      at Step 1),
  (d) pending human decisions and unresolved blockers raised this session,
  (e) WIP branch list and their SHAs (or "none"),
  (f) the exact resume command (e.g., `/vsdd-factory:next-step` or
      `/vsdd-factory:run-phase phase-3`).

### Phase Progress Record

**PC-9** — A Phase Progress / Current Phase Steps table row named
`SESSION-WRAP-PAUSE-<YYYY-MM-DD>` is appended, recording the human-requested pause with
status `COMPLETE`, agent `state-manager`, and a one-line output description. Trajectory-tail
`LENGTH=4` is unchanged by a bookkeeping-only pause burst.

### STATE.md Write Delegation (non-negotiable)

**PC-10** — All STATE.md mutations enumerated in PC-1 through PC-9 are performed exclusively
by `vsdd-factory:state-manager`. The wrap skill makes ZERO direct Write/Edit tool calls on
STATE.md at any step (BC-6.23.001 Invariant 5; TD-VSDD-053 single-writer). The skill
DELEGATES to state-manager via the `Agent` tool and waits for state-manager to report
completion before proceeding to Step 5.

### factory-artifacts Git Durability

**PC-11** — The state-manager pause commit uses exactly ONE commit with a subject prefix
`factory(pause):` (TD-VSDD-053 single-commit-per-burst). No `backfill` / `Stage N` parallel
commit subjects. The commit message includes the phase and one-line position string.

**PC-12** — The factory-artifacts commit is pushed to remote (`git push origin
factory-artifacts`) if a remote is configured. After push, `git -C .factory status
--porcelain` returns empty (factory-artifacts working tree clean). If no remote is
configured, the checkpoint is local-only; the Step 7 report notes `pushed: no
(no remote configured)`.

### Product Repository and Story Worktree Durability

**PC-13** — The product repo (`git status` at the main checkout) and all active
`.worktrees/STORY-*` story worktrees have no uncommitted changes that were intended to be
saved. One of two conditions holds for each worktree: (a) WIP committed to the story's
feature branch (never `main`/`develop`/default branch) with a commit message prefixed
`wip(<STORY-ID>): session wrap checkpoint — <what is half-done>`, and pushed to remote if
configured; OR (b) the exact un-committable state is explicitly documented in the checkpoint
(PC-8 field (c)) rather than forced. Mid-red-gate worktrees fall into path (b).

### Factory Lock

**PC-14** — Factory lock released via `/vsdd-factory:factory-unlock` (emitting
`factory.lock.released` per BC-6.23.001 PC4), OR the Step 7 report and checkpoint explicitly
note "no lock held" and the factory-unlock step was skipped silently. The lock state after
wrap is either FREE (released) or never-held; a held-but-unreleased lock is a wrap failure.

### Resume Guidance Ordering

**PC-15** — Resume guidance emitted in the Step 7 report names `/vsdd-factory:rehydrate-wave`
BEFORE `/vsdd-factory:next-step` (human-decided ordering). The ordering is non-negotiable per
BC-6.24.001: rehydrate-wave must execute first in a fresh session to inject the correct wave
scope before next-step attempts to route work. A wrap report that only lists `next-step`
(without `rehydrate-wave`) is a behavioral violation.

### Step 7 "Factory Wrapped" Report Content

**PC-16** — The Step 7 "Factory Wrapped" report includes all five required items:
  (a) pipeline PAUSED at `<phase>` / `<current_step>` (verbatim from the updated frontmatter),
  (b) `.factory/STATE.md` checkpoint: `committed <sha>`, `pushed: yes` or `pushed: no
      (reason)`,
  (c) WIP commits: `<branch>@<sha>, ...` or `none`,
  (d) lock status: `released` or `not held`,
  (e) `Safe to /clear or close this session.` (exact wording — this is the human-facing
      signal that all postconditions have been verified).

## Invariants

**INV-1** — The wrap skill MUST NOT write or edit STATE.md directly at any step, for any
reason, including error recovery. If state-manager is unavailable, the wrap fails with a
clear error; it does NOT fall back to direct STATE.md editing.

**INV-2** — The wrap skill MUST NOT emit the Step 7 "Factory Wrapped" report (including the
"Safe to /clear" declaration) while any PC-1 through PC-15 postcondition is unmet. The
verification sequence in Step 6 must pass completely before Step 7 runs. Reporting success
ahead of verified durability is a behavioral violation identical in severity to losing data.

**INV-3** — The wrap skill MUST NOT spawn new pipeline sub-agents (orchestrator, implementer,
test-writer, etc.) after Step 1 ("Halt new work"). state-manager dispatch at Step 4 is the
only agent delegation allowed post-halt; it is not a "pipeline sub-agent" — it is the
checkpoint-write mechanism. factory-unlock at Step 5 is a skill invocation, not an agent
dispatch, and is permitted.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | No factory lock held at wrap time | Step 5 factory-unlock is skipped silently; PC-14 satisfied via "not held" path; checkpoint notes "no lock held"; Step 7 report shows `Lock: not held` |
| EC-002 | STATE.md health check (Step 2) returns NEEDS-COMPACT | `/vsdd-factory:compact-state` runs before state-manager pause delegation; checkpoint lands in the slimmed STATE.md; all 16 PCs verified against the compacted file |
| EC-003 | STATE.md missing or corrupted | `/vsdd-factory:recover-state` runs; human approval of the reconstruction is REQUIRED before continuing; wrap cannot proceed past Step 2 without a valid, human-approved STATE.md; Step 7 is NOT emitted until PC-1 through PC-15 are satisfied on the reconstructed file |
| EC-004 | Sub-agent executing an in-progress step at Step 1 halt | Agent is allowed to complete its current atomic step (e.g., a micro-commit or a single test run); the exact state at halt is recorded in the checkpoint (PC-8 field (c)); the wrap does NOT kill running agents mid-step |
| EC-005 | Story worktree has uncommitted changes that cannot be cleanly committed (mid-red-gate, merge conflict, build failure) | State documented in checkpoint (PC-8 field (c)) with exact description; no forced commit; PC-13 satisfied via documentation path; Step 7 shows the worktree name and its un-committed status |
| EC-006 | factory-artifacts push fails or no remote is configured | Push step notes `pushed: no` with reason; working-tree clean assertion (PC-12) still checked; checkpoint is local-only; Step 7 warns operator that off-machine durability is not guaranteed |
| EC-007 | Step 6 durability verification fails (dirty tree, pipeline != PAUSED, checkpoint missing, banner wc-l mismatch) | Wrap does NOT declare success; the specific failed check is addressed (re-delegate to state-manager if STATE.md is wrong; re-run porcelain check if tree is dirty) and Step 6 re-verified; Step 7 is held until all checks pass (INV-2 enforced) |

## Canonical Test Vectors

> These vectors are DOCUMENTARY reference only (expected wrap behavior per fixture); no automated harness executes them, per human-directed decision 2026-08-29.
> Each row represents one invocation of the wrap procedure against a fixture factory state.

| Fixture State | Expected Post-Wrap State | Category |
|---------------|--------------------------|----------|
| Clean factory: `pipeline: IN-PROGRESS`, no lock held, no WIP story branches, healthy STATE.md, no compact needed | All 16 PCs satisfied; `pipeline: PAUSED`; exactly one Session Resume Checkpoint; `git -C .factory status --porcelain` empty; Step 7 shows `Lock: not held`, `WIP commits: none`, checkpoint SHA, `pushed: yes`, `Safe to /clear` | happy-path (clean state) |
| WIP story: `feature/S-24.01` worktree has uncommitted files at a mid-step; no lock held | Step 3 commits WIP to `feature/S-24.01` with `wip(S-24.01): session wrap checkpoint — <description>`; all 16 PCs satisfied; Step 7 shows `WIP commits: feature/S-24.01@<sha>` | happy-path (WIP story) |
| NEEDS-COMPACT: STATE.md at soft-limit (≥415 lines); lock held by this session | Step 2 runs compact-state first; Step 4 pause lands in slimmed STATE.md; Step 5 releases lock (`factory.lock.released` event per BC-6.23.001 PC4); all 16 PCs satisfied; Step 7 shows `Lock: released` | edge (compact + lock) |
| Durability gap: state-manager delegated at Step 4 but factory-artifacts push rejected (network error) | Step 6 detects PC-12 failure (`pushed: no`); Step 6 warns operator; Step 7 is NOT emitted until operator acknowledges local-only mode (INV-2); wrap report includes `pushed: no (push rejected — network error)` | error (push failure) |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-TBD | Executing `/vsdd-factory:wrap` against any valid non-terminal factory state yields a state satisfying all BC-6.28.001 postconditions (PC-1 through PC-16). The correctness claim is: `∀ state ∈ ValidNonTerminalFactoryStates, wrap(state) ⊨ {PC-1 ∧ PC-2 ∧ ... ∧ PC-16}`. | Verification is DOCUMENTARY per human-directed decision (2026-08-29): each S-24.01 AC traces to a specific BC-6.28.001 postcondition/invariant clause; no automated harness (no `tests/wrap-skill.bats`) is built for this LLM-executed procedure skill. Formal proof NOT applicable (procedure-document skill; no pure core logic). VP allocation deferred to architect/formal-verifier per POLICY 9 (VP-TBD). |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-040 |
| Capability Anchor Justification | CAP-040 ("Human-initiated factory session pause and resume checkpoint orchestration") per capabilities.md §CAP-040 — this BC specifies the exhaustive behavioral postconditions that the `/vsdd-factory:wrap` skill MUST satisfy; the skill is the sole implementation vehicle for CAP-040. CAP-040 was authored in the same F2 burst as this BC (2026-08-29) and defines exactly this orchestration sequence. |
| L2 Domain Invariants | none — operational infrastructure skill; not traceable to L2 domain invariants (DI-NNN series covers product domain, not engine session lifecycle) |
| Architecture Module | SS-06 (Skill Catalog) — `plugins/vsdd-factory/skills/wrap/SKILL.md` (new; S-24.01 deliverable) |
| Stories | S-24.01 (wrap skill — session pause, checkpoint, and lock-release orchestration; E-24; 5 pts; P1) |
| Cycle | v1.0-brownfield-backfill (F2 feature-mode wrap-skill E-24) |

## Related BCs

- BC-6.23.001 — composes with (PC-10 delegates all STATE.md writes to state-manager per BC-6.23.001 Invariant 5; PC-14 factory lock release uses the factory-unlock protocol from BC-6.23.001 PC4)
- BC-6.24.001 — composes with (PC-15: rehydrate-wave MUST be cited before next-step in resume guidance, per BC-6.24.001 Postcondition 4 + Precondition 4; PC-8 Session Resume Checkpoint format is the source this skill delegates to state-manager to produce)
- BC-5.39.005 — composes with (PC-7: banner `wc-l` claim in STATE.md must match actual line count; BC-5.39.005 codified this discipline after a CI regression caused by stale wc-l claims across versions v9.18–v9.25)

## Architecture Anchors

- `plugins/vsdd-factory/skills/wrap/SKILL.md` — new skill (to be created; S-24.01 primary deliverable)
- `plugins/vsdd-factory/skills/factory-unlock/SKILL.md` — invoked at Step 5 (PC-14)
- `plugins/vsdd-factory/skills/check-state-health/SKILL.md` — invoked at Step 2 (Precondition 4 + EC-002)
- `plugins/vsdd-factory/skills/compact-state/SKILL.md` — invoked at Step 2 conditionally (EC-002)
- `plugins/vsdd-factory/skills/recover-state/SKILL.md` — invoked at Step 2 conditionally (EC-003)
- `plugins/vsdd-factory/skills/rehydrate-wave/SKILL.md` — named in resume guidance (PC-15; BC-6.24.001)
- `plugins/vsdd-factory/agents/state-manager.md` — delegated to at Step 4 (PC-10 / BC-6.23.001 INV-5)
- `.factory/feature-delta/wrap-skill/F1-delta-analysis.md` — authoritative scope for this feature cycle

## Story Anchor

S-24.01 — `vsdd-factory:wrap skill — session pause, checkpoint, and lock-release orchestration`
(E-24 Session Lifecycle Orchestration; 5 pts; P1; depends on S-18.03 MERGED + S-18.10 MERGED + BC-6.23.001 active)

## VP Anchors

- VP-TBD — "Executing `/vsdd-factory:wrap` against any valid non-terminal factory state yields
  a state satisfying all BC-6.28.001 postconditions (PC-1 through PC-16)." Allocation by
  architect/formal-verifier per POLICY 9. Verification is DOCUMENTARY per human-directed
  decision (2026-08-29): each S-24.01 AC traces to a specific BC-6.28.001
  postcondition/invariant clause; no automated harness (no `tests/wrap-skill.bats`) is built
  for this LLM-executed procedure skill (VP-TBD).

## Changelog

| Version | Date | Author | Change |
|---------|------|--------|--------|
| v1.2 | 2026-08-29 | state-manager | POL-14 auto-promotion: lifecycle_status draft→active on S-24.01 PR #802 squash-merged 9ab5a6f6 (D-1132). |
| v1.1 | 2026-08-29 | product-owner | Finding-4 correction (consistency-audit): replace bats-harness verification-vehicle language in §VP Anchors, §Verification Properties, and §Canonical Test Vectors with documentary-verification statement per human-directed decision 2026-08-29 — no automated `tests/wrap-skill.bats` harness is built for this LLM-executed procedure skill. 16 Postconditions, 3 Invariants, 7 Edge Cases, H1 title, capability/subsystem anchors unchanged. |
| v1.0 | 2026-08-29 | product-owner | Initial authoring (F2 feature-mode wrap-skill E-24; CAP-040; 4 Preconditions; 16 Postconditions PC-1..PC-16 exhaustively derived from state-manager pause machinery, STATE.md frontmatter conventions, BC-6.23.001 INV-5, BC-6.24.001, BC-5.39.005; 3 Invariants; 7 Edge Cases EC-001..EC-007; 4 Canonical Test Vectors; VP-TBD completeness harness stub; full Traceability + Related BCs + Architecture Anchors). lifecycle_status: draft (POL-14 auto-promotion on S-24.01 PR merge). |
