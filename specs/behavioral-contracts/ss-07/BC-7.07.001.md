---
document_type: behavioral-contract
level: L3
version: "1.3"
status: draft
producer: product-owner
timestamp: 2026-06-14T00:00:00Z
last_amended: "2026-06-14 (v1.3) — F2 pass-3 fix-burst: (F-P3-006) PC8 exit-code contract corrected per BC-1.15.001 dispatcher taxonomy: exit 1 removed; git COMMIT failure → exit 2 (block compaction; durability-critical); append-to-precompact-flush-log failure that does NOT fail the commit → exit 0 + stderr WARNING (fail-open per on_error=continue; non-blocking); ONLY exit codes 0 and 2 now present; consistency check with BC-5.41.003 EC-003 stale-SHA tolerated fail-open window. ADR cite v1.1→v1.3. [Prior: 2026-06-14 (v1.2) — F2 pass-2 fix-burst: (F-P2-002/003) H1 enriched (F-P2-006): lock-renew when-held/no-op-when-absent, PreCompact flush <cycle>/<step> commit message, append-SHA-to-precompact-flush-log before push. PC8 updated: last-precompact-flush-sha side-channel point-file → precompact-flush-log append-log (SHA as new last line; append must succeed before push; exit 1 on append failure). Architecture Anchors: last-precompact-flush-sha → precompact-flush-log append-log. VP-085 row: phantom current_wave: → current_cycle+current_step; append-SHA-to-precompact-flush-log before push. Test vector: synthetic cycle example to avoid phantom current_cycle value; SHA → appended to precompact-flush-log. [Prior: 2026-06-14 (v1.1) — F2 pass-1 fix-burst: (F-1) PC1 re-anchored: removes `current_wave:` phantom field reference; hook derives context identity from STATE.md `current_cycle:` + `current_step:` fields. PC4 commit message format updated to `PreCompact flush <cycle>/<step> <ISO-timestamp>` (general prefix per locked convention). Inv 4 commit-message pattern updated to match new format. EC-007 updated to use new message format. (F-2) PC3 + Inv 3 + EC-004 re-anchored: lock-renew is no-op (advisory) when `factory_lock` is absent/null in STATE.md per ADR-025 opt-in model; added EC-009 explicit no-lock-held edge case. (DI) L2 Domain Invariants replaced TBD-DI with DI-021+DI-022+DI-025 per BC→DI lift map.]"
phase: F2
inputs:
  - .factory/feature-delta/issue-173/F1-delta-analysis.md
  - .factory/specs/architecture/decisions/ADR-026-wave-boundary-checkpoint-reset-and-lossless-intra-wave-compaction.md
  - .factory/specs/architecture/decisions/ADR-025-single-writer-factory-locklease-prevent-concurrent-session-races-on-factory-artifacts-orphan-branch.md
input-hash: "c2426d5"
traces_to: .factory/specs/prd.md
origin: greenfield
extracted_from: null
subsystem: "SS-07"
capability: "CAP-032"
lifecycle_status: draft
introduced: v1.0-feature-context-durability-E18
modified:
  - "2026-06-14 (v1.3) — F2 pass-3 fix-burst: PC8 exit-code contract corrected (F-P3-006): exit 1 removed; git COMMIT failure → exit 2; append failure (non-commit) → exit 0 + stderr WARNING; ADR cite v1.1→v1.3."
  - "2026-06-14 (v1.2) — F2 pass-2 fix-burst: H1 enriched (F-P2-006); PC8 side-channel → precompact-flush-log append-log (last line + append before push + exit 1 on append failure); Arch Anchors updated; VP-085 phantom current_wave: → current_cycle+current_step + append-log; test vector synthetic cycle example."
  - "2026-06-14 (v1.1) — F2 pass-1 fix-burst: PC1 re-anchored current_cycle+current_step (removes phantom current_wave:); PC3+Inv3+EC-004 re-anchored lock-renew as conditional no-op when factory_lock absent/null per ADR-025 opt-in; PC4+Inv4 commit message updated to general PreCompact flush <cycle>/<step> format; EC-007 + test vectors updated; EC-009 no-lock-held edge case added; TBD-DI replaced with DI-021+DI-022+DI-025; ADR cite v1.0→v1.1."
deprecated: null
deprecated_by: null
replacement: null
retired: null
removed: null
removal_reason: null
---

# BC-7.07.001: precompact-flush.sh fires synchronously on PreCompact; hermetic (STATE.md + git only); renews factory lock when held (no-op when absent); commits with `PreCompact flush <cycle>/<step>` message; appends commit SHA to precompact-flush-log before push; blocks (exit 2) only on commit failure; fail-open on crash

## Description

`precompact-flush.sh` is a synchronous shell hook registered under `event = "PreCompact"` in `hooks-registry.toml`. It fires before each harness auto-compaction event and ensures that `STATE.md` and wave-critical state are committed to the `factory-artifacts` branch before compaction proceeds. The hook is hermetic (reads only STATE.md and git; never reads in-context state), renews the factory lock per ADR-025 Decision 11 Mechanism 1 before committing, and exits 0 (fail-open) on any failure except a git commit error (which exits 2 to block compaction). This implements ADR-026 Decision 6 in full.

## Preconditions

1. The hook is registered in `hooks-registry.toml` as:
   ```toml
   [[hooks]]
   name = "precompact-flush"
   event = "PreCompact"
   plugin = "hook-plugins/legacy-bash-adapter.wasm"
   priority = 100
   timeout_ms = 30000
   on_error = "continue"
   async = false

   [hooks.config]
   script_path = "hooks/precompact-flush.sh"
   ```
   (Full capabilities block per ADR-026 §Decision 6 corrected TOML schema.)
2. The Claude Code harness version >= v2.1.105 (PreCompact blocking supported).
3. The dispatcher routes `PreCompact` events to registered plugins (BC-1.15.001 postconditions satisfied).
4. `factory-lock-write.sh` exists at `plugins/vsdd-factory/hooks/factory-lock-write.sh` and supports `renew` subcommand (ADR-025 D11 Mechanism 1 deliverable; S-17.04 dependency).
5. The `factory-artifacts` branch is accessible via `git`.
6. `STATE.md` is accessible at `.factory/STATE.md` relative to the plugin root or working directory.

## Postconditions

1. **STATE.md read from filesystem**: The hook reads `STATE.md` via direct filesystem read (`cat .factory/STATE.md` or equivalent); it determines context identity from `STATE.md` frontmatter `current_cycle:` and `current_step:` fields. It does NOT look for a non-existent `current_wave:` field. It does NOT rely on in-context reasoning, environment variables set by the LLM, or piped session context.

2. **custom_instructions NOT used**: The hook does NOT read from `custom_instructions`, `system_prompt`, or any in-context LLM mechanism. These are unreliable on auto-compaction (confirmed in F1 research). All flush data comes from STATE.md and git.

3. **Factory lock renewed or skipped**: The hook reads the `factory_lock:` block from `STATE.md`. If `factory_lock:` is absent from STATE.md or `factory_lock.holder` is absent/null, the lock is not held and the lock-renewal step is skipped entirely (no-op per ADR-025 opt-in model; ADR-026 Decision 6 §Step 2). If the lock IS held, the hook invokes `factory-lock-write.sh renew .factory/STATE.md` (ADR-025 D11 Mechanism 1) before any git commit. Lock renewal failure is treated as a non-fatal warning (advisory); the flush proceeds and attempts the commit regardless. Lock renewal failure does NOT cause exit 2.

4. **Factory-artifacts commit**: The hook stages and commits `STATE.md` (and any other factory-artifacts files with pending changes) to the `factory-artifacts` branch with commit message: `PreCompact flush <cycle>/<step> <ISO-8601-timestamp>` where `<cycle>` and `<step>` are the `current_cycle:` and `current_step:` values read from STATE.md frontmatter (e.g., `PreCompact flush v1.0-feature-context-durability-E18/S-18.04 2026-06-14T12:00:00Z`).

5. **Exit 0 on flush success or clean state**: If the commit lands successfully, the hook exits 0. If there are no pending changes on `factory-artifacts` (clean state, nothing to flush), the hook exits 0 without creating an empty commit.

6. **Exit 2 on commit failure (block compaction)**: If the git commit command fails for any reason (network error, lock contention, permission error), the hook exits 2. This signals the dispatcher to block compaction via `block_intent = true`. The hook writes a diagnostic message to stderr before exiting 2.

7. **Exit 0 + warn on STATE.md unreadable**: If STATE.md is absent or unreadable, the hook exits 0 (fail-open per ADR-026 Decision 6 `on_error = "continue"` spirit). A warning is written to stderr: `precompact-flush: STATE.md unreadable; flush skipped.`

8. **precompact-flush-log append**: After the git commit succeeds, the hook APPENDS the resulting commit SHA (40-char hex) as a new line to `.factory/hooks/precompact-flush-log` (creating the file if absent). If the append fails, the hook writes a WARNING to stderr (`precompact-flush: SHA append to precompact-flush-log failed; log entry absent; flush commit is durable`) and exits 0 (fail-open per `on_error = "continue"` spirit; the git commit has already succeeded). The append failure does NOT exit 2 and does NOT prevent the push; git push proceeds regardless of append outcome. The last line of this file is read by `wave-handoff` to populate `HANDOFF.md` `precompact_flush_sha` field (BC-5.41.001 PC5); if the log entry is absent due to a prior append failure, BC-5.41.003 EC-003 treats the absent log as a stale/skip case, so the fail-open window is tolerated. Prior lines are retained as history; only the last line is authoritative. The ONLY exit codes this hook ever produces are 0 (success, no-op, or non-fatal append failure) and 2 (git commit failure blocks compaction). Exit 1 is not used.

9. **Hook crash — on_error = "continue"**: If the hook script crashes before emitting an exit code (set -euo pipefail failure, WASM sandbox error), the dispatcher's `on_error = "continue"` setting treats the crash as exit 0. Compaction proceeds. The crash is logged to the dispatcher internal log with `plugin.crashed`.

## Invariants

1. **Hermetic invariant**: The hook reads ONLY from (a) `STATE.md` on the filesystem, and (b) `git` commands. It MUST NOT read from: environment variables set by the LLM, `custom_instructions`, in-context tool call results, or any other session-state mechanism. This closes the anti-deadlock invariant (F1 regression risk §4.1 R1).

2. **`custom_instructions` exclusion is absolute**: Any version of `precompact-flush.sh` that reads `$CUSTOM_INSTRUCTIONS` or invokes any harness API to retrieve session context is a specification violation, regardless of fallback behavior.

3. **factory-lock-write.sh renew is conditional**: If `factory_lock:` is absent or `factory_lock.holder` is absent/null in STATE.md, the lock is not held and the renew call is skipped (no-op). If the lock IS held, the renew call must occur before the git commit. Order: (1) read STATE.md, (2) check `factory_lock:` block — skip step 3 if absent/null, (3) if lock held: invoke `factory-lock-write.sh renew`, (4) git add, (5) git commit.

4. **Commit message format is canonical**: The commit message MUST match the pattern `PreCompact flush <cycle>/<step> <ISO-timestamp>` exactly (with a single space between `<cycle>/<step>` and the timestamp). The subject must begin with the literal prefix `PreCompact flush ` (case-sensitive, with trailing space). This general prefix is used by `validate-burst-log` and `validate-dispatch-advance` to exempt the commit from TD-VSDD-053 `MULTI_COMMIT_CHAIN_NOT_ALLOWED` detection (BC-5.41.003).

5. **Empty commit is forbidden**: If `git diff --cached factory-artifacts` shows no changes, the hook must NOT create an empty commit. It exits 0 silently or with an info message: `precompact-flush: no pending changes; flush skipped.`

6. **Script hardening**: The script uses `set -euo pipefail` throughout. Any unhandled error is a crash (triggering `on_error = "continue"` fail-open per Postcondition 9).

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | STATE.md exists; no pending factory-artifacts changes | Exit 0 with info message; no commit; no side-channel SHA file update |
| EC-002 | STATE.md unreadable (missing or permissions error) | Exit 0 + warn to stderr; flush skipped; compaction proceeds |
| EC-003 | git commit fails (network error to remote) | Exit 2; stderr diagnostic; compaction blocked |
| EC-004 | factory-lock-write.sh renew fails (lock held but renewal call exits non-zero) | Advisory warning to stderr; flush commit attempted anyway; exit 0 if commit succeeds |
| EC-009 | STATE.md `factory_lock:` block is absent or `factory_lock.holder` is null (no lock held) | Lock-renewal step skipped entirely (no-op per ADR-025 opt-in model); flush proceeds directly to git add + commit; exit 0 if commit succeeds |
| EC-005 | Harness version < v2.1.105 (pre-v2.1.105 notification-only PreCompact) | Hook fires as notification; exit 2 is shown to user in stderr but NOT honoured as a compaction block by older harness. Flush still commits to factory-artifacts (durability is achieved even if block is not). |
| EC-006 | Hook crashes before emitting exit code | on_error=continue; dispatcher exits 0; compaction proceeds; plugin.crashed in dispatcher log |
| EC-007 | Two consecutive PreCompact events (rapid compactions) | Two flush commits on factory-artifacts with different timestamps; both exempt from TD-VSDD-053 via `PreCompact flush ` prefix; no multi-commit-chain violation |
| EC-008 | PreCompact fires during a state-manager burst (burst in progress) | Both commits appear on factory-artifacts; the PreCompact commit is lifecycle-orthogonal (ADR-026 D10); the burst-log entry for the enclosing burst MUST NOT cite the PreCompact commit as Commit A/B/C/D/E |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| PreCompact event; STATE.md readable; `current_cycle: <cycle>`, `current_step: <step>` (synthetic example); pending changes on factory-artifacts | flush commit with msg `PreCompact flush <cycle>/<step> <ISO-timestamp>`; exit 0; SHA appended to precompact-flush-log as new last line | happy-path |
| PreCompact event; STATE.md readable; `factory_lock:` block absent | lock-renewal skipped; flush commit attempted; exit 0 if commit succeeds | no-lock-held |
| PreCompact event; STATE.md readable; no pending changes | exit 0; no commit; no SHA file update | clean-state |
| PreCompact event; STATE.md unreadable | exit 0; warn to stderr | unreadable-state |
| PreCompact event; git commit fails | exit 2; stderr diagnostic; block_intent=true propagated to harness | commit-failure-block |
| PreCompact event; factory-lock-write.sh renew exits non-zero | advisory warn; flush commit attempted; exit 0 if commit ok | lock-renew-advisory |
| bats: `echo '{"event":"PreCompact",...}' | factory-dispatcher` with precompact-flush.sh registered | dispatcher routes to hook; hook commits or exits 0; side-channel SHA written | integration-bats |

## Related BCs

- BC-5.41.003 — composes with: MULTI_COMMIT_CHAIN exemption for PreCompact flush commits enables this BC's commits to coexist with burst commits
- BC-5.40.001 — depends on: factory-lock-write.sh renew is S-17.04 deliverable; S-18.04 depends_on S-17.04
- BC-1.15.001 — depends on: dispatcher must route PreCompact events for this hook to fire
- BC-7.07.002 — sibling: postcompact-reanchor.sh is the companion PostCompact hook

## Architecture Anchors

- `plugins/vsdd-factory/hooks/precompact-flush.sh` — NEW shell hook (S-18.04 deliverable); follows `check-factory-commit.sh` family pattern
- `plugins/vsdd-factory/hooks-registry.toml` — `[[hooks]] event = "PreCompact"` entry for this hook (S-18.04 deliverable)
- `plugins/vsdd-factory/hooks/factory-lock-write.sh` — existing hook; `renew` subcommand invoked per ADR-025 D11 Mechanism 1
- `.factory/hooks/precompact-flush-log` — append-log; commit SHA appended as new line after each successful flush commit (before push); last line is read by wave-handoff for HANDOFF.md `precompact_flush_sha` field

## Story Anchor

S-18.04 (precompact-flush.sh shell hook + registry; depends_on: S-17.04)

## VP Anchors

- VP-082 — PreCompact Flush Commits to factory-artifacts Before Compaction Proceeds
- VP-085 — PreCompact Flush Hook Is Hermetic

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-082 | precompact-flush.sh commits STATE.md + wave state before exiting; blocks (exit 2) on commit failure; no-op when state is clean; renews factory lock per ADR-025 D11 | integration |
| VP-085 | precompact-flush.sh reads STATE.md+git only; ignores custom_instructions; determines current context from STATE.md `current_cycle:` + `current_step:` fields (no phantom `current_wave:` field); appends SHA to precompact-flush-log before push; exits 0 fail-open when STATE.md unreadable | unit-test |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-032 ("Guarantee lossless context-window transitions via wave-boundary checkpoint and PreCompact flush") per capabilities.md §CAP-032 |
| Capability Anchor Justification | CAP-032 ("Guarantee lossless context-window transitions via wave-boundary checkpoint and PreCompact flush") per capabilities.md §CAP-032 — this BC specifies the PreCompact synchronous flush hook that is the PRIMARY safety net for mid-wave compaction events (ADR-026 Decision 1 + Decision 6); it directly closes the "mid-wave compaction loses STATE.md SHA" failure class documented in issue #173; hermetic + lock-renewal + commit = the three-step durability guarantee |
| L2 Domain Invariants | DI-021 (Handoff claims must be cross-checked against verifiable external ground truth, never in-context memory — enforced by hermetic STATE.md read); DI-022 (The PreCompact flush derives all flushed state exclusively from durable persisted sources — enforced by hermetic invariant + custom_instructions exclusion); DI-025 (PreCompact flush commits are lifecycle-orthogonal to state-manager burst commits — enforced by canonical commit message prefix `PreCompact flush ` and no-op behavior when state is clean) |
| Architecture Module | SS-07 (Hook Bash Layer) — shell hook in `plugins/vsdd-factory/hooks/`; registry entry in `hooks-registry.toml` |
| ADR | ADR-026 v1.3 Decision 6 (PreCompact shell hook; hermetic; blocking; fail-open on crash; lock renewal no-op when lock absent; on_error=continue) |
| Stories | S-18.04 (depends_on: S-17.04) |
| Cycle | v1.0-feature-context-durability-E18 (F2) |
| Feature | issue #173 / E-18 |
