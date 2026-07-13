---
document_type: behavioral-contract
level: L3
version: "1.2"
status: active
producer: product-owner
timestamp: 2026-06-11T00:00:00Z
phase: brownfield-backfill
cycle: v1.0-brownfield-backfill
inputs:
  - .factory/specs/architecture/decisions/ADR-025-single-writer-factory-locklease-prevent-concurrent-session-races-on-factory-artifacts-orphan-branch.md
  - .factory/specs/behavioral-contracts/ss-05/BC-5.39.009.md
  - .factory/specs/behavioral-contracts/BC-INDEX.md
  - plugins/vsdd-factory/skills/state-burst/SKILL.md
input-hash: "688e195"
traces_to: .factory/specs/architecture/decisions/ADR-025-single-writer-factory-locklease-prevent-concurrent-session-races-on-factory-artifacts-orphan-branch.md
origin: brownfield
extracted_from: null
subsystem: "SS-05"
capability: "CAP-031"
lifecycle_status: active
introduced: v1.0-brownfield-backfill
modified:
  - "2026-06-11 (v1.1)"
  - "2026-07-13 (v1.2)"
deprecated: null
deprecated_by: null
replacement: null
retired: null
removed: null
removal_reason: null
bc_id: BC-5.40.001
section: "5.40"
last_amended: "2026-07-13 (v1.2) — S-19.08 Spec-First amendment (human-authorized; D-826/D-835): Precondition 6 added (verify-state-timestamp-refresh read capability: max_bytes=262144 (256 KiB); frontmatter-only via factory_lock_parse::extract_frontmatter (crates/factory-lock-parse/; S-19.02 PR #610; reuse-not-duplicate); cap mirrors BC-4.13.001 Phase-A Precondition 3 + ADR-025 §Decision 12 §12.5 parity; fail-open on OutputTooLarge per ADR-025 Decision 7). Invariant 7 added: frontmatter-only mandate for verify-state-timestamp-refresh (extract_frontmatter exclusive; mirrors BC-4.13.001 Invariant 9). Invariant 8 added: soft-warn threshold adjudication — verify-state-timestamp-refresh reads STATE.md in full → BC-4.13.001 Invariant 10 scope confirmed → state_md_approaching_cap MUST emit at bytes_read > 200000 AND ≤ 262144 (boundary table parity with BC-4.13.001 Invariant 10). EC-010 added (STATE.md exceeds 262144 bytes: OutputTooLarge → guard fail-open). Verification Properties updated: unit-test rows T-001..T-007 added; VP-096 back-cited (extract_frontmatter reuse). Story Anchor updated: S-17.01 + S-19.08. Traceability Stories updated: S-17.01 + S-19.08. Architecture Anchors: crates/factory-lock-parse/ added. [Prior: 2026-06-11 (v1.1) — POL-14 auto-promotion: lifecycle_status draft→active on PR #181 squash-merge c64b46d2 (S-17.01 merged); status draft→active; D-544 codified. [Prior: 2026-06-10 (v1.0) — Initial authoring (product-owner; brownfield-backfill issue #170; ADR-025 v1.2 D3/D6 deliverables). factory_lock STATE.md frontmatter schema, TTL auto-expiry, mid-burst renewal, state-burst CAS push fix.]]"
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

### verify-state-timestamp-refresh read capability

6. **Phase-A (active spec; implemented by S-19.08):** The `verify-state-timestamp-refresh`
   guard reads `.factory/STATE.md` via `host::read_file` with `max_bytes = 262144` (256 KiB).
   The plugin-side compile-time cap MUST be `STATE_MD_MAX_BYTES = 262144`. Before any YAML
   field extraction (`timestamp:` or `factory_lock.expires_at`), the guard MUST call
   `factory_lock_parse::extract_frontmatter(bytes)` (available in `crates/factory-lock-parse/`
   via S-19.02 PR #610 — reuse, not reimplementation; see Invariant 7) and operate exclusively
   on the returned frontmatter slice. The 256 KiB cap is established by ADR-025 §Decision 12
   §12.5 parity with `verify-factory-lock` (BC-4.13.001 Phase-A Precondition 3); this cap is
   above the worst-case observed STATE.md size (<200 KiB under 500-line compaction discipline
   per D-442(e)), giving ≥25% headroom. When `host::read_file` returns `OutputTooLarge` (file
   exceeds cap), the guard MUST fall back to `HookResult::Continue` (fail-open per ADR-025
   Decision 7; see EC-010). The soft-warn threshold contract (Invariant 8) MUST be observed on
   every successful read.

   **Defect context:** Without this Precondition, `read_file.rs::read_bounded()` checks
   `metadata.len()` BEFORE reading any bytes: when STATE.md exceeds the 64 KiB legacy cap
   (`STATE_MD_MAX_BYTES = 65536`), the host returns `OUTPUT_TOO_LARGE (-3)` before the guard
   reads a single byte, causing the timestamp-freshness gate to fail open silently on every
   PreToolUse dispatch to STATE.md — the same defect class as S-19.02 FINDING-1 for
   `verify-factory-lock`. D-826/D-835 confirm 3× production occurrences of this failure mode.

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

7. **`verify-state-timestamp-refresh` uses `extract_frontmatter` exclusively**: The guard MUST
   call `factory_lock_parse::extract_frontmatter(bytes)` on the byte slice returned by
   `host::read_file` before scanning for `timestamp:` or `factory_lock.expires_at`. The guard
   MUST operate only on the returned frontmatter slice and MUST NOT process the file body after
   the closing `---` delimiter. This mirrors BC-4.13.001 Invariant 9's frontmatter-only mandate
   applied to the `verify-state-timestamp-refresh` guard. The `extract_frontmatter` function is
   provided by `crates/factory-lock-parse/` (pure-core crate; S-19.02 PR #610;
   reuse-not-duplicate per CANONICAL PRINCIPLE Rule 4). Reimplementing the function in
   `crates/hook-plugins/verify-state-timestamp-refresh/` is a production blocker. When
   `extract_frontmatter` returns the full bytes (delimiter not found — fail-open behavior of
   the function), the guard applies its parse logic to the full returned slice without error.
   The `factory-lock-parse` crate is already a dependency of `verify-state-timestamp-refresh`
   (S-19.02 established the pattern); no new Cargo.toml dependencies are required.

8. **Soft-warn threshold for `verify-state-timestamp-refresh` (BC-4.13.001 Invariant 10
   adjudication)**: `soft_warn_threshold = 200000` bytes. **Adjudication:** BC-4.13.001
   Invariant 10 applies to "a hook that already reads STATE.md in full (i.e., calls
   `host::read_file` on `.factory/STATE.md`)." The `verify-state-timestamp-refresh` guard
   reads STATE.md on every PreToolUse Edit/Write/MultiEdit dispatch, placing it within
   Invariant 10's explicit scope. When a successful read observes
   `bytes_read > 200000 AND bytes_read ≤ cap_bytes (262144)`, the guard MUST emit a
   `state_md_approaching_cap` diagnostic event carrying `bytes_read: u64` and
   `cap_bytes: u64` (262144). This event is observability-only — it NEVER triggers a block
   or alters the `Continue`/`Block` verdict. The soft-warn range is
   `bytes_read ∈ (200000, 262144]` — inclusive at the cap boundary:

   | `bytes_read` | Outcome |
   |---|---|
   | ≤ 200000 | No warn emitted; normal read |
   | 200001 | `state_md_approaching_cap` emitted; read succeeds |
   | 262144 | `state_md_approaching_cap` emitted AND read succeeds — warn MUST fire at cap boundary |
   | 262145 | `OutputTooLarge` returned by host; soft-warn path not reached |

   This event requires zero new registry entries. The threshold is not a hard cap; it is a
   leading indicator for compaction scheduling (D-442(e)).

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
| EC-010 | `verify-state-timestamp-refresh` guard reads STATE.md exceeding `max_bytes = 262144` (256 KiB) | `host::read_file` returns `OutputTooLarge`; guard falls back to `HookResult::Continue` (fail-open per ADR-025 Decision 7); `StateReadError` warn emitted. The 262144-byte cap exceeds D-442(e) structural limits (≤200 KiB under 500-line compaction discipline); exceedance indicates either compaction overdue or anomalous STATE.md inflation. Timestamp-freshness gate is silently inert for this invocation. |

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
| (unit-test) | `STATE_MD_MAX_BYTES` constant equals 262144 in `verify-state-timestamp-refresh` | Rust unit test: `assert_eq!(STATE_MD_MAX_BYTES, 262144)` (S-19.08 T-001; AC-001) |
| (unit-test) | Guard reads STATE.md successfully when 64 KiB < file size < 256 KiB; detects stale timestamp and returns block intent | Rust unit test: 70 KiB fixture + stale `timestamp:` → `TimestampStale`; advanced timestamp → `Continue` (S-19.08 T-002/T-003; AC-002) |
| (unit-test) | `extract_frontmatter` wired before parse; body content excluded from parsed slice; no-delimiter fallback returns full content | Rust unit test: fixture with body after `---`; assert guard processes frontmatter only; delimiter-absent fixture → full content without error (S-19.08 T-004/T-005; AC-003) |
| (integration) | Zero `output_too_large` events emitted for `verify-state-timestamp-refresh` on 70 KiB STATE.md | Integration test: 70 KiB fixture; captured event stream asserts zero `internal.capability_denied reason=output_too_large` (S-19.08 T-006; AC-004) |
| (unit-test) | `state_md_approaching_cap` warn at bytes_read > 200000 ≤ 262144; no warn at ≤ 200000 (strict threshold); warn+read-success at cap-exact 262144; `StateReadError`+zero-warn at 262145 | Rust unit tests A/B/C/D/E (S-19.08 T-007; AC-005) |
| VP-096 | `extract_frontmatter` purity — output byte-equals file prefix up to (excluding) the second `---` delimiter line (bytes 0..delimiter_start_offset); deterministic for any input | proptest (S-19.02; `crates/factory-lock-parse/tests/proptest_extract_frontmatter.rs`); applies to `verify-state-timestamp-refresh` Invariant 7 use by transitivity — reuse of same function, same correctness guarantee |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-031 |
| Capability Anchor Justification | CAP-031 ("Enforce single-writer cross-session exclusivity on factory-artifacts state") per capabilities.md §CAP-031 — this BC defines the authoritative lock state data structure and the state-manager write discipline that underpins the entire CAP-031 mechanism. The `factory_lock` frontmatter schema is what the guard (BC-4.13.001) reads and what the skills (BC-6.23.001) manage; without a correct schema and renewal discipline, the guard cannot enforce exclusivity. |
| L2 Domain Invariants | none (operational infrastructure invariant, not L2 domain spec) |
| Architecture Module | `.factory/STATE.md` (frontmatter schema); `plugins/vsdd-factory/skills/state-burst/SKILL.md` (CAS push replacement D6); `agents/state-manager.md` (sole writer discipline); `plugins/vsdd-factory/hooks/verify-git-push.sh` (allows `--force-with-lease` — no change required) |
| Stories | S-17.01 (initial implementation; PR #181 merged 2026-06-11; D-544; v1.0-brownfield-backfill); S-19.08 (`verify-state-timestamp-refresh` read-cap amendment; implements Precondition 6, Invariants 7 and 8; E-19 Wave-2; D-826/D-835) |
| ADR Reference | ADR-025 v1.2 (Decisions 2, 3, 5, 8, 10 and deliverables D3, D6) |

## Related BCs

- BC-4.13.001 — depends on (the guard reads the schema defined here; PC4 malformed-block semantics mirror this BC's Invariant 6)
- BC-6.23.001 — composes with (the skills write the schema defined here; acquire/unlock operations produce the pre/postconditions defined in this BC)
- BC-5.39.009 — sibling (STATE.md mutation discipline; state-manager Commit-E cadence; the renewal heartbeat for `expires_at` follows the same state-manager burst commit discipline)

## Architecture Anchors

- `plugins/vsdd-factory/skills/state-burst/SKILL.md` — `git push origin factory-artifacts` (blind push; must be replaced with fetch-then-CAS; D6 target)
- `plugins/vsdd-factory/hooks/verify-git-push.sh` — allows `--force-with-lease` (no changes required; confirmed by ADR-025 §Decision 8)
- `.factory/STATE.md` — frontmatter region; `factory_lock:` block (new schema field)
- `crates/factory-lock-parse/src/lib.rs` — `extract_frontmatter` pure-core function (S-19.02 PR #610; reuse by `verify-state-timestamp-refresh` per Invariant 7; no modifications permitted by this story)
- `crates/hook-plugins/verify-state-timestamp-refresh/src/lib.rs` — `STATE_MD_MAX_BYTES` constant (raise to 262144); `extract_frontmatter` call site; soft-warn emission (S-19.08 implementation target)
- `.factory/specs/architecture/decisions/ADR-025-single-writer-factory-locklease-prevent-concurrent-session-races-on-factory-artifacts-orphan-branch.md` — authoritative design (§Decision 12 §12.5 cap parity; §Decision 7 fail-open)

## Story Anchor

Dual-story anchor: S-17.01 (initial implementation; `factory_lock` schema + state-burst CAS push; PR #181 merged 2026-06-11; D-544; v1.0-brownfield-backfill); S-19.08 (`verify-state-timestamp-refresh` read-cap amendment; implements Precondition 6, Invariants 7 and 8; E-19 Wave-2; D-826/D-835 tracked defect).

## VP Anchors

- VP-096 — `extract_frontmatter` Purity — Output Byte-Equals File Prefix Up To (Excluding) the Second `---` Delimiter Line (bytes 0..delimiter_start_offset; opening `---\n` included); Deterministic for Any Input (proptest; S-19.02; `crates/factory-lock-parse/tests/proptest_extract_frontmatter.rs`); back-cited per Invariant 7 reuse obligation — `verify-state-timestamp-refresh` calls the same pure function; VP-096 covers its correctness by transitivity.
- (S-19.08 unit-test VPs) — VP IDs for `STATE_MD_MAX_BYTES = 262144` assertion, 70 KiB fixture guard-operational, `state_md_approaching_cap` boundary tests, and integration zero-`output_too_large` test to be assigned by state-manager after S-19.08 VP authoring pass.

## Changelog

| Version | Date | Description |
|---------|------|-------------|
| 1.2 | 2026-07-13 | S-19.08 Spec-First amendment (human-authorized; D-826/D-835): Precondition 6 added (`verify-state-timestamp-refresh` read capability: `max_bytes = 262144` (256 KiB); frontmatter-only via `factory_lock_parse::extract_frontmatter` (`crates/factory-lock-parse/`; S-19.02 PR #610; reuse-not-duplicate); cap rationale mirrors BC-4.13.001 Phase-A Precondition 3 + ADR-025 §Decision 12 §12.5 parity; fail-open on `OutputTooLarge` per ADR-025 Decision 7). Invariant 7 added: frontmatter-only mandate for `verify-state-timestamp-refresh` (`extract_frontmatter` exclusive; mirrors BC-4.13.001 Invariant 9). Invariant 8 added: soft-warn threshold adjudication — `verify-state-timestamp-refresh` reads STATE.md in full → Invariant 10 scope confirmed → `state_md_approaching_cap` MUST emit at `bytes_read > 200000 AND ≤ 262144` (boundary table parity with BC-4.13.001 Invariant 10). EC-010 added (STATE.md exceeds 262144 bytes: `OutputTooLarge` → guard fail-open). Verification Properties updated: unit-test rows T-001..T-007 added; VP-096 back-cited (extract_frontmatter reuse). Story Anchor updated: S-17.01 + S-19.08. Traceability Stories updated: S-17.01 + S-19.08. modified[] appended 2026-07-13 (v1.2). |
| 1.1 | 2026-06-11 | POL-14 auto-promotion (state-manager; D-544; PR #181 squash-merged c64b46d2 2026-06-11; S-17.01 MERGED; status draft→active; lifecycle_status draft→active; modified[] appended 2026-06-11 (v1.1)). No BC content changes. BC-INDEX v2.66→v2.67 (body row draft→active). |
| 1.0 | 2026-06-10 | Initial authoring (product-owner; brownfield-backfill issue #170; ADR-025 v1.2 D3/D6 deliverables). factory_lock STATE.md schema (holder, locked_at, expires_at); TTL=45min; mid-burst renewal; state-burst CAS push fix; fail-open on malformed; sole-writer invariant. PC1 (schema correctness), PC2 (unlock clears block), PC3 (TTL expiry guard), PC4 (mid-burst renewal), PC5 (CAS push), PC6 (single-dev zero friction). 4 error variants: SchemaViolation, StaleNullBlock, RenewalMissed, CASPushRejected. 9 edge cases EC-001..EC-009. CAP-031 registered same burst. lifecycle_status: draft (POL-14 auto-promotion pending). |
