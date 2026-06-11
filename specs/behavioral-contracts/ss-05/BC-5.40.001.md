---
document_type: behavioral-contract
level: L3
version: "1.0"
status: draft
producer: product-owner
timestamp: 2026-06-10T00:00:00Z
phase: brownfield-backfill
cycle: v1.0-brownfield-backfill
inputs:
  - .factory/specs/architecture/decisions/ADR-025-single-writer-factory-locklease-prevent-concurrent-session-races-on-factory-artifacts-orphan-branch.md
  - .factory/specs/behavioral-contracts/ss-05/BC-5.39.009.md
  - .factory/specs/behavioral-contracts/BC-INDEX.md
  - plugins/vsdd-factory/skills/state-burst/SKILL.md
input-hash: "[pending-recompute]"
traces_to: .factory/specs/architecture/decisions/ADR-025-single-writer-factory-locklease-prevent-concurrent-session-races-on-factory-artifacts-orphan-branch.md
origin: brownfield
subsystem: "SS-05"
capability: "CAP-031"
lifecycle_status: draft
introduced: v1.0-brownfield-backfill
modified: []
deprecated: null
deprecated_by: null
replacement: null
retired: null
removed: null
removal_reason: null
bc_id: BC-5.40.001
section: "5.40"
last_amended: "2026-06-10 (v1.0) — Initial authoring (product-owner; brownfield-backfill issue #170; ADR-025 v1.2 D3/D6 deliverables). factory_lock STATE.md frontmatter schema, TTL auto-expiry, mid-burst renewal, state-burst CAS push fix. lifecycle_status: draft (POL-14 auto-promotion on implementing PR merge)."
---

# BC-5.40.001: STATE.md MUST carry a factory_lock frontmatter block (holder, locked_at, expires_at) as the authoritative lock state, state-manager MUST be its sole writer, TTL auto-expiry MUST be enforced at 45 minutes, state-manager MUST renew expires_at at each intermediate burst commit, and state-burst MUST use fetch-then-force-with-lease CAS push

## Description

The `factory_lock` block in STATE.md frontmatter is the authoritative cross-session lock state
for the factory-artifacts orphan branch. It carries three fields: `holder` (git user email of
the locking session), `locked_at` (ISO-8601 acquisition timestamp), and `expires_at` (ISO-8601
expiry timestamp = `locked_at + 45min` initially; refreshed to `now + 45min` on each intermediate
burst commit). Absent or null block = unlocked. Malformed block (missing required fields or
unparseable values) = treated as unlocked (fail-open, consistent with BC-4.13.001 PC4). The
`state-manager` agent is the sole writer of this block (TD-VSDD-053 single-writer discipline).

This BC also specifies the replacement of the blind `git push origin factory-artifacts` in
`skills/state-burst/SKILL.md` with a fetch-then-`--force-with-lease` CAS push (ADR-025
Decision 8, deliverable D6). This is a standalone complementary mitigation: even without the
WASM guard (BC-4.13.001), the CAS push converts concurrent pushes from silent clobbers to
detected collisions. The `/factory-lock` acquire path uses the same CAS primitive (BC-6.23.001).

This BC covers ADR-025 Decisions 2, 3, 5, 8, and 10, and deliverables D3 and D6.

## Preconditions

### STATE.md structure

1. STATE.md MUST contain a YAML frontmatter region (bounded by `---\n` delimiters at lines 1
   and N). The `factory_lock` block lives within this frontmatter region as a YAML mapping key.
   The three required sub-fields are `holder`, `locked_at`, and `expires_at`.

2. `state-manager` is the sole agent permitted to write STATE.md (TD-VSDD-053). No other agent,
   skill, or tool may modify the `factory_lock` block directly.

### Lock state schema

3. When a lock is in force, the `factory_lock` block MUST have the following canonical YAML form
   in STATE.md frontmatter:
   ```yaml
   factory_lock:
     holder: "developer@example.com"   # git config user.email of the locking session
     locked_at: "2026-06-10T14:00:00Z" # ISO-8601; when /factory-lock was run
     expires_at: "2026-06-10T14:45:00Z" # ISO-8601; = locked_at + 45min (refreshed on mid-burst commits)
   ```
   All three fields MUST be present when a lock is in force. The `holder` field MUST be the
   exact string returned by `git config user.email` at acquire time. `locked_at` and
   `expires_at` MUST be ISO-8601 UTC timestamps (format: `YYYY-MM-DDTHH:MM:SSZ`).

4. When the factory is unlocked, the `factory_lock` block MUST be absent from STATE.md
   frontmatter entirely (no null placeholder, no empty mapping). `state-manager` removes the
   block on `/factory-unlock` (BC-6.23.001 PC3/PC4).

### state-burst CAS push precondition

5. Before the CAS push, `state-manager` MUST perform a `git fetch origin factory-artifacts`
   to synchronize the local `factory-artifacts` ref with the remote. The expected SHA is
   captured immediately after the fetch: `EXPECTED_SHA=$(git -C .factory rev-parse
   origin/factory-artifacts)`.

## Postconditions

### PC1 — Factory lock schema correctness

When `state-manager` writes a lock (via the `/factory-lock` acquire skill — BC-6.23.001 PC1),
the resulting STATE.md frontmatter MUST contain a well-formed `factory_lock` block with:
- `holder`: the exact output of `git config user.email` at acquire time (no trimming beyond
  trailing newline removal)
- `locked_at`: ISO-8601 UTC timestamp of the acquire instant (precision: seconds)
- `expires_at`: `locked_at + 45 minutes` (exactly 2700 seconds added to `locked_at`)

**Error variant:** `SchemaViolation` (if any field is missing or malformed after write — detected
by BC-4.13.001's PC4 malformed-block fail-open path)

### PC2 — Unlock clears the block

When `state-manager` writes an unlock (via `/factory-unlock` — BC-6.23.001 PC3/PC4), the
`factory_lock` key MUST be absent from STATE.md frontmatter entirely after the write. A null
value (`factory_lock: null`) is NOT an acceptable unlock representation — the key must be
removed.

**Error variant:** `StaleNullBlock` (if key remains as null post-unlock)

### PC3 — TTL auto-expiry: guard treats expired lock as absent

The `verify-factory-lock` guard (BC-4.13.001 PC2) checks `now > factory_lock.expires_at` at
invocation time. When true, the lock is treated as absent and the operation proceeds. The expired
lock block remains in STATE.md frontmatter until the next `state-manager` write (which either
refreshes it if the session is still active, or removes it at explicit unlock). This "stale
expired block" state is safe: the guard's TTL check treats it as unlocked, and the next
`state-manager` commit cleans it up.

**Failure mode — long burst TTL self-eviction (ADR-025 Decision 5):**
A burst longer than 45 minutes between intermediate commits self-evicts the lock: `now >
expires_at` becomes true mid-burst, allowing another developer to acquire. Mitigation:
mid-burst renewal (PC4). Residual risk: fencing token absent — see Invariant 4.

### PC4 — Mid-burst TTL renewal

At EVERY intermediate `state-manager` commit within a burst (not only at burst-close),
`state-manager` MUST refresh `factory_lock.expires_at = now + 45 minutes` alongside the
commit payload. This resets the TTL clock to 45 minutes from each intermediate write rather
than from the original `locked_at`. The renewal MUST be atomic with the commit (same commit
that advances any other STATE.md fields). No separate background timer or subprocess is
required — the burst's own commit cadence is the renewal heartbeat.

Concretely: the `state-manager` Commit-E sequence template (STATE.md final-advance commit)
MUST include the `expires_at` refresh. Any intermediate Commit-A/B/C/D that touches STATE.md
must also refresh `expires_at` if a lock is held.

**Error variant:** `RenewalMissed` (if a burst completes without refreshing `expires_at` while
a lock is held — detectable by comparing old and new `expires_at` values post-commit)

### PC5 — state-burst fetch-then-CAS push

The `state-burst` skill MUST replace its blind `git push origin factory-artifacts` with the
following fetch-then-CAS push sequence:

```bash
git -C .factory fetch origin factory-artifacts
EXPECTED_SHA=$(git -C .factory rev-parse origin/factory-artifacts)
git -C .factory push --force-with-lease=factory-artifacts:"${EXPECTED_SHA}" origin factory-artifacts
```

On non-fast-forward rejection (push fails with a non-zero exit code indicating
`--force-with-lease` check failed), `state-burst` MUST:
1. Exit with a non-zero status.
2. Emit a human-readable error: "state-burst CAS push failed — concurrent write detected.
   Fetch origin/factory-artifacts and retry."
3. NOT silently clobber the remote state.

This is independently valuable whether or not the WASM guard is deployed: a concurrent push
from any source (another developer, a self-vs-self two-session case, a guard-crash fail-open
scenario) causes a detected collision rather than a silent clobber.

**Error variant:** `CASPushRejected` (non-fast-forward rejection from `--force-with-lease`)

### PC6 — Single-developer path: zero added friction

A developer who has run `/factory-lock` and holds the lock sees no friction during normal
`state-manager` burst operations. The guard reads STATE.md locally (no network calls in the
guard hot path per ADR-025 Decision 10), compares identities, and returns `Continue`
immediately on self-held lock (BC-4.13.001 PC3). The only observable changes are:
- The `factory_lock` block is present in STATE.md during the session.
- The `expires_at` field updates on each intermediate commit (renewal).
- `/factory-health` shows `Factory lock: HELD by this session (expires <time>)` (BC-6.23.001).

A developer who has NOT run `/factory-lock` is in the same position as today: the guard reads
`factory_lock: null` (absent) and passes all checks. The lock is opt-in.

## Invariants

1. **`state-manager` is the sole writer**: No other agent, skill, or tool writes the
   `factory_lock` block. The `/factory-lock` and `/factory-unlock` skills DELEGATE to
   `state-manager` to write STATE.md (they do not write directly). This preserves TD-VSDD-053.

2. **Default TTL is 45 minutes (2700 seconds)**: The TTL value is not configurable by users.
   45 minutes is the production-grade default (ADR-025 Decision 5 rationale: midpoint of
   2–5× expected burst duration range; expected burst duration ~10 minutes).

3. **`expires_at` is always = `now + 2700s` at the moment of write**: Whether the write is the
   initial acquire, a mid-burst renewal, or any other state-manager write that refreshes the
   lock, `expires_at` is computed as the wall-clock instant of the commit + 2700 seconds.
   The `locked_at` field is immutable after the initial acquire — it records when the session
   started, not the last renewal.

4. **Fencing token absent — residual risk accepted**: The current design has no fencing token
   (monotonically increasing value that storage can check to reject stale-holder writes). If
   the TTL expires between two intermediate commits under extreme network delay or WASM fuel
   exhaustion, a second developer could acquire between renewals and both parties proceed.
   This residual risk is explicitly attributed to the Decision 9 git-ref-CAS future path
   (ADR-025 §Decision 9). Under the cooperative threat model, this is accepted.

5. **`--force-with-lease` is already permitted by `verify-git-push.sh`**: The bash hook
   `hooks/verify-git-push.sh` only blocks raw `--force`; `--force-with-lease` is permitted
   (ADR-025 Decision 8 source verification). The CAS push change requires no modifications
   to the existing push-hook allow-list.

6. **Malformed block = unlocked (fail-open)**: The guard (BC-4.13.001 PC4) and this BC both
   treat any malformed `factory_lock` block as unlocked. `state-manager` MUST write
   well-formed blocks; however, if STATE.md is corrupted (e.g., manual edit), the system
   fails open rather than wedging.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | `state-manager` crash mid-burst before writing `expires_at` renewal | Existing lock remains with old `expires_at`; if old expiry elapses, lock auto-expires; next developer can acquire; crashed session leaves a stale lock (max 45 min wedge) |
| EC-002 | `state-burst` CAS push rejected (non-fast-forward) | `state-burst` exits non-zero; actionable error emitted; `.factory/` commit already exists locally; developer must `git -C .factory fetch origin factory-artifacts` and re-run state-burst after resolving the divergence |
| EC-003 | `git fetch origin factory-artifacts` fails (network error) in state-burst CAS path | `state-burst` exits non-zero; actionable error: "fetch failed before CAS push"; do NOT proceed with push using potentially stale EXPECTED_SHA |
| EC-004 | `EXPECTED_SHA` fetch succeeds but remote advances before push (true concurrent write) | `--force-with-lease` rejects the push (non-fast-forward); `CASPushRejected` error; safe — no silent clobber |
| EC-005 | Lock held by self; `expires_at` within 5 minutes of current time | Mid-burst renewal MUST still refresh `expires_at = now + 2700s` on the next commit; the approaching expiry does not trigger any special behavior |
| EC-006 | STATE.md `factory_lock` block has `holder`, `locked_at`, `expires_at` but additional unknown fields | Unknown fields are ignored (fail-open to unlocked is NOT triggered; the block is valid if the three required fields are present and parseable) |
| EC-007 | Factory is unlocked (`factory_lock` absent); `state-burst` CAS push proceeds | Fetch + CAS push proceeds normally; if remote has advanced (another developer pushed), `CASPushRejected` error; developer fetches and retries |
| EC-008 | `git -C .factory rev-parse origin/factory-artifacts` returns a SHA that does not exist locally after fetch | This indicates a fetch/parse race; `state-burst` MUST re-fetch before retrying; emit `CASPushRejected` with "stale SHA after fetch" detail |
| EC-009 | Long burst: 3 intermediate commits, each refreshing `expires_at`; total burst duration = 70 min | Lock remains valid throughout: each commit resets `expires_at = now + 45min`; at burst-end, `expires_at` is 45 minutes in the future from the last commit |

## Canonical Test Vectors

| Scenario | STATE.md `factory_lock` before | Operation | STATE.md `factory_lock` after | Result |
|----------|-------------------------------|-----------|-------------------------------|--------|
| Lock acquire | absent | `/factory-lock` (via state-manager) | `{holder: "dev@x.com", locked_at: T, expires_at: T+2700s}` | Block written; push succeeds |
| Unlock (self) | `{holder: "dev@x.com", ..., expires_at: T+1h}` | `/factory-unlock` (self) | absent | Block removed; push succeeds |
| Mid-burst renewal | `{holder: "dev@x.com", ..., expires_at: T}` | state-manager intermediate commit | `{holder: "dev@x.com", locked_at: T_orig, expires_at: now+2700s}` | `expires_at` refreshed; `locked_at` unchanged |
| CAS push: concurrent write | N/A | `state-burst` push; remote advanced | N/A | `CASPushRejected`; error emitted; no clobber |
| Expired lock cleanup | `{holder: "dev@x.com", ..., expires_at: T-1s}` | Guard check at any mutating tool | Guard: `HookResult::Continue` (expired); block remains in STATE.md until next state-manager write | Safe pass-through |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| (unit-test) | `factory_lock` block written with correct schema on acquire | Rust unit test: call acquire; assert frontmatter has all three fields; assert `expires_at = locked_at + 2700s` |
| (unit-test) | Unlock removes `factory_lock` key entirely (not null) | Rust unit test: unlock; assert key absent from frontmatter |
| (unit-test) | Mid-burst renewal updates `expires_at` but preserves `locked_at` | Rust unit test: intermediate commit; assert `expires_at` advanced; `locked_at` unchanged |
| (unit-test) | `state-burst` CAS push rejects concurrent write | Rust unit test: mock remote advancing after fetch; assert non-zero exit + error message |
| (bats) | Bats integration: lock blocked when held by other developer | D9 T-2 (BC-4.13.001 canonical test vectors) |
| (bats) | Bats integration: acquire CAS rejection on concurrent acquire | D9 T-10 (BC-6.23.001 canonical test vectors) |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-031 |
| Capability Anchor Justification | CAP-031 ("Enforce single-writer cross-session exclusivity on factory-artifacts state") per capabilities.md §CAP-031 — this BC defines the authoritative lock state data structure and the state-manager write discipline that underpins the entire CAP-031 mechanism. The `factory_lock` frontmatter schema is what the guard (BC-4.13.001) reads and what the skills (BC-6.23.001) manage; without a correct schema and renewal discipline, the guard cannot enforce exclusivity. |
| L2 Domain Invariants | none (operational infrastructure invariant, not L2 domain spec) |
| Architecture Module | `.factory/STATE.md` (frontmatter schema); `plugins/vsdd-factory/skills/state-burst/SKILL.md` (CAS push replacement D6); `agents/state-manager.md` (sole writer discipline); `plugins/vsdd-factory/hooks/verify-git-push.sh` (allows `--force-with-lease` — no change required) |
| Stories | TBD (v1.0-brownfield-backfill issue #170 decomposition pending) |
| ADR Reference | ADR-025 v1.2 (Decisions 2, 3, 5, 8, 10 and deliverables D3, D6) |

## Related BCs

- BC-4.13.001 — depends on (the guard reads the schema defined here; PC4 malformed-block semantics mirror this BC's Invariant 6)
- BC-6.23.001 — composes with (the skills write the schema defined here; acquire/unlock operations produce the pre/postconditions defined in this BC)
- BC-5.39.009 — sibling (STATE.md mutation discipline; state-manager Commit-E cadence; the renewal heartbeat for `expires_at` follows the same state-manager burst commit discipline)

## Architecture Anchors

- `plugins/vsdd-factory/skills/state-burst/SKILL.md` — `git push origin factory-artifacts` (blind push; must be replaced with fetch-then-CAS; D6 target)
- `plugins/vsdd-factory/hooks/verify-git-push.sh` — allows `--force-with-lease` (no changes required; confirmed by ADR-025 §Decision 8)
- `.factory/STATE.md` — frontmatter region; `factory_lock:` block (new schema field)
- `.factory/specs/architecture/decisions/ADR-025-single-writer-factory-locklease-prevent-concurrent-session-races-on-factory-artifacts-orphan-branch.md` — authoritative design

## Story Anchor

TBD — implementing story to be decomposed from issue #170, v1.0-brownfield-backfill cycle.

## VP Anchors

TBD — VP IDs to be assigned after VP authoring pass.

## Changelog

| Version | Date | Description |
|---------|------|-------------|
| 1.0 | 2026-06-10 | Initial authoring (product-owner; brownfield-backfill issue #170; ADR-025 v1.2 D3/D6 deliverables). factory_lock STATE.md schema (holder, locked_at, expires_at); TTL=45min; mid-burst renewal; state-burst CAS push fix; fail-open on malformed; sole-writer invariant. PC1 (schema correctness), PC2 (unlock clears block), PC3 (TTL expiry guard), PC4 (mid-burst renewal), PC5 (CAS push), PC6 (single-dev zero friction). 4 error variants: SchemaViolation, StaleNullBlock, RenewalMissed, CASPushRejected. 9 edge cases EC-001..EC-009. CAP-031 registered same burst. lifecycle_status: draft. |
