---
document_type: architecture-decision-record
level: L3
adr_id: ADR-025
version: "1.4"
status: accepted
producer: architect
timestamp: 2026-06-10T00:00:00Z
amended: 2026-06-11T00:00:00Z
amendment_reason: "v1.3→v1.4: [S-17.04] Automatic heartbeat renewal enforcement wiring. Decision 11 added: two complementary mechanisms close the prose-only PC4 enforcement gap — (1) mandatory executable factory-lock-write.sh renew step in state-burst SKILL before git add/commit (Option A); (2) new verify-lock-renewal.sh PreToolUse bash hook that blocks a held-lock factory-artifacts push when HEAD's expires_at equals origin/factory-artifacts' expires_at (RenewalMissed — renewal not committed in this burst), on_error=continue, async=false, no-op when unlocked or no remote baseline (Option C). Decision 5 vestigial burst-END-only sentence corrected: replaces 'Renewal happens at burst END (the state-manager commit that closes the burst writes the updated expires_at).' with the authoritative mid-burst-every-commit formulation pointing to Decision 11. Deliverables D10–D14 added. BC-5.40.001 PC4 unaffected (this amendment implements PC4, does not change it). v1.2→v1.3 amendment_reason preserved inline: [process-gap] S-17.02 TDD implementation finding — exec_subprocess env_allow omission footgun. Decision 2 / D2 capability block spec was incomplete: exec_subprocess capability block listed only binary_allow = [\"git\"] but omitted env_allow. The dispatcher's exec_subprocess host function calls env_clear() and passes ONLY vars listed in caps.env_allow; without HOME (and GIT_CONFIG_GLOBAL / XDG_CONFIG_HOME) in env_allow, git config user.email cannot read the developer's global gitconfig, returns empty string, plugin hits IdentityResolutionFailed, fails open (Continue), and the lock guard is a silent no-op. This is the THIRD instance of the deny-by-default silent-no-op footgun class (first: read_file block omitted; second: exec_subprocess binary_allow omitted; third: exec_subprocess env_allow omitted). Fix: Decision 2 and D2 canonical registry snippet updated to include env_allow = [\"HOME\", \"GIT_CONFIG_GLOBAL\", \"XDG_CONFIG_HOME\"] on the exec_subprocess capability block. Rationale section updated to name all three footgun vectors explicitly. Process note added."
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

**ACCEPTED — human design confirmed 2026-06-10; research-agent verification APPROVE-WITH-FIXES incorporated as v1.2. D-540 codification recorded by state-manager 2026-06-10. Implementation dispatch ready. v1.3 amended 2026-06-11: [process-gap] S-17.02 TDD finding — exec_subprocess env_allow omission footgun; env_allow = ["HOME", "GIT_CONFIG_GLOBAL", "XDG_CONFIG_HOME"] added to D2 canonical registry form. v1.4 amended 2026-06-11: [S-17.04] Decision 11 added — automatic heartbeat renewal enforcement (executable state-burst SKILL step + PreToolUse push gate); Decision 5 vestigial burst-END sentence corrected; Deliverables D10–D14 added.**

This ADR resolves the design for the factory lock/lease primitive requested in issue #170.
All eleven decisions are confirmed by human review. Five research-agent fixes are incorporated
in v1.2, one process-gap spec-drift amendment in v1.3, and one enforcement-wiring amendment
in v1.4 (see amendment_reason above). No further human-gated questions remain.

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
targeting `factory-artifacts`, the gate:

1. Checks for a `factory-artifacts` push pattern in the tool input. Any Bash command that
   does not match `git.*push.*factory-artifacts` returns exit 0 (Continue) immediately —
   non-push commands add zero overhead.
2. Reads `factory_lock.holder` and `factory_lock.expires_at` from the local committed HEAD:
   `git -C .factory show HEAD:STATE.md`. At PreToolUse time the commit already exists
   locally (the `git commit` ran before the push Bash command fires), so HEAD reflects the
   staged renew if Mechanism 1 was followed.
3. If `factory_lock.holder` is absent in HEAD (factory unlocked): returns exit 0. No-op.
4. If `origin/factory-artifacts` does not exist (first push to a new branch): returns exit 0.
5. Reads `factory_lock.expires_at` from the remote tip:
   `git -C .factory show origin/factory-artifacts:STATE.md`.
6. If HEAD `expires_at` equals `origin/factory-artifacts` `expires_at` (the value was NOT
   refreshed in this burst's commits): returns exit code 2 (block) with the message:
   ```
   RenewalMissed — factory_lock is held but expires_at was not refreshed in this burst.
   Run: factory-lock-write.sh renew .factory/STATE.md
   Then: git -C .factory add STATE.md && git -C .factory commit --amend --no-edit
   Then retry the push.
   ```
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
amendment is required. S-17.04 is the story that wires the enforcement mechanism.

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
| D10 | `state-burst` SKILL renewal step | `plugins/vsdd-factory/skills/state-burst/SKILL.md` | Add mandatory step before `git -C .factory add -A` / `git commit`: `bash plugins/vsdd-factory/bin/factory-lock-write.sh renew .factory/STATE.md`. Annotate as no-op when unlocked. Also add anti-pattern row: "Skipping renew before git add while lock held → RenewalMissed gate blocks the push." Reuses existing `factory-lock-write.sh renew` subcommand (S-17.01 deliverable). |
| D11 | `verify-lock-renewal.sh` PreToolUse gate | `plugins/vsdd-factory/hooks/verify-lock-renewal.sh` | New bash hook. Filters on `git.*push.*factory-artifacts` pattern; exit 0 immediately for non-push commands. Compares `git -C .factory show HEAD:STATE.md` `expires_at` vs `git -C .factory show origin/factory-artifacts:STATE.md` `expires_at`; blocks with `RenewalMissed` message (exit 2) when held lock's `expires_at` identical (not refreshed this burst). No-op when unlocked or no remote baseline (`origin/factory-artifacts` absent). |
| D12 | Registry entry for `verify-lock-renewal.sh` | `plugins/vsdd-factory/hooks-registry.toml` | New `[[hooks]]` entry: `name = "verify-lock-renewal"`, `event = "PreToolUse"`, `tool = "Bash"`, `plugin = "hook-plugins/legacy-bash-adapter.wasm"`, `async = false` (REQUIRED), `on_error = "continue"`, `timeout_ms = 5000`. Capabilities: `env_allow = ["PATH", "HOME", "TMPDIR", "CLAUDE_PROJECT_DIR", "CLAUDE_PLUGIN_ROOT"]`; `exec_subprocess.binary_allow = ["bash", "git"]`. Registered after `verify-factory-lock-bash` in priority order. |
| D13 | `state-manager.md` obligation amendment | `plugins/vsdd-factory/agents/state-manager.md` | Amendment to existing §"factory_lock Write/Renewal/Clear Obligation" (line ~240): add a cross-reference sentence at the end of §Sequencing invariants pointing to the `state-burst` SKILL as the executable enforcement mechanism for Invariant 2 (renew on every intermediate commit). The obligation table and sequencing prose are already correct; this adds the pointer to close the prose/skill gap explicitly. |
| D14 | Bats tests for Decision 11 | `plugins/vsdd-factory/tests/verify-lock-renewal.bats` (or appended to existing lock test suite) | Cover: (a) gate blocks push when lock held and HEAD `expires_at` equals remote `expires_at` (renewal not committed); (b) gate passes when lock held and HEAD `expires_at` differs from remote (renewal committed); (c) gate exit 0 when no lock held; (d) gate exit 0 when `origin/factory-artifacts` does not exist; (e) gate exit 0 for non-push Bash command (pattern filter). |

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

### Status as of v1.4 (amended, 2026-06-11)

Human design confirmed. Research-agent verification APPROVE-WITH-FIXES incorporated (v1.2).
v1.3 [process-gap] amendment incorporated: exec_subprocess env_allow footgun closed; D2
canonical registry form updated. v1.4 [S-17.04] amendment incorporated: Decision 11 added
(automatic heartbeat renewal enforcement — executable state-burst SKILL step + PreToolUse
push gate); Decision 5 vestigial burst-end-only sentence corrected; Deliverables D10–D14
added; BC-5.40.001 PC4 confirmed unaffected. No further human-gated questions remain. All
eleven decisions are final. D-540 codification recorded by state-manager 2026-06-10.
Implementation stories may be dispatched; S-17.04 implements Decision 11.

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
