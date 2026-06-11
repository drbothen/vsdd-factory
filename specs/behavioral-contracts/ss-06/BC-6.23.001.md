---
document_type: behavioral-contract
level: L3
version: "1.2"
status: active
producer: product-owner
timestamp: 2026-06-10T00:00:00Z
phase: brownfield-backfill
cycle: v1.0-brownfield-backfill
inputs:
  - .factory/specs/architecture/decisions/ADR-025-single-writer-factory-locklease-prevent-concurrent-session-races-on-factory-artifacts-orphan-branch.md
  - .factory/specs/behavioral-contracts/ss-05/BC-5.40.001.md
  - .factory/specs/behavioral-contracts/ss-04/BC-4.13.001.md
  - .factory/specs/behavioral-contracts/BC-INDEX.md
  - plugins/vsdd-factory/skills/factory-health/SKILL.md
input-hash: "[pending-recompute]"
traces_to: .factory/specs/architecture/decisions/ADR-025-single-writer-factory-locklease-prevent-concurrent-session-races-on-factory-artifacts-orphan-branch.md
origin: brownfield
subsystem: "SS-06"
capability: "CAP-031"
lifecycle_status: active
introduced: v1.0-brownfield-backfill
modified: ["2026-06-11 (v1.1)", "2026-06-11 (v1.2)"]
deprecated: null
deprecated_by: null
replacement: null
retired: null
removed: null
removal_reason: null
bc_id: BC-6.23.001
section: "6.23"
last_amended: "2026-06-11 (v1.2) — POL-14 auto-promotion: lifecycle_status draft→active on PR #183 squash-merge 60fd0233 (S-17.03 DELIVERED/MERGED 2026-06-11; D-547). /factory-lock + /factory-unlock skills + factory-health + factory-worktree-health three-state lock status SHIPPED; 26 bats; CI run 27343001859 all-green; issue #170 CLOSED; E-17 3/3 COMPLETE. [Prior: 2026-06-11 (v1.1) — Boundary-semantics staleness sync (product-owner; S-17.03 adversary O-1; issue #170; BC-4.13.001 v1.3 boundary correction). PC3 foreign-held blocking condition corrected from `now ≤ expires_at` to `now < expires_at`; PC7 two HELD-display conditions corrected from `now ≤ expires_at` to `now < expires_at`; EC-002 explicit operator added `now >= expires_at` for expired/proceed path. Outcomes unchanged. [Prior: 2026-06-10 (v1.0) — Initial authoring (product-owner; brownfield-backfill issue #170; ADR-025 v1.2 D4/D5/D7/D8 deliverables). /factory-lock acquire (CAS push), /factory-unlock release + force-steal, factory-health lock status, factory-worktree-health lock status. lifecycle_status: draft (POL-14 auto-promotion on implementing PR merge).]]"
---

# BC-6.23.001: /factory-lock MUST acquire the factory_lock via fetch-then-CAS push (emitting factory.lock.acquired), /factory-unlock MUST release a self-held lock or force-release any lock (emitting factory.lock.released or factory.lock.stolen), and /factory-health and /factory-worktree-health MUST display three-state lock status

## Description

The `/factory-lock` skill provides explicit cooperative lock acquisition for the
factory-artifacts orphan branch. It performs `git fetch origin factory-artifacts`, reads the
just-fetched STATE.md for an existing lock, and — if the factory is unlocked or the lock is
expired — delegates to `state-manager` to write the `factory_lock` block and push using
fetch-then-`--force-with-lease` CAS (the same CAS primitive as BC-5.40.001 PC5). On CAS push
rejection, the acquire fails with an actionable error naming the TOCTOU acquire-race (CWE-367).
On success, a `factory.lock.acquired` event is emitted.

The `/factory-unlock` skill releases a self-held lock (emitting `factory.lock.released`) or,
with `--force`, releases any foreign lock (emitting `factory.lock.stolen` audit event naming
both parties). Without `--force`, attempting to release a foreign lock exits with an error and
does NOT modify STATE.md.

The `/factory-health` and `/factory-worktree-health` skills display the three-state lock status
(FREE / HELD by this session / HELD by other developer) from the local STATE.md read.

This BC covers ADR-025 Decisions 5 (Path B), 6, 8, and deliverables D4, D5, D7, D8.

## Preconditions

### `/factory-lock` acquire preconditions

1. The user has explicitly run `/factory-lock`. There is no auto-acquire on first write
   (ADR-025 Decision 6 rationale: explicit acquire makes session ownership visible and
   intentional).

2. A `git fetch origin factory-artifacts` succeeds before any lock check or write. The fetch
   MUST complete before reading STATE.md for the lock check — the local STATE.md after fetch
   is the source of truth for the acquire decision.

3. The `git config user.email` is set and returns a non-empty value. If not set, the skill
   exits with an actionable error: "git user.email not configured — cannot acquire factory
   lock." The lock MUST NOT be written with an empty holder.

### `/factory-unlock` release preconditions

4. The user has explicitly run `/factory-unlock` (plain) or `/factory-unlock --force`.

5. For plain `/factory-unlock`: `factory_lock.holder` in the just-fetched STATE.md MUST equal
   `git config user.email`. If the holder is another developer, the skill exits with an error
   without modifying STATE.md (see PC5 non-holder rejection).

6. For `/factory-unlock --force`: no identity check is performed. Any developer may force-release
   any lock (including foreign locks). The audit event `factory.lock.stolen` MUST be emitted
   regardless of whether the force-release succeeds or fails.

## Postconditions

### PC1 — `/factory-lock` success: lock written and CAS push succeeds

When the acquire succeeds:
1. `state-manager` writes `factory_lock = { holder: <current_email>, locked_at: <now_iso8601>,
   expires_at: <now_iso8601 + 2700s> }` into STATE.md frontmatter.
2. `state-manager` commits and pushes using the CAS primitive:
   ```bash
   git -C .factory fetch origin factory-artifacts
   EXPECTED_SHA=$(git -C .factory rev-parse origin/factory-artifacts)
   git -C .factory push --force-with-lease=factory-artifacts:"${EXPECTED_SHA}" origin factory-artifacts
   ```
3. A `factory.lock.acquired` event is emitted to the SS-03 event pipeline with fields:
   `holder` (email), `locked_at` (ISO-8601), `expires_at` (ISO-8601).
4. The user sees a success message: `Factory lock acquired by <email>. Expires at <expires_at>.`

**Error variant:** `AcquireRaceRejected` (on CAS push non-fast-forward rejection — see PC2)

### PC2 — `/factory-lock` CAS rejection: acquire fails with actionable error

When the CAS push is rejected (non-zero exit from `git push --force-with-lease`), the acquire
MUST fail with the following error message (honoring CWE-367 honest TOCTOU statement):
```
Acquire failed — concurrent lock write detected. Fetch and retry.
(TOCTOU acquire-race CWE-367: another session wrote the lock between your fetch and your push.)
```
STATE.md is NOT modified in a partial state (the write was rejected by git). The user can
retry `/factory-lock` from the top of the acquire flow. No audit event is emitted for a
rejected acquire.

**Error variant:** `AcquireRaceRejected`

### PC3 — `/factory-lock` foreign lock held: refuse with refusal message

When the fetch reveals `factory_lock.holder` is set, `now < expires_at` (strictly future —
the lock has not yet expired), and `holder != current_git_email`, the `/factory-lock` skill
MUST exit without attempting the CAS push and MUST display the same refusal message format as
the guard (BC-4.13.001 PC1):
- Holder email
- `locked_at` timestamp
- `expires_at` timestamp
- `time_remaining` (human-readable duration)
- `/factory-unlock --force` command

**Error variant:** `ForeignLockHeld`

### PC4 — `/factory-unlock` self-release: lock cleared

When `factory_lock.holder == current_git_email` and the user runs `/factory-unlock`:
1. `state-manager` removes the `factory_lock` key entirely from STATE.md frontmatter.
2. `state-manager` commits and pushes (CAS push per BC-5.40.001 PC5).
3. A `factory.lock.released` event is emitted with fields: `holder` (email), `locked_at`,
   `released_at` (ISO-8601 of the release instant).
4. The user sees: `Factory lock released.`

**Error variant:** `UnlockCASRejected` (if the CAS push is rejected — same `CASPushRejected`
variant as BC-5.40.001 PC5; actionable error emitted; user retries)

### PC5 — `/factory-unlock` non-holder rejection (without `--force`)

When `factory_lock.holder != current_git_email` and the user runs plain `/factory-unlock`
(without `--force`):
1. The skill MUST exit with a non-zero status.
2. The skill MUST display an error: `Cannot unlock — factory is held by <holder_email>. Use
   /factory-unlock --force to force-release.`
3. STATE.md MUST NOT be modified.
4. No event is emitted.

**Error variant:** `NotLockHolder`

### PC6 — `/factory-unlock --force` break-glass: foreign lock cleared with audit event

When the user runs `/factory-unlock --force` (regardless of current holder):
1. `state-manager` removes the `factory_lock` key entirely from STATE.md frontmatter.
2. `state-manager` commits and pushes (CAS push per BC-5.40.001 PC5).
3. A `factory.lock.stolen` audit event MUST be emitted with ALL four required fields:
   - `stolen_by`: `current_git_email` (the developer who ran `/factory-unlock --force`)
   - `stolen_from`: `factory_lock.holder` at the time of the force-release (the original holder)
   - `holder_locked_at`: the original `locked_at` value from the lock block
   - `stolen_at`: ISO-8601 of the force-release instant
4. The `factory.lock.stolen` event is PERMANENT — it cannot be suppressed after emission.
5. The user sees: `Factory lock force-released. Audit event 'factory.lock.stolen' emitted.`

If the factory is already unlocked (no `factory_lock` block) when `/factory-unlock --force`
is run, the skill succeeds silently (no event; lock was already absent; exit 0).

**Error variant:** `ForceStealAudited` (for internal instrumentation; the event IS the audit)

### PC7 — `/factory-health` three-state lock status display

The `/factory-health` skill MUST display one of three lock status lines based on the local
STATE.md `factory_lock` block:
- `Factory lock: FREE` — `factory_lock` absent or expired (`now >= expires_at`)
- `Factory lock: HELD by this session (expires <expires_at>)` — `factory_lock.holder ==
  current_git_email` and `now < expires_at` (strictly future)
- `Factory lock: HELD by <holder_email> since <locked_at> (expires <expires_at>)` —
  `factory_lock.holder != current_git_email` and `now < expires_at` (strictly future)

The status is read from the LOCAL STATE.md (no fetch required for display — the lock state
visible to the local session is what matters for informational display).

**Error variant:** none (display-only; malformed block renders as `Factory lock: FREE (malformed
block — treated as unlocked)`)

### PC8 — `/factory-worktree-health` three-state lock status display

The `/factory-worktree-health` skill MUST display the same three-state lock status as PC7,
using the same display strings and the same local STATE.md read logic. No separate lock-check
implementation — same helper as `/factory-health`.

## Invariants

1. **Explicit acquire only — no auto-acquire**: The factory NEVER auto-acquires a lock on
   first write. A developer who does not run `/factory-lock` operates in the unlocked state
   (the guard reads absent `factory_lock` and passes). This is correct: the absence of a lock
   means no cross-session exclusivity protection is in force for this session.

2. **`factory.lock.stolen` is mandatory on force-release**: Emitting this event is NOT
   optional. If the event cannot be emitted (SS-03 sink unavailable), the force-release still
   proceeds (the lock MUST be cleared) but the emission failure MUST be logged.

3. **CAS push is the acquire's atomicity**: Two concurrent `/factory-lock` invocations that
   both see an unlocked STATE.md will race at the CAS push level. One succeeds; one gets
   `AcquireRaceRejected`. The losing party retries from the top (re-fetch + re-check). This
   closes the primary TOCTOU acquire-race (CWE-367) to a millisecond-scale window bounded by
   the fetch→push interval. The git-ref CAS future path (ADR-025 Decision 9) eliminates this
   window entirely; it is out of scope for this version.

4. **`/factory-unlock --force` on an already-unlocked factory is a no-op**: If `factory_lock`
   is absent when `/factory-unlock --force` is run, the skill exits 0 silently. No event
   is emitted (there is no holder to name in `stolen_from`).

5. **`state-manager` delegation**: Neither `/factory-lock` nor `/factory-unlock` writes
   STATE.md directly. They delegate to `state-manager` which performs the write and push.
   This preserves TD-VSDD-053 single-writer discipline.

6. **Fetch-before-check is mandatory**: The `/factory-lock` acquire MUST fetch before reading
   STATE.md. Checking a stale local STATE.md and then pushing would be a TOCTOU acquire-race
   of a different form (the fetch + CAS push is the fix). The fetch also pulls any TTL renewal
   commits from the current holder, ensuring the lock check sees the most recent `expires_at`.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | `/factory-lock` when factory already locked by self (re-lock) | Treat as already acquired: display "Factory lock already held by this session." No STATE.md write, no new event. Exit 0. |
| EC-002 | `/factory-lock` when lock is expired (held by other developer but `now >= expires_at`) | Treat as unlocked; proceed with acquire (same path as absent lock). The expiry test is `now >= expires_at`; at the exact boundary `now == expires_at` the lock is treated as just-expired (not blocking). The CAS push is the safety net. |
| EC-003 | `/factory-unlock` when factory is already unlocked | Exit 0 silently. No error. No event. |
| EC-004 | `/factory-lock` CAS push rejected (concurrent acquire race) | `AcquireRaceRejected` error; user retries. No partial STATE.md write. |
| EC-005 | `/factory-unlock --force` when no lock is held | No-op; exit 0; no event (nothing to steal). |
| EC-006 | `git fetch origin factory-artifacts` fails during `/factory-lock` | Skill exits non-zero: "Fetch failed before lock check. Cannot acquire safely." No lock written. |
| EC-007 | `git config user.email` not set during `/factory-lock` | Skill exits non-zero: "git user.email not configured." No lock written. |
| EC-008 | `factory.lock.stolen` event emission fails (SS-03 sink down) | Force-release proceeds (lock cleared); emission failure logged as `host::log_warn`; no abort. Invariant 2. |
| EC-009 | Locked STATE.md pushed from `/factory-lock`; guard blocks a concurrent Edit before next fetch | Guard reads local STATE.md (may be stale by one commit); the fetched copy on the blocked session's machine may not yet reflect the new lock. The fetch on the locked session's `/factory-lock` pushes the lock commit; the blocked session's guard will see it on its next fetch (state-burst fetch cycle). This is the cooperative model's expected convergence window. |
| EC-010 | `/factory-unlock --force` run against a self-held lock | Force-release proceeds; `factory.lock.stolen` MUST NOT be emitted (stolen_by == stolen_from — no meaningful audit event). Instead, emit `factory.lock.released` (PC4 path is the correct route; force-release on self-held is equivalent to plain unlock). |

## Canonical Test Vectors

| Test # | Precondition | Command | Expected Result |
|--------|-------------|---------|----------------|
| T-1 | Factory unlocked (absent); `user.email = dev@x.com` | `/factory-lock` | Lock written; `expires_at = now+45min`; `factory.lock.acquired` emitted; success message |
| T-2 | Factory locked by `other@x.com`; not expired | `/factory-lock` (from `dev@x.com`) | `ForeignLockHeld` error; 5-field refusal message; no STATE.md write |
| T-3 | Factory locked by `dev@x.com` (self) | `/factory-lock` | "Already held by this session"; no write; exit 0 |
| T-4 | Two concurrent `/factory-lock` from `dev-a@x.com` and `dev-b@x.com` (both see unlocked) | Both simultaneously | One succeeds (CAS push wins); one gets `AcquireRaceRejected`; loser retries |
| T-5 | Factory locked by self (`dev@x.com`) | `/factory-unlock` | Lock cleared; `factory.lock.released` with `released_at`; success |
| T-6 | Factory locked by `other@x.com` | `/factory-unlock` (from `dev@x.com`) | `NotLockHolder` error; "use --force"; no STATE.md write |
| T-7 | Factory locked by `other@x.com` (5 min remaining) | `/factory-unlock --force` (from `dev@x.com`) | Lock cleared; `factory.lock.stolen` with stolen_by/stolen_from/holder_locked_at/stolen_at; audit message displayed |
| T-8 | Factory unlocked | `/factory-health` | `Factory lock: FREE` |
| T-9 | Factory locked by self; `expires_at = now+37min` | `/factory-health` | `Factory lock: HELD by this session (expires <expires_at>)` |
| T-10 | Factory locked by `other@x.com`; `locked_at = T`; `expires_at = T+45min` | `/factory-health` | `Factory lock: HELD by other@x.com since <locked_at> (expires <expires_at>)` |

## SDK Grounding Evidence

**Grep 1 — `/factory-health` skill exists (D7 anchor):**
```
ls plugins/vsdd-factory/skills/factory-health/SKILL.md
```
Expected: file exists — confirms the skill path for D7 modification.

**Grep 2 — `/factory-worktree-health` skill exists (D8 anchor):**
```
ls plugins/vsdd-factory/skills/factory-worktree-health/SKILL.md
```
Expected: file exists — confirms the skill path for D8 modification.

**Grep 3 — existing `factory.lock` event namespace absence (new events):**
```
grep -rn "factory\.lock\." plugins/vsdd-factory/ crates/
```
Expected: no hits (confirms `factory.lock.acquired`, `factory.lock.released`,
`factory.lock.stolen` are new events; no naming conflicts).

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| (unit-test) | Successful acquire writes well-formed `factory_lock` block | Unit test: mock `state-manager` write; assert schema correctness |
| (unit-test) | CAS acquisition rejection emits `AcquireRaceRejected` error | Unit test: mock `--force-with-lease` rejection; assert error message content |
| (unit-test) | Force-release emits `factory.lock.stolen` with all 4 fields | Unit test: mock state-manager + SS-03; assert event fields |
| (unit-test) | Plain unlock of foreign lock is rejected with `NotLockHolder` | Unit test: `holder != current_email`; no `--force`; assert exit non-zero + error message |
| (unit-test) | `/factory-health` displays three-state correctly | Unit test: three STATE.md variants; assert display strings |
| (bats) | Bats integration: acquire + release round-trip (D9) | `/factory-lock` then `/factory-unlock` on test repo |
| (bats) | Bats integration: force-release emits audit event (D9) | `/factory-lock` as dev-a; `/factory-unlock --force` as dev-b; assert `factory.lock.stolen` event |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-031 |
| Capability Anchor Justification | CAP-031 ("Enforce single-writer cross-session exclusivity on factory-artifacts state") per capabilities.md §CAP-031 — this BC defines the explicit acquire and release skills (`/factory-lock`, `/factory-unlock`) that are the user-facing entry points for CAP-031 lock management, plus the lock status display in health skills. The skills are the mechanism by which developers voluntarily opt into the cross-session exclusivity protection CAP-031 provides. |
| L2 Domain Invariants | none (operational infrastructure, not L2 domain spec) |
| Architecture Module | `plugins/vsdd-factory/skills/factory-lock/SKILL.md` (new D4); `plugins/vsdd-factory/skills/factory-unlock/SKILL.md` (new D5); `plugins/vsdd-factory/skills/factory-health/SKILL.md` (amend D7); `plugins/vsdd-factory/skills/factory-worktree-health/SKILL.md` (amend D8); SS-03 event pipeline (audit events) |
| Stories | TBD (v1.0-brownfield-backfill issue #170 decomposition pending) |
| ADR Reference | ADR-025 v1.2 (Decisions 5 Path B, 6, 8 and deliverables D4, D5, D7, D8) |

## Related BCs

- BC-4.13.001 — composes with (the guard enforces the lock this skill sets; PC3 refusal message format matches BC-4.13.001 PC1 refusal message)
- BC-5.40.001 — depends on (this skill delegates to state-manager per that BC's schema and CAS push spec; PC1 acquire produces the frontmatter that BC-5.40.001 PC1 specifies)

## Architecture Anchors

- `plugins/vsdd-factory/skills/factory-lock/SKILL.md` — new skill (to be created; D4)
- `plugins/vsdd-factory/skills/factory-unlock/SKILL.md` — new skill (to be created; D5)
- `plugins/vsdd-factory/skills/factory-health/SKILL.md` — amend to add lock status display (D7)
- `plugins/vsdd-factory/skills/factory-worktree-health/SKILL.md` — amend to add lock status display (D8)
- `.factory/specs/architecture/decisions/ADR-025-single-writer-factory-locklease-prevent-concurrent-session-races-on-factory-artifacts-orphan-branch.md` — authoritative design

## Story Anchor

TBD — implementing story to be decomposed from issue #170, v1.0-brownfield-backfill cycle.

## VP Anchors

TBD — VP IDs to be assigned after VP authoring pass.

## Changelog

| Version | Date | Description |
|---------|------|-------------|
| 1.2 | 2026-06-11 | POL-14 auto-promotion: lifecycle_status draft→active on PR #183 squash-merge 60fd0233 (state-manager; D-547; S-17.03 DELIVERED/MERGED 2026-06-11). /factory-lock + /factory-unlock skills + factory-health + factory-worktree-health three-state lock status SHIPPED; 26 bats; CI run 27343001859 all-green; issue #170 CLOSED; E-17 3/3 COMPLETE. No content changes. |
| 1.1 | 2026-06-11 | Boundary-semantics staleness sync (product-owner; S-17.03 adversary O-1; issue #170; BC-4.13.001-v1.3 boundary correction). PC3 foreign-held blocking condition: `now ≤ expires_at` → `now < expires_at` (strictly future). PC7 HELD display conditions (×2): `now ≤ expires_at` → `now < expires_at`. EC-002 expired/proceed path: explicit `now >= expires_at` operator added (boundary `now == expires_at` → expired/proceed, not blocking). FREE display string: added `(now >= expires_at)` clarification. All outcomes unchanged — foreign-unexpired → refuse; expired → proceed. Syncs to BC-4.13.001 v1.3 canonical boundary semantics. |
| 1.0 | 2026-06-10 | Initial authoring (product-owner; brownfield-backfill issue #170; ADR-025 v1.2 D4/D5/D7/D8 deliverables). /factory-lock CAS acquire (PC1 success, PC2 CAS rejection, PC3 foreign lock); /factory-unlock self-release (PC4), non-holder rejection (PC5), force-release + audit (PC6); /factory-health (PC7) + /factory-worktree-health (PC8) three-state status. 6 error variants: AcquireRaceRejected, ForeignLockHeld, CASPushRejected, NotLockHolder, ForceStealAudited, UnlockCASRejected. 10 edge cases EC-001..EC-010. 10 canonical test vectors T-1..T-10. CAP-031 registered same burst. lifecycle_status: draft. |
