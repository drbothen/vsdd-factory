---
document_type: architecture-decision-record
level: L3
adr_id: ADR-025
version: "1.6"
status: accepted
producer: architect
timestamp: 2026-06-10T00:00:00Z
amended: 2026-06-11T00:00:00Z
amendment_reason: "v1.5→v1.6: [S-17.04 redirect — human approved, adversary pass 1 incorporated] Decision 12 added: `verify-state-timestamp-refresh` WASM PreToolUse guard plugin. Decision 11 Mechanism 2 SUPERSEDED. Mechanism 1 (D10) RETAINED. (a) DECISION 12: new WASM plugin `verify-state-timestamp-refresh` in `crates/hook-plugins/verify-state-timestamp-refresh/`, registered in `hooks-registry.toml` as PreToolUse on Edit|Write|MultiEdit where file_path resolves to `.factory/STATE.md`. Per-tool proposed-content extraction: Write → `tool_input.content` (full file body); Edit → on-disk STATE.md with `tool_input.old_string` replaced by `tool_input.new_string` (first occurrence; `replace_all` honored); MultiEdit → on-disk STATE.md with each `tool_input.edits[]` element applied sequentially. Guard reads on-disk via `host::read_file`; compares time fields: BLOCKS if `timestamp:` not advanced (TimestampStale) OR lock held in proposed content and `factory_lock.expires_at` not advanced (LockExpiryStale). Fails open on parse/IO errors per Decision 7 precedent. (b) CORRECTED FINDINGS (adversary pass 1): `new_content` stale field removed (not a real Claude Code payload field — correct fields are `content`/`old_string`+`new_string`/`edits[]`); `[hooks.capabilities.read_file]` corrected to `path_allow`-only (ReadFileCaps is `#[serde(deny_unknown_fields)]` with only `path_allow: Vec<String>` — `max_bytes`/`timeout_ms` fields do not exist in the struct and would break registry load); explicit priorities added (verify-factory-lock at 142, verify-state-timestamp-refresh at 143 — must run AFTER verify-factory-lock, so 143 > 142); canonical-path rule specified (strip leading `./`, collapse `//`, treat absolute `$CLAUDE_PROJECT_DIR/`-prefixed paths — robust normalization, not fail-open); block message format corrected to real `block_with_fix` segments; robust frontmatter extraction specified. (c) SUPERSESSION: Decision 11 Mechanism 2 (D11/D12-registry/D14) WITHDRAWN. Push-time enforcement dropped. (d) INV-019 CURE: (a) Decision 12 added; (b) D11/D12-registry/D14 withdrawn, D15/D16/D17 added; (c) S-17.04 Re-Scope Directive issued. ARCH-INDEX v2.21→v2.22 pending state-manager codification burst." v1.4→v1.5 amendment_reason preserved inline: [S-17.04 adversary F-1701-001] Gate-trigger fix for Decision 11 Mechanism 2 + block-message reconciliation + D12 jq capability sync. (1) TRIGGER CORRECTION: the v1.4 spec stated the gate triggers on `git.*push.*factory-artifacts` in the Bash tool-command string. This is inert on the production push path: post-S-17.01 the state-burst SKILL runs `bash plugins/vsdd-factory/bin/factory-cas-push.sh`, and the real `git push --force-with-lease` is a subprocess inside that helper — PreToolUse never inspects subprocess commands. The gate MUST trigger when `.tool_input.command` contains `factory-cas-push` (the canonical helper the SKILL uses) OR matches `git`+`push`+`factory-artifacts` (belt-and-suspenders for any hand-typed raw push). The check fires at PreToolUse on `bash factory-cas-push.sh`, at which point the burst commit already exists locally (HEAD STATE.md carries this burst's expires_at), so the HEAD-vs-origin comparison is valid. (2) BLOCK MESSAGE RECONCILIATION: the legacy-bash-adapter truncates plugin output to the first line of stdout. The implemented gate must therefore emit a single-line block_pre-form message: 'BLOCKED by verify-lock-renewal: RenewalMissed — factory_lock held but expires_at not refreshed in this burst. Fix: Run: factory-lock-write.sh renew .factory/STATE.md Code: RenewalMissed.' The multi-line verbatim text specified in v1.4 step 6 is unreachable through the legacy-bash-adapter; it is now replaced by this one-liner in the Decision 11 spec. (3) D12 JQ SYNC: D12 `exec_subprocess.binary_allow` must include `\"jq\"` alongside `\"bash\"` and `\"git\"`. The gate script execs `jq` to parse the JSON-envelope STATE.md frontmatter; omitting `jq` from binary_allow causes CapabilityDenied → silent fail-open → gate is inert. This is the fourth instance of the deny-by-default silent-no-op footgun class (vector 4: exec_subprocess binary_allow missing required tool for script internals). v1.3→v1.4 amendment_reason preserved inline: [S-17.04] Automatic heartbeat renewal enforcement wiring. Decision 11 added: two complementary mechanisms close the prose-only PC4 enforcement gap — (1) mandatory executable factory-lock-write.sh renew step in state-burst SKILL before git add/commit (Option A); (2) new verify-lock-renewal.sh PreToolUse bash hook that blocks a held-lock factory-artifacts push when HEAD's expires_at equals origin/factory-artifacts' expires_at (RenewalMissed — renewal not committed in this burst), on_error=continue, async=false, no-op when unlocked or no remote baseline (Option C). Decision 5 vestigial burst-END-only sentence corrected. Deliverables D10–D14 added. BC-5.40.001 PC4 unaffected. v1.2→v1.3 amendment_reason preserved inline: [process-gap] S-17.02 TDD implementation finding — exec_subprocess env_allow omission footgun. Decision 2 / D2 capability block spec was incomplete: exec_subprocess capability block listed only binary_allow = [\"git\"] but omitted env_allow. The dispatcher's exec_subprocess host function calls env_clear() and passes ONLY vars listed in caps.env_allow; without HOME (and GIT_CONFIG_GLOBAL / XDG_CONFIG_HOME) in env_allow, git config user.email cannot read the developer's global gitconfig, returns empty string, plugin hits IdentityResolutionFailed, fails open (Continue), and the lock guard is a silent no-op. This is the THIRD instance of the deny-by-default silent-no-op footgun class (first: read_file block omitted; second: exec_subprocess binary_allow omitted; third: exec_subprocess env_allow omitted). Fix: Decision 2 and D2 canonical registry snippet updated to include env_allow = [\"HOME\", \"GIT_CONFIG_GLOBAL\", \"XDG_CONFIG_HOME\"] on the exec_subprocess capability block. Rationale section updated to name all three footgun vectors explicitly. Process note added."
title: "ADR-025: Single-writer factory lock/lease — prevent concurrent session races on factory-artifacts orphan branch"
traces_to: .factory/specs/architecture/ARCH-INDEX.md
anchors:
  - SS-04
  - SS-05
  - SS-06
  - SS-07
  - issue-170
subsystems_affected:
  - SS-04
  - SS-05
  - SS-06
  - SS-07
supersedes: null
superseded_by: null
decision_status: accepted
human_gate_required: false
human_gate_reason: "All decisions confirmed by human design review 2026-06-10. Research-agent verification returned APPROVE-WITH-FIXES 2026-06-10; all five fixes incorporated in v1.2. No remaining human-gated open questions. D-540 codification recorded by state-manager 2026-06-10. Implementation may proceed."
---

# ADR-025: Single-writer factory lock/lease — prevent concurrent session races on factory-artifacts orphan branch

## Status

**ACCEPTED — human design confirmed 2026-06-10; research-agent verification APPROVE-WITH-FIXES incorporated as v1.2. D-540 codification recorded by state-manager 2026-06-10. Implementation dispatch ready. v1.3 amended 2026-06-11: [process-gap] S-17.02 TDD finding — exec_subprocess env_allow omission footgun; env_allow = ["HOME", "GIT_CONFIG_GLOBAL", "XDG_CONFIG_HOME"] added to D2 canonical registry form. v1.4 amended 2026-06-11: [S-17.04] Decision 11 added — automatic heartbeat renewal enforcement (executable state-burst SKILL step + PreToolUse push gate); Decision 5 vestigial burst-END sentence corrected; Deliverables D10–D14 added. v1.5 amended 2026-06-11: [S-17.04 adversary F-1701-001] Decision 11 gate-trigger correction (trigger must fire on `factory-cas-push` helper, not only raw `git push`; the real push runs as subprocess inside the helper and is invisible to PreToolUse), block-message reconciled to legacy-bash-adapter one-liner form, D12 binary_allow extended with "jq" (fourth deny-by-default silent-no-op vector closed). v1.6 amended 2026-06-11: [S-17.04 redirect — human approved; adversary pass 1 incorporated] Decision 12 added — `verify-state-timestamp-refresh` Rust WASM PreToolUse guard: blocks Edit/Write/MultiEdit to STATE.md when proposed full content (reconstructed for Edit/MultiEdit) does not advance `timestamp:` frontmatter (every write) or `factory_lock.expires_at` (when lock held). Per-tool field extraction: Write→`tool_input.content`; Edit→on-disk+`old_string`/`new_string` reconstruct; MultiEdit→sequential `edits[]` apply. Registry caps corrected: `[hooks.capabilities.read_file]` accepts `path_allow` ONLY (`max_bytes`/`timeout_ms` are not struct fields — adding them breaks registry load). Priorities made explicit: verify-factory-lock=142, verify-state-timestamp-refresh=143 (lock-identity check fires first). Canonical-path normalization rule specified. Block message format corrected to real `block_with_fix` segments. Decision 11 Mechanism 2 (D11/D12-registry/D14) SUPERSEDED. Mechanism 1 (D10) RETAINED. S-17.04 REDIRECTED to v1.2. S-17.04 + rc.21 HELD.**

This ADR resolves the design for the factory lock/lease primitive requested in issue #170.
Twelve decisions are confirmed. Five research-agent fixes are incorporated in v1.2, one
process-gap spec-drift amendment in v1.3, one enforcement-wiring amendment in v1.4, one
gate-trigger + message + capability correction in v1.5, and one Rust WASM guard adoption
with Decision 11 Mechanism 2 supersession plus adversary pass 1 corrections in v1.6
(per-tool payload extraction, registry caps reality, explicit priorities, canonical-path
rule, block-message format, robust time extraction — see amendment_reason above).
No further human-gated questions remain.

## Context

### The gap: cross-session single-writer is absent

The factory's within-session single-writer discipline is real and robust: `state-manager`
is the sole `.factory/` writer, runs last in every burst, and commits atomically via the
single-commit burst protocol (TD-VSDD-053). However, this discipline has no equivalent
across independent developer sessions.

Two developers — or two Claude Code sessions belonging to different developers — can run
pipelines against the same repo's `factory-artifacts` orphan branch concurrently. Because
all factory state converges on that one branch, concurrent runs race: one party's
state/spec commits can be silently lost, clobbered, or produce a painful orphan-branch
divergence with no merge base (which requires manual surgery to reconcile, as there is no
common ancestor).

Research cache (`issue-170.md`) confirms the gap with zero relevant grep hits for
`lock|flock|mutex|lease|heartbeat|session_id|exclusive` across `plugins/`. The push path
is a plain `git push origin factory-artifacts` at `skills/state-burst/SKILL.md` (the push
call) with no compare-and-swap. `hooks/verify-git-push.sh` explicitly allows
`factory-artifacts` pushes and `--force-with-lease` with no exclusivity check.

### Design principle: keep the mechanism local and simple

The factory's primary deployment is a single developer — or a small team where turn-based
coordination is natural. The concurrency hazard is not millisecond-scale races but
session-level mistakes: two people both start pipeline work without realizing the other is
active. A simple, local, human-readable cooperative lock — visible in STATE.md, enforced
by a WASM hook, controlled by explicit user commands — is the right fit for this threat
model.

A heavyweight server-side CAS mechanism (git refs, etcd, etc.) imposes verification
prerequisites and infrastructure assumptions that are not warranted for the actual threat
model. That upgrade path is preserved as a future option (see §Decision 9) but is not the
primary design.

### Scope

This ADR scopes the lock to **whole-factory granularity** (per repo's `factory-artifacts`
branch) and **developer-level identity** (`git config user.email`). The documented
tradeoff — that the same developer in two concurrent sessions will NOT self-block — is
accepted. The guard protects Developer A vs Developer B, not self-vs-self.

## Decision

This ADR makes ten numbered decisions. All are confirmed by human review 2026-06-10 and
verified by research-agent review 2026-06-10 (v1.2 incorporates five APPROVE-WITH-FIXES
corrections).

### Decision 1: Primary enforcement — native-WASM PreToolUse guard `verify-factory-lock`

We will implement a **new native-WASM hook plugin** `verify-factory-lock` as the primary
enforcement mechanism:

- **New Rust crate:** `crates/hook-plugins/verify-factory-lock/` compiled to
  `plugins/vsdd-factory/hook-plugins/verify-factory-lock.wasm`.
- **Registered in `plugins/vsdd-factory/hooks-registry.toml`** as a `PreToolUse` guard
  on mutating tools: `tool = "Edit|Write|Agent"` plus a separate entry for Bash covering
  `.factory/` pushes. See Decision 2 (deliverable D2) for the complete and mandatory
  capability block specification.
- **Guard logic:** reads `STATE.md` via `host::read_file`, parses the `factory_lock`
  frontmatter block, checks `holder != current_git_email` and `now <= expires_at`. If
  both conditions are true, returns `block_intent = true` (exit code 2) with the
  actionable refusal message (Decision 4). The production block path runs through
  `plugin_requests_block()` in `crates/factory-dispatcher/src/executor.rs:609`, invoked
  at `executor.rs:105–108` for sync-group plugins. The guard MUST be sync-group
  (`async = false`) — see Decision 2.
- **Read-only tool calls pass through** unconditionally. Only mutating tool calls (Edit,
  Write, Agent dispatch, and Bash commands that push to `factory-artifacts`) are blocked.

**Host ABI is unchanged.** The guard uses only `host::read_file` and
`host::exec_subprocess` with `binary_allow = ["git"]` — both already present in the
dispatcher host ABI at `HOST_ABI_VERSION = 1` (`crates/hook-sdk/src/lib.rs:65`). The
dispatcher binary (`crates/factory-dispatcher`) requires no changes for this feature.

The guard follows the same pattern as `validate-artifact-path.wasm` (an existing
native-WASM PreToolUse guard using `host::read_file`), confirming the structure is
established and the crate scaffolding is known.

Referencing SS-04 (Plugin Ecosystem) because `verify-factory-lock` is a new WASM plugin
crate in `crates/hook-plugins/`. Referencing SS-05 (Pipeline Orchestration) because
`state-manager` writes the `factory_lock` frontmatter block that the guard reads.
Referencing SS-06 (Skill Catalog) because `/factory-lock` and `/factory-unlock` are new
skills. Referencing SS-07 (Hook Bash Layer) because the guard is registered in
`hooks-registry.toml` alongside existing guards.

### Decision 2: Lock state — `factory_lock` frontmatter block in STATE.md

The authoritative lock state lives in the `factory_lock` block in `STATE.md` frontmatter:

```yaml
factory_lock:
  holder: "developer@example.com"   # git config user.email of the locking session
  locked_at: "2026-06-10T14:00:00Z" # ISO-8601
  expires_at: "2026-06-10T14:45:00Z" # ISO-8601; locked_at + TTL
```

Absent or null `factory_lock` block = unlocked. A malformed block (missing required
fields) is treated as unlocked (fail-open, consistent with Decision 7).

The `factory_lock` block travels on `factory-artifacts`, so any developer fetching the
branch sees the current lock state. Any developer can inspect it with
`cat .factory/STATE.md` or via `/factory-health`. The guard reads it via `host::read_file`
without any network call — the fetch of `factory-artifacts` that happens at burst start
is the synchronization point.

`state-manager` is the sole writer of this block, consistent with its role as the sole
`.factory/` writer (TD-VSDD-053). The `/factory-lock` and `/factory-unlock` skills
(Decision 6) delegate writing to `state-manager`.

**Note:** See Decision 3 and the D2 capability block specification for the env_allow
requirement on `exec_subprocess`. Without `HOME` in `env_allow`, `git config user.email`
cannot read the developer's global gitconfig and identity resolution fails open.

### Decision 3: Session identity — `git config user.email` (developer-level, coarse)

The lock holder identity is the output of `git config user.email`, obtained by the guard
via `host::exec_subprocess` with `binary_allow = ["git"]`.

**Documented tradeoff:** this is developer-level identity, not session-level. The same
developer running two concurrent sessions on two machines shares the same git email and
will NOT be self-blocked. The guard protects Developer A vs Developer B. Self-vs-self
concurrency (one developer, two sessions) is out of scope for this iteration and is
addressed by social coordination and the observability surfacing in `/factory-health`.

This tradeoff is accepted because:
1. The primary risk is two different developers inadvertently running concurrent sessions,
   not one developer deliberately doing so.
2. Composite session identity (hostname + pid + Claude session ID) introduces env-var
   dependencies (`CLAUDE_SESSION_ID`) and complexity that is not warranted by the threat
   model.
3. The blind-push fix (Decision 8) remains active as a safety net for the self-vs-self
   case: concurrent pushes from the same developer are detected rather than silently
   clobbering.

### Decision 4: Block semantics and refusal message

When `factory_lock.holder` is set, `now <= expires_at`, and `holder != current_git_email`:

- The guard returns `block_intent = true` (exit code 2) for mutating tools: Edit, Write,
  Agent dispatch, and Bash commands pushing to `factory-artifacts`. This signals the
  dispatcher's sync-group block path (`executor.rs:105–108`,
  `plugin_requests_block` at `executor.rs:609`).
- Read-only tools (Read, Bash reads, non-mutating tool calls) pass through unconditionally
  so the blocked developer can inspect STATE.md to see who holds the lock and when it
  expires.

The refusal message MUST include all of:
- `holder` — the git email of the current lock holder
- `locked_at` — ISO-8601 timestamp when the lock was acquired
- `expires_at` — ISO-8601 timestamp when the lock auto-expires
- `time_remaining` — human-readable duration (e.g., "37 min remaining")
- `/factory-unlock --force` — the exact command to break-glass force-release the lock

Example refusal output:
```
Factory locked by developer@example.com
  Locked at:  2026-06-10T14:00:00Z
  Expires at: 2026-06-10T14:45:00Z (37 min remaining)

To wait: the lock auto-expires at 14:45 UTC.
To force-release: /factory-unlock --force
```

### Decision 5: Stale-lock escape — TTL auto-expiry AND `/factory-unlock --force`

Both escape paths are required. A lock without escape is a stale-lock footgun in waiting.

**Path A — TTL auto-expiry:**
- Default TTL: **45 minutes** (midpoint of the research-backed 2–5× expected burst
  duration range; expected burst duration ~10 minutes).
- The guard computes `now > expires_at` on every check. An expired lock is treated as
  absent — the check passes and the operation proceeds.
- Heartbeat renewal: `state-manager` updates `expires_at = now + TTL` on every
  `state-burst` completion, extending the lease while the session is active. The renewal
  heartbeat fires on every `state-manager` commit in a burst (Commits A through E), not
  only at burst-close. See Decision 11 for the enforcement mechanism.
- A crashed session that never calls `/factory-unlock` auto-expires after 45 minutes at
  worst.

**Failure mode — long burst TTL self-eviction:**

A single burst longer than the 45-minute TTL (e.g., a 30-pass adversary cascade, a large
batch story delivery, a slow network during multi-file commits) self-evicts the lock
mid-burst: `now > expires_at` becomes true while the burst is still running, allowing
another developer to acquire. This is the long-operation hazard identified in lease
literature (Kleppmann §8 "Leases and Lease-Based Locks"; Kubernetes Lease API design notes;
HashiCorp Vault session TTL guidance).

**Mitigation chosen — mid-burst renewal via explicit `/factory-renew` call:**

`state-manager` MUST call a mid-burst renewal whenever a long-running sub-step (e.g.,
each adversary pass within a cascade) is about to commit. Concretely: at every intermediate
`state-manager` commit within a burst (not only at burst-close), `state-manager` writes
an updated `expires_at = now + TTL` alongside the commit. This resets the TTL clock to 45
minutes from each intermediate write rather than from the original `locked_at`. No
separate background timer process is required — the burst's own commit cadence is the
renewal heartbeat.

**Residual risk — fencing:**

Mid-burst renewal via commit does not provide a fencing token (a monotonically increasing
value that storage can check to reject stale-holder writes). If the TTL expires between
two intermediate commits — possible under extreme network delay or WASM fuel exhaustion
on the renewal commit itself — a second developer could acquire between renewals and both
parties proceed in parallel. This residual risk is **explicitly attributed to the Decision
9 git-ref-CAS future path** as the correctness-class upgrade: git ref CAS with monotonic
object-id chaining provides the fencing token that eliminates this window. Under the
current design (advisory/efficiency-class lock per Kleppmann's distinction — see Decision
7), the residual window is accepted because the threat model is cooperative teams, not
adversarial concurrent writers.

**Path B — `/factory-unlock --force` break-glass:**
- Any developer (not just the holder) may run `/factory-unlock --force` to clear the lock
  immediately.
- Force-release is **loudly audit-logged** via the SS-03 event pipeline as
  `factory.lock.stolen` including: `stolen_by` (git email of the releaser), `stolen_from`
  (git email of the original holder), `holder_locked_at`, `stolen_at`. This event is
  non-blocking but permanent — the audit trail cannot be suppressed.
- Without `--force`, `/factory-unlock` only succeeds if `current_git_email == holder`.
  Attempting `/factory-unlock` as a non-holder without `--force` exits with an error and
  does not modify STATE.md.

### Decision 6: Acquire/release UX — explicit `/factory-lock` and `/factory-unlock` skills

Lock acquisition and release are **explicit user actions**, not automatic. There is no
auto-acquire-on-first-write.

**`/factory-lock` skill — acquire with CAS push (Fix 1: closes TOCTOU acquire-race):**
- Performs a `git fetch origin factory-artifacts` to get the current remote state.
- Reads the current `factory_lock` block from the just-fetched local STATE.md.
- If locked by another developer (unexpired): exits with the refusal message (Decision 4).
- If unlocked or expired: delegates to `state-manager` to write
  `factory_lock = { holder: <my_email>, locked_at: <now>, expires_at: <now + 45m> }`
  into STATE.md frontmatter, commit, and push **using the same fetch-then-CAS primitive
  as Decision 8**:

  ```bash
  git -C .factory fetch origin factory-artifacts
  EXPECTED_SHA=$(git -C .factory rev-parse origin/factory-artifacts)
  git -C .factory push --force-with-lease=factory-artifacts:"${EXPECTED_SHA}" origin factory-artifacts
  ```

  If the push is rejected (non-zero exit, non-fast-forward), the acquire fails: another
  session acquired between our fetch and our push (a TOCTOU acquire-race per CWE-367).
  The skill exits with an actionable error: "Acquire failed — concurrent lock write
  detected. Fetch and retry."

- Emits `factory.lock.acquired` event on success.
- The burst heartbeat path (mid-burst `expires_at` renewal by `state-manager`) is
  invoked automatically inside each burst after the lock is held.

**Residual TOCTOU window (honest statement):** The CAS push closes the primary acquire
race: two developers who both see an unlocked STATE.md and both attempt acquire will have
one succeed and one receive a non-fast-forward rejection. The remaining residual window is
the **exact-simultaneity-before-either-push** scenario: two sessions that complete the
fetch step before either executes the push. In this window the `--force-with-lease` check
is the tiebreaker — one push will land and one will be rejected. The rejected session
retries from the top of the acquire flow. This window is a **TOCTOU acquire-race
(CWE-367)** that is narrowed but not eliminated by the CAS push; it is accepted as
residual because the window is bounded to the fetch→push interval (milliseconds under
normal conditions) and the cooperative threat model does not require zero-window
exclusivity. The git-ref CAS future path (Decision 9) eliminates this window entirely.

**`/factory-unlock` skill:**
- Without `--force`: only the current holder (`current_git_email == holder`) may release.
  Clears `factory_lock` block from STATE.md, commits, pushes. Emits `factory.lock.released`.
- With `--force`: any developer may release. Emits `factory.lock.stolen` audit event
  (Decision 5 Path B). Clears the block, commits, pushes.

**Rationale for explicit acquire:** "the user that locked it" is the correct mental model.
Auto-acquiring on first write would mean a crash before the first write leaves no lock,
and a developer might not know they own it. Explicit acquire makes the session boundary
clear, mirrors the `git stash` / `git commit` UX pattern (deliberate state transitions),
and avoids surprise: the developer knows exactly when they take ownership.

### Decision 7: Crash behavior — `on_error = "continue"` (fail-open)

The `verify-factory-lock` plugin's `on_error` field in `hooks-registry.toml` is set to
`"continue"`.

**Rationale:** fail-open is correct here because this is an **advisory/efficiency-class
lock** (Kleppmann §8: "efficiency" — avoiding unnecessary work by two parties — vs
"correctness" — preventing data corruption that cannot be fixed). Per Kleppmann's
distinction, efficiency-class locks can safely fail open because the consequence of a
missed block is duplicated work or a detected push collision (caught by Decision 8's CAS
push), not silent data corruption. Decision 8's `--force-with-lease` push is the
independent safety net that bounds the worst-case outcome of a guard crash to a detected
conflict rather than a silent clobber.

A crashing lock-checker that blocks all writes (`on_error = "block"`) is the stale-lock
footgun in a different costume: a broken guard permanently wedges the factory until the
plugin is fixed or the registry is manually edited. The cost of a false-positive (blocked
write due to guard crash) exceeds the cost of a false-negative (missed guard due to
crash) for this threat model.

Guard crashes are surfaced as advisory log events via `internal.dispatcher_error` (SS-03)
so developers are aware without being blocked.

Existing precedent: `validate-artifact-path.wasm` also uses `on_error = "continue"` for
the same reason (ADR-016).

### Decision 8: Complementary mitigation — blind-push fix in `state-burst` (secondary, standalone)

The blind push at `skills/state-burst/SKILL.md` (the `git push origin factory-artifacts`
call) MUST be changed to a **fetch-then-CAS push** regardless of the lock primitive:

```bash
git -C .factory fetch origin factory-artifacts
EXPECTED_SHA=$(git -C .factory rev-parse origin/factory-artifacts)
git -C .factory push --force-with-lease=factory-artifacts:"${EXPECTED_SHA}" origin factory-artifacts
```

This is a **separate, complementary mitigation** — not the primary enforcement mechanism.
It is independently deliverable and independently valuable: even without the WASM guard,
converting the blind push to `--force-with-lease` means that a concurrent push from
another developer causes a detected collision (non-zero exit, actionable error) rather
than a silent clobber. It is also the safety net for (a) the self-vs-self case that the
coarse identity (Decision 3) intentionally does not guard, and (b) guard-crash fail-open
scenarios (Decision 7) — if the guard misses a block, the push-layer CAS still rejects
concurrent writes.

This change is confirmed already allowed by `hooks/verify-git-push.sh` (which only blocks
raw `--force`; `--force-with-lease` is permitted).

This same CAS primitive is reused by the `/factory-lock` acquire path (Decision 6) at
zero additional cost.

### Decision 9: Future / Out of Scope — git-ref CAS upgrade path

The git ref `refs/factory-lock/<repo-slug>` compare-and-swap mechanism is a **future
upgrade path**, not a current deliverable.

It is the correct choice if the threat model escalates to high-velocity teams where
explicit `/factory-lock` coordination breaks down, or where session-level identity is
required, or where the residual TOCTOU acquire-race (Decision 6) or mid-burst self-eviction
residual risk (Decision 5) must be fully eliminated. This path also provides the fencing
token (monotonic object-id chain) that the current design lacks.

When this path is pursued, it requires an empirical GitHub.com server-side CAS verification
probe (research flags that GitLab historically did not enforce strict ref CAS; GitHub.com
behavior must be confirmed before relying on it). That probe is **not a blocking step for
the current implementation** because we are not relying on server-side CAS in v1.

### Decision 10: Single-developer behavior — hard invariant, no added human action

Single-developer single-session use of the factory is the primary case and MUST be
unaffected in the following sense: a developer who has run `/factory-lock` to acquire the
lock will see zero friction during normal operation. The guard passes all checks silently.
No additional human actions are required between lock acquisition and release.

Observable changes for a single developer:
- Running `/factory-lock` once at the start of a pipeline session (new deliberate step).
- `Factory lock: HELD by this session (expires <time>)` line in `/factory-health` output.
- Running `/factory-unlock` once at the end of the session (or letting it auto-expire).

The guard adds negligible latency per hook invocation: one `host::read_file` call on
STATE.md (a small local file) plus one `host::exec_subprocess` call to `git config
user.email` plus one timestamp comparison. Both calls are local (no network). The latency
budget constraint is well within ADR-020 Class A (p95 ≤ 1500ms for the hook chain).

A developer who does not run `/factory-lock` is in the same position as today: the guard
reads `factory_lock: null` and passes all checks. The lock is opt-in; absence of a lock
record is treated as unlocked.

### Decision 11: Automatic heartbeat renewal enforcement — executable skill step + PreToolUse push gate

The mid-burst `expires_at` renewal obligation (Decision 5 / BC-5.40.001 PC4) is enforced
by two complementary mechanisms, not by agent-remembered prose alone. Prior to this
decision, `state-manager.md` §"factory_lock Write/Renewal/Clear Obligation" documented the
requirement to call `factory-lock-write.sh renew` before each burst commit, but the
`state-burst` SKILL itself never invoked it. An agent that followed state-manager.md prose
but not the skill step — or ran the skill without loading the obligation section — would
silently miss the renewal, allowing the lock to self-evict mid-burst.

**Mechanism 1 — Executable `state-burst` step (Option A):**

The `state-burst` SKILL (`plugins/vsdd-factory/skills/state-burst/SKILL.md`) MUST include
`factory-lock-write.sh renew .factory/STATE.md` as a mandatory numbered step immediately
before the `git -C .factory add -A` / `git commit` block. The call is unconditional: when
no lock is held (absent `factory_lock:` key), the script exits 0 with "no factory_lock
block present — renew is a no-op" — zero friction on the common case. This converts PC4
from a prose obligation to a mechanically-invocable step executed every time the burst
skill is followed.

The `factory-lock-write.sh` script (`plugins/vsdd-factory/bin/factory-lock-write.sh`,
delivered by S-17.01) already implements the `renew` subcommand with a RenewalMissed
guard, post-renew assertion, and CRLF normalization. No new script is required.

**Mechanism 2 — `verify-lock-renewal.sh` PreToolUse gate (Option C):**

A new bash hook `plugins/vsdd-factory/hooks/verify-lock-renewal.sh`, registered in
`hooks-registry.toml` as `PreToolUse` / Bash / `on_error = "continue"` / `async = false`,
provides fail-closed enforcement at the push boundary. At PreToolUse on any Bash command
that invokes the factory-artifacts push, the gate:

1. Checks whether the Bash tool-input command triggers the push. The gate fires if
   `.tool_input.command` **contains `factory-cas-push`** (the canonical helper that
   `state-burst` SKILL runs — `bash plugins/vsdd-factory/bin/factory-cas-push.sh` — and
   which contains the real `git push --force-with-lease` as a subprocess invisible to
   PreToolUse) **OR** if `.tool_input.command` matches `git`+`push`+`factory-artifacts`
   (belt-and-suspenders for any hand-typed raw push). Both patterns are evaluated in order;
   either match triggers the gate. Any Bash command that matches neither pattern returns
   exit 0 (Continue) immediately — non-push commands add zero overhead.

   **Rationale for `factory-cas-push` as the primary trigger:** the v1.4 spec used only
   `git.*push.*factory-artifacts` as the trigger pattern. That pattern is inert on the
   production push path because `state-burst` post-S-17.01 runs `bash factory-cas-push.sh`,
   and the real `git push --force-with-lease=factory-artifacts:...` is a subprocess inside
   that helper. PreToolUse only inspects the top-level Bash command string, not subprocesses.
   A gate keyed solely on the raw `git push` pattern NEVER fires on the SKILL's canonical
   push path — enforcement is functionally inert. The primary trigger must therefore match
   the helper script name.

2. Reads `factory_lock.holder` and `factory_lock.expires_at` from the local committed HEAD:
   `git -C .factory show HEAD:STATE.md`. At PreToolUse time the commit already exists
   locally (the `git commit` ran before the push Bash command fires), so HEAD reflects the
   staged renew if Mechanism 1 was followed. The check firing at `bash factory-cas-push.sh`
   PreToolUse is valid: the burst commit was already composed at this point, so HEAD
   STATE.md carries this burst's `expires_at`.
3. If `factory_lock.holder` is absent in HEAD (factory unlocked): returns exit 0. No-op.
4. If `origin/factory-artifacts` does not exist (first push to a new branch): returns exit 0.
5. Reads `factory_lock.expires_at` from the remote tip:
   `git -C .factory show origin/factory-artifacts:STATE.md`.
6. If HEAD `expires_at` equals `origin/factory-artifacts` `expires_at` (the value was NOT
   refreshed in this burst's commits): returns exit code 2 (block) with the message:

   ```
   BLOCKED by verify-lock-renewal: RenewalMissed — factory_lock held but expires_at not refreshed in this burst. Fix: Run: factory-lock-write.sh renew .factory/STATE.md Code: RenewalMissed.
   ```

   **Message form rationale:** The gate runs via `legacy-bash-adapter.wasm`, which truncates
   plugin output to the **first line of stdout** before surfacing it as the block message.
   A multi-line message is therefore unreachable — only the first line is shown to the
   developer. The single-line `block_pre`-form above is the correct contract: it names the
   gate, the error code, the human-readable cause, and the fix command on one line. This is
   the same first-line-truncation constraint that governs all bash-adapter gates (e.g.,
   `verify-git-push.sh`).
7. If HEAD `expires_at` differs from remote (renewal was committed): returns exit 0.

**Why PreToolUse, not PostToolUse:**

PostToolUse fires after the push has already executed — it can flag but cannot block.
PreToolUse fires before the push runs, allowing a hard block. This is the same trigger
point as `verify-git-push.sh` (PreToolUse / Bash), which guards `factory-artifacts`
pushes using the same legacy-bash-adapter pattern. The gate mirrors that pattern exactly.

**`on_error = "continue"` rationale:**

Consistent with Decision 7: an efficiency-class lock's guard crash must not wedge the
factory. A broken gate that permanently blocks all pushes is a worse failure mode than
a missed renewal, which is bounded by the TTL auto-expiry (Decision 5 Path A). Fail-open
on crash; the audit trail via `internal.dispatcher_error` (SS-03) surfaces the crash
without blocking the developer.

**`async = false` requirement:**

Same as Decision 2 / Decisions 1 rationale: only sync-group plugins participate in the
`block_intent` aggregation at `executor.rs:100–117` (ADR-019). An async plugin's block
signal is advisory-only and would silently reduce the gate to telemetry.

**BC-5.40.001 PC4 unaffected:**

This decision implements BC-5.40.001 PC4 ("state-manager MUST refresh
`factory_lock.expires_at = now + 45 minutes` at every intermediate burst commit, atomic
with the commit"). PC4's postcondition text is correct and complete as written; no BC
amendment is required.

**SUPERSESSION NOTE (v1.6):** Decision 11 Mechanism 2 (the `verify-lock-renewal.sh`
PreToolUse bash gate, Deliverables D11 / D12-registry-entry / D14) is superseded by
Decision 12 (the `verify-state-timestamp-refresh` WASM PreToolUse guard). Mechanism 1 (the
`state-burst` SKILL renew step, Deliverable D10) is retained unchanged — it is the
mechanism that *performs* the renewal; the WASM guard is the mechanism that *enforces*
it happened at write-time. The supersession does NOT remove D10 from scope.

Push-time enforcement (a renewal gate on `factory-cas-push.sh`) is dropped entirely.
With freshness guaranteed at write-time by the WASM guard, the committed STATE.md always
carries a current heartbeat by the time it is pushed — `factory-cas-push.sh` needs no
renewal gate and remains a plain CAS push.

The v1.5 Decision 11 body above documents the design rationale for the PreToolUse bash
approach and its four bypass vectors; it is preserved as historical record. Implementers
MUST NOT build the PreToolUse bash gate (D11/D12-registry/D14 withdrawn). Implementers
MUST build the WASM guard per Decision 12.

### Decision 12: `verify-state-timestamp-refresh` Rust WASM PreToolUse guard (v1.6)

The hook SDK exposes exactly three outcomes: `Continue` (exit 0), `Block` (exit 2),
`Error` (exit 1). There is no mutate/write-content outcome. Therefore "update the time on
every STATE.md touch" is implemented as: **state-manager writes the fresh time (Mechanism 1,
D10), and a WASM PreToolUse guard blocks the write if the time was not refreshed.** This is
the exact shape of the existing `verify-factory-lock` plugin and follows the established
VSDD Rust hook pattern precisely.

#### 12.1 Plugin identity and trigger

**Plugin name:** `verify-state-timestamp-refresh`
**Crate:** `crates/hook-plugins/verify-state-timestamp-refresh/`
**Compiled to:** `plugins/vsdd-factory/hook-plugins/verify-state-timestamp-refresh.wasm`
**Event:** `PreToolUse`
**Tools:** `Edit`, `Write`, and `MultiEdit` — all three tools that mutate file content
**Trigger condition:** `tool_input.file_path` in the payload resolves to `.factory/STATE.md`
(after canonical-path normalization — see §12.7 R6). Any of the three tools whose
`file_path` does NOT resolve to `.factory/STATE.md` returns `Continue` immediately without
reading any file.

**`tool_input.file_path` is always the correct field name.** Claude Code's Write, Edit, and
MultiEdit tools all use `file_path` (not `path`). The dispatcher log confirms: `file_path`
appears in 5,235 captured PreToolUse events; the field `new_content` appears zero times.
The payload structure is `serde_json::Value` (no `deny_unknown_fields`) — there is no parse
error on an unknown field, it simply returns `None` when accessed. An implementation that
reads `tool_input.new_content` will hit the fail-open branch on every real write and the
guard will be a production no-op. Do NOT use `new_content`.

This trigger is structurally bypass-proof: the hook payload field `file_path` is set by
the Claude Code tool infrastructure, not by user command text. There is no Bash command
string to tokenize, no regex to defeat, and no subprocess whose inner invocations are
invisible. All four bypass vectors that afflicted Decision 11 Mechanism 2 (inert-match,
over-match, newline-injection, env-injection) never arise here — the trigger is a
structured field, not a free-text command string.

**Scope: `.factory/STATE.md` only.** The broader factory-artifacts mutation-protection
stays with the existing `verify-factory-lock` guard (Decision 1/2). This new guard's path
scope is exactly one file: `.factory/STATE.md`. Do not widen it.

**Priority ordering with verify-factory-lock:** Both plugins fire on `Edit|Write` to
`.factory/STATE.md`. `verify-factory-lock` must run first (identity check precedes
timestamp check). Assign explicit priorities in the registry: `verify-factory-lock = 142`,
`verify-state-timestamp-refresh = 143`. Lower numbers fire first; same-priority entries
run in parallel. Without explicit priorities, both entries inherit the default (500) and
run in parallel — the ordering is undefined. The registry currently shows no `priority =`
line in the `verify-factory-lock` entry, meaning it inherits 500. Both entries MUST be
given explicit priorities in D16 to make the ordering well-defined.

#### 12.2 What the guard enforces on every STATE.md write

The guard reads **two sources** and compares them:

1. **Proposed full content:** reconstructed from the tool payload depending on tool type
   (see Proposed-content extraction table below).
2. **Current on-disk content:** `.factory/STATE.md` read via `host::read_file`.

**Proposed-content extraction by tool type:**

| Tool | Payload fields | How to obtain proposed full content |
|------|---------------|--------------------------------------|
| `Write` | `tool_input.content` (full file body) + `tool_input.file_path` | Use `tool_input.content` directly — it is the complete new file content |
| `Edit` | `tool_input.old_string` + `tool_input.new_string` (fragment) + `tool_input.file_path` + optional `tool_input.replace_all` (bool, default false) | Read on-disk content via `host::read_file`; replace first occurrence of `old_string` with `new_string` (or all occurrences if `replace_all` is true) to produce proposed content. If `old_string` is not found in on-disk content → **Continue** (fail-open: the tool itself will reject the edit; not the guard's job to duplicate that check) |
| `MultiEdit` | `tool_input.edits[]` (array of `{old_string, new_string, replace_all?}`) + `tool_input.file_path` | Read on-disk content; apply each element of `edits[]` sequentially in array order, same substitution logic as Edit. If any `old_string` is not found → **Continue** (fail-open; same rationale) |

**Why reconstruction is required (not optional):** Edit and MultiEdit deliver only a
fragment in the payload — there is no full-file field. The guard MUST reconstruct the
full proposed file by applying the edit to the on-disk content. Without reconstruction,
the guard can only check the fragment, which will never contain the `timestamp:` or
`factory_lock.expires_at` lines (those are in the frontmatter, which is typically NOT
the fragment being edited). An implementation that only checks the fragment will always
fail to find the timestamp fields and will silently fail-open on every Edit — making the
guard a no-op for the most common STATE.md mutation path.

**Time fields extracted from both sources** (see §12.4 for robust extraction spec):

| Field | Location | Condition checked |
|-------|----------|-------------------|
| `timestamp:` | Top-level frontmatter (between first `---` fences) | Proposed string value MUST differ from on-disk value (every STATE.md write must advance this field) |
| `factory_lock.expires_at` | Nested under `factory_lock:` in frontmatter | Proposed value MUST differ from on-disk value — **only when** `factory_lock.holder` is present and non-empty in the proposed content (i.e., a lock is held in the write being proposed) |

**"Differ"** means the string values are not byte-for-byte identical after extraction.
The guard does NOT parse values as datetimes — string inequality is sufficient and avoids
ISO-8601 edge-case parsing failures being misused as a bypass. The full datetime semantics
are enforced by `factory-lock-write.sh renew` (Mechanism 1, D10). The guard's job is to
detect "value did not change", not "value is correctly formatted".

**Canonical block message format** (using `HookResult::block_with_fix` from
`crates/hook-sdk/src/result.rs`):

The `block_with_fix` constructor signature is:
```rust
pub fn block_with_fix(hook: &str, reason: impl AsRef<str>, recommendation: impl AsRef<str>, code: &str) -> Self
```
It formats to: `BLOCKED by {hook}: {reason}. Fix: {recommendation}. Code: {code}.`
The `reason` segment MUST be human-readable text WITHOUT the code value embedded in it.

1. **TimestampStale:**
   ```rust
   HookResult::block_with_fix(
       "verify-state-timestamp-refresh",
       "STATE.md timestamp not advanced in this write",
       "Update `timestamp:` to the current UTC time before writing STATE.md",
       "TimestampStale",
   )
   ```
   Output: `BLOCKED by verify-state-timestamp-refresh: STATE.md timestamp not advanced in this write. Fix: Update 'timestamp:' to the current UTC time before writing STATE.md. Code: TimestampStale.`

2. **LockExpiryStale** (only when lock held in proposed content):
   ```rust
   HookResult::block_with_fix(
       "verify-state-timestamp-refresh",
       "factory_lock.expires_at not refreshed in this write while lock is held",
       "Run: factory-lock-write.sh renew .factory/STATE.md before writing STATE.md",
       "LockExpiryStale",
   )
   ```
   Output: `BLOCKED by verify-state-timestamp-refresh: factory_lock.expires_at not refreshed in this write while lock is held. Fix: Run: factory-lock-write.sh renew .factory/STATE.md before writing STATE.md. Code: LockExpiryStale.`

**The `[hook] Code: …` bracket form used in the prior draft of AC-005/006 strings is NOT
what `block_with_fix` produces and MUST NOT appear in the implementation or the AC text.**
The correct emitted form is the `BLOCKED by …` line above. The product-owner must correct
AC-005 and AC-006 strings to match this format (see AC-correction directive, §12.7).

#### 12.3 Fail-open vs fail-closed decisions

| Situation | Outcome | Rationale |
|-----------|---------|-----------|
| `file_path` does not resolve to `.factory/STATE.md` (after canonical-path normalization per §12.7 R6) | **Continue** immediately (no `host::read_file` called) | Out of scope; non-STATE.md writes are not subject to this guard |
| `tool_input.file_path` field absent or null in payload | **Continue** (fail-open) | Structurally unexpected; guard cannot identify the target file; err on the side of not blocking |
| On-disk STATE.md `host::read_file` fails (`CapabilityDenied`, `Timeout`, `NotFound`, etc.) | **Continue** + `log_warn` | Consistent with Decision 7 and `verify-factory-lock` PC6. A guard that permanently blocks writes on read-failure is the stale-lock footgun in a different costume. Required for first-ever STATE.md creation (file does not exist yet → `host::read_file` returns NotFound → Continue). |
| `Edit` or `MultiEdit`: `old_string` not found in on-disk content | **Continue** (fail-open) | The tool itself will reject the edit; guard's job is timestamp enforcement, not edit-applicability validation |
| On-disk frontmatter unparseable (malformed YAML fences or timestamp field) | **Continue** + `log_warn` | No valid prior value to compare against; consistent with `verify-factory-lock` MalformedLockBlock pattern |
| Proposed content frontmatter unparseable (malformed) | **Continue** + `log_warn` | Guard cannot determine the proposed timestamp; consistent with fail-open error policy |
| `timestamp:` absent in on-disk content (first write ever, or on-disk has no frontmatter) | **Continue** | No prior value to compare against; any write is valid |
| `timestamp:` absent in proposed content (state-manager omitted the field) | **Block: TimestampStale** | Every STATE.md write is required to include `timestamp:`. Absence of the field in the proposed write is itself a timestamp-not-advanced violation. |
| `timestamp:` present in both and byte-identical | **Block: TimestampStale** | Core enforcement: the timestamp was not advanced |
| `timestamp:` present in both and different | Continue (for this check) | Timestamp was advanced; proceed to LockExpiryStale check if applicable |
| No lock held in proposed content (`factory_lock` absent or `factory_lock.holder` absent/empty) | Skip LockExpiryStale check; `TimestampStale` check still applies | Lock is not held; `expires_at` is irrelevant |
| Lock held in proposed content AND `factory_lock.expires_at` unchanged vs on-disk | **Block: LockExpiryStale** | Renewal was not performed before this write; Mechanism 1 (D10) was skipped |
| Lock held AND `expires_at` advanced | Continue | Renewal was performed |
| Guard plugin crashed (`on_error = "continue"`) | **Continue** (fail-open) | Consistent with Decision 7 efficiency-class lock. Crash → advisory `internal.dispatcher_error` record in dispatcher log |

#### 12.4 Robust frontmatter time-field extraction

**Problem:** STATE.md is a YAML-frontmatter document delimited by `---` fences. The
`timestamp:` and `factory_lock.expires_at` fields are the operative time fields. A naive
substring scan (e.g., `lines().find(|l| l.starts_with("timestamp:"))`) can misread:
- A `timestamp:` key inside a nested YAML block that happens to have leading whitespace
- A quoted value: `timestamp: "2026-06-12T00:00:00Z"` — the extracted value would include
  the quotes, causing a false byte-identical comparison if one side is quoted and the other
  is not
- An edge line: `timestamp:   2026-06-12T00:00:00Z` (extra spaces)

**Required extraction algorithm:**

1. **Locate the YAML frontmatter block:** find the first `---` line (line 0 or first
   non-empty line); find the second `---` line; the frontmatter body is the text between
   them. If fewer than two `---` fences exist → unparseable → fail-open (§12.3 row 5).
2. **Extract top-level scalar keys only:** iterate lines in the frontmatter body. A
   top-level key line has the form `^<key>:` with NO leading whitespace (lines with
   leading whitespace are nested keys; skip them for top-level extraction). For a line
   matching `^timestamp:\s*(.+)`, trim whitespace from the capture group, then strip
   surrounding `"` or `'` quote characters (one layer only). The result is the canonical
   timestamp string.
3. **Extract `factory_lock.expires_at`:** use the existing `parse_factory_lock` function
   from `factory-lock-parse` crate (see §12.5) — it already handles the `factory_lock:`
   nested block correctly. Do not re-implement nested YAML parsing.
4. **`last_amended:` field:** this field is a freeform string starting with a date.
   For enforcement purposes, checking only `timestamp:` is sufficient — `last_amended:`
   is human-readable prose, not a machine-comparable value. Do NOT attempt to compare
   `last_amended:` for staleness.

**Key invariant:** the comparison MUST use the same extraction path for both on-disk and
proposed content. If on-disk uses raw-line extraction and proposed uses parsed extraction,
quote normalization differences will cause spurious false-positive blocks. Use the same
`extract_yaml_string_value` function on both sides — it already does quote stripping per
the `factory-lock-parse` implementation.

**Test requirement (D17 addition):** test-writer MUST add a fixture for the quoted
timestamp case:
- on-disk: `timestamp: 2026-06-12T00:00:00Z` (unquoted)
- proposed: `timestamp: "2026-06-12T01:00:00Z"` (quoted)
- Expected: Continue (values differ after normalization, even though one is quoted)

And the false-positive guard:
- on-disk: `timestamp: "2026-06-12T00:00:00Z"` (quoted)
- proposed: `timestamp: "2026-06-12T00:00:00Z"` (same quoted value)
- Expected: Block TimestampStale

#### 12.5 Shared parse logic — no duplication

The guard requires the same `factory_lock` frontmatter parse logic that `verify-factory-lock`
already implements and tests. Rather than duplicating line-by-line scan code in a new crate,
the `parse_factory_lock` function and supporting types (`LockState`, `extract_yaml_string_value`)
from `crates/hook-plugins/verify-factory-lock/src/lib.rs` are extracted to a shared location.

**Decision:** promote `parse_factory_lock`, `LockState`, and `extract_yaml_string_value`
from `verify-factory-lock::lib` to a new workspace-internal crate
`crates/hook-plugins/factory-lock-parse/`. Both `verify-factory-lock` and
`verify-state-timestamp-refresh` declare `factory-lock-parse` as a dependency.
The existing `verify-factory-lock` tests continue to pass unmodified — only the import path
changes from `crate::` to `factory_lock_parse::`. This is the production-grade path
(single implementation, single test surface) per CLAUDE.md Rule 1 and the no-duplication
principle. Creating two independent implementations of the same frontmatter scanner violates
this principle; the shared crate is mandatory.

The `timestamp:` field is a top-level YAML scalar key. The guard extracts it using the
same `extract_yaml_string_value` helper already in the shared crate (see §12.4 for the
extraction algorithm). No additional YAML parser is needed.

#### 12.6 Capability block (D16 registry entry)

The guard uses `host::read_file` on `.factory/STATE.md` to read the on-disk content.
It reads the proposed content from the tool payload directly (no host call needed for that).
It does NOT call `host::exec_subprocess`.

**CRITICAL: `ReadFileCaps` struct accepts ONLY `path_allow`.** The dispatcher's
`ReadFileCaps` struct definition in `crates/factory-dispatcher/src/registry.rs` is:

```rust
#[derive(Debug, Clone, Default, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct ReadFileCaps {
    pub path_allow: Vec<String>,
}
```

The `#[serde(deny_unknown_fields)]` attribute means ANY field not present in the struct
will cause the entire registry file to fail to load. Adding `max_bytes = 65536` or
`timeout_ms = 5000` under `[hooks.capabilities.read_file]` will break the registry load
and render ALL 52 plugins non-operational. The `max_bytes` and `timeout_ms` parameters
exist in the `host::read_file` WASM host ABI call arguments (passed by the WASM code at
call time), but they are NOT registry config fields.

The `HOST_ABI.md` specification shows `read_file(path, max_bytes, timeout_ms)` as call
parameters — they are passed from the plugin code itself, not from the TOML registry.

Required (and complete) capability block for D16:

```toml
[hooks.capabilities.read_file]
path_allow = [".factory/STATE.md"]
```

This is the only permissible form. Compare with the existing `verify-factory-lock` entry
at line 1181–1182 of `hooks-registry.toml`:

```toml
[hooks.capabilities.read_file]
path_allow = [".factory/STATE.md"]
```

Identical. No `max_bytes`, no `timeout_ms`. This is correct.

The `max_bytes` and `timeout_ms` values are specified in the WASM plugin source code itself
(e.g., `host::read_file(path, 65536, 5000)`), not in the TOML registry. The implementer
MUST NOT add these to the TOML registry entry.

Omitting `path_allow` entirely (or providing an empty list `path_allow = []`) still causes
`CapabilityDenied` → silent fail-open → guard is a no-op. The `path_allow` field must be
present and non-empty.

No `exec_subprocess` capability needed — the guard never shells out to `git` or any other
process. This eliminates the `env_allow` footgun class entirely and makes the registry
entry simpler than `verify-factory-lock`.

#### 12.7 Resolved open questions

**R1 — Scope:** `.factory/STATE.md` only. Triggers on Edit|Write|MultiEdit where
`tool_input.file_path` resolves to `.factory/STATE.md` after canonical-path normalization
(see R6 below). All other file paths return Continue immediately without reading any file.

**R2 — Fail-open/fail-closed:** Documented in §12.3 table. Summary: fail-closed
on the positive stale signal (that is the load-bearing case); fail-open on every error
path (consistent with Decision 7 and `verify-factory-lock` precedent).

**R3 — S-17.04 disposition:** Redirect now. Landing a known-superseded mechanism then
deleting it burns review cycles and ships a defective guard to an rc. The WASM guard
approach is the correct target; the Re-Scope Directive is precise enough for immediate
story-writer dispatch.

**R4 — Force-unlock audit event:** Unchanged. `factory-unlock-decide.sh` continues to
emit decision tokens; the `/factory-unlock` SKILL continues to emit the
`factory.lock.stolen` audit event via `emit-event`. The new WASM guard has no impact on
the unlock path.

**R5 — rc cadence:** The `verify-state-timestamp-refresh.wasm` plugin reaches the
operator cache only at the next rc tag. **rc.21 is HELD** pending S-17.04 and the
associated issue bundle (#128, #130, #169, #176, #170). The WASM guard is S-17.04's
primary deliverable. There is NO pre-rc interim period where the guard is absent but
state-manager is expected to advance timestamps — the guard and the obligation are
co-deployed.

**R6 — Canonical-path normalization rule (H03/EC-006 resolution):** The guard receives
`tool_input.file_path` as a raw string. Claude Code tools may send the path in various
forms depending on context (user-typed path, absolute path, normalized path). The guard
MUST apply robust normalization before comparing:

1. **Strip leading `./`**: `"./. factory/STATE.md"` → `".factory/STATE.md"`
2. **Strip `$CLAUDE_PROJECT_DIR/` prefix** if present: an absolute path starting with
   the project root directory is equivalent to the repo-relative path
3. **Collapse double slashes**: `".factory//STATE.md"` → `".factory/STATE.md"`
4. **Collapse `/./` segments**: `".factory/./STATE.md"` → `".factory/STATE.md"`

After normalization, compare to the canonical string `".factory/STATE.md"`.

**Do NOT fail-open on non-canonical matches.** The prior implementation comment claimed
double-slash stripping while the code did not perform it — a doc-vs-code lie that created
an evasion path. This spec requires robust normalization, not fail-open on
non-canonical paths. A path that normalizes to `.factory/STATE.md` MUST trigger the guard
regardless of how it was written. Fail-open only for genuinely unresolvable paths (e.g.,
path traversal sequences with `..` components that cannot be canonicalized relative to the
project root — treat these as "not `.factory/STATE.md`").

The canonical form to compare against is always `.factory/STATE.md` (relative, leading dot,
no leading slash, single forward slash between components, no trailing slash).

**R7 — Priority ordering (H02 resolution):** Resolved in §12.1. Explicit priority values
are mandated: `verify-factory-lock = 142`, `verify-state-timestamp-refresh = 143`. The D2
registry entry for `verify-factory-lock` (in the D16 spec section) MUST be updated to add
`priority = 142` if it is not already present. The D16 registry entry for
`verify-state-timestamp-refresh` MUST include `priority = 143`. This is a required change
to D2 as well as D16.

**R8 — Block message format (H04 resolution):** Resolved in §12.2. The canonical emitted
form is the `BLOCKED by {hook}: {reason}. Fix: {recommendation}. Code: {code}.` single line
from `HookResult::block_with_fix`. The `[hook] TimestampStale: …` bracket form that
appeared in the AC-005/006 strings is not what `block_with_fix` produces and must be
corrected in the ACs (see §12.8 AC-correction directive).

#### 12.8 AC-correction directive for product-owner (adversary pass 1 findings)

The following ACs in story S-17.04 contain incorrect content that must be corrected before
implementation. Product-owner owns these corrections; this directive is an architect
finding routed to the correct specialist per CLAUDE.md Companion Principle.

**AC-005 (TimestampStale block message string):**
- Current (incorrect): `[verify-state-timestamp-refresh] TimestampStale: STATE.md timestamp not advanced`
- Correct: `BLOCKED by verify-state-timestamp-refresh: STATE.md timestamp not advanced in this write. Fix: Update 'timestamp:' to the current UTC time before writing STATE.md. Code: TimestampStale.`
- Root cause: `block_with_fix` emits `BLOCKED by {hook}: …` format, not `[hook] Code: …` bracket format

**AC-006 (LockExpiryStale block message string):**
- Current (incorrect): `[verify-state-timestamp-refresh] LockExpiryStale: factory_lock.expires_at not refreshed`
- Correct: `BLOCKED by verify-state-timestamp-refresh: factory_lock.expires_at not refreshed in this write while lock is held. Fix: Run: factory-lock-write.sh renew .factory/STATE.md before writing STATE.md. Code: LockExpiryStale.`
- Root cause: same as AC-005

**AC-010 (registry entry for `verify-state-timestamp-refresh`):**
- Current (incorrect): capability block contains `max_bytes = 65536` and `timeout_ms = 5000` fields
- Correct: capability block MUST be `path_allow = [".factory/STATE.md"]` ONLY — no other fields
- Root cause: `ReadFileCaps` is `#[serde(deny_unknown_fields)]` with only `path_allow: Vec<String>`; extra fields break registry load
- Also add: `priority = 143` in the plugin entry (not in the capabilities block — at the entry level)
- Also add: `priority = 142` to the existing `verify-factory-lock` registry entry (amendment to D2 deliverable)

**EC-006 (canonical-path matching rule):**
- Current: absent or underspecified
- Correct: add an EC or clarifying note specifying the canonical-path normalization algorithm from §12.7 R6 above: strip `./`, strip `$CLAUDE_PROJECT_DIR/` prefix, collapse `//`, collapse `/./`; compare result to `.factory/STATE.md`; fail-open ONLY for genuinely unresolvable traversal paths

**New ACs for Write/Edit/MultiEdit coverage (proposed additions — product-owner decides exact AC numbering):**
- AC-NEW-WRITE: When the guard receives a Write tool payload for `.factory/STATE.md` with `tool_input.content` containing an unchanged `timestamp:` → Block TimestampStale
- AC-NEW-EDIT: When the guard receives an Edit tool payload for `.factory/STATE.md` with `tool_input.old_string` + `tool_input.new_string` that, after applying to on-disk content, produces unchanged `timestamp:` → Block TimestampStale
- AC-NEW-MULTIEDIT: When the guard receives a MultiEdit tool payload for `.factory/STATE.md` with `tool_input.edits[]` that, after sequential application, produces unchanged `timestamp:` → Block TimestampStale
- AC-NEW-NOOP-EDIT: When `old_string` is not found in on-disk content (Edit or MultiEdit) → Continue (fail-open; the tool itself will reject it)

## Concrete Deliverables

The following artifacts are required to implement this ADR. Story decomposition MUST
trace to each entry:

| # | Deliverable | Owner crate / path | Notes |
|---|-------------|-------------------|-------|
| D1 | New Rust crate `verify-factory-lock` | `crates/hook-plugins/verify-factory-lock/` → compiled to `plugins/vsdd-factory/hook-plugins/verify-factory-lock.wasm` | Native WASM plugin; uses `host::read_file` + `host::exec_subprocess`; no dispatcher changes; HOST_ABI_VERSION=1 unchanged |
| D2 | Registry entries for `verify-factory-lock` | `plugins/vsdd-factory/hooks-registry.toml` | Two entries: `PreToolUse` on `Edit\|Write\|Agent` and `PreToolUse` on `Bash`; `async = false` (REQUIRED — sync-group for block decisions); `on_error = "continue"`; `timeout_ms = 5000`; MUST include BOTH capability blocks: `[hooks.capabilities.read_file] path_allow = [".factory/STATE.md"]` AND `[hooks.capabilities.exec_subprocess] binary_allow = ["git"] env_allow = ["HOME", "GIT_CONFIG_GLOBAL", "XDG_CONFIG_HOME"]`. Omitting ANY sub-field within a capability block causes the relevant host call to return CapabilityDenied → plugin graceful-degrades to Continue → THE LOCK NEVER ENFORCES. Three confirmed footgun vectors: (1) read_file block absent; (2) exec_subprocess binary_allow absent; (3) exec_subprocess env_allow absent — env_clear() strips HOME, git config user.email returns empty, IdentityResolutionFailed → fail-open. All three must be enumerated explicitly. |
| D3 | STATE.md frontmatter schema extension | `factory_lock` block (`holder`, `locked_at`, `expires_at`) | `state-manager` is sole writer; absent block = unlocked; malformed block = unlocked (fail-open) |
| D4 | `/factory-lock` skill | `plugins/vsdd-factory/skills/factory-lock/SKILL.md` | Acquires lock via fetch-then-CAS push (same primitive as D6); emits `factory.lock.acquired`; mid-burst renewal path in state-manager |
| D5 | `/factory-unlock` skill | `plugins/vsdd-factory/skills/factory-unlock/SKILL.md` | Releases lock (holder only without `--force`; any developer with `--force`); emits `factory.lock.released` or `factory.lock.stolen`; delegates write to state-manager |
| D6 | `state-burst` blind-push fix | `plugins/vsdd-factory/skills/state-burst/SKILL.md` | Change blind `git push origin factory-artifacts` to fetch-then-`git push --force-with-lease=factory-artifacts:<sha>`; same primitive reused by D4 acquire |
| D7 | Lock-status surfacing in `factory-health` | `plugins/vsdd-factory/skills/factory-health/SKILL.md` | Show `Factory lock: FREE` / `HELD by this session (expires <time>)` / `HELD by <holder> since <locked_at> (expires <expires_at>)` |
| D8 | Lock-status surfacing in `factory-worktree-health` | `plugins/vsdd-factory/skills/factory-worktree-health/SKILL.md` | Same three-state display as D7 |
| D9 | Bats integration tests | `plugins/vsdd-factory/tests/` | Cover: lock blocked when held by other developer; read passes when locked; TTL expiry treated as unlocked; acquire CAS rejection on concurrent acquire; mid-burst renewal extends TTL; force-release emits audit event; single-developer unlocked path adds zero friction; capability-omitted registry entry graceful-degrades (advisory only) |
| D10 | `state-burst` SKILL renewal step | `plugins/vsdd-factory/skills/state-burst/SKILL.md` | Add mandatory step before `git -C .factory add -A` / `git commit`: `bash plugins/vsdd-factory/bin/factory-lock-write.sh renew .factory/STATE.md`. Annotate as no-op when unlocked. Also add anti-pattern row: "Skipping renew before git add while lock held → `verify-state-timestamp-refresh` WASM guard blocks the subsequent write (LockExpiryStale)." Reuses existing `factory-lock-write.sh renew` subcommand (S-17.01 deliverable). **RETAINED in v1.6.** |
| ~~D11~~ | ~~`verify-lock-renewal.sh` PreToolUse gate~~ | ~~`plugins/vsdd-factory/hooks/verify-lock-renewal.sh`~~ | **WITHDRAWN in v1.6.** Superseded by D16 (`verify-state-timestamp-refresh` WASM guard — enforces freshness at write-time, not at push-time). Do NOT implement. |
| ~~D12-registry~~ | ~~Registry entry for `verify-lock-renewal.sh`~~ | ~~`plugins/vsdd-factory/hooks-registry.toml`~~ | **WITHDRAWN in v1.6.** No `verify-lock-renewal` entry is added to `hooks-registry.toml`. Do NOT implement. |
| D13 | `state-manager.md` obligation amendment | `plugins/vsdd-factory/agents/state-manager.md` | Amendment to existing §"factory_lock Write/Renewal/Clear Obligation": add cross-reference sentence at the end of §Sequencing invariants Invariant 2 pointing to the `state-burst` SKILL renew step (D10) as the executable enforcement mechanism, and noting that `verify-state-timestamp-refresh` (D16) enforces it at the WASM hook layer. **RETAINED in v1.6.** |
| ~~D14~~ | ~~Bats tests for Decision 11 (`verify-lock-renewal.bats`)~~ | ~~`plugins/vsdd-factory/tests/verify-lock-renewal.bats`~~ | **WITHDRAWN in v1.6.** The bash gate no longer exists. Renewal-check logic is tested at D17 (Rust unit tests + bats for `verify-state-timestamp-refresh`). Do NOT create `verify-lock-renewal.bats`. |
| D15 | Shared `factory-lock-parse` crate | `crates/hook-plugins/factory-lock-parse/` | New workspace-internal library crate. Promotes `parse_factory_lock`, `LockState`, `extract_yaml_string_value`, `parse_iso8601` from `verify-factory-lock::lib` to this shared crate. `verify-factory-lock` and `verify-state-timestamp-refresh` both depend on it. `verify-factory-lock/src/lib.rs` changes import paths from `crate::` to `factory_lock_parse::` — logic and tests unchanged. All existing `verify-factory-lock` tests continue to pass unmodified. No `serde_yaml`/`serde_norway` (manual line-by-line scan per Architecture Compliance Rule 4). `chrono` as workspace dep. |
| D16 | `verify-state-timestamp-refresh` WASM plugin + registry entry + priority amendment to `verify-factory-lock` entry | `crates/hook-plugins/verify-state-timestamp-refresh/` → `plugins/vsdd-factory/hook-plugins/verify-state-timestamp-refresh.wasm`; registry entry in `plugins/vsdd-factory/hooks-registry.toml`; also add `priority = 142` to existing `verify-factory-lock` entry | New PreToolUse guard. See Decision 12 for full spec. Crate pattern identical to `verify-factory-lock`: `[lib]` with pure `guard_logic(payload, callbacks)` injectable for unit tests + `[[bin]]` WASI entry point. Uses `factory-lock-parse` for `parse_factory_lock` and `extract_yaml_string_value`. Registry entry: `event = "PreToolUse"`, `tool = "Edit\|Write\|MultiEdit"`, `async = false` (REQUIRED per ADR-019), `on_error = "continue"`, `priority = 143`, `timeout_ms = 5000`. Capability block: `[hooks.capabilities.read_file]` with `path_allow = [".factory/STATE.md"]` ONLY — NO `max_bytes`/`timeout_ms` (ReadFileCaps is `#[serde(deny_unknown_fields)]` with only `path_allow: Vec<String>`; extra fields break registry load). No `exec_subprocess` capability needed. `max_bytes` and `timeout_ms` values are passed as arguments in the WASM plugin source code at `host::read_file` call sites, not in TOML. |
| D17 | Rust `#[test]` unit coverage + bats integration tests for `verify-state-timestamp-refresh` | `crates/hook-plugins/verify-state-timestamp-refresh/src/lib.rs`; `plugins/vsdd-factory/tests/verify-state-timestamp-refresh.bats` | Table-driven unit tests via injectable callbacks (matching `verify-factory-lock` test pattern). MUST cover: (a) Write payload, lock held, `factory_lock.expires_at` unchanged → Block LockExpiryStale; (b) Write payload, lock held, `expires_at` advanced → Continue; (c) Write payload, no lock held, `timestamp:` unchanged → Block TimestampStale; (d) Write payload, no lock held, `timestamp:` advanced → Continue; (e) Write payload, proposed content frontmatter unparseable → Continue (fail-open); (f) on-disk `host::read_file` fails (any HostError) → Continue (fail-open); (g) `file_path` not STATE.md (after normalization) → Continue immediately (no read_file called); (h) `timestamp:` absent in on-disk content → Continue; (i) `timestamp:` absent in proposed content → Block TimestampStale; (j) Edit payload, `old_string` found, reconstructed full content has stale `timestamp:` → Block TimestampStale; (k) Edit payload, `old_string` found, reconstructed full content has advanced `timestamp:` → Continue; (l) Edit payload, `old_string` NOT found in on-disk content → Continue (fail-open); (m) Edit payload with `replace_all=true`, all occurrences replaced, reconstructed content has advanced `timestamp:` → Continue; (n) MultiEdit payload, all edits apply, reconstructed content has stale `timestamp:` → Block TimestampStale; (o) MultiEdit payload, first edit's `old_string` not found → Continue (fail-open); (p) quoted `timestamp:` value normalization — on-disk unquoted, proposed quoted but different value → Continue (no false positive); (q) quoted `timestamp:` value normalization — both sides same quoted value → Block TimestampStale; (r) canonical-path normalization — `file_path = "./. factory/STATE.md"` → triggers guard (same as unadorned path); (s) file_path with `$CLAUDE_PROJECT_DIR/` prefix → triggers guard. Bats integration tests MUST cover: Write happy path (advanced timestamp → exit 0), Write stale path (unchanged timestamp → exit 2 with `BLOCKED by verify-state-timestamp-refresh` canonical message), Edit happy path (reconstructed content has advanced timestamp → exit 0), non-STATE.md path (`file_path = ".factory/OTHER.md"` → exit 0 without read_file). |

## Rationale

### Why native WASM over a new bash hook sibling

The existing `verify-git-push.sh` is a bash hook routed via `legacy-bash-adapter.wasm`.
A new bash sibling (`verify-factory-lock.sh`) is also viable, but a native WASM plugin
is preferred for the guard because:

1. The guard needs structured frontmatter parsing (YAML subset). Doing this robustly in
   bash requires `awk`/`grep` heuristics that are brittle on edge cases (e.g., multiline
   YAML values, quoted strings with colons). A native Rust implementation is precise and
   testable via `cargo test`.
2. The plugin ecosystem already has the exact pattern: `validate-artifact-path.wasm` is a
   native WASM PreToolUse guard using `host::read_file`. The structure is established and
   the crate scaffolding is known.
3. Binary allow-list for `exec_subprocess` is already proven for `["git"]` in
   `capture-commit-activity.wasm` (registry lines ~65–80). No new host capability is needed.

### Why `async = false` is mandatory (not optional)

The dispatcher partitions plugins into sync-group and async-group at execution time
(ADR-019). Only sync-group plugins participate in the block decision aggregated at
`executor.rs:100–117`. An `async = true` plugin's `block_intent` is ignored for the
PreToolUse gate — it becomes advisory telemetry only (per ADR-019 CI lint invariant:
`on_error=block ⇒ async=false`). For `verify-factory-lock`, blocking is the entire
purpose; `async = true` would silently reduce it to a no-op blocker. `async = false` is
therefore a correctness requirement, not a performance preference.

### Why capability blocks must be enumerated completely (deny-by-default)

The dispatcher enforces capability deny-by-default: a `host::read_file` call without a
matching `[hooks.capabilities.read_file]` block returns `CapabilityDenied`, which causes
the Rust plugin to graceful-degrade to `Continue` (no block, no error — invisible). The
same applies to `exec_subprocess`, and the deny-by-default principle applies equally to
every sub-field within a capability block.

Three confirmed silent-no-op footgun vectors for `verify-factory-lock`:

1. **read_file block absent** — `host::read_file` returns `CapabilityDenied`; plugin
   cannot read STATE.md; graceful-degrades to `Continue`; lock never enforces.
2. **exec_subprocess binary_allow absent (or does not list `"git"`)** — `host::exec_subprocess`
   returns `CapabilityDenied`; plugin cannot invoke `git config user.email`; graceful-degrades
   to `Continue`; lock never enforces.
3. **exec_subprocess env_allow absent (or does not include `"HOME"`)** — the dispatcher's
   `exec_subprocess` host function calls `env_clear()` and passes ONLY the env vars listed
   in `caps.env_allow`. Without `HOME` (and optionally `GIT_CONFIG_GLOBAL` /
   `XDG_CONFIG_HOME`) in `env_allow`, `git config user.email` cannot read the developer's
   global gitconfig → returns empty string → plugin hits `IdentityResolutionFailed` →
   fails open (`Continue`) → lock never enforces. This is the same deny-by-default
   silent-no-op class as vectors 1 and 2, surfaced via the env-isolation axis rather than
   the binary-allow axis. Discovered in S-17.02 TDD implementation; codified as v1.3
   [process-gap] amendment.

An implementer who scaffolds the registry entry from a minimal template and omits any of
these three fields ships a lock plugin that is indistinguishable from a working guard until
a concurrent-session incident reveals it. Enumerating all three sub-fields explicitly in D2
closes all three footguns.

### Why `on_error = "continue"` (fail-open) rather than `on_error = "block"`

This is an advisory/efficiency-class lock (Kleppmann §8 distinction). Fail-open is
correct for efficiency-class locks: a guard crash's worst case is a missed block, bounded
by Decision 8's CAS push (which still rejects the concurrent push at the network layer).
`on_error = "block"` would make a guard crash equivalent to a permanent lock — the factory
is wedged until the plugin is repaired or the registry is manually edited. That failure
mode is worse than the one being guarded against. See Decision 7.

### Why explicit `/factory-lock` acquire rather than auto-on-first-write

Auto-acquire creates an invisible state transition: the developer doesn't know they own
the lock, doesn't know when they acquired it, and has no obvious way to release it. On
crash, they can't tell if they left a lock behind. Explicit acquire makes ownership
visible and intentional, matches the "user that locked it" mental model confirmed by human
review, and makes the release step natural. The TTL auto-expiry (Decision 5 Path A)
handles the crash case without requiring auto-acquire.

### Why `git config user.email` and not a composite session identity

Composite identity (`hostname::pid::CLAUDE_SESSION_ID`) requires `CLAUDE_SESSION_ID` to
be set in the environment, which is Claude-Code-specific. The factory is designed to be
host-agnostic; requiring a Claude-specific env var in the guard logic couples the guard to
Claude Code's session model. `git config user.email` is universally available (the factory
already requires git), human-readable, and sufficient for the intended threat model
(Developer A vs Developer B). The tradeoff (self-vs-self not blocked) is accepted and
documented.

### Why the blind-push fix is a separate deliverable, not the primary mechanism

The blind-push fix is a guard at the push layer: it detects concurrent pushes after the
work is done. The WASM guard is a check before work begins: it blocks mutating operations
before they produce commits. These are complementary layers. The push fix is the safety
net for self-vs-self and guard-crash scenarios; the WASM guard is the proactive block for
cross-developer scenarios. Both are needed and neither subsumes the other.

## Consequences

### Positive

- **Eliminates the primary cross-developer race** (Developer A vs Developer B) by blocking
  all mutating factory operations when another developer holds the lock.
- **Acquire is CAS-protected:** two simultaneous `/factory-lock` attempts produce one
  success and one actionable rejection (non-fast-forward push error), closing the
  primary TOCTOU acquire-race (CWE-367).
- **Zero infrastructure overhead:** the entire mechanism runs locally (STATE.md read,
  git email query, timestamp comparison). No network calls in the guard hot path.
- **Human-readable lock state:** any developer can `cat .factory/STATE.md` to see who
  holds the lock. No opaque remote state.
- **Blind-push fix (Decision 8) delivers immediate standalone value:** concurrent pushes
  are detected rather than silently clobbered, even without the WASM guard.
- **Actionable failure messages (Decision 4):** blocked developer knows exactly who holds
  the lock, when it expires, and how to force-release.
- **Fail-open on guard crash (Decision 7):** a broken guard never wedges the factory; its
  worst case is bounded by Decision 8's CAS push.
- **Single-developer behavior unchanged:** the unlocked happy path adds zero friction
  beyond a one-time `/factory-lock` at session start (Decision 10).

### Negative / Trade-offs

- **Cooperative, not mandatory:** a developer who does not run `/factory-lock` bypasses
  the protection entirely. The lock is advisory in practice. The primary value is surfacing
  and blocking accidental concurrent work by well-intentioned developers — not preventing
  a determined adversary.
- **Residual TOCTOU acquire-race (CWE-367):** the exact-simultaneity window between two
  sessions' fetch and push steps is narrowed to milliseconds by the CAS push but not
  eliminated. See Decision 6 honest statement. Eliminated by the Decision 9 git-ref-CAS
  future path.
- **Long-burst TTL self-eviction residual risk:** bursts significantly longer than 45
  minutes between intermediate commits can self-evict mid-burst even with mid-burst renewal.
  Residual risk attributed to Decision 9 git-ref-CAS fencing path. See Decision 5 failure
  mode subsection.
- **Self-vs-self not protected:** same developer in two sessions shares the same git email
  and will not be blocked by the guard. Mitigated by the blind-push fix (Decision 8) and
  the `factory-health` observability surfacing.
- **45-minute maximum wedge on crash:** a developer whose session crashes without running
  `/factory-unlock` blocks others for up to 45 minutes. Break-glass `/factory-unlock
  --force` is always available. Acceptable for expected team size and session cadence.
- **Guard depends on local factory-artifacts being current:** the guard reads the local
  STATE.md. If a developer has not fetched `factory-artifacts` recently, the guard's view
  of the lock is stale. The burst fetch (Decision 8) and the fetch in `/factory-lock`
  acquire (Decision 6) mitigate this for the write path.
- **Capability footgun at implementation time:** three confirmed silent-no-op vectors —
  (1) read_file block absent, (2) exec_subprocess binary_allow absent, (3) exec_subprocess
  env_allow absent (env_clear() strips HOME; git config user.email returns empty;
  IdentityResolutionFailed → fail-open). All three explicitly documented in D2 and
  Rationale (v1.3). The bats test in D9 MUST cover all three omission cases.

### Status as of v1.6 (amended, 2026-06-12)

Human direction confirmed + adversary pass 1 incorporated: move enforcement into the Rust
hook system, replacing the PreToolUse bash gate with a WASM guard that inspects the
proposed full write content before it lands on disk. Decision 12 added
(`verify-state-timestamp-refresh` WASM guard with per-tool reconstruct semantics).
Decision 11 Mechanism 2 (D11/D12-registry/D14) withdrawn. Decision 11 Mechanism 1 (D10)
retained. Push-time enforcement dropped. S-17.04 redirected to v1.2. All architect open
questions resolved. Adversary pass 1 corrections applied: payload field root cause fixed
(`new_content` → `content`/`old_string`+`new_string`/`edits[]` with full-content
reconstruction); registry caps corrected (`path_allow`-only per `ReadFileCaps` struct);
explicit priorities mandated (142/143); canonical-path normalization rule specified;
block message format corrected to real `block_with_fix` segments; robust frontmatter
extraction with quote normalization specified. AC-correction directive issued for
product-owner. ARCH-INDEX v2.21→v2.22 pending (state-manager bump in follow-up
codification burst).

### Additional positive consequences of Decision 12 (v1.6)

- **All four bypass vectors eliminated structurally by construction.** The trigger is a
  structured `file_path` field, not a command string — there is nothing to tokenize,
  over-match, inject newlines into, or env-substitute.
- **Stronger invariant than push-time enforcement.** Freshness is guaranteed at the moment
  of the write, not retrospectively at push time. Commit and push inherit freshness.
- **No push-time gate required.** `factory-cas-push.sh` remains a plain CAS push. Renewal
  is guaranteed to be in the commit by the time it reaches the push step.
- **`parse_factory_lock` deduplicated.** Promoting to `factory-lock-parse` crate removes
  the risk of the same frontmatter scanner diverging between two WASM plugins.
- **No `exec_subprocess` needed.** The new guard has a simpler capability surface than
  `verify-factory-lock` — no `git config user.email` call, no `env_allow` footgun class.
- **Timestamp discipline enforced across all STATE.md writes**, not only during a held-lock
  burst. Every write that does not advance `timestamp:` is blocked regardless of lock state.

### Additional negative consequences / trade-offs of Decision 12 (v1.6)

- **rc-cadence gate.** The WASM guard reaches the operator cache only at rc.21 (held
  pending this story). During develop, the guard is absent; Mechanism 1 (D10) alone enforces
  the obligation via SKILL discipline. rc.21 is the co-deployment point.
- **`factory-lock-parse` crate extraction is in-scope.** Refactoring `verify-factory-lock`
  to import from a shared crate is required work in S-17.04. It expands scope slightly but
  prevents an immediate duplication debt.
- **S-17.04 story rework.** The in-flight branch requires a v1.2 rework (Re-Scope Directive
  below). Sunk cost: the 16 bats tests for prior ACs are mostly retained as CLI-contract
  tests for the existing lock helpers; the 12 Red Gate tests for the bash gate are replaced
  by Rust unit tests in the new WASM crate.

### Status as of v1.5 (amended, 2026-06-11)

Human design confirmed. Research-agent verification APPROVE-WITH-FIXES incorporated (v1.2).
v1.3 [process-gap] amendment incorporated: exec_subprocess env_allow footgun closed; D2
canonical registry form updated. v1.4 [S-17.04] amendment incorporated: Decision 11 added
(automatic heartbeat renewal enforcement — executable state-burst SKILL step + PreToolUse
push gate); Decision 5 vestigial burst-end-only sentence corrected; Deliverables D10–D14
added; BC-5.40.001 PC4 confirmed unaffected. v1.5 [S-17.04 adversary F-1701-001] amendment
incorporated: Decision 11 Mechanism 2 gate-trigger corrected (primary trigger is
`factory-cas-push` helper, not raw `git push` — the real push runs as subprocess inside the
helper and is invisible to PreToolUse; v1.4 trigger was functionally inert on the production
SKILL path); block message reconciled to legacy-bash-adapter one-liner form (multi-line
text is truncated to first line by the adapter; single-line `block_pre` form is the correct
contract); D12 `binary_allow` extended to `["bash", "git", "jq"]` (gate script execs `jq`
to parse STATE.md frontmatter; omitting `jq` → CapabilityDenied → silent fail-open → gate
inert — fourth instance of the deny-by-default silent-no-op footgun class). No further
human-gated questions remain. All eleven decisions are final. D-540 codification recorded by
state-manager 2026-06-10. Implementation stories may be dispatched; S-17.04 implements
Decision 11.

### Why Decision 11 Mechanism 2 was the correct v1.5 design and why Decision 12 supersedes it (v1.6)

Decision 11 Mechanism 2 was the right engineering response given the constraint that the
enforcement needed to fire at a specific push event and that only a Bash command string
was available to identify that event. The four bypass vectors (inert-match, over-match,
newline-injection, env-injection) are all properties of parsing an untrusted command string.
Given only a command string, the v1.5 design was the best achievable.

Decision 12 supersedes it because the enforcement point changes entirely: instead of
blocking the push after checking whether a renewal was committed, we block the **write**
itself if the renewal is absent. The PreToolUse Edit/Write `file_path` is a structured
field set by the tool infrastructure — not user text, not a command string. All four bypass
vectors vanish structurally because their precondition (a command string to parse) no longer
exists at the new enforcement point.

This is also the more correct invariant: "STATE.md always carries a current timestamp at the
moment it is written" is stronger than "STATE.md carried a current timestamp by push time."
The guard enforces freshness at the write; the commit and the push inherit freshness from the
write. No separate push-time gate is needed.

**Why a new WASM crate, not a dispatcher subcommand:** The hook SDK's three-outcome contract
(Continue / Block / Error) cannot inject content into a write — it can only allow or block.
The `factory-dispatcher lock cas-push` chokepoint idea (prior draft of v1.6) would have
needed to re-read the write content after the fact (PostToolUse, which cannot block) or
intercept it before (but the write content isn't available at PostToolUse without re-reading
the file). A PreToolUse WASM guard that reconstructs proposed full content from the tool
payload fields (`tool_input.content` for Write; `tool_input.old_string`+`new_string` for
Edit; `tool_input.edits[]` for MultiEdit) and compares against on-disk is the only mechanism
in the hook SDK that can inspect content before it lands on disk.

**Why redirect S-17.04 now, not land-then-supersede:** Landing D11/D12/D14 then deleting
them in a follow-up story violates CLAUDE.md Rule 2 ("ship each cycle production-grade").
The human approved the redirect. The total sunk cost in v1.1 is modest; the permanent
benefit (no four-vector-vulnerable mechanism ever in the codebase) is disproportionate.

## Alternatives Considered

- **Git ref `refs/factory-lock/<repo-slug>` CAS as primary enforcement (v1.0 design):**
  Demoted to Future/Out of Scope (Decision 9). Requires empirical GitHub.com CAS
  verification probe and adds server-side state management complexity not warranted by the
  threat model. Preserved as the correct upgrade path if the threat model escalates, and as
  the fencing-token mechanism that eliminates the residual TOCTOU and self-eviction risks.

- **New bash hook sibling `verify-factory-lock.sh` (via legacy-bash-adapter):** Viable
  but rejected in favor of native WASM. Bash YAML parsing is brittle; a native Rust
  crate is precise and unit-testable. The `validate-artifact-path.wasm` pattern is already
  established and should be followed.

- **Extend `verify-git-push.sh` for lock enforcement:** Rejected. `verify-git-push.sh`
  has a narrow declared scope (block raw force + protected branches). Conflating lock
  semantics widens its scope and testing surface without benefit.

- **Auto-acquire lock on first write:** Rejected. Creates invisible state transitions,
  complicates crash-recovery reasoning, and conflicts with the "user that locked it"
  mental model confirmed by human review. Explicit acquire is correct.

- **`on_error = "block"` for the guard:** Rejected. An efficiency-class lock (Kleppmann §8)
  that permanently wedges the factory on guard crash is a worse failure mode than the one
  it guards against. `on_error = "continue"` with Decision 8's CAS push as safety net is
  correct for this lock class.

- **Composite session identity (hostname + pid + claude-session-id):** Rejected.
  Requires `CLAUDE_SESSION_ID` env var (Claude-Code-specific); couples the guard to
  Claude's session model; insufficient benefit for the actual threat model. Git user.email
  is universal, human-readable, and sufficient (Decision 3).

- **Burst-end-only TTL renewal (v1.1 design):** Rejected in v1.2. Burst-end-only renewal
  allows a burst longer than the TTL to self-evict. Mid-burst renewal at each intermediate
  `state-manager` commit is the production-grade fix (Decision 5).

- **Per-story granularity lock:** Considered but rejected. The race window is not limited
  to story delivery; any `state-manager` write is a potential concurrent write. Whole-factory
  granularity is simpler and conservative.

## Process Note

**[process-gap]:** Capability enumeration completeness must include `env_allow` for any
guard whose subprocess depends on ambient environment configuration. `git config user.email`
reads from the developer's global gitconfig, which requires `HOME` to locate
`~/.gitconfig`. The dispatcher's `env_clear()` strips all ambient env vars before
subprocess execution; only vars listed in `caps.env_allow` are forwarded. Omitting
`env_allow` from the `exec_subprocess` capability block silently breaks identity resolution
via the same deny-by-default path as omitting the capability block itself. This footgun was
discovered during S-17.02 TDD implementation and codified in v1.3. The routing obligation
per ADR-024 Process Note applies: implementer TDD findings that change behavior the ADR's
canonical registry form specifies MUST route an architect ADR amendment in the same burst.

## Source / Origin

- **Issue:** [#170](https://github.com/drbothen/vsdd-factory/issues/170) —
  `feat(state): single-writer factory lock/lease — prevent concurrent developers racing
  the same repo's factory-artifacts state`
- **Research cache:** `.factory/research/issues/issue-170.md` (VALID-NEW, High confidence;
  2026-06-09) — primary research sources: git-scm `git-push` man page on
  `--force-with-lease` CAS semantics; kubernetes.io Lease API; etcd.io lease/lock docs;
  LWN `https://lwn.net/Articles/817905/` (POSIX lock failure modes); Kleppmann §8
  "Leases and Lease-Based Locks" (efficiency-vs-correctness distinction; long-operation
  TTL hazard; fencing token requirement).
- **TOCTOU acquire-race:** CWE-367 (Time-Of-Check Time-Of-Use Race Condition). The
  fetch→check→push window is bounded but not zero; see Decision 6 residual window statement.
- **Blind push confirmed at:** `skills/state-burst/SKILL.md` push call (`git push origin
  factory-artifacts`) — no CAS, no fetch, confirmed by research cache codebase grounding.
- **Push-hook gap confirmed at:** `hooks/verify-git-push.sh` — allows `factory-artifacts`
  pushes and `--force-with-lease` with no exclusivity check, confirmed by research cache.
- **Native WASM guard pattern:** `crates/hook-plugins/validate-artifact-path/` +
  `hook-plugins/validate-artifact-path.wasm` — the closest existing analogue (PreToolUse,
  `host::read_file`, `on_error = "continue"`).
- **`exec_subprocess` binary_allow=["git"] pattern:** `crates/hook-plugins/capture-commit-activity/`
  + registry entry `capture-commit-activity` (hooks-registry.toml lines ~65–80).
- **Host ABI version:** `crates/hook-sdk/src/lib.rs:65` — `HOST_ABI_VERSION: u32 = 1`.
  No change to dispatcher or ABI is required.
- **Block path in dispatcher:** `crates/factory-dispatcher/src/executor.rs:609`
  (`plugin_requests_block` function) invoked at `executor.rs:105–108` for sync-group
  plugins. `async = false` is required for the guard to participate in this path.
- **Sync/async partition:** ADR-019 (Plugin Async Semantics at Registry Layer) — CI lint
  invariant `on_error=block ⇒ async=false`; async plugins are advisory-only for block
  decisions.
- **Capability deny-by-default:** confirmed against `crates/factory-dispatcher/src/executor.rs`
  and `hooks-registry.toml` registry patterns. Missing capability blocks return
  `CapabilityDenied` → plugin graceful-degrades to `Continue`.
- **Within-session discipline:** `agents/state-manager.md`, TD-VSDD-053 (single-commit
  burst) — provides the within-session model that this ADR extends cross-session.
- **Decision D-540:** codification decision for this ADR in the v1.0-brownfield-backfill
  cycle decision log.
- **ADR cross-references:** ADR-016 (artifact path guard pattern; `on_error = "continue"`
  precedent), ADR-019 (push hook allow-list semantics; sync/async partition), ADR-020
  (Class A latency budget ≤1500ms p95 that governs the new guard's hook budget), ADR-013
  (cycle-keyed adversarial review structure, which this ADR protects from concurrent
  clobbering).
- **Human design review:** 2026-06-10 — all ten decisions confirmed. Primary enforcement
  changed from git-ref CAS to WASM guard; identity simplified to git user.email; acquire
  made explicit; fail-open on crash confirmed.
- **Research-agent verification:** 2026-06-10 — APPROVE-WITH-FIXES. Five fixes incorporated
  in v1.2: (1) acquire-race CAS fix + CWE-367 honest statement; (2) long-burst TTL failure
  mode + mid-burst renewal + fencing residual risk; (3) capability block enumeration in D2;
  (4) async=false sync-group requirement in D2; (5) Kleppmann efficiency-vs-correctness
  framing in Decision 7.
- **v1.3 [process-gap] amendment:** 2026-06-11 — S-17.02 TDD implementation finding.
  `exec_subprocess` capability block spec was missing `env_allow`. The dispatcher's
  `exec_subprocess` host function calls `env_clear()` before spawning the subprocess and
  passes only vars listed in `caps.env_allow`; without `HOME`, `git config user.email`
  returns empty → `IdentityResolutionFailed` → fail-open → silent no-op guard. Third
  instance of the deny-by-default silent-no-op footgun class. Fix: D2 canonical registry
  form updated to `env_allow = ["HOME", "GIT_CONFIG_GLOBAL", "XDG_CONFIG_HOME"]` on the
  `exec_subprocess` block; Rationale section updated to enumerate all three footgun vectors;
  Process note and Consequences bullet updated. ARCH-INDEX v2.19→v2.20. Issue #170, S-17.02.
- **v1.4 [S-17.04] amendment:** 2026-06-11 — enforcement wiring for BC-5.40.001 PC4. Gap:
  `state-burst` SKILL had no call to `factory-lock-write.sh renew` before `git add`/commit
  despite state-manager.md §obligation table requiring it. Decision 11 added: (1) mandatory
  executable `renew` step in `state-burst` SKILL before staging (Mechanism 1 — reuses
  existing `factory-lock-write.sh renew` from S-17.01, no new script); (2) new
  `verify-lock-renewal.sh` PreToolUse bash hook that blocks a held-lock `factory-artifacts`
  push when HEAD `expires_at` equals `origin/factory-artifacts` `expires_at` (RenewalMissed),
  `on_error=continue`, `async=false`, no-op when unlocked or no remote baseline (Mechanism 2).
  Decision 5 vestigial "burst END" sentence corrected to "every commit in a burst, not only
  at burst-close." Deliverables D10–D14 added. BC-5.40.001 PC4 confirmed unaffected.
  ARCH-INDEX v2.20→v2.21 (pending state-manager row update + version bump). S-17.04.
- **v1.5 [S-17.04 adversary F-1701-001] amendment:** 2026-06-11 — gate-trigger fix,
  block-message reconciliation, D12 jq capability sync. (1) Decision 11 Mechanism 2
  trigger: v1.4 specified `git.*push.*factory-artifacts` as the Bash command pattern. This
  is inert on the production push path: post-S-17.01 the state-burst SKILL runs
  `bash plugins/vsdd-factory/bin/factory-cas-push.sh`; the real `git push --force-with-lease`
  is a subprocess inside that helper — PreToolUse never inspects subprocess command strings.
  Corrected trigger: primary pattern is `.tool_input.command` contains `factory-cas-push`;
  secondary pattern `git`+`push`+`factory-artifacts` is retained belt-and-suspenders for
  hand-typed raw pushes. The check-timing analysis is unchanged: at PreToolUse on
  `bash factory-cas-push.sh`, the burst commit already exists locally (HEAD STATE.md carries
  this burst's `expires_at`), so the HEAD-vs-origin comparison is valid. (2) Block message:
  the legacy-bash-adapter truncates output to first line; the multi-line v1.4 message
  was unreachable. Reconciled to one-liner: `BLOCKED by verify-lock-renewal: RenewalMissed —
  factory_lock held but expires_at not refreshed in this burst. Fix: Run:
  factory-lock-write.sh renew .factory/STATE.md Code: RenewalMissed.` (3) D12
  `binary_allow`: extended from `["bash", "git"]` to `["bash", "git", "jq"]`; gate script
  execs `jq` to parse STATE.md JSON envelope; omitting `jq` → CapabilityDenied → silent
  fail-open → gate inert (fourth deny-by-default silent-no-op vector). S-17.04, F-1701-001.
- **v1.6 [S-17.04 redirect — human approved; adversary pass 1 incorporated] amendment:**
  2026-06-12 — WASM hook adoption + adversary pass 1 corrections. Human requirement:
  "make sure the time is updated on the state every time the state is touched — match
  existing patterns — move to a Rust-based hook system." Hook SDK constraint confirmed:
  three outcomes only (Continue/Block/Error); no mutate/rewrite-content outcome; enforcement
  must be Block-on-stale, not inject-timestamp. Decision 12 added: `verify-state-timestamp-refresh`
  new WASM PreToolUse guard crate (`crates/hook-plugins/verify-state-timestamp-refresh/`);
  triggers on Edit|Write|MultiEdit where `tool_input.file_path` resolves to
  `.factory/STATE.md` (canonical-path normalization per §12.7 R6); proposed full content
  reconstructed per tool type: Write→`tool_input.content`; Edit→on-disk+old/new_string
  reconstruct; MultiEdit→sequential `edits[]` apply; blocks TimestampStale /
  LockExpiryStale; fail-open on parse/IO errors per Decision 7 precedent;
  `host::read_file` capability only with `path_allow = [".factory/STATE.md"]` ONLY
  (ReadFileCaps has no `max_bytes`/`timeout_ms` fields — validated against `registry.rs`).
  Explicit priorities added: verify-factory-lock=142, verify-state-timestamp-refresh=143.
  Block messages corrected to real `block_with_fix` format. Robust frontmatter extraction
  with quote normalization specified (§12.4). Shared crate `factory-lock-parse` added
  (D15). D16 = guard crate + registry entry + `verify-factory-lock` priority amendment.
  D17 = Rust unit tests (19 cases) + bats integration tests (4 cases).
  Decision 11 Mechanism 2 (D11/D12-registry/D14) withdrawn. Push-time enforcement dropped.
  `factory-cas-push.sh` unchanged. Decision 11 Mechanism 1 (D10) retained.
  S-17.04 redirected to v1.2. INV-019 cure: (a) Decision 12 added; (b) D11/D12-registry/D14
  withdrawn, D15/D16/D17 added; (c) S-17.04 Re-Scope Directive issued. AC-correction
  directive for product-owner issued (§12.8): AC-005/006 block strings, AC-010 caps,
  EC-006 path rule, new Write/Edit/MultiEdit ACs. ARCH-INDEX v2.21→v2.22 pending
  state-manager codification burst. rc.21 HELD pending S-17.04 + Rust port.
  Architect: S-17.04, issue #170.
