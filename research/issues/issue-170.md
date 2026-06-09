# Issue #170 — Single-writer factory lock/lease (prevent concurrent developers racing factory-artifacts)

**Date:** 2026-06-09
**Issue:** [#170](https://github.com/drbothen/vsdd-factory/issues/170) — `feat(state): single-writer factory lock/lease — prevent concurrent developers racing the same repo's factory-artifacts state`
**Label:** enhancement
**Repo state:** `develop` @ `82163b7f`, plugin `v1.0.0-rc.20`
**Research agent:** Claude (vsdd-factory research-agent)

---

## Restated proposal

The factory has no mechanism to enforce **single-writer access at the session/developer level**. Two developers (or two Claude Code sessions) can run pipelines against the same repo's `factory-artifacts` orphan branch concurrently; because all factory state converges on that one branch that every session pushes to, concurrent runs race and one party's state/spec commits can be silently lost or clobbered (or produce a painful orphan-branch divergence with no merge base). The existing single-commit burst protocol (TD-VSDD-053) and the "`state-manager` writes last / is the only writer" rule eliminate version-race *within* a single session but have **no equivalent across sessions**. The issue proposes a **lock/lease primitive** — acquire at pipeline start, heartbeat-renew each burst, auto-expire on crash, explicit force-release/steal — scoped to `(repo, factory)`, with three candidate implementations (A: filesystem `.factory/.lock`; B: git ref / lightweight tag for true remote exclusivity; C: STATE.md frontmatter ownership). The reporter leans **C for UX + B for the real remote guarantee**.

---

## Codebase grounding

### What exists today (verified)

| Capability | Status | Evidence |
|---|---|---|
| Lock / lease / heartbeat / owner / session-id primitive | **ABSENT** | `grep -ri 'lock|flock|mutex|lease|heartbeat|session_id|exclusive'` across `plugins/` returns only unrelated hits (workflow names, "block" tokens, the plugin-cache `.in_use/` PID-refcount which is Claude Code's own cache mechanism, not factory state). No cross-session guard exists. |
| Single-writer (within session) | EXISTS | `state-manager` is sole `.factory/` writer, runs last in every burst, commits atomically via single-commit burst protocol (TD-VSDD-053). `agents/state-manager.md`, `skills/state-burst/SKILL.md`. **Within one orchestrated session only.** |
| `.factory/` push | EXISTS — **no CAS / no fetch-before-push** | `skills/state-burst/SKILL.md:159` → `git -C .factory push origin factory-artifacts`. No `git fetch` + compare, no `--force-with-lease`, no expected-old-value guard before the push. This is the exact race window. |
| `verify-git-push.sh` | EXISTS — does NOT lock | `hooks/verify-git-push.sh` only blocks `--force`/`-f` and pushes to protected branches (main/master/develop). It explicitly *allows* `git push origin factory-artifacts` (line 15) with no exclusivity check. |
| Post-hoc integrity checks | EXIST — not exclusionary | `bin/compute-input-hash --check` (exit 2 on input drift, *after the fact*); `verify-sha-currency.sh` (rejects multi-commit chains, verifies SHA currency *after* a commit); `validate-state-*` hooks gate STATE.md content. None acquire exclusive access. |
| `factory-worktree-health` / `factory-health` | EXIST — no lock observability | `skills/factory-worktree-health/SKILL.md` checks remote-branch existence, local mount, sync state, auto-repairs — `grep -i 'lock|lease|heartbeat|exclusive|owner'` → **0 matches**. No lock status surfaced. |
| `docs/AGENT-SOUL.md:199` | "Files support concurrent access from parallel agents" | True for parallel agents *inside one session* (serialized via `state-manager`); does **not** hold across independent sessions/developers — exactly the gap the issue identifies. |

**Conclusion:** The issue's reading of the codebase is **accurate**. The within-session single-writer discipline is real and robust; the cross-session guard is genuinely absent, and the push path (`state-burst` line 159) is a plain `git push` with no compare-and-swap, confirming the race window. No prior CHANGELOG/decision-log work on locking/lease.

---

## External research (primary sources)

Deep-research synthesis (Perplexity Sonar, `reasoning_effort=high`) over git-scm, kubernetes.io, etcd.io, and distributed-systems literature.

### (1) File-based advisory locking — failure modes
- POSIX-compliant file locks give **foundational atomicity** but **NFS inconsistencies and PID reuse** make pure lockfiles unsafe → hybrid approaches (lockfile + heartbeat/health-signal) are required.
- The **stale-lock problem** is the dominant failure: "a process holding the lock might verify the lockfile's contents, commence its critical section, and subsequently crash before deleting the lockfile — leaving behind a stale lock that permanently blocks other processes." In pipelines this causes *cascading* failures (subsequent jobs wait indefinitely on a nonexistent resource).
- **Mitigation = turn the lock into a lease:** embed a timestamp; contenders treat the lock as valid only if its timestamp exceeds a TTL (**typically 2–5× expected operation duration**). "This transforms the lock from a persistent state into a **lease** with automatic expiration, directly addressing the stale lock problem." Reinforce with **hybrid verification** (filesystem op + secondary health signal — e.g., is the holder PID/session still alive).
- **OS advisory-lock leases (`flock`/`fcntl`) don't cross machines** and orphan on crash — viable only locally, and only combined with a remote mechanism. (Primary: LWN `https://lwn.net/Articles/817905/`.)

### (2) Lease/TTL with fencing — split-brain prevention
- **Fencing tokens are non-negotiable** for correctness: "monotonically increasing values associated with each lock acquisition — which allow storage systems to reject operations bearing outdated tokens, thereby preventing split-brain scenarios where multiple processes concurrently believe they hold the lock." (Classic Kleppmann critique of Redlock.)
- **etcd and Kubernetes Leases implement robust fencing via monotonically increasing revision counters; Consul's implementation lacks equivalent split-brain prevention during network partitions** — so prefer revision/CAS-backed primitives over Consul-style session locks if correctness under partition matters. (Primary: kubernetes.io Lease API, etcd.io lease/lock docs.)
- A releasing process must **invalidate its fencing token before** the lock becomes available — no window where two tokens are simultaneously valid.

### (3) Git ref / tag as a remote mutual-exclusion lock — the key fact for Option B
- **Git's refspec compare-and-swap is the mechanism.** `git push --force-with-lease` "extends `--force` by adding a conditional check: the push succeeds only if the remote reference still points to the expected object ID … the verification and update occur as a **single atomic operation on the server side**." This is real CAS, and `--force-with-lease` is already *allowed* by the factory's `verify-git-push.sh` (it only blocks raw `--force`).
- **Server-side atomicity varies by host (critical caveat):** "GitHub Enterprise enforces strict compare-and-swap semantics while GitLab's implementation historically permitted ref collision under high contention." So the *guarantee* is host-dependent — do not assume universal server-side CAS; validate against the actual remote (GitHub.com for this repo).
- Acquire by creating a ref like `refs/factory-lock/<repo>` (creation is atomic server-side; an already-exists / non-fast-forward rejection = "someone else holds it"); heartbeat by re-pushing a new expiry; release by deleting the ref. This gives **true cross-machine mutual exclusion** that file content (clobberable) cannot. (Primary: git-scm push / `git-push` man page on `--force-with-lease` and refspec semantics.)

### (4) git-worktree concurrency hazards
- Multiple worktrees/processes pushing the **same branch** hit **non-fast-forward rejection** under branch protection, OR clobber under force. For an **orphan branch with no merge base** (exactly `factory-artifacts`), a diverged history is **painful to reconcile** (no common ancestor → manual surgery) — confirming the issue's failure-scenario step 3.
- The robust pattern is **fetch → rebase/CAS-check → push** (compare-and-swap), never a blind push. The factory's current `state-burst` blind push (line 159) is precisely the anti-pattern.

---

## Options comparison (for the implementer)

| Option | Cross-machine exclusivity | Stale-lock recovery | Infra cost | Race on the lock itself |
|---|---|---|---|---|
| **A — filesystem `.factory/.lock`** | NO (local only) | orphans on crash unless TTL'd | lowest | n/a (local) |
| **B — git ref `refs/factory-lock/<repo>`** | **YES (server-side atomic CAS, host-dependent)** | TTL-expiry encoded in ref payload + steal-by-delete | pre-push hook + ref convention | none — ref create is atomic |
| **C — STATE.md frontmatter `factory_lock`** | weak (travels in the branch it protects → clobberable) | TTL via `expires_at`; human-readable who/when | none (fits single-writer model) | YES unless acquisition uses fetch-then-CAS push |

**Synthesis matches the reporter's lean:** **C for developer-facing UX** (frontmatter shows holder / acquired_at / expires_at for humans) **+ B for the real remote guarantee** (git ref is the enforced CAS lock). A is only viable bolted onto B. Add a **fencing/expiry field** to whichever payload is authoritative, and make every acquisition go through **fetch-then-CAS** (`--force-with-lease` or ref-create), never a blind push.

---

## Verdict

> **VALID-NEW** — Confidence: **High**

The cross-session lock/lease primitive is verifiably absent (0 relevant grep hits; push path is a blind `git push` with no CAS; `verify-git-push.sh` does not lock; health skills surface no lock state). The race is real for the orphan `factory-artifacts` branch. The proposal is technically sound, and the reporter's preferred design (frontmatter UX + git-ref CAS enforcement + TTL auto-expiry) is the one best-supported by primary sources — with the **important caveat that server-side ref CAS is host-dependent** (verify against GitHub.com) and **fencing/TTL is mandatory** to avoid stale-lock wedging and split-brain.

---

## Recommended approach + scope (zero re-research)

### Primitive design (route: architect → ADR, then data-engineer + devops-engineer)
- **Enforcement layer (Option B): git ref `refs/factory-lock/<repo-slug>`.** Acquire via atomic ref create / `--force-with-lease` CAS at pipeline start (first `state-manager` write). Already-exists/non-fast-forward → refuse with actionable message (holder, acquired_at, expires_at, force-release cmd). Heartbeat = re-push new `expires_at` each burst. Release = delete ref on clean exit. **Verify GitHub.com enforces server-side CAS** (research flags GitLab historically did not — confirm the actual remote).
- **UX layer (Option C): STATE.md frontmatter `factory_lock`** block (`session_id`, `holder`, `acquired_at`, `expires_at`) — human-readable mirror of the ref; `state-manager` reads it before any write and refuses on a non-matching unexpired lock. **Acquisition of the frontmatter lock must itself go through fetch-then-CAS** (the issue's own Option-C caveat).
- **TTL:** default **30–60 min** (research: 2–5× expected burst duration), renewed each burst. **Fencing:** carry a monotonic token / use the git ref's object-id chain as the fencing value so a stale holder's late push is rejected.
- **Stale recovery:** explicit `factory release` / `factory steal` command; logged. Auto-expire when `now > expires_at`.

### Enforcement & observability points
- **Pre-push / PreToolUse hook:** block `.factory/` writes / `factory-artifacts` pushes when the lock is held by another live session. Extend `verify-git-push.sh` (shell, effectful — it already understands `factory-artifacts` and `--force-with-lease`) or add a sibling `verify-factory-lock.sh`.
- **Fix the blind push:** change `state-burst` line 159 from `git push origin factory-artifacts` to a **fetch-then-CAS push** (`git push --force-with-lease=factory-artifacts:<expected-sha>` after a fetch), so even *without* the lock, concurrent pushes are detected and blocked rather than clobbering. This is a high-value, low-cost mitigation that stands alone.
- **Observability:** surface lock state (holder/since/expires) in `factory-health` / `factory-worktree-health` at session start.

### Risks
- **Host-dependent server-side ref CAS** — biggest correctness risk; verify GitHub.com behavior before relying on it. If unconfirmed, the frontmatter-CAS push + non-fast-forward rejection is the fallback enforcement.
- **Stale-lock wedging** — mandatory TTL + steal path; never a lock without expiry (research: pipelines cascade-fail on stale locks).
- **Split-brain on partition** — fencing token required; do not adopt a Consul-style session lock without it.
- **Multi-machine identity:** canonical session identity is an open question (issue) — recommend `hostname + pid + claude-session-id` composite; the git user alone is insufficient (one dev, two machines).
- **Multi-repo `.factory-project/`** (issue open question): start whole-factory granularity; a project-level lock can layer on later.

### Test strategy
- Unit: TTL expiry, fencing-token rejection of a stale holder's late write, force-release logging.
- Integration (two-session sim): session B refused with actionable message while A holds; A's lease renews each burst; A crashes → lock auto-expires → B acquires without git surgery; concurrent push → `--force-with-lease` rejects B instead of clobbering A.

### Dependencies
- Architect ADR on identity + granularity + enforcement-layer choice. Operator-level cache picks up new hooks only after a release. Reduces blast radius of the **#173** fabrication failure mode (verified external state at every boundary). Single-developer behavior must be unchanged (issue AC).

---

## Research Methods

| Tool | Queries | Purpose |
|------|---------|---------|
| **Perplexity perplexity_research (PRIMARY)** | 1 | File-lock failure modes, lease/TTL + fencing (etcd/k8s/Consul), git-ref CAS atomicity, worktree concurrency hazards (deep multi-source, `reasoning_effort=high`) |
| Read / Grep / Glob | ~12 | Codebase grounding: lock-absence grep, state-burst push path, verify-git-push.sh, worktree-health, AGENT-SOUL.md:199, prior CHANGELOG/decision-log |
| Training data | 0 load-bearing | Fencing-token / Kleppmann / git CAS claims all cross-checked against research synthesis citing git-scm, kubernetes.io, etcd.io |

**Total MCP tool calls (this issue):** 1 research. **Training data reliance:** LOW — all distributed-lock and git-CAS facts sourced from the deep-research synthesis citing primary docs (git-scm, kubernetes.io, etcd.io, LWN).
